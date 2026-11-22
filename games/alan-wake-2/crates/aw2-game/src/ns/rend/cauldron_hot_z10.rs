//! Decompiled hot functions shard Z10 (`0x142390670..0x142536cb0`).
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

/// `unknown_142390670`
/// Original: 0x142390670 (1630 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142390670(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) {
    let mut local_res10: u64 = param_2;
    let mut buf_60 = [0u64; 4];
    let buf_60_ptr = buf_60.as_mut_ptr() as usize;

    exe::call_va!(
        0x1_4020_9da0,
        (
            param_1,
            buf_60_ptr,
            (&mut local_res10 as *mut u64) as usize
        )
    );

    let r15_arr = rd::<usize>(param_1 + 0x30);
    let node_ptr = rd::<usize>(buf_60_ptr + 0x08);
    let rbx_idx = rd::<u64>(node_ptr + 0x08) as usize;
    let rsi_off = rbx_idx << 6;
    wr::<u64>(r15_arr + rsi_off, local_res10);

    let hash = exe::call_va!(0x1_424b_16a0, (param_3, 0x70u32) -> u64);
    if hash == rd::<u64>(r15_arr + rsi_off + 0x08) {
        return;
    }

    wr::<u8>(param_1 + 0x50, 1);
    wr::<u64>(r15_arr + rsi_off + 0x08, hash);

    let rdx_dst = r15_arr + rsi_off + 0x10;
    wr::<u64>(rdx_dst, rd::<u64>(param_3));
    wr::<u32>(rdx_dst + 0x08, rd::<u32>(param_3 + 0x08));
    wr::<u64>(rdx_dst + 0x0c, rd::<u64>(param_3 + 0x0c));
    wr::<u32>(rdx_dst + 0x14, rd::<u32>(param_3 + 0x14));
    wr::<u64>(rdx_dst + 0x18, rd::<u64>(param_3 + 0x18));
    wr::<u32>(rdx_dst + 0x20, rd::<u32>(param_3 + 0x20));
    wr::<u64>(rdx_dst + 0x24, rd::<u64>(param_3 + 0x24));
    wr::<u32>(rdx_dst + 0x2c, rd::<u32>(param_3 + 0x2c));

    let r15_bmin = param_3 + 0x30;
    wr::<u128>(buf_60_ptr, rd::<u128>(r15_bmin));
    wr::<u64>(buf_60_ptr + 0x10, rd::<u64>(r15_bmin + 0x10));
    exe::call_va!(0x1_424b_8e50, (buf_60_ptr, rdx_dst));

    let mut mat_20 = [0u8; 0x30];
    let mat_20_ptr = mat_20.as_mut_ptr() as usize;
    wr::<u64>(mat_20_ptr, rd::<u64>(rdx_dst));
    wr::<u32>(mat_20_ptr + 0x08, rd::<u32>(rdx_dst + 0x08));
    wr::<u64>(mat_20_ptr + 0x0c, rd::<u64>(rdx_dst + 0x0c));
    wr::<u32>(mat_20_ptr + 0x14, rd::<u32>(rdx_dst + 0x14));
    wr::<u64>(mat_20_ptr + 0x18, rd::<u64>(rdx_dst + 0x18));
    wr::<u32>(mat_20_ptr + 0x20, rd::<u32>(rdx_dst + 0x20));
    wr::<u64>(mat_20_ptr + 0x24, rd::<u64>(rdx_dst + 0x24));
    wr::<u32>(mat_20_ptr + 0x2c, rd::<u32>(rdx_dst + 0x2c));

    let rsi_out = rd::<usize>(param_1 + 0x40).wrapping_add(rbx_idx.wrapping_mul(0x90));
    exe::call_va!(0x1_4024_2c40, (mat_20_ptr, rdx_dst));

    let m00 = rd::<f32>(mat_20_ptr);
    let m01 = rd::<f32>(mat_20_ptr + 0x04);
    let m02 = rd::<f32>(mat_20_ptr + 0x08);
    let m10 = rd::<f32>(mat_20_ptr + 0x0c);
    let m11 = rd::<f32>(mat_20_ptr + 0x10);
    let m12 = rd::<f32>(mat_20_ptr + 0x14);
    let m20 = rd::<f32>(mat_20_ptr + 0x18);
    let m21 = rd::<f32>(mat_20_ptr + 0x1c);
    let m22 = rd::<f32>(mat_20_ptr + 0x20);
    let m30 = rd::<f32>(mat_20_ptr + 0x24);
    let m31 = rd::<f32>(mat_20_ptr + 0x28);
    let m32 = rd::<f32>(mat_20_ptr + 0x2c);

    let tx = f32::from_bits((((m31 * m10) + (m00 * m30)) + (m32 * m20)).to_bits() ^ 0x8000_0000);
    let ty = f32::from_bits((((m31 * m11) + (m30 * m01)) + (m32 * m21)).to_bits() ^ 0x8000_0000);
    let tz = f32::from_bits((((m02 * m30) + (m12 * m31)) + (m32 * m22)).to_bits() ^ 0x8000_0000);

    wr::<[f32; 4]>(rsi_out, [m00, m01, m02, 0.0]);
    wr::<[f32; 4]>(rsi_out + 0x10, [m10, m11, m12, 0.0]);
    wr::<[f32; 4]>(rsi_out + 0x20, [m20, m21, m22, 0.0]);
    wr::<[f32; 4]>(rsi_out + 0x30, [tx, ty, tz, 1.0]);

    let c_inv255 = rd::<f32>(exe::ptr::<f32>(0x1_4342_64b4) as usize);
    let b4e = rd::<u8>(param_3 + 0x4e) as i32 as f32;
    let b4d = rd::<u8>(param_3 + 0x4d) as i32 as f32;
    let b4c = rd::<u8>(param_3 + 0x4c) as i32 as f32;
    wr::<f32>(rsi_out + 0x40, b4e * c_inv255);
    wr::<f32>(rsi_out + 0x44, b4d * c_inv255);
    wr::<f32>(rsi_out + 0x48, b4c * c_inv255);

    let f48 = rd::<f32>(param_3 + 0x48);
    let sign_one = f32::from_bits((f48.to_bits() & 0x8000_0000) | 0x3f80_0000);
    let (_, pow_xmm) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad46,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [f48.to_bits() as u128, 2.0f32.to_bits() as u128, 0, 0],
    );
    let pow_val = f32::from_bits(pow_xmm as u32);
    let c_0_1 = rd::<f32>(exe::ptr::<f32>(0x1_4342_65d4) as usize);
    wr::<f32>(rsi_out + 0x4c, (sign_one * pow_val) * c_0_1);

    wr::<u64>(rsi_out + 0x50, rd::<u64>(r15_bmin));
    wr::<u32>(rsi_out + 0x58, rd::<u32>(r15_bmin + 0x08));
    wr::<u32>(rsi_out + 0x5c, rd::<u32>(param_3 + 0x50));
    wr::<u64>(rsi_out + 0x60, rd::<u64>(param_3 + 0x3c));
    wr::<u32>(rsi_out + 0x68, rd::<u32>(param_3 + 0x44));
    wr::<u32>(rsi_out + 0x6c, rd::<u32>(param_3 + 0x54));

    let vx = rd::<f32>(param_3 + 0x60);
    let vy = rd::<f32>(param_3 + 0x64);
    let vz = rd::<f32>(param_3 + 0x68);
    let len_sq = ((vy * vy) + (vx * vx)) + (vz * vz);
    let r = rsqrt_ss(len_sq);
    let nr = (0.5f32 * r) * (3.0f32 - ((len_sq * r) * r));
    let inv_len = if len_sq == 0.0f32 { 0.0f32 } else { nr };
    let nx = vx * inv_len;
    let ny = vy * inv_len;
    let nz = vz * inv_len;

    let dir_x = ((m00 * nx) + (m10 * ny)) + (nz * m20);
    let dir_y = ((m01 * nx) + (m11 * ny)) + (nz * m21);
    let dir_z = ((m02 * nx) + (m12 * ny)) + (nz * m22);
    wr::<f32>(rsi_out + 0x70, dir_x);
    wr::<f32>(rsi_out + 0x74, dir_y);
    wr::<f32>(rsi_out + 0x78, dir_z);
    wr::<u32>(rsi_out + 0x7c, rd::<u32>(param_3 + 0x58));

    let neg_dir = [
        f32::from_bits(dir_x.to_bits() ^ 0x8000_0000),
        f32::from_bits(dir_y.to_bits() ^ 0x8000_0000),
        f32::from_bits(dir_z.to_bits() ^ 0x8000_0000),
    ];

    let mut t_min = 0.0f32;
    let mut t_max = f32::INFINITY;

    let bmin_x = rd::<f32>(param_3 + 0x30);
    let bmin_y = rd::<f32>(param_3 + 0x34);
    let bmin_z = rd::<f32>(param_3 + 0x38);
    let bmax_x = rd::<f32>(param_3 + 0x3c);
    let bmax_y = rd::<f32>(param_3 + 0x40);
    let bmax_z = rd::<f32>(param_3 + 0x44);

    if !(bmin_x > bmax_x && bmin_y > bmax_y && bmin_z > bmax_z) {
        let eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_6460) as usize);
        let bmin = [bmin_x, bmin_y, bmin_z];
        let bmax = [bmax_x, bmax_y, bmax_z];
        for i in 0..3 {
            let d = neg_dir[i];
            let abs_d = f32::from_bits(d.to_bits() & !0x8000_0000);
            if !(eps > abs_d) {
                let inv_d = 1.0f32 / d;
                let t1 = (bmin[i] - 0.0f32) * inv_d;
                let t2 = (bmax[i] - 0.0f32) * inv_d;
                let lo = if t2 < t1 { t2 } else { t1 };
                t_min = if t_min > lo { t_min } else { lo };
                let hi = if t2 < t1 { t1 } else { t2 };
                t_max = if t_max < hi { t_max } else { hi };
                if !(t_min <= t_max) {
                    break;
                }
            } else {
                if !(bmin[i] <= 0.0f32) {
                    break;
                }
                if !(0.0f32 <= bmax[i]) {
                    break;
                }
            }
        }
    }

    let scale = rd::<f32>(param_3 + 0x5c) - t_max;
    wr::<f32>(rsi_out + 0x80, rd::<f32>(rsi_out + 0x70) * scale);
    wr::<f32>(rsi_out + 0x84, scale * rd::<f32>(rsi_out + 0x74));
    wr::<f32>(rsi_out + 0x88, scale * rd::<f32>(rsi_out + 0x78));

    let rcx_e8 = rd::<usize>(param_1 + 0xe8);
    let off_20 = rbx_idx << 5;
    wr::<u64>(rcx_e8 + off_20, rd::<u64>(buf_60_ptr));
    wr::<u32>(rcx_e8 + off_20 + 0x08, rd::<u32>(buf_60_ptr + 0x08));
    wr::<u64>(rcx_e8 + off_20 + 0x10, rd::<u64>(buf_60_ptr + 0x0c));
    wr::<u32>(rcx_e8 + off_20 + 0x18, rd::<u32>(buf_60_ptr + 0x14));
}

