//! Decompiled hot functions shard Z3b (`0x141a72040..0x141e22a40`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vhaddps, vhsubps, vinsertps, vmaxps, vminps, vmulps, vpermilps,
    vshufps, vsubps,
};

/// `bigfish_141a72040`
/// Original: 0x141a72040 (162 bytes, Throws::No)
pub unsafe extern "win64" fn bigfish_141a72040(param_1: usize, _param_2: usize, param_3: usize) {
    let rsi = rd::<u64>(param_1 + 0x28);
    let mut local_40 = rd::<[u64; 4]>(param_1);
    let rdi = (rsi as usize).wrapping_shl(5);
    let rcx_obj = rd::<usize>((local_40[0] as usize).wrapping_add(rdi));
    let rax_20 = rd::<usize>(rcx_obj + 0x20);
    if rd::<u16>(rax_20 + 0x90) != 0 {
        let xmm2 = rd::<f32>(param_3);
        let (ret_ptr_u64, _) = crate::simd_ops::call_va8_xmm(
            0x1_4216_88a0,
            rcx_obj as u64,
            local_40.as_mut_ptr() as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, xmm2.to_bits() as u128, 0],
        );
        let ret_ptr = ret_ptr_u64 as usize;
        let p1_reload = rd::<[u64; 4]>(param_1);
        let mut local_20 = rd::<[u64; 4]>(ret_ptr);
        let rcx_arg = (p1_reload[2] as usize).wrapping_add((rsi as usize).wrapping_mul(0x18));
        let rdx_arg = (p1_reload[3] as usize).wrapping_add(rdi);
        exe::call_va!(
            0x1_41eb_29c0,
            (rcx_arg, rdx_arg, local_20.as_mut_ptr() as usize)
        );
    }
}

