//! Decompiled hot functions shard W2 (`0x141069350..0x1413383d0`).

use crate::exe::{self, rd, wr};

/// `unknown_141069350`
/// Original: 0x141069350 (1217 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141069350(param_1: usize) {
    let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    let neg = |v: f32| -> f32 { f32::from_bits(v.to_bits() ^ sign_mask) };

    let f1 = rd::<f32>(param_1 + 0x28);
    let f2 = rd::<f32>(param_1 + 0x24);
    let f3 = rd::<f32>(param_1 + 0x18);
    let f4 = rd::<f32>(param_1);
    let f5 = rd::<f32>(param_1 + 0x04);
    let f6 = rd::<f32>(param_1 + 0x08);
    let f7 = rd::<f32>(param_1 + 0x0c);
    let f8 = rd::<f32>(param_1 + 0x20);
    let f9 = rd::<f32>(param_1 + 0x10);
    let f10 = rd::<f32>(param_1 + 0x1c);
    let f11 = rd::<f32>(param_1 + 0x2c);
    let f12 = rd::<f32>(param_1 + 0x38);
    let f13 = rd::<f32>(param_1 + 0x3c);
    let f14 = rd::<f32>(param_1 + 0x34);
    let f15 = rd::<f32>(param_1 + 0x30);
    let f16 = rd::<f32>(param_1 + 0x14);

    let f17 = f13 * f1 - f12 * f11;
    let f18 = f12 * f2 - f14 * f1;
    let f19 = f13 * f2 - f14 * f11;
    wr::<f32>(param_1, (f17 * f16 - f19 * f3) + f18 * f10);

    let f20 = f13 * f8 - f15 * f11;
    let f21 = f12 * f8 - f15 * f1;
    let f22 = f14 * f8 - f15 * f2;
    wr::<f32>(param_1 + 0x10, neg((f17 * f9 - f20 * f3) + f21 * f10));
    wr::<f32>(param_1 + 0x20, (f19 * f9 - f20 * f16) + f22 * f10);
    wr::<f32>(param_1 + 0x30, neg((f18 * f9 - f21 * f16) + f22 * f3));
    wr::<f32>(param_1 + 0x04, neg((f17 * f5 - f19 * f6) + f18 * f7));
    wr::<f32>(param_1 + 0x14, (f17 * f4 - f20 * f6) + f21 * f7);
    wr::<f32>(param_1 + 0x24, neg((f19 * f4 - f20 * f5) + f22 * f7));
    wr::<f32>(param_1 + 0x34, (f18 * f4 - f21 * f5) + f22 * f6);

    let f17_b = f13 * f3 - f12 * f10;
    let f18_b = f12 * f16 - f14 * f3;
    let f19_b = f13 * f16 - f14 * f10;
    wr::<f32>(param_1 + 0x08, (f17_b * f5 - f19_b * f6) + f18_b * f7);

    let f13_b = f13 * f9 - f15 * f10;
    let f12_b = f12 * f9 - f15 * f3;
    let f14_b = f14 * f9 - f15 * f16;
    wr::<f32>(param_1 + 0x18, neg((f17_b * f4 - f13_b * f6) + f12_b * f7));
    wr::<f32>(param_1 + 0x28, (f19_b * f4 - f13_b * f5) + f14_b * f7);
    wr::<f32>(param_1 + 0x38, neg((f18_b * f4 - f12_b * f5) + f14_b * f6));

    let f12_c = f11 * f3 - f1 * f10;
    let f13_c = f11 * f16 - f2 * f10;
    let f14_c = f1 * f16 - f2 * f3;
    wr::<f32>(param_1 + 0x0c, neg((f12_c * f5 - f13_c * f6) + f14_c * f7));

    let f10_c = f11 * f9 - f8 * f10;
    let f1_c = f1 * f9 - f8 * f3;
    let f2_c = f2 * f9 - f8 * f16;
    wr::<f32>(param_1 + 0x1c, (f12_c * f4 - f10_c * f6) + f1_c * f7);
    wr::<f32>(param_1 + 0x2c, neg((f13_c * f4 - f10_c * f5) + f2_c * f7));
    wr::<f32>(param_1 + 0x3c, (f14_c * f4 - f1_c * f5) + f2_c * f6);
}

/// `unknown_1411642f0`
/// Original: 0x1411642f0 (364 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411642f0(
    _param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let count = rd::<u64>(param_2 + 0x18);
    if count != 0 {
        let mut pkt = [0u64; 15];
        for idx in 0..count {
            let body = rd::<[u64; 13]>(param_2 + 0x20);
            pkt[0..13].copy_from_slice(&body);
            pkt[13] = rd::<u64>(param_2 + 0x08);
            pkt[14] = idx;
            let a2 = rd::<u64>(param_3 + 0x18);
            let a3 = rd::<u64>(param_3 + 0x10);
            let a4 = rd::<u64>(param_3 + 0x08);
            let a5 = rd::<u64>(param_3);
            exe::call_va!(
                0x1_41fc_b9a0,
                (pkt.as_mut_ptr() as usize, a2, a3, a4, a5)
            );
        }
    }
}

