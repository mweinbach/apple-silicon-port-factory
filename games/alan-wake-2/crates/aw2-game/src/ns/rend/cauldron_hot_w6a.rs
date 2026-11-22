//! Decompiled hot functions shard W6A (`0x1421bbb90..0x1422e52c0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn rel_ptr(addr: usize) -> usize {
    let off = rd::<i32>(addr);
    if off == 0 {
        0
    } else {
        addr.wrapping_add(off as i64 as usize)
    }
}

#[inline(always)]
unsafe fn vrsqrtps_128(a: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64::{__m128, _mm_rsqrt_ps};
        let v: __m128 = core::mem::transmute(a);
        core::mem::transmute(_mm_rsqrt_ps(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vrsqrtps(a)
    }
}

#[inline(always)]
unsafe fn call_expf(x: f32) -> f32 {
    let (_, x0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad58,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [x.to_bits() as u128, 0, 0, 0],
    );
    f32::from_bits(x0 as u32)
}

#[inline(always)]
unsafe fn clear_bitset(bits_ptr: usize, bit_count: u64) {
    if bit_count != 0 {
        let words = (bit_count.wrapping_add(0x3f) >> 6) as usize;
        for i in 0..words.saturating_sub(1) {
            wr::<u64>(bits_ptr.wrapping_add(i * 8), 0);
        }
        let shift = (!(bit_count.wrapping_sub(1)) as u32) & 0x3f;
        let mask = !(u64::MAX >> shift);
        let last_slot = bits_ptr.wrapping_add((words - 1) * 8);
        wr::<u64>(last_slot, rd::<u64>(last_slot) & mask);
    }
}

/// `unknown_1421bbb90`
/// Original: 0x1421bbb90 (1232 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421bbb90(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u32,
) {
    let hdr_b8 = rd::<usize>(rd::<usize>(param_1) + 0xb8);
    let r13 = rel_ptr(hdr_b8 + 0x08);
    let entry = r13.wrapping_add((param_5 as usize) * 0x24);
    let count_4 = rd::<u16>(entry + 0x04);
    if count_4 == 0 {
        return;
    }

    let r8_idx_base = rd::<usize>(param_1 + 0x08);
    let xmm7_sign = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let mut r12 = rel_ptr(entry + 0x0c);
    let mut r14 = rel_ptr(entry + 0x10);
    let mut r15 = rel_ptr(entry + 0x08);
    let end_r15 = r15.wrapping_add((count_4 as usize) * 6);

    let mut local_vecs = [0u128; 256];
    if r15 != end_r15 {
        let start_idx_0 = rd::<u16>(entry) as usize;
        let mut rdi = r8_idx_base.wrapping_add(start_idx_0 * 2);
        let mut rsi = param_4;

        while r15 != end_r15 {
            let n_elems = rd::<u8>(r15 + 0x01) as usize;
            if rd::<u8>(r15) == 0 {
                for i in 0..n_elems {
                    let idx = rd::<u16>(rdi) as usize;
                    rdi = rdi.wrapping_add(2);
                    let xmm2 = rd::<u128>(param_2.wrapping_add(idx << 5));
                    let coeff = rd::<u128>(r12);
                    r12 = r12.wrapping_add(0x10);
                    let xmm0 = crate::simd_ops::vdpps(xmm2, coeff, 0xff);
                    let xmm1 = xmm0 & xmm7_sign;
                    local_vecs[i] = xmm1 ^ xmm2;
                }
            } else {
                let mask_7bc0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7bc0) as usize);
                for i in 0..n_elems {
                    let idx = rd::<u16>(rdi) as usize;
                    rdi = rdi.wrapping_add(2);
                    let v = rd::<u128>(param_2.wrapping_add((idx << 5) + 0x10));
                    local_vecs[i] = mask_7bc0 & v;
                }
            }

            let b2 = rd::<u8>(r15 + 0x02) as usize;
            let end_r14 = r14.wrapping_add((b2 * n_elems) << 4);
            let mut rcx_idx: usize = 0;
            let mut xmm2: u128 = 0;
            while r14 != end_r14 {
                let v1 = local_vecs[rcx_idx];
                rcx_idx += 1;
                let v_ref = rd::<u128>(r14);
                r14 = r14.wrapping_add(0x10);
                let diff = crate::simd_ops::vsubps(v1, v_ref);
                let dp = crate::simd_ops::vdpps(diff, diff, 0xff);
                xmm2 = crate::simd_ops::vaddps(dp, xmm2);
                if rcx_idx >= n_elems {
                    wr::<u32>(rsi, xmm2 as u32);
                    rsi = rsi.wrapping_add(4);
                    rcx_idx = 0;
                    xmm2 = 0;
                }
            }

            let b3 = rd::<u8>(r15 + 0x03) as usize;
            if b3 != 0 {
                let sz = b3 * 4;
                let _ = exe::call_va!(0x1_4183_acc2, (rsi, 0u32, (sz & !3usize) as u64) -> usize);
                rsi = rsi.wrapping_add(sz);
            }

            r15 = r15.wrapping_add(6);
        }
    }

    let count_6 = rd::<u16>(entry + 0x06);
    let rcx_tail = param_4.wrapping_add((count_6 as usize) * 4);
    let aligned_6 = (count_6.wrapping_add(7) & 0xfff8) as usize;
    let rdi_end = param_4.wrapping_add(aligned_6 * 4);
    let r8_dwords = if rcx_tail > rdi_end {
        0
    } else {
        rdi_end.wrapping_sub(rcx_tail).wrapping_add(3) >> 2
    };
    if r8_dwords != 0 {
        let _ = exe::call_va!(0x1_4183_acc2, (rcx_tail, 0u32, (r8_dwords << 2) as u64) -> usize);
    }

    let mut rbx_cur = param_4;
    loop {
        let lo = crate::simd_ops::vsqrtps(rd::<u128>(rbx_cur));
        let hi = crate::simd_ops::vsqrtps(rd::<u128>(rbx_cur + 0x10));
        wr::<u128>(rbx_cur, lo);
        wr::<u128>(rbx_cur + 0x10, hi);
        rbx_cur = rbx_cur.wrapping_add(0x20);
        if rbx_cur >= rdi_end {
            break;
        }
    }

    let hdr_b8_2 = rd::<usize>(rd::<usize>(param_1) + 0xb8);
    if rd::<u8>(hdr_b8_2 + 0x02) == 1 {
        let r10_idx_base = rd::<usize>(param_1 + 0x18);
        let mut r8_src = rel_ptr(entry + 0x18);
        let mut rdx_iter = rel_ptr(entry + 0x08);
        let cnt = rd::<u16>(entry + 0x04) as usize;
        let r9_end = rdx_iter.wrapping_add(cnt * 6);
        if rdx_iter != r9_end {
            let start_idx_2 = rd::<u16>(entry + 0x02) as usize;
            let mut rcx_idx_ptr = r10_idx_base.wrapping_add(start_idx_2 * 2);
            while rdx_iter != r9_end {
                let out_idx = rd::<u16>(rcx_idx_ptr) as usize;
                rcx_idx_ptr = rcx_idx_ptr.wrapping_add(2);
                let v = rd::<u128>(r8_src);
                r8_src = r8_src.wrapping_add(0x10);
                rdx_iter = rdx_iter.wrapping_add(6);
                wr::<u128>(param_3.wrapping_add(out_idx * 0x10), v);
            }
        }
    }

    let r8_out_idx_base = rd::<usize>(param_1 + 0x18);
    let mut rdx_weights = rel_ptr(entry + 0x1c);
    let mut r9_indices = rel_ptr(entry + 0x20);
    let mut r12_init = rel_ptr(entry + 0x14);
    let mut rsi_desc = rel_ptr(entry + 0x08);
    let cnt2 = rd::<u16>(entry + 0x04) as usize;
    let end_rsi = rsi_desc.wrapping_add(cnt2 * 6);
    if rsi_desc != end_rsi {
        let start_idx_2 = rd::<u16>(entry + 0x02) as usize;
        let xmm6_three = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);
        let xmm7_half = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
        let mut r15_idx_ptr = r8_out_idx_base.wrapping_add(start_idx_2 * 2);
        let mut r14_p4 = param_4;

        while rsi_desc != end_rsi {
            let out_idx = rd::<u16>(r15_idx_ptr) as usize;
            r15_idx_ptr = r15_idx_ptr.wrapping_add(2);
            let init_lo = rd::<u128>(r12_init);
            let init_hi = rd::<u128>(r12_init + 0x10);
            r12_init = r12_init.wrapping_add(0x20);
            let r10 = param_2.wrapping_add(out_idx << 5);
            let r11 = param_3.wrapping_add(out_idx << 4);
            wr::<u128>(r10, init_lo);
            wr::<u128>(r10 + 0x10, init_hi);

            let b5 = rd::<u8>(rsi_desc + 0x05) as usize;
            let b4 = rd::<u8>(rsi_desc + 0x04) as usize;
            let rdi_end_r9 = r9_indices.wrapping_add(b5);
            let rbx_stride = b4 << 4;

            loop {
                let r8_end_rdx = rdx_weights.wrapping_add(rbx_stride);
                let mut rcx_p4 = r14_p4;
                let mut xmm2: u128 = 0;
                loop {
                    let v0 = rd::<u128>(rcx_p4);
                    let w0 = rd::<u128>(rdx_weights);
                    rcx_p4 = rcx_p4.wrapping_add(0x10);
                    rdx_weights = rdx_weights.wrapping_add(0x10);
                    let dp = crate::simd_ops::vdpps(v0, w0, 0xff);
                    xmm2 = crate::simd_ops::vaddps(dp, xmm2);
                    if rdx_weights == r8_end_rdx {
                        break;
                    }
                }

                let comp_idx = rd::<u8>(r9_indices) as usize;
                r9_indices = r9_indices.wrapping_add(1);
                let acc_f32 = f32::from_bits(xmm2 as u32);
                if comp_idx < 8 {
                    let dst = r10.wrapping_add(comp_idx * 4);
                    wr::<f32>(dst, acc_f32 + rd::<f32>(dst));
                } else {
                    let dst = r11.wrapping_add(comp_idx * 4).wrapping_sub(0x20);
                    wr::<f32>(dst, acc_f32 + rd::<f32>(dst));
                }

                if r9_indices >= rdi_end_r9 {
                    break;
                }
            }

            let xmm4 = rd::<u128>(r10);
            let xmm0_dp = crate::simd_ops::vdpps(xmm4, xmm4, 0xff);
            let xmm2_rsq = vrsqrtps_128(xmm0_dp);
            let xmm0_m1 = crate::simd_ops::vmulps(xmm2_rsq, xmm0_dp);
            let xmm1_m2 = crate::simd_ops::vmulps(xmm0_m1, xmm2_rsq);
            let xmm3_sub = crate::simd_ops::vsubps(xmm6_three, xmm1_m2);
            let xmm2_h = crate::simd_ops::vmulps(xmm7_half, xmm2_rsq);
            let xmm0_scale = crate::simd_ops::vmulps(xmm3_sub, xmm2_h);
            r14_p4 = r14_p4.wrapping_add(rbx_stride);
            rsi_desc = rsi_desc.wrapping_add(6);
            let xmm1_norm = crate::simd_ops::vmulps(xmm0_scale, xmm4);
            wr::<u128>(r10, xmm1_norm);
        }
    }
}

