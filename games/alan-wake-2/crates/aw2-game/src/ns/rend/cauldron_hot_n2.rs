//! Decompiled hot functions shard N2 (`0x14230b890..0x142422da0`).

use crate::exe::{self, rd, wr};

/// `unknown_14230b890`
/// Original: `0x14230b890` (1394 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_14230b890(param_1: usize) {
    let g_82f8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82f8) as usize);
    exe::call_va!(0x1_4248_5780, (g_82f8, param_1.wrapping_add(0x70)));

    let mut has_state_3 = false;
    let mut min_val = rd::<u64>(param_1 + 0xe0).wrapping_sub(1);

    let mut cur_ptr = rd::<usize>(param_1 + 0x80);
    let count_88 = rd::<u32>(param_1 + 0x88) as usize;
    let end_ptr = cur_ptr.wrapping_add(count_88 * 8);

    while cur_ptr != end_ptr {
        let item = rd::<usize>(cur_ptr);
        if rd::<u8>(item) == 3 {
            has_state_3 = true;
            let idx_4 = rd::<u32>(item + 0x04);
            if idx_4 != 0xffff_ffff {
                let arr_20 = rd::<usize>(param_1 + 0x20);
                let old_byte = rd::<u8>(arr_20.wrapping_add(idx_4 as usize));
                wr::<u8>(arr_20.wrapping_add(idx_4 as usize), rd::<u8>(item + 0x80));

                let item2 = rd::<usize>(cur_ptr);
                let new_byte = rd::<u8>(item2 + 0x80);
                if new_byte < old_byte {
                    let idx2 = rd::<u32>(item2 + 0x04) as usize;
                    let rcx_arg = rd::<u64>(rd::<usize>(param_1).wrapping_add(idx2 * 8));
                    let mut span_50: [u64; 2] =
                        [rd::<u32>(item2 + 0xf0) as u64, rd::<u64>(item2 + 0xe8)];
                    exe::call_va!(
                        0x1_4232_f6f0,
                        (
                            rcx_arg,
                            new_byte as u64,
                            span_50.as_mut_ptr() as usize,
                            item2.wrapping_add(0x428)
                        )
                    );
                } else if new_byte > old_byte {
                    let r13 = item2.wrapping_add(0xe8);
                    let want_len = (new_byte as u64).wrapping_sub(old_byte as u64);
                    let old_len = rd::<u32>(r13 + 0x08) as u64;
                    let byte_len = old_len.wrapping_mul(0x320);
                    let old_data = rd::<usize>(r13);
                    let old_end = old_data.wrapping_add(byte_len as usize);
                    let old_cap = rd::<u32>(r13 + 0x0c) as u64;

                    if want_len > old_cap {
                        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
                        if new_cap < want_len {
                            new_cap = want_len;
                        }
                        exe::call_va!(0x1_4131_24e0, (r13, new_cap));
                        let mut cur_elem = rd::<usize>(r13)
                            .wrapping_add((rd::<u32>(r13 + 0x08) as usize).wrapping_mul(0x320));
                        let mut rem = want_len.wrapping_sub(old_len);
                        while rem != 0 {
                            let _ = exe::call_va!(
                                0x1_4183_acc2,
                                (cur_elem, 0u32, 0x320u64) -> usize
                            );
                            exe::call_va!(0x1_4131_2740, (cur_elem));
                            cur_elem = cur_elem.wrapping_add(0x320);
                            rem = rem.wrapping_sub(1);
                        }
                        wr::<u32>(r13 + 0x08, want_len as u32);
                    } else if want_len > old_len {
                        let mut cur_elem = old_end;
                        let mut rem = want_len.wrapping_sub(old_len);
                        while rem != 0 {
                            let _ = exe::call_va!(
                                0x1_4183_acc2,
                                (cur_elem, 0u32, 0x320u64) -> usize
                            );
                            exe::call_va!(0x1_4131_2740, (cur_elem));
                            cur_elem = cur_elem.wrapping_add(0x320);
                            rem = rem.wrapping_sub(1);
                        }
                        wr::<u32>(r13 + 0x08, want_len as u32);
                    } else if want_len != old_len {
                        let mut cur_elem =
                            old_data.wrapping_add((want_len as usize).wrapping_mul(0x320));
                        while cur_elem != old_end {
                            exe::call_va!(0x1_4131_2a40, (cur_elem));
                            cur_elem = cur_elem.wrapping_add(0x320);
                        }
                        wr::<u32>(r13 + 0x08, want_len as u32);
                    }

                    let item3 = rd::<usize>(cur_ptr);
                    let idx3 = rd::<u32>(item3 + 0x04) as usize;
                    let rcx_arg = rd::<u64>(rd::<usize>(param_1).wrapping_add(idx3 * 8));
                    let mut span_60: [u64; 2] =
                        [rd::<u32>(item3 + 0xf0) as u64, rd::<u64>(item3 + 0xe8)];
                    exe::call_va!(
                        0x1_4232_e9a0,
                        (rcx_arg, old_byte as u64, span_60.as_mut_ptr() as usize)
                    );
                }

                let item4 = rd::<usize>(cur_ptr);
                let idx4 = rd::<u32>(item4 + 0x04) as usize;
                wr::<u64>(rd::<usize>(param_1 + 0x40).wrapping_add(idx4 * 8), 0);
            }
            wr::<u8>(rd::<usize>(cur_ptr), 4);
        } else {
            let cand = rd::<u64>(item + 0x28).wrapping_sub(1);
            if cand < min_val {
                min_val = cand;
            }
        }
        cur_ptr = cur_ptr.wrapping_add(8);
    }

    wr::<u64>(param_1 + 0xe8, min_val);
    if has_state_3 {
        let mut zero_20: u64 = 0;
        exe::call_va!(
            0x1_4131_2050,
            (
                param_1.wrapping_add(0x80),
                (&mut zero_20 as *mut u64) as usize
            )
        );

        let rbp_obj = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
        let count_20 = rd::<u32>(rbp_obj + 0x20) as usize;
        for idx in 0..count_20 {
            let u16_idx = rd::<u32>(rd::<usize>(rbp_obj + 0x198).wrapping_add(idx * 4));
            if u16_idx != 0xffff_ffff {
                let u25_idx = rd::<u32>(
                    rd::<usize>(rbp_obj + 0x2b0).wrapping_add((u16_idx as usize) * 4),
                );
                if u25_idx != 0xffff_ffff {
                    let b_cur = rd::<u8>(
                        rd::<usize>(rbp_obj + 0x240)
                            .wrapping_add((u16_idx as usize) * 0x24 + 2),
                    );
                    let b_new = rd::<u8>(rd::<usize>(param_1 + 0x20).wrapping_add(u25_idx as usize));
                    if b_new < b_cur {
                        let r13_item = rd::<usize>(rd::<usize>(rbp_obj + 0x18).wrapping_add(idx * 8));
                        let rdx_sub = rd::<usize>(r13_item + 0x08);
                        let sub_cnt = rd::<i32>(rdx_sub + 0x138) as isize;
                        if sub_cnt > 0 {
                            let g_0190 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_0190) as usize);
                            for j in 0..(sub_cnt as usize) {
                                let rdx_cur = rd::<usize>(r13_item + 0x08);
                                let elem = rd::<usize>(rd::<usize>(rdx_cur + 0x130).wrapping_add(j * 8));
                                let lod = rd::<i32>(elem + 0xa8);
                                if lod >= (b_new as i32) && lod < (b_cur as i32) {
                                    let r10_ptr = rd::<usize>(elem + 0x88);
                                    let slot_id = rd::<u32>(
                                        rd::<usize>(r13_item + 0x20).wrapping_add(j * 8 + 4),
                                    );
                                    let mask_base = rd::<usize>(g_0190 + 0x1a0);
                                    let word_addr =
                                        mask_base.wrapping_add(((slot_id >> 6) as usize) * 8);
                                    wr::<u64>(
                                        word_addr,
                                        rd::<u64>(word_addr) | (1u64 << (slot_id & 0x3f)),
                                    );
                                    let dst_base = rd::<usize>(g_0190 + 0x190)
                                        .wrapping_add((slot_id as usize) * 0x34);
                                    wr::<u64>(dst_base, rd::<u64>(r10_ptr));
                                    wr::<u32>(dst_base + 0x08, rd::<u32>(r10_ptr + 0x10));
                                    let tag: u32 = if rd::<u64>(r10_ptr + 0x08) == 2 {
                                        0x39
                                    } else {
                                        0x2a
                                    };
                                    wr::<u32>(dst_base + 0x0c, tag);
                                }
                            }
                        }
                        exe::call_va!(0x1_4230_48f0, (r13_item, b_new as u32, b_cur as u32));
                    } else if b_new > b_cur {
                        let rbx_item = rd::<usize>(rd::<usize>(rbp_obj + 0x18).wrapping_add(idx * 8));
                        let rdx_sub = rd::<usize>(rbx_item + 0x08);
                        let sub_cnt = rd::<i32>(rdx_sub + 0x138) as isize;
                        if sub_cnt > 0 {
                            let g_0190 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_0190) as usize);
                            for j in 0..(sub_cnt as usize) {
                                let rdx_cur = rd::<usize>(rbx_item + 0x08);
                                let elem = rd::<usize>(rd::<usize>(rdx_cur + 0x130).wrapping_add(j * 8));
                                let lod = rd::<i32>(elem + 0xa8);
                                if lod >= (b_cur as i32) && lod < (b_new as i32) {
                                    let slot_id = rd::<u32>(
                                        rd::<usize>(rbx_item + 0x20).wrapping_add(j * 8 + 4),
                                    );
                                    let mask_base = rd::<usize>(g_0190 + 0x1a0);
                                    let word_addr =
                                        mask_base.wrapping_add(((slot_id >> 6) as usize) * 8);
                                    wr::<u64>(
                                        word_addr,
                                        rd::<u64>(word_addr) | (1u64 << (slot_id & 0x3f)),
                                    );
                                    let dst_base = rd::<usize>(g_0190 + 0x190)
                                        .wrapping_add((slot_id as usize) * 0x34);
                                    wr::<u64>(dst_base, 0);
                                    wr::<u64>(dst_base + 0x08, 0);
                                }
                            }
                        }
                        exe::call_va!(0x1_4230_48f0, (rbx_item, b_cur as u32, b_new as u32));
                    }
                }
            }
        }

        let count_248 = rd::<u32>(rbp_obj + 0x248) as usize;
        for k in 0..count_248 {
            let map_idx = rd::<u32>(rd::<usize>(rbp_obj + 0x2b0).wrapping_add(k * 4));
            if map_idx != 0xffff_ffff {
                let val = rd::<u8>(rd::<usize>(param_1 + 0x20).wrapping_add(map_idx as usize));
                wr::<u8>(rd::<usize>(rbp_obj + 0x240).wrapping_add(k * 0x24 + 2), val);
            }
        }
    }
}

