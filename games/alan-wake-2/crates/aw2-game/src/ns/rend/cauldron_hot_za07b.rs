//! Decompiled `rend` hot functions shard ZA07b (`0x141aca620..0x141df7140`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{vaddps, vblendps, vinsertps, vmulps, vpermilps, vsubps};

#[inline(always)]
unsafe fn rstring_ptr(s: usize) -> usize {
    let hdr = rd::<u32>(s);
    let b3 = hdr >> 24;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (hdr & 0x00ff_ffff) << 3;
    if len_bits > cap_bits {
        rd::<usize>(s + 0x08)
    } else {
        s.wrapping_add(0x08)
    }
}

/// `unknown_141aca620`
/// Original: 0x141aca620 (1660 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141aca620(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) {
    if rd::<u8>(param_5 + 0x424) == 0 {
        return;
    }

    let r12 = rd::<u64>(param_1 + 0x58);
    let r14 = rd::<usize>(param_1);
    let rbx = r14.wrapping_add((r12.wrapping_mul(0x630)) as usize);
    let rax_98 = rd::<usize>(rbx + 0x98);
    if rd::<u8>(rax_98 + 0x98) == 0
        || rd::<u8>(rbx + 0xa1) == 0
        || rd::<u32>(rbx + 0x31c) == 0
    {
        return;
    }

    let rsi_str = rstring_ptr(rbx + 0x318);
    let rdx_str = rstring_ptr(rbx + 0x48);
    let len_4c = rd::<u32>(rbx + 0x4c) as u64;
    let mut view_20: [u64; 2] = [rdx_str as u64, len_4c];
    let node1 = exe::call_va!(
        0x1_41ad_cb20,
        (param_5, view_20.as_mut_ptr() as usize) -> usize
    );
    if node1 == 0 {
        return;
    }

    let len_31c = rd::<u32>(rbx + 0x31c) as u64;
    view_20 = [rsi_str as u64, len_31c];
    let node2 = exe::call_va!(
        0x1_41ac_8bc0,
        (node1, view_20.as_mut_ptr() as usize) -> usize
    );
    if node2 == 0 {
        return;
    }

    let raw_c8 = rd::<u64>(node2 + 0xc8);
    let rdx_handle = if (raw_c8 as u32) != 0xffff_ffff {
        raw_c8
    } else {
        u64::MAX
    };

    let mut lookup_out = [0u64; 5];
    let lookup_ptr = lookup_out.as_mut_ptr() as usize;
    if (rdx_handle as u32) == 0xffff_ffff && ((rdx_handle >> 32) as u32) == 0xffff_ffff {
        wr::<u8>(lookup_ptr + 0x20, 0);
    } else {
        let p2_0 = rd::<usize>(param_2);
        let r8_idx = (rdx_handle as u32) as u64;
        let cap_20 = rd::<u32>(p2_0 + 0x20) as u64;
        let mut valid = false;
        if r8_idx < cap_20 {
            let p18 = rd::<usize>(p2_0 + 0x18);
            let ent = rd::<u64>(p18.wrapping_add((r8_idx as usize) * 8));
            if (ent as u32) == (rdx_handle as u32)
                && ((ent >> 32) as u32) == ((rdx_handle >> 32) as u32)
                && (rdx_handle as u32) != 0xffff_ffff
            {
                valid = true;
            }
        }
        if valid {
            let mut p2_copy = rd::<[u64; 3]>(param_2);
            exe::call_va!(
                0x1_4059_29e0,
                (lookup_ptr, rdx_handle, p2_copy.as_mut_ptr() as usize)
            );
        } else {
            wr::<u8>(lookup_ptr + 0x20, 0);
        }
    }

    if rd::<u8>(lookup_ptr + 0x20) == 0 {
        return;
    }

    let p1_08 = rd::<usize>(param_1 + 0x08);
    let mut xmm4_bits = rd::<u32>(param_5 + 0x4c);
    let idx_ent = rd::<u64>(lookup_ptr + 0x18);
    let rdx_tr = (rd::<usize>(lookup_ptr)).wrapping_add((idx_ent as usize).wrapping_shl(5));
    if rd::<u32>(p1_08.wrapping_add((r12 as usize) * 4)) == 0 {
        xmm4_bits ^= rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    }

    let xmm7_q1 = rd::<u128>(rdx_tr);
    let rax_90 = rd::<usize>(rbx + 0x90);
    let rcx_1c = (idx_ent.wrapping_mul(0x1c)) as usize;
    let xmm11 = rd::<f32>(rax_90 + 0x154);
    let xmm10 = rd::<f32>(rax_90 + 0x150);
    let base1 = rd::<usize>(lookup_ptr + 0x08);

    let xmm6_q1 = vpermilps(xmm7_q1, 0xc9);
    let v_y = (rd::<f32>(base1.wrapping_add(rcx_1c + 0x04)) - xmm11) - rd::<f32>(rbx + 0x154);
    let v_x = (rd::<f32>(base1.wrapping_add(rcx_1c)) - xmm10) - rd::<f32>(rbx + 0x150);

    let mut xmm8: u128 = 0;
    xmm8 = vblendps(xmm8, v_x.to_bits() as u128, 0x1);
    xmm8 = vinsertps(xmm8, v_y.to_bits() as u128, 0x10);
    xmm8 = vinsertps(xmm8, xmm4_bits as u128, 0x20);

    let c_7ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let xmm4_q1 = vpermilps(xmm7_q1, 0xd2);
    let cross1 = vsubps(
        vmulps(vpermilps(xmm8, 0xd2), xmm6_q1),
        vmulps(vpermilps(xmm8, 0xc9), xmm4_q1),
    );
    let xmm5_t1 = vmulps(cross1, c_7ab0);
    let cross2 = vsubps(
        vmulps(vpermilps(xmm5_t1, 0xd2), xmm6_q1),
        vmulps(vpermilps(xmm5_t1, 0xc9), xmm4_q1),
    );
    let q1_ff = vpermilps(xmm7_q1, 0xff);
    let xmm7_q2 = rd::<u128>(param_5 + 0x3c0);
    let xmm0_600 = rd::<u128>(rbx + 0x600);
    xmm8 = vaddps(
        vaddps(cross2, vaddps(vmulps(q1_ff, xmm5_t1), xmm8)),
        rd::<u128>(rdx_tr + 0x10),
    );

    let xmm4_q2 = vpermilps(xmm7_q2, 0xd2);
    let xmm6_q2 = vpermilps(xmm7_q2, 0xc9);
    let cross3 = vsubps(
        vmulps(vpermilps(xmm8, 0xd2), xmm6_q2),
        vmulps(vpermilps(xmm8, 0xc9), xmm4_q2),
    );
    let xmm5_t2 = vmulps(cross3, c_7ab0);
    let cross4 = vsubps(
        vmulps(vpermilps(xmm5_t2, 0xd2), xmm6_q2),
        vmulps(vpermilps(xmm5_t2, 0xc9), xmm4_q2),
    );
    let q2_ff = vpermilps(xmm7_q2, 0xff);
    let xmm1_fin = vaddps(
        vaddps(cross4, vaddps(vmulps(q2_ff, xmm5_t2), xmm8)),
        rd::<u128>(param_5 + 0x3d0),
    );

    let pad = rd::<f32>(exe::ptr::<f32>(0x1_4342_64c0) as usize);
    let ext_y = xmm11 + pad;
    let ext_x = xmm10 + pad;
    let fin_x = f32::from_bits(xmm1_fin as u32);
    let fin_y = f32::from_bits(vpermilps(xmm1_fin, 0x55) as u32);

    let max_x = ext_x + fin_x;
    let max_y = ext_y + fin_y;
    let min_x = fin_x - ext_x;
    let min_y = fin_y - ext_y;

    let out_1c = rd::<usize>(param_1 + 0x18).wrapping_add((r12.wrapping_mul(0x1c)) as usize);
    wr::<f32>(out_1c + 0x0c, min_x);
    wr::<f32>(out_1c + 0x10, min_y);
    wr::<f32>(out_1c + 0x14, max_x);
    wr::<f32>(out_1c + 0x18, max_y);

    let mut pair_a0: [u128; 2] = [xmm0_600, xmm8];
    if rd::<u8>(rbx + 0x5f2) != 0 {
        let mut ctx_40: [u64; 5] = [
            r14 as u64,
            rd::<u64>(param_1 + 0x28),
            rd::<u64>(param_1 + 0x48),
            rd::<u64>(param_1 + 0x50),
            r12,
        ];
        exe::call_va!(
            0x1_41ad_8220,
            (
                ctx_40.as_mut_ptr() as usize,
                param_5,
                pair_a0.as_mut_ptr() as usize,
                param_6
            )
        );
    } else {
        let r14_10 = rd::<u64>(param_1 + 0x10);
        let rdi_50 = rd::<u64>(param_1 + 0x50);
        let rsi_38 = rd::<u64>(param_1 + 0x38);
        let rbx_30 = rd::<u64>(param_1 + 0x30);

        let mut pair_70 = pair_a0;
        let mut p3_copy = rd::<[u64; 3]>(param_3);
        let mut ctx_c0_1: [u64; 5] = [
            r14_10,
            rd::<u64>(param_1 + 0x20),
            rd::<u64>(param_1 + 0x28),
            rdi_50,
            r12,
        ];
        exe::call_va!(
            0x1_41e2_34c0,
            (
                ctx_c0_1.as_mut_ptr() as usize,
                p3_copy.as_mut_ptr() as usize,
                pair_70.as_mut_ptr() as usize
            )
        );

        pair_70 = pair_a0;
        let mut p4_copy = rd::<[u64; 3]>(param_4);
        let mut ctx_c0_2: [u64; 5] = [r14_10, rbx_30, rsi_38, rdi_50, r12];
        exe::call_va!(
            0x1_41e2_2a40,
            (
                ctx_c0_2.as_mut_ptr() as usize,
                p4_copy.as_mut_ptr() as usize,
                pair_70.as_mut_ptr() as usize
            )
        );

        let p40 = rd::<usize>(param_1 + 0x40);
        wr::<u8>(p40.wrapping_add(r12 as usize), 1);
    }
}