/// `unknown_1421d5b40`
/// Original: 0x1421d5b40 (212 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421d5b40(param_1: usize) -> u64 {
    let rdi = rd::<usize>(param_1 + 0x160);
    let r8d = rd::<u32>(param_1 + 0x1b0);
    let rdx = (rd::<u16>(rdi + 0x130) as u64).wrapping_sub(1);
    let rcx_128 = rd::<u64>(rdi + 0x128);
    let found = exe::call_va!(0x1_4248_9e40, (rcx_128, rdx, r8d) -> u32);
    let bit_idx = found.wrapping_add(1);
    let word_off = ((bit_idx >> 6) as usize) * 8;
    let bit_shift = bit_idx & 0x3f;
    let bits_base = rd::<usize>(rd::<usize>(param_1 + 0x168) + 0x88);
    let word = rd::<u64>(bits_base.wrapping_add(word_off));
    let rdx_60 = rd::<u64>(rdi + 0x60);

    if ((word >> bit_shift) & 1) == 0 {
        let r8_ptr = exe::ptr::<u8>(0x1_4386_5860) as usize;
        exe::call_va!(0x1_421d_4c70, (param_1, rdx_60, r8_ptr));
        let bits_base2 = rd::<usize>(rd::<usize>(param_1 + 0x168) + 0x88);
        let slot = bits_base2.wrapping_add(word_off);
        wr::<u64>(slot, rd::<u64>(slot) | (1u64 << bit_shift));
    } else {
        let r8_ptr = exe::ptr::<u8>(0x1_4386_5ac0) as usize;
        exe::call_va!(0x1_421d_4c70, (param_1, rdx_60, r8_ptr));
    }

    let cur_20 = rd::<u64>(param_1 + 0x20);
    let step = (rd::<u32>(param_1 + 0x4c) as u64) << 5;
    let new_20 = cur_20.wrapping_sub(step);
    wr::<u32>(param_1 + 0x14, rd::<u32>(param_1 + 0x14).wrapping_sub(1));
    wr::<u64>(param_1 + 0x20, new_20);
    new_20
}

/// `unknown_1421d5c20`
/// Original: 0x1421d5c20 (829 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421d5c20(param_1: usize) {
    let p_168 = rd::<usize>(param_1 + 0x168);
    let r12 = rd::<usize>(p_168 + 0x10);
    let r13 = rd::<usize>(p_168 + 0x08);
    let r14 = rd::<usize>(p_168 + 0x20);
    let p_160 = rd::<usize>(param_1 + 0x160);
    let xmm10 = rd::<f32>(param_1 + 0x48);
    let r15 = rd::<u64>(p_160 + 0x50) as usize;
    let mut rdi = rd::<usize>(p_160 + 0x48);

    if r15 != 0 {
        let xmm7 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let xmm9 = rd::<f32>(exe::ptr::<f32>(0x1_4342_7080) as usize);
        let xmm8 = 0.0f32;

        for rbp in 0..r15 {
            let bit_id = rd::<u16>(rdi);
            let bit_shift = (bit_id & 0x3f) as u32;
            let word_idx = (bit_id >> 6) as usize;
            let bit_mask = 1u64 << bit_shift;

            if (rd::<u64>(r13.wrapping_add(word_idx * 8)) & bit_mask) != 0 {
                let rsi = rbp * 8;
                if (rd::<u64>(r12.wrapping_add(word_idx * 8)) & bit_mask) == 0 {
                    wr::<u64>(r14.wrapping_add(rsi), 0);
                }

                let rdx_08 = rd::<u64>(rdi + 0x08);
                let r8_990 = exe::ptr::<u8>(0x1_4386_5990) as usize;
                exe::call_va!(0x1_421d_4c70, (param_1, rdx_08, r8_990));

                let r9 = rd::<usize>(param_1 + 0x20);
                let rel_addr = r9.wrapping_sub(4);
                let next_20 = rel_ptr(rel_addr);
                wr::<u32>(param_1 + 0x14, rd::<u32>(param_1 + 0x14).wrapping_sub(1));
                wr::<usize>(param_1 + 0x20, next_20);

                let r10d = rd::<u32>(r9.wrapping_sub(8));
                if r10d == 0 {
                    let idx0 = rd::<u16>(rdi) as usize;
                    let map_20 = rd::<usize>(rd::<usize>(param_1 + 0x160) + 0x20);
                    let out_bit = rd::<u16>(map_20.wrapping_add(idx0 * 2));
                    let out_word = (out_bit >> 6) as usize;
                    let out_mask = 1u64 << (out_bit & 0x3f);
                    let dst_18 = rd::<usize>(rd::<usize>(param_1 + 0x168) + 0x18);
                    let slot = dst_18.wrapping_add(out_word * 8);
                    wr::<u64>(slot, (rd::<u64>(slot) & !out_mask) | out_mask);
                } else {
                    let packed = rd::<u64>(r14.wrapping_add(rsi));
                    let xmm2 = f32::from_bits((packed >> 32) as u32);
                    let r8_arr = rel_ptr(rel_addr);
                    let rem = ((packed as u32) % r10d) as usize;
                    let f0 = (xmm7 - xmm2) * rd::<f32>(r8_arr.wrapping_add(rem * 4));
                    let f1 = xmm2 * rd::<f32>(r8_arr.wrapping_add((rem + 1) * 4));
                    let mut xmm3 = f0 + f1;
                    if xmm3 > xmm7 {
                        xmm3 += xmm9;
                    }
                    let dur = rd::<f32>(r9.wrapping_sub(0x0c));
                    let xmm0_prod = xmm3 * dur;
                    let xmm1_inv = xmm7 / dur;
                    let xmm2_sum = xmm0_prod + xmm10;
                    let xmm6 = xmm2_sum * xmm1_inv;

                    let rounded =
                        f32::from_bits(crate::simd_ops::vroundss(0, xmm6.to_bits(), 1) as u32);
                    let eax_i32 = crate::simd_ops::cvt_f32_i32(rounded) as i32;
                    let xmm0_f = eax_i32 as f32;
                    let mut xmm1_frac = xmm6 - xmm0_f;
                    xmm1_frac = if xmm1_frac > xmm8 { xmm1_frac } else { xmm8 };
                    xmm1_frac = if xmm1_frac < xmm7 { xmm1_frac } else { xmm7 };

                    let (ret_rax, _) = crate::simd_ops::call_va8_xmm(
                        0x1_421c_06b0,
                        r9.wrapping_sub(0x0c) as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        [0, xmm1_frac.to_bits() as u128, 0, 0],
                    );
                    wr::<u64>(r14.wrapping_add(rsi), ret_rax);

                    let idx0 = rd::<u16>(rdi) as usize;
                    let map_20 = rd::<usize>(rd::<usize>(param_1 + 0x160) + 0x20);
                    let out_bit = rd::<u16>(map_20.wrapping_add(idx0 * 2));
                    let out_word = (out_bit >> 6) as usize;
                    let out_mask = 1u64 << (out_bit & 0x3f);
                    let dst_18 = rd::<usize>(rd::<usize>(param_1 + 0x168) + 0x18);
                    let slot = dst_18.wrapping_add(out_word * 8);
                    let bit_val = if xmm6 >= xmm7 { out_mask } else { 0 };
                    wr::<u64>(slot, (rd::<u64>(slot) & !out_mask) | bit_val);
                }

                let rdx_10 = rd::<u64>(rdi + 0x10);
                let r8_600 = exe::ptr::<u8>(0x1_4386_5600) as usize;
                exe::call_va!(0x1_421d_4c70, (param_1, rdx_10, r8_600));
            }

            rdi = rdi.wrapping_add(0x18);
        }
    }

    let rdx_58 = rd::<u64>(rd::<usize>(param_1 + 0x160) + 0x58);
    let r8_730 = exe::ptr::<u8>(0x1_4386_5730) as usize;
    exe::call_va!(0x1_421d_4c70, (param_1, rdx_58, r8_730));

    let rcx_88 = rd::<usize>(rd::<usize>(param_1 + 0x168) + 0x88);
    let edx_words =
        ((rd::<u16>(rd::<usize>(param_1 + 0x160) + 0x130) as u32).wrapping_add(0x3f)) >> 6;
    if edx_words != 0 {
        let _ = exe::call_va!(0x1_4183_acc2, (rcx_88, 0u32, (edx_words as u64) << 3) -> usize);
    }
}