/// `unknown_141acc4e0`
/// Original: 0x141acc4e0 (2304 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141acc4e0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: u64,
) {
    if rd::<u8>(param_5 + 0x424) == 0 {
        return;
    }
    let r15 = rd::<u64>(param_1 + 0x68);
    let rbx = (r15.wrapping_mul(0x630) as usize).wrapping_add(rd::<usize>(param_1 + 0x08));
    let rax_98 = rd::<usize>(rbx + 0x98);
    if rd::<u8>(rax_98 + 0x98) == 0 {
        return;
    }
    if rd::<u8>(rbx + 0xa1) == 0 {
        return;
    }

    let c1 = exe::call_va!(0x1_401e_6ff0, (rbx + 0x48, param_5 + 0x45c) -> u8);
    if c1 == 0 {
        let c2 = exe::call_va!(0x1_401e_6ff0, (rbx + 0x48, rbx) -> u8);
        if c2 != 0 {
            return;
        }
    }

    let r8d = rd::<i32>(param_4 + 0x1f8);
    if r8d != -1 {
        let p60 = rd::<usize>(param_1 + 0x60);
        let rdx_val = rd::<u64>(param_4 + 0x1f8);
        let rcx_val = rd::<u64>(p60.wrapping_add((r15 as usize) * 8 + 0x10));
        if (rdx_val as u32) == (rcx_val as u32)
            && ((rdx_val >> 32) as u32) == ((rcx_val >> 32) as u32)
            && rd::<u8>(param_4 + 0x218) == 0
        {
            let p50 = rd::<usize>(param_1 + 0x50);
            if p50 != 0 {
                let e50 = p50.wrapping_add((r15.wrapping_mul(0xe0)) as usize);
                if e50 != 0 {
                    wr::<u8>(e50 + 0xa0, 1);
                }
            }
            let p58 = rd::<usize>(param_1 + 0x58);
            if p58 != 0 {
                let e58 = p58.wrapping_add((r15.wrapping_mul(0x50)) as usize);
                if e58 != 0 {
                    wr::<u32>(e58, 9);
                }
            }
            return;
        }
    }

    let rax_90 = rd::<usize>(rbx + 0x90);
    let mut xmm3 = rd::<f32>(param_5 + 0x2fc);
    let xmm11 = rd::<f32>(rax_90 + 0x150);
    let xmm12 = rd::<f32>(rax_90 + 0x154);
    if rd::<u8>(rbx + 0x5f0) != 0 {
        xmm3 += rd::<f32>(param_5 + 0x4c);
    }

    let xmm1_554 = rd::<f32>(rbx + 0x554);
    let v148_lo = rd::<f32>(rbx + 0x148);
    let v148_hi = rd::<f32>(rbx + 0x14c);
    let mut sil = false;
    let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    let xmm8 = if 0.0f32 < xmm1_554 { xmm1_554 } else { xmm3 };

    let r13 = (r15.wrapping_mul(0x630) as usize).wrapping_add(rd::<usize>(param_1 + 0x08));
    let xmm9 = (0.0f32 - xmm11) - v148_lo;
    let xmm7 = (0.0f32 - xmm12) - v148_hi;

    let mut xmm6: u128 = 0;
    xmm6 = vblendps(xmm6, xmm9.to_bits() as u128, 0x1);
    xmm6 = vinsertps(xmm6, xmm7.to_bits() as u128, 0x10);
    xmm6 = vinsertps(xmm6, xmm8.to_bits() as u128, 0x20);

    let p10_entry = rd::<usize>(param_1 + 0x10).wrapping_add((r15.wrapping_mul(0x1c)) as usize);
    let mut rsp_30 = [0u32; 4];
    let mut rsp_40 = [0u32; 4];
    let mut rsp_c0 = [0.0f32; 8];
    rsp_30[0] = rd::<u32>(p10_entry);
    rsp_30[1] = rd::<u32>(p10_entry + 0x04);
    rsp_30[2] = rd::<u32>(p10_entry + 0x08);

    if rd::<i32>(r13 + 0x4f0) == 1 && r8d == -1 {
        let xmm1_low64 = (rsp_30[0] as u128) | ((rsp_30[1] as u128) << 32);
        let c_7ac0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ac0) as usize);
        let v390 = rd::<u128>(param_5 + 0x390);
        let xmm2 = vaddps(c_7ac0, v390);
        let xmm3_diff = vsubps(xmm2, xmm1_low64);
        let d0 = xmm3_diff as u32;
        let d1 = (xmm3_diff >> 32) as u32;
        let d2 = (xmm3_diff >> 64) as u32;
        rsp_30[0] = d0 ^ sign_mask;
        rsp_30[1] = d1 ^ sign_mask;
        rsp_30[2] = d2 ^ sign_mask;
        rsp_40[0] = d0;
        rsp_40[1] = d1;
        rsp_40[2] = d2;
        let f410 = rd::<f32>(param_5 + 0x410);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4195_0f60,
            rsp_c0.as_mut_ptr() as u64,
            rsp_30.as_mut_ptr() as u64,
            rsp_40.as_mut_ptr() as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, f410.to_bits() as u128],
        );
        if !(xmm9 >= rsp_c0[0]
            && xmm7 >= rsp_c0[1]
            && xmm8 >= rsp_c0[2]
            && rsp_c0[3] >= xmm9
            && rsp_c0[4] >= xmm7
            && rsp_c0[5] >= xmm8)
        {
            sil = true;
        }
    }

    let rax_90_2 = rd::<usize>(r13 + 0x90);
    if rd::<i32>(rax_90_2 + 0x50) == 0 {
        let c = exe::call_va!(0x1_401e_6ff0, (r13 + 0x48, param_5 + 0x45c) -> u8);
        if c == 0 {
            sil = true;
        }
    }

    if rd::<u64>(param_1 + 0x48) != 0 || sil {
        return;
    }

    let c_7ac0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ac0) as usize);
    let v390 = rd::<u128>(param_5 + 0x390);
    let xmm2 = vaddps(c_7ac0, v390);
    let xmm1_5e0 = rd::<u64>(rbx + 0x5e0) as u128;
    let rax_5e8 = rd::<u64>(rbx + 0x5e8);
    let xmm3_diff = vsubps(xmm2, xmm1_5e0);
    let d0 = xmm3_diff as u32;
    let d1 = (xmm3_diff >> 32) as u32;
    let d2 = (xmm3_diff >> 64) as u32;
    rsp_30[0] = d0 ^ sign_mask;
    rsp_30[1] = d1 ^ sign_mask;
    rsp_30[2] = d2 ^ sign_mask;
    rsp_40[0] = d0;
    rsp_40[1] = d1;
    rsp_40[2] = d2;
    let f410 = rd::<f32>(param_5 + 0x410);
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4195_0f60,
        rsp_c0.as_mut_ptr() as u64,
        rsp_30.as_mut_ptr() as u64,
        rsp_40.as_mut_ptr() as u64,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 0, f410.to_bits() as u128],
    );

    let xmm3_5e8 = rax_5e8 as u128;
    let xmm7_q1 = rd::<u128>(param_5 + 0x3a0);
    let mut xmm0_box = (rsp_c0[3].to_bits() as u128) | ((rsp_c0[4].to_bits() as u128) << 32);
    xmm0_box = vinsertps(xmm0_box, rsp_c0[5].to_bits() as u128, 0x20);
    let c_8560 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8560) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
    let c_7ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);

    let xmm4_box = vblendps(xmm0_box, c_8560, 0x4);
    let xmm0_neg = xmm4_box ^ c_8760;
    let xmm1_sum = vaddps(xmm3_5e8, xmm6);
    let xmm1_max = vmaxps(xmm1_sum, xmm0_neg);
    let xmm2_min = vminps(xmm1_max, xmm4_box);
    let xmm8_rel = vsubps(xmm2_min, xmm3_5e8);

    let q1_c9 = vpermilps(xmm7_q1, 0xc9);
    let q1_d2 = vpermilps(xmm7_q1, 0xd2);
    let v8_c9 = vpermilps(xmm8_rel, 0xc9);
    let v8_d2 = vpermilps(xmm8_rel, 0xd2);
    let cross1 = vsubps(vmulps(v8_d2, q1_c9), vmulps(v8_c9, q1_d2));
    let xmm5_t1 = vmulps(cross1, c_7ab0);
    let t1_c9 = vpermilps(xmm5_t1, 0xc9);
    let t1_d2 = vpermilps(xmm5_t1, 0xd2);
    let cross2 = vsubps(vmulps(t1_d2, q1_c9), vmulps(t1_c9, q1_d2));
    let q1_ff = vpermilps(xmm7_q1, 0xff);
    let xmm7_q2 = rd::<u128>(param_5 + 0x3c0);
    let xmm0_600 = rd::<u128>(rbx + 0x600);
    let xmm8_mid = vaddps(
        vaddps(cross2, vaddps(vmulps(q1_ff, xmm5_t1), xmm8_rel)),
        rd::<u128>(param_5 + 0x3b0),
    );

    let mut rsp_100: [u128; 2] = [xmm0_600, xmm8_mid];

    let q2_d2 = vpermilps(xmm7_q2, 0xd2);
    let q2_c9 = vpermilps(xmm7_q2, 0xc9);
    let m_c9 = vpermilps(xmm8_mid, 0xc9);
    let m_d2 = vpermilps(xmm8_mid, 0xd2);
    let cross3 = vsubps(vmulps(m_d2, q2_c9), vmulps(m_c9, q2_d2));
    let xmm5_t2 = vmulps(cross3, c_7ab0);
    let t2_c9 = vpermilps(xmm5_t2, 0xc9);
    let t2_d2 = vpermilps(xmm5_t2, 0xd2);
    let cross4 = vsubps(vmulps(t2_d2, q2_c9), vmulps(t2_c9, q2_d2));
    let q2_ff = vpermilps(xmm7_q2, 0xff);
    let xmm1_fin = vaddps(
        vaddps(cross4, vaddps(vmulps(q2_ff, xmm5_t2), xmm8_mid)),
        rd::<u128>(param_5 + 0x3d0),
    );

    let eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_64c0) as usize);
    let xmm3_ext = xmm11 + eps;
    let xmm4_ext = xmm12 + eps;
    let fin_x = f32::from_bits(xmm1_fin as u32);
    let fin_y = f32::from_bits((vpermilps(xmm1_fin, 0x55)) as u32);

    let rdx_1c = rd::<usize>(param_1 + 0x10).wrapping_add((r15.wrapping_mul(0x1c)) as usize);
    let max_x = xmm3_ext + fin_x;
    let max_y = xmm4_ext + fin_y;
    let max_pair = (max_x.to_bits() as u64) | ((max_y.to_bits() as u64) << 32);
    wr::<u64>(rdx_1c + 0x14, max_pair);

    let min_x = fin_x - xmm3_ext;
    let min_y = fin_y - xmm4_ext;
    let min_pair = (min_x.to_bits() as u64) | ((min_y.to_bits() as u64) << 32);
    wr::<u64>(rdx_1c + 0x0c, min_pair);

    let has_id = exe::call_va!(0x1_41ad_9cb0, (param_5, rd::<usize>(rbx + 0x90)) -> u8);
    if has_id != 0 {
        if rd::<u8>(rbx + 0x5f2) != 0 {
            let mut rsp_60: [u64; 5] = [
                rd::<u64>(param_1 + 0x08),
                rd::<u64>(param_1 + 0x28),
                rd::<u64>(param_1 + 0x58),
                rd::<u64>(param_1 + 0x60),
                r15,
            ];
            exe::call_va!(
                0x1_41ad_8220,
                (
                    rsp_60.as_mut_ptr() as usize,
                    param_5,
                    rsp_100.as_mut_ptr() as usize,
                    param_6
                )
            );
        } else {
            let mut rsp_60: [u64; 5] = [
                rd::<u64>(param_1 + 0x20),
                rd::<u64>(param_1 + 0x28),
                rd::<u64>(param_1 + 0x50),
                rd::<u64>(param_1 + 0x60),
                r15,
            ];
            exe::call_va!(
                0x1_4195_0630,
                (
                    rsp_60.as_mut_ptr() as usize,
                    rsp_100.as_mut_ptr() as usize,
                    param_5 + 0x58,
                    param_5 + 0x60,
                    param_6,
                    7u32
                )
            );
        }
    } else {
        let r12 = rd::<u64>(param_1 + 0x60);
        let p50 = rd::<usize>(param_1 + 0x50);
        if p50 != 0 {
            let e50 = p50.wrapping_add((r15.wrapping_mul(0xe0)) as usize);
            if e50 != 0 {
                wr::<u8>(e50 + 0xa0, 1);
            }
        }
        let p58 = rd::<usize>(param_1 + 0x58);
        if p58 != 0 {
            let e58 = p58.wrapping_add((r15.wrapping_mul(0x50)) as usize);
            if e58 != 0 {
                wr::<u32>(e58, 9);
            }
        }

        let rsi_18 = rd::<u64>(param_1 + 0x18);
        let rdi_38 = rd::<u64>(param_1 + 0x38);
        let rbx_30 = rd::<u64>(param_1 + 0x30);

        let mut rsp_90 = rsp_100;
        let mut rsp_40_p2 = rd::<[u64; 3]>(param_2);
        let mut rsp_c0_1: [u64; 5] = [
            rsi_18,
            rd::<u64>(param_1 + 0x20),
            rd::<u64>(param_1 + 0x28),
            r12,
            r15,
        ];
        exe::call_va!(
            0x1_41e2_34c0,
            (
                rsp_c0_1.as_mut_ptr() as usize,
                rsp_40_p2.as_mut_ptr() as usize,
                rsp_90.as_mut_ptr() as usize
            )
        );

        rsp_90 = rsp_100;
        let mut rsp_40_p3 = rd::<[u64; 3]>(param_3);
        let mut rsp_c0_2: [u64; 5] = [rsi_18, rbx_30, rdi_38, r12, r15];
        exe::call_va!(
            0x1_41e2_2a40,
            (
                rsp_c0_2.as_mut_ptr() as usize,
                rsp_40_p3.as_mut_ptr() as usize,
                rsp_90.as_mut_ptr() as usize
            )
        );

        let p40 = rd::<usize>(param_1 + 0x40);
        wr::<u8>(p40.wrapping_add(r15 as usize), 1);
    }
}

