//! Decompiled `rend` hot functions shard ZA15 (`0x14216fe00..0x1421d7250`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vblendvps, vbroadcastss, vcmpss, vdpps, vhaddps, vhsubps, vmulps,
    vpermilps, vroundps, vshufps, vsqrtps, vsubps,
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
fn vrcpss_f32(v: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(v)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        f32::from_bits(crate::simd_ops::vrcpss(0, v.to_bits()) as u32)
    }
}

#[inline(always)]
fn vfmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for lane in 0..4 {
        let fa = f32::from_bits(((a >> (lane * 32)) & 0xffff_ffff) as u32);
        let fb = f32::from_bits(((b >> (lane * 32)) & 0xffff_ffff) as u32);
        let fc = f32::from_bits(((acc >> (lane * 32)) & 0xffff_ffff) as u32);
        let r = fa.mul_add(fb, fc);
        out |= (r.to_bits() as u128) << (lane * 32);
    }
    out
}

#[inline(always)]
fn vfnmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for lane in 0..4 {
        let fa = f32::from_bits(((a >> (lane * 32)) & 0xffff_ffff) as u32);
        let fb = f32::from_bits(((b >> (lane * 32)) & 0xffff_ffff) as u32);
        let fc = f32::from_bits(((acc >> (lane * 32)) & 0xffff_ffff) as u32);
        let r = (-fa).mul_add(fb, fc);
        out |= (r.to_bits() as u128) << (lane * 32);
    }
    out
}

#[inline(always)]
unsafe fn call_atan2f(y: f32, x: f32) -> f32 {
    let (_, x0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad94,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [y.to_bits() as u128, x.to_bits() as u128, 0, 0],
    );
    f32::from_bits(x0 as u32)
}

/// `physics_14216fe00`
/// Original: 0x14216fe00 (197 bytes, Throws::No)
pub unsafe extern "win64" fn physics_14216fe00(param_1: usize) -> usize {
    wr::<u64>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    wr::<u64>(param_1 + 0x18, 0);
    wr::<u64>(param_1 + 0x20, 0);
    wr::<u64>(param_1 + 0x28, 0);

    let cap0 = rd::<u32>(param_1 + 0x0c);
    if cap0 < 8 {
        let mut new_cap = ((cap0 >> 1) as u64).wrapping_add(cap0 as u64);
        if new_cap < 8 {
            new_cap = 8;
        }
        exe::call_va!(0x1_4127_0ce0, (param_1, new_cap));
    }

    if rd::<u32>(param_1 + 0x1c) < 0x80 {
        exe::call_va!(0x1_4127_0290, (param_1 + 0x10, 0x80u64));
    }

    exe::call_va!(0x1_4127_0090, (param_1 + 0x20, 0x200u64));

    let pos_max: u64 = 0x7fef_ffff_ffff_ffff;
    let neg_max: u64 = 0xffef_ffff_ffff_ffff;
    wr::<u64>(param_1 + 0x30, pos_max);
    wr::<u64>(param_1 + 0x38, pos_max);
    wr::<u64>(param_1 + 0x40, pos_max);
    wr::<u64>(param_1 + 0x48, neg_max);
    wr::<u64>(param_1 + 0x50, neg_max);
    wr::<u64>(param_1 + 0x58, neg_max);
    wr::<u32>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x18, 0);
    wr::<u32>(param_1 + 0x28, 0);
    param_1
}

/// `r_1421825c0`
/// Original: 0x1421825c0 (363 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_1421825c0(param_1: usize, param_2: usize, param_3: i32) {
    if param_1 == 0 {
        return;
    }

    let mut rbx = rd::<usize>(param_2);
    let rdi_end = rbx.wrapping_add((rd::<u32>(param_2 + 0x08) as usize).wrapping_mul(0x2a8));
    while rbx != rdi_end {
        exe::call_va!(0x1_4128_ff80, (rbx));
        rbx = rbx.wrapping_add(0x2a8);
    }
    wr::<u32>(param_2 + 0x08, 0);

    let vtbl = rd::<usize>(param_1);
    let fn_28 = rd::<u64>(vtbl + 0x28);
    let rdi_mgr = exe::call_va!(fn_28, (param_1) -> usize);
    let rsi_lock = rdi_mgr.wrapping_add(0x2d40);
    exe::call_iat!(0x1_4264_55a8, (rsi_lock));

    let mut rcx_it = rd::<usize>(rdi_mgr + 0x08);
    let r8_end = rcx_it.wrapping_add((rd::<u32>(rdi_mgr + 0x10) as usize).wrapping_mul(0x2a8));
    let mut active_cnt: u64 = 0;
    while rcx_it != r8_end {
        if rd::<u32>(rcx_it + 0x04) != 0 {
            active_cnt = active_cnt.wrapping_add(1);
        }
        rcx_it = rcx_it.wrapping_add(0x2a8);
    }

    if active_cnt > (rd::<u32>(param_2 + 0x0c) as u64) {
        exe::call_va!(0x1_4129_1780, (param_2, active_cnt));
    }

    let mut rbx_it = rd::<usize>(rdi_mgr + 0x08);
    let rdi_it_end = rbx_it.wrapping_add((rd::<u32>(rdi_mgr + 0x10) as usize).wrapping_mul(0x2a8));
    while rbx_it != rdi_it_end {
        if rd::<u32>(rbx_it + 0x04) != 0 {
            let eax_128 = rd::<u32>(rbx_it + 0x128);
            let keep = if param_3 == 1 {
                rd::<u8>(param_1 + 0x940) != 0 || (eax_128 & 0xffff_fffc) != 0 || eax_128 == 2
            } else {
                true
            };
            if keep {
                let edx_len = rd::<u32>(param_2 + 0x08) as u64;
                let eax_cap = rd::<u32>(param_2 + 0x0c) as u64;
                if edx_len.wrapping_mul(0x2a8) == eax_cap.wrapping_mul(0x2a8) {
                    exe::call_va!(0x1_4129_1780, (param_2, edx_len.wrapping_add(1)));
                }
                let dst = rd::<usize>(param_2)
                    .wrapping_add((rd::<u32>(param_2 + 0x08) as usize).wrapping_mul(0x2a8));
                exe::call_va!(0x1_4129_1570, (dst, rbx_it));
                wr::<u32>(param_2 + 0x08, rd::<u32>(param_2 + 0x08).wrapping_add(1));
            }
        }
        rbx_it = rbx_it.wrapping_add(0x2a8);
    }

    exe::call_iat!(0x1_4264_5598, (rsi_lock));
}

/// `r_142183fb0`
/// Original: 0x142183fb0 (422 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142183fb0(param_1: usize, param_2: usize) {
    if param_1 == 0 {
        return;
    }

    let mut local_arr = [0u8; 0x2a90];
    let arr_ptr = local_arr.as_mut_ptr() as usize;
    exe::call_va!(0x1_4183_acc2, (arr_ptr, 0u32, 0x2a90u64));
    wr::<usize>(arr_ptr, arr_ptr + 0x10);
    wr::<u32>(arr_ptr + 0x08, 0);
    wr::<u32>(arr_ptr + 0x0c, 0x10);

    exe::call_va!(0x1_4218_25c0, (param_1, arr_ptr, 1i32));

    let mut rdi = rd::<usize>(param_2);
    let rbp = rdi.wrapping_add((rd::<u32>(param_2 + 0x08) as usize).wrapping_mul(0x90));
    if rdi != rbp {
        let mut rbx = rdi.wrapping_add(0x20);
        loop {
            exe::call_va!(0x1_4240_ab10, (rbx.wrapping_add(0x28)));
            wr::<u64>(rbx.wrapping_add(0x20), 0);
            rdi = rdi.wrapping_add(0x90);
            rbx = rbx.wrapping_add(0x90);
            if rdi == rbp {
                break;
            }
        }
    }
    wr::<u32>(param_2 + 0x08, 0);

    let r8d = rd::<u32>(arr_ptr + 0x08) as u64;
    let eax_cap = rd::<u32>(param_2 + 0x0c) as u64;
    if r8d > eax_cap {
        let mut new_cap = (eax_cap >> 1).wrapping_add(eax_cap);
        if new_cap < r8d {
            new_cap = r8d;
        }
        exe::call_va!(0x1_4129_1e20, (param_2, new_cap));
    }

    let count = rd::<u32>(arr_ptr + 0x08) as usize;
    let mut rbx_it = rd::<usize>(arr_ptr);
    let rdi_end = rbx_it.wrapping_add(count.wrapping_mul(0x2a8));
    while rbx_it != rdi_end {
        let edx_len = rd::<u32>(param_2 + 0x08) as u64;
        let r8d_cap = rd::<u32>(param_2 + 0x0c) as u64;
        if edx_len.wrapping_mul(0x90) == r8d_cap.wrapping_mul(0x90) {
            let min_cap = edx_len.wrapping_add(1);
            let mut new_cap = (r8d_cap >> 1).wrapping_add(r8d_cap);
            if new_cap < min_cap {
                new_cap = min_cap;
            }
            exe::call_va!(0x1_4129_1e20, (param_2, new_cap));
        }
        let dst = rd::<usize>(param_2)
            .wrapping_add((rd::<u32>(param_2 + 0x08) as usize).wrapping_mul(0x90));
        exe::call_va!(0x1_401f_3100, (dst, rbx_it));
        wr::<u32>(param_2 + 0x08, rd::<u32>(param_2 + 0x08).wrapping_add(1));
        rbx_it = rbx_it.wrapping_add(0x2a8);
    }

    exe::call_va!(0x1_4129_1860, (arr_ptr));
}

