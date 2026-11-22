//! Decompiled hot functions shard Q (`0x141ade1a0..0x141f45ff0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn rcp_ss(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        1.0f32 / x
    }
}

#[inline(always)]
fn lerp_rcp(x: f32, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let cond_le_x0 = x <= x0;
    let dx = x1 - x0;
    let denom = if dx == 0.0f32 { 1.0f32 } else { dx };
    let cond_ge_x1 = x1 <= x;
    let max_x = if x > x0 { x } else { x0 };
    let clamped_x = if max_x < x1 { max_x } else { x1 };
    let r = rcp_ss(denom);
    let r_sq_d = (r * r) * denom;
    let inv_denom = (r + r) - r_sq_d;
    let t = (clamped_x - x0) * inv_denom;
    let interp = y0 + (t * (y1 - y0));
    let sel1 = if cond_le_x0 { y0 } else { interp };
    if cond_ge_x1 { y1 } else { sel1 }
}

/// `unknown_141ade1a0`
/// Original: 0x141ade1a0 (332 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ade1a0(param_1: usize, param_2: usize) -> usize {
    let mut r8 = rd::<usize>(param_1 + 0x358);
    let count_outer = rd::<u32>(param_1 + 0x360) as usize;
    let rcx_end = r8.wrapping_add(count_outer.wrapping_mul(0xb8));
    if r8 == rcx_end {
        return 0;
    }

    let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;
    loop {
        let mut rbx = rd::<usize>(r8 + 0x58);
        let count_inner = rd::<u32>(r8 + 0x60) as usize;
        let rax_end = rbx.wrapping_add(count_inner.wrapping_mul(0x758));
        if rbx != rax_end {
            let rbp_ptr = rd::<usize>(param_2);
            let rsi_len = rd::<u64>(param_2 + 0x08);
            while rbx != rax_end {
                let b3 = rd::<u8>(rbx + 0x03) as u32;
                let cap_bits = if b3 < 0x80 {
                    b3 * 8
                } else {
                    (b3 << 8).wrapping_sub(0x7c00)
                };
                let alloc_bits = (rd::<u32>(rbx) & 0x00ff_ffff) << 3;
                let str_ptr = if alloc_bits > cap_bits {
                    rd::<usize>(rbx + 0x08)
                } else {
                    rbx.wrapping_add(0x08)
                };

                let len = rd::<u32>(rbx + 0x04) as u64;
                if len == rsi_len {
                    if len == 0 {
                        break;
                    }
                    let mut matched = true;
                    for i in 0..(len as usize) {
                        let c1 = rd::<u8>(lut + rd::<u8>(str_ptr + i) as usize) as i8;
                        let c2 = rd::<u8>(lut + rd::<u8>(rbp_ptr + i) as usize) as i8;
                        if c1 != c2 {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        break;
                    }
                }
                rbx = rbx.wrapping_add(0x758);
            }
        }
        if rbx == rax_end {
            rbx = 0;
        }
        if rbx != 0 {
            return rbx;
        }
        r8 = r8.wrapping_add(0xb8);
        if r8 == rcx_end {
            return 0;
        }
    }
}

/// `content_141c696e0`
/// Original: 0x141c696e0 (658 bytes, Throws::No, FLOAT_RETURN)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn content_141c696e0(param_1: usize, val: f32) -> f32 {
    content_141c696e0_impl(param_1, val)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn content_141c696e0(param_1: usize, _xmm0: f32, val: f32) -> f32 {
    content_141c696e0_impl(param_1, val)
}

#[inline(always)]
unsafe fn content_141c696e0_impl(param_1: usize, val: f32) -> f32 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let mode = rd::<u32>(param_1);
    let res = match mode {
        0 | 1 | 3 => {
            let mut xmm9 = rd::<f32>(param_1 + 0x14);
            let mut xmm11 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
            let mut xmm12 = if mode == 3 {
                rd::<f32>(param_1 + 0x24)
            } else {
                rd::<f32>(param_1 + 0x2c)
            };
            let half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
            let mut xmm8 = 0.0f32;
            for _ in 0..4 {
                let xmm7 = (xmm11 + xmm8) * half;
                let (_, ret_xmm0) = crate::simd_ops::call_va8_xmm(
                    0x1_41c6_9f70,
                    param_1 as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, xmm7.to_bits() as u128, 0, 0],
                );
                let xmm1 = f32::from_bits(ret_xmm0 as u32);
                if !(xmm1 >= val) {
                    xmm9 = xmm1;
                } else {
                    xmm12 = xmm1;
                }
                let le = val <= xmm1;
                xmm11 = if le { xmm7 } else { xmm11 };
                xmm8 = if le { xmm8 } else { xmm7 };
            }
            let t_param = lerp_rcp(val, xmm9, xmm8, xmm12, xmm11);
            let (_, out_xmm0) = crate::simd_ops::call_va8_xmm(
                0x1_41c6_9ef0,
                param_1 as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, t_param.to_bits() as u128, 0, 0],
            );
            f32::from_bits(out_xmm0 as u32)
        }
        2 => lerp_rcp(
            val,
            rd::<f32>(param_1 + 0x14),
            rd::<f32>(param_1 + 0x18),
            rd::<f32>(param_1 + 0x1c),
            rd::<f32>(param_1 + 0x20),
        ),
        4 => rd::<f32>(param_1 + 0x18),
        _ => 0.0f32,
    };
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

