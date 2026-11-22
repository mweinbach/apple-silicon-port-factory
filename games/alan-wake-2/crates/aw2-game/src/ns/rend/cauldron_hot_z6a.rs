//! Decompiled `rend` hot functions shard Z6A (`0x1421346d0..0x14217ab60`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vminss(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

#[inline(always)]
fn vmaxss(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

#[inline(always)]
fn vminps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        vminss(a[0], b[0]),
        vminss(a[1], b[1]),
        vminss(a[2], b[2]),
        vminss(a[3], b[3]),
    ]
}

#[inline(always)]
fn vmaxps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        vmaxss(a[0], b[0]),
        vmaxss(a[1], b[1]),
        vmaxss(a[2], b[2]),
        vmaxss(a[3], b[3]),
    ]
}

#[inline(always)]
fn vaddps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

#[inline(always)]
fn vsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

#[inline(always)]
fn vmulps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

#[inline(always)]
fn perm_c9(v: [f32; 4]) -> [f32; 4] {
    [v[1], v[2], v[0], v[3]]
}

#[inline(always)]
fn perm_d2(v: [f32; 4]) -> [f32; 4] {
    [v[2], v[0], v[1], v[3]]
}

#[inline(always)]
fn perm_ff(v: [f32; 4]) -> [f32; 4] {
    [v[3], v[3], v[3], v[3]]
}

#[inline(always)]
fn quat_rotate_with_parts(
    q_d2: [f32; 4],
    q_c9: [f32; 4],
    q_ff: [f32; 4],
    two_xyz: [f32; 4],
    v: [f32; 4],
) -> [f32; 4] {
    let v_c9 = perm_c9(v);
    let m1 = vmulps(q_d2, v_c9);
    let v_d2 = perm_d2(v);
    let m2 = vmulps(q_c9, v_d2);
    let cross1 = vsubps(m2, m1);
    let t = vmulps(cross1, two_xyz);
    let t_c9 = perm_c9(t);
    let m3 = vmulps(q_d2, t_c9);
    let t_d2 = perm_d2(t);
    let m4 = vmulps(q_c9, t_d2);
    let wt = vmulps(t, q_ff);
    let sum1 = vaddps(wt, v);
    let cross2 = vsubps(m4, m3);
    vaddps(sum1, cross2)
}

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

#[inline(always)]
fn nr_rsqrt(x: f32, half: f32, three: f32) -> f32 {
    let r = rsqrt_ss(x);
    let x_r = x * r;
    let x_r2 = x_r * r;
    let sub = three - x_r2;
    let half_r = half * r;
    let res = half_r * sub;
    if x == 0.0f32 { 0.0f32 } else { res }
}

