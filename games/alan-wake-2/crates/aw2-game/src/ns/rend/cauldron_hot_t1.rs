//! Decompiled hot functions shard T1 (`0x14222acb0..0x142320de0`).

use crate::exe::{self, rd, wr};

/// `unknown_14222acb0`
/// Original: `0x14222acb0` (1043 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_14222acb0(param_1: usize) {
    let g_ff48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let cnt_20 = rd::<u32>(g_ff48 + 0x20) as usize;
    let mask_180 = rd::<usize>(g_ff48 + 0x180);
    let words_20 = (cnt_20 + 0x3f) >> 6;
    if words_20 != 0 {
        let mut base_bit: usize = 0;
        for w in 0..words_20 {
            let mut bits = rd::<u64>(mask_180.wrapping_add(w * 8));
            while bits != 0 {
                let tz = bits.trailing_zeros() as usize;
                let idx = base_bit + tz;
                let arr_18 = rd::<usize>(g_ff48 + 0x18);
                let item = rd::<usize>(arr_18.wrapping_add(idx * 8));
                if item != 0 {
                    let slot = rd::<u32>(item + 0x50);
                    if slot != 0xffff_ffff {
                        let mask_170 = rd::<usize>(param_1 + 0x170);
                        let word_addr = mask_170.wrapping_add(((slot >> 6) as usize) * 8);
                        wr::<u64>(
                            word_addr,
                            rd::<u64>(word_addr) | (1u64 << (slot & 0x3f)),
                        );
                    }
                }
                bits ^= 1u64 << tz;
            }
            base_bit += 0x40;
        }
    }

    let cnt_128 = rd::<u32>(param_1 + 0x128) as usize;
    let mask_170 = rd::<usize>(param_1 + 0x170);
    let words_128 = (cnt_128 + 0x3f) >> 6;
    if words_128 != 0 {
        for w in 0..words_128 {
            let mut bits = rd::<u64>(mask_170.wrapping_add(w * 8));
            if bits != 0 {
                let base_bit = (w as u32) << 6;
                while bits != 0 {
                    let tz = bits.trailing_zeros();
                    let idx_u32 = base_bit.wrapping_add(tz);
                    let idx = idx_u32 as usize;
                    let elem = rd::<usize>(param_1 + 0x120).wrapping_add(idx * 0xa0);
                    let sub_90 = rd::<usize>(elem + 0x90);
                    if rd::<u32>(sub_90 + 0x34) == 0 {
                        let arr_e0 = rd::<usize>(param_1 + 0xe0);
                        let old_cnt = rd::<u32>(arr_e0.wrapping_add(idx * 4));
                        if old_cnt != 0 {
                            let arr_98 = rd::<usize>(param_1 + 0x98);
                            let handle = rd::<u32>(arr_98.wrapping_add(idx * 4));
                            exe::call_va!(
                                0x1_4222_9c90,
                                (g_ff48.wrapping_add(0x390), handle, old_cnt)
                            );
                            exe::call_va!(
                                0x1_4222_9c90,
                                (g_ff48.wrapping_add(0x438), handle, old_cnt)
                            );
                            wr::<u32>(arr_e0.wrapping_add(idx * 4), 0);
                            let arr_98_2 = rd::<usize>(param_1 + 0x98);
                            wr::<u32>(arr_98_2.wrapping_add(idx * 4), 0xffff_ffff);
                            let mask_a8 = rd::<usize>(param_1 + 0xa8);
                            let w_addr = mask_a8.wrapping_add(((idx_u32 >> 6) as usize) * 8);
                            wr::<u64>(
                                w_addr,
                                rd::<u64>(w_addr) | (1u64 << (idx_u32 & 0x3f)),
                            );
                        }
                    }
                    bits ^= 1u64 << tz;
                }
            }
        }

        for w in 0..words_128 {
            let mut bits = rd::<u64>(mask_170.wrapping_add(w * 8));
            if bits != 0 {
                let base_bit = (w as u32) << 6;
                while bits != 0 {
                    let tz = bits.trailing_zeros();
                    let idx_u32 = base_bit.wrapping_add(tz);
                    let idx = idx_u32 as usize;
                    let base_120 = rd::<usize>(param_1 + 0x120);
                    let elem = base_120.wrapping_add(idx * 0xa0);
                    let sub_90 = rd::<usize>(elem + 0x90);
                    if rd::<u32>(sub_90 + 0x34) == 0 {
                        let ptr_80 = rd::<usize>(elem + 0x80);
                        let off_4 = idx * 4;
                        let arr_e0 = rd::<usize>(param_1 + 0xe0);
                        let lod_idx_ptr = rd::<usize>(g_ff48 + 0x1a8)
                            .wrapping_add(rd::<u32>(ptr_80) as usize);
                        let b272 = rd::<u8>(sub_90 + 0x272);
                        if b272 == 0 {
                            wr::<u32>(arr_e0.wrapping_add(off_4), 0);
                        } else {
                            let req_lod = rd::<u8>(lod_idx_ptr) as i32;
                            let max_lod = (b272 as i32) - 1;
                            let sel_lod = if max_lod < req_lod { max_lod } else { req_lod };
                            let new_cnt = rd::<u32>(
                                sub_90
                                    .wrapping_add(0x1c8)
                                    .wrapping_add(((sel_lod as isize) * 4) as usize),
                            );
                            wr::<u32>(arr_e0.wrapping_add(off_4), new_cnt);
                            if new_cnt != 0 {
                                let h0 = exe::call_va!(
                                    0x1_412f_81b0,
                                    (g_ff48.wrapping_add(0x300), new_cnt) -> u32
                                );
                                let _ = exe::call_va!(
                                    0x1_412f_81b0,
                                    (g_ff48.wrapping_add(0x3a8), new_cnt) -> u32
                                );
                                let arr_98 = rd::<usize>(param_1 + 0x98);
                                wr::<u32>(arr_98.wrapping_add(off_4), h0);
                                let mask_a8 = rd::<usize>(param_1 + 0xa8);
                                let w_addr = mask_a8.wrapping_add(((idx_u32 >> 6) as usize) * 8);
                                wr::<u64>(
                                    w_addr,
                                    rd::<u64>(w_addr) | (1u64 << (idx_u32 & 0x3f)),
                                );
                                let arr_100 = rd::<usize>(param_1 + 0x100);
                                let slot_100 = rd::<u32>(arr_100.wrapping_add(off_4));
                                if slot_100 != 0xffff_ffff {
                                    let bit = slot_100 & 0x3f;
                                    let w_off = ((slot_100 >> 6) as usize) * 8;
                                    let m698 = rd::<usize>(g_ff48 + 0x698).wrapping_add(w_off);
                                    wr::<u64>(m698, rd::<u64>(m698) | (1u64 << bit));
                                    let m668 = rd::<usize>(g_ff48 + 0x668).wrapping_add(w_off);
                                    wr::<u64>(m668, rd::<u64>(m668) | (1u64 << bit));
                                    let m680 = rd::<usize>(g_ff48 + 0x680).wrapping_add(w_off);
                                    wr::<u64>(m680, rd::<u64>(m680) | (1u64 << bit));
                                }
                            }
                        }
                    }
                    bits ^= 1u64 << tz;
                }
            }
        }
    }

    let len_180 = rd::<u64>(param_1 + 0x180);
    let ptr_170 = rd::<usize>(param_1 + 0x170);
    let words_180 = len_180.wrapping_add(0x3f) >> 6;
    if len_180 != 0 {
        let full_words = (words_180 as usize).saturating_sub(1);
        for i in 0..full_words {
            wr::<u64>(ptr_170.wrapping_add(i * 8), 0);
        }
        let shift = (!(len_180.wrapping_sub(1) as u32)) & 0x3f;
        let mask = !(u64::MAX >> shift);
        let last_addr = ptr_170.wrapping_add((words_180 as usize) * 8).wrapping_sub(8);
        wr::<u64>(last_addr, rd::<u64>(last_addr) & mask);
    }
}

/// `std_14222f750`
/// Original: `0x14222f750` (113 bytes, `Throws::No`)
pub unsafe extern "win64" fn std_14222f750(param_1: usize, param_2: i32) -> u32 {
    let idx_98 = rd::<u32>(param_1 + 0x98) as usize;
    let g_fed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let arr_d0 = rd::<usize>(g_fed0 + 0xd0);
    let mut sum = rd::<u32>(arr_d0.wrapping_add(idx_98 * 4));
    let sub_90 = rd::<usize>(param_1 + 0x90);
    if rd::<u32>(sub_90 + 0x34) > 0
        && param_2 < (rd::<u8>(sub_90 + 0x270) as i32)
        && param_2 > 0
    {
        let b272 = rd::<u8>(sub_90 + 0x272);
        for i in 0..param_2 {
            let add_val = if b272 == 0 {
                0
            } else {
                let max_idx = (b272 as i32) - 1;
                let sel = if max_idx < i { max_idx } else { i };
                rd::<u32>(
                    sub_90
                        .wrapping_add(0x1c8)
                        .wrapping_add(((sel as isize) * 4) as usize),
                )
            };
            sum = sum.wrapping_add(add_val);
        }
    }
    sum
}