/// `unknown_141171ad0`
/// Original: 0x141171ad0 (572 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141171ad0(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let mut ctx10 = [0u64; 10];
        let _ = exe::call_va!(
            0x1_4117_13a0,
            (ctx10.as_mut_ptr() as usize, param_1) -> usize
        );
        let world_plus_40 = rd::<usize>(param_1).wrapping_add(0x40);
        let end_ptr = cur_ptr.wrapping_add(count * 2);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr);
            let block = rd::<usize>(
                world_plus_40
                    .wrapping_add((arch as usize) * 8)
                    .wrapping_add(0x48),
            );
            let ent_count = if block != 0 {
                rd::<u32>(
                    world_plus_40
                        .wrapping_add((arch as usize) * 4)
                        .wrapping_add(0x10048),
                ) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let mut offs = [0u64; 6];
                let _ = exe::call_va!(
                    0x1_4117_1670,
                    (offs.as_mut_ptr() as usize, ent_count, world_plus_40, arch) -> usize
                );
                let b_u64 = block as u64;
                let off0 = if offs[0] != 0 { offs[0].wrapping_add(b_u64) } else { 0 };
                let off1 = if offs[1] != 0 { offs[1].wrapping_add(b_u64) } else { 0 };
                let off2 = if offs[2] != 0 { offs[2].wrapping_add(b_u64) } else { 0 };
                let off3 = if offs[3] != 0 { offs[3].wrapping_add(b_u64) } else { 0 };
                let off4 = if offs[4] != 0 { offs[4].wrapping_add(b_u64) } else { 0 };
                let off5 = if offs[5] != 0 { offs[5].wrapping_add(b_u64) } else { 0 };

                for idx in 0..ent_count {
                    let mut pkt: [u64; 8] = [off0, off1, off2, off3, off4, off5, b_u64, idx];
                    let mut arg2: [u64; 3] = [ctx10[7], ctx10[8], ctx10[9]];
                    let mut arg3: [u64; 3] = [ctx10[4], ctx10[5], ctx10[6]];
                    let mut arg4: [u64; 3] = [ctx10[1], ctx10[2], ctx10[3]];
                    exe::call_va!(
                        0x1_41fd_6020,
                        (
                            pkt.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            arg3.as_mut_ptr() as usize,
                            arg4.as_mut_ptr() as usize,
                            ctx10[0]
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `coregame_141195f80`
/// Original: 0x141195f80 (213 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_141195f80(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let block = rd::<usize>(world.wrapping_add((param_2 as usize) * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(
            world
                .wrapping_add((param_2 as usize) * 4)
                .wrapping_add(0x10088),
        ) as u64
    } else {
        0
    };
    if ent_count != 0 {
        let mut pkt = [0u64; 6];
        let _ = exe::call_va!(
            0x1_4119_6060,
            (
                pkt.as_mut_ptr() as usize,
                param_2,
                block as u64,
                ent_count,
                world as u64
            ) -> usize
        );
        let count = pkt[3];
        if count != 0 {
            let saved_block = pkt[1];
            let off0 = pkt[4];
            let off1 = pkt[5];
            for idx in 0..count {
                pkt[0] = off0;
                pkt[1] = off1;
                pkt[2] = saved_block;
                pkt[3] = idx;
                exe::call_va!(0x1_41fe_fe90, (pkt.as_mut_ptr() as usize));
            }
        }
    }
}

/// `unknown_1411a5000`
/// Original: 0x1411a5000 (205 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411a5000(param_1: usize, param_2: usize) -> usize {
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let indices_ptr = rd::<usize>(param_1);
    if count != 0 {
        let base_tbl = exe::ptr::<u8>(0x1_43a8_ae10) as usize;
        for i in 0..count {
            let idx = rd::<i32>(indices_ptr.wrapping_add(i * 4)) as isize;
            let entry = base_tbl.wrapping_add((idx * 0x88) as usize);
            let mut str_ptr = entry.wrapping_add(0x08);
            let b3 = rd::<u8>(entry + 0x03) as u32;
            let cap_bits = if b3 < 0x80 {
                b3.wrapping_mul(8)
            } else {
                (b3 << 8).wrapping_sub(0x7c00)
            };
            let len_bits = (rd::<u32>(entry) & 0x00ff_ffff) << 3;
            if len_bits > cap_bits {
                str_ptr = rd::<usize>(str_ptr);
            }
            let diff_off = param_2.wrapping_sub(str_ptr);
            let mut p = str_ptr;
            let cmp = loop {
                let c1 = rd::<u8>(p) as u32;
                let c2 = rd::<u8>(p.wrapping_add(diff_off)) as u32;
                let d = c1.wrapping_sub(c2);
                if d != 0 {
                    break d;
                }
                p = p.wrapping_add(1);
                if c2 == 0 {
                    break 0;
                }
            };
            if cmp == 0 {
                return rd::<usize>(param_1 + 0x10).wrapping_add(i * 0x18);
            }
        }
    }
    0
}

/// `unknown_1411f1520`
/// Original: 0x1411f1520 (124 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f1520(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> i32 {
    let mut base: u64 = 0;
    let mut rem = param_2;
    while rem != 0 {
        let half = rem >> 1;
        let next_base = base.wrapping_sub(half).wrapping_add(rem);
        let mid = half.wrapping_add(base);
        rem = half;
        if rd::<u64>(param_1.wrapping_add((mid as usize) * 8)) <= param_3 {
            base = next_base;
        }
    }
    let cand = base.wrapping_sub(1);
    let idx: i32 = if cand < param_2 { cand as i32 } else { 0 };
    let dummy: u64 = !param_3;
    let ptr = if param_2 != 0 {
        param_1
    } else {
        (&dummy as *const u64) as usize
    };
    if rd::<u64>(ptr.wrapping_add(((idx as isize) * 8) as usize)) == param_3 {
        idx
    } else {
        -1
    }
}

/// `unknown_1411f1c40`
/// Original: 0x1411f1c40 (87 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f1c40(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> u64 {
    let mut base: u64 = 0;
    let mut rem = param_2;
    while rem != 0 {
        let half = rem >> 1;
        let rest = rem.wrapping_sub(half);
        let mid = half.wrapping_add(base);
        rem = half;
        if rd::<u64>(param_1.wrapping_add((mid as usize) * 8)) < param_3 {
            base = mid.wrapping_add(1);
            rem = rest.wrapping_sub(1);
        }
    }
    base
}

/// `rend::SparseUploadBuffer<r::BoundSphere>::vfunc_1` (`rend_1412f8710`)
/// Original: 0x1412f8710 (701 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_1412f8710(param_1: usize) {
    let words_base = rd::<usize>(param_1 + 0xa8);
    let num_bits = rd::<u64>(param_1 + 0xb8);
    let word_cnt = num_bits.wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(param_1 + 0x90) {
                let vec_ptr = param_1 + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len.wrapping_mul(0x18) == cur_cap.wrapping_mul(0x18) {
                    let next_len = cur_len.wrapping_add(1);
                    let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                    if new_cap < next_len {
                        new_cap = next_len;
                    }
                    exe::call_va!(0x1_408c_e8a0, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now.wrapping_mul(0x18));
                wr::<u128>(slot, 0);
                wr::<u64>(slot + 0x08, 0);
                wr::<u64>(slot + 0x10, 0);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot2 = rd::<usize>(vec_ptr)
                    .wrapping_add(new_len.wrapping_mul(0x18))
                    .wrapping_sub(0x18);
                wr::<u32>(slot2, elem_idx);
                let src_ptr = rd::<usize>(param_1 + 0x98)
                    .wrapping_add((elem_idx as usize).wrapping_mul(0x10));
                wr::<u128>(slot2 + 0x08, rd::<u128>(src_ptr));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    let needed = rd::<u64>(param_1 + 0x60).wrapping_mul(rd::<u32>(param_1 + 0xa0) as u64);
    let mut target_sz = rd::<u64>(param_1 + 0x68);
    if needed >= target_sz {
        let sum = target_sz.wrapping_sub(1).wrapping_add(needed);
        target_sz = if target_sz != 0 {
            sum.wrapping_sub(sum % target_sz)
        } else {
            needed
        };
    }
    let cur_buf_sz = rd::<u64>(rd::<usize>(param_1 + 0x08) + 0x10);
    if cur_buf_sz < target_sz
        || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8280) as usize) != 0 && cur_buf_sz > target_sz)
    {
        exe::call_va!(0x1_4226_1020, (param_1 + 0x08, target_sz));
    }

    let count = rd::<u32>(param_1 + 0xc8);
    if count != 0 {
        exe::call_va!(
            0x1_412f_9440,
            (
                param_1 + 0x10,
                count as u64,
                param_1 + 0x70,
                param_1 + 0x88
            )
        );
        if count != 0 {
            let mapped = exe::call_va!(
                0x1_4209_c560,
                (
                    rd::<usize>(param_1 + 0x10),
                    0u64,
                    (count as u64).wrapping_mul(0x18),
                    2u32
                ) -> u64
            );
            wr::<u64>(param_1 + 0x38, mapped);
            wr::<u8>(param_1 + 0x30, 1);
            wr::<u32>(param_1 + 0x20, count);
        }
        let prev_off = rd::<u32>(param_1 + 0x40);
        wr::<u32>(param_1 + 0x40, prev_off.wrapping_add(count));
        let dst = rd::<usize>(param_1 + 0x38).wrapping_add((prev_off as usize).wrapping_mul(0x18));
        let src = rd::<usize>(param_1 + 0xc0);
        let _ = exe::call_va!(
            0x1_4183_acbc,
            (dst, src, (count as u64).wrapping_mul(0x18)) -> usize
        );
        if rd::<u32>(param_1 + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(param_1 + 0x10), 0u32));
            wr::<u64>(param_1 + 0x38, 0);
            wr::<u8>(param_1 + 0x30, 0);
            wr::<u32>(param_1 + 0x40, 0);
        }
    }

    let pending = rd::<u32>(param_1 + 0x20);
    if pending != 0 && rd::<u8>(param_1 + 0x29) == 0 {
        exe::call_va!(
            0x1_4226_05c0,
            (
                rd::<u64>(param_1 + 0x08),
                rd::<u64>(param_1 + 0x10),
                pending,
                0x10u32,
                0u8
            )
        );
        wr::<u32>(param_1 + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = param_1 + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len.wrapping_mul(0x18);
        let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64).wrapping_mul(0x18);
        if len_bytes > 0x10_0000 && len_bytes != cap_bytes {
            exe::call_va!(0x1_408c_e8a0, (vec_ptr, len));
        }
    }
    wr::<u32>(param_1 + 0xc8, 0);
}