/// `unknown_1421346d0`
/// Original: 0x1421346d0 (3836 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421346d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) -> usize {
    let grid_base = rd::<usize>(param_2 + 0x08);
    let meta = rd::<usize>(param_2 + 0x28);

    let stride_z = rd::<i32>(meta + 0x08);
    let stride_x = rd::<i32>(meta + 0x0c);
    let cell_size = rd::<f32>(meta);

    let c_neg2 = rd::<f32>(exe::ptr::<f32>(0x1_4342_70a0) as usize);
    let clamp_lo = cell_size * c_neg2;
    let clamp_hi = cell_size + cell_size;

    let ix = rd::<i32>(param_4);
    let iy = rd::<i32>(param_4 + 0x04);
    let iz = rd::<i32>(param_4 + 0x08);

    let read_corner = |idx: i32| -> (f32, f32, f32) {
        let addr = grid_base.wrapping_add((idx as isize as usize).wrapping_mul(12));
        (rd::<f32>(addr), rd::<f32>(addr + 0x04), rd::<f32>(addr + 0x08))
    };

    let idx0 = iz
        .wrapping_mul(stride_z)
        .wrapping_add(ix)
        .wrapping_mul(stride_x)
        .wrapping_add(iy);
    let c0 = read_corner(idx0);

    let idx1 = ix
        .wrapping_add(1)
        .wrapping_add(iz.wrapping_mul(stride_z))
        .wrapping_mul(stride_x)
        .wrapping_add(iy);
    let c1 = read_corner(idx1);

    let idx2 = ix
        .wrapping_add(1)
        .wrapping_add(iz.wrapping_add(1).wrapping_mul(stride_z))
        .wrapping_mul(stride_x)
        .wrapping_add(iy);
    let c2 = read_corner(idx2);

    let idx3 = iz
        .wrapping_add(1)
        .wrapping_mul(stride_z)
        .wrapping_add(ix)
        .wrapping_mul(stride_x)
        .wrapping_add(iy);
    let c3 = read_corner(idx3);

    let idx4 = iy.wrapping_add(1).wrapping_add(
        iz.wrapping_mul(stride_z)
            .wrapping_add(ix)
            .wrapping_mul(stride_x),
    );
    let c4 = read_corner(idx4);

    let idx5 = iy.wrapping_add(1).wrapping_add(
        iz.wrapping_mul(stride_z)
            .wrapping_add(ix.wrapping_add(1))
            .wrapping_mul(stride_x),
    );
    let c5 = read_corner(idx5);

    let idx6 = iy.wrapping_add(1).wrapping_add(
        ix.wrapping_add(1)
            .wrapping_add(iz.wrapping_add(1).wrapping_mul(stride_z))
            .wrapping_mul(stride_x),
    );
    let c6 = read_corner(idx6);

    let idx7 = iy.wrapping_add(1).wrapping_add(
        iz.wrapping_add(1)
            .wrapping_mul(stride_z)
            .wrapping_add(ix)
            .wrapping_mul(stride_x),
    );
    let c7 = read_corner(idx7);

    let c_100 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ea4) as usize);
    let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);

    let eval_corner = |(cx, cy, cz): (f32, f32, f32)| -> f32 {
        let raw = if 0.0f32 > cy {
            f32::from_bits(cy.to_bits() ^ sign_mask)
        } else {
            let p3_0 = rd::<f32>(param_3);
            let p3_1 = rd::<f32>(param_3 + 0x04);
            let thresh = ((p3_0 + p3_0) + p3_1) + cell_size;
            if cy > thresh {
                cy - c_100
            } else {
                let x2 = cx * cx;
                let z2 = cz * cz;
                let p3_2 = rd::<f32>(param_3 + 0x08);
                let sum_sq = z2 + x2;
                let dist = sum_sq.sqrt();
                let extra = if p3_2 < cy { cell_size } else { 0.0f32 };
                let min_y = vminss(p3_0, cy);
                dist - (min_y + extra)
            }
        };
        vminss(vmaxss(raw, clamp_lo), clamp_hi)
    };

    let d0 = eval_corner(c0);
    let d1 = eval_corner(c1);
    let d2 = eval_corner(c2);
    let d3 = eval_corner(c3);
    let d4 = eval_corner(c4);
    let d5 = eval_corner(c5);
    let d6 = eval_corner(c6);
    let d7 = eval_corner(c7);

    let inv_cell = rd::<f32>(meta + 0x04);
    let d0_sub_d3 = d0 - d3;
    let coeff_xyz = ((((((d6 - d5) - d0) + d3) + d4) - d7) + d1) - d2;
    let coeff_xz = (d0_sub_d3 - d1) + d2;
    let coeff_yz = (d0_sub_d3 - d4) + d7;
    let coeff_x = d1 - d0;
    let coeff_xy = ((d5 + d0) - d4) - d1;
    let coeff_y = d4 - d0;

    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let c_eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_647c) as usize);
    let u_min = inv_cell * c_eps;
    let u_max = c_one - u_min;

    let off_x = rd::<i32>(meta + 0x10) as f32;
    let cell_x = ix as f32;
    let cell_y = iy as f32;
    let off_z = rd::<i32>(meta + 0x18) as f32;
    let off_y = rd::<i32>(meta + 0x14) as f32;
    let cell_z = iz as f32;

    let p5_x = rd::<f32>(param_5);
    let p5_y = rd::<f32>(param_5 + 0x04);
    let p5_z = rd::<f32>(param_5 + 0x08);

    let mut u_z = vminss(vmaxss(((inv_cell * p5_z) - off_z) - cell_z, u_min), u_max);
    let mut u_y = vminss(vmaxss(((p5_y * inv_cell) - off_y) - cell_y, u_min), u_max);
    let mut u_x = vminss(vmaxss(((p5_x * inv_cell) - off_x) - cell_x, u_min), u_max);

    let sign_bit_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
    let one_bits = c_one.to_bits();
    let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);

    for _ in 0..25 {
        let t0 = u_z * coeff_xyz + coeff_xy;
        let t1 = t0 * u_y;
        let term_x = (u_z * coeff_xz + t1) + coeff_x;
        let term_y = u_z * coeff_yz + coeff_y;
        let dz_base = d3 - d0;
        let f_val = ((term_x * u_x + term_y * u_y) + u_z * dz_base) + d0;
        if f_val > 0.0f32 {
            break;
        }
        let step_bits = ((f_val.to_bits() & sign_bit_mask) | one_bits) ^ sign_mask;
        let step_s = f32::from_bits(step_bits) * c_half;

        let gy = ((u_x * t0 + coeff_yz) * u_z) + coeff_y;
        let gz = (((u_y * coeff_xyz + coeff_xz) * u_x + coeff_yz) * u_y) + dz_base;
        let gx = ((coeff_xz + t1) * u_z) + coeff_x;

        u_x = vminss(vmaxss(gx * step_s + u_x, u_min), u_max);
        u_y = vminss(vmaxss(gy * step_s + u_y, u_min), u_max);
        u_z = vminss(vmaxss(gz * step_s + u_z, u_min), u_max);
    }

    let out_x = (u_x + cell_x) + off_x;
    let out_y = (u_y + cell_y) + off_y;
    let out_z = (u_z + cell_z) + off_z;

    wr::<f32>(param_1, out_x);
    wr::<f32>(param_1 + 0x04, out_y);
    let scale = rd::<f32>(meta);
    wr::<f32>(param_1 + 0x08, out_z);
    wr::<f32>(param_1, scale * rd::<f32>(param_1));
    wr::<f32>(param_1 + 0x04, scale * rd::<f32>(param_1 + 0x04));
    wr::<f32>(param_1 + 0x08, scale * rd::<f32>(param_1 + 0x08));
    param_1
}