/// `r_14225c0f0`
/// Original: `0x14225c0f0` (1700 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_14225c0f0(param_1: usize, param_2: u8) {
    let g_fed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let ptr_6b0 = rd::<usize>(param_1 + 0x6b0);
    let ptr_698 = rd::<usize>(param_1 + 0x698);
    let g_ff60 = if param_2 != 0 {
        rd::<usize>(exe::ptr::<usize>(0x1_4397_ff60) as usize)
    } else {
        0
    };

    let words_6a8 = (rd::<u64>(param_1 + 0x6a8).wrapping_add(0x3f) >> 6) as usize;
    if words_6a8 != 0 {
        let mut base_bit: usize = 0;
        for w in 0..words_6a8 {
            let mut bits =
                rd::<u64>(ptr_698.wrapping_add(w * 8)) ^ rd::<u64>(ptr_6b0.wrapping_add(w * 8));
            while bits != 0 {
                let tz = bits.trailing_zeros() as usize;
                let arr_130 = rd::<usize>(param_1 + 0x130);
                let elem_idx = rd::<u32>(arr_130.wrapping_add((base_bit + tz) * 8)) as usize;
                let elem_ptr = rd::<usize>(g_fed0 + 0x120).wrapping_add(elem_idx * 0xa0);
                exe::call_va!(0x1_4222_f0f0, (elem_ptr));
                bits ^= 1u64 << tz;
            }
            base_bit += 0x40;
        }
    }

    let mut g_ff48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    if param_2 != 0 {
        let words_678 = (rd::<u64>(param_1 + 0x678).wrapping_add(0x3f) >> 6) as usize;
        let ptr_668 = rd::<usize>(param_1 + 0x668);
        if words_678 != 0 {
            let mut base_bit: usize = 0;
            for w in 0..words_678 {
                let mut bits = rd::<u64>(ptr_668.wrapping_add(w * 8));
                while bits != 0 {
                    let tz = bits.trailing_zeros() as usize;
                    let arr_130 = rd::<usize>(param_1 + 0x130);
                    let elem_idx = rd::<u32>(arr_130.wrapping_add((base_bit + tz) * 8)) as usize;
                    let elem_ptr = rd::<usize>(g_fed0 + 0x120).wrapping_add(elem_idx * 0xa0);
                    let mut cur_18 = rd::<usize>(elem_ptr + 0x18);
                    let cnt_20 = rd::<u32>(elem_ptr + 0x20) as usize;
                    let end_18 = cur_18.wrapping_add(cnt_20 * 4);
                    while cur_18 != end_18 {
                        let sub_id = rd::<u32>(cur_18);
                        if sub_id != 0xffff_ffff {
                            let sub_ptr = rd::<usize>(g_fed0 + 0x148)
                                .wrapping_add((sub_id as usize) * 0x50);
                            let cnt_8 = rd::<u16>(sub_ptr + 0x08) as usize;
                            if cnt_8 != 0 {
                                let mut bit_id = rd::<u32>(sub_ptr + 0x48) as u64;
                                for _ in 0..cnt_8 {
                                    let mask_210 = rd::<usize>(g_ff60 + 0x210);
                                    let b = (bit_id as u32) & 0x3f;
                                    let w_addr = mask_210.wrapping_add(((bit_id >> 6) as usize) * 8);
                                    bit_id = bit_id.wrapping_add(1);
                                    wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << b));
                                }
                            }
                            let mut cur_30 = rd::<usize>(sub_ptr + 0x30);
                            let cnt_38 = rd::<u32>(sub_ptr + 0x38) as usize;
                            let end_30 = cur_30.wrapping_add(cnt_38 * 4);
                            while cur_30 != end_30 {
                                let grp_id = rd::<u32>(cur_30);
                                if grp_id != 0xffff_ffff {
                                    let mut off_170 = (grp_id as usize) << 6;
                                    for _ in 0..16 {
                                        let tbl_170 = rd::<usize>(g_ff48 + 0x170);
                                        let v = rd::<u32>(tbl_170.wrapping_add(off_170));
                                        if v == 0xffff_ffff {
                                            break;
                                        }
                                        let mask_1f8 = rd::<usize>(g_ff60 + 0x1f8);
                                        let w_addr =
                                            mask_1f8.wrapping_add(((v >> 6) as usize) * 8);
                                        off_170 += 4;
                                        wr::<u64>(
                                            w_addr,
                                            rd::<u64>(w_addr) | (1u64 << (v & 0x3f)),
                                        );
                                    }
                                }
                                cur_30 = cur_30.wrapping_add(4);
                            }
                        }
                        cur_18 = cur_18.wrapping_add(4);
                    }
                    bits ^= 1u64 << tz;
                }
                base_bit += 0x40;
            }
        }
    }

    let words_20 = ((rd::<u32>(param_1 + 0x20) as u64).wrapping_add(0x3f) >> 6) as usize;
    let ptr_180 = rd::<usize>(param_1 + 0x180);
    if words_20 != 0 {
        let mut base_bit: usize = 0;
        for w in 0..words_20 {
            let mut bits = rd::<u64>(ptr_180.wrapping_add(w * 8));
            while bits != 0 {
                let tz = bits.trailing_zeros() as usize;
                let idx = base_bit + tz;
                let arr_18 = rd::<usize>(param_1 + 0x18);
                let item = rd::<usize>(arr_18.wrapping_add(idx * 8));
                if item != 0 {
                    let slot_50 = rd::<u32>(item + 0x50);
                    if slot_50 == 0xffff_ffff {
                        if param_2 != 0 {
                            let mut off_160 = (rd::<u32>(item + 0x48) as usize) << 6;
                            for _ in 0..16 {
                                let tbl_160 = rd::<usize>(g_ff48 + 0x160);
                                let v = rd::<u32>(tbl_160.wrapping_add(off_160));
                                if v == 0xffff_ffff {
                                    break;
                                }
                                let mask_228 = rd::<usize>(g_ff60 + 0x228);
                                let w_addr = mask_228.wrapping_add(((v >> 6) as usize) * 8);
                                off_160 += 4;
                                wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (v & 0x3f)));
                            }
                        }
                    } else {
                        let elem_ptr = rd::<usize>(g_fed0 + 0x120)
                            .wrapping_add((slot_50 as usize) * 0xa0);
                        let req_lod =
                            rd::<u8>(rd::<usize>(param_1 + 0x1a8).wrapping_add(idx)) as u32;
                        let sub_90 = rd::<usize>(elem_ptr + 0x90);
                        let b272 = rd::<u8>(sub_90 + 0x272);
                        let out_val: u32 = if b272 == 0 {
                            0xffff_ffff
                        } else {
                            let max_lod = (b272 as i32) - 1;
                            let sel_lod = if max_lod < (req_lod as i32) {
                                max_lod
                            } else {
                                req_lod as i32
                            };
                            if rd::<u32>(
                                sub_90
                                    .wrapping_add(0x1c8)
                                    .wrapping_add(((sel_lod as isize) * 4) as usize),
                            ) == 0
                            {
                                0xffff_ffff
                            } else {
                                let mut acc: u32 = 0;
                                if rd::<u32>(sub_90 + 0x34) > 0
                                    && req_lod < (rd::<u8>(sub_90 + 0x270) as u32)
                                    && req_lod != 0
                                {
                                    for i in 0..(req_lod as i32) {
                                        let s = if max_lod < i { max_lod } else { i };
                                        acc = acc.wrapping_add(rd::<u32>(
                                            sub_90
                                                .wrapping_add(0x1c8)
                                                .wrapping_add(((s as isize) * 4) as usize),
                                        ));
                                    }
                                }
                                acc
                            }
                        };
                        let arr_28 = rd::<usize>(param_1 + 0x28);
                        wr::<u32>(arr_28.wrapping_add(idx * 12 + 4), out_val);
                        exe::call_va!(0x1_4222_ef60, (elem_ptr));
                        exe::call_va!(0x1_4222_f280, (elem_ptr));
                        g_ff48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
                        if param_2 != 0 {
                            let idx_5c = rd::<u32>(item + 0x5c) as usize;
                            let sub_5c =
                                rd::<usize>(g_fed0 + 0x148).wrapping_add(idx_5c * 0x50);
                            let cnt_8 = rd::<u16>(sub_5c + 0x08) as usize;
                            if cnt_8 != 0 {
                                let mut bit_id = rd::<u32>(sub_5c + 0x48) as u64;
                                for _ in 0..cnt_8 {
                                    let mask_210 = rd::<usize>(g_ff60 + 0x210);
                                    let b = (bit_id as u32) & 0x3f;
                                    let w_addr =
                                        mask_210.wrapping_add(((bit_id >> 6) as usize) * 8);
                                    bit_id = bit_id.wrapping_add(1);
                                    wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << b));
                                }
                            }
                            let mut off_170 = (rd::<u32>(item + 0x48) as usize) << 6;
                            for _ in 0..16 {
                                let tbl_170 = rd::<usize>(g_ff48 + 0x170);
                                let v = rd::<u32>(tbl_170.wrapping_add(off_170));
                                if v == 0xffff_ffff {
                                    break;
                                }
                                let mask_1f8 = rd::<usize>(g_ff60 + 0x1f8);
                                let w_addr = mask_1f8.wrapping_add(((v >> 6) as usize) * 8);
                                off_170 += 4;
                                wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (v & 0x3f)));
                            }
                        }
                        let slot_50_2 = rd::<u32>(item + 0x50) as usize;
                        let arr_100 = rd::<usize>(g_fed0 + 0x100);
                        let mapped = rd::<u32>(arr_100.wrapping_add(slot_50_2 * 4));
                        if mapped != 0xffff_ffff {
                            let mask_698 = rd::<usize>(param_1 + 0x698);
                            let w_addr = mask_698.wrapping_add(((mapped >> 6) as usize) * 8);
                            wr::<u64>(
                                w_addr,
                                rd::<u64>(w_addr) | (1u64 << (mapped & 0x3f)),
                            );
                        }
                    }
                }
                bits ^= 1u64 << tz;
            }
            base_bit += 0x40;
        }
    }

    if param_2 != 0 {
        let words_648 = (rd::<u64>(param_1 + 0x648).wrapping_add(0x3f) >> 6) as usize;
        let ptr_638 = rd::<usize>(param_1 + 0x638);
        if words_648 != 0 {
            let mut base_bit: usize = 0;
            for w in 0..words_648 {
                let mut bits = rd::<u64>(ptr_638.wrapping_add(w * 8));
                while bits != 0 {
                    let tz = bits.trailing_zeros() as usize;
                    let idx = base_bit + tz;
                    let arr_18 = rd::<usize>(param_1 + 0x18);
                    let item = rd::<usize>(arr_18.wrapping_add(idx * 8));
                    if item != 0 {
                        let mut off = (rd::<u32>(item + 0x48) as usize) << 6;
                        if rd::<u32>(item + 0x50) == 0xffff_ffff {
                            for _ in 0..16 {
                                let tbl_160 = rd::<usize>(g_ff48 + 0x160);
                                let v = rd::<u32>(tbl_160.wrapping_add(off));
                                if v == 0xffff_ffff {
                                    break;
                                }
                                let mask_228 = rd::<usize>(g_ff60 + 0x228);
                                let w_addr = mask_228.wrapping_add(((v >> 6) as usize) * 8);
                                off += 4;
                                wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (v & 0x3f)));
                            }
                        } else {
                            for _ in 0..16 {
                                let tbl_170 = rd::<usize>(g_ff48 + 0x170);
                                let v = rd::<u32>(tbl_170.wrapping_add(off));
                                if v == 0xffff_ffff {
                                    break;
                                }
                                let mask_1f8 = rd::<usize>(g_ff60 + 0x1f8);
                                let w_addr = mask_1f8.wrapping_add(((v >> 6) as usize) * 8);
                                off += 4;
                                wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (v & 0x3f)));
                            }
                        }
                    }
                    bits ^= 1u64 << tz;
                }
                base_bit += 0x40;
            }
        }
    }
}

