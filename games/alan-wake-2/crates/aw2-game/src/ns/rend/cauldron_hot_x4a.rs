//! Decompiled hot functions shard X4A (`0x141f7a230..0x1420b1ae0`).

use crate::exe::{self, rd, wr};

/// `unknown_141f7a230`
/// Original: 0x141f7a230 (372 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f7a230(
    param_1: u64,
    param_2: usize,
    param_3: u64,
    param_4: usize,
    param_5: usize,
) {
    let mut rsi = rd::<usize>(param_4 + 0x58);
    let r14 = rsi.wrapping_add((rd::<usize>(param_4 + 0x40)).wrapping_shl(4));
    if rsi != r14 {
        let empty_key = rd::<u64>(param_4 + 0x48);
        loop {
            let k = rd::<u64>(rsi);
            if k != empty_key
                && (rd::<u64>(param_4 + 0x30) == 0 || rd::<u64>(param_4 + 0x28) != k)
            {
                break;
            }
            rsi = rsi.wrapping_add(0x10);
            if rsi == r14 {
                break;
            }
        }
    }

    while rsi != r14 {
        let entry = rd::<u128>(rsi);
        let r15_key = entry as u64;
        let val = (entry >> 64) as u64;
        let ebx = rd::<u32>(param_2 + 0x68);
        wr::<u32>(param_2 + 0x68, ebx.wrapping_add(1));

        let mut arg_r8: [u64; 2] = [ebx as u64, 1];
        exe::call_va!(
            0x1_41f7_9b60,
            (
                param_1,
                param_2,
                arg_r8.as_mut_ptr() as usize,
                r15_key,
                param_3,
                val
            )
        );

        let callable = rd::<usize>(param_5 + 0x38);
        if callable != 0 {
            let mut a_rdx: u64 = param_2 as u64;
            let mut a_r8: u32 = ebx;
            let mut a_r9: u64 = r15_key;
            let vtbl = rd::<usize>(callable);
            exe::call_va!(
                rd::<usize>(vtbl + 0x10),
                (
                    callable,
                    (&mut a_rdx as *mut u64) as usize,
                    (&mut a_r8 as *mut u32) as usize,
                    (&mut a_r9 as *mut u64) as usize
                )
            );
        }

        rsi = rsi.wrapping_add(0x10);
        if rsi != r14 {
            let empty_key = rd::<u64>(param_4 + 0x48);
            loop {
                let k = rd::<u64>(rsi);
                if k != empty_key
                    && (rd::<u64>(param_4 + 0x30) == 0 || rd::<u64>(param_4 + 0x28) != k)
                {
                    break;
                }
                rsi = rsi.wrapping_add(0x10);
                if rsi == r14 {
                    break;
                }
            }
        }
    }

    let callable = rd::<usize>(param_5 + 0x38);
    if callable != 0 {
        let vtbl = rd::<usize>(callable);
        let not_self = (callable != param_5) as u8;
        exe::call_va!(rd::<usize>(vtbl + 0x20), (callable, not_self));
        wr::<usize>(param_5 + 0x38, 0);
    }
}

/// `unknown_141fcb9a0`
/// Original: 0x141fcb9a0 (1367 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141fcb9a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
) {
    let p1 = rd::<[u64; 15]>(param_1);
    let idx = p1[14] as usize;
    let rdi = (p1[0] as usize).wrapping_add(idx.wrapping_shl(5));
    let rbx = (p1[1] as usize).wrapping_add(idx.wrapping_mul(0x208));
    let r9_off = idx.wrapping_mul(0x10);
    let p_b8 = p1[2] as usize;
    let r15_base = p1[3] as usize;
    let p_a8 = p1[4] as usize;
    let r14 = (p1[5] as usize).wrapping_add(idx.wrapping_shl(6));
    let p_98 = p1[6] as usize;
    let val_b0 = rd::<u64>(param_3);

    let mut flag_10: u8 = 0;
    let mut flag_11: u8 = 0;
    let mut r13: usize = 0;
    let p48 = rd::<usize>(param_1 + 0x48);
    if p48 != 0 {
        let i70 = rd::<usize>(param_1 + 0x70);
        r13 = p48.wrapping_add(i70.wrapping_mul(0x18));
        flag_10 = rd::<u8>(r13 + 0x10);
        flag_11 = rd::<u8>(r13 + 0x11);
    }

    let count_ptr = rd::<usize>(p_b8.wrapping_add(r9_off));
    let count_val = rd::<u32>(count_ptr);
    if count_val == 0 && flag_10 == 0 {
        let i70 = rd::<usize>(param_1 + 0x70);
        let p68 = rd::<usize>(param_1 + 0x68);
        let rdx_val = rd::<u64>(p68.wrapping_add(i70 * 8).wrapping_add(0x10));
        let mut p5_slot: u64 = param_5;
        exe::call_va!(0x1_4115_dfe0, ((&mut p5_slot as *mut u64) as usize, rdx_val));
        return;
    }

    let mut v_140 = rd::<u128>(r14 + 0x10);
    let mut v_150 = rd::<u128>(r15_base.wrapping_add(r9_off));
    let mut v_110 = crate::simd_ops::vsubps(v_140, rd::<u128>(r14 + 0x30));
    let dt = rd::<f32>(param_2);
    if dt > 0.0 {
        v_110 = crate::simd_ops::vdivps(v_110, crate::simd_ops::vbroadcastss(dt.to_bits()));
    }
    wr::<u128>(r15_base.wrapping_add(r9_off), v_110);

    let mut opt_78: u128 = 0;
    let mut opt_88: u32 = 0;
    let p58 = rd::<usize>(param_1 + 0x58);
    if p58 != 0 {
        let i70 = rd::<usize>(param_1 + 0x70);
        opt_78 = rd::<u128>(p58.wrapping_add(i70.wrapping_mul(0x10)));
        opt_88 = 1;
    }

    let mut r15b: u8 = 0;
    let g_608 = exe::ptr::<u64>(0x1_43a3_4608) as usize;
    if rd::<u32>(rbx) < 10 && rd::<u64>(g_608) < 10 {
        let mut xmm2_bits = rd::<u32>(exe::ptr::<u32>(0x1_4342_668c) as usize);
        let mut r8b: u8 = 0;
        let p40 = rd::<usize>(param_1 + 0x40);
        if p40 != 0 {
            let i70 = rd::<usize>(param_1 + 0x70);
            let entry40 = p40.wrapping_add(i70.wrapping_shl(4));
            if entry40 != 0 {
                xmm2_bits = rd::<u32>(entry40 + 0x0c);
                r8b = rd::<u8>(entry40 + 0x08);
                r15b = rd::<u8>(entry40 + 0x09);
                let p38 = rd::<usize>(param_1 + 0x38);
                if p38 != 0 {
                    let entry38 = p38.wrapping_add(i70.wrapping_mul(0x148));
                    if entry38 != 0 {
                        let xmm1_f = rd::<f32>(entry38 + 0x18);
                        if 0.0f32 < xmm1_f {
                            xmm2_bits = xmm1_f.to_bits();
                        }
                    }
                }
            }
        }

        let mut mat_160: [u128; 2] = [rd::<u128>(r14), rd::<u128>(r14 + 0x10)];
        if flag_11 != 0 && rd::<u64>(param_1 + 0x60) != 0 {
            let xmm1 = rd::<u128>(r14 + 0x10);
            let c_3a8 = rd::<u32>(exe::ptr::<u32>(0x1_4385_63a8) as usize);
            let xmm0 = crate::simd_ops::vinsertps(0, c_3a8 as u128, 0x10);
            mat_160[1] = crate::simd_ops::vaddps(xmm0, xmm1);
        }

        let mut opt_f0: [u64; 3] = [opt_78 as u64, (opt_78 >> 64) as u64, opt_88 as u64];
        let r8_ent = rd::<u64>(p_a8.wrapping_add(idx * 8));
        exe::call_va!(
            0x1_41fc_fce0,
            (
                param_4.wrapping_add(0x90),
                val_b0,
                r8_ent,
                mat_160.as_mut_ptr() as usize,
                rbx.wrapping_add(0x08),
                rbx.wrapping_add(0x1e0),
                xmm2_bits,
                r8b,
                r15b,
                opt_f0.as_mut_ptr() as usize
            )
        );

        wr::<u32>(rbx + 0x04, 0);
        wr::<u32>(rbx, 0xffff_ffff);
        let _ = crate::simd_ops::atomic_xadd_u64(g_608 as u64, 1);

        if flag_11 != 0 && rd::<u64>(param_1 + 0x60) != 0 && rd::<u8>(rbx + 0x18d) == 0 {
            let xmm1 = rd::<u128>(r14 + 0x10);
            wr::<u64>(rbx + 0x24, xmm1 as u64);
            wr::<u32>(rbx + 0x2c, (xmm1 >> 64) as u32);
        }

        exe::call_va!(
            0x1_41fc_e480,
            (
                rdi,
                r13,
                (&mut v_110 as *mut u128) as usize,
                (&mut v_150 as *mut u128) as usize,
                (&mut v_140 as *mut u128) as usize,
                dt.to_bits()
            )
        );
    } else {
        wr::<u32>(rbx + 0x04, rd::<u32>(rbx + 0x04).wrapping_add(1));
    }

    let mut opt_58 = [0u8; 0x20];
    let p58_base = opt_58.as_mut_ptr() as usize;
    let p50 = rd::<usize>(param_1 + 0x50);
    if p50 != 0 {
        let i70 = rd::<usize>(param_1 + 0x70);
        let src = p50.wrapping_add(i70.wrapping_mul(0x18));
        wr::<u8>(p58_base, rd::<u8>(src));
        wr::<u8>(p58_base + 0x01, rd::<u8>(src + 0x01));
        wr::<u8>(p58_base + 0x02, rd::<u8>(src + 0x02));
        exe::call_va!(0x1_4116_30f0, (p58_base + 0x08, src + 0x08));
        wr::<u8>(p58_base + 0x18, 1);
    }

    let mut opt_c8 = [0u8; 0x20];
    let pc8_base = opt_c8.as_mut_ptr() as usize;
    let has_58 = rd::<u8>(p58_base + 0x18);
    if has_58 != 0 {
        wr::<u8>(pc8_base, rd::<u8>(p58_base));
        wr::<u8>(pc8_base + 0x01, rd::<u8>(p58_base + 0x01));
        wr::<u8>(pc8_base + 0x02, rd::<u8>(p58_base + 0x02));
        exe::call_va!(0x1_4116_30f0, (pc8_base + 0x08, p58_base + 0x08));
        wr::<u8>(pc8_base + 0x18, 1);
    }

    let mut opt_120: [u64; 3] = [opt_78 as u64, (opt_78 >> 64) as u64, opt_88 as u64];
    let rcx_ent = rd::<u64>(p_a8.wrapping_add(idx * 8));
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_41fc_f6c0,
        rcx_ent,
        r14 as u64,
        rbx as u64,
        dt.to_bits() as u64,
        pc8_base as u64,
        opt_120.as_mut_ptr() as usize as u64,
        r15b as u64,
        0,
        [0, 0, 0, dt.to_bits() as u128],
    );

    if rd::<u32>(p_98.wrapping_add(idx * 4)) != 0 && count_val != 0 {
        let mut item = [0u8; 0x20];
        let ip = item.as_mut_ptr() as usize;
        wr::<u16>(ip, 3);
        wr::<u8>(ip + 0x02, 0);
        wr::<u64>(ip + 0x04, 0);
        wr::<u32>(
            ip + 0x0c,
            rd::<u32>(exe::ptr::<u32>(0x1_4342_6780) as usize),
        );
        wr::<u64>(ip + 0x10, 0);
        wr::<u64>(ip + 0x18, 0);

        let cur_len = rd::<u32>(rdi + 0x08) as u64;
        let cur_cap = rd::<u32>(rdi + 0x0c) as u64;
        if (cur_len << 5) == (cur_cap << 5) {
            exe::call_va!(0x1_405e_9720, (rdi, cur_len.wrapping_add(1)));
        }
        let slot_idx = rd::<u32>(rdi + 0x08) as usize;
        let dst = rd::<usize>(rdi).wrapping_add(slot_idx.wrapping_shl(5));
        wr::<[u8; 0x20]>(dst, rd::<[u8; 0x20]>(ip));
        wr::<u32>(rdi + 0x08, (slot_idx as u32).wrapping_add(1));
    }

    wr::<u8>(rbx + 0x1f8, rd::<u8>(rbx + 0x1f8).wrapping_add(1));
    if has_58 != 0 {
        exe::call_va!(0x1_401f_2bc0, (p58_base + 0x08));
    }
}

