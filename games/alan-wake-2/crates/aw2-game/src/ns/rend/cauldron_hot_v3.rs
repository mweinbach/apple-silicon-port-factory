//! Decompiled hot functions shard V3 (`0x141acdfd0..0x141fce3b0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn rstring_view(s: usize) -> [u64; 2] {
    let hdr = rd::<u32>(s);
    let b3 = hdr >> 24;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (hdr & 0x00ff_ffff) << 3;
    let ptr = if len_bits > cap_bits {
        rd::<usize>(s + 0x08)
    } else {
        s.wrapping_add(0x08)
    };
    let len = rd::<u32>(s + 0x04) as u64;
    [ptr as u64, len]
}

/// `unknown_141acdfd0`
/// Original: 0x141acdfd0 (1491 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141acdfd0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) {
    let idx_30 = rd::<u64>(param_1 + 0x30) as usize;
    let rdi = rd::<usize>(param_1 + 0x20).wrapping_add(idx_30.wrapping_mul(0x104));
    if rd::<u8>(param_4 + 0x10c) != 0 && rd::<u8>(param_2 + 0x350) != 0 {
        let r14 = rd::<usize>(param_1).wrapping_add(idx_30.wrapping_mul(0x630));
        if rd::<i32>(r14 + 0x620) != -1 {
            let mut local_c8 = [0u64; 9];
            let lc8_ptr = local_c8.as_mut_ptr() as usize;
            if rd::<u8>(param_2 + 0x501) != 0 {
                let copy7 = rd::<[u64; 7]>(param_1);
                wr::<[u64; 7]>(lc8_ptr, copy7);
                exe::call_va!(0x1_41ad_7d00, (lc8_ptr));
            }

            let rsi = rd::<usize>(r14 + 0x90);
            let mut view_d7b8 = rstring_view(exe::ptr::<u8>(0x1_43a2_d7b8) as usize);
            let mut local_res8: u64 = 0;
            exe::call_va!(
                0x1_401e_8cc0,
                (
                    (&mut local_res8 as *mut u64) as usize,
                    view_d7b8.as_mut_ptr() as usize,
                    param_5
                )
            );
            exe::call_va!(0x1_401e_8c00, (param_5, lc8_ptr, local_res8));

            let vec_base = rd::<usize>(lc8_ptr);
            let vec_cnt = rd::<usize>(lc8_ptr + 0x10);
            let vec_end = vec_base.wrapping_add(vec_cnt.wrapping_mul(0x140));
            let found_ptr = exe::call_va!(0x1_41df_ae90, (lc8_ptr, vec_end) -> usize);
            let g_d650 = exe::ptr::<u8>(0x1_43a2_d650) as usize;
            let bl = exe::call_va!(0x1_401e_6ff0, (found_ptr, g_d650) -> u8);
            exe::call_va!(0x1_4240_ab10, (lc8_ptr));

            let flag_98 = if rd::<u8>(rsi + 0x301) == 0
                || (bl != 0 && rd::<u8>(param_3 + 0x01) == 0)
                || rd::<u8>(rsi + 0x307) == 0
            {
                0u8
            } else {
                let eq_45c =
                    exe::call_va!(0x1_401e_6ff0, (r14 + 0x48, param_2 + 0x45c) -> u8);
                if eq_45c == 0 || rd::<u8>(rsi + 0x306) == 0 {
                    0u8
                } else {
                    let mode_50 = rd::<u32>(rsi + 0x50);
                    if mode_50 == 3 || (mode_50 == 4 && rd::<u8>(param_3 + 0x01) == 0) {
                        1u8
                    } else {
                        0u8
                    }
                }
            };
            wr::<u8>(rdi + 0x98, flag_98);

            let flag_9a = if rd::<u8>(rsi + 0x303) == 0 || bl != 0 {
                1u8
            } else {
                0u8
            };
            wr::<u8>(rdi + 0x9a, flag_9a);
            wr::<u8>(rdi + 0x9b, u8::from(rd::<u8>(rsi + 0x303) == 0));

            let c_9870 = exe::call_va!(0x1_41a3_9870, () -> u8);
            if c_9870 != 0
                && rd::<u32>(param_3 + 0x08) != 0
                && rd::<u8>(param_3 + 0x01) == 0
                && rd::<u8>(rsi + 0x301) != 0
            {
                wr::<u8>(rdi + 0x98, 0);
            }
            if rd::<u8>(rdi + 0x98) == 0 {
                return;
            }

            wr::<u8>(rdi + 0xc7, 0);
            if rd::<u8>(r14 + 0xa3) != 0 {
                wr::<u64>(rdi + 0xb8, rd::<u64>(param_2 + 0x78));
                if rd::<u8>(rdi + 0xc0) == 0 {
                    wr::<u8>(rdi + 0xc0, 1);
                }
                wr::<u64>(rdi + 0xc8, rd::<u64>(param_2 + 0x80));
                if rd::<u8>(rdi + 0xd0) == 0 {
                    wr::<u8>(rdi + 0xd0, 1);
                }
            } else {
                wr::<u8>(rdi + 0xc0, 0);
                wr::<u8>(rdi + 0xd0, 0);
            }

            if rd::<u8>(rdi + 0x98) == 0 {
                return;
            }

            let val_1e8 = rd::<u64>(param_4 + 0x1e8);
            if (val_1e8 & 0x3fff) == 0 {
                return;
            }
            if (val_1e8 & 0x3fff_ffff_ffff_c000) == 0 {
                return;
            }

            let mut rbx = val_1e8;
            let mut r11 = exe::call_va!(0x1_41ad_d620, (param_2) -> usize);
            if r11 != 0 {
                if rd::<u8>(r11 + 0x60) != 0 && rd::<u32>(r11 + 0x57c) != 0 {
                    let mut view_578 = rstring_view(r11 + 0x578);
                    let mut view_64 = rstring_view(r11 + 0x64);
                    let l8 = exe::call_va!(
                        0x1_41ad_cb20,
                        (param_2, view_64.as_mut_ptr() as usize) -> usize
                    );
                    if l8 != 0 {
                        let l8_2 = exe::call_va!(
                            0x1_41ac_8bc0,
                            (l8, view_578.as_mut_ptr() as usize) -> usize
                        );
                        if l8_2 != 0 {
                            rbx = rd::<u64>(l8_2 + 0xc0);
                            r11 = l8_2;
                        }
                    }
                }

                if rd::<u32>(r11 + 0x57c) == 0
                    && rd::<u32>(rsi + 0x50) == 3
                    && rd::<u8>(rsi + 0x304) == 0
                {
                    wr::<u8>(rdi + 0x98, 1);
                    if rd::<u32>(rsi + 0x57c) == 0 {
                        return;
                    }

                    let mut local_str = [0u64; 9];
                    let lstr_ptr = local_str.as_mut_ptr() as usize;
                    wr::<u64>(lstr_ptr, 0x0800_0008);
                    wr::<u8>(lstr_ptr + 0x08, 0);

                    if lstr_ptr != rsi.wrapping_add(0x578) {
                        let mut view_rsi_578 = rstring_view(rsi + 0x578);
                        let vlen = view_rsi_578[1];
                        exe::call_va!(
                            0x1_4240_9b50,
                            (lstr_ptr, view_rsi_578.as_mut_ptr() as usize, 0u64, vlen)
                        );
                    }

                    while rd::<u32>(lstr_ptr + 0x04) != 0 {
                        let mut view_local = rstring_view(lstr_ptr);
                        let mut view_rsi_64 = rstring_view(rsi + 0x64);
                        let l16 = exe::call_va!(
                            0x1_41ad_cb20,
                            (param_2, view_rsi_64.as_mut_ptr() as usize) -> usize
                        );
                        if l16 == 0 {
                            break;
                        }
                        let l16_2 = exe::call_va!(
                            0x1_41ac_8bc0,
                            (l16, view_local.as_mut_ptr() as usize) -> usize
                        );
                        if l16_2 == 0 {
                            break;
                        }
                        if rd::<u64>(l16_2 + 0xc0) == rbx {
                            wr::<u8>(rdi + 0x98, 0);
                            break;
                        }
                        if lstr_ptr != l16_2.wrapping_add(0x578) {
                            let mut view_next = rstring_view(l16_2 + 0x578);
                            let nlen = view_next[1];
                            exe::call_va!(
                                0x1_4240_9b50,
                                (lstr_ptr, view_next.as_mut_ptr() as usize, 0u64, nlen)
                            );
                        }
                    }

                    exe::call_va!(0x1_4240_ab10, (lstr_ptr));
                    return;
                }
            }
        }
    }
    wr::<u8>(rdi + 0x98, 0);
}