/// `rend_1422f58c0`
/// Original: `0x1422f58c0` (858 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn rend_1422f58c0(param_1: usize, param_2: usize) -> u32 {
    let slot_c0 = param_1.wrapping_add(0xc0);
    let obj_c0 = exe::call_va!(0x1_40f7_5820, () -> usize);
    wr::<usize>(slot_c0, obj_c0);

    let mut frame_buf = [0u64; 0x380 / 8];
    let fstream_base = frame_buf.as_mut_ptr() as usize;
    let base = fstream_base.wrapping_add(0x10);
    let vft_filestream = exe::ptr::<u8>(0x1_4340_e9e0) as usize;
    let vft_bufstream = exe::ptr::<u8>(0x1_4307_b558) as usize;

    wr::<usize>(fstream_base, vft_filestream);
    wr::<usize>(fstream_base + 0x08, param_2);

    let _ = exe::call_va!(0x1_4183_acc2, (base, 0u32, 0x310u64) -> usize);
    wr::<usize>(base, base + 0x10);
    wr::<u32>(base + 0x0c, 0x10);
    wr::<u32>(base + 0x318, 0);
    wr::<usize>(base + 0x320, fstream_base);
    wr::<u32>(base + 0x328, 0xffff_ffff);
    wr::<u64>(base + 0x330, 0);
    wr::<u64>(base + 0x310, 0);
    wr::<[u64; 6]>(base + 0x10, [0; 6]);
    wr::<u32>(base + 0x08, 1);

    let slot_end = (base + 0x10).wrapping_add(0x30);
    wr::<u64>(slot_end - 0x30, 0);
    wr::<usize>(slot_end - 0x28, exe::ptr::<u8>(0x1_4309_f824) as usize);
    wr::<u64>(slot_end - 0x20, 0);
    wr::<u64>(slot_end - 0x18, 0);
    wr::<u32>(slot_end - 0x08, 0xffff_ffff);
    wr::<u64>(slot_end - 0x10, 0);

    let p320 = rd::<usize>(base + 0x320);
    if p320 != 0 {
        let inner = rd::<usize>(p320 + 0x08);
        if inner != 0 {
            let vtbl = rd::<usize>(inner);
            let pos = exe::call_va!(rd::<usize>(vtbl + 0x10), (inner) -> u64);
            wr::<u64>(slot_end - 0x18, pos);
        }
    }

    wr::<usize>(base + 0x338, fstream_base);
    let inner_file = rd::<usize>(fstream_base + 0x08);
    wr::<usize>(base + 0x340, inner_file);
    wr::<usize>(base + 0x348, vft_bufstream);
    wr::<u64>(base + 0x350, 0);
    wr::<usize>(base + 0x358, vft_filestream);
    wr::<usize>(base + 0x360, inner_file);
    wr::<u16>(base + 0x368, 0x0100);

    exe::call_va!(0x1_40eb_acf0, (rd::<usize>(slot_c0), base));
    wr::<usize>(base + 0x348, vft_bufstream);
    exe::call_va!(0x1_401e_e280, (base));

    let vec_c0 = rd::<usize>(slot_c0);
    let mut rdi = rd::<usize>(vec_c0);
    let cnt = rd::<u32>(vec_c0 + 0x08) as usize;
    let r12_end = rdi.wrapping_add(cnt * 0xc08);
    while rdi != r12_end {
        exe::call_va!(0x1_422f_5460, (rd::<usize>(slot_c0), rdi));
        wr::<u32>(rdi + 0xbfc, 1);
        wr::<u32>(rdi + 0xc00, 1);
        wr::<u8>(rdi + 0xc04, 0);
        if rd::<u32>(rdi + 0x1ec).wrapping_sub(1) <= 1 {
            let key_278 = rd::<u64>(rdi + 0x278);
            let g_82b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
            let arg_rcx = rd::<usize>(g_82b8 + 0x10);
            let arg_18 = rd::<usize>(g_82b8 + 0x18);
            let r8_ctx = exe::call_va!(0x1_4242_0c10, (arg_18) -> u64);
            let node = exe::call_va!(0x1_4242_48a0, (arg_rcx, key_278, r8_ctx) -> usize);
            if node != 0 {
                let res_obj = rd::<usize>(node + 0x18);
                if res_obj != 0 {
                    let expected_type = rd::<u32>(exe::ptr::<u32>(0x1_4388_1958) as usize);
                    let vtbl = rd::<usize>(res_obj);
                    let actual_type = exe::call_va!(rd::<usize>(vtbl + 0x98), (res_obj) -> u32);
                    if actual_type == expected_type {
                        wr::<u32>(rdi + 0xbfc, rd::<u32>(res_obj + 0x6c));
                        wr::<u32>(rdi + 0xc00, rd::<u32>(res_obj + 0x68));
                        let flag = ((rd::<u16>(res_obj + 0x14) as u8) >> 2) & 1;
                        wr::<u8>(rdi + 0xc04, flag);
                    }
                }
            }
        }
        exe::call_va!(0x1_41da_2730, (rdi));
        rdi = rdi.wrapping_add(0xc08);
    }

    let mut vec_c0_2 = rd::<usize>(slot_c0);
    if rd::<u32>(vec_c0_2 + 0x08) > 0 {
        let mut idx: usize = 0;
        let mut off: usize = 0;
        loop {
            let data = rd::<usize>(vec_c0_2);
            wr::<u32>(data.wrapping_add(off + 0x3b0), idx as u32);
            idx += 1;
            off += 0xc08;
            vec_c0_2 = rd::<usize>(slot_c0);
            if idx >= (rd::<u32>(vec_c0_2 + 0x08) as usize) {
                break;
            }
        }
    }

    let mut rbx = rd::<usize>(vec_c0_2);
    let cnt_end = rd::<u32>(vec_c0_2 + 0x08) as usize;
    let rdi_end = rbx.wrapping_add(cnt_end * 0xc08);
    while rbx != rdi_end {
        exe::call_va!(0x1_401f_2bc0, (rbx.wrapping_add(0xa0)));
        exe::call_va!(0x1_401f_2bc0, (rbx.wrapping_add(0xb0)));
        exe::call_va!(0x1_401d_e830, (rbx.wrapping_add(0x110)));
        exe::call_va!(0x1_401d_e830, (rbx.wrapping_add(0x180)));
        exe::call_va!(0x1_401d_e830, (rbx.wrapping_add(0x220)));
        exe::call_va!(0x1_401d_e830, (rbx.wrapping_add(0x260)));
        exe::call_va!(0x1_401f_2bc0, (rbx.wrapping_add(0x2a8)));
        exe::call_va!(0x1_401f_2bc0, (rbx.wrapping_add(0x298)));
        rbx = rbx.wrapping_add(0xc08);
    }

    crate::simd_ops::atomic_xchg_u32((param_1 + 0x80) as u64, 4)
}

