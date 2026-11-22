//! Decompiled hot runtime functions (`cauldron_hot_k`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vblendps, vbroadcastss, vcmpps, vdpps, vfmadd_ps, vmovmskps, vmulps, vpcmpgtb,
    vpermilps, vpmovmskb, vshufps, vsubps,
};

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

#[inline(always)]
fn canonical_f32_bits(v: f32) -> u32 {
    if v == 0.0f32 {
        0
    } else {
        v.to_bits()
    }
}

#[inline(always)]
fn fnv1a_u32(v: u32) -> u64 {
    let b = v.to_le_bytes();
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let mut i = 0;
    while i < 4 {
        h ^= b[i] as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
        i += 1;
    }
    h
}

#[inline(always)]
fn hash_combine(seed: u64, val_hash: u64) -> u64 {
    let mixed = val_hash
        .wrapping_add(seed << 6)
        .wrapping_add(seed >> 2)
        .wrapping_add(0x9e37_79b9);
    seed ^ mixed
}

#[inline(always)]
unsafe fn release_ref_24(obj: usize) {
    if obj != 0 {
        let old = crate::simd_ops::atomic_xadd_u32((obj + 0x24) as u64, u32::MAX);
        if old == 1 {
            let mgr = rd::<usize>(obj + 0x28);
            if mgr != 0 {
                exe::call_va!(0x1_4241_d6f0, (mgr, obj));
            }
        }
    }
}

#[inline(always)]
unsafe fn retain_ref_24(obj: usize) {
    if obj != 0 {
        let old = crate::simd_ops::atomic_xadd_u32((obj + 0x24) as u64, 1);
        if old == 0 {
            let mgr = rd::<usize>(obj + 0x28);
            if mgr != 0 {
                exe::call_va!(0x1_4241_d6f0, (mgr, obj));
            }
        }
    }
}

