//! Decompiled `rend` hot functions shard ZA14 (`0x14212c860..0x14213b7a0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn skip_neg_ctrl_bytes(mut r8: usize, mut r9: usize) -> (usize, usize) {
    if rd::<i8>(r8) < -1 {
        loop {
            let chunk = rd::<u128>(r8);
            let cmp = crate::simd_ops::vpcmpgtb(u128::MAX, chunk);
            let mask = (crate::simd_ops::vpmovmskb(cmp) as u32).wrapping_add(1);
            let step = if mask == 0 {
                0usize
            } else {
                mask.trailing_zeros() as usize
            };
            r8 = r8.wrapping_add(step);
            r9 = r9.wrapping_add(step << 5);
            if rd::<i8>(r8) >= -1 {
                break;
            }
        }
    }
    (r8, r9)
}

#[inline(always)]
fn fnv1a_u32_then_u64(val: u32) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    let mut h1 = FNV_OFFSET;
    for &b in &val.to_le_bytes() {
        h1 = (h1 ^ (b as u64)).wrapping_mul(FNV_PRIME);
    }
    let mut h2 = FNV_OFFSET;
    for &b in &h1.to_le_bytes() {
        h2 = (h2 ^ (b as u64)).wrapping_mul(FNV_PRIME);
    }
    h2
}

#[inline(always)]
fn hash_combine(seed: u64, h: u64) -> u64 {
    seed ^ h
        .wrapping_add(seed << 6)
        .wrapping_add((seed >> 2).wrapping_add(0x9e37_79b9))
}

/// `unknown_14212c860`
/// Original: 0x14212c860 (941 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14212c860(param_1: usize, param_2: usize, param_3: usize) {
    let rsi = rd::<usize>(param_3 + 0x28);
    let r11 = rd::<usize>(param_1 + 0x20);
    let r9d_m1 = rd::<i32>(rsi + 0x08).wrapping_sub(1);
    let s10 = rd::<i32>(rsi + 0x10);
    let s18 = rd::<i32>(rsi + 0x18);
    let s0c = rd::<i32>(rsi + 0x0c);
    let s14 = rd::<i32>(rsi + 0x14);
    let max_x = s10.wrapping_add(r9d_m1);
    let max_y = s0c.wrapping_sub(1).wrapping_add(s14);
    let max_z = s18.wrapping_add(r9d_m1);
    let r9_init = rd::<usize>(param_1 + 0x28);

    let (mut r8, mut r9) = skip_neg_ctrl_bytes(r11, r9_init);
    let end_ctrl = r11.wrapping_add(rd::<usize>(param_1 + 0x38));

    while r8 != end_ctrl {
        let r13 = r9;
        let cnt = rd::<u32>(r13 + 0x18) as usize;
        let data_ptr = rd::<usize>(r13 + 0x10);
        if data_ptr != data_ptr.wrapping_add(cnt.wrapping_mul(12)) {
            let mut local_b0 = [0u8; 64];
            let ret_ptr = exe::call_va!(
                0x1_4212_e700,
                (local_b0.as_mut_ptr() as usize, param_2, r13) -> usize
            );
            let dim_08 = rd::<i32>(ret_ptr + 0x08);
            let dim_0c = rd::<i32>(ret_ptr + 0x0c);
            let v10 = rd::<i32>(ret_ptr + 0x10);
            let v14 = rd::<i32>(ret_ptr + 0x14);
            let v18 = rd::<i32>(ret_ptr + 0x18);
            let v1c = rd::<i32>(ret_ptr + 0x1c);
            let v20 = rd::<i32>(ret_ptr + 0x20);
            let v24 = rd::<i32>(ret_ptr + 0x24);
            let v28 = rd::<i32>(ret_ptr + 0x28);
            let v2c = rd::<i32>(ret_ptr + 0x2c);
            let v30 = rd::<i32>(ret_ptr + 0x30);

            let eax_z = v30.wrapping_add(v18);
            let ebx_x = v1c.wrapping_add(v10);
            let ecx_y = v20.wrapping_add(v14);
            let edx_z = v24.wrapping_add(v18);
            let r8d_x = v28.wrapping_add(v10);
            let r10d_y = v2c.wrapping_add(v14);

            let y_start = (if ecx_y > s14 { ecx_y } else { s14 }).wrapping_sub(v14);
            let z_start = (if edx_z > s18 { edx_z } else { s18 }).wrapping_sub(v18);
            let x_end = (if r8d_x < max_x { r8d_x } else { max_x }).wrapping_sub(v10);
            let y_end = (if r10d_y < max_y { r10d_y } else { max_y }).wrapping_sub(v14);
            let z_end = (if eax_z < max_z { eax_z } else { max_z }).wrapping_sub(v18);

            let dx_off = v10.wrapping_sub(rd::<i32>(rsi + 0x10));
            let dy_off = v14.wrapping_sub(rd::<i32>(rsi + 0x14));
            let dz_off = v18.wrapping_sub(rd::<i32>(rsi + 0x18));

            let x_start = (if ebx_x > s10 { ebx_x } else { s10 }).wrapping_sub(v10);
            if x_start <= x_end {
                let mut x = x_start;
                while x <= x_end {
                    let mut y = y_start;
                    while y <= y_end {
                        if z_start <= z_end {
                            let base_idx = dim_08
                                .wrapping_mul(z_start)
                                .wrapping_add(x)
                                .wrapping_mul(dim_0c)
                                .wrapping_add(y) as isize;
                            let mut src_off = base_idx.wrapping_mul(12);
                            let src_step =
                                (dim_0c.wrapping_mul(dim_08) as isize).wrapping_mul(12);
                            let mut z = z_start;
                            while z <= z_end {
                                let src_base = rd::<usize>(r13 + 0x10);
                                let src_addr =
                                    (src_base as isize).wrapping_add(src_off) as usize;
                                let val_lo = rd::<u64>(src_addr);
                                let val_hi = rd::<u32>(src_addr + 0x08);
                                src_off = src_off.wrapping_add(src_step);

                                let dst_idx = dz_off
                                    .wrapping_add(z)
                                    .wrapping_mul(rd::<i32>(rsi + 0x08))
                                    .wrapping_add(dx_off)
                                    .wrapping_add(x)
                                    .wrapping_mul(rd::<i32>(rsi + 0x0c))
                                    .wrapping_add(dy_off)
                                    .wrapping_add(y)
                                    as isize;
                                let dst_base = rd::<usize>(param_3 + 0x08);
                                let dst_addr = (dst_base as isize)
                                    .wrapping_add(dst_idx.wrapping_mul(12))
                                    as usize;
                                wr::<u64>(dst_addr, val_lo);
                                wr::<u32>(dst_addr + 0x08, val_hi);

                                z = z.wrapping_add(1);
                            }
                        }
                        y = y.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
            }
        }
        r8 = r8.wrapping_add(1);
        r9 = r9.wrapping_add(0x20);
        let next = skip_neg_ctrl_bytes(r8, r9);
        r8 = next.0;
        r9 = next.1;
    }
}

/// `unknown_14212cc20`
/// Original: 0x14212cc20 (970 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14212cc20(param_1: usize, param_2: usize, param_3: usize) {
    wr::<u64>(param_1 + 0x08, rd::<u64>(param_2));
    wr::<u32>(param_1 + 0x10, rd::<u32>(param_2 + 0x08));

    let mut local_78 = [0u8; 96];
    let ret = exe::call_va!(
        0x1_4213_a660,
        (local_78.as_mut_ptr() as usize, param_3) -> usize
    );

    let r08 = rd::<i32>(ret + 0x08);
    let r0c = rd::<i32>(ret + 0x0c);
    let r10 = rd::<i32>(ret + 0x10);
    let r14 = rd::<i32>(ret + 0x14);
    let r18 = rd::<i32>(ret + 0x18);

    let g_bd0 = rd::<i32>(exe::ptr::<i32>(0x1_4386_3bd0) as usize) as f32;
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm6 = one / g_bd0;

    let ebx = crate::simd_ops::cvt_f32_i32(((r18 as f32) * xmm6).floor()) as i32;
    let r11d = crate::simd_ops::cvt_f32_i32(((r14 as f32) * xmm6).floor()) as i32;
    let r10d = crate::simd_ops::cvt_f32_i32(((r10 as f32) * xmm6).floor()) as i32;
    let edx = crate::simd_ops::cvt_f32_i32(((r18.wrapping_add(r08) as f32) * xmm6).ceil()) as i32;
    let ecx = crate::simd_ops::cvt_f32_i32(((r0c.wrapping_add(r14) as f32) * xmm6).ceil()) as i32;
    let eax = crate::simd_ops::cvt_f32_i32(((r10.wrapping_add(r08) as f32) * xmm6).ceil()) as i32;

    let mut box1: [i32; 6] = [r10d, r11d, ebx, eax, ecx, edx];
    let mut vec1: [u64; 2] = [0; 2];
    exe::call_va!(
        0x1_4212_d4e0,
        (
            vec1.as_mut_ptr() as usize,
            param_1,
            box1.as_mut_ptr() as usize
        )
    );

    let rsi_ptr = rd::<usize>(vec1.as_ptr() as usize);
    let len1 = rd::<u32>((vec1.as_ptr() as usize) + 0x08) as u64;
    let r15_cnt = ((len1.wrapping_mul(12)) as i64) / 12;

    let mut span1: [u64; 2] = [r15_cnt as u64, rsi_ptr as u64];
    exe::call_va!(
        0x1_4212_d050,
        (param_1, param_2, param_3, span1.as_mut_ptr() as usize)
    );

    let v30_0 = rd::<f32>(param_3 + 0x30);
    let v30_1 = rd::<f32>(param_3 + 0x34);
    let v30_2 = rd::<f32>(param_3 + 0x38);
    let g_bd0_2 = rd::<i32>(exe::ptr::<i32>(0x1_4386_3bd0) as usize) as f32;
    let g_170 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize);
    let prod = g_bd0_2 * g_170;
    let xmm5 = if prod.is_nan() || prod != 0.0 {
        one / prod
    } else {
        0.0
    };

    let pz = crate::simd_ops::cvt_f32_i32((v30_2 * xmm5).floor()) as i32;
    let py = crate::simd_ops::cvt_f32_i32((v30_1 * xmm5).floor()) as i32;
    let px = crate::simd_ops::cvt_f32_i32((v30_0 * xmm5).floor()) as i32;
    let rad = rd::<i32>(exe::ptr::<i32>(0x1_4386_3f50) as usize);

    let mut pt_and_box2: [i32; 9] = [
        px,
        py,
        pz,
        px.wrapping_sub(rad),
        py.wrapping_sub(rad),
        pz.wrapping_sub(rad),
        px.wrapping_add(rad),
        py.wrapping_add(rad),
        pz.wrapping_add(rad),
    ];
    let pt_ptr = pt_and_box2.as_mut_ptr() as usize;
    let box2_ptr = pt_ptr + 0x0c;

    exe::call_va!(0x1_4212_d730, (param_1, box2_ptr));

    let mut vec2: [u64; 2] = [0; 2];
    exe::call_va!(
        0x1_4212_d4e0,
        (vec2.as_mut_ptr() as usize, param_1, box2_ptr)
    );

    let rdi_ptr = rd::<usize>(vec2.as_ptr() as usize);
    let len2 = rd::<u32>((vec2.as_ptr() as usize) + 0x08) as u64;
    let rbx_cnt = ((len2.wrapping_mul(12)) as i64) / 12;
    let end_ptr = rdi_ptr.wrapping_add((rbx_cnt as usize).wrapping_mul(12));
    let sort_cnt = ((end_ptr.wrapping_sub(rdi_ptr)) as i64) / 12;

    exe::call_va!(
        0x1_4123_e490,
        (rdi_ptr, end_ptr, sort_cnt as u64, pt_ptr)
    );

    let g_098 = rd::<i32>(exe::ptr::<i32>(0x1_4386_4098) as usize);
    let rem = g_098.wrapping_sub(r15_cnt as i32);
    let clamped_rem = if rem > 0 { rem } else { 0 };
    let mut final_cnt = (clamped_rem as i64) as u64;
    if (rbx_cnt as u64) < final_cnt || final_cnt == u64::MAX {
        final_cnt = rbx_cnt as u64;
    }

    let mut span2: [u64; 2] = [final_cnt, rdi_ptr as u64];
    exe::call_va!(
        0x1_4212_d050,
        (param_1, param_2, param_3, span2.as_mut_ptr() as usize)
    );

    if rdi_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (rdi_ptr));
    }
    if rsi_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (rsi_ptr));
    }
}