/// `unknown_141acb3c0`
/// Original: 0x141acb3c0 (777 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141acb3c0(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    _param_4: usize,
    param_5: usize,
) {
    if rd::<u8>(param_5 + 0x424) == 0 {
        return;
    }

    let rsi = rd::<u64>(param_1 + 0x18);
    let r13 = rd::<usize>(param_1).wrapping_add((rsi.wrapping_mul(0x630)) as usize);
    let rax_98 = rd::<usize>(r13 + 0x98);
    if rd::<u8>(rax_98 + 0x98) == 0
        || rd::<u8>(r13 + 0xa1) == 0
        || rd::<u8>(r13 + 0xa7) == 0
    {
        return;
    }

    let mut p2_sub = rd::<[u64; 2]>(param_2 + 0x08);
    let r11_world = rd::<usize>(param_2).wrapping_add(0x40);
    wr::<u32>(r13 + 0x618, 0);
    let r12_box = rd::<usize>(param_1 + 0x08).wrapping_add((rsi.wrapping_mul(0x1c)) as usize);

    let mut range_out = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            range_out.as_mut_ptr() as usize,
            r11_world,
            p2_sub.as_mut_ptr() as usize
        )
    );

    let mut iter = [0u64; 9];
    let iter_ptr = iter.as_mut_ptr() as usize;
    wr::<usize>(iter_ptr, r11_world);
    wr::<u64>(iter_ptr + 0x08, range_out[0]);
    wr::<u64>(iter_ptr + 0x10, range_out[1]);
    wr::<u64>(iter_ptr + 0x18, range_out[2]);
    wr::<u64>(iter_ptr + 0x28, 0);
    wr::<u64>(iter_ptr + 0x30, 0);

    if range_out[1] != range_out[2] {
        let arch_idx = rd::<u16>(range_out[1] as usize) as usize;
        let block = rd::<u64>(r11_world.wrapping_add(arch_idx * 8 + 0x48));
        wr::<u64>(iter_ptr + 0x20, block);
        let cnt = if block != 0 {
            rd::<u32>(r11_world.wrapping_add(arch_idx * 4 + 0x10048)) as u64
        } else {
            0
        };
        wr::<u64>(iter_ptr + 0x28, cnt);
    }

    exe::call_va!(0x1_4058_94d0, (iter_ptr));
    let mut rbx = rd::<u64>(iter_ptr + 0x30);
    let mut r15 = rd::<u64>(iter_ptr + 0x28);
    if rbx >= r15 {
        return;
    }

    let p1_10 = rd::<usize>(param_1 + 0x10);
    loop {
        let block_ptr = rd::<usize>(iter_ptr + 0x20);
        let self_h = rd::<u64>(p1_10.wrapping_add((rsi as usize) * 8 + 0x10));
        let other_h = rd::<u64>(block_ptr.wrapping_add((rbx as usize) * 8 + 0x10));
        if (other_h as u32) != (self_h as u32)
            || ((other_h >> 32) as u32) != ((self_h >> 32) as u32)
        {
            let rdi_comp0 =
                rd::<usize>(iter_ptr + 0x38).wrapping_add((rbx.wrapping_mul(0x630)) as usize);
            if rd::<u8>(rdi_comp0 + 0xa1) != 0 && rd::<u8>(rdi_comp0 + 0xa7) != 0 {
                let rsi_comp1 =
                    rd::<usize>(iter_ptr + 0x40).wrapping_add((rbx.wrapping_mul(0x1c)) as usize);
                let eq = exe::call_va!(0x1_401e_6ff0, (r13 + 0x48, rdi_comp0 + 0x48) -> u8);
                if eq != 0
                    && rd::<f32>(r12_box + 0x14) >= rd::<f32>(rsi_comp1 + 0x0c)
                    && rd::<f32>(r12_box + 0x18) >= rd::<f32>(rsi_comp1 + 0x10)
                    && rd::<f32>(rsi_comp1 + 0x14) >= rd::<f32>(r12_box + 0x0c)
                    && rd::<f32>(rsi_comp1 + 0x18) >= rd::<f32>(r12_box + 0x10)
                {
                    let other_90 = rd::<usize>(rdi_comp0 + 0x90);
                    let self_90 = rd::<usize>(r13 + 0x90);
                    if rd::<i32>(self_90 + 0x73c) > rd::<i32>(other_90 + 0x73c) {
                        let vec_ptr = r13 + 0x610;
                        let mut val_slot: u64 = other_h;
                        let len = rd::<u32>(vec_ptr + 0x08);
                        let data = rd::<usize>(vec_ptr);
                        let end_ptr = data.wrapping_add((len as usize) * 8);
                        let cap = rd::<u32>(vec_ptr + 0x0c) & 0x3fff_ffff;
                        if len == cap {
                            let mut pos_slot: usize = end_ptr;
                            exe::call_va!(
                                0x1_4021_1010,
                                (
                                    vec_ptr,
                                    (&mut pos_slot as *mut usize) as usize,
                                    (&mut val_slot as *mut u64) as usize
                                )
                            );
                        } else {
                            wr::<u32>(vec_ptr + 0x08, len.wrapping_add(1));
                            wr::<u64>(end_ptr, other_h);
                        }
                    }
                }
            }
        }

        rbx = rbx.wrapping_add(1);
        wr::<u64>(iter_ptr + 0x30, rbx);
        if rbx >= r15 {
            let mut cur_arch = rd::<usize>(iter_ptr + 0x10).wrapping_add(2);
            let end_arch = rd::<usize>(iter_ptr + 0x18);
            wr::<usize>(iter_ptr + 0x10, cur_arch);
            if cur_arch != end_arch {
                let world1 = rd::<usize>(iter_ptr + 0x08);
                while cur_arch != end_arch {
                    let arch_idx = rd::<u16>(cur_arch) as usize;
                    if rd::<usize>(world1.wrapping_add(arch_idx * 8 + 0x48)) != 0
                        && rd::<u32>(world1.wrapping_add(arch_idx * 4 + 0x10048)) != 0
                    {
                        break;
                    }
                    cur_arch = cur_arch.wrapping_add(2);
                    wr::<usize>(iter_ptr + 0x10, cur_arch);
                }
                if cur_arch != end_arch {
                    let arch_idx = rd::<u16>(cur_arch) as usize;
                    let world0 = rd::<usize>(iter_ptr);
                    let block = rd::<u64>(world0.wrapping_add(arch_idx * 8 + 0x48));
                    wr::<u64>(iter_ptr + 0x20, block);
                    let cnt = if block != 0 {
                        rd::<u32>(world0.wrapping_add(arch_idx * 4 + 0x10048)) as u64
                    } else {
                        0
                    };
                    wr::<u64>(iter_ptr + 0x28, cnt);
                    wr::<u64>(iter_ptr + 0x30, 0);
                }
            }
            exe::call_va!(0x1_4058_94d0, (iter_ptr));
            rbx = rd::<u64>(iter_ptr + 0x30);
            r15 = rd::<u64>(iter_ptr + 0x28);
            if rbx >= r15 {
                break;
            }
        }
    }
}