/// `unknown_141f27160`
/// Original: `0x141f27160` (4644 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_141f27160(param_1: usize, param_2: usize) {
    let mut scratch_vec = [0u64; 2];
    let scratch_vec_ptr = scratch_vec.as_mut_ptr() as usize;

    let span1 = rd::<[u64; 2]>(param_1 + 0x08);
    let rbx_world = rd::<usize>(param_1);
    let rsi_base = rbx_world.wrapping_add(0x40);

    let mut iter_init1 = [0u64; 3];
    let iter1_ptr = exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init1.as_mut_ptr() as usize,
            rsi_base,
            span1.as_ptr() as usize
        ) -> usize
    );
    let iter1_base = rd::<usize>(iter1_ptr);
    let mut cur_arch1 = rd::<usize>(iter1_ptr + 0x08);
    let end_arch1 = rd::<usize>(iter1_ptr + 0x10);

    let mut total_count: u64 = 0;
    if cur_arch1 != end_arch1 {
        loop {
            let arch_id = rd::<u16>(cur_arch1) as usize;
            let count = if rd::<u64>(rbx_world.wrapping_add(arch_id * 8).wrapping_add(0x88)) != 0 {
                rd::<u32>(rbx_world.wrapping_add(arch_id * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            total_count = total_count.wrapping_add(count);
            cur_arch1 = cur_arch1.wrapping_add(2);
            if cur_arch1 == end_arch1 {
                break;
            }
            loop {
                let a_id = rd::<u16>(cur_arch1) as usize;
                if rd::<u64>(iter1_base.wrapping_add(a_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(iter1_base.wrapping_add(a_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_arch1 = cur_arch1.wrapping_add(2);
                if cur_arch1 == end_arch1 {
                    break;
                }
            }
            if cur_arch1 == end_arch1 {
                break;
            }
        }
    }

    if total_count != 0 {
        exe::call_va!(0x1_4108_ada0, (scratch_vec_ptr, total_count));
    }

    let span2 = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init2 = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init2.as_mut_ptr() as usize,
            rsi_base,
            span2.as_ptr() as usize
        ) -> usize
    );

    let mut outer = [0u64; 18];
    outer[0] = rsi_base as u64;
    outer[1] = iter_init2[0];
    outer[2] = iter_init2[1];
    outer[3] = iter_init2[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>(rsi_base.wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>(rsi_base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4108_4750, (outer.as_mut_ptr() as usize));

    if outer[6] < outer[5] {
        let xmm15_1_0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let c_120 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize);
        let c_760 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
        let c_240 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8240) as usize);
        let c_1b0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_81b0) as usize);
        let c_840 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8840) as usize);
        let c_0c0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_80c0) as usize);
        let c_280 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8280) as usize);
        let c_8d0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_88d0) as usize);
        let c_4c0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_84c0) as usize);
        let c_2c0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_82c0) as usize);
        let c_460 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8460) as usize);
        let c_3b0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_83b0) as usize);

        while outer[6] < outer[5] {
            let rdx_idx = outer[6] as usize;
            let rsi_off = rdx_idx << 5;
            let r9_2e0 = outer[7] as usize;
            let r10_2d8 = outer[8] as usize;
            let rbx_3 = rdx_idx * 3;
            let r8_2c8 = outer[10] as usize;
            let r13_elem = (outer[12] as usize).wrapping_add(rdx_idx * 0xa0);
            let r11_2b0 = outer[13] as usize;
            let r15_2a0 = outer[15] as usize;
            let r12_298 = outer[16] as usize;
            let r15_entry = r15_2a0.wrapping_add(rdx_idx * 0x28);

            if rd::<u8>(r15_entry) == 0 {
                let slot_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
                let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
                let xmm14 = rd::<f32>(rd::<usize>(tbl_580e0.wrapping_add(slot_0d8 * 8)));

                let ymm3_lo = rd::<u128>(r11_2b0.wrapping_add(rsi_off));
                let ymm3_hi = rd::<u128>(r11_2b0.wrapping_add(rsi_off + 0x10));
                let rcx_2a8 = outer[14] as usize;
                let ymm0_lo = rd::<u128>(rcx_2a8.wrapping_add(rsi_off));
                let ymm0_hi = rd::<u128>(rcx_2a8.wrapping_add(rsi_off + 0x10));

                let cmp_lo = vcmpps(ymm0_lo, ymm3_lo, 0);
                let cmp_hi = vcmpps(ymm0_hi, ymm3_hi, 0);
                let (ymm9_lo, ymm9_hi) = if vmovmskps(cmp_lo & cmp_hi) == 0xf {
                    (ymm3_lo, ymm3_hi)
                } else {
                    let mut xmm11 = vbroadcastss(xmm14.to_bits());
                    let ymm9_in = ymm3_lo;
                    let mut xmm12 = ymm0_lo;
                    let mut xmm13 = ymm0_hi;
                    xmm13 = vfmadd_ps(xmm13, xmm11, xmm13, true, false);
                    xmm13 = vfmadd_ps(ymm3_hi, xmm11, xmm13, false, false);
                    let mut xmm4 = vsubps(xmm11, c_120);
                    let xmm1 = vdpps(xmm12, ymm9_in, 0xff);
                    let xmm8 = xmm1 & c_760;
                    let xmm5 = xmm8 ^ xmm1;
                    let xmm2 = vsubps(xmm11, c_240);
                    let xmm1 = vmulps(xmm4, xmm11);
                    let xmm7 = vmulps(xmm2, xmm1);
                    let mut xmm6 = c_1b0;
                    let mut xmm3 = c_840;
                    xmm3 = vfmadd_ps(xmm5, c_0c0, xmm3, false, false);
                    xmm6 = vfmadd_ps(xmm5, xmm3, xmm6, false, false);
                    xmm4 = vmulps(xmm4, xmm4);
                    let mut xmm3 = c_280;
                    let mut xmm2 = c_8d0;
                    let mut xmm1 = c_4c0;
                    xmm1 = vfmadd_ps(xmm5, c_2c0, xmm1, true, false);
                    xmm2 = vfmadd_ps(xmm5, xmm1, xmm2, false, false);
                    xmm3 = vfmadd_ps(xmm5, xmm2, xmm3, false, false);
                    xmm6 = vfmadd_ps(xmm3, xmm4, xmm6, false, false);
                    xmm11 = vfmadd_ps(xmm6, xmm7, xmm11, false, false);
                    xmm12 = vfmadd_ps(xmm11, xmm12, xmm12, true, false);
                    let xmm0 = xmm11 ^ xmm8;
                    xmm12 = vfmadd_ps(xmm0, ymm9_in, xmm12, false, false);
                    let xmm0 = vdpps(xmm12, xmm12, 0xff);
                    let xmm3_rsq = vrsqrtps_128(xmm0);
                    let xmm0_m = vmulps(xmm3_rsq, xmm0);
                    let xmm1_m = vmulps(xmm0_m, xmm3_rsq);
                    let xmm2_s = vsubps(c_460, xmm1_m);
                    let xmm0_h = vmulps(xmm3_rsq, c_120);
                    let xmm1_n = vmulps(xmm0_h, xmm2_s);
                    let xmm0_q = vmulps(xmm1_n, xmm12);
                    (xmm0_q, xmm13)
                };

                let xmm3_col = rd::<u128>(r9_2e0.wrapping_add(rsi_off));
                let xmm2_col = rd::<u128>(r10_2d8.wrapping_add(rsi_off));
                let xmm5_col = if vmovmskps(vcmpps(xmm3_col, xmm2_col, 0)) == 0xf {
                    xmm3_col
                } else {
                    let xmm1 = vbroadcastss(xmm14.to_bits());
                    let xmm0 = vsubps(xmm3_col, xmm2_col);
                    let xmm1 = vmulps(xmm0, xmm1);
                    vaddps(xmm1, xmm2_col)
                };

                let mut xmm3_f = rd::<f32>(r9_2e0.wrapping_add(rsi_off + 0x10));
                let xmm0_f = rd::<f32>(r10_2d8.wrapping_add(rsi_off + 0x10));
                let xmm4_f = xmm15_1_0 - xmm14;
                if xmm0_f != xmm3_f {
                    xmm3_f = (xmm0_f * xmm4_f) + (xmm3_f * xmm14);
                }

                let mut xmm2_f = rd::<f32>(r9_2e0.wrapping_add(rsi_off + 0x14));
                let xmm0_f2 = rd::<f32>(r10_2d8.wrapping_add(rsi_off + 0x14));
                if xmm0_f2 != xmm2_f {
                    xmm2_f = (xmm0_f2 * xmm4_f) + (xmm2_f * xmm14);
                }

                let mut xmm10_f = rd::<f32>(r8_2c8.wrapping_add(rbx_3 * 8));
                let ptr_118 = outer[11] as usize;
                let xmm0_f3 = rd::<f32>(ptr_118.wrapping_add(rdx_idx * 4));
                if xmm0_f3 != xmm10_f {
                    xmm10_f = (xmm0_f3 * xmm4_f) + (xmm10_f * xmm14);
                }

                let ptr_108 = outer[9] as usize;
                xmm2_f *= rd::<f32>(ptr_108.wrapping_add(rdx_idx * 4));
                let xmm2_vec = vbroadcastss(xmm2_f.to_bits());
                let xmm1_vec = vbroadcastss(xmm3_f.to_bits());
                let xmm11_vec = vmulps(vmulps(xmm2_vec, xmm5_col), xmm1_vec);
                let local_1a0: [u32; 3] = [
                    xmm11_vec as u32,
                    (xmm11_vec >> 32) as u32,
                    (xmm11_vec >> 64) as u32,
                ];

                let xmm3_q = vmulps(ymm9_lo, c_3b0);
                let xmm5_q = vmulps(xmm3_q, ymm9_lo);
                let xmm0_q = vshufps(ymm9_lo, ymm9_lo, 0xe0);
                let xmm2_q = vmulps(xmm0_q, c_3b0);
                let xmm1_q = vshufps(ymm9_lo, ymm9_lo, 0xd9);
                let xmm6_q = vmulps(xmm2_q, xmm1_q);
                let xmm0_w = vshufps(ymm9_lo, ymm9_lo, 0xff);
                let xmm4_q = vmulps(xmm0_w, xmm3_q);
                let xmm1_p = vpermilps(xmm5_q, 0xc1);
                let xmm2_p = vsubps(c_240, xmm1_p);
                let xmm0_p = vpermilps(xmm5_q, 0xda);
                let xmm7_p = vsubps(xmm2_p, xmm0_p);
                let xmm1_a = vpermilps(xmm4_q, 0xd2);
                let xmm0_a = vpermilps(xmm6_q, 0xd8);
                let xmm3_a = vaddps(xmm1_a, xmm0_a);
                let xmm2_s = vpermilps(xmm4_q, 0xc9);
                let xmm0_s = vpermilps(xmm6_q, 0xe1);
                let xmm5_s = vsubps(xmm0_s, xmm2_s);
                let xmm2_m = vshufps(xmm7_p, xmm3_a, 0x44);
                let xmm4_m = vshufps(xmm7_p, xmm3_a, 0xee);
                let xmm0_m = vshufps(xmm5_s, 0, 0x44);
                let xmm1_m = vshufps(xmm5_s, 0, 0xee);
                let xmm6_r0 = vshufps(xmm2_m, xmm0_m, 0x88);
                let xmm0_r1 = vshufps(xmm2_m, xmm0_m, 0xdd);
                let xmm5_r1 = vpermilps(xmm0_r1, 0xd2);
                let xmm1_r2 = vshufps(xmm4_m, xmm1_m, 0x88);
                let xmm3_r2 = vpermilps(xmm1_r2, 0xc9);
                let xmm2_t = vblendps(c_240, ymm9_hi, 0x07);
                let xmm1_t = vshufps(xmm6_r0, xmm5_r1, 0x4e);
                let mat_row0 = vshufps(xmm6_r0, xmm1_t, 0x84);
                let mat_row1 = vshufps(xmm5_r1, xmm3_r2, 0x49);
                let xmm0_t = vshufps(xmm3_r2, xmm2_t, 0x4e);
                let mat_row2 = vshufps(xmm0_t, xmm2_t, 0x98);
                let local_170: [u128; 3] = [mat_row0, mat_row1, mat_row2];

                let (val_60, val_64) = if rd::<u8>(r9_2e0.wrapping_add(rsi_off + 0x1c)) != 0 {
                    (rd::<u32>(r13_elem + 0x14), rd::<u32>(r13_elem + 0x10))
                } else {
                    (0u32, 0u32)
                };

                let xmm12_f = rd::<f32>(r8_2c8.wrapping_add(rbx_3 * 8 + 0x04));
                let xmm13_f = rd::<f32>(r8_2c8.wrapping_add(rbx_3 * 8 + 0x08));
                let xmm14_f = rd::<f32>(r8_2c8.wrapping_add(rbx_3 * 8 + 0x0c));
                let xmm1_18 = rd::<f32>(r9_2e0.wrapping_add(rsi_off + 0x18));

                let mut h = fnv1a_u32(canonical_f32_bits(xmm10_f)).wrapping_add(0x9e37_79b9);
                h = hash_combine(h, fnv1a_u32(canonical_f32_bits(xmm12_f)));
                h = hash_combine(h, fnv1a_u32(canonical_f32_bits(xmm13_f)));
                h = hash_combine(h, fnv1a_u32(canonical_f32_bits(xmm14_f)));
                let h_18 = fnv1a_u32(canonical_f32_bits(xmm1_18))
                    .wrapping_add(h << 6)
                    .wrapping_add(h >> 2)
                    .wrapping_add(0x9e37_79b9);
                h ^= h_18;

                let h_vec3 = exe::call_va!(
                    0x1_4108_4400,
                    (h_18, local_1a0.as_ptr() as usize) -> u64
                );
                let h_vec3_mix = h_vec3
                    .wrapping_add(h << 6)
                    .wrapping_add(h >> 2)
                    .wrapping_add(0x9e37_79b9);
                h ^= h_vec3_mix;

                let h_mat = exe::call_va!(
                    0x1_4108_aa30,
                    (h_vec3_mix, local_170.as_ptr() as usize) -> u64
                );
                h = hash_combine(h, h_mat);
                h = hash_combine(h, fnv1a_u32(val_60));
                h = hash_combine(h, fnv1a_u32(val_64));

                if h != rd::<u64>(r15_entry + 0x08) {
                    let ebx_id = rd::<u32>(r15_entry + 0x24);
                    let v_len = rd::<u32>(scratch_vec_ptr + 0x08) as usize;
                    let v_cap = rd::<u32>(scratch_vec_ptr + 0x0c) as usize;
                    if v_len * 0x70 == v_cap * 0x70 {
                        exe::call_va!(0x1_4108_ada0, (scratch_vec_ptr, (v_len + 1) as u64));
                    }
                    let idx = rd::<u32>(scratch_vec_ptr + 0x08) as usize;
                    let dst = rd::<usize>(scratch_vec_ptr).wrapping_add(idx * 0x70);
                    wr::<u32>(dst, ebx_id);
                    wr::<f32>(dst + 0x04, xmm10_f);
                    wr::<f32>(dst + 0x08, xmm12_f);
                    wr::<f32>(dst + 0x0c, xmm13_f);
                    wr::<f32>(dst + 0x10, xmm14_f);
                    wr::<f32>(dst + 0x14, xmm1_18);
                    wr::<u32>(dst + 0x18, local_1a0[0]);
                    wr::<u32>(dst + 0x1c, local_1a0[1]);
                    wr::<u32>(dst + 0x20, local_1a0[2]);
                    wr::<u128>(dst + 0x30, mat_row0);
                    wr::<u128>(dst + 0x40, mat_row1);
                    wr::<u128>(dst + 0x50, mat_row2);
                    wr::<u32>(dst + 0x60, val_60);
                    wr::<u32>(dst + 0x64, val_64);
                    wr::<u32>(scratch_vec_ptr + 0x08, (idx as u32).wrapping_add(1));
                    wr::<u64>(r15_entry + 0x08, h);
                }

                let r8_id = rd::<u64>(r8_2c8.wrapping_add(rbx_3 * 8 + 0x10));
                if r8_id != rd::<u64>(r15_entry + 0x10)
                    || rd::<f32>(r15_entry + 0x18) != rd::<f32>(r13_elem + 0x7c)
                    || rd::<f32>(r15_entry + 0x1c) != rd::<f32>(r13_elem + 0x80)
                    || rd::<f32>(r15_entry + 0x20) != rd::<f32>(r13_elem + 0x84)
                {
                    let r12_slot = r12_298.wrapping_add(rsi_off);
                    if r8_id.wrapping_sub(1) <= 0xffff_ffff_ffff_fffd {
                        let g_82b8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
                        let found = exe::call_va!(0x1_4242_1fe0, (g_82b8, 0u64, r8_id) -> u64);
                        if found != 0 {
                            exe::call_va!(0x1_4220_5c30, (r12_slot + 0x10));
                            let g_82b8_2 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
                            let type_desc = exe::ptr::<u8>(0x1_43a8_ad98) as usize;
                            let new_obj = exe::call_va!(
                                0x1_4242_1ed0,
                                (g_82b8_2, type_desc, r8_id, 0u64, 0u64) -> usize
                            );
                            let old_obj = rd::<usize>(r12_slot);
                            wr::<usize>(r12_slot, new_obj);
                            release_ref_24(old_obj);

                            let mut local_80 = rd::<usize>(r12_slot);
                            let mut local_3e0 = rd::<u8>(local_80 + 0xb8);
                            retain_ref_24(local_80);
                            exe::call_va!(
                                0x1_4108_62f0,
                                (
                                    r15_entry + 0x24,
                                    (&mut local_80 as *mut usize) as usize,
                                    (&mut local_3e0 as *mut u8) as usize,
                                    r13_elem + 0x7c,
                                    r13_elem + 0x80
                                )
                            );
                            release_ref_24(local_80);

                            let cur_obj = rd::<usize>(r12_slot);
                            if rd::<u8>(cur_obj + 0xb8) != 0 {
                                let c_6e08 =
                                    rd::<u32>(exe::ptr::<u32>(0x1_4342_6e08) as usize);
                                let _ = crate::simd_ops::call_va8_xmm(
                                    0x1_4220_5c80,
                                    (r12_slot + 0x10) as u64,
                                    c_6e08 as u64,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    [0, c_6e08 as u128, 0, 0],
                                );
                            }
                            wr::<u64>(
                                r15_entry + 0x10,
                                rd::<u64>(r8_2c8.wrapping_add(rbx_3 * 8 + 0x10)),
                            );
                            wr::<u32>(r15_entry + 0x18, rd::<u32>(r13_elem + 0x7c));
                            wr::<u64>(r15_entry + 0x1c, rd::<u64>(r13_elem + 0x80));
                        }
                    } else {
                        let mut local_90 = [xmm15_1_0, xmm15_1_0];
                        let mut local_78 = 0.0f32;
                        let mut local_3f0 = 0u8;
                        let mut local_88: usize = 0;
                        exe::call_va!(
                            0x1_4108_62f0,
                            (
                                r15_entry + 0x24,
                                (&mut local_88 as *mut usize) as usize,
                                (&mut local_3f0 as *mut u8) as usize,
                                (&mut local_78 as *mut f32) as usize,
                                local_90.as_mut_ptr() as usize
                            )
                        );
                        release_ref_24(local_88);
                        exe::call_va!(0x1_4220_5c30, (r12_slot + 0x10));
                        let old_obj = rd::<usize>(r12_slot);
                        wr::<usize>(r12_slot, 0);
                        release_ref_24(old_obj);
                        wr::<u64>(
                            r15_entry + 0x10,
                            rd::<u64>(r8_2c8.wrapping_add(rbx_3 * 8 + 0x10)),
                        );
                        wr::<u32>(r15_entry + 0x18, 0);
                        let one_pair =
                            (xmm15_1_0.to_bits() as u64) | ((xmm15_1_0.to_bits() as u64) << 32);
                        wr::<u64>(r15_entry + 0x1c, one_pair);
                    }
                }
            }

            outer[6] = outer[6].wrapping_add(1);
            if outer[6] >= outer[5] {
                outer[2] = outer[2].wrapping_add(2);
                if outer[2] != outer[3] {
                    loop {
                        let arch_id = rd::<u16>(outer[2] as usize) as usize;
                        let base_c8 = outer[1] as usize;
                        if rd::<u64>(base_c8.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                            && rd::<u32>(base_c8.wrapping_add(arch_id * 4).wrapping_add(0x10048))
                                != 0
                        {
                            break;
                        }
                        outer[2] = outer[2].wrapping_add(2);
                        if outer[2] == outer[3] {
                            break;
                        }
                    }
                    if outer[2] != outer[3] {
                        let base_c0 = outer[0] as usize;
                        let arch_id = rd::<u16>(outer[2] as usize) as usize;
                        let cp = rd::<u64>(base_c0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                        outer[4] = cp;
                        outer[5] = if cp != 0 {
                            rd::<u32>(base_c0.wrapping_add(arch_id * 4).wrapping_add(0x10048))
                                as u64
                        } else {
                            0
                        };
                        outer[6] = 0;
                    }
                }
                exe::call_va!(0x1_4108_4750, (outer.as_mut_ptr() as usize));
            }
        }
    }

    let r15_cnt = rd::<u32>(scratch_vec_ptr + 0x08) as usize;
    if r15_cnt != 0 {
        let g_ff58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        let r14_bytes = r15_cnt * 0x70 + 4;
        let (pkt_hdr, dst_payload, pkt_size): (usize, usize, u64) = if r14_bytes < 0x00ff_ffc0 {
            let rdi_ring = rd::<usize>(g_ff58 + 0x168);
            exe::call_iat!(0x1_4264_55a8, (rdi_ring + 0x18));
            let r9_aligned = (r14_bytes + 7) & !7usize;
            let mut rdx_read = crate::simd_ops::rd_acq_usize(rdi_ring + 0x20);
            let mut rsi_write = crate::simd_ops::rd_acq_usize(rdi_ring + 0x28);
            let mut r8_next = rsi_write.wrapping_add(8).wrapping_add(r9_aligned);
            if r8_next > rd::<usize>(rdi_ring + 0x10) {
                while rsi_write <= rdx_read
                    && rdx_read < r8_next
                    && crate::simd_ops::rd_acq_u32(rdi_ring + 0x30) != crate::simd_ops::rd_acq_u32(rdi_ring + 0x34)
                {
                    core::hint::spin_loop();
                    rdx_read = crate::simd_ops::rd_acq_usize(rdi_ring + 0x20);
                }
                if rsi_write <= rd::<usize>(rdi_ring + 0x10).wrapping_sub(8) {
                    crate::simd_ops::wr_rel_u64(rsi_write, 0x0000_00ff_ffff_ffff);
                }
                rsi_write = rd::<usize>(rdi_ring + 0x08);
                r8_next = rsi_write.wrapping_add(8).wrapping_add(r9_aligned);
            }
            while rsi_write <= rdx_read
                && rdx_read < r8_next
                && crate::simd_ops::rd_acq_u32(rdi_ring + 0x30) != crate::simd_ops::rd_acq_u32(rdi_ring + 0x34)
            {
                core::hint::spin_loop();
                rdx_read = crate::simd_ops::rd_acq_usize(rdi_ring + 0x20);
            }
            crate::simd_ops::wr_rel_u64(rsi_write, 0x0000_69fa_fafa_faff);
            let hdr = rsi_write.wrapping_add(8);
            crate::simd_ops::atomic_xchg_u64((rdi_ring + 0x28) as u64, r8_next as u64);
            exe::call_iat!(0x1_4264_5598, (rdi_ring + 0x18));
            wr::<u32>(hdr, ((r15_cnt as u32) << 9) | 0xae);
            (hdr, hdr.wrapping_add(4), r14_bytes as u64)
        } else {
            let ext_buf = exe::call_va!(
                0x1_4020_c080,
                (0u64, 0u64, 0u64, (r15_cnt * 0x70) as u64) -> usize
            );
            let rsi_ring = rd::<usize>(g_ff58 + 0x168);
            exe::call_iat!(0x1_4264_55a8, (rsi_ring + 0x18));
            let mut rdx_read = crate::simd_ops::rd_acq_usize(rsi_ring + 0x20);
            let mut r14_write = crate::simd_ops::rd_acq_usize(rsi_ring + 0x28);
            let mut r8_next = r14_write.wrapping_add(0x20);
            if r8_next > rd::<usize>(rsi_ring + 0x10) {
                while r14_write <= rdx_read
                    && rdx_read < r8_next
                    && crate::simd_ops::rd_acq_u32(rsi_ring + 0x30) != crate::simd_ops::rd_acq_u32(rsi_ring + 0x34)
                {
                    core::hint::spin_loop();
                    rdx_read = crate::simd_ops::rd_acq_usize(rsi_ring + 0x20);
                }
                if r14_write <= rd::<usize>(rsi_ring + 0x10).wrapping_sub(8) {
                    crate::simd_ops::wr_rel_u64(r14_write, 0x0000_00ff_ffff_ffff);
                }
                r14_write = rd::<usize>(rsi_ring + 0x08);
                r8_next = r14_write.wrapping_add(0x20);
            }
            while r14_write <= rdx_read
                && rdx_read < r8_next
                && crate::simd_ops::rd_acq_u32(rsi_ring + 0x30) != crate::simd_ops::rd_acq_u32(rsi_ring + 0x34)
            {
                core::hint::spin_loop();
                rdx_read = crate::simd_ops::rd_acq_usize(rsi_ring + 0x20);
            }
            crate::simd_ops::wr_rel_u64(r14_write, 0x0000_69fa_fafa_faff);
            let hdr = r14_write.wrapping_add(8);
            crate::simd_ops::atomic_xchg_u64((rsi_ring + 0x28) as u64, r8_next as u64);
            exe::call_iat!(0x1_4264_5598, (rsi_ring + 0x18));
            wr::<u32>(hdr, ((r15_cnt as u32) << 9) | 1);
            wr::<u32>(hdr + 0x04, 0xae);
            wr::<usize>(hdr + 0x0c, ext_buf);
            (hdr, ext_buf, 0x14u64)
        };

        let mut src_cur = rd::<usize>(scratch_vec_ptr);
        let src_end =
            src_cur.wrapping_add((rd::<u32>(scratch_vec_ptr + 0x08) as usize) * 0x70);
        let mut dst_cur = dst_payload;
        while src_cur != src_end {
            wr::<u32>(dst_cur, rd::<u32>(src_cur));
            wr::<u32>(dst_cur + 0x04, rd::<u32>(src_cur + 0x04));
            wr::<u32>(dst_cur + 0x08, rd::<u32>(src_cur + 0x08));
            wr::<u32>(dst_cur + 0x0c, rd::<u32>(src_cur + 0x0c));
            wr::<u32>(dst_cur + 0x10, rd::<u32>(src_cur + 0x10));
            wr::<u32>(dst_cur + 0x14, rd::<u32>(src_cur + 0x14));
            wr::<u64>(dst_cur + 0x18, rd::<u64>(src_cur + 0x18));
            wr::<u32>(dst_cur + 0x20, rd::<u32>(src_cur + 0x20));
            wr::<u64>(dst_cur + 0x30, rd::<u64>(src_cur + 0x30));
            wr::<u32>(dst_cur + 0x38, rd::<u32>(src_cur + 0x38));
            wr::<u64>(dst_cur + 0x3c, rd::<u64>(src_cur + 0x3c));
            wr::<u32>(dst_cur + 0x44, rd::<u32>(src_cur + 0x44));
            wr::<u64>(dst_cur + 0x48, rd::<u64>(src_cur + 0x48));
            wr::<u32>(dst_cur + 0x50, rd::<u32>(src_cur + 0x50));
            wr::<u64>(dst_cur + 0x54, rd::<u64>(src_cur + 0x54));
            wr::<u32>(dst_cur + 0x5c, rd::<u32>(src_cur + 0x5c));
            wr::<u32>(dst_cur + 0x60, rd::<u32>(src_cur + 0x60));
            wr::<u32>(dst_cur + 0x64, rd::<u32>(src_cur + 0x64));
            src_cur = src_cur.wrapping_add(0x70);
            dst_cur = dst_cur.wrapping_add(0x70);
        }

        let r9_ff58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        let copied = (dst_cur.wrapping_sub(dst_payload) / 0x70) as u32;
        wr::<u32>(pkt_hdr, (rd::<u32>(pkt_hdr) & 0x1ff) | (copied << 9));
        let ring_rcx = rd::<usize>(r9_ff58 + 0x168);
        crate::simd_ops::wr_rel_u64(pkt_hdr - 8, pkt_size);
        let _ = crate::simd_ops::atomic_add_u32((ring_rcx + 0x30) as u64, 1);
        if rd::<u8>(r9_ff58 + 0x198) != 0 {
            let sem = rd::<u64>(r9_ff58 + 0x1a0);
            exe::call_iat!(0x1_4264_5580, (sem, 1u64, 0u64));
        }
    }

    let buf = rd::<usize>(scratch_vec_ptr);
    if buf != 0 {
        exe::call_va!(0x1_4241_c190, (buf));
    }
}