/// `unknown_141ad8220`
/// Original: 0x141ad8220 (299 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ad8220(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    if rd::<u64>(param_1 + 0x10) != 0 {
        return;
    }
    let rbx = rd::<u64>(param_1 + 0x20);
    let rax = (rbx.wrapping_mul(0x630) as usize).wrapping_add(rd::<usize>(param_1));
    let r10 = rd::<usize>(rax + 0x90);
    let r11 = rd::<i32>(r10 + 0x50) as isize;
    let mut ecx = rd::<i32>(rax + 0x4d8)
        .wrapping_mul(rd::<i32>(param_2 + 0xc0))
        .wrapping_add(rd::<i32>(r10 + 0x740))
        .wrapping_add(rd::<i32>((param_2 as isize + r11 * 4 + 0xc4) as usize));
    let dl: u8 = if rd::<usize>(param_2 + 0x400) == r10 {
        1
    } else {
        0
    };
    if dl != 0 {
        ecx = i32::MIN;
    }
    let f0_bits = rd::<u32>((param_2 as isize + r11 * 4 + 0xf0) as usize);

    let mut rsp_40 = [0u8; 0x50];
    let p40 = rsp_40.as_mut_ptr() as usize;
    wr::<u32>(p40, 0);
    wr::<i32>(p40 + 0x04, ecx);
    wr::<u32>(p40 + 0x08, f0_bits);
    let off_12 = r11 * 12;
    wr::<u64>(p40 + 0x0c, rd::<u64>((param_2 as isize + off_12 + 0x114) as usize));
    wr::<u32>(p40 + 0x14, rd::<u32>((param_2 as isize + off_12 + 0x11c) as usize));
    wr::<u32>(p40 + 0x18, 0);
    wr::<[u64; 4]>(p40 + 0x20, rd::<[u64; 4]>(param_3));
    wr::<u8>(p40 + 0x40, dl);
    wr::<u8>(p40 + 0x41, 1);
    wr::<u64>(p40 + 0x48, 0);

    let p18 = rd::<usize>(param_1 + 0x18);
    let h_val = rd::<u64>(p18.wrapping_add((rbx as usize) * 8 + 0x10));
    let mut rsp_30 = [0u8; 0x10];
    let p30 = rsp_30.as_mut_ptr() as usize;
    wr::<u64>(p30, h_val);
    wr::<u32>(p30 + 0x08, 0);

    exe::call_va!(0x1_4058_fde0, (param_4, p30, p40));
}

/// `unknown_141ad9cb0`
/// Original: 0x141ad9cb0 (59 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ad9cb0(param_1: usize, param_2: usize) -> u64 {
    let r8d = rd::<u32>(param_1 + 0x430);
    let mut eax: u32 = 0;
    let r10 = rd::<usize>(param_1 + 0x428);
    let r11 = r10.wrapping_add((r8d as usize) * 4);
    let mut rdx_ptr = r11;
    if r8d != 0 {
        let r9d = rd::<u32>(param_2 + 0x750);
        loop {
            let cur = r10.wrapping_add((eax as usize) * 4);
            if rd::<u32>(cur) == r9d {
                rdx_ptr = cur;
                break;
            }
            eax = eax.wrapping_add(1);
            if eax >= r8d {
                rdx_ptr = r11;
                break;
            }
        }
    }
    let al = if rdx_ptr != r11 { 1u64 } else { 0u64 };
    ((eax as u64) & !0xff) | al
}