/// `unknown_142305b70`
/// Original: `0x142305b70` (464 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_142305b70(param_1: usize) {
    let sub_08 = rd::<usize>(param_1 + 0x08);
    let count = rd::<i32>(sub_08 + 0x138) as isize;
    let new_buf = exe::call_va!(
        0x1_4230_5a60,
        (param_1, 0u64, 0u64, count as u64) -> usize
    );
    let old_buf = rd::<usize>(param_1 + 0x20);
    wr::<usize>(param_1 + 0x20, new_buf);
    if old_buf != 0 {
        exe::call_va!(0x1_4244_2de0, (old_buf));
    }

    if count > 0 {
        let mut pkt = [0u8; 0x28];
        let pkt_ptr = pkt.as_mut_ptr() as usize;
        for idx in 0..(count as usize) {
            let p10 = rd::<usize>(param_1 + 0x10);
            let p08 = rd::<usize>(param_1 + 0x08);
            let r10 = if p10 != 0
                && (idx as u64) < ((rd::<i32>(p08 + 0x138) as i64) as u64)
                && rd::<usize>(p10.wrapping_add(idx * 0x10 + 0x08)) != 0
            {
                rd::<usize>(p10.wrapping_add(idx * 0x10 + 0x08))
            } else {
                let b4f = rd::<u8>(param_1 + 0x4f);
                let arr_base = if b4f != 0 && rd::<u32>(p08 + 0x178) != 0 {
                    let tbl_170 = rd::<usize>(p08 + 0x170);
                    rd::<usize>(tbl_170.wrapping_add((b4f as usize) * 0x18).wrapping_sub(0x10))
                } else {
                    rd::<usize>(p08 + 0x160)
                };
                let entry = rd::<usize>(arr_base.wrapping_add(idx * 8));
                rd::<usize>(entry + 0xc8)
            };

            let arr_130 = rd::<usize>(p08 + 0x130);
            let r9 = rd::<usize>(arr_130.wrapping_add(idx * 8));
            let ecx_val: u32 = if rd::<i32>(r9 + 0xcc) <= 0 {
                rd::<u32>(r9 + 0xd8)
            } else {
                let slot_50 = rd::<u32>(param_1 + 0x50) as usize;
                let g_fed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
                let elem_120 = rd::<usize>(g_fed0 + 0x120).wrapping_add(slot_50 * 0xa0);
                let lod_a8 = rd::<u32>(r9 + 0xa8) as usize;
                let sub_40 = rd::<usize>(elem_120.wrapping_add(lod_a8 * 8 + 0x40));
                let cnt_2a = rd::<u16>(sub_40 + 0x2a) as usize;
                let mut found = 0xffff_ffffu32;
                if cnt_2a != 0 {
                    let mut cur = rd::<usize>(sub_40 + 0x08);
                    for _ in 0..cnt_2a {
                        if rd::<u32>(cur) == (idx as u32) {
                            found = rd::<u32>(cur + 0x04);
                            break;
                        }
                        cur = cur.wrapping_add(0x68);
                    }
                }
                found
            };

            wr::<u64>(pkt_ptr, rd::<u64>(r9 + 0x88));
            wr::<u32>(pkt_ptr + 0x08, rd::<u32>(r9 + 0xb0));
            wr::<u32>(pkt_ptr + 0x0c, rd::<u32>(r9 + 0xc8));
            wr::<u32>(pkt_ptr + 0x10, rd::<u32>(param_1 + 0x48));
            wr::<u16>(pkt_ptr + 0x14, rd::<u16>(r10 + 0x20));
            wr::<u32>(pkt_ptr + 0x18, ecx_val);
            wr::<u32>(pkt_ptr + 0x1c, rd::<u32>(r9 + 0x11c));
            wr::<u8>(
                pkt_ptr + 0x20,
                (rd::<u8>(rd::<usize>(r10) + 0x81) != 0) as u8,
            );

            let res_id = exe::call_va!(0x1_4232_0de0, (ecx_val, pkt_ptr) -> u32);
            let out_arr = rd::<usize>(param_1 + 0x20);
            wr::<u32>(out_arr.wrapping_add(idx * 8), 0);
            wr::<u32>(out_arr.wrapping_add(idx * 8 + 4), res_id);
        }
    }
}

/// `unknown_142307d90`
/// Original: `0x142307d90` (482 bytes, `Throws::Discard`)
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn unknown_142307d90(
    param_1: usize,
    param_2: u64,
    param_3: u32,
    param_4: u32,
    param_5: usize,
    param_6: usize,
    param_7: u64,
    param_8: u8,
    param_9: usize,
) {
    let mut out_ptr = param_9;
    let bit_idx = rd::<u32>(param_5);
    let p5_val8 = rd::<u64>(param_5 + 0x08);

    if param_2 != 0 {
        let mut rsi = param_1.wrapping_add(4);
        let mut rem = param_2;
        while rem != 0 {
            if (rd::<u32>(rsi - 4) & param_3) == param_4 {
                let rdi = rd::<usize>(rsi + 0x04);
                if rd::<i32>(rdi + 0x138) > 0 {
                    let mask_bit = 1u32.rotate_left(bit_idx);
                    let mut ebx: i32 = 0;
                    let mut r15_off: usize = 0;
                    while ebx < rd::<i32>(rdi + 0x138) {
                        let should_call = if param_8 == 0 {
                            true
                        } else {
                            let mode = rd::<i32>(rsi);
                            let entry_ptr_addr = if mode != 0 && rd::<u32>(rdi + 0x178) != 0 {
                                let tbl_170 = rd::<usize>(rdi + 0x170);
                                let sub_arr = rd::<usize>(
                                    tbl_170.wrapping_add(((mode as isize) * 0x18 - 0x10) as usize),
                                );
                                sub_arr.wrapping_add(((ebx as isize) * 8) as usize)
                            } else {
                                let tbl_160 = rd::<usize>(rdi + 0x160);
                                tbl_160.wrapping_add(((ebx as isize) * 8) as usize)
                            };
                            let entry = rd::<usize>(entry_ptr_addr);
                            let c8_ptr = rd::<usize>(entry + 0xc8);
                            let v0 = rd::<usize>(c8_ptr);
                            rd::<u8>(v0 + 0x83) != 0
                        };

                        if should_call {
                            let res = exe::call_va!(
                                0x1_4232_fb50,
                                (
                                    rdi,
                                    ebx,
                                    rd::<i32>(rsi),
                                    param_6,
                                    param_7,
                                    p5_val8,
                                    0usize
                                ) -> usize
                            );
                            if res != 0 {
                                let arr_c = rd::<usize>(rsi + 0x0c);
                                let id_val = rd::<u32>(arr_c.wrapping_add(r15_off + 4));
                                wr::<u32>(out_ptr, id_val);
                                wr::<usize>(out_ptr + 0x08, res);
                                out_ptr = out_ptr.wrapping_add(0x10);
                                let flag_addr = arr_c.wrapping_add(r15_off);
                                wr::<u32>(flag_addr, rd::<u32>(flag_addr) | mask_bit);
                            }
                        }

                        ebx += 1;
                        r15_off += 8;
                    }
                }
            }
            rsi = rsi.wrapping_add(0x18);
            rem -= 1;
        }
    }

    let written_count = ((out_ptr as isize).wrapping_sub(param_9 as isize) >> 4) as u64;
    exe::call_va!(
        0x1_4232_1130,
        (param_5, param_9, written_count, param_7 as u32)
    );
}

/// `r_142309e70`
/// Original: `0x142309e70` (36 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_142309e70(param_1: usize) {
    exe::call_va!(0x1_4230_ae90, (rd::<u64>(param_1 + 0x08)));
    exe::call_va!(
        0x1_4230_aab0,
        (rd::<u64>(param_1 + 0x08), rd::<u64>(param_1 + 0x10))
    );
}