/// `bonsai_141b923e0`
/// Original: 0x141b923e0 (241 bytes, Throws::Discard)
pub unsafe extern "win64" fn bonsai_141b923e0(param_1: usize, param_2: usize) {
    let g_c0e = exe::ptr::<u8>(0x1_43a2_8c0e) as usize;
    let g_e14 = exe::ptr::<u8>(0x1_43c8_9e14) as usize;
    let r11b = rd::<u8>(g_c0e);
    if rd::<u8>(g_e14) != r11b {
        let xmm2 = rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize);
        let xmm0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6dc0) as usize);
        let xmm1 = if r11b == 0 { xmm2 } else { xmm0 };
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4211_d5c0,
            param_1 as u64,
            param_2 as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [xmm0 as u128, xmm1 as u128, 0, 0],
        );
        wr::<u8>(g_e14, r11b);
    }

    if rd::<u8>(param_1 + 0x1c8) == 0 {
        return;
    }

    let dt_bits = rd::<u32>(param_2);
    let dt = f32::from_bits(dt_bits);
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4170_6260,
        rd::<u64>(param_1),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, dt_bits as u128, 0, 0],
    );

    let mut rbx = param_1.wrapping_add(0x40);
    for _ in 0..8 {
        let rcx = rd::<usize>(rbx);
        if rcx != 0 && rd::<i32>(rcx + 0x18) != -1 {
            if rd::<u8>(rbx + 0x08) != 0 {
                let cur_t = rd::<f32>(rbx + 0x0c);
                if cur_t >= 0.0 {
                    let next_t = cur_t - dt;
                    wr::<f32>(rbx + 0x0c, next_t);
                    if 0.0 >= next_t && rd::<i32>(rcx + 0x18) != -1 {
                        exe::call_va!(0x1_4171_9630, (rcx));
                    }
                }
            }
            let rax = rd::<usize>(rbx);
            if rax != 0 {
                wr::<u8>(rax + 0x80, 1);
            }
        }
        rbx = rbx.wrapping_add(0x30);
    }
}