/// `unknown_141df69d0`
/// Original: 0x141df69d0 (1521 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141df69d0(param_1: usize, param_2: usize, param_3: usize) {
    let tbl_base = rd::<usize>(param_3 + 0x580e0);
    let idx_e88 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
    let r8_map = rd::<usize>(tbl_base.wrapping_add(idx_e88 * 8));
    let idx_314 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0314) as usize) as usize;
    let mut rbp_80 = rd::<usize>(tbl_base.wrapping_add(idx_314 * 8));
    let mut flag_70: u8 = 0;

    let r14_38 = rd::<u64>(r8_map + 0x38);
    if rd::<u64>(r8_map + 0x40) != r14_38 {
        let mut r10_step: u64 = 0;
        let rdi_mask = rd::<u64>(r8_map + 0x48).wrapping_sub(1);
        let r12_k0 = rd::<u64>(exe::ptr::<u64>(0x1_43a4_4b80) as usize);
        let r15_k1 = rd::<u64>(exe::ptr::<u64>(0x1_43a4_4b88) as usize);
        let mut rcx_idx = r12_k0 ^ r15_k1;
        let mut r11_tomb: u64 = u64::MAX;
        let rbx_buckets = rd::<usize>(r8_map + 0x78);
        let r13_empty0 = rd::<u64>(r8_map + 0x50);

        loop {
            rcx_idx &= rdi_mask;
            let rdx_ent = rbx_buckets.wrapping_add((rcx_idx as usize) * 0x28);
            let r9_k0 = rd::<u64>(rdx_ent);
            if r13_empty0 == r9_k0 && rd::<u64>(r8_map + 0x58) == rd::<u64>(rdx_ent + 0x08) {
                break;
            }
            if r14_38 != 0
                && rd::<u64>(r8_map + 0x28) == r9_k0
                && rd::<u64>(r8_map + 0x30) == rd::<u64>(rdx_ent + 0x08)
            {
                if r11_tomb == u64::MAX {
                    r11_tomb = rcx_idx;
                    r10_step = r10_step.wrapping_add(1);
                    rcx_idx = rcx_idx.wrapping_add(r10_step);
                    continue;
                }
            } else if r12_k0 == r9_k0 && r15_k1 == rd::<u64>(rdx_ent + 0x08) {
                if rcx_idx != u64::MAX {
                    let end_ent =
                        rbx_buckets.wrapping_add((rd::<u64>(r8_map + 0x48) as usize) * 0x28);
                    if rdx_ent != end_ent {
                        let rax_10 = rdx_ent.wrapping_add(0x10);
                        if rd::<u16>(rax_10) != 6 && rax_10 != 0 {
                            flag_70 = if rd::<u64>(rax_10 + 0x08) != 0 { 1 } else { 0 };
                        }
                    }
                }
                break;
            }
            r10_step = r10_step.wrapping_add(1);
            rcx_idx = rcx_idx.wrapping_add(r10_step);
        }
    }

    let edi_len = rd::<u32>(param_1 + 0x08);
    let g_a60 = rd::<usize>(exe::ptr::<usize>(0x1_4397_da60) as usize);
    let g_a60_8 = rd::<usize>(g_a60 + 0x08);
    let idx_314_b = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0314) as usize) as usize;
    let rcx_tbl2 = rd::<usize>(g_a60_8 + 0x580e0);
    let rbx_obj2 = rd::<usize>(rcx_tbl2.wrapping_add(idx_314_b * 8));
    let r14_obj1 = rbp_80;

    let should_run = if flag_70 != 0 {
        true
    } else if exe::call_va!(0x1_41e4_ac30, (r14_obj1) -> u8) != 0 {
        true
    } else if exe::call_va!(0x1_41e4_ac30, (rbx_obj2) -> u8) != 0 {
        true
    } else {
        edi_len != 0
    };
    if !should_run {
        return;
    }

    let mut arr = [0u64; 2];
    let arr_ptr = arr.as_mut_ptr() as usize;
    let g_2e8 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_42e8) as usize);
    let g_a38 = rd::<u32>(exe::ptr::<u32>(0x1_43a2_9a38) as usize);
    if rd::<u32>(g_2e8 + 0xe0) == g_a38 {
        let b10 = rd::<u8>(param_1 + 0x10);
        exe::call_va!(0x1_41e4_acc0, (r14_obj1, arr_ptr, b10));
    }
    let b10 = rd::<u8>(param_1 + 0x10);
    exe::call_va!(0x1_41e4_acc0, (rbx_obj2, arr_ptr, b10));

    let mut pair_30: [usize; 2] = [arr_ptr, (&mut flag_70 as *mut u8) as usize];
    let rax_730 = exe::call_va!(
        0x1_40f4_a730,
        (param_1, pair_30.as_mut_ptr() as usize) -> u64
    );
    wr::<u8>(param_1 + 0x11, if rax_730 != 0 { 1 } else { 0 });

    let mut rdi_cur = arr[0] as usize;
    let r14_end = rdi_cur.wrapping_add(((arr[1] as u32) as usize) * 0x50);
    let r12_empty = exe::ptr::<u8>(0x1_43c1_6200) as usize;

    while rdi_cur != r14_end {
        let item_active = if flag_70 != 0 {
            true
        } else {
            let p1 = exe::call_va!(0x1_4021_7df0, (rdi_cur + 0x18) -> usize);
            if p1 == 0 {
                false
            } else {
                let p2 = exe::call_va!(0x1_4021_7df0, (rdi_cur + 0x18) -> usize);
                rd::<u8>(p2 + 0x7d) != 0
            }
        };

        if item_active {
            let target_id = rd::<u32>(rdi_cur + 0x34);
            let mut rbx_it = rd::<usize>(param_1);
            let end_it = rbx_it.wrapping_add((rd::<u32>(param_1 + 0x08) as usize) * 0xd8);
            while rbx_it != end_it {
                if rd::<u32>(rbx_it) == target_id {
                    break;
                }
                rbx_it = rbx_it.wrapping_add(0xd8);
            }

            let end_it2 =
                rd::<usize>(param_1).wrapping_add((rd::<u32>(param_1 + 0x08) as usize) * 0xd8);
            if rbx_it == end_it2 {
                let mut temp_elem = [0u64; 27];
                let ep = temp_elem.as_mut_ptr() as usize;
                let inline_wbuf = ep + 0x50;
                wr::<u32>(ep, target_id);
                wr::<u32>(ep + 0x04, 0x0400_0004);
                wr::<u32>(ep + 0x08, 0);
                wr::<u8>(ep + 0x0c, 0);
                wr::<usize>(ep + 0x30, inline_wbuf);
                wr::<u32>(ep + 0x38, 0x40);
                wr::<u64>(ep + 0x3c, 0x40);
                wr::<usize>(ep + 0x48, inline_wbuf);
                wr::<u16>(inline_wbuf, 0);
                wr::<u32>(ep + 0xd0, 0xffff_ffff);

                let p1 = exe::call_va!(0x1_4021_7df0, (rdi_cur + 0x18) -> usize);
                let src_str = if p1 == 0 {
                    r12_empty
                } else {
                    let p2 = exe::call_va!(0x1_4021_7df0, (rdi_cur + 0x18) -> usize);
                    let p3 = exe::call_va!(0x1_4021_7df0, (p2 + 0x48) -> usize);
                    if p3 == 0 {
                        r12_empty
                    } else {
                        let p4 = exe::call_va!(0x1_4021_7df0, (rdi_cur + 0x18) -> usize);
                        let p5 = exe::call_va!(0x1_4021_7df0, (p4 + 0x48) -> usize);
                        p5 + 0x108
                    }
                };

                if ep + 0x04 != src_str {
                    let s_len = rd::<u32>(src_str + 0x04) as u64;
                    let mut s_data = src_str + 0x08;
                    let b3 = rd::<u8>(src_str + 0x03) as u32;
                    let inline_bits = if b3 < 0x80 {
                        b3 * 8
                    } else {
                        (b3 << 8).wrapping_sub(0x7c00)
                    };
                    let req_bits = (rd::<u32>(src_str) & 0x00ff_ffff) << 3;
                    if req_bits > inline_bits {
                        s_data = rd::<usize>(s_data);
                    }
                    let mut view: [u64; 2] = [s_data as u64, s_len];
                    exe::call_va!(
                        0x1_4240_9b50,
                        (
                            ep + 0x04,
                            view.as_mut_ptr() as usize,
                            0u64,
                            u64::MAX
                        )
                    );
                }

                let cur_end =
                    rd::<usize>(param_1).wrapping_add((rd::<u32>(param_1 + 0x08) as usize) * 0xd8);
                exe::call_va!(
                    0x1_40f4_af70,
                    (
                        param_1,
                        (&mut rbp_80 as *mut usize) as usize,
                        cur_end,
                        ep
                    )
                );
                rbx_it = rbp_80;

                let wptr = rd::<usize>(ep + 0x30);
                let ibuf = rd::<usize>(ep + 0x48);
                if wptr != 0 && wptr != ibuf {
                    let r10 = wptr.wrapping_sub(1) & !0x01ff_ffffusize;
                    let is_owner = exe::teb() == rd::<usize>(r10 + 0x100);
                    let page_idx = wptr.wrapping_sub(r10) >> 16;
                    let hdr_slot = r10.wrapping_add(0x108).wrapping_add(page_idx * 0x50);
                    let rdx_hdr = hdr_slot.wrapping_sub(rd::<u32>(hdr_slot + 0x04) as usize);
                    if is_owner {
                        if rd::<u8>(rdx_hdr + 0x0e) == 0 {
                            wr::<u64>(wptr, rd::<u64>(rdx_hdr + 0x20));
                            wr::<usize>(rdx_hdr + 0x20, wptr);
                            let new_cnt = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
                            wr::<u32>(rdx_hdr + 0x18, new_cnt);
                            if new_cnt == 0 {
                                exe::call_va!(0x1_4253_71a0, (rdx_hdr));
                            }
                        } else {
                            exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, 1u8, wptr));
                        }
                    } else {
                        exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, 0u8, wptr));
                    }
                }
                let ibuf2 = rd::<usize>(ep + 0x48);
                wr::<usize>(ep + 0x30, ibuf2);
                wr::<u32>(ep + 0x3c, rd::<u32>(ep + 0x38));
                if ibuf2 != 0 {
                    wr::<u16>(ibuf2, 0);
                }
                wr::<u32>(ep + 0x40, 0);
                exe::call_va!(0x1_4240_ab10, (ep + 0x04));
            }

            let changed = exe::call_va!(0x1_41df_6440, (0usize, rbx_it, rdi_cur) -> u8);
            wr::<u8>(param_1 + 0x11, rd::<u8>(param_1 + 0x11) | changed);

            if rd::<u32>(rbx_it + 0x40) == 0 {
                let end_before =
                    rd::<usize>(param_1).wrapping_add((rd::<u32>(param_1 + 0x08) as usize) * 0xd8);
                exe::call_va!(0x1_40f4_b170, (rbx_it + 0xd8, end_before, rbx_it));
                let end_after =
                    rd::<usize>(param_1).wrapping_add((rd::<u32>(param_1 + 0x08) as usize) * 0xd8);
                let wptr = rd::<usize>(end_after - 0xa8);
                let ibuf = rd::<usize>(end_after - 0x90);
                if wptr != 0 && wptr != ibuf {
                    exe::call_va!(0x1_4244_2de0, (wptr));
                }
                let ibuf2 = rd::<usize>(end_after - 0x90);
                wr::<usize>(end_after - 0xa8, ibuf2);
                wr::<u32>(end_after - 0x9c, rd::<u32>(end_after - 0xa0));
                if ibuf2 != 0 {
                    wr::<u16>(ibuf2, 0);
                }
                wr::<u32>(end_after - 0x98, 0);
                exe::call_va!(0x1_4240_ab10, (end_after - 0xd4));
                wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_sub(1));
                wr::<u8>(param_1 + 0x11, 1);
            }
        }

        rdi_cur = rdi_cur.wrapping_add(0x50);
    }

    if rd::<u8>(param_1 + 0x11) != 0 {
        wr::<u8>(param_1 + 0x11, 0);
        let vtbl2 = rd::<usize>(param_2);
        let fn_230 = rd::<usize>(vtbl2 + 0x230);
        let fn_218 = rd::<usize>(vtbl2 + 0x218);
        let rdi_sub = exe::call_va!(fn_218, (param_2) -> usize);
        let mut r15_res: u64 = 0;
        if rdi_sub != 0 {
            let ebx = exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x78), (rdi_sub) -> u32);
            exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x70), (rdi_sub, 2u32));
            exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x28), (rdi_sub));
            exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x28), (rdi_sub));
            exe::call_va!(0x1_41df_6fd0, (rdi_sub, param_1));
            exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x48), (rdi_sub, param_1));
            exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x48), (rdi_sub, param_1));
            exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x70), (rdi_sub, ebx));
            r15_res = exe::call_va!(rd::<usize>(rd::<usize>(rdi_sub) + 0x40), (rdi_sub) -> u64);
        }
        exe::call_va!(fn_230, (param_2, param_1, r15_res));
    }

    let mut rdi_del = arr[0] as usize;
    let rbx_del_end = rdi_del.wrapping_add(((arr[1] as u32) as usize) * 0x50);
    while rdi_del != rbx_del_end {
        let vtbl = rd::<usize>(rdi_del);
        exe::call_va!(rd::<usize>(vtbl), (rdi_del, 0u32));
        rdi_del = rdi_del.wrapping_add(0x50);
    }
    let buf_to_free = arr[0] as usize;
    if buf_to_free != 0 {
        exe::call_va!(0x1_4241_c190, (buf_to_free));
    }
}