/// `unknown_141fdd230`
/// Original: 0x141fdd230 (1996 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141fdd230(param_1: usize, param_2: usize, param_3: u8) {
    #[repr(C, align(32))]
    struct Frame([u8; 0x10a0]);

    let _ = param_3;
    let mut len: u64 = 0;
    while rd::<u8>(param_2.wrapping_add(len as usize)) != 0 {
        len = len.wrapping_add(1);
    }

    let mut hash: u64 = 0;
    if len != 0 {
        hash = 0xcbf2_9ce4_8422_2325;
        let end = param_2.wrapping_add(len as usize);
        let mut cur = param_2;
        while cur != end {
            let b = (rd::<u8>(cur) as i8) as i64 as u64;
            hash = (hash ^ (b | 0x20)).wrapping_mul(0x0100_0000_01b3);
            cur = cur.wrapping_add(1);
        }
    }

    let cur_cnt = rd::<u32>(param_1 + 0x18);
    if cur_cnt != 0 {
        let arr = rd::<usize>(param_1 + 0x10);
        for idx in 0..(cur_cnt as usize) {
            let elem = rd::<usize>(arr.wrapping_add(idx * 8));
            if rd::<u64>(elem + 0x08) == hash {
                let elem2 = rd::<usize>(arr.wrapping_add(idx * 8));
                wr::<u32>(elem2 + 0x04, rd::<u32>(elem2 + 0x04).wrapping_add(1));
                return;
            }
        }
    }

    let mut frame = Frame([0u8; 0x10a0]);
    let rsp = frame.0.as_mut_ptr() as usize;
    let rbp = rsp + 0x100;

    wr::<usize>(rsp + 0x20, param_2);
    wr::<u64>(rsp + 0x28, len);
    wr::<usize>(rbp + 0x1b0, exe::ptr::<u8>(0x1_4385_66b0) as usize);
    wr::<usize>(rbp + 0xf70, rbp + 0x1b8);
    wr::<u32>(rbp + 0x1b8, 0x4000_0040);
    wr::<u32>(rbp + 0x1bc, 0);
    wr::<u8>(rbp + 0x1c0, 0);
    wr::<u128>(rsp + 0x30, rd::<u128>(rsp + 0x20));
    exe::call_va!(0x1_4240_9b50, (rbp + 0x1b8, rsp + 0x30, 0u8, len));

    wr::<u32>(rbp + 0x3c0, 1);
    wr::<u64>(rbp + 0x3c8, 0);

    let r14 = rd::<usize>(exe::tls_pointer());
    wr::<usize>(rsp + 0x40, r14);
    let tls_depth = rd::<i32>(r14 + 0x6320) as isize;
    wr::<usize>(
        (r14 as isize + 0x6380 + tls_depth * 8) as usize,
        rbp + 0x1b0,
    );
    wr::<i32>(r14 + 0x6320, (tls_depth as i32).wrapping_add(1));

    let fmt_str = exe::ptr::<u8>(0x1_433c_4a28) as usize;
    let flag_299 = rd::<u8>(exe::ptr::<u8>(0x1_426e_6299) as usize);
    exe::call_va!(0x1_405f_c3c0, (rbp + 0xa0, flag_299, fmt_str, param_2));

    let s_hdr = rd::<u32>(rbp + 0xa0);
    let b3 = s_hdr >> 24;
    let cap_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8) - 0x7c00
    };
    let len_bits = (s_hdr & 0x00ff_ffff) << 3;
    let s_ptr = if len_bits > cap_bits {
        rd::<usize>(rbp + 0xa8)
    } else {
        rbp + 0xa8
    };
    let s_len = rd::<u32>(rbp + 0xa4) as u64;
    wr::<usize>(rsp + 0x20, s_ptr);
    wr::<u64>(rsp + 0x28, s_len);
    wr::<u128>(rsp + 0x30, rd::<u128>(rsp + 0x20));
    exe::call_va!(0x1_401d_f850, (rsp + 0x20, rsp + 0x30));
    wr::<u128>(rsp + 0x30, rd::<u128>(rsp + 0x20));
    let exists = exe::call_va!(0x1_4249_1120, (rsp + 0x30) -> u8);

    let mut r15_obj: usize = 0;
    if exists != 0 {
        let s_hdr2 = rd::<u32>(rbp + 0xa0);
        let b3_2 = s_hdr2 >> 24;
        let cap_bits2 = if b3_2 < 0x80 {
            b3_2 * 8
        } else {
            (b3_2 << 8) - 0x7c00
        };
        let len_bits2 = (s_hdr2 & 0x00ff_ffff) << 3;
        let s_ptr2 = if len_bits2 > cap_bits2 {
            rd::<usize>(rbp + 0xa8)
        } else {
            rbp + 0xa8
        };
        let mut s_len2: u64 = 0;
        while rd::<u8>(s_ptr2.wrapping_add(s_len2 as usize)) != 0 {
            s_len2 = s_len2.wrapping_add(1);
        }
        wr::<usize>(rsp + 0x20, s_ptr2);
        wr::<u64>(rsp + 0x28, s_len2);
        wr::<u32>(rbp + 0xf80, 0);
        wr::<usize>(rsp + 0x60, exe::ptr::<u8>(0x1_4340_eb60) as usize);
        wr::<usize>(rbp + 0xf70, rsp + 0x68);
        wr::<u32>(rsp + 0x68, 0x2000_0020);
        wr::<u32>(rsp + 0x6c, 0);
        wr::<u8>(rsp + 0x70, 0);
        wr::<u128>(rsp + 0x30, rd::<u128>(rsp + 0x20));
        exe::call_va!(0x1_4240_9b50, (rsp + 0x68, rsp + 0x30, 0u8, s_len2));

        wr::<u64>(rbp + 0x70, 0);
        wr::<u64>(rbp + 0x78, 0);
        wr::<u64>(rbp + 0x80, 0);
        wr::<u8>(rbp + 0x88, 0);
        let ed8 = exe::call_va!(
            0x1_413d_1d80,
            (0usize, 0usize, rsp + 0x20, rbp + 0xf80) -> usize
        );
        wr::<usize>(rbp + 0x90, ed8);
        exe::call_va!(0x1_4249_70f0, (rsp + 0x60));

        let cur_depth = rd::<i32>(r14 + 0x6320) as isize;
        let top_ctx = rd::<usize>((r14 as isize + 0x6380 + cur_depth * 8 - 8) as usize);
        wr::<usize>(top_ctx + 0x218, rsp + 0x60);

        r15_obj = exe::call_va!(0x1_4117_5310, () -> usize);
        wr::<usize>(rsp + 0x48, r15_obj);
        wr::<u32>(r15_obj + 0x04, 1);
        wr::<u64>(r15_obj + 0x08, hash);
        wr::<u8>(r15_obj, 0);

        exe::call_va!(0x1_4249_e2d0, (r15_obj + 0x10, 0xcu32, rbp + 0x130));
        exe::call_va!(0x1_4249_e0c0, (r15_obj + 0x10, rbp + 0x720, rbp + 0x130));
        exe::call_va!(0x1_4117_53c0, (r15_obj + 0x20, 0usize, rbp + 0x720));

        let _ = exe::call_va!(0x1_4183_acc2, (rbp + 0x3d0, 0u32, 0x310u64) -> usize);
        let slot_base = rbp + 0x3e0;
        wr::<usize>(rbp + 0x3d0, slot_base);
        wr::<u32>(rbp + 0x3dc, 0x10);
        wr::<u32>(rbp + 0x6e8, 2);
        wr::<u64>(rbp + 0x6f0, 0);
        wr::<u32>(rbp + 0x6f8, 0xffff_ffff);
        wr::<u64>(rbp + 0x700, 0);
        wr::<u64>(rbp + 0x6e0, 0);
        wr::<[u64; 6]>(slot_base, [0; 6]);
        wr::<u32>(rbp + 0x3d8, 1);

        let rbx_end = slot_base + 0x30;
        wr::<u64>(rbx_end - 0x30, 0);
        wr::<usize>(rbx_end - 0x28, exe::ptr::<u8>(0x1_4309_f824) as usize);
        wr::<u64>(rbx_end - 0x20, 0);
        wr::<u64>(rbx_end - 0x18, 0);
        wr::<u32>(rbx_end - 0x08, 0xffff_ffff);
        wr::<u64>(rbx_end - 0x10, 0);

        wr::<usize>(rbp + 0x708, rbp + 0x720);
        wr::<usize>(rbp + 0x710, rbp + 0x130);

        let arr_38 = rd::<usize>(r15_obj + 0x38);
        let cnt_34 = rd::<i32>(r15_obj + 0x34);
        if cnt_34 > 0 {
            for i in 0..(cnt_34 as usize) {
                exe::call_va!(0x1_40ae_d6f0, (arr_38.wrapping_add(i * 0x170), rbp + 0x3d0));
            }
        }
        exe::call_va!(0x1_401e_e280, (rbp + 0x3d0));

        let mut rsi_handle: usize = 0;
        wr::<usize>(rbp + 0xf70, 0);
        let mut cur_38 = rd::<usize>(r15_obj + 0x38);
        let end_38 =
            cur_38.wrapping_add(((rd::<i32>(r15_obj + 0x34) as isize) * 0x170) as usize);
        while cur_38 != end_38 {
            let sub_cnt = rd::<u32>(cur_38 + 0x168) as u64;
            let sub_ptr = rd::<u64>(cur_38 + 0x160);
            if sub_ptr != sub_ptr.wrapping_add(sub_cnt * 8) {
                if rsi_handle == 0 {
                    wr::<u64>(rbp + 0xf78, sub_cnt);
                    wr::<u32>(rbp + 0xf80, 1000);
                    let ret_ptr = exe::call_va!(
                        0x1_4101_f0a0,
                        (rbp + 0xf88, sub_cnt, rbp + 0xf80, rbp + 0xf78) -> usize
                    );
                    rsi_handle = rd::<usize>(ret_ptr);
                    wr::<usize>(ret_ptr, 0);
                    wr::<usize>(rbp + 0xf70, rsi_handle);
                    let old_f88 = rd::<usize>(rbp + 0xf88);
                    if old_f88 != 0 {
                        exe::call_va!(0x1_4242_5650, (old_f88));
                        exe::call_va!(0x1_4244_2de0, (old_f88));
                    }
                }
                let sub_cnt2 = rd::<u32>(cur_38 + 0x168) as u64;
                let sub_ptr2 = rd::<u64>(cur_38 + 0x160);
                wr::<u64>(rsp + 0x30, sub_cnt2);
                wr::<u64>(rsp + 0x38, sub_ptr2);
                wr::<u128>(rsp + 0x50, rd::<u128>(rsp + 0x30));
                let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
                exe::call_va!(0x1_4242_0050, (g_2b8, rsp + 0x20, rsp + 0x50));

                let mut rbx_it = rd::<usize>(rsp + 0x20);
                let out_cnt = rd::<u32>(rsp + 0x28) as usize;
                let end_it = rbx_it.wrapping_add(out_cnt * 8);
                while rbx_it != end_it {
                    let elem_val = rd::<u64>(rbx_it);
                    if elem_val != 0 {
                        exe::call_va!(0x1_4242_50f0, (rsi_handle, elem_val));
                    }
                    rbx_it = rbx_it.wrapping_add(8);
                }
                let out_buf = rd::<usize>(rsp + 0x20);
                if out_buf != 0 {
                    exe::call_va!(0x1_4241_c190, (out_buf));
                }
                wr::<u64>(rsp + 0x20, 0);
                wr::<u64>(rsp + 0x28, 0);
            }
            cur_38 = cur_38.wrapping_add(0x170);
        }

        wr::<u64>(rbp + 0xf70, 0);
        let old_50 = rd::<usize>(r15_obj + 0x50);
        wr::<usize>(r15_obj + 0x50, rsi_handle);
        if old_50 != 0 {
            exe::call_va!(0x1_4242_5650, (old_50));
            exe::call_va!(0x1_4244_2de0, (old_50));
        }

        let vec_10 = param_1 + 0x10;
        let v_len = rd::<u32>(vec_10 + 0x08);
        let v_data = rd::<usize>(vec_10);
        let v_end = v_data.wrapping_add((v_len as usize) * 8);
        let v_cap = rd::<u32>(vec_10 + 0x0c) & 0x3fff_ffff;
        if v_len == v_cap {
            wr::<usize>(rbp + 0xf78, v_end);
            exe::call_va!(0x1_4026_8620, (vec_10, rbp + 0xf78, rsp + 0x48));
        } else {
            wr::<u32>(vec_10 + 0x08, v_len.wrapping_add(1));
            wr::<usize>(v_end, r15_obj);
        }

        wr::<usize>(rsp + 0x60, exe::ptr::<u8>(0x1_4340_eb60) as usize);
        if rd::<u32>(rbp + 0x80) == 1 {
            exe::call_va!(0x1_4249_6c20, (rsp + 0x60));
        }
        if rd::<u8>(rbp + 0x88) == 0 {
            exe::call_va!(0x1_413d_1ed0, (rbp + 0x70));
            let p90 = rd::<usize>(rbp + 0x90);
            if p90 != 0 {
                let vtbl = rd::<usize>(p90);
                exe::call_va!(rd::<usize>(vtbl), (p90, 0u32));
                exe::call_va!(0x1_4244_2de0, (p90));
            }
        }
        exe::call_va!(0x1_413d_1fb0, (rbp + 0x70));
        exe::call_va!(0x1_4240_ab10, (rsp + 0x68));
    }

    exe::call_va!(0x1_4240_ab10, (rbp + 0xa0));

    let edx_m1 = rd::<i32>(r14 + 0x6320).wrapping_sub(1);
    if edx_m1 >= 0 {
        let mut rcx_i = edx_m1 as isize;
        let mut slot_p = (r14 as isize + 0x6380 + rcx_i * 8) as usize;
        loop {
            if rd::<usize>(slot_p) == rbp + 0x1b0 {
                wr::<i32>(r14 + 0x6320, edx_m1);
                break;
            }
            slot_p = slot_p.wrapping_sub(8);
            rcx_i -= 1;
            if rcx_i < 0 {
                break;
            }
        }
    }

    exe::call_va!(0x1_4240_ab10, (rbp + 0x1b8));

    if r15_obj != 0 {
        let p50 = rd::<usize>(r15_obj + 0x50);
        if p50 != 0 {
            exe::call_va!(0x1_4242_5830, (p50));
            if rd::<u32>(p50 + 0x08) != 1 {
                let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
                let vtbl = rd::<usize>(g_2b8);
                exe::call_va!(rd::<usize>(vtbl + 0x10), (g_2b8, p50));
            }
        }
    }
}