/// `unknown_14212e240`
/// Original: 0x14212e240 (1156 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14212e240(param_1: usize, param_2: usize) {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const GOLDEN: u64 = 0x9e37_79b9;

    let ebx = rd::<u8>(exe::ptr::<u8>(0x1_4386_4190) as usize);
    let mut r15 = ((ebx as u64) ^ FNV_OFFSET)
        .wrapping_mul(FNV_PRIME)
        .wrapping_add(GOLDEN);

    let ebp = rd::<u8>(exe::ptr::<u8>(0x1_4386_3e60) as usize);
    r15 = hash_combine(r15, ((ebp as u64) ^ FNV_OFFSET).wrapping_mul(FNV_PRIME));

    let edi = rd::<u8>(exe::ptr::<u8>(0x1_4386_3ba0) as usize);
    let h_edi = ((edi as u64) ^ FNV_OFFSET).wrapping_mul(FNV_PRIME);
    let rcx_arg = h_edi
        .wrapping_add(r15 << 6)
        .wrapping_add((r15 >> 2).wrapping_add(GOLDEN));
    r15 ^= rcx_arg;

    let rax_hash = exe::call_va!(0x1_4213_aca0, (rcx_arg, param_2) -> u64);
    r15 = hash_combine(r15, rax_hash);

    let f_170_bits = rd::<u32>(exe::ptr::<u32>(0x1_4386_4170) as usize);
    let norm_170 = if f32::from_bits(f_170_bits) == 0.0 {
        0u32
    } else {
        f_170_bits
    };
    r15 = hash_combine(r15, fnv1a_u32_then_u64(norm_170));

    let f_ce0_bits = rd::<u32>(exe::ptr::<u32>(0x1_4386_3ce0) as usize);
    let norm_ce0 = if f32::from_bits(f_ce0_bits) == 0.0 {
        0u32
    } else {
        f_ce0_bits
    };
    r15 = hash_combine(r15, fnv1a_u32_then_u64(norm_ce0));

    let u_bd0 = rd::<u32>(exe::ptr::<u32>(0x1_4386_3bd0) as usize);
    r15 = hash_combine(r15, fnv1a_u32_then_u64(u_bd0));

    let any_flag = ebx != 0 || ebp != 0 || edi != 0;
    let f08 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3f08) as usize);
    let cc8 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3cc8) as usize);

    if f08 != 0 || (cc8 == 0 && !any_flag) || rd::<u64>(param_1 + 0x18) != r15 {
        let mut tmp_20 = [0u8; 0x50];
        let t20p = tmp_20.as_mut_ptr() as usize;
        let vtbl_618 = exe::ptr::<u8>(0x1_433e_2618) as usize;
        let empty_ctrl = exe::ptr::<u8>(0x1_4307_82a0) as usize;

        wr::<usize>(t20p, vtbl_618);
        wr::<u64>(t20p + 0x08, 0);
        wr::<u64>(t20p + 0x10, 0);
        wr::<u64>(t20p + 0x18, u64::MAX);
        wr::<usize>(t20p + 0x20, empty_ctrl);
        wr::<u64>(t20p + 0x28, 0);
        wr::<u64>(t20p + 0x30, 0);
        wr::<u64>(t20p + 0x38, 0);
        wr::<u64>(t20p + 0x40, 0);
        wr::<u64>(t20p + 0x48, 0);

        wr::<u64>(param_1 + 0x08, 0);
        wr::<u32>(param_1 + 0x10, 0);
        wr::<u64>(param_1 + 0x18, u64::MAX);

        let rbp_ctrl = rd::<usize>(param_1 + 0x20);
        wr::<usize>(param_1 + 0x20, empty_ctrl);
        let rdi_buckets = rd::<usize>(param_1 + 0x28);
        wr::<u64>(param_1 + 0x28, 0);
        wr::<u64>(param_1 + 0x30, 0);
        let r14_cap = rd::<u64>(param_1 + 0x38) as usize;
        wr::<u64>(param_1 + 0x38, 0);
        wr::<u64>(param_1 + 0x48, 0);

        if r14_cap != 0 {
            let mut cur_bucket = rdi_buckets.wrapping_add(0x10);
            let mut idx = 0usize;
            let mut last_rcx = 0usize;
            while idx != r14_cap {
                if rd::<i8>(rbp_ctrl.wrapping_add(idx)) >= 0 {
                    last_rcx = exe::call_va!(0x1_401d_e830, (cur_bucket) -> usize);
                }
                idx = idx.wrapping_add(1);
                cur_bucket = cur_bucket.wrapping_add(0x20);
            }
            exe::call_va!(0x1_4020_b500, (last_rcx, rbp_ctrl));
        }

        exe::call_va!(0x1_4212_b880, (t20p + 0x20));
        wr::<u64>(param_1 + 0x18, r15);
    }
}

