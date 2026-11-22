//! Decompiled `rend` hot functions shard Z8A (`0x142215330..0x14221fc20`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::*;

#[inline(always)]
unsafe fn call_f64_1(va: u64, x: f64) -> f64 {
    let (_, xmm0) = call_va8_xmm(
        va,
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
    f64::from_bits(xmm0 as u64)
}

#[inline(always)]
unsafe fn call_f32_1(va: u64, x: f32) -> f32 {
    let (_, xmm0) = call_va8_xmm(
        va,
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
    f32::from_bits(xmm0 as u32)
}

#[inline(always)]
unsafe fn sqrt_f64_checked(v: f64) -> f64 {
    if 0.0f64 > v {
        call_f64_1(0x1_4254_76dc, v)
    } else {
        v.sqrt()
    }
}

#[inline(always)]
unsafe fn push_f64_58(obj: usize, val: f64) {
    let len = rd::<u32>(obj + 0x60) as u64;
    let cap = rd::<u32>(obj + 0x64) as u64;
    if (len << 3) == (cap << 3) {
        let need = len.wrapping_add(1);
        let grow = (cap >> 1).wrapping_add(cap);
        let new_cap = if grow < need { need } else { grow };
        exe::call_va!(0x1_401d_eba0, (obj + 0x58, new_cap));
    }
    let idx = rd::<u32>(obj + 0x60) as usize;
    let base = rd::<usize>(obj + 0x58);
    wr::<f64>(base.wrapping_add(idx * 8), val);
    wr::<u32>(obj + 0x60, (idx as u32).wrapping_add(1));
}

#[inline(always)]
unsafe fn push_local_f64(hdr: usize, val: f64) {
    let old_ptr = rd::<usize>(hdr);
    let len = rd::<u32>(hdr + 0x08) as usize;
    let cap = rd::<u32>(hdr + 0x0c) as usize;
    let end_len = old_ptr.wrapping_add(len * 8);
    let end_cap = old_ptr.wrapping_add(cap * 8);
    if end_len == end_cap {
        let byte_len = end_len.wrapping_sub(old_ptr);
        let elem_len = ((byte_len as isize) >> 3) as usize;
        let need = (elem_len as u64).wrapping_add(1);
        let elem_cap = ((end_cap.wrapping_sub(old_ptr) as isize) >> 3) as u64;
        let grow = (elem_cap >> 1).wrapping_add(elem_cap);
        let new_cap = if grow < need { need } else { grow };
        let new_ptr = exe::call_va!(0x1_401d_ec00, (elem_cap, new_cap) -> usize);
        let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, old_ptr, byte_len as u64) -> usize);
        exe::call_va!(0x1_401d_e830, (hdr));
        wr::<usize>(hdr, new_ptr);
        wr::<u32>(hdr + 0x08, elem_len as u32);
        wr::<u32>(hdr + 0x0c, new_cap as u32);
    }
    let cur_ptr = rd::<usize>(hdr);
    let cur_len = rd::<u32>(hdr + 0x08);
    wr::<f64>(cur_ptr.wrapping_add((cur_len as usize) * 8), val);
    wr::<u32>(hdr + 0x08, cur_len.wrapping_add(1));
}

#[inline(always)]
unsafe fn push_local_i32(hdr: usize, val: i32) {
    let old_ptr = rd::<usize>(hdr);
    let len = rd::<u32>(hdr + 0x08) as usize;
    let cap = rd::<u32>(hdr + 0x0c) as usize;
    let end_len = old_ptr.wrapping_add(len * 4);
    let end_cap = old_ptr.wrapping_add(cap * 4);
    if end_len == end_cap {
        let byte_len = end_len.wrapping_sub(old_ptr);
        let elem_len = ((byte_len as isize) >> 2) as usize;
        let need = (elem_len as u64).wrapping_add(1);
        let elem_cap = ((end_cap.wrapping_sub(old_ptr) as isize) >> 2) as u64;
        let grow = (elem_cap >> 1).wrapping_add(elem_cap);
        let new_cap = if grow < need { need } else { grow };
        let new_ptr = exe::call_va!(0x1_401d_ecf0, (elem_cap, new_cap) -> usize);
        let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, old_ptr, byte_len as u64) -> usize);
        exe::call_va!(0x1_401d_e830, (hdr));
        wr::<usize>(hdr, new_ptr);
        wr::<u32>(hdr + 0x08, elem_len as u32);
        wr::<u32>(hdr + 0x0c, new_cap as u32);
    }
    let cur_ptr = rd::<usize>(hdr);
    let cur_len = rd::<u32>(hdr + 0x08);
    wr::<i32>(cur_ptr.wrapping_add((cur_len as usize) * 4), val);
    wr::<u32>(hdr + 0x08, cur_len.wrapping_add(1));
}

#[inline(always)]
unsafe fn push_rec64(param_1: usize, rec: &[u8; 64]) {
    let len = rd::<u32>(param_1 + 0x10) as u64;
    let cap = rd::<u32>(param_1 + 0x14) as u64;
    if (len << 6) == (cap << 6) {
        let new_cap = (((len << 6) as i64) >> 6).wrapping_add(1) as u64;
        exe::call_va!(0x1_411a_2010, (param_1 + 0x08, new_cap));
    }
    let idx = rd::<u32>(param_1 + 0x10) as usize;
    let dst = rd::<usize>(param_1 + 0x08).wrapping_add(idx << 6);
    wr::<[u8; 64]>(dst, *rec);
    wr::<u32>(param_1 + 0x10, (idx as u32).wrapping_add(1));
}

