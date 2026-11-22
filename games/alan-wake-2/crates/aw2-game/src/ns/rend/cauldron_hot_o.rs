//! Decompiled hot functions shard O (`0x140209da0..0x141412320`).

use crate::exe::{self, rd, wr};

/// `app_140209da0`
/// Original: 0x140209da0 (390 bytes, Throws::No)
pub unsafe extern "win64" fn app_140209da0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const FNV_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const MIX_MUL: u64 = 0xde5f_b9d2_6304_58e9;

    let key_bytes = rd::<[u8; 8]>(param_3);
    let mut h = FNV_BASIS;
    for &b in &key_bytes {
        h ^= b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }

    let prod = (h as u128).wrapping_mul(MIX_MUL as u128);
    let r15 = ((prod >> 64) as u64).wrapping_add(prod as u64);

    let mask = rd::<u64>(param_1 + 0x18);
    let ctrl_base = rd::<usize>(param_1);
    let mut r8 = (r15 >> 7) & mask;
    let h2 = (r15 as u8) & 0x7f;
    let xmm2 = crate::simd_ops::vpbroadcastb(h2);
    let mut step: u64 = 0;

    let (slot_idx, inserted): (u64, u8) = loop {
        let xmm1 = rd::<u128>(ctrl_base.wrapping_add(r8 as usize));
        let mut eq_mask =
            crate::simd_ops::vpmovmskb(crate::simd_ops::vpcmpeqb(xmm1, xmm2)) as u32;
        if eq_mask != 0 {
            let slots_base = rd::<usize>(param_1 + 0x08);
            let want_key = rd::<u64>(param_3);
            let mut found_idx: Option<u64> = None;
            while eq_mask != 0 {
                let bit = eq_mask.trailing_zeros() as u64;
                let idx = r8.wrapping_add(bit) & mask;
                if rd::<u64>(slots_base.wrapping_add((idx as usize) * 16)) == want_key {
                    found_idx = Some(idx);
                    break;
                }
                eq_mask &= eq_mask.wrapping_sub(1);
            }
            if let Some(idx) = found_idx {
                break (idx, 0);
            }
        }

        let empty_mask =
            crate::simd_ops::vpmovmskb(crate::simd_ops::vpsignb(xmm1, xmm1)) as u32;
        if empty_mask != 0 {
            let idx = exe::call_va!(0x1_4020_a020, (param_1, r15) -> u64);
            let key_val = rd::<u64>(param_3);
            let slot_ptr = rd::<usize>(param_1 + 0x08).wrapping_add((idx as usize) * 16);
            wr::<u64>(slot_ptr, key_val);
            wr::<u64>(slot_ptr + 0x08, 0);
            break (idx, 1);
        }

        step = step.wrapping_add(0x10);
        r8 = r8.wrapping_add(step) & mask;
    };

    let final_ctrl = rd::<usize>(param_1);
    let final_slots = rd::<usize>(param_1 + 0x08);
    wr::<usize>(param_2, final_ctrl.wrapping_add(slot_idx as usize));
    wr::<usize>(
        param_2 + 0x08,
        final_slots.wrapping_add((slot_idx as usize) * 16),
    );
    wr::<u8>(param_2 + 0x10, inserted);
    param_2
}

/// `unknown_140217680`
/// Original: 0x140217680 (82 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140217680(param_1: usize, param_2: usize) -> usize {
    let val_ptr = param_1.wrapping_add(0x08);
    if rd::<u16>(param_1) == 4 {
        return val_ptr;
    }

    let idx = rd::<usize>(val_ptr);
    let entry = rd::<usize>(param_2 + 0x80).wrapping_add(idx.wrapping_mul(0x48));
    let b3 = rd::<u8>(entry + 0x03) as u32;
    let data_field = entry.wrapping_add(0x08);
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(entry) & 0x00ff_ffff) << 3;
    if len_bits <= cap_bits {
        data_field
    } else {
        rd::<usize>(data_field)
    }
}

/// `unknown_140227390`
/// Original: 0x140227390 (735 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140227390(param_1: usize, param_2: usize) {
    let diff = rd::<u64>(param_2 + 0x38).wrapping_sub(rd::<u64>(param_2 + 0x30));
    let new_cap = exe::call_va!(0x1_401e_2020, (param_1, diff) -> u64);

    let old_buckets = rd::<usize>(param_1 + 0x58);
    let buckets = if old_buckets != 0 {
        if new_cap == rd::<u64>(param_1 + 0x40) {
            old_buckets
        } else {
            let r10 = old_buckets.wrapping_sub(1) & !0x01ff_ffffusize;
            let page_idx = old_buckets.wrapping_sub(r10) >> 16;
            let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
            let rdx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
            if exe::teb() == rd::<usize>(r10 + 0x100) {
                if rd::<u8>(rdx + 0x0e) == 0 {
                    wr::<u64>(old_buckets, rd::<u64>(rdx + 0x20));
                    let new_cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
                    wr::<u32>(rdx + 0x18, new_cnt);
                    wr::<usize>(rdx + 0x20, old_buckets);
                    if new_cnt == 0 {
                        exe::call_va!(0x1_4253_71a0, (rdx));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10, rdx, 1u8, old_buckets));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (r10, rdx, 0u8, old_buckets));
            }
            let allocated = exe::call_va!(0x1_4021_5e80, (0usize, new_cap) -> usize);
            wr::<usize>(param_1 + 0x58, allocated);
            allocated
        }
    } else {
        let allocated = exe::call_va!(0x1_4021_5e80, (0usize, new_cap) -> usize);
        wr::<usize>(param_1 + 0x58, allocated);
        allocated
    };

    let end_init = buckets.wrapping_add((new_cap as usize).wrapping_mul(16));
    let mut cur_init = buckets;
    while cur_init != end_init {
        let sentinel_pair = rd::<u128>(param_1 + 0x48);
        wr::<u128>(cur_init, sentinel_pair);
        cur_init = cur_init.wrapping_add(16);
    }

    wr::<u64>(param_1 + 0x38, 0);
    wr::<u64>(param_1 + 0x30, 0);
    wr::<u64>(param_1 + 0x40, new_cap);

    let cap_f32 = new_cap as f32;
    let two63 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fc8) as usize);

    let mut f0 = cap_f32 * rd::<f32>(param_1 + 0x10);
    let mut off0: u64 = 0;
    if f0 >= two63 {
        f0 -= two63;
        if f0 < two63 {
            off0 = 0x8000_0000_0000_0000;
        }
    }
    let thresh0 = crate::simd_ops::cvt_f32_i64(f0).wrapping_add(off0);

    let mut f1 = cap_f32 * rd::<f32>(param_1 + 0x14);
    wr::<u64>(param_1, thresh0);
    let mut off1: u64 = 0;
    if f1 >= two63 {
        f1 -= two63;
        if f1 < two63 {
            off1 = 0x8000_0000_0000_0000;
        }
    }
    wr::<u8>(param_1 + 0x18, 0);
    let thresh1 = crate::simd_ops::cvt_f32_i64(f1).wrapping_add(off1);
    wr::<u64>(param_1 + 0x08, thresh1);

    let mut src_cur = rd::<usize>(param_2 + 0x58);
    let src_end = src_cur.wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(16));
    if src_cur != src_end {
        let src_empty = rd::<u64>(param_2 + 0x48);
        loop {
            let k = rd::<u64>(src_cur);
            if k != src_empty && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
            {
                break;
            }
            src_cur = src_cur.wrapping_add(16);
            if src_cur == src_end {
                break;
            }
        }
    }

    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const FNV_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

    loop {
        let cur_src_end = rd::<usize>(param_2 + 0x58)
            .wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(16));
        if src_cur == cur_src_end {
            wr::<u32>(param_1 + 0x1c, rd::<u32>(param_1 + 0x1c).wrapping_add(1));
            return;
        }

        let key_bytes = rd::<[u8; 8]>(src_cur);
        let mask = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
        let dst_base = rd::<usize>(param_1 + 0x58);
        let dst_empty = rd::<u64>(param_1 + 0x48);

        let mut h = FNV_BASIS;
        for &b in &key_bytes {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        let mut idx = h & mask;
        let mut step: u64 = 0;
        while rd::<u64>(dst_base.wrapping_add((idx as usize) * 16)) != dst_empty {
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
        }

        let entry_val = rd::<u128>(src_cur);
        wr::<u128>(dst_base.wrapping_add((idx as usize) * 16), entry_val);
        wr::<u64>(param_1 + 0x38, rd::<u64>(param_1 + 0x38).wrapping_add(1));

        loop {
            src_cur = src_cur.wrapping_add(16);
            if src_cur == src_end {
                break;
            }
            let k = rd::<u64>(src_cur);
            if rd::<u64>(param_2 + 0x48) != k
                && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
            {
                break;
            }
        }
    }
}