/// `unknown_142207340`
/// Original: 0x142207340 (589 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142207340(param_1: usize) {
    let mut r12 = rd::<u64>(param_1 + 0x18).wrapping_sub(1);
    let mut rdi = rd::<usize>(param_1 + 0x280);
    let end0 = rdi.wrapping_add((rd::<u32>(param_1 + 0x288) as usize) << 4);
    if rdi == end0 {
        wr::<u64>(param_1 + 0x20, r12);
        return;
    }

    loop {
        let rbx = rd::<usize>(rdi);
        if rd::<u8>(rbx + 0xb9) != 0 {
            let last_idx = rd::<u8>(rbx + 0xbb).wrapping_sub(1) as usize;
            if rd::<u8>(rbx + 0x158 + last_idx) == 2 {
                wr::<u8>(rbx + 0xb9, 0);
                let g_148 = exe::ptr::<u32>(0x1_43a2_b148) as u64;
                let _ = crate::simd_ops::atomic_xadd_u32(g_148, u32::MAX);
            }
        }

        let mut bpl = rd::<u8>(rbx + 0xbc);
        let mut al: u8 = 0;
        let mut cl = rd::<u8>(rbx + 0xbb);
        if cl != 0 {
            loop {
                if rd::<u8>(rbx + 0x158 + (al as usize)) == 2 {
                    if al <= rd::<u8>(rbx + 0xbd) || al < bpl {
                        bpl = al;
                    }
                }
                al = al.wrapping_add(1);
                cl = rd::<u8>(rbx + 0xbb);
                if al >= cl {
                    break;
                }
            }
        }

        let mut r14b = bpl == rd::<u8>(rbx + 0xbd);
        let mut sil: u8 = 0;
        if cl != 1 {
            loop {
                if sil != bpl && sil != rd::<u8>(rbx + 0xbd) {
                    let state = rd::<u8>(rbx + 0x158 + (sil as usize));
                    if state == 1 {
                        r14b = false;
                    } else if state == 2 {
                        exe::call_va!(0x1_4207_e500, (rbx + 0x108, sil as u32));
                    }
                }
                sil = sil.wrapping_add(1);
                let limit = rd::<u8>(rbx + 0xbb).wrapping_sub(1);
                if sil >= limit {
                    break;
                }
            }
        }

        if bpl != rd::<u8>(rbx + 0xbc) {
            wr::<u8>(rbx + 0xbc, bpl);
            let val = rd::<u64>(rbx + 0x118 + (bpl as usize) * 8);
            wr::<u64>(rbx + 0x1a0, val);
            exe::call_va!(0x1_4220_8970, (rbx));
        }

        if r14b {
            let last_elem = rd::<usize>(param_1 + 0x280)
                .wrapping_add(((rd::<u32>(param_1 + 0x288) as usize) << 4).wrapping_sub(0x10));
            if rdi != last_elem {
                let rdx_obj = rd::<usize>(rdi);
                if rdx_obj != 0 {
                    let prev = crate::simd_ops::atomic_xadd_u32((rdx_obj + 0x24) as u64, u32::MAX);
                    if prev == 1 {
                        let rcx_28 = rd::<usize>(rdx_obj + 0x28);
                        if rcx_28 != 0 {
                            exe::call_va!(0x1_4241_d6f0, (rcx_28, rdx_obj));
                        }
                    }
                }
                wr::<usize>(rdi, 0);
                let moved_ptr = rd::<usize>(last_elem);
                wr::<usize>(last_elem, 0);
                wr::<usize>(rdi, moved_ptr);
                wr::<u64>(rdi + 0x08, rd::<u64>(last_elem + 0x08));
            }

            let end_after = rd::<usize>(param_1 + 0x280)
                .wrapping_add((rd::<u32>(param_1 + 0x288) as usize) << 4);
            let tail_slot = end_after.wrapping_sub(0x10);
            let rdx_tail = rd::<usize>(tail_slot);
            if rdx_tail != 0 {
                let prev = crate::simd_ops::atomic_xadd_u32((rdx_tail + 0x24) as u64, u32::MAX);
                if prev == 1 {
                    let rcx_28 = rd::<usize>(rdx_tail + 0x28);
                    if rcx_28 != 0 {
                        exe::call_va!(0x1_4241_d6f0, (rcx_28, rdx_tail));
                    }
                }
            }
            wr::<usize>(tail_slot, 0);
            wr::<u32>(param_1 + 0x288, rd::<u32>(param_1 + 0x288).wrapping_sub(1));
        } else {
            let v = rd::<u64>(rdi + 0x08).wrapping_sub(1);
            if v < r12 {
                r12 = v;
            }
            rdi = rdi.wrapping_add(0x10);
        }

        let cur_end =
            rd::<usize>(param_1 + 0x280).wrapping_add((rd::<u32>(param_1 + 0x288) as usize) << 4);
        if rdi == cur_end {
            break;
        }
    }

    wr::<u64>(param_1 + 0x20, r12);
}

/// `unknown_142210fa0`
/// Original: 0x142210fa0 (824 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142210fa0(param_1: usize, _param_2: u64, param_3: usize) {
    let mut rbp = param_1.wrapping_add(0x08);
    for rbx in 0..0x100usize {
        let c_sign = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
        let c_255 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eec) as usize);
        let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let c_four = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d10) as usize);
        let c_741 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6db0) as usize);
        let c_199 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ab8) as usize);
        let c_567 = rd::<f32>(exe::ptr::<f32>(0x1_4342_67b8) as usize);
        let c_187 = rd::<f32>(exe::ptr::<f32>(0x1_4342_663c) as usize);
        let c_0484 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6560) as usize);
        let c_0064 = rd::<f32>(exe::ptr::<f32>(0x1_4342_64c8) as usize);

        let xmm0_i = (rbx as i64) as f32;
        let t = xmm0_i / c_255;
        let a0 = (c_one - t) * 0.0f32;
        let a1 = t * c_four;
        let xmm2 = a0 + a1;
        let neg_xmm2 = f32::from_bits(xmm2.to_bits() ^ c_sign);
        let xmm7 = neg_xmm2 * xmm2;

        let e0 = call_expf(xmm7 / c_741);
        let p3_04 = rd::<f32>(param_3 + 0x04);
        let p3_08 = rd::<f32>(param_3 + 0x08);

        let e1 = call_expf(xmm7 / c_199);
        let p3_14 = rd::<f32>(param_3 + 0x14);
        let p3_18 = rd::<f32>(param_3 + 0x18);

        let e2 = call_expf(xmm7 / c_567);
        let p3_24 = rd::<f32>(param_3 + 0x24);
        let p3_28 = rd::<f32>(param_3 + 0x28);

        let e3 = call_expf(xmm7 / c_187);
        let p3_34 = rd::<f32>(param_3 + 0x34);
        let p3_38 = rd::<f32>(param_3 + 0x38);

        let e4 = call_expf(xmm7 / c_0484);
        let p3_44 = rd::<f32>(param_3 + 0x44);
        let p3_48 = rd::<f32>(param_3 + 0x48);

        let e5 = call_expf(xmm7 / c_0064);
        let p3_50 = rd::<f32>(param_3 + 0x50);
        let p3_40 = rd::<f32>(param_3 + 0x40);
        let p3_54 = rd::<f32>(param_3 + 0x54);
        let p3_58 = rd::<f32>(param_3 + 0x58);
        let p3_30 = rd::<f32>(param_3 + 0x30);
        let p3_20 = rd::<f32>(param_3 + 0x20);
        let p3_10 = rd::<f32>(param_3 + 0x10);
        let p3_00 = rd::<f32>(param_3);

        let r0_a = (e5 * p3_50) + (e4 * p3_40);
        let r0_b = r0_a + (e3 * p3_30);
        let r0_c = r0_b + (e2 * p3_20);
        let r0_d = r0_c + (e1 * p3_10);
        let r0 = r0_d + (e0 * p3_00);
        wr::<f32>(rbp - 0x08, r0);

        let r1_a = (e4 * p3_44) + (e5 * p3_54);
        let r1_b = r1_a + (e3 * p3_34);
        let r1_c = r1_b + (e2 * p3_24);
        let r1_d = r1_c + (e1 * p3_14);
        let r1 = r1_d + (e0 * p3_04);
        wr::<f32>(rbp - 0x04, r1);

        let r2_a = (e4 * p3_48) + (e5 * p3_58);
        let r2_b = r2_a + (e3 * p3_38);
        let r2_c = r2_b + (e2 * p3_28);
        let r2_d = r2_c + (e1 * p3_18);
        let r2 = r2_d + (e0 * p3_08);
        wr::<f32>(rbp, r2);

        rbp = rbp.wrapping_add(0x0c);
    }

    wr::<u64>(param_1 + 0xbf4, 0);
    wr::<u32>(param_1 + 0xbfc, 0);
}

