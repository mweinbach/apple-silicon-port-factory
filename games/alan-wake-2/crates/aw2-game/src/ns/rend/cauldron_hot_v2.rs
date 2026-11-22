//! Decompiled hot functions shard V2 (`0x1411641a0..0x141892bd0`).

use crate::exe::{self, rd, wr};

/// `unknown_1411641a0`
/// Original: 0x1411641a0 (324 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411641a0(param_1: usize, param_2: u16) {
    let arch = param_2 as usize;
    let world = rd::<usize>(param_1);
    let block = rd::<u64>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if ent_count != 0 {
        let id_910 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d910) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_910 = rd::<u64>(tbl_580e0.wrapping_add(id_910 * 8));

        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let val_63c = rd::<u64>(tbl_580e0.wrapping_add(id_63c * 8));

        let id_948 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_8948) as usize) as usize;
        let val_948 = rd::<u64>(tbl_580e0.wrapping_add(id_948 * 8));

        let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as u64;
        let worker_slot = (world as u64)
            .wrapping_add(worker_idx.wrapping_mul(0x188))
            .wrapping_add(0x580f0);

        let mut arg_r8: [u64; 4] = [worker_slot, val_948, val_63c, val_910];
        let mut pkt1 = [0u64; 17];
        let block_reloaded = rd::<u64>(world.wrapping_add(arch * 8).wrapping_add(0x88));
        let _ = exe::call_va!(
            0x1_4116_4af0,
            (
                pkt1.as_mut_ptr() as usize,
                param_2,
                block_reloaded,
                ent_count,
                world as u64
            ) -> usize
        );

        let mut pkt2 = pkt1;
        exe::call_va!(
            0x1_4116_42f0,
            (
                0usize,
                pkt2.as_mut_ptr() as usize,
                arg_r8.as_mut_ptr() as usize
            )
        );
    }
}

#[inline(always)]
fn sdiv_0x88(byte_offset: u64) -> u64 {
    let prod = (byte_offset as i64 as i128).wrapping_mul(0x7878_7878_7878_7879i64 as i128);
    let hi = (prod >> 64) as i64;
    let q = (hi >> 6).wrapping_add(((hi as u64) >> 63) as i64);
    q as u64
}

/// `unknown_1411f7760`
/// Original: 0x1411f7760 (595 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f7760(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let count0 = rd::<u32>(param_1 + 0x10) as u64;
    let byte_span0 = count0.wrapping_mul(0x88);
    let idx0 = sdiv_0x88(byte_span0);
    if idx0 == 0 {
        let base_08 = rd::<u64>(param_1 + 0x08);
        let base_18 = rd::<u64>(param_1 + 0x18);
        wr::<u64>(param_2, base_08.wrapping_add(byte_span0));
        wr::<u64>(param_2 + 0x08, base_18.wrapping_add(idx0.wrapping_mul(0x120)));
        return param_2;
    }

    exe::call_va!(0x1_411f_76e0, (param_1));

    let mut lo: u64 = 0;
    let mut arr_base = rd::<usize>(param_1 + 0x08);
    if rd::<u32>(param_1 + 0x10) != 0 {
        let p3_hdr = rd::<u32>(param_3);
        let p3_len = rd::<u32>(param_3 + 0x04) as u64;
        let p3_len_bits = (p3_hdr & 0x00ff_ffff) << 3;
        let p3_b3 = rd::<u8>(param_3 + 0x03) as u32;
        let p3_inline = param_3.wrapping_add(0x08);
        let mut count = rd::<u32>(param_1 + 0x10) as u64;
        let lut = exe::ptr::<i8>(0x1_426d_01e0) as usize;

        loop {
            let half = count >> 1;
            let mut next_lo = lo.wrapping_sub(half).wrapping_add(count);
            count = half;
            let mid = half.wrapping_add(lo);
            let elem = arr_base.wrapping_add((mid as usize).wrapping_mul(0x88));

            let elem_b3 = rd::<u8>(elem + 0x03) as u32;
            let mut elem_str = elem.wrapping_add(0x08);
            let elem_cap_bits = if elem_b3 < 0x80 {
                elem_b3.wrapping_mul(8)
            } else {
                (elem_b3 << 8).wrapping_sub(0x7c00)
            };
            let elem_len_bits = (rd::<u32>(elem) & 0x00ff_ffff) << 3;
            if elem_len_bits > elem_cap_bits {
                elem_str = rd::<usize>(elem_str);
            }
            let elem_len = rd::<u32>(elem + 0x04) as u64;

            let p3_cap_bits = if p3_b3 < 0x80 {
                p3_b3.wrapping_mul(8)
            } else {
                (p3_b3 << 8).wrapping_sub(0x7c00)
            };
            let mut p3_str = p3_inline;
            if p3_len_bits > p3_cap_bits {
                p3_str = rd::<usize>(p3_inline);
            }

            let min_len = if elem_len < p3_len { elem_len } else { p3_len };
            let mut cmp_res: Option<i32> = None;
            if min_len != 0 {
                let mut cur = elem_str;
                let diff = p3_str.wrapping_sub(elem_str);
                let mut rem = min_len;
                while rem != 0 {
                    rem -= 1;
                    let c_p3 = rd::<i8>(lut.wrapping_add(rd::<u8>(cur.wrapping_add(diff)) as usize));
                    let c_el = rd::<i8>(lut.wrapping_add(rd::<u8>(cur) as usize));
                    if c_p3 < c_el {
                        cmp_res = Some(-1);
                        break;
                    }
                    if c_p3 > c_el {
                        cmp_res = Some(1);
                        break;
                    }
                    cur = cur.wrapping_add(1);
                }
            }

            let cmp_val = match cmp_res {
                Some(v) => v,
                None => {
                    if p3_len < elem_len {
                        -1
                    } else if p3_len > elem_len {
                        1
                    } else {
                        0
                    }
                }
            };

            arr_base = rd::<usize>(param_1 + 0x08);
            if cmp_val < 0 {
                next_lo = lo;
            }
            lo = next_lo;
            if half == 0 {
                break;
            }
        }
    }

    let mut idx = if lo == 0 { 0 } else { lo.wrapping_sub(1) };
    let base_08 = rd::<usize>(param_1 + 0x08);
    let cand_ptr = base_08.wrapping_add((idx as usize).wrapping_mul(0x88));
    if exe::call_va!(0x1_4021_74f0, (cand_ptr, param_3) -> u8) != 0 {
        idx = idx.wrapping_add(1);
    }

    let base_18 = rd::<u64>(param_1 + 0x18);
    let byte_off = idx.wrapping_mul(0x88);
    wr::<u64>(param_2, (base_08 as u64).wrapping_add(byte_off));
    let idx_div = sdiv_0x88(byte_off);
    wr::<u64>(
        param_2 + 0x08,
        base_18.wrapping_add(idx_div.wrapping_mul(0x120)),
    );
    param_2
}