/// `unknown_141e1b980`
/// Original: 0x141e1b980 (128 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e1b980(param_1: usize) {
    let r10 = rd::<u64>(param_1 + 0x18) as usize;
    if r10 == 0 {
        return;
    }
    let mut r8_off: usize = 0;
    for rdx in 0..r10 {
        let p20 = rd::<usize>(param_1 + 0x20);
        let p28 = rd::<usize>(param_1 + 0x28);
        let p30 = rd::<usize>(param_1 + 0x30);
        let r9 = p20.wrapping_add(rdx * 4);
        let c0 = rd::<u8>(p30.wrapping_add(r8_off + 0x0c)) != 0
            && rd::<u8>(p28.wrapping_add(rdx * 2)) != 0;
        let c1 = rd::<u8>(p30.wrapping_add(r8_off + 0x0d)) != 0
            && rd::<u8>(p28.wrapping_add(rdx * 2 + 1)) != 0;
        if c0 || c1 {
            wr::<u32>(r9, rd::<u32>(r9) | 0x4);
        }
        wr::<u16>(p28.wrapping_add(rdx * 2), 0);
        r8_off = r8_off.wrapping_add(0x18);
    }
}

/// `unknown_141e1efc0`
/// Original: 0x141e1efc0 (98 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141e1efc0(param_1: usize) {
    let rsi = rd::<u64>(param_1 + 0x18) as usize;
    if rsi == 0 {
        return;
    }
    let rbx = rd::<usize>(param_1 + 0x20);
    let rbp = rd::<usize>(param_1 + 0x28);
    for rdi in 0..rsi {
        let is_zero: u8 = if rd::<u32>(rbp.wrapping_add(rdi * 4)) == 0 {
            1
        } else {
            0
        };
        let rcx_arg = rd::<u64>(rbx.wrapping_add(rdi * 8));
        exe::call_va!(0x1_4213_bb50, (rcx_arg, is_zero));
    }
}