/// `unknown_14024e6a0`
/// Original: 0x14024e6a0 (617 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14024e6a0(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let id_ed4 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ced4) as usize) as usize;
        let id_e88 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
        let world = rd::<usize>(param_1);
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let id_168 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0168) as usize) as usize;
        let v0 = rd::<usize>(world);
        let id_260 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0260) as usize) as usize;
        let lut = rd::<usize>(v0 + 0x20);
        let idx_ed4 = rd::<u16>(lut.wrapping_add(id_ed4 * 2)) as usize;
        let entry_ed4 = rd::<usize>(world + 0x08).wrapping_add(idx_ed4 * 16);
        let len_ed4 = rd::<u32>(entry_ed4 + 0x08) as u64;
        let ptr_ed4 = rd::<u64>(entry_ed4);

        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let id_ebc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cebc) as usize) as usize;
        let val_e88 = rd::<u64>(tbl_580e0.wrapping_add(id_e88 * 8));
        let val_168 = rd::<u64>(tbl_580e0.wrapping_add(id_168 * 8));
        let val_ebc = rd::<u64>(tbl_580e0.wrapping_add(id_ebc * 8));
        let val_260 = rd::<u64>(tbl_580e0.wrapping_add(id_260 * 8));

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_198 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0198) as usize) as u32;
                let arch_off = arch.wrapping_shl(5);
                let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
                let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
                let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_198) -> i32);
                let mut comp_off = if (found as isize) != -1 {
                    let off_tbl = rd::<usize>(
                        world
                            .wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5))
                            .wrapping_add(0x40),
                    );
                    rd::<u32>(off_tbl.wrapping_add((found as isize as usize) * 4)) as usize
                } else {
                    0
                };
                if comp_off != 0 {
                    comp_off = block.wrapping_add(comp_off);
                }

                for idx in 0..ent_count {
                    let mut arg1: [u64; 3] = [comp_off as u64, block as u64, idx];
                    let mut arg2: [u64; 3] = [world as u64, len_ed4, ptr_ed4];
                    exe::call_va!(
                        0x1_4189_2bd0,
                        (
                            arg1.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            world as u64,
                            val_e88,
                            val_168,
                            val_260,
                            val_ebc
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `r_14059c1d0`
/// Original: 0x14059c1d0 (583 bytes, Throws::No)
pub unsafe extern "win64" fn r_14059c1d0(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let mut ctx9 = [0u64; 9];
        let _ = exe::call_va!(
            0x1_4059_6760,
            (ctx9.as_mut_ptr() as usize, param_1) -> usize
        );
        let world_plus_40 = rd::<usize>(param_1).wrapping_add(0x40);
        let end_ptr = cur_ptr.wrapping_add(count * 2);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_fe8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_afe8) as usize) as u32;
                let arch_off = arch.wrapping_shl(5);
                let search_len =
                    rd::<u32>(world_plus_40.wrapping_add(arch_off).wrapping_add(0x1806c));
                let search_ptr =
                    rd::<u64>(world_plus_40.wrapping_add(arch_off).wrapping_add(0x18058));

                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_fe8) -> i32);
                let off_slot = world_plus_40.wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5));
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_358 = rd::<u16>(exe::ptr::<u16>(0x1_43cb_5358) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_358) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                if off1 != 0 {
                    off1 = off1.wrapping_add(block);
                }
                if off2 != 0 {
                    off2 = off2.wrapping_add(block);
                }

                for idx in 0..ent_count {
                    let mut pkt: [u64; 4] = [off1 as u64, off2 as u64, block as u64, idx];
                    let mut arg2: [u64; 3] = [ctx9[6], ctx9[7], ctx9[8]];
                    let mut arg3: [u64; 3] = [ctx9[3], ctx9[4], ctx9[5]];
                    exe::call_va!(
                        0x1_41ac_b3c0,
                        (
                            pkt.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            arg3.as_mut_ptr() as usize,
                            ctx9[2],
                            ctx9[1],
                            ctx9[0]
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `anonymous_namespace_140f4ef30`
/// Original: 0x140f4ef30 (488 bytes, Throws::Caught)
pub unsafe extern "win64" fn anonymous_namespace_140f4ef30(
    param_1: usize,
    param_2: usize,
    param_3: u16,
) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let world = rd::<usize>(param_1);
        let arr_08 = rd::<usize>(world + 0x08);
        let v0 = rd::<usize>(world);
        let lut = rd::<usize>(v0 + 0x20);

        let id_804 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0804) as usize) as usize;
        let idx_804 = rd::<u16>(lut.wrapping_add(id_804 * 2)) as usize;
        let entry_804 = arr_08.wrapping_add(idx_804 * 16);
        let len_804 = rd::<u32>(entry_804 + 0x08) as u64;
        let ptr_804 = rd::<u64>(entry_804);

        let id_eb4 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ceb4) as usize) as usize;
        let idx_eb4 = rd::<u16>(lut.wrapping_add(id_eb4 * 2)) as usize;
        let entry_eb4 = arr_08.wrapping_add(idx_eb4 * 16);
        let len_eb4 = rd::<u32>(entry_eb4 + 0x08) as u64;
        let ptr_eb4 = rd::<u64>(entry_eb4);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_338 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0338) as usize) as u32;
                let arch_off = arch.wrapping_shl(5);
                let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
                let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
                let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_338) -> i32);
                let mut comp_off = if (found as isize) != -1 {
                    let off_tbl = rd::<usize>(
                        world
                            .wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5))
                            .wrapping_add(0x40),
                    );
                    rd::<u32>(off_tbl.wrapping_add((found as isize as usize) * 4)) as usize
                } else {
                    0
                };
                if comp_off != 0 {
                    comp_off = block.wrapping_add(comp_off);
                }

                for idx in 0..ent_count {
                    let mut arg_68: [u64; 3] = [world as u64, len_eb4, ptr_eb4];
                    let mut arg_48: [u64; 3] = [world as u64, len_804, ptr_804];
                    let mut arg_88: [u64; 3] = [comp_off as u64, block as u64, idx];
                    let mut world_slot: u64 = world as u64;
                    let u9 = exe::call_va!(
                        0x1_4025_15f0,
                        (arg_88.as_mut_ptr() as usize) -> u64
                    );
                    exe::call_va!(
                        0x1_40f4_c9d0,
                        (
                            u9,
                            arg_48.as_mut_ptr() as usize,
                            arg_68.as_mut_ptr() as usize,
                            (&mut world_slot as *mut u64) as usize
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_140f73040`
/// Original: 0x140f73040 (390 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f73040(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let world = rd::<usize>(param_1);
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let arr_08 = rd::<usize>(world + 0x08);
        let v0 = rd::<usize>(world);
        let lut = rd::<usize>(v0 + 0x20);

        let id_388 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_7388) as usize) as usize;
        let idx_388 = rd::<u16>(lut.wrapping_add(id_388 * 2)) as usize;
        let entry_388 = arr_08.wrapping_add(idx_388 * 16);
        let len_388 = rd::<u32>(entry_388 + 0x08) as u64;
        let ptr_388 = rd::<u64>(entry_388);

        let id_d08 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_6d08) as usize) as usize;
        let idx_d08 = rd::<u16>(lut.wrapping_add(id_d08 * 2)) as usize;
        let entry_d08 = arr_08.wrapping_add(idx_d08 * 16);
        let len_d08 = rd::<u32>(entry_d08 + 0x08) as u64;
        let ptr_d08 = rd::<u64>(entry_d08);

        let id_eec = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ceec) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_eec = rd::<u64>(tbl_580e0.wrapping_add(id_eec * 8));

        let mut pkt = [0u64; 8];
        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr);
            let block = rd::<usize>(world.wrapping_add((arch as usize) * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add((arch as usize) * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                pkt[0] = (pkt[0] & !0xffffu64) | (arch as u64);
                pkt[1] = block as u64;
                pkt[2] = world as u64;
                pkt[3] = ent_count;
                let mut tmp_out = [0u64; 4];
                let out_ptr = exe::call_va!(
                    0x1_40f7_29f0,
                    (pkt.as_mut_ptr() as usize, tmp_out.as_mut_ptr() as usize) -> usize
                );
                let offs = rd::<[u64; 4]>(out_ptr);
                pkt[4..8].copy_from_slice(&offs);

                let mut arg2: [u64; 3] = [world as u64, len_388, ptr_388];
                let mut arg3: [u64; 3] = [world as u64, len_d08, ptr_d08];
                exe::call_va!(
                    0x1_41e1_c970,
                    (
                        pkt.as_mut_ptr() as usize,
                        arg2.as_mut_ptr() as usize,
                        arg3.as_mut_ptr() as usize,
                        val_eec
                    )
                );
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_1410f0d90`
/// Original: 0x1410f0d90 (747 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1410f0d90(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u32,
) {
    let mut stream_ctx = [0u64; 0x370 / 8];
    let base = stream_ctx.as_mut_ptr() as usize;

    let _ = exe::call_va!(0x1_4183_acc2, (base, 0u32, 0x310u64) -> usize);
    wr::<usize>(base, base + 0x10);
    wr::<u32>(base + 0x0c, 0x10);
    wr::<u32>(base + 0x318, 0);
    wr::<usize>(base + 0x320, param_2);
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

    wr::<usize>(base + 0x338, param_2);
    let inner_stream = rd::<u64>(param_2 + 0x08);
    wr::<u64>(base + 0x340, inner_stream);
    wr::<usize>(base + 0x348, exe::ptr::<u8>(0x1_4307_b558) as usize);
    wr::<u64>(base + 0x350, 0);
    wr::<usize>(base + 0x358, exe::ptr::<u8>(0x1_4340_e9e0) as usize);
    wr::<u64>(base + 0x360, inner_stream);
    wr::<u16>(base + 0x368, 0x0100);

    let type_info = exe::call_va!(0x1_424b_14c0, (param_4) -> usize);
    let first_key_out = rd::<usize>(param_1 + 0x20);
    if first_key_out != 0 && rd::<u64>(param_3) != 0 {
        let keys_arr = rd::<usize>(param_3 + 0x08);
        wr::<u64>(first_key_out, rd::<u64>(keys_arr));
    }

    let mut cur_key = rd::<usize>(param_3 + 0x08);
    let key_count = rd::<usize>(param_3);
    let end_key = cur_key.wrapping_add(key_count * 8);

    while cur_key != end_key {
        let key_val = rd::<u64>(cur_key);
        let p_off = rd::<usize>(param_1 + 0x10);
        let p_base = rd::<usize>(param_1);
        let sum = rd::<u64>(p_off).wrapping_add(rd::<u64>(p_base));

        let align = exe::call_va!(rd::<usize>(rd::<usize>(type_info) + 0x18), (type_info) -> u64);
        let aligned = sum.wrapping_sub(1).wrapping_add(align) & align.wrapping_neg();
        let new_off = aligned.wrapping_sub(rd::<u64>(rd::<usize>(param_1)));
        wr::<u64>(rd::<usize>(param_1 + 0x10), new_off);

        let obj_ptr = exe::call_va!(
            rd::<usize>(rd::<usize>(type_info) + 0x60),
            (type_info, aligned) -> u64
        );
        let obj_size =
            exe::call_va!(rd::<usize>(rd::<usize>(type_info) + 0x10), (type_info) -> u64);
        let p_off2 = rd::<usize>(param_1 + 0x10);
        wr::<u64>(p_off2, rd::<u64>(p_off2).wrapping_add(obj_size));

        exe::call_va!(
            rd::<usize>(rd::<usize>(type_info) + 0x190),
            (type_info, obj_ptr, base, 0u32)
        );

        let map = rd::<usize>(param_1 + 0x18);
        let mut kv: [u64; 2] = [key_val, obj_ptr];
        exe::call_va!(0x1_40fb_efa0, (map));
        let mut probe: [u64; 2] = [0; 2];
        exe::call_va!(
            0x1_401f_a730,
            (map, probe.as_mut_ptr() as usize, kv.as_mut_ptr() as usize)
        );

        if probe[0] == u64::MAX {
            let used = rd::<u64>(map + 0x38);
            let deleted = rd::<u64>(map + 0x30);
            if used.wrapping_sub(deleted) >= 0x0fff_ffff_ffff_ffff {
                let mut exc = [0u8; 0x20];
                let s_ovf = exe::ptr::<u8>(0x1_4307_aa70) as usize;
                exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, s_ovf));
                let ti = exe::ptr::<u8>(0x1_437f_cf88) as usize;
                exe::call_va!(0x1_4183_acce, (exc.as_mut_ptr() as usize, ti));
                return;
            }
            let slot_idx = probe[1] as usize;
            let buckets = rd::<usize>(map + 0x58);
            let is_tombstone = deleted != 0
                && rd::<u64>(buckets.wrapping_add(slot_idx * 16)) == rd::<u64>(map + 0x28);
            if is_tombstone {
                wr::<u64>(map + 0x30, deleted.wrapping_sub(1));
            } else {
                wr::<u64>(map + 0x38, used.wrapping_add(1));
            }
            let dst = buckets.wrapping_add(slot_idx * 16);
            wr::<u64>(dst, kv[0]);
            wr::<u64>(dst + 0x08, kv[1]);
        }

        cur_key = cur_key.wrapping_add(8);
    }

    wr::<usize>(base + 0x348, exe::ptr::<u8>(0x1_4307_b558) as usize);
    exe::call_va!(0x1_401e_e280, (base));
}

