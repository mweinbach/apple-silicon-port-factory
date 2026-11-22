//! Decompiled hot functions shard V4 (`0x141f91130..0x1420b8fe0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vrsqrtps_128(a: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{__m128, _mm_rsqrt_ps};
        let v: __m128 = core::mem::transmute(a);
        core::mem::transmute(_mm_rsqrt_ps(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vrsqrtps(a)
    }
}

/// `unknown_141f91130`
/// Original: 0x141f91130 (2340 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f91130(param_1: usize, param_2: usize) {
    let id_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
    let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
    let r12_float_ptr = rd::<usize>(tbl_580e0.wrapping_add(id_0d8 * 8));

    let mut total_ents: u64 = 0;
    let mut query1 = rd::<[u64; 2]>(param_1 + 0x08);
    let r11 = rd::<usize>(param_1);
    let r15 = r11.wrapping_add(0x40);
    let mut out_180 = [0u64; 4];
    let iter1_ptr = exe::call_va!(
        0x1_401e_7c10,
        (
            out_180.as_mut_ptr() as usize,
            r15,
            query1.as_mut_ptr() as usize
        ) -> usize
    );

    let iter1_rdx = rd::<usize>(iter1_ptr);
    let mut cur1 = rd::<usize>(iter1_ptr + 0x08);
    let end1 = rd::<usize>(iter1_ptr + 0x10);
    while cur1 != end1 {
        let arch = rd::<u16>(cur1) as usize;
        let cnt = if rd::<u64>(r11.wrapping_add(arch * 8).wrapping_add(0x88)) != 0 {
            rd::<u32>(r11.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
        } else {
            0
        };
        total_ents = total_ents.wrapping_add(cnt);
        cur1 = cur1.wrapping_add(2);
        while cur1 != end1 {
            let a = rd::<u16>(cur1) as usize;
            if rd::<u64>(iter1_rdx.wrapping_add(a * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(iter1_rdx.wrapping_add(a * 4).wrapping_add(0x10048)) != 0
            {
                break;
            }
            cur1 = cur1.wrapping_add(2);
        }
    }

    let g_f58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let r9_bytes = total_ents.wrapping_mul(0xa0);
    let r13_len = r9_bytes.wrapping_add(4);

    let (hdr_ptr, data_start, cmd_len) = if r13_len < 0x00ff_ffc0 {
        let ring = rd::<usize>(g_f58 + 0x168);
        exe::call_iat!(0x1_4264_55a8, (ring + 0x18));
        let r9_aligned = r13_len.wrapping_add(7) & !7u64;
        let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
        let mut next_wr = wr_ptr.wrapping_add(8).wrapping_add(r9_aligned as usize);
        let ring_end = rd::<usize>(ring + 0x10);
        if next_wr > ring_end {
            while wr_ptr <= rd_ptr
                && rd_ptr < next_wr
                && crate::simd_ops::rd_acq_u32(ring + 0x30)
                    != crate::simd_ops::rd_acq_u32(ring + 0x34)
            {
                core::hint::spin_loop();
                rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
            }
            if wr_ptr <= ring_end.wrapping_sub(8) {
                crate::simd_ops::wr_rel_u64(wr_ptr, 0x00ff_ffff_ffff);
            }
            wr_ptr = rd::<usize>(ring + 0x08);
            next_wr = wr_ptr.wrapping_add(8).wrapping_add(r9_aligned as usize);
        }
        while wr_ptr <= rd_ptr
            && rd_ptr < next_wr
            && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        }
        crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
        let rsi_hdr = wr_ptr.wrapping_add(8);
        crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
        exe::call_iat!(0x1_4264_5598, (ring + 0x18));
        let hdr_val = ((total_ents as u32) << 9) | 0x0a;
        wr::<u32>(rsi_hdr, hdr_val);
        (rsi_hdr, rsi_hdr.wrapping_add(4), r13_len)
    } else {
        let alloc_ptr = exe::call_va!(0x1_4020_c080, (0u64, 0u64, 0u64, r9_bytes) -> usize);
        let ring = rd::<usize>(g_f58 + 0x168);
        exe::call_iat!(0x1_4264_55a8, (ring + 0x18));
        let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
        let mut next_wr = wr_ptr.wrapping_add(0x20);
        let ring_end = rd::<usize>(ring + 0x10);
        if next_wr > ring_end {
            while wr_ptr <= rd_ptr
                && rd_ptr < next_wr
                && crate::simd_ops::rd_acq_u32(ring + 0x30)
                    != crate::simd_ops::rd_acq_u32(ring + 0x34)
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
        let rsi_hdr = wr_ptr.wrapping_add(8);
        crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
        exe::call_iat!(0x1_4264_5598, (ring + 0x18));
        let hdr_val = ((total_ents as u32) << 9) | 1;
        wr::<u32>(rsi_hdr + 0x04, 0x0a);
        wr::<usize>(rsi_hdr + 0x0c, alloc_ptr);
        wr::<u32>(rsi_hdr, hdr_val);
        (rsi_hdr, alloc_ptr, 0x14u64)
    };

    let mut query2 = rd::<[u64; 2]>(param_1 + 0x08);
    let mut out_110 = [0u64; 4];
    let _ = exe::call_va!(
        0x1_401e_7c10,
        (
            out_110.as_mut_ptr() as usize,
            r15,
            query2.as_mut_ptr() as usize
        ) -> usize
    );

    let mut iter_ctx = [0u64; 10];
    iter_ctx[0] = r15 as u64;
    iter_ctx[1] = out_110[0];
    iter_ctx[2] = out_110[1];
    iter_ctx[3] = out_110[2];
    iter_ctx[5] = 0;
    iter_ctx[6] = 0;
    if iter_ctx[2] != iter_ctx[3] {
        let arch = rd::<u16>(iter_ctx[2] as usize) as usize;
        let blk = rd::<u64>(r15.wrapping_add(arch * 8).wrapping_add(0x48));
        iter_ctx[4] = blk;
        iter_ctx[5] = if blk != 0 {
            rd::<u32>(r15.wrapping_add(arch * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4110_b4d0, (iter_ctx.as_mut_ptr() as usize));

    let mut cur_out = data_start;
    if iter_ctx[6] < iter_ctx[5] {
        let xmm14 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
        let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
        let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
        let c_8240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
        let c_81b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
        let c_8840 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
        let c_80c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize);
        let c_8280 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
        let c_88d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
        let c_84c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
        let c_82c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize);
        let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);
        let c_83b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_83b0) as usize);

        let mut local_1e0 = [0u64; 18];
        loop {
            loop {
                let rdi = iter_ctx[6] as usize;
                let rax_off = rdi.wrapping_shl(5);
                let rcx_68 = iter_ctx[7] as usize;
                let rdx_70 = iter_ctx[8] as usize;
                let rbx = (iter_ctx[9] as usize).wrapping_add(rdi.wrapping_shl(7));

                if rd::<u8>(rbx + 0x78) != 0 {
                    let xmm13 = crate::simd_ops::vbroadcastss(rd::<u32>(r12_float_ptr));
                    let ymm12_lo = rd::<u128>(rdx_70.wrapping_add(rax_off));
                    let ymm12_hi = rd::<u128>(rdx_70.wrapping_add(rax_off).wrapping_add(0x10));
                    let ymm11_lo = rd::<u128>(rcx_68.wrapping_add(rax_off));
                    let ymm11_hi = rd::<u128>(rcx_68.wrapping_add(rax_off).wrapping_add(0x10));

                    let xmm3 = crate::simd_ops::vsubps(xmm13, c_8120);
                    let xmm1_dp = crate::simd_ops::vdpps(ymm11_lo, ymm12_lo, 0xff);
                    let xmm8 = xmm1_dp & c_8760;
                    let xmm6 = xmm8 ^ xmm1_dp;
                    let xmm1_m = crate::simd_ops::vmulps(xmm3, xmm13);
                    let xmm0_s = crate::simd_ops::vsubps(xmm13, c_8240);
                    let xmm7 = crate::simd_ops::vmulps(xmm1_m, xmm0_s);

                    let mut xmm5 = c_81b0;
                    let mut xmm2 = c_8840;
                    xmm2 = crate::simd_ops::vfmadd_ps(xmm6, c_80c0, xmm2, false, false);
                    xmm5 = crate::simd_ops::vfmadd_ps(xmm6, xmm2, xmm5, false, false);
                    let xmm4_sq = crate::simd_ops::vmulps(xmm3, xmm3);
                    let mut xmm3_p = c_8280;
                    let mut xmm2_p = c_88d0;
                    let mut xmm1_p = c_84c0;
                    xmm1_p = crate::simd_ops::vfmadd_ps(xmm6, c_82c0, xmm1_p, true, false);
                    xmm2_p = crate::simd_ops::vfmadd_ps(xmm6, xmm1_p, xmm2_p, false, false);
                    xmm3_p = crate::simd_ops::vfmadd_ps(xmm6, xmm2_p, xmm3_p, false, false);
                    xmm5 = crate::simd_ops::vfmadd_ps(xmm3_p, xmm4_sq, xmm5, false, false);
                    let xmm0_w = crate::simd_ops::vfmadd_ps(xmm5, xmm7, xmm13, false, false);
                    let mut xmm4_q =
                        crate::simd_ops::vfmadd_ps(xmm0_w, ymm11_lo, ymm11_lo, true, false);
                    let xmm0_wx = xmm0_w ^ xmm8;
                    xmm4_q = crate::simd_ops::vfmadd_ps(xmm0_wx, ymm12_lo, xmm4_q, false, false);

                    let xmm0_n = crate::simd_ops::vdpps(xmm4_q, xmm4_q, 0xff);
                    let xmm3_rsq = vrsqrtps_128(xmm0_n);
                    let xmm0_r1 = crate::simd_ops::vmulps(xmm3_rsq, xmm0_n);
                    let xmm1_r2 = crate::simd_ops::vmulps(xmm0_r1, xmm3_rsq);
                    let xmm2_nr = crate::simd_ops::vsubps(c_8460, xmm1_r2);
                    let xmm0_h = crate::simd_ops::vmulps(xmm3_rsq, c_8120);
                    let xmm1_s = crate::simd_ops::vmulps(xmm0_h, xmm2_nr);
                    let xmm5_q = crate::simd_ops::vmulps(xmm1_s, xmm4_q);

                    let xmm3_2q = crate::simd_ops::vmulps(xmm5_q, c_83b0);
                    let xmm4_2q2 = crate::simd_ops::vmulps(xmm5_q, xmm3_2q);
                    let xmm0_e0 = crate::simd_ops::vpermilps(xmm5_q, 0xe0);
                    let xmm2_e0 = crate::simd_ops::vmulps(xmm0_e0, c_83b0);
                    let xmm1_d9 = crate::simd_ops::vpermilps(xmm5_q, 0xd9);
                    let xmm6_xy = crate::simd_ops::vmulps(xmm2_e0, xmm1_d9);
                    let xmm0_ff = crate::simd_ops::vpermilps(xmm5_q, 0xff);
                    let xmm5_wz = crate::simd_ops::vmulps(xmm0_ff, xmm3_2q);

                    let xmm1_c1 = crate::simd_ops::vpermilps(xmm4_2q2, 0xc1);
                    let xmm2_diag = crate::simd_ops::vsubps(c_8240, xmm1_c1);
                    let xmm0_da = crate::simd_ops::vpermilps(xmm4_2q2, 0xda);
                    let xmm7_diag = crate::simd_ops::vsubps(xmm2_diag, xmm0_da);

                    let xmm1_d8 = crate::simd_ops::vpermilps(xmm6_xy, 0xd8);
                    let xmm0_d2 = crate::simd_ops::vpermilps(xmm5_wz, 0xd2);
                    let xmm3_add = crate::simd_ops::vaddps(xmm1_d8, xmm0_d2);

                    let xmm2_e1 = crate::simd_ops::vpermilps(xmm6_xy, 0xe1);
                    let xmm0_c9 = crate::simd_ops::vpermilps(xmm5_wz, 0xc9);
                    let xmm4_sub = crate::simd_ops::vsubps(xmm2_e1, xmm0_c9);

                    let xmm2_44 = crate::simd_ops::vshufps(xmm7_diag, xmm3_add, 0x44);
                    let xmm5_ee = crate::simd_ops::vshufps(xmm7_diag, xmm3_add, 0xee);
                    let xmm0_44 = crate::simd_ops::vshufps(xmm4_sub, 0, 0x44);
                    let xmm1_ee = crate::simd_ops::vshufps(xmm4_sub, 0, 0xee);
                    let xmm6_r0 = crate::simd_ops::vshufps(xmm2_44, xmm0_44, 0x88);
                    let xmm0_dd = crate::simd_ops::vshufps(xmm2_44, xmm0_44, 0xdd);
                    let xmm4_d2 = crate::simd_ops::vpermilps(xmm0_dd, 0xd2);
                    let xmm1_88 = crate::simd_ops::vshufps(xmm5_ee, xmm1_ee, 0x88);
                    let xmm3_c9 = crate::simd_ops::vpermilps(xmm1_88, 0xc9);

                    let mut xmm2_pos =
                        crate::simd_ops::vfmadd_ps(ymm11_hi, xmm13, ymm11_hi, true, false);
                    xmm2_pos = crate::simd_ops::vfmadd_ps(ymm12_hi, xmm13, xmm2_pos, false, false);
                    let xmm1_pos = crate::simd_ops::vblendps(c_8240, xmm2_pos, 0x07);

                    let xmm0_t0 = crate::simd_ops::vshufps(xmm6_r0, xmm4_d2, 0x4e);
                    let mat_row0 = crate::simd_ops::vshufps(xmm6_r0, xmm0_t0, 0x84);
                    let mat_row1 = crate::simd_ops::vshufps(xmm4_d2, xmm3_c9, 0x49);
                    let xmm0_t2 = crate::simd_ops::vshufps(xmm3_c9, xmm1_pos, 0x4e);
                    let mat_row2 = crate::simd_ops::vshufps(xmm0_t2, xmm1_pos, 0x98);
                    let mut local_e0: [u128; 3] = [mat_row0, mat_row1, mat_row2];

                    let v1c_lo = rd::<f32>(rbx + 0x1c);
                    let v1c_hi = rd::<f32>(rbx + 0x20);
                    let mut local_20 = [0.0f32; 4];
                    let arg_xmm3 = if rd::<u32>(rbx + 0x68) == 1 {
                        let avg_1c = ((xmm14 * v1c_hi) + (xmm14 * v1c_lo)) * xmm14;
                        let v24_lo = rd::<f32>(rbx + 0x24);
                        let v24_hi = rd::<f32>(rbx + 0x28);
                        let sum_24 = (xmm14 * v24_hi) + (xmm14 * v24_lo);
                        local_20[0] = avg_1c;
                        local_20[1] = avg_1c;
                        local_20[2] = sum_24 * xmm14;
                        (rd::<f32>(rbx + 0x30) + rd::<f32>(rbx + 0x2c)) * xmm14
                    } else {
                        local_20[0] = xmm14 * v1c_lo;
                        local_20[1] = xmm14 * v1c_hi;
                        local_20[2] = xmm14 * rd::<f32>(rbx + 0x24);
                        rd::<f32>(rbx + 0x2c)
                    };

                    let mut local_1a0 = [0u64; 8];
                    let _ = crate::simd_ops::call_va8_xmm(
                        0x1_4204_a140,
                        local_1a0.as_mut_ptr() as u64,
                        local_e0.as_mut_ptr() as u64,
                        local_20.as_mut_ptr() as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        [0, 0, 0, arg_xmm3.to_bits() as u128],
                    );
                    exe::call_va!(
                        0x1_4204_a3b0,
                        (
                            local_1e0.as_mut_ptr() as usize,
                            rbx,
                            local_1a0.as_mut_ptr() as usize
                        )
                    );

                    let out_slot = cur_out;
                    cur_out = cur_out.wrapping_add(0xa0);
                    wr::<u64>(out_slot, rd::<u64>(rbx + 0x70));
                    exe::call_va!(
                        0x1_4110_e410,
                        (out_slot.wrapping_add(0x10), local_1e0.as_mut_ptr() as usize)
                    );

                    let rdx_ref = local_1e0[9] as usize;
                    if rdx_ref != 0 {
                        let prev =
                            crate::simd_ops::atomic_xadd_u32((rdx_ref + 0x24) as u64, 0xffff_ffff);
                        if prev == 1 {
                            let rcx_mgr = rd::<usize>(rdx_ref + 0x28);
                            if rcx_mgr != 0 {
                                exe::call_va!(0x1_4241_d6f0, (rcx_mgr, rdx_ref));
                            }
                        }
                    }
                    local_1e0[9] = 0;
                }

                iter_ctx[6] = iter_ctx[6].wrapping_add(1);
                if iter_ctx[6] >= iter_ctx[5] {
                    break;
                }
            }

            let mut next_arch_ptr = (iter_ctx[2] as usize).wrapping_add(2);
            iter_ctx[2] = next_arch_ptr as u64;
            let end_arch_ptr = iter_ctx[3] as usize;
            if next_arch_ptr != end_arch_ptr {
                let rdx_38 = iter_ctx[1] as usize;
                loop {
                    let a = rd::<u16>(next_arch_ptr) as usize;
                    if rd::<u64>(rdx_38.wrapping_add(a * 8).wrapping_add(0x48)) != 0
                        && rd::<u32>(rdx_38.wrapping_add(a * 4).wrapping_add(0x10048)) != 0
                    {
                        break;
                    }
                    next_arch_ptr = next_arch_ptr.wrapping_add(2);
                    iter_ctx[2] = next_arch_ptr as u64;
                    if next_arch_ptr == end_arch_ptr {
                        break;
                    }
                }
                if next_arch_ptr != end_arch_ptr {
                    let a = rd::<u16>(next_arch_ptr) as usize;
                    let rdx_30 = iter_ctx[0] as usize;
                    let blk = rd::<u64>(rdx_30.wrapping_add(a * 8).wrapping_add(0x48));
                    iter_ctx[4] = blk;
                    iter_ctx[5] = if blk != 0 {
                        rd::<u32>(rdx_30.wrapping_add(a * 4).wrapping_add(0x10048)) as u64
                    } else {
                        0
                    };
                    iter_ctx[6] = 0;
                }
            }
            exe::call_va!(0x1_4110_b4d0, (iter_ctx.as_mut_ptr() as usize));
            if iter_ctx[6] >= iter_ctx[5] {
                break;
            }
        }
    }

    let g_f58_2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let actual_count = ((cur_out as isize).wrapping_sub(data_start as isize) / 0xa0) as u32;
    let masked = rd::<u32>(hdr_ptr) & 0x1ff;
    wr::<u32>(hdr_ptr, masked | (actual_count << 9));
    let ring2 = rd::<usize>(g_f58_2 + 0x168);
    crate::simd_ops::wr_rel_u64(hdr_ptr.wrapping_sub(8), cmd_len);
    let _ = crate::simd_ops::atomic_xadd_u32((ring2 + 0x30) as u64, 1);
    if rd::<u8>(g_f58_2 + 0x198) != 0 {
        let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(g_f58_2 + 0x1a0), 1u32, 0usize) -> i32);
    }
}