/// `unknown_14246e970`
/// Original: 0x14246e970 (653 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14246e970(
    param_1: usize,
    param_2: u64,
    param_3: u64,
    param_4: usize,
    param_5: u64,
    param_6: u32,
) -> u8 {
    let ok = exe::call_va!(
        0x1_420e_ce90,
        (param_1, param_5, param_2, param_3, param_6) -> u32
    );
    if ok == 0 {
        let top = rd::<usize>(param_1 + 0x08);
        let p10 = rd::<usize>(param_1 + 0x10);
        let lim = if p10 < top {
            p10
        } else {
            exe::ptr::<u8>(0x1_4298_68d0) as usize
        };
        let stk_base = rd::<usize>(param_1 + 0x30);
        let slot = top.wrapping_sub(0x18);
        let mut local_30: [u64; 2] = [slot as u64, 0];
        let fn_cd0 = exe::ptr::<u8>(0x1_4211_6cd0) as usize;
        let status = exe::call_va!(
            0x1_4210_dbb0,
            (
                param_1,
                fn_cd0,
                local_30.as_mut_ptr() as usize,
                slot.wrapping_sub(stk_base) as u64,
                lim.wrapping_sub(stk_base) as u64
            ) -> u32
        );
        if status == 0 {
            return 1;
        }
    }

    let mut rdx_slot = rd::<usize>(param_1 + 0x08).wrapping_sub(0x18);
    let mut converted_ok = true;
    if rd::<u32>(rdx_slot + 0x10) != 5 {
        let b1 = rd::<u8>(param_1 + 0x01);
        if (b1 & 4) != 0 {
            let g = rd::<usize>(param_1 + 0x18);
            wr::<u8>(param_1 + 0x01, b1 & !4u8);
            wr::<u64>(param_1 + 0x68, rd::<u64>(g + 0x30));
            wr::<usize>(g + 0x30, param_1);
        }
        let conv = exe::call_va!(0x1_420e_cc90, (param_1) -> u32);
        if conv == 0 {
            converted_ok = false;
        } else {
            let g = rd::<usize>(param_1 + 0x18);
            if rd::<u64>(g + 0x48) >= rd::<u64>(g + 0x40) {
                exe::call_va!(0x1_4210_c770, (param_1));
            }
            rdx_slot = rd::<usize>(param_1 + 0x08).wrapping_sub(0x18);
        }
    }

    if converted_ok {
        let str_ptr = rd::<usize>(rdx_slot).wrapping_add(0x18);
        if str_ptr != 0 {
            let mut len: usize = 0;
            while rd::<u8>(str_ptr + len) != 0 {
                len += 1;
            }
            let mut view: [u64; 2] = [str_ptr as u64, len as u64];
            exe::call_va!(
                0x1_4240_9b50,
                (param_4, view.as_mut_ptr() as usize, 0u64, len as u64)
            );
            wr::<usize>(param_1 + 0x08, rd::<usize>(param_1 + 0x08).wrapping_sub(0x18));
            return 0;
        }
    }

    let b3 = rd::<u8>(param_4 + 0x03) as u32;
    let mut data_ptr = param_4 + 0x08;
    wr::<u32>(param_4 + 0x04, 0);
    let cap_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8) - 0x7c00
    };
    let len_bits = (rd::<u32>(param_4) & 0x00ff_ffff) << 3;
    if len_bits > cap_bits {
        data_ptr = rd::<usize>(data_ptr);
    }
    wr::<u8>(data_ptr, 0);
    wr::<usize>(param_1 + 0x08, rd::<usize>(param_1 + 0x08).wrapping_sub(0x18));
    0
}