/// `coregame_141df0600` (`coregame::ui_data_transform::world_to_screen::Transform::vfunc_1`)
/// Original: 0x141df0600 (1324 bytes, Throws::Caught)
pub unsafe extern "win64" fn coregame_141df0600(
    _param_1: usize,
    _param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
) -> u8 {
    let rbx_world = param_7;
    let v0 = rd::<usize>(rbx_world);
    let id_0cc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f0cc) as usize) as usize;
    let lut = rd::<usize>(v0 + 0x20);
    let idx = rd::<u16>(lut.wrapping_add(id_0cc * 2)) as usize;
    let entry = rd::<usize>(rbx_world + 0x08).wrapping_add(idx * 16);
    let len_0cc = rd::<u32>(entry + 0x08) as u64;
    let ptr_0cc = rd::<u64>(entry);
    let mut arg1_350: [u64; 3] = [rbx_world as u64, len_0cc, ptr_0cc];
    let mut out_350 = [0u8; 32];
    let _ = exe::call_va!(
        0x1_4023_7350,
        (
            arg1_350.as_mut_ptr() as usize,
            out_350.as_mut_ptr() as usize
        ) -> usize
    );
    if out_350[0x18] == 0 {
        return 0;
    }

    let r12 = param_6;
    let mut str1 = [0u64; 9];
    let has_str1 = if rd::<u32>(r12 + 0x08) > 1 {
        let rdx = rd::<usize>(r12).wrapping_add(0x60);
        if rd::<u32>(rdx + 0x58) == 2 {
            let s1 = str1.as_mut_ptr() as usize;
            wr::<u32>(s1, 0x0800_0008);
            wr::<u32>(s1 + 0x04, 0);
            wr::<u8>(s1 + 0x08, 0);
            let _ = exe::call_va!(0x1_4240_aa00, (s1, rdx) -> usize);
            true
        } else {
            false
        }
    } else {
        false
    };

    if has_str1 {
        let s1 = str1.as_ptr() as usize;
        let hdr = rd::<u32>(s1);
        let b3 = hdr >> 24;
        let cap_bits = if b3 < 0x80 {
            b3 * 8
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let alloc_bits = (hdr & 0x00ff_ffff) << 3;
        let p_chars = if alloc_bits > cap_bits {
            rd::<usize>(s1 + 0x08)
        } else {
            s1 + 0x08
        };
        let mut len1 = 0usize;
        while rd::<u8>(p_chars + len1) != 0 {
            len1 += 1;
        }
        let hash1 = exe::call_va!(0x1_424b_16a0, (p_chars, len1 as u64) -> u64);
        let mut empty_val = rd::<u64>(param_3 + 0x38);
        if rd::<u64>(param_3 + 0x40) != empty_val {
            let mut step: u64 = 0;
            let mask = rd::<u64>(param_3 + 0x48).wrapping_sub(1);
            let mut idx_slot = mask & hash1;
            let mut tomb = u64::MAX;
            let entries = rd::<usize>(param_3 + 0x78);
            loop {
                let slot = entries.wrapping_add((idx_slot as usize).wrapping_mul(0x28));
                let k0 = rd::<u64>(slot);
                let k1 = rd::<u64>(slot + 0x08);
                if rd::<u64>(param_3 + 0x50) == k0 {
                    if rd::<u64>(param_3 + 0x58) == k1 {
                        break;
                    }
                    empty_val = rd::<u64>(param_3 + 0x38);
                }
                if empty_val != 0
                    && rd::<u64>(param_3 + 0x28) == k0
                    && rd::<u64>(param_3 + 0x30) == k1
                {
                    if tomb == u64::MAX {
                        tomb = idx_slot;
                    }
                    step = step.wrapping_add(1);
                    idx_slot = idx_slot.wrapping_add(step) & mask;
                    empty_val = rd::<u64>(param_3 + 0x38);
                    continue;
                }
                if k0 == 0 && hash1 == k1 {
                    if idx_slot != u64::MAX {
                        let end_slot =
                            entries.wrapping_add((rd::<usize>(param_3 + 0x48)).wrapping_mul(0x28));
                        if slot != end_slot {
                            let val_ptr = slot.wrapping_add(0x10);
                            if rd::<u16>(val_ptr) != 6
                                && val_ptr != 0
                                && rd::<u64>(val_ptr + 0x08) != 0
                            {
                                exe::call_va!(0x1_4240_ab10, (str1.as_mut_ptr() as usize));
                                return 0;
                            }
                        }
                    }
                    break;
                }
                step = step.wrapping_add(1);
                idx_slot = idx_slot.wrapping_add(step) & mask;
                empty_val = rd::<u64>(param_3 + 0x38);
            }
        }
    }

    let mut pos_x = rd::<f32>(param_4 + 0x08);
    let mut pos_y = rd::<f32>(param_4 + 0x0c);
    let mut pos_z = rd::<f32>(param_4 + 0x10);
    let mut r15d = rd::<u32>(r12 + 0x08);
    let mut r14 = rd::<usize>(r12);

    let mut str2 = [0u64; 9];
    let has_str2 = if r15d > 2 && rd::<u32>(r14.wrapping_add(0xc0).wrapping_add(0x58)) == 2 {
        let s2 = str2.as_mut_ptr() as usize;
        wr::<u32>(s2, 0x0800_0008);
        wr::<u32>(s2 + 0x04, 0);
        wr::<u8>(s2 + 0x08, 0);
        let _ = exe::call_va!(0x1_4240_aa00, (s2, r14.wrapping_add(0xc0)) -> usize);
        r15d = rd::<u32>(r12 + 0x08);
        r14 = rd::<usize>(r12);
        true
    } else {
        false
    };

    if has_str2 {
        let mut ref_x = 0.0f32;
        let mut ref_y = 0.0f32;
        let mut ref_z = 0.0f32;
        let s2 = str2.as_ptr() as usize;
        let hdr = rd::<u32>(s2);
        let b3 = hdr >> 24;
        let cap_bits = if b3 < 0x80 {
            b3 * 8
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let alloc_bits = (hdr & 0x00ff_ffff) << 3;
        let p_chars = if alloc_bits > cap_bits {
            rd::<usize>(s2 + 0x08)
        } else {
            s2 + 0x08
        };
        let mut len2 = 0usize;
        while rd::<u8>(p_chars + len2) != 0 {
            len2 += 1;
        }
        let hash2 = exe::call_va!(0x1_424b_16a0, (p_chars, len2 as u64) -> u64);
        let mut key_pair: [u64; 2] = [0, hash2];
        let val_ptr =
            exe::call_va!(0x1_401e_8210, (param_3, key_pair.as_mut_ptr() as usize) -> usize);
        if val_ptr != 0 {
            ref_x = rd::<f32>(val_ptr + 0x08);
            ref_y = rd::<f32>(val_ptr + 0x0c);
            ref_z = rd::<f32>(val_ptr + 0x10);
        }
        let id_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
        let tbl_580e0 = rd::<usize>(rbx_world + 0x580e0);
        let factor_ptr = rd::<usize>(tbl_580e0.wrapping_add(id_0d8 * 8));
        let factor = rd::<f32>(factor_ptr);
        pos_x = (pos_x - ref_x) * factor + ref_x;
        pos_y = (pos_y - ref_y) * factor + ref_y;
        pos_z = (pos_z - ref_z) * factor + ref_z;
    }

    let mut clamp_vals = 0u64;
    let do_clamp: u8 = if r15d != 0 && rd::<u32>(r14 + 0x58) == 4 {
        clamp_vals = rd::<u64>(r14 + 0x48);
        1
    } else {
        0
    };

    let mut pos_buf: [f32; 3] = [pos_x, pos_y, pos_z];
    let mut screen_uv: [f32; 2] = [0.0, 0.0];
    let g_5b0 = exe::ptr::<u8>(0x1_43a4_85b0) as usize;
    exe::call_va!(
        0x1_41df_14f0,
        (
            screen_uv.as_mut_ptr() as usize,
            pos_buf.as_mut_ptr() as usize,
            g_5b0,
            do_clamp as u32
        )
    );

    let g_a50 = rd::<usize>(exe::ptr::<usize>(0x1_4397_da50) as usize);
    let height = (rd::<i32>(g_a50 + 0x44)) as f32;
    let width = (rd::<i32>(g_a50 + 0x40)) as f32;
    let mut sx = width * screen_uv[0];
    let mut sy = height * screen_uv[1];

    if do_clamp != 0 {
        let cx = f32::from_bits(clamp_vals as u32);
        let cy = f32::from_bits((clamp_vals >> 32) as u32);
        let c_1080 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6f40) as usize);
        let margin_x = (height * cx) / c_1080;
        let margin_y = (height * cy) / c_1080;
        let max_x = width - margin_x;
        sx = if sx > margin_x { sx } else { margin_x };
        sx = if sx < max_x { sx } else { max_x };
        let max_y = height - margin_y;
        sy = if sy > margin_y { sy } else { margin_y };
        sy = if sy < max_y { sy } else { max_y };
    }

    if rd::<u8>(param_5 + 0xc8) != 3 {
        exe::call_va!(0x1_4020_0070, ());
        return 0;
    }

    let changed: u8 = if rd::<f32>(param_5 + 0x80) == sx && rd::<f32>(param_5 + 0x84) == sy {
        0
    } else {
        wr::<f32>(param_5 + 0x80, sx);
        wr::<f32>(param_5 + 0x84, sy);
        1
    };

    if has_str2 {
        exe::call_va!(0x1_4240_ab10, (str2.as_mut_ptr() as usize));
    }
    if has_str1 {
        exe::call_va!(0x1_4240_ab10, (str1.as_mut_ptr() as usize));
    }
    changed
}