/// `unknown_141f91bd0`
/// Original: 0x141f91bd0 (667 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f91bd0(param_1: usize, param_2: usize) {
    let r14 = rd::<usize>(param_1 + 0x28);
    let rax = rd::<usize>(param_1);
    let rdi = rd::<usize>(param_1 + 0x18).wrapping_add(r14.wrapping_mul(0x70));
    let r15 = rd::<usize>(param_1 + 0x08).wrapping_add(r14.wrapping_mul(0x20));
    let rbp = rd::<usize>(param_1 + 0x10).wrapping_add(r14.wrapping_mul(0xc0));
    let flag_60 = rd::<u8>(rdi + 0x60);
    let ecx = rd::<u32>(rax.wrapping_add(r14.wrapping_mul(4)));

    if flag_60 != 0 {
        if ecx != 0 {
            let obj_50 = rd::<usize>(rdi + 0x50);
            let sub_obj = exe::call_va!(rd::<usize>(rd::<usize>(obj_50) + 0x48), (obj_50) -> usize);
            let val_58 = rd::<u64>(rdi + 0x58);
            let vfn_68 = rd::<usize>(rd::<usize>(sub_obj) + 0x68);
            exe::call_va!(vfn_68, (sub_obj, val_58, 1u8));
            exe::call_va!(0x1_4214_d650, (rdi));
            if rd::<u8>(rdi + 0x60) != 0 {
                let vfn_0 = rd::<usize>(rd::<usize>(rdi));
                exe::call_va!(vfn_0, (rdi, 0u32));
                wr::<u8>(rdi + 0x60, 0);
            }
        }
    } else if ecx == 0 {
        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
        let rbx_ptr = rd::<usize>(tbl_580e0.wrapping_add(id_63c * 8));
        let r12 = exe::call_va!(0x1_4021_7df0, (rbp.wrapping_add(0x98)) -> usize);
        let was_active = rd::<u8>(rdi + 0x60) != 0;

        let xmm1 = rd::<u128>(r15);
        let xmm5_c = rd::<u128>(exe::ptr::<u128>(0x1_4342_83b0) as usize);
        let xmm8_c = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
        let rbx_inner = rd::<usize>(rbx_ptr);

        let xmm0 = crate::simd_ops::vmulps(xmm5_c, xmm1);
        let xmm4 = crate::simd_ops::vmulps(xmm0, xmm1);
        let xmm0_e0 = crate::simd_ops::vpermilps(xmm1, 0xe0);
        let xmm2 = crate::simd_ops::vmulps(xmm0_e0, xmm5_c);
        let xmm3 = xmm1;
        let xmm1_d9 = crate::simd_ops::vpermilps(xmm1, 0xd9);
        let xmm7 = crate::simd_ops::vmulps(xmm2, xmm1_d9);
        let xmm1_m = crate::simd_ops::vmulps(xmm3, xmm5_c);
        let xmm0_ff = crate::simd_ops::vpermilps(xmm3, 0xff);
        let xmm5 = crate::simd_ops::vmulps(xmm1_m, xmm0_ff);

        let xmm0_c1 = crate::simd_ops::vpermilps(xmm4, 0xc1);
        let xmm2_s = crate::simd_ops::vsubps(xmm8_c, xmm0_c1);
        let xmm1_da = crate::simd_ops::vpermilps(xmm4, 0xda);
        let xmm6 = crate::simd_ops::vsubps(xmm2_s, xmm1_da);

        let xmm3_d2 = crate::simd_ops::vpermilps(xmm5, 0xd2);
        let xmm1_c9 = crate::simd_ops::vpermilps(xmm5, 0xc9);
        let xmm0_d8 = crate::simd_ops::vpermilps(xmm7, 0xd8);
        let xmm4_a = crate::simd_ops::vaddps(xmm3_d2, xmm0_d8);
        let xmm5_44 = crate::simd_ops::vshufps(xmm6, xmm4_a, 0x44);

        let xmm0_e1 = crate::simd_ops::vpermilps(xmm7, 0xe1);
        let xmm3_s = crate::simd_ops::vsubps(xmm0_e1, xmm1_c9);
        let xmm1_ee = crate::simd_ops::vshufps(xmm3_s, 0, 0xee);
        let xmm0_44 = crate::simd_ops::vshufps(xmm3_s, 0, 0x44);
        let xmm3_88 = crate::simd_ops::vshufps(xmm5_44, xmm0_44, 0x88);
        let xmm0_dd = crate::simd_ops::vshufps(xmm5_44, xmm0_44, 0xdd);
        let xmm5_d2 = crate::simd_ops::vpermilps(xmm0_dd, 0xd2);

        let xmm4_ee = crate::simd_ops::vshufps(xmm6, xmm4_a, 0xee);
        let xmm0_88 = crate::simd_ops::vshufps(xmm4_ee, xmm1_ee, 0x88);
        let xmm1_bl = crate::simd_ops::vblendps(xmm8_c, rd::<u128>(r15 + 0x10), 0x07);
        let xmm2_c9 = crate::simd_ops::vpermilps(xmm0_88, 0xc9);

        let xmm0_4e = crate::simd_ops::vshufps(xmm3_88, xmm5_d2, 0x4e);
        let row0_xmm7 = crate::simd_ops::vshufps(xmm3_88, xmm0_4e, 0x84);
        let xmm0_4e2 = crate::simd_ops::vshufps(xmm2_c9, xmm1_bl, 0x4e);
        let row2_xmm8 = crate::simd_ops::vshufps(xmm0_4e2, xmm1_bl, 0x98);
        let row1_xmm6 = crate::simd_ops::vshufps(xmm5_d2, xmm2_c9, 0x49);

        if was_active {
            let vfn_0 = rd::<usize>(rd::<usize>(rdi));
            exe::call_va!(vfn_0, (rdi, 0u32));
            wr::<u8>(rdi + 0x60, 0);
        }

        let byte_8 = rd::<u8>(r12 + 0x08) as u32;
        let vtbl_decal = exe::ptr::<u8>(0x1_433a_3388) as usize;
        let rdx_20 = rd::<usize>(param_1 + 0x20);
        wr::<usize>(rdi, vtbl_decal);
        wr::<[u128; 3]>(rdi + 0x10, [row0_xmm7, row1_xmm6, row2_xmm8]);
        let val_5c = rd::<u64>(rbp + 0x5c);
        wr::<u32>(rdi + 0x48, byte_8);
        wr::<u64>(rdi + 0x40, val_5c);
        wr::<u64>(rdi + 0x50, rd::<u64>(rbx_inner));
        wr::<u64>(rdi + 0x58, 0);
        wr::<u8>(rdi + 0x60, 1);

        let arg_rdx = rd::<u64>(rdx_20.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x10));
        exe::call_va!(0x1_4214_d710, (rdi, arg_rdx));
        let obj_50 = rd::<usize>(rdi + 0x50);
        let sub_obj = exe::call_va!(rd::<usize>(rd::<usize>(obj_50) + 0x48), (obj_50) -> usize);
        let val_58 = rd::<u64>(rdi + 0x58);
        let vfn_50 = rd::<usize>(rd::<usize>(sub_obj) + 0x50);
        exe::call_va!(vfn_50, (sub_obj, val_58));
    }
}