/// `unknown_142489da0`
/// Original: 0x142489da0 (144 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142489da0(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> i32 {
    let mut base: u64 = 0;
    let mut count: u64 = param_2;
    while count != 0 {
        let half = count >> 1;
        let next_base = base.wrapping_sub(half).wrapping_add(count);
        let mid = half.wrapping_add(base) as usize;
        count = half;
        if param_3 >= rd::<u64>(param_1 + mid * 8) {
            base = next_base;
        }
    }
    let cand = base.wrapping_sub(1);
    let idx: i32 = if cand < param_2 { cand as i32 } else { 0 };
    if param_2 == 0 {
        return -1;
    }
    let elem = rd::<u64>(param_1.wrapping_add((idx as isize as usize).wrapping_mul(8)));
    if param_3 == elem { idx } else { -1 }
}

#[inline(always)]
fn city_mur_k(v: u32) -> u32 {
    v.wrapping_mul(0xcc9e_2d51)
        .rotate_right(17)
        .wrapping_mul(0x1b87_3593)
}

#[inline(always)]
fn city_mix(h: u32) -> u32 {
    h.rotate_right(19)
        .wrapping_add(0xfadd_af14)
        .wrapping_mul(5)
}

#[inline(always)]
fn city_fmix(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    h
}

/// `cityhash_1424b1ec0`
/// Original: 0x1424b1ec0 (1077 bytes, Throws::No)
pub unsafe extern "win64" fn cityhash_1424b1ec0(param_1: usize, param_2: u64) -> u32 {
    let len32 = param_2 as u32;
    let n = param_2 as usize;

    if param_2 > 24 {
        let r8d_init = len32.wrapping_mul(0xcc9e_2d51);
        let w_m4 = rd::<u32>(param_1 + n - 4);
        let w_m16 = rd::<u32>(param_1 + n - 16);
        let w_m8 = rd::<u32>(param_1 + n - 8);
        let w_m12 = rd::<u32>(param_1 + n - 12);
        let w_m20 = rd::<u32>(param_1 + n - 20);

        let mut esi = city_mix(city_mix(city_mur_k(w_m4) ^ len32) ^ city_mur_k(w_m16));
        let mut ebp = city_mix(city_mix(city_mur_k(w_m8) ^ r8d_init) ^ city_mur_k(w_m12));
        let mut ebx = city_mix(city_mur_k(w_m20).wrapping_add(r8d_init));

        let mut iters = (param_2 - 1) / 20;
        let mut p = param_1;
        while iters != 0 {
            let a0 = rd::<u32>(p);
            let a1 = rd::<u32>(p + 0x04);
            let a2 = rd::<u32>(p + 0x08);
            let a3 = rd::<u32>(p + 0x0c);
            let a4 = rd::<u32>(p + 0x10);

            let r9d = city_mur_k(a0);
            let ecx_1 = a1
                .wrapping_add(ebx)
                .rotate_right(19)
                .wrapping_mul(0x3361_d2af);
            let edx = r9d.wrapping_sub(ecx_1);
            let r9_xor = (r9d ^ esi).rotate_right(18);
            esi = edx;
            let ecx_2 = city_mur_k(a3);
            let eax_2 = r9_xor.wrapping_add(0xfadd_af14).wrapping_mul(5);
            let mut r8d = a1.wrapping_add(ecx_2) ^ eax_2;
            r8d = r8d
                .rotate_right(19)
                .wrapping_add(0xfadd_af14)
                .wrapping_add(a4)
                .wrapping_mul(5)
                .swap_bytes();
            let eax_3 = city_mur_k(a2).wrapping_add(ebp);
            ebp = r8d;
            let eax_4 = (eax_3
                .rotate_right(18)
                .wrapping_add(0xfadd_af14)
                .wrapping_mul(5)
                ^ a4)
                .swap_bytes();
            ebx = eax_4.wrapping_mul(5);

            p = p.wrapping_add(0x14);
            iters -= 1;
        }

        let ebx_r = ebx.rotate_right(11);
        let r8d_r = ebp.rotate_right(11);
        let ecx_fin = r8d_r
            .wrapping_mul(0xcc9e_2d51)
            .rotate_right(17)
            .wrapping_mul(0x3361_d2af);
        let eax_fin = ebx_r.wrapping_mul(0xcc9e_2d51).rotate_right(17);
        let edx_fin = esi.wrapping_sub(ecx_fin).rotate_right(19);
        let ecx_fin2 = edx_fin
            .wrapping_add(0xfadd_af14)
            .wrapping_mul(5)
            .rotate_right(17)
            .wrapping_add(eax_fin);
        return ecx_fin2
            .wrapping_mul(0xcc9e_2d51)
            .rotate_right(19)
            .wrapping_mul(5)
            .wrapping_add(0xe654_6b64)
            .rotate_right(17)
            .wrapping_mul(0xcc9e_2d51);
    }

    if param_2 > 12 {
        let half = (param_2 >> 1) as usize;
        let a = rd::<u32>(param_1 + half - 4);
        let b = rd::<u32>(param_1 + 4);
        let c = rd::<u32>(param_1 + n - 8);
        let d = rd::<u32>(param_1 + half);
        let e = rd::<u32>(param_1);
        let f = rd::<u32>(param_1 + n - 4);

        let mut h = len32;
        h = city_mix(city_mur_k(a) ^ h);
        h = city_mix(h ^ city_mur_k(b));
        h = city_mix(h ^ city_mur_k(c));
        h = city_mix(h ^ city_mur_k(d));
        h = city_mix(h ^ city_mur_k(e));
        h = city_mix(h ^ city_mur_k(f));
        return city_fmix(h);
    }

    if param_2 > 4 {
        let w0 = rd::<u32>(param_1);
        let r8d = len32.wrapping_mul(5);
        let ecx = city_mur_k(w0.wrapping_add(len32)) ^ r8d;
        let edx = city_mix(ecx);
        let w_last = rd::<u32>(param_1 + n - 4).wrapping_add(r8d);
        let mid_off = ((param_2 >> 1) as usize) & 4;
        let w_mid = rd::<u32>(param_1 + mid_off);
        let r8_2 = city_mix(city_mur_k(w_last) ^ edx);
        let eax_mid = 0x318f_97d9u32
            .wrapping_sub(w_mid.wrapping_mul(0x3361_d2af))
            .rotate_right(17)
            .wrapping_mul(0x1b87_3593);
        return city_fmix(city_mix(eax_mid ^ r8_2));
    }

    let mut a: u32 = 0;
    let mut c: u32 = 9;
    for i in 0..n {
        let b = rd::<u8>(param_1 + i) as i8 as i32 as u32;
        a = b.wrapping_sub(a.wrapping_mul(0x3361_d2af));
        c ^= a;
    }
    let c_mixed = city_mix(c ^ city_mur_k(len32));
    city_fmix(city_mix(city_mur_k(a) ^ c_mixed))
}