/// `rend_142320de0`
/// Original: `0x142320de0` (590 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_142320de0(_param_1: u64, param_2: usize) -> u32 {
    let rdi = rd::<usize>(exe::ptr::<usize>(0x1_43a0_0190) as usize);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let g_lock = exe::ptr::<u8>(0x1_4386_c5a8) as usize;
    let g_tid = exe::ptr::<u32>(0x1_4386_c5b0) as usize;
    let g_cnt = exe::ptr::<u32>(0x1_4386_c5b4) as usize;

    if rd::<u32>(g_tid) != tid {
        exe::call_iat!(0x1_4264_55a8, (g_lock));
        wr::<u32>(g_tid, tid);
    }

    let new_depth = rd::<u32>(g_cnt).wrapping_add(1);
    wr::<u32>(g_cnt, new_depth);

    let g_free_cnt = exe::ptr::<u32>(0x1_43bb_d848) as usize;
    let free_cnt = rd::<u32>(g_free_cnt);
    let slot_id: u32 = if free_cnt == 0 {
        let g_next = exe::ptr::<u32>(0x1_43a2_b3b8) as usize;
        let id = rd::<u32>(g_next);
        wr::<u32>(g_next, id.wrapping_add(1));
        id
    } else {
        let g_head = exe::ptr::<u32>(0x1_43bb_d840) as usize;
        let g_cap = exe::ptr::<u32>(0x1_43bb_d84c) as usize;
        let g_buf = exe::ptr::<usize>(0x1_43bb_d838) as usize;
        let head = rd::<u32>(g_head);
        let cap = rd::<u32>(g_cap);
        let rem = ((head as u64) % (cap as u64)) as usize;
        let buf = rd::<usize>(g_buf);
        let next_head = head.wrapping_add(1);
        let id = rd::<u32>(buf.wrapping_add(rem * 4));
        wr::<u32>(g_head, next_head);
        if next_head == cap {
            wr::<u32>(g_head, 0);
        }
        wr::<u32>(g_free_cnt, free_cnt.wrapping_sub(1));
        id
    };

    let rem_depth = new_depth.wrapping_sub(1);
    wr::<u32>(g_cnt, rem_depth);
    if rem_depth == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32(g_tid as u64, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (g_lock));
    }

    let rbp = slot_id as u64;
    let obj_f8 = rdi.wrapping_add(0xf8);
    let vtbl_f8 = rd::<usize>(obj_f8);
    let len_f8 = exe::call_va!(rd::<usize>(vtbl_f8 + 0x38), (obj_f8) -> u64);
    if len_f8 <= rbp {
        let vtbl_f8_2 = rd::<usize>(obj_f8);
        let len_f8_2 = exe::call_va!(rd::<usize>(vtbl_f8_2 + 0x38), (obj_f8) -> u64);
        let add_cnt = rbp.wrapping_sub(len_f8_2).wrapping_add(1);
        exe::call_va!(rd::<usize>(vtbl_f8_2 + 0x30), (obj_f8, add_cnt));
    }

    let vtbl_0 = rd::<usize>(rdi);
    let len_0 = exe::call_va!(rd::<usize>(vtbl_0 + 0x38), (rdi) -> u64);
    if len_0 <= rbp {
        let vtbl_0_2 = rd::<usize>(rdi);
        let len_0_2 = exe::call_va!(rd::<usize>(vtbl_0_2 + 0x38), (rdi) -> u64);
        let add_cnt = rbp.wrapping_sub(len_0_2).wrapping_add(1);
        exe::call_va!(rd::<usize>(vtbl_0_2 + 0x30), (rdi, add_cnt));
    }

    let bit = slot_id & 0x3f;
    let word_off = ((slot_id >> 6) as usize) * 8;
    let mask_a8 = rd::<usize>(rdi + 0xa8).wrapping_add(word_off);
    wr::<u64>(mask_a8, rd::<u64>(mask_a8) | (1u64 << bit));

    let entry_10 = rd::<usize>(rdi + 0x98).wrapping_add((slot_id as usize) << 4);
    wr::<u32>(entry_10, rd::<u32>(param_2 + 0x10));
    wr::<u32>(entry_10 + 0x04, slot_id);
    let high_bit = if rd::<u8>(param_2 + 0x20) != 0 {
        0x0001_0000u32
    } else {
        0u32
    };
    let low_16 = rd::<u16>(param_2 + 0x14) as u32;
    wr::<u32>(entry_10 + 0x08, high_bit | low_16);
    wr::<u32>(entry_10 + 0x0c, rd::<u32>(param_2 + 0x18));

    let mask_1a0 = rd::<usize>(rdi + 0x1a0).wrapping_add(word_off);
    wr::<u64>(mask_1a0, rd::<u64>(mask_1a0) | (1u64 << bit));

    let entry_34 = rd::<usize>(rdi + 0x190).wrapping_add((slot_id as usize) * 0x34);
    let src_ptr = rd::<usize>(param_2);
    let tag: u32 = if src_ptr != 0 {
        wr::<u64>(entry_34, rd::<u64>(src_ptr));
        wr::<u32>(entry_34 + 0x08, rd::<u32>(src_ptr + 0x10));
        if rd::<u64>(src_ptr + 0x08) == 2 {
            0x39
        } else {
            0x2a
        }
    } else {
        wr::<u64>(entry_34, 0);
        wr::<u32>(entry_34 + 0x08, 0);
        0
    };
    wr::<u32>(entry_34 + 0x0c, tag);
    let c8 = rd::<u32>(param_2 + 0x08);
    let r8c = rd::<u32>(param_2 + 0x0c);
    wr::<u32>(entry_34 + 0x20, c8.wrapping_mul(3));
    wr::<u32>(entry_34 + 0x24, 1);
    wr::<u32>(entry_34 + 0x28, r8c);
    wr::<u64>(entry_34 + 0x2c, 0);
    wr::<u64>(entry_34 + 0x10, rd::<u64>(entry_10));
    wr::<u64>(entry_34 + 0x18, rd::<u64>(entry_10 + 0x08));

    slot_id
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14222acb0(orig: usize, new: usize) -> Result<u32, String> {
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

    unsafe extern "win64" fn mock_9c90(rcx: usize, h: u32, cnt: u32) {
        unsafe { rec(0x9c90 ^ ((rcx & 0xfff) as u64) ^ ((h as u64) << 16) ^ (cnt as u64)) };
    }
    unsafe extern "win64" fn mock_81b0(rcx: usize, cnt: u32) -> u32 {
        unsafe { rec(0x81b0 ^ ((rcx & 0xfff) as u64) ^ (cnt as u64)) };
        0x700 + cnt
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_9c90, mock_9c90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412f_81b0, mock_81b0 as *const ())?;

    let g_ff48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let s_ff48 = rd::<usize>(g_ff48);

    let run = |f: F| -> (u64, [u32; 2], [u32; 2], u64, u64, [u64; 3]) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut item0 = [0u8; 0x60];
        wr::<u32>(item0.as_mut_ptr() as usize + 0x50, 1);
        let arr_18: [usize; 2] = [item0.as_ptr() as usize, 0];
        let mask_180: u64 = 0b01;
        let lod_bytes: [u8; 2] = [2, 1];
        let mut masks_ff48 = [0u64; 3]; // 668, 680, 698

        let mut ff48_buf = vec![0u8; 0x700];
        let fp = ff48_buf.as_mut_ptr() as usize;
        wr::<usize>(fp + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(fp + 0x20, 2);
        wr::<usize>(fp + 0x180, (&mask_180 as *const u64) as usize);
        wr::<usize>(fp + 0x1a8, lod_bytes.as_ptr() as usize);
        wr::<usize>(fp + 0x668, (&mut masks_ff48[0] as *mut u64) as usize);
        wr::<usize>(fp + 0x680, (&mut masks_ff48[1] as *mut u64) as usize);
        wr::<usize>(fp + 0x698, (&mut masks_ff48[2] as *mut u64) as usize);
        wr::<usize>(g_ff48, fp);

        let mut sub0 = vec![0u8; 0x280];
        wr::<u32>(sub0.as_mut_ptr() as usize + 0x34, 0);
        wr::<u8>(sub0.as_mut_ptr() as usize + 0x272, 0);

        let mut sub1 = vec![0u8; 0x280];
        wr::<u32>(sub1.as_mut_ptr() as usize + 0x34, 0);
        wr::<u8>(sub1.as_mut_ptr() as usize + 0x272, 2);
        wr::<u32>(sub1.as_mut_ptr() as usize + 0x1c8, 10);
        wr::<u32>(sub1.as_mut_ptr() as usize + 0x1cc, 25);

        let mut sub2 = vec![0u8; 0x280];
        wr::<u32>(sub2.as_mut_ptr() as usize + 0x34, 1);

        let idx_80_0: u32 = 0;
        let idx_80_1: u32 = 1;

        let mut elems_120 = vec![0u8; 0xa0 * 3];
        let e0 = elems_120.as_mut_ptr() as usize;
        let e1 = e0 + 0xa0;
        let e2 = e1 + 0xa0;
        wr::<usize>(e0 + 0x80, (&idx_80_0 as *const u32) as usize);
        wr::<usize>(e0 + 0x90, sub0.as_mut_ptr() as usize);
        wr::<usize>(e1 + 0x80, (&idx_80_1 as *const u32) as usize);
        wr::<usize>(e1 + 0x90, sub1.as_mut_ptr() as usize);
        wr::<usize>(e2 + 0x90, sub2.as_mut_ptr() as usize);

        let mut arr_98: [u32; 2] = [0x111, 0x222];
        let mut mask_a8: u64 = 0;
        let mut arr_e0: [u32; 2] = [5, 0];
        let arr_100: [u32; 2] = [0xffff_ffff, 3];
        let mut mask_170: u64 = 0b101; // bit 0 set initially, bit 1 set by first loop, bit 2 above len_180=2

        let mut p1_buf = vec![0u8; 0x190];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x98, arr_98.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0xa8, (&mut mask_a8 as *mut u64) as usize);
        wr::<usize>(p1 + 0xe0, arr_e0.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0x100, arr_100.as_ptr() as usize);
        wr::<usize>(p1 + 0x120, elems_120.as_mut_ptr() as usize);
        wr::<u32>(p1 + 0x128, 2);
        wr::<usize>(p1 + 0x170, (&mut mask_170 as *mut u64) as usize);
        wr::<u64>(p1 + 0x180, 2);

        f(p1);

        (
            unsafe { TRACE_HASH },
            arr_98,
            arr_e0,
            mask_a8,
            mask_170,
            masks_ff48,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_ff48, s_ff48);

    crate::diff_eq!(o, n, "14222acb0 out");
    Ok(1)
}

