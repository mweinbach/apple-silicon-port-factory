//! Decompiled `rend` hot functions shard Z7 (`0x142210a10..0x142213f60`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn rsqrt_ss(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        f32::from_bits(crate::simd_ops::vrsqrtss(0, x.to_bits()) as u32)
    }
}

/// `unknown_142210a10`
/// Original: 0x142210a10 (991 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_142210a10(param_1: usize, param_2: usize) -> u32 {
    let xmm = crate::simd_ops::take_xmm_args();
    let param_3_bits = xmm[2] as u32;
    let param_4_bits = xmm[3] as u32;

    let y = rd::<f32>(param_2 + 0x04);
    let x = rd::<f32>(param_2);
    let z = rd::<f32>(param_2 + 0x08);
    let len_sq = ((y * y) + (x * x)) + (z * z);
    let eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_6460) as usize);
    if eps > len_sq {
        return 0xff;
    }

    let w0 = rd::<u32>(param_2);
    let w1 = rd::<u32>(param_2 + 0x04);
    let w2 = rd::<u32>(param_2 + 0x08);

    let mut ebp: u32 = 0x318f_97d9;
    let mut eax = w0.wrapping_mul(0x3361_d2af);
    let mut ecx = 0x976a_1fccu32.wrapping_sub(eax).rotate_right(0x11);
    eax = (ecx.wrapping_mul(0x1b87_3593) ^ 0x3c)
        .rotate_right(0x13)
        .wrapping_add(0xfadd_af14);
    let mut r8d = eax.wrapping_mul(5);

    eax = w2.wrapping_mul(0x3361_d2af);
    ecx = 0xf512_9efcu32.wrapping_sub(eax).rotate_right(0x11);
    eax = ecx.wrapping_mul(0x1b87_3593);
    r8d = (r8d ^ eax)
        .rotate_right(0x13)
        .wrapping_add(0xfadd_af14)
        .wrapping_mul(5);

    eax = w1.wrapping_mul(0x3361_d2af);
    ecx = ebp.wrapping_sub(eax).rotate_right(0x11);
    eax = ecx.wrapping_mul(0x1b87_3593);
    r8d = (r8d ^ eax).rotate_right(0x13).wrapping_add(0xfadd_af14);
    eax = r8d.wrapping_mul(5);
    eax = (eax ^ (eax >> 16)).wrapping_mul(0x85eb_ca6b);
    eax = (eax ^ (eax >> 13)).wrapping_mul(0xc2b2_ae35);
    let ebx = eax ^ (eax >> 16);

    let p4_bytes = param_4_bits.to_le_bytes();
    let b0 = (p4_bytes[0] as i8 as i32) as u32;
    let b1 = (p4_bytes[1] as i8 as i32) as u32;
    let b2 = (p4_bytes[2] as i8 as i32) as u32;
    let b3 = (p4_bytes[3] as i8 as i32) as u32;

    let mut r8d_p4: u32 = 9;
    let c0 = b0;
    r8d_p4 ^= c0;
    let c1 = b1.wrapping_sub(c0.wrapping_mul(0x3361_d2af));
    r8d_p4 ^= c1;
    let c2 = b2.wrapping_sub(c1.wrapping_mul(0x3361_d2af));
    r8d_p4 ^= c2;
    let r10d = b3.wrapping_sub(c2.wrapping_mul(0x3361_d2af));
    r8d_p4 ^= r10d;

    r8d_p4 = (r8d_p4 ^ 0x3cf1_e974).rotate_right(0x13);
    eax = r10d
        .wrapping_mul(0xcc9e_2d51)
        .rotate_right(0x11)
        .wrapping_mul(0x1b87_3593);
    ecx = r8d_p4
        .wrapping_sub(0x0522_50ec)
        .wrapping_mul(5);
    ecx = (ecx ^ eax).rotate_right(0x13).wrapping_add(0xfadd_af14);
    eax = ecx.wrapping_mul(5);
    eax = (eax ^ (eax >> 16)).wrapping_mul(0x85eb_ca6b);
    eax = (eax ^ (eax >> 13)).wrapping_mul(0xc2b2_ae35);
    eax ^= eax >> 16;

    ecx = 0x64f1_6a88u32
        .wrapping_sub(eax.wrapping_mul(0x3361_d2af))
        .rotate_right(0x11);
    eax = (ecx.wrapping_mul(0x1b87_3593) ^ 0x28)
        .rotate_right(0x13)
        .wrapping_add(0xfadd_af14);
    let mut edx = eax.wrapping_mul(5);

    ecx = 0xf8b7_14a8u32
        .wrapping_sub(ebx.wrapping_mul(0x3361_d2af))
        .rotate_right(0x11);
    eax = ecx.wrapping_mul(0x1b87_3593);
    edx = (edx ^ eax).rotate_right(0x13).wrapping_add(0xfadd_af14);

    ebp = ebp
        .wrapping_sub(ebx.wrapping_mul(0x3361_d2af))
        .rotate_right(0x11);
    ecx = edx.wrapping_mul(5);
    eax = ebp.wrapping_mul(0x1b87_3593);
    ecx = (ecx ^ eax).rotate_right(0x13).wrapping_add(0xfadd_af14);
    eax = ecx.wrapping_mul(5);
    eax = (eax ^ (eax >> 16)).wrapping_mul(0x85eb_ca6b);
    eax = (eax ^ (eax >> 13)).wrapping_mul(0xc2b2_ae35);
    let seed_r8d = eax ^ (eax >> 16);

    let mut local_a0: u32 = param_3_bits;
    let hash_eax = exe::call_va!(
        0x1_412e_8510,
        ((&mut local_a0 as *mut u32) as usize, edx, seed_r8d) -> u32
    );

    let r15: u64 = if (hash_eax as u64) != rd::<u64>(param_1 + 0x98) {
        hash_eax as u64
    } else {
        0
    };
    let mut key_98: u64 = r15;

    let idx_rcx: u64 = if rd::<u64>(param_1 + 0x88) == rd::<u64>(param_1 + 0x80) {
        rd::<u64>(param_1 + 0x90)
    } else {
        let mut out_20: u64 = 0;
        exe::call_va!(
            0x1_4105_2dc0,
            (
                param_1 + 0x50,
                (&mut out_20 as *mut u64) as usize,
                (&mut key_98 as *mut u64) as usize
            )
        );
        out_20
    };

    if idx_rcx != u64::MAX {
        let base_a8 = rd::<usize>(param_1 + 0xa8);
        let end_ptr = base_a8.wrapping_add((rd::<u64>(param_1 + 0x90) as usize).wrapping_shl(4));
        let cur_ptr = base_a8.wrapping_add((idx_rcx as usize).wrapping_shl(4));
        if cur_ptr != end_ptr {
            let obj = rd::<usize>(cur_ptr + 0x08);
            return rd::<u32>(obj);
        }
    }

    let rbx = exe::call_va!(0x1_4243_b450, (param_1 + 0x10) -> usize);
    if rbx != 0 {
        let _ = exe::call_va!(0x1_4183_acc2, (rbx, 0u32, 0xcc8u64) -> usize);
    }
    let id = exe::call_va!(0x1_4243_b280, (param_1 + 0x10, rbx) -> u32);
    wr::<u32>(rbx, id);
    wr::<u32>(rbx + 0x04, param_3_bits);
    wr::<u32>(rbx + 0x08, param_4_bits);
    wr::<u64>(rbx + 0x0c, rd::<u64>(param_2));
    wr::<u32>(rbx + 0x14, rd::<u32>(param_2 + 0x08));

    key_98 = r15;
    let slot = exe::call_va!(
        0x1_411d_e360,
        (param_1 + 0x50, (&mut key_98 as *mut u64) as usize) -> usize
    );
    wr::<usize>(slot + 0x08, rbx);
    wr::<u8>(param_1 + 0xc0, 1);
    rd::<u32>(rbx)
}