/// `content_141c69d30`
/// Original: 0x141c69d30 (384 bytes, Throws::No)
pub unsafe extern "win64" fn content_141c69d30(param_1: usize) {
    let len0 = rd::<u32>(param_1 + 0x08) as usize;
    let base0 = rd::<usize>(param_1);
    if base0 != base0.wrapping_add(len0.wrapping_mul(0x54)) {
        exe::call_va!(0x1_401d_e830, (param_1));
    }

    let cnt_28 = rd::<u32>(param_1 + 0x28) as u64;
    let rbp = if cnt_28 == 0 { 0 } else { cnt_28 - 1 };
    if rbp > rd::<u32>(param_1 + 0x0c) as u64 {
        exe::call_va!(0x1_4083_a860, (param_1, rbp));
    }

    if rbp != 0 {
        let mut esi: i32 = 0;
        loop {
            let p20 = rd::<usize>(param_1 + 0x20);
            let rdi = p20.wrapping_add(((esi as isize) << 5) as usize);
            let r14d = esi.wrapping_add(1);
            let r15 = (r14d as isize) as u64;
            let rsi_ptr = p20.wrapping_add(((r14d as isize) << 5) as usize);
            let edx = rd::<u32>(param_1 + 0x08) as u64;
            let eax = rd::<u32>(param_1 + 0x0c) as u64;
            if edx.wrapping_mul(0x54) == eax.wrapping_mul(0x54) {
                exe::call_va!(0x1_4083_a860, (param_1, edx.wrapping_add(1)));
            }
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let dst = rd::<usize>(param_1).wrapping_add(cur_len.wrapping_mul(0x54));
            exe::call_va!(0x1_41c6_99a0, (dst, rdi, rsi_ptr));
            wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));
            esi = r14d;
            if r15 >= rbp {
                break;
            }
        }
    }

    wr::<u32>(param_1 + 0x10, 0x7f7f_ffff);
    wr::<u32>(param_1 + 0x14, 0x7f7f_ffff);
    wr::<u32>(param_1 + 0x18, 0xff7f_ffff);
    wr::<u32>(param_1 + 0x1c, 0xff7f_ffff);

    let mut rax = rd::<usize>(param_1);
    let rcx_end = rax.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0x54));
    if rax != rcx_end {
        let mut xmm4 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fd0) as usize);
        let mut xmm1 = rd::<f32>(param_1 + 0x14);
        let mut xmm2 = rd::<f32>(exe::ptr::<f32>(0x1_4342_7174) as usize);
        let mut xmm3 = rd::<f32>(param_1 + 0x1c);
        while rax != rcx_end {
            let f0 = rd::<f32>(rax + 0x04);
            if xmm4 > f0 {
                xmm4 = f0;
                wr::<f32>(param_1 + 0x10, f0);
            }
            let f1 = rd::<f32>(rax + 0x08);
            if xmm1 > f1 {
                xmm1 = f1;
                wr::<f32>(param_1 + 0x14, f1);
            }
            let f2 = rd::<f32>(rax + 0x0c);
            if f2 > xmm2 {
                xmm2 = f2;
                wr::<f32>(param_1 + 0x18, f2);
            }
            let f3 = rd::<f32>(rax + 0x10);
            if f3 > xmm3 {
                xmm3 = f3;
                wr::<f32>(param_1 + 0x1c, f3);
            }
            rax = rax.wrapping_add(0x54);
        }
    }
}

/// `unknown_141df5490`
/// Original: 0x141df5490 (222 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141df5490(param_1: usize, param_2: usize) {
    wr::<u8>(param_1 + 0x10, 1);
    let vtbl2 = rd::<usize>(param_2);
    let r14_fn = rd::<u64>(vtbl2 + 0x240);
    let rdi = exe::call_va!(rd::<u64>(vtbl2 + 0x218), (param_2) -> usize);
    let r9_val: u64 = if rdi != 0 {
        let ebx = exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x78), (rdi) -> u32);
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x70), (rdi, 2u32));
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x28), (rdi));
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x28), (rdi));
        exe::call_va!(0x1_41df_4980, (rdi, param_1));
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x48), (rdi, param_1));
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x48), (rdi, param_1));
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x70), (rdi, ebx));
        exe::call_va!(rd::<u64>(rd::<usize>(rdi) + 0x40), (rdi) -> u64)
    } else {
        0
    };
    let str_user_settings = exe::ptr::<u8>(0x1_4330_f1a0) as usize;
    let fn_48350 = exe::ptr::<u8>(0x1_40f4_8350) as usize;
    exe::call_va!(
        r14_fn,
        (param_2, str_user_settings, param_1, r9_val, fn_48350)
    );
}

