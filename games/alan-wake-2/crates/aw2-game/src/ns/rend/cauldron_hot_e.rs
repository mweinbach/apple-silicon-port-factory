//! Decompiled hot functions shard E (`0x141eb13e0`, `0x141fcc240`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vbroadcastss, vdivps, vdpps, vfmadd_ps, vhaddps, vhsubps,
    vinsertps, vmaxps, vminps, vmulps, vpermilps, vshufps, vsqrtps, vpslldq, vpsrldq, vsubps,
};

/// `unknown_141eb13e0`
/// Original: `0x141eb13e0` (2945 bytes, `Throws::Discard`)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_141eb13e0(
    param_1: usize,
    param_2: usize,
    param_3: f32,
    param_4: usize,
) {
    unknown_141eb13e0_impl(param_1, param_2, param_3, param_4);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_141eb13e0(
    param_1: usize,
    param_2: usize,
    _r8: usize,
    param_4: usize,
    _xmm0: f32,
    _xmm1: f32,
    param_3: f32,
) {
    unknown_141eb13e0_impl(param_1, param_2, param_3, param_4);
}

#[inline(always)]
unsafe fn unknown_141eb13e0_impl(
    param_1: usize,
    param_2: usize,
    param_3: f32,
    param_4: usize,
) {
    let p1_0 = rd::<usize>(param_1);
    let p1_1 = rd::<usize>(param_1 + 0x08);
    let p1_2 = rd::<usize>(param_1 + 0x10);
    let p1_3 = rd::<usize>(param_1 + 0x18);
    let p1_4 = rd::<usize>(param_1 + 0x20);
    let p1_5 = rd::<usize>(param_1 + 0x28);
    let idx = rd::<usize>(param_1 + 0x38);

    let off_18 = idx.wrapping_mul(0x18);
    let rsi_slot = p1_3.wrapping_add(idx.wrapping_mul(0x28));
    let r13_out = p1_4.wrapping_add(idx.wrapping_mul(0x2c));

    if rd::<u32>(p1_1.wrapping_add(idx.wrapping_mul(4))) != 0 {
        return;
    }
    let count_u32 = rd::<u32>(p1_5.wrapping_add(off_18).wrapping_add(0x08));
    if count_u32 == 0 {
        return;
    }

    let rbx = rd::<usize>(param_2);
    let p2_1 = rd::<usize>(param_2 + 0x08);
    let r14 = count_u32 as usize;

    let toggle = rd::<u8>(rsi_slot).wrapping_sub(1) & 1;
    wr::<u8>(rsi_slot, toggle);
    let rsi_vec = rd::<usize>(rsi_slot.wrapping_add((toggle as usize) * 0x10).wrapping_add(0x08));
    exe::call_va!(0x1_40a0_2480, (rsi_vec, r14 as u64));

    let mut xmm10 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7af0) as usize);
    let mut xmm11 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ba0) as usize);

    let mut local_38: usize = 0;
    let b_var11 = if p1_0 == 0 {
        false
    } else {
        let entry = p1_0.wrapping_add(rd::<usize>(param_1 + 0x38) << 4);
        let cnt = rd::<u32>(entry + 0x08) as u64;
        local_38 = rd::<usize>(entry);
        cnt != 0 && (rd::<u16>(rbx + 0x04) as u64) == cnt
    };

    let mesh0 = rd::<usize>(p1_2.wrapping_add(off_18).wrapping_add(0x08));
    let ptr_1a0 = rd::<usize>(mesh0 + 0x1a0)
        .wrapping_add((rd::<u32>(mesh0 + 0x1e8) as usize) << 4);

    if rbx == 0 {
        wr::<u32>(rsi_vec + 0x08, 0);
        exe::call_va!(0x1_40ff_f550, (p1_2.wrapping_add(off_18).wrapping_add(0x10)));
        return;
    }
    let n_bones = rd::<u16>(rbx + 0x04) as usize;
    if r14 > n_bones {
        wr::<u32>(rsi_vec + 0x08, 0);
        exe::call_va!(0x1_40ff_f550, (p1_2.wrapping_add(off_18).wrapping_add(0x10)));
        return;
    }

    let mut buf_rsi: usize = 0;
    if n_bones != 0 {
        buf_rsi = exe::call_va!(0x1_4241_c0a0, ((n_bones << 5) as u64, 0x10u64) -> usize);
        exe::call_va!(0x1_40fb_0e90, (buf_rsi, n_bones as u64));
    }

    let c_1_0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    if param_3 == c_1_0 || rd::<u8>(rbx + 0x06) == rd::<u8>(rbx + 0x07) {
        exe::call_va!(0x1_421d_e560, (rbx, buf_rsi));
    } else {
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_421d_e220,
            rbx as u64,
            buf_rsi as u64,
            param_3.to_bits() as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, param_3.to_bits() as u128, 0],
        );
    }

    let mesh1 = rd::<usize>(p1_2.wrapping_add(off_18).wrapping_add(0x08));
    let sub_idx = rd::<u32>(mesh1 + 0x1e8) as usize;
    let ptr_1b0 = rd::<usize>(mesh1 + 0x1b0).wrapping_add(sub_idx.wrapping_mul(0x30));
    let ptr_1c0 = rd::<usize>(mesh1 + 0x1c0).wrapping_add(sub_idx << 5);

    let c_ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let c_240 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8240) as usize);
    let c_3b0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_83b0) as usize);

    let mut took_masked_branch = false;
    if p2_1 != 0 {
        let mask_bits = rd::<u32>(p2_1) as u64;
        let popcnt = exe::call_va!(0x1_40ff_39e0, (rd::<u64>(p2_1 + 0x08), mask_bits) -> u64);
        if popcnt != mask_bits {
            took_masked_branch = true;
            if r14 != 0 {
                let xmm9: u128 = 0;
                for i in 0..r14 {
                    let indices_arr = rd::<usize>(p1_5.wrapping_add(off_18));
                    let bone_idx = rd::<u32>(indices_arr.wrapping_add(i * 4)) as usize;
                    let bone_off = bone_idx << 5;
                    let ymm7_lo = rd::<u128>(buf_rsi.wrapping_add(bone_off));
                    let ymm7_hi = rd::<u128>(buf_rsi.wrapping_add(bone_off + 0x10));
                    let ymm6_lo = rd::<u128>(ptr_1c0.wrapping_add(i * 0x20));
                    let xmm5 = rd::<u128>(ptr_1c0.wrapping_add(i * 0x20 + 0x10));

                    let xmm0 = vpermilps(xmm5, 0xc9);
                    let xmm4 = vshufps(ymm7_lo, ymm7_lo, 0xd2);
                    let xmm1 = vmulps(xmm0, xmm4);
                    let xmm0 = vpermilps(xmm5, 0xd2);
                    let xmm2 = vshufps(ymm7_lo, ymm7_lo, 0xc9);
                    let xmm0 = vmulps(xmm0, xmm2);
                    let xmm0 = vsubps(xmm0, xmm1);
                    let xmm3 = vmulps(xmm0, c_ab0);
                    let xmm1 = vpermilps(xmm3, 0xc9);
                    let xmm4 = vmulps(xmm1, xmm4);
                    let xmm0 = vpermilps(xmm3, 0xd2);
                    let xmm2 = vmulps(xmm0, xmm2);
                    let xmm0 = vshufps(ymm7_lo, ymm7_lo, 0xff);
                    let xmm1 = vmulps(xmm0, xmm3);
                    let xmm3 = vaddps(xmm1, xmm5);
                    let xmm2 = vsubps(xmm2, xmm4);
                    let xmm4 = vaddps(xmm3, xmm2);
                    let xmm12 = vaddps(xmm4, ymm7_hi);

                    let xmm5 = vpermilps(ymm7_lo, 0x1b);
                    let xmm2 = vpermilps(ymm6_lo, 0x11);
                    let xmm3 = vpermilps(ymm6_lo, 0xbb);
                    let xmm1 = vmulps(xmm3, xmm5);
                    let xmm0 = vmulps(xmm2, ymm7_lo);
                    let xmm4 = vhsubps(xmm0, xmm1);
                    let xmm2 = vmulps(xmm2, xmm5);
                    let xmm0 = vmulps(xmm3, ymm7_lo);
                    let xmm1 = vhaddps(xmm0, xmm2);
                    let xmm3 = vshufps(xmm4, xmm1, 0xb1);
                    let xmm0 = vblendps(xmm1, xmm4, 0x0c);
                    let xmm1 = vaddsubps(xmm0, xmm3);
                    let xmm8 = vpermilps(xmm1, 0x9c);

                    let out_base = rd::<usize>(rsi_vec);
                    let mask_ptr = rd::<usize>(p2_1 + 0x08);
                    let word = rd::<u64>(mask_ptr.wrapping_add((bone_idx >> 6) * 8));
                    let dst_m = out_base.wrapping_add(i * 0x30);
                    if (word & (1u64 << (bone_idx & 0x3f))) != 0 {
                        let xmm7 = c_240;
                        let xmm3 = vmulps(xmm8, c_3b0);
                        let xmm4 = vmulps(xmm8, xmm3);
                        let xmm0 = vpermilps(xmm8, 0xe0);
                        let xmm2 = vmulps(xmm0, c_3b0);
                        let xmm1 = vpermilps(xmm8, 0xd9);
                        let xmm6 = vmulps(xmm2, xmm1);
                        let xmm0 = vpermilps(xmm8, 0xff);
                        let xmm5 = vmulps(xmm0, xmm3);
                        let xmm1 = vpermilps(xmm4, 0xc1);
                        let xmm2 = vsubps(xmm7, xmm1);
                        let xmm0 = vpermilps(xmm4, 0xda);
                        let xmm4 = vsubps(xmm2, xmm0);
                        let xmm1 = vpermilps(xmm6, 0xd8);
                        let xmm0 = vpermilps(xmm5, 0xd2);
                        let xmm3 = vaddps(xmm1, xmm0);
                        let xmm2 = vpermilps(xmm6, 0xe1);
                        let xmm0 = vpermilps(xmm5, 0xc9);
                        let xmm1 = vsubps(xmm2, xmm0);
                        let xmm2 = vshufps(xmm4, xmm3, 0x44);
                        let xmm4 = vshufps(xmm4, xmm3, 0xee);
                        let xmm0 = vshufps(xmm1, xmm9, 0x44);
                        let xmm1 = vshufps(xmm1, xmm9, 0xee);
                        let xmm3 = vshufps(xmm2, xmm0, 0x88);
                        let xmm0 = vshufps(xmm2, xmm0, 0xdd);
                        let xmm2 = vpermilps(xmm0, 0xd2);
                        let xmm1 = vshufps(xmm4, xmm1, 0x88);
                        let xmm5 = vpermilps(xmm1, 0xc9);
                        let xmm4 = vblendps(xmm7, xmm12, 0x07);
                        let xmm0 = vshufps(xmm3, xmm2, 0x4e);
                        let xmm3 = vshufps(xmm3, xmm0, 0x84);
                        let xmm2 = vshufps(xmm2, xmm5, 0x49);
                        let xmm0 = vshufps(xmm5, xmm4, 0x4e);
                        let xmm1 = vshufps(xmm0, xmm4, 0x98);
                        wr::<u128>(dst_m, xmm3);
                        wr::<u128>(dst_m + 0x10, xmm2);
                        wr::<u128>(dst_m + 0x20, xmm1);
                    } else {
                        wr::<u128>(dst_m, xmm9);
                        wr::<u128>(dst_m + 0x10, xmm9);
                        wr::<u128>(dst_m + 0x20, xmm9);
                    }

                    let sphere_ptr = ptr_1a0.wrapping_add(i * 0x10);
                    let xmm6 = vinsertps(
                        rd::<u64>(sphere_ptr) as u128,
                        rd::<u32>(sphere_ptr + 0x08) as u128,
                        0x20,
                    );
                    let xmm0 = vpermilps(xmm6, 0xc9);
                    let xmm4 = vpermilps(xmm8, 0xd2);
                    let xmm2 = vmulps(xmm4, xmm0);
                    let xmm1 = vpermilps(xmm6, 0xd2);
                    let xmm5 = vpermilps(xmm8, 0xc9);
                    let xmm0 = vmulps(xmm5, xmm1);
                    let xmm0 = vsubps(xmm0, xmm2);
                    let xmm3 = vmulps(xmm0, c_ab0);
                    let xmm1 = vpermilps(xmm3, 0xc9);
                    let xmm4 = vmulps(xmm4, xmm1);
                    let xmm0 = vpermilps(xmm3, 0xd2);
                    let xmm2 = vmulps(xmm5, xmm0);
                    let xmm0 = vpermilps(xmm8, 0xff);
                    let xmm1 = vmulps(xmm0, xmm3);
                    let xmm3 = vaddps(xmm1, xmm6);
                    let xmm2 = vsubps(xmm2, xmm4);
                    let xmm0 = vaddps(xmm3, xmm2);
                    let xmm5 = vaddps(xmm0, xmm12);
                    let xmm1 = vbroadcastss(rd::<u32>(sphere_ptr + 0x0c));
                    let xmm0 = vsubps(xmm5, xmm1);
                    xmm10 = vminps(xmm10, xmm0);
                    let xmm0 = vaddps(xmm5, xmm1);
                    xmm11 = vmaxps(xmm11, xmm0);
                }
            }
        }
    }

    if !took_masked_branch {
        if b_var11 {
            if r14 != 0 {
                let mut xmm4 = c_3b0;
                let mut xmm8: u128 = 0;
                for i in 0..r14 {
                    let indices_arr = rd::<usize>(p1_5.wrapping_add(off_18));
                    let bone_idx = rd::<u32>(indices_arr.wrapping_add(i * 4)) as usize;
                    let bone_off = bone_idx << 5;
                    let xmm1 = rd::<u128>(buf_rsi.wrapping_add(bone_off));
                    let xmm7 = c_240;

                    let xmm0 = vmulps(xmm4, xmm1);
                    let xmm5 = vmulps(xmm0, xmm1);
                    let xmm3 = xmm1;
                    let xmm0 = vpermilps(xmm1, 0xe0);
                    let xmm2 = vmulps(xmm0, xmm4);
                    let xmm1 = vpermilps(xmm1, 0xd9);
                    let xmm6 = vmulps(xmm2, xmm1);
                    let xmm1 = vmulps(xmm3, xmm4);
                    let xmm0 = vpermilps(xmm3, 0xff);
                    let xmm4_q = vmulps(xmm1, xmm0);
                    let xmm1 = vpermilps(xmm5, 0xc1);
                    let xmm2 = vsubps(xmm7, xmm1);
                    let xmm0 = vpermilps(xmm5, 0xda);
                    let xmm5 = vsubps(xmm2, xmm0);
                    let xmm1 = vpermilps(xmm4_q, 0xd2);
                    let xmm0 = vpermilps(xmm6, 0xd8);
                    let xmm3 = vaddps(xmm1, xmm0);
                    let xmm2 = vpermilps(xmm4_q, 0xc9);
                    let xmm0 = vpermilps(xmm6, 0xe1);
                    let xmm1 = vsubps(xmm0, xmm2);
                    let xmm6 = vshufps(xmm5, xmm3, 0x44);
                    let xmm2 = vshufps(xmm5, xmm3, 0xee);
                    let xmm4_s = vshufps(xmm1, xmm8, 0x44);
                    let xmm0 = vshufps(xmm1, xmm8, 0xee);
                    let xmm3 = vshufps(xmm6, xmm4_s, 0x88);

                    let xmm15 = rd::<u128>(local_38.wrapping_add(bone_idx * 0x10));
                    let xmm0 = vshufps(xmm2, xmm0, 0x88);
                    let xmm2 = vpermilps(xmm0, 0xc9);
                    let xmm1 = vshufps(xmm15, xmm15, 0xaa);
                    let xmm13 = vmulps(xmm2, xmm1);
                    let xmm0 = vshufps(xmm6, xmm4_s, 0xdd);
                    let xmm2 = vpermilps(xmm0, 0xd2);
                    let xmm1 = vshufps(xmm15, xmm15, 0x55);
                    let xmm12 = vmulps(xmm2, xmm1);
                    let xmm0 = vshufps(xmm15, xmm15, 0x00);
                    let xmm10_scale = vmulps(xmm0, xmm3);

                    let m_off = i * 0x30;
                    let xmm3 = rd::<u128>(ptr_1b0.wrapping_add(m_off));
                    let xmm1 = rd::<u128>(ptr_1b0.wrapping_add(m_off + 0x10));
                    let xmm2 = rd::<u128>(ptr_1b0.wrapping_add(m_off + 0x20));
                    let xmm0 = vshufps(xmm3, xmm1, 0x4f);
                    let xmm4_m = vpermilps(xmm0, 0xf9);
                    let xmm5 = vshufps(xmm1, xmm2, 0x0e);
                    let xmm0 = vshufps(xmm2, xmm7, 0x0f);
                    let xmm8_col = vshufps(xmm2, xmm0, 0xd9);
                    let xmm6 = vpsrldq(vpslldq(xmm3, 4), 4);
                    let xmm3 = vpsrldq(vpslldq(xmm4_m, 4), 4);
                    let xmm7_c = vpsrldq(vpslldq(xmm5, 4), 4);

                    let xmm2 = vpermilps(xmm6, 0x00);
                    let xmm5 = vpermilps(xmm3, 0x00);
                    let xmm1 = vpermilps(xmm6, 0x55);
                    let xmm4_c = vpermilps(xmm3, 0x55);
                    let xmm0 = vpermilps(xmm6, 0xaa);
                    let xmm3 = vpermilps(xmm3, 0xaa);

                    let mut xmm11_col = vmulps(xmm10_scale, xmm2);
                    xmm11_col = vfmadd_ps(xmm12, xmm1, xmm11_col, false, false);
                    xmm11_col = vfmadd_ps(xmm13, xmm0, xmm11_col, false, false);

                    let mut xmm9_col = vmulps(xmm10_scale, xmm5);
                    xmm9_col = vfmadd_ps(xmm12, xmm4_c, xmm9_col, false, false);
                    xmm9_col = vfmadd_ps(xmm13, xmm3, xmm9_col, false, false);

                    let xmm2 = vpermilps(xmm7_c, 0x00);
                    let xmm4_8 = vpermilps(xmm8_col, 0x00);
                    let xmm1 = vpermilps(xmm7_c, 0x55);
                    let xmm3 = vpermilps(xmm8_col, 0x55);
                    let xmm0 = vpermilps(xmm7_c, 0xaa);

                    let mut xmm5_col = vmulps(xmm10_scale, xmm2);
                    xmm5_col = vfmadd_ps(xmm12, xmm1, xmm5_col, false, false);
                    xmm5_col = vfmadd_ps(xmm13, xmm0, xmm5_col, false, false);

                    let xmm0 = vpermilps(xmm8_col, 0xaa);
                    let mut xmm6_col = vblendps(
                        c_240,
                        rd::<u128>(buf_rsi.wrapping_add(bone_off + 0x10)),
                        0x07,
                    );
                    xmm6_col = vfmadd_ps(xmm10_scale, xmm4_8, xmm6_col, false, false);
                    xmm6_col = vfmadd_ps(xmm12, xmm3, xmm6_col, false, false);
                    xmm6_col = vfmadd_ps(xmm13, xmm0, xmm6_col, false, false);

                    let out_base = rd::<usize>(rsi_vec);
                    let xmm0 = vshufps(xmm11_col, xmm9_col, 0x4e);
                    let xmm3 = vshufps(xmm11_col, xmm0, 0x84);
                    let xmm2 = vshufps(xmm9_col, xmm5_col, 0x49);
                    let xmm0 = vshufps(xmm5_col, xmm6_col, 0x4e);
                    let xmm1 = vshufps(xmm0, xmm6_col, 0x98);
                    wr::<u128>(out_base.wrapping_add(m_off), xmm3);
                    wr::<u128>(out_base.wrapping_add(m_off + 0x10), xmm2);
                    wr::<u128>(out_base.wrapping_add(m_off + 0x20), xmm1);

                    let sphere_ptr = ptr_1a0.wrapping_add(i * 0x10);
                    let xmm0 = vbroadcastss(rd::<u32>(sphere_ptr + 0x08));
                    xmm6_col = vfmadd_ps(xmm0, xmm5_col, xmm6_col, false, false);
                    let xmm0 = vbroadcastss(rd::<u32>(sphere_ptr + 0x04));
                    xmm6_col = vfmadd_ps(xmm0, xmm9_col, xmm6_col, false, false);
                    let xmm0 = vbroadcastss(rd::<u32>(sphere_ptr));
                    xmm6_col = vfmadd_ps(xmm0, xmm11_col, xmm6_col, false, false);

                    let xmm0 = vshufps(xmm15, xmm15, 0x4e);
                    let xmm2 = vmaxps(xmm15, xmm0);
                    let xmm1 = vpermilps(xmm2, 0xb1);
                    let xmm0 = vmaxps(xmm2, xmm1);
                    let rad = f32::from_bits(xmm0 as u32) * rd::<f32>(sphere_ptr + 0x0c);
                    let xmm1 = vbroadcastss(rad.to_bits());
                    let xmm0 = vsubps(xmm6_col, xmm1);
                    xmm10 = vminps(xmm10, xmm0);
                    let xmm0 = vaddps(xmm1, xmm6_col);
                    xmm11 = vmaxps(xmm11, xmm0);

                    xmm4 = c_3b0;
                    xmm8 = 0;
                }
            }
        } else if r14 != 0 {
            let xmm12 = c_ab0;
            let xmm13 = c_240;
            let xmm14 = c_3b0;
            let xmm15: u128 = 0;
            for i in 0..r14 {
                let indices_arr = rd::<usize>(p1_5.wrapping_add(off_18));
                let bone_idx = rd::<u32>(indices_arr.wrapping_add(i * 4)) as usize;
                let bone_off = bone_idx << 5;
                let ymm7_lo = rd::<u128>(buf_rsi.wrapping_add(bone_off));
                let ymm7_hi = rd::<u128>(buf_rsi.wrapping_add(bone_off + 0x10));
                let ymm6_lo = rd::<u128>(ptr_1c0.wrapping_add(i * 0x20));
                let xmm5 = rd::<u128>(ptr_1c0.wrapping_add(i * 0x20 + 0x10));

                let xmm0 = vpermilps(xmm5, 0xc9);
                let xmm4 = vshufps(ymm7_lo, ymm7_lo, 0xd2);
                let xmm1 = vmulps(xmm4, xmm0);
                let xmm0 = vpermilps(xmm5, 0xd2);
                let xmm2 = vshufps(ymm7_lo, ymm7_lo, 0xc9);
                let xmm0 = vmulps(xmm2, xmm0);
                let xmm0 = vsubps(xmm0, xmm1);
                let xmm3 = vmulps(xmm0, xmm12);
                let xmm1 = vpermilps(xmm3, 0xc9);
                let xmm4 = vmulps(xmm1, xmm4);
                let xmm0 = vpermilps(xmm3, 0xd2);
                let xmm2 = vmulps(xmm0, xmm2);
                let xmm0 = vshufps(ymm7_lo, ymm7_lo, 0xff);
                let xmm1 = vmulps(xmm0, xmm3);
                let xmm3 = vaddps(xmm1, xmm5);
                let xmm2 = vsubps(xmm2, xmm4);
                let xmm4 = vaddps(xmm3, xmm2);
                let xmm9 = vaddps(xmm4, ymm7_hi);

                let xmm5 = vpermilps(ymm7_lo, 0x1b);
                let xmm2 = vpermilps(ymm6_lo, 0x11);
                let xmm3 = vpermilps(ymm6_lo, 0xbb);
                let xmm1 = vmulps(xmm3, xmm5);
                let xmm0 = vmulps(xmm2, ymm7_lo);
                let xmm4 = vhsubps(xmm0, xmm1);
                let xmm2 = vmulps(xmm2, xmm5);
                let xmm0 = vmulps(xmm3, ymm7_lo);
                let xmm1 = vhaddps(xmm0, xmm2);
                let xmm3 = vshufps(xmm4, xmm1, 0xb1);
                let xmm0 = vblendps(xmm1, xmm4, 0x0c);
                let xmm1 = vaddsubps(xmm0, xmm3);
                let xmm7 = vpermilps(xmm1, 0x9c);

                let xmm3 = vmulps(xmm7, xmm14);
                let xmm5 = vmulps(xmm3, xmm7);
                let xmm0 = vpermilps(xmm7, 0xe0);
                let xmm2 = vmulps(xmm0, xmm14);
                let xmm1 = vpermilps(xmm7, 0xd9);
                let xmm6 = vmulps(xmm2, xmm1);
                let xmm8 = vpermilps(xmm7, 0xff);
                let xmm4 = vmulps(xmm3, xmm8);
                let xmm0 = vpermilps(xmm5, 0xc1);
                let xmm2 = vsubps(xmm13, xmm0);
                let xmm1 = vpermilps(xmm5, 0xda);
                let xmm5 = vsubps(xmm2, xmm1);
                let xmm3 = vpermilps(xmm4, 0xd2);
                let xmm0 = vpermilps(xmm6, 0xd8);
                let xmm2 = vaddps(xmm3, xmm0);
                let xmm1 = vpermilps(xmm4, 0xc9);
                let xmm0 = vpermilps(xmm6, 0xe1);
                let xmm3 = vsubps(xmm0, xmm1);
                let xmm4 = vshufps(xmm5, xmm2, 0x44);
                let xmm5 = vshufps(xmm5, xmm2, 0xee);
                let xmm0 = vshufps(xmm3, xmm15, 0x44);
                let xmm1 = vshufps(xmm3, xmm15, 0xee);
                let xmm3 = vshufps(xmm4, xmm0, 0x88);
                let xmm0 = vshufps(xmm4, xmm0, 0xdd);
                let xmm2 = vpermilps(xmm0, 0xd2);
                let xmm1 = vshufps(xmm5, xmm1, 0x88);
                let xmm4 = vpermilps(xmm1, 0xc9);
                let xmm5 = vblendps(xmm13, xmm9, 0x07);

                let out_base = rd::<usize>(rsi_vec);
                let xmm0 = vshufps(xmm3, xmm2, 0x4e);
                let xmm3 = vshufps(xmm3, xmm0, 0x84);
                let xmm2 = vshufps(xmm2, xmm4, 0x49);
                let xmm0 = vshufps(xmm4, xmm5, 0x4e);
                let xmm1 = vshufps(xmm0, xmm5, 0x98);
                let dst_m = out_base.wrapping_add(i * 0x30);
                wr::<u128>(dst_m, xmm3);
                wr::<u128>(dst_m + 0x10, xmm2);
                wr::<u128>(dst_m + 0x20, xmm1);

                let sphere_ptr = ptr_1a0.wrapping_add(i * 0x10);
                let xmm6 = vinsertps(
                    rd::<u64>(sphere_ptr) as u128,
                    rd::<u32>(sphere_ptr + 0x08) as u128,
                    0x20,
                );
                let xmm0 = vpermilps(xmm6, 0xc9);
                let xmm3 = vpermilps(xmm7, 0xd2);
                let xmm2 = vmulps(xmm3, xmm0);
                let xmm1 = vpermilps(xmm6, 0xd2);
                let xmm5 = vpermilps(xmm7, 0xc9);
                let xmm0 = vmulps(xmm5, xmm1);
                let xmm0 = vsubps(xmm0, xmm2);
                let xmm4 = vmulps(xmm0, xmm12);
                let xmm1 = vpermilps(xmm4, 0xc9);
                let xmm3 = vmulps(xmm3, xmm1);
                let xmm0 = vpermilps(xmm4, 0xd2);
                let xmm1 = vmulps(xmm5, xmm0);
                let xmm0 = vmulps(xmm4, xmm8);
                let xmm2 = vaddps(xmm0, xmm6);
                let xmm1 = vsubps(xmm1, xmm3);
                let xmm2 = vaddps(xmm2, xmm1);
                let xmm4 = vaddps(xmm2, xmm9);
                let xmm1 = vbroadcastss(rd::<u32>(sphere_ptr + 0x0c));
                let xmm0 = vsubps(xmm4, xmm1);
                xmm10 = vminps(xmm10, xmm0);
                let xmm0 = vaddps(xmm1, xmm4);
                xmm11 = vmaxps(xmm11, xmm0);
            }
        }
    }

    let c_120 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize);
    let c_780 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let xmm0 = vaddps(xmm11, xmm10);
    let xmm3 = vmulps(xmm0, c_120);
    let xmm1 = vsubps(xmm11, xmm10);
    let xmm0 = vdpps(xmm1, xmm1, 0x7f);
    let xmm2 = vsqrtps(xmm0);
    let half_diag = (f32::from_bits(xmm2 as u32) * c_780).to_bits();

    let r9d = (xmm3 >> 64) as u32;
    let r8d = (xmm10 >> 64) as u32;
    let edx = (xmm11 >> 64) as u32;

    wr::<u64>(r13_out, xmm3 as u64);
    wr::<u32>(r13_out + 0x08, r9d);
    wr::<u32>(r13_out + 0x0c, half_diag);
    wr::<u64>(r13_out + 0x10, xmm10 as u64);
    wr::<u32>(r13_out + 0x18, r8d);
    wr::<u64>(r13_out + 0x1c, xmm11 as u64);
    wr::<u32>(r13_out + 0x24, edx);
    wr::<u8>(r13_out + 0x28, 1);

    let out_rcx = rd::<usize>(param_4 + 0x10);
    wr::<usize>(param_4 + 0x10, out_rcx + 0x2c);
    let id = rd::<u32>(p1_2.wrapping_add(off_18).wrapping_add(0x10));
    wr::<u32>(out_rcx, id);
    wr::<u64>(out_rcx + 0x04, xmm3 as u64);
    wr::<u32>(out_rcx + 0x0c, r9d);
    wr::<u32>(out_rcx + 0x10, half_diag);
    wr::<u64>(out_rcx + 0x14, xmm10 as u64);
    wr::<u32>(out_rcx + 0x1c, r8d);
    wr::<u64>(out_rcx + 0x20, xmm11 as u64);
    wr::<u32>(out_rcx + 0x28, edx);

    if buf_rsi != 0 {
        exe::call_va!(0x1_4241_c190, (buf_rsi));
    }
}