/// `unknown_142212350`
/// Original: 0x142212350 (3096 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142212350(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> u64 {
    if rd::<usize>(param_1) == 0 {
        let mut err_buf = [0u64; 4];
        let msg = exe::ptr::<u8>(0x1_433f_0388) as usize;
        let exc = exe::call_va!(0x1_412e_8690, (err_buf.as_mut_ptr() as usize, msg) -> usize);
        exe::call_va!(0x1_412e_88c0, (exc));
        return 0;
    }

    let cfg0 = rd::<usize>(param_1 + 0x08);
    let raw_8 = rd::<i32>(cfg0 + 0x08);
    let total_items = (raw_8 / 2)
        .wrapping_mul(rd::<i32>(cfg0 + 0x2c))
        .wrapping_mul(rd::<i32>(cfg0 + 0x28)) as i64;
    let grid_buf = exe::call_va!(
        0x1_4221_2230,
        (cfg0, (raw_8 >> 31) as u32, param_3, total_items as u64) -> usize
    );
    if total_items > 0 {
        for i in 0..(total_items as usize) {
            let elem = grid_buf.wrapping_add(i * 0x1c);
            wr::<u32>(elem, 0xffff_ffff);
            wr::<u64>(elem + 0x04, 0);
            wr::<u64>(elem + 0x0c, 0);
            wr::<u64>(elem + 0x14, 0);
        }
    }

    let rdi = rd::<usize>(param_1 + 0x08);
    let xmm14 = (rd::<i32>(rdi + 0x20).wrapping_sub(rd::<i32>(rdi + 0x28)) / 2) as f32;
    let xmm15 = (rd::<i32>(rdi + 0x24).wrapping_sub(rd::<i32>(rdi + 0x2c)) / 2) as f32;
    let xmm11 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let xmm9_init = xmm11 * rd::<f32>(rdi + 0x14);
    let xmm7_two = rd::<f32>(exe::ptr::<f32>(0x1_4342_6abc) as usize);
    let xmm8_half_sqrt2 = xmm7_two.sqrt() * xmm11;

    let arg0_a = xmm11 * rd::<f32>(rdi + 0x18);
    let (_, pow1_xmm0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad46,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [arg0_a.to_bits() as u128, xmm7_two.to_bits() as u128, 0, 0],
    );
    let xmm6_pow1 = f32::from_bits(pow1_xmm0 as u32);
    let (_, pow2_xmm0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad46,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [
            xmm9_init.to_bits() as u128,
            xmm7_two.to_bits() as u128,
            0,
            0,
        ],
    );
    let pow2 = f32::from_bits(pow2_xmm0 as u32);
    let sum_sq = xmm6_pow1 + pow2;
    let hypot = if 0.0f32 > sum_sq {
        let (_, s) = crate::simd_ops::call_va8_xmm(
            0x1_4183_add0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [sum_sq.to_bits() as u128, 0, 0, 0],
        );
        f32::from_bits(s as u32)
    } else {
        sum_sq.sqrt()
    };

    let xmm10 = hypot + xmm8_half_sqrt2;
    let c_1_5 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6978) as usize);
    let mut esi_radius = crate::simd_ops::cvt_f32_i32(xmm10 + c_1_5) as i32;
    let local_54 = esi_radius;
    let mask_84 = rd::<u32>(rdi + 0x40).wrapping_sub(1);
    let stride_88 = rd::<u32>(rdi + 0x3c);
    let mask_8c = stride_88.wrapping_sub(1);
    let xmm12 = (xmm9_init + xmm8_half_sqrt2) as f64;

    let rdx_init = rd::<usize>(param_1);
    if rd::<u32>(rdx_init + 0x08) > 0 {
        let xmm9_eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
        let mut r8d_max_slots = rd::<i32>(exe::ptr::<i32>(0x1_43cf_0300) as usize);
        let c_3_0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6bf8) as usize);
        let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
        let abs_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8720) as usize);

        let mut edi_idx: i32 = 0;
        loop {
            let rdx_vec = rd::<usize>(param_1);
            let line_ptr = rd::<usize>(rdx_vec).wrapping_add((edi_idx as isize as usize) * 0x28);
            let line_32 = rd::<[u8; 32]>(line_ptr);
            let l_base = line_32.as_ptr() as usize;

            let cfg = rd::<usize>(param_1 + 0x08);
            let max_slice = (rd::<i32>(cfg + 0x08) / 2).wrapping_sub(1);
            let r12d = rd::<i32>(l_base + 0x10);
            let step_cnt = rd::<i32>(l_base + 0x14);

            if r12d <= max_slice && step_cnt != 0 {
                let h_f32 = rd::<i32>(cfg + 0x24) as f32;
                let dy_line = rd::<f32>(l_base + 0x0c);
                let xmm7 = dy_line * h_f32;
                let w_f32 = rd::<i32>(cfg + 0x20) as f32;
                let dx_line = rd::<f32>(l_base + 0x08);
                let mut xmm6 = w_f32 * dx_line;
                let xmm3 = (xmm6 * xmm6) + (xmm7 * xmm7);
                let xmm1 = rsqrt_ss(xmm3);
                let xmm4 = (xmm3 * xmm1) * xmm1;
                let xmm3_nr = c_3_0 - xmm4;
                let xmm5 = (xmm11 * xmm1) * xmm3_nr;
                let local_90 = f32::from_bits((xmm7 * xmm5).to_bits() ^ sign_mask);
                xmm6 *= xmm5;
                let local_1f8 = xmm6;

                if step_cnt > 0 {
                    let dx_f32 = rd::<f32>(l_base + 0x08);
                    let dy_f32 = rd::<f32>(l_base + 0x0c);
                    let ecx_off_x = crate::simd_ops::cvt_f32_i32(xmm14 + xmm11) as i32;
                    let edx_off_y = crate::simd_ops::cvt_f32_i32(xmm15 + xmm11) as i32;
                    let local_6c = rd::<i32>(l_base + 0x18);
                    let mut r11d = local_6c.wrapping_neg();
                    let x0_f32 = rd::<f32>(l_base);
                    let y0_f32 = rd::<f32>(l_base + 0x04);
                    let base_1c = rd::<i32>(l_base + 0x1c);

                    loop {
                        let step_idx = local_6c.wrapping_add(r11d);
                        let xmm2_step = step_idx as f32;
                        let xmm8_px = (xmm2_step * dx_f32) + x0_f32;
                        let xmm13_py = (xmm2_step * dy_f32) + y0_f32;
                        let r9 = rd::<usize>(param_1 + 0x08);
                        let xmm5_h = rd::<i32>(r9 + 0x24) as f32;
                        let iy_trunc = crate::simd_ops::cvt_f32_i32(xmm5_h * xmm13_py) as i32;
                        let xmm3_y = ((iy_trunc as f32) / xmm5_h) + (xmm11 / xmm5_h);
                        let xmm4_w = rd::<i32>(r9 + 0x20) as f32;
                        let ix_trunc = crate::simd_ops::cvt_f32_i32(xmm4_w * xmm8_px) as i32;
                        let xmm1_x = ((ix_trunc as f32) / xmm4_w) + (xmm11 / xmm4_w);
                        let xmm2_fy = xmm5_h * xmm3_y;
                        let xmm1_fx = xmm1_x * xmm4_w;
                        let ebx = (crate::simd_ops::cvt_f32_i32(xmm1_fx) as i32)
                            .wrapping_sub(ecx_off_x);
                        let r14d_gy = (crate::simd_ops::cvt_f32_i32(xmm2_fy) as i32)
                            .wrapping_sub(edx_off_y);

                        if ebx >= 0
                            && r14d_gy >= 0
                            && ebx < rd::<i32>(r9 + 0x28)
                            && r14d_gy < rd::<i32>(r9 + 0x2c)
                        {
                            if r11d < -5 {
                                let mut arg_1f0: u64 =
                                    (r11d as u32 as u64) | ((xmm13_py.to_bits() as u64) << 32);
                                let fmt = exe::ptr::<u8>(0x1_433f_0348) as usize;
                                let msg_ptr = exe::call_va!(
                                    0x1_403b_4100,
                                    (fmt, (&mut arg_1f0 as *mut u64) as usize) -> usize
                                );
                                let mut err_buf = [0u64; 4];
                                let exc = exe::call_va!(
                                    0x1_412e_8690,
                                    (err_buf.as_mut_ptr() as usize, msg_ptr) -> usize
                                );
                                exe::call_va!(0x1_412e_88c0, (exc));
                                return 0;
                            }

                            let raw_94 =
                                base_1c.wrapping_add(r11d.wrapping_mul(rd::<i32>(r9 + 0x34)));
                            let local_94 = if raw_94 >= 0 { raw_94 } else { 0 };
                            let ecx_ymin = r14d_gy.wrapping_sub(esi_radius);
                            let mut r10d_y = if ecx_ymin > 0 { ecx_ymin } else { 0 };
                            let esi_diam = esi_radius.wrapping_add(esi_radius);
                            let edx_ymax = r10d_y.wrapping_add(esi_diam);
                            let mut r15d_ymax = rd::<i32>(r9 + 0x2c).wrapping_sub(1);
                            let mut edge_flag = ecx_ymin <= 0;
                            if edx_ymax >= r15d_ymax {
                                edge_flag = true;
                            } else {
                                r15d_ymax = edx_ymax;
                            }

                            let eax_xmin = ebx.wrapping_sub(local_54);
                            if eax_xmin <= 0 {
                                edge_flag = true;
                            }
                            let ecx_xmin = if eax_xmin > 0 { eax_xmin } else { 0 };
                            let ecx_xmax = ecx_xmin.wrapping_add(esi_diam);
                            let mut r9d_xmax = rd::<i32>(r9 + 0x28).wrapping_sub(1);
                            if ecx_xmax >= r9d_xmax {
                                edge_flag = true;
                            } else {
                                r9d_xmax = ecx_xmax;
                            }

                            if r10d_y <= r15d_ymax {
                                let mut dy_grid = r10d_y.wrapping_sub(r14d_gy);
                                loop {
                                    let cfg_cur = rd::<usize>(param_1 + 0x08);
                                    let edx_inv_y = rd::<i32>(cfg_cur + 0x2c)
                                        .wrapping_sub(r10d_y)
                                        .wrapping_sub(1);
                                    let mut r14d_x = ecx_xmin;
                                    if r14d_x <= r9d_xmax {
                                        let ecx_dy_sq = dy_grid.wrapping_mul(dy_grid);
                                        let mut r15d_dx = r14d_x.wrapping_sub(ebx);
                                        loop {
                                            let dist_sq_i32 = r15d_dx
                                                .wrapping_mul(r15d_dx)
                                                .wrapping_add(ecx_dy_sq);
                                            let xmm7_dist_sq = dist_sq_i32 as f32;
                                            if edge_flag || !(xmm7_dist_sq > xmm10 * xmm10) {
                                                let tile_idx = (((edx_inv_y as u32) & mask_84)
                                                    .wrapping_mul(stride_88))
                                                .wrapping_add((r14d_x as u32) & mask_8c);
                                                let rax_cfg = rd::<usize>(param_1 + 0x08);
                                                let xmm3_cx = ((rd::<i32>(rax_cfg + 0x20) as f32)
                                                    * xmm8_px)
                                                    - xmm14;
                                                let xmm5_cy = ((rd::<i32>(rax_cfg + 0x24) as f32)
                                                    * xmm13_py)
                                                    - xmm15;
                                                let xmm4_tx = (((r14d_x as f32) + xmm11)
                                                    - xmm3_cx)
                                                    * local_90;
                                                let xmm3_ty =
                                                    (((r10d_y as f32) + xmm11) - xmm5_cy) * xmm6;
                                                let xmm1_sum = xmm4_tx + xmm3_ty;
                                                let xmm1_abs =
                                                    f32::from_bits(xmm1_sum.to_bits() & abs_mask);
                                                let xmm0_abs_f64 = xmm1_abs as f64;
                                                if edge_flag || !(xmm0_abs_f64 > xmm12) {
                                                    let xmm6_metric = xmm0_abs_f64 as f32;
                                                    let cell_idx = r10d_y
                                                        .wrapping_mul(rd::<i32>(rax_cfg + 0x28))
                                                        .wrapping_add(r14d_x);
                                                    let half_slices = rd::<i32>(rax_cfg + 0x08) / 2;
                                                    let base_slot_idx = half_slices
                                                        .wrapping_mul(cell_idx)
                                                        as isize
                                                        as usize;
                                                    let rbx_cell_ptr =
                                                        grid_buf.wrapping_add(base_slot_idx * 0x1c);

                                                    let tls0 = rd::<usize>(exe::tls_pointer());
                                                    let epoch = rd::<i32>(tls0 + 0x74);
                                                    let guard_addr =
                                                        exe::ptr::<i32>(0x1_43cf_0304) as usize;
                                                    let stat_addr =
                                                        exe::ptr::<i32>(0x1_43cf_0300) as usize;
                                                    if rd::<i32>(guard_addr) > epoch {
                                                        exe::call_va!(0x1_4183_4a78, (guard_addr));
                                                        if rd::<i32>(guard_addr) == -1 {
                                                            let v = rd::<i32>(
                                                                rd::<usize>(param_1 + 0x08) + 0x08,
                                                            ) / 2;
                                                            wr::<i32>(stat_addr, v);
                                                            exe::call_va!(
                                                                0x1_4183_4a0c,
                                                                (guard_addr)
                                                            );
                                                        }
                                                        r8d_max_slots = rd::<i32>(stat_addr);
                                                    }

                                                    let mut rsi_slot: usize = 0;
                                                    let mut do_write = true;
                                                    if r8d_max_slots > 0 {
                                                        let mut edx_i: i32 = 0;
                                                        loop {
                                                            let cur_slot_ptr = rbx_cell_ptr
                                                                .wrapping_add(rsi_slot * 0x1c);
                                                            let cur_tag = rd::<i32>(cur_slot_ptr);
                                                            if cur_tag == -1 {
                                                                break;
                                                            }
                                                            if cur_tag == r12d {
                                                                let prev_dist_sq =
                                                                    rd::<f32>(cur_slot_ptr + 0x14);
                                                                let prev_metric =
                                                                    rd::<f32>(cur_slot_ptr + 0x18);
                                                                if prev_metric
                                                                    > xmm6_metric + xmm9_eps
                                                                {
                                                                    do_write = true;
                                                                } else if xmm6_metric - xmm9_eps
                                                                    > prev_metric
                                                                {
                                                                    do_write = false;
                                                                } else {
                                                                    do_write = prev_dist_sq
                                                                        > xmm7_dist_sq;
                                                                }
                                                                break;
                                                            }
                                                            edx_i += 1;
                                                            rsi_slot += 1;
                                                            if edx_i >= r8d_max_slots {
                                                                break;
                                                            }
                                                        }
                                                    }

                                                    if do_write {
                                                        let dst_slot = rbx_cell_ptr
                                                            .wrapping_add(rsi_slot * 0x1c);
                                                        wr::<i32>(dst_slot, r12d);
                                                        wr::<i32>(dst_slot + 0x04, local_94);
                                                        wr::<i32>(dst_slot + 0x08, ebx);
                                                        wr::<i32>(dst_slot + 0x0c, r14d_gy);
                                                        wr::<u32>(dst_slot + 0x10, tile_idx);
                                                        wr::<f32>(dst_slot + 0x14, xmm7_dist_sq);
                                                        wr::<f32>(dst_slot + 0x18, xmm6_metric);
                                                    }
                                                    xmm6 = local_1f8;
                                                }
                                            }
                                            r14d_x += 1;
                                            r15d_dx += 1;
                                            if r14d_x > r9d_xmax {
                                                break;
                                            }
                                        }
                                    }
                                    r10d_y += 1;
                                    dy_grid += 1;
                                    if r10d_y > r15d_ymax {
                                        break;
                                    }
                                }
                            }
                            esi_radius = local_54;
                        }
                        r11d += 1;
                        if local_6c.wrapping_add(r11d) >= step_cnt {
                            break;
                        }
                    }
                }
            }

            edi_idx += 1;
            let cur_vec = rd::<usize>(param_1);
            if (edi_idx as i64 as u64) >= (rd::<u32>(cur_vec + 0x08) as u64) {
                break;
            }
        }
    }

    let mut vec_30 = [0u64; 2];
    let vec_30_ptr = vec_30.as_mut_ptr() as usize;

    let mut r13d_y: i32 = 0;
    if rd::<i32>(rd::<usize>(param_1 + 0x08) + 0x2c) > 0 {
        loop {
            let mut r12d_x: i32 = 0;
            let mut rcx_cfg = rd::<usize>(param_1 + 0x08);
            if rd::<i32>(rcx_cfg + 0x28) > 0 {
                loop {
                    let raw_8 = rd::<i32>(rcx_cfg + 0x08);
                    let base_idx = (r13d_y
                        .wrapping_mul(rd::<i32>(rcx_cfg + 0x28))
                        .wrapping_add(r12d_x))
                    .wrapping_mul(raw_8)
                        / 2;
                    let mut rbx_cell = grid_buf.wrapping_add((base_idx as isize as usize) * 0x1c);
                    let mut r14d_k: i32 = 0;
                    if raw_8 / 2 > 0 {
                        let mut rdi_src = rbx_cell;
                        loop {
                            let cur_len = rd::<u32>(vec_30_ptr + 0x08) as usize;
                            let cur_cap = rd::<u32>(vec_30_ptr + 0x0c) as usize;
                            let cur_data = rd::<usize>(vec_30_ptr);
                            if cur_data.wrapping_add(cur_len * 0x1c)
                                == cur_data.wrapping_add(cur_cap * 0x1c)
                            {
                                exe::call_va!(
                                    0x1_412e_91d0,
                                    (vec_30_ptr, (cur_len as u64).wrapping_add(1))
                                );
                            }
                            let cur_len = rd::<u32>(vec_30_ptr + 0x08) as usize;
                            let cur_data = rd::<usize>(vec_30_ptr);
                            let dst = cur_data.wrapping_add(cur_len * 0x1c);
                            wr::<[u8; 0x1c]>(dst, rd::<[u8; 0x1c]>(rdi_src));
                            wr::<u32>(vec_30_ptr + 0x08, (cur_len as u32).wrapping_add(1));
                            r14d_k += 1;
                            rdi_src = rdi_src.wrapping_add(0x1c);
                            let half_now = rd::<i32>(rd::<usize>(param_1 + 0x08) + 0x08) / 2;
                            if r14d_k >= half_now {
                                break;
                            }
                        }
                    }

                    let cur_len = rd::<u32>(vec_30_ptr + 0x08) as usize;
                    let cur_data = rd::<usize>(vec_30_ptr);
                    let mut first_ptr = cur_data;
                    let mut last_ptr = cur_data.wrapping_add(cur_len * 0x1c);
                    let sort_cnt = ((last_ptr.wrapping_sub(first_ptr) as isize) / 0x1c) as i64;
                    let cmp_fn = exe::ptr::<u8>(0x1_4221_1f20) as usize;
                    exe::call_va!(
                        0x1_412e_8d70,
                        (
                            (&mut first_ptr as *mut usize) as usize,
                            (&mut last_ptr as *mut usize) as usize,
                            sort_cnt,
                            cmp_fn
                        )
                    );

                    let mut ecx_k: i32 = 0;
                    rcx_cfg = rd::<usize>(param_1 + 0x08);
                    if rd::<i32>(rcx_cfg + 0x08) / 2 > 0 {
                        let cur_data = rd::<usize>(vec_30_ptr);
                        loop {
                            let src = cur_data.wrapping_add((ecx_k as isize as usize) * 0x1c);
                            wr::<[u8; 0x1c]>(rbx_cell, rd::<[u8; 0x1c]>(src));
                            ecx_k += 1;
                            rbx_cell = rbx_cell.wrapping_add(0x1c);
                            rcx_cfg = rd::<usize>(param_1 + 0x08);
                            if ecx_k >= rd::<i32>(rcx_cfg + 0x08) / 2 {
                                break;
                            }
                        }
                    }

                    wr::<u32>(vec_30_ptr + 0x08, 0);
                    r12d_x += 1;
                    if r12d_x >= rd::<i32>(rcx_cfg + 0x28) {
                        break;
                    }
                }
            }
            r13d_y += 1;
            if r13d_y >= rd::<i32>(rcx_cfg + 0x2c) {
                break;
            }
        }
    }

    let cfg_end = rd::<usize>(param_1 + 0x08);
    let h_2c = rd::<i32>(cfg_end + 0x2c) as i64 as u64;
    let w_28 = rd::<i32>(cfg_end + 0x28) as i64 as u64;
    let s_08 = rd::<i32>(cfg_end + 0x08) as i64 as u64;
    let rbx_out_bytes = (h_2c.wrapping_mul(w_28).wrapping_mul(s_08) << 2) >> 1;
    let r10_out = exe::call_va!(0x1_4221_2120, (0u64, 0u64, 0u64, rbx_out_bytes) -> usize);
    wr::<usize>(param_2, r10_out);

    let mut r9d_y: i32 = 0;
    let mut rdi_cfg = rd::<usize>(param_1 + 0x08);
    if rd::<i32>(rdi_cfg + 0x2c) > 0 {
        loop {
            let mut r8d_x: i32 = 0;
            if rd::<i32>(rdi_cfg + 0x28) > 0 {
                loop {
                    let mut esi_k: i32 = 0;
                    if rd::<i32>(rdi_cfg + 0x08) / 2 > 0 {
                        loop {
                            let w = rd::<i32>(rdi_cfg + 0x28);
                            let h = rd::<i32>(rdi_cfg + 0x2c);
                            let dst_idx = ((esi_k
                                .wrapping_add(1)
                                .wrapping_mul(h)
                                .wrapping_sub(r9d_y)
                                .wrapping_sub(1))
                            .wrapping_mul(w)
                            .wrapping_add(r8d_x))
                                as isize as usize;
                            let src_idx = (((w.wrapping_mul(r9d_y).wrapping_add(r8d_x))
                                .wrapping_mul(rd::<i32>(rdi_cfg + 0x08))
                                / 2)
                            .wrapping_add(esi_k))
                                as isize as usize;
                            let val = rd::<u32>(grid_buf.wrapping_add(src_idx * 0x1c + 0x04));
                            wr::<u32>(r10_out.wrapping_add(dst_idx * 4), val);
                            esi_k += 1;
                            rdi_cfg = rd::<usize>(param_1 + 0x08);
                            if esi_k >= rd::<i32>(rdi_cfg + 0x08) / 2 {
                                break;
                            }
                        }
                    }
                    r8d_x += 1;
                    if r8d_x >= rd::<i32>(rdi_cfg + 0x28) {
                        break;
                    }
                }
            }
            r9d_y += 1;
            if r9d_y >= rd::<i32>(rdi_cfg + 0x2c) {
                break;
            }
        }
    }

    let area = (rd::<i32>(rdi_cfg + 0x2c).wrapping_mul(rd::<i32>(rdi_cfg + 0x28))) as f32;
    let ratio = 0.0f32 / area;
    let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4342_6564) as usize);
    if ratio > thresh {
        let mut err_buf = [0u64; 4];
        let msg = exe::ptr::<u8>(0x1_433f_03c0) as usize;
        let exc = exe::call_va!(0x1_412e_8690, (err_buf.as_mut_ptr() as usize, msg) -> usize);
        exe::call_va!(0x1_412e_88c0, (exc));
        return 0;
    }

    if grid_buf != 0 {
        exe::call_va!(0x1_4244_2de0, (grid_buf));
    }
    exe::call_va!(0x1_401d_e830, (vec_30_ptr));
    rbx_out_bytes
}