/// `unknown_141dfb350`
/// Original: 0x141dfb350 (1661 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141dfb350(param_1: usize, param_2: usize, param_3: usize) {
    let r12_elem = rd::<usize>(param_1).wrapping_add((rd::<usize>(param_1 + 0x10)).wrapping_mul(0x6c));
    if rd::<u8>(r12_elem + 0x69) != 0 {
        return;
    }

    let mut found_any = false;
    let world3 = rd::<usize>(param_3);
    let r13_world3_40 = world3.wrapping_add(0x40);
    let mut span3: [u64; 2] = [rd::<u64>(param_3 + 0x08), rd::<u64>(param_3 + 0x10)];
    let mut iter3: [u64; 3] = [0; 3];
    let _ = exe::call_va!(
        0x1_401e_7c10,
        (
            iter3.as_mut_ptr() as usize,
            r13_world3_40,
            span3.as_mut_ptr() as usize
        ) -> usize
    );
    let iter3_base = iter3[0] as usize;
    let mut cur_arch3 = iter3[1] as usize;
    let end_arch3 = iter3[2] as usize;

    let mut idx3: u64 = 0;
    let mut ent_count3: u64 = 0;
    let mut comp_base3: usize = 0;

    if cur_arch3 != end_arch3 {
        let arch = rd::<u16>(cur_arch3) as usize;
        let block3 = rd::<usize>(r13_world3_40.wrapping_add(arch * 8).wrapping_add(0x48));
        ent_count3 = if block3 != 0 {
            rd::<u32>(r13_world3_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
        let want_ecc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u32;
        let arch_off = arch.wrapping_shl(5);
        let search_len = rd::<u32>(r13_world3_40.wrapping_add(arch_off).wrapping_add(0x1806c));
        let search_ptr = rd::<u64>(r13_world3_40.wrapping_add(arch_off).wrapping_add(0x18058));
        let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_ecc) -> i32);
        if (found as isize) != -1 {
            let off_tbl = rd::<usize>(r13_world3_40.wrapping_add((arch + 0xc03).wrapping_shl(5)));
            let off = rd::<u32>(off_tbl.wrapping_add((found as isize as usize) * 4)) as usize;
            comp_base3 = if off != 0 {
                off.wrapping_add(block3)
            } else {
                0
            };
        }
    }

    if ent_count3 != 0 {
        while idx3 < ent_count3 {
            let rdi_comp = comp_base3.wrapping_add((idx3 as usize).wrapping_mul(0x140));
            let ok = exe::call_va!(0x1_41df_aaa0, (rdi_comp, param_3) -> u8);
            if ok != 0 {
                let b3 = rd::<u8>(r12_elem + 0x2b) as u32;
                let cap_bits = if b3 < 0x80 {
                    b3 * 8
                } else {
                    (b3 << 8).wrapping_sub(0x7c00)
                };
                let alloc_bits = (rd::<u32>(r12_elem + 0x28) & 0x00ff_ffff) << 3;
                let str_ptr = if alloc_bits > cap_bits {
                    rd::<usize>(r12_elem + 0x30)
                } else {
                    r12_elem.wrapping_add(0x30)
                };
                let str_len = rd::<u32>(r12_elem + 0x2c) as u64;
                let mut str_view: [u64; 2] = [str_ptr as u64, str_len];
                let found_idx =
                    exe::call_va!(0x1_41df_a9a0, (rdi_comp, str_view.as_mut_ptr() as usize) -> i32);
                if found_idx != -1 {
                    wr::<i32>(r12_elem + 0x64, found_idx);
                    found_any = true;
                    let target_idx = rd::<i32>(rdi_comp + 0x11c);
                    if found_idx == target_idx {
                        if rd::<u8>(r12_elem + 0x68) != 0 {
                            let mut vec_buf: [u64; 3] = [0; 3];
                            let _ = exe::call_va!(
                                0x1_40f4_d380,
                                (vec_buf.as_mut_ptr() as usize, r12_elem, param_2) -> usize
                            );
                            let vec_ptr = vec_buf[0] as usize;
                            let vec_len = (vec_buf[1] as u32) as usize;
                            let vec_end = vec_ptr.wrapping_add(vec_len * 4);
                            if vec_ptr != vec_end {
                                let rsi_world2_40 = rd::<usize>(param_2).wrapping_add(0x40);
                                let mut r11_cur = vec_ptr;
                                while r11_cur != vec_end {
                                    let mut span2: [u64; 2] =
                                        [rd::<u64>(param_2 + 0x08), rd::<u64>(param_2 + 0x10)];
                                    let mut iter2: [u64; 3] = [0; 3];
                                    let _ = exe::call_va!(
                                        0x1_401e_7c10,
                                        (
                                            iter2.as_mut_ptr() as usize,
                                            rsi_world2_40,
                                            span2.as_mut_ptr() as usize
                                        ) -> usize
                                    );
                                    let iter2_base = iter2[0] as usize;
                                    let mut cur_arch2 = iter2[1] as usize;
                                    let end_arch2 = iter2[2] as usize;
                                    let mut ent_count2: u64 = 0;
                                    let mut idx2: u64 = 0;
                                    let mut block2: usize;
                                    let mut comp_base2: usize = 0;

                                    if cur_arch2 != end_arch2 {
                                        let arch2 = rd::<u16>(cur_arch2) as usize;
                                        block2 = rd::<usize>(
                                            rsi_world2_40.wrapping_add(arch2 * 8).wrapping_add(0x48),
                                        );
                                        ent_count2 = if block2 != 0 {
                                            rd::<u32>(
                                                rsi_world2_40
                                                    .wrapping_add(arch2 * 4)
                                                    .wrapping_add(0x10048),
                                            ) as u64
                                        } else {
                                            0
                                        };
                                        let want_330 =
                                            rd::<u16>(exe::ptr::<u16>(0x1_43c9_0330) as usize)
                                                as u32;
                                        let arch2_off = arch2.wrapping_shl(5);
                                        let s_len2 = rd::<u32>(
                                            rsi_world2_40
                                                .wrapping_add(arch2_off)
                                                .wrapping_add(0x1806c),
                                        );
                                        let s_ptr2 = rd::<u64>(
                                            rsi_world2_40
                                                .wrapping_add(arch2_off)
                                                .wrapping_add(0x18058),
                                        );
                                        let f2 = exe::call_va!(
                                            0x1_4248_9ee0,
                                            (s_ptr2, s_len2, want_330) -> i32
                                        );
                                        if (f2 as isize) != -1 {
                                            let off_tbl2 = rd::<usize>(
                                                rsi_world2_40
                                                    .wrapping_add((arch2 + 0xc03).wrapping_shl(5)),
                                            );
                                            let off2 = rd::<u32>(
                                                off_tbl2.wrapping_add((f2 as isize as usize) * 4),
                                            ) as usize;
                                            comp_base2 = if off2 != 0 {
                                                off2.wrapping_add(block2)
                                            } else {
                                                0
                                            };
                                        }
                                    }

                                    if ent_count2 != 0 {
                                        let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;
                                        let s_b3 = rd::<u8>(r12_elem + 0x2b) as u32;
                                        let s_alloc_bits =
                                            (rd::<u32>(r12_elem + 0x28) & 0x00ff_ffff) << 3;
                                        let s_len = rd::<u32>(r12_elem + 0x2c) as u64;
                                        while idx2 < ent_count2 {
                                            let r14_item = comp_base2
                                                .wrapping_add((idx2 as usize).wrapping_mul(0x3c));
                                            let s_cap_bits = if s_b3 < 0x80 {
                                                s_b3 * 8
                                            } else {
                                                (s_b3 << 8).wrapping_sub(0x7c00)
                                            };
                                            let s_ptr = if s_alloc_bits > s_cap_bits {
                                                rd::<usize>(r12_elem + 0x30)
                                            } else {
                                                r12_elem.wrapping_add(0x30)
                                            };

                                            let item_b3 = rd::<u8>(r14_item + 0x03) as u32;
                                            let item_cap_bits = if item_b3 < 0x80 {
                                                item_b3 * 8
                                            } else {
                                                (item_b3 << 8).wrapping_sub(0x7c00)
                                            };
                                            let item_alloc_bits =
                                                (rd::<u32>(r14_item) & 0x00ff_ffff) << 3;
                                            let item_ptr = if item_alloc_bits > item_cap_bits {
                                                rd::<usize>(r14_item + 0x08)
                                            } else {
                                                r14_item.wrapping_add(0x08)
                                            };
                                            let item_len = rd::<u32>(r14_item + 0x04) as u64;
                                            if item_len == s_len {
                                                let mut str_eq = true;
                                                for k in 0..(item_len as usize) {
                                                    let c1 = rd::<u8>(
                                                        lut + rd::<u8>(item_ptr + k) as usize,
                                                    ) as i8;
                                                    let c2 = rd::<u8>(
                                                        lut + rd::<u8>(s_ptr + k) as usize,
                                                    ) as i8;
                                                    if c1 != c2 {
                                                        str_eq = false;
                                                        break;
                                                    }
                                                }
                                                if str_eq
                                                    && rd::<u32>(r14_item + 0x28)
                                                        == rd::<u32>(r11_cur)
                                                    && rd::<u8>(r14_item + 0x34) == 0
                                                {
                                                    exe::call_va!(
                                                        0x1_41df_bfe0,
                                                        (r12_elem, param_2)
                                                    );
                                                    wr::<u8>(r12_elem + 0x68, 0);
                                                    if vec_ptr != 0 {
                                                        exe::call_va!(0x1_4241_c190, (vec_ptr));
                                                    }
                                                    return;
                                                }
                                            }

                                            idx2 += 1;
                                            if idx2 >= ent_count2 {
                                                cur_arch2 = cur_arch2.wrapping_add(2);
                                                while cur_arch2 != end_arch2 {
                                                    let a = rd::<u16>(cur_arch2) as usize;
                                                    if rd::<usize>(
                                                        iter2_base
                                                            .wrapping_add(a * 8)
                                                            .wrapping_add(0x48),
                                                    ) != 0
                                                        && rd::<u32>(
                                                            iter2_base
                                                                .wrapping_add(a * 4)
                                                                .wrapping_add(0x10048),
                                                        ) != 0
                                                    {
                                                        break;
                                                    }
                                                    cur_arch2 = cur_arch2.wrapping_add(2);
                                                }
                                                if cur_arch2 != end_arch2 {
                                                    let a = rd::<u16>(cur_arch2) as usize;
                                                    block2 = rd::<usize>(
                                                        rsi_world2_40
                                                            .wrapping_add(a * 8)
                                                            .wrapping_add(0x48),
                                                    );
                                                    ent_count2 = if block2 != 0 {
                                                        rd::<u32>(
                                                            rsi_world2_40
                                                                .wrapping_add(a * 4)
                                                                .wrapping_add(0x10048),
                                                        ) as u64
                                                    } else {
                                                        0
                                                    };
                                                    idx2 = 0;
                                                    let want_330 = rd::<u16>(
                                                        exe::ptr::<u16>(0x1_43c9_0330) as usize,
                                                    )
                                                        as u32;
                                                    let a_off = a.wrapping_shl(5);
                                                    let s_len2 = rd::<u32>(
                                                        rsi_world2_40
                                                            .wrapping_add(a_off)
                                                            .wrapping_add(0x1806c),
                                                    );
                                                    let s_ptr2 = rd::<u64>(
                                                        rsi_world2_40
                                                            .wrapping_add(a_off)
                                                            .wrapping_add(0x18058),
                                                    );
                                                    let f2 = exe::call_va!(
                                                        0x1_4248_9ee0,
                                                        (s_ptr2, s_len2, want_330) -> i32
                                                    );
                                                    comp_base2 = if (f2 as isize) != -1 {
                                                        let off_tbl2 = rd::<usize>(
                                                            rsi_world2_40.wrapping_add(
                                                                (a + 0xc03).wrapping_shl(5),
                                                            ),
                                                        );
                                                        let off2 = rd::<u32>(
                                                            off_tbl2.wrapping_add(
                                                                (f2 as isize as usize) * 4,
                                                            ),
                                                        )
                                                            as usize;
                                                        if off2 != 0 {
                                                            off2.wrapping_add(block2)
                                                        } else {
                                                            0
                                                        }
                                                    } else {
                                                        0
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    r11_cur = r11_cur.wrapping_add(4);
                                }
                            }
                            if vec_ptr != 0 {
                                exe::call_va!(0x1_4241_c190, (vec_ptr));
                            }
                        }
                    } else if found_idx > target_idx {
                        wr::<u8>(r12_elem + 0x68, 1);
                        return;
                    }
                }
            }

            idx3 += 1;
            if idx3 >= ent_count3 {
                cur_arch3 = cur_arch3.wrapping_add(2);
                while cur_arch3 != end_arch3 {
                    let a = rd::<u16>(cur_arch3) as usize;
                    if rd::<usize>(iter3_base.wrapping_add(a * 8).wrapping_add(0x48)) != 0
                        && rd::<u32>(iter3_base.wrapping_add(a * 4).wrapping_add(0x10048)) != 0
                    {
                        break;
                    }
                    cur_arch3 = cur_arch3.wrapping_add(2);
                }
                if cur_arch3 != end_arch3 {
                    let a = rd::<u16>(cur_arch3) as usize;
                    let block3 = rd::<usize>(r13_world3_40.wrapping_add(a * 8).wrapping_add(0x48));
                    ent_count3 = if block3 != 0 {
                        rd::<u32>(r13_world3_40.wrapping_add(a * 4).wrapping_add(0x10048)) as u64
                    } else {
                        0
                    };
                    idx3 = 0;
                    let want_ecc = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u32;
                    let a_off = a.wrapping_shl(5);
                    let s_len3 =
                        rd::<u32>(r13_world3_40.wrapping_add(a_off).wrapping_add(0x1806c));
                    let s_ptr3 =
                        rd::<u64>(r13_world3_40.wrapping_add(a_off).wrapping_add(0x18058));
                    let f3 = exe::call_va!(0x1_4248_9ee0, (s_ptr3, s_len3, want_ecc) -> i32);
                    comp_base3 = if (f3 as isize) != -1 {
                        let off_tbl3 =
                            rd::<usize>(r13_world3_40.wrapping_add((a + 0xc03).wrapping_shl(5)));
                        let off3 =
                            rd::<u32>(off_tbl3.wrapping_add((f3 as isize as usize) * 4)) as usize;
                        if off3 != 0 {
                            off3.wrapping_add(block3)
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                }
            }
        }
        if found_any {
            return;
        }
    }

    if rd::<i32>(r12_elem + 0x64) != -1 {
        wr::<i32>(r12_elem + 0x64, -1);
        wr::<u8>(r12_elem + 0x68, 1);
    }
}

/// `unknown_141e009b0`
/// Original: 0x141e009b0 (156 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e009b0(param_1: usize, param_2: usize, param_3: usize) {
    if rd::<u64>(exe::ptr::<u64>(0x1_43a3_40f8) as usize) != 0 {
        let s1 = exe::call_va!(0x1_420e_8920, () -> usize);
        let mut flag: u8 = 0;
        let old_bit = ((rd::<u32>(s1 + 0x5aa0) >> 9) & 1) as u8;
        exe::call_va!(
            0x1_40f5_5140,
            (param_1, param_3, (&mut flag as *mut u8) as usize)
        );
        exe::call_va!(
            0x1_40f5_5790,
            (param_2, param_3, (&mut flag as *mut u8) as usize)
        );
        if flag != old_bit {
            let s2 = exe::call_va!(0x1_420e_8920, () -> usize);
            let cur = rd::<u32>(s2 + 0x5aa0);
            let next = if flag != 0 {
                cur | 0x200
            } else {
                cur & !0x200
            };
            wr::<u32>(s2 + 0x5aa0, next);
        }
    }
}

/// `unknown_141e1c970`
/// Original: 0x141e1c970 (1216 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e1c970(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut idx: u64 = 0;
    if rd::<u64>(param_1 + 0x18) != 0 {
        let mut off_18: usize = 0;
        let mut off_90: usize = 0;
        loop {
            let a0 = rd::<usize>(param_1 + 0x20);
            let a1 = rd::<usize>(param_1 + 0x28);
            let a2 = rd::<usize>(param_1 + 0x30);
            let a3 = rd::<usize>(param_1 + 0x38);
            let flag_pair = a0.wrapping_add((idx as usize) * 2);
            let block_90 = a2.wrapping_add(off_90);
            let val_ptr = a3.wrapping_add((idx as usize) * 8);

            let mut p = rd::<usize>(block_90 + 0x30);
            let end = p.wrapping_add((rd::<u32>(block_90 + 0x38) as usize) * 8);
            while p != end {
                let item = rd::<u64>(p);
                if rd::<u8>(a1.wrapping_add(off_18).wrapping_add(0x11)) != 0 {
                    let mut span: [u64; 3] = [
                        rd::<u64>(param_2),
                        rd::<u64>(param_2 + 0x08),
                        rd::<u64>(param_2 + 0x10),
                    ];
                    let mut out_a8 = [0u8; 32];
                    let _ = exe::call_va!(
                        0x1_4021_b890,
                        (out_a8.as_mut_ptr() as usize, item, span.as_mut_ptr() as usize) -> usize
                    );
                    if out_a8[0x18] != 0 {
                        let v = rd::<u64>(val_ptr);
                        let entry =
                            exe::call_va!(0x1_4020_f310, (param_4.wrapping_add(0x68)) -> usize);
                        wr::<u64>(entry, v);
                        wr::<u32>(entry + 0x08, 0);
                        let rdx_idx = rd::<u64>(param_4 + 0xa8);
                        let arr_88 = rd::<usize>(param_4 + 0x88);
                        let diff = (rd::<u32>(param_4 + 0xa0))
                            .wrapping_sub(rd::<u32>(arr_88.wrapping_add((rdx_idx as usize) * 8)));
                        let packed = diff | ((rdx_idx as u32) << 16);
                        let out_0 = rd::<usize>(out_a8.as_ptr() as usize);
                        let out_2 = rd::<usize>(out_a8.as_ptr() as usize + 0x10);
                        let payload = rd::<u64>(out_0.wrapping_add(out_2 * 8));
                        let cmd9 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 9u32) -> usize);
                        wr::<u8>(cmd9, 9);
                        wr::<u64>(cmd9 + 1, payload);
                        let cmd1 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 1u32) -> usize);
                        wr::<u8>(cmd1, 0x0c);
                        wr::<u32>(entry + 0x0c, packed);
                        wr::<u32>(entry + 0x10, 0xffff_ffff);
                        wr::<u8>(flag_pair, 1);
                    }
                }
                if rd::<u8>(a1.wrapping_add(off_18).wrapping_add(0x10)) != 0 {
                    let mut span: [u64; 3] = [
                        rd::<u64>(param_3),
                        rd::<u64>(param_3 + 0x08),
                        rd::<u64>(param_3 + 0x10),
                    ];
                    let mut out_a8 = [0u8; 32];
                    let _ = exe::call_va!(
                        0x1_4021_b890,
                        (out_a8.as_mut_ptr() as usize, item, span.as_mut_ptr() as usize) -> usize
                    );
                    if out_a8[0x18] != 0 {
                        let v = rd::<u64>(val_ptr);
                        let entry =
                            exe::call_va!(0x1_4020_f310, (param_4.wrapping_add(0x68)) -> usize);
                        wr::<u64>(entry, v);
                        wr::<u32>(entry + 0x08, 0);
                        let rdx_idx = rd::<u64>(param_4 + 0xa8);
                        let arr_88 = rd::<usize>(param_4 + 0x88);
                        let diff = (rd::<u32>(param_4 + 0xa0))
                            .wrapping_sub(rd::<u32>(arr_88.wrapping_add((rdx_idx as usize) * 8)));
                        let packed = diff | ((rdx_idx as u32) << 16);
                        let out_0 = rd::<usize>(out_a8.as_ptr() as usize);
                        let out_2 = rd::<usize>(out_a8.as_ptr() as usize + 0x10);
                        let payload = rd::<u64>(out_0.wrapping_add(out_2 * 8));
                        let cmd9 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 9u32) -> usize);
                        wr::<u8>(cmd9, 9);
                        wr::<u64>(cmd9 + 1, payload);
                        let cmd1 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 1u32) -> usize);
                        wr::<u8>(cmd1, 0x0c);
                        wr::<u32>(entry + 0x0c, packed);
                        wr::<u32>(entry + 0x10, 0xffff_ffff);
                        wr::<u8>(flag_pair, 1);
                    }
                }
                p = p.wrapping_add(8);
            }

            let mut p2 = rd::<usize>(block_90 + 0x60);
            let end2 = p2.wrapping_add((rd::<u32>(block_90 + 0x68) as usize) * 8);
            while p2 != end2 {
                let item = rd::<u64>(p2);
                if rd::<u8>(a1.wrapping_add(off_18).wrapping_add(0x11)) != 0 {
                    let mut span: [u64; 3] = [
                        rd::<u64>(param_2),
                        rd::<u64>(param_2 + 0x08),
                        rd::<u64>(param_2 + 0x10),
                    ];
                    let mut out_a8 = [0u8; 32];
                    let _ = exe::call_va!(
                        0x1_4021_b890,
                        (out_a8.as_mut_ptr() as usize, item, span.as_mut_ptr() as usize) -> usize
                    );
                    if out_a8[0x18] != 0 {
                        let v = rd::<u64>(val_ptr);
                        let entry =
                            exe::call_va!(0x1_4020_f310, (param_4.wrapping_add(0x68)) -> usize);
                        wr::<u64>(entry, v);
                        wr::<u32>(entry + 0x08, 1);
                        let rdx_idx = rd::<u64>(param_4 + 0xa8);
                        let arr_88 = rd::<usize>(param_4 + 0x88);
                        let diff = (rd::<u32>(param_4 + 0xa0))
                            .wrapping_sub(rd::<u32>(arr_88.wrapping_add((rdx_idx as usize) * 8)));
                        let packed = diff | ((rdx_idx as u32) << 16);
                        let out_0 = rd::<usize>(out_a8.as_ptr() as usize);
                        let out_2 = rd::<usize>(out_a8.as_ptr() as usize + 0x10);
                        let payload = rd::<u64>(out_0.wrapping_add(out_2 * 8));
                        let cmd9 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 9u32) -> usize);
                        wr::<u8>(cmd9, 9);
                        wr::<u64>(cmd9 + 1, payload);
                        let cmd1 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 1u32) -> usize);
                        wr::<u8>(cmd1, 0x0c);
                        wr::<u32>(entry + 0x0c, packed);
                        wr::<u32>(entry + 0x10, 0xffff_ffff);
                        wr::<u8>(flag_pair + 1, 1);
                    }
                }
                if rd::<u8>(a1.wrapping_add(off_18).wrapping_add(0x10)) != 0 {
                    let mut span: [u64; 3] = [
                        rd::<u64>(param_3),
                        rd::<u64>(param_3 + 0x08),
                        rd::<u64>(param_3 + 0x10),
                    ];
                    let mut out_a8 = [0u8; 32];
                    let _ = exe::call_va!(
                        0x1_4021_b890,
                        (out_a8.as_mut_ptr() as usize, item, span.as_mut_ptr() as usize) -> usize
                    );
                    if out_a8[0x18] != 0 {
                        let v = rd::<u64>(val_ptr);
                        let entry =
                            exe::call_va!(0x1_4020_f310, (param_4.wrapping_add(0x68)) -> usize);
                        wr::<u64>(entry, v);
                        wr::<u32>(entry + 0x08, 1);
                        let rdx_idx = rd::<u64>(param_4 + 0xa8);
                        let arr_88 = rd::<usize>(param_4 + 0x88);
                        let diff = (rd::<u32>(param_4 + 0xa0))
                            .wrapping_sub(rd::<u32>(arr_88.wrapping_add((rdx_idx as usize) * 8)));
                        let packed = diff | ((rdx_idx as u32) << 16);
                        let out_0 = rd::<usize>(out_a8.as_ptr() as usize);
                        let out_2 = rd::<usize>(out_a8.as_ptr() as usize + 0x10);
                        let payload = rd::<u64>(out_0.wrapping_add(out_2 * 8));
                        let cmd9 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 9u32) -> usize);
                        wr::<u8>(cmd9, 9);
                        wr::<u64>(cmd9 + 1, payload);
                        let cmd1 =
                            exe::call_va!(0x1_4020_bfd0, (param_4.wrapping_add(0x88), 1u32) -> usize);
                        wr::<u8>(cmd1, 0x0c);
                        wr::<u32>(entry + 0x0c, packed);
                        wr::<u32>(entry + 0x10, 0xffff_ffff);
                        wr::<u8>(flag_pair + 1, 1);
                    }
                }
                p2 = p2.wrapping_add(8);
            }

            idx += 1;
            off_18 = off_18.wrapping_add(0x18);
            off_90 = off_90.wrapping_add(0x90);
            if idx >= rd::<u64>(param_1 + 0x18) {
                break;
            }
        }
    }
}