/// `rend_1412f8ca0`
/// Original: 0x1412f8ca0 (234 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412f8ca0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    let len = rd::<u32>(param_1 + 0x08) as u64;
    if cap.wrapping_sub(len) < param_3 {
        let needed = len.wrapping_add(param_3);
        let mut new_cap = (cap >> 1).wrapping_add(cap);
        if new_cap < needed {
            new_cap = needed;
        }
        exe::call_va!(0x1_40a5_b7c0, (param_1, new_cap));
    }
    let cur_len = rd::<u32>(param_1 + 0x08) as usize;
    let first_elem = rd::<usize>(param_1).wrapping_add(cur_len.wrapping_mul(0x30));
    if param_3 != 0 {
        let c16_ptr = exe::ptr::<u8>(0x1_4342_7660) as usize;
        for i in 0..(param_3 as usize) {
            let elem = first_elem.wrapping_add(i.wrapping_mul(0x30));
            let c16 = rd::<u128>(c16_ptr);
            wr::<u64>(elem, 0x3f80_0000);
            wr::<u64>(elem + 0x08, 0);
            wr::<u128>(elem + 0x10, c16);
            wr::<u128>(elem + 0x20, c16);
        }
    }
    wr::<u32>(
        param_1 + 0x08,
        rd::<u32>(param_1 + 0x08).wrapping_add(param_3 as u32),
    );
    wr::<usize>(param_2, first_elem);
    param_2
}

/// `rend::TrackedSparseUploadBuffer<rend::PreSkinnedMeshManager::SkinnedVertexBatch>::vfunc_1` (`rend_14130ba50`)
/// Original: 0x14130ba50 (932 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_14130ba50(param_1: usize) {
    let words_base = rd::<usize>(param_1 + 0xa8);
    let num_bits = rd::<u64>(param_1 + 0xb8);
    let word_cnt = num_bits.wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(param_1 + 0x90) {
                let vec_ptr = param_1 + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if (cur_len << 5) == (cur_cap << 5) {
                    let next_len = cur_len.wrapping_add(1);
                    let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                    if new_cap < next_len {
                        new_cap = next_len;
                    }
                    exe::call_va!(0x1_4084_0130, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now << 5);
                wr::<[u64; 4]>(slot, [0; 4]);
                wr::<u32>(slot + 0x08, 0xffff_ffff);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot2 = rd::<usize>(vec_ptr)
                    .wrapping_add(new_len << 5)
                    .wrapping_sub(0x20);
                wr::<u32>(slot2, elem_idx);
                let src_ptr = rd::<usize>(param_1 + 0x98)
                    .wrapping_add((elem_idx as usize).wrapping_mul(0x18));
                wr::<u128>(slot2 + 0x08, rd::<u128>(src_ptr));
                wr::<u64>(slot2 + 0x18, rd::<u64>(src_ptr + 0x10));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    let needed = rd::<u64>(param_1 + 0x60).wrapping_mul(rd::<u32>(param_1 + 0xa0) as u64);
    let mut target_sz = rd::<u64>(param_1 + 0x68);
    if needed >= target_sz {
        let sum = target_sz.wrapping_sub(1).wrapping_add(needed);
        target_sz = if target_sz != 0 {
            sum.wrapping_sub(sum % target_sz)
        } else {
            needed
        };
    }
    let cur_buf_sz = rd::<u64>(rd::<usize>(param_1 + 0x08) + 0x10);
    if cur_buf_sz < target_sz
        || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8280) as usize) != 0 && cur_buf_sz > target_sz)
    {
        exe::call_va!(0x1_422e_2520, (param_1 + 0x08, target_sz));
    }

    let count = rd::<u32>(param_1 + 0xc8);
    if count != 0 {
        let needed_up = (count as u64).wrapping_mul(rd::<u64>(param_1 + 0x78));
        let mut alloc_sz = rd::<u64>(param_1 + 0x80);
        if needed_up >= alloc_sz {
            let sum = alloc_sz.wrapping_add(needed_up).wrapping_sub(1);
            alloc_sz = if alloc_sz != 0 {
                sum.wrapping_sub(sum % alloc_sz)
            } else {
                needed_up
            };
        }
        let time_fn: unsafe extern "win64" fn() -> f64 = exe::func(0x1_4240_69a0);
        let now_sec = time_fn();
        let buf_obj = rd::<usize>(param_1 + 0x10);
        let cur_sz = rd::<u64>(buf_obj + 0x10);
        let one_sec = rd::<f64>(exe::ptr::<f64>(0x1_4342_6a48) as usize);
        if cur_sz < alloc_sz
            || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0
                && cur_sz > alloc_sz
                && (now_sec - rd::<f64>(param_1 + 0x88)) > one_sec)
        {
            exe::call_va!(0x1_4209_d670, (buf_obj));
            exe::call_va!(
                0x1_4207_5af0,
                (
                    alloc_sz,
                    0u64,
                    0u64,
                    0x11u32,
                    0u32,
                    0u64,
                    buf_obj as u64
                )
            );
            wr::<u32>(buf_obj + 0x50, u32::from(alloc_sz > 0x40_0000) + 2);
            wr::<f64>(param_1 + 0x88, now_sec);
        }
        if count != 0 {
            let mapped = exe::call_va!(
                0x1_4209_c560,
                (
                    rd::<usize>(param_1 + 0x10),
                    0u64,
                    (count as u64) << 5,
                    2u32
                ) -> u64
            );
            wr::<u64>(param_1 + 0x38, mapped);
            wr::<u8>(param_1 + 0x30, 1);
            wr::<u32>(param_1 + 0x20, count);
        }
        let prev_off = rd::<u32>(param_1 + 0x40);
        wr::<u32>(param_1 + 0x40, prev_off.wrapping_add(count));
        let dst = rd::<usize>(param_1 + 0x38).wrapping_add((prev_off as usize) << 5);
        let src = rd::<usize>(param_1 + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst, src, (count as u64) << 5) -> usize);
        if rd::<u32>(param_1 + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(param_1 + 0x10), 0u32));
            wr::<u64>(param_1 + 0x38, 0);
            wr::<u8>(param_1 + 0x30, 0);
            wr::<u32>(param_1 + 0x40, 0);
        }
    }

    let pending = rd::<u32>(param_1 + 0x20);
    if pending != 0 && rd::<u8>(param_1 + 0x29) == 0 {
        exe::call_va!(
            0x1_422e_1ac0,
            (
                rd::<u64>(param_1 + 0x08),
                rd::<u64>(param_1 + 0x10),
                pending,
                0x18u32,
                0u8
            )
        );
        wr::<u32>(param_1 + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = param_1 + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08);
        let len_bytes = (len as u64) << 5;
        if len_bytes > 0x10_0000 && len_bytes != ((rd::<u32>(vec_ptr + 0x0c) as u64) << 5) {
            let new_buf = exe::call_va!(0x1_401f_fc30, (len_bytes, len) -> usize);
            let copy_bytes = (rd::<u32>(vec_ptr + 0x08) as u64) << 5;
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (new_buf, rd::<usize>(vec_ptr), copy_bytes) -> usize
            );
            exe::call_va!(0x1_401d_e830, (vec_ptr));
            wr::<usize>(vec_ptr, new_buf);
            wr::<u32>(vec_ptr + 0x08, len);
            wr::<u32>(vec_ptr + 0x0c, len);
        }
    }
    wr::<u32>(param_1 + 0xc8, 0);
}