/// `unknown_141fefe90`
/// Original: 0x141fefe90 (527 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fefe90(param_1: usize) {
    let p0 = rd::<usize>(param_1);
    let r12 = rd::<usize>(param_1 + 0x08);
    let rax_idx = rd::<usize>(param_1 + 0x18);
    let rsi = p0.wrapping_add(rax_idx.wrapping_mul(0x10));
    let r15_off = rax_idx.wrapping_mul(0x18);
    let r14 = rd::<u32>(rsi + 0x08) as usize;
    let byte_size = r14.wrapping_mul(0x20);

    let mut tmp = vec![0u8; byte_size.max(0x20)];
    let tmp_ptr = tmp.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_4183_acbc, (tmp_ptr, rd::<usize>(rsi), byte_size) -> usize);
    wr::<u32>(rsi + 0x08, 0);

    if r14 != 0 {
        for i in 0..r14 {
            let ent = tmp_ptr.wrapping_add(i.wrapping_mul(0x20));
            let search_len = rd::<u32>(r12.wrapping_add(r15_off).wrapping_add(0x10));
            let key_id = rd::<u32>(ent);
            let search_ptr = rd::<u64>(r12.wrapping_add(r15_off).wrapping_add(0x08));
            let found = exe::call_va!(0x1_4248_9d30, (search_ptr, search_len, key_id) -> u32);
            if found != 0xffff_ffff && (found as u16) != 0xffff {
                let idx = (found as u16) as usize;
                let r8_base = rd::<usize>(r12.wrapping_add(r15_off));
                let slot = r8_base.wrapping_add(idx.wrapping_mul(0x10));
                let ebx = rd::<u32>(slot);
                if ebx == rd::<u32>(ent + 0x04) {
                    let dst_ptr = rd::<usize>(slot + 0x08);
                    match ebx {
                        2 | 4 => wr::<u32>(dst_ptr, rd::<u32>(ent + 0x10)),
                        3 => wr::<u8>(dst_ptr, if rd::<u32>(ent + 0x10) != 0 { 1 } else { 0 }),
                        5 | 6 => wr::<u128>(dst_ptr, rd::<u128>(ent + 0x10)),
                        _ => {}
                    }
                    if (rd::<u8>(ent + 0x08) & 1) != 0 {
                        let cur_len = rd::<u32>(rsi + 0x08) as u64;
                        let cur_cap = rd::<u32>(rsi + 0x0c) as u64;
                        if (cur_len << 5) == (cur_cap << 5) {
                            exe::call_va!(0x1_4069_6a30, (rsi, cur_len.wrapping_add(1)));
                        }
                        let off = (rd::<u32>(rsi + 0x08) as usize).wrapping_mul(0x20);
                        let new_ent = rd::<usize>(rsi).wrapping_add(off);
                        wr::<u64>(new_ent + 0x0c, 0);
                        wr::<u64>(new_ent + 0x14, 0);
                        wr::<u32>(new_ent + 0x1c, 0);
                        wr::<u64>(new_ent, 0);
                        wr::<u32>(new_ent + 0x08, 0);

                        let next_len = rd::<u32>(rsi + 0x08).wrapping_add(1);
                        wr::<u32>(rsi + 0x08, next_len);
                        let written_ent = rd::<usize>(rsi)
                            .wrapping_add((next_len as usize).wrapping_mul(0x20))
                            .wrapping_sub(0x20);
                        wr::<u32>(written_ent, rd::<u32>(ent));
                        wr::<u32>(written_ent + 0x04, ebx);
                        match ebx {
                            2 | 3 | 4 => wr::<u32>(written_ent + 0x10, 0),
                            5 => wr::<u128>(written_ent + 0x10, 0),
                            6 => {
                                let c = rd::<u128>(exe::ptr::<u128>(0x1_4342_81d0) as usize);
                                wr::<u128>(written_ent + 0x10, c);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

/// `unknown_142023800`
/// Original: 0x142023800 (633 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142023800(param_1: usize, param_2: usize) {
    let mut file_obj = [0u64; 0x130 / 8];
    let file_obj_ptr = file_obj.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_4244_36a0,
        (
            file_obj_ptr,
            rd::<u64>(param_2),
            rd::<u64>(param_2 + 0x08),
            0u32
        )
    );

    let vtbl_file_stream = exe::ptr::<u8>(0x1_4340_e9e0) as usize;
    let mut file_stream: [usize; 2] = [vtbl_file_stream, file_obj_ptr];
    let file_stream_ptr = file_stream.as_mut_ptr() as usize;

    let b3 = rd::<u8>(file_obj_ptr + 0x0f) as u32;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(file_obj_ptr + 0x0c) & 0x00ff_ffff) << 3;
    let str_ptr = if len_bits > cap_bits {
        rd::<usize>(file_obj_ptr + 0x14)
    } else {
        file_obj_ptr.wrapping_add(0x14)
    };

    let mut stream_ctx = [0u64; 0x370 / 8];
    let base = stream_ctx.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_4183_acc2, (base, 0u32, 0x310u64) -> usize);
    wr::<usize>(base, base + 0x10);
    wr::<u32>(base + 0x0c, 0x10);
    wr::<u32>(base + 0x318, 0);
    wr::<usize>(base + 0x320, file_stream_ptr);
    wr::<u32>(base + 0x328, 0xffff_ffff);
    wr::<usize>(base + 0x330, str_ptr);
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

    wr::<usize>(base + 0x338, file_stream_ptr);
    let inner_stream = rd::<usize>(file_stream_ptr + 0x08);
    wr::<usize>(base + 0x340, inner_stream);
    let vtbl_buf_stream = exe::ptr::<u8>(0x1_4307_b558) as usize;
    wr::<usize>(base + 0x348, vtbl_buf_stream);
    wr::<u64>(base + 0x350, 0);
    wr::<usize>(base + 0x358, vtbl_file_stream);
    wr::<usize>(base + 0x360, inner_stream);
    wr::<u16>(base + 0x368, 0x0100);

    exe::call_va!(0x1_411a_f6e0, (param_1 + 0xb8, base));

    let mut sum: u64 = 0;
    let mut cur = rd::<usize>(param_1 + 0xc8);
    let end = cur.wrapping_add((rd::<u32>(param_1 + 0xd0) as usize).wrapping_mul(8));
    while cur != end {
        sum = sum.wrapping_add(rd::<u64>(cur));
        cur = cur.wrapping_add(8);
    }
    if sum > rd::<u32>(param_1 + 0xe4) as u64 {
        exe::call_va!(0x1_402d_7910, (param_1 + 0xd8));
    }

    let mut count_i32: i32 = 0;
    let vtbl_fs = rd::<usize>(file_stream_ptr);
    exe::call_va!(
        rd::<usize>(vtbl_fs + 0x48),
        (file_stream_ptr, (&mut count_i32 as *mut i32) as usize)
    );
    exe::call_va!(0x1_4028_e490, (param_1 + 0xd8, count_i32 as i64));

    let mut edi: i32 = 0;
    while edi < count_i32 {
        let vtbl_fs2 = rd::<usize>(file_stream_ptr);
        let dst = rd::<usize>(param_1 + 0xd8).wrapping_add(edi as isize as usize);
        exe::call_va!(rd::<usize>(vtbl_fs2 + 0x78), (file_stream_ptr, dst));
        edi = edi.wrapping_add(1);
    }

    let _ = crate::simd_ops::atomic_xchg_u32((param_1 + 0x80) as u64, 4);
    wr::<usize>(base + 0x348, vtbl_buf_stream);
    exe::call_va!(0x1_401e_e280, (base));
    exe::call_va!(0x1_4240_ab10, (file_obj_ptr + 0x0c));
}

/// `r_1420abf40`
/// Original: 0x1420abf40 (630 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420abf40(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u8,
) {
    let r15_off = (param_4 as usize).wrapping_mul(0x28);
    let rbp = rd::<usize>(param_2);
    let mut r14 = rd::<usize>(rbp.wrapping_add(r15_off).wrapping_add(0x18));
    let mut rdi: usize = 0;
    let mut local_60: usize = 0;
    let count0 = rd::<u32>(rbp.wrapping_add(r15_off).wrapping_add(0x20)) as usize;
    let end0 = r14.wrapping_add(count0.wrapping_mul(0x40));

    while r14 != end0 {
        local_60 = r14;
        rdi = rd::<usize>(r14);
        let sub_cnt = rd::<u32>(r14 + 0x08) as usize;
        if rdi != rdi.wrapping_add(sub_cnt.wrapping_mul(2)) {
            break;
        }
        r14 = r14.wrapping_add(0x40);
    }

    let cur_end0 = rd::<usize>(rbp.wrapping_add(r15_off).wrapping_add(0x18)).wrapping_add(
        (rd::<u32>(rbp.wrapping_add(r15_off).wrapping_add(0x20)) as usize).wrapping_mul(0x40),
    );
    if r14 == cur_end0 {
        return;
    }

    loop {
        let idx_u16 = rd::<u16>(rdi) as usize;
        let p3_08 = rd::<usize>(param_3 + 0x08);
        let rec = p3_08.wrapping_add(idx_u16.wrapping_mul(0x130));
        let mut rbx = rd::<usize>(rec + 0x100);
        let rsi = rbx.wrapping_add((rd::<u32>(rec + 0x108) as usize).wrapping_mul(0x10));

        while rbx != rsi {
            let r8w = rd::<u16>(rbx + 0x08) as usize;
            let r12 = rd::<usize>(param_1 + 0x5e308).wrapping_add(r8w.wrapping_mul(0x10));
            let bit_arr = rd::<usize>(param_1 + 0x5e2f0);
            let word_idx = r8w >> 6;
            let bit_idx = (r8w & 0x3f) as u32;
            let mask_word = rd::<u64>(bit_arr.wrapping_add(word_idx * 8));

            if ((mask_word >> bit_idx) & 1) != 0 {
                wr::<u64>(r12 + 0x08, rd::<u64>(r12 + 0x08).wrapping_add(1));
            } else {
                if rd::<u64>(r12) == 0 {
                    let mut local_68: usize = 0;
                    let fn_ptr = rd::<usize>(rbx);
                    let ret_ptr =
                        exe::call_va!(fn_ptr, ((&mut local_68 as *mut usize) as usize) -> usize);
                    let new_obj = rd::<usize>(ret_ptr);
                    wr::<usize>(ret_ptr, 0);
                    let old_obj = rd::<usize>(r12);
                    wr::<usize>(r12, new_obj);
                    if old_obj != 0 {
                        let vfn0 = rd::<usize>(rd::<usize>(old_obj));
                        exe::call_va!(vfn0, (old_obj, 0u32));
                        exe::call_va!(0x1_4244_2de0, (old_obj));
                    }
                    if local_68 != 0 {
                        let vfn0 = rd::<usize>(rd::<usize>(local_68));
                        exe::call_va!(vfn0, (local_68, 0u32));
                        exe::call_va!(0x1_4244_2de0, (local_68));
                    }
                }
                wr::<u64>(r12 + 0x08, 1);
                let r8w2 = rd::<u16>(rbx + 0x08) as usize;
                let bit_arr2 = rd::<usize>(param_1 + 0x5e2f0);
                let slot_addr = bit_arr2.wrapping_add((r8w2 >> 6) * 8);
                let bit = 1u64 << ((r8w2 & 0x3f) as u32);
                wr::<u64>(slot_addr, (rd::<u64>(slot_addr) & !bit) | bit);
            }
            rbx = rbx.wrapping_add(0x10);
        }

        rdi = rdi.wrapping_add(2);
        let sub_end =
            rd::<usize>(local_60).wrapping_add((rd::<u32>(local_60 + 0x08) as usize).wrapping_mul(2));
        if rdi == sub_end {
            loop {
                r14 = r14.wrapping_add(0x40);
                let end_r14 = rd::<usize>(rbp.wrapping_add(r15_off).wrapping_add(0x18))
                    .wrapping_add(
                        (rd::<u32>(rbp.wrapping_add(r15_off).wrapping_add(0x20)) as usize)
                            .wrapping_mul(0x40),
                    );
                if r14 == end_r14 {
                    break;
                }
                local_60 = r14;
                rdi = rd::<usize>(r14);
                let next_sub_end =
                    rdi.wrapping_add((rd::<u32>(r14 + 0x08) as usize).wrapping_mul(2));
                if rdi != next_sub_end {
                    break;
                }
            }
        }

        let cur_end = rd::<usize>(rbp.wrapping_add(r15_off).wrapping_add(0x18)).wrapping_add(
            (rd::<u32>(rbp.wrapping_add(r15_off).wrapping_add(0x20)) as usize).wrapping_mul(0x40),
        );
        if r14 == cur_end {
            break;
        }
    }
}

/// `r_1420b01c0`
/// Original: 0x1420b01c0 (753 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1420b01c0(
    param_1: usize,
    param_2: usize,
    param_3: u32,
    param_4: i64,
    param_5: usize,
    param_6: usize,
) {
    let seg_lo = rd::<i64>(param_1);
    let seg_hi = rd::<i64>(param_1 + 0x08);
    let seg_depth = rd::<u64>(param_1 + 0x10);

    if (seg_depth as u32) == param_3 || seg_hi.wrapping_sub(seg_lo) <= param_4 {
        let sub_base = param_2.wrapping_add((seg_lo as usize).wrapping_mul(8));
        let sub_len = (seg_hi as u32).wrapping_sub(seg_lo as u32);
        exe::call_va!(0x1_40fa_e550, (sub_base, sub_len));
        return;
    }

    let rbx_last = seg_hi.wrapping_sub(1);
    exe::call_va!(param_6, (param_2, seg_lo, rbx_last));

    let r10_pivot_addr = param_2.wrapping_add((rbx_last as usize).wrapping_mul(8));
    let pivot = rd::<u64>(r10_pivot_addr);
    let mut rdx_i = seg_lo.wrapping_sub(1);

    if seg_lo < rbx_last {
        let mut cur_ptr = param_2.wrapping_add((seg_lo as usize).wrapping_mul(8));
        let mut rem = rbx_last.wrapping_sub(seg_lo) as u64;
        while rem != 0 {
            let val = rd::<u64>(cur_ptr);
            if val <= pivot {
                rdx_i = rdx_i.wrapping_add(1);
                let dst_i = param_2.wrapping_add((rdx_i as usize).wrapping_mul(8));
                let tmp = rd::<u64>(dst_i);
                wr::<u64>(dst_i, val);
                wr::<u64>(cur_ptr, tmp);
            }
            cur_ptr = cur_ptr.wrapping_add(8);
            rem -= 1;
        }
    }

    let r15_mid = rdx_i.wrapping_add(1);
    let mid_addr = param_2.wrapping_add((r15_mid as usize).wrapping_mul(8));
    let pivot_now = rd::<u64>(r10_pivot_addr);
    let mid_val = rd::<u64>(mid_addr);
    wr::<u64>(mid_addr, pivot_now);
    wr::<u64>(r10_pivot_addr, mid_val);

    if seg_lo < r15_mid {
        let g_2f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        let rbx_mgr = rd::<usize>(g_2f8 + 0x10);
        let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let pool = rd::<usize>(
            rbx_mgr
                .wrapping_add(worker_idx.wrapping_mul(0x40))
                .wrapping_add(0x48),
        )
        .wrapping_add(0x80);
        let mut job_ptr: usize = 0;
        exe::call_va!(0x1_4248_5aa0, (pool, (&mut job_ptr as *mut usize) as usize));
        let _ = crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
        wr::<u64>(job_ptr + 0x08, 0);
        wr::<i64>(job_ptr + 0x18, seg_lo);
        wr::<i64>(job_ptr + 0x20, r15_mid);
        wr::<u64>(job_ptr + 0x28, (seg_depth as u32).wrapping_add(1) as u64);
        wr::<usize>(job_ptr + 0x30, param_2);
        wr::<u64>(job_ptr + 0x38, param_3 as u64);
        wr::<i64>(job_ptr + 0x40, param_4);
        wr::<usize>(job_ptr + 0x48, param_5);
        wr::<usize>(job_ptr + 0x50, param_6);
        wr::<usize>(job_ptr + 0x10, exe::ptr::<u8>(0x1_433d_d718) as usize);

        let handle_target = rd::<usize>(param_5);
        let _ = crate::simd_ops::atomic_xadd_u32(handle_target as u64, 1);
        wr::<usize>(job_ptr + 0x08, handle_target);
        if crate::simd_ops::rd_acq_u32(job_ptr) == 1 {
            let mgr_10 = rd::<usize>(g_2f8 + 0x10);
            exe::call_va!(0x1_4248_6460, (mgr_10, job_ptr));
            let sem = rd::<u64>(rd::<usize>(mgr_10) + 0x1008);
            let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
        }
    }

    let right_lo = r15_mid.wrapping_add(1);
    if right_lo < seg_hi {
        let g_2f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        let rbx_mgr = rd::<usize>(g_2f8 + 0x10);
        let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let pool = rd::<usize>(
            rbx_mgr
                .wrapping_add(worker_idx.wrapping_mul(0x40))
                .wrapping_add(0x48),
        )
        .wrapping_add(0x80);
        let mut job_ptr: usize = 0;
        exe::call_va!(0x1_4248_5aa0, (pool, (&mut job_ptr as *mut usize) as usize));
        let _ = crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
        wr::<u64>(job_ptr + 0x08, 0);
        wr::<i64>(job_ptr + 0x18, right_lo);
        wr::<i64>(job_ptr + 0x20, seg_hi);
        wr::<u64>(job_ptr + 0x28, (seg_depth as u32).wrapping_add(1) as u64);
        wr::<usize>(job_ptr + 0x30, param_2);
        wr::<u64>(job_ptr + 0x38, param_3 as u64);
        wr::<i64>(job_ptr + 0x40, param_4);
        wr::<usize>(job_ptr + 0x48, param_5);
        wr::<usize>(job_ptr + 0x50, param_6);
        wr::<usize>(job_ptr + 0x10, exe::ptr::<u8>(0x1_433d_d6e8) as usize);

        let handle_target = rd::<usize>(param_5);
        let _ = crate::simd_ops::atomic_xadd_u32(handle_target as u64, 1);
        wr::<usize>(job_ptr + 0x08, handle_target);
        if crate::simd_ops::rd_acq_u32(job_ptr) == 1 {
            let mgr_10 = rd::<usize>(g_2f8 + 0x10);
            exe::call_va!(0x1_4248_6460, (mgr_10, job_ptr));
            let sem = rd::<u64>(rd::<usize>(mgr_10) + 0x1008);
            let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
        }
    }
}

/// `r_1420b5670`
/// Original: 0x1420b5670 (586 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b5670(param_1: usize) {
    let ctx = rd::<usize>(param_1 + 0x40);
    let mut rem = rd::<u64>(param_1 + 0x08);
    let mut node2 = rd::<usize>(param_1 + 0x30);
    let mut idx2 = rd::<usize>(param_1 + 0x38);

    let u58 = rd::<usize>(ctx + 0x58);
    let u50 = rd::<usize>(ctx + 0x50);
    let u48 = rd::<usize>(ctx + 0x48);
    let u38 = rd::<usize>(ctx + 0x38);
    let u28 = rd::<usize>(ctx + 0x28);
    let u00 = rd::<u64>(ctx);
    let u08 = rd::<u64>(ctx + 0x08);
    let u18 = rd::<usize>(ctx + 0x18);

    let mut node0 = rd::<usize>(param_1 + 0x10);
    let mut idx0 = rd::<usize>(param_1 + 0x18);
    let mut node1 = rd::<usize>(param_1 + 0x20);
    let mut idx1 = rd::<usize>(param_1 + 0x28);

    while rem != 0 {
        let cur_node0 = node0;
        let cur_idx0 = idx0;
        idx0 = idx0.wrapping_add(1);
        if idx0 == rd::<usize>(node0 + 0x08) {
            let next = rd::<usize>(node0);
            if next != 0 {
                node0 = next;
                idx0 = 0;
            }
        }

        let cur_node1 = node1;
        let cur_idx1 = idx1;
        idx1 = idx1.wrapping_add(1);
        let r9d = rd::<u32>(cur_node0.wrapping_add(cur_idx0.wrapping_mul(4)).wrapping_add(0x10));
        if idx1 == rd::<usize>(node1 + 0x08) {
            let next = rd::<usize>(node1);
            if next != 0 {
                node1 = next;
                idx1 = 0;
            }
        }

        let cur_node2 = node2;
        let cur_idx2 = idx2;
        idx2 = idx2.wrapping_add(1);
        let edi = rd::<u16>(cur_node1.wrapping_add(cur_idx1.wrapping_mul(2)).wrapping_add(0x10));
        if idx2 == rd::<usize>(node2 + 0x08) {
            let next = rd::<usize>(node2);
            if next != 0 {
                node2 = next;
                idx2 = 0;
            }
        }

        let target_obj = rd::<u64>(cur_node2.wrapping_add(cur_idx2.wrapping_mul(8)).wrapping_add(0x10));
        if target_obj != 0 {
            let found1 = exe::call_va!(0x1_411f_36a0, (u08, u00, r9d) -> i32);
            if found1 != -1 {
                let esi = rd::<u32>(u18.wrapping_add((found1 as isize as usize).wrapping_mul(4)))
                    as usize;
                let grp_id = rd::<u16>(u28.wrapping_add(esi * 8).wrapping_add(0x04)) as usize;
                let r11 = u48.wrapping_add(grp_id.wrapping_mul(0x30));
                let arch = rd::<u16>(r11) as usize;
                let arch_off = arch.wrapping_shl(5);
                let search_len = rd::<u32>(u50.wrapping_add(arch_off).wrapping_add(0x1806c));
                let search_ptr = rd::<u64>(u50.wrapping_add(arch_off).wrapping_add(0x18058));
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, edi as u32) -> i32);
                let off_tbl = rd::<usize>(r11 + 0x18);
                let comp_off = (rd::<u32>(
                    off_tbl.wrapping_add((found2 as isize as usize).wrapping_mul(4)),
                ) as u64)
                    .wrapping_add(rd::<u64>(r11 + 0x08));
                let arg_r8d = rd::<u32>(u38.wrapping_add(esi * 4));
                let fn_entry = rd::<usize>(u58).wrapping_add((edi as usize).wrapping_mul(0x50));
                let fn_ptr = rd::<usize>(fn_entry + 0x30);
                exe::call_va!(fn_ptr, (target_obj, comp_off, arg_r8d));
            }
        }

        rem -= 1;
    }
}