/// `unknown_141e1ebf0`
/// Original: 0x141e1ebf0 (864 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e1ebf0(param_1: usize) {
    let mut idx: u64 = 0;
    if rd::<u64>(param_1 + 0x18) != 0 {
        let mut off_18: usize = 0;
        let mut off_90: usize = 0;
        loop {
            let a0 = rd::<usize>(param_1 + 0x20);
            let a1 = rd::<usize>(param_1 + 0x28);
            let a2 = rd::<usize>(param_1 + 0x30);
            let a3 = rd::<usize>(param_1 + 0x38);
            let r15_dst = a1.wrapping_add(off_90);
            let rdx_info = a2.wrapping_add(off_18);
            let r13_int_ptr = a3.wrapping_add((idx as usize) * 4);
            let r12_src = rd::<usize>(a0.wrapping_add((idx as usize) * 8));

            let mut rdi = rd::<usize>(r12_src);
            let rbp = rdi.wrapping_add((rd::<u32>(r12_src + 0x08) as usize) * 8);
            while rdi != rbp {
                let rbx = rd::<u64>(rdi);
                if rd::<u8>(rdx_info + 0x0f) == 0 || rd::<i32>(r13_int_ptr) == 0 {
                    let arr2 = rd::<usize>(r12_src + 0x10);
                    let len2 = rd::<u32>(r12_src + 0x18) as usize;
                    let mut found_in_arr2 = None;
                    for k in 0..len2 {
                        if rd::<u64>(arr2.wrapping_add(k * 8)) == rbx {
                            found_in_arr2 = Some(k);
                            break;
                        }
                    }
                    if let Some(pos) = found_in_arr2 {
                        let mut j = pos + 1;
                        while j < len2 {
                            let base2 = rd::<usize>(r12_src + 0x10);
                            let v = rd::<u64>(base2.wrapping_add(j * 8));
                            wr::<u64>(base2.wrapping_add((j - 1) * 8), v);
                            j += 1;
                        }
                        wr::<u32>(r12_src + 0x18, (rd::<u32>(r12_src + 0x18)).wrapping_sub(1));
                    } else {
                        let dst0_ptr = rd::<usize>(r15_dst);
                        let dst0_len = rd::<u32>(r15_dst + 0x08) as usize;
                        let mut p = dst0_ptr;
                        let end = dst0_ptr.wrapping_add(dst0_len * 8);
                        while p != end {
                            if rd::<u64>(p) == rbx {
                                break;
                            }
                            p = p.wrapping_add(8);
                        }
                        let cur_end =
                            rd::<usize>(r15_dst).wrapping_add((rd::<u32>(r15_dst + 0x08) as usize) * 8);
                        if p == cur_end {
                            let len_30 = rd::<u32>(r15_dst + 0x38) as u64;
                            let cap_30 = rd::<u32>(r15_dst + 0x3c) as u64;
                            if (len_30 << 3) == (cap_30 << 3) {
                                exe::call_va!(
                                    0x1_40f7_8430,
                                    (r15_dst.wrapping_add(0x30), len_30.wrapping_add(1))
                                );
                            }
                            let idx_30 = rd::<u32>(r15_dst + 0x38) as usize;
                            let buf_30 = rd::<usize>(r15_dst + 0x30);
                            wr::<u64>(buf_30.wrapping_add(idx_30 * 8), rbx);
                            wr::<u32>(r15_dst + 0x38, (idx_30 as u32).wrapping_add(1));

                            let len_00 = rd::<u32>(r15_dst + 0x08) as u64;
                            let cap_00 = rd::<u32>(r15_dst + 0x0c) as u64;
                            if (len_00 << 3) == (cap_00 << 3) {
                                exe::call_va!(0x1_40f7_8430, (r15_dst, len_00.wrapping_add(1)));
                            }
                            let idx_00 = rd::<u32>(r15_dst + 0x08) as usize;
                            let buf_00 = rd::<usize>(r15_dst);
                            wr::<u64>(buf_00.wrapping_add(idx_00 * 8), rbx);
                            wr::<u32>(r15_dst + 0x08, (idx_00 as u32).wrapping_add(1));
                        }
                    }
                }
                rdi = rdi.wrapping_add(8);
            }
            wr::<u32>(r12_src + 0x08, 0);

            let mut rsi = rd::<usize>(r12_src + 0x10);
            let rbp2 = rsi.wrapping_add((rd::<u32>(r12_src + 0x18) as usize) * 8);
            while rsi != rbp2 {
                let rdi_val = rd::<u64>(rsi);
                if rd::<u8>(rdx_info + 0x0f) == 0 || rd::<i32>(r13_int_ptr) == 0 {
                    let dst0_ptr = rd::<usize>(r15_dst);
                    let dst0_len = rd::<u32>(r15_dst + 0x08) as usize;
                    let mut rbx = dst0_ptr;
                    let end = dst0_ptr.wrapping_add(dst0_len * 8);
                    while rbx != end {
                        if rd::<u64>(rbx) == rdi_val {
                            break;
                        }
                        rbx = rbx.wrapping_add(8);
                    }
                    let cur_end =
                        rd::<usize>(r15_dst).wrapping_add((rd::<u32>(r15_dst + 0x08) as usize) * 8);
                    if rbx != cur_end {
                        let len_60 = rd::<u32>(r15_dst + 0x68) as u64;
                        let cap_60 = rd::<u32>(r15_dst + 0x6c) as u64;
                        if (len_60 << 3) == (cap_60 << 3) {
                            exe::call_va!(
                                0x1_40f7_8430,
                                (r15_dst.wrapping_add(0x60), len_60.wrapping_add(1))
                            );
                        }
                        let idx_60 = rd::<u32>(r15_dst + 0x68) as usize;
                        let buf_60 = rd::<usize>(r15_dst + 0x60);
                        wr::<u64>(buf_60.wrapping_add(idx_60 * 8), rdi_val);
                        wr::<u32>(r15_dst + 0x68, (idx_60 as u32).wrapping_add(1));

                        let next_ptr = rbx.wrapping_add(8);
                        let move_bytes = ((rd::<u32>(r15_dst + 0x08) as usize) * 8)
                            .wrapping_sub(next_ptr)
                            .wrapping_add(rd::<usize>(r15_dst));
                        exe::call_va!(0x1_4183_acc8, (rbx, next_ptr, move_bytes as u64));
                        wr::<u32>(r15_dst + 0x08, (rd::<u32>(r15_dst + 0x08)).wrapping_sub(1));
                    }
                }
                rsi = rsi.wrapping_add(8);
            }
            wr::<u32>(r12_src + 0x18, 0);

            idx += 1;
            off_90 = off_90.wrapping_add(0x90);
            off_18 = off_18.wrapping_add(0x18);
            if idx >= rd::<u64>(param_1 + 0x18) {
                break;
            }
        }
    }
}