/// `unknown_142139f10`
/// Original: 0x142139f10 (1485 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_142139f10(param_1: usize, param_2: usize) {
    let r13 = rd::<usize>(param_1 + 0x28);
    let mut rbx = rd::<usize>(param_2);
    let off_60 = rd::<f64>(param_2 + 0x60) as f32;
    let grid_08_m1 = rd::<i32>(r13 + 0x08).wrapping_sub(1);
    let grid_0c_m1 = rd::<i32>(r13 + 0x0c).wrapping_sub(1);
    let step0 = rd::<f32>(r13);
    let off_68 = rd::<f64>(param_2 + 0x68) as f32;
    let pad = rd::<f32>(exe::ptr::<f32>(0x1_4386_3ce0) as usize);
    let off_70 = rd::<f64>(param_2 + 0x70) as f32;
    let cnt = rd::<u32>(param_2 + 0x08) as usize;
    let rdi_end = rbx.wrapping_add(cnt.wrapping_mul(0x50));
    let step_sq = step0 * step0;

    if rbx == rdi_end {
        return;
    }

    let mut rsi = rbx.wrapping_add(0x28);
    let half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let inflation = rd::<f32>(exe::ptr::<f32>(0x1_4342_68e0) as usize);

    while rbx != rdi_end {
        let cl = rd::<u8>(rsi + 0x04);
        if (cl & 1) == 0 {
            let bit2_inv = (!(cl >> 2)) & 1;
            let extra = if (cl & 2) != 0 {
                rd::<f32>(exe::ptr::<f32>(0x1_4386_3b60) as usize)
                    * rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize)
            } else {
                0.0
            };
            let radius = extra + rd::<f32>(rsi + 0x20);
            let dx = rd::<f32>(rsi + 0x14) - rd::<f32>(rsi + 0x08);
            let dy = rd::<f32>(rsi + 0x18) - rd::<f32>(rsi + 0x0c);
            let dz = rd::<f32>(rsi + 0x1c) - rd::<f32>(rsi + 0x10);
            let dist_sq = (dy * dy + dx * dx) + dz * dz;

            let mut geom_sphere = [0u32; 2];
            let mut geom_capsule = [0u32; 3];
            let gs_ptr = geom_sphere.as_mut_ptr() as usize;
            let gc_ptr = geom_capsule.as_mut_ptr() as usize;
            wr::<u32>(gs_ptr, 0);
            wr::<f32>(gs_ptr + 0x04, radius);
            wr::<u32>(gc_ptr, 2);
            wr::<f32>(gc_ptr + 0x04, radius);

            let dist = if 0.0f32 > dist_sq {
                let (_, x0) = crate::simd_ops::call_va8_xmm(
                    0x1_4183_add0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [dist_sq.to_bits() as u128, 0, 0, 0],
                );
                f32::from_bits(x0 as u32)
            } else {
                dist_sq.sqrt()
            };
            wr::<f32>(gc_ptr + 0x08, dist * half);

            let obj = rd::<usize>(rbx);
            let vtbl = rd::<usize>(obj);
            let vfn = rd::<u64>(vtbl + 0x30);
            let is_capsule = exe::call_va!(vfn, (obj) -> u32);

            let mut transform = [0u8; 0x20];
            let tf_ptr = transform.as_mut_ptr() as usize;
            wr::<u128>(tf_ptr, rd::<u128>(rsi - 0x18));
            wr::<f32>(tf_ptr + 0x10, off_60 + rd::<f32>(rsi - 0x08));
            wr::<f32>(tf_ptr + 0x14, off_68 + rd::<f32>(rsi - 0x04));
            wr::<f32>(tf_ptr + 0x18, off_70 + rd::<f32>(rsi));

            let geom_ptr = if is_capsule != 0 { gc_ptr } else { gs_ptr };
            let mut bounds = [0.0f32; 6];
            let b_ptr = bounds.as_mut_ptr() as usize;

            let fn_world_bounds = rd::<u64>(exe::ptr::<u64>(0x1_4264_5d08) as usize);
            let _ = crate::simd_ops::call_va8_xmm(
                fn_world_bounds,
                b_ptr as u64,
                geom_ptr as u64,
                tf_ptr as u64,
                0,
                0,
                0,
                0,
                0,
                [0, 0, 0, inflation.to_bits() as u128],
            );

            let inv_step = rd::<f32>(r13 + 0x04);
            let min_x = rd::<f32>(b_ptr);
            let min_y = rd::<f32>(b_ptr + 0x04);
            let min_z = rd::<f32>(b_ptr + 0x08);
            let max_x = rd::<f32>(b_ptr + 0x0c);
            let max_y = rd::<f32>(b_ptr + 0x10);
            let max_z = rd::<f32>(b_ptr + 0x14);

            let off_z = rd::<i32>(r13 + 0x18) as f32;
            let off_y = rd::<i32>(r13 + 0x14) as f32;
            let off_x = rd::<i32>(r13 + 0x10) as f32;

            let mut y0 =
                crate::simd_ops::cvt_f32_i32(((min_y - pad) * inv_step - off_y).floor()) as i32;
            let mut x0 =
                crate::simd_ops::cvt_f32_i32(((min_x - pad) * inv_step - off_x).floor()) as i32;
            let mut z0 =
                crate::simd_ops::cvt_f32_i32(((min_z - pad) * inv_step - off_z).floor()) as i32;
            if x0 < 0 {
                x0 = 0;
            }
            if y0 < 0 {
                y0 = 0;
            }
            if z0 < 0 {
                z0 = 0;
            }

            let mut x1 =
                crate::simd_ops::cvt_f32_i32(((pad + max_x) * inv_step - off_x).floor()) as i32;
            let mut y1 = (crate::simd_ops::cvt_f32_i32(((pad + max_y) * inv_step - off_y).floor())
                as i32)
                .wrapping_add(1);
            let mut z1 =
                crate::simd_ops::cvt_f32_i32(((pad + max_z) * inv_step - off_z).floor()) as i32;
            if grid_08_m1 < x1 {
                x1 = grid_08_m1;
            }
            if grid_0c_m1 < y1 {
                y1 = grid_0c_m1;
            }
            if grid_08_m1 < z1 {
                z1 = grid_08_m1;
            }

            let mut z = z0;
            while z <= z1 {
                let fz = (rd::<i32>(r13 + 0x18).wrapping_add(z) as f32) * rd::<f32>(r13);
                let mut x = x0;
                while x <= x1 {
                    let fx = (rd::<i32>(r13 + 0x10).wrapping_add(x) as f32) * rd::<f32>(r13);
                    let mut y = y0;
                    while y <= y1 {
                        let fy = (rd::<i32>(r13 + 0x14).wrapping_add(y) as f32) * rd::<f32>(r13);
                        let mut pt = [fx, fy, fz];
                        let mut closest = [0.0f32; 3];
                        exe::call_iat!(
                            0x1_4264_5d40,
                            (
                                pt.as_mut_ptr() as usize,
                                geom_ptr,
                                tf_ptr,
                                closest.as_mut_ptr() as usize
                            )
                        );
                        let c_ptr = closest.as_ptr() as usize;
                        let vy = rd::<f32>(c_ptr + 0x04) - fy;
                        let vx = rd::<f32>(c_ptr) - fx;
                        let vz = rd::<f32>(c_ptr + 0x08) - fz;
                        let d2 = (vx * vx + vy * vy) + vz * vz;

                        if d2 <= step_sq {
                            let info = rd::<usize>(param_1 + 0x28);
                            let idx = z
                                .wrapping_mul(rd::<i32>(info + 0x08))
                                .wrapping_add(x)
                                .wrapping_mul(rd::<i32>(info + 0x0c))
                                .wrapping_add(y) as isize;
                            let vec_base = rd::<usize>(param_1 + 0x08);
                            let cell_addr =
                                (vec_base as isize).wrapping_add(idx.wrapping_mul(12)) as usize;
                            let oy = rd::<f32>(cell_addr + 0x04);
                            let ox = rd::<f32>(cell_addr);
                            let oz = rd::<f32>(cell_addr + 0x08);
                            let old_d2 = (oy * oy + ox * ox) + oz * oz;

                            if old_d2 > d2 {
                                wr::<f32>(cell_addr, vx);
                                wr::<f32>(cell_addr + 0x04, vy);
                                wr::<f32>(cell_addr + 0x08, vz);

                                let info2 = rd::<usize>(param_1 + 0x28);
                                let idx2 = z
                                    .wrapping_mul(rd::<i32>(info2 + 0x08))
                                    .wrapping_add(x)
                                    .wrapping_mul(rd::<i32>(info2 + 0x0c))
                                    .wrapping_add(y)
                                    as isize;
                                let flag_addr =
                                    (rd::<usize>(param_1 + 0x18) as isize).wrapping_add(idx2)
                                        as usize;
                                let old_b = rd::<u8>(flag_addr);
                                let dl = if (rd::<u8>(rsi + 0x04) & 2) != 0 {
                                    (old_b & 0xfa) | 2
                                } else {
                                    (old_b & 0xfc) | 4
                                };
                                let r8b = if bit2_inv != 0 { dl | 8 } else { dl };
                                let best_low = if (old_b & 7) < (r8b & 7) { r8b } else { old_b };
                                wr::<u8>(flag_addr, (best_low & 7) | ((old_b | r8b) & 8));
                            }
                        }
                        y = y.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
                z = z.wrapping_add(1);
            }
        }
        rbx = rbx.wrapping_add(0x50);
        rsi = rsi.wrapping_add(0x50);
    }
}