/// `unknown_141311210`
/// Original: 0x141311210 (226 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141311210(param_1: usize, param_2: i32) -> usize {
    let keys_len = rd::<u32>(param_1 + 0x08) as usize;
    let keys_ptr = rd::<usize>(param_1);
    for i in 0..keys_len {
        if rd::<i32>(keys_ptr.wrapping_add(i * 4)) == param_2 {
            return rd::<usize>(param_1 + 0x10).wrapping_add(i * 0x18);
        }
    }

    if (keys_len as u64) * 4 == (rd::<u32>(param_1 + 0x0c) as u64) * 4 {
        exe::call_va!(0x1_405a_f8d0, (param_1, (keys_len as u64).wrapping_add(1)));
    }
    let cur_keys_len = rd::<u32>(param_1 + 0x08) as usize;
    wr::<i32>(rd::<usize>(param_1).wrapping_add(cur_keys_len * 4), param_2);
    wr::<u32>(param_1 + 0x08, (cur_keys_len as u32).wrapping_add(1));

    let vals_len = rd::<u32>(param_1 + 0x18) as u64;
    let vals_cap = rd::<u32>(param_1 + 0x1c) as u64;
    if vals_len.wrapping_mul(0x18) == vals_cap.wrapping_mul(0x18) {
        exe::call_va!(0x1_4131_19e0, (param_1 + 0x10, vals_len.wrapping_add(1)));
    }
    let cur_vals_len = rd::<u32>(param_1 + 0x18) as usize;
    let slot = rd::<usize>(param_1 + 0x10).wrapping_add(cur_vals_len * 0x18);
    wr::<u128>(slot, 0);
    wr::<u64>(slot + 0x10, 0);
    wr::<u32>(param_1 + 0x18, (cur_vals_len as u32).wrapping_add(1));

    let new_vals_len = rd::<u32>(param_1 + 0x18) as usize;
    rd::<usize>(param_1 + 0x10)
        .wrapping_add(new_vals_len * 0x18)
        .wrapping_sub(0x18)
}