#[inline(always)]
unsafe fn finish_rel_transform(
    out_ptr: usize,
    q_inv: u128,
    neg_p0: u128,
    pos: u128,
    rot: u128,
    c_7ab0: u128,
) -> usize {
    let q_d2 = vpermilps(q_inv, 0xd2);
    let q_c9 = vpermilps(q_inv, 0xc9);

    let xmm0_1 = vpermilps(neg_p0, 0xd2);
    let xmm3_1 = vmulps(xmm0_1, q_c9);
    let xmm1_1 = vpermilps(neg_p0, 0xc9);
    let xmm2_1 = vmulps(xmm1_1, q_d2);
    let xmm5 = vmulps(vsubps(xmm3_1, xmm2_1), c_7ab0);

    let xmm0_2 = vpermilps(pos, 0xd2);
    let xmm3_2 = vmulps(xmm0_2, q_c9);
    let xmm1_2 = vpermilps(pos, 0xc9);
    let xmm2_2 = vmulps(xmm1_2, q_d2);
    let xmm6_t = vmulps(vsubps(xmm3_2, xmm2_2), c_7ab0);

    let xmm1_3 = vpermilps(xmm5, 0xd2);
    let xmm3_3 = vmulps(xmm1_3, q_c9);
    let xmm0_3 = vpermilps(xmm5, 0xc9);
    let xmm2_3 = vmulps(xmm0_3, q_d2);
    let xmm4_3 = vsubps(xmm3_3, xmm2_3);

    let xmm2_4 = vpermilps(xmm6_t, 0xd2);
    let xmm3_4 = vmulps(xmm2_4, q_c9);
    let q_ff = vpermilps(q_inv, 0xff);
    let xmm1_4 = vmulps(q_ff, xmm5);
    let xmm0_4 = vaddps(xmm1_4, neg_p0);
    let xmm5_sum = vaddps(xmm4_3, xmm0_4);

    let xmm0_5 = vpermilps(xmm6_t, 0xc9);
    let xmm1_5 = vmulps(xmm0_5, q_d2);
    let xmm4_5 = vsubps(xmm3_4, xmm1_5);
    let xmm2_5 = vmulps(xmm6_t, q_ff);
    let xmm0_5b = vaddps(xmm2_5, pos);
    let xmm1_5b = vaddps(xmm4_5, xmm0_5b);
    let xmm6_pos = vaddps(xmm5_sum, xmm1_5b);

    let xmm2_r = vpermilps(rot, 0xbb);
    let xmm5_r = vpermilps(q_inv, 0x1b);
    let xmm3_r = vpermilps(rot, 0x11);
    let xmm0_r = vmulps(xmm2_r, xmm5_r);
    let xmm2_r2 = vmulps(xmm2_r, q_inv);
    let xmm1_r = vmulps(xmm3_r, q_inv);
    let xmm4_r = vhsubps(xmm1_r, xmm0_r);
    let xmm0_r2 = vmulps(xmm3_r, xmm5_r);
    let xmm3_r2 = vhaddps(xmm2_r2, xmm0_r2);
    let xmm0_sh = vshufps(xmm4_r, xmm3_r2, 0xb1);
    let xmm1_bl = vblendps(xmm3_r2, xmm4_r, 0x0c);
    let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);
    let xmm2_quat = vpermilps(xmm1_as, 0x9c);

    wr::<u128>(out_ptr, xmm2_quat);
    wr::<u128>(out_ptr + 0x10, xmm6_pos);
    out_ptr
}

/// `unknown_1421be220`
/// Original: 0x1421be220 (1102 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421be220(
    param_1: usize,
    param_2: usize,
    _param_3_f32: usize,
    _param_4_f32: usize,
) -> usize {
    let [_, _, xmm2_in, xmm3_in] = crate::simd_ops::take_xmm_args();
    let rsi_base = exe::ptr::<u8>(0x1_4386_6000) as usize;
    let xmm8 = rd::<f32>(param_2 + 0x14);
    let xmm10 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm9 = xmm10 / rd::<f32>(param_2 + 0x18);
    let idx0 = rd::<u16>(param_2 + 0x0e) as usize;
    let mut xmm7 = f32::from_bits(xmm3_in as u32);
    let mut xmm6 = xmm9 + f32::from_bits(xmm2_in as u32);

    let mut rsp_40 = [0u128; 2];
    let fn_va0 = rd::<u64>(rsi_base + idx0 * 8 + 0x90);
    let _ = crate::simd_ops::call_va8_xmm(
        fn_va0,
        rsp_40.as_mut_ptr() as usize as u64,
        param_2 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm2_in & 0xffff_ffff, 0],
    );

    let mut ymm13 = rsp_40;
    let mut rsp_20 = ymm13;

    let c_7b00 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7b00) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let c_7ab0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7ab0) as usize);

    if xmm8 > xmm6 {
        loop {
            let mut rsp_60 = [0u128; 2];
            let idx = rd::<u16>(param_2 + 0x0e) as usize;
            let fn_va = rd::<u64>(rsi_base + idx * 8 + 0x90);
            let _ = crate::simd_ops::call_va8_xmm(
                fn_va,
                rsp_60.as_mut_ptr() as usize as u64,
                param_2 as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, 0, xmm6.to_bits() as u128, 0],
            );

            let ymm15 = rsp_20;
            let xmm0_hi = ymm15[1];
            rsp_20[0] = xmm0_hi;
            let diff = vsubps(xmm0_hi, rsp_60[1]);
            let dp = vdpps(diff, diff, 0x7f);
            let xmm1_vec = vsqrtps(dp);
            let xmm1_ss = f32::from_bits(xmm1_vec as u32);

            if xmm1_ss >= xmm7 {
                let xmm0_inv = if xmm1_ss.is_nan() || xmm1_ss != 0.0 {
                    xmm10 / xmm1_ss
                } else {
                    0.0
                };
                let xmm9_neg_p0 = c_8760 ^ rsp_40[1];
                let mut xmm3 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
                let mut xmm6_p = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
                let xmm13_ss = xmm0_inv * xmm7;
                let xmm0_dp = vdpps(ymm15[0], rsp_60[0], 0xff);
                let xmm8_sign = xmm0_dp & c_8760;
                let xmm5_abs = xmm8_sign ^ xmm0_dp;
                let c_80c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize);
                xmm3 = vfmadd231ps_128(xmm3, xmm5_abs, c_80c0);
                xmm6_p = vfmadd231ps_128(xmm6_p, xmm5_abs, xmm3);
                xmm3 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
                let xmm13_b = vbroadcastss(xmm13_ss.to_bits());
                let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
                let c_8240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
                let mut xmm4 = vsubps(xmm13_b, c_8120);
                let xmm2 = vsubps(xmm13_b, c_8240);
                let xmm1_m = vmulps(xmm13_b, xmm4);
                let xmm7_t = vmulps(xmm2, xmm1_m);
                let c_84c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
                let c_82c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize);
                let xmm1_p = vfnmadd231ps_128(c_84c0, xmm5_abs, c_82c0);
                let c_88d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
                let xmm2_p = vfmadd231ps_128(c_88d0, xmm5_abs, xmm1_p);
                xmm3 = vfmadd231ps_128(xmm3, xmm5_abs, xmm2_p);
                xmm4 = vmulps(xmm4, xmm4);
                xmm6_p = vfmadd231ps_128(xmm6_p, xmm3, xmm4);
                let mut xmm0_w = vfmadd231ps_128(xmm13_b, xmm6_p, xmm7_t);
                let mut xmm6_pos = rsp_20[0];
                let mut xmm15_q = vfnmadd231ps_128(ymm15[0], xmm0_w, ymm15[0]);
                xmm0_w ^= xmm8_sign;
                xmm15_q = vfmadd231ps_128(xmm15_q, xmm0_w, rsp_60[0]);
                xmm6_pos = vfnmadd231ps_128(xmm6_pos, xmm6_pos, xmm13_b);
                xmm6_pos = vfmadd231ps_128(xmm6_pos, xmm13_b, rsp_60[1]);

                let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);
                let xmm11_dp = vdpps(xmm15_q, xmm15_q, 0xff);
                let xmm12_rsq = vrsqrtps_128(xmm11_dp);
                let xmm0_r1 = vmulps(xmm12_rsq, xmm11_dp);
                let xmm2_r2 = vmulps(xmm0_r1, xmm12_rsq);
                let xmm0_half = vmulps(xmm12_rsq, c_8120);
                let xmm3_nr = vsubps(c_8460, xmm2_r2);
                let xmm2_sc = vmulps(xmm3_nr, xmm0_half);
                let xmm1_rot = vmulps(xmm2_sc, xmm15_q);

                let xmm13_qinv = c_7b00 ^ rsp_40[0];
                return finish_rel_transform(
                    param_1,
                    xmm13_qinv,
                    xmm9_neg_p0,
                    xmm6_pos,
                    xmm1_rot,
                    c_7ab0,
                );
            }

            ymm13 = rsp_60;
            xmm6 += xmm9;
            rsp_20 = ymm13;
            xmm7 -= xmm1_ss;
            if !(xmm8 > xmm6) {
                break;
            }
        }
    }

    let xmm12_qinv = c_7b00 ^ rsp_40[0];
    let xmm10_neg_p0 = c_8760 ^ rsp_40[1];
    finish_rel_transform(param_1, xmm12_qinv, xmm10_neg_p0, ymm13[1], ymm13[0], c_7ab0)
}