/// `unknown_1412cbda0`
/// Original: 0x1412cbda0 (4043 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1412cbda0(param_1: usize, param_2: usize) {
    let r8_hdr = rd::<usize>(param_1);
    let count = rd::<u32>(r8_hdr + 0x10);
    if count == 0 || !(rd::<f32>(param_1 + 0x34) >= 0.0) {
        return;
    }

    let has_scale = rd::<u8>(param_1 + 0x1f);
    let num_words = count.wrapping_add(0x0f) >> 4;
    let r8_plus_20 = r8_hdr.wrapping_add(0x20);
    let mask_arr0 = r8_plus_20.wrapping_add(rd::<u32>(r8_hdr + 0x48) as usize);
    let rem_items = (num_words << 4).wrapping_sub(count);
    let last_word_idx = num_words.wrapping_sub(1);
    let tail_mask: u32 = if rem_items == 0 {
        0xffff_ffff
    } else {
        !(0xffff_ffffu32 >> ((16u32.wrapping_sub(rem_items).wrapping_mul(2)) & 0x1f))
    };

    let mut r13d_cnt = rd::<u32>(r8_plus_20 + 0x14);
    let mask_arr1 = mask_arr0.wrapping_add((num_words as usize) * 4);
    let mask_arr2 = mask_arr1.wrapping_add((num_words as usize) * 4);
    let mut r14_stream = r8_plus_20.wrapping_add(rd::<u32>(r8_plus_20 + 0x2c) as usize);

    let mode_1c = rd::<u8>(param_1 + 0x1c);
    let eff_mode = if mode_1c == 3 { 2 } else { mode_1c };
    let cnt_18 = rd::<u32>(r8_plus_20 + 0x18);

    let mut frame = [0u64; 0xa80 / 8];
    let rbp = (frame.as_mut_ptr() as usize).wrapping_add(0x80);

    wr::<u64>(rbp + 0x180, 0);
    wr::<u32>(rbp + 0x188, r13d_cnt);
    let rot_stride: u32 = if eff_mode == 0 { 0x10 } else { 0x0c };
    wr::<usize>(rbp + 0x190, r14_stream);
    let ptr_198 = r14_stream.wrapping_add(rot_stride.wrapping_mul(r13d_cnt) as usize);
    let ptr_1a0 = ptr_198.wrapping_add(cnt_18.wrapping_mul(0x0c) as usize);
    wr::<usize>(rbp + 0x198, ptr_198);
    wr::<usize>(rbp + 0x1a0, ptr_1a0);
    wr::<u64>(rbp + 0x3b0, 0);
    wr::<u64>(rbp + 0x5c0, 0);
    wr::<u64>(rbp + 0x7d0, 0);

    exe::call_va!(0x1_412c_d590, (rbp + 0x1b0, param_1));

    // Loop 1: Constant rotations
    let const_quat = rd::<u128>(exe::ptr::<u128>(0x1_4342_81d0) as usize);
    for word_idx in 0..=last_word_idx {
        let w = rd::<u32>(mask_arr0.wrapping_add((word_idx as usize) * 4));
        let m = if word_idx == last_word_idx {
            tail_mask
        } else {
            0xffff_ffff
        };
        let mut bits = m & (!w).wrapping_sub(0x5555_5555) & 0xaaaa_aaaa;
        let mut base_item = word_idx * 16;
        while bits != 0 {
            let cur = bits;
            let item = base_item;
            bits <<= 8;
            base_item += 4;
            if (cur & 0xaa00_0000) != 0 {
                for k in 0..4u32 {
                    if ((cur >> (31 - k * 2)) & 1) != 0 {
                        let out_idx = rd::<u16>(
                            rd::<usize>(param_2 + 0x08).wrapping_add(((item + k) as usize) * 2),
                        );
                        if out_idx != 0xffff {
                            wr::<u128>(
                                rd::<usize>(param_2).wrapping_add((out_idx as usize) * 0x20),
                                const_quat,
                            );
                        }
                    }
                }
            }
        }
    }

    // Loop 2: Static / palette rotations
    let mut edi_read = rd::<u32>(rbp + 0x184);
    for word_idx in 0..num_words {
        let w = rd::<u32>(mask_arr0.wrapping_add((word_idx as usize) * 4));
        let mut r11d = w & 0x5555_5555;
        let mut esi_item = word_idx * 16;

        if r13d_cnt != 0 {
            let ecx_write = rd::<u32>(rbp + 0x180);
            if ecx_write.wrapping_sub(edi_read) < 0x10 {
                let batch = if r13d_cnt > 0x10 { 0x10 } else { r13d_cnt };
                let mut r9_dst = (rbp - 0x80).wrapping_add(((ecx_write & 0x1f) as usize) * 0x10);
                r13d_cnt = r13d_cnt.wrapping_sub(batch);
                wr::<u32>(rbp + 0x188, r13d_cnt);
                wr::<u32>(rbp + 0x180, ecx_write.wrapping_add(batch));
                if batch != 0 {
                    if rd::<u8>(param_1 + 0x1c) == 0 {
                        for _ in 0..batch {
                            wr::<u128>(r9_dst, rd::<u128>(r14_stream));
                            r14_stream = r14_stream.wrapping_add(0x10);
                            r9_dst = r9_dst.wrapping_add(0x10);
                        }
                    } else {
                        let xmm8_abs = rd::<u128>(exe::ptr::<u128>(0x1_4342_8720) as usize);
                        let xmm9_one = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
                        let mut rem = batch;
                        while rem != 0 {
                            let step = if rem > 4 { 4 } else { rem };
                            rem -= step;
                            let xmm7 = rd::<u128>(r14_stream);
                            let xmm5 = rd::<u128>(r14_stream.wrapping_add((step as usize) * 4));
                            let xmm6 = rd::<u128>(r14_stream.wrapping_add((step as usize) * 8));
                            let xmm0_a = crate::simd_ops::vmulps(xmm7, xmm7);
                            let xmm2_a = crate::simd_ops::vsubps(xmm9_one, xmm0_a);
                            let xmm1_a = crate::simd_ops::vmulps(xmm5, xmm5);
                            let xmm3_a = crate::simd_ops::vsubps(xmm2_a, xmm1_a);
                            let xmm0_b = crate::simd_ops::vmulps(xmm6, xmm6);
                            let xmm2_b = crate::simd_ops::vsubps(xmm3_a, xmm0_b);
                            let xmm4 = crate::simd_ops::vsqrtps(xmm2_b & xmm8_abs);
                            let xmm3 = crate::simd_ops::vshufps(xmm7, xmm5, 0x44);
                            let xmm1 = crate::simd_ops::vshufps(xmm6, xmm4, 0x44);
                            let out0 = crate::simd_ops::vshufps(xmm3, xmm1, 0x88);
                            let out1 = crate::simd_ops::vshufps(xmm3, xmm1, 0xdd);
                            let xmm5_hi = crate::simd_ops::vshufps(xmm7, xmm5, 0xee);
                            let xmm2_hi = crate::simd_ops::vshufps(xmm6, xmm4, 0xee);
                            let out2 = crate::simd_ops::vshufps(xmm5_hi, xmm2_hi, 0x88);
                            let out3 = crate::simd_ops::vshufps(xmm5_hi, xmm2_hi, 0xdd);
                            wr::<u128>(r9_dst, out0);
                            wr::<u128>(r9_dst + 0x10, out1);
                            wr::<u128>(r9_dst + 0x20, out2);
                            wr::<u128>(r9_dst + 0x30, out3);
                            r9_dst = r9_dst.wrapping_add(0x40);
                            r14_stream = r14_stream.wrapping_add((step as usize) * 0x0c);
                        }
                    }
                    edi_read = rd::<u32>(rbp + 0x184);
                    r13d_cnt = rd::<u32>(rbp + 0x188);
                }
                wr::<usize>(rbp + 0x190, r14_stream);
            }
        }

        while r11d != 0 {
            let cur = r11d;
            let item = esi_item;
            r11d <<= 8;
            esi_item += 4;
            if (cur & 0x5500_0000) != 0 {
                for k in 0..4u32 {
                    if ((cur >> (30 - k * 2)) & 1) != 0 {
                        let ring_idx = edi_read;
                        edi_read = edi_read.wrapping_add(1);
                        wr::<u32>(rbp + 0x184, edi_read);
                        let out_idx = rd::<u16>(
                            rd::<usize>(param_2 + 0x08).wrapping_add(((item + k) as usize) * 2),
                        );
                        if out_idx != 0xffff {
                            let q = rd::<u128>(
                                (rbp - 0x80).wrapping_add(((ring_idx & 0x1f) as usize) * 0x10),
                            );
                            wr::<u128>(
                                rd::<usize>(param_2).wrapping_add((out_idx as usize) * 0x20),
                                q,
                            );
                        }
                    }
                }
            }
        }
    }

    // Loop 3: Constant translations
    for word_idx in 0..=last_word_idx {
        let w = rd::<u32>(mask_arr1.wrapping_add((word_idx as usize) * 4));
        let m = if word_idx == last_word_idx {
            tail_mask
        } else {
            0xffff_ffff
        };
        let mut bits = m & (!w).wrapping_sub(0x5555_5555) & 0xaaaa_aaaa;
        let mut base_item = word_idx * 16;
        while bits != 0 {
            let cur = bits;
            let item = base_item;
            bits <<= 8;
            base_item += 4;
            if (cur & 0xaa00_0000) != 0 {
                for k in 0..4u32 {
                    if ((cur >> (31 - k * 2)) & 1) != 0 {
                        let out_idx = rd::<u16>(
                            rd::<usize>(param_2 + 0x08).wrapping_add(((item + k) as usize) * 2),
                        );
                        if out_idx != 0xffff {
                            wr::<u128>(
                                rd::<usize>(param_2)
                                    .wrapping_add((out_idx as usize) * 0x20 + 0x10),
                                0,
                            );
                        }
                    }
                }
            }
        }
    }

    // Loop 4: Static / palette translations
    let mut rdx_trans = rd::<usize>(rbp + 0x198);
    for word_idx in 0..num_words {
        let w = rd::<u32>(mask_arr1.wrapping_add((word_idx as usize) * 4));
        let mut r10d = w & 0x5555_5555;
        let mut edi_item = word_idx * 16;
        while r10d != 0 {
            let cur = r10d;
            let item = edi_item;
            r10d <<= 8;
            edi_item += 4;
            if (cur & 0x5500_0000) != 0 {
                for k in 0..4u32 {
                    if ((cur >> (30 - k * 2)) & 1) != 0 {
                        let src_ptr = rdx_trans;
                        rdx_trans = rdx_trans.wrapping_add(0x0c);
                        let out_idx = rd::<u16>(
                            rd::<usize>(param_2 + 0x08).wrapping_add(((item + k) as usize) * 2),
                        );
                        if out_idx != 0xffff {
                            let xmm0 = rd::<u128>(src_ptr);
                            let xmm2 = crate::simd_ops::vblendps(xmm0, 0, 0x08);
                            wr::<u128>(
                                rd::<usize>(param_2)
                                    .wrapping_add((out_idx as usize) * 0x20 + 0x10),
                                xmm2,
                            );
                        }
                    }
                }
            }
        }
    }

    // Loop 5: Animated rotations
    for word_idx in 0..num_words {
        let w = rd::<u32>(mask_arr0.wrapping_add((word_idx as usize) * 4));
        let mut r14d = w & 0xaaaa_aaaa;
        let mut r15d_item = word_idx * 16;
        while r14d != 0 {
            let cur = r14d;
            let item = r15d_item;
            r14d <<= 8;
            r15d_item += 4;
            if (cur & 0xaa00_0000) != 0 {
                exe::call_va!(0x1_412c_e250, (rbp + 0x1b0, param_1));
                for k in 0..4u32 {
                    if ((cur >> (31 - k * 2)) & 1) != 0 {
                        let edx_idx = rd::<u32>(rbp + 0x3b4);
                        wr::<u32>(rbp + 0x3b4, edx_idx.wrapping_add(1));
                        let out_idx = rd::<u16>(
                            rd::<usize>(param_2 + 0x08).wrapping_add(((item + k) as usize) * 2),
                        );
                        if out_idx != 0xffff {
                            let xmm0 = rd::<u128>(
                                (rbp + 0x1b0).wrapping_add(((edx_idx & 7) as usize) * 0x10),
                            );
                            wr::<u128>(
                                rd::<usize>(param_2).wrapping_add((out_idx as usize) * 0x20),
                                xmm0,
                            );
                        }
                    }
                }
            }
        }
    }

    // Loop 6: Animated translations
    for word_idx in 0..num_words {
        let w = rd::<u32>(mask_arr1.wrapping_add((word_idx as usize) * 4));
        let mut esi = w & 0xaaaa_aaaa;
        let mut r12d_item = word_idx * 16;
        while esi != 0 {
            let cur = esi;
            let item = r12d_item;
            esi <<= 8;
            r12d_item += 4;
            if (cur & 0xaa00_0000) != 0 {
                let edi_rem = rd::<u32>(rbp + 0x5c8);
                if edi_rem != 0 {
                    let r14d_wr = rd::<u32>(rbp + 0x5c0);
                    if r14d_wr.wrapping_sub(rd::<u32>(rbp + 0x5c4)) < 4 {
                        let batch = if edi_rem > 4 { 4 } else { edi_rem };
                        wr::<u32>(rbp + 0x5c8, edi_rem.wrapping_sub(batch));
                        wr::<u32>(rbp + 0x5c0, r14d_wr.wrapping_add(batch));
                        let slot_idx = (r14d_wr & 7) as usize;
                        exe::call_va!(
                            0x1_412c_ece0,
                            (param_1, rbp + 0x7e0, batch, rbp + 0x868, rbp + 0x8b8)
                        );
                        exe::call_va!(
                            0x1_412c_ece0,
                            (param_1, rbp + 0x820, batch, rbp + 0x868, rbp + 0x8d8)
                        );
                        let xmm4 = crate::simd_ops::vbroadcastss(rd::<u32>(param_1 + 0x78));
                        for i in 0..(batch as usize) {
                            let xmm1_a = rd::<u128>((rbp + 0x7e0).wrapping_add(i * 0x10));
                            let xmm0_a = crate::simd_ops::vmulps(xmm4, xmm1_a);
                            let xmm2_a = crate::simd_ops::vsubps(xmm1_a, xmm0_a);
                            let xmm1_b = crate::simd_ops::vmulps(
                                xmm4,
                                rd::<u128>((rbp + 0x820).wrapping_add(i * 0x10)),
                            );
                            let xmm2_b = crate::simd_ops::vaddps(xmm2_a, xmm1_b);
                            wr::<u128>((rbp + 0x3c0).wrapping_add((slot_idx + i) * 0x10), xmm2_b);
                        }
                        if rd::<u8>(param_1 + 0x1d) == 1 {
                            let old_868 = rd::<u64>(rbp + 0x868);
                            wr::<u64>(
                                rbp + 0x868,
                                old_868.wrapping_add((batch as u64).wrapping_mul(0x18)),
                            );
                        }
                    }
                }
                for k in 0..4u32 {
                    if ((cur >> (31 - k * 2)) & 1) != 0 {
                        let edx_idx = rd::<u32>(rbp + 0x5c4);
                        wr::<u32>(rbp + 0x5c4, edx_idx.wrapping_add(1));
                        let out_idx = rd::<u16>(
                            rd::<usize>(param_2 + 0x08).wrapping_add(((item + k) as usize) * 2),
                        );
                        if out_idx != 0xffff {
                            let xmm1 = rd::<u128>(
                                (rbp + 0x3c0).wrapping_add(((edx_idx & 7) as usize) * 0x10),
                            );
                            let xmm2 = crate::simd_ops::vinsertps(xmm1, 0, 0xf0);
                            wr::<u128>(
                                rd::<usize>(param_2)
                                    .wrapping_add((out_idx as usize) * 0x20 + 0x10),
                                xmm2,
                            );
                        }
                    }
                }
            }
        }
    }

    // Loop 7: Animated scales
    if has_scale != 0 {
        let mut edx_rem = rd::<u32>(rbp + 0x7d8);
        let mut ecx_rd = rd::<u32>(rbp + 0x7d4);
        let mut r8d_wr = rd::<u32>(rbp + 0x7d0);
        for word_idx in 0..num_words {
            let w = rd::<u32>(mask_arr2.wrapping_add((word_idx as usize) * 4));
            let mut esi = w & 0xaaaa_aaaa;
            while esi != 0 {
                let cur = esi;
                esi <<= 8;
                if (cur & 0xaa00_0000) != 0 {
                    if edx_rem != 0 && r8d_wr.wrapping_sub(ecx_rd) < 4 {
                        let batch = if edx_rem > 4 { 4 } else { edx_rem };
                        let slot_idx = (r8d_wr & 7) as usize;
                        edx_rem = edx_rem.wrapping_sub(batch);
                        r8d_wr = r8d_wr.wrapping_add(batch);
                        wr::<u32>(rbp + 0x7d8, edx_rem);
                        wr::<u32>(rbp + 0x7d0, r8d_wr);
                        exe::call_va!(
                            0x1_412c_ef60,
                            (0usize, rbp + 0x7e0, batch, 0usize, rbp + 0x8f8)
                        );
                        exe::call_va!(
                            0x1_412c_ef60,
                            (0usize, rbp + 0x820, batch, 0usize, rbp + 0x918)
                        );
                        let xmm4 = crate::simd_ops::vbroadcastss(rd::<u32>(param_1 + 0x78));
                        for i in 0..(batch as usize) {
                            let xmm1_a = rd::<u128>((rbp + 0x7e0).wrapping_add(i * 0x10));
                            let xmm0_a = crate::simd_ops::vmulps(xmm4, xmm1_a);
                            let xmm2_a = crate::simd_ops::vsubps(xmm1_a, xmm0_a);
                            let xmm1_b = crate::simd_ops::vmulps(
                                xmm4,
                                rd::<u128>((rbp + 0x820).wrapping_add(i * 0x10)),
                            );
                            let xmm2_b = crate::simd_ops::vaddps(xmm2_a, xmm1_b);
                            wr::<u128>((rbp + 0x5d0).wrapping_add((slot_idx + i) * 0x10), xmm2_b);
                        }
                        r8d_wr = rd::<u32>(rbp + 0x7d0);
                        ecx_rd = rd::<u32>(rbp + 0x7d4);
                        edx_rem = rd::<u32>(rbp + 0x7d8);
                    }
                    for k in 0..4u32 {
                        if ((cur >> (31 - k * 2)) & 1) != 0 {
                            ecx_rd = ecx_rd.wrapping_add(1);
                            wr::<u32>(rbp + 0x7d4, ecx_rd);
                        }
                    }
                }
            }
        }
    }
}