/// `unknown_141da2730`
/// Original: 0x141da2730 (1505 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141da2730(param_1: usize) {
    exe::call_va!(0x1_41c6_9d30, (param_1 + 0xf0));
    exe::call_va!(0x1_41c6_9d30, (param_1 + 0x160));
    exe::call_va!(0x1_41c6_9d30, (param_1 + 0x200));
    exe::call_va!(0x1_41c6_9d30, (param_1 + 0x240));

    let mut xmm6 = 0.0f32;
    let xmm9_62 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e78) as usize);
    let xmm7_1 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm8_63 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e7c) as usize);

    for i in 0..64i32 {
        wr::<f32>(param_1 + 0x3f8 + (i as usize) * 4, xmm6);
        if (i as u32) < 0x3f {
            let xmm1_arg = (i as f32) / xmm9_62;
            let rcx_110 = rd::<usize>(param_1 + 0x110);
            let mode_118 = rd::<u32>(param_1 + 0x118);
            let raw = if mode_118 == 0 {
                xmm7_1
            } else if mode_118 == 1 {
                rd::<f32>(rcx_110 + 0x04)
            } else {
                let (_, x0) = crate::simd_ops::call_va8_xmm(
                    0x1_41c6_a020,
                    (param_1 + 0xf0) as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, xmm1_arg.to_bits() as u128, 0, 0],
                );
                f32::from_bits(x0 as u32)
            };
            let max0 = if raw > 0.0f32 { raw } else { 0.0f32 };
            let clamped = if max0 < xmm7_1 { max0 } else { xmm7_1 };
            let step = ((clamped * rd::<f32>(param_1 + 0xe8)) * rd::<f32>(param_1 + 0x124)) / xmm8_63;
            xmm6 += step;
        }
    }

    let mut vec_40: [u64; 2] = [0, 0];
    exe::call_va!(0x1_402c_0a70, (vec_40.as_mut_ptr() as usize, param_1 + 0x298));
    let mut vec_50: [u64; 2] = [0, 0];
    exe::call_va!(0x1_4022_5740, (vec_50.as_mut_ptr() as usize, param_1 + 0x2a8));

    let cnt_40 = (vec_40[1] as u32) as u64;
    let ptr_40 = vec_40[0];
    let mut slot_a0: u64 = (cnt_40 << 4).wrapping_add(ptr_40);
    let mut slot_a8: u64 = ptr_40;
    let r9_b0 = (slot_a0 & 0xff) as u32;
    exe::call_va!(
        0x1_40e6_8b90,
        (
            (&mut slot_a8 as *mut u64) as usize,
            (&mut slot_a0 as *mut u64) as usize,
            cnt_40,
            r9_b0
        )
    );

    let cnt_50 = (vec_50[1] as u32) as u64;
    let ptr_50 = vec_50[0];
    slot_a0 = ptr_50.wrapping_add(cnt_50.wrapping_mul(8));
    slot_a8 = ptr_50;
    let r9_b1 = (slot_a0 & 0xff) as u32;
    exe::call_va!(
        0x1_40e6_8f00,
        (
            (&mut slot_a8 as *mut u64) as usize,
            (&mut slot_a0 as *mut u64) as usize,
            cnt_50,
            r9_b1
        )
    );

    wr::<u32>(param_1 + 0xbf8, 0);
    for i in 0..64i32 {
        let xmm1_arg = (i as f32) / xmm8_63;
        let rcx_180 = rd::<usize>(param_1 + 0x180);
        let mode_188 = rd::<u32>(param_1 + 0x188);
        let val = if mode_188 == 0 {
            xmm7_1
        } else if mode_188 == 1 {
            rd::<f32>(rcx_180 + 0x04)
        } else {
            let (_, x0) = crate::simd_ops::call_va8_xmm(
                0x1_41c6_a020,
                (param_1 + 0x160) as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, xmm1_arg.to_bits() as u128, 0, 0],
            );
            f32::from_bits(x0 as u32)
        };
        wr::<f32>(param_1 + 0x4f8 + (i as usize) * 4, val);
    }

    let xmm9_31 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e44) as usize);
    let xmm10_deg = rd::<f32>(exe::ptr::<f32>(0x1_4342_6514) as usize);
    for i in 0..32i32 {
        let dst = param_1 + 0x5fc + (i as usize) * 16;
        let xmm8_t = (i as f32) / xmm9_31;
        let rcx_220 = rd::<usize>(param_1 + 0x220);
        let mode_228 = rd::<u32>(param_1 + 0x228);
        let v_220 = if mode_228 == 0 {
            xmm7_1
        } else if mode_228 == 1 {
            rd::<f32>(rcx_220 + 0x04)
        } else {
            let (_, x0) = crate::simd_ops::call_va8_xmm(
                0x1_41c6_a020,
                (param_1 + 0x200) as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, xmm8_t.to_bits() as u128, 0, 0],
            );
            f32::from_bits(x0 as u32)
        };
        wr::<f32>(dst - 0x04, v_220);

        let rcx_260 = rd::<usize>(param_1 + 0x260);
        let mode_268 = rd::<u32>(param_1 + 0x268);
        let v_260 = if mode_268 == 0 {
            0.0f32
        } else if mode_268 == 1 {
            rd::<f32>(rcx_260 + 0x04)
        } else {
            let (_, x0) = crate::simd_ops::call_va8_xmm(
                0x1_41c6_a020,
                (param_1 + 0x240) as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, xmm8_t.to_bits() as u128, 0, 0],
            );
            f32::from_bits(x0 as u32)
        };
        wr::<f32>(dst, v_260 * xmm10_deg);
        wr::<u64>(dst + 0x04, 0);

        let cur_max = rd::<f32>(param_1 + 0xbf8);
        let next_max = if cur_max > v_220 { cur_max } else { v_220 };
        wr::<f32>(param_1 + 0xbf8, next_max);
    }

    let mut def_vec3: [f32; 3] = [xmm7_1, xmm7_1, xmm7_1];
    wr::<f32>((&mut slot_a0 as *mut u64) as usize, xmm7_1);
    let span_color: [u64; 2] = [(vec_40[1] as u32) as u64, vec_40[0]];
    let span_alpha: [u64; 2] = [(vec_50[1] as u32) as u64, vec_50[0]];
    let mut out_vec3: [u64; 2] = span_alpha;

    let vtbl_4e8 = exe::ptr::<u8>(0x1_432f_c4e8) as usize as u64;
    let vtbl_520 = exe::ptr::<u8>(0x1_432f_c520) as usize as u64;
    let vtbl_5c8 = exe::ptr::<u8>(0x1_432f_c5c8) as usize as u64;
    let vtbl_600 = exe::ptr::<u8>(0x1_432f_c600) as usize as u64;

    let mut fn_80 = [0u64; 8];
    let mut fn_40 = [0u64; 8];
    let fn_80_ptr = fn_80.as_mut_ptr() as usize;
    let fn_40_ptr = fn_40.as_mut_ptr() as usize;

    for i in 0..32i32 {
        let xmm6_t = (i as f32) / xmm9_31;
        fn_80[0] = vtbl_4e8;
        fn_80[7] = fn_80_ptr as u64;
        fn_40[0] = vtbl_520;
        fn_40[7] = fn_40_ptr as u64;
        let mut span_30 = span_color;
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_41da_1a60,
            out_vec3.as_mut_ptr() as usize as u64,
            0,
            span_30.as_mut_ptr() as usize as u64,
            fn_40_ptr as u64,
            fn_80_ptr as u64,
            def_vec3.as_mut_ptr() as usize as u64,
            0,
            0,
            [0, xmm6_t.to_bits() as u128, 0, 0],
        );

        fn_40[0] = vtbl_5c8;
        fn_40[7] = fn_40_ptr as u64;
        fn_80[0] = vtbl_600;
        fn_80[7] = fn_80_ptr as u64;
        span_30 = span_alpha;
        let (_, alpha_xmm0) = crate::simd_ops::call_va8_xmm(
            0x1_41da_1810,
            0,
            span_30.as_mut_ptr() as usize as u64,
            fn_80_ptr as u64,
            fn_40_ptr as u64,
            (&mut slot_a0 as *mut u64) as usize as u64,
            0,
            0,
            0,
            [xmm6_t.to_bits() as u128, 0, 0, 0],
        );
        let alpha = f32::from_bits(alpha_xmm0 as u32);
        let scale_290 = rd::<f32>(param_1 + 0x290);
        let out_ptr = out_vec3.as_ptr() as usize;
        let c0 = rd::<f32>(out_ptr) * scale_290;
        let c1 = scale_290 * rd::<f32>(out_ptr + 0x04);
        let c2 = scale_290 * rd::<f32>(out_ptr + 0x08);
        wr::<[f32; 4]>(param_1 + 0x7f8 + (i as usize) * 16, [c0, c1, c2, alpha]);
    }

    exe::call_va!(0x1_401d_e830, (param_1 + 0xf0));
    exe::call_va!(0x1_401d_e830, (param_1 + 0x160));
    exe::call_va!(0x1_401d_e830, (param_1 + 0x200));
    exe::call_va!(0x1_401d_e830, (param_1 + 0x240));

    let f_1fc = rd::<f32>(param_1 + 0x1fc);
    let f_1f8 = rd::<f32>(param_1 + 0x1f8);
    let xmm5 = f_1f8 - f_1fc;
    let xmm6_2 = f_1fc + f_1fc;

    let mask_abs = !rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
    let abs_238 = f32::from_bits(rd::<u32>(param_1 + 0x238) & mask_abs);
    let lim_180 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ed8) as usize);
    let xmm4_clamped = if abs_238 < lim_180 { abs_238 } else { lim_180 };
    let sign_xor = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    let neg_clamped = f32::from_bits(xmm4_clamped.to_bits() ^ sign_xor);
    let xmm7_rot = neg_clamped * xmm10_deg;
    let xmm4_rot = (xmm4_clamped + xmm4_clamped) * xmm10_deg;

    let f_2c0 = rd::<f32>(param_1 + 0x2c0);
    let f_2bc = rd::<f32>(param_1 + 0x2bc);
    let xmm8_sub = f_2bc - f_2c0;
    let xmm10_add = f_2c0 + f_2c0;

    let c00_m1 = rd::<i32>(param_1 + 0xc00).wrapping_sub(1);
    let max_idx = if c00_m1 > 0 { c00_m1 } else { 0 };
    let xmm11_cnt = max_idx as f32;

    for i in 0..32i32 {
        let dst = param_1 + 0x9fc + (i as usize) * 16;
        let xmm3 = (i as f32) / xmm9_31;
        wr::<f32>(dst - 0x04, xmm3 * xmm6_2 + xmm5);
        wr::<f32>(dst, xmm3 * xmm4_rot + xmm7_rot);
        wr::<f32>(dst + 0x04, xmm3 * xmm10_add + xmm8_sub);
        let prod = xmm3 * xmm11_cnt;
        let rounded_bits = crate::simd_ops::vroundss(0, prod.to_bits(), 0xc) as u32;
        let rounded_i32 = crate::simd_ops::cvt_f32_i32(f32::from_bits(rounded_bits)) as i32;
        wr::<f32>(dst + 0x08, rounded_i32 as f32);
    }

    exe::call_va!(0x1_401f_2bc0, (vec_50.as_mut_ptr() as usize));
    exe::call_va!(0x1_401f_2bc0, (vec_40.as_mut_ptr() as usize));
}