#[inline(always)]
unsafe fn push_vec_64_inline(vec_ptr: usize, item_ptr: usize, rcx_hint: usize) {
    let len = rd::<u32>(vec_ptr + 0x08) as u64;
    let cap = rd::<u32>(vec_ptr + 0x0c) as u64;
    let mut buf = rd::<usize>(vec_ptr);
    if (len << 6) == (cap << 6) {
        let min_cap = len.wrapping_add(1);
        let grown = cap.wrapping_add(cap >> 1);
        let new_cap = if grown < min_cap { min_cap } else { grown };
        let new_buf = exe::call_va!(0x1_4023_b7a0, (rcx_hint, new_cap) -> usize);
        let _ = exe::call_va!(0x1_4183_acc8, (new_buf, buf, len << 6) -> usize);
        exe::call_va!(0x1_401d_e830, (vec_ptr));
        buf = new_buf;
        wr::<usize>(vec_ptr, new_buf);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    let dst = buf.wrapping_add((len as usize) << 6);
    wr::<[u64; 8]>(dst, rd::<[u64; 8]>(item_ptr));
    wr::<u32>(vec_ptr + 0x08, (len as u32).wrapping_add(1));
}

/// `unknown_142213f60`
/// Original: 0x142213f60 (4728 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142213f60(param_1: usize, param_2: usize) -> usize {
    exe::call_iat!(0x1_4264_6a80, (0x7bu32));
    let cfg0 = rd::<usize>(param_1);
    let coalesce_width = rd::<i32>(cfg0 + 0x30);
    let align_38 = rd::<i32>(cfg0 + 0x38);
    if coalesce_width < 1 || coalesce_width > rd::<i32>(param_1 + 0x10) {
        let mut err_buf = [0u64; 4];
        let msg = exe::ptr::<u8>(0x1_433f_04e8) as usize;
        let exc = exe::call_va!(0x1_412e_8690, (err_buf.as_mut_ptr() as usize, msg) -> usize);
        exe::call_va!(0x1_412e_88c0, (exc));
        return 0;
    }

    let mut vec_30 = [0u64; 2];
    let mut vec_68 = [0u64; 2];
    let mut vec_58 = [0u64; 2];
    let mut vec_20 = [0u64; 2];
    let vec_30_ptr = vec_30.as_mut_ptr() as usize;
    let vec_68_ptr = vec_68.as_mut_ptr() as usize;
    let vec_58_ptr = vec_58.as_mut_ptr() as usize;
    let vec_20_ptr = vec_20.as_mut_ptr() as usize;

    let def_pair = rd::<[u64; 2]>(exe::ptr::<u64>(0x1_4342_7690) as usize);
    let def_64: [u64; 8] = [0, 0, 0, 0, def_pair[0], def_pair[1], 0, 0];
    let def_40: [u64; 5] = [0, 0, def_pair[0], def_pair[1], 0];

    let half_slices = rd::<i32>(cfg0 + 0x08) / 2;
    if half_slices > 0 {
        for r11d in 0..half_slices {
            let mut r10d: i32 = 1_000_000;
            let cnt_10 = rd::<u32>(param_1 + 0x10) as u64;
            let arr_08 = rd::<usize>(param_1 + 0x08);
            if cnt_10 != 0 {
                for ecx in 0..(cnt_10 as i32) {
                    let elem = arr_08.wrapping_add((ecx as isize as usize) << 6);
                    if rd::<i32>(elem + 0x20) == r11d {
                        let v34 = rd::<i32>(elem + 0x34);
                        if v34 < r10d {
                            r10d = v34;
                        }
                    }
                }
            }

            let mut keep_going = true;
            while keep_going {
                let mut rem_width = coalesce_width as i64;
                while rem_width != 0 {
                    let mut ecx_found: i32 = -1;
                    let cnt_10 = rd::<u32>(param_1 + 0x10) as u64;
                    let rdi_arr = rd::<usize>(param_1 + 0x08);
                    if cnt_10 != 0 {
                        let mut edx: i32 = 0;
                        while (edx as i64 as u64) < cnt_10 {
                            if ecx_found != -1 {
                                break;
                            }
                            let elem = rdi_arr.wrapping_add((edx as isize as usize) << 6);
                            if rd::<i32>(elem + 0x20) == r11d && rd::<i32>(elem + 0x34) == r10d {
                                ecx_found = edx;
                            }
                            edx += 1;
                        }
                    }

                    if ecx_found != -1 {
                        let off_rcx = ((ecx_found as isize) << 6) as usize;
                        push_vec_64_inline(vec_68_ptr, rdi_arr.wrapping_add(off_rcx), off_rcx);

                        let src_item = rd::<usize>(param_1 + 0x08).wrapping_add(off_rcx);
                        let mut tmp_b0 = rd::<[u64; 8]>(src_item);
                        let mut tmp_160 = [0u64; 8];
                        let r8_ret = exe::call_va!(
                            0x1_4221_2fa0,
                            (
                                param_1,
                                tmp_160.as_mut_ptr() as usize,
                                tmp_b0.as_mut_ptr() as usize
                            ) -> usize
                        );
                        push_vec_64_inline(vec_58_ptr, r8_ret, param_1);
                        r10d = r10d.wrapping_add(1);
                    } else {
                        push_vec_64_inline(vec_68_ptr, def_64.as_ptr() as usize, param_1);
                        push_vec_64_inline(vec_58_ptr, def_64.as_ptr() as usize, param_1);
                        keep_going = false;
                    }
                    rem_width -= 1;
                }
            }
        }
    }

    let len_68 = rd::<u32>(vec_68_ptr + 0x08) as usize;
    let xmm13_half = rd::<f64>(exe::ptr::<f64>(0x1_4342_69f8) as usize);
    if len_68 != 0 {
        let xmm6_c = rd::<u32>(exe::ptr::<u32>(0x1_4342_6728) as usize);
        let xmm7_c = rd::<u32>(exe::ptr::<u32>(0x1_4342_7044) as usize);
        let xmm8_c = rd::<u32>(exe::ptr::<u32>(0x1_4342_661c) as usize);
        let xmm9_rand_max = rd::<f64>(exe::ptr::<f64>(0x1_4342_6d98) as usize);
        let tbl5_lo = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_87a0) as usize);
        let tbl5: [u32; 5] = [tbl5_lo[0], tbl5_lo[1], tbl5_lo[2], tbl5_lo[3], xmm6_c];
        let tbl3: [u32; 3] = [xmm7_c, 0, xmm8_c];

        for i in 0..len_68 {
            let mode_1c = rd::<i32>(rd::<usize>(param_1) + 0x1c);
            let buf_68 = rd::<usize>(vec_68_ptr);
            if mode_1c != 0 {
                let jitter_bits: u32 = if mode_1c == 2 {
                    let r8d = rd::<i32>(param_1 + 0xb0);
                    let idx = (r8d % 5) as isize as usize;
                    wr::<i32>(param_1 + 0xb0, r8d.wrapping_add(1));
                    tbl5[idx]
                } else if mode_1c == 3 {
                    let r8d = rd::<i32>(param_1 + 0xb0);
                    let idx = (r8d % 3) as isize as usize;
                    wr::<i32>(param_1 + 0xb0, r8d.wrapping_add(1));
                    tbl3[idx]
                } else {
                    let r = exe::call_iat!(0x1_4264_6a90, () -> i32);
                    let f = (((r as f64) / xmm9_rand_max) - xmm13_half) as f32;
                    f.to_bits()
                };
                wr::<u32>(buf_68.wrapping_add(i * 64 + 0x3c), jitter_bits);
            }

            let len_30 = rd::<u32>(vec_30_ptr + 0x08) as u64;
            let cap_30 = rd::<u32>(vec_30_ptr + 0x0c) as u64;
            if (len_30 << 6) == (cap_30 << 6) {
                exe::call_va!(0x1_411a_2010, (vec_30_ptr, len_30.wrapping_add(1)));
            }
            let len_30 = rd::<u32>(vec_30_ptr + 0x08) as usize;
            let dst = rd::<usize>(vec_30_ptr).wrapping_add(len_30 * 64);
            let src = rd::<usize>(vec_68_ptr).wrapping_add(i * 64);
            wr::<[u64; 8]>(dst, rd::<[u64; 8]>(src));
            wr::<u32>(vec_30_ptr + 0x08, (len_30 as u32).wrapping_add(1));
        }
    }

    let len_58 = rd::<u32>(vec_58_ptr + 0x08) as usize;
    if len_58 != 0 {
        let sign_bit = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
        for i in 0..len_58 {
            let buf_68 = rd::<usize>(vec_68_ptr);
            let buf_58 = rd::<usize>(vec_58_ptr);
            if rd::<i32>(rd::<usize>(param_1) + 0x1c) != 0 {
                let v = rd::<u32>(buf_68.wrapping_add(i * 64 + 0x3c)) ^ sign_bit;
                wr::<u32>(buf_58.wrapping_add(i * 64 + 0x3c), v);
            }
            push_vec_64_inline(vec_30_ptr, buf_58.wrapping_add(i * 64), param_1);
        }
    }

    exe::call_va!(
        0x1_4221_3400,
        (param_1, vec_30_ptr, coalesce_width as u32 as u64)
    );
    exe::call_va!(0x1_4221_3ae0, (param_1, vec_30_ptr));

    let mut r9d_min_iters: i32 = 10_000_000;
    let mut edx_grp_cnt: i32 = 0;
    let mut r14d_fill_start: i32 = 0;
    let mut r15d_idx: i32 = 0;
    let r13d_len = rd::<u32>(vec_30_ptr + 0x08);
    let rbx_data = rd::<usize>(vec_30_ptr);
    let total_30 = r13d_len as u64;

    if total_30 != 0 {
        while (r15d_idx as i64 as u64) < total_30 {
            let elem = rbx_data.wrapping_add((r15d_idx as isize as usize) * 64);
            let mut ecx_iters: i32 = 0;
            if rd::<i32>(elem + 0x24) != 0 {
                let dx_f64 = rd::<f64>(elem + 0x10);
                let dy_f64 = rd::<f64>(elem + 0x18);
                let x0_f64 = rd::<f64>(elem);
                let y0_f64 = rd::<f64>(elem + 0x08);
                let cfg = rd::<usize>(param_1);
                let w_f64 = rd::<i32>(cfg + 0x20) as f64;
                let h_f64 = rd::<i32>(cfg + 0x24) as f64;
                let mut xmm4_x = x0_f64 * w_f64;
                let mut xmm5_y = h_f64 * y0_f64;
                let xmm2_sx = dx_f64 * w_f64;
                let xmm3_sy = h_f64 * dy_f64;
                let max_iters = rd::<i32>(param_1 + 0xac);
                if max_iters <= 0 {
                    let mut err_buf = [0u64; 4];
                    let msg = exe::ptr::<u8>(0x1_433f_04b0) as usize;
                    let exc = exe::call_va!(
                        0x1_412e_8690,
                        (err_buf.as_mut_ptr() as usize, msg) -> usize
                    );
                    exe::call_va!(0x1_412e_88c0, (exc));
                    return 0;
                }
                let min_x = rd::<f64>(param_1 + 0x68);
                while !(xmm4_x >= min_x
                    && xmm5_y >= rd::<f64>(param_1 + 0x70)
                    && rd::<f64>(param_1 + 0x88) >= xmm4_x
                    && rd::<f64>(param_1 + 0x90) >= xmm5_y)
                {
                    ecx_iters += 1;
                    xmm4_x += xmm2_sx;
                    xmm5_y += xmm3_sy;
                    if ecx_iters >= max_iters {
                        let mut err_buf = [0u64; 4];
                        let msg = exe::ptr::<u8>(0x1_433f_04b0) as usize;
                        let exc = exe::call_va!(
                            0x1_412e_8690,
                            (err_buf.as_mut_ptr() as usize, msg) -> usize
                        );
                        exe::call_va!(0x1_412e_88c0, (exc));
                        return 0;
                    }
                }
                if ecx_iters == -1 {
                    let mut err_buf = [0u64; 4];
                    let msg = exe::ptr::<u8>(0x1_433f_04b0) as usize;
                    let exc = exe::call_va!(
                        0x1_412e_8690,
                        (err_buf.as_mut_ptr() as usize, msg) -> usize
                    );
                    exe::call_va!(0x1_412e_88c0, (exc));
                    return 0;
                }
            }

            if edx_grp_cnt == rd::<i32>(rd::<usize>(param_1) + 0x38) {
                if r14d_fill_start < r15d_idx {
                    for idx in r14d_fill_start..r15d_idx {
                        wr::<i32>(
                            rbx_data.wrapping_add((idx as isize as usize) * 64 + 0x28),
                            r9d_min_iters,
                        );
                    }
                    r14d_fill_start = r15d_idx;
                }
                edx_grp_cnt = 0;
                r9d_min_iters = 1_000_000;
            }

            let v24 = rd::<i32>(elem + 0x24);
            if v24 != 0 && ecx_iters < r9d_min_iters {
                r9d_min_iters = ecx_iters;
            }
            wr::<i32>(elem + 0x2c, v24.wrapping_sub(ecx_iters));
            edx_grp_cnt += 1;
            r15d_idx += 1;
        }
    }

    while (r14d_fill_start as i64 as u64) < total_30 {
        wr::<i32>(
            rbx_data.wrapping_add((r14d_fill_start as isize as usize) * 64 + 0x28),
            r9d_min_iters,
        );
        r14d_fill_start += 1;
    }

    let mut cfg = rd::<usize>(param_1);
    let r15d_step = rd::<i32>(cfg + 0x34);
    let mut r12d_acc = r15d_step.wrapping_mul(5);
    let r10d_five_step = r12d_acc;
    let mut r14d_in_grp: i32 = 0;
    let mut r8d_max_span: i32 = 0;
    let mut r11d_i: i32 = 0;

    if total_30 != 0 {
        while (r11d_i as i64 as u64) < total_30 {
            let elem = rbx_data.wrapping_add((r11d_i as isize as usize) * 64);
            let half = rd::<i32>(cfg + 0x08) / 2;
            let v20 = rd::<i32>(elem + 0x20);
            if v20 < half {
                if r14d_in_grp == r15d_step {
                    r12d_acc = r12d_acc.wrapping_add(
                        r8d_max_span
                            .wrapping_mul(r15d_step)
                            .wrapping_add(r10d_five_step),
                    );
                    r14d_in_grp = 0;
                    r8d_max_span = 0;
                }
                wr::<i32>(elem + 0x30, r14d_in_grp.wrapping_add(r12d_acc));
                let v24 = rd::<i32>(elem + 0x24);
                let v28 = rd::<i32>(elem + 0x28);
                let span = v24.wrapping_sub(v28);
                if v24 != 0 && span > r8d_max_span {
                    r8d_max_span = span;
                }
                r14d_in_grp += 1;
                cfg = rd::<usize>(param_1);
            } else {
                r11d_i = r11d_i.wrapping_sub(1).wrapping_add(rd::<i32>(cfg + 0x30));
            }
            r11d_i += 1;
        }
    }

    let total_span = (r15d_step
        .wrapping_mul(5)
        .wrapping_add(r8d_max_span.wrapping_mul(r15d_step))
        .wrapping_add(r12d_acc))
        / r15d_step;
    wr::<i32>(param_1 + 0xa8, total_span);
    wr::<i32>(param_1 + 0x24, r8d_max_span);
    wr::<i32>(cfg + 0x04, total_span);

    if total_30 != 0 {
        let count_30 = total_30 as usize;
        for idx_30 in 0..count_30 {
            let elem = rbx_data.wrapping_add(idx_30 * 64);
            let x0_f64 = rd::<f64>(elem);
            let y0_f64 = rd::<f64>(elem + 0x08);
            let dx_f64 = rd::<f64>(elem + 0x10);
            let dy_f64 = rd::<f64>(elem + 0x18);
            let xmm12_x0 = x0_f64 as f32;
            let xmm11_y0 = y0_f64 as f32;
            let dx_f32 = dx_f64 as f32;
            let dy_f32 = dy_f64 as f32;
            let packed_dxy: u64 = (dx_f32.to_bits() as u64) | ((dy_f32.to_bits() as u64) << 32);

            let r11d_slice = rd::<i32>(elem + 0x20);
            let r8d_v24 = rd::<i32>(elem + 0x24);
            let v28 = rd::<i32>(elem + 0x28);
            let v2c = rd::<u32>(elem + 0x2c);
            let mut v30 = rd::<i32>(elem + 0x30);
            let r14d_v34 = rd::<i32>(elem + 0x34);
            let v38 = rd::<u32>(elem + 0x38);
            let jitter_f32 = rd::<f32>(elem + 0x3c);

            let half = rd::<i32>(rd::<usize>(param_1) + 0x08) / 2;
            let mut emit = false;
            if r11d_slice < half {
                emit = true;
            } else if r8d_v24 != 0 {
                let target_slice = r11d_slice.wrapping_sub(half);
                let mut r9d_opp: i32 = -1;
                for cand_idx in 0..(count_30 as i32) {
                    let cand = rbx_data.wrapping_add((cand_idx as isize as usize) * 64);
                    if rd::<i32>(cand + 0x20) == target_slice && rd::<i32>(cand + 0x34) == r14d_v34
                    {
                        r9d_opp = cand_idx;
                    }
                }
                if r9d_opp == -1 {
                    let mut err_buf = [0u64; 4];
                    let msg = exe::ptr::<u8>(0x1_433f_0450) as usize;
                    let exc = exe::call_va!(
                        0x1_412e_8690,
                        (err_buf.as_mut_ptr() as usize, msg) -> usize
                    );
                    exe::call_va!(0x1_412e_88c0, (exc));
                    return 0;
                }
                let opp = rbx_data.wrapping_add((r9d_opp as isize as usize) * 64);
                let opp_x0 = rd::<f64>(opp);
                let opp_y0 = rd::<f64>(opp + 0x08);
                let opp_dx = rd::<f64>(opp + 0x10);
                let opp_dy = rd::<f64>(opp + 0x18);
                let opp_v24 = rd::<i32>(opp + 0x24);
                let opp_v28 = rd::<i32>(opp + 0x28);
                let opp_v30 = rd::<i32>(opp + 0x30);

                let xmm3_steps = (opp_v24.wrapping_sub(1) as f32) as f64;
                let xmm4_ody = opp_dy * xmm3_steps;
                let xmm5_len = ((dx_f64 * dx_f64) + (dy_f64 * dy_f64)).sqrt();
                let xmm2_dx = ((opp_dx * xmm3_steps) + opp_x0) - x0_f64;
                let xmm1_dy = (opp_y0 + xmm4_ody) - y0_f64;
                let xmm1_dist = ((xmm1_dy * xmm1_dy) + (xmm2_dx * xmm2_dx)).sqrt();
                let xmm1_f32 = (xmm1_dist / xmm5_len) as f32;
                let xmm3_sum = (xmm1_f32 as f64) + xmm13_half;
                let r8d_ws = (crate::simd_ops::cvt_f64_i32(xmm3_sum) as i32).wrapping_sub(v28);
                if r8d_ws < 0 {
                    let mut err_buf = [0u64; 4];
                    let msg = exe::ptr::<u8>(0x1_433f_0438) as usize;
                    let exc = exe::call_va!(
                        0x1_412e_8690,
                        (err_buf.as_mut_ptr() as usize, msg) -> usize
                    );
                    exe::call_va!(0x1_412e_88c0, (exc));
                    return 0;
                }
                let edx_term = opp_v24
                    .wrapping_sub(opp_v28)
                    .wrapping_sub(1)
                    .wrapping_mul(r15d_step)
                    .wrapping_add(opp_v30);
                v30 = r8d_ws.wrapping_mul(r15d_step).wrapping_add(edx_term);
                emit = true;
            }

            if emit {
                let out_x0 = (jitter_f32 * dx_f32) + xmm12_x0;
                let out_y0 = (jitter_f32 * dy_f32) + xmm11_y0;
                let len_20 = rd::<u32>(vec_20_ptr + 0x08) as usize;
                let cap_20 = rd::<u32>(vec_20_ptr + 0x0c) as usize;
                if len_20 * 0x28 == cap_20 * 0x28 {
                    exe::call_va!(0x1_411e_9730, (vec_20_ptr, (len_20 as u64).wrapping_add(1)));
                }
                let len_20 = rd::<u32>(vec_20_ptr + 0x08) as usize;
                let dst = rd::<usize>(vec_20_ptr).wrapping_add(len_20 * 0x28);
                wr::<f32>(dst, out_x0);
                wr::<f32>(dst + 0x04, out_y0);
                wr::<u64>(dst + 0x08, packed_dxy);
                wr::<i32>(dst + 0x10, r11d_slice);
                wr::<i32>(dst + 0x14, r8d_v24);
                wr::<i32>(dst + 0x18, v28);
                wr::<i32>(dst + 0x1c, v30);
                wr::<u32>(dst + 0x20, v2c);
                wr::<u32>(dst + 0x24, v38);
                wr::<u32>(vec_20_ptr + 0x08, (len_20 as u32).wrapping_add(1));
            }
        }
    }

    let count_20 = rd::<u32>(vec_20_ptr + 0x08) as usize;
    let rdi_20 = rd::<usize>(vec_20_ptr);
    if count_20 != 0 {
        let mut r15d_base: i32 = 0;
        while (r15d_base as i64 as usize) < count_20 {
            let w30 = rd::<i32>(rd::<usize>(param_1) + 0x30);
            let mut ecx = r15d_base.wrapping_add(1);
            let mut edx = r15d_base;
            while ecx < r15d_base.wrapping_add(w30) {
                let e_cur = rdi_20.wrapping_add((ecx as isize as usize) * 0x28);
                let e_prev = rdi_20.wrapping_add((edx as isize as usize) * 0x28);
                if rd::<i32>(e_cur + 0x14) != 0
                    && rd::<i32>(e_prev + 0x14) != 0
                    && rd::<i32>(e_cur + 0x1c) != rd::<i32>(e_prev + 0x1c).wrapping_add(1)
                {
                    let mut err_buf = [0u64; 4];
                    let msg = exe::ptr::<u8>(0x1_433f_0410) as usize;
                    let exc = exe::call_va!(
                        0x1_412e_8690,
                        (err_buf.as_mut_ptr() as usize, msg) -> usize
                    );
                    exe::call_va!(0x1_412e_88c0, (exc));
                    return 0;
                }
                ecx += 1;
                edx += 1;
            }
            r15d_base = r15d_base.wrapping_add(rd::<i32>(rd::<usize>(param_1) + 0x30));
        }
    }

    let cur_cnt = rd::<u32>(vec_20_ptr + 0x08) as i32;
    let sum = align_38.wrapping_sub(1).wrapping_add(cur_cnt);
    let padded_cnt = sum.wrapping_sub(sum % align_38);
    if cur_cnt < padded_cnt {
        let pad_n = (padded_cnt.wrapping_sub(cur_cnt)) as u32;
        for _ in 0..pad_n {
            let len_20 = rd::<u32>(vec_20_ptr + 0x08) as usize;
            let cap_20 = rd::<u32>(vec_20_ptr + 0x0c) as usize;
            if len_20 * 0x28 == cap_20 * 0x28 {
                exe::call_va!(0x1_411e_9730, (vec_20_ptr, (len_20 as u64).wrapping_add(1)));
            }
            let len_20 = rd::<u32>(vec_20_ptr + 0x08) as usize;
            let dst = rd::<usize>(vec_20_ptr).wrapping_add(len_20 * 0x28);
            wr::<[u64; 5]>(dst, def_40);
            wr::<u32>(vec_20_ptr + 0x08, (len_20 as u32).wrapping_add(1));
        }
    }

    let final_cnt = rd::<u32>(vec_20_ptr + 0x08) as i32;
    wr::<i32>(param_1 + 0x20, final_cnt);
    let cfg_final = rd::<usize>(param_1);
    wr::<u32>(cfg_final, rd::<u32>(cfg_final + 0x34));
    let total_bytes = (final_cnt as i64 as u64).wrapping_mul(0x28);
    wr::<u64>(param_2, total_bytes);
    let out_buf = exe::call_va!(0x1_4221_3e50, (0u64, 0u64, 0u64, total_bytes) -> usize);
    let _ = exe::call_va!(
        0x1_4183_acbc,
        (out_buf, rd::<usize>(vec_20_ptr), rd::<u64>(param_2)) -> usize
    );

    exe::call_va!(0x1_401d_e830, (vec_20_ptr));
    exe::call_va!(0x1_401d_e830, (vec_58_ptr));
    exe::call_va!(0x1_401d_e830, (vec_68_ptr));
    exe::call_va!(0x1_401d_e830, (vec_30_ptr));
    out_buf
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142210a10(orig: usize, new: usize) -> Result<u32, String> {
    static mut POOL_OBJ: [u8; 0xcc8] = [0; 0xcc8];
    static mut SLOT_MEM: [u64; 4] = [0; 4];
    static mut LOOKUP_IDX: u64 = 0;
    static mut KEY_LOG: u64 = 0;

    unsafe extern "win64" fn mock_2dc0(_map: usize, out_idx: usize, key_ptr: usize) -> usize {
        KEY_LOG ^= rd::<u64>(key_ptr);
        wr::<u64>(out_idx, LOOKUP_IDX);
        out_idx
    }
    unsafe extern "win64" fn mock_b450(_pool: usize) -> usize {
        core::ptr::addr_of_mut!(POOL_OBJ) as usize
    }
    unsafe extern "win64" fn mock_b280(_pool: usize, _obj: usize) -> u32 {
        0x42
    }
    unsafe extern "win64" fn mock_e360(_map: usize, key_ptr: usize) -> usize {
        KEY_LOG = KEY_LOG.wrapping_add(rd::<u64>(key_ptr));
        core::ptr::addr_of_mut!(SLOT_MEM) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4105_2dc0, mock_2dc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b450, mock_b450 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_b280, mock_b280 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411d_e360, mock_e360 as *const ())?;

    let run = |fn_addr: usize, vec3: [f32; 3], hit: bool| -> (u32, u64, [u32; 6], u8) {
        POOL_OBJ = [0xaa; 0xcc8];
        SLOT_MEM = [0; 4];
        KEY_LOG = 0;
        LOOKUP_IDX = if hit { 0 } else { u64::MAX };

        let existing_id: u32 = 0x77;
        let mut arr_a8 = [0u64; 4];
        let ap = arr_a8.as_mut_ptr() as usize;
        wr::<usize>(ap + 0x08, (&existing_id as *const u32) as usize);

        let mut p1 = [0u8; 0xd0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x80, 1);
        wr::<u64>(p1p + 0x88, 2);
        wr::<u64>(p1p + 0x90, 1);
        wr::<u64>(p1p + 0x98, 0xdead_beef);
        wr::<usize>(p1p + 0xa8, ap);

        let (ret, _) = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            vec3.as_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [
                0,
                0,
                0.75f32.to_bits() as u128,
                (-1.25f32).to_bits() as u128,
            ],
        );
        let obj_hdr = rd::<[u32; 6]>(core::ptr::addr_of!(POOL_OBJ) as usize);
        (ret as u32, KEY_LOG, obj_hdr, rd::<u8>(p1p + 0xc0))
    };

    for (v, hit) in [
        ([0.0f32, 0.0, 0.0], false),
        ([0.5, -0.5, 0.7071], true),
        ([0.5, -0.5, 0.7071], false),
    ] {
        let o = run(orig, v, hit);
        let n = run(new, v, hit);
        crate::diff_eq!(o, n, "142210a10 v={:?} hit={}", v, hit);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142212350(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GRID_MEM: [u8; 0x1c * 64] = [0; 0x1c * 64];
    static mut VEC_MEM: [u8; 0x1c * 16] = [0; 0x1c * 16];
    static mut OUT_MEM: [u32; 64] = [0; 64];

    unsafe extern "win64" fn mock_2230(_c: usize, _d: u32, _r8: u64, _cnt: u64) -> usize {
        core::ptr::addr_of_mut!(GRID_MEM) as usize
    }
    unsafe extern "win64" fn mock_91d0(vec_ptr: usize, req: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC_MEM) as usize);
        wr::<u32>(vec_ptr + 0x0c, (req as u32).max(16));
    }
    unsafe extern "win64" fn mock_2120(_c: usize, _d: usize, _r8: usize, _bytes: u64) -> usize {
        core::ptr::addr_of_mut!(OUT_MEM) as usize
    }
    unsafe extern "win64" fn mock_noop(_p: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4221_2230, mock_2230 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_91d0, mock_91d0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4221_2120, mock_2120 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4244_2de0, mock_noop as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401d_e830, mock_noop as *const ())?;

    let g_300 = exe::ptr::<i32>(0x1_43cf_0300) as usize;
    let g_304 = exe::ptr::<i32>(0x1_43cf_0304) as usize;
    let saved_300 = rd::<i32>(g_300);
    let saved_304 = rd::<i32>(g_304);
    wr::<i32>(g_300, 2);
    wr::<i32>(g_304, 0);

    let run = |f: F| -> (u64, Vec<u8>, [u32; 64]) {
        GRID_MEM = [0; 0x1c * 64];
        VEC_MEM = [0; 0x1c * 16];
        OUT_MEM = [0; 64];

        let mut cfg = [0u8; 0x50];
        let cp = cfg.as_mut_ptr() as usize;
        wr::<i32>(cp + 0x08, 4);
        wr::<f32>(cp + 0x14, 1.0);
        wr::<f32>(cp + 0x18, 1.0);
        wr::<i32>(cp + 0x20, 4);
        wr::<i32>(cp + 0x24, 4);
        wr::<i32>(cp + 0x28, 4);
        wr::<i32>(cp + 0x2c, 4);
        wr::<i32>(cp + 0x34, 2);
        wr::<i32>(cp + 0x3c, 4);
        wr::<i32>(cp + 0x40, 4);

        let mut lines = [0u8; 0x28 * 2];
        let lp = lines.as_mut_ptr() as usize;
        wr::<f32>(lp, 0.25);
        wr::<f32>(lp + 0x04, 0.25);
        wr::<f32>(lp + 0x08, 0.25);
        wr::<f32>(lp + 0x0c, 0.0);
        wr::<i32>(lp + 0x10, 0);
        wr::<i32>(lp + 0x14, 3);
        wr::<i32>(lp + 0x18, 0);
        wr::<i32>(lp + 0x1c, 10);

        let lp1 = lp + 0x28;
        wr::<f32>(lp1, 0.25);
        wr::<f32>(lp1 + 0x04, 0.5);
        wr::<f32>(lp1 + 0x08, 0.0);
        wr::<f32>(lp1 + 0x0c, 0.25);
        wr::<i32>(lp1 + 0x10, 1);
        wr::<i32>(lp1 + 0x14, 3);
        wr::<i32>(lp1 + 0x18, 0);
        wr::<i32>(lp1 + 0x1c, 20);

        let mut line_vec = [0u64; 2];
        let lvp = line_vec.as_mut_ptr() as usize;
        wr::<usize>(lvp, lp);
        wr::<u32>(lvp + 0x08, 2);
        wr::<u32>(lvp + 0x0c, 2);

        let p1: [usize; 2] = [lvp, cp];
        let mut out_ptr: usize = 0;
        let ret = f(p1.as_ptr() as usize, (&mut out_ptr as *mut usize) as usize, 0);
        let grid_slice = core::slice::from_raw_parts(
            core::ptr::addr_of!(GRID_MEM) as *const u8,
            0x1c * 32,
        )
        .to_vec();
        let out_arr = rd::<[u32; 64]>(core::ptr::addr_of!(OUT_MEM) as usize);
        (ret, grid_slice, out_arr)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<i32>(g_300, saved_300);
    wr::<i32>(g_304, saved_304);
    crate::diff_eq!(o, n, "142212350");
    Ok(1)
}