/// `rend_1412eb5b0`
/// Original: 0x1412eb5b0 (222 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412eb5b0(param_1: usize, param_2: u64, param_3: usize) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > cap {
        let mut new_cap = (cap >> 1).wrapping_add(cap);
        if new_cap < param_2 {
            new_cap = param_2;
        }
        exe::call_va!(0x1_401d_eb40, (param_1, new_cap));
        let mut diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let mut dst = rd::<usize>(param_1).wrapping_add(cur_len * 4);
            if rd::<u32>(param_3) == 0 {
                let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff.wrapping_mul(4)) -> usize);
                wr::<u32>(param_1 + 0x08, param_2 as u32);
                return;
            }
            while diff != 0 {
                wr::<u32>(dst, rd::<u32>(param_3));
                dst = dst.wrapping_add(4);
                diff -= 1;
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 <= old_len {
        if param_2 != old_len {
            wr::<u32>(param_1 + 0x08, param_2 as u32);
        }
    } else {
        let mut diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let mut dst = rd::<usize>(param_1).wrapping_add((old_len as usize) * 4);
            if rd::<u32>(param_3) == 0 {
                let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff.wrapping_mul(4)) -> usize);
                wr::<u32>(param_1 + 0x08, param_2 as u32);
                return;
            }
            while diff != 0 {
                wr::<u32>(dst, rd::<u32>(param_3));
                dst = dst.wrapping_add(4);
                diff -= 1;
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

#[inline(always)]
unsafe fn rstring_view_at(base: usize) -> [u64; 2] {
    let b3 = rd::<u8>(base + 0x03) as u32;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(base) & 0x00ff_ffff) << 3;
    let data_ptr = if len_bits > cap_bits {
        rd::<u64>(base + 0x08)
    } else {
        (base + 0x08) as u64
    };
    let len = rd::<u32>(base + 0x04) as u64;
    [data_ptr, len]
}