/// `unknown_141dc0630`
/// Original: 0x141dc0630 (678 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141dc0630(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let mut count_u32: u32 = 0;
    let vtbl_1 = rd::<usize>(param_1);
    exe::call_va!(
        rd::<usize>(vtbl_1 + 0x48),
        (param_1, (&mut count_u32 as *mut u32) as usize)
    );

    let rbx_cnt = (count_u32 as i32) as isize as usize;
    let mut r13: usize = 0;
    let rax_cnt: usize = if rbx_cnt != 0 {
        r13 = exe::call_va!(0x1_4241_c0a0, (rbx_cnt.wrapping_mul(8), 4u32) -> usize);
        for k in 0..rbx_cnt {
            wr::<u64>(r13.wrapping_add(k * 8), 0);
        }
        rbx_cnt as u32 as usize
    } else {
        0
    };

    let r12 = r13.wrapping_add(rax_cnt.wrapping_mul(8));
    let mut cur = r13;
    while cur != r12 {
        let vtbl = rd::<usize>(param_1);
        exe::call_va!(rd::<usize>(vtbl + 0x40), (param_1, cur));
        let vtbl = rd::<usize>(param_1);
        exe::call_va!(rd::<usize>(vtbl + 0x40), (param_1, cur.wrapping_add(4)));
        cur = cur.wrapping_add(8);
    }

    let mut slot_60: u64 = 0;
    if rd::<usize>(param_3 + 0x38) != 0 && r13 != r12 {
        let mut r15_total: u64 = 0;
        let mut p = r13;
        while p != r12 {
            let type_id = rd::<u32>(p);
            if type_id != 0 {
                let ti = exe::call_va!(0x1_424b_14c0, (type_id) -> usize);
                let ti_size = exe::call_va!(rd::<usize>(rd::<usize>(ti) + 0x10), (ti) -> u64);
                let ti_align = exe::call_va!(rd::<usize>(rd::<usize>(ti) + 0x18), (ti) -> u64);
                let elem_cnt = rd::<u32>(p + 0x04) as u64;
                let aligned =
                    r15_total.wrapping_sub(1).wrapping_add(ti_align) & !ti_align.wrapping_sub(1);
                r15_total = elem_cnt.wrapping_mul(ti_size).wrapping_add(aligned);
            }
            p = p.wrapping_add(8);
        }
        if r15_total != 0 {
            slot_60 = r15_total;
            let fn3 = rd::<usize>(param_3 + 0x38);
            if fn3 == 0 {
                exe::call_iat!(0x1_4264_5878, ());
                return;
            }
            exe::call_va!(
                rd::<usize>(rd::<usize>(fn3) + 0x10),
                (fn3, (&mut slot_60 as *mut u64) as usize)
            );
        }
    }

    let mut rdi = r13;
    while rdi != r12 {
        let mut r15_buf: usize = 0;
        let mut r14_cnt: u32 = 0;
        let ebx_cnt = rd::<u32>(rdi + 0x04);
        if ebx_cnt != 0 {
            let byte_len = (ebx_cnt as u64).wrapping_mul(8);
            r15_buf = exe::call_va!(0x1_4241_c0a0, (byte_len, 8u32) -> usize);
            r14_cnt = ebx_cnt;
            let _ = exe::call_va!(0x1_4183_acc2, (r15_buf, 0u32, byte_len) -> usize);
        }
        let end_buf = r15_buf.wrapping_add((r14_cnt as usize).wrapping_mul(8));
        let mut p = r15_buf;
        while p != end_buf {
            let vtbl = rd::<usize>(param_1);
            exe::call_va!(rd::<usize>(vtbl), (param_1, p));
            p = p.wrapping_add(8);
        }

        wr::<u32>((&mut slot_60 as *mut u64) as usize, rd::<u32>(rdi));
        let span_len = ((end_buf.wrapping_sub(r15_buf) as isize) >> 3) as u64;
        let mut span_18: [u64; 2] = [span_len, r15_buf as u64];
        let fn2 = rd::<usize>(param_2 + 0x38);
        if fn2 == 0 {
            exe::call_iat!(0x1_4264_5878, ());
            return;
        }
        exe::call_va!(
            rd::<usize>(rd::<usize>(fn2) + 0x10),
            (
                fn2,
                param_1,
                span_18.as_mut_ptr() as usize,
                (&mut slot_60 as *mut u64) as usize
            )
        );
        if r15_buf != 0 {
            exe::call_va!(0x1_4241_c190, (r15_buf));
        }
        rdi = rdi.wrapping_add(8);
    }

    if r13 != 0 {
        exe::call_va!(0x1_4241_c190, (r13));
    }
    let fn2 = rd::<usize>(param_2 + 0x38);
    if fn2 != 0 {
        let flag = u8::from(fn2 != param_2);
        exe::call_va!(rd::<usize>(rd::<usize>(fn2) + 0x20), (fn2, flag));
        wr::<usize>(param_2 + 0x38, 0);
    }
    let fn3 = rd::<usize>(param_3 + 0x38);
    if fn3 != 0 {
        let flag = u8::from(fn3 != param_3);
        exe::call_va!(rd::<usize>(rd::<usize>(fn3) + 0x20), (fn3, flag));
        wr::<usize>(param_3 + 0x38, 0);
    }
}