pub unsafe fn diff_std_14222f750(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_fed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let s_fed0 = rd::<usize>(g_fed0);

    let arr_d0: [u32; 4] = [100, 200, 300, 400];
    let mut fed0_buf = [0u8; 0xe0];
    wr::<usize>(fed0_buf.as_mut_ptr() as usize + 0xd0, arr_d0.as_ptr() as usize);
    wr::<usize>(g_fed0, fed0_buf.as_ptr() as usize);

    let run = |f: F, p2: i32, b272: u8| -> u32 {
        let mut sub_90 = vec![0u8; 0x280];
        let sp = sub_90.as_mut_ptr() as usize;
        wr::<u32>(sp + 0x34, 1);
        wr::<u8>(sp + 0x270, 5);
        wr::<u8>(sp + 0x272, b272);
        wr::<u32>(sp + 0x1c8, 10);
        wr::<u32>(sp + 0x1cc, 20);
        wr::<u32>(sp + 0x1d0, 30);

        let mut p1_buf = [0u8; 0xa0];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x90, sp);
        wr::<u32>(p1 + 0x98, 2);

        f(p1, p2)
    };

    for (p2, b272) in [(0, 2u8), (3, 2u8), (3, 0u8), (6, 2u8)] {
        let o = run(f_o, p2, b272);
        let n = run(f_n, p2, b272);
        crate::diff_eq!(o, n, "14222f750 p2={p2} b272={b272}");
    }

    wr::<usize>(g_fed0, s_fed0);
    Ok(4)
}

pub unsafe fn diff_r_14225c0f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_f0f0(elem: usize) {
        unsafe { rec(0xf0f0 ^ (rd::<u32>(elem + 0x20) as u64)) };
    }
    unsafe extern "win64" fn mock_ef60(elem: usize) {
        unsafe { rec(0xef60 ^ (rd::<u32>(elem + 0x20) as u64)) };
    }
    unsafe extern "win64" fn mock_f280(elem: usize) {
        unsafe { rec(0xf280 ^ (rd::<u32>(elem + 0x20) as u64)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_f0f0, mock_f0f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4222_ef60, mock_ef60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4222_f280, mock_f280 as *const ())?;

    let g_fed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let g_ff48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let g_ff60 = exe::ptr::<usize>(0x1_4397_ff60) as usize;
    let s_fed0 = rd::<usize>(g_fed0);
    let s_ff48 = rd::<usize>(g_ff48);
    let s_ff60 = rd::<usize>(g_ff60);

    let run = |f: F, p2: u8| -> (u64, [u64; 3], [u32; 6], u64) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut masks_ff60 = [0u64; 3]; // 1f8, 210, 228
        let mut ff60_buf = vec![0u8; 0x240];
        let p60 = ff60_buf.as_mut_ptr() as usize;
        wr::<usize>(p60 + 0x1f8, (&mut masks_ff60[0] as *mut u64) as usize);
        wr::<usize>(p60 + 0x210, (&mut masks_ff60[1] as *mut u64) as usize);
        wr::<usize>(p60 + 0x228, (&mut masks_ff60[2] as *mut u64) as usize);
        wr::<usize>(g_ff60, p60);

        let mut tbl_160 = [0xffff_ffffu32; 16];
        tbl_160[0] = 2;
        tbl_160[1] = 5;
        let mut tbl_170 = [0xffff_ffffu32; 16];
        tbl_170[0] = 3;
        tbl_170[1] = 7;

        let mut ff48_buf = vec![0u8; 0x190];
        let p48 = ff48_buf.as_mut_ptr() as usize;
        wr::<usize>(p48 + 0x160, tbl_160.as_ptr() as usize);
        wr::<usize>(p48 + 0x170, tbl_170.as_ptr() as usize);
        wr::<usize>(g_ff48, p48);

        let grp_list: [u32; 1] = [0];
        let mut sub_148 = [0u8; 0x50];
        let s148 = sub_148.as_mut_ptr() as usize;
        wr::<u16>(s148 + 0x08, 2);
        wr::<usize>(s148 + 0x30, grp_list.as_ptr() as usize);
        wr::<u32>(s148 + 0x38, 1);
        wr::<u32>(s148 + 0x48, 10);

        let mut sub_90 = vec![0u8; 0x280];
        let s90 = sub_90.as_mut_ptr() as usize;
        wr::<u32>(s90 + 0x34, 1);
        wr::<u8>(s90 + 0x270, 4);
        wr::<u8>(s90 + 0x272, 2);
        wr::<u32>(s90 + 0x1c8, 11);
        wr::<u32>(s90 + 0x1cc, 22);

        let sub_ids_18: [u32; 1] = [0];
        let mut elem_120 = [0u8; 0xa0];
        let e120 = elem_120.as_mut_ptr() as usize;
        wr::<usize>(e120 + 0x18, sub_ids_18.as_ptr() as usize);
        wr::<u32>(e120 + 0x20, 1);
        wr::<usize>(e120 + 0x90, s90);

        let arr_100: [u32; 1] = [4];
        let mut fed0_buf = vec![0u8; 0x160];
        let pfed0 = fed0_buf.as_mut_ptr() as usize;
        wr::<usize>(pfed0 + 0x100, arr_100.as_ptr() as usize);
        wr::<usize>(pfed0 + 0x120, e120);
        wr::<usize>(pfed0 + 0x148, s148);
        wr::<usize>(g_fed0, pfed0);

        let mut item0 = [0u8; 0x60];
        wr::<u32>(item0.as_mut_ptr() as usize + 0x48, 0);
        wr::<u32>(item0.as_mut_ptr() as usize + 0x50, 0);
        wr::<u32>(item0.as_mut_ptr() as usize + 0x5c, 0);

        let mut item1 = [0u8; 0x60];
        wr::<u32>(item1.as_mut_ptr() as usize + 0x48, 0);
        wr::<u32>(item1.as_mut_ptr() as usize + 0x50, 0xffff_ffff);

        let arr_18: [usize; 2] = [item0.as_ptr() as usize, item1.as_ptr() as usize];
        let mut arr_28 = [0u32; 6];
        let arr_130: [u64; 1] = [0];
        let mask_180: u64 = 0b11;
        let lod_1a8: [u8; 2] = [2, 0];
        let mask_638: u64 = 0b11;
        let mask_668: u64 = 0b01;
        let mut mask_698: u64 = 0b01;
        let mask_6b0: u64 = 0b00;

        let mut p1_buf = vec![0u8; 0x6c0];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(p1 + 0x20, 2);
        wr::<usize>(p1 + 0x28, arr_28.as_mut_ptr() as usize);
        wr::<usize>(p1 + 0x130, arr_130.as_ptr() as usize);
        wr::<usize>(p1 + 0x180, (&mask_180 as *const u64) as usize);
        wr::<usize>(p1 + 0x1a8, lod_1a8.as_ptr() as usize);
        wr::<usize>(p1 + 0x638, (&mask_638 as *const u64) as usize);
        wr::<u64>(p1 + 0x648, 2);
        wr::<usize>(p1 + 0x668, (&mask_668 as *const u64) as usize);
        wr::<u64>(p1 + 0x678, 1);
        wr::<usize>(p1 + 0x698, (&mut mask_698 as *mut u64) as usize);
        wr::<u64>(p1 + 0x6a8, 1);
        wr::<usize>(p1 + 0x6b0, (&mask_6b0 as *const u64) as usize);

        f(p1, p2);

        (unsafe { TRACE_HASH }, masks_ff60, arr_28, mask_698)
    };

    for p2 in [0u8, 1u8] {
        let o = run(f_o, p2);
        let n = run(f_n, p2);
        crate::diff_eq!(o, n, "14225c0f0 p2={p2}");
    }

    wr::<usize>(g_fed0, s_fed0);
    wr::<usize>(g_ff48, s_ff48);
    wr::<usize>(g_ff60, s_ff60);
    Ok(2)
}