/// `r_142069110`
/// Original: 0x142069110 (482 bytes, Throws::No)
pub unsafe extern "win64" fn r_142069110(
    param_1: usize,
    param_2: i32,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u8,
    param_7: u32,
    _param_8: u64,
    param_9: usize,
) -> usize {
    wr::<u32>(param_9, 0);
    if (param_7 & 0x20) == 0 && (param_7 & 0x10) == 0 {
        let flags = param_7 & 5;
        let fmt_idx = (param_6 as i8) as isize;
        let tbl_base = exe::ptr::<u8>(0x1_4297_eda0) as usize;
        let entry = (tbl_base as isize).wrapping_add(fmt_idx.wrapping_mul(0x14)) as usize;
        let dxgi_fmt = if (param_7 & 1) != 0 && ((param_7 >> 2) & 1) == 0 {
            rd::<u32>(entry)
        } else {
            rd::<u32>(entry + 0x04)
        };

        wr::<u32>(param_1, 3);
        wr::<u32>(param_1 + 0x04, 0);
        wr::<u64>(param_1 + 0x08, 0);
        wr::<u64>(param_1 + 0x10, (param_2 as i64) as u64);
        wr::<u32>(param_1 + 0x18, param_3);
        wr::<u16>(param_1 + 0x1c, param_4);
        wr::<u16>(param_1 + 0x1e, param_5);
        wr::<u32>(param_1 + 0x20, dxgi_fmt);
        wr::<u64>(param_1 + 0x24, 1);
        wr::<u32>(param_1 + 0x2c, 0);
        wr::<u32>(param_1 + 0x30, flags);
        wr::<u32>(param_1 + 0x34, 0);

        let mut tile: u32 = 0x10;
        let mut sz = exe::call_va!(0x1_4204_e7f0, (0x10u32, 0x10u32, param_6) -> i32);
        while sz < 0x1000 {
            sz = sz.wrapping_shl(2);
            tile = tile.wrapping_add(tile);
        }

        let mask = !tile.wrapping_sub(1);
        let w_aligned = (param_2 as u32).wrapping_sub(1).wrapping_add(tile) & mask;
        let h_aligned = param_3.wrapping_sub(1).wrapping_add(tile) & mask;
        let aligned_sz = exe::call_va!(0x1_4204_e7f0, (w_aligned, h_aligned, param_6) -> u32);
        if aligned_sz < 0x10000
            && rd::<u32>(param_1) == 3
            && rd::<u16>(param_1 + 0x1c) == 1
            && (param_7 & 1) == 0
            && (param_7 & 8) == 0
        {
            wr::<u64>(param_1 + 0x08, 0x1000);
        }
    } else {
        let row_bytes = exe::call_va!(0x1_4204_e7f0, (param_2 as u32, 1u32, param_6) -> u32);
        let pitch = row_bytes.wrapping_add(0xff) & 0xffff_ff00;
        wr::<u32>(param_9, pitch);
        let total_width = ((pitch.wrapping_mul(param_3) as i32) as i64) as u64;

        wr::<u32>(param_1, 1);
        wr::<u32>(param_1 + 0x04, 0);
        wr::<u64>(param_1 + 0x08, 0);
        wr::<u64>(param_1 + 0x10, total_width);
        wr::<u32>(param_1 + 0x18, 1);
        wr::<u64>(param_1 + 0x1c, 0x0001_0001);
        wr::<u64>(param_1 + 0x24, 1);
        wr::<u64>(param_1 + 0x2c, 1);
        wr::<u32>(param_1 + 0x34, 0);
    }
    param_1
}