/// `unknown_142533300`
/// Original: 0x142533300 (118 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142533300(
    param_1: usize,
    param_2: u8,
    param_3: usize,
) {
    if param_2 == 0 {
        exe::call_va!(0x1_4253_3380, (param_1, param_3));
        return;
    }
    wr::<u64>(param_3, rd::<u64>(param_1 + 0x20));
    let new_cnt = rd::<u32>(param_1 + 0x18).wrapping_sub(1);
    wr::<u32>(param_1 + 0x18, new_cnt);
    wr::<usize>(param_1 + 0x20, param_3);
    if new_cnt == 0 {
        exe::call_va!(0x1_4253_71a0, (param_1));
        return;
    }
    let b0e = rd::<u8>(param_1 + 0x0e);
    if (b0e & 1) != 0 {
        let r9_heap = rd::<usize>(param_1 + 0x30);
        wr::<u8>(param_1 + 0x0e, b0e & !1u8);
        let bsize = rd::<u32>(param_1 + 0x1c);
        let bin = exe::call_va!(0x1_4253_6cb0, (bsize) -> u8) as usize;
        wr::<u8>(param_1 + 0x0e, rd::<u8>(param_1 + 0x0e) | 1u8);
        let rcx_q = r9_heap.wrapping_add(0x410).wrapping_add(bin * 0x18);
        let rdx_q = r9_heap.wrapping_add(0xb00);
        exe::call_va!(0x1_4253_68c0, (rcx_q, rdx_q, param_1));
    }
}