/// `unknown_1422e4a90`
/// Original: 0x1422e4a90 (1741 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1422e4a90(param_1: usize) {
    wr::<u32>(param_1 + 0x98, 0);
    let rdi_90 = param_1 + 0x90;
    let len_28 = rd::<u32>(param_1 + 0x28) as u64;
    let cap_9c = rd::<u32>(rdi_90 + 0x0c) as u64;
    if len_28 > cap_9c {
        exe::call_va!(0x1_401e_6d50, (rdi_90, len_28));
    }

    let g_fed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let mut rbp_word_ptr = rd::<usize>(param_1 + 0x60);
    let num_words_60 = (rd::<u64>(param_1 + 0x70).wrapping_add(0x3f) >> 6) as usize;

    for word_idx in 0..num_words_60 {
        let mut word = rd::<u64>(rbp_word_ptr);
        if word != 0 {
            let base_bit = (word_idx as u32).wrapping_shl(6);
            loop {
                let tz = word.trailing_zeros();
                let bit_idx = base_bit.wrapping_add(tz);
                let w_off = ((bit_idx >> 6) as usize) * 8;
                let b_mask = !(1u64 << (bit_idx & 0x3f));

                let p60 = rd::<usize>(param_1 + 0x60).wrapping_add(w_off);
                wr::<u64>(p60, rd::<u64>(p60) & b_mask);
                let p78 = rd::<usize>(param_1 + 0x78).wrapping_add(w_off);
                wr::<u64>(p78, rd::<u64>(p78) & b_mask);

                if (bit_idx as u64) < (rd::<u32>(param_1 + 0x28) as u64) {
                    let mut dl_valid = false;
                    let r14_elem =
                        rd::<usize>(param_1 + 0x20).wrapping_add((bit_idx as usize) * 0x30);
                    let idx_4 = rd::<u32>(r14_elem + 0x04);
                    if idx_4 != u32::MAX {
                        let r8_item = rd::<usize>(g_fed0 + 0x120).wrapping_add((idx_4 as usize) * 0xa0);
                        let r11d = rd::<u32>(r14_elem);
                        let rdx_90 = rd::<usize>(r8_item + 0x90);
                        let mut found_ecx = u32::MAX;
                        let b_272 = rd::<u8>(rdx_90 + 0x272) as u32;
                        if b_272 != 0 {
                            let max_idx = b_272.wrapping_sub(1);
                            let clamped = if (max_idx as i32) < (r11d as i32) {
                                max_idx as i32
                            } else {
                                r11d as i32
                            };
                            let off_1c8 = (rdx_90 as isize + (clamped as isize) * 4 + 0x1c8) as usize;
                            if rd::<u32>(off_1c8) != 0 && r11d < (rd::<u8>(rdx_90 + 0x270) as u32) {
                                let g_ff48 =
                                    rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
                                let mut cur_u32 = rd::<usize>(r8_item + 0x80);
                                let end_u32 = cur_u32
                                    .wrapping_add((rd::<u32>(r8_item + 0x88) as usize) * 4);
                                while cur_u32 != end_u32 {
                                    let ecx_val = rd::<u32>(cur_u32);
                                    if ecx_val != u32::MAX {
                                        let arr_1a8 = rd::<usize>(g_ff48 + 0x1a8);
                                        if (rd::<u8>(arr_1a8.wrapping_add(ecx_val as usize)) as u32)
                                            == r11d
                                        {
                                            let arr_18 = rd::<usize>(g_ff48 + 0x18);
                                            let obj_ptr =
                                                rd::<usize>(arr_18.wrapping_add((ecx_val as usize) * 8));
                                            if (rd::<u8>(obj_ptr + 0x4c) & 1) == 0 {
                                                found_ecx = ecx_val;
                                                break;
                                            }
                                        }
                                    }
                                    cur_u32 = cur_u32.wrapping_add(4);
                                }
                            }
                        }
                        dl_valid = found_ecx != u32::MAX;
                    }

                    let rcx_sub = rd::<usize>(r14_elem + 0x08);
                    let state_60 = rd::<i32>(rcx_sub + 0x60);
                    if dl_valid {
                        if state_60 == -1 {
                            let lo = rd::<u32>(rcx_sub + 0x54) as u64;
                            let hi = (rd::<u16>(rcx_sub + 0x28) as u32) as u64;
                            let packed = lo | (hi << 32);
                            let cur_len = rd::<u32>(rdi_90 + 0x08) as u64;
                            let cur_cap = rd::<u32>(rdi_90 + 0x0c) as u64;
                            if (cur_len << 3) == (cur_cap << 3) {
                                exe::call_va!(0x1_401e_6d50, (rdi_90, cur_len + 1));
                            }
                            let idx_ins = rd::<u32>(rdi_90 + 0x08) as usize;
                            let arr_90 = rd::<usize>(rdi_90);
                            wr::<u64>(arr_90.wrapping_add(idx_ins * 8), packed);
                            wr::<u32>(rdi_90 + 0x08, (idx_ins as u32).wrapping_add(1));
                            wr::<u32>(param_1 + 0x5c, rd::<u32>(param_1 + 0x5c).wrapping_add(1));
                        }
                    } else if state_60 != -1 {
                        exe::call_va!(0x1_422e_06c0, (param_1, r14_elem));
                        wr::<u32>(param_1 + 0x5c, rd::<u32>(param_1 + 0x5c).wrapping_sub(1));
                    }
                }

                word = rd::<u64>(rbp_word_ptr);
                if word == 0 {
                    break;
                }
            }
        }
        rbp_word_ptr = rbp_word_ptr.wrapping_add(8);
    }

    clear_bitset(rd::<usize>(param_1 + 0x60), rd::<u64>(param_1 + 0x70));

    let raw_b38 = rd::<u32>(exe::ptr::<u32>(0x1_4386_bb38) as usize);
    let thresh_r14 = ((raw_b38.wrapping_add(0x3f) & !0x3fu32) << 0x14) as u64;
    let g_fed0_2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let mut rbx_cur = rd::<usize>(rdi_90);
    let rdi_end = rbx_cur.wrapping_add((rd::<u32>(rdi_90 + 0x08) as usize) * 8);
    while rbx_cur != rdi_end {
        let lo_idx = rd::<u32>(rbx_cur) as usize;
        let hi_idx = rd::<u32>(rbx_cur + 0x04) as usize;
        let item_base = rd::<usize>(g_fed0_2 + 0x120).wrapping_add(lo_idx * 0xa0);
        let ptr_40 = rd::<usize>(item_base.wrapping_add(hi_idx * 8 + 0x40));
        let idx_58 = rd::<u32>(ptr_40 + 0x58) as usize;
        let rdx_arg = rd::<usize>(param_1 + 0x20).wrapping_add(idx_58 * 0x30);
        exe::call_va!(0x1_422e_0890, (param_1, rdx_arg));
        rbx_cur = rbx_cur.wrapping_add(8);
    }

    wr::<u32>(param_1 + 0xa8, 0);
    wr::<u32>(param_1 + 0xb8, 0);

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_bb88) as usize) != 0 {
        let r9_ff48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
        let mut r14_word_ptr = rd::<usize>(param_1 + 0x78);
        let r13_words = (rd::<u64>(param_1 + 0x88).wrapping_add(0x3f) >> 6) as usize;
        for r12_idx in 0..r13_words {
            let mut rcx_word = rd::<u64>(r14_word_ptr);
            if rcx_word != 0 {
                let r15d = (r12_idx as u32).wrapping_shl(6);
                loop {
                    let ebp_tz = rcx_word.trailing_zeros();
                    let elem_idx = r15d.wrapping_add(ebp_tz) as usize;
                    let rdi_elem = rd::<usize>(param_1 + 0x20).wrapping_add(elem_idx * 0x30);
                    let sub_ptr = rd::<usize>(rdi_elem + 0x08);
                    if rd::<i32>(sub_ptr + 0x60) != -1 {
                        let g_fed0_3 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
                        let ecx_4 = rd::<u32>(rdi_elem + 0x04) as usize;
                        let arr_100 = rd::<usize>(g_fed0_3 + 0x100);
                        let r8d = rd::<u32>(arr_100.wrapping_add(ecx_4 * 4));
                        let bits_668 = rd::<usize>(r9_ff48 + 0x668);
                        let w_668 = rd::<u64>(bits_668.wrapping_add(((r8d >> 6) as usize) * 8));
                        let vec_hdr = if ((w_668 >> (r8d & 0x3f)) & 1) != 0 {
                            param_1 + 0xa0
                        } else {
                            param_1 + 0xb0
                        };
                        let v_len = rd::<u32>(vec_hdr + 0x08) as u64;
                        let v_cap = rd::<u32>(vec_hdr + 0x0c) as u64;
                        if (v_len << 3) == (v_cap << 3) {
                            exe::call_va!(0x1_4020_bcb0, (vec_hdr, v_len + 1));
                        }
                        let ins_idx = rd::<u32>(vec_hdr + 0x08) as usize;
                        let v_ptr = rd::<usize>(vec_hdr);
                        wr::<usize>(v_ptr.wrapping_add(ins_idx * 8), rd::<usize>(rdi_elem + 0x08));
                        wr::<u32>(vec_hdr + 0x08, (ins_idx as u32).wrapping_add(1));
                    }
                    rcx_word = rd::<u64>(r14_word_ptr) ^ (1u64 << (ebp_tz & 0x3f));
                    wr::<u64>(r14_word_ptr, rcx_word);
                    if rcx_word == 0 {
                        break;
                    }
                }
            }
            r14_word_ptr = r14_word_ptr.wrapping_add(8);
        }

        let mut cur_b0 = rd::<usize>(param_1 + 0xb0);
        let end_b0 = cur_b0.wrapping_add((rd::<u32>(param_1 + 0xb8) as usize) * 8);
        while cur_b0 != end_b0 {
            let item = rd::<usize>(cur_b0);
            exe::call_va!(0x1_422e_0440, (param_1, item));
            exe::call_va!(0x1_422e_0260, (param_1, item));
            cur_b0 = cur_b0.wrapping_add(8);
        }

        let mut cur_a0 = rd::<usize>(param_1 + 0xa0);
        let end_a0 = cur_a0.wrapping_add((rd::<u32>(param_1 + 0xa8) as usize) * 8);
        while cur_a0 != end_a0 {
            let item = rd::<usize>(cur_a0);
            exe::call_va!(0x1_422e_0550, (param_1, item));
            exe::call_va!(0x1_422e_0340, (param_1, item));
            cur_a0 = cur_a0.wrapping_add(8);
        }
    } else {
        if rd::<u8>(param_1 + 0xc0) != 0 && rd::<u32>(param_1 + 0x28) != 0 {
            let mut rbp_idx: usize = 0;
            let mut rdi_off: usize = 0;
            let mut rbx_base = rd::<usize>(param_1 + 0x20);
            loop {
                let item = rd::<usize>(rbx_base.wrapping_add(rdi_off + 0x08));
                if item != 0 && rd::<i32>(item + 0x60) != -1 {
                    exe::call_va!(0x1_422e_0550, (param_1, item));
                    exe::call_va!(0x1_422e_0340, (param_1, item));
                }
                rbp_idx += 1;
                rdi_off = rdi_off.wrapping_add(0x30);
                let cnt = rd::<u32>(param_1 + 0x28) as usize;
                rbx_base = rd::<usize>(param_1 + 0x20);
                if rbp_idx >= cnt {
                    break;
                }
            }
        }
        clear_bitset(rd::<usize>(param_1 + 0x78), rd::<u64>(param_1 + 0x88));
    }

    wr::<u8>(
        param_1 + 0xc0,
        rd::<u8>(exe::ptr::<u8>(0x1_4386_bb88) as usize),
    );

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_bb58) as usize) != 0 {
        let mut rbx_30 = rd::<usize>(param_1 + 0x30);
        let rdi_end_30 = rbx_30.wrapping_add((rd::<u32>(param_1 + 0x38) as usize) * 0x38);
        while rbx_30 != rdi_end_30 {
            if rd::<usize>(rbx_30 + 0x08) != 0 {
                exe::call_va!(0x1_4222_9ab0, (rbx_30 + 0x10));
                let mut edx_sum: u32 = 0;
                let mut rax_cur = rd::<usize>(rbx_30 + 0x18);
                let r8_end = rax_cur.wrapping_add((rd::<u32>(rbx_30 + 0x20) as usize) * 8);
                while rax_cur != r8_end {
                    edx_sum = edx_sum.wrapping_add(rd::<u32>(rax_cur + 0x04));
                    rax_cur = rax_cur.wrapping_add(8);
                }
                if edx_sum == rd::<u32>(rbx_30 + 0x10) {
                    let rbp_obj = rd::<usize>(rbx_30 + 0x08);
                    wr::<usize>(rbx_30 + 0x08, 0);
                    if rbp_obj != 0 {
                        exe::call_va!(0x1_4209_d670, (rbp_obj));
                        exe::call_va!(0x1_411e_7fc0, (rbp_obj + 0x58));
                        exe::call_va!(0x1_4244_2de0, (rbp_obj));
                    }
                    let handle = rd::<u32>(rbx_30);
                    if handle != u32::MAX {
                        exe::call_va!(0x1_4233_01f0, (handle));
                        exe::call_va!(0x1_4233_0010, (rd::<u32>(rbx_30)));
                        wr::<u32>(rbx_30, u32::MAX);
                    }
                }
            }
            rbx_30 = rbx_30.wrapping_add(0x38);
        }
    }

    let mut rbx_48 = rd::<usize>(param_1 + 0x48);
    let rdi_end_48 = rbx_48.wrapping_add((rd::<u32>(param_1 + 0x50) as usize) * 0x120);
    while rbx_48 != rdi_end_48 {
        exe::call_va!(0x1_422e_0ed0, (rbx_48));
        rbx_48 = rbx_48.wrapping_add(0x120);
    }

    if rd::<u64>(param_1 + 0x40) < thresh_r14 {
        wr::<u8>(param_1 + 0x58, 0);
    } else if rd::<u8>(param_1 + 0x58) == 0 {
        wr::<u8>(param_1 + 0x58, 1);
    }
}