/// `unknown_14209ac80`
/// Original: 0x14209ac80 (848 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14209ac80(param_1: usize) {
    const K1: u64 = 0x9ae1_6a3b_2f90_4057; // -0x651e95c4d06fbfa9
    const K2: u64 = 0x651e_95c4_d06f_bfb1;
    const K3: u64 = 0x9ddf_ea08_eb38_2d69; // -0x622015f714c7d297

    let h0 = exe::call_va!(0x1_424b_16a0, (param_1, 4u32) -> u64);
    let h1 = exe::call_va!(0x1_411d_ea10, (param_1 + 0x04, h0) -> u64);
    let mut r10 = exe::call_va!(0x1_411d_ea10, (param_1 + 0x08, h1) -> u64);

    let mut rbx = rd::<usize>(param_1 + 0x18);
    let cnt = rd::<u32>(param_1 + 0x20) as usize;
    let rdi_end = rbx.wrapping_add(cnt * 0x18);
    while rbx != rdi_end {
        let v0 = rd::<u32>(rbx) as u64;
        let mut a = ((v0 * 8 + 4) ^ v0).wrapping_mul(K1);
        a = ((a >> 0x2f) ^ a ^ v0).wrapping_mul(K1);
        let c = (((a >> 0x2f) ^ a).wrapping_mul(K1).wrapping_add(K2) ^ r10).wrapping_mul(K3);
        let r8 = ((((c >> 0x2f) ^ r10 ^ c).wrapping_mul(K3)) >> 0x2f
            ^ ((c >> 0x2f) ^ r10 ^ c).wrapping_mul(K3))
        .wrapping_mul(K3);

        let v1 = rd::<u32>(rbx + 0x04) as u64;
        let mut a1 = ((v1 * 8 + 4) ^ v1).wrapping_mul(K1);
        a1 = ((a1 >> 0x2f) ^ a1 ^ v1).wrapping_mul(K1);
        let c1 = (((a1 >> 0x2f) ^ a1).wrapping_mul(K1).wrapping_add(K2) ^ r8).wrapping_mul(K3);
        let t1 = ((c1 >> 0x2f) ^ c1 ^ r8).wrapping_mul(K3);
        let mut r14 = ((t1 >> 0x2f) ^ t1).wrapping_mul(K3);

        let sub2 = exe::call_va!(0x1_424b_16a0, (rbx + 0x08, 4u32) -> u64);
        let c2 = (sub2.wrapping_add(K2) ^ r14).wrapping_mul(K3);
        let t2 = ((c2 >> 0x2f) ^ c2 ^ r14).wrapping_mul(K3);
        r14 = ((t2 >> 0x2f) ^ t2).wrapping_mul(K3);

        let sub3 = exe::call_va!(0x1_424b_16a0, (rbx + 0x0c, 4u32) -> u64);
        let c3 = (sub3.wrapping_add(K2) ^ r14).wrapping_mul(K3);
        let t3 = ((c3 >> 0x2f) ^ c3 ^ r14).wrapping_mul(K3);
        r14 = ((t3 >> 0x2f) ^ t3).wrapping_mul(K3);

        let sub4 = exe::call_va!(0x1_424b_16a0, (rbx + 0x10, 4u32) -> u64);
        let c4 = (sub4.wrapping_add(K2) ^ r14).wrapping_mul(K3);
        let t4 = ((c4 >> 0x2f) ^ c4 ^ r14).wrapping_mul(K3);
        r10 = ((t4 >> 0x2f) ^ t4).wrapping_mul(K3);

        rbx = rbx.wrapping_add(0x18);
    }

    let mut hash_slot: u64 = r10;
    let g_608 = exe::ptr::<u64>(0x1_4386_1608) as usize;
    let g_5e0 = exe::ptr::<u64>(0x1_4386_15e0) as usize;
    let g_5e8 = exe::ptr::<u64>(0x1_4386_15e8) as usize;
    let g_5f0 = exe::ptr::<u64>(0x1_4386_15f0) as usize;

    let mut iter_f8 = [0u64; 9];
    let f8_ptr = iter_f8.as_mut_ptr() as usize;

    if rd::<u64>(g_608) == rd::<u64>(g_5e0) {
        let _ = exe::call_va!(0x1_411e_3790, (0usize, f8_ptr) -> usize);
    } else {
        let mut probe_20 = [0u64; 4];
        let p20_ptr = probe_20.as_mut_ptr() as usize;
        let _ = exe::call_va!(
            0x1_411e_3960,
            (0usize, p20_ptr, (&mut hash_slot as *mut u64) as usize) -> usize
        );
        let found_idx = rd::<u64>(p20_ptr);
        if found_idx == u64::MAX {
            let _ = exe::call_va!(0x1_411e_3790, (0usize, f8_ptr) -> usize);
        } else {
            let v_5f0 = rd::<u64>(g_5f0);
            let v_5e8 = rd::<u64>(g_5e8);
            wr::<u64>(p20_ptr, v_5e8);
            wr::<u64>(p20_ptr + 0x08, v_5f0);
            wr::<u64>(p20_ptr + 0x10, v_5f0);
            wr::<u64>(p20_ptr + 0x18, 0);

            let mut tmp_48 = [0u64; 4];
            let out_ptr = exe::call_va!(
                0x1_4104_efd0,
                (g_5e8, tmp_48.as_mut_ptr() as usize, found_idx) -> usize
            );
            let mut copy_30 = rd::<[u64; 4]>(out_ptr);
            let _ = exe::call_va!(
                0x1_411e_38a0,
                (
                    f8_ptr,
                    0usize,
                    copy_30.as_mut_ptr() as usize,
                    p20_ptr
                ) -> usize
            );
        }
    }

    let mut iter_b0 = [0u64; 9];
    let b0_ptr = iter_b0.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_411e_3790, (0usize, b0_ptr) -> usize);

    let eq = rd::<u64>(f8_ptr + 0x08) == rd::<u64>(b0_ptr + 0x08)
        && rd::<u64>(f8_ptr + 0x10) == rd::<u64>(b0_ptr + 0x10)
        && rd::<u64>(f8_ptr + 0x18) == rd::<u64>(b0_ptr + 0x18)
        && (rd::<u64>(f8_ptr + 0x18) == rd::<u64>(f8_ptr + 0x10)
            || rd::<u64>(f8_ptr + 0x20) == rd::<u64>(b0_ptr + 0x20));

    if eq {
        let diff = (rd::<u32>(g_608)).wrapping_sub(rd::<u32>(g_5e0));
        let slot = exe::call_va!(
            0x1_411e_6b60,
            (0usize, (&mut hash_slot as *mut u64) as usize) -> usize
        );
        wr::<u32>(slot + 0x08, diff);
        wr::<u32>(param_1 + 0x10, diff);
    } else {
        let slot = exe::call_va!(
            0x1_411e_6b60,
            (0usize, (&mut hash_slot as *mut u64) as usize) -> usize
        );
        wr::<u32>(param_1 + 0x10, rd::<u32>(slot + 0x08));
    }
}