/// `unknown_142534f80`
/// Original: 0x142534f80 (107 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142534f80(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> usize {
    if param_3 == 0
        || (param_3 & param_3.wrapping_sub(1)) != 0
        || param_2 > 0x7fff_ffff_ffff_ffff
    {
        return 0;
    }
    if param_2 <= 0x400 && param_3 <= param_2 {
        let idx = (param_2.wrapping_add(7) >> 3) as usize;
        let page = rd::<usize>(param_1 + 0x08 + idx * 8);
        let block = rd::<usize>(page + 0x10);
        if block != 0 && ((block as u64) & param_3.wrapping_sub(1)) == 0 {
            wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
            wr::<u64>(page + 0x10, rd::<u64>(block));
            return block;
        }
    }
    exe::call_va!(0x1_4253_5070, (param_1, param_2, param_3) -> usize)
}

/// `unknown_1425368c0`
/// Original: 0x1425368c0 (216 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425368c0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let heap = rd::<usize>(param_3 + 0x30);
    let prev = rd::<usize>(param_3 + 0x40);
    if prev != 0 {
        wr::<usize>(prev + 0x38, rd::<usize>(param_3 + 0x38));
    }
    let next = rd::<usize>(param_3 + 0x38);
    if next != 0 {
        wr::<usize>(next + 0x40, rd::<usize>(param_3 + 0x40));
    }
    if param_3 == rd::<usize>(param_2 + 0x08) {
        wr::<usize>(param_2 + 0x08, rd::<usize>(param_3 + 0x40));
    }
    if param_3 == rd::<usize>(param_2) {
        wr::<usize>(param_2, rd::<usize>(param_3 + 0x38));
        exe::call_va!(0x1_4253_6aa0, (heap, param_2));
    }
    let dst_tail = rd::<usize>(param_1 + 0x08);
    wr::<usize>(param_3 + 0x40, dst_tail);
    wr::<usize>(param_3 + 0x38, 0);
    let dst_tail2 = rd::<usize>(param_1 + 0x08);
    if dst_tail2 != 0 {
        wr::<usize>(dst_tail2 + 0x38, param_3);
        wr::<usize>(param_1 + 0x08, param_3);
    } else {
        wr::<usize>(param_1, param_3);
        wr::<usize>(param_1 + 0x08, param_3);
        exe::call_va!(0x1_4253_6aa0, (heap, param_1));
    }
    let is_full_q = u8::from(rd::<u64>(param_1 + 0x10) == 0x20010);
    let b0e = (rd::<u8>(param_3 + 0x0e) & !1u8) | is_full_q;
    wr::<u8>(param_3 + 0x0e, b0e);
}