/// `rend_142321130`
/// Original: `0x142321130` (622 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_142321130(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u32,
) {
    if param_3 == 0 {
        return;
    }

    let g_0180 = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize);
    let r10 = g_0180 as i64;
    let r12 = g_0180.wrapping_sub(1) as i64;
    let prev_4e8 = rd::<i64>(param_1 + 0x4e8);
    wr::<i64>(param_1 + 0x4f0, r10);

    let obj_278 = param_1.wrapping_add(0x278);
    let vtbl_278 = rd::<usize>(obj_278);
    let cap1 = exe::call_va!(rd::<usize>(vtbl_278 + 0x38), (obj_278) -> u64);
    let is_full = if cap1 != 0 {
        let v260 = rd::<u32>(param_1 + 0x260) as u64;
        let v250 = rd::<u32>(param_1 + 0x250) as u64;
        let vtbl_278_2 = rd::<usize>(obj_278);
        let cap2 = exe::call_va!(rd::<usize>(vtbl_278_2 + 0x38), (obj_278) -> u64);
        cap2 == v250.wrapping_add(v260)
    } else {
        false
    };

    if prev_4e8 != r12 && is_full {
        let vtbl_a = rd::<usize>(obj_278);
        exe::call_va!(rd::<usize>(vtbl_a + 0x18), (obj_278));
        let obj_348 = param_1.wrapping_add(0x348);
        let vtbl_b = rd::<usize>(obj_348);
        exe::call_va!(rd::<usize>(vtbl_b + 0x18), (obj_348));
        wr::<u32>(param_1 + 0x260, 0);
        wr::<u32>(param_1 + 0x250, 0);
    }

    let scratch_buf = exe::call_va!(0x1_4241_c0a0, (param_3.wrapping_mul(4), 0x10u64) -> usize);
    exe::call_va!(0x1_4231_e750, (param_1, param_2, param_3, scratch_buf));

    let total = param_3 as usize;
    let mut cur_p2 = param_2;
    for i in 0..total {
        let item_id = rd::<u32>(cur_p2);
        let mut sub_ptr = rd::<usize>(cur_p2 + 0x08);
        let slot_idx = rd::<u32>(scratch_buf.wrapping_add(i * 4));

        if param_4 != 0 {
            let base_idx = slot_idx.wrapping_mul(param_4);
            let mut offset_acc: u32 = 0;
            let mut off_38: usize = 0;
            for j in 0..param_4 {
                let val = rd::<u32>(sub_ptr);
                let arr_18 = rd::<usize>(param_1 + 0x18);
                let out_val = val.wrapping_add(offset_acc);
                if (val >> 31) == 0 {
                    let target_arr = rd::<usize>(arr_18.wrapping_add(off_38));
                    let cnt_addr = target_arr.wrapping_add((val as usize).wrapping_mul(0x28));
                    wr::<u32>(cnt_addr, rd::<u32>(cnt_addr).wrapping_add(1));
                }
                let dst_idx = base_idx.wrapping_add(j);
                sub_ptr = sub_ptr.wrapping_add(4);
                wr::<u32>(
                    rd::<usize>(param_1 + 0x3e0).wrapping_add((dst_idx as usize) * 4),
                    out_val,
                );
                let mask_word_addr =
                    rd::<usize>(param_1 + 0x3f0).wrapping_add(((dst_idx >> 6) as usize) * 8);
                wr::<u64>(
                    mask_word_addr,
                    rd::<u64>(mask_word_addr) | (1u64 << (dst_idx & 0x3f)),
                );
                offset_acc = offset_acc.wrapping_add(rd::<u32>(arr_18.wrapping_add(off_38 + 0x08)));
                off_38 = off_38.wrapping_add(0x38);
            }
        }

        let obj_418 = param_1.wrapping_add(0x418);
        let word_idx = item_id >> 5;
        let vtbl_418 = rd::<usize>(obj_418);
        let cur_len = exe::call_va!(rd::<usize>(vtbl_418 + 0x38), (obj_418) -> u64);
        if cur_len <= (word_idx as u64) {
            let new_size = (word_idx.wrapping_add(0x400) & 0xffff_fc00) as u64;
            let mut fill_zero: u64 = (i as u64) & 0xffff_ffff_0000_0000;
            let vtbl_418_2 = rd::<usize>(obj_418);
            exe::call_va!(
                rd::<usize>(vtbl_418_2 + 0x20),
                (obj_418, new_size, (&mut fill_zero as *mut u64) as usize)
            );
        }

        cur_p2 = cur_p2.wrapping_add(0x10);
        let l2_addr = rd::<usize>(param_1 + 0x4c0).wrapping_add(((word_idx >> 6) as usize) * 8);
        wr::<u64>(l2_addr, rd::<u64>(l2_addr) | (1u64 << (word_idx & 0x3f)));
        let l1_addr = rd::<usize>(param_1 + 0x4b0).wrapping_add((word_idx as usize) * 4);
        wr::<u32>(l1_addr, rd::<u32>(l1_addr) | (1u32 << (item_id & 0x1f)));
    }

    exe::call_va!(0x1_4241_c190, (scratch_buf));
}