/// `unknown_142215330`
/// Original: 0x142215330 (4753 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142215330(param_1: usize) {
    let r14 = param_1;
    let rbx0 = rd::<usize>(r14);
    let k_val = rd::<i32>(rbx0 + 0x08);
    let two_pi = rd::<f64>(exe::ptr::<f64>(0x1_4342_6b60) as usize);
    let rsp_78 = two_pi / (k_val as f64);
    let xmm15 = (rd::<f32>(rbx0 + 0x0c) as f64) * rsp_78;

    wr::<u32>(r14 + 0x1c, 0);
    wr::<u32>(r14 + 0x24, 0xffff_ffff);
    wr::<u32>(r14 + 0xb0, 0);

    let h = rd::<i32>(rbx0 + 0x24);
    let w = rd::<i32>(rbx0 + 0x20);
    let diag_sq = (h.wrapping_mul(h).wrapping_add(w.wrapping_mul(w))) as f32;
    let diag = if 0.0f32 > diag_sq {
        call_f32_1(0x1_4183_add0, diag_sq)
    } else {
        diag_sq.sqrt()
    };
    let diag_i32 = (cvt_f32_i32(diag) as i32).wrapping_mul(2);
    wr::<i32>(r14 + 0xac, diag_i32);

    let r8d = rd::<i32>(rbx0 + 0x08) / 2;
    let rsp_68_half = r8d;
    let adj_half = if r8d == 0x12 { 0x14 } else { r8d };

    let mut vec_50_buf = [0u64; 2];
    let vec_50 = vec_50_buf.as_mut_ptr() as usize;
    let mut vec_20_buf = [0u64; 2];
    let vec_20 = vec_20_buf.as_mut_ptr() as usize;

    if rd::<u8>(rbx0 + 0x10) != 0 {
        let sign_mask = ((adj_half >> 31) as u32) & 3;
        let sum = (adj_half as u32).wrapping_add(sign_mask);
        let ecx = (sum as i32) >> 2;
        if (sum & 3) != sign_mask {
            let msg = exe::ptr::<u8>(0x1_433f_0508) as usize;
            let exc = exe::call_va!(0x1_412e_8690, (vec_20, msg) -> usize);
            exe::call_va!(0x1_412e_88c0, (exc));
            return;
        }

        let mut xmm6 = 0.0f64;
        let mut xmm7 = 0.0f64;
        let xmm10 = rd::<f64>(exe::ptr::<f64>(0x1_4342_6a48) as usize);
        let limit_70 = if r8d == 0x12 { ecx } else { ecx.wrapping_add(1) };
        let edx_sq = ecx.wrapping_mul(ecx);

        if limit_70 > 0 {
            let xmm11 = ecx as f64;
            for ebx in 0..limit_70 {
                let cur_obj = rd::<usize>(r14);
                let step = rd::<f32>(cur_obj + 0x18) as f64;
                let dist_sq = (ebx.wrapping_mul(ebx).wrapping_add(edx_sq)) as f64;
                let xmm8 = sqrt_f64_checked(dist_sq) * step;
                let atan_val = if ebx == 0 {
                    0.0f64
                } else {
                    call_f64_1(0x1_4254_76b8, (ebx as f64) / xmm11)
                };
                let xmm9 = atan_val + xmm15;
                push_f64_58(rd::<usize>(r14), xmm9);
                push_local_f64(vec_50, xmm8);
                if ebx != 0 && ecx != 0 {
                    xmm6 += xmm8;
                    xmm7 += xmm10;
                } else {
                    let idx = rd::<i32>(rd::<usize>(r14) + 0x60).wrapping_sub(1);
                    push_local_i32(vec_20, idx);
                }
            }
        }

        let mut edi = ecx.wrapping_sub(1);
        let ecx2 = ecx.wrapping_mul(2);
        let count2 = if rsp_68_half == 0x12 {
            ecx2.wrapping_sub(1)
        } else {
            ecx2
        };
        let xmm11_pi = rd::<f64>(exe::ptr::<f64>(0x1_4342_6af8) as usize);
        if count2 > 0 {
            let xmm12 = ecx as f64;
            for _ in 0..count2 {
                let cur_obj = rd::<usize>(r14);
                let step = rd::<f32>(cur_obj + 0x18) as f64;
                let dist_sq = (edi.wrapping_mul(edi).wrapping_add(edx_sq)) as f64;
                let xmm8 = sqrt_f64_checked(dist_sq) * step;
                let mut atan_val = call_f64_1(0x1_4254_76b8, xmm12 / (edi as f64));
                if 0.0f64 > atan_val {
                    atan_val += xmm11_pi;
                }
                let xmm9 = atan_val + xmm15;
                push_f64_58(cur_obj, xmm9);
                push_local_f64(vec_50, xmm8);
                if ecx != 0 && edi != 0 {
                    xmm6 += xmm8;
                    xmm7 += xmm10;
                } else {
                    let idx = rd::<i32>(rd::<usize>(r14) + 0x60).wrapping_sub(1);
                    push_local_i32(vec_20, idx);
                }
                edi = edi.wrapping_sub(1);
            }
        }

        let mut ebx = ecx.wrapping_sub(1);
        let esi = ecx.wrapping_neg();
        let count3 = ecx.wrapping_sub(1);
        if count3 > 0 {
            let esi_sq = esi.wrapping_mul(esi);
            let xmm12 = esi as f64;
            for _ in 0..count3 {
                let cur_obj = rd::<usize>(r14);
                let step = rd::<f32>(cur_obj + 0x18) as f64;
                let dist_sq = (ebx.wrapping_mul(ebx).wrapping_add(esi_sq)) as f64;
                let xmm8 = sqrt_f64_checked(dist_sq) * step;
                let atan_val = call_f64_1(0x1_4254_76b8, (ebx as f64) / xmm12);
                let xmm9 = (atan_val + xmm11_pi) + xmm15;
                push_f64_58(cur_obj, xmm9);
                push_local_f64(vec_50, xmm8);
                if ebx != 0 && esi != 0 {
                    xmm6 += xmm8;
                    xmm7 += xmm10;
                } else {
                    let idx = rd::<i32>(rd::<usize>(r14) + 0x60).wrapping_sub(1);
                    push_local_i32(vec_20, idx);
                }
                ebx = ebx.wrapping_sub(1);
            }
        }

        let xmm3 = xmm6 / xmm7;
        let idx_count = rd::<u32>(vec_20 + 0x08) as usize;
        let idx_base = rd::<usize>(vec_20);
        let rad_base = rd::<usize>(vec_50);
        for i in 0..idx_count {
            let slot = rd::<i32>(idx_base.wrapping_add(i * 4)) as isize;
            wr::<f64>((rad_base as isize + slot * 8) as usize, xmm3);
        }

        let obj_a = rd::<usize>(r14);
        let old_18 = rd::<f32>(obj_a + 0x18) as f64;
        let ratio = (xmm3 / old_18) as f32;
        wr::<f32>(obj_a + 0x14, ratio * rd::<f32>(obj_a + 0x14));
        let obj_b = rd::<usize>(r14);
        wr::<f32>(obj_b + 0x18, xmm3 as f32);
        exe::call_va!(0x1_401d_e830, (vec_20));
        wr::<[u64; 2]>(vec_20, [0; 2]);
    }

    let mut rbx = rd::<usize>(r14);
    if rd::<u32>(rbx + 0x60) > 0 {
        let r15_half_bits: u64 = 0x3fe0_0000_0000_0000;
        let xmm13_sign = rd::<u64>(exe::ptr::<u64>(0x1_4342_8740) as usize);
        let xmm9_half = rd::<f64>(exe::ptr::<f64>(0x1_4342_69f8) as usize);
        let mut r9d: i32 = 0;
        let mut rdi_idx: usize = 0;

        loop {
            let mut angle = (r9d as f64) * rsp_78 + xmm15;
            let (xmm7_cos, xmm3_sin, xmm0_step) = if rd::<u8>(rbx + 0x10) != 0 {
                let arr_58 = rd::<usize>(rbx + 0x58);
                angle = rd::<f64>(arr_58.wrapping_add(rdi_idx * 8));
                let c = call_f64_1(0x1_4183_ad6a, angle);
                let s = call_f64_1(0x1_4183_adb8, angle);
                let step = rd::<f64>(rd::<usize>(vec_50).wrapping_add(rdi_idx * 8));
                (c, s, step)
            } else {
                let c = call_f64_1(0x1_4183_ad6a, angle);
                let s = call_f64_1(0x1_4183_adb8, angle);
                let step = rd::<f32>(rbx + 0x18) as f64;
                (c, s, step)
            };

            let xmm4_neg_cos = f64::from_bits(xmm7_cos.to_bits() ^ xmm13_sign);
            let xmm1_neg_sin = f64::from_bits(xmm3_sin.to_bits() ^ xmm13_sign);
            let xmm5 = xmm3_sin * xmm0_step;
            let xmm6 = xmm7_cos * xmm0_step;
            let xmm2_perp = rd::<f32>(rbx + 0x14) as f64;
            let rbp_dirs: [[f64; 2]; 2] = [
                [xmm7_cos * xmm2_perp, xmm1_neg_sin * xmm2_perp],
                [xmm2_perp * xmm4_neg_cos, xmm3_sin * xmm2_perp],
            ];

            wr::<u64>(r14 + 0x28, r15_half_bits);
            wr::<u64>(r14 + 0x30, r15_half_bits);
            let obj1 = rd::<usize>(r14);
            let h1 = rd::<i32>(obj1 + 0x24);
            wr::<u64>(r14 + 0x38, r15_half_bits);
            wr::<f64>(r14 + 0x40, (h1.wrapping_sub(1) as f64) + xmm9_half);

            let obj2 = rd::<usize>(r14);
            let h2 = rd::<i32>(obj2 + 0x24);
            let w2 = rd::<i32>(obj2 + 0x20);
            wr::<f64>(r14 + 0x48, (w2.wrapping_sub(1) as f64) + xmm9_half);
            wr::<f64>(r14 + 0x50, (h2.wrapping_sub(1) as f64) + xmm9_half);

            let obj3 = rd::<usize>(r14);
            let w3 = rd::<i32>(obj3 + 0x20);
            wr::<f64>(r14 + 0x58, (w3.wrapping_sub(1) as f64) + xmm9_half);
            wr::<u64>(r14 + 0x60, r15_half_bits);

            let obj4 = rd::<usize>(r14);
            let dx = (rd::<i32>(obj4 + 0x20).wrapping_sub(rd::<i32>(obj4 + 0x28)) as f64) * xmm9_half;
            let dy = (rd::<i32>(obj4 + 0x24).wrapping_sub(rd::<i32>(obj4 + 0x2c)) as f64) * xmm9_half;
            let neg_dy = f64::from_bits(dy.to_bits() ^ xmm13_sign);
            let neg_dx = f64::from_bits(dx.to_bits() ^ xmm13_sign);

            let c0 = rd::<u128>(r14 + 0x28);
            wr::<f64>(r14 + 0x68, f64::from_bits(c0 as u64) + dx);
            let c0_hi = vunpckhpd(c0, c0);
            wr::<f64>(r14 + 0x70, f64::from_bits(c0_hi as u64) + dy);

            let c1 = rd::<u128>(r14 + 0x38);
            wr::<f64>(r14 + 0x78, f64::from_bits(c1 as u64) + dx);
            let c1_hi = vunpckhpd(c1, c1);
            wr::<f64>(r14 + 0x80, f64::from_bits(c1_hi as u64) + neg_dy);

            let c2 = rd::<u128>(r14 + 0x48);
            wr::<f64>(r14 + 0x88, f64::from_bits(c2 as u64) + neg_dx);
            let c2_hi = vunpckhpd(c2, c2);
            wr::<f64>(r14 + 0x90, f64::from_bits(c2_hi as u64) + neg_dy);

            let c3 = rd::<u128>(r14 + 0x58);
            wr::<f64>(r14 + 0x98, f64::from_bits(c3 as u64) + neg_dx);
            let c3_hi = vunpckhpd(c3, c3);
            wr::<f64>(r14 + 0xa0, f64::from_bits(c3_hi as u64) + dy);

            let corner_idx: usize = if xmm6 > 0.0f64 {
                if xmm5 > 0.0f64 { 0 } else { 3 }
            } else if xmm5 > 0.0f64 {
                1
            } else {
                2
            };

            let mut rec64 = [0u8; 64];
            let rec_ptr = rec64.as_mut_ptr() as usize;
            let corner_vec = rd::<u128>(r14 + corner_idx * 16 + 0x28);
            wr::<u128>(rec_ptr, corner_vec);
            let step_vec = vunpcklpd(xmm5.to_bits() as u128, xmm6.to_bits() as u128);
            wr::<u128>(rec_ptr + 0x10, step_vec);
            wr::<i32>(rec_ptr + 0x20, r9d);

            let mut edi_steps: i32 = 0;
            let xmm3_min_x = rd::<f64>(r14 + 0x68);
            let mut xmm2_x = f64::from_bits(corner_vec as u64);
            let mut xmm1_y = f64::from_bits((corner_vec >> 64) as u64);

            let mut hit_inner = false;
            loop {
                if xmm2_x >= xmm3_min_x
                    && xmm1_y >= rd::<f64>(r14 + 0x70)
                    && rd::<f64>(r14 + 0x88) >= xmm2_x
                    && rd::<f64>(r14 + 0x90) >= xmm1_y
                {
                    hit_inner = true;
                    break;
                }
                xmm2_x += xmm5;
                xmm1_y += xmm6;
                edi_steps = edi_steps.wrapping_add(1);
                if !(xmm2_x >= rd::<f64>(r14 + 0x28)
                    && xmm1_y >= rd::<f64>(r14 + 0x30)
                    && rd::<f64>(r14 + 0x48) >= xmm2_x
                    && rd::<f64>(r14 + 0x50) >= xmm1_y)
                {
                    break;
                }
            }

            if hit_inner {
                while xmm2_x >= xmm3_min_x
                    && xmm1_y >= rd::<f64>(r14 + 0x70)
                    && rd::<f64>(r14 + 0x88) >= xmm2_x
                    && rd::<f64>(r14 + 0x90) >= xmm1_y
                {
                    xmm2_x += xmm5;
                    xmm1_y += xmm6;
                    edi_steps = edi_steps.wrapping_add(1);
                }
                wr::<i32>(rec_ptr + 0x24, edi_steps);
                wr::<u64>(rec_ptr + 0x30, 0);
                if edi_steps != 0 {
                    push_rec64(r14, &rec64);
                }
            } else {
                edi_steps = 0;
                wr::<i32>(rec_ptr + 0x24, 0);
                wr::<u64>(rec_ptr + 0x30, 0);
            }

            for dir_idx in 0..2usize {
                let mut cur64 = rec64;
                let cur_ptr = cur64.as_mut_ptr() as usize;
                let r13d_step: i32 = if dir_idx == 1 { 1 } else { -1 };
                let xmm6_sy = rd::<f64>(cur_ptr + 0x18);
                let xmm7_sx = rd::<f64>(cur_ptr + 0x10);
                let xmm12_dx = rbp_dirs[dir_idx][0];
                let xmm11_dy = rbp_dirs[dir_idx][1];
                let mut esi_cnt = rd::<i32>(cur_ptr + 0x34);
                let mut r15d_off = rd::<i32>(cur_ptr + 0x30);
                let mut xmm10_px = rd::<f64>(cur_ptr);
                let mut xmm9_py = rd::<f64>(cur_ptr + 0x08);

                loop {
                    let xmm8_nx = xmm10_px + xmm12_dx;
                    xmm9_py += xmm11_dy;
                    esi_cnt = esi_cnt.wrapping_add(r13d_step);
                    wr::<i32>(cur_ptr + 0x34, esi_cnt);

                    let max_steps = rd::<i32>(r14 + 0xac);
                    if max_steps <= 0 {
                        break;
                    }

                    let mut eax_step: i32 = 0;
                    let mut tx = xmm8_nx;
                    let mut ty = xmm9_py;
                    let outer_min_x = rd::<f64>(r14 + 0x28);
                    let mut found_outer = false;
                    while eax_step < max_steps {
                        if tx >= outer_min_x
                            && ty >= rd::<f64>(r14 + 0x30)
                            && rd::<f64>(r14 + 0x48) >= tx
                            && rd::<f64>(r14 + 0x50) >= ty
                        {
                            found_outer = true;
                            break;
                        }
                        eax_step = eax_step.wrapping_add(1);
                        tx += xmm7_sx;
                        ty += xmm6_sy;
                    }
                    if !found_outer || eax_step == -1 {
                        break;
                    }

                    let step_f64 = eax_step as f64;
                    xmm10_px = xmm8_nx + xmm7_sx * step_f64;
                    xmm9_py += xmm6_sy * step_f64;
                    r15d_off = r15d_off.wrapping_add(eax_step);
                    wr::<i32>(cur_ptr + 0x30, r15d_off);
                    let pos_vec = vunpcklpd(xmm10_px.to_bits() as u128, xmm9_py.to_bits() as u128);
                    wr::<u128>(cur_ptr, pos_vec);

                    let mut eax2: i32 = 0;
                    let inner_min_x = rd::<f64>(r14 + 0x68);
                    let outer_min_x2 = rd::<f64>(r14 + 0x28);
                    let mut xmm4_x = xmm10_px;
                    let mut xmm3_y = xmm9_py;
                    let mut hit_inner2 = false;
                    loop {
                        if xmm4_x >= inner_min_x
                            && xmm3_y >= rd::<f64>(r14 + 0x70)
                            && rd::<f64>(r14 + 0x88) >= xmm4_x
                            && rd::<f64>(r14 + 0x90) >= xmm3_y
                        {
                            hit_inner2 = true;
                            break;
                        }
                        xmm4_x += xmm7_sx;
                        xmm3_y += xmm6_sy;
                        eax2 = eax2.wrapping_add(1);
                        if !(xmm4_x >= outer_min_x2
                            && xmm3_y >= rd::<f64>(r14 + 0x30)
                            && rd::<f64>(r14 + 0x48) >= xmm4_x
                            && rd::<f64>(r14 + 0x50) >= xmm3_y)
                        {
                            break;
                        }
                    }

                    if !hit_inner2 {
                        wr::<i32>(cur_ptr + 0x24, 0);
                        continue;
                    }

                    while xmm4_x >= inner_min_x
                        && xmm3_y >= rd::<f64>(r14 + 0x70)
                        && rd::<f64>(r14 + 0x88) >= xmm4_x
                        && rd::<f64>(r14 + 0x90) >= xmm3_y
                    {
                        xmm4_x += xmm7_sx;
                        xmm3_y += xmm6_sy;
                        eax2 = eax2.wrapping_add(1);
                    }
                    wr::<i32>(cur_ptr + 0x24, eax2);
                    if eax2 != 0 {
                        edi_steps = edi_steps.wrapping_add(eax2);
                        push_rec64(r14, &cur64);
                    }
                }
            }

            let add_total = (edi_steps as u32).wrapping_mul(2);
            wr::<u32>(r14 + 0x1c, rd::<u32>(r14 + 0x1c).wrapping_add(add_total));
            r9d = r9d.wrapping_add(1);
            rbx = rd::<usize>(r14);
            let cnt_60 = rd::<u32>(rbx + 0x60) as u64;
            rdi_idx = (r9d as i64) as usize;
            if (rdi_idx as u64) >= cnt_60 {
                break;
            }
        }
    }

    wr::<u32>(rbx + 0x04, rd::<u32>(r14 + 0xa8));
    wr::<[u64; 2]>(vec_20, [0; 2]);
    let rcx_obj = rd::<usize>(r14);
    if rd::<u32>(rcx_obj + 0x60) > 0 {
        let mut idx: i32 = 0;
        loop {
            let cur_obj = rd::<usize>(r14);
            let arr_58 = rd::<usize>(cur_obj + 0x58);
            let elem = rd::<f64>(arr_58.wrapping_add(((idx as i64) as usize) * 8));
            push_local_f64(vec_20, elem);
            idx = idx.wrapping_add(1);
            let cur_obj2 = rd::<usize>(r14);
            let rem_cnt = rd::<u32>(cur_obj2 + 0x60) as u64;
            if ((idx as i64) as u64) >= rem_cnt {
                break;
            }
        }
    }

    let rcx_obj2 = rd::<usize>(r14);
    wr::<u32>(rcx_obj2 + 0x60, 0);
    let copied_len = rd::<u32>(vec_20 + 0x08) as usize;
    if copied_len != 0 {
        let src_angles = rd::<usize>(vec_20);
        let src_radii = rd::<usize>(vec_50);
        for i in 0..copied_len {
            let a = rd::<f64>(src_angles.wrapping_add(i * 8));
            push_f64_58(rd::<usize>(r14), a);
            let r = rd::<f64>(src_radii.wrapping_add(i * 8));
            push_f64_58(rd::<usize>(r14), r);
        }
    }

    exe::call_va!(0x1_401d_e830, (vec_20));
    exe::call_va!(0x1_401d_e830, (vec_50));
}