/// `unknown_142536cb0`
/// Original: 0x142536cb0 (94 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142536cb0(param_1: u64) -> u8 {
    let wsize = param_1.wrapping_add(7) >> 3;
    if wsize <= 1 {
        return 1;
    }
    if wsize <= 8 {
        return ((wsize as u8).wrapping_add(1)) & !1u8;
    }
    if wsize > 0x4000 {
        return 0x49;
    }
    let w = wsize - 1;
    let b: u8 = if w == 0 {
        0x40
    } else {
        (63 - w.leading_zeros()) as u8
    };
    let shift = (b as u32).wrapping_sub(2) & 0x3f;
    let top2 = ((w >> shift) as u8) & 3;
    (b << 2).wrapping_add(top2).wrapping_sub(3)
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142390670(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODE_MEM: [u64; 2] = [0, 1];

    unsafe extern "win64" fn mock_9da0(_p1: usize, out_60: usize, io_key: usize) {
        wr::<u64>(out_60, 0x1111);
        wr::<usize>(out_60 + 0x08, core::ptr::addr_of_mut!(NODE_MEM) as usize);
        wr::<u64>(io_key, rd::<u64>(io_key).wrapping_add(10));
    }
    unsafe extern "win64" fn mock_16a0(p3: usize, len: u32) -> u64 {
        (rd::<u32>(p3) as u64).wrapping_add(len as u64)
    }
    unsafe extern "win64" fn mock_8e50(b60: usize, _rdx_dst: usize) {
        wr::<f32>(b60, rd::<f32>(b60) + 1.0);
        wr::<f32>(b60 + 0x0c, rd::<f32>(b60 + 0x0c) + 2.0);
    }
    unsafe extern "win64" fn mock_2c40(m20: usize, _rdx_dst: usize) {
        let a = rd::<f32>(m20 + 0x04);
        let b = rd::<f32>(m20 + 0x0c);
        wr::<f32>(m20 + 0x04, b);
        wr::<f32>(m20 + 0x0c, a);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_9da0, mock_9da0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424b_8e50, mock_8e50 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4024_2c40, mock_2c40 as *const ())?;

    let run = |f: F, same_hash: bool| -> (u8, Vec<u8>, Vec<u32>, Vec<u8>) {
        let mut p1 = vec![0u8; 0x100];
        let mut arr_30 = vec![0u8; 0x80];
        let mut arr_40 = vec![0u32; 0x120 / 4];
        let mut arr_e8 = vec![0u8; 0x40];
        let mut p3 = vec![0u8; 0x70];

        let p1p = p1.as_mut_ptr() as usize;
        let a30p = arr_30.as_mut_ptr() as usize;
        let a40p = arr_40.as_mut_ptr() as usize;
        let ae8p = arr_e8.as_mut_ptr() as usize;
        let p3p = p3.as_mut_ptr() as usize;

        wr::<usize>(p1p + 0x30, a30p);
        wr::<usize>(p1p + 0x40, a40p);
        wr::<usize>(p1p + 0xe8, ae8p);

        // 3x4 transform at p3 + 0x00..0x30
        wr::<[f32; 12]>(
            p3p,
            [
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0, -3.0, 4.0,
            ],
        );
        // AABB min/max at p3 + 0x30..0x48
        wr::<[f32; 6]>(p3p + 0x30, [-5.0, -4.0, -3.0, 5.0, 4.0, 3.0]);
        wr::<f32>(p3p + 0x48, -3.0);
        wr::<u8>(p3p + 0x4c, 50);
        wr::<u8>(p3p + 0x4d, 100);
        wr::<u8>(p3p + 0x4e, 200);
        wr::<u32>(p3p + 0x50, 0x1234);
        wr::<u32>(p3p + 0x54, 0x5678);
        wr::<u32>(p3p + 0x58, 0x9abc);
        wr::<f32>(p3p + 0x5c, 25.0);
        wr::<[f32; 3]>(p3p + 0x60, [3.0, 4.0, 12.0]);

        if same_hash {
            let expected_h = (rd::<u32>(p3p) as u64).wrapping_add(0x70);
            wr::<u64>(a30p + 0x40 + 0x08, expected_h);
        }

        f(p1p, 0x500, p3p);
        (rd::<u8>(p1p + 0x50), arr_30, arr_40, arr_e8)
    };

    for same_hash in [false, true] {
        let o = run(f_o, same_hash);
        let n = run(f_n, same_hash);
        crate::diff_eq!(o, n, "142390670 same_hash={}", same_hash);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14246e970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, usize, u64, u32) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MODE_CE90: u32 = 1;
    static mut MODE_DBB0: u32 = 1;
    static mut MODE_CC90: u32 = 1;
    static mut GC_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_ce90(
        _l: usize,
        _p5: u64,
        _p2: u64,
        _p3: u64,
        _p6: u32,
    ) -> u32 {
        MODE_CE90
    }
    unsafe extern "win64" fn mock_dbb0(
        _l: usize,
        _fn_ptr: usize,
        _s: usize,
        _a4: u64,
        _a5: u64,
    ) -> u32 {
        MODE_DBB0
    }
    unsafe extern "win64" fn mock_cc90(_l: usize) -> u32 {
        MODE_CC90
    }
    unsafe extern "win64" fn mock_c770(_l: usize) {
        GC_CALLED += 1;
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, view: usize, _pos: u64, count: u64) {
        let src = rd::<usize>(view);
        wr::<u32>(dst + 0x04, count as u32);
        for i in 0..(count as usize) {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(src + i));
        }
        wr::<u8>(dst + 0x08 + count as usize, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_420e_ce90, mock_ce90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4210_dbb0, mock_dbb0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420e_cc90, mock_cc90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4210_c770, mock_c770 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let run = |f: F, mode: u8| -> (u8, usize, u32, u8, u32) {
        GC_CALLED = 0;
        let mut l_state = [0u8; 0x80];
        let mut g_state = [0u8; 0x60];
        let mut stk = [0u8; 0x60];
        let mut tstr = [0u8; 0x30];
        let mut out_str = [0u8; 0x30];

        let lp = l_state.as_mut_ptr() as usize;
        let gp = g_state.as_mut_ptr() as usize;
        let sp = stk.as_mut_ptr() as usize;
        let tp = tstr.as_mut_ptr() as usize;
        let op = out_str.as_mut_ptr() as usize;

        for (i, &b) in b"hello\0".iter().enumerate() {
            wr::<u8>(tp + 0x18 + i, b);
        }

        wr::<u32>(op, 0x0400_0004);
        wr::<u32>(op + 0x04, 3);
        wr::<u8>(op + 0x08, b'x');

        wr::<u8>(lp + 0x01, 4);
        wr::<usize>(lp + 0x08, sp + 0x30);
        wr::<usize>(lp + 0x10, sp + 0x48);
        wr::<usize>(lp + 0x18, gp);
        wr::<usize>(lp + 0x30, sp);
        wr::<u64>(gp + 0x40, 100);
        wr::<u64>(gp + 0x48, 200);

        let slot = sp + 0x18;
        wr::<usize>(slot, tp);
        wr::<u32>(slot + 0x10, if mode == 0 { 5 } else { 3 });

        MODE_CE90 = u32::from(mode != 2);
        MODE_DBB0 = u32::from(mode != 2);
        MODE_CC90 = u32::from(mode != 1);

        let ret = f(lp, 1, 2, op, 3, 4);
        (
            ret,
            rd::<usize>(lp + 0x08).wrapping_sub(sp),
            rd::<u32>(op + 0x04),
            rd::<u8>(op + 0x08),
            GC_CALLED,
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14246e970 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142489da0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> i32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let arr: [u64; 6] = [10, 20, 20, 40, 80, 160];
    let ap = arr.as_ptr() as usize;

    for (len, key) in [(0u64, 10u64), (6, 5), (6, 10), (6, 20), (6, 50), (6, 160), (6, 200)] {
        let o = f_o(ap, len, key);
        let n = f_n(ap, len, key);
        crate::diff_eq!(o, n, "142489da0 len={} key={}", len, key);
    }
    Ok(7)
}

pub unsafe fn diff_cityhash_1424b1ec0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut data = [0u8; 80];
    for (i, b) in data.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(0x85);
    }
    let dp = data.as_ptr() as usize;

    for len in [0u64, 1, 3, 4, 5, 8, 12, 13, 18, 24, 25, 44, 64] {
        let o = f_o(dp, len);
        let n = f_n(dp, len);
        crate::diff_eq!(o, n, "1424b1ec0 len={}", len);
    }
    Ok(13)
}

pub unsafe fn diff_unknown_142533300(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_TAG: u64 = 0;

    unsafe extern "win64" fn mock_3380(p1: usize, p3: usize) {
        LOG_TAG = rd::<u32>(p1 + 0x1c) as u64 ^ rd::<u64>(p3);
    }
    unsafe extern "win64" fn mock_71a0(_p1: usize) {
        LOG_TAG = 0x71a0;
    }
    unsafe extern "win64" fn mock_6cb0(bsize: u32) -> u8 {
        (bsize >> 3) as u8
    }
    unsafe extern "win64" fn mock_68c0(rcx_q: usize, rdx_q: usize, p1: usize) {
        LOG_TAG = rdx_q
            .wrapping_sub(rcx_q)
            .wrapping_add(rd::<u8>(p1 + 0x0e) as usize) as u64;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3380, mock_3380 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_6cb0, mock_6cb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_68c0, mock_68c0 as *const ())?;

    let run = |f: F, mode: u8| -> (u64, u32, bool, u8, u64) {
        LOG_TAG = 0;
        let mut page = [0u8; 0x40];
        let mut block = [0u64; 2];
        let pp = page.as_mut_ptr() as usize;
        let bp = block.as_mut_ptr() as usize;

        wr::<u8>(pp + 0x0e, u8::from(mode == 2));
        wr::<u32>(pp + 0x18, if mode == 1 { 1 } else { 3 });
        wr::<u32>(pp + 0x1c, 0x20);
        wr::<u64>(pp + 0x20, 0xdead_beef);
        wr::<usize>(pp + 0x30, 0x10_0000);

        f(pp, u8::from(mode != 0), bp);
        (
            rd::<u64>(bp),
            rd::<u32>(pp + 0x18),
            rd::<usize>(pp + 0x20) == bp,
            rd::<u8>(pp + 0x0e),
            LOG_TAG,
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "142533300 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142534f80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_5070(_p1: usize, sz: u64, align: u64) -> usize {
        (sz ^ align) as usize + 0x5070
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;

    let run = |f: F, sz: u64, align: u64| -> (usize, u32, u64) {
        let mut heap = [0usize; 0x90];
        let mut page = [0u8; 0x30];
        let mut block = [0u64; 8];
        let hp = heap.as_mut_ptr() as usize;
        let pp = page.as_mut_ptr() as usize;
        let bp = ((block.as_mut_ptr() as usize) + 0x1f) & !0x1fusize;

        for slot in heap.iter_mut() {
            *slot = pp;
        }
        wr::<u64>(bp, 0x7777);
        wr::<usize>(pp + 0x10, bp);
        wr::<u32>(pp + 0x18, 4);

        let ret = f(hp, sz, align);
        (ret, rd::<u32>(pp + 0x18), rd::<u64>(pp + 0x10))
    };

    for (sz, align) in [(32u64, 0u64), (32, 3), (32, 16), (32, 64), (0x800, 16)] {
        let o = run(f_o, sz, align);
        let n = run(f_n, sz, align);
        crate::diff_eq!(o, n, "142534f80 sz={} align={}", sz, align);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1425368c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_6AA0: Vec<(usize, usize)> = Vec::new();

    unsafe extern "win64" fn mock_6aa0(heap: usize, q: usize) {
        LOG_6AA0.push((heap, q));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_6aa0, mock_6aa0 as *const ())?;

    static mut Q_DST: [usize; 3] = [0; 3];
    static mut Q_SRC: [usize; 3] = [0; 3];
    static mut PAGE_BUF: [u8; 0x50] = [0; 0x50];
    static mut OTHER_BUF: [u8; 0x50] = [0; 0x50];

    let run = |f: F, empty_dst: bool, full_q: bool| -> ([usize; 2], [usize; 2], u8, Vec<(usize, usize)>) {
        LOG_6AA0.clear();
        Q_DST = [0usize; 3];
        Q_SRC = [0usize; 3];
        PAGE_BUF = [0u8; 0x50];
        OTHER_BUF = [0u8; 0x50];

        let qd = core::ptr::addr_of_mut!(Q_DST) as usize;
        let qs = core::ptr::addr_of_mut!(Q_SRC) as usize;
        let pp = core::ptr::addr_of_mut!(PAGE_BUF) as usize;
        let op = core::ptr::addr_of_mut!(OTHER_BUF) as usize;

        wr::<u64>(qd + 0x10, if full_q { 0x20010 } else { 0x40 });
        if !empty_dst {
            wr::<usize>(qd, op);
            wr::<usize>(qd + 0x08, op);
        }

        wr::<usize>(qs, pp);
        wr::<usize>(qs + 0x08, pp);
        wr::<u8>(pp + 0x0e, 4);
        wr::<usize>(pp + 0x30, 0xabcd_0000);

        f(qd, qs, pp);
        (
            [rd::<usize>(qd), rd::<usize>(qd + 0x08)],
            [rd::<usize>(qs), rd::<usize>(qs + 0x08)],
            rd::<u8>(pp + 0x0e),
            LOG_6AA0.clone(),
        )
    };

    for (empty_dst, full_q) in [(true, false), (false, true)] {
        let o = run(f_o, empty_dst, full_q);
        let n = run(f_n, empty_dst, full_q);
        crate::diff_eq!(o, n, "1425368c0 empty_dst={} full_q={}", empty_dst, full_q);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142536cb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    for sz in [
        0u64, 1, 8, 9, 16, 24, 64, 65, 72, 128, 256, 511, 512, 513, 4096, 0x20000, 0x20008,
    ] {
        let o = f_o(sz);
        let n = f_n(sz);
        crate::diff_eq!(o, n, "142536cb0 sz={:#x}", sz);
    }
    Ok(17)
}