/// `unknown_14213a510`
/// Original: 0x14213a510 (329 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14213a510(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let step = rd::<f32>(param_2);
    let v60_0 = rd::<f32>(param_3 + 0x60) as f64;
    let v60_1 = rd::<f32>(param_3 + 0x64) as f64;
    let v60_2 = rd::<f32>(param_3 + 0x68) as f64;

    let d18 = rd::<i32>(param_2 + 0x18);
    let d10 = rd::<i32>(param_2 + 0x10);
    let d08 = rd::<i32>(param_2 + 0x08);
    let xmm7 = rd::<u128>(param_3 + 0x50);
    let d14 = rd::<i32>(param_2 + 0x14);
    let d0c = rd::<i32>(param_2 + 0x0c);

    wr::<f64>(param_1, ((d10.wrapping_sub(1) as f32) * step) as f64);
    wr::<f64>(param_1 + 0x08, ((d14.wrapping_sub(1) as f32) * step) as f64);
    wr::<f64>(param_1 + 0x10, ((d18.wrapping_sub(1) as f32) * step) as f64);
    wr::<f64>(param_1 + 0x18, ((d08.wrapping_add(d10) as f32) * step) as f64);
    wr::<f64>(param_1 + 0x20, ((d14.wrapping_add(d0c) as f32) * step) as f64);
    wr::<f64>(param_1 + 0x28, ((d08.wrapping_add(d18) as f32) * step) as f64);

    wr::<u32>(param_1 + 0x30, xmm7 as u32);
    wr::<u32>(param_1 + 0x34, (xmm7 >> 32) as u32);
    wr::<u32>(param_1 + 0x38, (xmm7 >> 64) as u32);
    wr::<u32>(param_1 + 0x3c, (xmm7 >> 96) as u32);

    wr::<f64>(param_1 + 0x40, v60_0);
    wr::<f64>(param_1 + 0x48, v60_1);
    wr::<f64>(param_1 + 0x50, v60_2);
    param_1
}

/// `unknown_14213a660`
/// Original: 0x14213a660 (469 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14213a660(param_1: usize, param_2: usize) -> usize {
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let g_e98 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3e98) as usize);
    let g_d38 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3d38) as usize);
    let g_170 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize);
    let inv_170 = one / g_170;

    let mut xmm3 = rd::<u128>(param_2 + 0x70);
    if g_e98.is_nan() || g_e98 != 0.0 || g_d38.is_nan() || g_d38 != 0.0 {
        xmm3 = crate::simd_ops::vinsertps(0, g_e98.to_bits() as u128, 0);
        xmm3 = crate::simd_ops::vinsertps(xmm3, g_d38.to_bits() as u128, 0x20);
    }

    let xmm1_f32 = if rd::<u8>(param_2 + 0xb0) == 0 {
        let dp = crate::simd_ops::vdpps(xmm3, xmm3, 0x5f);
        f32::from_bits(crate::simd_ops::vsqrtps(dp) as u32)
    } else {
        f32::from_bits(((xmm3 >> 32) as u32) & 0x7fff_ffff)
    };

    let mut ecx = rd::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize);
    let xmm0_scaled = xmm1_f32 * inv_170;
    let xmm6_val = xmm0_scaled + xmm0_scaled;

    let abs_y = f32::from_bits(((xmm3 >> 32) as u32) & 0x7fff_ffff);
    let y_scaled = abs_y * inv_170;
    let y_twice = y_scaled + y_scaled;
    let eax_y = (crate::simd_ops::cvt_f32_i32(y_twice.floor()) as i32).wrapping_add(10);

    if 0.0f32 > (ecx as f32) {
        ecx = (crate::simd_ops::cvt_f32_i32(xmm6_val.floor()) as i32).wrapping_add(10);
    }

    let mut ebx = rd::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize);
    let r11d = ecx.wrapping_sub(2);
    if 0.0f32 > (ebx as f32) {
        ebx = eax_y;
    }

    let v30_0 = rd::<f32>(param_2 + 0x30);
    let v30_1 = rd::<f32>(param_2 + 0x34);
    let v30_2 = rd::<f32>(param_2 + 0x38);

    let half_ecx = ecx / 2;
    let half_ebx = ebx / 2;

    let r8d = (crate::simd_ops::cvt_f32_i32((v30_0 * inv_170).round_ties_even()) as i32)
        .wrapping_sub(half_ecx)
        .wrapping_add(1);
    let r9d = (crate::simd_ops::cvt_f32_i32((v30_1 * inv_170).round_ties_even()) as i32)
        .wrapping_sub(half_ebx)
        .wrapping_add(1);
    let r10d = (crate::simd_ops::cvt_f32_i32((v30_2 * inv_170).round_ties_even()) as i32)
        .wrapping_sub(half_ecx)
        .wrapping_add(1);

    wr::<f32>(param_1, g_170);
    wr::<f32>(param_1 + 0x04, inv_170);
    wr::<i32>(param_1 + 0x08, ecx);
    wr::<i32>(param_1 + 0x0c, ebx);
    wr::<i32>(param_1 + 0x10, r8d);
    wr::<i32>(param_1 + 0x14, r9d);
    wr::<i32>(param_1 + 0x18, r10d);
    wr::<u64>(param_1 + 0x1c, 0);
    wr::<u32>(param_1 + 0x24, 0);
    wr::<i32>(param_1 + 0x28, r11d);
    wr::<i32>(param_1 + 0x2c, ebx.wrapping_sub(2));
    wr::<i32>(param_1 + 0x30, r11d);
    param_1
}