/// `rend::SparseUploadBuffer<rend::MeshEngineGPUIndirectCommandVS>::vfunc_1` (`rend_1413383d0`)
/// Original: 0x1413383d0 (841 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_1413383d0(param_1: usize) {
    let words_base = rd::<usize>(param_1 + 0xa8);
    let num_bits = rd::<u64>(param_1 + 0xb8);
    let word_cnt = num_bits.wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(param_1 + 0x90) {
                let vec_ptr = param_1 + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len.wrapping_mul(0x3c) == cur_cap.wrapping_mul(0x3c) {
                    let next_len = cur_len.wrapping_add(1);
                    let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                    if new_cap < next_len {
                        new_cap = next_len;
                    }
                    exe::call_va!(0x1_4133_a7a0, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now.wrapping_mul(0x3c));
                wr::<[u64; 7]>(slot, [0; 7]);
                wr::<u32>(slot + 0x38, 0);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot2 = rd::<usize>(vec_ptr)
                    .wrapping_add(new_len.wrapping_mul(0x3c))
                    .wrapping_sub(0x3c);
                wr::<u32>(slot2, elem_idx);
                let src_ptr = rd::<usize>(param_1 + 0x98)
                    .wrapping_add((elem_idx as usize).wrapping_mul(0x34));
                wr::<[u64; 6]>(slot2 + 0x08, rd::<[u64; 6]>(src_ptr));
                wr::<u32>(slot2 + 0x38, rd::<u32>(src_ptr + 0x30));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    exe::call_va!(
        0x1_412e_c110,
        (param_1 + 0x08, rd::<u32>(param_1 + 0xa0), param_1 + 0x58)
    );

    let count = rd::<u32>(param_1 + 0xc8);
    if count != 0 {
        let needed_up = (count as u64).wrapping_mul(rd::<u64>(param_1 + 0x78));
        let mut alloc_sz = rd::<u64>(param_1 + 0x80);
        if needed_up >= alloc_sz {
            let sum = alloc_sz.wrapping_add(needed_up).wrapping_sub(1);
            alloc_sz = if alloc_sz != 0 {
                sum.wrapping_sub(sum % alloc_sz)
            } else {
                needed_up
            };
        }
        let time_fn: unsafe extern "win64" fn() -> f64 = exe::func(0x1_4240_69a0);
        let now_sec = time_fn();
        let buf_obj = rd::<usize>(param_1 + 0x10);
        let cur_sz = rd::<u64>(buf_obj + 0x10);
        let one_sec = rd::<f64>(exe::ptr::<f64>(0x1_4342_6a48) as usize);
        if cur_sz < alloc_sz
            || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0
                && cur_sz > alloc_sz
                && (now_sec - rd::<f64>(param_1 + 0x88)) > one_sec)
        {
            exe::call_va!(0x1_4209_d670, (buf_obj));
            exe::call_va!(
                0x1_4207_5af0,
                (
                    alloc_sz,
                    0u64,
                    0u64,
                    0x11u32,
                    0u32,
                    0u64,
                    buf_obj as u64
                )
            );
            wr::<u32>(buf_obj + 0x50, u32::from(alloc_sz > 0x40_0000) + 2);
            wr::<f64>(param_1 + 0x88, now_sec);
        }
        if count != 0 {
            let mapped = exe::call_va!(
                0x1_4209_c560,
                (
                    rd::<usize>(param_1 + 0x10),
                    0u64,
                    (count as u64).wrapping_mul(0x3c),
                    2u32
                ) -> u64
            );
            wr::<u64>(param_1 + 0x38, mapped);
            wr::<u8>(param_1 + 0x30, 1);
            wr::<u32>(param_1 + 0x20, count);
        }
        let prev_off = rd::<u32>(param_1 + 0x40);
        wr::<u32>(param_1 + 0x40, prev_off.wrapping_add(count));
        let dst = rd::<usize>(param_1 + 0x38).wrapping_add((prev_off as usize).wrapping_mul(0x3c));
        let src = rd::<usize>(param_1 + 0xc0);
        let _ = exe::call_va!(
            0x1_4183_acbc,
            (dst, src, (count as u64).wrapping_mul(0x3c)) -> usize
        );
        if rd::<u32>(param_1 + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(param_1 + 0x10), 0u32));
            wr::<u64>(param_1 + 0x38, 0);
            wr::<u8>(param_1 + 0x30, 0);
            wr::<u32>(param_1 + 0x40, 0);
        }
    }

    let pending = rd::<u32>(param_1 + 0x20);
    if pending != 0 && rd::<u8>(param_1 + 0x29) == 0 {
        exe::call_va!(
            0x1_4232_0000,
            (
                rd::<u64>(param_1 + 0x08),
                rd::<u64>(param_1 + 0x10),
                pending,
                0x34u32,
                0u8
            )
        );
        wr::<u32>(param_1 + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = param_1 + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len.wrapping_mul(0x3c);
        let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64).wrapping_mul(0x3c);
        if len_bytes > 0x10_0000 && len_bytes != cap_bytes {
            exe::call_va!(0x1_4133_a7a0, (vec_ptr, len));
        }
    }
    wr::<u32>(param_1 + 0xc8, 0);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141069350(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mats: [[f32; 16]; 2] = [
        [
            1.5, -0.5, 2.25, 0.75, 0.25, 3.0, -1.25, 0.5, -2.0, 1.0, 1.75, -0.25, 0.5, -1.5, 0.25,
            2.5,
        ],
        [
            2.0, 0.0, -1.0, 4.0, -3.0, 1.0, 0.5, -2.0, 1.25, -2.5, 3.0, 0.75, -0.5, 1.75, -1.5,
            1.0,
        ],
    ];

    for (idx, m) in mats.iter().enumerate() {
        let mut mo = *m;
        let mut mn = *m;
        f_o(mo.as_mut_ptr() as usize);
        f_n(mn.as_mut_ptr() as usize);
        let bo = rd::<[u32; 16]>(mo.as_ptr() as usize);
        let bn = rd::<[u32; 16]>(mn.as_ptr() as usize);
        crate::diff_eq!(bo, bn, "141069350 case {}", idx);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411642f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_BUF: [u64; 38] = [0; 38];
    static mut LOG_CALLS: usize = 0;

    unsafe extern "win64" fn mock_b9a0(
        pkt_ptr: usize,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
    ) {
        let pkt = rd::<[u64; 15]>(pkt_ptr);
        let calls_ptr = core::ptr::addr_of_mut!(LOG_CALLS) as usize;
        let idx = rd::<usize>(calls_ptr);
        if idx < 2 {
            let dst = (core::ptr::addr_of_mut!(LOG_BUF) as usize) + idx * (19 * 8);
            wr::<[u64; 15]>(dst, pkt);
            wr::<[u64; 4]>(dst + 15 * 8, [a2, a3, a4, a5]);
        }
        wr::<usize>(calls_ptr, idx + 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41fc_b9a0, mock_b9a0 as *const ())?;

    let run = |f: F, count: u64| -> (usize, [u64; 38]) {
        let log_ptr = core::ptr::addr_of_mut!(LOG_BUF) as usize;
        let calls_ptr = core::ptr::addr_of_mut!(LOG_CALLS) as usize;
        wr::<[u64; 38]>(log_ptr, [0; 38]);
        wr::<usize>(calls_ptr, 0);

        let mut p2 = [0u64; 17];
        p2[1] = 0x9999_8888; // +0x08
        p2[3] = count; // +0x18
        for i in 0..13usize {
            p2[4 + i] = 0x1000 + i as u64; // +0x20..+0x88
        }
        let p3: [u64; 4] = [0xaa00, 0xbb01, 0xcc02, 0xdd03];
        f(0, p2.as_ptr() as usize, p3.as_ptr() as usize);
        (rd::<usize>(calls_ptr), rd::<[u64; 38]>(log_ptr))
    };

    for cnt in [0u64, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1411642f0 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141171ad0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_13a0(out_ptr: usize, p1: usize) -> usize {
        let world = rd::<usize>(p1) as u64;
        wr::<u64>(out_ptr, world);
        for i in 1..10usize {
            wr::<u64>(out_ptr + i * 8, 0x7000 + i as u64);
        }
        out_ptr
    }

    unsafe extern "win64" fn mock_1670(
        out_ptr: usize,
        ent_cnt: u64,
        _world_p40: usize,
        arch: u16,
    ) -> usize {
        wr::<u64>(out_ptr, 0x100 + arch as u64);
        wr::<u64>(out_ptr + 0x08, 0);
        wr::<u64>(out_ptr + 0x10, 0x200 + ent_cnt);
        wr::<u64>(out_ptr + 0x18, 0x300);
        wr::<u64>(out_ptr + 0x20, 0);
        wr::<u64>(out_ptr + 0x28, 0x500);
        out_ptr
    }

    unsafe extern "win64" fn mock_6020(
        pkt_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        a4_ptr: usize,
        a5_world: usize,
    ) {
        let pkt = rd::<[u64; 8]>(pkt_ptr);
        let a2 = rd::<[u64; 3]>(a2_ptr);
        let a3 = rd::<[u64; 3]>(a3_ptr);
        let a4 = rd::<[u64; 3]>(a4_ptr);
        let idx = pkt[7] as usize;
        let dst = a5_world + 0x200 + idx * (17 * 8);
        wr::<[u64; 8]>(dst, pkt);
        wr::<[u64; 3]>(dst + 64, a2);
        wr::<[u64; 3]>(dst + 88, a3);
        wr::<[u64; 3]>(dst + 112, a4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4117_13a0, mock_13a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4117_1670, mock_1670 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41fd_6020, mock_6020 as *const ())?;

    let run = |f: F| -> [u64; 34] {
        let mut world = vec![0u8; 0x12000];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x40 + 0x48 + 2 * 8, 0x80_0000);
        wr::<u32>(wp + 0x40 + 0x10048 + 2 * 4, 2);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 34]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141171ad0 out");
    Ok(1)
}

pub unsafe fn diff_coregame_141195f80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut OUT_LOG: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_6060(
        out_ptr: usize,
        arch: u16,
        block: u64,
        cnt: u64,
        world: u64,
    ) -> usize {
        wr::<u64>(out_ptr, arch as u64);
        wr::<u64>(out_ptr + 0x08, block);
        wr::<u64>(out_ptr + 0x10, world);
        wr::<u64>(out_ptr + 0x18, cnt);
        wr::<u64>(out_ptr + 0x20, block + 0x110);
        wr::<u64>(out_ptr + 0x28, block + 0x220);
        out_ptr
    }

    unsafe extern "win64" fn mock_fe90(pkt_ptr: usize) {
        let pkt = rd::<[u64; 4]>(pkt_ptr);
        let idx = pkt[3] as usize;
        if idx < 2 {
            let dst = (core::ptr::addr_of_mut!(OUT_LOG) as usize) + idx * 32;
            wr::<[u64; 4]>(dst, pkt);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4119_6060, mock_6060 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41fe_fe90, mock_fe90 as *const ())?;

    let run = |f: F, arch: u16| -> [u64; 8] {
        let out_ptr = core::ptr::addr_of_mut!(OUT_LOG) as usize;
        wr::<[u64; 8]>(out_ptr, [0; 8]);
        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x90_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        rd::<[u64; 8]>(out_ptr)
    };

    for arch in [0u16, 1] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        crate::diff_eq!(o, n, "141195f80 arch={}", arch);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411a5000(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_tbl = exe::ptr::<u8>(0x1_43a8_ae10) as usize;
    let saved = rd::<[u8; 0x88 * 3]>(g_tbl);

    let heap_str = b"heap_target_str\0";

    // Entry 0: inline string "alpha\0", b3 = 4 (cap_bits = 32), len = 4 (len_bits = 32 <= 32)
    wr::<[u8; 0x88 * 3]>(g_tbl, [0u8; 0x88 * 3]);
    wr::<u32>(g_tbl, (4u32 << 24) | 4);
    wr::<[u8; 6]>(g_tbl + 0x08, *b"alpha\0");

    // Entry 1: heap string "heap_target_str\0", b3 = 2 (cap_bits = 16), len = 15 (len_bits = 120 > 16)
    let e1 = g_tbl + 0x88;
    wr::<u32>(e1, (2u32 << 24) | 15);
    wr::<usize>(e1 + 0x08, heap_str.as_ptr() as usize);

    // Entry 2: inline string "beta\0", b3 = 0x80 (cap_bits = 0x400), len = 5 (len_bits = 40 <= 0x400)
    let e2 = g_tbl + 0x110;
    wr::<u32>(e2, (0x80u32 << 24) | 5);
    wr::<[u8; 5]>(e2 + 0x08, *b"beta\0");

    let indices: [i32; 3] = [0, 1, 2];
    let base_vals: usize = 0x5555_0000;
    let mut p1 = [0u64; 3];
    let p1p = p1.as_mut_ptr() as usize;
    wr::<usize>(p1p, indices.as_ptr() as usize);
    wr::<u32>(p1p + 0x08, 3);
    wr::<usize>(p1p + 0x10, base_vals);

    let queries: [&[u8]; 4] = [b"alpha\0", b"heap_target_str\0", b"beta\0", b"missing\0"];
    let mut res_o = [0usize; 5];
    let mut res_n = [0usize; 5];
    for (i, q) in queries.iter().enumerate() {
        res_o[i] = f_o(p1p, q.as_ptr() as usize);
        res_n[i] = f_n(p1p, q.as_ptr() as usize);
    }
    wr::<u32>(p1p + 0x08, 0);
    res_o[4] = f_o(p1p, b"alpha\0".as_ptr() as usize);
    res_n[4] = f_n(p1p, b"alpha\0".as_ptr() as usize);

    wr::<[u8; 0x88 * 3]>(g_tbl, saved);
    crate::diff_eq!(res_o, res_n, "1411a5000 out");
    Ok(5)
}

pub unsafe fn diff_unknown_1411f1520(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> i32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let arr: [u64; 6] = [10, 20, 20, 30, 40, 50];
    let ap = arr.as_ptr() as usize;

    for (cnt, target) in [
        (0u64, 20u64),
        (6, 5),
        (6, 10),
        (6, 20),
        (6, 25),
        (6, 30),
        (6, 50),
        (6, 60),
    ] {
        let ptr = if cnt == 0 { 0 } else { ap };
        let o = f_o(ptr, cnt, target);
        let n = f_n(ptr, cnt, target);
        crate::diff_eq!(o, n, "1411f1520 cnt={} target={}", cnt, target);
    }
    Ok(8)
}

pub unsafe fn diff_unknown_1411f1c40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let arr: [u64; 6] = [10, 20, 20, 30, 40, 50];
    let ap = arr.as_ptr() as usize;

    for (cnt, target) in [
        (0u64, 20u64),
        (6, 5),
        (6, 10),
        (6, 20),
        (6, 25),
        (6, 30),
        (6, 50),
        (6, 60),
    ] {
        let ptr = if cnt == 0 { 0 } else { ap };
        let o = f_o(ptr, cnt, target);
        let n = f_n(ptr, cnt, target);
        crate::diff_eq!(o, n, "1411f1c40 cnt={} target={}", cnt, target);
    }
    Ok(8)
}

pub unsafe fn diff_rend_1412f8710(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_MEM: [u8; 0x18 * 4] = [0; 0x18 * 4];
    static mut MAPPED_MEM: [u8; 0x18 * 4] = [0; 0x18 * 4];
    static mut TRACE: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_e8a0(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC_MEM) as usize);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        wr::<u64>(core::ptr::addr_of_mut!(TRACE) as usize, new_cap);
    }
    unsafe extern "win64" fn mock_1020(_p8: usize, target_sz: u64) {
        wr::<u64>((core::ptr::addr_of_mut!(TRACE) as usize) + 8, target_sz);
    }
    unsafe extern "win64" fn mock_9440(_p10: usize, cnt: u64, _p70: usize, _p88: usize) {
        wr::<u64>((core::ptr::addr_of_mut!(TRACE) as usize) + 16, cnt);
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: u64, bytes: u64, mode: u32) -> u64 {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 24,
            bytes ^ (mode as u64),
        );
        core::ptr::addr_of_mut!(MAPPED_MEM) as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, flag: u32) {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 32,
            (flag as u64) + 1,
        );
    }
    unsafe extern "win64" fn mock_05c0(b8: u64, b10: u64, cnt: u32, stride: u32, flag: u8) {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 40,
            b8 ^ b10 ^ ((cnt as u64) << 16) ^ ((stride as u64) << 8) ^ (flag as u64),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_408c_e8a0, mock_e8a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4226_1020, mock_1020 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_9440, mock_9440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4226_05c0, mock_05c0 as *const ())?;

    let run = |f: F| -> (u64, [u8; 0x18 * 4], [u8; 0x18 * 4], [u64; 8], u32, u32) {
        let vec_ptr_raw = core::ptr::addr_of_mut!(VEC_MEM) as usize;
        let mapped_ptr_raw = core::ptr::addr_of_mut!(MAPPED_MEM) as usize;
        let trace_ptr_raw = core::ptr::addr_of_mut!(TRACE) as usize;
        wr::<[u8; 0x18 * 4]>(vec_ptr_raw, [0; 0x18 * 4]);
        wr::<[u8; 0x18 * 4]>(mapped_ptr_raw, [0; 0x18 * 4]);
        wr::<[u64; 8]>(trace_ptr_raw, [0; 8]);

        let mut word: u64 = 0b1011; // bits 0, 1, 3 (3 >= 3 so skipped)
        let src_spheres: [u64; 6] = [0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666];
        let mut buf_08 = [0u64; 4];
        buf_08[2] = 0x100; // cur_buf_sz at +0x10

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, buf_08.as_mut_ptr() as usize);
        wr::<u64>(tp + 0x10, 0xabc0);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x60, 16);
        wr::<u64>(tp + 0x68, 256);
        wr::<u64>(tp + 0x90, 3);
        wr::<usize>(tp + 0x98, src_spheres.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 20); // needed = 320 >= 256 -> rounds up to 512
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_ptr_raw);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 0); // force growth on first push

        f(tp);
        (
            word,
            rd::<[u8; 0x18 * 4]>(vec_ptr_raw),
            rd::<[u8; 0x18 * 4]>(mapped_ptr_raw),
            rd::<[u64; 8]>(trace_ptr_raw),
            rd::<u32>(tp + 0x20),
            rd::<u32>(tp + 0xc8),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412f8710 out");
    Ok(1)
}