/// `unknown_1421bf670`
/// Original: 0x1421bf670 (511 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421bf670(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u32,
    param_5_bits: u32,
) {
    let param_5 = f32::from_bits(param_5_bits);
    let scale_ss = param_5 * rd::<f32>(exe::ptr::<f32>(0x1_4342_64b4) as usize);
    let xmm12 = vbroadcastss(scale_ss.to_bits());
    if param_4 == 0 {
        return;
    }

    let c_81b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
    let c_8840 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
    let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let c_8240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
    let c_80c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize);
    let c_84c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
    let c_82c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize);
    let c_88d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
    let c_8280 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
    let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);

    for i in 0..(param_4 as usize) {
        let b = rd::<u8>(param_3.wrapping_add(i));
        let p1_elem = param_1.wrapping_add(i * 0x20);
        let p2_elem = param_2.wrapping_add(i * 0x20);
        let ymm9_lo = rd::<u128>(p1_elem);
        let ymm9_hi = rd::<u128>(p1_elem + 0x10);
        let ymm10_lo = rd::<u128>(p2_elem);
        let ymm10_hi = rd::<u128>(p2_elem + 0x10);

        let xmm0_b = vbroadcastss((b as i32 as f32).to_bits());
        let xmm11 = vmulps(xmm0_b, xmm12);
        let xmm0_dp = vdpps(ymm9_lo, ymm10_lo, 0xff);
        let xmm7 = xmm0_dp & c_8760;
        let xmm5 = xmm7 ^ xmm0_dp;
        let xmm2 = vfmadd231ps_128(c_8840, xmm5, c_80c0);
        let mut xmm4 = vfmadd231ps_128(c_81b0, xmm5, xmm2);
        let mut xmm8 = vbroadcastss(xmm11 as u32);
        let xmm0_sub = vsubps(xmm8, c_8240);
        let mut xmm3 = vsubps(xmm8, c_8120);
        let xmm1_m = vmulps(xmm3, xmm8);
        let xmm6 = vmulps(xmm1_m, xmm0_sub);
        let xmm0_p = vfnmadd231ps_128(c_84c0, xmm5, c_82c0);
        let xmm1_p = vfmadd231ps_128(c_88d0, xmm5, xmm0_p);
        let xmm0_p2 = vfmadd231ps_128(c_8280, xmm5, xmm1_p);
        xmm3 = vmulps(xmm3, xmm3);
        xmm4 = vfmadd231ps_128(xmm4, xmm0_p2, xmm3);
        xmm8 = vfmadd231ps_128(xmm8, xmm4, xmm6);
        let xmm0_w = xmm8 ^ xmm7;
        let mut xmm4_q = vfnmadd231ps_128(ymm9_lo, xmm8, ymm9_lo);
        xmm4_q = vfmadd231ps_128(xmm4_q, xmm0_w, ymm10_lo);
        let xmm0_qdp = vdpps(xmm4_q, xmm4_q, 0xff);
        let xmm2_rsq = vrsqrtps_128(xmm0_qdp);
        let xmm0_r1 = vmulps(xmm2_rsq, xmm0_qdp);
        let xmm1_r2 = vmulps(xmm0_r1, xmm2_rsq);
        let xmm2_half = vmulps(c_8120, xmm2_rsq);
        let xmm3_nr = vsubps(c_8460, xmm1_r2);
        let xmm0_sc = vmulps(xmm3_nr, xmm2_half);
        let xmm1_rot = vmulps(xmm0_sc, xmm4_q);

        let mut xmm2_pos = vfnmadd231ps_128(ymm9_hi, ymm9_hi, xmm11);
        xmm2_pos = vfmadd231ps_128(xmm2_pos, ymm10_hi, xmm11);

        wr::<u128>(p1_elem, xmm1_rot);
        wr::<u128>(p1_elem + 0x10, xmm2_pos);
    }
}

/// `unknown_1421bf880`
/// Original: 0x1421bf880 (554 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421bf880(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u32,
    param_5_bits: u32,
) {
    if param_4 == 0 {
        return;
    }
    let param_5 = f32::from_bits(param_5_bits);
    let scale_ss = param_5 * rd::<f32>(exe::ptr::<f32>(0x1_4342_64b4) as usize);
    let xmm12 = vbroadcastss(scale_ss.to_bits());

    let c_8240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
    let c_81d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81d0) as usize);
    let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let c_81b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
    let c_8840 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
    let c_80c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize);
    let c_88d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
    let c_84c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
    let c_82c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize);
    let c_8280 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
    let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);

    for i in 0..(param_4 as usize) {
        let b = rd::<u8>(param_3.wrapping_add(i));
        let p1_elem = param_1.wrapping_add(i * 0x20);
        let p2_elem = param_2.wrapping_add(i * 0x20);
        let xmm10 = rd::<u128>(p1_elem);
        let p1_pos = rd::<u128>(p1_elem + 0x10);
        let p2_rot = rd::<u128>(p2_elem);
        let p2_pos = rd::<u128>(p2_elem + 0x10);

        let xmm0_b = vbroadcastss((b as i32 as f32).to_bits());
        let xmm2 = vmulps(xmm0_b, xmm12);
        let xmm0_pos = vmulps(xmm2, p2_pos);
        let xmm11 = vaddps(xmm0_pos, p1_pos);

        let xmm0_dp = vdpps(c_81d0, p2_rot, 0xff);
        let xmm7 = xmm0_dp & c_8760;
        let xmm5 = xmm7 ^ xmm0_dp;
        let mut xmm8 = vbroadcastss(xmm2 as u32);
        let xmm0_sub = vsubps(xmm8, c_8240);
        let mut xmm3 = vsubps(xmm8, c_8120);
        let xmm1_m = vmulps(xmm3, xmm8);
        let xmm6 = vmulps(xmm1_m, xmm0_sub);
        let xmm0_p1 = vfmadd231ps_128(c_8840, xmm5, c_80c0);
        let mut xmm4 = vfmadd231ps_128(c_81b0, xmm5, xmm0_p1);
        let xmm0_p2 = vfnmadd231ps_128(c_84c0, xmm5, c_82c0);
        let xmm1_p2 = vfmadd231ps_128(c_88d0, xmm5, xmm0_p2);
        let xmm0_p3 = vfmadd231ps_128(c_8280, xmm5, xmm1_p2);
        xmm3 = vmulps(xmm3, xmm3);
        xmm4 = vfmadd231ps_128(xmm4, xmm0_p3, xmm3);
        xmm8 = vfmadd231ps_128(xmm8, xmm4, xmm6);
        let xmm0_w = xmm7 ^ xmm8;
        let xmm5_perm = vpermilps(xmm10, 0x1b);
        let mut xmm4_q = vfnmadd231ps_128(c_81d0, xmm8, c_81d0);
        xmm4_q = vfmadd231ps_128(xmm4_q, xmm0_w, p2_rot);
        let xmm0_qdp = vdpps(xmm4_q, xmm4_q, 0xff);
        let xmm2_rsq = vrsqrtps_128(xmm0_qdp);
        let xmm0_r1 = vmulps(xmm2_rsq, xmm0_qdp);
        let xmm1_r2 = vmulps(xmm0_r1, xmm2_rsq);
        let xmm3_nr = vsubps(c_8460, xmm1_r2);
        let xmm2_half = vmulps(c_8120, xmm2_rsq);
        let xmm0_sc = vmulps(xmm3_nr, xmm2_half);
        let xmm1_q = vmulps(xmm0_sc, xmm4_q);

        let xmm3_q = vpermilps(xmm1_q, 0x11);
        let xmm2_q = vpermilps(xmm1_q, 0xbb);
        let xmm0_m1 = vmulps(xmm2_q, xmm5_perm);
        let xmm1_m1 = vmulps(xmm10, xmm3_q);
        let xmm4_hs = vhsubps(xmm1_m1, xmm0_m1);
        let xmm0_m2 = vmulps(xmm3_q, xmm5_perm);
        let xmm2_m2 = vmulps(xmm10, xmm2_q);
        let xmm3_ha = vhaddps(xmm2_m2, xmm0_m2);
        let xmm1_bl = vblendps(xmm3_ha, xmm4_hs, 0x0c);
        let xmm0_sh = vshufps(xmm4_hs, xmm3_ha, 0xb1);
        wr::<u128>(p1_elem + 0x10, xmm11);
        let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);
        let xmm2_out = vpermilps(xmm1_as, 0x9c);
        wr::<u128>(p1_elem, xmm2_out);
    }
}