/// `unknown_1422e52c0`
/// Original: 0x1422e52c0 (1514 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1422e52c0(
    param_1: usize,
    param_2: u32,
    param_3: usize,
    param_4: u32,
    param_5: u64,
    param_6: u64,
    param_7: u64,
) {
    wr::<u64>(param_1 + 0x18, param_6);
    wr::<u64>(param_1 + 0x20, param_7);
    wr::<u32>(param_1 + 0x54, param_2);
    let r10 = rd::<usize>(param_3);
    wr::<u16>(param_1 + 0x28, rd::<u16>(r10 + 0xa8));
    wr::<u64>(param_1 + 0x08, param_5);
    wr::<u16>(param_1 + 0x2a, param_4 as u16);
    wr::<u32>(param_1 + 0x40, rd::<u32>(r10 + 0xe8));
    wr::<u32>(param_1 + 0x50, rd::<u32>(r10 + 0xec));
    wr::<u64>(param_1 + 0x10, rd::<u64>(r10 + 0x80));
    wr::<u32>(param_1 + 0x4c, rd::<u32>(r10 + 0xc0));
    wr::<u32>(param_1 + 0x44, rd::<u32>(r10 + 0x48));

    match rd::<u32>(r10 + 0xd0) {
        1 => wr::<u16>(param_1 + 0x2e, 0),
        4 => wr::<u16>(param_1 + 0x2e, 1),
        8 => wr::<u16>(param_1 + 0x2e, 2),
        _ => {}
    }

    wr::<u32>(param_1 + 0x34, rd::<u32>(r10 + 0xac));
    wr::<u16>(param_1 + 0x2c, rd::<u16>(r10 + 0x50));

    let count_40 = rd::<u8>(r10 + 0x40) as usize;
    if count_40 != 0 {
        let mut rcx_cur = r10 + 0x01;
        let lut_r11 = exe::ptr::<u8>(0x1_4297_89d0) as usize;
        for _ in 0..count_40 {
            let b1 = rd::<u8>(rcx_cur + 0x01);
            if (b1.wrapping_sub(2) & 0xfd) == 0 {
                let idx = rd::<u8>(rcx_cur) as usize;
                let delta = rd::<u8>(lut_r11.wrapping_add(idx)) as u16;
                wr::<u16>(
                    param_1 + 0x2c,
                    rd::<u16>(param_1 + 0x2c).wrapping_sub(delta),
                );
            }
            rcx_cur = rcx_cur.wrapping_add(4);
        }
    }

    wr::<u32>(param_1 + 0x5c, rd::<u32>(r10 + 0xd8));

    if param_4 != 0 {
        let mut rsi_off: usize = 0;
        let mut r13_cur = param_3;
        for _ in 0..(param_4 as usize) {
            let rbp = rd::<usize>(r13_cur);
            let rdi_base = rd::<usize>(param_1 + 0x08);
            let dst_elem = rdi_base.wrapping_add(rsi_off);
            wr::<u32>(
                param_1 + 0x48,
                rd::<u32>(param_1 + 0x48).wrapping_add(rd::<u32>(rbp + 0xb0)),
            );
            wr::<u32>(dst_elem, rd::<u32>(r13_cur + 0x08));

            let id_4 = exe::call_va!(0x1_422d_f920, () -> u32);
            let r14_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
            wr::<u32>(dst_elem + 0x04, id_4);
            let v_11c = rd::<u32>(rbp + 0x11c);
            wr::<u32>(dst_elem + 0x0c, v_11c);
            wr::<u32>(dst_elem + 0x10, rd::<u32>(rbp + 0x120));
            wr::<u32>(dst_elem + 0x14, rd::<u32>(rbp + 0x124));
            wr::<u32>(dst_elem + 0x08, rd::<u32>(rbp + 0xd8));
            wr::<u32>(param_1 + 0x38, rd::<u32>(param_1 + 0x38).wrapping_add(v_11c));

            let src_d8 =
                rd::<usize>(r14_mgr + 0x98).wrapping_add((rd::<u32>(rbp + 0xd8) as usize) * 0x28);
            let chunk_32 = rd::<[u64; 4]>(src_d8);
            let chunk_8 = rd::<u64>(src_d8 + 0x20);
            wr::<[u64; 4]>(dst_elem + 0x18, chunk_32);
            wr::<u64>(dst_elem + 0x38, chunk_8);
            wr::<[u64; 4]>(dst_elem + 0x40, chunk_32);
            wr::<u64>(dst_elem + 0x60, chunk_8);
            wr::<u8>(dst_elem + 0x2c, 8);
            wr::<u8>(dst_elem + 0x34, 0x10);
            wr::<u8>(dst_elem + 0x35, rd::<u8>(param_1 + 0x2c));
            wr::<u8>(dst_elem + 0x36, 0);

            let r15d = rd::<u32>(dst_elem + 0x04) as u64;
            let vtbl0 = rd::<usize>(r14_mgr);
            let cap0 = exe::call_va!(rd::<usize>(vtbl0 + 0x38), (r14_mgr) -> u64);
            if cap0 <= r15d {
                let vtbl0_b = rd::<usize>(r14_mgr);
                let new_cap = r15d.wrapping_add(0x400) & !0x3ffu64;
                exe::call_va!(rd::<usize>(vtbl0_b + 0x28), (r14_mgr, new_cap));
            }
            let dst_98 = rd::<usize>(r14_mgr + 0x98).wrapping_add((r15d as usize) * 0x28);
            wr::<[u64; 4]>(dst_98, rd::<[u64; 4]>(dst_elem + 0x18));
            wr::<u64>(dst_98 + 0x20, rd::<u64>(dst_elem + 0x38));
            let bits_a8 = rd::<usize>(r14_mgr + 0xa8).wrapping_add(((r15d as usize) >> 6) * 8);
            wr::<u64>(bits_a8, rd::<u64>(bits_a8) | (1u64 << (r15d & 0x3f)));
            let r15_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
            wr::<u32>(r14_mgr + 0x410, rd::<u32>(r14_mgr + 0x410).wrapping_add(1));

            let v_b4 = rd::<u32>(rbp + 0xb4);
            let v_e4 = rd::<u32>(rbp + 0xe4);
            let r14d = rd::<u32>(dst_elem + 0x04) as u64;
            let prod_c8 = v_b4.wrapping_mul(rd::<u32>(rbp + 0xc8));
            let sub1 = r15_mgr + 0xd0;
            let vtbl1 = rd::<usize>(sub1);
            let cap1 = exe::call_va!(rd::<usize>(vtbl1 + 0x38), (sub1) -> u64);
            if cap1 <= r14d {
                let vtbl1_b = rd::<usize>(sub1);
                let new_cap = r14d.wrapping_add(0x400) & !0x3ffu64;
                exe::call_va!(rd::<usize>(vtbl1_b + 0x28), (sub1, new_cap));
            }
            let dst_168 = rd::<usize>(r15_mgr + 0x168).wrapping_add((r14d as usize) * 0x10);
            wr::<[u32; 4]>(dst_168, [v_e4, v_b4, prod_c8, 0]);
            let bits_178 = rd::<usize>(r15_mgr + 0x178).wrapping_add(((r14d as usize) >> 6) * 8);
            wr::<u64>(bits_178, rd::<u64>(bits_178) | (1u64 << (r14d & 0x3f)));
            wr::<u32>(r15_mgr + 0x410, rd::<u32>(r15_mgr + 0x410).wrapping_add(1));

            let u_28 = rd::<u16>(param_1 + 0x28) as u32;
            let r15_mgr2 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
            let f_140 = rd::<u32>(rbp + 0x140);
            let f_144 = rd::<u32>(rbp + 0x144);
            let f_148 = rd::<u32>(rbp + 0x148);
            let f_14c = rd::<u32>(rbp + 0x14c);
            let r14d_2 = rd::<u32>(dst_elem + 0x04) as u64;
            let v_ac = rd::<u32>(rbp + 0xac);
            let v_b0 = rd::<u32>(rbp + 0xb0);
            let v_11c_2 = rd::<u32>(rbp + 0x11c);
            let sub2 = r15_mgr2 + 0x340;
            let vtbl2 = rd::<usize>(sub2);
            let cap2 = exe::call_va!(rd::<usize>(vtbl2 + 0x38), (sub2) -> u64);
            if cap2 <= r14d_2 {
                let vtbl2_b = rd::<usize>(sub2);
                let new_cap = r14d_2.wrapping_add(0x400) & !0x3ffu64;
                exe::call_va!(rd::<usize>(vtbl2_b + 0x28), (sub2, new_cap));
            }
            let dst_3d8 = rd::<usize>(r15_mgr2 + 0x3d8).wrapping_add((r14d_2 as usize) * 0x20);
            wr::<[u32; 8]>(
                dst_3d8,
                [u_28, v_ac, v_b0, v_11c_2, f_140, f_144, f_148, f_14c],
            );
            let bits_3e8 =
                rd::<usize>(r15_mgr2 + 0x3e8).wrapping_add(((r14d_2 as usize) >> 6) * 8);
            wr::<u64>(bits_3e8, rd::<u64>(bits_3e8) | (1u64 << (r14d_2 & 0x3f)));
            wr::<u32>(
                r15_mgr2 + 0x410,
                rd::<u32>(r15_mgr2 + 0x410).wrapping_add(1),
            );

            let v_e8 = rd::<u32>(rbp + 0xe8);
            let r15_mgr3 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
            let r14d_3 = rd::<u32>(dst_elem + 0x04) as u64;
            let v_ec = rd::<u32>(rbp + 0xec);
            let v_120 = rd::<u32>(rbp + 0x120);
            let v_124_x20 = rd::<u32>(rbp + 0x124).wrapping_mul(0x14);
            let sub3 = r15_mgr3 + 0x1a0;
            let vtbl3 = rd::<usize>(sub3);
            let cap3 = exe::call_va!(rd::<usize>(vtbl3 + 0x38), (sub3) -> u64);
            if cap3 <= r14d_3 {
                let vtbl3_b = rd::<usize>(sub3);
                let new_cap = r14d_3.wrapping_add(0x400) & !0x3ffu64;
                exe::call_va!(rd::<usize>(vtbl3_b + 0x28), (sub3, new_cap));
            }
            let dst_238 = rd::<usize>(r15_mgr3 + 0x238).wrapping_add((r14d_3 as usize) * 0x10);
            wr::<[u32; 4]>(dst_238, [v_e8, v_ec, v_120, v_124_x20]);
            let bits_248 =
                rd::<usize>(r15_mgr3 + 0x248).wrapping_add(((r14d_3 as usize) >> 6) * 8);
            wr::<u64>(bits_248, rd::<u64>(bits_248) | (1u64 << (r14d_3 & 0x3f)));
            wr::<u32>(
                r15_mgr3 + 0x410,
                rd::<u32>(r15_mgr3 + 0x410).wrapping_add(1),
            );

            let v_f0 = rd::<u32>(rbp + 0xf0);
            let edi_id = rd::<u32>(dst_elem + 0x04) as u64;
            let v_130 = rd::<u32>(rbp + 0x130);
            let rbp_mgr4 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
            let sub4 = rbp_mgr4 + 0x270;
            let vtbl4 = rd::<usize>(sub4);
            let cap4 = exe::call_va!(rd::<usize>(vtbl4 + 0x38), (sub4) -> u64);
            if cap4 <= edi_id {
                let vtbl4_b = rd::<usize>(sub4);
                let new_cap = edi_id.wrapping_add(0x400) & !0x3ffu64;
                exe::call_va!(rd::<usize>(vtbl4_b + 0x28), (sub4, new_cap));
            }
            let dst_308 = rd::<usize>(rbp_mgr4 + 0x308).wrapping_add((edi_id as usize) * 8);
            wr::<u64>(dst_308, (v_f0 as u64) | ((v_130 as u64) << 32));
            let bits_318 =
                rd::<usize>(rbp_mgr4 + 0x318).wrapping_add(((edi_id as usize) >> 6) * 8);
            wr::<u64>(bits_318, rd::<u64>(bits_318) | (1u64 << (edi_id & 0x3f)));
            wr::<u32>(
                rbp_mgr4 + 0x410,
                rd::<u32>(rbp_mgr4 + 0x410).wrapping_add(1),
            );

            rsi_off = rsi_off.wrapping_add(0x68);
            r13_cur = r13_cur.wrapping_add(0x10);
        }
    }

    let g_ff70 = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff70) as usize);
    exe::call_va!(0x1_422e_1780, (g_ff70, param_1));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1421bbb90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, mode_b0: u8, copy_flag: u8| -> ([u32; 16], [u32; 8], [u32; 8]) {
        let mut blob = [0u8; 0x300];
        let bp = blob.as_mut_ptr() as usize;

        // hdr_b8 at bp + 0x00:
        // [bp + 0x02] = copy_flag
        // [bp + 0x08] = rel offset to entries at bp + 0x20 => +0x18
        wr::<u8>(bp + 0x02, copy_flag);
        wr::<i32>(bp + 0x08, 0x18);

        // entry 0 at bp + 0x20 (size 0x24):
        let entry = bp + 0x20;
        wr::<u16>(entry, 0); // start_idx_0
        wr::<u16>(entry + 0x02, 0); // start_idx_2
        wr::<u16>(entry + 0x04, 1); // count_4 = 1 (6-byte descriptor)
        wr::<u16>(entry + 0x06, 2); // count_6 = 2 floats (aligned to 8)

        // Rel offsets from entry fields:
        // +0x08 -> desc at bp + 0x60
        // +0x0c -> r12 coeffs at bp + 0x80
        // +0x10 -> r14 refs at bp + 0xa0
        // +0x14 -> r12_init at bp + 0xe0
        // +0x18 -> r8_src at bp + 0x120
        // +0x1c -> rdx_weights at bp + 0x140
        // +0x20 -> r9_indices at bp + 0x180
        wr::<i32>(entry + 0x08, (0x60isize - 0x28isize) as i32);
        wr::<i32>(entry + 0x0c, (0x80isize - 0x2cisize) as i32);
        wr::<i32>(entry + 0x10, (0xa0isize - 0x30isize) as i32);
        wr::<i32>(entry + 0x14, (0xe0isize - 0x34isize) as i32);
        wr::<i32>(entry + 0x18, (0x120isize - 0x38isize) as i32);
        wr::<i32>(entry + 0x1c, (0x140isize - 0x3cisize) as i32);
        wr::<i32>(entry + 0x20, (0x180isize - 0x40isize) as i32);

        // 6-byte descriptor at bp + 0x60:
        // [0] = mode_b0, [1] = n_elems(1), [2] = b2(1), [3] = b3(1), [4] = b4(1), [5] = b5(2)
        wr::<[u8; 6]>(bp + 0x60, [mode_b0, 1, 1, 1, 1, 2]);

        // r12 coeffs at bp + 0x80 (1 vec4)
        wr::<[f32; 4]>(bp + 0x80, [1.0, -2.0, 0.5, 0.0]);
        // r14 refs at bp + 0xa0 (1 vec4)
        wr::<[f32; 4]>(bp + 0xa0, [0.5, 0.25, -0.5, 1.0]);
        // r12_init at bp + 0xe0 (8 floats = 32 bytes)
        wr::<[f32; 8]>(bp + 0xe0, [2.0, 1.0, 0.5, 0.25, 0.1, 0.2, 0.3, 0.4]);
        // r8_src at bp + 0x120 (4 floats = 16 bytes)
        wr::<[f32; 4]>(bp + 0x120, [5.0, 6.0, 7.0, 8.0]);
        // rdx_weights at bp + 0x140 (2 vec4s for b5=2, b4=1)
        wr::<[f32; 4]>(bp + 0x140, [0.5, 0.25, 0.125, 0.0]);
        wr::<[f32; 4]>(bp + 0x150, [1.0, 0.5, 0.25, 0.0]);
        // r9_indices at bp + 0x180: component 1 (<8, in param_2) and component 9 (>=8, in param_3)
        wr::<[u8; 2]>(bp + 0x180, [1, 9]);

        let idx_arr0: [u16; 2] = [0, 1];
        let idx_arr1: [u16; 2] = [1, 0];

        let mut v0 = [0u8; 0xc8];
        wr::<usize>(v0.as_mut_ptr() as usize + 0xb8, bp);

        let p1: [usize; 4] = [
            v0.as_ptr() as usize,
            idx_arr0.as_ptr() as usize,
            0,
            idx_arr1.as_ptr() as usize,
        ];

        let mut p2_buf = [0u32; 16]; // 2 elements * 32 bytes
        wr::<[f32; 8]>(
            p2_buf.as_mut_ptr() as usize,
            [1.5, -0.5, 2.0, 1.0, 3.0, 4.0, 5.0, 6.0],
        );
        let mut p3_buf = [0u32; 8]; // 2 elements * 16 bytes
        let mut p4_buf = [0u32; 8]; // 8 floats = 32 bytes

        f(
            p1.as_ptr() as usize,
            p2_buf.as_mut_ptr() as usize,
            p3_buf.as_mut_ptr() as usize,
            p4_buf.as_mut_ptr() as usize,
            0,
        );
        (p2_buf, p3_buf, p4_buf)
    };

    for (mode_b0, copy_flag) in [(0u8, 1u8), (1, 0)] {
        let o = run(f_o, mode_b0, copy_flag);
        let n = run(f_n, mode_b0, copy_flag);
        crate::diff_eq!(o, n, "1421bbb90 mode_b0={} copy_flag={}", mode_b0, copy_flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421d5b40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_TAG: u64 = 0;

    unsafe extern "win64" fn mock_9e40(_arr: u64, _max_idx: u64, key: u32) -> u32 {
        key
    }
    unsafe extern "win64" fn mock_4c70(_p1: usize, rdx: u64, r8: usize) {
        let tag_off = r8.wrapping_sub(exe::ptr::<u8>(0x1_4386_5860) as usize) as u64;
        LOG_TAG = rdx.wrapping_add(tag_off);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9e40, mock_9e40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421d_4c70, mock_4c70 as *const ())?;

    let run = |f: F, pre_set: bool| -> (u64, u64, u32, u64, u64) {
        LOG_TAG = 0;
        let mut bits: u64 = if pre_set { 1u64 << 6 } else { 0 };
        let mut p160 = [0u8; 0x140];
        let p160p = p160.as_mut_ptr() as usize;
        wr::<u64>(p160p + 0x60, 0x1111_0000);
        wr::<u64>(p160p + 0x128, 0x2222_0000);
        wr::<u16>(p160p + 0x130, 10);

        let mut p168 = [0u8; 0x90];
        wr::<usize>(
            p168.as_mut_ptr() as usize + 0x88,
            (&mut bits as *mut u64) as usize,
        );

        let mut p1 = [0u8; 0x1c0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x14, 7);
        wr::<u64>(p1p + 0x20, 0x5000);
        wr::<u32>(p1p + 0x4c, 3);
        wr::<usize>(p1p + 0x160, p160p);
        wr::<usize>(p1p + 0x168, p168.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x1b0, 5); // found = 5 -> bit_idx = 6

        let ret = f(p1p);
        (ret, bits, rd::<u32>(p1p + 0x14), rd::<u64>(p1p + 0x20), LOG_TAG)
    };

    for pre_set in [false, true] {
        let o = run(f_o, pre_set);
        let n = run(f_n, pre_set);
        crate::diff_eq!(o, n, "1421d5b40 pre_set={}", pre_set);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421d5c20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_SUM: u64 = 0;

    unsafe extern "win64" fn mock_4c70(_p1: usize, rdx: u64, r8: usize) {
        let off = r8.wrapping_sub(exe::ptr::<u8>(0x1_4386_5600) as usize) as u64;
        CALL_SUM = CALL_SUM.wrapping_mul(31).wrapping_add(rdx).wrapping_add(off);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421d_4c70, mock_4c70 as *const ())?;

    let run = |f: F, r10_cnt: u32| -> (u64, u64, [u64; 2], u32, usize, u64) {
        CALL_SUM = 0;
        let bits_r13: u64 = 0b11; // bit 0 and bit 1 active
        let bits_r12: u64 = 0b01; // bit 0 preserved, bit 1 reset to 0
        let mut bits_18: u64 = 0;
        let mut bits_88: [u64; 2] = [0xdead_beef_cafe_babe, 0x1234_5678_9abc_def0];
        let mut r14_state: [u64; 2] = [(0.25f32.to_bits() as u64) << 32 | 1, 0x9999];

        let mut p168 = [0u8; 0x90];
        let p168p = p168.as_mut_ptr() as usize;
        wr::<usize>(p168p + 0x08, (&bits_r13 as *const u64) as usize);
        wr::<usize>(p168p + 0x10, (&bits_r12 as *const u64) as usize);
        wr::<usize>(p168p + 0x18, (&mut bits_18 as *mut u64) as usize);
        wr::<usize>(p168p + 0x20, r14_state.as_mut_ptr() as usize);
        wr::<usize>(p168p + 0x88, bits_88.as_mut_ptr() as usize);

        // Stream buffer for [param_1 + 0x20]
        // Entry at +0x80: [r9 - 0x0c] = dur (f32), [r9 - 0x08] = r10d (u32), [r9 - 0x04] = rel_off to float table
        let mut stream = [0u8; 0xc0];
        let sp = stream.as_mut_ptr() as usize;
        let r9_0 = sp + 0x80;
        wr::<f32>(r9_0 - 0x0c, 2.0);
        wr::<u32>(r9_0 - 0x08, r10_cnt);
        wr::<[f32; 4]>(sp + 0x50, [0.1, 0.4, 0.8, 1.0]);
        wr::<i32>(r9_0 - 0x04, (0x50isize - 0x7cisize) as i32);

        // 1 entry in [p160 + 0x48] (0x18 bytes)
        let mut item = [0u8; 0x18];
        let ip = item.as_mut_ptr() as usize;
        wr::<u16>(ip, 0);
        wr::<u64>(ip + 0x08, 0x101);
        wr::<u64>(ip + 0x10, 0x202);

        let map_20: [u16; 2] = [3, 7];

        let mut p160 = [0u8; 0x140];
        let p160p = p160.as_mut_ptr() as usize;
        wr::<usize>(p160p + 0x20, map_20.as_ptr() as usize);
        wr::<usize>(p160p + 0x48, ip);
        wr::<u64>(p160p + 0x50, 1);
        wr::<u64>(p160p + 0x58, 0x303);
        wr::<u16>(p160p + 0x130, 80); // (80 + 63) >> 6 = 2 u64 words

        let mut p1 = [0u8; 0x180];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x14, 5);
        wr::<usize>(p1p + 0x20, r9_0);
        wr::<f32>(p1p + 0x48, 0.5);
        wr::<usize>(p1p + 0x160, p160p);
        wr::<usize>(p1p + 0x168, p168p);

        f(p1p);
        (
            bits_18,
            r14_state[0],
            bits_88,
            rd::<u32>(p1p + 0x14),
            rd::<usize>(p1p + 0x20).wrapping_sub(sp),
            CALL_SUM,
        )
    };

    for r10_cnt in [0u32, 2u32] {
        let o = run(f_o, r10_cnt);
        let n = run(f_n, r10_cnt);
        crate::diff_eq!(o, n, "1421d5c20 r10_cnt={}", r10_cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142207340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_EVENTS: u64 = 0;

    unsafe extern "win64" fn mock_e500(_rcx: usize, edx: u32) {
        LOG_EVENTS = LOG_EVENTS.wrapping_add(0x100 + edx as u64);
    }
    unsafe extern "win64" fn mock_8970(rbx: usize) {
        LOG_EVENTS = LOG_EVENTS.wrapping_add(0x1000 + rd::<u8>(rbx + 0xbc) as u64);
    }
    unsafe extern "win64" fn mock_d6f0(rcx: usize, _rdx: usize) {
        LOG_EVENTS = LOG_EVENTS.wrapping_add(rcx as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4207_e500, mock_e500 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_8970, mock_8970 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let g_148 = exe::ptr::<u32>(0x1_43a2_b148) as usize;
    let s_148 = rd::<u32>(g_148);

    let run = |f: F| -> (u32, u64, u32, u8, u8, u64, u64) {
        LOG_EVENTS = 0;
        wr::<u32>(g_148, 10);

        // Object 0: becomes ready (bpl == bd) and gets removed via swap-with-last
        let mut obj0 = [0u8; 0x1b0];
        let o0 = obj0.as_mut_ptr() as usize;
        wr::<u32>(o0 + 0x24, 1); // refcount 1 -> triggers 14241d6f0 on removal
        wr::<usize>(o0 + 0x28, 0x5555);
        wr::<u8>(o0 + 0xb9, 1);
        wr::<u8>(o0 + 0xbb, 3);
        wr::<u8>(o0 + 0xbc, 2);
        wr::<u8>(o0 + 0xbd, 1);
        wr::<u64>(o0 + 0x118 + 8, 0xaaaa_bbbb);
        wr::<u8>(o0 + 0x158, 2); // sil=0 has state 2 -> calls 14207e500
        wr::<u8>(o0 + 0x158 + 1, 2); // al=1 has state 2 and <= bd(1) -> bpl becomes 1
        wr::<u8>(o0 + 0x158 + 2, 2); // last_idx=2 has state 2 -> clears +0xb9 and decrements g_148

        // Object 1: stays in list (state 1 at sil=0 makes r14b = false), updates min frame
        let mut obj1 = [0u8; 0x1b0];
        let o1 = obj1.as_mut_ptr() as usize;
        wr::<u32>(o1 + 0x24, 2);
        wr::<u8>(o1 + 0xb9, 0);
        wr::<u8>(o1 + 0xbb, 3);
        wr::<u8>(o1 + 0xbc, 2);
        wr::<u8>(o1 + 0xbd, 2);
        wr::<u8>(o1 + 0x158, 1); // state 1 -> r14b = false

        let mut list: [u64; 4] = [o0 as u64, 50, o1 as u64, 30];
        let mut p1 = [0u8; 0x290];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 100);
        wr::<usize>(p1p + 0x280, list.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x288, 2);

        f(p1p);
        (
            rd::<u32>(g_148),
            rd::<u64>(p1p + 0x20),
            rd::<u32>(p1p + 0x288),
            rd::<u8>(o0 + 0xb9),
            rd::<u8>(o0 + 0xbc),
            rd::<u64>(o0 + 0x1a0),
            LOG_EVENTS,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u32>(g_148, s_148);
    crate::diff_eq!(o, n, "142207340 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142210fa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut params = [0.0f32; 24];
    for (i, v) in params.iter_mut().enumerate() {
        *v = (i as f32 + 1.0) * 0.125;
    }

    let run = |f: F| -> Vec<u32> {
        let mut out = vec![0xdead_beefu32; 0xc00 / 4];
        f(out.as_mut_ptr() as usize, 0, params.as_ptr() as usize);
        out
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142210fa0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1422e4a90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_ACC: u64 = 0;

    unsafe extern "win64" fn mock_noop2(_a: usize, _b: u64) {}
    unsafe extern "win64" fn mock_06c0(_p1: usize, _elem: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x06c0);
    }
    unsafe extern "win64" fn mock_0890(_p1: usize, _elem: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0890);
    }
    unsafe extern "win64" fn mock_0440(_p1: usize, _item: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0440);
    }
    unsafe extern "win64" fn mock_0260(_p1: usize, _item: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0260);
    }
    unsafe extern "win64" fn mock_0550(_p1: usize, _item: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0550);
    }
    unsafe extern "win64" fn mock_0340(_p1: usize, _item: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0340);
    }
    unsafe extern "win64" fn mock_9ab0(_rcx: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x9ab0);
    }
    unsafe extern "win64" fn mock_d670(_rcx: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0xd670);
    }
    unsafe extern "win64" fn mock_7fc0(_rcx: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x7fc0);
    }
    unsafe extern "win64" fn mock_2de0(_rcx: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x2de0);
    }
    unsafe extern "win64" fn mock_01f0(h: u32) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x01f0 + h as u64);
    }
    unsafe extern "win64" fn mock_0010(h: u32) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0010 + h as u64);
    }
    unsafe extern "win64" fn mock_0ed0(_rcx: usize) {
        TRACE_ACC = TRACE_ACC.wrapping_add(0x0ed0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_6d50, mock_noop2 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422e_06c0, mock_06c0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_422e_0890, mock_0890 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4020_bcb0, mock_noop2 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_422e_0440, mock_0440 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_422e_0260, mock_0260 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_422e_0550, mock_0550 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_422e_0340, mock_0340 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4222_9ab0, mock_9ab0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_411e_7fc0, mock_7fc0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4233_01f0, mock_01f0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4233_0010, mock_0010 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_422e_0ed0, mock_0ed0 as *const ())?;

    let g_fed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let g_ff48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let g_b38 = exe::ptr::<u32>(0x1_4386_bb38) as usize;
    let g_b88 = exe::ptr::<u8>(0x1_4386_bb88) as usize;
    let g_b58 = exe::ptr::<u8>(0x1_4386_bb58) as usize;
    let saved = (
        rd::<usize>(g_fed0),
        rd::<usize>(g_ff48),
        rd::<u32>(g_b38),
        rd::<u8>(g_b88),
        rd::<u8>(g_b58),
    );

    let run = |f: F, b88_flag: u8| -> (u64, u32, u32, u8, u8, u32) {
        TRACE_ACC = 0;
        wr::<u32>(g_b38, 1);
        wr::<u8>(g_b88, b88_flag);
        wr::<u8>(g_b58, 1);

        let mut sub_obj = [0u8; 0x70];
        let sp = sub_obj.as_mut_ptr() as usize;
        wr::<u16>(sp + 0x28, 0);
        wr::<u32>(sp + 0x54, 0);
        wr::<u32>(sp + 0x58, 0);
        wr::<i32>(sp + 0x60, -1); // state_60 == -1 so valid branch pushes to +0x90

        let mut sub_obj2 = [0u8; 0x70];
        let sp2 = sub_obj2.as_mut_ptr() as usize;
        wr::<i32>(sp2 + 0x60, 1); // state_60 != -1

        let mut rdx_90 = [0u8; 0x280];
        let r90p = rdx_90.as_mut_ptr() as usize;
        wr::<u32>(r90p + 0x1c8, 1);
        wr::<u8>(r90p + 0x270, 5);
        wr::<u8>(r90p + 0x272, 2);

        let u32_list: [u32; 1] = [0];
        let mut r8_item = [0u8; 0xa0];
        let r8p = r8_item.as_mut_ptr() as usize;
        wr::<usize>(r8p + 0x40, sp);
        wr::<usize>(r8p + 0x80, u32_list.as_ptr() as usize);
        wr::<u32>(r8p + 0x88, 1);
        wr::<usize>(r8p + 0x90, r90p);

        let arr_100: [u32; 1] = [0];
        let mut fed0_obj = [0u8; 0x130];
        wr::<usize>(
            fed0_obj.as_mut_ptr() as usize + 0x100,
            arr_100.as_ptr() as usize,
        );
        wr::<usize>(fed0_obj.as_mut_ptr() as usize + 0x120, r8p);
        wr::<usize>(g_fed0, fed0_obj.as_ptr() as usize);

        let mut obj_4c = [0u8; 0x50];
        wr::<u8>(obj_4c.as_mut_ptr() as usize + 0x4c, 0);
        let arr_18: [usize; 1] = [obj_4c.as_ptr() as usize];
        let arr_1a8: [u8; 1] = [0];
        let bits_668: [u64; 1] = [1];
        let mut ff48_obj = [0u8; 0x670];
        let ffp = ff48_obj.as_mut_ptr() as usize;
        wr::<usize>(ffp + 0x18, arr_18.as_ptr() as usize);
        wr::<usize>(ffp + 0x1a8, arr_1a8.as_ptr() as usize);
        wr::<usize>(ffp + 0x668, bits_668.as_ptr() as usize);
        wr::<usize>(g_ff48, ffp);

        let mut elems_20 = [0u8; 0x60];
        let e20p = elems_20.as_mut_ptr() as usize;
        // elem 0 (for bit 0 in +0x60): valid -> pushes to +0x90
        wr::<u32>(e20p, 0);
        wr::<u32>(e20p + 0x04, 0);
        wr::<usize>(e20p + 0x08, sp);
        // elem 1 (for bit 1 in +0x78): active state_60 != -1
        wr::<u32>(e20p + 0x30, 0);
        wr::<u32>(e20p + 0x34, 0);
        wr::<usize>(e20p + 0x38, sp2);

        let mut bits_60: [u64; 1] = [1];
        let mut bits_78: [u64; 1] = [2];
        let mut buf_90: [u64; 4] = [0; 4];
        let mut buf_a0: [u64; 4] = [0; 4];
        let mut buf_b0: [u64; 4] = [0; 4];

        let sub_18: [u32; 2] = [0, 5];
        let mut dummy_rbp = [0u8; 0x60];
        let mut elem_30 = [0u8; 0x38];
        let e30p = elem_30.as_mut_ptr() as usize;
        wr::<u32>(e30p, 7);
        wr::<usize>(e30p + 0x08, dummy_rbp.as_mut_ptr() as usize);
        wr::<u32>(e30p + 0x10, 5);
        wr::<usize>(e30p + 0x18, sub_18.as_ptr() as usize);
        wr::<u32>(e30p + 0x20, 1);

        let mut elem_48 = [0u8; 0x120];

        let mut p1 = [0u8; 0xd0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x20, e20p);
        wr::<u32>(p1p + 0x28, 2);
        wr::<usize>(p1p + 0x30, e30p);
        wr::<u32>(p1p + 0x38, 1);
        wr::<u64>(p1p + 0x40, 0x8000_0000);
        wr::<usize>(p1p + 0x48, elem_48.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x50, 1);
        wr::<u8>(p1p + 0x58, 0);
        wr::<u32>(p1p + 0x5c, 10);
        wr::<usize>(p1p + 0x60, bits_60.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x70, 2);
        wr::<usize>(p1p + 0x78, bits_78.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x88, 2);
        wr::<usize>(p1p + 0x90, buf_90.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x9c, 4);
        wr::<usize>(p1p + 0xa0, buf_a0.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0xac, 4);
        wr::<usize>(p1p + 0xb0, buf_b0.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0xbc, 4);
        wr::<u8>(p1p + 0xc0, 1);

        f(p1p);
        (
            TRACE_ACC,
            rd::<u32>(p1p + 0x5c),
            rd::<u32>(p1p + 0x98),
            rd::<u8>(p1p + 0x58),
            rd::<u8>(p1p + 0xc0),
            rd::<u32>(e30p),
        )
    };

    for b88 in [1u8, 0u8] {
        let o = run(f_o, b88);
        let n = run(f_n, b88);
        wr::<usize>(g_fed0, saved.0);
        wr::<usize>(g_ff48, saved.1);
        wr::<u32>(g_b38, saved.2);
        wr::<u8>(g_b88, saved.3);
        wr::<u8>(g_b58, saved.4);
        crate::diff_eq!(o, n, "1422e4a90 b88={}", b88);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1422e52c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, usize, u32, u64, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VTBL: [usize; 8] = [0; 8];
    static mut TAIL_SUM: u64 = 0;

    unsafe extern "win64" fn mock_f920() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_get_cap(_this: usize) -> u64 {
        8
    }
    unsafe extern "win64" fn mock_grow(_this: usize, _new_cap: u64) {}
    unsafe extern "win64" fn mock_1780(rcx: u64, rdx: usize) {
        TAIL_SUM = rcx.wrapping_add(rd::<u32>(rdx + 0x48) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_422d_f920, mock_f920 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422e_1780, mock_1780 as *const ())?;

    let g_1e8 = exe::ptr::<usize>(0x1_43a0_01e8) as usize;
    let g_f70 = exe::ptr::<u64>(0x1_4397_ff70) as usize;
    let s_1e8 = rd::<usize>(g_1e8);
    let s_f70 = rd::<u64>(g_f70);

    let run = |f: F| -> (Vec<u8>, Vec<u8>, [u64; 10], [u32; 8], [u32; 16], [u32; 8], [u64; 2], u32, u64) {
        TAIL_SUM = 0;
        VTBL = [0; 8];
        VTBL[0x28 / 8] = mock_grow as *const () as usize;
        VTBL[0x38 / 8] = mock_get_cap as *const () as usize;
        let vp = core::ptr::addr_of!(VTBL) as usize;

        let mut arr_98 = [0u64; 10]; // 2 * 0x28 bytes
        arr_98[0] = 0x1111_2222_3333_4444;
        arr_98[1] = 0x5555_6666_7777_8888;
        arr_98[2] = 0x9999_aaaa_bbbb_cccc;
        arr_98[3] = 0xdddd_eeee_ffff_0001;
        arr_98[4] = 0x1234_5678_9abc_def0;
        let mut bits_a8: u64 = 0;

        let mut arr_168 = [0u32; 8];
        let mut bits_178: u64 = 0;

        let mut arr_238 = [0u32; 8];
        let mut bits_248: u64 = 0;

        let mut arr_308 = [0u64; 2];
        let mut bits_318: u64 = 0;

        let mut arr_3d8 = [0u32; 16];
        let mut bits_3e8: u64 = 0;

        let mut mgr = vec![0u8; 0x420];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp, vp);
        wr::<usize>(mp + 0x98, arr_98.as_mut_ptr() as usize);
        wr::<usize>(mp + 0xa8, (&mut bits_a8 as *mut u64) as usize);

        wr::<usize>(mp + 0xd0, vp);
        wr::<usize>(mp + 0x168, arr_168.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x178, (&mut bits_178 as *mut u64) as usize);

        wr::<usize>(mp + 0x1a0, vp);
        wr::<usize>(mp + 0x238, arr_238.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x248, (&mut bits_248 as *mut u64) as usize);

        wr::<usize>(mp + 0x270, vp);
        wr::<usize>(mp + 0x308, arr_308.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x318, (&mut bits_318 as *mut u64) as usize);

        wr::<usize>(mp + 0x340, vp);
        wr::<usize>(mp + 0x3d8, arr_3d8.as_mut_ptr() as usize);
        wr::<usize>(mp + 0x3e8, (&mut bits_3e8 as *mut u64) as usize);

        wr::<usize>(g_1e8, mp);
        wr::<u64>(g_f70, 0x7000);

        let mut r10 = vec![0u8; 0x160];
        let r10p = r10.as_mut_ptr() as usize;
        wr::<u8>(r10p + 0x01, 1);
        wr::<u8>(r10p + 0x02, 2); // matches ((b1 - 2) & 0xfd) == 0
        wr::<u8>(r10p + 0x40, 1);
        wr::<u32>(r10p + 0x48, 11);
        wr::<u16>(r10p + 0x50, 100);
        wr::<u64>(r10p + 0x80, 0x8888);
        wr::<u16>(r10p + 0xa8, 7);
        wr::<u32>(r10p + 0xac, 22);
        wr::<u32>(r10p + 0xb0, 33);
        wr::<u32>(r10p + 0xb4, 4);
        wr::<u32>(r10p + 0xc0, 44);
        wr::<u32>(r10p + 0xc8, 5);
        wr::<u32>(r10p + 0xd0, 4); // maps to +0x2e = 1
        wr::<u32>(r10p + 0xd8, 0);
        wr::<u32>(r10p + 0xe4, 55);
        wr::<u32>(r10p + 0xe8, 66);
        wr::<u32>(r10p + 0xec, 77);
        wr::<u32>(r10p + 0xf0, 88);
        wr::<u32>(r10p + 0x11c, 99);
        wr::<u32>(r10p + 0x120, 101);
        wr::<u32>(r10p + 0x124, 3);
        wr::<u32>(r10p + 0x130, 102);
        wr::<u32>(r10p + 0x140, 201);
        wr::<u32>(r10p + 0x144, 202);
        wr::<u32>(r10p + 0x148, 203);
        wr::<u32>(r10p + 0x14c, 204);

        let p3: [u64; 2] = [r10p as u64, 42];
        let mut p5_buf = vec![0u8; 0x68];
        let mut p1 = vec![0u8; 0x68];
        let p1p = p1.as_mut_ptr() as usize;

        f(
            p1p,
            9,
            p3.as_ptr() as usize,
            1,
            p5_buf.as_mut_ptr() as usize as u64,
            0xaaaa,
            0xbbbb,
        );
        wr::<u64>(p1p + 0x08, 0); // clear p5_buf pointer
        arr_168[7] = 0; // clear uninitialized stack padding dword at [rsp + 0x2c] in original
        (
            p1,
            p5_buf,
            arr_98,
            arr_168,
            arr_3d8,
            arr_238,
            arr_308,
            rd::<u32>(mp + 0x410),
            TAIL_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_1e8, s_1e8);
    wr::<u64>(g_f70, s_f70);
    crate::diff_eq!(o, n, "1422e52c0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_421b_bb90, "unknown_1421bbb90", rend::cauldron_hot_w6a::unknown_1421bbb90 as *const (), rend::cauldron_hot_w6a::diff_unknown_1421bbb90),
// Replacement::verified(0x1_421d_5b40, "unknown_1421d5b40", rend::cauldron_hot_w6a::unknown_1421d5b40 as *const (), rend::cauldron_hot_w6a::diff_unknown_1421d5b40),
// Replacement::verified(0x1_421d_5c20, "unknown_1421d5c20", rend::cauldron_hot_w6a::unknown_1421d5c20 as *const (), rend::cauldron_hot_w6a::diff_unknown_1421d5c20),
// Replacement::verified_throws(0x1_4220_7340, "unknown_142207340", rend::cauldron_hot_w6a::unknown_142207340 as *const (), Throws::Caught, rend::cauldron_hot_w6a::diff_unknown_142207340),
// Replacement::verified(0x1_4221_0fa0, "unknown_142210fa0", rend::cauldron_hot_w6a::unknown_142210fa0 as *const (), rend::cauldron_hot_w6a::diff_unknown_142210fa0),
// Replacement::verified_throws(0x1_422e_4a90, "unknown_1422e4a90", rend::cauldron_hot_w6a::unknown_1422e4a90 as *const (), Throws::Caught, rend::cauldron_hot_w6a::diff_unknown_1422e4a90),
// Replacement::verified(0x1_422e_52c0, "unknown_1422e52c0", rend::cauldron_hot_w6a::unknown_1422e52c0 as *const (), rend::cauldron_hot_w6a::diff_unknown_1422e52c0),