/// `coregame_141e77080` (`coregame::ResourceManager::vfunc_2`)
/// Original: 0x141e77080 (173 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_141e77080(_param_1: usize, param_2: usize) {
    if param_2 != 0 && crate::simd_ops::rd_acq_u32(param_2 + 0x08) == 1 {
        return;
    }
    loop {
        if param_2 != 0 && crate::simd_ops::rd_acq_u32(param_2 + 0x08) == 1 {
            return;
        }
        let mut cur = rd::<usize>(exe::ptr::<usize>(0x1_43a4_82a8) as usize);
        let count = rd::<u32>(exe::ptr::<u32>(0x1_43a4_82b0) as usize) as usize;
        let end = cur.wrapping_add(count * 16);
        while cur != end {
            let fn_ptr = rd::<usize>(cur + 0x08);
            let arg = rd::<usize>(cur);
            exe::call_va!(fn_ptr, (arg));
            cur = cur.wrapping_add(16);
        }
        if param_2 == 0 {
            break;
        }
    }
}

/// `unknown_141e8ece0`
/// Original: 0x141e8ece0 (139 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e8ece0(param_1: usize) {
    let count = rd::<u64>(param_1 + 0x18) as usize;
    let flags_ptr = rd::<usize>(param_1 + 0x20);
    let mut items_ptr = rd::<usize>(param_1 + 0x28);
    for i in 0..count {
        let obj = rd::<usize>(items_ptr);
        let p20 = rd::<usize>(obj + 0x20);
        let active = if rd::<u16>(p20 + 0x90) == 0 {
            let p48 = rd::<usize>(obj + 0x48);
            let start = rd::<usize>(p48 + 0x128);
            let end = start.wrapping_add((rd::<u32>(p48 + 0x130) as usize).wrapping_shl(5));
            start != end
        } else {
            false
        };
        let old = rd::<u32>(flags_ptr.wrapping_add(i * 4));
        let next = if active { old | 1 } else { old & !1 };
        items_ptr = items_ptr.wrapping_add(0x20);
        wr::<u32>(flags_ptr.wrapping_add(i * 4), next);
    }
}