/// `unknown_1421bfb00`
/// Original: 0x1421bfb00 (528 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421bfb00(
    param_1: usize,
    param_2: usize,
    param_3: u32,
    _param_4_f32: usize,
) {
    let [_, _, _, xmm3_in] = crate::simd_ops::take_xmm_args();
    if param_3 == 0 {
        return;
    }

    let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
    let c_81d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81d0) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let c_8240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
    let c_8840 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
    let c_80c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize);
    let c_81b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
    let c_84c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
    let c_82c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize);
    let c_88d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
    let c_8280 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
    let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);

    let xmm9 = vbroadcastss(xmm3_in as u32);
    let xmm2_sub = vsubps(xmm9, c_8240);
    let xmm10 = vbroadcastss(xmm3_in as u32);
    let xmm3_sub = vsubps(xmm9, c_8120);
    let xmm1_m = vmulps(xmm3_sub, xmm9);
    let xmm14 = vmulps(xmm2_sub, xmm1_m);
    let xmm0_sq = vmulps(xmm3_sub, xmm3_sub);

    for i in 0..(param_3 as usize) {
        let p1_elem = param_1.wrapping_add(i * 0x20);
        let p2_elem = param_2.wrapping_add(i * 0x20);
        let p2_rot = rd::<u128>(p2_elem);
        let p2_pos = rd::<u128>(p2_elem + 0x10);
        let xmm0_dp = vdpps(c_81d0, p2_rot, 0xff);
        let xmm7 = rd::<u128>(p1_elem);
        let mut xmm8 = rd::<u128>(p1_elem + 0x10);
        xmm8 = vfmadd231ps_128(xmm8, xmm10, p2_pos);

        let xmm5 = xmm0_dp & c_8760;
        let xmm3_abs = xmm5 ^ xmm0_dp;
        let xmm0_p1 = vfnmadd231ps_128(c_84c0, xmm3_abs, c_82c0);
        let xmm1_p1 = vfmadd231ps_128(c_8840, xmm3_abs, c_80c0);
        let mut xmm4 = vfmadd231ps_128(c_81b0, xmm3_abs, xmm1_p1);
        let xmm1_p2 = vfmadd231ps_128(c_88d0, xmm3_abs, xmm0_p1);
        let xmm0_p2 = vfmadd231ps_128(c_8280, xmm3_abs, xmm1_p2);
        xmm4 = vfmadd231ps_128(xmm4, xmm0_p2, xmm0_sq);
        let mut xmm0_w = vfmadd231ps_128(xmm9, xmm4, xmm14);
        let mut xmm4_q = vfnmadd231ps_128(c_81d0, xmm0_w, c_81d0);
        xmm0_w ^= xmm5;
        xmm4_q = vfmadd231ps_128(xmm4_q, xmm0_w, p2_rot);
        let xmm0_qdp = vdpps(xmm4_q, xmm4_q, 0xff);
        let xmm2_rsq = vrsqrtps_128(xmm0_qdp);
        let xmm0_r1 = vmulps(xmm2_rsq, xmm0_qdp);
        let xmm1_r2 = vmulps(xmm0_r1, xmm2_rsq);
        let xmm3_nr = vsubps(c_8460, xmm1_r2);
        let xmm2_half = vmulps(c_8120, xmm2_rsq);
        let xmm0_sc = vmulps(xmm3_nr, xmm2_half);
        let xmm1_q = vmulps(xmm0_sc, xmm4_q);

        let xmm3_q = vpermilps(xmm1_q, 0x11);
        let xmm2_q = vpermilps(xmm1_q, 0xbb);
        let xmm1_m1 = vmulps(xmm7, xmm3_q);
        let xmm5_perm = vpermilps(xmm7, 0x1b);
        let xmm0_m1 = vmulps(xmm2_q, xmm5_perm);
        let xmm4_hs = vhsubps(xmm1_m1, xmm0_m1);
        let xmm2_m2 = vmulps(xmm7, xmm2_q);
        let xmm0_m2 = vmulps(xmm3_q, xmm5_perm);
        let xmm3_ha = vhaddps(xmm2_m2, xmm0_m2);
        wr::<u128>(p1_elem + 0x10, xmm8);
        let xmm1_bl = vblendps(xmm3_ha, xmm4_hs, 0x0c);
        let xmm0_sh = vshufps(xmm4_hs, xmm3_ha, 0xb1);
        let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);
        let xmm2_out = vpermilps(xmm1_as, 0x9c);
        wr::<u128>(p1_elem, xmm2_out);
    }
}

/// `unknown_1421bfd20`
/// Original: 0x1421bfd20 (484 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421bfd20(
    param_1: usize,
    param_2: usize,
    param_3: u32,
    _param_4_f32: usize,
) {
    let [_, _, _, xmm3_in] = crate::simd_ops::take_xmm_args();
    if param_3 == 0 {
        return;
    }

    let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let c_81b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
    let c_8240 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
    let c_8840 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
    let c_8280 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
    let c_80c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize);
    let c_84c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
    let c_82c0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize);
    let c_88d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
    let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);

    let xmm9 = vbroadcastss(xmm3_in as u32);
    let xmm8 = vbroadcastss(xmm3_in as u32);
    let xmm2_sub = vsubps(xmm8, c_8240);
    let xmm3_sub = vsubps(xmm8, c_8120);
    let xmm1_m = vmulps(xmm3_sub, xmm8);
    let xmm0_sq = vmulps(xmm3_sub, xmm3_sub);
    let xmm12 = vmulps(xmm2_sub, xmm1_m);

    for i in 0..(param_3 as usize) {
        let p1_elem = param_1.wrapping_add(i * 0x20);
        let p2_elem = param_2.wrapping_add(i * 0x20);
        let ymm6_lo = rd::<u128>(p1_elem);
        let ymm6_hi = rd::<u128>(p1_elem + 0x10);
        let ymm7_lo = rd::<u128>(p2_elem);
        let ymm7_hi = rd::<u128>(p2_elem + 0x10);

        let xmm0_dp = vdpps(ymm6_lo, ymm7_lo, 0xff);
        let xmm5 = xmm0_dp & c_8760;
        let xmm3_abs = xmm5 ^ xmm0_dp;
        let xmm0_p1 = vfnmadd231ps_128(c_84c0, xmm3_abs, c_82c0);
        let xmm1_p1 = vfmadd231ps_128(c_8840, xmm3_abs, c_80c0);
        let mut xmm4 = vfmadd231ps_128(c_81b0, xmm3_abs, xmm1_p1);
        let xmm1_p2 = vfmadd231ps_128(c_88d0, xmm3_abs, xmm0_p1);
        let xmm2_p2 = vfmadd231ps_128(c_8280, xmm3_abs, xmm1_p2);
        xmm4 = vfmadd231ps_128(xmm4, xmm2_p2, xmm0_sq);
        let mut xmm0_w = vfmadd231ps_128(xmm8, xmm4, xmm12);
        let mut xmm4_q = vfnmadd231ps_128(ymm6_lo, xmm0_w, ymm6_lo);
        xmm0_w ^= xmm5;
        xmm4_q = vfmadd231ps_128(xmm4_q, xmm0_w, ymm7_lo);
        let xmm0_qdp = vdpps(xmm4_q, xmm4_q, 0xff);
        let xmm2_rsq = vrsqrtps_128(xmm0_qdp);
        let xmm0_r1 = vmulps(xmm2_rsq, xmm0_qdp);
        let xmm1_r2 = vmulps(xmm0_r1, xmm2_rsq);
        let xmm2_half = vmulps(c_8120, xmm2_rsq);
        let xmm3_nr = vsubps(c_8460, xmm1_r2);
        let xmm0_sc = vmulps(xmm3_nr, xmm2_half);
        let xmm1_rot = vmulps(xmm0_sc, xmm4_q);

        let mut xmm2_pos = vfnmadd231ps_128(ymm6_hi, ymm6_hi, xmm9);
        xmm2_pos = vfmadd231ps_128(xmm2_pos, ymm7_hi, xmm9);

        wr::<u128>(p1_elem, xmm1_rot);
        wr::<u128>(p1_elem + 0x10, xmm2_pos);
    }
}

/// `unknown_1421bff10`
/// Original: 0x1421bff10 (142 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421bff10(param_1: usize, param_2: usize, param_3: u32) {
    if param_3 == 0 {
        return;
    }
    let mut r9 = param_1;
    let mut rdx = param_2;
    for _ in 0..(param_3 as usize) {
        let ax = rd::<u16>(rdx);
        if ax != 0xffff {
            let cur = rd::<u128>(r9);
            let xmm3 = vpermilps(cur, 0x11);
            let xmm2 = vpermilps(cur, 0xbb);
            let xmm6 = rd::<u128>(param_1.wrapping_add((ax as usize) << 5));
            let xmm1 = vmulps(xmm3, xmm6);
            let xmm5 = vpermilps(xmm6, 0x1b);
            let xmm0 = vmulps(xmm2, xmm5);
            let xmm4 = vhsubps(xmm1, xmm0);
            let xmm0_2 = vmulps(xmm3, xmm5);
            let xmm2_2 = vmulps(xmm2, xmm6);
            let xmm3_2 = vhaddps(xmm2_2, xmm0_2);
            let xmm1_bl = vblendps(xmm3_2, xmm4, 0x0c);
            let xmm0_sh = vshufps(xmm4, xmm3_2, 0xb1);
            let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);
            let xmm2_out = vpermilps(xmm1_as, 0x9c);
            wr::<u128>(r9, xmm2_out);
        }
        rdx = rdx.wrapping_add(2);
        r9 = r9.wrapping_add(0x20);
    }
}

/// `unknown_1421c0270`
/// Original: 0x1421c0270 (164 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421c0270(param_1: usize, param_2: usize, param_3: i32) {
    let mut rax = (param_3.wrapping_sub(1)) as i64;
    if rax < 0 {
        return;
    }
    let c_7b00 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7b00) as usize);
    let mut r8 = param_1.wrapping_add((rax as usize) << 5);
    loop {
        let cx = rd::<u16>(param_2.wrapping_add((rax as usize) * 2));
        if cx != 0xffff {
            let cur = rd::<u128>(r8);
            let xmm3 = vpermilps(cur, 0x11);
            let xmm2 = vpermilps(cur, 0xbb);
            let xmm1 = rd::<u128>(param_1.wrapping_add((cx as usize) << 5));
            let xmm6 = xmm1 ^ c_7b00;
            let xmm1_m = vmulps(xmm6, xmm3);
            let xmm5 = vpermilps(xmm6, 0x1b);
            let xmm0 = vmulps(xmm2, xmm5);
            let xmm4 = vhsubps(xmm1_m, xmm0);
            let xmm0_2 = vmulps(xmm3, xmm5);
            let xmm2_2 = vmulps(xmm6, xmm2);
            let xmm3_2 = vhaddps(xmm2_2, xmm0_2);
            let xmm1_bl = vblendps(xmm3_2, xmm4, 0x0c);
            let xmm0_sh = vshufps(xmm4, xmm3_2, 0xb1);
            let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);
            let xmm2_out = vpermilps(xmm1_as, 0x9c);
            wr::<u128>(r8, xmm2_out);
        }
        r8 = r8.wrapping_sub(0x20);
        rax -= 1;
        if rax < 0 {
            break;
        }
    }
}