/// `unknown_141df7140`
/// Original: 0x141df7140 (230 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141df7140(param_1: usize, param_2: usize) {
    let vtbl2 = rd::<usize>(param_2);
    let rbp_fn = rd::<u64>(vtbl2 + 0x240);
    let rbx = exe::call_va!(rd::<u64>(vtbl2 + 0x218), (param_2) -> usize);
    let r9_val: u64 = if rbx != 0 {
        let r14d = exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x78), (rbx) -> u32);
        exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x70), (rbx, 2u32));
        exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x28), (rbx));
        if param_1 != 0 {
            exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x28), (rbx));
            exe::call_va!(0x1_41df_6fd0, (rbx, param_1));
            exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x48), (rbx, param_1));
        } else {
            exe::call_va!(0x1_41df_6fd0, (rbx, 0usize));
        }
        exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x48), (rbx, param_1));
        exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x70), (rbx, r14d));
        exe::call_va!(rd::<u64>(rd::<usize>(rbx) + 0x40), (rbx) -> u64)
    } else {
        0
    };
    let str_subtitles = exe::ptr::<u8>(0x1_4330_f3a0) as usize;
    let fn_4b760 = exe::ptr::<u8>(0x1_40f4_b760) as usize;
    exe::call_va!(
        rbp_fn,
        (param_2, str_subtitles, param_1, r9_val, fn_4b760)
    );
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141aca620(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODE_MEM: [u8; 0x100] = [0; 0x100];
    static mut TR0_MEM: [u8; 0x80] = [0; 0x80];
    static mut TR1_MEM: [u8; 0x80] = [0; 0x80];
    static mut LOG_TAGS: u32 = 0;
    static mut LOG_VEC: [u64; 4] = [0; 4];
    static mut LOG_CTX: [u64; 5] = [0; 5];

    unsafe extern "win64" fn mock_cb20(_p5: usize, view: usize) -> usize {
        let len = rd::<u64>(view + 0x08);
        LOG_TAGS = LOG_TAGS.wrapping_add(1 + len as u32);
        0x1000
    }
    unsafe extern "win64" fn mock_8bc0(_n1: usize, view: usize) -> usize {
        let len = rd::<u64>(view + 0x08);
        LOG_TAGS = LOG_TAGS.wrapping_add(10 + len as u32);
        core::ptr::addr_of_mut!(NODE_MEM) as usize
    }
    unsafe extern "win64" fn mock_29e0(out_ptr: usize, handle: u64, p2_copy: usize) {
        LOG_TAGS = LOG_TAGS
            .wrapping_add(100)
            .wrapping_add((handle as u32) ^ (rd::<u32>(p2_copy + 0x10)));
        wr::<usize>(out_ptr, core::ptr::addr_of_mut!(TR0_MEM) as usize);
        wr::<usize>(out_ptr + 0x08, core::ptr::addr_of_mut!(TR1_MEM) as usize);
        wr::<u64>(out_ptr + 0x10, 0);
        wr::<u64>(out_ptr + 0x18, 1);
        wr::<u64>(out_ptr + 0x20, 1);
    }
    unsafe extern "win64" fn mock_8220(ctx: usize, _p5: usize, pair: usize, p6: usize) {
        LOG_TAGS = LOG_TAGS.wrapping_add(1000 + p6 as u32);
        LOG_CTX = rd::<[u64; 5]>(ctx);
        LOG_VEC = rd::<[u64; 4]>(pair);
    }
    unsafe extern "win64" fn mock_34c0(ctx: usize, p3_copy: usize, pair: usize) {
        LOG_TAGS = LOG_TAGS
            .wrapping_add(10000)
            .wrapping_add(rd::<u32>(p3_copy));
        LOG_CTX = rd::<[u64; 5]>(ctx);
        LOG_VEC = rd::<[u64; 4]>(pair);
    }
    unsafe extern "win64" fn mock_2a40(ctx: usize, p4_copy: usize, pair: usize) {
        LOG_TAGS = LOG_TAGS
            .wrapping_add(100000)
            .wrapping_add(rd::<u32>(p4_copy));
        LOG_CTX = rd::<[u64; 5]>(ctx);
        LOG_VEC = rd::<[u64; 4]>(pair);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41ad_cb20, mock_cb20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41ac_8bc0, mock_8bc0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4059_29e0, mock_29e0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41ad_8220, mock_8220 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41e2_34c0, mock_34c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e2_2a40, mock_2a40 as *const ())?;

    static mut ENT_ARR: [u8; 0x630 * 2] = [0; 0x630 * 2];
    static mut S90_MEM: [u8; 0x160] = [0; 0x160];
    static mut S98_MEM: [u8; 0xa0] = [0; 0xa0];

    let run = |f: F, mode: u8| -> (u32, [u64; 5], [u64; 4], [u32; 4], u8) {
        LOG_TAGS = 0;
        LOG_VEC = [0; 4];
        LOG_CTX = [0; 5];
        NODE_MEM = [0; 0x100];
        TR0_MEM = [0; 0x80];
        TR1_MEM = [0; 0x80];
        ENT_ARR = [0; 0x630 * 2];
        S90_MEM = [0; 0x160];
        S98_MEM = [0; 0xa0];

        let np = core::ptr::addr_of_mut!(NODE_MEM) as usize;
        if mode == 0 {
            wr::<u64>(np + 0xc8, u64::MAX);
        } else {
            wr::<u64>(np + 0xc8, 0x0002_0000_0001);
        }

        let tr0p = core::ptr::addr_of_mut!(TR0_MEM) as usize;
        wr::<[f32; 4]>(tr0p + 0x20, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(tr0p + 0x30, [1.5, 2.5, 3.5, 0.0]);

        let tr1p = core::ptr::addr_of_mut!(TR1_MEM) as usize;
        wr::<f32>(tr1p + 0x1c, 10.0);
        wr::<f32>(tr1p + 0x20, 20.0);

        let s90p = core::ptr::addr_of_mut!(S90_MEM) as usize;
        wr::<f32>(s90p + 0x150, 1.0);
        wr::<f32>(s90p + 0x154, 2.0);

        let s98p = core::ptr::addr_of_mut!(S98_MEM) as usize;
        wr::<u8>(s98p + 0x98, 1);

        let ent_base = core::ptr::addr_of_mut!(ENT_ARR) as usize;
        let ent = ent_base + 0x630;
        wr::<u32>(ent + 0x48, 0x0400_0004);
        wr::<u32>(ent + 0x4c, 3);
        wr::<usize>(ent + 0x90, s90p);
        wr::<usize>(ent + 0x98, s98p);
        wr::<u8>(ent + 0xa1, 1);
        wr::<f32>(ent + 0x150, 0.25);
        wr::<f32>(ent + 0x154, 0.5);
        wr::<u32>(ent + 0x318, 0x0400_0004);
        wr::<u32>(ent + 0x31c, 4);
        wr::<u8>(ent + 0x5f2, if mode == 2 { 1 } else { 0 });
        wr::<u64>(ent + 0x600, 0x1122_3344_5566_7788);
        wr::<u64>(ent + 0x608, 0x99aa_bbcc_ddee_ff00);

        let p08_arr: [u32; 2] = [0, if mode == 3 { 0 } else { 1 }];
        let mut p18_arr = [0u32; 14];
        let mut p40_arr = [0u8; 4];

        let mut p1 = [0u64; 12];
        p1[0] = ent_base as u64;
        p1[1] = p08_arr.as_ptr() as usize as u64;
        p1[2] = 0xaaaa_1000;
        p1[3] = p18_arr.as_mut_ptr() as usize as u64;
        p1[4] = 0xbbbb_2000;
        p1[5] = 0xcccc_3000;
        p1[6] = 0xdddd_4000;
        p1[7] = 0xeeee_5000;
        p1[8] = p40_arr.as_mut_ptr() as usize as u64;
        p1[9] = 0xffff_6000;
        p1[10] = 0x7777_7000;
        p1[11] = 1;

        let p2_ents: [u64; 2] = [0, 0x0002_0000_0001];
        let mut p2_world = [0u8; 0x30];
        let p2wp = p2_world.as_mut_ptr() as usize;
        wr::<usize>(p2wp + 0x18, p2_ents.as_ptr() as usize);
        wr::<u32>(p2wp + 0x20, 2);
        let p2: [u64; 3] = [p2wp as u64, 0x22, 0x33];

        let p3: [u64; 3] = [101, 102, 103];
        let p4: [u64; 3] = [201, 202, 203];

        let mut p5 = [0u8; 0x440];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<f32>(p5p + 0x4c, 1.5);
        wr::<[f32; 4]>(p5p + 0x3c0, [0.0, 0.0, 0.0, 1.0]);
        wr::<[f32; 4]>(p5p + 0x3d0, [4.0, 5.0, 6.0, 0.0]);
        wr::<u8>(p5p + 0x424, 1);

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5p,
            0x55,
        );

        let box_out = [p18_arr[10], p18_arr[11], p18_arr[12], p18_arr[13]];
        (LOG_TAGS, LOG_CTX, LOG_VEC, box_out, p40_arr[1])
    };

    for mode in [0u8, 1, 2, 3] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141aca620 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_141acb3c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_IDS: [u16; 2] = [0, 1];
    static mut WORLD_MEM: [u8; 0x10100] = [0; 0x10100];
    static mut BLOCK_HANDLES: [u64; 6] = [0; 6];
    static mut COMP0_MEM: [u8; 0x630 * 2] = [0; 0x630 * 2];
    static mut COMP1_MEM: [u8; 0x1c * 2] = [0; 0x1c * 2];
    static mut GROW_CALLED: u64 = 0;

    unsafe extern "win64" fn mock_7c10(out_ptr: usize, world_ptr: usize, _sub: usize) {
        let arch_p = core::ptr::addr_of!(ARCH_IDS) as usize;
        wr::<usize>(out_ptr, world_ptr);
        wr::<usize>(out_ptr + 0x08, arch_p);
        wr::<usize>(out_ptr + 0x10, arch_p + 4);
    }
    unsafe extern "win64" fn mock_94d0(iter_ptr: usize) {
        let cur = rd::<usize>(iter_ptr + 0x10);
        let end = rd::<usize>(iter_ptr + 0x18);
        if cur != end {
            wr::<usize>(iter_ptr + 0x38, core::ptr::addr_of_mut!(COMP0_MEM) as usize);
            wr::<usize>(iter_ptr + 0x40, core::ptr::addr_of_mut!(COMP1_MEM) as usize);
        }
    }
    unsafe extern "win64" fn mock_6ff0(_a: usize, _b: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_1010(vec_ptr: usize, _pos_ptr: usize, val_ptr: usize) {
        GROW_CALLED = rd::<u64>(val_ptr);
        let len = rd::<u32>(vec_ptr + 0x08);
        wr::<u32>(vec_ptr + 0x08, len + 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4058_94d0, mock_94d0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4021_1010, mock_1010 as *const ())?;

    let run = |f: F, cap: u32| -> (u32, u64, u64) {
        GROW_CALLED = 0;
        WORLD_MEM = [0; 0x10100];
        COMP0_MEM = [0; 0x630 * 2];
        COMP1_MEM = [0; 0x1c * 2];
        BLOCK_HANDLES = [0, 0, 0x1000_0001, 0x2000_0002, 0, 0];

        let wp = (core::ptr::addr_of_mut!(WORLD_MEM) as usize) + 0x40;
        let bhp = core::ptr::addr_of_mut!(BLOCK_HANDLES) as usize;
        wr::<usize>(wp + 0x48, bhp);
        wr::<u32>(wp + 0x10048, 2);

        let mut s98 = [0u8; 0xa0];
        s98[0x98] = 1;
        let mut self_90 = [0u8; 0x750];
        let mut other_90 = [0u8; 0x750];
        wr::<i32>((self_90.as_mut_ptr() as usize) + 0x73c, 10);
        wr::<i32>((other_90.as_mut_ptr() as usize) + 0x73c, 5);

        let mut vec_buf: [u64; 4] = [0; 4];
        let c0p = core::ptr::addr_of_mut!(COMP0_MEM) as usize;
        wr::<usize>(c0p + 0x90, self_90.as_mut_ptr() as usize);
        wr::<usize>(c0p + 0x98, s98.as_mut_ptr() as usize);
        wr::<u8>(c0p + 0xa1, 1);
        wr::<u8>(c0p + 0xa7, 1);
        wr::<usize>(c0p + 0x610, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(c0p + 0x618, 99);
        wr::<u32>(c0p + 0x61c, cap);

        let c0_1 = c0p + 0x630;
        wr::<usize>(c0_1 + 0x90, other_90.as_mut_ptr() as usize);
        wr::<u8>(c0_1 + 0xa1, 1);
        wr::<u8>(c0_1 + 0xa7, 1);

        let c1p = core::ptr::addr_of_mut!(COMP1_MEM) as usize;
        for base in [c1p, c1p + 0x1c] {
            wr::<f32>(base + 0x0c, -5.0);
            wr::<f32>(base + 0x10, -5.0);
            wr::<f32>(base + 0x14, 5.0);
            wr::<f32>(base + 0x18, 5.0);
        }

        let p1: [u64; 4] = [c0p as u64, c1p as u64, bhp as u64, 0];
        let p2: [u64; 3] = [
            core::ptr::addr_of_mut!(WORLD_MEM) as usize as u64,
            0x11,
            0x22,
        ];
        let mut p5 = [0u8; 0x430];
        p5[0x424] = 1;

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            0,
            0,
            p5.as_ptr() as usize,
        );
        (rd::<u32>(c0p + 0x618), vec_buf[0], GROW_CALLED)
    };

    for cap in [0u32, 4] {
        let o = run(f_o, cap);
        let n = run(f_n, cap);
        crate::diff_eq!(o, n, "141acb3c0 cap={}", cap);
    }
    Ok(2)
}

pub unsafe fn diff_bonsai_141b923e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_D5C0: (u32, u32) = (0, 0);
    static mut LOG_6260: (usize, u32) = (0, 0);
    static mut LOG_9630: usize = 0;
    static mut NODE_A: [u8; 0x90] = [0; 0x90];
    static mut NODE_B: [u8; 0x90] = [0; 0x90];

    unsafe extern "win64" fn mock_d5c0(_rcx: usize, _rdx: usize) {
        let [xmm0, xmm1, _, _] = crate::simd_ops::take_xmm_args();
        LOG_D5C0 = (xmm0 as u32, xmm1 as u32);
    }
    unsafe extern "win64" fn mock_6260(rcx: usize) {
        let [_, xmm1, _, _] = crate::simd_ops::take_xmm_args();
        LOG_6260 = (rcx, xmm1 as u32);
    }
    unsafe extern "win64" fn mock_9630(rcx: usize) {
        LOG_9630 = rcx;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4211_d5c0, mock_d5c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4170_6260, mock_6260 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4171_9630, mock_9630 as *const ())?;

    let g_c0e = exe::ptr::<u8>(0x1_43a2_8c0e) as usize;
    let g_e14 = exe::ptr::<u8>(0x1_43c8_9e14) as usize;
    let saved_c0e = rd::<u8>(g_c0e);
    let saved_e14 = rd::<u8>(g_e14);

    let run = |f: F, c0e_val: u8, active: u8| -> ((u32, u32), (usize, u32), usize, u8, u32, u8) {
        LOG_D5C0 = (0, 0);
        LOG_6260 = (0, 0);
        LOG_9630 = 0;
        NODE_A = [0; 0x90];
        NODE_B = [0; 0x90];

        wr::<u8>(g_c0e, c0e_val);
        wr::<u8>(g_e14, c0e_val ^ 1);

        let nap = core::ptr::addr_of_mut!(NODE_A) as usize;
        let nbp = core::ptr::addr_of_mut!(NODE_B) as usize;
        wr::<i32>(nap + 0x18, 3);
        wr::<i32>(nbp + 0x18, -1);

        let mut p1 = [0u8; 0x1d0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, 0x7788);
        wr::<usize>(p1p + 0x40, nap);
        wr::<u8>(p1p + 0x48, 1);
        wr::<f32>(p1p + 0x4c, 0.25);
        wr::<usize>(p1p + 0x70, nbp);
        wr::<u8>(p1p + 0x78, 1);
        wr::<f32>(p1p + 0x7c, 0.25);
        wr::<u8>(p1p + 0x1c8, active);

        let dt: f32 = 0.5;
        f(p1p, (&dt as *const f32) as usize);

        let new_e14 = rd::<u8>(g_e14);
        wr::<u8>(g_c0e, saved_c0e);
        wr::<u8>(g_e14, saved_e14);

        (
            LOG_D5C0,
            LOG_6260,
            LOG_9630,
            new_e14,
            rd::<u32>(p1p + 0x4c),
            rd::<u8>(nap + 0x80),
        )
    };

    for (c0e_val, active) in [(0u8, 0u8), (0, 1), (1, 1)] {
        let o = run(f_o, c0e_val, active);
        let n = run(f_n, c0e_val, active);
        crate::diff_eq!(o, n, "141b923e0 c0e={} active={}", c0e_val, active);
    }
    Ok(3)
}

pub unsafe fn diff_content_141c69d30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ELEM_BUF: [u8; 0x54 * 4] = [0; 0x54 * 4];
    static mut SRC_BUF: [u8; 0x20 * 4] = [0; 0x20 * 4];
    static mut LOG_CALLS: Vec<(u32, u64)> = Vec::new();

    unsafe extern "win64" fn mock_e830(vec_ptr: usize) {
        LOG_CALLS.push((0xe830, rd::<u32>(vec_ptr + 0x08) as u64));
        wr::<u32>(vec_ptr + 0x08, 0);
    }
    unsafe extern "win64" fn mock_a860(vec_ptr: usize, new_cap: u64) {
        LOG_CALLS.push((0xa860, new_cap));
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_99a0(dst: usize, s0: usize, s1: usize) {
        let a = rd::<f32>(s0);
        let b = rd::<f32>(s1);
        wr::<f32>(dst + 0x04, a - 2.0);
        wr::<f32>(dst + 0x08, b - 3.0);
        wr::<f32>(dst + 0x0c, a + 4.0);
        wr::<f32>(dst + 0x10, b + 5.0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4083_a860, mock_a860 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41c6_99a0, mock_99a0 as *const ())?;

    let run = |f: F, init_len: u32, init_cap: u32, src_cnt: u32| -> (Vec<(u32, u64)>, u32, [u32; 4]) {
        LOG_CALLS.clear();
        ELEM_BUF = [0; 0x54 * 4];
        SRC_BUF = [0; 0x20 * 4];

        let sp = core::ptr::addr_of_mut!(SRC_BUF) as usize;
        wr::<f32>(sp, 10.0);
        wr::<f32>(sp + 0x20, -4.0);
        wr::<f32>(sp + 0x40, 25.0);

        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, core::ptr::addr_of_mut!(ELEM_BUF) as usize);
        wr::<u32>(p1p + 0x08, init_len);
        wr::<u32>(p1p + 0x0c, init_cap);
        wr::<usize>(p1p + 0x20, sp);
        wr::<u32>(p1p + 0x28, src_cnt);

        f(p1p);
        let bounds = [
            rd::<u32>(p1p + 0x10),
            rd::<u32>(p1p + 0x14),
            rd::<u32>(p1p + 0x18),
            rd::<u32>(p1p + 0x1c),
        ];
        (LOG_CALLS.clone(), rd::<u32>(p1p + 0x08), bounds)
    };

    for (il, ic, sc) in [(0u32, 0u32, 0u32), (1, 0, 3), (0, 4, 2)] {
        let o = run(f_o, il, ic, sc);
        let n = run(f_n, il, ic, sc);
        crate::diff_eq!(o, n, "141c69d30 il={} ic={} sc={}", il, ic, sc);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141df5490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SUB_OBJ: [usize; 2] = [0; 2];
    static mut SUB_VTBL: [usize; 16] = [0; 16];
    static mut P1_BUF: [u8; 0x20] = [0; 0x20];
    static mut HAS_SUB: bool = false;
    static mut LOG_SEQ: Vec<(u32, usize)> = Vec::new();

    unsafe extern "win64" fn v218(_p2: usize) -> usize {
        LOG_SEQ.push((0x218, 0));
        if HAS_SUB {
            core::ptr::addr_of_mut!(SUB_OBJ) as usize
        } else {
            0
        }
    }
    unsafe extern "win64" fn v240(
        _p2: usize,
        name_ptr: usize,
        p1: usize,
        r9_val: u64,
        cb_fn: usize,
    ) {
        LOG_SEQ.push((0x240, rd::<u8>(name_ptr) as usize));
        LOG_SEQ.push((0x241, p1));
        LOG_SEQ.push((0x242, r9_val as usize));
        LOG_SEQ.push((0x243, cb_fn));
    }
    unsafe extern "win64" fn s28(this: usize) {
        LOG_SEQ.push((0x28, this));
    }
    unsafe extern "win64" fn s40(this: usize) -> u64 {
        LOG_SEQ.push((0x40, this));
        0xabcd_1234
    }
    unsafe extern "win64" fn s48(this: usize, p1: usize) {
        LOG_SEQ.push((0x48, this ^ p1));
    }
    unsafe extern "win64" fn s70(this: usize, mode: u32) {
        LOG_SEQ.push((0x70, (this & 0xffff) ^ (mode as usize)));
    }
    unsafe extern "win64" fn s78(_this: usize) -> u32 {
        LOG_SEQ.push((0x78, 0));
        7
    }
    unsafe extern "win64" fn mock_4980(sub: usize, p1: usize) {
        LOG_SEQ.push((0x4980, sub ^ p1));
    }

    let _m1 = crate::difftest::Mock::new(0x1_41df_4980, mock_4980 as *const ())?;

    let run = |f: F, has_sub: bool| -> (u8, Vec<(u32, usize)>) {
        HAS_SUB = has_sub;
        LOG_SEQ.clear();
        P1_BUF = [0; 0x20];

        let svp = core::ptr::addr_of_mut!(SUB_VTBL) as usize;
        wr::<usize>(svp + 0x28, s28 as *const () as usize);
        wr::<usize>(svp + 0x40, s40 as *const () as usize);
        wr::<usize>(svp + 0x48, s48 as *const () as usize);
        wr::<usize>(svp + 0x70, s70 as *const () as usize);
        wr::<usize>(svp + 0x78, s78 as *const () as usize);
        SUB_OBJ[0] = svp;

        let mut p2_vtbl = [0usize; 80];
        let p2vp = p2_vtbl.as_mut_ptr() as usize;
        wr::<usize>(p2vp + 0x218, v218 as *const () as usize);
        wr::<usize>(p2vp + 0x240, v240 as *const () as usize);
        let p2_obj: [usize; 1] = [p2vp];

        let p1p = core::ptr::addr_of_mut!(P1_BUF) as usize;
        f(p1p, p2_obj.as_ptr() as usize);
        (rd::<u8>(p1p + 0x10), LOG_SEQ.clone())
    };

    for has_sub in [false, true] {
        let o = run(f_o, has_sub);
        let n = run(f_n, has_sub);
        crate::diff_eq!(o, n, "141df5490 has_sub={}", has_sub);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141df7140(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SUB_OBJ: [usize; 2] = [0; 2];
    static mut SUB_VTBL: [usize; 16] = [0; 16];
    static mut HAS_SUB: bool = false;
    static mut LOG_SEQ: Vec<(u32, usize)> = Vec::new();

    unsafe extern "win64" fn v218(_p2: usize) -> usize {
        LOG_SEQ.push((0x218, 0));
        if HAS_SUB {
            core::ptr::addr_of_mut!(SUB_OBJ) as usize
        } else {
            0
        }
    }
    unsafe extern "win64" fn v240(
        _p2: usize,
        name_ptr: usize,
        p1: usize,
        r9_val: u64,
        cb_fn: usize,
    ) {
        LOG_SEQ.push((0x240, rd::<u8>(name_ptr) as usize));
        LOG_SEQ.push((0x241, p1));
        LOG_SEQ.push((0x242, r9_val as usize));
        LOG_SEQ.push((0x243, cb_fn));
    }
    unsafe extern "win64" fn s28(this: usize) {
        LOG_SEQ.push((0x28, this));
    }
    unsafe extern "win64" fn s40(this: usize) -> u64 {
        LOG_SEQ.push((0x40, this));
        0x5566_7788
    }
    unsafe extern "win64" fn s48(this: usize, p1: usize) {
        LOG_SEQ.push((0x48, this ^ p1));
    }
    unsafe extern "win64" fn s70(this: usize, mode: u32) {
        LOG_SEQ.push((0x70, (this & 0xffff) ^ (mode as usize)));
    }
    unsafe extern "win64" fn s78(_this: usize) -> u32 {
        LOG_SEQ.push((0x78, 0));
        9
    }
    unsafe extern "win64" fn mock_6fd0(sub: usize, p1: usize) {
        LOG_SEQ.push((0x6fd0, sub ^ p1));
    }

    let _m1 = crate::difftest::Mock::new(0x1_41df_6fd0, mock_6fd0 as *const ())?;

    let run = |f: F, has_sub: bool, p1_arg: usize| -> Vec<(u32, usize)> {
        HAS_SUB = has_sub;
        LOG_SEQ.clear();

        let svp = core::ptr::addr_of_mut!(SUB_VTBL) as usize;
        wr::<usize>(svp + 0x28, s28 as *const () as usize);
        wr::<usize>(svp + 0x40, s40 as *const () as usize);
        wr::<usize>(svp + 0x48, s48 as *const () as usize);
        wr::<usize>(svp + 0x70, s70 as *const () as usize);
        wr::<usize>(svp + 0x78, s78 as *const () as usize);
        SUB_OBJ[0] = svp;

        let mut p2_vtbl = [0usize; 80];
        let p2vp = p2_vtbl.as_mut_ptr() as usize;
        wr::<usize>(p2vp + 0x218, v218 as *const () as usize);
        wr::<usize>(p2vp + 0x240, v240 as *const () as usize);
        let p2_obj: [usize; 1] = [p2vp];

        f(p1_arg, p2_obj.as_ptr() as usize);
        LOG_SEQ.clone()
    };

    for (has_sub, p1_arg) in [(false, 0usize), (true, 0), (true, 0x4444)] {
        let o = run(f_o, has_sub, p1_arg);
        let n = run(f_n, has_sub, p1_arg);
        crate::diff_eq!(o, n, "141df7140 has_sub={} p1={}", has_sub, p1_arg);
    }
    Ok(3)
}