pub unsafe fn diff_rend_1412f8ca0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF_MEM: [u32; 36] = [0; 36]; // 3 elements of 0x30 bytes (12 u32s each)
    static mut GROW_CAP: u64 = 0;

    unsafe extern "win64" fn mock_b7c0(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(BUF_MEM) as usize);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        wr::<u64>(core::ptr::addr_of_mut!(GROW_CAP) as usize, new_cap);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40a5_b7c0, mock_b7c0 as *const ())?;

    let run =
        |f: F, init_len: u32, init_cap: u32, add_cnt: u64| -> (usize, u32, u32, u64, [u32; 36]) {
            let base_ptr = core::ptr::addr_of_mut!(BUF_MEM) as usize;
            let grow_ptr = core::ptr::addr_of_mut!(GROW_CAP) as usize;
            wr::<[u32; 36]>(base_ptr, [0xdead_beef; 36]);
            wr::<u64>(grow_ptr, 0);

            let mut vec_hdr = [0u64; 2];
            let vp = vec_hdr.as_mut_ptr() as usize;
            wr::<usize>(vp, base_ptr);
            wr::<u32>(vp + 0x08, init_len);
            wr::<u32>(vp + 0x0c, init_cap);

            let mut out_ptr: usize = 0;
            let ret = f(vp, (&mut out_ptr as *mut usize) as usize, add_cnt);
            assert_eq!(ret, (&mut out_ptr as *mut usize) as usize);

            (
                out_ptr.wrapping_sub(base_ptr),
                rd::<u32>(vp + 0x08),
                rd::<u32>(vp + 0x0c),
                rd::<u64>(grow_ptr),
                rd::<[u32; 36]>(base_ptr),
            )
        };

    for (len, cap, cnt) in [(1u32, 3u32, 2u64), (1, 1, 2), (2, 2, 0)] {
        let o = run(f_o, len, cap, cnt);
        let n = run(f_n, len, cap, cnt);
        crate::diff_eq!(o, n, "1412f8ca0 len={} cap={} cnt={}", len, cap, cnt);
    }
    Ok(3)
}