#[inline(always)]
unsafe fn rstring_c_str(s: usize) -> usize {
    let mut rdx = s + 0x08;
    let b3 = rd::<u8>(s + 0x03) as u32;
    let cap_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(s) & 0x00ff_ffff) << 3;
    if len_bits > cap_bits {
        rdx = rd::<usize>(rdx);
    }
    rdx
}

#[inline(always)]
unsafe fn c_strlen(s: usize) -> u64 {
    let mut len = 0usize;
    while rd::<u8>(s + len) != 0 {
        len += 1;
    }
    len as u64
}

#[inline(always)]
unsafe fn init_r_file(file_ptr: usize) {
    wr::<usize>(file_ptr, exe::ptr::<u8>(0x1_4340_ecd8) as usize);
    wr::<u32>(file_ptr + 0x08, 0x2000_0020);
    wr::<u32>(file_ptr + 0x0c, 0);
    wr::<u8>(file_ptr + 0x10, 0);
}

/// `rend_14221fc20`
/// Original: 0x14221fc20 (2398 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_14221fc20(
    param_1: usize,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u8,
    param_6: usize,
    param_7: usize,
    param_8: usize,
    param_9: i32,
) {
    wr::<u32>(param_1 + 0x18c, param_3);
    wr::<u32>(param_1 + 0x190, param_4);
    wr::<u64>(param_1 + 0x168, 0);

    if param_9 > 0 {
        let scale_step = rd::<f32>(exe::ptr::<f32>(0x1_4342_668c) as usize);
        let glob_b8_ptr = exe::ptr::<usize>(0x1_4397_feb8) as usize;
        let glob_c0_ptr = exe::ptr::<usize>(0x1_4397_fec0) as usize;
        let glob_c8_ptr = exe::ptr::<u64>(0x1_4397_fec8) as usize;

        for iter_idx in 0..param_9 {
            let rbx_off = (iter_idx as usize) * 8;
            let rdi_slot = param_1 + 0x148 + rbx_off;
            let r12_vec = param_1 + 0xd0 + (iter_idx as usize) * 0x10;

            let sc = (iter_idx as f32) * scale_step;
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_4221_7490,
                param_3 as u64,
                param_4 as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, 0, sc.to_bits() as u128, 0],
            );

            let mut local_80: u64 = 0;
            let p6_elem = rd::<usize>(param_6).wrapping_add(rbx_off);
            let res_148 = exe::call_va!(
                0x1_4221_70a0,
                (p6_elem, (&mut local_80 as *mut u64) as usize) -> u64
            );
            wr::<u64>(rdi_slot, res_148);
            local_80 = local_80.wrapping_mul(8);
            wr::<u32>(r12_vec, 0);

            let mut rsi_off: usize = 0;
            let mut r15_idx: u64 = 1;
            let mut r14_len: u32 = 0;
            let mut rbx_glob = rd::<usize>(glob_b8_ptr);

            for _ in 0..0x12 {
                let slot_16: [u32; 4] = if rbx_glob != 0
                    && r15_idx < (rd::<u32>(rbx_glob + 0x60) as u64)
                {
                    let arr_58 = rd::<usize>(rbx_glob + 0x58);
                    let d_rad = rd::<f64>(arr_58 + rsi_off + 0x08);
                    let f_rad_bits = vcvtpd2ps(d_rad.to_bits() as u128) as u32;
                    let d_ang = rd::<f64>(arr_58 + rsi_off);
                    let f_cos = call_f64_1(0x1_4183_ad6a, d_ang) as f32;
                    let f_sin = call_f64_1(0x1_4183_adb8, d_ang) as f32;
                    [f_sin.to_bits(), f_cos.to_bits(), f_rad_bits, 0]
                } else {
                    [0; 4]
                };

                let vec_hdr = r12_vec - 0x08;
                let cap = rd::<u32>(r12_vec + 0x04) as u64;
                if ((r14_len as u64) << 4) == (cap << 4) {
                    let need = (r14_len as u64).wrapping_add(1);
                    let grow = (cap >> 1).wrapping_add(cap);
                    let new_cap = if grow < need { need } else { grow };
                    exe::call_va!(0x1_4062_ee30, (vec_hdr, new_cap));
                    rbx_glob = rd::<usize>(glob_b8_ptr);
                }
                let cur_idx = rd::<u32>(r12_vec) as usize;
                let dst = rd::<usize>(vec_hdr).wrapping_add(cur_idx << 4);
                wr::<[u32; 4]>(dst, slot_16);
                r14_len = rd::<u32>(r12_vec).wrapping_add(1);
                wr::<u32>(r12_vec, r14_len);
                r15_idx = r15_idx.wrapping_add(2);
                rsi_off = rsi_off.wrapping_add(0x10);
            }

            if rd::<u64>(param_1 + 0x168) < local_80 {
                wr::<u64>(param_1 + 0x168, local_80);
            }

            let p7_slot = rd::<usize>(param_7).wrapping_add(rbx_off);
            let c0_ptr = rd::<usize>(glob_c0_ptr);
            let c8_cnt = rd::<u64>(glob_c8_ptr);
            let r13_cnt: u64 = if c0_ptr != 0 && c8_cnt != 0 {
                let dst = exe::call_va!(0x1_4221_6df0, (0u64, 0u64, 0u64, c8_cnt) -> usize);
                wr::<usize>(p7_slot, dst);
                let _ = exe::call_va!(0x1_4183_acbc, (dst, c0_ptr, c8_cnt.wrapping_mul(4)) -> usize);
                c8_cnt
            } else {
                let mut pair_30: [usize; 2] = [rbx_glob.wrapping_add(0x48), rbx_glob];
                let bytes = exe::call_va!(
                    0x1_4221_2350,
                    (pair_30.as_mut_ptr() as usize, p7_slot) -> u64
                );
                bytes >> 2
            };

            let r15_buf = rd::<usize>(p7_slot);
            let cur_b8 = rd::<usize>(glob_b8_ptr);
            let rbx_w = rd::<i32>(cur_b8 + 0x20);
            let edi_h = rd::<i32>(cur_b8 + 0x24);
            if edi_h > 0 {
                let plane_elems = (edi_h.wrapping_mul(rbx_w)) as isize;
                let mut rsi_row = (r15_buf as isize + plane_elems * 4) as usize;
                let row_step = (rbx_w as isize) * 4;
                for r9d in 0..edi_h {
                    if rbx_w > 0 {
                        let r14d = (((r9d as u32) / 3).wrapping_mul(3)) as i32;
                        let mut r11_dst = rsi_row;
                        for r10d in 0..rbx_w {
                            let div3 = r10d / 3;
                            let mut r8d = r9d.wrapping_sub(div3).wrapping_sub(r14d);
                            r8d = r8d.wrapping_add(r10d.wrapping_add(r8d.wrapping_mul(2)));
                            let idx0 = r8d
                                .wrapping_mul(edi_h)
                                .wrapping_add(r9d)
                                .wrapping_mul(rbx_w)
                                .wrapping_add(r10d) as isize;
                            let v0 = rd::<u32>((r15_buf as isize + idx0 * 4) as usize);
                            wr::<u32>((r11_dst as isize - plane_elems * 4) as usize, v0);

                            let idx1 = r8d
                                .wrapping_add(9)
                                .wrapping_mul(edi_h)
                                .wrapping_add(r9d)
                                .wrapping_mul(rbx_w)
                                .wrapping_add(r10d) as isize;
                            let v1 = rd::<u32>((r15_buf as isize + idx1 * 4) as usize);
                            wr::<u32>(r11_dst, v1);
                            r11_dst = r11_dst.wrapping_add(4);
                        }
                    }
                    rsi_row = (rsi_row as isize + row_step) as usize;
                }
            }

            let p8_slot = rd::<usize>(param_8).wrapping_add(rbx_off);
            wr::<u64>(p8_slot, r13_cnt / 9);
        }
    }

    let _ = atomic_add_u32((param_1 + 0x188) as u64, 1);

    let mut str_buf = [0u64; 40];
    let str_4c0 = str_buf.as_mut_ptr() as usize;
    let s0 = exe::call_va!(
        0x1_4222_08c0,
        (param_1, str_4c0, param_2, param_3, param_4, param_5) -> usize
    );
    let cstr0 = rstring_c_str(s0);
    let rdi_meta = exe::call_va!(
        0x1_423f_e030,
        (exe::ptr::<u8>(0x1_433f_07b0) as usize, cstr0) -> usize
    );
    exe::call_va!(0x1_4240_ab10, (str_4c0));

    let mut span_30: [u64; 2] = [rdi_meta as u64, c_strlen(rdi_meta)];
    let mut tmp_70: [u64; 2] = [0; 2];
    exe::call_va!(
        0x1_401d_f850,
        (tmp_70.as_mut_ptr() as usize, span_30.as_mut_ptr() as usize)
    );
    span_30 = tmp_70;
    let exists = exe::call_va!(0x1_4249_13d0, (span_30.as_mut_ptr() as usize) -> u8);
    if exists == 0 {
        let mut span_60: [u64; 2] = [rdi_meta as u64, c_strlen(rdi_meta)];
        let mut tmp_90: [u64; 2] = [0; 2];
        exe::call_va!(
            0x1_401d_f850,
            (tmp_90.as_mut_ptr() as usize, span_60.as_mut_ptr() as usize)
        );
        let rbx_320 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_8320) as usize);
        span_30 = tmp_90;
        let mut tmp_80: [u64; 2] = [0; 2];
        exe::call_va!(
            0x1_40f3_5ac0,
            (span_30.as_mut_ptr() as usize, tmp_80.as_mut_ptr() as usize)
        );
        span_30 = tmp_80;
        exe::call_va!(0x1_4248_e060, (span_30.as_mut_ptr() as usize, rbx_320));
    }

    span_30 = [rdi_meta as u64, c_strlen(rdi_meta)];
    let mut file_370_buf = [0u64; 44];
    let file_370 = file_370_buf.as_mut_ptr() as usize;
    init_r_file(file_370);
    exe::call_va!(
        0x1_4249_7fd0,
        (file_370, span_30.as_mut_ptr() as usize, 1u32)
    );

    let mut stream_40_buf: [usize; 2] = [exe::ptr::<u8>(0x1_4340_e9e0) as usize, file_370];
    let stream_40 = stream_40_buf.as_mut_ptr() as usize;
    exe::call_va!(0x1_4249_57b0, (stream_40, param_1 + 0x18c));
    let vtbl0 = rd::<usize>(stream_40);
    exe::call_va!(rd::<usize>(vtbl0 + 0xe8), (stream_40, param_1 + 0x190));
    let vtbl1 = rd::<usize>(stream_40);
    exe::call_va!(rd::<usize>(vtbl1 + 0x88), (stream_40, param_1 + 0x168));

    for ebx in 0..param_9 {
        let rdi = (ebx as isize) as usize;
        let vtbl_a = rd::<usize>(stream_40);
        exe::call_va!(
            rd::<usize>(vtbl_a + 0x88),
            (stream_40, param_1.wrapping_add(rdi.wrapping_add(0x29) * 8))
        );
        let vtbl_b = rd::<usize>(stream_40);
        let fn_88 = rd::<usize>(vtbl_b + 0x88);
        let p8_base = rd::<usize>(param_8);
        exe::call_va!(fn_88, (stream_40, p8_base.wrapping_add(rdi * 8)));
    }

    for edi in 0..param_9 {
        let rsi = (edi as isize) as usize;
        let s_line = exe::call_va!(
            0x1_4222_08c0,
            (param_1, str_4c0, param_2, param_3, param_4, param_5) -> usize
        );
        let cstr_line = rstring_c_str(s_line);
        let rbx_line = exe::call_va!(
            0x1_423f_e030,
            (exe::ptr::<u8>(0x1_433f_07c0) as usize, cstr_line, edi as u32) -> usize
        );
        exe::call_va!(0x1_4240_ab10, (str_4c0));

        span_30 = [rbx_line as u64, c_strlen(rbx_line)];
        let mut file_220_buf = [0u64; 44];
        let file_220 = file_220_buf.as_mut_ptr() as usize;
        init_r_file(file_220);
        exe::call_va!(
            0x1_4249_7fd0,
            (file_220, span_30.as_mut_ptr() as usize, 1u32)
        );
        let line_cnt = rd::<u64>(param_1 + 0x148 + rsi * 8);
        let line_bytes = line_cnt.wrapping_mul(40);
        let line_ptr = rd::<u64>(rd::<usize>(param_6).wrapping_add(rsi * 8));
        exe::call_va!(0x1_4249_8740, (file_220, line_ptr, line_bytes));

        let s_idx = exe::call_va!(
            0x1_4222_08c0,
            (param_1, str_4c0, param_2, param_3, param_4, param_5) -> usize
        );
        let cstr_idx = rstring_c_str(s_idx);
        let rbx_idx = exe::call_va!(
            0x1_423f_e030,
            (exe::ptr::<u8>(0x1_433f_07d8) as usize, cstr_idx, edi as u32) -> usize
        );
        exe::call_va!(0x1_4240_ab10, (str_4c0));

        span_30 = [rbx_idx as u64, c_strlen(rbx_idx)];
        let mut file_d0_buf = [0u64; 44];
        let file_d0 = file_d0_buf.as_mut_ptr() as usize;
        init_r_file(file_d0);
        exe::call_va!(
            0x1_4249_7fd0,
            (file_d0, span_30.as_mut_ptr() as usize, 1u32)
        );
        let idx_bytes = rd::<u64>(rd::<usize>(param_8).wrapping_add(rsi * 8)) << 2;
        let idx_ptr = rd::<u64>(rd::<usize>(param_7).wrapping_add(rsi * 8));
        exe::call_va!(0x1_4249_8740, (file_d0, idx_ptr, idx_bytes));

        exe::call_va!(0x1_4249_8af0, (file_d0));
        exe::call_va!(0x1_4249_8af0, (file_220));
    }

    exe::call_va!(0x1_4249_8af0, (file_370));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142215330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA: [u64; 4096] = [0; 4096];
    static mut ARENA_OFF: usize = 0;
    static mut FREE_CALLS: u32 = 0;

    unsafe fn arena_alloc(bytes: usize) -> usize {
        let aligned = (bytes + 7) & !7;
        let words = aligned / 8;
        let base = core::ptr::addr_of_mut!(ARENA) as usize + ARENA_OFF * 8;
        ARENA_OFF += words;
        core::ptr::write_bytes(base as *mut u8, 0, aligned);
        base
    }

    unsafe extern "win64" fn mock_eba0(hdr: usize, new_cap: u64) {
        let old_ptr = rd::<usize>(hdr);
        let old_len = rd::<u32>(hdr + 0x08) as usize;
        let new_ptr = arena_alloc((new_cap as usize) * 8);
        if old_ptr != 0 && old_len != 0 {
            core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, old_len * 8);
        }
        wr::<usize>(hdr, new_ptr);
        wr::<u32>(hdr + 0x0c, new_cap as u32);
    }

    unsafe extern "win64" fn mock_ec00(_old_cap: u64, new_cap: u64) -> usize {
        arena_alloc((new_cap as usize) * 8)
    }

    unsafe extern "win64" fn mock_ecf0(_old_cap: u64, new_cap: u64) -> usize {
        arena_alloc((new_cap as usize) * 4)
    }

    unsafe extern "win64" fn mock_e830(_hdr: usize) {
        FREE_CALLS += 1;
    }

    unsafe extern "win64" fn mock_2010(hdr: usize, new_cap: u64) {
        let old_ptr = rd::<usize>(hdr);
        let old_len = rd::<u32>(hdr + 0x08) as usize;
        let new_ptr = arena_alloc((new_cap as usize) * 64);
        if old_ptr != 0 && old_len != 0 {
            core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, old_len * 64);
        }
        wr::<usize>(hdr, new_ptr);
        wr::<u32>(hdr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_ec00, mock_ec00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_ecf0, mock_ecf0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411a_2010, mock_2010 as *const ())?;

    let run = |f: F, preset_flag: u8| -> (Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, u32) {
        ARENA = [0; 4096];
        ARENA_OFF = 0;
        FREE_CALLS = 0;

        let mut cfg = [0u64; 16];
        let cfg_p = cfg.as_mut_ptr() as usize;
        wr::<i32>(cfg_p + 0x08, 16);
        wr::<f32>(cfg_p + 0x0c, 0.125f32);
        wr::<u8>(cfg_p + 0x10, preset_flag);
        wr::<f32>(cfg_p + 0x14, 2.0f32);
        wr::<f32>(cfg_p + 0x18, 1.5f32);
        wr::<i32>(cfg_p + 0x20, 8);
        wr::<i32>(cfg_p + 0x24, 8);
        wr::<i32>(cfg_p + 0x28, 4);
        wr::<i32>(cfg_p + 0x2c, 4);

        if preset_flag == 0 {
            wr::<usize>(cfg_p + 0x58, 0);
            wr::<u32>(cfg_p + 0x60, 0);
            wr::<u32>(cfg_p + 0x64, 0);
        }

        let mut state = [0u64; 24];
        let st_p = state.as_mut_ptr() as usize;
        wr::<usize>(st_p, cfg_p);
        wr::<u32>(st_p + 0xa8, 0x42);

        f(st_p);

        let rec_len = rd::<u32>(st_p + 0x10) as usize;
        let rec_ptr = rd::<usize>(st_p + 0x08);
        let mut recs = Vec::with_capacity(rec_len * 8);
        for i in 0..(rec_len * 8) {
            recs.push(rd::<u64>(rec_ptr + i * 8));
        }

        let arr58_len = rd::<u32>(cfg_p + 0x60) as usize;
        let arr58_ptr = rd::<usize>(cfg_p + 0x58);
        let mut arr58 = Vec::with_capacity(arr58_len);
        for i in 0..arr58_len {
            arr58.push(rd::<u64>(arr58_ptr + i * 8));
        }

        wr::<usize>(st_p, 0);
        wr::<usize>(st_p + 0x08, 0);
        wr::<usize>(cfg_p + 0x58, 0);

        (state.to_vec(), cfg.to_vec(), recs, arr58, FREE_CALLS)
    };

    for flag in [1u8, 0u8] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        crate::diff_eq!(o, n, "142215330 preset_flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_rend_14221fc20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32, u32, u8, usize, usize, usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: Vec<u64> = Vec::new();
    static mut VEC_BUF: [u32; 128] = [0; 128];
    static mut IDX_BUF: [u32; 256] = [0; 256];
    static mut FAKE_CSTR: [u8; 32] = *b"cache/test_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

    unsafe extern "win64" fn mock_7490(p3: u32, p4: u32, scale: f32) {
        CALL_LOG.push(0x7490_0000_0000 | ((p3 as u64) << 16) | (p4 as u64));
        CALL_LOG.push(scale.to_bits() as u64);
    }

    unsafe extern "win64" fn mock_70a0(_p6_elem: usize, out_u64: usize) -> u64 {
        wr::<u64>(out_u64, 5);
        3
    }

    unsafe extern "win64" fn mock_ee30(hdr: usize, new_cap: u64) {
        wr::<usize>(hdr, core::ptr::addr_of_mut!(VEC_BUF) as usize);
        wr::<u32>(hdr + 0x0c, new_cap as u32);
    }

    unsafe extern "win64" fn mock_6df0(_a1: u64, _a2: u64, _a3: u64, cnt: u64) -> usize {
        CALL_LOG.push(0x6df0_0000 | cnt);
        core::ptr::addr_of_mut!(IDX_BUF) as usize
    }

    unsafe extern "win64" fn mock_2350(_pair: usize, out_ptr: usize) -> u64 {
        for i in 0..256 {
            IDX_BUF[i] = (i as u32).wrapping_mul(7).wrapping_add(1);
        }
        wr::<usize>(out_ptr, core::ptr::addr_of_mut!(IDX_BUF) as usize);
        72
    }

    unsafe extern "win64" fn mock_08c0(
        _p1: usize,
        out_str: usize,
        p2: u32,
        p3: u32,
        p4: u32,
        p5: u8,
    ) -> usize {
        CALL_LOG.push(0x08c0_0000 | (p2 as u64) | ((p3 as u64) << 8) | ((p4 as u64) << 16) | ((p5 as u64) << 24));
        wr::<u32>(out_str, 0x1000_0004);
        wr::<[u8; 8]>(out_str + 0x08, *b"testdir\0");
        out_str
    }

    unsafe extern "win64" fn mock_e030(fmt: usize, _cstr: usize, idx: u32) -> usize {
        let extra = if fmt == exe::ptr::<u8>(0x1_433f_07b0) as usize {
            0
        } else {
            (idx & 0xff) as u64
        };
        CALL_LOG.push(0xe030_0000 | extra);
        core::ptr::addr_of_mut!(FAKE_CSTR) as usize
    }

    unsafe extern "win64" fn mock_ab10(_s: usize) {
        CALL_LOG.push(0xab10);
    }

    unsafe extern "win64" fn mock_f850(out_span: usize, in_span: usize) {
        wr::<[u64; 2]>(out_span, rd::<[u64; 2]>(in_span));
    }

    unsafe extern "win64" fn mock_13d0(span: usize) -> u8 {
        CALL_LOG.push(0x13d0_0000 | rd::<u64>(span + 0x08));
        0
    }

    unsafe extern "win64" fn mock_5ac0(in_span: usize, out_span: usize) {
        wr::<[u64; 2]>(out_span, rd::<[u64; 2]>(in_span));
    }

    unsafe extern "win64" fn mock_e060(span: usize, flag: u64) {
        CALL_LOG.push(0xe060_0000 | rd::<u64>(span + 0x08) ^ flag);
    }

    unsafe extern "win64" fn mock_7fd0(file_ptr: usize, span: usize, mode: u32) {
        CALL_LOG.push(
            0x7fd0_0000
                | (rd::<u32>(file_ptr + 0x08) as u64)
                | ((rd::<u64>(span + 0x08) & 0xff) << 32)
                | ((mode as u64) << 40),
        );
    }

    unsafe extern "win64" fn mock_57b0(_stream: usize, val_ptr: usize) {
        CALL_LOG.push(0x57b0_0000 | (rd::<u32>(val_ptr) as u64));
    }

    unsafe extern "win64" fn mock_vfunc_88(_stream: usize, val_ptr: usize) {
        CALL_LOG.push(0x8800_0000 | rd::<u64>(val_ptr));
    }

    unsafe extern "win64" fn mock_8740(_file: usize, _data: usize, bytes: u64) {
        CALL_LOG.push(0x8740_0000 | bytes);
    }

    unsafe extern "win64" fn mock_8af0(_file: usize) {
        CALL_LOG.push(0x8af0);
    }

    let vtbl_e9e0 = exe::ptr::<u8>(0x1_4340_e9e0) as usize;
    let fn_88_va = rd::<u64>(vtbl_e9e0 + 0x88);

    let _m1 = crate::difftest::Mock::new(0x1_4221_7490, mock_7490 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4221_70a0, mock_70a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4062_ee30, mock_ee30 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4221_6df0, mock_6df0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4221_2350, mock_2350 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4222_08c0, mock_08c0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_423f_e030, mock_e030 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4249_13d0, mock_13d0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_40f3_5ac0, mock_5ac0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4248_e060, mock_e060 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4249_7fd0, mock_7fd0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4249_57b0, mock_57b0 as *const ())?;
    let _m15 = crate::difftest::Mock::new(fn_88_va, mock_vfunc_88 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4249_8740, mock_8740 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4249_8af0, mock_8af0 as *const ())?;

    let glob_b8 = exe::ptr::<usize>(0x1_4397_feb8) as usize;
    let glob_c0 = exe::ptr::<usize>(0x1_4397_fec0) as usize;
    let glob_c8 = exe::ptr::<u64>(0x1_4397_fec8) as usize;
    let saved_b8 = rd::<usize>(glob_b8);
    let saved_c0 = rd::<usize>(glob_c0);
    let saved_c8 = rd::<u64>(glob_c8);

    let run = |f: F| -> (Vec<u64>, [u32; 32], [u32; 16], u64, Vec<u64>) {
        CALL_LOG.clear();
        VEC_BUF = [0; 128];
        IDX_BUF = [0; 256];

        let mut angles_radii: [f64; 4] = [0.5, 2.5, 1.0, 3.5];
        let mut b8_obj = [0u64; 16];
        let b8_p = b8_obj.as_mut_ptr() as usize;
        wr::<i32>(b8_p + 0x20, 2);
        wr::<i32>(b8_p + 0x24, 2);
        wr::<usize>(b8_p + 0x58, angles_radii.as_mut_ptr() as usize);
        wr::<u32>(b8_p + 0x60, 4);

        wr::<usize>(glob_b8, b8_p);
        wr::<usize>(glob_c0, 0);
        wr::<u64>(glob_c8, 0);

        let mut p1 = [0u64; 64];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1_p + 0x188, 10);

        let mut p6_arr: [u64; 2] = [0x1111_0000, 0];
        let mut p7_arr: [u64; 2] = [0, 0];
        let mut p8_arr: [u64; 2] = [0, 0];
        let p6_base = p6_arr.as_mut_ptr() as usize;
        let p7_base = p7_arr.as_mut_ptr() as usize;
        let p8_base = p8_arr.as_mut_ptr() as usize;

        f(
            p1_p,
            2,
            3,
            4,
            1,
            (&p6_base as *const usize) as usize,
            (&p7_base as *const usize) as usize,
            (&p8_base as *const usize) as usize,
            1,
        );

        wr::<usize>(glob_b8, saved_b8);
        wr::<usize>(glob_c0, saved_c0);
        wr::<u64>(glob_c8, saved_c8);

        // Clear pointer at p1 + 0xc8 (populated by mock_ee30)
        wr::<usize>(p1_p + 0xc8, 0);

        let mut vec_out = [0u32; 32];
        vec_out.copy_from_slice(&VEC_BUF[..32]);
        let mut idx_out = [0u32; 16];
        idx_out.copy_from_slice(&IDX_BUF[..16]);

        (p1.to_vec(), vec_out, idx_out, p8_arr[0], CALL_LOG.clone())
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14221fc20");
    Ok(1)
}