/// `unknown_14213a860`
/// Original: 0x14213a860 (780 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14213a860(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let g_e98 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3e98) as usize);
    let g_d38 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3d38) as usize);
    let g_170 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize);
    let inv_170 = one / g_170;

    let mut xmm5 = rd::<u128>(param_2 + 0x70);
    if g_e98.is_nan() || g_e98 != 0.0 || g_d38.is_nan() || g_d38 != 0.0 {
        xmm5 = crate::simd_ops::vinsertps(0, g_e98.to_bits() as u128, 0);
        xmm5 = crate::simd_ops::vinsertps(xmm5, g_d38.to_bits() as u128, 0x20);
    }

    let xmm1_f32 = if rd::<u8>(param_2 + 0xb0) == 0 {
        let dp = crate::simd_ops::vdpps(xmm5, xmm5, 0x5f);
        f32::from_bits(crate::simd_ops::vsqrtps(dp) as u32)
    } else {
        f32::from_bits(((xmm5 >> 32) as u32) & 0x7fff_ffff)
    };

    let mut ebx = rd::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize);
    let g_e78 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3e78) as usize);
    let xmm10 = if xmm1_f32 > g_e78 { xmm1_f32 } else { g_e78 };

    let p3_0 = rd::<f32>(param_3);
    let p3_1 = rd::<f32>(param_3 + 0x04);
    let p3_2 = rd::<f32>(param_3 + 0x08);
    let two_p3_0 = p3_0 + p3_0;
    let xmm1_sum = two_p3_0 + xmm10;
    let xmm4 = two_p3_0 + p3_1;
    let half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let xmm0_half = xmm4 * half;
    let xmm2_scaled = xmm1_sum * inv_170;

    let abs_y = f32::from_bits(((xmm5 >> 32) as u32) & 0x7fff_ffff);
    let xmm1_y = (abs_y + p3_2 + xmm0_half) * inv_170;
    let xmm8 = xmm2_scaled + xmm2_scaled;
    let xmm3_y = xmm1_y + xmm1_y;

    let eax_y = (crate::simd_ops::cvt_f32_i32(xmm3_y.floor()) as i32).wrapping_add(3);
    if 0.0f32 > (ebx as f32) {
        ebx = (crate::simd_ops::cvt_f32_i32(xmm8.floor()) as i32).wrapping_add(3);
    }

    let v30_0 = rd::<f32>(param_2 + 0x30);
    let v30_1 = rd::<f32>(param_2 + 0x34);
    let v30_2 = rd::<f32>(param_2 + 0x38);

    let mut r13d = rd::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize);
    if 0.0f32 > (r13d as f32) {
        r13d = eax_y;
    }

    let ecx_rad = (crate::simd_ops::cvt_f32_i32(((xmm10 + xmm10) * inv_170).floor()) as i32)
        .wrapping_add(4);
    let esi = r13d.wrapping_sub(2);
    let half_ebx = ebx / 2;

    let xmm6 = v30_2 * inv_170;
    let xmm3_mid = ((xmm4 + v30_1 + p3_2) - (g_170 * half) * (r13d as f32)) * inv_170;
    let xmm2_x = v30_0 * inv_170;

    let mut r14d = crate::simd_ops::cvt_f32_i32(xmm2_x.round_ties_even()) as i32;
    let mut r11d = half_ebx.wrapping_sub(ecx_rad);
    let mut edx = half_ebx.wrapping_add(ecx_rad);
    let mut r12d = crate::simd_ops::cvt_f32_i32(xmm6.round_ties_even()) as i32;
    let mut r15d = crate::simd_ops::cvt_f32_i32(xmm3_mid.round_ties_even()) as i32;

    let r13_m1 = r13d.wrapping_sub(1);
    let r8d = ebx.wrapping_sub(2);

    let ebp: i32;
    let r10d = if esi < 0 { esi } else { 0 };
    if r11d < 0 {
        r11d = 0;
        ebp = 0;
    } else {
        ebp = if r11d > r8d { r8d } else { r11d };
        if r11d > r8d {
            r11d = r8d;
        }
    }

    let edi = if edx < 0 {
        0
    } else if edx > r8d {
        r8d
    } else {
        edx
    };

    let ecx_out = if r13_m1 < 0 {
        0
    } else {
        ((if r13_m1 <= esi { 1i32 } else { 0i32 }).wrapping_sub(2)).wrapping_add(r13d)
    };

    if edx < 0 {
        edx = 0;
    } else if edx > r8d {
        edx = r8d;
    }

    r14d = r14d.wrapping_sub(half_ebx).wrapping_add(1);
    r12d = r12d.wrapping_sub(half_ebx).wrapping_add(1);
    let half_r13 = r13d / 2;
    r15d = r15d.wrapping_sub(half_r13).wrapping_add(1);

    wr::<f32>(param_1, g_170);
    wr::<f32>(param_1 + 0x04, inv_170);
    wr::<i32>(param_1 + 0x08, ebx);
    wr::<i32>(param_1 + 0x0c, r13d);
    wr::<i32>(param_1 + 0x10, r14d);
    wr::<i32>(param_1 + 0x14, r15d);
    wr::<i32>(param_1 + 0x18, r12d);
    wr::<i32>(param_1 + 0x1c, r11d);
    wr::<i32>(param_1 + 0x20, r10d);
    wr::<i32>(param_1 + 0x24, ebp);
    wr::<i32>(param_1 + 0x28, edx);
    wr::<i32>(param_1 + 0x2c, ecx_out);
    wr::<i32>(param_1 + 0x30, edi);
    param_1
}

/// `unknown_14213b1f0`
/// Original: 0x14213b1f0 (454 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14213b1f0(param_1: usize, param_2: usize) -> usize {
    wr::<u64>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    wr::<u64>(param_1 + 0x18, 0);

    let fill_u32 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6fd0) as usize);
    let d08 = rd::<i32>(param_2 + 0x08);
    let d0c = rd::<i32>(param_2 + 0x0c);
    let ebp = d08.wrapping_mul(d0c).wrapping_mul(d08);
    let rdi = (ebp as i64) as u64;
    let esi = rd::<u32>(param_1 + 0x08) as u64;
    let rcx_ptr = rd::<usize>(param_1);
    let eax_cap = rd::<u32>(param_1 + 0x0c) as u64;

    if rdi > eax_cap {
        let mut r14 = (eax_cap >> 1).wrapping_add(eax_cap);
        if r14 < rdi {
            r14 = rdi;
        }
        let new_ptr = exe::call_va!(0x1_4241_c0a0, (r14.wrapping_mul(12), 4u32) -> usize);
        let move_bytes = (rd::<u32>(param_1 + 0x08) as u64).wrapping_mul(12);
        let old_ptr = rd::<usize>(param_1);
        exe::call_va!(0x1_4183_acc8, (new_ptr, old_ptr, move_bytes));
        let old_ptr2 = rd::<usize>(param_1);
        if old_ptr2 != 0 {
            exe::call_va!(0x1_4241_c190, (old_ptr2));
        }
        wr::<usize>(param_1, new_ptr);
        wr::<u32>(param_1 + 0x0c, r14 as u32);
        let fill_base = new_ptr.wrapping_add((esi as usize).wrapping_mul(12));
        let count = rdi.wrapping_sub(esi) as usize;
        for i in 0..(count * 3) {
            wr::<u32>(fill_base.wrapping_add(i * 4), fill_u32);
        }
        wr::<i32>(param_1 + 0x08, ebp);
    } else if rdi > esi {
        let fill_base = rcx_ptr.wrapping_add((esi as usize).wrapping_mul(12));
        let count = rdi.wrapping_sub(esi) as usize;
        for i in 0..(count * 3) {
            wr::<u32>(fill_base.wrapping_add(i * 4), fill_u32);
        }
        wr::<i32>(param_1 + 0x08, ebp);
    } else if rdi != esi {
        wr::<i32>(param_1 + 0x08, ebp);
    }

    let d08_2 = rd::<i32>(param_2 + 0x08);
    let esi_2 = d08_2
        .wrapping_mul(d08_2)
        .wrapping_mul(rd::<i32>(param_2 + 0x0c));
    let rdi_2 = (esi_2 as i64) as u64;
    let ebp_2 = rd::<u32>(param_1 + 0x18) as u64;
    let rcx_2 = rd::<usize>(param_1 + 0x10);
    let eax_2 = rd::<u32>(param_1 + 0x1c) as u64;

    if rdi_2 > eax_2 {
        let mut r14 = (eax_2 >> 1).wrapping_add(eax_2);
        if r14 < rdi_2 {
            r14 = rdi_2;
        }
        let new_ptr = exe::call_va!(0x1_4241_c0a0, (r14, 1u32) -> usize);
        let move_bytes = rd::<u32>(param_1 + 0x18) as u64;
        let old_ptr = rd::<usize>(param_1 + 0x10);
        exe::call_va!(0x1_4183_acc8, (new_ptr, old_ptr, move_bytes));
        let old_ptr2 = rd::<usize>(param_1 + 0x10);
        if old_ptr2 != 0 {
            exe::call_va!(0x1_4241_c190, (old_ptr2));
        }
        wr::<usize>(param_1 + 0x10, new_ptr);
        wr::<u32>(param_1 + 0x18, ebp_2 as u32);
        wr::<u32>(param_1 + 0x1c, r14 as u32);
        let fill_ptr = new_ptr.wrapping_add(ebp_2 as usize);
        let fill_len = rdi_2.wrapping_sub(ebp_2);
        if fill_len != 0 {
            exe::call_va!(0x1_4183_acc2, (fill_ptr, 1u32, fill_len));
        }
        wr::<i32>(param_1 + 0x18, esi_2);
    } else if rdi_2 > ebp_2 {
        let fill_ptr = rcx_2.wrapping_add(ebp_2 as usize);
        let fill_len = rdi_2.wrapping_sub(ebp_2);
        if fill_len != 0 {
            exe::call_va!(0x1_4183_acc2, (fill_ptr, 1u32, fill_len));
        }
        wr::<i32>(param_1 + 0x18, esi_2);
    } else if rdi_2 != ebp_2 {
        wr::<i32>(param_1 + 0x18, esi_2);
    }

    param_1
}