pub unsafe fn diff_rend_14130ba50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_MEM: [u8; 0x20 * 4] = [0; 0x20 * 4];
    static mut MAPPED_MEM: [u8; 0x20 * 4] = [0; 0x20 * 4];
    static mut TRACE: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_0130(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC_MEM) as usize);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        wr::<u64>(core::ptr::addr_of_mut!(TRACE) as usize, new_cap);
    }
    unsafe extern "win64" fn mock_2520(_p8: usize, target_sz: u64) {
        wr::<u64>((core::ptr::addr_of_mut!(TRACE) as usize) + 8, target_sz);
    }
    unsafe extern "win64" fn mock_69a0() -> f64 {
        3.5
    }
    unsafe extern "win64" fn mock_d670(_buf: usize) {
        let p = (core::ptr::addr_of_mut!(TRACE) as usize) + 16;
        wr::<u64>(p, rd::<u64>(p) + 1);
    }
    unsafe extern "win64" fn mock_5af0(
        sz: u64,
        a2: u64,
        a3: u64,
        flags: u32,
        a5: u32,
        _a6: u64,
        buf_obj: usize,
    ) {
        wr::<u64>(buf_obj + 0x10, sz);
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 24,
            sz ^ a2 ^ a3 ^ ((flags as u64) << 8) ^ (a5 as u64),
        );
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: u64, bytes: u64, mode: u32) -> u64 {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 32,
            bytes ^ (mode as u64),
        );
        core::ptr::addr_of_mut!(MAPPED_MEM) as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, flag: u32) {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 40,
            (flag as u64) + 1,
        );
    }
    unsafe extern "win64" fn mock_1ac0(b8: u64, b10: u64, cnt: u32, stride: u32, flag: u8) {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 48,
            b8 ^ (b10 & 0xffff) ^ ((cnt as u64) << 16) ^ ((stride as u64) << 8) ^ (flag as u64),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4084_0130, mock_0130 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422e_2520, mock_2520 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_69a0, mock_69a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_422e_1ac0, mock_1ac0 as *const ())?;

    let run = |f: F| -> (u64, [u8; 0x20 * 4], [u8; 0x20 * 4], [u64; 8], u32, u64) {
        let vec_ptr_raw = core::ptr::addr_of_mut!(VEC_MEM) as usize;
        let mapped_ptr_raw = core::ptr::addr_of_mut!(MAPPED_MEM) as usize;
        let trace_ptr_raw = core::ptr::addr_of_mut!(TRACE) as usize;
        wr::<[u8; 0x20 * 4]>(vec_ptr_raw, [0; 0x20 * 4]);
        wr::<[u8; 0x20 * 4]>(mapped_ptr_raw, [0; 0x20 * 4]);
        wr::<[u64; 8]>(trace_ptr_raw, [0; 8]);

        let mut word: u64 = 0b0101; // bits 0 and 2
        let src_batches: [u64; 9] = [
            0x101, 0x102, 0x103, 0x201, 0x202, 0x203, 0x301, 0x302, 0x303,
        ];
        let mut buf_08 = [0u64; 4];
        buf_08[2] = 0x80; // +0x10
        let mut buf_10 = [0u64; 12];
        buf_10[2] = 0x40; // +0x10 (< alloc_sz so triggers reinit)

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, buf_08.as_mut_ptr() as usize);
        wr::<usize>(tp + 0x10, buf_10.as_mut_ptr() as usize);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x60, 24);
        wr::<u64>(tp + 0x68, 128);
        wr::<u64>(tp + 0x78, 32);
        wr::<u64>(tp + 0x80, 128);
        wr::<f64>(tp + 0x88, 1.0);
        wr::<u64>(tp + 0x90, 4);
        wr::<usize>(tp + 0x98, src_batches.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 10);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_ptr_raw);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 0);

        f(tp);
        wr::<u64>(
            trace_ptr_raw + 56,
            rd::<u32>(buf_10.as_ptr() as usize + 0x50) as u64,
        );
        (
            word,
            rd::<[u8; 0x20 * 4]>(vec_ptr_raw),
            rd::<[u8; 0x20 * 4]>(mapped_ptr_raw),
            rd::<[u64; 8]>(trace_ptr_raw),
            rd::<u32>(tp + 0xc8),
            rd::<u64>(tp + 0x88),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14130ba50 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141311210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut KEYS_BUF: [i32; 8] = [0; 8];
    static mut VALS_BUF: [u64; 24] = [0; 24];
    static mut GROW_LOG: [u64; 2] = [0; 2];

    unsafe extern "win64" fn mock_f8d0(p1: usize, min_cap: u64) {
        wr::<usize>(p1, core::ptr::addr_of_mut!(KEYS_BUF) as usize);
        wr::<u32>(p1 + 0x0c, (min_cap as u32) + 2);
        wr::<u64>(core::ptr::addr_of_mut!(GROW_LOG) as usize, min_cap);
    }
    unsafe extern "win64" fn mock_19e0(p1_10: usize, min_cap: u64) {
        wr::<usize>(p1_10, core::ptr::addr_of_mut!(VALS_BUF) as usize);
        wr::<u32>(p1_10 + 0x0c, (min_cap as u32) + 2);
        wr::<u64>((core::ptr::addr_of_mut!(GROW_LOG) as usize) + 8, min_cap);
    }

    let _m1 = crate::difftest::Mock::new(0x1_405a_f8d0, mock_f8d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4131_19e0, mock_19e0 as *const ())?;

    let run = |f: F| -> ([usize; 3], [i32; 8], [u64; 24], [u64; 2], u32, u32) {
        let kp = core::ptr::addr_of_mut!(KEYS_BUF) as usize;
        let vp = core::ptr::addr_of_mut!(VALS_BUF) as usize;
        let gp = core::ptr::addr_of_mut!(GROW_LOG) as usize;
        wr::<[i32; 8]>(kp, [10, 20, 0, 0, 0, 0, 0, 0]);
        wr::<[u64; 24]>(vp, [0xaaaa_bbbb_cccc_dddd; 24]);
        wr::<[u64; 2]>(gp, [0; 2]);

        let mut p1 = [0u64; 4];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, kp);
        wr::<u32>(p1p + 0x08, 2);
        wr::<u32>(p1p + 0x0c, 2); // full -> next insert grows
        wr::<usize>(p1p + 0x10, vp);
        wr::<u32>(p1p + 0x18, 2);
        wr::<u32>(p1p + 0x1c, 2); // full -> next insert grows

        let r0 = f(p1p, 20).wrapping_sub(vp); // existing key at index 1
        let r1 = f(p1p, 30).wrapping_sub(vp); // new key at index 2 (triggers grow)
        let r2 = f(p1p, 40).wrapping_sub(vp); // new key at index 3 (no grow needed)
        (
            [r0, r1, r2],
            rd::<[i32; 8]>(kp),
            rd::<[u64; 24]>(vp),
            rd::<[u64; 2]>(gp),
            rd::<u32>(p1p + 0x08),
            rd::<u32>(p1p + 0x18),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141311210 out");
    Ok(3)
}

pub unsafe fn diff_rend_1413383d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_MEM: [u8; 0x3c * 4] = [0; 0x3c * 4];
    static mut MAPPED_MEM: [u8; 0x3c * 4] = [0; 0x3c * 4];
    static mut TRACE: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_a7a0(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC_MEM) as usize);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        wr::<u64>(core::ptr::addr_of_mut!(TRACE) as usize, new_cap);
    }
    unsafe extern "win64" fn mock_c110(_p8: usize, a0: u32, _p58: usize) {
        wr::<u64>((core::ptr::addr_of_mut!(TRACE) as usize) + 8, a0 as u64);
    }
    unsafe extern "win64" fn mock_69a0() -> f64 {
        4.25
    }
    unsafe extern "win64" fn mock_d670(_buf: usize) {
        let p = (core::ptr::addr_of_mut!(TRACE) as usize) + 16;
        wr::<u64>(p, rd::<u64>(p) + 1);
    }
    unsafe extern "win64" fn mock_5af0(
        sz: u64,
        a2: u64,
        a3: u64,
        flags: u32,
        a5: u32,
        _a6: u64,
        buf_obj: usize,
    ) {
        wr::<u64>(buf_obj + 0x10, sz);
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 24,
            sz ^ a2 ^ a3 ^ ((flags as u64) << 8) ^ (a5 as u64),
        );
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: u64, bytes: u64, mode: u32) -> u64 {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 32,
            bytes ^ (mode as u64),
        );
        core::ptr::addr_of_mut!(MAPPED_MEM) as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, flag: u32) {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 40,
            (flag as u64) + 1,
        );
    }
    unsafe extern "win64" fn mock_0000(b8: u64, b10: u64, cnt: u32, stride: u32, flag: u8) {
        wr::<u64>(
            (core::ptr::addr_of_mut!(TRACE) as usize) + 48,
            b8 ^ (b10 & 0xffff) ^ ((cnt as u64) << 16) ^ ((stride as u64) << 8) ^ (flag as u64),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4133_a7a0, mock_a7a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_c110, mock_c110 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_69a0, mock_69a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4232_0000, mock_0000 as *const ())?;

    let run = |f: F| -> (u64, [u8; 0x3c * 4], [u8; 0x3c * 4], [u64; 8], u32, u64) {
        let vec_ptr_raw = core::ptr::addr_of_mut!(VEC_MEM) as usize;
        let mapped_ptr_raw = core::ptr::addr_of_mut!(MAPPED_MEM) as usize;
        let trace_ptr_raw = core::ptr::addr_of_mut!(TRACE) as usize;
        wr::<[u8; 0x3c * 4]>(vec_ptr_raw, [0; 0x3c * 4]);
        wr::<[u8; 0x3c * 4]>(mapped_ptr_raw, [0; 0x3c * 4]);
        wr::<[u64; 8]>(trace_ptr_raw, [0; 8]);

        let mut word: u64 = 0b0011; // bits 0 and 1
        let mut src_cmds = [0u32; 13 * 2];
        for (i, slot) in src_cmds.iter_mut().enumerate() {
            *slot = 0x5000 + i as u32;
        }
        let mut buf_10 = [0u64; 12];
        buf_10[2] = 0x40; // +0x10

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<u64>(tp + 0x08, 0x7770);
        wr::<usize>(tp + 0x10, buf_10.as_mut_ptr() as usize);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x78, 60);
        wr::<u64>(tp + 0x80, 64);
        wr::<f64>(tp + 0x88, 1.0);
        wr::<u64>(tp + 0x90, 2);
        wr::<usize>(tp + 0x98, src_cmds.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 15);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_ptr_raw);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 0);

        f(tp);
        wr::<u64>(
            trace_ptr_raw + 56,
            rd::<u32>(buf_10.as_ptr() as usize + 0x50) as u64,
        );
        (
            word,
            rd::<[u8; 0x3c * 4]>(vec_ptr_raw),
            rd::<[u8; 0x3c * 4]>(mapped_ptr_raw),
            rd::<[u64; 8]>(trace_ptr_raw),
            rd::<u32>(tp + 0xc8),
            rd::<u64>(tp + 0x88),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1413383d0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_4106_9350, "unknown_141069350", rend::cauldron_hot_w2::unknown_141069350 as *const (), rend::cauldron_hot_w2::diff_unknown_141069350),