/// `unknown_1420b8fe0`
/// Original: 0x1420b8fe0 (755 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420b8fe0(param_1: usize) {
    let count = rd::<u64>(param_1);
    for rax in 0..count {
        let r8_arr = rd::<usize>(param_1 + 0x08);
        let r10_val = rd::<u64>(r8_arr.wrapping_add((rax as usize) * 8));
        if (r10_val as u32) != 0xffff_ffff {
            let rcx_18 = rd::<usize>(param_1 + 0x18);
            let rbx_28 = rd::<usize>(param_1 + 0x28);
            let hi16 = (r10_val >> 48) as usize;
            let lo32 = (r10_val as u32) as usize;
            let sub_arr = rd::<usize>(rcx_18.wrapping_add(hi16 * 8));
            let r11d = rd::<u32>(sub_arr.wrapping_add(lo32 * 8).wrapping_add(0x10));
            let mut rem = rd::<u64>(param_1 + 0x20);
            let mut r10_idx: u64 = 0;
            while rem != 0 {
                let half = rem >> 1;
                let step = rem.wrapping_sub(half).wrapping_add(r10_idx);
                let mid = half.wrapping_add(r10_idx);
                if r11d >= rd::<u32>(rbx_28.wrapping_add((mid as usize) * 4)) {
                    r10_idx = step;
                }
                rem = half;
            }
            let out_38 = rd::<usize>(param_1 + 0x38);
            wr::<u32>(
                out_38
                    .wrapping_add((r10_idx as usize).wrapping_mul(4))
                    .wrapping_sub(4),
                rax as u32,
            );
        }
    }

    exe::call_va!(0x1_420b_bd40, (param_1 + 0x40, rd::<u64>(param_1 + 0xa8)));

    for (vec_off, pad_word) in [
        (0xc0usize, rd::<u64>(param_1 + 0xd0) & 0xffff_0000),
        (0xc8usize, (param_1 as u64) & 0xffff_0000),
    ] {
        let r14_d0 = rd::<usize>(param_1 + 0xd0);
        let r9_b8 = rd::<usize>(param_1 + 0xb8);
        let rcx_18 = rd::<usize>(param_1 + 0x18);
        let vec_ptr = rd::<usize>(param_1 + vec_off);
        let mut rbx = rd::<usize>(vec_ptr + 0x08);
        let rdi = rbx.wrapping_add((rd::<u32>(vec_ptr + 0x10) as usize).wrapping_mul(0x10));

        while rbx != rdi {
            let rbp = rd::<u64>(rbx + 0x08);
            let rsi_q = rd::<u64>(rbx);
            let r15w = (rbp >> 32) as u16;
            if r15w != 0xffff {
                let hi16 = (rbp >> 48) as usize;
                let r13 = rd::<usize>(rcx_18.wrapping_add(hi16 * 8)).wrapping_add(0x10);
                let mut esi = rsi_q as u32;
                let end_idx = (rsi_q >> 32) as u32;
                let n = end_idx.wrapping_sub(esi);
                let ebp_lo = rbp as u32;
                let src_ptr = r13.wrapping_add((esi as usize) * 8);
                let dst_block =
                    rd::<usize>(r9_b8.wrapping_add((r15w as usize).wrapping_mul(0x30)).wrapping_add(0x08));
                let dst_ptr = dst_block.wrapping_add(0x10).wrapping_add((ebp_lo as usize) * 8);
                let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, (n as usize) * 8) -> usize);
                if n != 0 {
                    let delta = ebp_lo.wrapping_sub(esi);
                    for _ in 0..n {
                        let eax = esi.wrapping_add(delta);
                        let rcx_val = pad_word | (r15w as u64) | ((eax as u64) << 32);
                        let edx = rd::<u32>(r13.wrapping_add((esi as usize) * 8)) as usize;
                        esi = esi.wrapping_add(1);
                        let out_arr = rd::<usize>(r14_d0);
                        wr::<u64>(out_arr.wrapping_add(edx * 8), rcx_val);
                    }
                }
            }
            rbx = rbx.wrapping_add(0x10);
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141f91130(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static STREAM_68: AtomicUsize = AtomicUsize::new(0);
    static STREAM_70: AtomicUsize = AtomicUsize::new(0);
    static STREAM_78: AtomicUsize = AtomicUsize::new(0);
    static REF_OBJ_PTR: AtomicUsize = AtomicUsize::new(0);
    static DTOR_LOG: AtomicU64 = AtomicU64::new(0);
    static A140_XMM3: AtomicU32 = AtomicU32::new(0);

    unsafe extern "win64" fn mock_b4d0(ctx: usize) {
        if rd::<u64>(ctx + 0x10) != rd::<u64>(ctx + 0x18) {
            wr::<usize>(ctx + 0x38, STREAM_68.load(Ordering::Relaxed));
            wr::<usize>(ctx + 0x40, STREAM_70.load(Ordering::Relaxed));
            wr::<usize>(ctx + 0x48, STREAM_78.load(Ordering::Relaxed));
        }
    }
    unsafe extern "win64" fn mock_a140(out_1a0: usize, mat_e0: usize, vec_20: usize) -> usize {
        let xmm = crate::simd_ops::take_xmm_args();
        let x3 = xmm[3] as u32;
        A140_XMM3.fetch_xor(x3, Ordering::Relaxed);
        let m0 = rd::<u64>(mat_e0);
        let m1 = rd::<u64>(mat_e0 + 0x10);
        let m2 = rd::<u64>(mat_e0 + 0x20);
        let v0 = rd::<u64>(vec_20);
        let v1 = rd::<u32>(vec_20 + 0x08) as u64;
        wr::<u64>(out_1a0, m0 ^ m1 ^ m2);
        wr::<u64>(out_1a0 + 0x08, v0.wrapping_add(v1).wrapping_add(x3 as u64));
        out_1a0
    }
    unsafe extern "win64" fn mock_a3b0(out_1e0: usize, rbx_ent: usize, in_1a0: usize) -> usize {
        wr::<u64>(out_1e0, rd::<u64>(in_1a0));
        wr::<u64>(out_1e0 + 0x08, rd::<u64>(in_1a0 + 0x08));
        wr::<u64>(out_1e0 + 0x10, rd::<u64>(rbx_ent + 0x70));
        let ref_obj = REF_OBJ_PTR.load(Ordering::Relaxed);
        wr::<usize>(out_1e0 + 0x48, ref_obj);
        out_1e0
    }
    unsafe extern "win64" fn mock_e410(dst: usize, src_1e0: usize) -> usize {
        wr::<u64>(dst, rd::<u64>(src_1e0));
        wr::<u64>(dst + 0x08, rd::<u64>(src_1e0 + 0x08));
        wr::<u64>(dst + 0x10, rd::<u64>(src_1e0 + 0x10));
        dst
    }
    unsafe extern "win64" fn mock_d6f0(mgr: usize, obj: usize) {
        DTOR_LOG.fetch_add(
            (mgr as u64).wrapping_add(rd::<u32>(obj + 0x24) as u64),
            Ordering::Relaxed,
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4110_b4d0, mock_b4d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4204_a140, mock_a140 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4204_a3b0, mock_a3b0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4110_e410, mock_e410 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let s_0d8 = rd::<u16>(g_0d8);
    let s_f58 = rd::<usize>(g_f58);
    wr::<u16>(g_0d8, 0);

    let run = |f: F| -> ([u64; 56], usize, u32, u64, u32, u32) {
        DTOR_LOG.store(0, Ordering::Relaxed);
        A140_XMM3.store(0, Ordering::Relaxed);

        let mut ring_mem = [0u64; 56];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring = [0u8; 0x40];
        let rp = ring.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x08, rmp);
        wr::<usize>(rp + 0x10, rmp + 56 * 8);
        wr::<u64>(rp + 0x18, 0);
        wr::<usize>(rp + 0x20, rmp);
        wr::<usize>(rp + 0x28, rmp);
        wr::<u32>(rp + 0x30, 3);
        wr::<u32>(rp + 0x34, 3);

        let mut mgr_f58 = [0u8; 0x1b0];
        let mp = mgr_f58.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x168, rp);
        wr::<u8>(mp + 0x198, 0);
        wr::<usize>(g_f58, mp);

        let mut ref_obj = [0u8; 0x40];
        let rop = ref_obj.as_mut_ptr() as usize;
        wr::<u32>(rop + 0x24, 2); // 2 entities will decrement 2 -> 1 -> 0
        wr::<usize>(rop + 0x28, 0x1234_0000);
        REF_OBJ_PTR.store(rop, Ordering::Relaxed);

        // 2 entities in stream_68 (32 bytes each: quat + pos) and stream_70
        let mut s68 = [0.0f32; 16];
        s68[3] = 1.0;
        s68[4] = 2.0;
        s68[5] = 4.0;
        s68[6] = 6.0;
        s68[11] = 1.0;
        s68[12] = 1.0;
        s68[13] = 2.0;
        s68[14] = 3.0;

        let mut s70 = [0.0f32; 16];
        s70[3] = 1.0;
        s70[4] = 4.0;
        s70[5] = 8.0;
        s70[6] = 12.0;
        s70[11] = 1.0;
        s70[12] = 3.0;
        s70[13] = 6.0;
        s70[14] = 9.0;

        let mut s78 = [0u8; 0x100];
        let p78 = s78.as_mut_ptr() as usize;
        // Entity 0: mode (+0x68) == 1, active (+0x78) == 1
        wr::<f32>(p78 + 0x1c, 2.0);
        wr::<f32>(p78 + 0x20, 6.0);
        wr::<f32>(p78 + 0x24, 4.0);
        wr::<f32>(p78 + 0x28, 8.0);
        wr::<f32>(p78 + 0x2c, 1.5);
        wr::<f32>(p78 + 0x30, 2.5);
        wr::<u32>(p78 + 0x68, 1);
        wr::<u64>(p78 + 0x70, 0xaaaa_1111);
        wr::<u8>(p78 + 0x78, 1);

        // Entity 1 (+0x80): mode (+0x68) == 0, active (+0x78) == 1
        let e1 = p78 + 0x80;
        wr::<f32>(e1 + 0x1c, 10.0);
        wr::<f32>(e1 + 0x20, 20.0);
        wr::<f32>(e1 + 0x24, 30.0);
        wr::<f32>(e1 + 0x2c, 3.25);
        wr::<u32>(e1 + 0x68, 0);
        wr::<u64>(e1 + 0x70, 0xbbbb_2222);
        wr::<u8>(e1 + 0x78, 1);

        STREAM_68.store(s68.as_ptr() as usize, Ordering::Relaxed);
        STREAM_70.store(s70.as_ptr() as usize, Ordering::Relaxed);
        STREAM_78.store(p78, Ordering::Relaxed);

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x50_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);

        let arch_list: [u16; 2] = [0, 1];
        let mut p1 = [0u64; 3];
        p1[0] = wp as u64;
        p1[1] = 2;
        p1[2] = arch_list.as_ptr() as u64;

        let blend_t: f32 = 0.5;
        let tbl_580e0: [usize; 1] = [(&blend_t as *const f32) as usize];
        let mut p2 = vec![0u8; 0x58100];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x580e0, tbl_580e0.as_ptr() as usize);

        f(p1.as_ptr() as usize, p2p);

        (
            ring_mem,
            rd::<usize>(rp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rp + 0x30),
            DTOR_LOG.load(Ordering::Relaxed),
            A140_XMM3.load(Ordering::Relaxed),
            rd::<u32>(rop + 0x24),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_0d8, s_0d8);
    wr::<usize>(g_f58, s_f58);

    crate::diff_eq!(o, n, "141f91130 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f91bd0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static CALL_LOG: AtomicU64 = AtomicU64::new(0);
    static SUB_OBJ_ADDR: AtomicU64 = AtomicU64::new(0);
    static RES_7DF0_ADDR: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_get_sub(this: usize) -> usize {
        CALL_LOG.fetch_add(rd::<u64>(this + 0x08), Ordering::Relaxed);
        SUB_OBJ_ADDR.load(Ordering::Relaxed) as usize
    }
    unsafe extern "win64" fn mock_sub_50(this: usize, handle: u64) {
        CALL_LOG.fetch_add(
            rd::<u64>(this + 0x08).wrapping_add(handle).wrapping_add(0x50),
            Ordering::Relaxed,
        );
    }
    unsafe extern "win64" fn mock_sub_68(this: usize, handle: u64, flag: u8) {
        CALL_LOG.fetch_add(
            rd::<u64>(this + 0x08)
                .wrapping_add(handle)
                .wrapping_add(flag as u64)
                .wrapping_add(0x68),
            Ordering::Relaxed,
        );
    }
    unsafe extern "win64" fn mock_dtor(this: usize, mode: u32) {
        CALL_LOG.fetch_add(
            rd::<u64>(this + 0x40).wrapping_add(mode as u64).wrapping_add(0xd0),
            Ordering::Relaxed,
        );
    }
    unsafe extern "win64" fn mock_d650(rdi: usize) {
        CALL_LOG.fetch_add(rd::<u64>(rdi + 0x58).wrapping_add(0xd650), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_7df0(rcx: usize) -> usize {
        CALL_LOG.fetch_add(rd::<u64>(rcx), Ordering::Relaxed);
        RES_7DF0_ADDR.load(Ordering::Relaxed) as usize
    }
    unsafe extern "win64" fn mock_d710(rdi: usize, arg_rdx: u64) {
        wr::<u64>(rdi + 0x58, arg_rdx.wrapping_add(0x777));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4214_d650, mock_d650 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4021_7df0, mock_7df0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4214_d710, mock_d710 as *const ())?;

    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let s_63c = rd::<u16>(g_63c);
    wr::<u16>(g_63c, 0);

    let run = |f: F, init_60: u8, state_val: u32| -> ([u8; 0x70], u64) {
        CALL_LOG.store(0, Ordering::Relaxed);

        let mut sub_vtbl = [0usize; 14];
        sub_vtbl[0x50 / 8] = mock_sub_50 as *const () as usize;
        sub_vtbl[0x68 / 8] = mock_sub_68 as *const () as usize;
        let sub_obj: [u64; 2] = [sub_vtbl.as_ptr() as u64, 0x2000];
        SUB_OBJ_ADDR.store(sub_obj.as_ptr() as u64, Ordering::Relaxed);

        let mut obj50_vtbl = [0usize; 10];
        obj50_vtbl[0x48 / 8] = mock_get_sub as *const () as usize;
        let obj50: [u64; 2] = [obj50_vtbl.as_ptr() as u64, 0x1000];
        let obj50_ptr = obj50.as_ptr() as usize;

        let res_7df0: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0];
        RES_7DF0_ADDR.store(res_7df0.as_ptr() as u64, Ordering::Relaxed);

        let dtor_vtbl: [usize; 1] = [mock_dtor as *const () as usize];

        let mut rdi_buf = [0u8; 0x70];
        let rdi_ptr = rdi_buf.as_mut_ptr() as usize;
        wr::<usize>(rdi_ptr, dtor_vtbl.as_ptr() as usize);
        wr::<u64>(rdi_ptr + 0x40, 0x3333);
        wr::<usize>(rdi_ptr + 0x50, obj50_ptr);
        wr::<u64>(rdi_ptr + 0x58, 0x4444);
        wr::<u8>(rdi_ptr + 0x60, init_60);

        let r15_transform: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 5.0, 6.0, 7.0, 1.0];
        let mut rbp_buf = [0u8; 0xc0];
        wr::<u64>(rbp_buf.as_mut_ptr() as usize + 0x5c, 0x8888_9999);
        wr::<u64>(rbp_buf.as_mut_ptr() as usize + 0x98, 0x1234);

        let mut rdx20_buf = [0u64; 4];
        rdx20_buf[2] = 0xabcd_0000;

        let states: [u32; 1] = [state_val];
        let p1: [usize; 6] = [
            states.as_ptr() as usize,
            r15_transform.as_ptr() as usize,
            rbp_buf.as_ptr() as usize,
            rdi_ptr,
            rdx20_buf.as_ptr() as usize,
            0,
        ];

        let ptr_to_obj50: [usize; 1] = [obj50_ptr];
        let ptr_to_ptr_obj50: [usize; 1] = [ptr_to_obj50.as_ptr() as usize];
        let tbl_580e0: [usize; 1] = [ptr_to_ptr_obj50.as_ptr() as usize];
        let mut p2 = vec![0u8; 0x58100];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x580e0, tbl_580e0.as_ptr() as usize);

        f(p1.as_ptr() as usize, p2p);

        wr::<usize>(rdi_ptr, 0);
        wr::<usize>(rdi_ptr + 0x50, 0);
        (rdi_buf, CALL_LOG.load(Ordering::Relaxed))
    };

    for (init_60, st) in [(0u8, 0u32), (0, 1), (1, 1), (1, 0)] {
        let o = run(f_o, init_60, st);
        let n = run(f_n, init_60, st);
        crate::diff_eq!(o, n, "141f91bd0 init_60={} st={}", init_60, st);
    }
    wr::<u16>(g_63c, s_63c);
    Ok(4)
}