/// `unknown_1421c6680`
/// Original: 0x1421c6680 (609 bytes, Throws::No, FLOAT_RETURN)
pub unsafe extern "win64" fn unknown_1421c6680(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5_bits: u32,
    param_6_bits: u32,
    param_7_bits: u32,
) -> f32 {
    let xmm1_in = rd::<u128>(param_3);
    let xmm0_sh = vshufps(xmm1_in, xmm1_in, 0x44);
    let xmm2_m = vmulps(xmm0_sh, xmm1_in);
    let c_83b0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_83b0) as usize);
    let xmm4 = vmulps(xmm2_m, c_83b0);
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm3_w = f32::from_bits(vpermilps(xmm4, 0xff) as u32);
    let xmm0_z = f32::from_bits(vpermilps(xmm4, 0xaa) as u32);
    let y0 = xmm3_w + xmm0_z;
    let xmm4_x = f32::from_bits(xmm4 as u32);
    let xmm2_y = f32::from_bits(vpermilps(xmm4, 0x55) as u32);
    let x0 = (one - xmm4_x) - xmm2_y;
    let atan0 = call_atan2f(y0, x0);

    let c_8760 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let xmm9 = (atan0.to_bits() as u128) ^ c_8760;

    let v_e0 = rd::<u128>(param_1 + 0xe0);
    let dp_e0 = vdpps(v_e0, v_e0, 0x7f);
    let c_7f40 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7f40) as usize);
    let xmm0_eps = vaddps(dp_e0, c_7f40);
    let xmm1_rsq = vrsqrtps_128(xmm0_eps);
    let xmm2_len = f32::from_bits(vmulps(xmm1_rsq, dp_e0) as u32);
    let min_len = rd::<f32>(exe::ptr::<f32>(0x1_4342_65d4) as usize);
    if !(xmm2_len > min_len) {
        crate::simd_ops::set_xmm_ret(xmm9);
        return f32::from_bits(xmm9 as u32);
    }

    let diff_5_7 = f32::from_bits(param_5_bits) - f32::from_bits(param_7_bits);
    let xmm2_t = if diff_5_7 > 0.0 { diff_5_7 } else { 0.0 };
    let mut local_30 = [0u128; 2];
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_421b_e220,
        local_30.as_mut_ptr() as usize as u64,
        param_4 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm2_t.to_bits() as u128, param_6_bits as u128],
    );

    let mut local_20: u128 = 0;
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_421c_1c80,
        param_2 as u64,
        (&mut local_20 as *mut u128) as usize as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, param_6_bits as u128, 0],
    );

    let ymm8_lo = rd::<u128>(param_1 + 0x90);
    let ymm8_hi = rd::<u128>(param_1 + 0xa0);
    let xmm7 = local_20;
    let xmm4_p = vpermilps(ymm8_lo, 0xc9);
    let xmm6_p = vpermilps(ymm8_lo, 0xd2);
    let xmm0_1 = vpermilps(xmm7, 0xd2);
    let xmm3_1 = vmulps(xmm0_1, xmm4_p);
    let xmm1_1 = vpermilps(xmm7, 0xc9);
    let xmm2_1 = vmulps(xmm1_1, xmm6_p);
    let c_7ab0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7ab0) as usize);
    let xmm5 = vmulps(vsubps(xmm3_1, xmm2_1), c_7ab0);
    let xmm1_2 = vpermilps(xmm5, 0xd2);
    let xmm3_2 = vmulps(xmm1_2, xmm4_p);
    let xmm1_w = vpermilps(ymm8_lo, 0xff);
    let xmm0_2 = vpermilps(xmm5, 0xc9);
    let xmm2_2 = vmulps(xmm0_2, xmm6_p);
    let xmm0_w = vmulps(xmm1_w, xmm5);
    let xmm4_sub = vsubps(xmm3_2, xmm2_2);
    let xmm2_add = vaddps(xmm0_w, xmm7);
    let xmm3_add = vaddps(xmm4_sub, xmm2_add);
    let xmm2_vec = vaddps(xmm3_add, ymm8_hi);

    let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4342_64a4) as usize);
    let dp1 = f32::from_bits(vdpps(xmm2_vec, xmm2_vec, 0x5f) as u32);
    if !(dp1 >= thresh) {
        crate::simd_ops::set_xmm_ret(xmm9);
        return f32::from_bits(xmm9 as u32);
    }

    let xmm7_hi = local_30[1];
    let dp2 = f32::from_bits(vdpps(xmm7_hi, xmm7_hi, 0x5f) as u32);
    if !(dp2 >= thresh) {
        crate::simd_ops::set_xmm_ret(xmm9);
        return f32::from_bits(xmm9 as u32);
    }

    let a1 = call_atan2f(
        f32::from_bits(xmm2_vec as u32),
        f32::from_bits(vpermilps(xmm2_vec, 0xaa) as u32),
    );
    let a2 = call_atan2f(
        f32::from_bits(xmm7_hi as u32),
        f32::from_bits(vshufps(xmm7_hi, xmm7_hi, 0xaa) as u32),
    );
    let c_pi = rd::<f32>(exe::ptr::<f32>(0x1_4342_6c2c) as usize);
    let xmm2_ss = (a1 - a2) + c_pi;
    let xmm6_2pi = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d88) as usize);
    let xmm1_sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);

    let xmm3_v = xmm2_ss.to_bits() as u128;
    let xmm4_rcp = vrcpss_f32(xmm6_2pi);
    let xmm0_sq = (xmm4_rcp * xmm4_rcp) * xmm6_2pi;
    let xmm5_mask = vcmpss(0, xmm2_ss.to_bits(), 0, 1);
    let xmm2_abs = ((!xmm1_sign_mask) & xmm2_ss.to_bits()) as u128;
    let xmm3_blended = f32::from_bits(vblendvps(xmm3_v, xmm2_abs, xmm5_mask) as u32);
    let xmm4_nr = (xmm4_rcp + xmm4_rcp) - xmm0_sq;
    let mut xmm3_scaled = xmm3_blended * xmm4_nr;
    let xmm1_trunc = f32::from_bits(vroundps(xmm3_scaled.to_bits() as u128, 3) as u32);
    xmm3_scaled = (xmm3_scaled - xmm1_trunc) * xmm6_2pi;
    let xmm6_rem = xmm6_2pi - xmm3_scaled;
    let xmm1_final = f32::from_bits(
        vblendvps(
            xmm3_scaled.to_bits() as u128,
            xmm6_rem.to_bits() as u128,
            xmm5_mask,
        ) as u32,
    );
    let res = xmm1_final - c_pi;
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

/// `unknown_1421d6da0`
/// Original: 0x1421d6da0 (348 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421d6da0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut edi = rd::<u32>(param_3 + 0x1c);
    let r14 = param_2.wrapping_add(0x08);
    let xmm1_in = rd::<f32>(param_2 + 0x0c);
    let mut ebx = rd::<u32>(param_1 + 0x4c);
    let rdx_off = rd::<i32>(r14) as i64;
    let r15d = rd::<u32>(param_3 + 0x18);
    let eax_idx = (edi & 3) as usize;
    let r8_base = if rdx_off == 0 {
        0usize
    } else {
        r14.wrapping_add(rdx_off as usize)
    };
    let ecx_byte = rd::<u8>(param_3 + eax_idx * 8 + 0x24) as usize;
    let xmm2_rcp = vrcpss_f32(xmm1_in);
    let xmm9_ss = if xmm2_rcp.is_nan() { 0.0f32 } else { xmm2_rcp };

    let rax_slot = r8_base.wrapping_add(ecx_byte * 4);
    let rcx_off = rd::<i32>(rax_slot) as i64;
    let rdx_src = if rcx_off == 0 {
        0usize
    } else {
        rax_slot.wrapping_add(rcx_off as usize)
    };
    exe::call_va!(0x1_4183_acbc, (param_4, rdx_src, ebx as u64));

    if edi == r15d {
        return;
    }

    ebx = ebx.wrapping_add(3) & !3u32;
    let xmm7_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm8_three = rd::<f32>(exe::ptr::<f32>(0x1_4342_6bf8) as usize);

    while edi != r15d {
        let rax_off = rd::<i32>(r14) as i64;
        let edx_idx = (edi & 3) as usize;
        let rcx_base = if rax_off == 0 {
            0usize
        } else {
            r14.wrapping_add(rax_off as usize)
        };
        let xmm1_prod = xmm9_ss * rd::<f32>(param_3 + edx_idx * 8 + 0x20);
        let eax_byte = rd::<u8>(param_3 + edx_idx * 8 + 0x25) as usize;
        let mut xmm2_t = if xmm1_prod > 0.0 { xmm1_prod } else { 0.0 };
        xmm2_t = if xmm2_t < xmm7_one { xmm2_t } else { xmm7_one };

        let rcx_slot = rcx_base.wrapping_add(eax_byte * 4);
        let rax_rel = rd::<i32>(rcx_slot) as i64;
        let xmm1_sq = xmm2_t * xmm2_t;
        let xmm0_cb = xmm1_sq * xmm2_t;
        let rdx_src2 = if rax_rel == 0 {
            0usize
        } else {
            rcx_slot.wrapping_add(rax_rel as usize)
        };
        let xmm0_2cb = xmm0_cb + xmm0_cb;
        let xmm1_3sq = xmm1_sq * xmm8_three;
        let xmm3_smooth = xmm1_3sq - xmm0_2cb;
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_421b_ddb0,
            param_4 as u64,
            rdx_src2 as u64,
            ebx as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, xmm3_smooth.to_bits() as u128],
        );
        edi = edi.wrapping_add(1);
    }
}