/// `unknown_141e08e80`
/// Original: 0x141e08e80 (619 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141e08e80(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
) -> usize {
    let mut local_str = [0u64; 9];
    let lstr_ptr = local_str.as_mut_ptr() as usize;

    let arr_ptr = rd::<usize>(param_3);
    let arr_len = rd::<u32>(param_3 + 0x08) as u64;
    let bpl: u8 = if param_4 < arr_len {
        let elem = arr_ptr.wrapping_add((param_4 as usize).wrapping_mul(0x60));
        if rd::<u32>(elem + 0x58) == 2 {
            let hdr = (rd::<u32>(lstr_ptr) & 0xff00_0008) | 8;
            wr::<u32>(lstr_ptr, hdr);
            wr::<u8>(lstr_ptr + 0x03, 8);
            wr::<u32>(lstr_ptr + 0x04, 0);
            wr::<u8>(lstr_ptr + 0x08, 0);
            exe::call_va!(0x1_4240_aa00, (lstr_ptr, elem));
            1
        } else {
            0
        }
    } else {
        0
    };

    'fallback: {
        if bpl != 0 {
            let view = rstring_view(lstr_ptr);
            let cstr_ptr = view[0] as usize;
            let mut cstr_len: usize = 0;
            while rd::<u8>(cstr_ptr.wrapping_add(cstr_len)) != 0 {
                cstr_len += 1;
            }
            let hash = exe::call_va!(0x1_424b_16a0, (cstr_ptr, cstr_len as u64) -> u64);
            let r12 = rd::<u64>(param_2 + 0x38);
            if rd::<u64>(param_2 + 0x40) != r12 {
                let mut r9_step: u64 = 0;
                let mask = rd::<u64>(param_2 + 0x48).wrapping_sub(1);
                let mut rcx_idx = hash;
                let mut r10_tomb: u64 = u64::MAX;
                let r11_tbl = rd::<usize>(param_2 + 0x78);
                let r13_empty0 = rd::<u64>(param_2 + 0x50);

                loop {
                    rcx_idx &= mask;
                    let entry = r11_tbl.wrapping_add((rcx_idx as usize).wrapping_mul(0x28));
                    let r8_k0 = rd::<u64>(entry);
                    if r13_empty0 == r8_k0 && rd::<u64>(param_2 + 0x58) == rd::<u64>(entry + 0x08) {
                        break 'fallback;
                    }
                    if r12 != 0
                        && rd::<u64>(param_2 + 0x28) == r8_k0
                        && rd::<u64>(param_2 + 0x30) == rd::<u64>(entry + 0x08)
                    {
                        if r10_tomb == u64::MAX {
                            r10_tomb = rcx_idx;
                        }
                        r9_step = r9_step.wrapping_add(1);
                        rcx_idx = rcx_idx.wrapping_add(r9_step);
                        continue;
                    }
                    if r8_k0 == 0 && hash == rd::<u64>(entry + 0x08) {
                        break;
                    }
                    r9_step = r9_step.wrapping_add(1);
                    rcx_idx = rcx_idx.wrapping_add(r9_step);
                }

                if rcx_idx != u64::MAX {
                    let rdx_entry = r11_tbl.wrapping_add((rcx_idx as usize).wrapping_mul(0x28));
                    let end_entry =
                        r11_tbl.wrapping_add((rd::<usize>(param_2 + 0x48)).wrapping_mul(0x28));
                    if rdx_entry != end_entry {
                        let rcx_val = rdx_entry.wrapping_add(0x10);
                        if rd::<u16>(rcx_val) != 6 && rcx_val != 0 {
                            let str_ptr =
                                exe::call_va!(0x1_4021_7680, (rcx_val, param_2) -> usize);
                            let mut slen: usize = 0;
                            while rd::<u8>(str_ptr.wrapping_add(slen)) != 0 {
                                slen += 1;
                            }
                            let h0 = (rd::<u32>(param_1) & 0xff00_0008) | 8;
                            wr::<u32>(param_1, h0);
                            wr::<u8>(param_1 + 0x03, 8);
                            wr::<u32>(param_1 + 0x04, 0);
                            wr::<u8>(param_1 + 0x08, 0);
                            if slen != 0 {
                                let mgr = exe::call_va!(0x1_420e_af00, () -> u64);
                                exe::call_va!(0x1_420e_a540, (mgr, str_ptr, param_1));
                            }
                            if bpl != 0 {
                                exe::call_va!(0x1_4240_ab10, (lstr_ptr));
                            }
                            return param_1;
                        }
                    }
                }
            }
        }
    }

    let h0 = (rd::<u32>(param_1) & 0xff00_0008) | 8;
    wr::<u32>(param_1, h0);
    wr::<u8>(param_1 + 0x03, 8);
    wr::<u32>(param_1 + 0x04, 0);
    wr::<u8>(param_1 + 0x08, 0);
    let mut empty_view: [u64; 2] = [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
    exe::call_va!(
        0x1_4240_9b50,
        (
            param_1,
            empty_view.as_mut_ptr() as usize,
            0u64,
            0u64
        )
    );
    if bpl != 0 {
        exe::call_va!(0x1_4240_ab10, (lstr_ptr));
    }
    param_1
}

/// `unknown_141e5b280`
/// Original: 0x141e5b280 (315 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e5b280(param_1: usize) {
    let mut cur = rd::<usize>(param_1);
    let end = cur.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_shl(6));
    while cur != end {
        if rd::<u32>(cur + 0x20) == 2 {
            let cur_38 = rd::<u64>(cur + 0x38);
            let rem = (rd::<u32>(cur + 0x30) as u64).wrapping_sub(cur_38);
            let batch = if rem < 0x1f4 { rem } else { 0x1f4 };
            let r14 = rd::<u64>(cur + 0x10);
            let span_ptr = rd::<usize>(cur + 0x28).wrapping_add((cur_38 as usize).wrapping_mul(8));
            let span_len = if batch == u64::MAX { rem } else { batch };
            let mut span_40: [u64; 2] = [span_len, span_ptr as u64];
            let mut out_vec: [u64; 2] = [0, 0];
            let g_2b8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
            exe::call_va!(
                0x1_4242_0050,
                (
                    g_2b8,
                    out_vec.as_mut_ptr() as usize,
                    span_40.as_mut_ptr() as usize
                )
            );

            let rbx_ptr = out_vec[0] as usize;
            let count = (out_vec[1] as u32) as usize;
            let rbp_end = rbx_ptr.wrapping_add(count * 8);
            let mut p = rbx_ptr;
            while p != rbp_end {
                let item = rd::<u64>(p);
                if item != 0 {
                    exe::call_va!(0x1_4242_50f0, (r14, item));
                }
                p = p.wrapping_add(8);
            }
            let rbx_free = out_vec[0] as usize;
            if rbx_free != 0 {
                exe::call_va!(0x1_4241_c190, (rbx_free));
            }

            let next_38 = rd::<u64>(cur + 0x38).wrapping_add(batch);
            wr::<u64>(cur + 0x38, next_38);
            if next_38 >= (rd::<u32>(cur + 0x30) as u64) {
                exe::call_va!(0x1_4242_5830, (rd::<u64>(cur + 0x10)));
                wr::<u32>(cur + 0x20, 3);
            }
        }
        cur = cur.wrapping_add(0x40);
    }
}

/// `unknown_141eb0210`
/// Original: 0x141eb0210 (845 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eb0210(param_1: usize) {
    let g_f58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let r14 = rd::<u64>(param_1 + 0x18);
    let r9_bytes = r14.wrapping_mul(0x18);
    let rbp_size = r9_bytes.wrapping_add(4);

    let (hdr_ptr, data_start, pkt_len): (usize, usize, u64) = if rbp_size < 0x00ff_ffc0 {
        let ring = rd::<usize>(g_f58 + 0x168);
        exe::call_iat!(0x1_4264_55a8, (ring + 0x18));
        let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        let aligned_payload = (rbp_size.wrapping_add(7) & !7u64) as usize;
        let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
        let mut next_wr = wr_ptr.wrapping_add(8).wrapping_add(aligned_payload);
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
            next_wr = wr_ptr.wrapping_add(8).wrapping_add(aligned_payload);
        }
        while wr_ptr <= rd_ptr
            && rd_ptr < next_wr
            && crate::simd_ops::rd_acq_u32(ring + 0x30)
                != crate::simd_ops::rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        }
        crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
        crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
        exe::call_iat!(0x1_4264_5598, (ring + 0x18));

        let rcx_hdr = wr_ptr.wrapping_add(8);
        wr::<u32>(rcx_hdr, ((r14 as u32) << 9) | 0x5a);
        let start = rcx_hdr.wrapping_add(4);
        (rcx_hdr, start, rbp_size)
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
            && crate::simd_ops::rd_acq_u32(ring + 0x30)
                != crate::simd_ops::rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        }
        crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
        crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
        exe::call_iat!(0x1_4264_5598, (ring + 0x18));

        let rcx_hdr = wr_ptr.wrapping_add(8);
        wr::<u32>(rcx_hdr + 0x04, 0x5a);
        wr::<usize>(rcx_hdr + 0x0c, alloc_ptr);
        wr::<u32>(rcx_hdr, ((r14 as u32) << 9) | 1);
        (rcx_hdr, alloc_ptr, 0x14)
    };

    let mut out_cur = data_start;
    if r14 != 0 {
        let arr_20 = rd::<usize>(param_1 + 0x20);
        let arr_28 = rd::<usize>(param_1 + 0x28);
        let mut r9_off: usize = 0;
        let mut r10_off: usize = 0;
        for _ in 0..r14 {
            let rcx_item = arr_28.wrapping_add(r9_off);
            let r8_idx = rd::<u8>(rcx_item) as usize;
            let r8_sub = rcx_item.wrapping_add(r8_idx * 16);
            let rax_vec = rd::<usize>(r8_sub + 0x08);
            let vec_beg = rd::<usize>(rax_vec);
            let vec_len = rd::<u32>(rax_vec + 0x08) as usize;
            if vec_beg != vec_beg.wrapping_add(vec_len.wrapping_mul(0x30)) {
                let rdx_dst = out_cur;
                out_cur = out_cur.wrapping_add(0x18);
                wr::<u32>(
                    rdx_dst,
                    rd::<u32>(arr_20.wrapping_add(r10_off).wrapping_add(0x10)),
                );
                wr::<u64>(rdx_dst + 0x08, 0);
                wr::<u64>(rdx_dst + 0x10, 0);
                let ref_ctrl = rd::<usize>(r8_sub + 0x10);
                if ref_ctrl != 0 {
                    let _ = crate::simd_ops::atomic_xadd_u32((ref_ctrl + 0x08) as u64, 1);
                }
                wr::<u64>(rdx_dst + 0x08, rd::<u64>(r8_sub + 0x08));
                wr::<u64>(rdx_dst + 0x10, rd::<u64>(r8_sub + 0x10));
            }
            r9_off = r9_off.wrapping_add(0x28);
            r10_off = r10_off.wrapping_add(0x18);
        }
    }

    let written_items = ((out_cur.wrapping_sub(data_start) as isize) / 0x18) as u32;
    let g_f58_2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let new_hdr = (rd::<u32>(hdr_ptr) & 0x1ff) | (written_items << 9);
    wr::<u32>(hdr_ptr, new_hdr);
    let ring2 = rd::<usize>(g_f58_2 + 0x168);
    crate::simd_ops::wr_rel_u64(hdr_ptr - 8, pkt_len);
    let _ = crate::simd_ops::atomic_xadd_u32((ring2 + 0x30) as u64, 1);
    if rd::<u8>(g_f58_2 + 0x198) != 0 {
        let _ = exe::call_iat!(
            0x1_4264_5580,
            (rd::<u64>(g_f58_2 + 0x1a0), 1u32, 0usize) -> i32
        );
    }
}