/// `unknown_14217ab60`
/// Original: 0x14217ab60 (4322 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14217ab60(
    param_1: usize,
    _param_2: u64,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6_bits: u32,
    param_7_bits: u64,
    param_8_bits: u32,
    param_9_in: u64,
    param_10_in: u64,
    param_11: usize,
) {
    let param_6 = f32::from_bits(param_6_bits);
    let param_7 = f64::from_bits(param_7_bits);
    let param_8 = f32::from_bits(param_8_bits);
    let param_9 = param_9_in as u8;
    let param_10 = param_10_in as u8;
    let q_in = rd::<[u32; 4]>(param_5);
    let conj_mask = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7b00) as usize);
    let q_conj = [
        f32::from_bits(q_in[0] ^ conj_mask[0]),
        f32::from_bits(q_in[1] ^ conj_mask[1]),
        f32::from_bits(q_in[2] ^ conj_mask[2]),
        f32::from_bits(q_in[3] ^ conj_mask[3]),
    ];
    let mut trans_13 = rd::<[f32; 4]>(param_5 + 0x10);

    let p_e0 = rd::<usize>(param_1 + 0xe0);
    let v_p4 = rd::<[f32; 4]>(param_4);
    wr::<[f32; 4]>(p_e0 + 0x10, v_p4);

    let q_conj_ff = perm_ff(q_conj);
    let two_xyz = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_7ab0) as usize);
    let q_conj_d2 = perm_d2(q_conj);
    let q_conj_c9 = perm_c9(q_conj);

    let mut vel_rot = quat_rotate_with_parts(q_conj_d2, q_conj_c9, q_conj_ff, two_xyz, v_p4);
    let saved_a0 = rd::<[f32; 4]>(param_1 + 0xa0);

    let p0 = rd::<usize>(param_1);
    let (deg, val_28) = if rd::<u8>(exe::ptr::<u8>(0x1_4386_4eb0) as usize) == 0 {
        (rd::<f32>(p0 + 0x2c), rd::<u32>(p0 + 0x28))
    } else {
        (
            rd::<f32>(exe::ptr::<f32>(0x1_4386_5170) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_4386_50d8) as usize),
        )
    };

    let deg_to_rad = rd::<f32>(exe::ptr::<f32>(0x1_4342_6514) as usize);
    let rad = deg * deg_to_rad;
    let (_, cos_xmm) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad82,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [rad.to_bits() as u128, 0, 0, 0],
    );
    let cos_val = f32::from_bits(cos_xmm as u32);
    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let cos_clamped = vminss(vmaxss(cos_val, 0.0f32), c_one);

    wr::<f32>(rd::<usize>(param_1 + 0x78) + 0xb4, cos_clamped);
    if param_9 == 0 && rd::<u8>(param_1 + 0x17) != 0 {
        wr::<u32>(rd::<usize>(param_1 + 0x78) + 0xb4, 0x3f80_0000);
    }
    wr::<u32>(rd::<usize>(param_1 + 0x78) + 0x48, val_28);

    let p0_b32 = rd::<u8>(rd::<usize>(param_1) + 0x32);
    let b0_flag = if p0_b32 == 0 || rd::<u8>(param_1 + 0x18) != 0 {
        1u8
    } else {
        0u8
    };
    wr::<u8>(rd::<usize>(param_1 + 0x78) + 0xb0, b0_flag);
    wr::<u8>(
        rd::<usize>(param_1 + 0x78) + 0xb1,
        rd::<u8>(param_1 + 0x17),
    );
    wr::<[u64; 2]>(
        rd::<usize>(param_1 + 0x78) + 0x80,
        rd::<[u64; 2]>(param_1 + 0x20),
    );

    if rd::<u8>(rd::<usize>(param_1) + 0x32) != 0 {
        let eps_47c = rd::<f32>(exe::ptr::<f32>(0x1_4342_647c) as usize);
        if eps_47c >= vel_rot[1] {
            let (_, sin_xmm) = crate::simd_ops::call_va8_xmm(
                0x1_4183_ad88,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [rad.to_bits() as u128, 0, 0, 0],
            );
            let sin_val = f32::from_bits(sin_xmm as u32);
            let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
            let neg_sin = f32::from_bits(sin_val.to_bits() ^ sign_mask);
            let c_4_5 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d30) as usize);
            let dy = ((neg_sin * c_4_5) * param_8) * param_6;
            vel_rot[1] += dy;
        }
    }

    let mut filter_buf = [0u64; 9];
    let filter_obj = filter_buf.as_mut_ptr() as usize;
    wr::<usize>(filter_obj, exe::ptr::<u8>(0x1_433e_6fd0) as usize);
    wr::<usize>(filter_obj + 0x40, 0);
    exe::call_va!(0x1_4128_cba0, (filter_obj + 0x08, param_3));

    let mut filter_data_buf = [0u64; 2];
    let filter_data = filter_data_buf.as_mut_ptr() as usize;
    let p1_10 = rd::<u32>(param_1 + 0x10);
    let b30 = rd::<u8>(rd::<usize>(param_1) + 0x30);
    let mask = if b30 != 0 { !6u32 } else { !7u32 };
    wr::<u32>(filter_data, mask);
    wr::<u32>(filter_data + 0x04, p1_10);
    wr::<u64>(filter_data + 0x08, 0);

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_4e48) as usize) != 0 {
        let p68 = rd::<usize>(param_1 + 0x68);
        wr::<u64>(p68 + 0x30, 0x7fef_ffff_ffff_ffff);
        wr::<u64>(p68 + 0x38, 0x7fef_ffff_ffff_ffff);
        wr::<u64>(p68 + 0x40, 0x7fef_ffff_ffff_ffff);
        wr::<u64>(p68 + 0x48, 0xffef_ffff_ffff_ffff);
        wr::<u64>(p68 + 0x50, 0xffef_ffff_ffff_ffff);
        wr::<u64>(p68 + 0x58, 0xffef_ffff_ffff_ffff);
        wr::<u32>(p68 + 0x08, 0);
        wr::<u32>(p68 + 0x18, 0);
        wr::<u32>(p68 + 0x28, 0);
    }

    let p0_2 = rd::<usize>(param_1);
    let f24 = rd::<f32>(p0_2 + 0x24);
    let f20 = rd::<f32>(p0_2 + 0x20);
    let min_494 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
    let f44 = vmaxss(f20 - (f24 + f24), min_494);

    let a0_rel = vsubps(rd::<[f32; 4]>(param_1 + 0xa0), trans_13);
    let pos_rot = vaddps(
        quat_rotate_with_parts(q_conj_d2, q_conj_c9, q_conj_ff, two_xyz, a0_rel),
        trans_13,
    );

    wr::<[f32; 4]>(rd::<usize>(param_1 + 0x78) + 0x30, pos_rot);
    wr::<[u64; 4]>(
        rd::<usize>(param_1 + 0x78) + 0x50,
        rd::<[u64; 4]>(param_5),
    );
    wr::<f32>(rd::<usize>(param_1 + 0x78) + 0x40, f24);
    wr::<f32>(rd::<usize>(param_1 + 0x78) + 0x44, f44);
    let vel_scaled = [
        param_8 * vel_rot[0],
        param_8 * vel_rot[1],
        param_8 * vel_rot[2],
        param_8 * vel_rot[3],
    ];
    wr::<[f32; 4]>(rd::<usize>(param_1 + 0x78) + 0x70, vel_scaled);

    let c0_u64 = rd::<u32>(param_1 + 0xc0) as u64;
    let b8_u64 = rd::<u64>(param_1 + 0xb8);
    let p78_a = rd::<usize>(param_1 + 0x78);
    wr::<u64>(p78_a + 0x90, c0_u64);
    wr::<u64>(p78_a + 0x98, b8_u64);

    let p78_b = rd::<usize>(param_1 + 0x78);
    wr::<usize>(p78_b + 0x08, filter_data);
    wr::<usize>(p78_b + 0x10, filter_obj);
    wr::<u16>(p78_b + 0x18, 7);
    wr::<u64>(p78_b + 0x20, 0);

    let rbx = rd::<usize>(param_1 + 0x78);
    let mut ctx_f0_buf: [u64; 4] = [
        rbx as u64,
        rd::<u64>(param_1 + 0x80),
        (param_1 + 0x90) as u64,
        rd::<u64>(param_1 + 0x88),
    ];
    let ctx_f0 = ctx_f0_buf.as_mut_ptr() as usize;
    let rsi = rd::<usize>(param_1 + 0x68);

    let mut buf_1a0_arr = [0u64; 14];
    let mut buf_250_arr = [0u64; 14];
    let buf_1a0 = buf_1a0_arr.as_mut_ptr() as usize;
    let buf_250 = buf_250_arr.as_mut_ptr() as usize;

    let res_ptr = if param_9 != 0 {
        exe::call_va!(0x1_4213_b7a0, ());

        let mut arg_40_buf: [u64; 2] = [c0_u64, b8_u64];
        let arg_40 = arg_40_buf.as_mut_ptr() as usize;

        let mut f_d18 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3d18) as usize);
        if 0.0f32 > f_d18 {
            f_d18 = rd::<f32>(rbx + 0x44);
        }
        let f_b60 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3b60) as usize);
        let f_170 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize);
        let mut f0 = vmaxss(0.0f32, rd::<f32>(rbx + 0x40) - f_b60 * f_170);
        let f_e10 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3e10) as usize);
        if !(0.0f32 > f_e10) {
            f0 = f_e10;
        }
        let mut f2 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4078) as usize);
        if 0.0f32 > f2 {
            f2 = rd::<f32>(rbx + 0x48);
        }
        wr::<f32>(arg_40, f0);
        wr::<f32>(arg_40 + 0x04, f_d18);
        wr::<f32>(arg_40 + 0x08, f2);

        let mut buf_2c0 = [0u64; 42];
        let ret_a860 = exe::call_va!(
            0x1_4213_a860,
            (buf_2c0.as_mut_ptr() as usize, rbx, arg_40) -> usize
        );
        let mut buf_210 = [0u8; 0x38];
        wr::<[u8; 0x34]>(buf_210.as_mut_ptr() as usize, rd::<[u8; 0x34]>(ret_a860));

        exe::call_va!(
            0x1_4213_a510,
            (buf_1a0, buf_210.as_mut_ptr() as usize, rbx)
        );
        wr::<[u64; 2]>(arg_40, rd::<[u64; 2]>(rbx + 0x08));
        let flag_df8 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3df8) as usize);
        exe::call_va!(
            0x1_4216_f470,
            (
                rsi,
                buf_1a0,
                buf_1a0 + 0x30,
                rd::<u64>(rbx),
                arg_40,
                rbx + 0xf8,
                flag_df8,
                0u8
            )
        );
        exe::call_va!(
            0x1_4212_c3e0,
            (buf_250, rd::<u64>(param_1 + 0x70), ctx_f0) -> usize
        )
    } else {
        exe::call_va!(0x1_4217_33f0, (buf_1a0, rsi, rbx) -> usize)
    };

    let mut vec_00 = rd::<[f32; 4]>(res_ptr);
    let vec_10 = rd::<[f32; 4]>(res_ptr + 0x10);
    let vec_20 = rd::<[f32; 4]>(res_ptr + 0x20);
    let vec_30 = rd::<[f32; 4]>(res_ptr + 0x30);
    let ptr_40 = rd::<usize>(res_ptr + 0x40);
    let cnt_48 = rd::<u32>(res_ptr + 0x48);
    wr::<u64>(res_ptr + 0x40, 0);
    wr::<u64>(res_ptr + 0x48, 0);
    let ptr_50 = rd::<usize>(res_ptr + 0x50);
    let cnt_58 = rd::<u32>(res_ptr + 0x58);
    wr::<u64>(res_ptr + 0x50, 0);
    wr::<u64>(res_ptr + 0x58, 0);
    let res_b60 = rd::<u8>(res_ptr + 0x60);
    let res_f64 = rd::<u32>(res_ptr + 0x64);

    let min_bound = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_8910) as usize);
    let max_bound = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_8620) as usize);

    if param_9 != 0 {
        let q_orig = rd::<[f32; 4]>(param_5);
        let q_d2 = perm_d2(q_orig);
        let q_c9 = perm_c9(q_orig);
        let q_ff = perm_ff(q_orig);

        let rot1 = vaddps(
            quat_rotate_with_parts(q_d2, q_c9, q_ff, two_xyz, vsubps(vec_10, trans_13)),
            trans_13,
        );
        let c1 = vminps(vmaxps(rot1, min_bound), max_bound);

        let rot2 = vaddps(
            quat_rotate_with_parts(q_d2, q_c9, q_ff, two_xyz, vsubps(vec_20, trans_13)),
            trans_13,
        );
        let c2 = vminps(vmaxps(rot2, min_bound), max_bound);

        let diff_vec = vsubps(c2, c1);
        let inv_dt = if param_6 == 0.0f32 && !param_6.is_nan() {
            0.0f32
        } else {
            c_one / param_6
        };
        wr::<[f32; 4]>(
            param_1 + 0x50,
            [
                diff_vec[0] * inv_dt,
                diff_vec[1] * inv_dt,
                diff_vec[2] * inv_dt,
                diff_vec[3] * inv_dt,
            ],
        );

        if param_10 != 0 {
            vec_00 = vec_10;
        }

        if rd::<u8>(exe::ptr::<u8>(0x1_4386_5068) as usize) != 0 {
            let v3 = [diff_vec[0], 0.0f32, diff_vec[2], diff_vec[3]];
            let p4_v = rd::<[f32; 4]>(param_4);
            let v5 = [p4_v[0], 0.0f32, p4_v[2], p4_v[3]];
            let eps_f30 = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_7f30) as usize);

            let dp3 = (v3[0] * v3[0] + v3[1] * v3[1]) + (v3[2] * v3[2] + 0.0f32);
            let sqrt3 = dp3.sqrt();
            let mut n3 = [0.0f32; 4];
            for i in 0..4 {
                if eps_f30[i] <= dp3 {
                    n3[i] = v3[i] / sqrt3;
                }
            }

            let dp5 = (v5[0] * v5[0] + v5[1] * v5[1]) + (v5[2] * v5[2] + 0.0f32);
            let sqrt5 = dp5.sqrt();
            let mut n5 = [0.0f32; 4];
            for i in 0..4 {
                if eps_f30[i] <= dp5 {
                    n5[i] = v5[i] / sqrt5;
                }
            }

            let dot = (n3[0] * n5[0] + n3[1] * n5[1]) + (n3[2] * n5[2] + 0.0f32);
            let f_f48 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4f48) as usize);
            wr::<f32>(param_1 + 0x60, vmaxss(f_f48, dot));

            let rad_e90 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4e90) as usize) * deg_to_rad;
            let (_, cos_e90_xmm) = crate::simd_ops::call_va8_xmm(
                0x1_4183_ad82,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [rad_e90.to_bits() as u128, 0, 0, 0],
            );
            let cos_e90 = f32::from_bits(cos_e90_xmm as u32);
            let cond = cos_e90 > dot && (dot != 0.0f32 || dot.is_nan());
            if rd::<u8>(exe::ptr::<u8>(0x1_4386_50c0) as usize) != 0 && cond {
                wr::<u8>(param_1 + 0x64, 1);
            }
        }

        if rd::<u8>(exe::ptr::<u8>(0x1_4386_4e30) as usize) != 0 {
            let y_pos = pos_rot[1];
            let vy = (vec_30[1] - y_pos) / param_6;
            let abs_mask = !rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
            let abs_vy = f32::from_bits(vy.to_bits() & abs_mask);
            let c_10 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6dd4) as usize);
            if c_10 > abs_vy {
                let dp_xz = (vel_rot[0] * vel_rot[0] + 0.0f32) + (vel_rot[2] * vel_rot[2] + 0.0f32);
                let spd = dp_xz.sqrt() / param_6;
                let f7 = if !(spd > rd::<f32>(exe::ptr::<f32>(0x1_4386_51e0) as usize))
                    && !(vy > rd::<f32>(exe::ptr::<f32>(0x1_4386_4ec8) as usize))
                    && !(rd::<f32>(exe::ptr::<f32>(0x1_4386_50f8) as usize) > vy)
                {
                    rd::<f32>(exe::ptr::<f32>(0x1_4386_5190) as usize)
                } else {
                    rd::<f32>(exe::ptr::<f32>(0x1_4386_5200) as usize)
                };
                let val_0a0 = rd::<f32>(exe::ptr::<f32>(0x1_4386_50a0) as usize);
                let sqrt_0a0 = if 0.0f32 > val_0a0 {
                    let (_, sq_xmm) = crate::simd_ops::call_va8_xmm(
                        0x1_4183_add0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        [val_0a0.to_bits() as u128, 0, 0, 0],
                    );
                    f32::from_bits(sq_xmm as u32)
                } else {
                    val_0a0.sqrt()
                };
                let mut pair_458: [f32; 2] = [val_0a0, (f7 + f7) * sqrt_0a0];
                let mut pair_468: [f32; 2] = [y_pos, vy];
                let f_ee8 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4ee8) as usize);
                let _ = crate::simd_ops::call_va8_xmm(
                    0x1_4058_ad10,
                    pair_468.as_mut_ptr() as u64,
                    pair_458.as_mut_ptr() as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, 0, vec_00[1].to_bits() as u128, f_ee8.to_bits() as u128],
                );
                vec_00[1] = pair_468[0];
            }
        }

        if param_10 == 0 && rd::<u8>(exe::ptr::<u8>(0x1_4386_4e78) as usize) != 0 {
            let p78 = rd::<usize>(param_1 + 0x78);
            let base_pos = vaddps(rd::<[f32; 4]>(p78 + 0x80), rd::<[f32; 4]>(p78 + 0x30));
            let dx = vec_00[0] - base_pos[0];
            let dy = vec_00[1] - base_pos[1];
            let dz = vec_00[2] - base_pos[2];

            let dx2 = dx * dx;
            let dz2 = dz * dz;
            let len_xz_sq = (dx2 + 0.0f32) + dz2;
            let len_xz = len_xz_sq.sqrt();
            let len_xyz_sq = (dy * dy + dx2) + dz2;

            let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
            let c_three = rd::<f32>(exe::ptr::<f32>(0x1_4342_6bf8) as usize);

            let inv_xyz = nr_rsqrt(len_xyz_sq, c_half, c_three);
            let dir_x = dx * inv_xyz;
            let dir_y = dy * inv_xyz;
            let dir_z = dz * inv_xyz;

            let inv_xz = nr_rsqrt(len_xz_sq, c_half, c_three);
            let prev_x = rd::<f32>(param_1 + 0xc8);
            let prev_z = rd::<f32>(param_1 + 0xd0);
            let prev_xz_sq = (prev_x * prev_x + 0.0f32) + prev_z * prev_z;
            let inv_prev_xz = nr_rsqrt(prev_xz_sq, c_half, c_three);

            let cos_thresh =
                c_one - rd::<f32>(exe::ptr::<f32>(0x1_4386_5138) as usize) * deg_to_rad;
            let dot_xz = ((prev_x * inv_prev_xz) * (dx * inv_xz)
                + (inv_prev_xz * 0.0f32) * (inv_xz * 0.0f32))
                + (prev_z * inv_prev_xz) * (dz * inv_xz);

            let mut elapsed = (param_7 - rd::<f64>(param_1 + 0xd8)) as f32;
            let mut dl = 0u8;
            let c_001 = rd::<f32>(exe::ptr::<f32>(0x1_4342_64e4) as usize);
            let abs_mask = !rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
            let abs_dy = f32::from_bits(dy.to_bits() & abs_mask);
            if !(cos_thresh < dot_xz) && len_xz > c_001 && c_001 > abs_dy {
                wr::<f64>(param_1 + 0xd8, param_7);
                let fa0 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4fa0) as usize);
                dl = if fa0 > elapsed { 1 } else { 0 };
                elapsed = 0.0f32;
            }

            let fa0 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4fa0) as usize);
            if fa0 > elapsed {
                let t_raw = elapsed / fa0;
                let min_t = if dl == 0 {
                    0.0f32
                } else {
                    rd::<f32>(exe::ptr::<f32>(0x1_4342_66d0) as usize)
                };
                let t = vminss(vmaxss(vmaxss(t_raw, min_t), 0.0f32), c_one);
                let len_xyz = len_xyz_sq.sqrt();
                let new_z = (len_xyz * dir_z) * t + base_pos[2];
                let new_y = (len_xyz * dir_y) * t + base_pos[1];
                let new_x = (len_xyz * dir_x) * t + base_pos[0];
                vec_00 = [new_x, new_y, new_z, 0.0f32];
            }

            wr::<f32>(param_1 + 0xc8, dir_x);
            wr::<f32>(param_1 + 0xcc, dir_y);
            wr::<f32>(param_1 + 0xd0, dir_z);
            trans_13 = rd::<[f32; 4]>(param_5 + 0x10);
        }
    }

    let q_orig = rd::<[f32; 4]>(param_5);
    let final_rot = vaddps(
        quat_rotate_with_parts(
            perm_d2(q_orig),
            perm_c9(q_orig),
            perm_ff(q_orig),
            two_xyz,
            vsubps(vec_00, trans_13),
        ),
        trans_13,
    );
    let final_clamped = vminps(vmaxps(final_rot, min_bound), max_bound);
    wr::<[f32; 4]>(param_1 + 0xa0, final_clamped);
    wr::<u8>(param_1 + 0xb0, res_b60);
    wr::<u32>(param_1 + 0xb4, res_f64);

    if param_10 == 0 {
        let mut span_e0: [u64; 2] = [cnt_48 as u64, ptr_40 as u64];
        exe::call_va!(
            0x1_4217_8890,
            (span_e0.as_mut_ptr() as usize, param_11)
        );

        let vec_b8 = param_1 + 0xb8;
        let end_50 = ptr_50.wrapping_add((cnt_58 as usize).wrapping_mul(12));
        let cur_cap = rd::<u32>(vec_b8 + 0x0c) as u64;
        let new_cnt = ((end_50.wrapping_sub(ptr_50) as i64) / 12) as u64;
        if new_cnt > cur_cap {
            wr::<u32>(vec_b8 + 0x08, 0);
            let grow = (cur_cap >> 1).wrapping_add(cur_cap);
            let new_cap = if grow < new_cnt { new_cnt } else { grow };
            exe::call_va!(0x1_4087_4640, (vec_b8, new_cap));
        }
        let mut dst = rd::<usize>(vec_b8);
        let mut src = ptr_50;
        while src != end_50 {
            wr::<u64>(dst, rd::<u64>(src));
            wr::<u32>(dst + 0x08, rd::<u32>(src + 0x08));
            dst = dst.wrapping_add(12);
            src = src.wrapping_add(12);
        }
        wr::<u32>(vec_b8 + 0x08, new_cnt as u32);
    }

    let delta_a0 = vsubps(saved_a0, rd::<[f32; 4]>(param_1 + 0xa0));
    let rot_delta = quat_rotate_with_parts(q_conj_d2, q_conj_c9, q_conj_ff, two_xyz, delta_a0);
    let neg_001 = rd::<f32>(exe::ptr::<f32>(0x1_4342_7014) as usize);
    let abs_mask_vec = rd::<u32>(exe::ptr::<u32>(0x1_4342_8720) as usize);
    let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let abs_dy = f32::from_bits(rot_delta[1].to_bits() & abs_mask_vec);
    let abs_vy = f32::from_bits(vel_rot[1].to_bits() & abs_mask_vec);
    let flag_19 = if neg_001 > vel_rot[1] && (abs_vy * c_half) > abs_dy {
        1u8
    } else {
        0u8
    };
    wr::<u8>(param_1 + 0x19, flag_19);

    if ptr_50 != 0 {
        exe::call_va!(0x1_4241_c190, (ptr_50));
    }
    if ptr_40 != 0 {
        exe::call_va!(0x1_4241_c190, (ptr_40));
    }

    let cb1 = rd::<usize>(filter_obj + 0x40);
    if cb1 != 0 {
        let vtbl = rd::<usize>(cb1);
        let not_inline = if cb1 != filter_obj + 0x08 { 1u8 } else { 0u8 };
        exe::call_va!(rd::<usize>(vtbl + 0x20), (cb1, not_inline));
        wr::<usize>(filter_obj + 0x40, 0);
    }
    wr::<usize>(filter_obj, exe::ptr::<u8>(0x1_433e_4400) as usize);

    let cb2 = rd::<usize>(param_3 + 0x38);
    if cb2 != 0 {
        let vtbl = rd::<usize>(cb2);
        let not_inline = if cb2 != param_3 { 1u8 } else { 0u8 };
        exe::call_va!(rd::<usize>(vtbl + 0x20), (cb2, not_inline));
        wr::<usize>(param_3 + 0x38, 0);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1421346d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, case_idx: usize| -> (usize, [u32; 3]) {
        let mut grid = [0.0f32; 64 * 3];
        let gp = grid.as_mut_ptr() as usize;
        for i in 0..64 {
            let base = gp + i * 12;
            let cx = ((i % 5) as f32) * 0.35 - 0.5;
            let cy = match (i + case_idx) % 4 {
                0 => -0.25 - (i as f32) * 0.02,
                1 => 0.4 + (i as f32) * 0.03,
                2 => 1.8 + (i as f32) * 0.05,
                _ => 12.0 + (i as f32) * 0.1,
            };
            let cz = ((i / 5) as f32) * 0.25 - 0.4;
            wr::<f32>(base, cx);
            wr::<f32>(base + 0x04, cy);
            wr::<f32>(base + 0x08, cz);
        }

        let mut meta = [0u8; 0x20];
        let mp = meta.as_mut_ptr() as usize;
        wr::<f32>(mp, 2.0);
        wr::<f32>(mp + 0x04, 0.5);
        wr::<i32>(mp + 0x08, 4);
        wr::<i32>(mp + 0x0c, 4);
        wr::<i32>(mp + 0x10, 1);
        wr::<i32>(mp + 0x14, 2);
        wr::<i32>(mp + 0x18, 1);

        let mut p2 = [0u8; 0x38];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x08, gp);
        wr::<usize>(p2p + 0x28, mp);

        let p3: [f32; 3] = [1.5, 2.0, 1.0];
        let p4: [i32; 3] = [1, 1, 1];
        let p5: [f32; 3] = if case_idx == 0 {
            [4.7, 6.8, 4.6]
        } else {
            [5.2, 7.4, 5.1]
        };

        let mut out = [0u32; 3];
        let out_p = out.as_mut_ptr() as usize;
        let ret = f(
            out_p,
            p2p,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
        );
        (ret.wrapping_sub(out_p), out)
    };

    for case_idx in [0usize, 1] {
        let o = run(f_o, case_idx);
        let n = run(f_n, case_idx);
        crate::diff_eq!(o, n, "1421346d0 case={}", case_idx);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14217ab60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        u64,
        usize,
        usize,
        usize,
        u32,
        u64,
        u32,
        u64,
        u64,
        usize,
    );
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: u64 = 0;
    static mut DUMMY_ARR_40: [u32; 6] = [10, 20, 30, 40, 50, 60];
    static mut DUMMY_ARR_50: [u32; 6] = [101, 102, 103, 201, 202, 203];

    unsafe extern "win64" fn mock_cba0(filter_sub: usize, _p3: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(1);
        wr::<u64>(filter_sub, 0x1234);
    }
    unsafe extern "win64" fn mock_b7a0() {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(2);
    }
    unsafe extern "win64" fn mock_a860(out: usize, _rbx: usize, arg_40: usize) -> usize {
        CALL_TRACE = CALL_TRACE
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(arg_40) as u64);
        for i in 0..13 {
            wr::<u32>(out + i * 4, 0x1000 + i as u32);
        }
        out
    }
    unsafe extern "win64" fn mock_a510(buf_1a0: usize, buf_210: usize, _rbx: usize) {
        CALL_TRACE = CALL_TRACE
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(buf_210) as u64);
        wr::<u64>(buf_1a0, 0x5566);
    }
    unsafe extern "win64" fn mock_f470(
        _rsi: usize,
        _b1a0: usize,
        _b1d0: usize,
        rbx0: u64,
        _p40: usize,
        _rbx_f8: usize,
        flag: u8,
        zero: u8,
    ) {
        CALL_TRACE = CALL_TRACE
            .wrapping_mul(31)
            .wrapping_add(rbx0 ^ (flag as u64) ^ ((zero as u64) << 8));
    }
    unsafe extern "win64" fn populate_res(out: usize) -> usize {
        wr::<[f32; 4]>(out, [1.5, 2.5, -1.0, 0.0]);
        wr::<[f32; 4]>(out + 0x10, [0.5, 1.2, -0.5, 0.0]);
        wr::<[f32; 4]>(out + 0x20, [0.8, 1.6, -0.2, 0.0]);
        wr::<[f32; 4]>(out + 0x30, [0.0, 2.2, 0.0, 0.0]);
        wr::<usize>(out + 0x40, core::ptr::addr_of_mut!(DUMMY_ARR_40) as usize);
        wr::<u32>(out + 0x48, 2);
        wr::<usize>(out + 0x50, core::ptr::addr_of_mut!(DUMMY_ARR_50) as usize);
        wr::<u32>(out + 0x58, 2);
        wr::<u8>(out + 0x60, 1);
        wr::<f32>(out + 0x64, 0.75);
        out
    }
    unsafe extern "win64" fn mock_c3e0(out: usize, p70: u64, _ctx: usize) -> usize {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(p70);
        populate_res(out)
    }
    unsafe extern "win64" fn mock_33f0(out: usize, _rsi: usize, _rbx: usize) -> usize {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(77);
        populate_res(out)
    }
    unsafe extern "win64" fn mock_ad10(p1: usize, _p2: usize, y: f32, _ee8: f32) {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(88);
        wr::<f32>(p1, y + 0.125);
    }
    unsafe extern "win64" fn mock_8890(span: usize, p11: usize) {
        CALL_TRACE = CALL_TRACE
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(span) ^ (p11 as u64));
    }
    unsafe extern "win64" fn mock_4640(_vec: usize, new_cap: u64) {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(new_cap);
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_mul(31).wrapping_add(99);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4128_cba0, mock_cba0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4213_b7a0, mock_b7a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4213_a860, mock_a860 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4213_a510, mock_a510 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4216_f470, mock_f470 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4212_c3e0, mock_c3e0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4217_33f0, mock_33f0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4058_ad10, mock_ad10 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4217_8890, mock_8890 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4087_4640, mock_4640 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F, p9: u8, p10: u8| -> (u64, Vec<u32>, Vec<u32>, [u32; 6]) {
        CALL_TRACE = 0;

        let mut p0_cfg = [0u8; 0x40];
        let p0p = p0_cfg.as_mut_ptr() as usize;
        wr::<f32>(p0p + 0x20, 1.8);
        wr::<f32>(p0p + 0x24, 0.35);
        wr::<f32>(p0p + 0x28, 0.5);
        wr::<f32>(p0p + 0x2c, 45.0);
        wr::<u8>(p0p + 0x30, 1);
        wr::<u8>(p0p + 0x32, 1);

        let mut p68_sub = [0u8; 0x60];
        let p68p = p68_sub.as_mut_ptr() as usize;

        let mut p78_sub = [0u8; 0x110];
        let p78p = p78_sub.as_mut_ptr() as usize;
        wr::<u64>(p78p, 0x777);

        let mut p_e0_sub = [0u8; 0x20];
        let pe0p = p_e0_sub.as_mut_ptr() as usize;

        let mut vec_b8_storage = [0u32; 6];
        let vb8p = vec_b8_storage.as_mut_ptr() as usize;

        let mut p1 = [0u8; 0x100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, p0p);
        wr::<u32>(p1p + 0x10, 0x42);
        wr::<u8>(p1p + 0x17, 1);
        wr::<u8>(p1p + 0x18, 0);
        wr::<[f32; 4]>(p1p + 0x20, [0.1, 0.2, 0.3, 0.0]);
        wr::<usize>(p1p + 0x68, p68p);
        wr::<u64>(p1p + 0x70, 0x999);
        wr::<usize>(p1p + 0x78, p78p);
        wr::<[f32; 4]>(p1p + 0xa0, [1.0, 2.0, -0.5, 0.0]);
        wr::<usize>(p1p + 0xb8, vb8p);
        wr::<u32>(p1p + 0xc0, 0);
        wr::<u32>(p1p + 0xc4, 4);
        wr::<[f32; 3]>(p1p + 0xc8, [0.6, 0.0, 0.8]);
        wr::<f64>(p1p + 0xd8, 1.0);
        wr::<usize>(p1p + 0xe0, pe0p);

        let mut p3 = [0u64; 8];
        let p4: [f32; 4] = [0.2, -0.5, 0.4, 0.0];
        let p5: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 0.5, 0.25, -0.25, 0.0];

        f(
            p1p,
            0,
            p3.as_mut_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            0.016f32.to_bits(),
            1.25f64.to_bits(),
            2.0f32.to_bits(),
            p9 as u64,
            p10 as u64,
            0xabcd,
        );

        // Zero stack pointers stored in p78 before comparing
        wr::<usize>(p78p + 0x08, 0);
        wr::<usize>(p78p + 0x10, 0);
        wr::<usize>(p78p + 0x98, 0);
        wr::<usize>(p1p, 0);
        wr::<usize>(p1p + 0x68, 0);
        wr::<usize>(p1p + 0x78, 0);
        wr::<usize>(p1p + 0xb8, 0);
        wr::<usize>(p1p + 0xe0, 0);

        let p1_words: Vec<u32> = (0..0x40).map(|i| rd::<u32>(p1p + i * 4)).collect();
        let p78_words: Vec<u32> = (0..0x30).map(|i| rd::<u32>(p78p + i * 4)).collect();
        (CALL_TRACE, p1_words, p78_words, vec_b8_storage)
    };

    for (p9, p10) in [(0u8, 0u8), (1, 0), (1, 1)] {
        let o = run(f_o, p9, p10);
        let n = run(f_n, p9, p10);
        crate::diff_eq!(o, n, "14217ab60 p9={} p10={}", p9, p10);
    }
    Ok(3)
}