/// `unknown_14232fb50`
/// Original: `0x14232fb50` (946 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_14232fb50(
    param_1: usize,
    param_2: i32,
    param_3: i32,
    param_4: usize,
    param_5: u64,
    param_6: u64,
    param_7: usize,
) -> usize {
    let table_base = if param_3 != 0 && rd::<u32>(param_1 + 0x178) != 0 {
        let r10 = rd::<usize>(param_1 + 0x170);
        rd::<usize>(r10.wrapping_add(((param_3 as isize) * 0x18 - 0x10) as usize))
    } else {
        rd::<usize>(param_1 + 0x160)
    };

    let p2_off = ((param_2 as isize) * 8) as usize;
    let entry = rd::<usize>(table_base.wrapping_add(p2_off));
    if entry == 0 {
        return 0;
    }
    let r8_c8 = rd::<usize>(entry + 0xc8);
    if r8_c8 == 0 {
        return 0;
    }

    let mut p7 = param_7;
    if p7 == 0 {
        p7 = rd::<usize>(r8_c8 + 0x18);
    }

    let elem_130 = rd::<usize>(rd::<usize>(param_1 + 0x130).wrapping_add(p2_off));
    let mut edx_lod = rd::<i32>(elem_130 + 0xa8);
    let b272 = rd::<u8>(param_1 + 0x272);
    let ecx_val: i32 = if b272 == 0 {
        0
    } else {
        let max_idx = (b272 as i32) - 1;
        if max_idx < edx_lod {
            edx_lod = max_idx;
        }
        rd::<i32>(param_1.wrapping_add(0x1c8).wrapping_add(((edx_lod as isize) * 4) as usize))
    };

    let off_28_30: usize = if ecx_val > 0 { 0x30 } else { 0x28 };
    let r10_val = rd::<u64>(r8_c8 + off_28_30);

    let r9_d0 = rd::<u64>(p7 + 0xd0);
    let h1 = (r9_d0 << 6)
        .wrapping_add(0x9e37_79b9)
        .wrapping_add((r9_d0 >> 2).wrapping_add(param_6))
        ^ r9_d0;
    let mut key_30 = (h1 << 6)
        .wrapping_add(r10_val)
        .wrapping_add((h1 >> 2).wrapping_add(0x9e37_79b9))
        ^ h1;

    let mut find_out = [0u64; 2];
    let _ = exe::call_va!(
        0x1_4133_ca40,
        (
            key_30,
            find_out.as_mut_ptr() as usize,
            (&mut key_30 as *mut u64) as usize
        ) -> usize
    );

    let end_ctrl = rd::<u64>(exe::ptr::<u64>(0x1_4386_c5d0) as usize)
        .wrapping_add(rd::<u64>(exe::ptr::<u64>(0x1_4386_c5b8) as usize));
    if find_out[0] != end_ctrl {
        return rd::<usize>((find_out[1] as usize) + 0x08);
    }

    let rdi_70 = rd::<usize>(p7 + 0x70);
    if rdi_70 == 0 {
        return 0;
    }

    let mut ins_out = [0u64; 3];
    let _ = exe::call_va!(
        0x1_4133_def0,
        (
            end_ctrl,
            ins_out.as_mut_ptr() as usize,
            (&mut key_30 as *mut u64) as usize
        ) -> usize
    );

    let rsi_vec = (ins_out[1] as usize).wrapping_add(0x08);
    wr::<u32>(rsi_vec + 0x08, 0);
    let old_cap = rd::<u32>(rsi_vec + 0x0c) as u64;
    if param_5 > old_cap {
        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
        if new_cap < param_5 {
            new_cap = param_5;
        }
        exe::call_va!(0x1_401d_eb40, (rsi_vec, new_cap));
    }

    for idx in 0..(param_5 as usize) {
        let str_ptr = rd::<usize>(param_4.wrapping_add(idx * 8));
        let mut rstr = [0u64; 18];
        let rstr_p = rstr.as_mut_ptr() as usize;
        wr::<u64>(rstr_p, 0x1000_0010);
        wr::<u8>(rstr_p + 0x08, 0);

        let (r14d, r12d): (u32, u32) = if str_ptr == 0 {
            wr::<u32>(rstr_p + 0x04, 0);
            wr::<u8>(rstr_p + 0x08, 0);
            (0x1000_0010, 0)
        } else {
            let mut slen: usize = 0;
            while rd::<u8>(str_ptr.wrapping_add(slen)) != 0 {
                slen = slen.wrapping_add(1);
            }
            let mut view: [u64; 2] = [str_ptr as u64, slen as u64];
            exe::call_va!(
                0x1_4240_9b50,
                (rstr_p, view.as_mut_ptr() as usize, 0u32, slen as u64)
            );
            (rd::<u32>(rstr_p), rd::<u32>(rstr_p + 0x04))
        };

        let head = rd::<usize>(rdi_70 + 0x50);
        let mut cur = rd::<usize>(head + 0x08);
        let mut cand = head;
        while rd::<u8>(cur + 0x19) == 0 {
            if exe::call_va!(0x1_4021_74f0, (cur + 0x1c, rstr_p) -> u8) != 0 {
                cur = rd::<usize>(cur + 0x10);
            } else {
                cand = cur;
                cur = rd::<usize>(cur);
            }
        }

        if rd::<u8>(cand + 0x19) != 0
            || exe::call_va!(0x1_4021_74f0, (rstr_p, cand + 0x1c) -> u8) != 0
            || cand == head
        {
            exe::call_va!(0x1_401d_e830, (rsi_vec));
            exe::call_va!(0x1_4240_ab10, (rstr_p));
            return 0;
        }

        let top_byte = r14d >> 24;
        let cap_bits = if top_byte < 0x80 {
            top_byte * 8
        } else {
            (top_byte << 8).wrapping_sub(0x7c00)
        };
        let used_bits = (r14d & 0x00ff_ffff) << 3;
        let data_ptr = if used_bits > cap_bits {
            rd::<usize>(rstr_p + 0x08)
        } else {
            rstr_p + 0x08
        };

        let mut view_60: [u64; 2] = [data_ptr as u64, r12d as u64];
        let lod_a8 = rd::<u32>(rd::<usize>(rd::<usize>(param_1 + 0x130).wrapping_add(p2_off)) + 0xa8);
        let res_eax = exe::call_va!(
            0x1_4232_ac70,
            (
                param_1,
                r8_c8,
                p7,
                view_60.as_mut_ptr() as usize,
                lod_a8
            ) -> u32
        );

        if (res_eax & (1u32 << 30)) != 0 {
            exe::call_va!(0x1_401d_e830, (rsi_vec));
            exe::call_va!(0x1_4240_ab10, (rstr_p));
            return 0;
        }

        let cur_len = rd::<u32>(rsi_vec + 0x08) as u64;
        let cur_cap = rd::<u32>(rsi_vec + 0x0c) as u64;
        if cur_len.wrapping_mul(4) == cur_cap.wrapping_mul(4) {
            let want = cur_len.wrapping_add(1);
            let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
            if new_cap < want {
                new_cap = want;
            }
            exe::call_va!(0x1_401d_eb40, (rsi_vec, new_cap));
        }

        let idx_w = rd::<u32>(rsi_vec + 0x08) as usize;
        wr::<u32>(rd::<usize>(rsi_vec).wrapping_add(idx_w * 4), res_eax);
        wr::<u32>(rsi_vec + 0x08, (idx_w as u32).wrapping_add(1));
        exe::call_va!(0x1_4240_ab10, (rstr_p));
    }

    rd::<usize>(rsi_vec)
}