/// `r_1420b1ae0`
/// Original: 0x1420b1ae0 (2117 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_1420b1ae0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
    param_5: usize,
    param_6: u64,
) {
    #[repr(C, align(32))]
    struct Frame([u8; 0x6a0]);

    let mut r15 = rd::<u64>(param_5);
    if r15 == 0 {
        return;
    }

    let mut frame = Frame([0u8; 0x6a0]);
    let rsp = frame.0.as_mut_ptr() as usize;
    let rbp = rsp + 0x100;

    wr::<u64>(rsp + 0x50, r15);
    wr::<usize>(rbp + 0x18, param_1.wrapping_add(0x70));
    wr::<usize>(rbp + 0x10, param_1.wrapping_add(0xc0));
    wr::<usize>(rbp + 0x08, param_1.wrapping_add(0x10));

    let r13 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    wr::<usize>(rsp + 0x40, r13);
    wr::<usize>(rsp + 0x48, rd::<usize>(r13 + 0x10));
    let rsi_workers = rd::<u64>(r13);
    wr::<u64>(rbp + 0x20, rsi_workers);
    wr::<u64>(rbp + 0x38, rd::<u32>(param_1 + 0x68) as u64);
    wr::<u64>(rbp + 0x40, rd::<u64>(param_1 + 0x60));

    let ctor = exe::ptr::<u8>(0x1_420b_1a80) as usize;
    let dtor = exe::ptr::<u8>(0x1_420b_1850) as usize;
    exe::call_va!(0x1_4183_4fc0, (rbp + 0x110, 0x10u64, 0x40u64, ctor, dtor));

    wr::<usize>(rbp + 0x58, exe::ptr::<u8>(0x1_433d_d7a8) as usize);
    wr::<u64>(rbp + 0x60, 0x23);
    let rbx_10 = rd::<usize>(r13 + 0x10);
    let wid0 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool40_0 = rd::<usize>(rbx_10.wrapping_add(wid0.wrapping_shl(6)).wrapping_add(0x48))
        .wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool40_0, rsp + 0x58));
    let jc1 = rd::<usize>(rsp + 0x58);
    let _ = crate::simd_ops::atomic_xchg_u32(jc1 as u64, 1);
    let jc1 = rd::<usize>(rsp + 0x58);
    wr::<u64>(jc1 + 0x08, 0);
    wr::<usize>(jc1 + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<u128>(jc1 + 0x18, rd::<u128>(rbp + 0x58));
    let (ok1, _) = crate::simd_ops::atomic_cmpxchg_u8((jc1 + 0x04) as u64, 1, 3);
    if ok1 {
        let jc1_b = rd::<usize>(rsp + 0x58);
        let _ = crate::simd_ops::atomic_xadd_u32(jc1_b as u64, 0xffff);
    }
    wr::<u8>(rsp + 0x62, 1);
    wr::<u64>(rsp + 0x78, 0);

    let r12_arr = rd::<usize>(param_2 + 0x10);
    let mut r14_total = rd::<u32>(param_2 + 0x18) as u64;
    let div_by = rsi_workers.wrapping_mul(3);
    let rdi_chunk = r14_total / div_by;
    wr::<u64>(rbp - 0x80, r14_total);
    let mut rsi_done: u64 = 0;

    if rdi_chunk != 0 && (r14_total as i64) >= (rdi_chunk as i64) {
        wr::<u64>(rbp + 0x78, 0x40);
        wr::<usize>(rbp + 0x80, rbp + 0x110);
        let mut r15_off: usize = 0;
        let step_bytes = (rdi_chunk as usize).wrapping_mul(0x28);
        wr::<u64>(rbp, step_bytes as u64);

        loop {
            let rcx_ptr = r12_arr.wrapping_add(r15_off);
            let chunk_n = if rdi_chunk == u64::MAX {
                r14_total.wrapping_sub(rsi_done)
            } else {
                rdi_chunk
            };
            wr::<u64>(rbp + 0x68, chunk_n);
            wr::<usize>(rbp + 0x70, rcx_ptr);
            wr::<u128>(rbp - 0x58, rd::<u128>(rbp + 0x68));
            wr::<u128>(rbp - 0x48, rd::<u128>(param_5));
            wr::<u64>(rbp - 0x38, param_4);
            let xmm6_val = rd::<u128>(rbp + 0x78);
            wr::<u128>(rbp - 0x30, xmm6_val);

            let rbx_10 = rd::<usize>(r13 + 0x10);
            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool80 = rd::<usize>(rbx_10.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48))
                .wrapping_add(0x80);
            exe::call_va!(0x1_4248_5aa0, (pool80, rbp + 0x28));
            let job1 = rd::<usize>(rbp + 0x28);
            let _ = crate::simd_ops::atomic_xchg_u32(job1 as u64, 1);
            let job1 = rd::<usize>(rbp + 0x28);
            wr::<u64>(job1 + 0x08, 0);
            wr::<usize>(job1 + 0x10, exe::ptr::<u8>(0x1_433d_d808) as usize);
            wr::<[u128; 2]>(job1 + 0x18, rd::<[u128; 2]>(rbp - 0x58));
            wr::<u128>(job1 + 0x38, rd::<u128>(rbp - 0x38));
            wr::<u64>(job1 + 0x48, (xmm6_val >> 64) as u64);

            let jc1_p = rd::<usize>(rsp + 0x58);
            let _ = crate::simd_ops::atomic_xadd_u32(jc1_p as u64, 1);
            let jc1_p = rd::<usize>(rsp + 0x58);
            wr::<usize>(job1 + 0x08, jc1_p);
            exe::call_va!(
                0x1_4248_6460,
                (rd::<usize>(rsp + 0x48), rd::<usize>(rbp + 0x28))
            );

            let r9_cnt = rd::<u64>(rsp + 0x78).wrapping_add(1);
            wr::<u64>(rsp + 0x78, r9_cnt);
            let rem = rd::<u64>(rbp - 0x80).wrapping_sub(rdi_chunk);
            wr::<u64>(rbp - 0x80, rem);
            rsi_done = rsi_done.wrapping_add(rdi_chunk);
            r15_off = r15_off.wrapping_add(rd::<usize>(rbp));

            if (rem as i64) < (rdi_chunk as i64) {
                if r9_cnt != 0 {
                    let q_ptr = rd::<usize>(rd::<usize>(rsp + 0x48));
                    let sem = rd::<u64>(q_ptr + 0x1008);
                    let _ = exe::call_iat!(0x1_4264_5580, (sem, r9_cnt as u32, 0usize) -> i32);
                }
                break;
            }
        }
        r15 = rd::<u64>(rsp + 0x50);
    }

    wr::<u64>(rbp + 0x90, 0x40);
    wr::<usize>(rbp + 0x98, rbp + 0x110);
    r14_total = r14_total.wrapping_sub(rsi_done);
    wr::<u128>(rbp + 0xf0, rd::<u128>(rbp + 0x90));
    wr::<u128>(rbp + 0x100, rd::<u128>(param_5));
    wr::<u64>(rbp + 0xa0, r14_total);
    wr::<usize>(
        rbp + 0xa8,
        r12_arr.wrapping_add((rsi_done as usize).wrapping_mul(0x28)),
    );
    exe::call_va!(
        0x1_420b_2b50,
        (rbp + 0xa0, rbp + 0x100, param_4, rbp + 0xf0)
    );
    exe::call_va!(0x1_4248_5680, (r13, rsp + 0x58));

    wr::<u64>(rbp + 0xb0, 0x40);
    wr::<usize>(rbp + 0xb8, rbp + 0x110);
    wr::<u128>(rbp - 0x58, rd::<u128>(rbp + 0xb0));
    wr::<u128>(rbp - 0x48, rd::<u128>(rbp + 0x38));
    wr::<u64>(rbp - 0x38, rd::<u64>(rbp + 0x08));
    let rbx_10 = rd::<usize>(r13 + 0x10);
    let wid2 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool80_2 = rd::<usize>(rbx_10.wrapping_add(wid2.wrapping_shl(6)).wrapping_add(0x48))
        .wrapping_add(0x80);
    exe::call_va!(0x1_4248_5aa0, (pool80_2, rbp - 0x78));
    let job2 = rd::<usize>(rbp - 0x78);
    let _ = crate::simd_ops::atomic_xchg_u32(job2 as u64, 1);
    let job2 = rd::<usize>(rbp - 0x78);
    wr::<u64>(job2 + 0x08, 0);
    wr::<usize>(job2 + 0x10, exe::ptr::<u8>(0x1_433d_d7d8) as usize);
    wr::<[u128; 2]>(job2 + 0x18, rd::<[u128; 2]>(rbp - 0x58));
    wr::<u64>(job2 + 0x38, rd::<u64>(rbp - 0x38));
    if rd::<u32>(job2) == 1 {
        let rbx_10 = rd::<usize>(r13 + 0x10);
        exe::call_va!(0x1_4248_6460, (rbx_10, rd::<usize>(rbp - 0x78)));
        let sem = rd::<u64>(rd::<usize>(rbx_10) + 0x1008);
        let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
    }

    wr::<u64>(rbp + 0xc0, 0x40);
    wr::<usize>(rbp + 0xc8, rbp + 0x110);
    wr::<u16>(rbp - 0x58, r15 as u16);
    wr::<u128>(rbp - 0x50, rd::<u128>(param_5));
    wr::<u128>(rbp - 0x40, rd::<u128>(rbp + 0xc0));
    wr::<u128>(rbp - 0x30, rd::<u128>(rbp + 0x38));
    wr::<u64>(rbp - 0x20, param_4);
    wr::<u64>(rbp - 0x18, rd::<u64>(rbp + 0x10));
    wr::<u64>(rbp - 0x10, rd::<u64>(rbp + 0x18));
    wr::<u64>(rbp - 0x08, param_6);
    let rbx_10 = rd::<usize>(r13 + 0x10);
    let wid3 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool80_3 = rd::<usize>(rbx_10.wrapping_add(wid3.wrapping_shl(6)).wrapping_add(0x48))
        .wrapping_add(0x80);
    exe::call_va!(0x1_4248_5aa0, (pool80_3, rbp - 0x68));
    let job3 = rd::<usize>(rbp - 0x68);
    let _ = crate::simd_ops::atomic_xchg_u32(job3 as u64, 1);
    let job3 = rd::<usize>(rbp - 0x68);
    wr::<u64>(job3 + 0x08, 0);
    wr::<usize>(job3 + 0x10, exe::ptr::<u8>(0x1_433d_d820) as usize);
    wr::<[u128; 2]>(job3 + 0x18, rd::<[u128; 2]>(rbp - 0x58));
    wr::<[u128; 2]>(job3 + 0x38, rd::<[u128; 2]>(rbp - 0x38));
    wr::<u128>(job3 + 0x58, rd::<u128>(rbp - 0x18));
    wr::<u64>(job3 + 0x68, rd::<u64>(rbp - 0x08));
    if rd::<u32>(job3) == 1 {
        let rbx_10 = rd::<usize>(r13 + 0x10);
        exe::call_va!(0x1_4248_6460, (rbx_10, rd::<usize>(rbp - 0x68)));
        let sem = rd::<u64>(rd::<usize>(rbx_10) + 0x1008);
        let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
    }

    wr::<usize>(rbp + 0xd0, exe::ptr::<u8>(0x1_433d_d778) as usize);
    wr::<u64>(rbp + 0xd8, 0x2f);
    let rbx_10 = rd::<usize>(r13 + 0x10);
    let wid4 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool40_2 = rd::<usize>(rbx_10.wrapping_add(wid4.wrapping_shl(6)).wrapping_add(0x48))
        .wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool40_2, rsp + 0x68));
    let jc2 = rd::<usize>(rsp + 0x68);
    let _ = crate::simd_ops::atomic_xchg_u32(jc2 as u64, 1);
    let jc2 = rd::<usize>(rsp + 0x68);
    wr::<u64>(jc2 + 0x08, 0);
    wr::<usize>(jc2 + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<u128>(jc2 + 0x18, rd::<u128>(rbp + 0xd0));
    let (ok2, _) = crate::simd_ops::atomic_cmpxchg_u8((jc2 + 0x04) as u64, 1, 3);
    if ok2 {
        let jc2_b = rd::<usize>(rsp + 0x68);
        let _ = crate::simd_ops::atomic_xadd_u32(jc2_b as u64, 0xffff);
    }
    wr::<u8>(rsp + 0x72, 1);

    let mut total_elems: u64 = 0;
    let mut cur_vec = rbp + 0x110;
    while cur_vec != rbp + 0x510 {
        total_elems = total_elems.wrapping_add(rd::<u32>(cur_vec + 0x08) as u64);
        cur_vec = cur_vec.wrapping_add(0x10);
    }

    let r12_buf = exe::call_va!(0x1_4241_c0a0, (total_elems.wrapping_mul(4), 4u32) -> usize);
    let mut dst_idx: usize = 0;
    let mut cur_vec = rbp + 0x110;
    while cur_vec != rbp + 0x510 {
        let n = rd::<u32>(cur_vec + 0x08) as usize;
        let _ = exe::call_va!(
            0x1_4183_acbc,
            (
                r12_buf.wrapping_add(dst_idx * 4),
                rd::<usize>(cur_vec),
                (n * 4) as u64
            ) -> usize
        );
        dst_idx = dst_idx.wrapping_add(n);
        cur_vec = cur_vec.wrapping_add(0x10);
    }

    exe::call_va!(0x1_411e_fa40, (r12_buf, total_elems as u32));
    wr::<u64>(rbp + 0x580, 0);

    let chunk2 = total_elems / rd::<u64>(rbp + 0x20);
    let mut rsi_pos: u64 = 0;
    if chunk2 != 0 && (total_elems as i64) > (chunk2 as i64) {
        let mut rdi_rem = total_elems;
        loop {
            let mut rdx_len = chunk2;
            let r8_start = rsi_pos;
            rsi_pos = rsi_pos.wrapping_add(chunk2);
            rdi_rem = rdi_rem.wrapping_sub(chunk2);

            let mut rax_hi = r12_buf.wrapping_add((rsi_pos as usize) * 4 + 2);
            let cx_hi = rd::<u16>(rax_hi);
            while (rdi_rem as i64) > 0 && rd::<u16>(rax_hi) == cx_hi {
                rsi_pos = rsi_pos.wrapping_add(1);
                rax_hi = rax_hi.wrapping_add(4);
                rdx_len = rdx_len.wrapping_add(1);
                rdi_rem = rdi_rem.wrapping_sub(1);
            }

            let rcx_sub = r12_buf.wrapping_add((r8_start as usize) * 4);
            if rdx_len == u64::MAX {
                rdx_len = total_elems.wrapping_sub(r8_start);
            }
            wr::<u64>(rbp + 0xe0, rdx_len);
            wr::<usize>(rbp + 0xe8, rcx_sub);
            let xmm6_val = rd::<u128>(rbp + 0xe0);
            wr::<usize>(rbp - 0x48, param_3);

            let r13_cur = rd::<usize>(rsp + 0x40);
            let rbx_10 = rd::<usize>(r13_cur + 0x10);
            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool80 = rd::<usize>(rbx_10.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48))
                .wrapping_add(0x80);
            exe::call_va!(0x1_4248_5aa0, (pool80, rbp + 0x48));
            let job4 = rd::<usize>(rbp + 0x48);
            let _ = crate::simd_ops::atomic_xchg_u32(job4 as u64, 1);
            let job4 = rd::<usize>(rbp + 0x48);
            wr::<u64>(job4 + 0x08, 0);
            wr::<usize>(job4 + 0x10, exe::ptr::<u8>(0x1_433d_d7f0) as usize);
            wr::<u128>(job4 + 0x18, xmm6_val);
            wr::<u64>(job4 + 0x28, rd::<u64>(rbp - 0x48));

            let jc2_p = rd::<usize>(rsp + 0x68);
            let _ = crate::simd_ops::atomic_xadd_u32(jc2_p as u64, 1);
            let jc2_p = rd::<usize>(rsp + 0x68);
            wr::<usize>(job4 + 0x08, jc2_p);
            let rbx_48 = rd::<usize>(rsp + 0x48);
            exe::call_va!(0x1_4248_6460, (rbx_48, rd::<usize>(rbp + 0x48)));

            let r9_cnt = rd::<u64>(rbp + 0x580).wrapping_add(1);
            wr::<u64>(rbp + 0x580, r9_cnt);
            if (rdi_rem as i64) <= (chunk2 as i64) {
                if r9_cnt != 0 {
                    let sem = rd::<u64>(rd::<usize>(rbx_48) + 0x1008);
                    let _ = exe::call_iat!(0x1_4264_5580, (sem, r9_cnt as u32, 0usize) -> i32);
                }
                break;
            }
        }
    }

    let rem_count = total_elems.wrapping_sub(rsi_pos) as usize;
    let mut rbx_cur = r12_buf.wrapping_add((rsi_pos as usize) * 4);
    let rsi_end = rbx_cur.wrapping_add(rem_count * 4);
    while rbx_cur != rsi_end {
        let r10d = rd::<u32>(rbx_cur);
        let r10w = r10d as u16;
        wr::<u16>(rsp + 0x38, r10w);
        let rdi_vec = rd::<usize>(param_3).wrapping_add(((r10d >> 16) as usize) * 0x10);
        let r11_data = rd::<usize>(rdi_vec);
        let mut rcx_len = rd::<u32>(rdi_vec + 0x08) as usize;
        let mut rax_lo: usize = 0;
        while rcx_len != 0 {
            let r9_half = rcx_len >> 1;
            let rdx_mid = rax_lo.wrapping_add(r9_half);
            let rem_half = rcx_len.wrapping_sub(r9_half);
            if rd::<u16>(r11_data.wrapping_add(rdx_mid * 2)) <= r10w {
                rax_lo = rax_lo.wrapping_add(rem_half);
            }
            rcx_len = r9_half;
        }
        exe::call_va!(
            0x1_411f_00d0,
            (
                rdi_vec,
                rsp + 0x50,
                r11_data.wrapping_add(rax_lo * 2),
                rsp + 0x38
            )
        );
        rbx_cur = rbx_cur.wrapping_add(4);
    }

    let r13_fin = rd::<usize>(rsp + 0x40);
    exe::call_va!(0x1_4248_5680, (r13_fin, rbp - 0x78));
    exe::call_va!(0x1_4248_5680, (r13_fin, rbp - 0x68));
    exe::call_va!(0x1_4248_5680, (r13_fin, rsp + 0x68));
    exe::call_va!(0x1_4241_c190, (r12_buf));
    exe::call_va!(0x1_4183_4ee8, (rbp + 0x110, 0x10u64, 0x40u64, dtor));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141f7a230(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_9b60(
        p1: u64,
        _p2: usize,
        r8_ptr: usize,
        r9_key: u64,
        arg5_p3: u64,
        arg6_val: u64,
    ) {
        let idx = rd::<u32>(r8_ptr) as u64;
        let flag = rd::<u8>(r8_ptr + 0x08) as u64;
        LOG_SUM = LOG_SUM
            .wrapping_mul(0x100_0000_01b3)
            .wrapping_add(p1 ^ idx ^ (flag << 16) ^ r9_key ^ arg5_p3 ^ arg6_val);
    }

    unsafe extern "win64" fn mock_cb_call(
        _this: usize,
        rdx_p2: usize,
        r8_idx: usize,
        r9_key: usize,
    ) {
        let p2_cnt = rd::<u32>(rd::<usize>(rdx_p2) + 0x68) as u64;
        let idx = rd::<u32>(r8_idx) as u64;
        let key = rd::<u64>(r9_key);
        LOG_SUM = LOG_SUM
            .wrapping_mul(0x100_0000_01b3)
            .wrapping_add(p2_cnt ^ (idx << 8) ^ key);
    }

    unsafe extern "win64" fn mock_cb_destroy(_this: usize, not_self: u8) {
        LOG_SUM = LOG_SUM
            .wrapping_mul(0x100_0000_01b3)
            .wrapping_add(0xdd00 + not_self as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f7_9b60, mock_9b60 as *const ())?;

    let run = |f: F, has_cb: bool| -> (u32, usize, u64) {
        LOG_SUM = 0;

        let mut p2 = [0u8; 0x70];
        let p2_ptr = p2.as_mut_ptr() as usize;
        wr::<u32>(p2_ptr + 0x68, 10);

        // 4 buckets: empty, valid1, tombstone, valid2
        let buckets: [u64; 8] = [
            u64::MAX,
            0,
            0x1111,
            0xaaaa,
            u64::MAX - 1,
            0,
            0x2222,
            0xbbbb,
        ];
        let mut p4 = [0u8; 0x60];
        let p4_ptr = p4.as_mut_ptr() as usize;
        wr::<u64>(p4_ptr + 0x28, u64::MAX - 1);
        wr::<u64>(p4_ptr + 0x30, 1);
        wr::<u64>(p4_ptr + 0x40, 4);
        wr::<u64>(p4_ptr + 0x48, u64::MAX);
        wr::<usize>(p4_ptr + 0x58, buckets.as_ptr() as usize);

        let vtbl: [usize; 5] = [
            0,
            0,
            mock_cb_call as *const () as usize,
            0,
            mock_cb_destroy as *const () as usize,
        ];
        let mut p5 = [0u64; 8];
        let p5_ptr = p5.as_mut_ptr() as usize;
        wr::<usize>(p5_ptr, vtbl.as_ptr() as usize);
        if has_cb {
            wr::<usize>(p5_ptr + 0x38, p5_ptr);
        }

        f(0x5555, p2_ptr, 0x7777, p4_ptr, p5_ptr);
        (rd::<u32>(p2_ptr + 0x68), rd::<usize>(p5_ptr + 0x38), LOG_SUM)
    };

    for has_cb in [true, false] {
        let o = run(f_o, has_cb);
        let n = run(f_n, has_cb);
        crate::diff_eq!(o, n, "141f7a230 has_cb={}", has_cb);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141fcb9a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;

    fn mix(v: u64) {
        unsafe {
            LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_dfe0(rcx_p5: usize, rdx_val: u64) {
        mix(rd::<u64>(rcx_p5) ^ rdx_val);
    }
    unsafe extern "win64" fn mock_fce0(
        _rcx: usize,
        rdx_b0: u64,
        r8_ent: u64,
        r9_mat: usize,
        _a5: usize,
        _a6: usize,
        a7_xmm2: u32,
        a8_r8b: u8,
        a9_r15b: u8,
        a10_opt: usize,
    ) {
        let m1_y = (rd::<u128>(r9_mat + 0x10) >> 32) as u32;
        let opt_has = rd::<u8>(a10_opt + 0x10) as u64;
        mix(
            rdx_b0
                ^ r8_ent
                ^ (m1_y as u64)
                ^ (a7_xmm2 as u64)
                ^ ((a8_r8b as u64) << 8)
                ^ ((a9_r15b as u64) << 16)
                ^ (opt_has << 24),
        );
    }
    unsafe extern "win64" fn mock_e480(
        _rcx: usize,
        _rdx: usize,
        r8_110: usize,
        r9_150: usize,
        a5_140: usize,
        a6_dt: u32,
    ) {
        let v110 = rd::<u32>(r8_110) as u64;
        let v150 = rd::<u32>(r9_150) as u64;
        let v140 = rd::<u32>(a5_140) as u64;
        mix(v110 ^ v150 ^ v140 ^ (a6_dt as u64));
    }
    unsafe extern "win64" fn mock_30f0(dst: usize, src: usize) {
        wr::<u128>(dst, rd::<u128>(src));
        mix(0x30f0 ^ rd::<u64>(src));
    }
    unsafe extern "win64" fn mock_f6c0(
        rcx_ent: u64,
        _rdx: usize,
        _r8: usize,
        _r9_unused: u64,
        a5_c8: usize,
        a6_120: usize,
        a7_r15b: u8,
    ) {
        let has_c8 = rd::<u8>(a5_c8 + 0x18) as u64;
        let b0 = rd::<u8>(a5_c8) as u64;
        let has_120 = rd::<u8>(a6_120 + 0x10) as u64;
        mix(rcx_ent ^ (has_c8 << 8) ^ b0 ^ (has_120 << 16) ^ ((a7_r15b as u64) << 24));
    }
    unsafe extern "win64" fn mock_2bc0(rcx: usize) {
        mix(0x2bc0 ^ rd::<u64>(rcx));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4115_dfe0, mock_dfe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41fc_fce0, mock_fce0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41fc_e480, mock_e480 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4116_30f0, mock_30f0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41fc_f6c0, mock_f6c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401f_2bc0, mock_2bc0 as *const ())?;

    let g_608 = exe::ptr::<u64>(0x1_43a3_4608) as usize;
    let s_608 = rd::<u64>(g_608);

    let run = |f: F, active: bool| -> (u64, u32, u32, u8, [u8; 32], u64) {
        LOG_HASH = 0;
        wr::<u64>(g_608, 2);

        let mut pushed_buf = [0u8; 64];
        let mut rdi_vec = [0u64; 8];
        let rdi_p = rdi_vec.as_mut_ptr() as usize;
        wr::<usize>(rdi_p, pushed_buf.as_mut_ptr() as usize);
        wr::<u32>(rdi_p + 0x08, 0);
        wr::<u32>(rdi_p + 0x0c, 2);

        let mut rbx_arr = [0u8; 0x208];
        let rbx_p = rbx_arr.as_mut_ptr() as usize;
        wr::<u32>(rbx_p, 3);
        wr::<u32>(rbx_p + 0x04, 7);

        let cnt_val: u32 = if active { 1 } else { 0 };
        let b8_arr: [usize; 2] = [(&cnt_val as *const u32) as usize, 0];

        let mut b0_vel = [0f32; 4];
        let a8_arr: [u64; 1] = [0x9999];

        let mut a0_mat = [0f32; 16];
        a0_mat[4] = 10.0;
        a0_mat[5] = 20.0;
        a0_mat[6] = 30.0;
        a0_mat[12] = 2.0;
        a0_mat[13] = 4.0;
        a0_mat[14] = 6.0;

        let arr_98: [u32; 1] = [1];

        let mut arr_90 = [0u8; 0x148];
        wr::<f32>(arr_90.as_mut_ptr() as usize + 0x18, 0.75);

        let mut arr_88 = [0u8; 0x10];
        wr::<u8>(arr_88.as_mut_ptr() as usize + 0x08, 3);
        wr::<u8>(arr_88.as_mut_ptr() as usize + 0x09, 5);
        wr::<f32>(arr_88.as_mut_ptr() as usize + 0x0c, 0.5);

        let mut arr_80 = [0u8; 0x18];
        wr::<u8>(arr_80.as_mut_ptr() as usize + 0x10, if active { 1 } else { 0 });
        wr::<u8>(arr_80.as_mut_ptr() as usize + 0x11, 1);

        let mut arr_78 = [0u8; 0x18];
        wr::<u8>(arr_78.as_mut_ptr() as usize, 11);
        wr::<u8>(arr_78.as_mut_ptr() as usize + 1, 22);
        wr::<u8>(arr_78.as_mut_ptr() as usize + 2, 33);
        wr::<u64>(arr_78.as_mut_ptr() as usize + 8, 0x1234_5678);

        let arr_70: [u64; 2] = [0xaaaa, 0xbbbb];
        let arr_60: [u64; 3] = [0, 0, 0x7777_8888];

        let p1: [u64; 15] = [
            rdi_p as u64,
            rbx_p as u64,
            b8_arr.as_ptr() as usize as u64,
            b0_vel.as_mut_ptr() as usize as u64,
            a8_arr.as_ptr() as usize as u64,
            a0_mat.as_ptr() as usize as u64,
            arr_98.as_ptr() as usize as u64,
            arr_90.as_ptr() as usize as u64,
            arr_88.as_ptr() as usize as u64,
            arr_80.as_ptr() as usize as u64,
            arr_78.as_ptr() as usize as u64,
            arr_70.as_ptr() as usize as u64,
            1,
            arr_60.as_ptr() as usize as u64,
            0,
        ];

        let dt: f32 = 2.0;
        let p3: [u64; 1] = [0x4444];
        let p4 = [0u8; 0xa0];

        f(
            p1.as_ptr() as usize,
            (&dt as *const f32) as usize,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            0xfeed_cafe,
        );

        let mut item0 = [0u8; 32];
        item0.copy_from_slice(&pushed_buf[0..32]);
        item0[3] = 0; // mask out 1-byte uninitialized stack padding at +3

        (
            rd::<u64>(g_608),
            rd::<u32>(rbx_p),
            rd::<u32>(rbx_p + 0x04),
            rd::<u8>(rbx_p + 0x1f8),
            item0,
            LOG_HASH,
        )
    };

    for active in [false, true] {
        let o = run(f_o, active);
        let n = run(f_n, active);
        wr::<u64>(g_608, s_608);
        crate::diff_eq!(o, n, "141fcb9a0 active={}", active);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141fdd230(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    static mut NEW_ARCH: [u8; 0x60] = [0; 0x60];
    static mut SUB_ARR: [u8; 0x170] = [0; 0x170];
    static mut SUB_ITEMS: [u64; 1] = [0x7777];

    fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_9b50(_dst: usize, sv: usize, _flag: u8) {
        rec(0x9b50 ^ rd::<u64>(sv + 0x08));
    }
    unsafe extern "win64" fn mock_c3c0(dst: usize, _flag: u8, _fmt: usize, _arg: usize) {
        // Set SSO string "a" (len 1, inline cap 0x10 => hdr = (0x10 << 24) | 1, crc = 0x1234)
        wr::<u32>(dst, (0x10u32 << 24) | 1);
        wr::<u32>(dst + 0x04, 0x1234);
        wr::<u8>(dst + 0x08, b'a');
        wr::<u8>(dst + 0x09, 0);
        rec(0xc3c0);
    }
    unsafe extern "win64" fn mock_f850(out_sv: usize, in_sv: usize) {
        wr::<u128>(out_sv, rd::<u128>(in_sv));
        rec(0xf850);
    }
    unsafe extern "win64" fn mock_1120(_sv: usize) -> u8 {
        rec(0x1120);
        1
    }
    unsafe extern "win64" fn mock_1d80(_c: usize, _d: usize, _r8: usize, r9: usize) -> usize {
        rec(0x1d80 ^ (rd::<u32>(r9) as u64));
        0
    }
    unsafe extern "win64" fn mock_70f0(_fa: usize) {
        rec(0x70f0);
    }
    unsafe extern "win64" fn mock_5310() -> usize {
        rec(0x5310);
        NEW_ARCH = [0; 0x60];
        SUB_ARR = [0; 0x170];
        let sp = core::ptr::addr_of_mut!(SUB_ARR) as usize;
        wr::<usize>(sp + 0x160, core::ptr::addr_of!(SUB_ITEMS) as usize);
        wr::<u32>(sp + 0x168, 1);
        let ap = core::ptr::addr_of_mut!(NEW_ARCH) as usize;
        wr::<i32>(ap + 0x34, 1);
        wr::<usize>(ap + 0x38, sp);
        ap
    }
    unsafe extern "win64" fn mock_e2d0(_rcx: usize, edx: u32, _r8: usize) {
        rec(0xe2d0 ^ (edx as u64));
    }
    unsafe extern "win64" fn mock_e0c0(_rcx: usize, _rdx: usize, _r8: usize) {
        rec(0xe0c0);
    }
    unsafe extern "win64" fn mock_53c0(_rcx: usize, _rdx: usize, _r8: usize) {
        rec(0x53c0);
    }
    unsafe extern "win64" fn mock_d6f0(_rcx: usize, rdx_ctx: usize) {
        rec(0xd6f0 ^ (rd::<u32>(rdx_ctx + 0x318) as u64));
    }
    unsafe extern "win64" fn mock_e280(_rcx: usize) {
        rec(0xe280);
    }
    unsafe extern "win64" fn mock_f0a0(
        out_ptr: usize,
        rdx_cnt: u64,
        r8_1000: usize,
        _r9: usize,
    ) -> usize {
        rec(0xf0a0 ^ rdx_cnt ^ (rd::<u32>(r8_1000) as u64));
        wr::<usize>(out_ptr, 0);
        out_ptr
    }
    unsafe extern "win64" fn mock_0050(_rcx: usize, out_span: usize, in_span: usize) {
        rec(0x0050 ^ rd::<u64>(in_span));
        wr::<u64>(out_span, 0);
        wr::<u64>(out_span + 0x08, 0);
    }
    unsafe extern "win64" fn mock_1ed0(_rcx: usize) {
        rec(0x1ed0);
    }
    unsafe extern "win64" fn mock_1fb0(_rcx: usize) {
        rec(0x1fb0);
    }
    unsafe extern "win64" fn mock_ab10(_rcx: usize) {
        rec(0xab10);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_405f_c3c0, mock_c3c0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4249_1120, mock_1120 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_413d_1d80, mock_1d80 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4249_70f0, mock_70f0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4117_5310, mock_5310 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4249_e2d0, mock_e2d0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4249_e0c0, mock_e0c0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4117_53c0, mock_53c0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_40ae_d6f0, mock_d6f0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4101_f0a0, mock_f0a0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4242_0050, mock_0050 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_413d_1ed0, mock_1ed0 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_413d_1fb0, mock_1fb0 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let tls_block = rd::<usize>(exe::tls_pointer());
    let s_depth = rd::<i32>(tls_block + 0x6320);

    let run = |f: F, cached: bool| -> (u32, u32, u64, u64) {
        TRACE = 0;
        wr::<i32>(tls_block + 0x6320, 0);

        let name = b"Hero\0";
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &b in &name[..4] {
            h = (h ^ ((b as i8 as i64 as u64) | 0x20)).wrapping_mul(0x0100_0000_01b3);
        }

        let mut existing_arch = [0u64; 4];
        let ep = existing_arch.as_mut_ptr() as usize;
        wr::<u32>(ep + 0x04, 5);
        wr::<u64>(ep + 0x08, h);

        let mut arr_slots: [usize; 2] = [ep, 0];
        let mut p1 = [0u8; 0x28];
        let p1_ptr = p1.as_mut_ptr() as usize;
        wr::<usize>(p1_ptr + 0x10, arr_slots.as_mut_ptr() as usize);
        wr::<u32>(p1_ptr + 0x18, if cached { 1 } else { 0 });
        wr::<u32>(p1_ptr + 0x1c, 2);

        f(p1_ptr, name.as_ptr() as usize, 0);

        let new_hash = if !cached {
            rd::<u64>(arr_slots[0] + 0x08)
        } else {
            rd::<u64>(ep + 0x08)
        };
        (
            rd::<u32>(ep + 0x04),
            rd::<u32>(p1_ptr + 0x18),
            new_hash,
            TRACE,
        )
    };

    for cached in [true, false] {
        let o = run(f_o, cached);
        let n = run(f_n, cached);
        wr::<i32>(tls_block + 0x6320, s_depth);
        crate::diff_eq!(o, n, "141fdd230 cached={}", cached);
    }
    Ok(2)
}

pub unsafe fn diff_r_142069110(orig: usize, new: usize) -> Result<u32, String> {
    type F =
        unsafe extern "win64" fn(usize, i32, u32, u16, u16, u8, u32, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_e7f0(w: u32, h: u32, fmt: u8) -> u32 {
        w.wrapping_mul(h).wrapping_mul((fmt as u32) + 2)
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_e7f0, mock_e7f0 as *const ())?;

    let run = |f: F, w: i32, h: u32, depth: u16, mips: u16, fmt: u8, flags: u32| -> ([u32; 14], u32)
    {
        let mut desc = [0u32; 14];
        let mut out_pitch: u32 = 0xdead;
        let dp = desc.as_mut_ptr() as usize;
        let ret = f(
            dp,
            w,
            h,
            depth,
            mips,
            fmt,
            flags,
            0,
            (&mut out_pitch as *mut u32) as usize,
        );
        assert_eq!(ret, dp);
        desc[1] = 0; // padding at +0x04
        desc[13] = 0; // padding at +0x34
        (desc, out_pitch)
    };

    for (w, h, depth, mips, fmt, flags) in [
        (32i32, 32u32, 1u16, 1u16, 1u8, 0u32),
        (512, 512, 1, 4, 2, 1),
        (512, 512, 1, 4, 2, 5),
        (128, 4, 1, 1, 2, 0x10),
        (64, 2, 1, 1, 1, 0x20),
    ] {
        let o = run(f_o, w, h, depth, mips, fmt, flags);
        let n = run(f_n, w, h, depth, mips, fmt, flags);
        crate::diff_eq!(o, n, "142069110 flags={:#x}", flags);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14209ac80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT_MEM: [u64; 4] = [0; 4];
    static mut LAST_HASH: u64 = 0;

    unsafe extern "win64" fn mock_16a0(ptr: usize, len: u32) -> u64 {
        (rd::<u32>(ptr) as u64).wrapping_mul((len as u64) + 0x1357)
    }
    unsafe extern "win64" fn mock_ea10(ptr: usize, prev: u64) -> u64 {
        prev.wrapping_mul(0x100_0000_01b3)
            .wrapping_add(rd::<u32>(ptr) as u64)
    }
    unsafe extern "win64" fn mock_3790(_rcx: usize, out_it: usize) -> usize {
        wr::<u64>(out_it + 0x08, 0x100);
        wr::<u64>(out_it + 0x10, 0x200);
        wr::<u64>(out_it + 0x18, 0x200);
        wr::<u64>(out_it + 0x20, 0x300);
        out_it
    }
    unsafe extern "win64" fn mock_3960(_rcx: usize, out_probe: usize, r8_hash: usize) -> usize {
        LAST_HASH = rd::<u64>(r8_hash);
        wr::<u64>(out_probe, u64::MAX);
        out_probe
    }
    unsafe extern "win64" fn mock_6b60(_rcx: usize, rdx_hash: usize) -> usize {
        LAST_HASH = rd::<u64>(rdx_hash);
        core::ptr::addr_of_mut!(SLOT_MEM) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_ea10, mock_ea10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_3790, mock_3790 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411e_3960, mock_3960 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411e_6b60, mock_6b60 as *const ())?;

    let g_608 = exe::ptr::<u64>(0x1_4386_1608) as usize;
    let g_5e0 = exe::ptr::<u64>(0x1_4386_15e0) as usize;
    let s_608 = rd::<u64>(g_608);
    let s_5e0 = rd::<u64>(g_5e0);

    let run = |f: F, n_items: u32, equal_globals: bool| -> (u32, u32, u64) {
        SLOT_MEM = [0, 77, 0, 0];
        LAST_HASH = 0;
        wr::<u64>(g_5e0, 10);
        wr::<u64>(g_608, if equal_globals { 10 } else { 15 });

        let items: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut p1 = [0u8; 0x30];
        let p1_ptr = p1.as_mut_ptr() as usize;
        wr::<u32>(p1_ptr, 0x11);
        wr::<u32>(p1_ptr + 0x04, 0x22);
        wr::<u32>(p1_ptr + 0x08, 0x33);
        wr::<usize>(p1_ptr + 0x18, items.as_ptr() as usize);
        wr::<u32>(p1_ptr + 0x20, n_items);

        f(p1_ptr);
        (
            rd::<u32>(p1_ptr + 0x10),
            rd::<u32>(core::ptr::addr_of!(SLOT_MEM) as usize + 0x08),
            LAST_HASH,
        )
    };

    for (cnt, eq) in [(0u32, true), (2, false)] {
        let o = run(f_o, cnt, eq);
        let n = run(f_n, cnt, eq);
        wr::<u64>(g_608, s_608);
        wr::<u64>(g_5e0, s_5e0);
        crate::diff_eq!(o, n, "14209ac80 cnt={} eq={}", cnt, eq);
    }
    Ok(2)
}

pub unsafe fn diff_r_1420b1ae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut JOBS: [[u64; 14]; 8] = [[0; 14]; 8];
    static mut JOB_IDX: usize = 0;
    static mut R12_BUF: [u32; 16] = [0; 16];
    static mut LOG_HASH: u64 = 0;

    fn mix(v: u64) {
        unsafe {
            LOG_HASH = LOG_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_alloc_job(_pool: usize, out_handle: usize) {
        let idx = JOB_IDX;
        JOB_IDX = (idx + 1) & 7;
        let ptr = core::ptr::addr_of_mut!(JOBS[idx]) as usize;
        wr::<usize>(out_handle, ptr);
    }
    unsafe extern "win64" fn mock_enqueue(sched: usize, job: usize) {
        let sem = rd::<u64>(rd::<usize>(sched) + 0x1008);
        mix(sem ^ rd::<u64>(job + 0x10));
    }
    unsafe extern "win64" fn mock_rel_sem(sem: u64, count: u32, _prev: usize) -> i32 {
        mix(sem ^ (count as u64));
        1
    }
    unsafe extern "win64" fn mock_2b50(
        rcx_span: usize,
        rdx_p5: usize,
        r8_p4: u64,
        r9_vecs: usize,
    ) {
        let cnt = rd::<u64>(rcx_span);
        let p5_0 = rd::<u64>(rdx_p5);
        let vec_cnt = rd::<u64>(r9_vecs);
        let vec_arr = rd::<usize>(r9_vecs + 0x08);
        mix(cnt ^ p5_0 ^ r8_p4 ^ vec_cnt);
        // Populate first vector in the 64-element array with 2 items
        static ITEMS_0: [u32; 2] = [(1u32 << 16) | 5, (0u32 << 16) | 3];
        wr::<usize>(vec_arr, ITEMS_0.as_ptr() as usize);
        wr::<u32>(vec_arr + 0x08, 2);
    }
    unsafe extern "win64" fn mock_5680(_r13: usize, _handle: usize) {
        mix(0x5680);
    }
    unsafe extern "win64" fn mock_c0a0(bytes: u64, align: u32) -> usize {
        mix(bytes ^ (align as u64));
        core::ptr::addr_of_mut!(R12_BUF) as usize
    }
    unsafe extern "win64" fn mock_fa40(buf: usize, count: u32) {
        mix((count as u64) ^ (rd::<u32>(buf) as u64));
    }
    unsafe extern "win64" fn mock_00d0(
        rdi_vec: usize,
        rdx_r15_ptr: usize,
        r8_pos_ptr: usize,
        r9_val_ptr: usize,
    ) {
        let base = rd::<usize>(rdi_vec);
        let offset = r8_pos_ptr.wrapping_sub(base);
        let val = rd::<u16>(r9_val_ptr) as u64;
        let r15_v = rd::<u64>(rdx_r15_ptr);
        mix((offset as u64) ^ (val << 8) ^ r15_v);
    }
    unsafe extern "win64" fn mock_c190(_buf: usize) {
        mix(0xc190);
    }
    unsafe extern "win64" fn mock_1850(_elem: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc_job as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_6460, mock_enqueue as *const ())?;
    let _m4 = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420b_2b50, mock_2b50 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_411e_fa40, mock_fa40 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_411f_00d0, mock_00d0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_420b_1850, mock_1850 as *const ())?;

    let g_2f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_2f8 = rd::<usize>(g_2f8);

    let run = |f: F, p5_cnt: u64| -> ([[u64; 14]; 8], [u32; 4], u64) {
        JOBS = [[0; 14]; 8];
        JOB_IDX = 0;
        R12_BUF = [0; 16];
        LOG_HASH = 0;

        let mut sched_inner = vec![0u8; 0x1020];
        wr::<usize>(sched_inner.as_mut_ptr() as usize + 0x1008, 0x8888);
        let mut sched = [0u8; 0x90];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sched_inner.as_mut_ptr() as usize);
        wr::<usize>(sp + 0x48, 0x3000);
        let mut g_obj = [0u64; 4];
        g_obj[0] = 2; // 2 workers => chunk1 = 4 / 6 = 0, chunk2 = 2 / 2 = 1
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sp);
        wr::<usize>(g_2f8, g_obj.as_mut_ptr() as usize);

        let mut p1 = [0u8; 0x100];
        let p1_ptr = p1.as_mut_ptr() as usize;
        wr::<u64>(p1_ptr + 0x60, 0xaaaa_0000);
        wr::<u32>(p1_ptr + 0x68, 3);

        let mut p2 = [0u8; 0x30];
        let p2_ptr = p2.as_mut_ptr() as usize;
        wr::<usize>(p2_ptr + 0x10, 0x40_0000);
        wr::<u32>(p2_ptr + 0x18, 4);

        let vec0_items: [u16; 3] = [1, 3, 7];
        let vec1_items: [u16; 2] = [2, 6];
        let mut vecs = [0u64; 4];
        let vp = vecs.as_mut_ptr() as usize;
        wr::<usize>(vp, vec0_items.as_ptr() as usize);
        wr::<u32>(vp + 0x08, 3);
        wr::<usize>(vp + 0x10, vec1_items.as_ptr() as usize);
        wr::<u32>(vp + 0x18, 2);
        let p3: [usize; 1] = [vp];

        let p5: [u64; 2] = [p5_cnt, 0xbbbb_0000];

        f(
            p1_ptr,
            p2_ptr,
            p3.as_ptr() as usize,
            0xcccc_0000,
            p5.as_ptr() as usize,
            0xdddd_0000,
        );

        let mut jobs_clean = JOBS;
        for j in &mut jobs_clean {
            j[4] = 0; // clear stack pointer in job2
            j[6] = 0; // clear stack pointer in job3
            j[7] = 0; // clear p1-relative pointer in job2 (+0x38)
            j[9] = 0; // clear stack pointer in job1
            j[11] = 0; // clear p1-relative pointer in job3 (+0x58)
            j[12] = 0; // clear p1-relative pointer in job3 (+0x60)
        }
        // Clear p3 pointer in job4 (+0x28 = index 5)
        jobs_clean[4][5] = 0;

        let r12_head = [R12_BUF[0], R12_BUF[1], R12_BUF[2], R12_BUF[3]];
        (jobs_clean, r12_head, LOG_HASH)
    };

    for cnt in [0u64, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        wr::<usize>(g_2f8, s_2f8);
        crate::diff_eq!(o, n, "1420b1ae0 cnt={}", cnt);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified(0x1_41f7_a230, "unknown_141f7a230", rend::cauldron_hot_x4a::unknown_141f7a230 as *const (), rend::cauldron_hot_x4a::diff_unknown_141f7a230),
// Replacement::verified_throws(0x1_41fc_b9a0, "unknown_141fcb9a0", rend::cauldron_hot_x4a::unknown_141fcb9a0 as *const (), Throws::Caught, rend::cauldron_hot_x4a::diff_unknown_141fcb9a0),
// Replacement::verified_throws(0x1_41fd_d230, "unknown_141fdd230", rend::cauldron_hot_x4a::unknown_141fdd230 as *const (), Throws::Caught, rend::cauldron_hot_x4a::diff_unknown_141fdd230),
// Replacement::verified(0x1_4206_9110, "r_142069110", rend::cauldron_hot_x4a::r_142069110 as *const (), rend::cauldron_hot_x4a::diff_r_142069110),
// Replacement::verified_throws(0x1_4209_ac80, "unknown_14209ac80", rend::cauldron_hot_x4a::unknown_14209ac80 as *const (), Throws::Discard, rend::cauldron_hot_x4a::diff_unknown_14209ac80),
// Replacement::verified_throws(0x1_420b_1ae0, "r_1420b1ae0", rend::cauldron_hot_x4a::r_1420b1ae0 as *const (), Throws::Caught, rend::cauldron_hot_x4a::diff_r_1420b1ae0),