pub unsafe fn diff_unknown_141fefe90(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU32, Ordering};
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static GROW_CALLS: AtomicU32 = AtomicU32::new(0);

    unsafe extern "win64" fn mock_9d30(_ptr: u64, _len: u32, key: u32) -> u32 {
        match key {
            10 => 0,
            20 => 1,
            30 => 2,
            40 => 3,
            50 => 4,
            _ => 0xffff_ffff,
        }
    }
    unsafe extern "win64" fn mock_6a30(vec_ptr: usize, new_cap: u64) {
        GROW_CALLS.fetch_add(1, Ordering::Relaxed);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32 + 8);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9d30, mock_9d30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4069_6a30, mock_6a30 as *const ())?;

    let run = |f: F| -> (u32, u8, u32, u128, u128, [u8; 0x20 * 5], u32, u32) {
        GROW_CALLS.store(0, Ordering::Relaxed);

        let mut val_t2: u32 = 0;
        let mut val_t3: u8 = 0;
        let mut val_t4: u32 = 0;
        let mut val_t5: u128 = 0;
        let mut val_t6: u128 = 0;

        let mut slots = [0u64; 10];
        let sp = slots.as_mut_ptr() as usize;
        wr::<u32>(sp, 2);
        wr::<usize>(sp + 0x08, (&mut val_t2 as *mut u32) as usize);
        wr::<u32>(sp + 0x10, 3);
        wr::<usize>(sp + 0x18, (&mut val_t3 as *mut u8) as usize);
        wr::<u32>(sp + 0x20, 4);
        wr::<usize>(sp + 0x28, (&mut val_t4 as *mut u32) as usize);
        wr::<u32>(sp + 0x30, 5);
        wr::<usize>(sp + 0x38, (&mut val_t5 as *mut u128) as usize);
        wr::<u32>(sp + 0x40, 6);
        wr::<usize>(sp + 0x48, (&mut val_t6 as *mut u128) as usize);

        let mut entries = [0u8; 0x20 * 5];
        let ep = entries.as_mut_ptr() as usize;
        for (i, (key, ty, keep)) in [
            (10u32, 2u32, 1u8),
            (20, 3, 1),
            (30, 4, 0),
            (40, 5, 1),
            (50, 6, 1),
        ]
        .iter()
        .enumerate()
        {
            let e = ep + i * 0x20;
            wr::<u32>(e, *key);
            wr::<u32>(e + 0x04, *ty);
            wr::<u8>(e + 0x08, *keep);
            wr::<u128>(e + 0x10, 0x1111_2222_3333_4444_5555_6666_7777_8888u128 + i as u128);
        }

        let mut vec_hdr = [0u8; 0x20];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp + 0x10, ep);
        wr::<u32>(vp + 0x18, 5);
        wr::<u32>(vp + 0x1c, 2); // triggers grow when len reaches 2

        let mut r12_tbl = [0u8; 0x30];
        let rp = r12_tbl.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x18, sp);
        wr::<u64>(rp + 0x20, 0x9999);
        wr::<u32>(rp + 0x28, 5);

        let p1: [usize; 4] = [vp, rp, 0, 1];
        f(p1.as_ptr() as usize);

        (
            val_t2,
            val_t3,
            val_t4,
            val_t5,
            val_t6,
            entries,
            rd::<u32>(vp + 0x18),
            GROW_CALLS.load(Ordering::Relaxed),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141fefe90 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142023800(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static LOG_HASH: AtomicU64 = AtomicU64::new(0);
    static INNER_VTBL_PTR: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_get_pos(this: usize) -> u64 {
        rd::<u64>(this + 0x20)
    }
    unsafe extern "win64" fn mock_36a0(out_obj: usize, p2_0: u64, p2_1: u64, flag: u32) {
        wr::<u64>(out_obj, INNER_VTBL_PTR.load(Ordering::Relaxed));
        wr::<u32>(out_obj + 0x0c, (4u32 << 24) | 3); // inline string
        wr::<u64>(out_obj + 0x14, 0x6162_6300);
        wr::<u64>(out_obj + 0x20, p2_0.wrapping_add(p2_1).wrapping_add(flag as u64));
    }
    unsafe extern "win64" fn mock_f6e0(rcx_b8: usize, stream_ctx: usize) {
        let pos = rd::<u64>(stream_ctx + 0x28);
        let mode = rd::<u16>(stream_ctx + 0x368) as u64;
        wr::<u64>(rcx_b8, pos.wrapping_add(mode));
    }
    unsafe extern "win64" fn mock_7910(rcx_d8: usize) {
        LOG_HASH.fetch_add(rd::<u64>(rcx_d8).wrapping_add(0x7910), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_fs_48(_fs: usize, out_cnt: usize) {
        wr::<i32>(out_cnt, 3);
    }
    unsafe extern "win64" fn mock_e490(_rcx_d8: usize, cnt: i64) {
        LOG_HASH.fetch_add((cnt as u64).wrapping_add(0xe490), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_fs_78(_fs: usize, dst: usize) {
        wr::<u8>(dst, 0x5a);
    }
    unsafe extern "win64" fn mock_e280(_stream_ctx: usize) {
        LOG_HASH.fetch_add(0xe280, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_ab10(str_hdr: usize) {
        LOG_HASH.fetch_add(rd::<u32>(str_hdr) as u64, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4244_36a0, mock_36a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411a_f6e0, mock_f6e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_402d_7910, mock_7910 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4249_5ac0, mock_fs_48 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4028_e490, mock_e490 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4249_5b80, mock_fs_78 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, cap_e4: u32| -> (u32, u64, [u8; 8], u64) {
        LOG_HASH.store(0, Ordering::Relaxed);
        let mut inner_vtbl = [0usize; 4];
        inner_vtbl[0x10 / 8] = mock_get_pos as *const () as usize;
        INNER_VTBL_PTR.store(inner_vtbl.as_ptr() as u64, Ordering::Relaxed);

        let arr_c8: [u64; 2] = [10, 20];
        let mut out_bytes = [0u8; 8];

        let mut p1 = [0u8; 0x100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xc8, arr_c8.as_ptr() as usize);
        wr::<u32>(p1p + 0xd0, 2);
        wr::<usize>(p1p + 0xd8, out_bytes.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0xe4, cap_e4);

        let p2: [u64; 2] = [0x100, 0x200];
        f(p1p, p2.as_ptr() as usize);

        (
            rd::<u32>(p1p + 0x80),
            rd::<u64>(p1p + 0xb8),
            out_bytes,
            LOG_HASH.load(Ordering::Relaxed),
        )
    };

    for cap in [15u32, 50u32] {
        let o = run(f_o, cap);
        let n = run(f_n, cap);
        crate::diff_eq!(o, n, "142023800 cap={}", cap);
    }
    Ok(2)
}

pub unsafe fn diff_r_1420abf40(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    type F = unsafe extern "win64" fn(usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static CREATED_OBJ_PTR: AtomicUsize = AtomicUsize::new(0);
    static TEMP_OBJ_PTR: AtomicUsize = AtomicUsize::new(0);
    static FREE_LOG: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_factory(out_ptr: usize) -> usize {
        wr::<usize>(out_ptr, CREATED_OBJ_PTR.load(Ordering::Relaxed));
        out_ptr
    }
    unsafe extern "win64" fn mock_obj_dtor(this: usize, mode: u32) {
        FREE_LOG.fetch_add(
            rd::<u64>(this + 0x08).wrapping_add(mode as u64),
            Ordering::Relaxed,
        );
    }
    unsafe extern "win64" fn mock_2de0(_ptr: usize) {
        FREE_LOG.fetch_add(0x2de0, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let run = |f: F| -> ([u64; 6], u64, u64) {
        FREE_LOG.store(0, Ordering::Relaxed);
        let vtbl: [usize; 1] = [mock_obj_dtor as *const () as usize];
        let created_obj: [u64; 2] = [vtbl.as_ptr() as u64, 0xaaaa];
        let temp_obj: [u64; 2] = [vtbl.as_ptr() as u64, 0xbbbb];
        CREATED_OBJ_PTR.store(created_obj.as_ptr() as usize, Ordering::Relaxed);
        TEMP_OBJ_PTR.store(temp_obj.as_ptr() as usize, Ordering::Relaxed);

        let mut bit_words: [u64; 1] = [0];
        let mut slots_5e308 = [0u64; 6]; // 3 entries of 16 bytes

        let mut p1 = vec![0u8; 0x5e320];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x5e2f0, bit_words.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x5e308, slots_5e308.as_mut_ptr() as usize);

        // Two items in rec[1]: both referencing slot 1 (first initializes it, second increments its count)
        let mut items = [0u8; 0x20];
        let ip = items.as_mut_ptr() as usize;
        wr::<usize>(ip, mock_factory as *const () as usize);
        wr::<u16>(ip + 0x08, 1);
        wr::<usize>(ip + 0x10, mock_factory as *const () as usize);
        wr::<u16>(ip + 0x18, 1);

        let mut recs = vec![0u8; 0x130 * 2];
        let rp = recs.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x130 + 0x100, ip);
        wr::<u32>(rp + 0x130 + 0x108, 2);

        let mut p3 = [0u64; 2];
        wr::<usize>(p3.as_mut_ptr() as usize + 0x08, rp);

        let sub_indices: [u16; 1] = [1];
        let mut spans = [0u8; 0x80]; // 2 spans of 0x40: first empty, second has 1 index
        let sp = spans.as_mut_ptr() as usize;
        wr::<usize>(sp, sub_indices.as_ptr() as usize);
        wr::<u32>(sp + 0x08, 0);
        wr::<usize>(sp + 0x40, sub_indices.as_ptr() as usize);
        wr::<u32>(sp + 0x48, 1);

        let mut rbp_buf = [0u8; 0x60];
        let rbp_p = rbp_buf.as_mut_ptr() as usize;
        wr::<usize>(rbp_p + 0x28 + 0x18, sp);
        wr::<u32>(rbp_p + 0x28 + 0x20, 2);
        let p2: [usize; 1] = [rbp_p];

        f(p1p, p2.as_ptr() as usize, p3.as_ptr() as usize, 1);

        slots_5e308[2] = (slots_5e308[2] != 0) as u64;
        (slots_5e308, bit_words[0], FREE_LOG.load(Ordering::Relaxed))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420abf40 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b01c0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    type F = unsafe extern "win64" fn(usize, usize, u32, i64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static BASE_SORT_LOG: AtomicU64 = AtomicU64::new(0);
    static JOB_IDX: AtomicUsize = AtomicUsize::new(0);
    static JOB_BUFS_BASE: AtomicUsize = AtomicUsize::new(0);
    static QUEUED_JOBS: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_e550(base: usize, count: u32) {
        let first = if count > 0 { rd::<u64>(base) } else { 0 };
        BASE_SORT_LOG.fetch_add(first.wrapping_add(count as u64), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_pivot(arr: usize, lo: i64, hi: i64) {
        // Swap middle element with hi so pivot is predictable
        let mid = (lo + hi) / 2;
        let a = arr.wrapping_add((mid as usize) * 8);
        let b = arr.wrapping_add((hi as usize) * 8);
        let va = rd::<u64>(a);
        let vb = rd::<u64>(b);
        wr::<u64>(a, vb);
        wr::<u64>(b, va);
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_job: usize) {
        let idx = JOB_IDX.fetch_add(1, Ordering::Relaxed);
        let base = JOB_BUFS_BASE.load(Ordering::Relaxed);
        wr::<usize>(out_job, base + idx * 0x60);
    }
    unsafe extern "win64" fn mock_6460(_mgr: usize, job_ptr: usize) {
        let lo = rd::<i64>(job_ptr + 0x18) as u64;
        let hi = rd::<i64>(job_ptr + 0x20) as u64;
        let depth = rd::<u32>(job_ptr + 0x28) as u64;
        let max_d = rd::<u32>(job_ptr + 0x38) as u64;
        let min_sz = rd::<i64>(job_ptr + 0x40) as u64;
        let vtbl = rd::<u64>(job_ptr + 0x10) & 0xffff;
        let h = lo
            .wrapping_mul(31)
            .wrapping_add(hi)
            .wrapping_mul(31)
            .wrapping_add(depth)
            .wrapping_mul(31)
            .wrapping_add(max_d)
            .wrapping_mul(31)
            .wrapping_add(min_sz)
            .wrapping_mul(31)
            .wrapping_add(vtbl);
        QUEUED_JOBS.fetch_add(h, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fa_e550, mock_e550 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;

    let g_2f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_2f8 = rd::<usize>(g_2f8);

    let run = |f: F, max_depth: u32| -> ([u64; 6], u32, u64, u64) {
        BASE_SORT_LOG.store(0, Ordering::Relaxed);
        JOB_IDX.store(0, Ordering::Relaxed);
        QUEUED_JOBS.store(0, Ordering::Relaxed);

        let mut job_bufs = [0u8; 0x60 * 4];
        JOB_BUFS_BASE.store(job_bufs.as_mut_ptr() as usize, Ordering::Relaxed);

        let mut mgr_inner = vec![0u8; 0x1020];
        let mut mgr_10 = [0u8; 0x80];
        let m10p = mgr_10.as_mut_ptr() as usize;
        wr::<usize>(m10p, mgr_inner.as_mut_ptr() as usize);
        wr::<usize>(m10p + 0x48, 0x5000);
        let g_obj: [usize; 3] = [0, 0, m10p];
        wr::<usize>(g_2f8, g_obj.as_ptr() as usize);

        let mut handle_cnt: u32 = 1;
        let handle_ptr: usize = (&mut handle_cnt as *mut u32) as usize;
        let param_5_ptr: usize = (&handle_ptr as *const usize) as usize;

        let mut arr: [u64; 6] = [50, 10, 40, 30, 60, 20];
        let seg: [u64; 3] = [0, 6, 1];

        f(
            seg.as_ptr() as usize,
            arr.as_mut_ptr() as usize,
            max_depth,
            1,
            param_5_ptr,
            mock_pivot as *const () as usize,
        );

        (
            arr,
            handle_cnt,
            BASE_SORT_LOG.load(Ordering::Relaxed),
            QUEUED_JOBS.load(Ordering::Relaxed),
        )
    };

    for max_d in [1u32, 5u32] {
        let o = run(f_o, max_d);
        let n = run(f_n, max_d);
        wr::<usize>(g_2f8, s_2f8);
        crate::diff_eq!(o, n, "1420b01c0 max_d={}", max_d);
    }
    Ok(2)
}

pub unsafe fn diff_r_1420b5670(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static CB_LOG: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_36a0(_u08: u64, _u00: u64, key: u32) -> i32 {
        match key {
            100 => 0,
            200 => 1,
            _ => -1,
        }
    }
    unsafe extern "win64" fn mock_9ee0(_ptr: u64, _len: u32, comp: u16) -> i32 {
        (comp & 1) as i32
    }
    unsafe extern "win64" fn mock_cb(target: u64, comp_ptr: u64, arg_r8d: u32) {
        let h = target
            .wrapping_mul(31)
            .wrapping_add(comp_ptr)
            .wrapping_mul(31)
            .wrapping_add(arg_r8d as u64);
        CB_LOG.fetch_add(h, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_36a0, mock_36a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;

    let run = |f: F| -> u64 {
        CB_LOG.store(0, Ordering::Relaxed);

        // Chained nodes for iterators 0, 1, 2
        let mut n0_b = [0u8; 0x20];
        wr::<usize>(n0_b.as_mut_ptr() as usize + 0x08, 2);
        wr::<u32>(n0_b.as_mut_ptr() as usize + 0x10, 200);
        let mut n0_a = [0u8; 0x20];
        wr::<usize>(n0_a.as_mut_ptr() as usize, n0_b.as_ptr() as usize);
        wr::<usize>(n0_a.as_mut_ptr() as usize + 0x08, 1);
        wr::<u32>(n0_a.as_mut_ptr() as usize + 0x10, 100);

        let mut n1_b = [0u8; 0x20];
        wr::<usize>(n1_b.as_mut_ptr() as usize + 0x08, 2);
        wr::<u16>(n1_b.as_mut_ptr() as usize + 0x10, 1);
        let mut n1_a = [0u8; 0x20];
        wr::<usize>(n1_a.as_mut_ptr() as usize, n1_b.as_ptr() as usize);
        wr::<usize>(n1_a.as_mut_ptr() as usize + 0x08, 1);
        wr::<u16>(n1_a.as_mut_ptr() as usize + 0x10, 0);

        let mut n2_b = [0u8; 0x20];
        wr::<usize>(n2_b.as_mut_ptr() as usize + 0x08, 2);
        wr::<u64>(n2_b.as_mut_ptr() as usize + 0x10, 0x8002);
        let mut n2_a = [0u8; 0x20];
        wr::<usize>(n2_a.as_mut_ptr() as usize, n2_b.as_ptr() as usize);
        wr::<usize>(n2_a.as_mut_ptr() as usize + 0x08, 1);
        wr::<u64>(n2_a.as_mut_ptr() as usize + 0x10, 0x8001);

        let u18_arr: [u32; 2] = [0, 1];
        let mut u28_arr = [0u8; 16];
        wr::<u16>(u28_arr.as_mut_ptr() as usize + 0x04, 0);
        wr::<u16>(u28_arr.as_mut_ptr() as usize + 0x0c, 1);
        let u38_arr: [u32; 2] = [0xaa, 0xbb];

        let off_tbl: [u32; 2] = [0x10, 0x20];
        let mut u48_arr = [0u8; 0x60];
        let u48p = u48_arr.as_mut_ptr() as usize;
        wr::<u16>(u48p, 0);
        wr::<u64>(u48p + 0x08, 0x1000);
        wr::<usize>(u48p + 0x18, off_tbl.as_ptr() as usize);
        wr::<u16>(u48p + 0x30, 1);
        wr::<u64>(u48p + 0x38, 0x2000);
        wr::<usize>(u48p + 0x48, off_tbl.as_ptr() as usize);

        let u50_buf = vec![0u8; 0x18100];
        let mut fn_entries = [0u8; 0xa0];
        let fep = fn_entries.as_mut_ptr() as usize;
        wr::<usize>(fep + 0x30, mock_cb as *const () as usize);
        wr::<usize>(fep + 0x50 + 0x30, mock_cb as *const () as usize);
        let u58_box: [usize; 1] = [fep];

        let mut ctx = [0u64; 12];
        ctx[0] = 0x1111;
        ctx[1] = 0x2222;
        ctx[3] = u18_arr.as_ptr() as u64;
        ctx[5] = u28_arr.as_ptr() as u64;
        ctx[7] = u38_arr.as_ptr() as u64;
        ctx[9] = u48p as u64;
        ctx[10] = u50_buf.as_ptr() as u64;
        ctx[11] = u58_box.as_ptr() as u64;

        let mut p1 = [0u64; 9];
        p1[1] = 2; // 2 iterations across node boundaries
        p1[2] = n0_a.as_ptr() as u64;
        p1[3] = 0;
        p1[4] = n1_a.as_ptr() as u64;
        p1[5] = 0;
        p1[6] = n2_a.as_ptr() as u64;
        p1[7] = 0;
        p1[8] = ctx.as_ptr() as u64;

        f(p1.as_ptr() as usize);
        CB_LOG.load(Ordering::Relaxed)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b5670 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420b8fe0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static BBD40_LOG: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_bbd40(rcx: usize, rdx: u64) {
        BBD40_LOG.fetch_add(rd::<u64>(rcx).wrapping_add(rdx), Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_420b_bd40, mock_bbd40 as *const ())?;

    let mut out_38_o = [0u32; 4];
    let mut out_38_n = [0u32; 4];
    let mut dst_b8_o = [0u64; 8];
    let mut dst_b8_n = [0u64; 8];
    let mut out_d0_o = [0u64; 8];
    let mut out_d0_n = [0u64; 8];

    let run = |f: F,
               out_38: &mut [u32; 4],
               dst_b8: &mut [u64; 8],
               out_d0: &mut [u64; 8]|
     -> u64 {
        BBD40_LOG.store(0, Ordering::Relaxed);

        let arr_08: [u64; 3] = [0xffff_ffff, 0, 1];
        let mut sub_arr0 = [0u8; 0x30];
        let sp0 = sub_arr0.as_mut_ptr() as usize;
        // Entries at sp0 + 0x10: [key0=15, extra0=0xaa], [key1=35, extra1=0xbb]
        wr::<u32>(sp0 + 0x10, 2); // index into out_d0
        wr::<u32>(sp0 + 0x14, 0xaa);
        wr::<u32>(sp0 + 0x18, 4); // index into out_d0
        wr::<u32>(sp0 + 0x1c, 0xbb);
        let ptrs_18: [usize; 1] = [sp0];

        let sorted_28: [u32; 3] = [0, 3, 10];

        let mut b8_hdr = [0u8; 0x30];
        wr::<usize>(
            b8_hdr.as_mut_ptr() as usize + 0x08,
            dst_b8.as_mut_ptr() as usize,
        );

        let c0_items: [u64; 2] = [1u64 << 32, 1]; // start=0, end=1, dst_off=1, r15w=0
        let mut vec_c0 = [0u8; 0x18];
        wr::<usize>(vec_c0.as_mut_ptr() as usize + 0x08, c0_items.as_ptr() as usize);
        wr::<u32>(vec_c0.as_mut_ptr() as usize + 0x10, 1);

        let c8_items: [u64; 2] = [(2u64 << 32) | 1, 3]; // start=1, end=2, dst_off=3, r15w=0
        let mut vec_c8 = [0u8; 0x18];
        wr::<usize>(vec_c8.as_mut_ptr() as usize + 0x08, c8_items.as_ptr() as usize);
        wr::<u32>(vec_c8.as_mut_ptr() as usize + 0x10, 1);

        let d0_box: [usize; 1] = [out_d0.as_mut_ptr() as usize];

        let mut p1 = [0u8; 0xe0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p, 3);
        wr::<usize>(p1p + 0x08, arr_08.as_ptr() as usize);
        wr::<usize>(p1p + 0x18, ptrs_18.as_ptr() as usize);
        wr::<u64>(p1p + 0x20, 3);
        wr::<usize>(p1p + 0x28, sorted_28.as_ptr() as usize);
        wr::<usize>(p1p + 0x38, out_38.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x40, 0x1234);
        wr::<u64>(p1p + 0xa8, 0x5678);
        wr::<usize>(p1p + 0xb8, b8_hdr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xc0, vec_c0.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xc8, vec_c8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xd0, d0_box.as_ptr() as usize);

        f(p1p);
        BBD40_LOG.load(Ordering::Relaxed)
    };

    let log_o = run(f_o, &mut out_38_o, &mut dst_b8_o, &mut out_d0_o);
    let log_n = run(f_n, &mut out_38_n, &mut dst_b8_n, &mut out_d0_n);
    // Clear the 16-bit uninitialized stack padding hole in out_d0 before comparing
    for v in out_d0_o.iter_mut() {
        *v &= !0xffff_0000u64;
    }
    for v in out_d0_n.iter_mut() {
        *v &= !0xffff_0000u64;
    }
    crate::diff_eq!(
        (log_o, out_38_o, dst_b8_o, out_d0_o),
        (log_n, out_38_n, dst_b8_n, out_d0_n),
        "1420b8fe0 out"
    );
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_41f9_1130, "unknown_141f91130", rend::cauldron_hot_v4::unknown_141f91130 as *const (), rend::cauldron_hot_v4::diff_unknown_141f91130),
// Replacement::verified(0x1_41f9_1bd0, "unknown_141f91bd0", rend::cauldron_hot_v4::unknown_141f91bd0 as *const (), rend::cauldron_hot_v4::diff_unknown_141f91bd0),
// Replacement::verified(0x1_41fe_fe90, "unknown_141fefe90", rend::cauldron_hot_v4::unknown_141fefe90 as *const (), rend::cauldron_hot_v4::diff_unknown_141fefe90),
// Replacement::verified_throws(0x1_4202_3800, "unknown_142023800", rend::cauldron_hot_v4::unknown_142023800 as *const (), Throws::Caught, rend::cauldron_hot_v4::diff_unknown_142023800),
// Replacement::verified(0x1_420a_bf40, "r_1420abf40", rend::cauldron_hot_v4::r_1420abf40 as *const (), rend::cauldron_hot_v4::diff_r_1420abf40),
// Replacement::verified_throws(0x1_420b_01c0, "r_1420b01c0", rend::cauldron_hot_v4::r_1420b01c0 as *const (), Throws::Discard, rend::cauldron_hot_v4::diff_r_1420b01c0),
// Replacement::verified(0x1_420b_5670, "r_1420b5670", rend::cauldron_hot_v4::r_1420b5670 as *const (), rend::cauldron_hot_v4::diff_r_1420b5670),
// Replacement::verified(0x1_420b_8fe0, "unknown_1420b8fe0", rend::cauldron_hot_v4::unknown_1420b8fe0 as *const (), rend::cauldron_hot_v4::diff_unknown_1420b8fe0),