/// `unknown_141fcc240`
/// Original: `0x141fcc240` (4938 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_141fcc240(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut scratch_vec = [0u64; 2];
    let r11 = rd::<u64>(param_1).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            span.as_ptr() as usize
        ) -> usize
    );

    let mut outer = [0u64; 21];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4115_da90, (outer.as_mut_ptr() as usize));

    if outer[6] < outer[5] {
        let c_1_0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let c_0_25 = rd::<f32>(exe::ptr::<f32>(0x1_4342_668c) as usize);
        let mut xmm6_saved: u128 = 0;

        while outer[6] < outer[5] {
            let r14 = outer[6] as usize;
            let r15 = (outer[7] as usize).wrapping_add(r14 * 0x20);
            let rbx = (outer[8] as usize).wrapping_add(r14 * 8);
            let r13 = (outer[9] as usize).wrapping_add(r14 * 0x40);
            let ptr_280 = (outer[10] as usize).wrapping_add(r14 * 4);

            if rd::<u32>(ptr_280) != 0 {
                exe::call_va!(0x1_41fc_e3b0, (r15));
            } else {
                let mut ptr_90: usize = 0;
                let mut flag_970: u8 = 0;
                let s_1a0 = outer[14] as usize;
                if s_1a0 != 0 {
                    ptr_90 = s_1a0.wrapping_add(r14 * 0x18);
                    flag_970 = rd::<u8>(ptr_90 + 0x11);
                }

                let r9_vec = rd::<usize>(r15 + 0x10);
                let rdx_end = r9_vec.wrapping_add((rd::<u32>(r15 + 0x18) as usize) << 5);
                let r15_0 = rd::<usize>(r15);
                let rax_end = r15_0.wrapping_add((rd::<u32>(r15 + 0x08) as usize) << 5);
                let active = (r15_0 != rax_end)
                    || (ptr_90 != 0 && rd::<u32>(ptr_90 + 0x08) > 0)
                    || (r9_vec != rdx_end);

                if active {
                    let mut opt_70 = [0u8; 0x20];
                    let s_1c0 = outer[18] as usize;
                    if s_1c0 != 0 {
                        let src_1c0 = s_1c0.wrapping_add(r14 * 0x18);
                        opt_70[0] = rd::<u8>(src_1c0);
                        opt_70[1] = rd::<u8>(src_1c0 + 1);
                        opt_70[2] = rd::<u8>(src_1c0 + 2);
                        exe::call_va!(
                            0x1_4116_30f0,
                            (opt_70.as_mut_ptr() as usize + 0x08, src_1c0 + 0x08)
                        );
                        opt_70[0x18] = 1;
                    }

                    let mut flag_b0: u32 = 0;
                    let s_1c8 = outer[19] as usize;
                    if s_1c8 != 0 {
                        xmm6_saved = rd::<u128>(s_1c8.wrapping_add(r14 * 0x10));
                        flag_b0 = 1;
                    }

                    let r12_entry: usize;
                    let ptr_98: usize;

                    if rd::<u64>(rbx) == 0 {
                        let old_id = crate::simd_ops::atomic_xadd_u64(
                            exe::ptr::<u64>(0x1_43a4_1c58) as u64,
                            1,
                        );
                        let new_id = old_id.wrapping_add(1);
                        wr::<u64>(rbx, new_id);
                        exe::call_va!(0x1_4158_60a0, (new_id));

                        let mut tmp_3e0 = [0u8; 0x208];
                        let p3e0 = tmp_3e0.as_mut_ptr() as usize;
                        wr::<u16>(p3e0 + 0x18c, 1);
                        let node_rax = exe::call_va!(0x1_4024_d8b0, () -> usize);
                        wr::<usize>(node_rax, node_rax);
                        wr::<usize>(node_rax + 0x08, node_rax);
                        wr::<usize>(p3e0 + 0x1a8, node_rax);
                        wr::<u64>(p3e0 + 0x1d0, 7);
                        wr::<u64>(p3e0 + 0x1d8, 8);
                        wr::<f32>(p3e0 + 0x1a0, c_1_0);
                        exe::call_va!(0x1_401e_5730, (p3e0 + 0x1b8, 0x10u64, node_rax));
                        wr::<u8>(p3e0 + 0x1f8, 0xff);

                        let cur_idx = outer[6] as usize;
                        let chunk_ptr = outer[4] as usize;
                        let key_1e0 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                        r12_entry = exe::call_va!(
                            0x1_4116_0760,
                            (param_3, key_1e0.as_ptr() as usize, p3e0) -> usize
                        );

                        let p_5c0 = rd::<usize>(p3e0 + 0x1e0);
                        if p_5c0 != 0 {
                            crate::ns::r::mem::region_free(p_5c0 as *mut u8);
                        }
                        wr::<u64>(p3e0 + 0x1e0, 0);
                        wr::<u64>(p3e0 + 0x1e8, 0);

                        let p_598 = rd::<usize>(p3e0 + 0x1b8);
                        if p_598 != 0 {
                            crate::ns::r::mem::region_free(p_598 as *mut u8);
                            wr::<u128>(p3e0 + 0x1b8, 0);
                            wr::<u64>(p3e0 + 0x1c8, 0);
                        }

                        let head = rd::<usize>(p3e0 + 0x1a8);
                        let prev = rd::<usize>(head + 0x08);
                        wr::<u64>(prev, 0);
                        let mut cur_node = rd::<usize>(head);
                        while cur_node != 0 {
                            let next_node = rd::<usize>(cur_node);
                            exe::call_va!(0x1_401d_e830, (cur_node + 0x18));
                            exe::call_va!(0x1_401e_66f0, (cur_node + 0x18, cur_node));
                            cur_node = next_node;
                        }
                        let head_after = rd::<usize>(p3e0 + 0x1a8);
                        crate::ns::r::mem::region_free(head_after as *mut u8);

                        let mut sp_2e0 = [0u64; 2];
                        let g_8420 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_8420) as usize);
                        exe::call_va!(
                            0x1_424c_d080,
                            (g_8420, sp_2e0.as_mut_ptr() as usize, rd::<u64>(rbx))
                        );

                        let cur_idx = outer[6] as usize;
                        let chunk_ptr = outer[4] as usize;
                        let key_1f0 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                        ptr_98 = exe::call_va!(
                            0x1_4116_0960,
                            (
                                param_3,
                                key_1f0.as_ptr() as usize,
                                sp_2e0.as_mut_ptr() as usize
                            ) -> usize
                        );
                        let ref_ctrl = sp_2e0[1] as usize;
                        if ref_ctrl != 0 {
                            let old_use = crate::simd_ops::atomic_xadd_u32(
                                (ref_ctrl + 0x08) as u64,
                                u32::MAX,
                            );
                            if old_use == 1 {
                                let vtbl = rd::<usize>(ref_ctrl);
                                exe::call_va!(rd::<usize>(vtbl), (ref_ctrl));
                                let old_weak = crate::simd_ops::atomic_xadd_u32(
                                    (ref_ctrl + 0x0c) as u64,
                                    u32::MAX,
                                );
                                if old_weak == 1 {
                                    let vtbl2 = rd::<usize>(ref_ctrl);
                                    exe::call_va!(rd::<usize>(vtbl2 + 0x08), (ref_ctrl));
                                }
                            }
                        }

                        let cur_idx = outer[6] as usize;
                        let chunk_ptr = outer[4] as usize;
                        let mut zero_380 = 0u128;
                        let key_200 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                        let vel_dst = exe::call_va!(
                            0x1_4116_0b60,
                            (
                                param_3,
                                key_200.as_ptr() as usize,
                                (&mut zero_380 as *mut u128) as usize
                            ) -> usize
                        );

                        let mut pos_390 = rd::<u128>(r13 + 0x10);
                        let mut vel_250 = vsubps(pos_390, rd::<u128>(r13 + 0x30));
                        let dt = rd::<f32>(param_2);
                        if dt > 0.0f32 {
                            vel_250 = vdivps(vel_250, vbroadcastss(dt.to_bits()));
                        }
                        wr::<u128>(vel_dst, vel_250);
                        exe::call_va!(
                            0x1_41fc_e480,
                            (
                                r15,
                                ptr_90,
                                (&mut vel_250 as *mut u128) as usize,
                                (&mut vel_250 as *mut u128) as usize,
                                (&mut pos_390 as *mut u128) as usize,
                                dt.to_bits() as u64
                            )
                        );

                        let mut xmm8_val = c_0_25.to_bits();
                        let mut r11b: u8 = 0;
                        let mut r8b_50: u8 = 0;
                        let s_1b0 = outer[16] as usize;
                        if s_1b0 != 0 {
                            let p_1b0 = s_1b0.wrapping_add(cur_idx * 0x10);
                            if p_1b0 != 0 {
                                xmm8_val = rd::<u32>(p_1b0 + 0x0c);
                                r11b = rd::<u8>(p_1b0 + 0x08);
                                r8b_50 = rd::<u8>(p_1b0 + 0x09);
                                let s_1b8 = outer[17] as usize;
                                if s_1b8 != 0 {
                                    let p_1b8 = s_1b8.wrapping_add(cur_idx * 0x148);
                                    if p_1b8 != 0 {
                                        let v18 = rd::<f32>(p_1b8 + 0x18);
                                        if 0.0f32 < v18 {
                                            xmm8_val = v18.to_bits();
                                        }
                                    }
                                }
                            }
                        }

                        wr::<u64>(r12_entry + 0x18, pos_390 as u64);
                        wr::<u32>(r12_entry + 0x20, (pos_390 >> 64) as u32);
                        wr::<u64>(r12_entry + 0x24, pos_390 as u64);
                        wr::<u32>(r12_entry + 0x2c, (pos_390 >> 64) as u32);

                        let xmm7 = rd::<u128>(exe::ptr::<u8>(0x1_433e_4530) as usize);
                        let xmm6 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
                        let xmm3 = rd::<u128>(r13);
                        let xmm1 = vpermilps(xmm3, 0xe0);
                        let xmm0 = vpermilps(xmm3, 0xd1);
                        let xmm1 = vmulps(xmm1, xmm0);
                        let xmm4 = vmulps(xmm1, xmm6);
                        let xmm2 = vpermilps(xmm3, 0xfb);
                        let xmm0 = vpermilps(xmm3, 0xca);
                        let xmm1 = vmulps(xmm2, xmm0);
                        let xmm3 = vmulps(xmm1, xmm6);
                        let xmm2 = vaddps(xmm3, xmm4);
                        let xmm1 = vsubps(xmm7, xmm2);
                        let xmm0 = vsubps(xmm4, xmm3);
                        let xmm1 = vshufps(xmm0, xmm1, 0x50);
                        let xmm4 = vshufps(xmm1, xmm2, 0xe8);
                        wr::<u64>(r12_entry + 0x3c, xmm4 as u64);
                        wr::<u32>(r12_entry + 0x44, (xmm4 >> 64) as u32);

                        let xmm3 = rd::<u128>(r13);
                        let xmm1 = vpermilps(xmm3, 0xca);
                        let xmm0 = vpermilps(xmm3, 0xc4);
                        let xmm1 = vmulps(xmm1, xmm0);
                        let xmm5 = vmulps(xmm1, xmm6);
                        let xmm2 = vpermilps(xmm3, 0xdf);
                        let xmm0 = vpermilps(xmm3, 0xd1);
                        let xmm1 = vmulps(xmm2, xmm0);
                        let xmm3 = vmulps(xmm1, xmm6);
                        let xmm4 = vaddps(xmm3, xmm5);
                        let xmm0 = vsubps(xmm5, xmm3);
                        let xmm2 = vshufps(xmm4, xmm0, 0x50);
                        let xmm1 = vsubps(xmm7, xmm4);
                        let xmm3 = vshufps(xmm2, xmm1, 0xe8);
                        wr::<u64>(r12_entry + 0x30, xmm3 as u64);
                        wr::<u32>(r12_entry + 0x38, (xmm3 >> 64) as u32);

                        let mut xform_5f0 = [rd::<u128>(r13), rd::<u128>(r13 + 0x10)];
                        if flag_970 != 0 && outer[20] != 0 {
                            let xmm1 = rd::<u128>(r13 + 0x10);
                            let xmm0 = vinsertps(
                                0,
                                rd::<u32>(exe::ptr::<u32>(0x1_4385_63a8) as usize) as u128,
                                0x10,
                            );
                            xform_5f0[1] = vaddps(xmm0, xmm1);
                        }

                        let tbl_580e0 = rd::<usize>(param_4 + 0x580e0);
                        let slot_63c =
                            rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
                        let r10_arg = rd::<u64>(rd::<usize>(tbl_580e0.wrapping_add(slot_63c * 8)));
                        let slot_948 =
                            rd::<u16>(exe::ptr::<u16>(0x1_43cc_8948) as usize) as usize;
                        let rcx_arg =
                            rd::<usize>(tbl_580e0.wrapping_add(slot_948 * 8)).wrapping_add(0x90);

                        let mut opt_320 = [0u8; 0x18];
                        wr::<u128>(opt_320.as_mut_ptr() as usize, xmm6_saved);
                        wr::<u32>(opt_320.as_mut_ptr() as usize + 0x10, flag_b0);

                        exe::call_va!(
                            0x1_41fc_fce0,
                            (
                                rcx_arg,
                                r10_arg,
                                rd::<u64>(rbx),
                                xform_5f0.as_mut_ptr() as usize,
                                r12_entry + 0x08,
                                r12_entry + 0x1e0,
                                xmm8_val as u64,
                                r11b as u64,
                                r8b_50 as u64,
                                opt_320.as_mut_ptr() as usize
                            )
                        );

                        wr::<u32>(r12_entry + 0x04, 0);
                        wr::<u32>(r12_entry, 0xffff_ffff);
                        let _ = crate::simd_ops::atomic_add_u64(
                            exe::ptr::<u64>(0x1_43a3_4608) as u64,
                            1,
                        );

                        if flag_970 != 0 && outer[20] != 0 && rd::<u8>(r12_entry + 0x18d) == 0 {
                            let xmm1 = rd::<u128>(r13 + 0x10);
                            wr::<u64>(r12_entry + 0x24, xmm1 as u64);
                            wr::<u32>(r12_entry + 0x2c, (xmm1 >> 64) as u32);
                        }

                        let mut opt_260 = [0u8; 0x20];
                        if opt_70[0x18] != 0 {
                            opt_260[0] = opt_70[0];
                            opt_260[1] = opt_70[1];
                            opt_260[2] = opt_70[2];
                            exe::call_va!(
                                0x1_4116_30f0,
                                (
                                    opt_260.as_mut_ptr() as usize + 0x08,
                                    opt_70.as_ptr() as usize + 0x08
                                )
                            );
                            opt_260[0x18] = 1;
                        }

                        let mut opt_340 = [0u8; 0x18];
                        wr::<u128>(opt_340.as_mut_ptr() as usize, xmm6_saved);
                        wr::<u32>(opt_340.as_mut_ptr() as usize + 0x10, flag_b0);

                        let _ = crate::simd_ops::call_va8_xmm(
                            0x1_41fc_f6c0,
                            rd::<u64>(rbx),
                            r13 as u64,
                            r12_entry as u64,
                            dt.to_bits() as u64,
                            opt_260.as_mut_ptr() as u64,
                            opt_340.as_mut_ptr() as u64,
                            r8b_50 as u64,
                            0,
                            [0, 0, 0, dt.to_bits() as u128],
                        );
                    } else {
                        let base_580f0 = param_4.wrapping_add(0x580f0);
                        let cur_idx = outer[6] as usize;
                        if outer[12] != 0 {
                            ptr_98 = (outer[12] as usize).wrapping_add(cur_idx * 0x10);
                        } else {
                            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                            let worker_ctx = base_580f0.wrapping_add(wid * 0x188);
                            let mut sp_2b8 = [0u64; 2];
                            let g_8420 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_8420) as usize);
                            exe::call_va!(
                                0x1_424c_d080,
                                (g_8420, sp_2b8.as_mut_ptr() as usize, rd::<u64>(rbx))
                            );
                            let chunk_ptr = outer[4] as usize;
                            let key_210 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                            ptr_98 = exe::call_va!(
                                0x1_4116_0960,
                                (
                                    worker_ctx,
                                    key_210.as_ptr() as usize,
                                    sp_2b8.as_mut_ptr() as usize
                                ) -> usize
                            );
                            exe::call_va!(0x1_4020_0d90, (sp_2b8.as_mut_ptr() as usize));
                        }

                        if outer[11] != 0 {
                            r12_entry = (outer[11] as usize).wrapping_add(cur_idx * 0x208);
                        } else {
                            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                            let worker_ctx = base_580f0.wrapping_add(wid * 0x188);
                            let mut tmp_610 = [0u8; 0x208];
                            let p610 = tmp_610.as_mut_ptr() as usize;
                            wr::<u16>(p610 + 0x18c, 1);
                            exe::call_va!(0x1_4107_5c80, (p610 + 0x1a0));
                            wr::<u8>(p610 + 0x1f8, 0xff);
                            let chunk_ptr = outer[4] as usize;
                            let key_220 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                            r12_entry = exe::call_va!(
                                0x1_4116_0760,
                                (worker_ctx, key_220.as_ptr() as usize, p610) -> usize
                            );
                            exe::call_va!(0x1_401d_e830, (p610 + 0x1e0));
                            exe::call_va!(0x1_401e_5380, (p610 + 0x1b8));
                            exe::call_va!(0x1_405e_ee80, (p610 + 0x1a8));
                        }

                        if outer[13] == 0 {
                            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                            let worker_ctx = base_580f0.wrapping_add(wid * 0x188);
                            let mut zero_3a0 = 0u128;
                            let chunk_ptr = outer[4] as usize;
                            let key_230 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                            let vel_dst = exe::call_va!(
                                0x1_4116_0b60,
                                (
                                    worker_ctx,
                                    key_230.as_ptr() as usize,
                                    (&mut zero_3a0 as *mut u128) as usize
                                ) -> usize
                            );
                            let mut vel = vsubps(rd::<u128>(r13 + 0x10), rd::<u128>(r13 + 0x30));
                            let dt = rd::<f32>(param_2);
                            if dt > 0.0f32 {
                                vel = vdivps(vel, vbroadcastss(dt.to_bits()));
                            }
                            wr::<u128>(vel_dst, vel);
                        }
                        wr::<u32>(r12_entry + 0x04, 0x000f_4240);
                    }

                    let cur_idx = outer[6] as usize;
                    let s_1a8 = outer[15] as usize;
                    if s_1a8 != 0 {
                        wr::<u32>(r12_entry + 0x190, rd::<u32>(s_1a8 + cur_idx * 4));
                    }

                    wr::<u32>(scratch_vec.as_mut_ptr() as usize + 0x08, 0);
                    exe::call_va!(0x1_41fc_eee0, (r15, rbx, r12_entry));

                    let cnt_18 = rd::<u32>(r15 + 0x18) as u64;
                    let ptr_10 = rd::<u64>(r15 + 0x10);
                    if cnt_18 != 0 {
                        let mut span_3b0 = [cnt_18, ptr_10];
                        exe::call_va!(
                            0x1_41fd_1a70,
                            (
                                span_3b0.as_mut_ptr() as usize,
                                scratch_vec.as_mut_ptr() as usize
                            )
                        );
                        wr::<u32>(r15 + 0x18, 0);
                    }

                    let cnt_08 = rd::<u32>(r15 + 0x08) as u64;
                    let ptr_00 = rd::<u64>(r15);
                    let mut span_3c0 = [cnt_08, ptr_00];
                    exe::call_va!(
                        0x1_41fd_1a70,
                        (
                            span_3c0.as_mut_ptr() as usize,
                            scratch_vec.as_mut_ptr() as usize
                        )
                    );

                    let mut cur_item = rd::<usize>(r15);
                    let end_item = cur_item.wrapping_add((rd::<u32>(r15 + 0x08) as usize) << 5);
                    while cur_item != end_item {
                        if (rd::<u8>(cur_item + 0x01) & 1) != 0 {
                            exe::call_va!(0x1_4158_bb70, (rd::<u64>(cur_item + 0x10)));
                        }
                        cur_item = cur_item.wrapping_add(0x20);
                    }

                    if ptr_90 != 0 {
                        let flag_val: u32 = if rd::<u32>(ptr_280) != 0 {
                            1
                        } else {
                            0xffff_ffff
                        };
                        let mut opt_290 = [0u8; 0x20];
                        if opt_70[0x18] != 0 {
                            opt_290[0] = opt_70[0];
                            opt_290[1] = opt_70[1];
                            opt_290[2] = opt_70[2];
                            exe::call_va!(
                                0x1_4116_30f0,
                                (
                                    opt_290.as_mut_ptr() as usize + 0x08,
                                    opt_70.as_ptr() as usize + 0x08
                                )
                            );
                            opt_290[0x18] = 1;
                        }
                        let mut opt_360 = [0u8; 0x18];
                        wr::<u128>(opt_360.as_mut_ptr() as usize, xmm6_saved);
                        wr::<u32>(opt_360.as_mut_ptr() as usize + 0x10, flag_b0);
                        exe::call_va!(
                            0x1_41fc_f1f0,
                            (
                                ptr_90,
                                r12_entry,
                                opt_290.as_mut_ptr() as usize,
                                opt_360.as_mut_ptr() as usize,
                                flag_val
                            )
                        );
                    }
                    wr::<u32>(r15 + 0x08, 0);

                    let lock_obj = rd::<usize>(ptr_98);
                    let scratch_len = rd::<u32>(scratch_vec.as_ptr() as usize + 0x08) as u64;
                    let delta = ((scratch_len * 4) as i64 >> 2) as u32;
                    let mut backoff = 100u64;
                    loop {
                        while core::sync::atomic::AtomicU8::from_ptr((lock_obj + 0x04) as *mut u8)
                            .load(core::sync::atomic::Ordering::Acquire)
                            != 0
                        {
                            core::hint::spin_loop();
                        }
                        if crate::simd_ops::atomic_xchg_u8((lock_obj + 0x04) as u64, 1) == 0 {
                            break;
                        }
                        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
                    }
                    let cur_cnt = rd::<u32>(lock_obj);
                    let new_cnt = cur_cnt.wrapping_add(delta);
                    crate::simd_ops::atomic_xchg_u32(lock_obj as u64, new_cnt);
                    let after_cnt = rd::<u32>(lock_obj);
                    core::sync::atomic::AtomicU8::from_ptr((lock_obj + 0x04) as *mut u8)
                        .store(0, core::sync::atomic::Ordering::Release);

                    let base_580f0 = param_4.wrapping_add(0x580f0);
                    let cur_idx = outer[6] as usize;
                    if after_cnt != 0 {
                        let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                        let worker_ctx = base_580f0.wrapping_add(wid * 0x188);
                        let chunk_ptr = outer[4] as usize;
                        let key_240 = [rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10), 0u64];
                        exe::call_va!(0x1_4116_0e90, (worker_ctx, key_240.as_ptr() as usize));
                    }
                    let chunk_ptr = outer[4] as usize;
                    if opt_70[0x18] != 0 && opt_70[0] == 0 {
                        let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                        let mut worker_ctx_288 = base_580f0.wrapping_add(wid * 0x188) as u64;
                        let entity_id = rd::<u64>(chunk_ptr + cur_idx * 8 + 0x10);
                        exe::call_va!(
                            0x1_4115_e0c0,
                            ((&mut worker_ctx_288 as *mut u64) as usize, entity_id)
                        );
                    }
                    wr::<u8>(r12_entry + 0x1f8, rd::<u8>(r12_entry + 0x1f8).wrapping_add(1));
                    if opt_70[0x18] != 0 {
                        exe::call_va!(0x1_401f_2bc0, (opt_70.as_mut_ptr() as usize + 0x08));
                    }
                }
            }

            outer[6] = outer[6].wrapping_add(1);
            if outer[6] >= outer[5] {
                outer[2] = outer[2].wrapping_add(2);
                if outer[2] != outer[3] {
                    loop {
                        let arch_id = rd::<u16>(outer[2] as usize) as usize;
                        let base_138 = outer[1] as usize;
                        if rd::<u64>(base_138.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                            && rd::<u32>(
                                base_138.wrapping_add(arch_id * 4).wrapping_add(0x10048),
                            ) != 0
                        {
                            break;
                        }
                        outer[2] = outer[2].wrapping_add(2);
                        if outer[2] == outer[3] {
                            break;
                        }
                    }
                    if outer[2] != outer[3] {
                        let base_130 = outer[0] as usize;
                        let arch_id = rd::<u16>(outer[2] as usize) as usize;
                        let cp = rd::<u64>(base_130.wrapping_add(arch_id * 8).wrapping_add(0x48));
                        outer[4] = cp;
                        outer[5] = if cp != 0 {
                            rd::<u32>(base_130.wrapping_add(arch_id * 4).wrapping_add(0x10048))
                                as u64
                        } else {
                            0
                        };
                        outer[6] = 0;
                    }
                }
                exe::call_va!(0x1_4115_da90, (outer.as_mut_ptr() as usize));
            }
        }
    }

    if scratch_vec[0] != 0 {
        exe::call_va!(0x1_4241_c190, (scratch_vec[0]));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141eb13e0(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(align(32))]
    struct AlignedScratch([u8; 0x100]);
    static mut BONE_SCRATCH: AlignedScratch = AlignedScratch([0; 0x100]);
    static mut POPCNT_RET: u64 = 0;

    unsafe extern "win64" fn noop1(_a: usize) {}
    unsafe extern "win64" fn noop2(_a: usize, _b: u64) {}
    unsafe extern "win64" fn mock_alloc(_sz: u64, _align: u64) -> usize {
        core::ptr::addr_of_mut!(BONE_SCRATCH.0) as usize
    }
    unsafe extern "win64" fn mock_eval_bones(_rbx: usize, dst: usize) {
        let bones: [f32; 16] = [
            0.0,
            0.0,
            0.707_106_77,
            0.707_106_77,
            1.5,
            -2.0,
            3.25,
            0.0,
            0.5,
            -0.5,
            0.5,
            0.5,
            -1.25,
            4.0,
            0.5,
            0.0,
        ];
        for (i, &v) in bones.iter().enumerate() {
            wr::<f32>(dst + i * 4, v);
        }
    }
    unsafe extern "win64" fn mock_popcnt(_ptr: usize, _bits: u64) -> u64 {
        unsafe { POPCNT_RET }
    }

    let _m1 = crate::difftest::Mock::new(0x1_40a0_2480, noop2 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40ff_f550, noop1 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40fb_0e90, noop2 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_421d_e560, mock_eval_bones as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_421d_e220, mock_eval_bones as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40ff_39e0, mock_popcnt as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4241_c190, noop1 as *const ())?;

    let run = |fn_addr: usize, mode: u32| -> ([u8; 0x60], [u8; 0x2c], [u8; 0x2c], u8, usize) {
        let mut out_mats = [0u8; 0x60];
        let mut rsi_vec = [0u64; 3];
        rsi_vec[0] = out_mats.as_mut_ptr() as u64;

        let mut p1_3_buf = [0u8; 0x40];
        let p1_3 = p1_3_buf.as_mut_ptr() as usize;
        wr::<u8>(p1_3, 1);
        wr::<usize>(p1_3 + 0x08, rsi_vec.as_mut_ptr() as usize);
        wr::<usize>(p1_3 + 0x18, rsi_vec.as_mut_ptr() as usize);

        let mut p1_4_buf = [0u8; 0x2c];
        let p1_4 = p1_4_buf.as_mut_ptr() as usize;

        let indices: [u32; 2] = [0, 1];
        let mut p1_5_buf = [0u8; 0x18];
        let p1_5 = p1_5_buf.as_mut_ptr() as usize;
        wr::<usize>(p1_5, indices.as_ptr() as usize);
        wr::<u32>(p1_5 + 0x08, 2);

        let spheres: [f32; 8] = [0.25, -0.5, 1.0, 0.75, -1.0, 2.0, -0.25, 1.25];
        let bind_mats: [f32; 24] = [
            1.0, 0.0, 0.0, 0.5, 0.0, 1.0, 0.0, -0.25, 0.0, 0.0, 1.0, 1.5, 0.0, 1.0, 0.0, -1.0,
            1.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0, 0.25,
        ];
        let bind_dq: [f32; 16] = [
            0.0,
            0.707_106_77,
            0.0,
            0.707_106_77,
            0.5,
            1.0,
            -0.5,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            -1.0,
            0.25,
            2.0,
            0.0,
        ];

        let mut mesh_buf = [0u8; 0x200];
        let mp = mesh_buf.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x1a0, spheres.as_ptr() as usize);
        wr::<usize>(mp + 0x1b0, bind_mats.as_ptr() as usize);
        wr::<usize>(mp + 0x1c0, bind_dq.as_ptr() as usize);
        wr::<u32>(mp + 0x1e8, 0);

        let mut p1_2_buf = [0u8; 0x18];
        let p1_2 = p1_2_buf.as_mut_ptr() as usize;
        wr::<usize>(p1_2 + 0x08, mp);
        wr::<u32>(p1_2 + 0x10, 0x4242_1337);

        let scales: [f32; 8] = [1.5, 2.0, 0.5, 0.0, 0.75, 1.25, 1.5, 0.0];
        let mut p1_0_entry = [0u8; 0x10];
        wr::<usize>(p1_0_entry.as_mut_ptr() as usize, scales.as_ptr() as usize);
        wr::<u32>(p1_0_entry.as_mut_ptr() as usize + 0x08, 2);

        let p1_1_flag = 0u32;
        let mut rbx_buf = [0u8; 0x10];
        let rbx = rbx_buf.as_mut_ptr() as usize;
        wr::<u16>(rbx + 0x04, 2);
        wr::<u8>(rbx + 0x06, 1);
        wr::<u8>(rbx + 0x07, 2);

        let mask_words: [u64; 1] = [1]; // bone 0 enabled, bone 1 disabled
        let mut p2_1_buf = [0u8; 0x10];
        let p2_1 = p2_1_buf.as_mut_ptr() as usize;
        wr::<u32>(p2_1, 2);
        wr::<usize>(p2_1 + 0x08, mask_words.as_ptr() as usize);

        unsafe {
            POPCNT_RET = if mode == 0 { 1 } else { 2 };
        }

        let p1: [usize; 8] = [
            if mode == 1 {
                p1_0_entry.as_ptr() as usize
            } else {
                0
            },
            (&p1_1_flag as *const u32) as usize,
            p1_2,
            p1_3,
            p1_4,
            p1_5,
            0,
            0,
        ];
        let p2: [usize; 2] = [rbx, if mode == 0 { p2_1 } else { 0 }];

        let mut p4_stream = [0u8; 0x2c];
        let mut p4_buf = [0u8; 0x20];
        let p4 = p4_buf.as_mut_ptr() as usize;
        let stream_start = p4_stream.as_mut_ptr() as usize;
        wr::<usize>(p4 + 0x10, stream_start);

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1.as_ptr() as usize as u64,
            p2.as_ptr() as usize as u64,
            0,
            p4 as u64,
            0,
            0,
            0,
            0,
            [0, 0, 0.5f32.to_bits() as u128, 0],
        );
        let stream_off = rd::<usize>(p4 + 0x10).wrapping_sub(stream_start);
        (out_mats, p1_4_buf, p4_stream, rd::<u8>(p1_3), stream_off)
    };

    for mode in 0..3 {
        let o = run(orig, mode);
        let n = run(new, mode);
        crate::diff_eq!(o, n, "141eb13e0 mode={mode}");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141fcc240(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_BOUNDS: [usize; 2] = [0; 2];
    static mut STREAMS: [u64; 14] = [0; 14];
    static mut NODE_SCRATCH: usize = 0;
    static mut ENTRY_PTRS: [usize; 2] = [0; 2];
    static mut LOCK_PTRS: [usize; 2] = [0; 2];
    static mut VEL_PTRS: [usize; 2] = [0; 2];
    static mut CALL_IDX_760: usize = 0;
    static mut CALL_IDX_960: usize = 0;
    static mut CALL_IDX_B60: usize = 0;
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: usize, _span: usize) -> usize {
        wr::<usize>(out, r11);
        wr::<usize>(out + 0x08, unsafe { ARCH_BOUNDS[0] });
        wr::<usize>(out + 0x10, unsafe { ARCH_BOUNDS[1] });
        out
    }
    unsafe extern "win64" fn mock_da90(outer: usize) {
        if rd::<u64>(outer + 0x30) < rd::<u64>(outer + 0x28) {
            for i in 0..14 {
                wr::<u64>(outer + 0x38 + i * 8, unsafe { STREAMS[i] });
            }
        }
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    unsafe extern "win64" fn noop2(_a: usize, _b: usize) {}
    unsafe extern "win64" fn noop3(_a: usize, _b: usize, _c: usize) {}
    unsafe extern "win64" fn noop4(_a: usize, _b: usize, _c: u8, _d: usize) {}
    unsafe extern "win64" fn mock_copy16(dst: usize, src: usize) {
        wr::<u128>(dst, rd::<u128>(src));
    }
    unsafe extern "win64" fn mock_reg_obj(id: u64) {
        unsafe { rec(0x100 ^ id) };
    }
    unsafe extern "win64" fn mock_alloc_node() -> usize {
        unsafe { NODE_SCRATCH }
    }
    unsafe extern "win64" fn mock_760(_ctx: usize, key: usize, _val: usize) -> usize {
        unsafe {
            rec(0x760 ^ rd::<u64>(key));
            let idx = CALL_IDX_760;
            CALL_IDX_760 = (idx + 1) & 1;
            ENTRY_PTRS[idx]
        }
    }
    unsafe extern "win64" fn mock_960(_ctx: usize, key: usize, _sp: usize) -> usize {
        unsafe {
            rec(0x960 ^ rd::<u64>(key));
            let idx = CALL_IDX_960;
            CALL_IDX_960 = (idx + 1) & 1;
            LOCK_PTRS[idx]
        }
    }
    unsafe extern "win64" fn mock_b60(_ctx: usize, key: usize, _vec: usize) -> usize {
        unsafe {
            rec(0xb60 ^ rd::<u64>(key));
            let idx = CALL_IDX_B60;
            CALL_IDX_B60 = (idx + 1) & 1;
            VEL_PTRS[idx]
        }
    }
    unsafe extern "win64" fn mock_e480(
        _r15: usize,
        _p90: usize,
        vel1: usize,
        _vel2: usize,
        pos: usize,
        dt_bits: u32,
    ) {
        unsafe { rec(0xe480 ^ (rd::<u32>(vel1) as u64) ^ (rd::<u32>(pos) as u64) ^ (dt_bits as u64)) };
    }
    unsafe extern "win64" fn mock_fce0(
        rcx: usize,
        rdx: u64,
        r8: u64,
        r9: usize,
        _a5: usize,
        _a6: usize,
        a7: u32,
        a8: u8,
        a9: u8,
        a10: usize,
    ) {
        let _ = rcx;
        unsafe {
            rec(
                0xfce0
                    ^ rdx
                    ^ r8
                    ^ (rd::<u32>(r9 + 0x14) as u64)
                    ^ (a7 as u64)
                    ^ ((a8 as u64) << 8)
                    ^ ((a9 as u64) << 16)
                    ^ (rd::<u8>(a10 + 0x10) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_f6c0(
        r1: u64,
        _r2: usize,
        _r3: usize,
        _r9: u64,
        a5: usize,
        a6: usize,
        a7: u8,
    ) {
        unsafe {
            rec(
                0xf6c0
                    ^ r1
                    ^ (rd::<u8>(a5 + 0x18) as u64)
                    ^ (rd::<u8>(a6 + 0x10) as u64)
                    ^ (a7 as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_1a70(_span: usize, scratch_vec: usize) {
        wr::<u32>(scratch_vec + 0x08, 3);
    }
    unsafe extern "win64" fn mock_f1f0(
        _p90: usize,
        _entry: usize,
        a3: usize,
        a4: usize,
        flag: u32,
    ) {
        unsafe {
            rec(0xf1f0 ^ (rd::<u8>(a3 + 0x18) as u64) ^ (rd::<u8>(a4 + 0x10) as u64) ^ (flag as u64));
        }
    }
    unsafe extern "win64" fn mock_0e90(_ctx: usize, key: usize) {
        unsafe { rec(0x0e90 ^ rd::<u64>(key)) };
    }
    unsafe extern "win64" fn mock_e0c0(_ctx_ptr: usize, id: u64) {
        unsafe { rec(0xe0c0 ^ id) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4115_da90, mock_da90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41fc_e3b0, noop1 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4116_30f0, mock_copy16 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4158_60a0, mock_reg_obj as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4024_d8b0, mock_alloc_node as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401e_5730, noop3 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4116_0760, mock_760 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4253_3200, noop4 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_424c_d080, noop3 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4116_0960, mock_960 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4116_0b60, mock_b60 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_41fc_e480, mock_e480 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_41fc_fce0, mock_fce0 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_41fc_f6c0, mock_f6c0 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4020_0d90, noop1 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4107_5c80, noop1 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_401d_e830, noop1 as *const ())?;
    let _m20 = crate::difftest::Mock::new(0x1_401e_5380, noop1 as *const ())?;
    let _m21 = crate::difftest::Mock::new(0x1_405e_ee80, noop1 as *const ())?;
    let _m22 = crate::difftest::Mock::new(0x1_41fc_eee0, noop3 as *const ())?;
    let _m23 = crate::difftest::Mock::new(0x1_41fd_1a70, mock_1a70 as *const ())?;
    let _m24 = crate::difftest::Mock::new(0x1_4158_bb70, mock_reg_obj as *const ())?;
    let _m25 = crate::difftest::Mock::new(0x1_41fc_f1f0, mock_f1f0 as *const ())?;
    let _m26 = crate::difftest::Mock::new(0x1_4241_7470, noop1 as *const ())?;
    let _m27 = crate::difftest::Mock::new(0x1_4116_0e90, mock_0e90 as *const ())?;
    let _m28 = crate::difftest::Mock::new(0x1_4115_e0c0, mock_e0c0 as *const ())?;
    let _m29 = crate::difftest::Mock::new(0x1_401f_2bc0, noop1 as *const ())?;
    let _m30 = crate::difftest::Mock::new(0x1_401e_66f0, noop2 as *const ())?;

    let scratch = exe::ptr::<u8>(0x1_43c1_c400) as *mut u8;
    let snap = crate::difftest::Snapshot::take(scratch, 32);
    unsafe { NODE_SCRATCH = scratch as usize };

    let g_c58 = exe::ptr::<u64>(0x1_43a4_1c58) as usize;
    let g_608 = exe::ptr::<u64>(0x1_43a3_4608) as usize;
    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let g_948 = exe::ptr::<u16>(0x1_43cc_8948) as usize;
    let s_c58 = rd::<u64>(g_c58);
    let s_608 = rd::<u64>(g_608);
    let s_63c = rd::<u16>(g_63c);
    let s_948 = rd::<u16>(g_948);
    wr::<u16>(g_63c, 0);
    wr::<u16>(g_948, 1);

    let run = |f: F| -> (u64, [u8; 0x208], [u8; 0x208], [u128; 2], [u64; 2], u32, u32) {
        wr::<u64>(g_c58, 100);
        wr::<u64>(g_608, 200);

        let arch_ids: [u16; 1] = [0];
        unsafe {
            ARCH_BOUNDS[0] = arch_ids.as_ptr() as usize;
            ARCH_BOUNDS[1] = arch_ids.as_ptr().add(1) as usize;
        }

        let mut chunk = [0u64; 4];
        chunk[2] = 0xaaaa_0001;
        chunk[3] = 0xbbbb_0002;

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        let r11 = wp + 0x40;
        wr::<usize>(r11 + 0x48, chunk.as_ptr() as usize);
        wr::<u32>(r11 + 0x10048, 2);

        let mut sub_items = [0u8; 0x20];
        sub_items[1] = 1;
        wr::<u64>(sub_items.as_mut_ptr() as usize + 0x10, 777);

        let mut s_168 = [0u8; 0x40];
        wr::<usize>(s_168.as_mut_ptr() as usize, sub_items.as_mut_ptr() as usize);
        wr::<u32>(s_168.as_mut_ptr() as usize + 0x08, 1);
        wr::<usize>(s_168.as_mut_ptr() as usize + 0x20, sub_items.as_mut_ptr() as usize);
        wr::<u32>(s_168.as_mut_ptr() as usize + 0x28, 1);

        let mut s_170: [u64; 2] = [0, 55];

        let mut s_178 = [0.0f32; 32];
        // Entity 0 quat + pos + prev_pos
        s_178[0] = 0.0;
        s_178[1] = 0.0;
        s_178[2] = 0.707_106_77;
        s_178[3] = 0.707_106_77;
        s_178[4] = 2.0;
        s_178[5] = 4.0;
        s_178[6] = 6.0;
        s_178[12] = 1.0;
        s_178[13] = 2.0;
        s_178[14] = 3.0;
        // Entity 1 pos + prev_pos
        s_178[20] = 10.0;
        s_178[21] = 20.0;
        s_178[22] = 30.0;
        s_178[28] = 4.0;
        s_178[29] = 8.0;
        s_178[30] = 12.0;

        let s_180: [u32; 2] = [0, 0];
        let mut s_1a0 = [0u8; 0x30];
        wr::<u32>(s_1a0.as_mut_ptr() as usize + 0x08, 1);
        wr::<u8>(s_1a0.as_mut_ptr() as usize + 0x11, 1);
        wr::<u32>(s_1a0.as_mut_ptr() as usize + 0x20, 1);
        wr::<u8>(s_1a0.as_mut_ptr() as usize + 0x29, 0);

        let s_1a8: [u32; 2] = [0x1122_3344, 0x5566_7788];
        let mut s_1b0 = [0u8; 0x20];
        s_1b0[0x08] = 3;
        s_1b0[0x09] = 5;
        wr::<f32>(s_1b0.as_mut_ptr() as usize + 0x0c, 0.5);

        let mut s_1b8 = [0u8; 0x290];
        wr::<f32>(s_1b8.as_mut_ptr() as usize + 0x18, 0.75);

        let mut s_1c0 = [0u8; 0x30];
        s_1c0[0x00] = 0;
        s_1c0[0x01] = 2;
        s_1c0[0x02] = 3;
        s_1c0[0x18] = 1;

        let s_1c8: [u128; 2] = [0x1111_2222_3333_4444, 0x5555_6666_7777_8888];

        unsafe {
            STREAMS = [
                s_168.as_mut_ptr() as u64,
                s_170.as_mut_ptr() as u64,
                s_178.as_mut_ptr() as u64,
                s_180.as_ptr() as u64,
                0,
                0,
                0,
                s_1a0.as_mut_ptr() as u64,
                s_1a8.as_ptr() as u64,
                s_1b0.as_ptr() as u64,
                s_1b8.as_ptr() as u64,
                s_1c0.as_ptr() as u64,
                s_1c8.as_ptr() as u64,
                1,
            ];
        }

        let mut entry0 = [0u8; 0x208];
        let mut entry1 = [0u8; 0x208];
        let mut lock_obj0 = [0u8; 8];
        let mut lock_obj1 = [0u8; 8];
        let lock_ptr0 = lock_obj0.as_mut_ptr() as usize;
        let lock_ptr1 = lock_obj1.as_mut_ptr() as usize;
        let mut vels = [0u128; 2];

        unsafe {
            ENTRY_PTRS = [entry0.as_mut_ptr() as usize, entry1.as_mut_ptr() as usize];
            LOCK_PTRS = [
                (&lock_ptr0 as *const usize) as usize,
                (&lock_ptr1 as *const usize) as usize,
            ];
            VEL_PTRS = [
                vels.as_mut_ptr() as usize,
                vels.as_mut_ptr().add(1) as usize,
            ];
            CALL_IDX_760 = 0;
            CALL_IDX_960 = 0;
            CALL_IDX_B60 = 0;
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        let obj_63c: u64 = 0x9999;
        let mut obj_948 = [0u8; 0x100];
        let tbl_580e0: [usize; 2] = [
            (&obj_63c as *const u64) as usize,
            obj_948.as_mut_ptr() as usize,
        ];
        let mut p4 = vec![0u8; 0x58300];
        wr::<usize>(p4.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1: [u64; 3] = [wp as u64, 0, 0];
        let dt: f32 = 0.5;
        f(
            p1.as_ptr() as usize,
            (&dt as *const f32) as usize,
            0x3000,
            p4.as_mut_ptr() as usize,
        );

        (
            unsafe { TRACE_HASH },
            entry0,
            entry1,
            vels,
            s_170,
            rd::<u32>(lock_ptr0),
            rd::<u32>(lock_ptr1),
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    snap.restore();
    wr::<u64>(g_c58, s_c58);
    wr::<u64>(g_608, s_608);
    wr::<u16>(g_63c, s_63c);
    wr::<u16>(g_948, s_948);

    crate::diff_eq!(o, n, "141fcc240 out");
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_41eb_13e0, "unknown_141eb13e0", rend::cauldron_hot_e::unknown_141eb13e0 as *const (), Throws::Discard, rend::cauldron_hot_e::diff_unknown_141eb13e0),
// Replacement::verified_throws(0x1_41fc_c240, "unknown_141fcc240", rend::cauldron_hot_e::unknown_141fcc240 as *const (), Throws::Caught, rend::cauldron_hot_e::diff_unknown_141fcc240),