/// `unknown_1423da290`
/// Original: `0x1423da290` (89 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1423da290(param_1: usize, param_2: u64) {
    if param_2 != 0 {
        let tbl_base = exe::ptr::<u64>(0x1_43bd_6048) as usize;
        let count = param_2 as usize;
        for i in 0..count {
            let idx = rd::<u32>(param_1.wrapping_add(i * 4)) as usize;
            let arg = rd::<u64>(tbl_base.wrapping_add(idx * 8));
            exe::call_va!(0x1_4229_9ad0, (arg));
        }
    }
}

/// `r_142422da0`
/// Original: `0x142422da0` (765 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_142422da0(param_1: usize) {
    let mut stack_flags = [0u8; 16];
    stack_flags[0] = 0;
    stack_flags[2] = 1;
    stack_flags[8] = 1;
    stack_flags[9] = 1;
    stack_flags[10] = 0;

    let vtbl1 = rd::<usize>(param_1);
    let _ = exe::call_va!(rd::<usize>(vtbl1 + 0x18), (param_1) -> u64);

    let vtbl2 = rd::<usize>(param_1);
    let g_1438 = rd::<i32>(exe::ptr::<i32>(0x1_4387_1438) as usize) as i64;
    let u_var3 = exe::call_va!(rd::<usize>(vtbl2 + 0x18), (param_1) -> u64);
    let limit = (g_1438 as u64) << 20;
    stack_flags[1] = if u_var3 < limit { 1 } else { 0 };

    let g_1360 = rd::<u8>(exe::ptr::<u8>(0x1_4387_1360) as usize);
    let mut cur_ptr = rd::<usize>(param_1 + 0x90);
    let count = rd::<u32>(param_1 + 0x98) as usize;
    let end_ptr = cur_ptr.wrapping_add(count * 8);

    if g_1360 == 0 {
        while cur_ptr != end_ptr {
            exe::call_va!(0x1_4241_cf50, (rd::<usize>(cur_ptr)));
            cur_ptr = cur_ptr.wrapping_add(8);
        }
        return;
    }

    let flags_base = stack_flags.as_ptr() as usize;
    while cur_ptr != end_ptr {
        let item = rd::<usize>(cur_ptr);
        let idx_150 = rd::<i32>(item + 0x150) as isize;
        if rd::<u8>(flags_base.wrapping_add(idx_150 as usize)) != 0 {
            exe::call_va!(0x1_4241_cf50, (item));
        }
        cur_ptr = cur_ptr.wrapping_add(8);
    }

    let mut qpc_e0: i64 = 0;
    exe::call_iat!(0x1_4264_56f8, ((&mut qpc_e0 as *mut i64) as usize));
    let freq0 = exe::call_va!(0x1_4240_6920, () -> i64);
    let c_1000_d = rd::<f64>(exe::ptr::<f64>(0x1_4342_6d28) as usize);
    let c_100_f = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ea4) as usize);
    let start_ticks = qpc_e0.wrapping_sub(((freq0 as f64) * 0.0f64) as i64);

    loop {
        let budget_f = rd::<f32>(exe::ptr::<f32>(0x1_4387_13f8) as usize);
        let mut any_work: u8 = 0;
        exe::call_iat!(0x1_4264_56f8, ((&mut qpc_e0 as *mut i64) as usize));
        let elapsed_ticks = qpc_e0.wrapping_sub(start_ticks);
        let freq = exe::call_va!(0x1_4240_6920, () -> i64);
        let elapsed_ms = ((elapsed_ticks as f64) / (freq as f64)) * c_1000_d;
        let budget_d = budget_f as f64;
        if !(elapsed_ms < budget_d) {
            break;
        }

        let mut iter_ptr = rd::<usize>(param_1 + 0x90);
        let iter_cnt = rd::<u32>(param_1 + 0x98) as usize;
        let iter_end = iter_ptr.wrapping_add(iter_cnt * 8);
        if iter_ptr == iter_end {
            break;
        }

        while iter_ptr != iter_end {
            let item = rd::<usize>(iter_ptr);
            let idx_150 = rd::<i32>(item + 0x150) as isize;
            if rd::<u8>(flags_base.wrapping_add(8).wrapping_add(idx_150 as usize)) != 0
                && rd::<u8>(flags_base.wrapping_add(idx_150 as usize)) == 0
            {
                let val_68 = rd::<i32>(item + 0x68);
                let val_d0 = rd::<i32>(item + 0xd0);
                let g_1398 = rd::<f32>(exe::ptr::<f32>(0x1_4387_1398) as usize);
                let f_d0 = val_d0 as f32;
                let ratio1 = (val_68 as f32) / f_d0;
                let thresh1 = g_1398 / c_100_f;
                let should_call = if ratio1 > thresh1 {
                    true
                } else {
                    let rem = val_d0.wrapping_sub(rd::<i32>(item + 0x28)).wrapping_sub(val_68);
                    let g_1378 = rd::<f32>(exe::ptr::<f32>(0x1_4387_1378) as usize);
                    let ratio2 = (rem as f32) / f_d0;
                    let thresh2 = g_1378 / c_100_f;
                    thresh2 > ratio2
                };
                if should_call {
                    let r = exe::call_va!(0x1_4241_df90, (item) -> u8);
                    any_work |= r;
                }
            }
            iter_ptr = iter_ptr.wrapping_add(8);
        }

        if any_work == 0 {
            break;
        }
    }

    if rd::<u8>(param_1 + 0x221) != 0 {
        let mut qpc_d0: i64 = 0;
        exe::call_iat!(0x1_4264_56f8, ((&mut qpc_d0 as *mut i64) as usize));
        let freq2 = exe::call_va!(0x1_4240_6920, () -> i64);
        let ticks = qpc_d0.wrapping_sub(((freq2 as f64) * 0.0f64) as i64);
        let mut pkt_20 = [0u64; 5];
        pkt_20[0] = ticks as u64;
        let budget_sec = (rd::<f32>(exe::ptr::<f32>(0x1_4387_13f8) as usize) as f64) / c_1000_d;
        let budget_bits = budget_sec.to_bits();
        let vtbl_end = rd::<usize>(param_1);
        let _ = crate::simd_ops::call_va8_xmm(
            rd::<u64>(vtbl_end + 0x08),
            param_1 as u64,
            pkt_20.as_mut_ptr() as usize as u64,
            budget_bits,
            0,
            0,
            0,
            0,
            0,
            [0, 0, budget_bits as u128, 0],
        );
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14230b890(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_5780(rcx: u64, _rdx: usize) {
        unsafe { rec(0x5780 ^ rcx) };
    }
    unsafe extern "win64" fn mock_f6f0(rcx: u64, rdx: u64, r8_span: usize, _r9: usize) {
        unsafe { rec(0xf6f0 ^ rcx ^ (rdx << 16) ^ rd::<u64>(r8_span)) };
    }
    unsafe extern "win64" fn mock_24e0(r13_vec: usize, new_cap: u64) {
        wr::<u32>(r13_vec + 0x0c, new_cap as u32);
        unsafe { rec(0x24e0 ^ new_cap) };
    }
    unsafe extern "win64" fn mock_2740(elem: usize) {
        wr::<u32>(elem + 0x10, 0x1234_5678);
        unsafe { rec(0x2740) };
    }
    unsafe extern "win64" fn mock_2a40(elem: usize) {
        wr::<u32>(elem + 0x14, 0xdead_beef);
        unsafe { rec(0x2a40) };
    }
    unsafe extern "win64" fn mock_e9a0(rcx: u64, rdx: u64, r8_span: usize) {
        unsafe { rec(0xe9a0 ^ rcx ^ (rdx << 16) ^ rd::<u64>(r8_span)) };
    }
    unsafe extern "win64" fn mock_2050(_rcx: usize, rdx_zero: usize) {
        unsafe { rec(0x2050 ^ rd::<u64>(rdx_zero)) };
    }
    unsafe extern "win64" fn mock_48f0(_rcx: usize, lo: u32, hi: u32) {
        unsafe { rec(0x48f0 ^ (lo as u64) ^ ((hi as u64) << 8)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_5780, mock_5780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4232_f6f0, mock_f6f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4131_24e0, mock_24e0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4131_2740, mock_2740 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4131_2a40, mock_2a40 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4232_e9a0, mock_e9a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4131_2050, mock_2050 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4230_48f0, mock_48f0 as *const ())?;

    let g_82f8 = exe::ptr::<u64>(0x1_43a0_82f8) as usize;
    let g_ff48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let g_0190 = exe::ptr::<usize>(0x1_43a0_0190) as usize;
    let s_82f8 = rd::<u64>(g_82f8);
    let s_ff48 = rd::<usize>(g_ff48);
    let s_0190 = rd::<usize>(g_0190);

    let run = |f: F| -> (u64, u64, [u8; 4], [u64; 3], [u8; 4], u64, [u8; 0x68], [u8; 2]) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };
        wr::<u64>(g_82f8, 0x7788_99aa);

        let mut vec_buf2 = vec![0x5au8; 0x320 * 3];
        let mut vec_buf3 = vec![0xa5u8; 0x320 * 3];

        let mut item0 = vec![0u8; 0x440];
        let mut item1 = vec![0u8; 0x440];
        let mut item2 = vec![0u8; 0x440];
        let mut item3 = vec![0u8; 0x440];

        let i0 = item0.as_mut_ptr() as usize;
        wr::<u8>(i0, 2);
        wr::<u64>(i0 + 0x28, 50);

        let i1 = item1.as_mut_ptr() as usize;
        wr::<u8>(i1, 3);
        wr::<u32>(i1 + 0x04, 0);
        wr::<u8>(i1 + 0x80, 2);
        wr::<usize>(i1 + 0xe8, 0x1111_0000);
        wr::<u32>(i1 + 0xf0, 4);

        let i2 = item2.as_mut_ptr() as usize;
        wr::<u8>(i2, 3);
        wr::<u32>(i2 + 0x04, 1);
        wr::<u8>(i2 + 0x80, 3);
        wr::<usize>(i2 + 0xe8, vec_buf2.as_mut_ptr() as usize);
        wr::<u32>(i2 + 0xf0, 1);
        wr::<u32>(i2 + 0xf4, 1);

        let i3 = item3.as_mut_ptr() as usize;
        wr::<u8>(i3, 3);
        wr::<u32>(i3 + 0x04, 2);
        wr::<u8>(i3 + 0x80, 2);
        wr::<usize>(i3 + 0xe8, vec_buf3.as_mut_ptr() as usize);
        wr::<u32>(i3 + 0xf0, 2);
        wr::<u32>(i3 + 0xf4, 3);

        let items: [usize; 4] = [i0, i1, i2, i3];
        let p0_arr: [u64; 3] = [0xaa00, 0xbb00, 0xcc00];
        let mut arr_20: [u8; 4] = [5, 1, 1, 0];
        let mut arr_40: [u64; 3] = [99, 88, 77];

        let mut p1_buf = vec![0u8; 0x100];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1, p0_arr.as_ptr() as usize);
        wr::<usize>(p1 + 0x20, arr_20.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0x40, arr_40.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0x80, items.as_ptr() as usize);
        wr::<u32>(p1 + 0x88, 4);
        wr::<u64>(p1 + 0xe0, 100);

        // Set up rbp_obj (0x14397ff48) and g_0190 (0x143a00190)
        let mut r10_info0 = [0u8; 0x18];
        wr::<u64>(r10_info0.as_mut_ptr() as usize, 0x1122_3344_5566_7788);
        wr::<u64>(r10_info0.as_mut_ptr() as usize + 0x08, 2);
        wr::<u32>(r10_info0.as_mut_ptr() as usize + 0x10, 0x99aa_bbcc);

        let mut sub_elem0 = [0u8; 0xb0];
        wr::<usize>(sub_elem0.as_mut_ptr() as usize + 0x88, r10_info0.as_ptr() as usize);
        wr::<i32>(sub_elem0.as_mut_ptr() as usize + 0xa8, 3);
        let sub_elems: [usize; 1] = [sub_elem0.as_ptr() as usize];

        let mut rdx_sub = [0u8; 0x140];
        wr::<usize>(rdx_sub.as_mut_ptr() as usize + 0x130, sub_elems.as_ptr() as usize);
        wr::<i32>(rdx_sub.as_mut_ptr() as usize + 0x138, 1);

        let slot_map0: [u32; 2] = [0, 0];
        let slot_map1: [u32; 2] = [0, 1];

        let mut item_18_0 = [0u8; 0x30];
        wr::<usize>(item_18_0.as_mut_ptr() as usize + 0x08, rdx_sub.as_ptr() as usize);
        wr::<usize>(item_18_0.as_mut_ptr() as usize + 0x20, slot_map0.as_ptr() as usize);

        let mut item_18_1 = [0u8; 0x30];
        wr::<usize>(item_18_1.as_mut_ptr() as usize + 0x08, rdx_sub.as_ptr() as usize);
        wr::<usize>(item_18_1.as_mut_ptr() as usize + 0x20, slot_map1.as_ptr() as usize);

        let arr_18: [usize; 2] = [item_18_0.as_ptr() as usize, item_18_1.as_ptr() as usize];
        let arr_198: [u32; 2] = [0, 1];
        let arr_2b0: [u32; 2] = [0, 1];
        let mut arr_240 = [0u8; 0x48];
        arr_240[2] = 5; // entry 0: b_cur = 5, b_new = arr_20[0] = 2 -> b_new < b_cur
        arr_240[0x24 + 2] = 1; // entry 1: b_cur = 1, b_new = arr_20[1] = 3 -> b_new > b_cur

        let mut rbp_obj = [0u8; 0x2c0];
        let rp = rbp_obj.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(rp + 0x20, 2);
        wr::<usize>(rp + 0x198, arr_198.as_ptr() as usize);
        wr::<usize>(rp + 0x240, arr_240.as_mut_ptr() as usize);
        wr::<u32>(rp + 0x248, 2);
        wr::<usize>(rp + 0x2b0, arr_2b0.as_ptr() as usize);

        let mut mask_1a0: u64 = 0;
        let mut data_190 = [0xffu8; 0x68];
        let mut obj_0190 = [0u8; 0x1b0];
        wr::<usize>(obj_0190.as_mut_ptr() as usize + 0x190, data_190.as_mut_ptr() as usize);
        wr::<usize>(obj_0190.as_mut_ptr() as usize + 0x1a0, (&mut mask_1a0 as *mut u64) as usize);

        wr::<usize>(g_ff48, rp);
        wr::<usize>(g_0190, obj_0190.as_mut_ptr() as usize);

        f(p1);

        let states = [rd::<u8>(i0), rd::<u8>(i1), rd::<u8>(i2), rd::<u8>(i3)];
        let post_240 = [arr_240[2], arr_240[0x24 + 2]];
        (
            unsafe { TRACE_HASH },
            rd::<u64>(p1 + 0xe8),
            arr_20,
            arr_40,
            states,
            mask_1a0,
            data_190,
            post_240,
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<u64>(g_82f8, s_82f8);
    wr::<usize>(g_ff48, s_ff48);
    wr::<usize>(g_0190, s_0190);

    crate::diff_eq!(o, n, "14230b890 out");
    Ok(1)
}

pub unsafe fn diff_rend_142321130(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH_SLOTS: [u32; 2] = [0, 1];
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_v278_38(_this: usize) -> u64 {
        10
    }
    unsafe extern "win64" fn mock_v278_18(_this: usize) {
        unsafe { rec(0x27818) };
    }
    unsafe extern "win64" fn mock_v348_18(_this: usize) {
        unsafe { rec(0x34818) };
    }
    unsafe extern "win64" fn mock_v418_38(_this: usize) -> u64 {
        1
    }
    unsafe extern "win64" fn mock_v418_20(_this: usize, new_sz: u64, fill_ptr: usize) {
        unsafe { rec(0x41820 ^ new_sz ^ (rd::<u32>(fill_ptr) as u64)) };
    }
    unsafe extern "win64" fn mock_alloc(_sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(SCRATCH_SLOTS) as usize
    }
    unsafe extern "win64" fn mock_e750(_p1: usize, _p2: usize, _p3: u64, out: usize) {
        wr::<u32>(out, 0);
        wr::<u32>(out + 4, 1);
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        if ptr == core::ptr::addr_of!(SCRATCH_SLOTS) as usize {
            unsafe { rec(0xc190) };
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4231_e750, mock_e750 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let g_0180 = exe::ptr::<i32>(0x1_43a0_0180) as usize;
    let s_0180 = rd::<i32>(g_0180);
    wr::<i32>(g_0180, 42);

    let run = |f: F| -> (u64, i64, u32, u32, [u32; 4], u64, [u32; 2], u64, [u32; 2]) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut vtbl_278 = [0usize; 8];
        vtbl_278[3] = mock_v278_18 as *const () as usize;
        vtbl_278[7] = mock_v278_38 as *const () as usize;

        let mut vtbl_348 = [0usize; 8];
        vtbl_348[3] = mock_v348_18 as *const () as usize;

        let mut vtbl_418 = [0usize; 8];
        vtbl_418[4] = mock_v418_20 as *const () as usize;
        vtbl_418[7] = mock_v418_38 as *const () as usize;

        let mut sub_target0 = [0u8; 0x50];
        let mut sub_target1 = [0u8; 0x50];
        let mut arr_18 = [0u8; 0x70];
        wr::<usize>(arr_18.as_mut_ptr() as usize, sub_target0.as_mut_ptr() as usize);
        wr::<u32>(arr_18.as_mut_ptr() as usize + 0x08, 100);
        wr::<usize>(arr_18.as_mut_ptr() as usize + 0x38, sub_target1.as_mut_ptr() as usize);
        wr::<u32>(arr_18.as_mut_ptr() as usize + 0x40, 200);

        let mut buf_3e0 = [0u32; 4];
        let mut mask_3f0: u64 = 0;
        let mut buf_4b0 = [0u32; 2];
        let mut mask_4c0: u64 = 0;

        let mut p1_buf = vec![0u8; 0x500];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x18, arr_18.as_mut_ptr() as usize);
        wr::<u32>(p1 + 0x250, 4);
        wr::<u32>(p1 + 0x260, 6);
        wr::<usize>(p1 + 0x278, vtbl_278.as_ptr() as usize);
        wr::<usize>(p1 + 0x348, vtbl_348.as_ptr() as usize);
        wr::<usize>(p1 + 0x3e0, buf_3e0.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0x3f0, (&mut mask_3f0 as *mut u64) as usize);
        wr::<usize>(p1 + 0x418, vtbl_418.as_ptr() as usize);
        wr::<usize>(p1 + 0x4b0, buf_4b0.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0x4c0, (&mut mask_4c0 as *mut u64) as usize);
        wr::<i64>(p1 + 0x4e8, 10);

        let subs0: [u32; 2] = [1, 0x8000_0005];
        let subs1: [u32; 2] = [0, 1];
        let mut p2_arr = [0u8; 0x20];
        wr::<u32>(p2_arr.as_mut_ptr() as usize, 5);
        wr::<usize>(p2_arr.as_mut_ptr() as usize + 0x08, subs0.as_ptr() as usize);
        wr::<u32>(p2_arr.as_mut_ptr() as usize + 0x10, 35);
        wr::<usize>(p2_arr.as_mut_ptr() as usize + 0x18, subs1.as_ptr() as usize);

        f(p1, p2_arr.as_ptr() as usize, 2, 2);

        let cnts = [
            rd::<u32>(sub_target0.as_ptr() as usize + 0x28),
            rd::<u32>(sub_target1.as_ptr() as usize + 0x28),
        ];
        (
            unsafe { TRACE_HASH },
            rd::<i64>(p1 + 0x4f0),
            rd::<u32>(p1 + 0x250),
            rd::<u32>(p1 + 0x260),
            buf_3e0,
            mask_3f0,
            buf_4b0,
            mask_4c0,
            cnts,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<i32>(g_0180, s_0180);

    crate::diff_eq!(o, n, "142321130 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14232fb50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32, usize, u64, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FIND_HIT: bool = false;
    static mut FIND_SLOT_PTR: usize = 0;
    static mut INS_SLOT_PTR: usize = 0;
    static mut VEC_DATA_PTR: usize = 0;
    static mut AC70_FAIL: bool = false;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_ca40(_rcx: u64, out_pair: usize, key_ptr: usize) -> usize {
        let k = rd::<u64>(key_ptr);
        unsafe { rec(0xca40 ^ k) };
        if unsafe { FIND_HIT } {
            wr::<u64>(out_pair, 0x1111);
            wr::<usize>(out_pair + 0x08, unsafe { FIND_SLOT_PTR });
        } else {
            let end_ctrl = rd::<u64>(exe::ptr::<u64>(0x1_4386_c5d0) as usize)
                .wrapping_add(rd::<u64>(exe::ptr::<u64>(0x1_4386_c5b8) as usize));
            wr::<u64>(out_pair, end_ctrl);
            wr::<usize>(out_pair + 0x08, 0);
        }
        out_pair
    }
    unsafe extern "win64" fn mock_def0(_rcx: u64, out_triple: usize, key_ptr: usize) -> usize {
        unsafe { rec(0xdef0 ^ rd::<u64>(key_ptr)) };
        wr::<u64>(out_triple, 0x2222);
        wr::<usize>(out_triple + 0x08, unsafe { INS_SLOT_PTR });
        wr::<u8>(out_triple + 0x10, 1);
        out_triple
    }
    unsafe extern "win64" fn mock_eb40(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, unsafe { VEC_DATA_PTR });
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        unsafe { rec(0xeb40 ^ new_cap) };
    }
    unsafe extern "win64" fn mock_9b50(rstr_p: usize, view_p: usize, _flag: u32) {
        let s_ptr = rd::<usize>(view_p);
        let s_len = rd::<u64>(view_p + 0x08) as u32;
        wr::<u32>(rstr_p, 0x1000_0000 | s_len);
        wr::<u32>(rstr_p + 0x04, s_len);
        for i in 0..(s_len as usize).min(15) {
            wr::<u8>(rstr_p + 0x08 + i, rd::<u8>(s_ptr + i));
        }
        wr::<u8>(rstr_p + 0x08 + (s_len as usize).min(15), 0);
    }
    unsafe extern "win64" fn mock_74f0(_lhs: usize, _rhs: usize) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_ac70(
        _p1: usize,
        _r8_c8: usize,
        _p7: usize,
        view_p: usize,
        lod_a8: u32,
    ) -> u32 {
        let len = rd::<u64>(view_p + 0x08) as u32;
        let first_ch = rd::<u8>(rd::<usize>(view_p)) as u32;
        unsafe { rec(0xac70 ^ (len as u64) ^ (first_ch as u64) ^ (lod_a8 as u64)) };
        if unsafe { AC70_FAIL } {
            0x4000_0005
        } else {
            0x100 + len
        }
    }
    unsafe extern "win64" fn mock_e830(_vec_ptr: usize) {
        unsafe { rec(0xe830) };
    }
    unsafe extern "win64" fn mock_ab10(_rstr_p: usize) {
        unsafe { rec(0xab10) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4133_ca40, mock_ca40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4133_def0, mock_def0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_eb40, mock_eb40 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4021_74f0, mock_74f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4232_ac70, mock_ac70 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, mode: u32| -> (usize, u64, [u32; 2], u32) {
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
            FIND_HIT = mode == 0;
            AC70_FAIL = mode == 2;
        }

        let mut vec_storage: [u32; 2] = [0, 0];
        let mut ins_slot = [0u64; 3];
        let mut find_slot: [u64; 2] = [0, 0x7777_8888];
        unsafe {
            VEC_DATA_PTR = vec_storage.as_mut_ptr() as usize;
            INS_SLOT_PTR = ins_slot.as_mut_ptr() as usize;
            FIND_SLOT_PTR = find_slot.as_mut_ptr() as usize;
        }

        // RB-tree with sentinel head and 1 valid child node
        let mut head_node = [0u8; 0x40];
        let mut child_node = [0u8; 0x40];
        let hp = head_node.as_mut_ptr() as usize;
        let cp = child_node.as_mut_ptr() as usize;
        wr::<u8>(hp + 0x19, 1);
        wr::<usize>(hp + 0x08, cp);
        wr::<usize>(cp, hp);
        wr::<usize>(cp + 0x10, hp);
        wr::<u8>(cp + 0x19, 0);

        let mut rdi_70 = [0u8; 0x60];
        wr::<usize>(rdi_70.as_mut_ptr() as usize + 0x50, hp);

        let mut p7_buf = [0u8; 0xe0];
        wr::<usize>(p7_buf.as_mut_ptr() as usize + 0x70, rdi_70.as_mut_ptr() as usize);
        wr::<u64>(p7_buf.as_mut_ptr() as usize + 0xd0, 0x1234_5678);

        let mut r8_c8 = [0u8; 0x40];
        wr::<usize>(r8_c8.as_mut_ptr() as usize + 0x18, p7_buf.as_mut_ptr() as usize);
        wr::<u64>(r8_c8.as_mut_ptr() as usize + 0x28, 0xaaaa_1111);
        wr::<u64>(r8_c8.as_mut_ptr() as usize + 0x30, 0xbbbb_2222);

        let mut entry_buf = [0u8; 0xd0];
        wr::<usize>(entry_buf.as_mut_ptr() as usize + 0xc8, r8_c8.as_mut_ptr() as usize);
        let tbl_160: [usize; 2] = [0, entry_buf.as_mut_ptr() as usize];

        let mut elem_130 = [0u8; 0xb0];
        wr::<i32>(elem_130.as_mut_ptr() as usize + 0xa8, 2);
        let tbl_130: [usize; 2] = [0, elem_130.as_mut_ptr() as usize];

        let mut p1_buf = vec![0u8; 0x280];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x130, tbl_130.as_ptr() as usize);
        wr::<usize>(p1 + 0x160, tbl_160.as_ptr() as usize);
        wr::<i32>(p1 + 0x1c8 + 4, 1);
        wr::<u8>(p1 + 0x272, 2);

        let cstr = b"abc\0";
        let str_ptrs: [usize; 2] = [0, cstr.as_ptr() as usize];

        let ret = f(p1, 1, 0, str_ptrs.as_ptr() as usize, 2, 0x9999, 0);
        let norm_ret = if ret == unsafe { VEC_DATA_PTR } {
            1
        } else {
            ret
        };
        let out_len = rd::<u32>((unsafe { INS_SLOT_PTR }) + 0x10);
        (norm_ret, unsafe { TRACE_HASH }, vec_storage, out_len)
    };

    for mode in 0..3 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14232fb50 mode={mode}");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1423da290(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_9ad0(arg: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ arg).wrapping_mul(0x1000_0000_01b3);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4229_9ad0, mock_9ad0 as *const ())?;

    let tbl_ptr = exe::ptr::<u8>(0x1_43bd_6048) as *mut u8;
    let snap = crate::difftest::Snapshot::take(tbl_ptr, 32);
    let base = tbl_ptr as usize;
    wr::<u64>(base, 0x1111_2222);
    wr::<u64>(base + 0x08, 0x3333_4444);
    wr::<u64>(base + 0x10, 0x5555_6666);
    wr::<u64>(base + 0x18, 0x7777_8888);

    let indices: [u32; 3] = [2, 0, 3];
    let run = |f: F| -> u64 {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };
        f(indices.as_ptr() as usize, 0);
        f(indices.as_ptr() as usize, 3);
        unsafe { TRACE_HASH }
    };

    let o = run(f_o);
    let n = run(f_n);
    snap.restore();

    crate::diff_eq!(o, n, "1423da290 out");
    Ok(2)
}

pub unsafe fn diff_r_142422da0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut QPC_STEP: i64 = 0;
    static mut DF90_CALLS: u32 = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_vfunc_18(_this: usize) -> u64 {
        unsafe { rec(0x0018) };
        512
    }
    unsafe extern "win64" fn mock_vfunc_08(_this: usize, pkt_ptr: usize) {
        let p0 = rd::<u64>(pkt_ptr);
        let p1 = rd::<u64>(pkt_ptr + 0x08);
        let p2 = rd::<u64>(pkt_ptr + 0x10);
        unsafe { rec(0x0008 ^ p0 ^ p1 ^ p2) };
    }
    unsafe extern "win64" fn mock_qpc(out_ptr: usize) -> i32 {
        let v = unsafe {
            QPC_STEP += 100;
            QPC_STEP
        };
        wr::<i64>(out_ptr, v);
        1
    }
    unsafe extern "win64" fn mock_freq() -> i64 {
        1000
    }
    unsafe extern "win64" fn mock_cf50(item: usize) {
        unsafe { rec(0xcf50 ^ (rd::<i32>(item + 0x150) as u64)) };
    }
    unsafe extern "win64" fn mock_df90(item: usize) -> u8 {
        unsafe {
            rec(0xdf90 ^ (rd::<i32>(item + 0x68) as u64));
            DF90_CALLS += 1;
            if DF90_CALLS == 1 { 1 } else { 0 }
        }
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_56f8, mock_qpc as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4240_6920, mock_freq as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_cf50, mock_cf50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_df90, mock_df90 as *const ())?;

    let g_1438 = exe::ptr::<i32>(0x1_4387_1438) as usize;
    let g_1360 = exe::ptr::<u8>(0x1_4387_1360) as usize;
    let g_13f8 = exe::ptr::<f32>(0x1_4387_13f8) as usize;
    let g_1398 = exe::ptr::<f32>(0x1_4387_1398) as usize;
    let g_1378 = exe::ptr::<f32>(0x1_4387_1378) as usize;

    let s_1438 = rd::<i32>(g_1438);
    let s_1360 = rd::<u8>(g_1360);
    let s_13f8 = rd::<f32>(g_13f8);
    let s_1398 = rd::<f32>(g_1398);
    let s_1378 = rd::<f32>(g_1378);

    wr::<i32>(g_1438, 1); // 1 << 20 > 512 -> stack_flags[1] = 1
    wr::<f32>(g_13f8, 500.0);
    wr::<f32>(g_1398, 50.0);
    wr::<f32>(g_1378, 20.0);

    let run = |f: F, mode_1360: u8| -> u64 {
        unsafe {
            QPC_STEP = 1000;
            DF90_CALLS = 0;
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }
        wr::<u8>(g_1360, mode_1360);

        let mut vtbl = [0usize; 4];
        vtbl[1] = mock_vfunc_08 as *const () as usize;
        vtbl[3] = mock_vfunc_18 as *const () as usize;

        // item0: idx_150 = 0 (stack_flags[0]=0, stack_flags[8]=1 -> enters df90 check)
        let mut item0 = vec![0u8; 0x160];
        wr::<i32>(item0.as_mut_ptr() as usize + 0x28, 10);
        wr::<i32>(item0.as_mut_ptr() as usize + 0x68, 60);
        wr::<i32>(item0.as_mut_ptr() as usize + 0xd0, 100);
        wr::<i32>(item0.as_mut_ptr() as usize + 0x150, 0);

        // item1: idx_150 = 2 (stack_flags[2]=1, stack_flags[10]=0 -> enters cf50 check)
        let mut item1 = vec![0u8; 0x160];
        wr::<i32>(item1.as_mut_ptr() as usize + 0x150, 2);

        let items: [usize; 2] = [item0.as_mut_ptr() as usize, item1.as_mut_ptr() as usize];

        let mut p1_buf = vec![0u8; 0x230];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1, vtbl.as_ptr() as usize);
        wr::<usize>(p1 + 0x90, items.as_ptr() as usize);
        wr::<u32>(p1 + 0x98, 2);
        wr::<u8>(p1 + 0x221, 1);

        f(p1);
        unsafe { TRACE_HASH }
    };

    for m in [0u8, 1u8] {
        let o = run(f_o, m);
        let n = run(f_n, m);
        crate::diff_eq!(o, n, "142422da0 mode={m}");
    }

    wr::<i32>(g_1438, s_1438);
    wr::<u8>(g_1360, s_1360);
    wr::<f32>(g_13f8, s_13f8);
    wr::<f32>(g_1398, s_1398);
    wr::<f32>(g_1378, s_1378);

    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4230_b890, "unknown_14230b890", rend::cauldron_hot_n2::unknown_14230b890 as *const (), Throws::Caught, rend::cauldron_hot_n2::diff_unknown_14230b890),
// Replacement::verified_throws(0x1_4232_1130, "rend_142321130", rend::cauldron_hot_n2::rend_142321130 as *const (), Throws::Discard, rend::cauldron_hot_n2::diff_rend_142321130),
// Replacement::verified_throws(0x1_4232_fb50, "unknown_14232fb50", rend::cauldron_hot_n2::unknown_14232fb50 as *const (), Throws::Caught, rend::cauldron_hot_n2::diff_unknown_14232fb50),
// Replacement::verified_throws(0x1_423d_a290, "unknown_1423da290", rend::cauldron_hot_n2::unknown_1423da290 as *const (), Throws::Discard, rend::cauldron_hot_n2::diff_unknown_1423da290),
// Replacement::verified_throws(0x1_4242_2da0, "r_142422da0", rend::cauldron_hot_n2::r_142422da0 as *const (), Throws::Discard, rend::cauldron_hot_n2::diff_r_142422da0),