// Replacement::verified_throws(0x1_4116_42f0, "unknown_1411642f0", rend::cauldron_hot_w2::unknown_1411642f0 as *const (), Throws::Discard, rend::cauldron_hot_w2::diff_unknown_1411642f0),
// Replacement::verified(0x1_4117_1ad0, "unknown_141171ad0", rend::cauldron_hot_w2::unknown_141171ad0 as *const (), rend::cauldron_hot_w2::diff_unknown_141171ad0),
// Replacement::verified(0x1_4119_5f80, "coregame_141195f80", rend::cauldron_hot_w2::coregame_141195f80 as *const (), rend::cauldron_hot_w2::diff_coregame_141195f80),
// Replacement::verified(0x1_411a_5000, "unknown_1411a5000", rend::cauldron_hot_w2::unknown_1411a5000 as *const (), rend::cauldron_hot_w2::diff_unknown_1411a5000),
// Replacement::verified(0x1_411f_1520, "unknown_1411f1520", rend::cauldron_hot_w2::unknown_1411f1520 as *const (), rend::cauldron_hot_w2::diff_unknown_1411f1520),
// Replacement::verified(0x1_411f_1c40, "unknown_1411f1c40", rend::cauldron_hot_w2::unknown_1411f1c40 as *const (), rend::cauldron_hot_w2::diff_unknown_1411f1c40),
// Replacement::verified_throws(0x1_412f_8710, "rend_1412f8710", rend::cauldron_hot_w2::rend_1412f8710 as *const (), Throws::Caught, rend::cauldron_hot_w2::diff_rend_1412f8710),
// Replacement::verified(0x1_412f_8ca0, "rend_1412f8ca0", rend::cauldron_hot_w2::rend_1412f8ca0 as *const (), rend::cauldron_hot_w2::diff_rend_1412f8ca0),
// Replacement::verified_throws(0x1_4130_ba50, "rend_14130ba50", rend::cauldron_hot_w2::rend_14130ba50 as *const (), Throws::Caught, rend::cauldron_hot_w2::diff_rend_14130ba50),
// Replacement::verified(0x1_4131_1210, "unknown_141311210", rend::cauldron_hot_w2::unknown_141311210 as *const (), rend::cauldron_hot_w2::diff_unknown_141311210),
// Replacement::verified_throws(0x1_4133_83d0, "rend_1413383d0", rend::cauldron_hot_w2::rend_1413383d0 as *const (), Throws::Caught, rend::cauldron_hot_w2::diff_rend_1413383d0),