pub unsafe fn diff_unknown_142213f60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA: [u8; 0x8000] = [0; 0x8000];
    static mut ARENA_OFF: usize = 0;

    unsafe fn arena_alloc(bytes: usize) -> usize {
        let aligned = (ARENA_OFF + 63) & !63;
        let ptr = (core::ptr::addr_of_mut!(ARENA) as usize).wrapping_add(aligned);
        ARENA_OFF = aligned + bytes.max(64);
        ptr
    }

    unsafe extern "win64" fn mock_b7a0(_rcx: usize, cap: u64) -> usize {
        arena_alloc((cap as usize) * 64)
    }
    unsafe extern "win64" fn mock_2010(vec_ptr: usize, req: u64) {
        let old_ptr = rd::<usize>(vec_ptr);
        let old_len = rd::<u32>(vec_ptr + 0x08) as usize;
        let new_cap = (req as usize).max(16);
        let new_ptr = arena_alloc(new_cap * 64);
        if old_ptr != 0 && old_len != 0 {
            core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, old_len * 64);
        }
        wr::<usize>(vec_ptr, new_ptr);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_9730(vec_ptr: usize, req: u64) {
        let old_ptr = rd::<usize>(vec_ptr);
        let old_len = rd::<u32>(vec_ptr + 0x08) as usize;
        let new_cap = (req as usize).max(16);
        let new_ptr = arena_alloc(new_cap * 0x28);
        if old_ptr != 0 && old_len != 0 {
            core::ptr::copy_nonoverlapping(
                old_ptr as *const u8,
                new_ptr as *mut u8,
                old_len * 0x28,
            );
        }
        wr::<usize>(vec_ptr, new_ptr);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_3e50(_c: usize, _d: usize, _r8: usize, bytes: u64) -> usize {
        arena_alloc(bytes as usize)
    }
    unsafe extern "win64" fn mock_e830(vec_ptr: usize) {
        wr::<usize>(vec_ptr, 0);
        wr::<u64>(vec_ptr + 0x08, 0);
    }
    unsafe extern "win64" fn mock_3400(_p1: usize, _v30: usize, _w: u64) {}
    unsafe extern "win64" fn mock_3ae0(_p1: usize, _v30: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4023_b7a0, mock_b7a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411a_2010, mock_2010 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_9730, mock_9730 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4221_3e50, mock_3e50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4221_3400, mock_3400 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4221_3ae0, mock_3ae0 as *const ())?;

    let run = |f: F, mode_1c: i32| -> (u64, [i32; 4], Vec<u8>) {
        ARENA = [0; 0x8000];
        ARENA_OFF = 0;

        let mut cfg = [0u8; 0x40];
        let cp = cfg.as_mut_ptr() as usize;
        wr::<i32>(cp + 0x08, 2); // half_slices = 1
        wr::<i32>(cp + 0x1c, mode_1c);
        wr::<i32>(cp + 0x20, 4);
        wr::<i32>(cp + 0x24, 4);
        wr::<i32>(cp + 0x30, 2); // coalesce_width = 2
        wr::<i32>(cp + 0x34, 4); // step_stride = 4
        wr::<i32>(cp + 0x38, 2); // align_38 = 2

        let mut in_lines = [0u8; 64 * 2];
        let ilp = in_lines.as_mut_ptr() as usize;
        for k in 0..2usize {
            let e = ilp + k * 64;
            wr::<f64>(e, 0.25);
            wr::<f64>(e + 0x08, 0.25 + (k as f64) * 0.25);
            wr::<f64>(e + 0x10, 0.25);
            wr::<f64>(e + 0x18, 0.0);
            wr::<i32>(e + 0x20, 0);
            wr::<i32>(e + 0x24, 3);
            wr::<i32>(e + 0x28, 0);
            wr::<i32>(e + 0x2c, 3);
            wr::<i32>(e + 0x30, (k as i32) * 4);
            wr::<i32>(e + 0x34, k as i32);
        }

        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, cp);
        wr::<usize>(p1p + 0x08, ilp);
        wr::<u32>(p1p + 0x10, 2);
        wr::<f64>(p1p + 0x28, -2.0);
        wr::<f64>(p1p + 0x30, -2.0);
        wr::<f64>(p1p + 0x48, 2.0);
        wr::<f64>(p1p + 0x50, 2.0);
        wr::<f64>(p1p + 0x68, 0.0);
        wr::<f64>(p1p + 0x70, 0.0);
        wr::<f64>(p1p + 0x88, 4.0);
        wr::<f64>(p1p + 0x90, 4.0);
        wr::<i32>(p1p + 0xac, 10);
        wr::<i32>(p1p + 0xb0, 0);

        let mut out_bytes: u64 = 0;
        let out_ptr = f(p1p, (&mut out_bytes as *mut u64) as usize);
        let out_slice =
            core::slice::from_raw_parts(out_ptr as *const u8, out_bytes as usize).to_vec();
        let meta = [
            rd::<i32>(p1p + 0x20),
            rd::<i32>(p1p + 0x24),
            rd::<i32>(p1p + 0xa8),
            rd::<i32>(cp + 0x04),
        ];
        (out_bytes, meta, out_slice)
    };

    for mode_1c in [0i32, 2, 3] {
        let o = run(f_o, mode_1c);
        let n = run(f_n, mode_1c);
        crate::diff_eq!(o, n, "142213f60 mode_1c={}", mode_1c);
    }
    Ok(3)
}

// REGISTRY (for games/alan-wake-2/crates/aw2-game/src/registry/core.rs):
// Replacement::verified_throws(0x1_4221_0a10, "unknown_142210a10", rend::cauldron_hot_z7::unknown_142210a10 as *const (), Throws::Discard, rend::cauldron_hot_z7::diff_unknown_142210a10),
// Replacement::verified_throws(0x1_4221_2350, "unknown_142212350", rend::cauldron_hot_z7::unknown_142212350 as *const (), Throws::Caught, rend::cauldron_hot_z7::diff_unknown_142212350),
// Replacement::verified_throws(0x1_4221_3f60, "unknown_142213f60", rend::cauldron_hot_z7::unknown_142213f60 as *const (), Throws::Caught, rend::cauldron_hot_z7::diff_unknown_142213f60),