/// `unknown_141f45ff0`
/// Original: 0x141f45ff0 (151 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141f45ff0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let idx = rd::<usize>(param_1 + 0x10);
    let base = rd::<usize>(param_1);
    let prod = rd::<f32>(param_3) * rd::<f32>(param_4);
    let xmm1 = prod.to_bits() as u128;
    let elem = base.wrapping_add(idx.wrapping_mul(0x28));

    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4202_b8d0,
        rd::<u64>(elem),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, xmm1, 0, 0],
    );
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4202_f140,
        rd::<u64>(elem + 0x08),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, xmm1, 0, 0],
    );

    let id_020 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d020) as usize) as usize;
    let fn_9fa0 = exe::ptr::<u8>(0x1_41fd_9fa0) as usize as u64;
    let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
    let slot_ptr = rd::<usize>(tbl_580e0.wrapping_add(id_020 * 8));
    let val_020 = rd::<u64>(slot_ptr);
    let mut cb_pair: [u64; 2] = [val_020, fn_9fa0];

    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4202_ea90,
        rd::<u64>(elem + 0x10),
        0,
        cb_pair.as_mut_ptr() as usize as u64,
        0,
        0,
        0,
        0,
        0,
        [0, xmm1, 0, 0],
    );
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141ade1a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut inner_items = vec![0u8; 0x758 * 3];
    let ip = inner_items.as_mut_ptr() as usize;

    // Item 0: inline SSO string "Alpha" (b3 = 2 -> cap_bits = 16, alloc_bits = 0)
    wr::<u32>(ip, 0x0200_0000);
    wr::<u32>(ip + 0x04, 5);
    let s0 = b"Alpha\0";
    for (k, &b) in s0.iter().enumerate() {
        wr::<u8>(ip + 0x08 + k, b);
    }

    // Item 1: heap SSO string "BetaTest" (b3 = 1 -> cap_bits = 8, alloc_bits = 64)
    let heap_str = b"BetaTest\0";
    let ip1 = ip + 0x758;
    wr::<u32>(ip1, 0x0100_0008);
    wr::<u32>(ip1 + 0x04, 8);
    wr::<usize>(ip1 + 0x08, heap_str.as_ptr() as usize);

    // Item 2: empty string (len = 0)
    let ip2 = ip + 0x758 * 2;
    wr::<u32>(ip2, 0x0200_0000);
    wr::<u32>(ip2 + 0x04, 0);

    let mut outer_items = vec![0u8; 0xb8 * 2];
    let op = outer_items.as_mut_ptr() as usize;
    // Outer 0: empty
    wr::<usize>(op + 0x58, ip);
    wr::<u32>(op + 0x60, 0);
    // Outer 1: 3 items
    wr::<usize>(op + 0xb8 + 0x58, ip);
    wr::<u32>(op + 0xb8 + 0x60, 3);

    let mut ctx = vec![0u8; 0x370];
    let cp = ctx.as_mut_ptr() as usize;
    wr::<usize>(cp + 0x358, op);
    wr::<u32>(cp + 0x360, 2);

    let queries: [(&[u8], u64); 4] = [
        (&b"ALPHA"[..], 5),
        (&b"betatest"[..], 8),
        (&b""[..], 0),
        (&b"Gamma"[..], 5),
    ];

    for (q_bytes, q_len) in queries {
        let q: [u64; 2] = [q_bytes.as_ptr() as usize as u64, q_len];
        let qp = q.as_ptr() as usize;
        let o = f_o(cp, qp);
        let n = f_n(cp, qp);
        crate::diff_eq!(o, n, "141ade1a0 query");
    }
    Ok(4)
}

pub unsafe fn diff_content_141c696e0(orig: usize, new: usize) -> Result<u32, String> {
    let mut curve = [0u8; 0x60];
    let cp = curve.as_mut_ptr() as usize;
    wr::<f32>(cp + 0x14, 0.1);
    wr::<f32>(cp + 0x18, 1.25);
    wr::<f32>(cp + 0x1c, 0.8);
    wr::<f32>(cp + 0x20, 3.5);
    wr::<f32>(cp + 0x24, 0.9);
    wr::<f32>(cp + 0x2c, 0.95);
    wr::<f32>(cp + 0x34, 0.2);
    wr::<f32>(cp + 0x38, 0.4);
    wr::<f32>(cp + 0x3c, 0.3);
    wr::<f32>(cp + 0x40, 0.6);
    wr::<f32>(cp + 0x44, 0.5);
    wr::<f32>(cp + 0x48, 0.8);
    wr::<f32>(cp + 0x4c, 0.1);
    wr::<f32>(cp + 0x50, 0.2);

    let mut tests = 0u32;
    for mode in [0u32, 1, 2, 3, 4, 5] {
        wr::<u32>(cp, mode);
        for val in [0.0f32, 0.25, 0.5, 0.75, 1.0] {
            let r_o = crate::simd_ops::call_va8_xmm(
                orig as u64,
                cp as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, val.to_bits() as u128, 0, 0],
            )
            .1 as u32;
            let r_n = crate::simd_ops::call_va8_xmm(
                new as u64,
                cp as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, val.to_bits() as u128, 0, 0],
            )
            .1 as u32;
            crate::diff_eq!(r_o, r_n, "141c696e0 mode={} val={}", mode, val);
            tests += 1;
        }
    }
    Ok(tests)
}