#[inline(always)]
unsafe fn skip_empty_ctrl(mut ctrl: usize, mut slot: usize, slot_stride: usize) -> (usize, usize) {
    if rd::<i8>(ctrl) < -1 {
        let all_m1 = u128::MAX;
        loop {
            let cmp = vpcmpgtb(all_m1, rd::<u128>(ctrl));
            let mask = (vpmovmskb(cmp) as u32).wrapping_add(1);
            let step = mask.trailing_zeros() as usize;
            ctrl = ctrl.wrapping_add(step);
            slot = slot.wrapping_add(step.wrapping_mul(slot_stride));
            if rd::<i8>(ctrl) >= -1 {
                break;
            }
        }
    }
    (ctrl, slot)
}

/// `unknown_14202f140`
/// Original: `0x14202f140` (614 bytes, `Throws::No`)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_14202f140(param_1: usize, param_2: f32) {
    unknown_14202f140_impl(param_1, param_2);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_14202f140(param_1: usize, _xmm0: f32, param_2: f32) {
    unknown_14202f140_impl(param_1, param_2);
}

#[inline(always)]
unsafe fn unknown_14202f140_impl(param_1: usize, param_2: f32) {
    let r8_ctrl_base = rd::<usize>(param_1);
    let rdi_slot_base = rd::<usize>(param_1 + 0x08);
    let (mut rbp_ctrl, mut rdi_slot) = skip_empty_ctrl(r8_ctrl_base, rdi_slot_base, 0x48);
    let r15_ctrl_end = r8_ctrl_base.wrapping_add(rd::<usize>(param_1 + 0x18));
    if rbp_ctrl == r15_ctrl_end {
        return;
    }

    loop {
        let rdx_inner_ctrl = rd::<usize>(rdi_slot + 0x18);
        let rsi_inner_slot = rd::<usize>(rdi_slot + 0x20);
        let (mut rbx_ctrl, mut rsi_slot) = skip_empty_ctrl(rdx_inner_ctrl, rsi_inner_slot, 0x10);
        let inner_end_init = rdx_inner_ctrl.wrapping_add(rd::<usize>(rdi_slot + 0x30));
        if rbx_ctrl != inner_end_init {
            loop {
                let new_val = rd::<f32>(rsi_slot + 0x08) - param_2;
                wr::<f32>(rsi_slot + 0x08, new_val);
                if !(new_val <= 0.0f32) {
                    let next =
                        skip_empty_ctrl(rbx_ctrl.wrapping_add(1), rsi_slot.wrapping_add(0x10), 0x10);
                    rbx_ctrl = next.0;
                    rsi_slot = next.1;
                } else {
                    let mut rcx_arr = rd::<usize>(rdi_slot + 0x08);
                    let mut edx_len = rd::<u32>(rdi_slot + 0x10);
                    let arr_end = rcx_arr.wrapping_add((edx_len as usize) * 8);
                    if rcx_arr != arr_end {
                        let mut r8_arr_base = rcx_arr;
                        loop {
                            let target_key = rd::<u64>(rsi_slot);
                            if rd::<u64>(rcx_arr) == target_key {
                                let last_idx = (edx_len as usize).wrapping_sub(1);
                                let last_ptr = r8_arr_base.wrapping_add(last_idx.wrapping_mul(8));
                                if rcx_arr != last_ptr {
                                    wr::<u64>(rcx_arr, rd::<u64>(last_ptr));
                                }
                                let next_len = rd::<u32>(rdi_slot + 0x10).wrapping_sub(1);
                                wr::<u32>(rdi_slot + 0x10, next_len);
                                edx_len = rd::<u32>(rdi_slot + 0x10);
                            } else {
                                rcx_arr = rcx_arr.wrapping_add(8);
                            }
                            r8_arr_base = rd::<usize>(rdi_slot + 0x08);
                            if rcx_arr == r8_arr_base.wrapping_add((edx_len as usize) * 8) {
                                break;
                            }
                        }
                    }

                    let mut erase_it = [rbx_ctrl as u64, rsi_slot as u64];
                    let next =
                        skip_empty_ctrl(rbx_ctrl.wrapping_add(1), rsi_slot.wrapping_add(0x10), 0x10);
                    rbx_ctrl = next.0;
                    rsi_slot = next.1;
                    exe::call_va!(
                        0x1_411b_c750,
                        (rdi_slot + 0x18, erase_it.as_mut_ptr() as usize)
                    );
                }

                let inner_end =
                    rd::<usize>(rdi_slot + 0x18).wrapping_add(rd::<usize>(rdi_slot + 0x30));
                if rbx_ctrl == inner_end {
                    break;
                }
            }
        }

        let next_outer =
            skip_empty_ctrl(rbp_ctrl.wrapping_add(1), rdi_slot.wrapping_add(0x48), 0x48);
        rbp_ctrl = next_outer.0;
        rdi_slot = next_outer.1;
        if rbp_ctrl == r15_ctrl_end {
            break;
        }
    }
}