#[inline(always)]
unsafe fn global_rstring_view(va: u64) -> [u64; 2] {
    rstring_view_at(exe::ptr::<u8>(va) as usize)
}

/// `unknown_141892bd0`
/// Original: 0x141892bd0 (2113 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141892bd0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
) {
    let p1_0 = rd::<u64>(param_1);
    let idx = rd::<u64>(param_1 + 0x10);
    let local_70 = (p1_0 as usize).wrapping_add((idx as usize).wrapping_mul(0x18));

    let mut buf_30: [u64; 3] = [0; 3];
    let v0 = global_rstring_view(0x1_43a1_0978);
    buf_30[0] = v0[0];
    buf_30[1] = v0[1];
    buf_30[2] = idx;
    let mut out_4f: u64 = 0;
    let _ = exe::call_va!(
        0x1_401e_8cc0,
        (
            (&mut out_4f as *mut u64) as usize,
            buf_30.as_mut_ptr() as usize,
            param_2
        ) -> usize
    );
    if (out_4f as u32) == 0xffff_ffff {
        return;
    }

    let _ = exe::call_va!(
        0x1_401e_8c00,
        (param_2, buf_30.as_mut_ptr() as usize, out_4f) -> usize
    );
    let rsi = (buf_30[0] as usize).wrapping_add((buf_30[2] as usize).wrapping_mul(0x140));
    let mut view_30: [u64; 2];

    let g_9c0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_d9c0) as usize);
    let dil_18 = if g_9c0 != 0 {
        rd::<u8>(g_9c0 + 0x05) != 0 && rd::<u8>(g_9c0 + 0x04) != 0
    } else {
        false
    };

    let id_1e8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_01e8) as usize) as usize;
    let tbl_580e0 = rd::<usize>(param_3 + 0x580e0);
    let local_res10 = rd::<usize>(tbl_580e0.wrapping_add(id_1e8 * 8));

    let r12_19 = if dil_18 {
        false
    } else {
        rd::<u8>(local_res10) != 0
            && rd::<u8>(exe::ptr::<u8>(0x1_4384_4628) as usize) == 0
            && exe::call_va!(0x1_4189_3440, (param_3) -> u8) == 0
    };

    let p_598 = exe::call_va!(
        0x1_401e_8210,
        (param_4, exe::ptr::<u8>(0x1_4384_4598) as usize) -> usize
    );
    let has_598 = p_598 != 0 && rd::<u64>(p_598 + 0x08) != 0;

    let p_690 = exe::call_va!(
        0x1_401e_8210,
        (param_4, exe::ptr::<u8>(0x1_4384_4690) as usize) -> usize
    );
    let has_690 = p_690 != 0 && rd::<u64>(p_690 + 0x08) != 0;

    let p_630 = exe::call_va!(
        0x1_401e_8210,
        (param_4, exe::ptr::<u8>(0x1_4384_4630) as usize) -> usize
    );
    let has_630 = p_630 != 0 && rd::<u64>(p_630 + 0x08) != 0;

    let mut str_68 = [0u64; 6];
    let _ = exe::call_va!(0x1_41df_ae90, (str_68.as_mut_ptr() as usize, rsi) -> usize);

    let mut str_98 = [0u64; 6];
    wr::<u64>(str_98.as_mut_ptr() as usize, 0x0400_0004);
    wr::<u8>(str_98.as_mut_ptr() as usize + 0x08, 0);
    view_30 = [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
    exe::call_va!(
        0x1_4240_9b50,
        (
            str_98.as_mut_ptr() as usize,
            view_30.as_mut_ptr() as usize,
            0u32,
            0u32
        )
    );

    view_30 = if dil_18 {
        global_rstring_view(0x1_43a1_11b8)
    } else if !r12_19 {
        global_rstring_view(0x1_43a1_0e70)
    } else if has_598 {
        global_rstring_view(0x1_43a1_0c40)
    } else if rd::<u8>(param_6 + 0xad) != 0 || has_690 {
        global_rstring_view(0x1_43a1_0dd0)
    } else if rd::<u8>(param_5) != 0 {
        global_rstring_view(0x1_43a1_0928)
    } else if has_630 {
        global_rstring_view(0x1_43a1_0a38)
    } else if rd::<u32>(local_70).wrapping_sub(2) <= 2 {
        global_rstring_view(0x1_43a1_0950)
    } else {
        global_rstring_view(0x1_43a1_0900)
    };

    let append_len = view_30[1] as u32;
    exe::call_va!(
        0x1_4240_9b50,
        (
            str_98.as_mut_ptr() as usize,
            view_30.as_mut_ptr() as usize,
            0u32,
            append_len
        )
    );

    let v_68 = rstring_view_at(str_68.as_ptr() as usize);
    let v_98 = rstring_view_at(str_98.as_ptr() as usize);
    let mut same_str = v_98[1] == v_68[1];
    if same_str && v_98[1] != 0 {
        let lut = exe::ptr::<i8>(0x1_426d_01e0) as usize;
        let p_68 = v_68[0] as usize;
        let p_98 = v_98[0] as usize;
        for i in 0..(v_98[1] as usize) {
            let c98 = rd::<i8>(lut.wrapping_add(rd::<u8>(p_98.wrapping_add(i)) as usize));
            let c68 = rd::<i8>(lut.wrapping_add(rd::<u8>(p_68.wrapping_add(i)) as usize));
            if c98 != c68 {
                same_str = false;
                break;
            }
        }
    }

    if !same_str {
        view_30 = v_98;
        if rd::<u32>(rsi + 0x128) > 0 {
            exe::call_va!(0x1_41df_99a0, (rsi, view_30.as_mut_ptr() as usize, 0u32));
        } else {
            exe::call_va!(0x1_41df_a6f0, (rsi, view_30.as_mut_ptr() as usize));
        }
    }

    let p7_val = rd::<u32>(param_7);
    let ne_a38 = p7_val != rd::<u32>(exe::ptr::<u32>(0x1_43a2_9a38) as usize);
    let eq_c58 = p7_val == rd::<u32>(exe::ptr::<u32>(0x1_43a2_9c58) as usize);
    view_30 = global_rstring_view(0x1_43a1_11b8);
    let find_res = exe::call_va!(0x1_41df_a9a0, (rsi, view_30.as_mut_ptr() as usize) -> i32);
    let eq_c2c = p7_val == rd::<u32>(exe::ptr::<u32>(0x1_43a2_9c2c) as usize);

    let mut tmp_str_30 = [0u64; 6];
    let rax_str = exe::call_va!(
        0x1_41df_ae90,
        (tmp_str_30.as_mut_ptr() as usize, rsi) -> usize
    );
    let is_dd0 = exe::call_va!(
        0x1_401e_6ff0,
        (rax_str, exe::ptr::<u8>(0x1_43a1_0dd0) as usize) -> u8
    );
    exe::call_va!(0x1_4240_ab10, (tmp_str_30.as_mut_ptr() as usize));

    let mut flag_byte: u8 = if (ne_a38 && find_res == -1) || eq_c2c {
        1
    } else {
        0
    };
    exe::call_va!(
        0x1_401e_acd0,
        (
            param_4,
            exe::ptr::<u8>(0x1_4384_4670) as usize,
            0usize,
            (&mut flag_byte as *mut u8) as usize,
            0u32
        )
    );

    flag_byte = if eq_c58 || find_res != -1 || is_dd0 != 0 {
        1
    } else {
        0
    };
    exe::call_va!(
        0x1_401e_acd0,
        (
            param_4,
            exe::ptr::<u8>(0x1_4384_45d8) as usize,
            0usize,
            (&mut flag_byte as *mut u8) as usize,
            0u32
        )
    );

    exe::call_va!(0x1_4189_37b0, (local_res10, param_4, rsi));
    let _ = exe::call_va!(
        0x1_41df_ae90,
        (tmp_str_30.as_mut_ptr() as usize, rsi) -> usize
    );

    for (str_va, off) in [
        (0x1_43a1_1078u64, 0x18usize),
        (0x1_43a1_0928, 0x28),
        (0x1_43a1_10c8, 0x38),
        (0x1_43a1_0c68, 0x48),
        (0x1_43a1_0f38, 0x58),
    ] {
        flag_byte = exe::call_va!(
            0x1_401e_6ff0,
            (
                tmp_str_30.as_mut_ptr() as usize,
                exe::ptr::<u8>(str_va) as usize
            ) -> u8
        );
        exe::call_va!(
            0x1_401e_acd0,
            (
                param_4,
                local_res10.wrapping_add(off),
                0usize,
                (&mut flag_byte as *mut u8) as usize,
                0u32
            )
        );
    }

    exe::call_va!(0x1_4240_ab10, (tmp_str_30.as_mut_ptr() as usize));
    exe::call_va!(0x1_4240_ab10, (str_98.as_mut_ptr() as usize));
    exe::call_va!(0x1_4240_ab10, (str_68.as_mut_ptr() as usize));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1411641a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_7c80() -> u32 {
        2
    }
    unsafe extern "win64" fn mock_4af0(
        out_ptr: usize,
        arch: u16,
        block: u64,
        cnt: u32,
        world: u64,
    ) -> usize {
        wr::<u64>(out_ptr, arch as u64);
        wr::<u64>(out_ptr + 0x08, block);
        wr::<u64>(out_ptr + 0x10, world);
        wr::<u64>(out_ptr + 0x18, cnt as u64);
        for i in 4..17usize {
            wr::<u64>(out_ptr + i * 8, 0x5000 + i as u64);
        }
        out_ptr
    }
    unsafe extern "win64" fn mock_42f0(_unused_rcx: usize, pkt_ptr: usize, r8_ptr: usize) {
        let mut pkt = rd::<[u64; 17]>(pkt_ptr);
        let mut r8_arr = rd::<[u64; 4]>(r8_ptr);
        let world = pkt[2] as usize;
        pkt[2] = 0;
        r8_arr[0] = r8_arr[0].wrapping_sub(world as u64);
        let dst = world + 0x200;
        wr::<[u64; 17]>(dst, pkt);
        wr::<[u64; 4]>(dst + 17 * 8, r8_arr);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4116_4af0, mock_4af0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4116_42f0, mock_42f0 as *const ())?;

    let g_910 = exe::ptr::<u16>(0x1_43c8_d910) as usize;
    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let g_948 = exe::ptr::<u16>(0x1_43cc_8948) as usize;
    let saved = (rd::<u16>(g_910), rd::<u16>(g_63c), rd::<u16>(g_948));
    wr::<u16>(g_910, 0);
    wr::<u16>(g_63c, 1);
    wr::<u16>(g_948, 2);

    let run = |f: F, arch: u16| -> [u64; 21] {
        let tbl_580e0: [u64; 3] = [0xaaaa_1111, 0xbbbb_2222, 0xcccc_3333];
        let mut world = vec![0u8; 0x58600];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 3 * 8, 0x80_0000);
        wr::<u32>(wp + 0x10088 + 3 * 4, 7);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        rd::<[u64; 21]>(wp + 0x200)
    };

    for arch in [1u16, 3u16] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        wr::<u16>(g_910, saved.0);
        wr::<u16>(g_63c, saved.1);
        wr::<u16>(g_948, saved.2);
        crate::diff_eq!(o, n, "1411641a0 arch={}", arch);
        wr::<u16>(g_910, 0);
        wr::<u16>(g_63c, 1);
        wr::<u16>(g_948, 2);
    }
    wr::<u16>(g_910, saved.0);
    wr::<u16>(g_63c, saved.1);
    wr::<u16>(g_948, saved.2);
    Ok(2)
}