pub unsafe fn diff_coregame_141df0600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_7350(arg1: usize, out: usize) -> usize {
        let len = rd::<u64>(arg1 + 0x08);
        wr::<u8>(out + 0x18, (len != 0) as u8);
        out
    }
    unsafe extern "win64" fn mock_aa00(dst: usize, src: usize) -> usize {
        let tag = rd::<u32>(src);
        wr::<u8>(dst + 0x08, b'K');
        wr::<u8>(dst + 0x09, b'0'.wrapping_add(tag as u8));
        wr::<u8>(dst + 0x0a, 0);
        wr::<u32>(dst + 0x04, 2);
        dst
    }
    unsafe extern "win64" fn mock_16a0(s: usize, len: u64) -> u64 {
        if len >= 2 {
            (rd::<u8>(s + 1).wrapping_sub(b'0')) as u64 + 10
        } else {
            1
        }
    }
    unsafe extern "win64" fn mock_8210(tbl: usize, key_pair: usize) -> usize {
        let h = rd::<u64>(key_pair + 0x08);
        if h == 12 {
            rd::<usize>(tbl + 0x80)
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_14f0(out_uv: usize, in_pos: usize, _g: usize, clamp: u32) {
        let x = rd::<f32>(in_pos);
        let y = rd::<f32>(in_pos + 0x04);
        let z = rd::<f32>(in_pos + 0x08);
        wr::<f32>(out_uv, x * 0.1 + z * 0.01 + (clamp as f32) * 0.05);
        wr::<f32>(out_uv + 0x04, y * 0.1 + z * 0.02);
    }
    unsafe extern "win64" fn mock_ab10(s: usize) {
        wr::<u32>(s + 0x04, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4023_7350, mock_7350 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_8210, mock_8210 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41df_14f0, mock_14f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let g_0cc = exe::ptr::<u16>(0x1_43c8_f0cc) as usize;
    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let g_a50 = exe::ptr::<usize>(0x1_4397_da50) as usize;
    let s_0cc = rd::<u16>(g_0cc);
    let s_0d8 = rd::<u16>(g_0d8);
    let s_a50 = rd::<usize>(g_a50);
    wr::<u16>(g_0cc, 0);
    wr::<u16>(g_0d8, 0);

    let mut disp_cfg = [0u8; 0x60];
    let dcp = disp_cfg.as_mut_ptr() as usize;
    wr::<i32>(dcp + 0x40, 1920);
    wr::<i32>(dcp + 0x44, 1080);
    wr::<usize>(g_a50, dcp);

    let run = |f: F| -> (u8, u32, u32) {
        let lut: [u16; 4] = [0, 0, 0, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x20];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap, 0x1234);
        wr::<u32>(ap + 0x08, 3);

        let factor_val: f32 = 0.5;
        let tbl_580e0: [usize; 1] = [(&factor_val as *const f32) as usize];

        let mut world = vec![0u8; 0x58100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let mut ref_node = [0u8; 0x20];
        let rnp = ref_node.as_mut_ptr() as usize;
        wr::<f32>(rnp + 0x08, 1.0);
        wr::<f32>(rnp + 0x0c, 2.0);
        wr::<f32>(rnp + 0x10, 3.0);

        let mut htbl_entries = [0u8; 0x28 * 4];
        let hep = htbl_entries.as_mut_ptr() as usize;
        // Put sentinel at slot (11 & 3) = 3 so probe terminates cleanly
        wr::<u64>(hep + 3 * 0x28, 0xdead);
        wr::<u64>(hep + 3 * 0x28 + 0x08, 0xbeef);

        let mut p3_tbl = [0u8; 0x90];
        let p3p = p3_tbl.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x38, 1);
        wr::<u64>(p3p + 0x40, 2);
        wr::<u64>(p3p + 0x48, 4);
        wr::<u64>(p3p + 0x50, 0xdead);
        wr::<u64>(p3p + 0x58, 0xbeef);
        wr::<usize>(p3p + 0x78, hep);
        wr::<usize>(p3p + 0x80, rnp);

        let mut p4_pos = [0u8; 0x20];
        let p4p = p4_pos.as_mut_ptr() as usize;
        wr::<f32>(p4p + 0x08, 4.0);
        wr::<f32>(p4p + 0x0c, 6.0);
        wr::<f32>(p4p + 0x10, 8.0);

        let mut p5_out = [0u8; 0xd0];
        let p5p = p5_out.as_mut_ptr() as usize;
        wr::<u8>(p5p + 0xc8, 3);

        let mut p6_items = [0u8; 0x140];
        let p6ip = p6_items.as_mut_ptr() as usize;
        // Item 0: clamp (type 4 at +0x58)
        let clamp_pair: u64 = (10.0f32.to_bits() as u64) | ((20.0f32.to_bits() as u64) << 32);
        wr::<u64>(p6ip + 0x48, clamp_pair);
        wr::<u32>(p6ip + 0x58, 4);
        // Item 1 (+0x60): str1 (type 2 at +0x58)
        wr::<u32>(p6ip + 0x60, 1);
        wr::<u32>(p6ip + 0x60 + 0x58, 2);
        // Item 2 (+0xc0): str2 (type 2 at +0x58)
        wr::<u32>(p6ip + 0xc0, 2);
        wr::<u32>(p6ip + 0xc0 + 0x58, 2);

        let mut p6_hdr = [0u8; 0x10];
        let p6p = p6_hdr.as_mut_ptr() as usize;
        wr::<usize>(p6p, p6ip);
        wr::<u32>(p6p + 0x08, 3);

        let ret = f(0, 0, p3p, p4p, p5p, p6p, wp);
        (ret, rd::<u32>(p5p + 0x80), rd::<u32>(p5p + 0x84))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_0cc, s_0cc);
    wr::<u16>(g_0d8, s_0d8);
    wr::<usize>(g_a50, s_a50);

    crate::diff_eq!(o, n, "141df0600 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141dfb350(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_LIST_3: [u16; 1] = [1];
    static mut ARCH_LIST_2: [u16; 1] = [1];
    static mut VEC_ITEMS: [u32; 1] = [77];
    static mut BFE0_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_7c10(out: usize, world_40: usize, span: usize) -> usize {
        let tag = rd::<u64>(span);
        wr::<usize>(out, world_40);
        if tag == 3 {
            let p = core::ptr::addr_of!(ARCH_LIST_3) as usize;
            wr::<usize>(out + 0x08, p);
            wr::<usize>(out + 0x10, p + 2);
        } else {
            let p = core::ptr::addr_of!(ARCH_LIST_2) as usize;
            wr::<usize>(out + 0x08, p);
            wr::<usize>(out + 0x10, p + 2);
        }
        out
    }
    unsafe extern "win64" fn mock_find(_p: u64, _n: u32, _want: u16) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_aaa0(_comp: usize, _p3: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_a9a0(_comp: usize, _sview: usize) -> i32 {
        5
    }
    unsafe extern "win64" fn mock_d380(out: usize, _elem: usize, _p2: usize) -> usize {
        wr::<usize>(out, core::ptr::addr_of_mut!(VEC_ITEMS) as usize);
        wr::<u32>(out + 0x08, 1);
        wr::<u32>(out + 0x0c, 1);
        out
    }
    unsafe extern "win64" fn mock_bfe0(_elem: usize, _p2: usize) {
        unsafe { BFE0_CALLED += 1 };
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41df_aaa0, mock_aaa0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41df_a9a0, mock_a9a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40f4_d380, mock_d380 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41df_bfe0, mock_bfe0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F| -> (i32, u8, u32) {
        unsafe { BFE0_CALLED = 0 };

        let mut comp3 = [0u8; 0x140];
        let c3p = comp3.as_mut_ptr() as usize;
        wr::<i32>(c3p + 0x11c, 5);

        let mut comp2 = [0u8; 0x40];
        let c2p = comp2.as_mut_ptr() as usize;
        wr::<u32>(c2p, 0x0200_0000);
        wr::<u32>(c2p + 0x04, 4);
        for (i, &b) in b"Test".iter().enumerate() {
            wr::<u8>(c2p + 0x08 + i, b);
        }
        wr::<u32>(c2p + 0x28, 77);
        wr::<u8>(c2p + 0x34, 0);

        let off_tbl3: [u32; 1] = [0x100];
        let mut world3 = vec![0u8; 0x19000];
        let w3p = world3.as_mut_ptr() as usize;
        wr::<usize>(w3p + 0x40 + 8 + 0x48, c3p.wrapping_sub(0x100));
        wr::<u32>(w3p + 0x40 + 4 + 0x10048, 1);
        wr::<usize>(w3p + 0x40 + (1 + 0xc03) * 0x20, off_tbl3.as_ptr() as usize);

        let off_tbl2: [u32; 1] = [0x100];
        let mut world2 = vec![0u8; 0x19000];
        let w2p = world2.as_mut_ptr() as usize;
        wr::<usize>(w2p + 0x40 + 8 + 0x48, c2p.wrapping_sub(0x100));
        wr::<u32>(w2p + 0x40 + 4 + 0x10048, 1);
        wr::<usize>(w2p + 0x40 + (1 + 0xc03) * 0x20, off_tbl2.as_ptr() as usize);

        let mut elem = [0u8; 0x6c * 2];
        let ep = elem.as_mut_ptr() as usize;
        let e1 = ep + 0x6c;
        wr::<u32>(e1 + 0x28, 0x0200_0000);
        wr::<u32>(e1 + 0x2c, 4);
        for (i, &b) in b"test".iter().enumerate() {
            wr::<u8>(e1 + 0x30 + i, b);
        }
        wr::<i32>(e1 + 0x64, -1);
        wr::<u8>(e1 + 0x68, 1);
        wr::<u8>(e1 + 0x69, 0);

        let p1: [usize; 3] = [ep, 0, 1];
        let p2: [u64; 3] = [w2p as u64, 2, 0];
        let p3: [u64; 3] = [w3p as u64, 3, 0];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
        );
        (rd::<i32>(e1 + 0x64), rd::<u8>(e1 + 0x68), unsafe { BFE0_CALLED })
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141dfb350 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e009b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SINGLETON_BUF: [u8; 0x5ab0] = [0; 0x5ab0];

    unsafe extern "win64" fn mock_8920() -> usize {
        core::ptr::addr_of_mut!(SINGLETON_BUF) as usize
    }
    unsafe extern "win64" fn mock_5140(p1: usize, p3: usize, flag_ptr: usize) {
        if p1 == 1 && p3 == 99 {
            wr::<u8>(flag_ptr, 1);
        }
    }
    unsafe extern "win64" fn mock_5790(p2: usize, p3: usize, flag_ptr: usize) {
        if p2 == 2 && p3 == 99 {
            wr::<u8>(flag_ptr, 0);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_420e_8920, mock_8920 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f5_5140, mock_5140 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40f5_5790, mock_5790 as *const ())?;

    let g_0f8 = exe::ptr::<u64>(0x1_43a3_40f8) as usize;
    let s_0f8 = rd::<u64>(g_0f8);
    wr::<u64>(g_0f8, 1);

    let run = |f: F, init_val: u32, p1: usize, p2: usize| -> u32 {
        let sb = core::ptr::addr_of_mut!(SINGLETON_BUF) as usize;
        wr::<u32>(sb + 0x5aa0, init_val);
        f(p1, p2, 99);
        rd::<u32>(sb + 0x5aa0)
    };

    for (init_v, p1, p2) in [(0x1234_0000u32, 1usize, 0usize), (0x1234_0200u32, 1usize, 2usize)] {
        let o = run(f_o, init_v, p1, p2);
        let n = run(f_n, init_v, p1, p2);
        if o != n {
            wr::<u64>(g_0f8, s_0f8);
            return Err(format!("141e009b0 mismatch: {o:#x} != {n:#x}"));
        }
    }
    wr::<u64>(g_0f8, s_0f8);
    Ok(2)
}

pub unsafe fn diff_unknown_141e1c970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PAYLOAD_ARR: [u64; 2] = [0x1111_2222_3333_4444, 0x5555_6666_7777_8888];

    unsafe extern "win64" fn mock_b890(out: usize, item: u64, span: usize) -> usize {
        let s0 = rd::<u64>(span);
        wr::<usize>(out, core::ptr::addr_of!(PAYLOAD_ARR) as usize);
        wr::<u64>(out + 0x10, item & 1);
        wr::<u8>(out + 0x18, (s0 == 0xaa) as u8);
        out
    }
    unsafe extern "win64" fn mock_f310(p4_68: usize) -> usize {
        let cur = rd::<usize>(p4_68);
        wr::<usize>(p4_68, cur + 0x18);
        cur
    }
    unsafe extern "win64" fn mock_bfd0(p4_88: usize, sz: u32) -> usize {
        let cur = rd::<usize>(p4_88 + 0x08);
        wr::<usize>(p4_88 + 0x08, cur + (sz as usize));
        cur
    }

    let _m1 = crate::difftest::Mock::new(0x1_4021_b890, mock_b890 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4020_f310, mock_f310 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4020_bfd0, mock_bfd0 as *const ())?;

    let run = |f: F| -> ([u8; 4], [u8; 0x60], [u8; 0x40]) {
        let mut flags = [0u8; 4];
        let mut info = [0u8; 0x18];
        info[0x10] = 1;
        info[0x11] = 1;

        let items_30: [u64; 1] = [0];
        let items_60: [u64; 1] = [1];
        let mut block_90 = [0u8; 0x90];
        let bp = block_90.as_mut_ptr() as usize;
        wr::<usize>(bp + 0x30, items_30.as_ptr() as usize);
        wr::<u32>(bp + 0x38, 1);
        wr::<usize>(bp + 0x60, items_60.as_ptr() as usize);
        wr::<u32>(bp + 0x68, 1);

        let vals: [u64; 1] = [0x9999_aaaa_bbbb_cccc];
        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 1);
        wr::<usize>(p1p + 0x20, flags.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, info.as_ptr() as usize);
        wr::<usize>(p1p + 0x30, bp);
        wr::<usize>(p1p + 0x38, vals.as_ptr() as usize);

        let p2: [u64; 3] = [0xaa, 1, 2];
        let p3: [u64; 3] = [0xbb, 3, 4];

        let mut entries_sink = [0u8; 0x60];
        let mut cmd_sink = [0u8; 0x40];
        let base_offs: [u64; 2] = [0, 10];
        let mut p4 = [0u8; 0xb8];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x68, entries_sink.as_mut_ptr() as usize);
        wr::<usize>(p4p + 0x88, base_offs.as_ptr() as usize);
        wr::<usize>(p4p + 0x90, cmd_sink.as_mut_ptr() as usize);
        wr::<u32>(p4p + 0xa0, 25);
        wr::<u64>(p4p + 0xa8, 1);

        f(p1p, p2.as_ptr() as usize, p3.as_ptr() as usize, p4p);
        (flags, entries_sink, cmd_sink)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e1c970 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e1ebf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u64; 4], [u64; 4], [u64; 4], u32, u32, u32, u32, u32) {
        let mut dst0 = [0x100u64, 0x200, 0, 0];
        let mut dst30 = [0u64; 4];
        let mut dst60 = [0u64; 4];

        let mut r15_dst = [0u8; 0x90];
        let dp = r15_dst.as_mut_ptr() as usize;
        wr::<usize>(dp, dst0.as_mut_ptr() as usize);
        wr::<u32>(dp + 0x08, 2);
        wr::<u32>(dp + 0x0c, 4);
        wr::<usize>(dp + 0x30, dst30.as_mut_ptr() as usize);
        wr::<u32>(dp + 0x38, 0);
        wr::<u32>(dp + 0x3c, 4);
        wr::<usize>(dp + 0x60, dst60.as_mut_ptr() as usize);
        wr::<u32>(dp + 0x68, 0);
        wr::<u32>(dp + 0x6c, 4);

        let mut src_add = [0x300u64, 0x400];
        let mut src_rem = [0x300u64, 0x100];
        let mut r12_src = [0u8; 0x20];
        let sp = r12_src.as_mut_ptr() as usize;
        wr::<usize>(sp, src_add.as_mut_ptr() as usize);
        wr::<u32>(sp + 0x08, 2);
        wr::<usize>(sp + 0x10, src_rem.as_mut_ptr() as usize);
        wr::<u32>(sp + 0x18, 2);

        let a0_arr: [usize; 1] = [sp];
        let info = [0u8; 0x18];
        let a3_arr: [i32; 1] = [0];

        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 1);
        wr::<usize>(p1p + 0x20, a0_arr.as_ptr() as usize);
        wr::<usize>(p1p + 0x28, dp);
        wr::<usize>(p1p + 0x30, info.as_ptr() as usize);
        wr::<usize>(p1p + 0x38, a3_arr.as_ptr() as usize);

        f(p1p);
        (
            dst0,
            dst30,
            dst60,
            rd::<u32>(dp + 0x08),
            rd::<u32>(dp + 0x38),
            rd::<u32>(dp + 0x68),
            rd::<u32>(sp + 0x08),
            rd::<u32>(sp + 0x18),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e1ebf0 out");
    Ok(1)
}

pub unsafe fn diff_coregame_141e77080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_cb(ctx: usize) {
        let c = rd::<u32>(ctx).wrapping_add(1);
        wr::<u32>(ctx, c);
        if c >= 2 {
            wr::<u32>(ctx + 0x08, 1);
        }
    }

    let g_2a8 = exe::ptr::<usize>(0x1_43a4_82a8) as usize;
    let g_2b0 = exe::ptr::<u32>(0x1_43a4_82b0) as usize;
    let s_2a8 = rd::<usize>(g_2a8);
    let s_2b0 = rd::<u32>(g_2b0);

    let run = |f: F, mode: u32| -> u32 {
        let mut state = [0u32; 4];
        let sp = state.as_mut_ptr() as usize;
        let cb_entry: [usize; 2] = [sp, mock_cb as *const () as usize];
        wr::<usize>(g_2a8, cb_entry.as_ptr() as usize);
        wr::<u32>(g_2b0, 1);

        match mode {
            0 => f(0, 0),
            1 => {
                wr::<u32>(sp + 0x08, 1);
                f(0, sp);
            }
            _ => {
                wr::<u32>(sp + 0x08, 0);
                f(0, sp);
            }
        }
        rd::<u32>(sp)
    };

    for m in [0u32, 1, 2] {
        let o = run(f_o, m);
        let n = run(f_n, m);
        if o != n {
            wr::<usize>(g_2a8, s_2a8);
            wr::<u32>(g_2b0, s_2b0);
            return Err(format!("141e77080 mode {m}: {o} != {n}"));
        }
    }
    wr::<usize>(g_2a8, s_2a8);
    wr::<u32>(g_2b0, s_2b0);
    Ok(3)
}

pub unsafe fn diff_unknown_141e8ece0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> [u32; 3] {
        let mut p20_a = [0u8; 0xa0];
        wr::<u16>(p20_a.as_mut_ptr() as usize + 0x90, 0);
        let mut p48_a = [0u8; 0x140];
        wr::<usize>(p48_a.as_mut_ptr() as usize + 0x128, 0x1000);
        wr::<u32>(p48_a.as_mut_ptr() as usize + 0x130, 2);
        let mut obj_a = [0u8; 0x50];
        wr::<usize>(obj_a.as_mut_ptr() as usize + 0x20, p20_a.as_ptr() as usize);
        wr::<usize>(obj_a.as_mut_ptr() as usize + 0x48, p48_a.as_ptr() as usize);

        let mut p20_b = [0u8; 0xa0];
        wr::<u16>(p20_b.as_mut_ptr() as usize + 0x90, 1);
        let mut obj_b = [0u8; 0x50];
        wr::<usize>(obj_b.as_mut_ptr() as usize + 0x20, p20_b.as_ptr() as usize);

        let mut p48_c = [0u8; 0x140];
        wr::<usize>(p48_c.as_mut_ptr() as usize + 0x128, 0x2000);
        wr::<u32>(p48_c.as_mut_ptr() as usize + 0x130, 0);
        let mut obj_c = [0u8; 0x50];
        wr::<usize>(obj_c.as_mut_ptr() as usize + 0x20, p20_a.as_ptr() as usize);
        wr::<usize>(obj_c.as_mut_ptr() as usize + 0x48, p48_c.as_ptr() as usize);

        let mut items = [0u64; 12];
        items[0] = obj_a.as_ptr() as usize as u64;
        items[4] = obj_b.as_ptr() as usize as u64;
        items[8] = obj_c.as_ptr() as usize as u64;

        let mut flags: [u32; 3] = [0x10, 0x11, 0x13];
        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 3);
        wr::<usize>(p1p + 0x20, flags.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, items.as_ptr() as usize);

        f(p1p);
        flags
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e8ece0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f45ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut REC: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_b8d0(rcx: u64) {
        let xmm = crate::simd_ops::take_xmm_args();
        unsafe {
            REC[0] = rcx;
            REC[1] = xmm[1] as u32 as u64;
        }
    }
    unsafe extern "win64" fn mock_f140(rcx: u64) {
        let xmm = crate::simd_ops::take_xmm_args();
        unsafe {
            REC[2] = rcx;
            REC[3] = xmm[1] as u32 as u64;
        }
    }
    unsafe extern "win64" fn mock_ea90(rcx: u64, _rdx: u64, r8: usize) {
        let xmm = crate::simd_ops::take_xmm_args();
        unsafe {
            REC[4] = rcx;
            REC[5] = xmm[1] as u32 as u64;
            REC[6] = rd::<u64>(r8);
            REC[7] = rd::<u64>(r8 + 0x08);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4202_b8d0, mock_b8d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4202_f140, mock_f140 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4202_ea90, mock_ea90 as *const ())?;

    let g_020 = exe::ptr::<u16>(0x1_43c8_d020) as usize;
    let s_020 = rd::<u16>(g_020);
    wr::<u16>(g_020, 1);

    let run = |f: F| -> [u64; 8] {
        unsafe { REC = [0; 8] };
        let mut elems = [0u64; 10];
        elems[5] = 0xaaaa_0001;
        elems[6] = 0xbbbb_0002;
        elems[7] = 0xcccc_0003;

        let p1: [usize; 3] = [elems.as_ptr() as usize, 0, 1];
        let slot_val: u64 = 0x7777_8888_9999_0000;
        let tbl_580e0: [usize; 2] = [0, (&slot_val as *const u64) as usize];
        let mut world = vec![0u8; 0x58100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let a: f32 = 2.5;
        let b: f32 = 3.0;
        f(
            p1.as_ptr() as usize,
            wp,
            (&a as *const f32) as usize,
            (&b as *const f32) as usize,
        );
        unsafe { REC }
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_020, s_020);

    crate::diff_eq!(o, n, "141f45ff0 out");
    Ok(1)
}

// REGISTRY (for games/alan-wake-2/crates/aw2-game/src/registry/core.rs):
// FLOAT_RETURN: 0x1_41c6_96e0 (`content_141c696e0` returns `f32` in `xmm0`)
// Replacement::verified(0x1_41ad_e1a0, "unknown_141ade1a0", rend::cauldron_hot_q::unknown_141ade1a0 as *const (), rend::cauldron_hot_q::diff_unknown_141ade1a0),
// Replacement::verified(0x1_41c6_96e0, "content_141c696e0", rend::cauldron_hot_q::content_141c696e0 as *const (), rend::cauldron_hot_q::diff_content_141c696e0),
// Replacement::verified_throws(0x1_41df_0600, "coregame_141df0600", rend::cauldron_hot_q::coregame_141df0600 as *const (), Throws::Caught, rend::cauldron_hot_q::diff_coregame_141df0600),
// Replacement::verified_throws(0x1_41df_b350, "unknown_141dfb350", rend::cauldron_hot_q::unknown_141dfb350 as *const (), Throws::Discard, rend::cauldron_hot_q::diff_unknown_141dfb350),
// Replacement::verified(0x1_41e0_09b0, "unknown_141e009b0", rend::cauldron_hot_q::unknown_141e009b0 as *const (), rend::cauldron_hot_q::diff_unknown_141e009b0),
// Replacement::verified(0x1_41e1_c970, "unknown_141e1c970", rend::cauldron_hot_q::unknown_141e1c970 as *const (), rend::cauldron_hot_q::diff_unknown_141e1c970),
// Replacement::verified(0x1_41e1_ebf0, "unknown_141e1ebf0", rend::cauldron_hot_q::unknown_141e1ebf0 as *const (), rend::cauldron_hot_q::diff_unknown_141e1ebf0),
// Replacement::verified(0x1_41e7_7080, "coregame_141e77080", rend::cauldron_hot_q::coregame_141e77080 as *const (), rend::cauldron_hot_q::diff_coregame_141e77080),
// Replacement::verified(0x1_41e8_ece0, "unknown_141e8ece0", rend::cauldron_hot_q::unknown_141e8ece0 as *const (), rend::cauldron_hot_q::diff_unknown_141e8ece0),
// Replacement::verified_throws(0x1_41f4_5ff0, "unknown_141f45ff0", rend::cauldron_hot_q::unknown_141f45ff0 as *const (), Throws::Discard, rend::cauldron_hot_q::diff_unknown_141f45ff0),