/// `unknown_141e22a40`
/// Original: 0x141e22a40 (971 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e22a40(param_1: usize, param_2: usize, param_3: usize) {
    let r14 = (rd::<u64>(param_1 + 0x20) as usize).wrapping_shl(5);
    let p1_10 = rd::<usize>(param_1 + 0x10);
    let cur_slot = p1_10.wrapping_add(r14);

    let p3_f = rd::<[f32; 8]>(param_3);
    let cur_f = rd::<[f32; 8]>(cur_slot);
    if p3_f[0] == cur_f[0]
        && p3_f[1] == cur_f[1]
        && p3_f[2] == cur_f[2]
        && p3_f[3] == cur_f[3]
        && cur_f[4] == p3_f[4]
        && cur_f[5] == p3_f[5]
        && cur_f[6] == p3_f[6]
        && cur_f[7] == p3_f[7]
    {
        return;
    }

    let ymm3_lo = rd::<u128>(param_3);
    let ymm3_hi = rd::<u128>(param_3 + 0x10);
    wr::<u128>(cur_slot, ymm3_lo);
    wr::<u128>(cur_slot + 0x10, ymm3_hi);

    let idx = rd::<u64>(param_1 + 0x20) as usize;
    let p1_0 = rd::<usize>(param_1);
    let rdx_h = rd::<u64>(p1_0.wrapping_add(idx * 8));
    let rax_h = if (rdx_h as u32) != 0xffff_ffff {
        rdx_h
    } else {
        u64::MAX
    };

    let p2_0 = rd::<usize>(param_2);
    let valid_idx: u32 = if (rax_h as u32) == 0xffff_ffff && ((rax_h >> 32) as u32) == 0xffff_ffff
    {
        0xffff_ffff
    } else {
        let r8d = rax_h as u32;
        let cap_20 = rd::<u32>(p2_0 + 0x20);
        if (r8d as u64) >= (cap_20 as u64) {
            0xffff_ffff
        } else {
            let p18 = rd::<usize>(p2_0 + 0x18);
            let ent = rd::<u64>(p18.wrapping_add((r8d as usize) * 8));
            if (ent as u32) == r8d && ((ent >> 32) as u32) == ((rax_h >> 32) as u32) {
                r8d
            } else {
                0xffff_ffff
            }
        }
    };

    let mut rsp_30_lo: u128 = 0;
    let mut rsp_30_hi: u128 = 0;
    let has_parent = if valid_idx == 0xffff_ffff {
        false
    } else {
        let p30 = rd::<usize>(p2_0 + 0x30);
        let rbx = rd::<u64>(p30.wrapping_add((valid_idx as usize) * 8));
        if (rbx as u16) == 0xffff {
            exe::call_iat!(0x1_4264_67e8, ());
            return;
        }
        let r11 = (rbx as u16) as usize;
        let r8_tag = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f6d8) as usize) as u32;
        let rcx_arg = rd::<u64>(p2_0.wrapping_add(r11 * 0x20 + 0x18098));
        let edx_arg = rd::<u32>(p2_0.wrapping_add(r11 * 0x20 + 0x180ac));
        let eax_idx = exe::call_va!(0x1_4248_9ee0, (rcx_arg, edx_arg, r8_tag) -> i32);
        let tbl_40 = rd::<usize>(p2_0.wrapping_add((r11 + 0xc03) * 0x20 + 0x40));
        let off_dw = rd::<u32>((tbl_40 as isize + (eax_idx as isize) * 4) as usize) as usize;
        let base_88 = rd::<usize>(p2_0.wrapping_add(r11 * 8 + 0x88));
        let parent_ptr = base_88
            .wrapping_add(off_dw)
            .wrapping_add(((rbx >> 32) as usize) * 0x20);
        rsp_30_lo = rd::<u128>(parent_ptr);
        rsp_30_hi = rd::<u128>(parent_ptr + 0x10);
        true
    };

    let (out_lo, out_hi) = if has_parent {
        let ymm4_lo = rd::<u128>(param_3);
        let xmm7 = rd::<u128>(param_3 + 0x10);
        let mask_7b00 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7b00) as usize);
        let mask_8760 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
        let c_7ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);

        let xmm9 = rsp_30_lo ^ mask_7b00;
        let xmm13 = rsp_30_hi ^ mask_8760;
        let xmm12 = vpermilps(xmm9, 0xc9);
        let xmm11 = vpermilps(xmm9, 0xd2);
        let xmm5 = vpermilps(xmm9, 0x1b);
        let xmm8 = vpermilps(xmm9, 0xff);

        let t13_c9 = vpermilps(xmm13, 0xc9);
        let t13_d2 = vpermilps(xmm13, 0xd2);
        let xmm10 = vmulps(
            vsubps(vmulps(t13_d2, xmm12), vmulps(t13_c9, xmm11)),
            c_7ab0,
        );

        let t7_c9 = vpermilps(xmm7, 0xc9);
        let t7_d2 = vpermilps(xmm7, 0xd2);
        let xmm6 = vmulps(vsubps(vmulps(t7_d2, xmm12), vmulps(t7_c9, xmm11)), c_7ab0);

        let xmm3_a = vpermilps(ymm4_lo, 0x11);
        let xmm1_a = vmulps(xmm3_a, xmm9);
        let xmm2_a = vpermilps(ymm4_lo, 0xbb);
        let xmm0_a = vmulps(xmm2_a, xmm5);
        let xmm4_hsub = vhsubps(xmm1_a, xmm0_a);
        let xmm0_b = vmulps(xmm3_a, xmm5);
        let xmm2_b = vmulps(xmm2_a, xmm9);
        let xmm3_hadd = vhaddps(xmm2_b, xmm0_b);
        let xmm0_shuf = vshufps(xmm4_hsub, xmm3_hadd, 0xb1);
        let xmm1_blend = vblendps(xmm3_hadd, xmm4_hsub, 0x0c);
        let xmm1_addsub = vaddsubps(xmm1_blend, xmm0_shuf);
        let res_lo = vpermilps(xmm1_addsub, 0x9c);

        let t6_d2 = vpermilps(xmm6, 0xd2);
        let t6_c9 = vpermilps(xmm6, 0xc9);
        let xmm4_r1 = vsubps(vmulps(t6_d2, xmm12), vmulps(t6_c9, xmm11));
        let xmm1_r1 = vaddps(vmulps(xmm6, xmm8), xmm7);
        let xmm5_r1 = vaddps(xmm4_r1, xmm1_r1);

        let t10_c9 = vpermilps(xmm10, 0xc9);
        let t10_d2 = vpermilps(xmm10, 0xd2);
        let xmm4_r2 = vsubps(vmulps(t10_d2, xmm12), vmulps(t10_c9, xmm11));
        let xmm0_r2 = vaddps(vmulps(xmm8, xmm10), xmm13);
        let xmm1_r2 = vaddps(xmm4_r2, xmm0_r2);
        let res_hi = vaddps(xmm5_r1, xmm1_r2);
        (res_lo, res_hi)
    } else {
        (rd::<u128>(param_3), rd::<u128>(param_3 + 0x10))
    };

    let p1_08 = rd::<usize>(param_1 + 0x08);
    let dst_slot = p1_08.wrapping_add(r14);
    wr::<u128>(dst_slot, out_lo);
    wr::<u128>(dst_slot + 0x10, out_hi);

    let p1_18 = rd::<usize>(param_1 + 0x18);
    let idx2 = rd::<u64>(param_1 + 0x20) as usize;
    let arg_rcx = rd::<u64>(p1_18.wrapping_add(idx2 * 8 + 0x10));
    exe::call_va!(0x1_41e2_2790, (arg_rcx));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_bigfish_141a72040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RET_BUF: [u64; 4] = [0; 4];
    static mut CALL_LOG: (usize, [u64; 4], u32, usize, usize, [u64; 4]) =
        (0, [0; 4], 0, 0, 0, [0; 4]);

    unsafe extern "win64" fn mock_88a0(rcx: usize, rdx: usize) -> usize {
        let [_, _, xmm2, _] = crate::simd_ops::take_xmm_args();
        CALL_LOG.0 = rcx;
        CALL_LOG.1 = rd::<[u64; 4]>(rdx);
        CALL_LOG.2 = xmm2 as u32;
        core::ptr::addr_of_mut!(RET_BUF) as usize
    }

    unsafe extern "win64" fn mock_29c0(rcx: usize, rdx: usize, r8: usize) {
        CALL_LOG.3 = rcx;
        CALL_LOG.4 = rdx;
        CALL_LOG.5 = rd::<[u64; 4]>(r8);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4216_88a0, mock_88a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41eb_29c0, mock_29c0 as *const ())?;

    let run = |f: F, flag_90: u16| -> (u32, usize, usize, [u64; 4]) {
        RET_BUF = [0x1111_2222, 0x3333_4444, 0x5555_6666, 0x7777_8888];
        CALL_LOG = (0, [0; 4], 0, 0, 0, [0; 4]);

        let mut sub_90 = [0u8; 0xa0];
        let s90p = sub_90.as_mut_ptr() as usize;
        wr::<u16>(s90p + 0x90, flag_90);

        let mut obj = [0u8; 0x30];
        let objp = obj.as_mut_ptr() as usize;
        wr::<usize>(objp + 0x20, s90p);

        let mut arr0 = [0u64; 16];
        let arr0p = arr0.as_mut_ptr() as usize;
        wr::<usize>(arr0p + 2 * 0x20, objp);

        let mut p1 = [0u64; 6];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p, arr0p as u64);
        wr::<u64>(p1p + 0x08, 0xaaaa_0000);
        wr::<u64>(p1p + 0x10, 0xbbbb_0000);
        wr::<u64>(p1p + 0x18, 0xcccc_0000);
        wr::<u64>(p1p + 0x28, 2);

        let f_arg: f32 = 3.5;
        f(p1p, 0, (&f_arg as *const f32) as usize);
        (CALL_LOG.2, CALL_LOG.3, CALL_LOG.4, CALL_LOG.5)
    };

    for flag in [0u16, 5] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        crate::diff_eq!(o, n, "141a72040 flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141acc4e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MOCK_HAS_ID: u8 = 0;
    static mut LOG_TAGS: u32 = 0;
    static mut LOG_VEC: [u64; 4] = [0; 4];

    unsafe extern "win64" fn mock_6ff0(_rcx: usize, _rdx: usize) -> u8 {
        LOG_TAGS += 1;
        1
    }

    unsafe extern "win64" fn mock_0f60(rcx: usize, rdx: usize, _r8: usize, xmm3: f32) {
        LOG_TAGS += 10;
        let d0 = rd::<f32>(rdx);
        wr::<f32>(rcx, -100.0 + d0 * 0.0 + xmm3 * 0.0);
        wr::<f32>(rcx + 0x04, -100.0);
        wr::<f32>(rcx + 0x08, -100.0);
        wr::<f32>(rcx + 0x0c, 100.0);
        wr::<f32>(rcx + 0x10, 100.0);
        wr::<f32>(rcx + 0x14, 100.0);
    }

    unsafe extern "win64" fn mock_9cb0(_rcx: usize, _rdx: usize) -> u8 {
        MOCK_HAS_ID
    }

    unsafe extern "win64" fn mock_8220(_rcx: usize, _rdx: usize, r8: usize, _r9: u64) {
        LOG_TAGS += 100;
        LOG_VEC = rd::<[u64; 4]>(r8);
    }

    unsafe extern "win64" fn mock_0630(
        _rcx: usize,
        rdx: usize,
        _r8: usize,
        _r9: usize,
        _a5: u64,
        a6: u32,
    ) {
        LOG_TAGS += 1000 + a6;
        LOG_VEC = rd::<[u64; 4]>(rdx);
    }

    unsafe extern "win64" fn mock_34c0(_rcx: usize, _rdx: usize, r8: usize) {
        LOG_TAGS += 10000;
        LOG_VEC = rd::<[u64; 4]>(r8);
    }

    unsafe extern "win64" fn mock_2a40(_rcx: usize, _rdx: usize, r8: usize) {
        LOG_TAGS += 100000;
        LOG_VEC = rd::<[u64; 4]>(r8);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4195_0f60, mock_0f60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41ad_9cb0, mock_9cb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41ad_8220, mock_8220 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4195_0630, mock_0630 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e2_34c0, mock_34c0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41e2_2a40, mock_2a40 as *const ())?;

    let run = |f: F, mode: u8| -> (u32, [u64; 4], [u32; 7], u8, u32, u8) {
        LOG_TAGS = 0;
        LOG_VEC = [0; 4];
        MOCK_HAS_ID = if mode == 1 { 0 } else { 1 };

        let mut s98 = [0u8; 0xa0];
        s98[0x98] = 1;
        let mut s90 = [0u8; 0x160];
        let s90p = s90.as_mut_ptr() as usize;
        wr::<i32>(s90p + 0x50, 1);
        wr::<f32>(s90p + 0x150, 2.0);
        wr::<f32>(s90p + 0x154, 3.0);

        let mut rbx_arr = vec![0u8; 0x630 * 2];
        let rbxp = rbx_arr.as_mut_ptr() as usize;
        let ent = rbxp + 0x630;
        wr::<usize>(ent + 0x90, s90p);
        wr::<usize>(ent + 0x98, s98.as_mut_ptr() as usize);
        wr::<u8>(ent + 0xa1, 1);
        wr::<f32>(ent + 0x148, 0.5);
        wr::<f32>(ent + 0x14c, 1.5);
        wr::<i32>(ent + 0x4f0, 1);
        wr::<f32>(ent + 0x554, 4.0);
        wr::<f32>(ent + 0x5e0, 1.0);
        wr::<f32>(ent + 0x5e4, 2.0);
        wr::<f32>(ent + 0x5e8, 0.25);
        wr::<f32>(ent + 0x5ec, 0.75);
        wr::<u8>(ent + 0x5f0, 1);
        wr::<u8>(ent + 0x5f2, if mode == 2 { 1 } else { 0 });
        wr::<u64>(ent + 0x600, 0x1234_5678_9abc_def0);
        wr::<u64>(ent + 0x608, 0xfedc_ba98_7654_3210);

        let mut p10_arr = [0u32; 14];
        p10_arr[7] = 1.0f32.to_bits();
        p10_arr[8] = 2.0f32.to_bits();
        p10_arr[9] = 3.0f32.to_bits();

        let mut p40_arr = [0u8; 8];
        let mut p50_arr = [0u8; 0x1c0];
        let mut p58_arr = [0u8; 0xa0];
        let mut p60_arr = [0u64; 8];
        p60_arr[3] = 0x0002_0000_0001;

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, rbxp);
        wr::<usize>(p1p + 0x10, p10_arr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x40, p40_arr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x50, p50_arr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x58, p58_arr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x60, p60_arr.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x68, 1);

        let p2: [u64; 3] = [11, 22, 33];
        let p3: [u64; 3] = [44, 55, 66];
        let mut p4 = [0u8; 0x220];
        let p4p = p4.as_mut_ptr() as usize;
        if mode == 0 {
            wr::<u64>(p4p + 0x1f8, 0x0002_0000_0001);
        } else {
            wr::<i32>(p4p + 0x1f8, -1);
        }

        let mut p5 = [0u8; 0x480];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<f32>(p5p + 0x4c, 1.25);
        wr::<f32>(p5p + 0x2fc, 2.5);
        wr::<[f32; 4]>(p5p + 0x390, [1.0, 2.0, 3.0, 0.0]);
        wr::<[f32; 4]>(p5p + 0x3a0, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(p5p + 0x3b0, [10.0, 20.0, 30.0, 0.0]);
        wr::<[f32; 4]>(p5p + 0x3c0, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(p5p + 0x3d0, [5.0, 6.0, 7.0, 0.0]);
        wr::<f32>(p5p + 0x410, 15.0);
        wr::<u8>(p5p + 0x424, 1);

        f(p1p, p2.as_ptr() as usize, p3.as_ptr() as usize, p4p, p5p, 0x99);

        let out_1c: [u32; 7] = [
            p10_arr[7],
            p10_arr[8],
            p10_arr[9],
            p10_arr[10],
            p10_arr[11],
            p10_arr[12],
            p10_arr[13],
        ];
        (
            LOG_TAGS,
            LOG_VEC,
            out_1c,
            p50_arr[0xe0 + 0xa0],
            rd::<u32>((p58_arr.as_ptr() as usize) + 0x50),
            p40_arr[1],
        )
    };

    for mode in [0u8, 1, 2, 3] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141acc4e0 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_141ad8220(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_DATA: (usize, u64, u8, [u32; 7], [u64; 4], u8, u8, u64) =
        (0, 0, 0, [0; 7], [0; 4], 0, 0, 0);

    unsafe extern "win64" fn mock_fde0(rcx: usize, rdx: usize, r8: usize) {
        CALL_DATA = (
            rcx,
            rd::<u64>(rdx),
            rd::<u8>(rdx + 0x08),
            rd::<[u32; 7]>(r8),
            rd::<[u64; 4]>(r8 + 0x20),
            rd::<u8>(r8 + 0x40),
            rd::<u8>(r8 + 0x41),
            rd::<u64>(r8 + 0x48),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4058_fde0, mock_fde0 as *const ())?;

    let run = |f: F, match_r10: bool| -> (usize, u64, u8, [u32; 7], [u64; 4], u8, u8, u64) {
        CALL_DATA = (0, 0, 0, [0; 7], [0; 4], 0, 0, 0);

        let mut r10_obj = [0u8; 0x750];
        let r10p = r10_obj.as_mut_ptr() as usize;
        wr::<i32>(r10p + 0x50, 1);
        wr::<i32>(r10p + 0x740, 100);

        let mut rbx_arr = [0u8; 0x630 * 2];
        let rbxp = rbx_arr.as_mut_ptr() as usize;
        wr::<usize>(rbxp + 0x630 + 0x90, r10p);
        wr::<i32>(rbxp + 0x630 + 0x4d8, 3);

        let mut p18_arr = [0u64; 4];
        p18_arr[3] = 0xdead_beef_cafe_1234;

        let p1: [u64; 5] = [
            rbxp as u64,
            0,
            0,
            p18_arr.as_mut_ptr() as u64,
            1,
        ];

        let mut p2 = [0u8; 0x410];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0xc0, 5);
        wr::<i32>(p2p + 0xc8, 7);
        wr::<u32>(p2p + 0xf4, 0x3f80_0000);
        wr::<u64>(p2p + 0x120, 0x1122_3344_5566_7788);
        wr::<u32>(p2p + 0x128, 0x99aa_bbcc);
        wr::<usize>(p2p + 0x400, if match_r10 { r10p } else { 0 });

        let p3: [u64; 4] = [10, 20, 30, 40];
        f(p1.as_ptr() as usize, p2p, p3.as_ptr() as usize, 0x7777);
        CALL_DATA
    };

    for match_r10 in [false, true] {
        let o = run(f_o, match_r10);
        let n = run(f_n, match_r10);
        crate::diff_eq!(o, n, "141ad8220 match_r10={}", match_r10);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141ad9cb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u32, target: u32| -> u64 {
        let arr: [u32; 4] = [10, 20, 30, 40];
        let mut p1 = [0u8; 0x440];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x428, arr.as_ptr() as usize);
        wr::<u32>(p1p + 0x430, count);

        let mut p2 = [0u8; 0x760];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x750, target);
        f(p1p, p2p)
    };

    for (cnt, tgt) in [(0u32, 20u32), (4, 30), (4, 99)] {
        let o = run(f_o, cnt, tgt);
        let n = run(f_n, cnt, tgt);
        crate::diff_eq!(o, n, "141ad9cb0 cnt={} tgt={}", cnt, tgt);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141df69d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_STEPS: u32 = 0;

    unsafe extern "win64" fn mock_ac30(_rcx: usize) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_acc0(_rcx: usize, _rdx: usize, _r8: u8) {
        CALL_STEPS += 1;
    }
    unsafe extern "win64" fn mock_a730(_rcx: usize, _rdx: usize) -> u64 {
        CALL_STEPS += 10;
        1
    }
    unsafe extern "win64" fn mock_6fd0(_rcx: usize, _rdx: usize) {
        CALL_STEPS += 100;
    }
    unsafe extern "win64" fn mock_v218(_this: usize) -> usize {
        CALL_STEPS += 1000;
        0
    }
    unsafe extern "win64" fn mock_v230(_this: usize, _p1: usize, _res: u64) {
        CALL_STEPS += 10000;
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e4_ac30, mock_ac30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e4_acc0, mock_acc0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40f4_a730, mock_a730 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41df_6fd0, mock_6fd0 as *const ())?;

    let g_a60_addr = exe::ptr::<usize>(0x1_4397_da60) as usize;
    let g_2e8_addr = exe::ptr::<usize>(0x1_43a3_42e8) as usize;
    let g_e88_addr = exe::ptr::<u16>(0x1_43c8_ce88) as usize;
    let g_314_addr = exe::ptr::<u16>(0x1_43c9_0314) as usize;
    let saved_a60 = rd::<usize>(g_a60_addr);
    let saved_2e8 = rd::<usize>(g_2e8_addr);
    let saved_e88 = rd::<u16>(g_e88_addr);
    let saved_314 = rd::<u16>(g_314_addr);

    let run = |f: F, flag_active: bool| -> (u32, u8) {
        CALL_STEPS = 0;

        wr::<u16>(g_e88_addr, 1);
        wr::<u16>(g_314_addr, 2);

        let k0 = rd::<u64>(exe::ptr::<u64>(0x1_43a4_4b80) as usize);
        let k1 = rd::<u64>(exe::ptr::<u64>(0x1_43a4_4b88) as usize);
        let mut buckets = [0u8; 0x50];
        let bp = buckets.as_mut_ptr() as usize;
        let b_idx = ((k0 ^ k1) & 1) as usize;
        let ent = bp + b_idx * 0x28;
        wr::<u64>(ent, k0);
        wr::<u64>(ent + 0x08, k1);
        wr::<u16>(ent + 0x10, 1);
        wr::<u64>(ent + 0x18, if flag_active { 1 } else { 0 });

        let mut map_obj = [0u8; 0x80];
        let mp = map_obj.as_mut_ptr() as usize;
        wr::<u64>(mp + 0x38, 0);
        wr::<u64>(mp + 0x40, 1);
        wr::<u64>(mp + 0x48, 2);
        wr::<u64>(mp + 0x50, !k0);
        wr::<u64>(mp + 0x58, !k1);
        wr::<usize>(mp + 0x78, bp);

        let idx_e88 = rd::<u16>(g_e88_addr) as usize;
        let idx_314 = rd::<u16>(g_314_addr) as usize;
        let max_idx = idx_e88.max(idx_314) + 2;
        let mut ptr_tbl = vec![0usize; max_idx];
        ptr_tbl[idx_e88] = mp;
        ptr_tbl[idx_314] = 0x1111;

        let mut p3 = vec![0u8; 0x580f0];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x580e0, ptr_tbl.as_mut_ptr() as usize);

        let mut a60_root = [0usize; 2];
        a60_root[1] = p3p;
        let mut g2e8_buf = [0u8; 0xf0];
        wr::<u32>(
            (g2e8_buf.as_mut_ptr() as usize) + 0xe0,
            rd::<u32>(exe::ptr::<u32>(0x1_43a2_9a38) as usize),
        );

        wr::<usize>(g_a60_addr, a60_root.as_mut_ptr() as usize);
        wr::<usize>(g_2e8_addr, g2e8_buf.as_mut_ptr() as usize);

        let mut vtbl2 = [0usize; 0x50];
        vtbl2[0x218 / 8] = mock_v218 as *const () as usize;
        vtbl2[0x230 / 8] = mock_v230 as *const () as usize;
        let p2: usize = vtbl2.as_mut_ptr() as usize;

        let mut p1 = [0u8; 0x20];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p + 0x10, 3);

        f(p1p, (&p2 as *const usize) as usize, p3p);

        wr::<usize>(g_a60_addr, saved_a60);
        wr::<usize>(g_2e8_addr, saved_2e8);
        wr::<u16>(g_e88_addr, saved_e88);
        wr::<u16>(g_314_addr, saved_314);

        (CALL_STEPS, rd::<u8>(p1p + 0x11))
    };

    for active in [false, true] {
        let o = run(f_o, active);
        let n = run(f_n, active);
        crate::diff_eq!(o, n, "141df69d0 active={}", active);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141e1b980(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u64| -> ([u32; 4], [u16; 4]) {
        let mut p20 = [0x10u32, 0x20, 0x30, 0x40];
        let mut p28 = [0x0001u16, 0x0100, 0x0101, 0x0000];
        let mut p30 = [0u8; 0x18 * 4];
        p30[0x0c] = 1;
        p30[0x18 + 0x0d] = 1;
        p30[2 * 0x18 + 0x0c] = 0;
        p30[2 * 0x18 + 0x0d] = 0;
        p30[3 * 0x18 + 0x0c] = 1;
        p30[3 * 0x18 + 0x0d] = 1;

        let mut p1 = [0u8; 0x38];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, count);
        wr::<usize>(p1p + 0x20, p20.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, p28.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x30, p30.as_mut_ptr() as usize);

        f(p1p);
        (p20, p28)
    };

    for count in [0u64, 4] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "141e1b980 count={}", count);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141e1efc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_CALLS: [(u64, u8); 4] = [(0, 0); 4];
    static mut LOG_CNT: usize = 0;

    unsafe extern "win64" fn mock_bb50(rcx: u64, dl: u8) {
        if LOG_CNT < 4 {
            LOG_CALLS[LOG_CNT] = (rcx, dl);
            LOG_CNT += 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4213_bb50, mock_bb50 as *const ())?;

    let run = |f: F, count: u64| -> (usize, [(u64, u8); 4]) {
        LOG_CALLS = [(0, 0); 4];
        LOG_CNT = 0;

        let rbx: [u64; 3] = [0x1111, 0x2222, 0x3333];
        let rbp: [u32; 3] = [0, 5, 0];

        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, count);
        wr::<usize>(p1p + 0x20, rbx.as_ptr() as usize);
        wr::<usize>(p1p + 0x28, rbp.as_ptr() as usize);

        f(p1p);
        (LOG_CNT, LOG_CALLS)
    };

    for count in [0u64, 3] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "141e1efc0 count={}", count);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141e22a40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NOTIFIED_ARG: u64 = 0;

    unsafe extern "win64" fn mock_9ee0(_rcx: u64, _edx: u32, _r8d: u32) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_2790(rcx: u64) {
        NOTIFIED_ARG = rcx;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e2_2790, mock_2790 as *const ())?;

    let run = |f: F, mode: u8| -> (u64, [u32; 8], [u32; 8]) {
        NOTIFIED_ARG = 0;

        let p3_val: [f32; 8] = [0.0, 0.70710677, 0.0, 0.70710677, 4.0, 5.0, 6.0, 0.0];
        let mut slot10 = [0.0f32; 16];
        if mode == 0 {
            slot10[8..16].copy_from_slice(&p3_val);
        }
        let mut slot08 = [0.0f32; 16];

        let mut handles: [u64; 2] = [u64::MAX, if mode == 2 { 0x0001_0000_0000 } else { u64::MAX }];
        let mut notify_arr: [u64; 4] = [0, 0, 0, 0xabcd_ef01];

        let p1: [u64; 5] = [
            handles.as_mut_ptr() as u64,
            slot08.as_mut_ptr() as u64,
            slot10.as_mut_ptr() as u64,
            notify_arr.as_mut_ptr() as u64,
            1,
        ];

        let ent_18: [u64; 1] = [0x0001_0000_0000];
        let ent_30: [u64; 1] = [0x0000_0001_0000_0000];
        let off_tbl: [u32; 1] = [0];
        let parent_tf: [f32; 16] = [
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, // index 0
            0.0, 0.0, 0.70710677, 0.70710677, 1.0, 2.0, 3.0, 0.0, // index 1
        ];

        let mut p2_obj = vec![0u8; 0x18100];
        let p20p = p2_obj.as_mut_ptr() as usize;
        wr::<usize>(p20p + 0x18, ent_18.as_ptr() as usize);
        wr::<u32>(p20p + 0x20, 1);
        wr::<usize>(p20p + 0x30, ent_30.as_ptr() as usize);
        wr::<usize>(p20p + 0x88, parent_tf.as_ptr() as usize);
        wr::<usize>(p20p + 0xc03 * 0x20 + 0x40, off_tbl.as_ptr() as usize);
        wr::<u64>(p20p + 0x18098, 0x1234);
        wr::<u32>(p20p + 0x180ac, 0x5678);

        let p2: [u64; 3] = [p20p as u64, 0, 0];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3_val.as_ptr() as usize,
        );

        let out10 = rd::<[u32; 8]>((slot10.as_ptr() as usize) + 0x20);
        let out08 = rd::<[u32; 8]>((slot08.as_ptr() as usize) + 0x20);
        (NOTIFIED_ARG, out10, out08)
    };

    for mode in [0u8, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141e22a40 mode={}", mode);
    }
    Ok(3)
}