pub unsafe fn diff_rend_1422f58c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_C0_PTR: usize = 0;
    static mut NODE_PTR: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_5820() -> usize {
        unsafe {
            rec(0x5820);
            VEC_C0_PTR
        }
    }
    unsafe extern "win64" fn mock_file_pos(_this: usize) -> u64 {
        unsafe { rec(0x0010) };
        0x1234
    }
    unsafe extern "win64" fn mock_acf0(vec_c0: usize, stream_base: usize) {
        let pos = rd::<u64>(stream_base + 0x10 + 0x18);
        let flag_368 = rd::<u16>(stream_base + 0x368) as u64;
        unsafe { rec(0xacf0 ^ (vec_c0 as u64) ^ pos ^ flag_368) };
    }
    unsafe extern "win64" fn mock_e280(stream_base: usize) {
        unsafe { rec(0xe280 ^ (rd::<u32>(stream_base + 0x0c) as u64)) };
    }
    unsafe extern "win64" fn mock_5460(_vec_c0: usize, elem: usize) {
        unsafe { rec(0x5460 ^ (rd::<u32>(elem + 0x1ec) as u64)) };
    }
    unsafe extern "win64" fn mock_0c10(arg: usize) -> u64 {
        unsafe { rec(0x0c10 ^ (arg as u64)) };
        0x9999
    }
    unsafe extern "win64" fn mock_48a0(rcx: usize, key: u64, r8: u64) -> usize {
        unsafe {
            rec(0x48a0 ^ (rcx as u64) ^ key ^ r8);
            NODE_PTR
        }
    }
    unsafe extern "win64" fn mock_vfunc_98(_this: usize) -> u32 {
        0x4242
    }
    unsafe extern "win64" fn mock_2730(elem: usize) {
        unsafe { rec(0x2730 ^ (rd::<u32>(elem + 0xbfc) as u64)) };
    }
    unsafe extern "win64" fn mock_2bc0(_ptr: usize) {
        unsafe { rec(0x2bc0) };
    }
    unsafe extern "win64" fn mock_e830(_ptr: usize) {
        unsafe { rec(0xe830) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f7_5820, mock_5820 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40eb_acf0, mock_acf0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_422f_5460, mock_5460 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4242_0c10, mock_0c10 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4242_48a0, mock_48a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41da_2730, mock_2730 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_401f_2bc0, mock_2bc0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let g_82b8 = exe::ptr::<usize>(0x1_43a0_82b8) as usize;
    let g_1958 = exe::ptr::<u32>(0x1_4388_1958) as usize;
    let s_82b8 = rd::<usize>(g_82b8);
    let s_1958 = rd::<u32>(g_1958);

    let mut mgr_82b8 = [0u64; 4];
    mgr_82b8[2] = 0x1111;
    mgr_82b8[3] = 0x2222;
    wr::<usize>(g_82b8, mgr_82b8.as_ptr() as usize);
    wr::<u32>(g_1958, 0x4242);

    let run = |f: F| -> (u32, u64, [u32; 4], [u8; 2]) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut file_vtbl = [0usize; 4];
        file_vtbl[2] = mock_file_pos as *const () as usize;
        let file_obj: [usize; 1] = [file_vtbl.as_ptr() as usize];

        let mut res_vtbl = [0usize; 20];
        res_vtbl[19] = mock_vfunc_98 as *const () as usize;
        let mut res_obj = [0u8; 0x80];
        let rp = res_obj.as_mut_ptr() as usize;
        wr::<usize>(rp, res_vtbl.as_ptr() as usize);
        wr::<u16>(rp + 0x14, 0x0004);
        wr::<u32>(rp + 0x68, 77);
        wr::<u32>(rp + 0x6c, 88);

        let mut node = [0u64; 4];
        node[3] = rp as u64;
        unsafe { NODE_PTR = node.as_mut_ptr() as usize };

        let mut elems = vec![0u8; 0xc08 * 2];
        let e0 = elems.as_mut_ptr() as usize;
        let e1 = e0 + 0xc08;
        wr::<u32>(e0 + 0x1ec, 1);
        wr::<u64>(e0 + 0x278, 0xabcd_ef01);
        wr::<u32>(e1 + 0x1ec, 0);

        let mut vec_c0 = [0u64; 2];
        wr::<usize>(vec_c0.as_mut_ptr() as usize, e0);
        wr::<u32>(vec_c0.as_mut_ptr() as usize + 0x08, 2);
        unsafe { VEC_C0_PTR = vec_c0.as_mut_ptr() as usize };

        let mut p1_buf = [0u8; 0xd0];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<u32>(p1 + 0x80, 2);

        let ret = f(p1, file_obj.as_ptr() as usize);
        let out_u32 = [
            rd::<u32>(e0 + 0xbfc),
            rd::<u32>(e0 + 0xc00),
            rd::<u32>(e0 + 0x3b0),
            rd::<u32>(e1 + 0x3b0),
        ];
        let out_u8 = [rd::<u8>(e0 + 0xc04), rd::<u32>(p1 + 0x80) as u8];
        (ret, unsafe { TRACE_HASH }, out_u32, out_u8)
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<usize>(g_82b8, s_82b8);
    wr::<u32>(g_1958, s_1958);

    crate::diff_eq!(o, n, "1422f58c0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142305b70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUF_PTR: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_5a60(_rcx: usize, _rdx: u64, _r8: u64, r9_cnt: u64) -> usize {
        unsafe {
            rec(0x5a60 ^ r9_cnt);
            ALLOC_BUF_PTR
        }
    }
    unsafe extern "win64" fn mock_2de0(old_ptr: usize) {
        unsafe { rec(0x2de0 ^ (old_ptr as u64)) };
    }
    unsafe extern "win64" fn mock_0de0(ecx_val: u32, pkt_ptr: usize) -> u32 {
        let p0 = rd::<u64>(pkt_ptr);
        let p1 = rd::<u64>(pkt_ptr + 0x08);
        let p2 = rd::<u64>(pkt_ptr + 0x10);
        let p3 = rd::<u64>(pkt_ptr + 0x18);
        let p4 = rd::<u8>(pkt_ptr + 0x20) as u64;
        unsafe { rec(0x0de0 ^ (ecx_val as u64) ^ p0 ^ p1 ^ p2 ^ p3 ^ p4) };
        0x500 + ecx_val
    }

    let _m1 = crate::difftest::Mock::new(0x1_4230_5a60, mock_5a60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4232_0de0, mock_0de0 as *const ())?;

    let g_fed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let s_fed0 = rd::<usize>(g_fed0);

    let run = |f: F| -> (u64, [u32; 4]) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut alloc_storage = [0u32; 4];
        unsafe { ALLOC_BUF_PTR = alloc_storage.as_mut_ptr() as usize };

        let mut r10_v0 = [0u8; 0x90];
        r10_v0[0x81] = 1;
        let mut r10_obj = [0u8; 0x30];
        wr::<usize>(r10_obj.as_mut_ptr() as usize, r10_v0.as_ptr() as usize);
        wr::<u16>(r10_obj.as_mut_ptr() as usize + 0x20, 0x3344);

        let mut entry_160 = [0u8; 0xd0];
        wr::<usize>(entry_160.as_mut_ptr() as usize + 0xc8, r10_obj.as_ptr() as usize);
        let arr_160: [usize; 2] = [entry_160.as_ptr() as usize, entry_160.as_ptr() as usize];

        let mut r9_0 = [0u8; 0x120];
        let p9_0 = r9_0.as_mut_ptr() as usize;
        wr::<u64>(p9_0 + 0x88, 0x1111_2222);
        wr::<u32>(p9_0 + 0xb0, 10);
        wr::<u32>(p9_0 + 0xc8, 20);
        wr::<i32>(p9_0 + 0xcc, 0);
        wr::<u32>(p9_0 + 0xd8, 7);
        wr::<u32>(p9_0 + 0x11c, 30);

        let mut r9_1 = [0u8; 0x120];
        let p9_1 = r9_1.as_mut_ptr() as usize;
        wr::<u64>(p9_1 + 0x88, 0x3333_4444);
        wr::<u32>(p9_1 + 0xa8, 0);
        wr::<u32>(p9_1 + 0xb0, 11);
        wr::<u32>(p9_1 + 0xc8, 21);
        wr::<i32>(p9_1 + 0xcc, 1);
        wr::<u32>(p9_1 + 0x11c, 31);

        let arr_130: [usize; 2] = [p9_0, p9_1];

        let mut p08_buf = [0u8; 0x180];
        let p08 = p08_buf.as_mut_ptr() as usize;
        wr::<usize>(p08 + 0x130, arr_130.as_ptr() as usize);
        wr::<i32>(p08 + 0x138, 2);
        wr::<usize>(p08 + 0x160, arr_160.as_ptr() as usize);

        let mut search_arr = [0u8; 0x68 * 2];
        wr::<u32>(search_arr.as_mut_ptr() as usize, 0);
        wr::<u32>(search_arr.as_mut_ptr() as usize + 0x04, 15);
        wr::<u32>(search_arr.as_mut_ptr() as usize + 0x68, 1);
        wr::<u32>(search_arr.as_mut_ptr() as usize + 0x6c, 42);

        let mut sub_40 = [0u8; 0x30];
        wr::<usize>(sub_40.as_mut_ptr() as usize + 0x08, search_arr.as_ptr() as usize);
        wr::<u16>(sub_40.as_mut_ptr() as usize + 0x2a, 2);

        let mut elem_120 = [0u8; 0xa0];
        wr::<usize>(elem_120.as_mut_ptr() as usize + 0x40, sub_40.as_ptr() as usize);

        let mut fed0_buf = [0u8; 0x130];
        wr::<usize>(fed0_buf.as_mut_ptr() as usize + 0x120, elem_120.as_ptr() as usize);
        wr::<usize>(g_fed0, fed0_buf.as_ptr() as usize);

        let mut p1_buf = [0u8; 0x60];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x08, p08);
        wr::<usize>(p1 + 0x10, 0);
        wr::<usize>(p1 + 0x20, 0x9999);
        wr::<u32>(p1 + 0x48, 88);
        wr::<u8>(p1 + 0x4f, 0);
        wr::<u32>(p1 + 0x50, 0);

        f(p1);

        (unsafe { TRACE_HASH }, alloc_storage)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_fed0, s_fed0);

    crate::diff_eq!(o, n, "142305b70 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142307d90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u32, u32, usize, usize, u64, u8, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_fb50(
        _rdi: usize,
        ebx: i32,
        mode: i32,
        p6: usize,
        p7: u64,
        p5_8: u64,
        p_zero: usize,
    ) -> usize {
        unsafe {
            rec(0xfb50
                ^ (ebx as u64)
                ^ (mode as u64)
                ^ (p6 as u64)
                ^ p7
                ^ p5_8
                ^ (p_zero as u64));
        }
        if ebx == 0 {
            0x7777_0000
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_1130(_p5: usize, _p9: usize, cnt: u64, p7_lo: u32) {
        unsafe { rec(0x1130 ^ (cnt << 16) ^ (p7_lo as u64)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4232_fb50, mock_fb50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4232_1130, mock_1130 as *const ())?;

    let run = |f: F, p8: u8| -> (u64, [u32; 4], [u64; 4]) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut v0_buf = [0u8; 0x90];
        v0_buf[0x83] = 1;
        let c8_obj: [usize; 1] = [v0_buf.as_ptr() as usize];
        let mut entry = [0u8; 0xd0];
        wr::<usize>(entry.as_mut_ptr() as usize + 0xc8, c8_obj.as_ptr() as usize);
        let tbl_160: [usize; 2] = [entry.as_ptr() as usize, entry.as_ptr() as usize];

        let mut rdi_buf = [0u8; 0x180];
        let rdi = rdi_buf.as_mut_ptr() as usize;
        wr::<i32>(rdi + 0x138, 2);
        wr::<usize>(rdi + 0x160, tbl_160.as_ptr() as usize);

        let mut arr_c: [u32; 4] = [0, 101, 0, 202];
        let mut items = [0u8; 0x30];
        let ip = items.as_mut_ptr() as usize;
        wr::<u32>(ip, 0x03);
        wr::<i32>(ip + 0x04, 0);
        wr::<usize>(ip + 0x08, rdi);
        wr::<usize>(ip + 0x10, arr_c.as_mut_ptr() as usize);
        wr::<u32>(ip + 0x18, 0x00);

        let mut p5_buf = [0u64; 2];
        wr::<u32>(p5_buf.as_mut_ptr() as usize, 3);
        wr::<u64>(p5_buf.as_mut_ptr() as usize + 0x08, 0xaaaa_bbbb);

        let mut out_buf = [0u64; 4];
        f(
            ip,
            2,
            0x03,
            0x03,
            p5_buf.as_ptr() as usize,
            0x1234,
            5,
            p8,
            out_buf.as_mut_ptr() as usize,
        );

        (unsafe { TRACE_HASH }, arr_c, out_buf)
    };

    for p8 in [0u8, 1u8] {
        let o = run(f_o, p8);
        let n = run(f_n, p8);
        crate::diff_eq!(o, n, "142307d90 p8={p8}");
    }
    Ok(2)
}

pub unsafe fn diff_r_142309e70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_ae90(a0: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ 0xae90 ^ a0).wrapping_mul(0x1000_0000_01b3);
        }
    }
    unsafe extern "win64" fn mock_aab0(a0: u64, a1: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ 0xaab0 ^ a0 ^ a1.rotate_left(13)).wrapping_mul(0x1000_0000_01b3);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4230_ae90, mock_ae90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4230_aab0, mock_aab0 as *const ())?;

    let job: [u64; 3] = [0, 0x1122_3344, 0x5566_7788];
    let run = |f: F| -> u64 {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };
        f(job.as_ptr() as usize);
        unsafe { TRACE_HASH }
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142309e70 out");
    Ok(1)
}