/// `unknown_1421d7250`
/// Original: 0x1421d7250 (512 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421d7250(param_1: usize) {
    let rsi = rd::<usize>(param_1 + 0x08);
    let r14 = rd::<usize>(param_1);
    if rd::<u8>(rsi + 0x15) != 0 {
        return;
    }

    if rd::<u16>(param_1 + 0x1b4) != 0 {
        let cl = rd::<u8>(r14 + 0x05);
        let r8d = rd::<u32>(param_1 + 0x4c);
        let mut rdx = rd::<usize>(param_1 + 0x20);
        let r9 = (r8d as usize) << 5;
        let xmm3_bits = rd::<u32>(rsi + 0x0c);
        rdx = rdx.wrapping_sub(r9);
        wr::<usize>(param_1 + 0x20, rdx);
        wr::<u32>(param_1 + 0x14, rd::<u32>(param_1 + 0x14).wrapping_sub(1));
        let rcx = rdx.wrapping_sub(r9);
        if (cl & 1) != 0 {
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_421b_fb00,
                rcx as u64,
                rdx as u64,
                r8d as u64,
                0,
                0,
                0,
                0,
                0,
                [0, 0, 0, xmm3_bits as u128],
            );
        } else {
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_421b_fd20,
                rcx as u64,
                rdx as u64,
                r8d as u64,
                0,
                0,
                0,
                0,
                0,
                [0, 0, 0, xmm3_bits as u128],
            );
        }
        return;
    }

    let mut dyn_buf: Vec<u8>;
    let rdi_weights = if rd::<u32>(rsi + 0x1c) == rd::<u32>(rsi + 0x18) {
        let rax = r14.wrapping_add(0x08);
        let rcx_off = rd::<i32>(rax) as i64;
        let rdx_base = if rcx_off == 0 {
            0usize
        } else {
            rax.wrapping_add(rcx_off as usize)
        };
        let eax_byte = rd::<u8>(rsi + 0x14) as usize;
        let rcx_slot = rdx_base.wrapping_add(eax_byte * 4);
        let rax_rel = rd::<i32>(rcx_slot) as i64;
        if rax_rel == 0 {
            0usize
        } else {
            rcx_slot.wrapping_add(rax_rel as usize)
        }
    } else {
        let cnt = rd::<u32>(param_1 + 0x4c) as usize;
        dyn_buf = vec![0u8; cnt + 32];
        let aligned = (dyn_buf.as_mut_ptr() as usize + 15) & !15usize;
        exe::call_va!(0x1_421d_6da0, (param_1, r14, rsi, aligned));
        aligned
    };

    let cl = rd::<u8>(r14 + 0x05);
    let xmm6_bits = rd::<u32>(rsi + 0x0c);
    let esi_cnt = rd::<u32>(param_1 + 0x4c);
    let r8b = cl & 2;
    let rdx_bytes = (esi_cnt as usize) << 5;
    let r11 = rd::<usize>(param_1 + 0x20).wrapping_sub(rdx_bytes);
    wr::<usize>(param_1 + 0x20, r11);
    wr::<u32>(param_1 + 0x14, rd::<u32>(param_1 + 0x14).wrapping_sub(1));
    let r14_dst = r11.wrapping_sub(rdx_bytes);

    if (cl & 1) == 0 {
        if r8b != 0 {
            exe::call_va!(0x1_421b_ff10, (r11, rd::<usize>(param_1 + 0x50), esi_cnt));
            exe::call_va!(0x1_421b_ff10, (r14_dst, rd::<usize>(param_1 + 0x50), esi_cnt));
            exe::call_va!(0x1_421b_f670, (r14_dst, r11, rdi_weights, esi_cnt, xmm6_bits));
            exe::call_va!(0x1_421c_0270, (r14_dst, rd::<usize>(param_1 + 0x50), esi_cnt));
        } else {
            exe::call_va!(0x1_421b_f670, (r14_dst, r11, rdi_weights, esi_cnt, xmm6_bits));
        }
    } else if r8b != 0 {
        exe::call_va!(0x1_421b_ff10, (r14_dst, rd::<usize>(param_1 + 0x50), esi_cnt));
        exe::call_va!(0x1_421b_f880, (r14_dst, r11, rdi_weights, esi_cnt, xmm6_bits));
        exe::call_va!(0x1_421c_0270, (r14_dst, rd::<usize>(param_1 + 0x50), esi_cnt));
    } else {
        exe::call_va!(0x1_421b_f880, (r14_dst, r11, rdi_weights, esi_cnt, xmm6_bits));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

struct GlobalSave {
    addr: usize,
    bytes: Vec<u8>,
}

impl GlobalSave {
    unsafe fn new(va: u64, len: usize) -> Self {
        let addr = exe::ptr::<u8>(va) as usize;
        let mut bytes = vec![0u8; len];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = rd::<u8>(addr + i);
        }
        Self { addr, bytes }
    }
}

impl Drop for GlobalSave {
    fn drop(&mut self) {
        unsafe {
            for (i, &b) in self.bytes.iter().enumerate() {
                wr::<u8>(self.addr + i, b);
            }
        }
    }
}

pub unsafe fn diff_physics_14216fe00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_CALLS: Vec<(u32, u64)> = Vec::new();

    unsafe extern "win64" fn mock_0ce0(vec_ptr: usize, cap: u64) {
        wr::<u32>(vec_ptr + 0x08, 3);
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
        LOG_CALLS.push((1, cap));
    }
    unsafe extern "win64" fn mock_0290(vec_ptr: usize, cap: u64) {
        wr::<u32>(vec_ptr + 0x08, 5);
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
        LOG_CALLS.push((2, cap));
    }
    unsafe extern "win64" fn mock_0090(vec_ptr: usize, cap: u64) {
        wr::<u32>(vec_ptr + 0x08, 7);
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
        LOG_CALLS.push((3, cap));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4127_0ce0, mock_0ce0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4127_0290, mock_0290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4127_0090, mock_0090 as *const ())?;

    let run = |f: F, seed: u8| -> (bool, Vec<u8>, Vec<(u32, u64)>) {
        LOG_CALLS.clear();
        let mut buf = vec![seed; 0x60];
        let p = buf.as_mut_ptr() as usize;
        let ret = f(p);
        (ret == p, buf, LOG_CALLS.clone())
    };

    for seed in [0u8, 0xffu8] {
        let o = run(f_o, seed);
        let n = run(f_n, seed);
        crate::diff_eq!(o, n, "14216fe00 seed={}", seed);
    }
    Ok(2)
}

pub unsafe fn diff_r_1421825c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MGR_MEM: [u8; 0x2d60] = [0; 0x2d60];
    static mut SRC_ITEMS: [u8; 3 * 0x2a8] = [0; 3 * 0x2a8];
    static mut DST_ITEMS: [u8; 4 * 0x2a8] = [0; 4 * 0x2a8];
    static mut LOG_DTOR: u32 = 0;
    static mut LOG_RESERVE: Vec<u64> = Vec::new();
    static mut LOG_COPY: Vec<u32> = Vec::new();

    unsafe extern "win64" fn mock_get_mgr(_this: usize) -> usize {
        core::ptr::addr_of_mut!(MGR_MEM) as usize
    }
    unsafe extern "win64" fn mock_ff80(_elem: usize) {
        LOG_DTOR += 1;
    }
    unsafe extern "win64" fn mock_1780(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_RESERVE.push(new_cap);
    }
    unsafe extern "win64" fn mock_1570(dst: usize, src: usize) {
        let id = rd::<u32>(src);
        wr::<u32>(dst, id);
        LOG_COPY.push(id);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4128_ff80, mock_ff80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4129_1780, mock_1780 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4129_1570, mock_1570 as *const ())?;

    let run = |f: F, p3: i32, flag_940: u8| -> (u32, u32, u32, Vec<u64>, Vec<u32>) {
        LOG_DTOR = 0;
        LOG_RESERVE.clear();
        LOG_COPY.clear();
        MGR_MEM = [0; 0x2d60];
        SRC_ITEMS = [0; 3 * 0x2a8];
        DST_ITEMS = [0; 4 * 0x2a8];

        let sp = core::ptr::addr_of_mut!(SRC_ITEMS) as usize;
        // Item 0: inactive (+0x04 == 0)
        wr::<u32>(sp, 10);
        wr::<u32>(sp + 0x04, 0);
        // Item 1: active, +0x128 = 1 (filtered when p3 == 1 && flag_940 == 0)
        wr::<u32>(sp + 0x2a8, 20);
        wr::<u32>(sp + 0x2a8 + 0x04, 1);
        wr::<u32>(sp + 0x2a8 + 0x128, 1);
        // Item 2: active, +0x128 = 2 (always kept)
        wr::<u32>(sp + 2 * 0x2a8, 30);
        wr::<u32>(sp + 2 * 0x2a8 + 0x04, 1);
        wr::<u32>(sp + 2 * 0x2a8 + 0x128, 2);

        let mp = core::ptr::addr_of_mut!(MGR_MEM) as usize;
        wr::<usize>(mp + 0x08, sp);
        wr::<u32>(mp + 0x10, 3);

        let mut vtbl = [0usize; 8];
        vtbl[0x28 / 8] = mock_get_mgr as *const () as usize;
        let mut p1 = vec![0u8; 0x950];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, vtbl.as_ptr() as usize);
        wr::<u8>(p1p + 0x940, flag_940);

        let dp = core::ptr::addr_of_mut!(DST_ITEMS) as usize;
        let mut p2 = [0u8; 0x10];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, dp);
        wr::<u32>(p2p + 0x08, 1);
        wr::<u32>(p2p + 0x0c, 1);

        f(p1p, p2p, p3);
        (
            LOG_DTOR,
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x0c),
            LOG_RESERVE.clone(),
            LOG_COPY.clone(),
        )
    };

    for (p3, flag_940) in [(1i32, 0u8), (1i32, 1u8), (0i32, 0u8)] {
        let o = run(f_o, p3, flag_940);
        let n = run(f_n, p3, flag_940);
        crate::diff_eq!(o, n, "1421825c0 p3={} flag={}", p3, flag_940);
    }
    Ok(3)
}