/// `unknown_141112180`
/// Original: 0x141112180 (480 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141112180(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let world = rd::<usize>(param_1);
        let id_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_0d8 = rd::<u64>(tbl_580e0.wrapping_add(id_0d8 * 8));

        let mut pkt = [0u64; 6];
        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr);
            let block = rd::<usize>(world.wrapping_add((arch as usize) * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add((arch as usize) * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                pkt[0] = (pkt[0] & !0xffffu64) | (arch as u64);
                pkt[1] = block as u64;
                pkt[2] = world as u64;
                pkt[3] = ent_count;
                let mut tmp_out = [0u64; 4];
                let out_ptr = exe::call_va!(
                    0x1_4111_1640,
                    (pkt.as_mut_ptr() as usize, tmp_out.as_mut_ptr() as usize) -> usize
                );
                let offs = rd::<[u64; 4]>(out_ptr);
                for idx in 0..ent_count {
                    pkt[0] = offs[0];
                    pkt[1] = offs[1];
                    pkt[2] = offs[2];
                    pkt[3] = offs[3];
                    pkt[4] = block as u64;
                    pkt[5] = idx;
                    exe::call_va!(
                        0x1_41f9_1bd0,
                        (pkt.as_mut_ptr() as usize, world as u64, val_0d8)
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `physics_14123e200`
/// Original: 0x14123e200 (168 bytes, Throws::No)
pub unsafe extern "win64" fn physics_14123e200(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) -> usize {
    let mut dst = param_1;
    let mut rem = param_2;
    if rem != 0 {
        if rd::<u32>(param_3) == 0 {
            let byte_len = rem.wrapping_mul(4);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, byte_len) -> usize);
            return dst.wrapping_add(byte_len as usize);
        }
        if rem >= 4 {
            let mut groups = ((rem - 4) >> 2).wrapping_add(1);
            rem = rem.wrapping_sub(groups.wrapping_mul(4));
            while groups != 0 {
                wr::<u32>(dst, rd::<u32>(param_3));
                wr::<u32>(dst + 0x04, rd::<u32>(param_3));
                wr::<u32>(dst + 0x08, rd::<u32>(param_3));
                wr::<u32>(dst + 0x0c, rd::<u32>(param_3));
                dst = dst.wrapping_add(0x10);
                groups -= 1;
            }
            if rem == 0 {
                return dst;
            }
        }
        while rem != 0 {
            wr::<u32>(dst, rd::<u32>(param_3));
            dst = dst.wrapping_add(4);
            rem -= 1;
        }
    }
    dst
}

/// `unknown_14139f3b0`
/// Original: 0x14139f3b0 (360 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14139f3b0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> u8 {
    if rd::<u8>(param_2 + 0x0b) == 0 {
        if rd::<u8>(param_2 + 0x0a) == 1 {
            let rdx_ptr = rd::<usize>(param_2);
            while rd::<u8>(rdx_ptr + 0x04) == 3 {
                let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((rdx_ptr + 0x04) as u64, 3, 4);
                if ok {
                    let _ = crate::simd_ops::atomic_xadd_u32(rdx_ptr as u64, 0xffff_0001);
                }
            }
            loop {
                let rcx = rd::<usize>(param_2);
                if rcx == 0 {
                    break;
                }
                if (rd::<u16>(rcx + 0x06) >> 3) != rd::<u16>(param_2 + 0x08) {
                    break;
                }
                if crate::simd_ops::rd_acq_u32(rcx) == 0 {
                    break;
                }
                let rcx2 = rd::<usize>(param_2);
                if crate::simd_ops::rd_acq_u32(rcx2) == 1 {
                    exe::call_va!(0x1_4248_6230, (rd::<u64>(param_1 + 0x10), rcx2 as u64));
                } else {
                    let g_2f0 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
                    let idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                    let arg_rcx = rd::<u64>(g_2f0.wrapping_add(idx * 8).wrapping_add(0x2d8));
                    exe::call_va!(0x1_4247_5db0, (arg_rcx, 0xffff_ffffu32));
                    let rsi = rd::<usize>(param_3 + 0x10);
                    let task = exe::call_va!(0x1_4248_6380, (rd::<u64>(rsi + 0x10), 1u8) -> u64);
                    if task != 0 {
                        exe::call_va!(0x1_4248_62c0, (rd::<u64>(rsi + 0x10), task));
                    }
                }
            }
        } else {
            loop {
                let rcx = rd::<usize>(param_2);
                if rcx == 0 {
                    break;
                }
                if (rd::<u16>(rcx + 0x06) >> 3) != rd::<u16>(param_2 + 0x08) {
                    break;
                }
                if crate::simd_ops::rd_acq_u32(rcx) == 0 {
                    break;
                }
                let g_2f0 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
                let idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                let arg_rcx = rd::<u64>(g_2f0.wrapping_add(idx * 8).wrapping_add(0x2d8));
                exe::call_va!(0x1_4247_5db0, (arg_rcx, 0xffff_ffffu32));
                let rsi = rd::<usize>(param_3 + 0x10);
                let task = exe::call_va!(0x1_4248_6380, (rd::<u64>(rsi + 0x10), 1u8) -> u64);
                if task != 0 {
                    exe::call_va!(0x1_4248_62c0, (rd::<u64>(rsi + 0x10), task));
                }
            }
        }
    }
    wr::<u16>(param_2 + 0x0b, 0x0101);
    1
}

/// `ui_141412320`
/// Original: 0x141412320 (332 bytes, Throws::Discard)
pub unsafe extern "win64" fn ui_141412320(param_1: usize, param_2: u64) {
    let idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let tbl = exe::ptr::<usize>(0x1_43c4_aa00) as usize;
    let slot = rd::<usize>(tbl.wrapping_add(idx * 8));
    if slot != 0 && rd::<u32>(slot) == 2 {
        exe::call_va!(0x1_424f_2ba0, ());
        return;
    }

    let ring = rd::<usize>(param_1 + 0x168);
    exe::call_iat!(0x1_4264_55a8, (ring + 0x18));

    let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
    let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
    let mut next_wr = wr_ptr.wrapping_add(0x20);
    let ring_end = rd::<usize>(ring + 0x10);
    if next_wr > ring_end {
        while wr_ptr <= rd_ptr
            && rd_ptr < next_wr
            && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        }
        if wr_ptr <= ring_end.wrapping_sub(8) {
            crate::simd_ops::wr_rel_u64(wr_ptr, 0x00ff_ffff_ffff);
        }
        wr_ptr = rd::<usize>(ring + 0x08);
        next_wr = wr_ptr.wrapping_add(0x20);
    }
    while wr_ptr <= rd_ptr
        && rd_ptr < next_wr
        && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
    {
        core::hint::spin_loop();
        rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
    }

    crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
    crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
    exe::call_iat!(0x1_4264_5598, (ring + 0x18));

    let fn_ptr = exe::ptr::<u8>(0x1_424f_5450) as usize as u64;
    wr::<u32>(wr_ptr + 0x08, 0);
    wr::<u64>(wr_ptr + 0x0c, fn_ptr);
    wr::<u64>(wr_ptr + 0x14, param_2);

    let ring2 = rd::<usize>(param_1 + 0x168);
    crate::simd_ops::wr_rel_u64(wr_ptr, 0x14);
    let _ = crate::simd_ops::atomic_xadd_u32((ring2 + 0x30) as u64, 1);
    if rd::<u8>(param_1 + 0x198) != 0 {
        let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(param_1 + 0x1a0), 1u32, 0usize) -> i32);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_app_140209da0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_a020(tbl: usize, hash: u64) -> u64 {
        let mask = rd::<u64>(tbl + 0x18);
        let idx = ((hash >> 7) + 3) & mask;
        let ctrl = rd::<usize>(tbl);
        wr::<u8>(ctrl.wrapping_add(idx as usize), (hash as u8) & 0x7f);
        idx
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_a020, mock_a020 as *const ())?;

    let run = |f: F, key: u64, prefill: bool| -> (usize, usize, u8, [u64; 64], [u8; 48]) {
        let mut ctrl = [0x80u8; 48];
        let mut slots = [0u64; 64]; // 32 pairs of (key, val)
        let mask: u64 = 31;

        if prefill {
            // Compute hash of key to place a matching h2 with different key, and real key at +1
            let key_bytes = key.to_le_bytes();
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            for &b in &key_bytes {
                h ^= b as u64;
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
            let prod = (h as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u64 as u128);
            let r15 = ((prod >> 64) as u64).wrapping_add(prod as u64);
            let pos0 = ((r15 >> 7) & mask) as usize;
            let pos1 = ((pos0 + 1) as u64 & mask) as usize;
            let h2 = (r15 as u8) & 0x7f;

            ctrl[pos0] = h2;
            slots[pos0 * 2] = key ^ 1;
            slots[pos0 * 2 + 1] = 0x1111;

            ctrl[pos0 + 1] = h2;
            slots[pos1 * 2] = key;
            slots[pos1 * 2 + 1] = 0x2222;
        }

        let mut tbl = [0u64; 4];
        let tp = tbl.as_mut_ptr() as usize;
        wr::<usize>(tp, ctrl.as_mut_ptr() as usize);
        wr::<usize>(tp + 0x08, slots.as_mut_ptr() as usize);
        wr::<u64>(tp + 0x18, mask);

        let mut out = [0u64; 3];
        let op = out.as_mut_ptr() as usize;
        let ret = f(tp, op, (&key as *const u64) as usize);
        assert_eq!(ret, op);

        let ctrl_off = rd::<usize>(op).wrapping_sub(ctrl.as_ptr() as usize);
        let slot_off = rd::<usize>(op + 0x08).wrapping_sub(slots.as_ptr() as usize);
        let flag = rd::<u8>(op + 0x10);
        (ctrl_off, slot_off, flag, slots, ctrl)
    };

    for (k, prefill) in [(0x1234_5678_9abc_def0u64, true), (0xdead_beef_cafe_babeu64, false)] {
        let o = run(f_o, k, prefill);
        let n = run(f_n, k, prefill);
        crate::diff_eq!(o, n, "140209da0 out");
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140217680(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut entries = [0u8; 0x48 * 4];
    let ep = entries.as_mut_ptr() as usize;

    // Entry 0: b3 = 4 (< 0x80 => cap_bits = 32), len = 3 (len_bits = 24 <= 32) -> inline (entry + 8)
    wr::<u32>(ep, (4u32 << 24) | 3);
    wr::<usize>(ep + 0x08, 0xaaaa_1111);

    // Entry 1: b3 = 4 (< 0x80 => cap_bits = 32), len = 5 (len_bits = 40 > 32) -> heap ([entry + 8])
    let e1 = ep + 0x48;
    wr::<u32>(e1, (4u32 << 24) | 5);
    wr::<usize>(e1 + 0x08, 0xbbbb_2222);

    // Entry 2: b3 = 0x80 (>= 0x80 => cap_bits = 0x8000 - 0x7c00 = 0x400), len = 0x80 (len_bits = 0x400 <= 0x400) -> inline
    let e2 = ep + 0x90;
    wr::<u32>(e2, (0x80u32 << 24) | 0x80);
    wr::<usize>(e2 + 0x08, 0xcccc_3333);

    // Entry 3: b3 = 0x80 (>= 0x80 => cap_bits = 0x400), len = 0x81 (len_bits = 0x408 > 0x400) -> heap
    let e3 = ep + 0xd8;
    wr::<u32>(e3, (0x80u32 << 24) | 0x81);
    wr::<usize>(e3 + 0x08, 0xdddd_4444);

    let mut p2 = [0u8; 0x90];
    let p2_ptr = p2.as_mut_ptr() as usize;
    wr::<usize>(p2_ptr + 0x80, ep);

    for (tag, idx) in [(4u16, 0usize), (1, 0), (1, 1), (1, 2), (1, 3)] {
        let mut p1 = [0u8; 0x10];
        let p1_ptr = p1.as_mut_ptr() as usize;
        wr::<u16>(p1_ptr, tag);
        wr::<usize>(p1_ptr + 0x08, idx);
        let o = f_o(p1_ptr, p2_ptr);
        let n = f_n(p1_ptr, p2_ptr);
        crate::diff_eq!(o, n, "140217680 tag={} idx={}", tag, idx);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_140227390(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_CALLS: u32 = 0;
    static mut NEXT_CAP: u64 = 4;
    static mut BUCKET_BUF: [u64; 16] = [0; 16];

    unsafe extern "win64" fn mock_2020(_p1: usize, _diff: u64) -> u64 {
        unsafe { NEXT_CAP }
    }
    unsafe extern "win64" fn mock_5e80(_unused: usize, _cap: u64) -> usize {
        unsafe {
            ALLOC_CALLS += 1;
            core::ptr::addr_of_mut!(BUCKET_BUF) as usize
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_2020, mock_2020 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4021_5e80, mock_5e80 as *const ())?;

    let run = |f: F, reuse_existing: bool| -> ([u64; 12], [u64; 16], u32) {
        unsafe {
            ALLOC_CALLS = 0;
            NEXT_CAP = 4;
            BUCKET_BUF = [0; 16];
        }
        let mut p1 = [0u64; 12];
        let p1_ptr = p1.as_mut_ptr() as usize;
        wr::<f32>(p1_ptr + 0x10, 0.75);
        wr::<f32>(p1_ptr + 0x14, 0.25);
        wr::<u8>(p1_ptr + 0x18, 1);
        wr::<u32>(p1_ptr + 0x1c, 10);
        wr::<u64>(p1_ptr + 0x48, u64::MAX); // empty sentinel
        wr::<u64>(p1_ptr + 0x50, 0x9999);
        if reuse_existing {
            wr::<u64>(p1_ptr + 0x40, 4);
            wr::<usize>(
                p1_ptr + 0x58,
                core::ptr::addr_of_mut!(BUCKET_BUF) as usize,
            );
        }

        // Source buckets: 4 entries (empty, deleted, valid1, valid2)
        let src_entries: [u64; 8] = [
            u64::MAX,
            0,
            u64::MAX - 1,
            0,
            0x1111_2222_3333_4444,
            0xaaaa,
            0x5555_6666_7777_8888,
            0xbbbb,
        ];
        let mut p2 = [0u64; 12];
        let p2_ptr = p2.as_mut_ptr() as usize;
        wr::<u64>(p2_ptr + 0x28, u64::MAX - 1); // deleted key
        wr::<u64>(p2_ptr + 0x30, 1); // num_deleted = 1
        wr::<u64>(p2_ptr + 0x38, 3); // num_used = 3
        wr::<u64>(p2_ptr + 0x40, 4); // capacity = 4
        wr::<u64>(p2_ptr + 0x48, u64::MAX); // empty key
        wr::<usize>(p2_ptr + 0x58, src_entries.as_ptr() as usize);

        f(p1_ptr, p2_ptr);
        p1[11] = 0; // clear pointer field at +0x58 for comparison
        (p1, unsafe { BUCKET_BUF }, unsafe { ALLOC_CALLS })
    };

    for reuse in [false, true] {
        let o = run(f_o, reuse);
        let n = run(f_n, reuse);
        crate::diff_eq!(o, n, "140227390 reuse={}", reuse);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14024e6a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x8880 && n == 3 && want == 19 {
            1
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_2bd0(
        a1_ptr: usize,
        a2_ptr: usize,
        r8_world: usize,
        r9_e88: u64,
        arg5_168: u64,
        arg6_260: u64,
        arg7_ebc: u64,
    ) {
        let a1 = rd::<[u64; 3]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        a2[0] = (a2[0] as usize).wrapping_sub(r8_world) as u64;
        let idx = a1[2] as usize;
        let dst = r8_world + 0x200 + idx * (10 * 8);
        wr::<[u64; 3]>(dst, a1);
        wr::<[u64; 3]>(dst + 24, a2);
        wr::<[u64; 4]>(dst + 48, [r9_e88, arg5_168, arg6_260, arg7_ebc]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4189_2bd0, mock_2bd0 as *const ())?;

    let g_ed4 = exe::ptr::<u16>(0x1_43c8_ced4) as usize;
    let g_e88 = exe::ptr::<u16>(0x1_43c8_ce88) as usize;
    let g_168 = exe::ptr::<u16>(0x1_43c9_0168) as usize;
    let g_260 = exe::ptr::<u16>(0x1_43c9_0260) as usize;
    let g_ebc = exe::ptr::<u16>(0x1_43c8_cebc) as usize;
    let g_198 = exe::ptr::<u16>(0x1_43c9_0198) as usize;
    let saved = (
        rd::<u16>(g_ed4),
        rd::<u16>(g_e88),
        rd::<u16>(g_168),
        rd::<u16>(g_260),
        rd::<u16>(g_ebc),
        rd::<u16>(g_198),
    );
    wr::<u16>(g_ed4, 1);
    wr::<u16>(g_e88, 0);
    wr::<u16>(g_168, 1);
    wr::<u16>(g_260, 2);
    wr::<u16>(g_ebc, 3);
    wr::<u16>(g_198, 19);

    let run = |f: F| -> [u64; 20] {
        let lut: [u16; 4] = [0, 1, 0, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0x1234_5678);
        wr::<u32>(ap + 0x18, 7);

        let tbl_580e0: [u64; 4] = [0xa001, 0xb002, 0xc003, 0xd004];
        let off_tbl: [u32; 2] = [0x10, 0x340];

        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 1 * 8, 0x50_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x18098 + 0x20, 0x8880);
        wr::<u32>(wp + 0x180ac + 0x20, 3);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 20]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_ed4, saved.0);
    wr::<u16>(g_e88, saved.1);
    wr::<u16>(g_168, saved.2);
    wr::<u16>(g_260, saved.3);
    wr::<u16>(g_ebc, saved.4);
    wr::<u16>(g_198, saved.5);

    crate::diff_eq!(o, n, "14024e6a0 out");
    Ok(1)
}

pub unsafe fn diff_r_14059c1d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_6760(out: usize, p1: usize) -> usize {
        let world = rd::<usize>(p1) as u64;
        wr::<u64>(out, world);
        for i in 1..9usize {
            wr::<u64>(out + i * 8, 0x6000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x4440 && n == 5 {
            match want {
                11 => 0,
                22 => 1,
                _ => -1,
            }
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_b3c0(
        pkt_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        a4: u64,
        a5: u64,
        a6: u64,
    ) {
        let pkt = rd::<[u64; 4]>(pkt_ptr);
        let a2 = rd::<[u64; 3]>(a2_ptr);
        let a3 = rd::<[u64; 3]>(a3_ptr);
        let world = a6 as usize;
        let idx = pkt[3] as usize;
        let dst = world + 0x200 + idx * (12 * 8);
        wr::<[u64; 4]>(dst, pkt);
        wr::<[u64; 3]>(dst + 32, a2);
        wr::<[u64; 3]>(dst + 56, a3);
        wr::<[u64; 2]>(dst + 80, [a4, a5]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4059_6760, mock_6760 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41ac_b3c0, mock_b3c0 as *const ())?;

    let g_fe8 = exe::ptr::<u16>(0x1_43c9_afe8) as usize;
    let g_358 = exe::ptr::<u16>(0x1_43cb_5358) as usize;
    let s_fe8 = rd::<u16>(g_fe8);
    let s_358 = rd::<u16>(g_358);
    wr::<u16>(g_fe8, 11);
    wr::<u16>(g_358, 22);

    let run = |f: F| -> [u64; 24] {
        let off_tbl: [u32; 2] = [0x100, 0x280];
        let mut world = vec![0u8; 0x30200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x60_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x40 + 0x20 + 0x18058, 0x4440);
        wr::<u32>(wp + 0x40 + 0x20 + 0x1806c, 5);
        wr::<usize>(wp + 0x40 + (1 + 0xc03) * 0x20, off_tbl.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 24]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_fe8, s_fe8);
    wr::<u16>(g_358, s_358);

    crate::diff_eq!(o, n, "14059c1d0 out");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_140f4ef30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x9990 && n == 4 && want == 33 {
            1
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_15f0(arg_ptr: usize) -> u64 {
        let a = rd::<[u64; 3]>(arg_ptr);
        a[0].wrapping_add(a[1]).wrapping_add(a[2] * 0x10)
    }
    unsafe extern "win64" fn mock_c9d0(
        rcx_val: u64,
        rdx_48: usize,
        r8_68: usize,
        r9_world_ptr: usize,
    ) {
        let world = rd::<usize>(r9_world_ptr);
        let mut a48 = rd::<[u64; 3]>(rdx_48);
        let mut a68 = rd::<[u64; 3]>(r8_68);
        a48[0] = (a48[0] as usize).wrapping_sub(world) as u64;
        a68[0] = (a68[0] as usize).wrapping_sub(world) as u64;
        let call_idx = rd::<u64>(world + 0x1f0) as usize;
        wr::<u64>(world + 0x1f0, (call_idx + 1) as u64);
        let dst = world + 0x200 + call_idx * (7 * 8);
        wr::<u64>(dst, rcx_val);
        wr::<[u64; 3]>(dst + 8, a48);
        wr::<[u64; 3]>(dst + 32, a68);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4025_15f0, mock_15f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40f4_c9d0, mock_c9d0 as *const ())?;

    let g_804 = exe::ptr::<u16>(0x1_43c9_0804) as usize;
    let g_eb4 = exe::ptr::<u16>(0x1_43c8_ceb4) as usize;
    let g_338 = exe::ptr::<u16>(0x1_43c9_0338) as usize;
    let s_804 = rd::<u16>(g_804);
    let s_eb4 = rd::<u16>(g_eb4);
    let s_338 = rd::<u16>(g_338);
    wr::<u16>(g_804, 1);
    wr::<u16>(g_eb4, 2);
    wr::<u16>(g_338, 33);

    let run = |f: F| -> [u64; 14] {
        let lut: [u16; 4] = [0, 1, 2, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x40];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0xaaaa_1111);
        wr::<u32>(ap + 0x18, 5);
        wr::<u64>(ap + 0x20, 0xbbbb_2222);
        wr::<u32>(ap + 0x28, 8);

        let off_tbl: [u32; 2] = [0x10, 0x480];
        let mut world = vec![0u8; 0x30200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 1 * 8, 0x40_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x18098 + 0x20, 0x9990);
        wr::<u32>(wp + 0x180ac + 0x20, 4);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 14]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_804, s_804);
    wr::<u16>(g_eb4, s_eb4);
    wr::<u16>(g_338, s_338);

    crate::diff_eq!(o, n, "140f4ef30 out");
    Ok(1)
}

pub unsafe fn diff_unknown_140f73040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_29f0(in_ptr: usize, out_ptr: usize) -> usize {
        let arch = rd::<u16>(in_ptr) as u64;
        let block = rd::<u64>(in_ptr + 0x08);
        let cnt = rd::<u64>(in_ptr + 0x18);
        wr::<u64>(out_ptr, block + arch * 0x10);
        wr::<u64>(out_ptr + 0x08, block + cnt * 0x100);
        wr::<u64>(out_ptr + 0x10, 0x3333);
        wr::<u64>(out_ptr + 0x18, 0x4444);
        out_ptr
    }
    unsafe extern "win64" fn mock_c970(
        pkt_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        r9_eec: u64,
    ) {
        let mut pkt = rd::<[u64; 8]>(pkt_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let mut a3 = rd::<[u64; 3]>(a3_ptr);
        let world = pkt[2] as usize;
        pkt[0] &= 0xffff;
        pkt[2] = 0;
        a2[0] = (a2[0] as usize).wrapping_sub(world) as u64;
        a3[0] = (a3[0] as usize).wrapping_sub(world) as u64;
        let dst = world + 0x200;
        wr::<[u64; 8]>(dst, pkt);
        wr::<[u64; 3]>(dst + 64, a2);
        wr::<[u64; 3]>(dst + 88, a3);
        wr::<u64>(dst + 112, r9_eec);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f7_29f0, mock_29f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e1_c970, mock_c970 as *const ())?;

    let g_388 = exe::ptr::<u16>(0x1_43cc_7388) as usize;
    let g_d08 = exe::ptr::<u16>(0x1_43cc_6d08) as usize;
    let g_eec = exe::ptr::<u16>(0x1_43c8_ceec) as usize;
    let s_388 = rd::<u16>(g_388);
    let s_d08 = rd::<u16>(g_d08);
    let s_eec = rd::<u16>(g_eec);
    wr::<u16>(g_388, 1);
    wr::<u16>(g_d08, 2);
    wr::<u16>(g_eec, 0);

    let run = |f: F| -> [u64; 15] {
        let lut: [u16; 4] = [0, 1, 2, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x40];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0x1111_aaaa);
        wr::<u32>(ap + 0x18, 3);
        wr::<u64>(ap + 0x20, 0x2222_bbbb);
        wr::<u32>(ap + 0x28, 6);

        let tbl_580e0: [u64; 2] = [0xfeed_face_cafe, 0];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 2 * 8, 0x70_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 5);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 15]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_388, s_388);
    wr::<u16>(g_d08, s_d08);
    wr::<u16>(g_eec, s_eec);

    crate::diff_eq!(o, n, "140f73040 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1410f0d90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TI_VTBL: [usize; 52] = [0; 52];
    static mut TI_OBJ: [usize; 2] = [0; 2];
    static mut DESER_SUM: u64 = 0;
    static mut DTOR_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_get_pos(this: usize) -> u64 {
        rd::<u64>(this + 0x08)
    }
    unsafe extern "win64" fn mock_ti_size(_this: usize) -> u64 {
        24
    }
    unsafe extern "win64" fn mock_ti_align(_this: usize) -> u64 {
        16
    }
    unsafe extern "win64" fn mock_ti_create(this: usize, placement_ptr: u64) -> u64 {
        let next = rd::<u64>(this + 0x08).wrapping_add(placement_ptr);
        wr::<u64>(this + 0x08, next);
        next
    }
    unsafe extern "win64" fn mock_ti_deser(_this: usize, obj: u64, stream_ctx: usize, flag: u32) {
        let depth = rd::<u32>(stream_ctx + 0x08) as u64;
        let pos = rd::<u64>(stream_ctx + 0x28);
        let mode = rd::<u16>(stream_ctx + 0x368) as u64;
        unsafe {
            DESER_SUM = DESER_SUM
                .wrapping_add(obj)
                .wrapping_add(depth)
                .wrapping_add(pos)
                .wrapping_add(mode)
                .wrapping_add(flag as u64);
        }
    }
    unsafe extern "win64" fn mock_find_ti(_crc: u32) -> usize {
        core::ptr::addr_of_mut!(TI_OBJ) as usize
    }
    unsafe extern "win64" fn mock_efa0(map: usize) {
        wr::<u64>(map + 0x20, rd::<u64>(map + 0x20).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_a730(_map: usize, out_probe: usize, kv_ptr: usize) {
        let k = rd::<u64>(kv_ptr);
        match k {
            100 => {
                // Already exists
                wr::<u64>(out_probe, 0);
                wr::<u64>(out_probe + 0x08, 0);
            }
            200 => {
                // Insert into empty bucket 1
                wr::<u64>(out_probe, u64::MAX);
                wr::<u64>(out_probe + 0x08, 1);
            }
            _ => {
                // Insert into tombstone bucket 2
                wr::<u64>(out_probe, u64::MAX);
                wr::<u64>(out_probe + 0x08, 2);
            }
        }
    }
    unsafe extern "win64" fn mock_e280(_stream_ctx: usize) {
        unsafe { DTOR_CALLED += 1 };
    }

    let _m1 = crate::difftest::Mock::new(0x1_424b_14c0, mock_find_ti as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40fb_efa0, mock_efa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;

    let run = |f: F| -> (u64, u64, [u64; 12], [u64; 8], u64, u32) {
        unsafe {
            TI_VTBL = [0; 52];
            TI_VTBL[0x10 / 8] = mock_ti_size as *const () as usize;
            TI_VTBL[0x18 / 8] = mock_ti_align as *const () as usize;
            TI_VTBL[0x60 / 8] = mock_ti_create as *const () as usize;
            TI_VTBL[0x190 / 8] = mock_ti_deser as *const () as usize;
            TI_OBJ = [core::ptr::addr_of!(TI_VTBL) as usize, 0x5000];
            DESER_SUM = 0;
            DTOR_CALLED = 0;
        }

        let mut inner_vtbl = [0usize; 4];
        inner_vtbl[0x10 / 8] = mock_get_pos as *const () as usize;
        let inner_obj: [u64; 2] = [inner_vtbl.as_ptr() as u64, 0x80];
        let stream_obj: [u64; 2] = [0, inner_obj.as_ptr() as u64];

        let mut buckets = [0u64; 8];
        buckets[2 * 2] = 0xdead_0000; // slot 2 has tombstone key

        let mut map = [0u64; 12];
        let mp = map.as_mut_ptr() as usize;
        wr::<u64>(mp + 0x28, 0xdead_0000); // tombstone key
        wr::<u64>(mp + 0x30, 1); // deleted = 1
        wr::<u64>(mp + 0x38, 2); // used = 2
        wr::<usize>(mp + 0x58, buckets.as_mut_ptr() as usize);

        let mut base_val: u64 = 0x1000;
        let mut off_val: u64 = 3;
        let mut first_key_out: u64 = 0;
        let p1: [usize; 5] = [
            (&mut base_val as *mut u64) as usize,
            0,
            (&mut off_val as *mut u64) as usize,
            mp,
            (&mut first_key_out as *mut u64) as usize,
        ];

        let keys: [u64; 3] = [100, 200, 300];
        let p3: [u64; 2] = [3, keys.as_ptr() as u64];

        f(
            p1.as_ptr() as usize,
            stream_obj.as_ptr() as usize,
            p3.as_ptr() as usize,
            0x1234,
        );
        map[11] = 0; // clear pointer field at +0x58
        (
            off_val,
            first_key_out,
            map,
            buckets,
            unsafe { DESER_SUM },
            unsafe { DTOR_CALLED },
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1410f0d90 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141112180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_1640(in_ptr: usize, out_ptr: usize) -> usize {
        let arch = rd::<u16>(in_ptr) as u64;
        let block = rd::<u64>(in_ptr + 0x08);
        let cnt = rd::<u64>(in_ptr + 0x18);
        wr::<u64>(out_ptr, block + arch);
        wr::<u64>(out_ptr + 0x08, block + 0x100);
        wr::<u64>(out_ptr + 0x10, block + 0x200);
        wr::<u64>(out_ptr + 0x18, block + cnt * 0x10);
        out_ptr
    }
    unsafe extern "win64" fn mock_1bd0(pkt_ptr: usize, rdx_world: usize, r8_0d8: u64) {
        let pkt = rd::<[u64; 6]>(pkt_ptr);
        let idx = pkt[5] as usize;
        let dst = rdx_world + 0x200 + idx * (7 * 8);
        wr::<[u64; 6]>(dst, pkt);
        wr::<u64>(dst + 48, r8_0d8);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4111_1640, mock_1640 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f9_1bd0, mock_1bd0 as *const ())?;

    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let s_0d8 = rd::<u16>(g_0d8);
    wr::<u16>(g_0d8, 1);

    let run = |f: F| -> [u64; 14] {
        let tbl_580e0: [u64; 2] = [0, 0x9988_7766_5544];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x60_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 2);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 14]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_0d8, s_0d8);

    crate::diff_eq!(o, n, "141112180 out");
    Ok(1)
}

pub unsafe fn diff_physics_14123e200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u64, val: u32| -> ([u32; 12], usize) {
        let mut buf = [0xdead_beefu32; 12];
        let bp = buf.as_mut_ptr() as usize;
        let ret = f(bp, count, (&val as *const u32) as usize);
        (buf, ret.wrapping_sub(bp))
    };

    for (cnt, v) in [(0u64, 42u32), (5, 0), (3, 0x1234_5678), (4, 0x9999_0001), (7, 0xabcd_ef01)]
    {
        let o = run(f_o, cnt, v);
        let n = run(f_n, cnt, v);
        crate::diff_eq!(o, n, "14123e200 cnt={} v={:#x}", cnt, v);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14139f3b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TARGET_PTR: usize = 0;
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_6230(rcx: u64, rdx_node: usize) {
        unsafe { LOG_SUM = LOG_SUM.wrapping_add(rcx) };
        wr::<u32>(rdx_node, 0);
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_5db0(rcx: u64, rdx: u32) {
        unsafe { LOG_SUM = LOG_SUM.wrapping_add(rcx).wrapping_add(rdx as u64) };
    }
    unsafe extern "win64" fn mock_6380(rcx: u64, dl: u8) -> u64 {
        unsafe { LOG_SUM = LOG_SUM.wrapping_add(rcx).wrapping_add(dl as u64) };
        0x7777
    }
    unsafe extern "win64" fn mock_62c0(rcx: u64, rdx: u64) {
        unsafe {
            LOG_SUM = LOG_SUM.wrapping_add(rcx).wrapping_add(rdx);
            let tp = TARGET_PTR;
            let cur = rd::<u32>(tp);
            wr::<u32>(tp, cur.wrapping_sub(1));
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_6230, mock_6230 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4247_5db0, mock_5db0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_6380, mock_6380 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4248_62c0, mock_62c0 as *const ())?;

    let g_2f0 = exe::ptr::<usize>(0x1_43a0_82f0) as usize;
    let s_2f0 = rd::<usize>(g_2f0);

    let run = |f: F, mode_a: u8, already_done: u8| -> (u8, [u8; 16], [u8; 16], u64) {
        unsafe { LOG_SUM = 0 };

        let mut g_obj = [0u8; 0x300];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<u64>(gp + 0x2d8 + 8, 0xaaaa_0000);
        wr::<usize>(g_2f0, gp);

        let mut node = [0u8; 16];
        let np = node.as_mut_ptr() as usize;
        unsafe { TARGET_PTR = np };
        // If mode_a == 1, cmpxchg transitions byte +4 from 3 -> 4 and adds 0xffff0001 (-0xffff) to [node],
        // so starting with 0x10001 leaves 2, which then loops once via 62c0 (2 -> 1) and once via 6230 (1 -> 0).
        let init_cnt: u32 = if mode_a == 1 { 0x10001 } else { 1 };
        wr::<u32>(np, init_cnt);
        wr::<u8>(np + 0x04, 3);
        wr::<u16>(np + 0x06, 5 << 3);

        let mut p2 = [0u8; 16];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, np);
        wr::<u16>(p2p + 0x08, 5);
        wr::<u8>(p2p + 0x0a, mode_a);
        wr::<u8>(p2p + 0x0b, already_done);

        let p1: [u64; 3] = [0, 0, 0xbbbb_0000];
        let sub_p3: [u64; 3] = [0, 0, 0xcccc_0000];
        let p3: [usize; 3] = [0, 0, sub_p3.as_ptr() as usize];

        let ret = f(p1.as_ptr() as usize, p2p, p3.as_ptr() as usize);
        wr::<usize>(p2p, 0); // clear pointer for comparison
        (ret, node, p2, unsafe { LOG_SUM })
    };

    for (mode_a, done) in [(1u8, 0u8), (0, 0), (1, 1)] {
        let o = run(f_o, mode_a, done);
        let n = run(f_n, mode_a, done);
        wr::<usize>(g_2f0, s_2f0);
        crate::diff_eq!(o, n, "14139f3b0 mode_a={} done={}", mode_a, done);
    }
    Ok(3)
}

pub unsafe fn diff_ui_141412320(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FAST_PATH_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_2ba0() {
        unsafe { FAST_PATH_COUNT += 1 };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424f_2ba0, mock_2ba0 as *const ())?;

    let g_tbl = exe::ptr::<usize>(0x1_43c4_aa00) as usize;
    let s_slot0 = rd::<usize>(g_tbl);

    let run = |f: F, worker_mode: u32, wrap: bool| -> ([u32; 24], usize, u32, u32) {
        unsafe { FAST_PATH_COUNT = 0 };
        let worker_val: u32 = worker_mode;
        wr::<usize>(g_tbl, (&worker_val as *const u32) as usize);

        let mut ring_mem = [0u32; 24];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let init_wr = if wrap { rmp + 0x28 } else { rmp };
        let mut ring = [0u8; 0x40];
        let rp = ring.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x08, rmp);
        wr::<usize>(rp + 0x10, rmp + 0x38);
        wr::<u64>(rp + 0x18, 0);
        wr::<usize>(rp + 0x20, rmp);
        wr::<usize>(rp + 0x28, init_wr);
        wr::<u32>(rp + 0x30, 5);
        wr::<u32>(rp + 0x34, 5);

        let mut mgr = [0u8; 0x1b0];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x168, rp);
        wr::<u8>(mp + 0x198, 0);

        f(mp, 0x1234_5678_9abc_def0);
        (
            ring_mem,
            rd::<usize>(rp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rp + 0x30),
            unsafe { FAST_PATH_COUNT },
        )
    };

    for (mode, wrap) in [(2u32, false), (1, false), (1, true)] {
        let o = run(f_o, mode, wrap);
        let n = run(f_n, mode, wrap);
        wr::<usize>(g_tbl, s_slot0);
        crate::diff_eq!(o, n, "141412320 mode={} wrap={}", mode, wrap);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_4020_9da0, "app_140209da0", rend::cauldron_hot_o::app_140209da0 as *const (), rend::cauldron_hot_o::diff_app_140209da0),
// Replacement::verified(0x1_4021_7680, "unknown_140217680", rend::cauldron_hot_o::unknown_140217680 as *const (), rend::cauldron_hot_o::diff_unknown_140217680),
// Replacement::verified_throws(0x1_4022_7390, "unknown_140227390", rend::cauldron_hot_o::unknown_140227390 as *const (), Throws::Discard, rend::cauldron_hot_o::diff_unknown_140227390),
// Replacement::verified_throws(0x1_4024_e6a0, "unknown_14024e6a0", rend::cauldron_hot_o::unknown_14024e6a0 as *const (), Throws::Discard, rend::cauldron_hot_o::diff_unknown_14024e6a0),
// Replacement::verified(0x1_4059_c1d0, "r_14059c1d0", rend::cauldron_hot_o::r_14059c1d0 as *const (), rend::cauldron_hot_o::diff_r_14059c1d0),
// Replacement::verified_throws(0x1_40f4_ef30, "anonymous_namespace_140f4ef30", rend::cauldron_hot_o::anonymous_namespace_140f4ef30 as *const (), Throws::Caught, rend::cauldron_hot_o::diff_anonymous_namespace_140f4ef30),
// Replacement::verified(0x1_40f7_3040, "unknown_140f73040", rend::cauldron_hot_o::unknown_140f73040 as *const (), rend::cauldron_hot_o::diff_unknown_140f73040),
// Replacement::verified_throws(0x1_410f_0d90, "unknown_1410f0d90", rend::cauldron_hot_o::unknown_1410f0d90 as *const (), Throws::Caught, rend::cauldron_hot_o::diff_unknown_1410f0d90),
// Replacement::verified(0x1_4111_2180, "unknown_141112180", rend::cauldron_hot_o::unknown_141112180 as *const (), rend::cauldron_hot_o::diff_unknown_141112180),
// Replacement::verified(0x1_4123_e200, "physics_14123e200", rend::cauldron_hot_o::physics_14123e200 as *const (), rend::cauldron_hot_o::diff_physics_14123e200),
// Replacement::verified_throws(0x1_4139_f3b0, "unknown_14139f3b0", rend::cauldron_hot_o::unknown_14139f3b0 as *const (), Throws::Discard, rend::cauldron_hot_o::diff_unknown_14139f3b0),
// Replacement::verified_throws(0x1_4141_2320, "ui_141412320", rend::cauldron_hot_o::ui_141412320 as *const (), Throws::Discard, rend::cauldron_hot_o::diff_ui_141412320),