/// `unknown_1420a3e40`
/// Original: `0x1420a3e40` (1004 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420a3e40(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u16,
    param_5: u16,
) -> usize {
    let mut local_ctx = [0u64; 5];
    let ctx_ptr = local_ctx.as_mut_ptr() as usize;
    wr::<usize>(ctx_ptr, param_3);

    let ebx_cnt = rd::<u32>(param_3 + 0x58);
    if ebx_cnt != 0 {
        let visited_buf = exe::call_va!(0x1_4241_c0a0, (ebx_cnt as u64, 1u64) -> usize);
        wr::<usize>(ctx_ptr + 0x08, visited_buf);
        wr::<u32>(ctx_ptr + 0x14, ebx_cnt);
        core::ptr::write_bytes(visited_buf as *mut u8, 0, ebx_cnt as usize);
    }
    wr::<u32>(ctx_ptr + 0x10, ebx_cnt);

    exe::call_va!(0x1_420a_4ee0, (ctx_ptr, param_4 as u32));

    let mut esi_len = rd::<u32>(ctx_ptr + 0x20);
    let mut r15_stack = rd::<usize>(ctx_ptr + 0x18);
    if r15_stack != r15_stack.wrapping_add((esi_len as usize) * 4) {
        let r10_nodes = rd::<usize>(param_2 + 0x08);
        let mut r12_cap = rd::<u32>(ctx_ptr + 0x24);

        loop {
            let ecx_item = rd::<u32>(r15_stack.wrapping_add((esi_len as usize) * 4).wrapping_sub(4));
            let r9b_flag = ((ecx_item >> 16) & 0xff) as u8;
            esi_len = esi_len.wrapping_sub(1);
            let r8w_idx = (ecx_item & 0xffff) as u16;
            let r8_idx = r8w_idx as usize;

            let node_entry = r10_nodes.wrapping_add(r8_idx << 6);
            if rd::<u16>(node_entry + 0x22) == param_5 {
                wr::<u16>(param_1, rd::<u16>(node_entry + 0x20));
                wr::<u8>(param_1 + 0x02, 1);
                if r15_stack != 0 {
                    exe::call_va!(0x1_4241_c190, (r15_stack));
                }
                let visited = rd::<usize>(ctx_ptr + 0x08);
                if visited != 0 {
                    exe::call_va!(0x1_4241_c190, (visited));
                }
                return param_1;
            }

            let mut r9_visited = rd::<usize>(ctx_ptr + 0x08);
            if r9b_flag != 0 {
                let lo_b = (r8w_idx & 0xff) as u64;
                let hi_b = (r8w_idx >> 8) as u64;
                let hash = ((lo_b ^ 0xcbf2_9ce4_8422_2325u64)
                    .wrapping_mul(0x0100_0000_01b3)
                    ^ hi_b)
                    .wrapping_mul(0x0100_0000_01b3);
                let p3 = rd::<usize>(ctx_ptr);
                let bucket_idx = (rd::<u64>(p3 + 0x30) & hash) as usize;
                let buckets_base = rd::<usize>(p3 + 0x18);
                let mut rbx_node = rd::<usize>(buckets_base.wrapping_add(bucket_idx * 16));
                let sentinel = rd::<usize>(p3 + 0x08);
                if rbx_node != sentinel {
                    let bucket_last =
                        rd::<usize>(buckets_base.wrapping_add(bucket_idx * 16 + 0x08));
                    let mut found = true;
                    while rd::<u16>(rbx_node + 0x10) != r8w_idx {
                        if rbx_node == bucket_last {
                            found = false;
                            break;
                        }
                        rbx_node = rd::<usize>(rbx_node);
                    }
                    if found {
                        let mut rbp_end = rd::<usize>(rbx_node);
                        if rbx_node != bucket_last {
                            loop {
                                let cur = rbp_end;
                                if rd::<u16>(rbp_end + 0x10) != r8w_idx {
                                    break;
                                }
                                rbp_end = rd::<usize>(rbp_end);
                                if cur == bucket_last {
                                    break;
                                }
                            }
                        }
                        r9_visited = rd::<usize>(ctx_ptr + 0x08);
                        while rbx_node != rbp_end {
                            let target_u16 = rd::<u16>(rbx_node + 0x12);
                            let target_idx = target_u16 as usize;
                            if rd::<u8>(r9_visited.wrapping_add(target_idx)) == 0 {
                                if esi_len == r12_cap {
                                    let cur_len = esi_len as usize;
                                    let cur_cap = r12_cap as usize;
                                    let new_cap = (cur_cap + (cur_cap >> 1)).max(cur_len + 1);
                                    r12_cap = new_cap as u32;
                                    let new_buf = exe::call_va!(
                                        0x1_4241_c0a0,
                                        ((new_cap * 4) as u64, 2u64) -> usize
                                    );
                                    core::ptr::copy(
                                        r15_stack as *const u8,
                                        new_buf as *mut u8,
                                        cur_len * 4,
                                    );
                                    if r15_stack != 0 {
                                        exe::call_va!(0x1_4241_c190, (r15_stack));
                                    }
                                    r15_stack = new_buf;
                                    r9_visited = rd::<usize>(ctx_ptr + 0x08);
                                }
                                wr::<u32>(
                                    r15_stack.wrapping_add((esi_len as usize) * 4),
                                    (target_u16 as u32) | 0x0001_0000,
                                );
                                esi_len = esi_len.wrapping_add(1);
                                wr::<u8>(r9_visited.wrapping_add(target_idx), 1);
                            }
                            rbx_node = rd::<usize>(rbx_node);
                        }
                    }
                }
            }

            let r10_p3 = rd::<usize>(ctx_ptr);
            let adj_ranges = rd::<usize>(r10_p3 + 0x50);
            let packed_range = rd::<u32>(adj_ranges.wrapping_add(r8_idx * 4));
            let start_edge = (packed_range & 0xffff) as u16;
            let end_edge = (packed_range >> 16) as u16;
            if start_edge != end_edge {
                let edge_cnt = end_edge.wrapping_sub(start_edge) as usize;
                let adj_targets = rd::<usize>(r10_p3 + 0x40);
                let mut rbx_edge = adj_targets.wrapping_add((start_edge as usize) * 2);
                let rdx_edge_end = rbx_edge.wrapping_add(edge_cnt * 2);
                while rbx_edge != rdx_edge_end {
                    let target_u16 = rd::<u16>(rbx_edge);
                    let target_idx = target_u16 as usize;
                    if rd::<u8>(r9_visited.wrapping_add(target_idx)) == 0 {
                        if esi_len == r12_cap {
                            let cur_len = esi_len as usize;
                            let cur_cap = r12_cap as usize;
                            let new_cap = (cur_cap + (cur_cap >> 1)).max(cur_len + 1);
                            r12_cap = new_cap as u32;
                            let new_buf = exe::call_va!(
                                0x1_4241_c0a0,
                                ((new_cap * 4) as u64, 2u64) -> usize
                            );
                            core::ptr::copy(
                                r15_stack as *const u8,
                                new_buf as *mut u8,
                                cur_len * 4,
                            );
                            if r15_stack != 0 {
                                exe::call_va!(0x1_4241_c190, (r15_stack));
                            }
                            r15_stack = new_buf;
                            r9_visited = rd::<usize>(ctx_ptr + 0x08);
                        }
                        wr::<u32>(
                            r15_stack.wrapping_add((esi_len as usize) * 4),
                            target_u16 as u32,
                        );
                        esi_len = esi_len.wrapping_add(1);
                        wr::<u8>(r9_visited.wrapping_add(target_idx), 1);
                    }
                    rbx_edge = rbx_edge.wrapping_add(2);
                }
            }

            if r15_stack == r15_stack.wrapping_add((esi_len as usize) * 4) {
                break;
            }
        }
    }

    wr::<u8>(param_1 + 0x02, 0);
    if r15_stack != 0 {
        exe::call_va!(0x1_4241_c190, (r15_stack));
    }
    let visited = rd::<usize>(ctx_ptr + 0x08);
    if visited != 0 {
        exe::call_va!(0x1_4241_c190, (visited));
    }
    param_1
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141f27160(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_BOUNDS: [usize; 2] = [0; 2];
    static mut STREAMS: [u64; 10] = [0; 10];
    static mut SCRATCH_70: [u8; 0x100] = [0; 0x100];
    static mut FAKE_RES_OBJ: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_iter_begin(out: usize, rsi: usize, _span: usize) -> usize {
        wr::<usize>(out, rsi);
        wr::<usize>(out + 0x08, unsafe { ARCH_BOUNDS[0] });
        wr::<usize>(out + 0x10, unsafe { ARCH_BOUNDS[1] });
        out
    }
    unsafe extern "win64" fn mock_ada0(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(SCRATCH_70) as usize);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_4750(outer: usize) {
        if rd::<u64>(outer + 0x30) < rd::<u64>(outer + 0x28) {
            for i in 0..10 {
                wr::<u64>(outer + 0x38 + i * 8, unsafe { STREAMS[i] });
            }
        }
    }
    unsafe extern "win64" fn mock_4400(_rcx: u64, vec_ptr: usize) -> u64 {
        let a = rd::<u32>(vec_ptr) as u64;
        let b = rd::<u32>(vec_ptr + 4) as u64;
        let c = rd::<u32>(vec_ptr + 8) as u64;
        a.wrapping_mul(31).wrapping_add(b).wrapping_mul(31).wrapping_add(c)
    }
    unsafe extern "win64" fn mock_aa30(_rcx: u64, mat_ptr: usize) -> u64 {
        let a = rd::<u64>(mat_ptr);
        let b = rd::<u64>(mat_ptr + 0x10);
        let c = rd::<u64>(mat_ptr + 0x20);
        a ^ b.rotate_left(13) ^ c.rotate_left(29)
    }
    unsafe extern "win64" fn mock_1fe0(_mgr: u64, _rdx: u64, id: u64) -> u64 {
        unsafe { rec(0x1fe0 ^ id) };
        1
    }
    unsafe extern "win64" fn mock_5c30(ptr: usize) {
        wr::<u32>(ptr, rd::<u32>(ptr).wrapping_add(10));
    }
    unsafe extern "win64" fn mock_1ed0(
        _mgr: u64,
        _td: usize,
        id: u64,
        _r9: u64,
        s5: u8,
    ) -> usize {
        unsafe {
            rec(0x1ed0 ^ id ^ (s5 as u64));
            FAKE_RES_OBJ
        }
    }
    unsafe extern "win64" fn mock_d6f0(mgr: usize, obj: usize) {
        unsafe { rec(0xd6f0 ^ (mgr as u64) ^ (rd::<u32>(obj + 0x24) as u64)) };
    }
    unsafe extern "win64" fn mock_62f0(
        rcx: usize,
        rdx: usize,
        r8: usize,
        r9: usize,
        s5: usize,
    ) {
        unsafe {
            rec(
                0x62f0
                    ^ (rd::<u32>(rcx) as u64)
                    ^ (rd::<usize>(rdx) as u64 != 0) as u64
                    ^ (rd::<u8>(r8) as u64)
                    ^ (rd::<u32>(r9) as u64)
                    ^ (rd::<u32>(s5) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_5c80(rcx: usize) {
        wr::<u32>(rcx + 4, 0x7777);
    }
    unsafe extern "win64" fn noop1(_a: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4108_ada0, mock_ada0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4108_4750, mock_4750 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4108_4400, mock_4400 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4108_aa30, mock_aa30 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4242_1fe0, mock_1fe0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4220_5c30, mock_5c30 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4242_1ed0, mock_1ed0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4108_62f0, mock_62f0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4220_5c80, mock_5c80 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4241_c190, noop1 as *const ())?;

    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let g_82b8 = exe::ptr::<u64>(0x1_43a0_82b8) as usize;
    let g_ff58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let s_0d8 = rd::<u16>(g_0d8);
    let s_82b8 = rd::<u64>(g_82b8);
    let s_ff58 = rd::<usize>(g_ff58);
    wr::<u16>(g_0d8, 0);
    wr::<u64>(g_82b8, 0x5555);

    let run = |f: F| -> (u64, [u8; 0x100], [u8; 0x50], [u8; 0x40], u32, u32) {
        let arch_ids: [u16; 1] = [0];
        unsafe {
            ARCH_BOUNDS[0] = arch_ids.as_ptr() as usize;
            ARCH_BOUNDS[1] = arch_ids.as_ptr().add(1) as usize;
            SCRATCH_70 = [0; 0x100];
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88, 0x1000);
        wr::<u32>(wp + 0x10088, 2);

        let mut s_f8 = [0u8; 0x40];
        let pf8 = s_f8.as_mut_ptr() as usize;
        wr::<f32>(pf8 + 0x00, 1.0);
        wr::<f32>(pf8 + 0x04, 2.0);
        wr::<f32>(pf8 + 0x08, 3.0);
        wr::<f32>(pf8 + 0x10, 4.0);
        wr::<f32>(pf8 + 0x14, 5.0);
        wr::<f32>(pf8 + 0x18, 6.0);
        wr::<u8>(pf8 + 0x1c, 1);

        let mut s_100 = [0u8; 0x40];
        let p100 = s_100.as_mut_ptr() as usize;
        wr::<f32>(p100 + 0x00, 0.5);
        wr::<f32>(p100 + 0x04, 1.0);
        wr::<f32>(p100 + 0x08, 1.5);
        wr::<f32>(p100 + 0x10, 2.0);
        wr::<f32>(p100 + 0x14, 3.0);

        let s_108: [f32; 2] = [2.0, 1.0];

        let mut s_110 = [0u8; 0x30];
        let p110 = s_110.as_mut_ptr() as usize;
        wr::<f32>(p110 + 0x00, 0.5);
        wr::<f32>(p110 + 0x04, 1.25);
        wr::<f32>(p110 + 0x08, 2.5);
        wr::<f32>(p110 + 0x0c, 3.75);
        wr::<u64>(p110 + 0x10, 0x42);
        wr::<u64>(p110 + 0x28, 0);

        let s_118: [f32; 2] = [0.25, 0.0];

        let mut s_120 = [0u8; 0x140];
        let p120 = s_120.as_mut_ptr() as usize;
        wr::<u32>(p120 + 0x10, 0xaaaa);
        wr::<u32>(p120 + 0x14, 0xbbbb);
        wr::<f32>(p120 + 0x7c, 0.5);
        wr::<f32>(p120 + 0x80, 1.5);
        wr::<f32>(p120 + 0x84, 2.5);
        wr::<f32>(p120 + 0xa0 + 0x7c, 1.0);

        let mut s_128: [f32; 16] = [0.0; 16];
        s_128[2] = 0.707_106_77;
        s_128[3] = 0.707_106_77;
        s_128[4] = 10.0;
        s_128[5] = 20.0;
        s_128[6] = 30.0;
        s_128[11] = 1.0;

        let mut s_130: [f32; 16] = [0.0; 16];
        s_130[3] = 1.0;
        s_130[4] = 2.0;
        s_130[5] = 4.0;
        s_130[6] = 6.0;
        s_130[11] = 1.0;

        let mut s_138 = [0u8; 0x50];
        let p138 = s_138.as_mut_ptr() as usize;
        wr::<u32>(p138 + 0x24, 101);
        wr::<u32>(p138 + 0x28 + 0x24, 102);

        let mut old_res = [0u8; 0xd0];
        wr::<u32>(old_res.as_mut_ptr() as usize + 0x24, 1);
        wr::<usize>(old_res.as_mut_ptr() as usize + 0x28, 0x7000);

        let mut new_res = [0u8; 0xd0];
        wr::<u32>(new_res.as_mut_ptr() as usize + 0x24, 1);
        wr::<usize>(new_res.as_mut_ptr() as usize + 0x28, 0x8000);
        wr::<u8>(new_res.as_mut_ptr() as usize + 0xb8, 1);
        unsafe { FAKE_RES_OBJ = new_res.as_mut_ptr() as usize };

        let mut s_140 = [0u8; 0x40];
        wr::<usize>(s_140.as_mut_ptr() as usize, old_res.as_mut_ptr() as usize);

        unsafe {
            STREAMS = [
                pf8 as u64,
                p100 as u64,
                s_108.as_ptr() as u64,
                p110 as u64,
                s_118.as_ptr() as u64,
                p120 as u64,
                s_128.as_ptr() as u64,
                s_130.as_ptr() as u64,
                p138 as u64,
                s_140.as_mut_ptr() as u64,
            ];
        }

        let mut ring_buf = [0u8; 0x100];
        let rbp = ring_buf.as_mut_ptr() as usize;
        let mut ring_struct = [0u8; 0x40];
        let rsp_ring = ring_struct.as_mut_ptr() as usize;
        wr::<usize>(rsp_ring + 0x08, rbp);
        wr::<usize>(rsp_ring + 0x10, rbp + 0x100);
        wr::<usize>(rsp_ring + 0x20, rbp);
        wr::<usize>(rsp_ring + 0x28, rbp);

        let mut ff58_obj = [0u8; 0x1b0];
        wr::<usize>(ff58_obj.as_mut_ptr() as usize + 0x168, rsp_ring);
        wr::<u8>(ff58_obj.as_mut_ptr() as usize + 0x198, 0);
        wr::<usize>(g_ff58, ff58_obj.as_mut_ptr() as usize);

        let interp_alpha: f32 = 0.5;
        let tbl_580e0: [usize; 1] = [(&interp_alpha as *const f32) as usize];
        let mut p2 = vec![0u8; 0x58100];
        wr::<usize>(p2.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1: [u64; 3] = [wp as u64, 0, 0];
        f(p1.as_ptr() as usize, p2.as_mut_ptr() as usize);

        (
            unsafe { TRACE_HASH },
            ring_buf,
            s_138,
            s_140,
            rd::<u32>(rsp_ring + 0x30),
            rd::<u32>(new_res.as_ptr() as usize + 0x24),
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<u16>(g_0d8, s_0d8);
    wr::<u64>(g_82b8, s_82b8);
    wr::<usize>(g_ff58, s_ff58);

    crate::diff_eq!(o, n, "141f27160 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14202f140(orig: usize, new: usize) -> Result<u32, String> {
    static mut ERASED_CTRL_OFF: usize = 0;

    unsafe extern "win64" fn mock_erase(table: usize, it_ptr: usize) {
        let ctrl_base = rd::<usize>(table);
        let ctrl_it = rd::<usize>(it_ptr);
        unsafe { ERASED_CTRL_OFF = ctrl_it.wrapping_sub(ctrl_base) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_411b_c750, mock_erase as *const ())?;

    let run = |fn_addr: usize| -> ([f32; 2], [u64; 3], u32, usize) {
        let mut outer_ctrl = [0x80u8; 32];
        outer_ctrl[2] = 0;
        outer_ctrl[3] = 0xff; // end sentinel (-1i8)

        let mut inner_ctrl = [0x80u8; 32];
        inner_ctrl[1] = 0;
        inner_ctrl[3] = 0;
        inner_ctrl[4] = 0xff; // end sentinel (-1i8)

        let mut inner_slots = [0u8; 0x50];
        let isp = inner_slots.as_mut_ptr() as usize;
        // Slot 1: key = 0x111, timer = 1.0 (> 0.25, kept)
        wr::<u64>(isp + 0x10, 0x111);
        wr::<f32>(isp + 0x18, 1.0);
        // Slot 3: key = 0x222, timer = 0.1 (<= 0.25, erased)
        wr::<u64>(isp + 0x30, 0x222);
        wr::<f32>(isp + 0x38, 0.1);

        let mut key_arr: [u64; 3] = [0x111, 0x222, 0x333];

        let mut outer_slots = [0u8; 0x48 * 4];
        let osp = outer_slots.as_mut_ptr() as usize;
        let slot2 = osp + 0x48 * 2;
        wr::<usize>(slot2 + 0x08, key_arr.as_mut_ptr() as usize);
        wr::<u32>(slot2 + 0x10, 3);
        wr::<usize>(slot2 + 0x18, inner_ctrl.as_mut_ptr() as usize);
        wr::<usize>(slot2 + 0x20, isp);
        wr::<usize>(slot2 + 0x30, 4);

        let p1: [usize; 4] = [outer_ctrl.as_mut_ptr() as usize, osp, 0, 3];
        unsafe { ERASED_CTRL_OFF = 999 };

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1.as_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0.25f32.to_bits() as u128, 0, 0],
        );

        (
            [rd::<f32>(isp + 0x18), rd::<f32>(isp + 0x38)],
            key_arr,
            rd::<u32>(slot2 + 0x10),
            unsafe { ERASED_CTRL_OFF },
        )
    };

    let o = run(orig);
    let n = run(new);
    crate::diff_eq!(o, n, "14202f140 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420a3e40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u16, u16) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HEAP_BUF: [u8; 0x200] = [0; 0x200];
    static mut HEAP_OFF: usize = 0;

    unsafe extern "win64" fn mock_alloc(sz: u64, _align: u64) -> usize {
        unsafe {
            let base = core::ptr::addr_of_mut!(HEAP_BUF) as usize;
            let off = HEAP_OFF;
            HEAP_OFF = (off + (sz as usize) + 15) & !15;
            base + off
        }
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    unsafe extern "win64" fn mock_4ee0(ctx: usize, start_node: u16) {
        let buf = mock_alloc(4, 2);
        wr::<u32>(buf, (start_node as u32) | 0x0001_0000);
        let visited = rd::<usize>(ctx + 0x08);
        wr::<u8>(visited + (start_node as usize), 1);
        wr::<usize>(ctx + 0x18, buf);
        wr::<u32>(ctx + 0x20, 1);
        wr::<u32>(ctx + 0x24, 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, noop1 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420a_4ee0, mock_4ee0 as *const ())?;

    let run = |f: F, target_prop: u16| -> (bool, u16, u8) {
        unsafe {
            HEAP_BUF = [0; 0x200];
            HEAP_OFF = 0;
        }

        let mut nodes = [0u8; 0x40 * 3];
        let np = nodes.as_mut_ptr() as usize;
        wr::<u16>(np + 0x20, 100);
        wr::<u16>(np + 0x22, 10);
        wr::<u16>(np + 0x40 + 0x20, 200);
        wr::<u16>(np + 0x40 + 0x22, 20);
        wr::<u16>(np + 0x80 + 0x20, 300);
        wr::<u16>(np + 0x80 + 0x22, 30);

        let p2: [usize; 2] = [0, np];

        let mut sentinel_node = [0u8; 0x18];
        let mut edge_node = [0u8; 0x18];
        let sp = sentinel_node.as_mut_ptr() as usize;
        let ep = edge_node.as_mut_ptr() as usize;
        wr::<usize>(ep, sp);
        wr::<u16>(ep + 0x10, 0); // from node 0
        wr::<u16>(ep + 0x12, 1); // to node 1

        let buckets: [usize; 2] = [ep, ep];
        let adj_targets: [u16; 1] = [2]; // edge to node 2
        let adj_ranges: [u32; 3] = [0, 0x0001_0000, 0]; // node 1 has range [0..1)

        let mut p3 = [0u8; 0x60];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x08, sp);
        wr::<usize>(p3p + 0x18, buckets.as_ptr() as usize);
        wr::<u64>(p3p + 0x30, 0); // bucket mask = 0
        wr::<usize>(p3p + 0x40, adj_targets.as_ptr() as usize);
        wr::<usize>(p3p + 0x50, adj_ranges.as_ptr() as usize);
        wr::<u32>(p3p + 0x58, 3);

        let mut out = [0u8; 4];
        let out_p = out.as_mut_ptr() as usize;
        let ret = f(out_p, p2.as_ptr() as usize, p3p, 0, target_prop);
        (ret == out_p, rd::<u16>(out_p), rd::<u8>(out_p + 2))
    };

    for target in [30u16, 99u16] {
        let o = run(f_o, target);
        let n = run(f_n, target);
        crate::diff_eq!(o, n, "1420a3e40 target={target}");
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_41f2_7160, "unknown_141f27160", rend::cauldron_hot_k::unknown_141f27160 as *const (), Throws::Caught, rend::cauldron_hot_k::diff_unknown_141f27160),
// Replacement::verified(0x1_4202_f140, "unknown_14202f140", rend::cauldron_hot_k::unknown_14202f140 as *const (), rend::cauldron_hot_k::diff_unknown_14202f140),
// Replacement::verified_throws(0x1_420a_3e40, "unknown_1420a3e40", rend::cauldron_hot_k::unknown_1420a3e40 as *const (), Throws::Discard, rend::cauldron_hot_k::diff_unknown_1420a3e40),