/// `unknown_141f573a0`
/// Original: 0x141f573a0 (212 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f573a0(param_1: usize) {
    let val_698 = rd::<u32>(exe::ptr::<u32>(0x1_4387_9698) as usize);
    let rcx_arg = rd::<u64>(param_1);
    let g_258 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4258) as usize);
    let flag_38 = rd::<u8>(g_258 + 0x38);
    let xmm1_bits = if flag_38 == 0 { val_698 } else { 0 };
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_41f5_7ce0,
        rcx_arg,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, xmm1_bits as u128, 0, 0],
    );

    let rdi = exe::call_va!(0x1_4248_a640, () -> usize);
    let mut rbx = rd::<usize>(rdi + 0x50);
    let rsi = rbx.wrapping_add((rd::<usize>(rdi + 0x38)).wrapping_shl(4));
    if rbx != rsi {
        let empty_k = rd::<u32>(rdi + 0x40);
        while rbx != rsi {
            let k = rd::<u32>(rbx);
            if empty_k != k && (rd::<u64>(rdi + 0x28) == 0 || rd::<u32>(rdi + 0x24) != k) {
                break;
            }
            rbx = rbx.wrapping_add(0x10);
        }
        while rbx != rsi {
            let obj = rd::<usize>(rbx + 0x08);
            let vtbl = rd::<usize>(obj);
            exe::call_va!(rd::<usize>(vtbl + 0x28), (obj));
            rbx = rbx.wrapping_add(0x10);
            if rbx == rsi {
                break;
            }
            let empty_k2 = rd::<u32>(rdi + 0x40);
            while rbx != rsi {
                let k = rd::<u32>(rbx);
                if empty_k2 != k && (rd::<u64>(rdi + 0x28) == 0 || rd::<u32>(rdi + 0x24) != k) {
                    break;
                }
                rbx = rbx.wrapping_add(0x10);
            }
        }
    }

    let g_f58 = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff58) as usize);
    exe::call_va!(0x1_410d_2dd0, (g_f58, 0u32));
}