pub unsafe fn diff_r_142183fb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUF: [u8; 4 * 0x90] = [0; 4 * 0x90];
    static mut LOG_AB10: u32 = 0;
    static mut LOG_1E20: Vec<u64> = Vec::new();
    static mut LOG_3100: Vec<u32> = Vec::new();
    static mut LOG_1860: u32 = 0;

    unsafe extern "win64" fn mock_25c0(_p1: usize, arr_ptr: usize, p3: i32) {
        let base = rd::<usize>(arr_ptr);
        wr::<u32>(base, 100 + p3 as u32);
        wr::<u32>(base + 0x2a8, 200 + p3 as u32);
        wr::<u32>(arr_ptr + 0x08, 2);
    }
    unsafe extern "win64" fn mock_ab10(_str_ptr: usize) {
        LOG_AB10 += 1;
    }
    unsafe extern "win64" fn mock_1e20(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_1E20.push(new_cap);
    }
    unsafe extern "win64" fn mock_3100(dst: usize, src: usize) {
        let v = rd::<u32>(src);
        wr::<u32>(dst, v);
        LOG_3100.push(v);
    }
    unsafe extern "win64" fn mock_1860(_arr_ptr: usize) {
        LOG_1860 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4218_25c0, mock_25c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4129_1e20, mock_1e20 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401f_3100, mock_3100 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4129_1860, mock_1860 as *const ())?;

    let run = |f: F, p1: usize, init_len: u32, init_cap: u32| -> (u32, u32, u32, Vec<u64>, Vec<u32>, u32) {
        LOG_AB10 = 0;
        LOG_1E20.clear();
        LOG_3100.clear();
        LOG_1860 = 0;
        DST_BUF = [0; 4 * 0x90];

        let dp = core::ptr::addr_of_mut!(DST_BUF) as usize;
        wr::<u64>(dp + 0x40, 0xdead_beef);
        let mut p2 = [0u8; 0x10];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, dp);
        wr::<u32>(p2p + 0x08, init_len);
        wr::<u32>(p2p + 0x0c, init_cap);

        f(p1, p2p);
        (
            LOG_AB10,
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x0c),
            LOG_1E20.clone(),
            LOG_3100.clone(),
            LOG_1860,
        )
    };

    for (p1, len, cap) in [(0usize, 1u32, 1u32), (0x1000usize, 1u32, 1u32), (0x1000usize, 0u32, 4u32)] {
        let o = run(f_o, p1, len, cap);
        let n = run(f_n, p1, len, cap);
        crate::diff_eq!(o, n, "142183fb0 p1={} len={} cap={}", p1, len, cap);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421be220(orig: usize, new: usize) -> Result<u32, String> {
    let _g1 = GlobalSave::new(0x1_4386_6090, 8);

    unsafe extern "win64" fn mock_sample(out_tf: usize, _p2: usize, _x2: usize) {
        let [_, _, xmm2, _] = crate::simd_ops::take_xmm_args();
        let t = f32::from_bits(xmm2 as u32);
        let s = (t * 0.5).sin();
        let c = (t * 0.5).cos();
        wr::<[f32; 8]>(out_tf, [0.0, s, 0.0, c, t * 2.0, t * 1.5, t * 0.5, 0.0]);
    }

    wr::<usize>(
        exe::ptr::<usize>(0x1_4386_6090) as usize,
        mock_sample as *const () as usize,
    );

    let run = |fn_addr: usize, target_dist: f32| -> (bool, [u32; 8]) {
        let mut p2 = [0u8; 0x20];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u16>(p2p + 0x0e, 0);
        wr::<f32>(p2p + 0x14, 1.0);
        wr::<f32>(p2p + 0x18, 4.0);

        let mut out_tf = [0u32; 8];
        let out_p = out_tf.as_mut_ptr() as usize;
        let (rax, _) = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            out_p as u64,
            p2p as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0.1f32.to_bits() as u128, target_dist.to_bits() as u128],
        );
        (rax as usize == out_p, out_tf)
    };

    for dist in [0.25f32, 10.0f32] {
        let o = run(orig, dist);
        let n = run(new, dist);
        crate::diff_eq!(o, n, "1421be220 dist={}", dist);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421bf670(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u32, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, cnt: u32, w_scale: f32| -> [[u32; 8]; 2] {
        let mut p1: [[f32; 8]; 2] = [
            [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0],
            [0.0, 0.70710677, 0.0, 0.70710677, -1.0, 4.0, 0.5, 0.0],
        ];
        let p2: [[f32; 8]; 2] = [
            [0.0, 0.70710677, 0.0, 0.70710677, 5.0, 6.0, 7.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 2.0, -3.0, 8.0, 0.0],
        ];
        let weights: [u8; 2] = [64, 192];
        f(
            p1.as_mut_ptr() as usize,
            p2.as_ptr() as usize,
            weights.as_ptr() as usize,
            cnt,
            w_scale.to_bits(),
        );
        core::mem::transmute(p1)
    };

    for (cnt, w) in [(0u32, 1.0f32), (2u32, 0.5f32), (2u32, 1.0f32)] {
        let o = run(f_o, cnt, w);
        let n = run(f_n, cnt, w);
        crate::diff_eq!(o, n, "1421bf670 cnt={} w={}", cnt, w);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421bf880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u32, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, cnt: u32, w_scale: f32| -> [[u32; 8]; 2] {
        let mut p1: [[f32; 8]; 2] = [
            [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0],
            [0.0, 0.70710677, 0.0, 0.70710677, -1.0, 4.0, 0.5, 0.0],
        ];
        let p2: [[f32; 8]; 2] = [
            [0.0, 0.70710677, 0.0, 0.70710677, 5.0, 6.0, 7.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 2.0, -3.0, 8.0, 0.0],
        ];
        let weights: [u8; 2] = [80, 160];
        f(
            p1.as_mut_ptr() as usize,
            p2.as_ptr() as usize,
            weights.as_ptr() as usize,
            cnt,
            w_scale.to_bits(),
        );
        core::mem::transmute(p1)
    };

    for (cnt, w) in [(0u32, 1.0f32), (2u32, 0.5f32), (2u32, 1.0f32)] {
        let o = run(f_o, cnt, w);
        let n = run(f_n, cnt, w);
        crate::diff_eq!(o, n, "1421bf880 cnt={} w={}", cnt, w);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421bfb00(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, cnt: u32, w: f32| -> [[u32; 8]; 2] {
        let mut p1: [[f32; 8]; 2] = [
            [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0],
            [0.0, 0.70710677, 0.0, 0.70710677, -1.0, 4.0, 0.5, 0.0],
        ];
        let p2: [[f32; 8]; 2] = [
            [0.0, 0.70710677, 0.0, 0.70710677, 5.0, 6.0, 7.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 2.0, -3.0, 8.0, 0.0],
        ];
        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1.as_mut_ptr() as usize as u64,
            p2.as_ptr() as usize as u64,
            cnt as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, w.to_bits() as u128],
        );
        core::mem::transmute(p1)
    };

    for (cnt, w) in [(0u32, 0.5f32), (2u32, 0.25f32), (2u32, 0.75f32)] {
        let o = run(orig, cnt, w);
        let n = run(new, cnt, w);
        crate::diff_eq!(o, n, "1421bfb00 cnt={} w={}", cnt, w);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421bfd20(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, cnt: u32, w: f32| -> [[u32; 8]; 2] {
        let mut p1: [[f32; 8]; 2] = [
            [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0],
            [0.0, 0.70710677, 0.0, 0.70710677, -1.0, 4.0, 0.5, 0.0],
        ];
        let p2: [[f32; 8]; 2] = [
            [0.0, 0.70710677, 0.0, 0.70710677, 5.0, 6.0, 7.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 2.0, -3.0, 8.0, 0.0],
        ];
        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1.as_mut_ptr() as usize as u64,
            p2.as_ptr() as usize as u64,
            cnt as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, w.to_bits() as u128],
        );
        core::mem::transmute(p1)
    };

    for (cnt, w) in [(0u32, 0.5f32), (2u32, 0.25f32), (2u32, 0.75f32)] {
        let o = run(orig, cnt, w);
        let n = run(new, cnt, w);
        crate::diff_eq!(o, n, "1421bfd20 cnt={} w={}", cnt, w);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1421bff10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, cnt: u32| -> [[u32; 8]; 3] {
        let mut tfs: [[f32; 8]; 3] = [
            [0.0, 0.70710677, 0.0, 0.70710677, 1.0, 2.0, 3.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 4.0, 5.0, 6.0, 0.0],
            [0.70710677, 0.0, 0.0, 0.70710677, 7.0, 8.0, 9.0, 0.0],
        ];
        let parents: [u16; 3] = [0xffff, 0, 1];
        f(tfs.as_mut_ptr() as usize, parents.as_ptr() as usize, cnt);
        core::mem::transmute(tfs)
    };

    for cnt in [0u32, 3u32] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1421bff10 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421c0270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, cnt: i32| -> [[u32; 8]; 3] {
        let mut tfs: [[f32; 8]; 3] = [
            [0.0, 0.70710677, 0.0, 0.70710677, 1.0, 2.0, 3.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 4.0, 5.0, 6.0, 0.0],
            [0.70710677, 0.0, 0.0, 0.70710677, 7.0, 8.0, 9.0, 0.0],
        ];
        let parents: [u16; 3] = [0xffff, 0, 1];
        f(tfs.as_mut_ptr() as usize, parents.as_ptr() as usize, cnt);
        core::mem::transmute(tfs)
    };

    for cnt in [0i32, 3i32] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1421c0270 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421c6680(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn mock_be220(out_30: usize, _p4: usize, _x2: usize, _x3: usize) -> usize {
        let [_, _, xmm2, xmm3] = crate::simd_ops::take_xmm_args();
        let t = f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32) * 0.1;
        wr::<[f32; 8]>(out_30, [0.0, 0.0, 0.0, 1.0, 1.5 + t, 0.0, 2.0, 0.0]);
        out_30
    }
    unsafe extern "win64" fn mock_c1c80(_p2: usize, out_20: usize, _x2: usize) {
        wr::<[f32; 4]>(out_20, [0.5, 0.0, 1.2, 0.0]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421b_e220, mock_be220 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421c_1c80, mock_c1c80 as *const ())?;

    let run = |fn_addr: usize, vel_len: f32| -> u32 {
        let mut p1 = [0u8; 0x100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<[f32; 8]>(p1p + 0x90, [0.0, 0.0, 0.0, 1.0, 0.2, 0.0, 0.3, 0.0]);
        wr::<[f32; 4]>(p1p + 0xe0, [vel_len, 0.0, 0.0, 0.0]);

        let p3: [f32; 4] = [0.0, 0.70710677, 0.0, 0.70710677];
        let (_, x0) = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            0x2222,
            p3.as_ptr() as usize as u64,
            0x4444,
            1.0f32.to_bits() as u64,
            0.5f32.to_bits() as u64,
            0.25f32.to_bits() as u64,
            0,
            [0; 4],
        );
        x0 as u32
    };

    for vel_len in [0.0f32, 1.0f32] {
        let o = run(orig, vel_len);
        let n = run(new, vel_len);
        crate::diff_eq!(o, n, "1421c6680 vel_len={}", vel_len);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421d6da0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_DDB0: Vec<(u32, u8, u32)> = Vec::new();

    unsafe extern "win64" fn mock_ddb0(dst: usize, src: usize, cnt: u32, _x3: usize) {
        let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
        let b = rd::<u8>(src);
        wr::<u8>(dst, rd::<u8>(dst).wrapping_add(b));
        LOG_DDB0.push((cnt, b, xmm3 as u32));
    }

    let _m1 = crate::difftest::Mock::new(0x1_421b_ddb0, mock_ddb0 as *const ())?;

    let run = |f: F, idx_18: u32, idx_1c: u32| -> (Vec<u8>, Vec<(u32, u8, u32)>) {
        LOG_DDB0.clear();

        // Build relative-offset table inside a single buffer:
        // +0x00..+0x10: param_2 (with +0x08 -> +0x10, +0x0c = 2.0f32)
        // +0x10..+0x20: 2 i32 relative offsets pointing to +0x20 and +0x28
        // +0x20..+0x28: weight array 0
        // +0x28..+0x30: weight array 1
        let mut p2_blob = [0u8; 0x30];
        let p2p = p2_blob.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x08, 0x08); // (p2p + 0x08) + 0x08 == p2p + 0x10
        wr::<f32>(p2p + 0x0c, 2.0);
        wr::<i32>(p2p + 0x10, 0x10); // (p2p + 0x10) + 0x10 == p2p + 0x20
        wr::<i32>(p2p + 0x14, 0x14); // (p2p + 0x14) + 0x14 == p2p + 0x28
        wr::<[u8; 6]>(p2p + 0x20, [10, 20, 30, 40, 50, 60]);
        wr::<[u8; 6]>(p2p + 0x28, [5, 15, 25, 35, 45, 55]);

        let mut p1 = [0u8; 0x50];
        wr::<u32>(p1.as_mut_ptr() as usize + 0x4c, 6);

        let mut p3 = [0u8; 0x40];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u32>(p3p + 0x18, idx_18);
        wr::<u32>(p3p + 0x1c, idx_1c);
        wr::<f32>(p3p + 0x20, 0.5);
        wr::<u8>(p3p + 0x24, 0);
        wr::<u8>(p3p + 0x25, 1);
        wr::<f32>(p3p + 0x28, 1.5);
        wr::<u8>(p3p + 0x2c, 0);
        wr::<u8>(p3p + 0x2d, 1);

        let mut out = vec![0u8; 8];
        f(p1.as_mut_ptr() as usize, p2p, p3p, out.as_mut_ptr() as usize);
        (out, LOG_DDB0.clone())
    };

    for (i18, i1c) in [(0u32, 0u32), (2u32, 0u32)] {
        let o = run(f_o, i18, i1c);
        let n = run(f_n, i18, i1c);
        crate::diff_eq!(o, n, "1421d6da0 i18={} i1c={}", i18, i1c);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421d7250(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_6DA0: u32 = 0;

    unsafe extern "win64" fn mock_6da0(_p1: usize, _r14: usize, _rsi: usize, dst: usize) {
        wr::<[u8; 2]>(dst, [96, 160]);
        LOG_6DA0 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_421d_6da0, mock_6da0 as *const ())?;

    let run = |f: F, skip_15: u8, mode_1b4: u16, cl_flags: u8, neq_18_1c: bool| -> (usize, u32, u32, [[u32; 8]; 4]) {
        LOG_6DA0 = 0;

        let mut tfs: [[f32; 8]; 4] = [
            [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0],
            [0.0, 0.70710677, 0.0, 0.70710677, 4.0, 5.0, 6.0, 0.0],
            [0.0, 0.70710677, 0.0, 0.70710677, 7.0, 8.0, 9.0, 0.0],
            [0.5, 0.5, 0.5, 0.5, 10.0, 11.0, 12.0, 0.0],
        ];
        let tf_base = tfs.as_mut_ptr() as usize;
        let tf_end = tf_base + 4 * 0x20;
        let parents: [u16; 2] = [0xffff, 0];

        let mut r14_blob = [0u8; 0x20];
        let r14p = r14_blob.as_mut_ptr() as usize;
        wr::<u8>(r14p + 0x05, cl_flags);
        wr::<i32>(r14p + 0x08, 0x08); // -> r14p + 0x10
        wr::<i32>(r14p + 0x10, 0x08); // -> r14p + 0x18
        wr::<[u8; 2]>(r14p + 0x18, [64, 192]);

        let mut rsi_blob = [0u8; 0x28];
        let rsip = rsi_blob.as_mut_ptr() as usize;
        wr::<f32>(rsip + 0x0c, 0.5);
        wr::<u8>(rsip + 0x14, 0);
        wr::<u8>(rsip + 0x15, skip_15);
        wr::<u32>(rsip + 0x18, u32::from(neq_18_1c));
        wr::<u32>(rsip + 0x1c, 0);

        let mut p1 = [0u8; 0x1c0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, r14p);
        wr::<usize>(p1p + 0x08, rsip);
        wr::<u32>(p1p + 0x14, 5);
        wr::<usize>(p1p + 0x20, tf_end);
        wr::<u32>(p1p + 0x4c, 2);
        wr::<usize>(p1p + 0x50, parents.as_ptr() as usize);
        wr::<u16>(p1p + 0x1b4, mode_1b4);

        f(p1p);
        (
            rd::<usize>(p1p + 0x20).wrapping_sub(tf_base),
            rd::<u32>(p1p + 0x14),
            LOG_6DA0,
            core::mem::transmute(tfs),
        )
    };

    for (skip_15, mode_1b4, cl, neq) in [
        (1u8, 0u16, 0u8, false),
        (0u8, 1u16, 0u8, false),
        (0u8, 1u16, 1u8, false),
        (0u8, 0u16, 0u8, false),
        (0u8, 0u16, 2u8, true),
        (0u8, 0u16, 1u8, false),
        (0u8, 0u16, 3u8, false),
    ] {
        let o = run(f_o, skip_15, mode_1b4, cl, neq);
        let n = run(f_n, skip_15, mode_1b4, cl, neq);
        crate::diff_eq!(
            o,
            n,
            "1421d7250 skip={} m1b4={} cl={} neq={}",
            skip_15,
            mode_1b4,
            cl,
            neq
        );
    }
    Ok(7)
}