/// `unknown_14213b7a0`
/// Original: 0x14213b7a0 (196 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14213b7a0() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_3eb8) as usize) != 0 {
        let v_ea4 = rd::<u32>(exe::ptr::<u32>(0x1_4386_3ea4) as usize);
        let v_d44 = rd::<u32>(exe::ptr::<u32>(0x1_4386_3d44) as usize);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3e98) as usize, v_ea4);
        let v_014 = rd::<u32>(exe::ptr::<u32>(0x1_4386_4014) as usize);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_4008) as usize, v_014);
        let v_bfc = rd::<u32>(exe::ptr::<u32>(0x1_4386_3bfc) as usize);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3d38) as usize, v_d44);
        let v_edc = rd::<u32>(exe::ptr::<u32>(0x1_4386_3edc) as usize);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3bf0) as usize, v_bfc);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3ed0) as usize, v_edc);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3eb8) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3eb4) as usize, 1);
    }

    let min_val = rd::<f32>(exe::ptr::<f32>(0x1_4342_651c) as usize);
    let cur_170 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize);
    let max_170 = if min_val > cur_170 { min_val } else { cur_170 };
    let mut dd8 = rd::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize);
    wr::<u8>(exe::ptr::<u8>(0x1_4386_416c) as usize, 1);
    wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, max_170);

    if dd8 >= 0 {
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3dd4) as usize, 1);
        if dd8 < 4 {
            dd8 = 4;
        }
        wr::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize, dd8);
    }

    let mut a8 = rd::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize);
    if a8 >= 0 {
        wr::<u8>(exe::ptr::<u8>(0x1_4386_41a4) as usize, 1);
        if a8 < 4 {
            a8 = 4;
        }
        wr::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize, a8);
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