/// `unknown_141fce3b0`
/// Original: 0x141fce3b0 (187 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fce3b0(param_1: usize) {
    let mut rbx = rd::<usize>(param_1);
    let mut len = rd::<u32>(param_1 + 0x08);
    if rbx != rbx.wrapping_add((len as usize) * 0x20) {
        let mut rbp = rbx.wrapping_add(0x20);
        loop {
            if rd::<u8>(rbx) == 0 {
                let dst_len = rd::<u32>(param_1 + 0x18) as u64;
                let dst_cap = rd::<u32>(param_1 + 0x1c) as u64;
                if (dst_len << 5) == (dst_cap << 5) {
                    exe::call_va!(0x1_405e_9720, (param_1 + 0x10, dst_len.wrapping_add(1)));
                }
                let dst_idx = rd::<u32>(param_1 + 0x18) as usize;
                let dst_ptr = rd::<usize>(param_1 + 0x10).wrapping_add(dst_idx * 0x20);
                let item = rd::<[u64; 4]>(rbx);
                wr::<[u64; 4]>(dst_ptr, item);
                wr::<u32>(param_1 + 0x18, rd::<u32>(param_1 + 0x18).wrapping_add(1));

                let cur_len = rd::<u32>(param_1 + 0x08) as usize;
                let move_bytes =
                    (cur_len * 0x20).wrapping_sub(rbp).wrapping_add(rd::<usize>(param_1));
                let _ = exe::call_va!(0x1_4183_acc8, (rbx, rbp, move_bytes) -> usize);
                let new_len = rd::<u32>(param_1 + 0x08).wrapping_sub(1);
                wr::<u32>(param_1 + 0x08, new_len);
                len = rd::<u32>(param_1 + 0x08);
            } else {
                rbx = rbx.wrapping_add(0x20);
                rbp = rbp.wrapping_add(0x20);
            }
            let end = rd::<usize>(param_1).wrapping_add((len as usize) * 0x20);
            if rbx == end {
                break;
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141acdfd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut NODE_INIT_PTR: usize = 0;
    static mut NODE_A_PTR: usize = 0;
    static mut NODE_B_PTR: usize = 0;

    unsafe extern "win64" fn mock_7d00(p: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(p + 0x08));
    }
    unsafe extern "win64" fn mock_8cc0(out: usize, _view: usize, p5: u64) {
        wr::<u64>(out, p5 ^ 0x5555);
    }
    unsafe extern "win64" fn mock_8c00(p5: u64, lc8: usize, res8: u64) {
        wr::<u64>(lc8, 0x2000);
        wr::<u64>(lc8 + 0x10, 3);
        LOG_SUM = LOG_SUM.wrapping_add(p5 ^ res8);
    }
    unsafe extern "win64" fn mock_ae90(lc8: usize, end: usize) -> usize {
        let base = rd::<usize>(lc8);
        end.wrapping_sub(base)
    }
    unsafe extern "win64" fn mock_6ff0(a: usize, _b: usize) -> u8 {
        if a == 3 * 0x140 {
            0
        } else {
            1
        }
    }
    unsafe extern "win64" fn mock_ab10(_p: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_9870() -> u8 {
        0
    }
    unsafe extern "win64" fn mock_d620(_p2: usize) -> usize {
        NODE_INIT_PTR
    }
    unsafe extern "win64" fn mock_cb20(_p2: usize, view: usize) -> usize {
        let len = rd::<u64>(view + 0x08);
        len as usize + 0x100
    }
    unsafe extern "win64" fn mock_8bc0(_l8: usize, view: usize) -> usize {
        let str_ptr = rd::<usize>(view);
        let first_byte = rd::<u8>(str_ptr);
        match first_byte {
            b'A' => NODE_A_PTR,
            b'B' => NODE_B_PTR,
            _ => 0,
        }
    }
    unsafe extern "win64" fn mock_9b50(dst_str: usize, view: usize, _r8: u64, _r9: u64) {
        let src_ptr = rd::<usize>(view);
        let src_len = rd::<u32>(view + 0x08);
        wr::<u32>(dst_str, 0x0800_0008);
        wr::<u32>(dst_str + 0x04, src_len);
        for k in 0..(src_len as usize) {
            wr::<u8>(dst_str + 0x08 + k, rd::<u8>(src_ptr + k));
        }
        wr::<u8>(dst_str + 0x08 + (src_len as usize), 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41ad_7d00, mock_7d00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_8cc0, mock_8cc0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_8c00, mock_8c00 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41df_ae90, mock_ae90 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41a3_9870, mock_9870 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41ad_d620, mock_d620 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_41ad_cb20, mock_cb20 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_41ac_8bc0, mock_8bc0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let run = |f: F, match_target: bool| -> (Vec<u8>, u64) {
        LOG_SUM = 0;

        let mut node_init = vec![0u8; 0x600];
        let mut node_a = vec![0u8; 0x600];
        let mut node_b = vec![0u8; 0x600];
        let nip = node_init.as_mut_ptr() as usize;
        let nap = node_a.as_mut_ptr() as usize;
        let nbp = node_b.as_mut_ptr() as usize;
        NODE_INIT_PTR = nip;
        NODE_A_PTR = nap;
        NODE_B_PTR = nbp;

        // node_init redirects to node_a via +0x60 != 0 and +0x57c != 0 ("A")
        wr::<u8>(nip + 0x60, 1);
        wr::<u32>(nip + 0x64, 0x0800_0008);
        wr::<u32>(nip + 0x68, 2);
        wr::<u32>(nip + 0x578, 0x0800_0008);
        wr::<u32>(nip + 0x57c, 1);
        wr::<u8>(nip + 0x580, b'A');

        // node_a has +0x57c == 0 (for initial check) or wait: node_a is returned first, then in the loop rsi starts with "B" -> node_b!
        wr::<u64>(nap + 0xc0, 0x1111_4001);
        wr::<u32>(nap + 0x57c, 0);

        wr::<u64>(
            nbp + 0xc0,
            if match_target { 0x1111_4001 } else { 0x2222_4001 },
        );
        wr::<u32>(nbp + 0x578, 0x0800_0008);
        wr::<u32>(nbp + 0x57c, 0);

        let mut rsi_obj = vec![0u8; 0x600];
        let rsp_ptr = rsi_obj.as_mut_ptr() as usize;
        wr::<u32>(rsp_ptr + 0x50, 3);
        wr::<u32>(rsp_ptr + 0x64, 0x0800_0008);
        wr::<u32>(rsp_ptr + 0x68, 3);
        wr::<u8>(rsp_ptr + 0x301, 1);
        wr::<u8>(rsp_ptr + 0x303, 0);
        wr::<u8>(rsp_ptr + 0x304, 0);
        wr::<u8>(rsp_ptr + 0x306, 1);
        wr::<u8>(rsp_ptr + 0x307, 1);
        wr::<u32>(rsp_ptr + 0x578, 0x0800_0008);
        wr::<u32>(rsp_ptr + 0x57c, 1);
        wr::<u8>(rsp_ptr + 0x580, b'B');

        let mut r14_arr = vec![0u8; 0x630 * 2];
        let r14p = r14_arr.as_mut_ptr() as usize;
        let r14_elem = r14p + 0x630;
        wr::<usize>(r14_elem + 0x90, rsp_ptr);
        wr::<u8>(r14_elem + 0xa3, 1);
        wr::<i32>(r14_elem + 0x620, 5);

        let mut rdi_arr = vec![0u8; 0x104 * 2];
        let rdip = rdi_arr.as_mut_ptr() as usize;

        let mut p1 = [0u64; 7];
        p1[0] = r14p as u64;
        p1[1] = 0x77;
        p1[4] = rdip as u64;
        p1[6] = 1;

        let mut p2 = vec![0u8; 0x520];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x78, 0xaaaa_bbbb);
        wr::<u64>(p2p + 0x80, 0xcccc_dddd);
        wr::<u8>(p2p + 0x350, 1);
        wr::<u8>(p2p + 0x501, 1);

        let mut p3 = [0u8; 0x10];
        p3[1] = 1;

        let mut p4 = vec![0u8; 0x200];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<u8>(p4p + 0x10c, 1);
        wr::<u64>(p4p + 0x1e8, 0x9999_4001);

        f(p1.as_ptr() as usize, p2p, p3.as_ptr() as usize, p4p, 0x1234);
        (rdi_arr, LOG_SUM)
    };

    for match_target in [true, false] {
        let o = run(f_o, match_target);
        let n = run(f_n, match_target);
        crate::diff_eq!(o, n, "141acdfd0 match_target={}", match_target);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141da2730(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HOOK_LOG: u64 = 0;
    static mut OBJ_BASE: usize = 0;

    unsafe extern "win64" fn mock_9d30(rcx: usize) {
        // Record the offset from the test object, not the raw pointer: the two
        // runs allocate `obj` at different heap addresses and `rcx & 0xff`
        // would compare address bits instead of behavior.
        HOOK_LOG = HOOK_LOG.wrapping_add(rcx.wrapping_sub(OBJ_BASE) as u64);
    }
    unsafe extern "win64" fn mock_a020(_rcx: usize) -> u64 {
        let xmm = crate::simd_ops::take_xmm_args();
        let t = f32::from_bits(xmm[1] as u32);
        let out = t * 0.5 + 0.1;
        crate::simd_ops::set_xmm_ret(out.to_bits() as u128);
        0
    }
    unsafe extern "win64" fn mock_0a70(out_vec: usize, _src: usize) {
        wr::<u64>(out_vec, 0x1000);
        wr::<u64>(out_vec + 0x08, 3);
    }
    unsafe extern "win64" fn mock_5740(out_vec: usize, _src: usize) {
        wr::<u64>(out_vec, 0x2000);
        wr::<u64>(out_vec + 0x08, 2);
    }
    unsafe extern "win64" fn mock_8b90(_beg: usize, _end: usize, cnt: u64, b: u32) {
        HOOK_LOG = HOOK_LOG.wrapping_add(cnt).wrapping_add(b as u64);
    }
    unsafe extern "win64" fn mock_8f00(_beg: usize, _end: usize, cnt: u64, b: u32) {
        HOOK_LOG = HOOK_LOG.wrapping_add(cnt * 10).wrapping_add(b as u64);
    }
    unsafe extern "win64" fn mock_1a60(
        out_vec3: usize,
        _rdx: u64,
        span_ptr: usize,
        fn40: usize,
        fn80: usize,
        def_vec3: usize,
    ) -> usize {
        let xmm = crate::simd_ops::take_xmm_args();
        let t = f32::from_bits(xmm[1] as u32);
        let cnt = rd::<u64>(span_ptr) as f32;
        let d0 = rd::<f32>(def_vec3);
        let has_vtbl = u32::from(rd::<usize>(fn40 + 0x38) == fn40 && rd::<usize>(fn80 + 0x38) == fn80) as f32;
        wr::<f32>(out_vec3, t + d0);
        wr::<f32>(out_vec3 + 0x04, t * 2.0 + cnt);
        wr::<f32>(out_vec3 + 0x08, t * 3.0 + has_vtbl);
        out_vec3
    }
    unsafe extern "win64" fn mock_1810(
        _rcx: u64,
        span_ptr: usize,
        fn80: usize,
        fn40: usize,
        def_alpha: usize,
    ) -> u64 {
        let xmm = crate::simd_ops::take_xmm_args();
        let t = f32::from_bits(xmm[0] as u32);
        let cnt = rd::<u64>(span_ptr) as f32;
        let da = rd::<f32>(def_alpha);
        let ok = u32::from(rd::<usize>(fn80 + 0x38) == fn80 && rd::<usize>(fn40 + 0x38) == fn40) as f32;
        let res = t * 0.25 + cnt * 0.1 + da + ok;
        crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
        0
    }
    unsafe extern "win64" fn mock_e830(_rcx: usize) {
        HOOK_LOG = HOOK_LOG.wrapping_add(7);
    }
    unsafe extern "win64" fn mock_2bc0(vec_ptr: usize) {
        HOOK_LOG = HOOK_LOG.wrapping_add(rd::<u64>(vec_ptr + 0x08));
    }

    let _m1 = crate::difftest::Mock::new(0x1_41c6_9d30, mock_9d30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41c6_a020, mock_a020 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_402c_0a70, mock_0a70 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4022_5740, mock_5740 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40e6_8b90, mock_8b90 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40e6_8f00, mock_8f00 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41da_1a60, mock_1a60 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41da_1810, mock_1810 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_401f_2bc0, mock_2bc0 as *const ())?;

    let run = |f: F, mode: u32| -> (Vec<u32>, u64) {
        HOOK_LOG = 0;
        let single_val: [f32; 2] = [0.0, 0.65];
        let mut obj = vec![0u32; 0xc10 / 4];
        let p = obj.as_mut_ptr() as usize;
        OBJ_BASE = p;
        wr::<f32>(p + 0xe8, 2.0);
        wr::<usize>(p + 0x110, single_val.as_ptr() as usize);
        wr::<u32>(p + 0x118, mode);
        wr::<f32>(p + 0x124, 1.5);
        wr::<usize>(p + 0x180, single_val.as_ptr() as usize);
        wr::<u32>(p + 0x188, mode);
        wr::<f32>(p + 0x1f8, 10.0);
        wr::<f32>(p + 0x1fc, 2.5);
        wr::<usize>(p + 0x220, single_val.as_ptr() as usize);
        wr::<u32>(p + 0x228, mode);
        wr::<f32>(p + 0x238, -45.0);
        wr::<usize>(p + 0x260, single_val.as_ptr() as usize);
        wr::<u32>(p + 0x268, mode);
        wr::<f32>(p + 0x290, 0.8);
        wr::<f32>(p + 0x2bc, 5.0);
        wr::<f32>(p + 0x2c0, 1.25);
        wr::<i32>(p + 0xc00, 6);

        f(p);
        // Zero out pointer fields before comparing
        wr::<usize>(p + 0x110, 0);
        wr::<usize>(p + 0x180, 0);
        wr::<usize>(p + 0x220, 0);
        wr::<usize>(p + 0x260, 0);
        (obj, HOOK_LOG)
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141da2730 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141dc0630(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUMP: usize = 0;
    static mut HEAP_MEM: [u64; 64] = [0; 64];
    static mut TI_VTBL: [usize; 4] = [0; 4];
    static mut TI_OBJ: [usize; 1] = [0; 1];
    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_alloc(size: usize, _align: u32) -> usize {
        let base = core::ptr::addr_of_mut!(HEAP_MEM) as usize;
        let off = ALLOC_BUMP;
        ALLOC_BUMP += (size + 7) & !7;
        base + off
    }
    unsafe extern "win64" fn mock_free(_ptr: usize) {
        TRACE_HASH = TRACE_HASH.wrapping_add(0x100);
    }
    unsafe extern "win64" fn mock_ti_size(_this: usize) -> u64 {
        12
    }
    unsafe extern "win64" fn mock_ti_align(_this: usize) -> u64 {
        8
    }
    unsafe extern "win64" fn mock_find_ti(type_id: u32) -> usize {
        TRACE_HASH = TRACE_HASH.wrapping_add(type_id as u64);
        core::ptr::addr_of_mut!(TI_OBJ) as usize
    }
    unsafe extern "win64" fn mock_stream_read_u64(this: usize, out_ptr: usize) {
        let seq = rd::<u64>(this + 0x08).wrapping_add(1);
        wr::<u64>(this + 0x08, seq);
        wr::<u64>(out_ptr, 0x9000 + seq);
    }
    unsafe extern "win64" fn mock_stream_read_u32(this: usize, out_ptr: usize) {
        let idx = rd::<u32>(this + 0x10);
        wr::<u32>(this + 0x10, idx + 1);
        let vals: [u32; 4] = [10, 2, 20, 1];
        wr::<u32>(out_ptr, vals[(idx as usize) & 3]);
    }
    unsafe extern "win64" fn mock_stream_read_cnt(_this: usize, out_ptr: usize) {
        wr::<u32>(out_ptr, 2);
    }
    unsafe extern "win64" fn mock_fn3_call(_this: usize, total_ptr: usize) {
        let total = rd::<u64>(total_ptr);
        TRACE_HASH = TRACE_HASH.wrapping_mul(31).wrapping_add(total);
    }
    unsafe extern "win64" fn mock_fn2_call(
        _this: usize,
        _p1: usize,
        span_ptr: usize,
        type_ptr: usize,
    ) {
        let cnt = rd::<u64>(span_ptr);
        let buf = rd::<usize>(span_ptr + 0x08);
        let tid = rd::<u32>(type_ptr) as u64;
        let first = if cnt != 0 { rd::<u64>(buf) } else { 0 };
        TRACE_HASH = TRACE_HASH
            .wrapping_mul(31)
            .wrapping_add(cnt)
            .wrapping_add(tid)
            .wrapping_add(first);
    }
    unsafe extern "win64" fn mock_fn_destroy(_this: usize, flag: u8) {
        TRACE_HASH = TRACE_HASH.wrapping_mul(31).wrapping_add(flag as u64 + 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424b_14c0, mock_find_ti as *const ())?;

    let run = |f: F| -> (u64, usize, usize) {
        ALLOC_BUMP = 0;
        HEAP_MEM = [0; 64];
        TI_VTBL = [
            0,
            0,
            mock_ti_size as *const () as usize,
            mock_ti_align as *const () as usize,
        ];
        TI_OBJ = [core::ptr::addr_of!(TI_VTBL) as usize];
        TRACE_HASH = 7;

        let mut stream_vtbl = [0usize; 10];
        stream_vtbl[0] = mock_stream_read_u64 as *const () as usize;
        stream_vtbl[0x40 / 8] = mock_stream_read_u32 as *const () as usize;
        stream_vtbl[0x48 / 8] = mock_stream_read_cnt as *const () as usize;
        let mut stream_obj: [u64; 3] = [stream_vtbl.as_ptr() as u64, 0, 0];

        let fn2_vtbl: [usize; 5] = [
            0,
            0,
            mock_fn2_call as *const () as usize,
            0,
            mock_fn_destroy as *const () as usize,
        ];
        let mut p2 = [0u64; 8];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, fn2_vtbl.as_ptr() as usize);
        wr::<usize>(p2p + 0x38, p2p);

        let fn3_vtbl: [usize; 5] = [
            0,
            0,
            mock_fn3_call as *const () as usize,
            0,
            mock_fn_destroy as *const () as usize,
        ];
        let mut p3 = [0u64; 8];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p, fn3_vtbl.as_ptr() as usize);
        wr::<usize>(p3p + 0x38, p3p);

        f(stream_obj.as_mut_ptr() as usize, p2p, p3p);
        (
            TRACE_HASH,
            rd::<usize>(p2p + 0x38),
            rd::<usize>(p3p + 0x38),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141dc0630 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e08e80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_VAL: u64 = 0;

    unsafe extern "win64" fn mock_aa00(lstr: usize, elem: usize) {
        wr::<u32>(lstr, 0x0800_0008);
        wr::<u32>(lstr + 0x04, 3);
        wr::<u32>(lstr + 0x08, rd::<u32>(elem));
    }
    unsafe extern "win64" fn mock_16a0(cstr: usize, len: u64) -> u64 {
        (rd::<u8>(cstr) as u64).wrapping_add(len * 0x10)
    }
    unsafe extern "win64" fn mock_af00() -> u64 {
        0x7777
    }
    unsafe extern "win64" fn mock_a540(mgr: u64, str_ptr: usize, out_p1: usize) {
        let b0 = rd::<u8>(str_ptr) as u32;
        wr::<u32>(out_p1 + 0x04, (mgr as u32) ^ b0);
    }
    unsafe extern "win64" fn mock_9b50(out_p1: usize, _view: usize, _r8: u64, _r9: u64, _a5: u32) {
        wr::<u32>(out_p1 + 0x04, 0xdead);
    }
    unsafe extern "win64" fn mock_ab10(_lstr: usize) {
        LOG_VAL += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420e_af00, mock_af00 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420e_a540, mock_a540 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, idx: u64, hit: bool| -> ([u32; 4], u64) {
        LOG_VAL = 0;

        let mut elems = [0u8; 0x60 * 2];
        let ep = elems.as_mut_ptr() as usize;
        // "abc\0" -> len = 3, first byte = b'a' (0x61) -> hash = 0x61 + 0x30 = 0x91, & 3 = 1
        wr::<u32>(ep, u32::from_le_bytes(*b"abc\0"));
        wr::<u32>(ep + 0x58, 2);

        let mut p3 = [0u64; 2];
        wr::<usize>(p3.as_mut_ptr() as usize, ep);
        wr::<u32>(p3.as_mut_ptr() as usize + 0x08, 1);

        let mut tbl = [0u8; 0x28 * 4];
        let tp = tbl.as_mut_ptr() as usize;
        // Slot 1: tombstone (k0 = 0xdead, k1 = 0xbeef)
        wr::<u64>(tp + 0x28, 0xdead);
        wr::<u64>(tp + 0x28 + 0x08, 0xbeef);
        // Slot 2 (next quadratic probe 1 + 1 = 2): either matching entry or empty sentinel
        let s2 = tp + 0x28 * 2;
        if hit {
            wr::<u64>(s2, 0);
            wr::<u64>(s2 + 0x08, 0x91);
            wr::<u16>(s2 + 0x10, 4); // tag 4 -> inline in 0x140217680 (returns rcx_val + 8)
            wr::<u32>(s2 + 0x18, u32::from_le_bytes(*b"Z!\0\0"));
        } else {
            wr::<u64>(s2, 0xffff);
            wr::<u64>(s2 + 0x08, 0xffff);
        }

        let mut p2 = [0u8; 0x90];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x28, 0xdead);
        wr::<u64>(p2p + 0x30, 0xbeef);
        wr::<u64>(p2p + 0x38, 1);
        wr::<u64>(p2p + 0x40, 2);
        wr::<u64>(p2p + 0x48, 4);
        wr::<u64>(p2p + 0x50, 0xffff);
        wr::<u64>(p2p + 0x58, 0xffff);
        wr::<usize>(p2p + 0x78, tp);

        let mut out_p1 = [0u32; 4];
        let op = out_p1.as_mut_ptr() as usize;
        let ret = f(op, p2p, p3.as_ptr() as usize, idx);
        assert_eq!(ret, op);
        (out_p1, LOG_VAL)
    };

    for (idx, hit) in [(1u64, false), (0, true), (0, false)] {
        let o = run(f_o, idx, hit);
        let n = run(f_n, idx, hit);
        crate::diff_eq!(o, n, "141e08e80 idx={} hit={}", idx, hit);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141e5b280(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ITEMS_BUF: [u64; 3] = [0x1111, 0, 0x2222];
    static mut ACC: u64 = 0;

    unsafe extern "win64" fn mock_0050(g_2b8: u64, out_vec: usize, span: usize) {
        let len = rd::<u64>(span);
        ACC = ACC.wrapping_add(g_2b8).wrapping_add(len);
        wr::<usize>(out_vec, core::ptr::addr_of_mut!(ITEMS_BUF) as usize);
        wr::<u64>(out_vec + 0x08, 3);
    }
    unsafe extern "win64" fn mock_50f0(r14: u64, item: u64) {
        ACC = ACC.wrapping_mul(31).wrapping_add(r14 ^ item);
    }
    unsafe extern "win64" fn mock_c190(_p: usize) {
        ACC = ACC.wrapping_add(99);
    }
    unsafe extern "win64" fn mock_5830(r14: u64) {
        ACC = ACC.wrapping_add(r14 * 3);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4242_0050, mock_0050 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4242_50f0, mock_50f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4242_5830, mock_5830 as *const ())?;

    let g_2b8 = exe::ptr::<u64>(0x1_43a0_82b8) as usize;
    let s_2b8 = rd::<u64>(g_2b8);
    wr::<u64>(g_2b8, 0x5000);

    let run = |f: F| -> ([u64; 16], u64) {
        ACC = 0;
        let mut entries = [0u64; 16]; // two 0x40-byte entries
        let ep = entries.as_mut_ptr() as usize;
        // Entry 0: state = 1 (ignored)
        wr::<u32>(ep + 0x20, 1);
        // Entry 1: state = 2, cur_38 = 1, total_30 = 4 -> finishes in one batch of 3 -> state = 3
        let e1 = ep + 0x40;
        wr::<u64>(e1 + 0x10, 0xabc0);
        wr::<u32>(e1 + 0x20, 2);
        wr::<u64>(e1 + 0x28, 0x8000);
        wr::<u32>(e1 + 0x30, 4);
        wr::<u64>(e1 + 0x38, 1);

        let mut p1 = [0u64; 2];
        wr::<usize>(p1.as_mut_ptr() as usize, ep);
        wr::<u32>(p1.as_mut_ptr() as usize + 0x08, 2);

        f(p1.as_ptr() as usize);
        (entries, ACC)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u64>(g_2b8, s_2b8);

    crate::diff_eq!(o, n, "141e5b280 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eb0210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let s_f58 = rd::<usize>(g_f58);

    let run = |f: F, wrap: bool| -> ([u32; 32], usize, u32, u32) {
        let mut ring_mem = [0u32; 32];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let init_wr = if wrap { rmp + 0x58 } else { rmp };
        let mut ring = [0u8; 0x40];
        let rp = ring.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x08, rmp);
        wr::<usize>(rp + 0x10, rmp + 0x68);
        wr::<u64>(rp + 0x18, 0);
        wr::<usize>(rp + 0x20, rmp);
        wr::<usize>(rp + 0x28, init_wr);
        wr::<u32>(rp + 0x30, 2);
        wr::<u32>(rp + 0x34, 2);

        let mut mgr = [0u8; 0x1b0];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x168, rp);
        wr::<u8>(mp + 0x198, 0);
        wr::<usize>(g_f58, mp);

        // Two items: item 0 has non-empty vec + ref_ctrl, item 1 has empty vec (skipped)
        let mut vec0 = [0u64; 2];
        wr::<usize>(vec0.as_mut_ptr() as usize, 0x1000);
        wr::<u32>(vec0.as_mut_ptr() as usize + 0x08, 2);
        let mut vec1 = [0u64; 2];
        wr::<usize>(vec1.as_mut_ptr() as usize, 0x2000);
        wr::<u32>(vec1.as_mut_ptr() as usize + 0x08, 0);

        let mut ref_ctrl = [0u32; 4];
        ref_ctrl[2] = 10; // +0x08 refcount

        let mut arr_20 = [0u8; 0x30];
        let a20p = arr_20.as_mut_ptr() as usize;
        wr::<u32>(a20p + 0x10, 0x1234_5678);
        wr::<u32>(a20p + 0x28, 0x9abc_def0);

        let mut arr_28 = [0u8; 0x50];
        let a28p = arr_28.as_mut_ptr() as usize;
        // Item 0: r8_idx = 1 -> sub at +0x10
        wr::<u8>(a28p, 1);
        wr::<usize>(a28p + 0x18, vec0.as_ptr() as usize);
        wr::<usize>(a28p + 0x20, ref_ctrl.as_mut_ptr() as usize);
        // Item 1 (+0x28): r8_idx = 0 -> sub at +0x28
        wr::<u8>(a28p + 0x28, 0);
        wr::<usize>(a28p + 0x30, vec1.as_ptr() as usize);

        let mut p1 = [0u64; 6];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 2);
        wr::<usize>(p1p + 0x20, a20p);
        wr::<usize>(p1p + 0x28, a28p);

        f(p1p);

        // Normalize the two written pointers in ring_mem so stack addresses don't differ
        let pkt_base = rmp;
        wr::<u64>(pkt_base + 0x0c + 0x08, 0xaaaa);
        wr::<u64>(pkt_base + 0x0c + 0x10, 0xbbbb);

        (
            ring_mem,
            rd::<usize>(rp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rp + 0x30),
            ref_ctrl[2],
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        wr::<usize>(g_f58, s_f58);
        crate::diff_eq!(o, n, "141eb0210 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f573a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;
    static mut MAP_PTR: usize = 0;

    unsafe extern "win64" fn mock_7ce0(rcx: u64) {
        let xmm = crate::simd_ops::take_xmm_args();
        let x1 = xmm[1] as u32 as u64;
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(rcx ^ x1);
    }
    unsafe extern "win64" fn mock_a640() -> usize {
        MAP_PTR
    }
    unsafe extern "win64" fn mock_vfunc_28(this: usize) {
        let id = rd::<u64>(this + 0x08);
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(id);
    }
    unsafe extern "win64" fn mock_2dd0(rcx: u64, edx: u32) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(rcx + edx as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f5_7ce0, mock_7ce0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_a640, mock_a640 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_410d_2dd0, mock_2dd0 as *const ())?;

    let g_258 = exe::ptr::<usize>(0x1_43a3_4258) as usize;
    let g_f58 = exe::ptr::<u64>(0x1_4397_ff58) as usize;
    let s_258 = rd::<usize>(g_258);
    let s_f58 = rd::<u64>(g_f58);

    let run = |f: F, flag_38: u8| -> u64 {
        LOG_HASH = 5;
        let mut g258_obj = [0u8; 0x40];
        g258_obj[0x38] = flag_38;
        wr::<usize>(g_258, g258_obj.as_ptr() as usize);
        wr::<u64>(g_f58, 0x4321);

        let mut vtbl = [0usize; 6];
        vtbl[0x28 / 8] = mock_vfunc_28 as *const () as usize;
        let obj1: [u64; 2] = [vtbl.as_ptr() as u64, 0x101];
        let obj2: [u64; 2] = [vtbl.as_ptr() as u64, 0x202];

        // 4 buckets: empty (0xff), valid1 (1), tombstone (0xfe), valid2 (2)
        let mut buckets = [0u8; 0x10 * 4];
        let bp = buckets.as_mut_ptr() as usize;
        wr::<u32>(bp, 0xff);
        wr::<u32>(bp + 0x10, 1);
        wr::<usize>(bp + 0x18, obj1.as_ptr() as usize);
        wr::<u32>(bp + 0x20, 0xfe);
        wr::<u32>(bp + 0x30, 2);
        wr::<usize>(bp + 0x38, obj2.as_ptr() as usize);

        let mut map = [0u8; 0x60];
        let mp = map.as_mut_ptr() as usize;
        wr::<u32>(mp + 0x24, 0xfe);
        wr::<u64>(mp + 0x28, 1);
        wr::<u64>(mp + 0x38, 4);
        wr::<u32>(mp + 0x40, 0xff);
        wr::<usize>(mp + 0x50, bp);
        MAP_PTR = mp;

        let p1: [u64; 1] = [0x8888];
        f(p1.as_ptr() as usize);
        LOG_HASH
    };

    for flag in [0u8, 1] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        wr::<usize>(g_258, s_258);
        wr::<u64>(g_f58, s_f58);
        crate::diff_eq!(o, n, "141f573a0 flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141fce3b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_9720(vec_ptr: usize, min_cap: u64) {
        GROW_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, min_cap as u32 + 4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_405e_9720, mock_9720 as *const ())?;

    let run = |f: F| -> ([u64; 16], [u64; 16], u32, u32, u32) {
        GROW_CALLS = 0;
        // 4 items of 32 bytes each: [keep, move, move, keep]
        let mut src_buf: [u64; 16] = [
            1, 10, 11, 12, // byte 0 == 1 -> keep
            0, 20, 21, 22, // byte 0 == 0 -> move
            0, 30, 31, 32, // byte 0 == 0 -> move
            2, 40, 41, 42, // byte 0 == 2 -> keep
        ];
        let mut dst_buf = [0u64; 16];

        let mut p1 = [0u64; 4];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, src_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x08, 4);
        wr::<u32>(p1p + 0x0c, 4);
        wr::<usize>(p1p + 0x10, dst_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x18, 0);
        wr::<u32>(p1p + 0x1c, 0); // triggers 0x1405e9720 on first moved item

        f(p1p);
        (
            src_buf,
            dst_buf,
            rd::<u32>(p1p + 0x08),
            rd::<u32>(p1p + 0x18),
            GROW_CALLS,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141fce3b0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_41ac_dfd0, "unknown_141acdfd0", rend::cauldron_hot_v3::unknown_141acdfd0 as *const (), rend::cauldron_hot_v3::diff_unknown_141acdfd0),
// Replacement::verified_throws(0x1_41da_2730, "unknown_141da2730", rend::cauldron_hot_v3::unknown_141da2730 as *const (), Throws::Discard, rend::cauldron_hot_v3::diff_unknown_141da2730),
// Replacement::verified_throws(0x1_41dc_0630, "unknown_141dc0630", rend::cauldron_hot_v3::unknown_141dc0630 as *const (), Throws::Discard, rend::cauldron_hot_v3::diff_unknown_141dc0630),
// Replacement::verified_throws(0x1_41e0_8e80, "unknown_141e08e80", rend::cauldron_hot_v3::unknown_141e08e80 as *const (), Throws::Discard, rend::cauldron_hot_v3::diff_unknown_141e08e80),
// Replacement::verified_throws(0x1_41e5_b280, "unknown_141e5b280", rend::cauldron_hot_v3::unknown_141e5b280 as *const (), Throws::Caught, rend::cauldron_hot_v3::diff_unknown_141e5b280),
// Replacement::verified(0x1_41eb_0210, "unknown_141eb0210", rend::cauldron_hot_v3::unknown_141eb0210 as *const (), rend::cauldron_hot_v3::diff_unknown_141eb0210),
// Replacement::verified(0x1_41f5_73a0, "unknown_141f573a0", rend::cauldron_hot_v3::unknown_141f573a0 as *const (), rend::cauldron_hot_v3::diff_unknown_141f573a0),
// Replacement::verified(0x1_41fc_e3b0, "unknown_141fce3b0", rend::cauldron_hot_v3::unknown_141fce3b0 as *const (), rend::cauldron_hot_v3::diff_unknown_141fce3b0),