pub unsafe fn diff_rend_142320de0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_vfunc_38(_this: usize) -> u64 {
        1
    }
    unsafe extern "win64" fn mock_vfunc_30(_this: usize, add_cnt: u64) {
        unsafe { rec(0x0030 ^ add_cnt) };
    }

    let g_0190 = exe::ptr::<usize>(0x1_43a0_0190) as usize;
    let g_tid = exe::ptr::<u32>(0x1_4386_c5b0) as usize;
    let g_cnt = exe::ptr::<u32>(0x1_4386_c5b4) as usize;
    let g_free_buf = exe::ptr::<usize>(0x1_43bb_d838) as usize;
    let g_free_head = exe::ptr::<u32>(0x1_43bb_d840) as usize;
    let g_free_cnt = exe::ptr::<u32>(0x1_43bb_d848) as usize;
    let g_free_cap = exe::ptr::<u32>(0x1_43bb_d84c) as usize;
    let g_next_id = exe::ptr::<u32>(0x1_43a2_b3b8) as usize;

    let s_0190 = rd::<usize>(g_0190);
    let s_tid = rd::<u32>(g_tid);
    let s_cnt = rd::<u32>(g_cnt);
    let s_free_buf = rd::<usize>(g_free_buf);
    let s_free_head = rd::<u32>(g_free_head);
    let s_free_cnt = rd::<u32>(g_free_cnt);
    let s_free_cap = rd::<u32>(g_free_cap);
    let s_next_id = rd::<u32>(g_next_id);

    let run = |f: F, use_freelist: bool, non_null_src: bool| -> (u32, u64, [u32; 12], [u8; 0x34 * 3], u64, u64) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let free_ids: [u32; 2] = [1, 2];
        wr::<u32>(g_tid, 0xffff_ffff);
        wr::<u32>(g_cnt, 0);
        wr::<usize>(g_free_buf, free_ids.as_ptr() as usize);
        wr::<u32>(g_free_head, 1);
        wr::<u32>(g_free_cnt, if use_freelist { 1 } else { 0 });
        wr::<u32>(g_free_cap, 2);
        wr::<u32>(g_next_id, 1);

        let mut vtbl = [0usize; 8];
        vtbl[6] = mock_vfunc_30 as *const () as usize;
        vtbl[7] = mock_vfunc_38 as *const () as usize;

        let mut buf_98 = [0u32; 12]; // 3 slots * 16 bytes
        let mut mask_a8: u64 = 0;
        let mut buf_190 = [0u8; 0x34 * 3];
        let mut mask_1a0: u64 = 0;

        let mut rdi_buf = vec![0u8; 0x1b0];
        let rp = rdi_buf.as_mut_ptr() as usize;
        wr::<usize>(rp, vtbl.as_ptr() as usize);
        wr::<usize>(rp + 0x98, buf_98.as_mut_ptr() as usize);
        wr::<usize>(rp + 0xa8, (&mut mask_a8 as *mut u64) as usize);
        wr::<usize>(rp + 0xf8, vtbl.as_ptr() as usize);
        wr::<usize>(rp + 0x190, buf_190.as_mut_ptr() as usize);
        wr::<usize>(rp + 0x1a0, (&mut mask_1a0 as *mut u64) as usize);
        wr::<usize>(g_0190, rp);

        let mut src_info = [0u8; 0x18];
        wr::<u64>(src_info.as_mut_ptr() as usize, 0x1122_3344_5566_7788);
        wr::<u64>(src_info.as_mut_ptr() as usize + 0x08, 2);
        wr::<u32>(src_info.as_mut_ptr() as usize + 0x10, 0x99aa_bbcc);

        let mut p2_buf = [0u8; 0x28];
        let p2 = p2_buf.as_mut_ptr() as usize;
        wr::<usize>(
            p2,
            if non_null_src {
                src_info.as_ptr() as usize
            } else {
                0
            },
        );
        wr::<u32>(p2 + 0x08, 10);
        wr::<u32>(p2 + 0x0c, 20);
        wr::<u32>(p2 + 0x10, 30);
        wr::<u16>(p2 + 0x14, 0x1234);
        wr::<u32>(p2 + 0x18, 40);
        wr::<u8>(p2 + 0x20, 1);

        let id = f(0, p2);
        (id, unsafe { TRACE_HASH }, buf_98, buf_190, mask_a8, mask_1a0)
    };

    for (use_fl, non_null) in [(false, true), (true, false)] {
        let o = run(f_o, use_fl, non_null);
        let n = run(f_n, use_fl, non_null);
        crate::diff_eq!(o, n, "142320de0 fl={use_fl} nn={non_null}");
    }

    wr::<usize>(g_0190, s_0190);
    wr::<u32>(g_tid, s_tid);
    wr::<u32>(g_cnt, s_cnt);
    wr::<usize>(g_free_buf, s_free_buf);
    wr::<u32>(g_free_head, s_free_head);
    wr::<u32>(g_free_cnt, s_free_cnt);
    wr::<u32>(g_free_cap, s_free_cap);
    wr::<u32>(g_next_id, s_next_id);

    Ok(2)
}

// =============================================================================
// REGISTRY (for `games/alan-wake-2/crates/aw2-game/src/registry/core.rs`)
// =============================================================================
// Replacement::verified(0x1_4222_acb0, "unknown_14222acb0", rend::cauldron_hot_t1::unknown_14222acb0 as *const (), rend::cauldron_hot_t1::diff_unknown_14222acb0),
// Replacement::verified(0x1_4222_f750, "std_14222f750", rend::cauldron_hot_t1::std_14222f750 as *const (), rend::cauldron_hot_t1::diff_std_14222f750),
// Replacement::verified_throws(0x1_4225_c0f0, "r_14225c0f0", rend::cauldron_hot_t1::r_14225c0f0 as *const (), Throws::Discard, rend::cauldron_hot_t1::diff_r_14225c0f0),
// Replacement::verified_throws(0x1_422f_58c0, "rend_1422f58c0", rend::cauldron_hot_t1::rend_1422f58c0 as *const (), Throws::Caught, rend::cauldron_hot_t1::diff_rend_1422f58c0),
// Replacement::verified(0x1_4230_5b70, "unknown_142305b70", rend::cauldron_hot_t1::unknown_142305b70 as *const (), rend::cauldron_hot_t1::diff_unknown_142305b70),
// Replacement::verified_throws(0x1_4230_7d90, "unknown_142307d90", rend::cauldron_hot_t1::unknown_142307d90 as *const (), Throws::Discard, rend::cauldron_hot_t1::diff_unknown_142307d90),
// Replacement::verified_throws(0x1_4230_9e70, "r_142309e70", rend::cauldron_hot_t1::r_142309e70 as *const (), Throws::Discard, rend::cauldron_hot_t1::diff_r_142309e70),
// Replacement::verified(0x1_4232_0de0, "rend_142320de0", rend::cauldron_hot_t1::rend_142320de0 as *const (), rend::cauldron_hot_t1::diff_rend_142320de0),