pub unsafe fn diff_unknown_14212c860(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_e700(out: usize, _p2: usize, _bucket: usize) -> usize {
        wr::<i32>(out + 0x08, 2);
        wr::<i32>(out + 0x0c, 2);
        wr::<i32>(out + 0x10, 0);
        wr::<i32>(out + 0x14, 0);
        wr::<i32>(out + 0x18, 0);
        wr::<i32>(out + 0x1c, 0);
        wr::<i32>(out + 0x20, 0);
        wr::<i32>(out + 0x24, 0);
        wr::<i32>(out + 0x28, 1);
        wr::<i32>(out + 0x2c, 1);
        wr::<i32>(out + 0x30, 1);
        out
    }

    let _m1 = crate::difftest::Mock::new(0x1_4212_e700, mock_e700 as *const ())?;

    let run = |f: F, skip_first: bool| -> Vec<u32> {
        let mut ctrl = [-2i8; 32];
        ctrl[0] = if skip_first { -2 } else { 0 };
        ctrl[1] = 0;
        ctrl[2] = -1;

        let mut src_cells = [0u32; 24];
        for (i, v) in src_cells.iter_mut().enumerate() {
            *v = (i as u32 + 1) * 11;
        }

        let mut buckets = [0u8; 0x80];
        let bp = buckets.as_mut_ptr() as usize;
        let idx_active = if skip_first { 1usize } else { 0usize };
        wr::<usize>(bp + idx_active * 0x20 + 0x10, src_cells.as_ptr() as usize);
        wr::<u32>(bp + idx_active * 0x20 + 0x18, 8);

        let mut p1 = [0u8; 0x48];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x20, ctrl.as_ptr() as usize);
        wr::<usize>(p1p + 0x28, bp);
        wr::<usize>(p1p + 0x38, 2);

        let mut grid_info = [0i32; 8];
        let gip = grid_info.as_mut_ptr() as usize;
        wr::<i32>(gip + 0x08, 2);
        wr::<i32>(gip + 0x0c, 2);
        wr::<i32>(gip + 0x10, 0);
        wr::<i32>(gip + 0x14, 0);
        wr::<i32>(gip + 0x18, 0);

        let mut dst_cells = vec![0u32; 24];
        let mut p3 = [0u8; 0x38];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x08, dst_cells.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x28, gip);

        f(p1p, 0, p3p);
        dst_cells
    };

    for skip_first in [false, true] {
        let o = run(f_o, skip_first);
        let n = run(f_n, skip_first);
        crate::diff_eq!(o, n, "14212c860 skip_first={}", skip_first);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14212cc20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_3bd0, 4);
    let _g2 = GlobalSave::new(0x1_4386_4170, 4);
    let _g3 = GlobalSave::new(0x1_4386_3f50, 4);
    let _g4 = GlobalSave::new(0x1_4386_4098, 4);
    let _g5 = GlobalSave::new(0x1_4386_3e98, 4);
    let _g6 = GlobalSave::new(0x1_4386_3d38, 4);
    let _g7 = GlobalSave::new(0x1_4386_3dd8, 4);
    let _g8 = GlobalSave::new(0x1_4386_41a8, 4);

    static mut VEC_BUF_A: [u32; 12] = [0; 12];
    static mut VEC_BUF_B: [u32; 12] = [0; 12];
    static mut LOG_CC20: Vec<(u64, usize, [i32; 6])> = Vec::new();
    static mut CALL_IDX: u32 = 0;

    unsafe extern "win64" fn mock_d4e0(out_vec: usize, _p1: usize, box_ptr: usize) -> usize {
        let mut b = [0i32; 6];
        for (i, v) in b.iter_mut().enumerate() {
            *v = rd::<i32>(box_ptr + i * 4);
        }
        let c = CALL_IDX;
        CALL_IDX += 1;
        let ptr = if c == 0 {
            core::ptr::addr_of_mut!(VEC_BUF_A) as usize
        } else {
            core::ptr::addr_of_mut!(VEC_BUF_B) as usize
        };
        wr::<usize>(out_vec, ptr);
        wr::<u32>(out_vec + 0x08, 2);
        wr::<u32>(out_vec + 0x0c, 4);
        LOG_CC20.push((c as u64, ptr, b));
        out_vec
    }
    unsafe extern "win64" fn mock_d050(_p1: usize, _p2: usize, _p3: usize, span_ptr: usize) {
        let cnt = rd::<u64>(span_ptr);
        let ptr = rd::<usize>(span_ptr + 0x08);
        LOG_CC20.push((cnt, ptr, [0; 6]));
    }
    unsafe extern "win64" fn mock_d730(_p1: usize, box_ptr: usize) {
        let mut b = [0i32; 6];
        for (i, v) in b.iter_mut().enumerate() {
            *v = rd::<i32>(box_ptr + i * 4);
        }
        LOG_CC20.push((99, 0, b));
    }
    unsafe extern "win64" fn mock_e490(beg: usize, end: usize, cnt: u64, pt_ptr: usize) {
        let b = [
            rd::<i32>(pt_ptr),
            rd::<i32>(pt_ptr + 4),
            rd::<i32>(pt_ptr + 8),
            (end - beg) as i32,
            cnt as i32,
            0,
        ];
        LOG_CC20.push((cnt, beg, b));
    }
    unsafe extern "win64" fn mock_c190(ptr: usize) {
        LOG_CC20.push((77, ptr, [0; 6]));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4212_d4e0, mock_d4e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4212_d050, mock_d050 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4212_d730, mock_d730 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4123_e490, mock_e490 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F, max_budget: i32| -> (u64, u32, Vec<(u64, usize, [i32; 6])>) {
        LOG_CC20.clear();
        CALL_IDX = 0;

        wr::<i32>(exe::ptr::<i32>(0x1_4386_3bd0) as usize, 4);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, 0.5);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_3f50) as usize, 2);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_4098) as usize, max_budget);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3e98) as usize, 0.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3d38) as usize, 0.0);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize, 8);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize, 6);

        let mut p1 = [0u8; 0x30];
        let mut p2: [u32; 3] = [11, 22, 33];
        let mut p3 = [0u8; 0xc0];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<f32>(p3p + 0x30, 3.0);
        wr::<f32>(p3p + 0x34, -2.0);
        wr::<f32>(p3p + 0x38, 5.0);
        wr::<f32>(p3p + 0x70, 1.0);
        wr::<f32>(p3p + 0x74, 2.0);
        wr::<f32>(p3p + 0x78, 3.0);
        wr::<u8>(p3p + 0xb0, 0);

        let p1p = p1.as_mut_ptr() as usize;
        f(p1p, p2.as_mut_ptr() as usize, p3p);
        (rd::<u64>(p1p + 0x08), rd::<u32>(p1p + 0x10), LOG_CC20.clone())
    };

    for budget in [1i32, 5i32] {
        let o = run(f_o, budget);
        let n = run(f_n, budget);
        crate::diff_eq!(o, n, "14212cc20 budget={}", budget);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14212e240(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_4190, 1);
    let _g2 = GlobalSave::new(0x1_4386_3e60, 1);
    let _g3 = GlobalSave::new(0x1_4386_3ba0, 1);
    let _g4 = GlobalSave::new(0x1_4386_4170, 4);
    let _g5 = GlobalSave::new(0x1_4386_3ce0, 4);
    let _g6 = GlobalSave::new(0x1_4386_3bd0, 4);
    let _g7 = GlobalSave::new(0x1_4386_3f08, 1);
    let _g8 = GlobalSave::new(0x1_4386_3cc8, 1);

    static mut LOG_E240: Vec<u64> = Vec::new();

    unsafe extern "win64" fn mock_aca0(_rcx: u64, p2: usize) -> u64 {
        (p2 as u64).wrapping_mul(0x1234_5678)
    }
    unsafe extern "win64" fn mock_e830(ptr: usize) -> usize {
        LOG_E240.push(rd::<u64>(ptr));
        0x7777
    }
    unsafe extern "win64" fn mock_b500(_this: usize, _ptr: usize) {
        LOG_E240.push(0xb500);
    }
    unsafe extern "win64" fn mock_b880(tmp_ptr: usize) {
        LOG_E240.push(rd::<u64>(tmp_ptr + 0x08));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4213_aca0, mock_aca0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4020_b500, mock_b500 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4212_b880, mock_b880 as *const ())?;

    let run = |f: F, force_reset: u8| -> (u64, u64, Vec<u64>) {
        LOG_E240.clear();
        wr::<u8>(exe::ptr::<u8>(0x1_4386_4190) as usize, 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3e60) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3ba0) as usize, 1);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, -0.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3ce0) as usize, 1.25);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3bd0) as usize, 7);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3f08) as usize, force_reset);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3cc8) as usize, 1);

        let ctrl: [i8; 2] = [0, -1];
        let mut buckets = [0u64; 8];
        buckets[2] = 0xdead_beef;
        buckets[6] = 0xcafe_babe;

        let mut p1 = [0u8; 0x50];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x08, 99);
        wr::<u64>(p1p + 0x18, 0);
        wr::<usize>(p1p + 0x20, ctrl.as_ptr() as usize);
        wr::<usize>(p1p + 0x28, buckets.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x38, 2);

        f(p1p, 0x42);
        (rd::<u64>(p1p + 0x18), rd::<u64>(p1p + 0x38), LOG_E240.clone())
    };

    for force_reset in [0u8, 1u8] {
        let o = run(f_o, force_reset);
        let n = run(f_n, force_reset);
        crate::diff_eq!(o, n, "14212e240 force_reset={}", force_reset);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142139f10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_3ce0, 4);
    let _g2 = GlobalSave::new(0x1_4386_3b60, 4);
    let _g3 = GlobalSave::new(0x1_4386_4170, 4);

    unsafe extern "win64" fn mock_vfn30(_obj: usize) -> u32 {
        1
    }
    unsafe extern "win64" fn mock_world_bounds(
        bounds_out: usize,
        _geom: usize,
        _tf: usize,
        _p4: usize,
    ) {
        let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
        let inf = f32::from_bits(xmm3 as u32);
        wr::<f32>(bounds_out, 0.0 * inf);
        wr::<f32>(bounds_out + 0x04, 0.0);
        wr::<f32>(bounds_out + 0x08, 0.0);
        wr::<f32>(bounds_out + 0x0c, 1.0);
        wr::<f32>(bounds_out + 0x10, 1.0);
        wr::<f32>(bounds_out + 0x14, 1.0);
    }
    unsafe extern "win64" fn mock_point_dist(
        pt: usize,
        _geom: usize,
        _tf: usize,
        closest: usize,
    ) {
        wr::<f32>(closest, rd::<f32>(pt) + 0.25);
        wr::<f32>(closest + 0x04, rd::<f32>(pt + 0x04) - 0.25);
        wr::<f32>(closest + 0x08, rd::<f32>(pt + 0x08) + 0.125);
    }

    let _im1 = crate::difftest::IatMock::new(0x1_4264_5d08, mock_world_bounds as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5d40, mock_point_dist as *const ())?;

    let run = |f: F, flags: u8| -> (Vec<u32>, Vec<u8>) {
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3ce0) as usize, 0.1);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3b60) as usize, 2.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, 0.5);

        let vtbl: [usize; 8] = [0, 0, 0, 0, 0, 0, mock_vfn30 as *const () as usize, 0];
        let obj: usize = vtbl.as_ptr() as usize;

        let mut elem = [0u8; 0x50];
        let ep = elem.as_mut_ptr() as usize;
        wr::<usize>(ep, (&obj as *const usize) as usize);
        wr::<u8>(ep + 0x2c, flags);
        wr::<f32>(ep + 0x30, 0.0);
        wr::<f32>(ep + 0x34, 0.0);
        wr::<f32>(ep + 0x38, 0.0);
        wr::<f32>(ep + 0x3c, 1.0);
        wr::<f32>(ep + 0x40, 2.0);
        wr::<f32>(ep + 0x44, 2.0);
        wr::<f32>(ep + 0x48, 0.5);

        let mut p2 = [0u8; 0x80];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, ep);
        wr::<u32>(p2p + 0x08, 1);
        wr::<f64>(p2p + 0x60, 1.0);
        wr::<f64>(p2p + 0x68, 2.0);
        wr::<f64>(p2p + 0x70, 3.0);

        let mut grid_info = [0u8; 0x20];
        let gip = grid_info.as_mut_ptr() as usize;
        wr::<f32>(gip, 1.0);
        wr::<f32>(gip + 0x04, 1.0);
        wr::<i32>(gip + 0x08, 2);
        wr::<i32>(gip + 0x0c, 2);
        wr::<i32>(gip + 0x10, 0);
        wr::<i32>(gip + 0x14, 0);
        wr::<i32>(gip + 0x18, 0);

        let mut vec_cells = vec![10.0f32.to_bits(); 24];
        let mut flag_cells = vec![1u8; 8];

        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, vec_cells.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x18, flag_cells.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, gip);

        f(p1p, p2p);
        (vec_cells, flag_cells)
    };

    for flags in [0u8, 2u8, 4u8, 1u8] {
        let o = run(f_o, flags);
        let n = run(f_n, flags);
        crate::diff_eq!(o, n, "142139f10 flags={}", flags);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_14213a510(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, step: f32| -> (bool, Vec<u64>) {
        let mut out = vec![0u64; 11];
        let mut p2 = [0u8; 0x20];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<f32>(p2p, step);
        wr::<i32>(p2p + 0x08, 10);
        wr::<i32>(p2p + 0x0c, 8);
        wr::<i32>(p2p + 0x10, -3);
        wr::<i32>(p2p + 0x14, 4);
        wr::<i32>(p2p + 0x18, -5);

        let mut p3 = [0u8; 0x70];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<f32>(p3p + 0x50, 1.0);
        wr::<f32>(p3p + 0x54, 2.0);
        wr::<f32>(p3p + 0x58, 3.0);
        wr::<f32>(p3p + 0x5c, 4.0);
        wr::<f32>(p3p + 0x60, 5.5);
        wr::<f32>(p3p + 0x64, -6.25);
        wr::<f32>(p3p + 0x68, 7.75);

        let out_ptr = out.as_mut_ptr() as usize;
        let ret = f(out_ptr, p2p, p3p);
        (ret == out_ptr, out)
    };

    for step in [0.5f32, 1.25f32] {
        let o = run(f_o, step);
        let n = run(f_n, step);
        crate::diff_eq!(o, n, "14213a510 step={}", step);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14213a660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_3e98, 4);
    let _g2 = GlobalSave::new(0x1_4386_3d38, 4);
    let _g3 = GlobalSave::new(0x1_4386_4170, 4);
    let _g4 = GlobalSave::new(0x1_4386_3dd8, 4);
    let _g5 = GlobalSave::new(0x1_4386_41a8, 4);

    let run = |f: F, b0: u8, dd8: i32| -> (bool, Vec<u32>) {
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3e98) as usize, 0.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3d38) as usize, 0.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, 0.5);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize, dd8);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize, dd8);

        let mut p2 = [0u8; 0xc0];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<f32>(p2p + 0x30, 3.2);
        wr::<f32>(p2p + 0x34, -1.7);
        wr::<f32>(p2p + 0x38, 4.9);
        wr::<f32>(p2p + 0x70, 3.0);
        wr::<f32>(p2p + 0x74, -4.0);
        wr::<f32>(p2p + 0x78, 4.0);
        wr::<u8>(p2p + 0xb0, b0);

        let mut out = vec![0u32; 13];
        let out_ptr = out.as_mut_ptr() as usize;
        let ret = f(out_ptr, p2p);
        (ret == out_ptr, out)
    };

    for (b0, dd8) in [(0u8, -1i32), (1u8, -1i32), (0u8, 12i32)] {
        let o = run(f_o, b0, dd8);
        let n = run(f_n, b0, dd8);
        crate::diff_eq!(o, n, "14213a660 b0={} dd8={}", b0, dd8);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14213a860(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_3e98, 4);
    let _g2 = GlobalSave::new(0x1_4386_3d38, 4);
    let _g3 = GlobalSave::new(0x1_4386_4170, 4);
    let _g4 = GlobalSave::new(0x1_4386_3dd8, 4);
    let _g5 = GlobalSave::new(0x1_4386_3e78, 4);
    let _g6 = GlobalSave::new(0x1_4386_41a8, 4);

    let run = |f: F, b0: u8, dd8: i32| -> (bool, Vec<u32>) {
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3e98) as usize, 0.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3d38) as usize, 0.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, 0.5);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize, dd8);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3e78) as usize, 1.0);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize, dd8);

        let mut p2 = [0u8; 0xc0];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<f32>(p2p + 0x30, 2.5);
        wr::<f32>(p2p + 0x34, -3.25);
        wr::<f32>(p2p + 0x38, 1.75);
        wr::<f32>(p2p + 0x70, 3.0);
        wr::<f32>(p2p + 0x74, -2.0);
        wr::<f32>(p2p + 0x78, 4.0);
        wr::<u8>(p2p + 0xb0, b0);

        let p3: [f32; 3] = [0.5, 1.5, 0.25];
        let mut out = vec![0u32; 13];
        let out_ptr = out.as_mut_ptr() as usize;
        let ret = f(out_ptr, p2p, p3.as_ptr() as usize);
        (ret == out_ptr, out)
    };

    for (b0, dd8) in [(0u8, -1i32), (1u8, -1i32), (0u8, 24i32)] {
        let o = run(f_o, b0, dd8);
        let n = run(f_n, b0, dd8);
        crate::diff_eq!(o, n, "14213a860 b0={} dd8={}", b0, dd8);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14213b1f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_1: [u32; 64] = [0; 64];
    static mut ALLOC_2: [u8; 64] = [0; 64];
    static mut ALLOC_STEP: u32 = 0;

    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        let s = ALLOC_STEP;
        ALLOC_STEP += 1;
        if s == 0 {
            core::ptr::addr_of_mut!(ALLOC_1) as usize
        } else {
            core::ptr::addr_of_mut!(ALLOC_2) as usize
        }
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F, d08: i32, d0c: i32| -> (bool, [u64; 4], Vec<u32>, Vec<u8>) {
        ALLOC_1 = [0; 64];
        ALLOC_2 = [0; 64];
        ALLOC_STEP = 0;

        let mut p1 = [0u64; 4];
        let mut p2 = [0i32; 4];
        p2[2] = d08;
        p2[3] = d0c;

        let p1p = p1.as_mut_ptr() as usize;
        let ret = f(p1p, p2.as_ptr() as usize);
        let a1 = unsafe { ALLOC_1[..24].to_vec() };
        let a2 = unsafe { ALLOC_2[..8].to_vec() };
        (ret == p1p, p1, a1, a2)
    };

    for (d08, d0c) in [(2i32, 2i32), (0i32, 2i32)] {
        let o = run(f_o, d08, d0c);
        let n = run(f_n, d08, d0c);
        crate::diff_eq!(o, n, "14213b1f0 d08={} d0c={}", d08, d0c);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14213b7a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_3eb8, 1);
    let _g2 = GlobalSave::new(0x1_4386_3eb4, 1);
    let _g3 = GlobalSave::new(0x1_4386_3ea4, 4);
    let _g4 = GlobalSave::new(0x1_4386_3e98, 4);
    let _g5 = GlobalSave::new(0x1_4386_4014, 4);
    let _g6 = GlobalSave::new(0x1_4386_4008, 4);
    let _g7 = GlobalSave::new(0x1_4386_3d44, 4);
    let _g8 = GlobalSave::new(0x1_4386_3d38, 4);
    let _g9 = GlobalSave::new(0x1_4386_3bfc, 4);
    let _g10 = GlobalSave::new(0x1_4386_3bf0, 4);
    let _g11 = GlobalSave::new(0x1_4386_3edc, 4);
    let _g12 = GlobalSave::new(0x1_4386_3ed0, 4);
    let _g13 = GlobalSave::new(0x1_4386_416c, 1);
    let _g14 = GlobalSave::new(0x1_4386_4170, 4);
    let _g15 = GlobalSave::new(0x1_4386_3dd4, 1);
    let _g16 = GlobalSave::new(0x1_4386_3dd8, 4);
    let _g17 = GlobalSave::new(0x1_4386_41a4, 1);
    let _g18 = GlobalSave::new(0x1_4386_41a8, 4);

    let run = |f: F, eb8: u8, dd8: i32| -> (u8, u8, u32, u32, i32, i32) {
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3eb8) as usize, eb8);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3eb4) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3ea4) as usize, 0x3f80_0000);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3e98) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_4014) as usize, 0x4000_0000);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_4008) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3d44) as usize, 0x4040_0000);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3d38) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3bfc) as usize, 0x4080_0000);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3bf0) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3edc) as usize, 0x40a0_0000);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_3ed0) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_416c) as usize, 0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, 0.001);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3dd4) as usize, 0);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize, dd8);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_41a4) as usize, 0);
        wr::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize, dd8);

        f();
        (
            rd::<u8>(exe::ptr::<u8>(0x1_4386_3eb8) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4386_3eb4) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_4386_3e98) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_4386_4170) as usize),
            rd::<i32>(exe::ptr::<i32>(0x1_4386_3dd8) as usize),
            rd::<i32>(exe::ptr::<i32>(0x1_4386_41a8) as usize),
        )
    };

    for (eb8, dd8) in [(1u8, 2i32), (0u8, -1i32), (1u8, 8i32)] {
        let o = run(f_o, eb8, dd8);
        let n = run(f_n, eb8, dd8);
        crate::diff_eq!(o, n, "14213b7a0 eb8={} dd8={}", eb8, dd8);
    }
    Ok(3)
}