pub unsafe fn diff_unknown_1411f7760(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PREP_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_76e0(_p1: usize) {
        PREP_CALLS += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_76e0, mock_76e0 as *const ())?;

    let write_sso = |dst: usize, s: &[u8]| {
        let len = s.len() as u32;
        wr::<u32>(dst, (16u32 << 24) | len);
        wr::<u32>(dst + 0x04, len);
        for (i, &b) in s.iter().enumerate() {
            wr::<u8>(dst + 0x08 + i, b);
        }
    };

    let run = |f: F, count: u32, query: &[u8]| -> (u64, u64, u32) {
        PREP_CALLS = 0;
        let mut elems = [0u8; 0x88 * 4];
        let ep = elems.as_mut_ptr() as usize;
        write_sso(ep, b"alpha");
        write_sso(ep + 0x88, b"charlie");
        write_sso(ep + 0x88 * 2, b"echo");
        write_sso(ep + 0x88 * 3, b"golf");

        let mut p1 = [0u64; 4];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, ep);
        wr::<u32>(p1p + 0x10, count);
        wr::<u64>(p1p + 0x18, 0x10_0000);

        let mut p3 = [0u8; 0x30];
        let p3p = p3.as_mut_ptr() as usize;
        write_sso(p3p, query);

        let mut out = [0u64; 2];
        let op = out.as_mut_ptr() as usize;
        let ret = f(p1p, op, p3p);
        assert_eq!(ret, op);
        (
            out[0].wrapping_sub(ep as u64),
            out[1].wrapping_sub(0x10_0000),
            PREP_CALLS,
        )
    };

    for (cnt, q) in [
        (0u32, &b"charlie"[..]),
        (4, &b"aaa"[..]),
        (4, &b"bravo"[..]),
        (4, &b"CHARLIE"[..]),
        (4, &b"zulu"[..]),
    ] {
        let o = run(f_o, cnt, q);
        let n = run(f_n, cnt, q);
        crate::diff_eq!(o, n, "1411f7760 cnt={} q={:?}", cnt, q);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1412cbda0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCALE_CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_d590(ctx_1b0: usize, _p1: usize) {
        // Set remaining counts for animated rot (+0x208), animated trans (+0x418), animated scale (+0x628)
        wr::<u32>(ctx_1b0 + 0x208, 2);
        wr::<u32>(ctx_1b0 + 0x418, 2);
        wr::<u32>(ctx_1b0 + 0x628, 2);
        wr::<u64>(ctx_1b0 + 0x6b8, 0x5000); // rbp + 0x868
    }
    unsafe extern "win64" fn mock_e250(ctx_1b0: usize, _p1: usize) {
        let rem = rd::<u32>(ctx_1b0 + 0x208);
        let wr_idx = rd::<u32>(ctx_1b0 + 0x200);
        let rd_idx = rd::<u32>(ctx_1b0 + 0x204);
        if rem != 0 && wr_idx.wrapping_sub(rd_idx) < 4 {
            let batch = if rem > 4 { 4 } else { rem };
            wr::<u32>(ctx_1b0 + 0x208, rem - batch);
            wr::<u32>(ctx_1b0 + 0x200, wr_idx + batch);
            let slot = (wr_idx & 7) as usize;
            for i in 0..(batch as usize) {
                let v = (0.25f32 + i as f32).to_bits() as u128;
                let q = v | (v << 32) | (v << 64) | (v << 96);
                wr::<u128>(ctx_1b0.wrapping_add((slot + i) * 0x10), q);
            }
        }
    }
    unsafe extern "win64" fn mock_ece0(
        _p1: usize,
        dst_buf: usize,
        batch: u32,
        r9_868: usize,
        _arg5: usize,
    ) {
        let seed = rd::<u64>(r9_868) as f32 * 0.001;
        for i in 0..(batch as usize) {
            let f0 = (1.0f32 + i as f32 + seed).to_bits() as u128;
            let f1 = (2.0f32 + i as f32 + seed).to_bits() as u128;
            let f2 = (3.0f32 + i as f32 + seed).to_bits() as u128;
            let f3 = (4.0f32 + i as f32 + seed).to_bits() as u128;
            wr::<u128>(
                dst_buf.wrapping_add(i * 0x10),
                f0 | (f1 << 32) | (f2 << 64) | (f3 << 96),
            );
        }
    }
    unsafe extern "win64" fn mock_ef60(
        _rcx: usize,
        dst_buf: usize,
        batch: u32,
        _r9: usize,
        _arg5: usize,
    ) {
        SCALE_CALL_LOG = SCALE_CALL_LOG.wrapping_add(batch as u64);
        for i in 0..(batch as usize) {
            let v = (1.5f32 + i as f32).to_bits() as u128;
            wr::<u128>(
                dst_buf.wrapping_add(i * 0x10),
                v | (v << 32) | (v << 64) | (v << 96),
            );
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_412c_d590, mock_d590 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412c_e250, mock_e250 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412c_ece0, mock_ece0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412c_ef60, mock_ef60 as *const ())?;

    let run = |f: F, mode_1c: u8| -> ([u32; 32], u64) {
        SCALE_CALL_LOG = 0;

        let mut clip_blob = [0u8; 0x200];
        let cp = clip_blob.as_mut_ptr() as usize;
        wr::<u32>(cp + 0x10, 4); // count = 4 joints
        wr::<u32>(cp + 0x34, 2); // static rot count = 2
        wr::<u32>(cp + 0x38, 2); // static trans count = 2
        wr::<u32>(cp + 0x48, 0x40); // mask_arr0 offset from cp + 0x20 => cp + 0x60
        wr::<u32>(cp + 0x4c, 0x60); // stream offset from cp + 0x20 => cp + 0x80

        // 4 joints in top 8 bits of 32-bit mask word:
        // joint 0 (bits 31..30) = 00 (const)
        // joint 1 (bits 29..28) = 01 (static)
        // joint 2 (bits 27..26) = 10 (animated)
        // joint 3 (bits 25..24) = 01 (static)
        let mask_word: u32 = (0b00 << 30) | (0b01 << 28) | (0b10 << 26) | (0b01 << 24);
        wr::<u32>(cp + 0x60, mask_word); // mask_arr0 (rot)
        wr::<u32>(cp + 0x64, mask_word); // mask_arr1 (trans)
        wr::<u32>(cp + 0x68, 0b1010_0000 << 24); // mask_arr2 (scale: 2 animated joints)

        // Fill static stream data at cp + 0x80 with small valid floats (< 0.5 so 1 - x^2 - y^2 - z^2 > 0)
        for i in 0..24usize {
            wr::<f32>(cp + 0x80 + i * 4, 0.1 + (i as f32) * 0.01);
        }

        let mut p1 = [0u8; 0x90];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, cp);
        wr::<u8>(p1p + 0x1c, mode_1c);
        wr::<u8>(p1p + 0x1d, 1);
        wr::<u8>(p1p + 0x1f, 1); // has_scale = 1
        wr::<f32>(p1p + 0x34, 1.0); // weight >= 0.0
        wr::<f32>(p1p + 0x78, 0.5); // blend factor

        let mut out_pose = [0u32; 32]; // 4 joints * 8 u32s (0x20 bytes each)
        let mut joint_map: [u16; 4] = [0, 1, 2, 3];
        let p2: [usize; 2] = [
            out_pose.as_mut_ptr() as usize,
            joint_map.as_mut_ptr() as usize,
        ];

        f(p1p, p2.as_ptr() as usize);
        (out_pose, SCALE_CALL_LOG)
    };

    for mode_1c in [0u8, 1u8] {
        let o = run(f_o, mode_1c);
        let n = run(f_n, mode_1c);
        crate::diff_eq!(o, n, "1412cbda0 mode_1c={}", mode_1c);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412eb5b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESERVE_ARG: u64 = 0;

    unsafe extern "win64" fn mock_eb40(vec_ptr: usize, new_cap: u64) {
        RESERVE_ARG = new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_eb40, mock_eb40 as *const ())?;

    let run = |f: F, init_len: u32, init_cap: u32, new_len: u64, fill_val: u32| -> ([u32; 12], u32, u32, u64) {
        RESERVE_ARG = 0;
        let mut storage = [0xdead_beefu32; 12];
        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, storage.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, init_len);
        wr::<u32>(vp + 0x0c, init_cap);

        f(vp, new_len, (&fill_val as *const u32) as usize);
        (
            storage,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            RESERVE_ARG,
        )
    };

    for (len, cap, target, fill) in [
        (2u32, 4u32, 6u64, 0u32),
        (2, 4, 8, 0x1234_5678),
        (2, 8, 5, 0),
        (2, 8, 5, 0xabcd_ef01),
        (5, 8, 3, 99),
        (5, 8, 5, 99),
    ] {
        let o = run(f_o, len, cap, target, fill);
        let n = run(f_n, len, cap, target, fill);
        crate::diff_eq!(o, n, "1412eb5b0 len={} cap={} target={} fill={}", len, cap, target, fill);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_141892bd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MOCK_FOUND_8CC0: i32 = 1;
    static mut MOCK_ELEM_BUF: [u8; 0x300] = [0; 0x300];
    static mut ACCUM_HASH: u64 = 0;

    unsafe extern "win64" fn mock_8cc0(out_ptr: usize, _view: usize, _p2: usize) -> usize {
        wr::<i32>(out_ptr, MOCK_FOUND_8CC0);
        out_ptr
    }
    unsafe extern "win64" fn mock_8c00(_p2: usize, out_view: usize, r8_idx: u64) -> usize {
        wr::<usize>(out_view, core::ptr::addr_of_mut!(MOCK_ELEM_BUF) as usize);
        wr::<u64>(out_view + 0x08, 2);
        wr::<u64>(out_view + 0x10, r8_idx);
        out_view
    }
    unsafe extern "win64" fn mock_3440(_world: usize) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_8210(p4: usize, _key: usize) -> usize {
        p4
    }
    unsafe extern "win64" fn mock_ae90(out_rstr: usize, _rsi: usize) -> usize {
        wr::<u32>(out_rstr, (16u32 << 24) | 4);
        wr::<u32>(out_rstr + 0x04, 4);
        wr::<u32>(out_rstr + 0x08, u32::from_le_bytes(*b"test"));
        out_rstr
    }
    unsafe extern "win64" fn mock_9b50(
        dst_rstr: usize,
        view_ptr: usize,
        _off: u32,
        len: u32,
    ) {
        let v_len = rd::<u64>(view_ptr + 0x08);
        ACCUM_HASH = ACCUM_HASH
            .wrapping_mul(31)
            .wrapping_add(v_len)
            .wrapping_add(len as u64);
        wr::<u32>(dst_rstr, (16u32 << 24) | 4);
        wr::<u32>(dst_rstr + 0x04, 4);
        wr::<u32>(dst_rstr + 0x08, u32::from_le_bytes(*b"diff"));
    }
    unsafe extern "win64" fn mock_99a0(_rsi: usize, view_ptr: usize, flag: u32) {
        let len = rd::<u64>(view_ptr + 0x08);
        ACCUM_HASH = ACCUM_HASH
            .wrapping_mul(37)
            .wrapping_add(0x99a0)
            .wrapping_add(len)
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_a6f0(_rsi: usize, view_ptr: usize) {
        let len = rd::<u64>(view_ptr + 0x08);
        ACCUM_HASH = ACCUM_HASH
            .wrapping_mul(41)
            .wrapping_add(0xa6f0)
            .wrapping_add(len);
    }
    unsafe extern "win64" fn mock_a9a0(_rsi: usize, _view_ptr: usize) -> i32 {
        -1
    }
    unsafe extern "win64" fn mock_6ff0(_rstr: usize, global_str: usize) -> u8 {
        ((global_str >> 4) & 1) as u8
    }
    unsafe extern "win64" fn mock_ab10(_rstr: usize) {
        ACCUM_HASH = ACCUM_HASH.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_acd0(
        _p4: usize,
        _rdx: usize,
        _r8: usize,
        flag_ptr: usize,
        arg5: u32,
    ) {
        let b = rd::<u8>(flag_ptr) as u64;
        ACCUM_HASH = ACCUM_HASH
            .wrapping_mul(43)
            .wrapping_add(b)
            .wrapping_add(arg5 as u64);
    }
    unsafe extern "win64" fn mock_37b0(res10: usize, _p4: usize, _rsi: usize) {
        ACCUM_HASH = ACCUM_HASH
            .wrapping_mul(47)
            .wrapping_add(rd::<u8>(res10) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8cc0, mock_8cc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_8c00, mock_8c00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4189_3440, mock_3440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_8210, mock_8210 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41df_ae90, mock_ae90 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41df_99a0, mock_99a0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41df_a6f0, mock_a6f0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_41df_a9a0, mock_a9a0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_401e_acd0, mock_acd0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4189_37b0, mock_37b0 as *const ())?;

    let g_9c0 = exe::ptr::<usize>(0x1_4397_d9c0) as usize;
    let g_1e8 = exe::ptr::<u16>(0x1_43c9_01e8) as usize;
    let g_628 = exe::ptr::<u8>(0x1_4384_4628) as usize;
    let s_9c0 = rd::<usize>(g_9c0);
    let s_1e8 = rd::<u16>(g_1e8);
    let s_628 = rd::<u8>(g_628);

    let run = |f: F, found: i32, mode_128: u32, p5_flag: u8| -> u64 {
        MOCK_FOUND_8CC0 = found;
        MOCK_ELEM_BUF = [0; 0x300];
        ACCUM_HASH = 0;

        wr::<u32>(
            core::ptr::addr_of_mut!(MOCK_ELEM_BUF) as usize + 0x140 + 0x128,
            mode_128,
        );
        wr::<usize>(g_9c0, 0);
        wr::<u16>(g_1e8, 0);
        wr::<u8>(g_628, 0);

        let mut comp_arr = [0u32; 12];
        comp_arr[6] = 3; // at idx=1 (offset 0x18 = 6 u32s), value 3 -> (3 - 2) <= 2

        let mut p1: [u64; 3] = [comp_arr.as_mut_ptr() as u64, 0, 1];
        let mut p2: [u64; 3] = [0; 3];

        let mut res10_obj = [0u8; 0x80];
        res10_obj[0] = 1;
        let tbl_580e0: [usize; 1] = [res10_obj.as_mut_ptr() as usize];
        let mut world = vec![0u8; 0x58100];
        wr::<usize>(world.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p4: [u64; 2] = [0, 0]; // has_598/690/630 = false
        let p5: [u8; 4] = [p5_flag, 0, 0, 0];
        let p6 = [0u8; 0xc0];
        let p7: [u32; 1] = [0x1234_5678];

        f(
            p1.as_mut_ptr() as usize,
            p2.as_mut_ptr() as usize,
            world.as_mut_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            p6.as_ptr() as usize,
            p7.as_ptr() as usize,
        );
        ACCUM_HASH
    };

    for (found, mode_128, p5_flag) in [(-1i32, 0u32, 0u8), (1, 0, 0), (1, 1, 1)] {
        let o = run(f_o, found, mode_128, p5_flag);
        let n = run(f_n, found, mode_128, p5_flag);
        wr::<usize>(g_9c0, s_9c0);
        wr::<u16>(g_1e8, s_1e8);
        wr::<u8>(g_628, s_628);
        crate::diff_eq!(o, n, "141892bd0 found={} mode={} p5={}", found, mode_128, p5_flag);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4116_41a0, "unknown_1411641a0", rend::cauldron_hot_v2::unknown_1411641a0 as *const (), Throws::Discard, rend::cauldron_hot_v2::diff_unknown_1411641a0),
// Replacement::verified(0x1_411f_7760, "unknown_1411f7760", rend::cauldron_hot_v2::unknown_1411f7760 as *const (), rend::cauldron_hot_v2::diff_unknown_1411f7760),
// Replacement::verified(0x1_412c_bda0, "unknown_1412cbda0", rend::cauldron_hot_v2::unknown_1412cbda0 as *const (), rend::cauldron_hot_v2::diff_unknown_1412cbda0),
// Replacement::verified(0x1_412e_b5b0, "rend_1412eb5b0", rend::cauldron_hot_v2::rend_1412eb5b0 as *const (), rend::cauldron_hot_v2::diff_rend_1412eb5b0),
// Replacement::verified_throws(0x1_4189_2bd0, "unknown_141892bd0", rend::cauldron_hot_v2::unknown_141892bd0 as *const (), Throws::Caught, rend::cauldron_hot_v2::diff_unknown_141892bd0),
