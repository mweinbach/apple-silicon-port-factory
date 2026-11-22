//! Decompiled `rend` hot functions shard ZA12 (`0x141f5a270..0x142063760`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn skip_map_buckets(
    mut cur: usize,
    end: usize,
    empty_key: u64,
    del_cnt: u64,
    del_key: u64,
) -> usize {
    while cur != end {
        let k = rd::<u64>(cur);
        if k != empty_key && (del_cnt == 0 || k != del_key) {
            break;
        }
        cur = cur.wrapping_add(0x18);
    }
    cur
}

#[inline(always)]
unsafe fn lock_spin_byte(lock_addr: usize) {
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(lock_addr) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8(lock_addr as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }
}

/// `unknown_141f5a270`
/// Original: 0x141f5a270 (1779 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f5a270(
    param_1: usize,
    param_2: u64,
    param_3: u64,
    param_4: u64,
) {
    let base_e0 = rd::<usize>(param_1 + 0xe0);
    let cap_c0 = rd::<u64>(param_1 + 0xc0) as usize;
    let end_e0 = base_e0.wrapping_add(cap_c0.wrapping_mul(0x18));

    if base_e0 != end_e0 {
        let mut r10 = skip_map_buckets(
            base_e0,
            end_e0,
            rd::<u64>(param_1 + 0xc8),
            rd::<u64>(param_1 + 0xb0),
            rd::<u64>(param_1 + 0xa8),
        );
        while r10 != end_e0 {
            let mut rax = rd::<usize>(r10 + 0x08);
            let rdx = rax.wrapping_add((rd::<u32>(r10 + 0x10) as usize) << 5);
            while rax != rdx {
                if rd::<u32>(rax + 0x18) == 0 {
                    wr::<u32>(rax + 0x18, 1);
                }
                rax = rax.wrapping_add(0x20);
            }
            r10 = r10.wrapping_add(0x18);
            if r10 == end_e0 {
                break;
            }
            r10 = skip_map_buckets(
                r10,
                end_e0,
                rd::<u64>(param_1 + 0xc8),
                rd::<u64>(param_1 + 0xb0),
                rd::<u64>(param_1 + 0xa8),
            );
        }
    }

    let rdi_start = rd::<usize>(param_1 + 0xe0);
    let rbx_end = rdi_start.wrapping_add((rd::<u64>(param_1 + 0xc0) as usize).wrapping_mul(0x18));
    let mut rdi = if rdi_start != rbx_end {
        skip_map_buckets(
            rdi_start,
            rbx_end,
            rd::<u64>(param_1 + 0xc8),
            rd::<u64>(param_1 + 0xb0),
            rd::<u64>(param_1 + 0xa8),
        )
    } else {
        rdi_start
    };

    let r13_end =
        rd::<usize>(param_1 + 0xe0).wrapping_add((rd::<u64>(param_1 + 0xc0) as usize).wrapping_mul(0x18));
    if rdi != r13_end {
        let r15_map = param_1 + 0x08;
        loop {
            let mut find_18 = [0u64; 2];
            exe::call_va!(
                0x1_410d_3d20,
                (r15_map, find_18.as_mut_ptr() as usize, rdi)
            );
            let entry_70: usize = if find_18[0] != u64::MAX {
                rd::<usize>(r15_map + 0x60)
                    .wrapping_add((find_18[0] as usize).wrapping_mul(0x18))
            } else {
                let cond = exe::call_va!(0x1_410d_6300, (r15_map) -> u8);
                if cond != 0 {
                    let mut buf_18 = [0u64; 3];
                    let rbx_key = exe::call_va!(
                        0x1_410d_5810,
                        (r15_map, buf_18.as_mut_ptr() as usize, rdi) -> usize
                    );
                    let mut find_28 = [0u64; 2];
                    exe::call_va!(
                        0x1_410d_3d20,
                        (r15_map, find_28.as_mut_ptr() as usize, rbx_key)
                    );
                    let resolved_entry = if find_28[0] != u64::MAX {
                        let rdx_base = rd::<usize>(r15_map + 0x60);
                        rdx_base.wrapping_add((find_28[0] as usize).wrapping_mul(0x18))
                    } else {
                        let mut buf_48 = [0u64; 3];
                        let ins = exe::call_va!(
                            0x1_410d_6210,
                            (
                                r15_map,
                                buf_48.as_mut_ptr() as usize,
                                rbx_key,
                                find_28[1]
                            ) -> usize
                        );
                        rd::<usize>(ins + 0x08)
                    };
                    exe::call_va!(
                        0x1_4023_6010,
                        ((buf_18.as_mut_ptr() as usize) + 0x08)
                    );
                    resolved_entry
                } else {
                    let mut buf_30 = [0u64; 3];
                    let tmp_key = exe::call_va!(
                        0x1_410d_5810,
                        (r15_map, buf_30.as_mut_ptr() as usize, rdi) -> usize
                    );
                    let mut buf_60 = [0u64; 5];
                    let ins = exe::call_va!(
                        0x1_410d_6210,
                        (
                            r15_map,
                            buf_60.as_mut_ptr() as usize,
                            tmp_key,
                            find_18[1]
                        ) -> usize
                    );
                    let resolved_entry = rd::<usize>(ins + 0x08);
                    exe::call_va!(
                        0x1_4023_6010,
                        ((buf_30.as_mut_ptr() as usize) + 0x08)
                    );
                    resolved_entry
                }
            };

            let mut rsi = rd::<usize>(rdi + 0x08);
            let r14_end = rsi.wrapping_add((rd::<u32>(rdi + 0x10) as usize) << 5);
            if rsi != r14_end {
                let mut r13_lock = rsi + 0x14;
                loop {
                    if rd::<u32>(r13_lock + 0x04) != 2 {
                        let mut local_30 = [0u8; 0x18];
                        let l30p = local_30.as_mut_ptr() as usize;
                        lock_spin_byte(l30p + 0x14);
                        let _ = crate::simd_ops::atomic_xchg_u64((l30p + 0x08) as u64, 0);
                        let _ = crate::simd_ops::atomic_xchg_u32((l30p + 0x10) as u64, 0);
                        wr::<u8>(l30p + 0x14, 0);

                        let key_rsi = rd::<u64>(rsi);
                        wr::<u64>(l30p, key_rsi);

                        lock_spin_byte(r13_lock);
                        let ptr_val = rd::<u64>(r13_lock - 0x0c);
                        let _ = crate::simd_ops::atomic_xchg_u64((l30p + 0x08) as u64, ptr_val);
                        let ver_val = rd::<u32>(r13_lock - 0x04);
                        let _ = crate::simd_ops::atomic_xchg_u32((l30p + 0x10) as u64, ver_val);
                        crate::simd_ops::wu8((r13_lock) as u64, 0); // spinlock release (x86 mov = release)

                        let resolved_obj: usize = if rd::<u64>(l30p) == 0 {
                            0
                        } else {
                            let cur_ver = rd::<u32>(l30p + 0x10);
                            let g_ver_addr = exe::ptr::<u32>(0x1_4384_1b24) as usize;
                            let g_ver = rd::<u32>(g_ver_addr);
                            if cur_ver != g_ver {
                                lock_spin_byte(l30p + 0x14);
                                let mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8300) as usize);
                                let g_ver2 = rd::<u32>(g_ver_addr);
                                let looked_up =
                                    exe::call_va!(0x1_4248_b000, (mgr, l30p) -> u64);
                                let _ = crate::simd_ops::atomic_xchg_u64(
                                    (l30p + 0x08) as u64,
                                    looked_up,
                                );
                                let _ = crate::simd_ops::atomic_xchg_u32(
                                    (l30p + 0x10) as u64,
                                    g_ver2,
                                );
                                wr::<u8>(l30p + 0x14, 0);
                            }
                            rd::<usize>(l30p + 0x08)
                        };

                        let mut rbx_sub = rd::<usize>(resolved_obj + 0x10);
                        let rdi_sub_end =
                            rbx_sub.wrapping_add((rd::<u32>(resolved_obj + 0x18) as usize) << 5);
                        let mut all_ready = true;
                        if rbx_sub != rdi_sub_end {
                            let mut r12_sub_lock = rbx_sub + 0x14;
                            loop {
                                let mut local_48 = [0u8; 0x28];
                                let l48p = local_48.as_mut_ptr() as usize;
                                exe::call_va!(0x1_401e_8620, (l48p));
                                wr::<u64>(l48p, 0x100);
                                wr::<u16>(l48p + 0x18, 0);
                                wr::<u64>(l48p + 0x1c, 0);
                                wr::<u32>(l48p + 0x24, 0);

                                lock_spin_byte(l48p + 0x14);
                                let _ = crate::simd_ops::atomic_xchg_u64((l48p + 0x08) as u64, 0);
                                let _ = crate::simd_ops::atomic_xchg_u32((l48p + 0x10) as u64, 0);
                                wr::<u8>(l48p + 0x14, 0);

                                let sub_key = rd::<u64>(rbx_sub);
                                wr::<u64>(l48p, sub_key);

                                lock_spin_byte(r12_sub_lock);
                                let sp = rd::<u64>(r12_sub_lock - 0x0c);
                                let _ = crate::simd_ops::atomic_xchg_u64((l48p + 0x08) as u64, sp);
                                let sv = rd::<u32>(r12_sub_lock - 0x04);
                                let _ = crate::simd_ops::atomic_xchg_u32((l48p + 0x10) as u64, sv);
                                crate::simd_ops::wu8((r12_sub_lock) as u64, 0); // spinlock release (x86 mov = release)

                                let r10_cnt = rd::<u32>(entry_70 + 0x10) as u64;
                                let rcx_base = rd::<usize>(entry_70 + 0x08);
                                if r10_cnt != 0 {
                                    let mut r8d: i32 = 0;
                                    let mut rdx_idx: u64 = 0;
                                    let mut rcx_cur = rcx_base;
                                    loop {
                                        if rd::<u64>(rcx_cur) == sub_key {
                                            if rdx_idx != u64::MAX {
                                                let st = rd::<u32>(
                                                    rcx_base
                                                        .wrapping_add((rdx_idx as usize) * 0x28 + 0x1c),
                                                );
                                                if st != 2 {
                                                    all_ready = false;
                                                }
                                            }
                                            break;
                                        }
                                        r8d = r8d.wrapping_add(1);
                                        rdx_idx = rdx_idx.wrapping_add(1);
                                        rcx_cur = rcx_cur.wrapping_add(0x28);
                                        if ((r8d as i64) as u64) >= r10_cnt {
                                            break;
                                        }
                                    }
                                }

                                let _ = crate::simd_ops::atomic_xchg_u64((l48p + 0x08) as u64, 0);
                                if !all_ready {
                                    break;
                                }
                                rbx_sub = rbx_sub.wrapping_add(0x20);
                                r12_sub_lock = r12_sub_lock.wrapping_add(0x20);
                                if rbx_sub == rdi_sub_end {
                                    break;
                                }
                            }
                        }

                        if all_ready {
                            wr::<u32>(r13_lock + 0x04, 2);
                        }
                        let _ = crate::simd_ops::atomic_xchg_u64((l30p + 0x08) as u64, 0);
                    }

                    rsi = rsi.wrapping_add(0x20);
                    r13_lock = r13_lock.wrapping_add(0x20);
                    if rsi == r14_end {
                        break;
                    }
                }
            }

            rdi = rdi.wrapping_add(0x18);
            if rdi != rbx_end {
                rdi = skip_map_buckets(
                    rdi,
                    rbx_end,
                    rd::<u64>(param_1 + 0xc8),
                    rd::<u64>(param_1 + 0xb0),
                    rd::<u64>(param_1 + 0xa8),
                );
            }
            if rdi == r13_end {
                break;
            }
        }
    }

    let rbx_start = rd::<usize>(param_1 + 0x68);
    let rdi_end = rbx_start.wrapping_add((rd::<u64>(param_1 + 0x48) as usize).wrapping_mul(0x18));
    let mut rbx_it = if rbx_start != rdi_end {
        skip_map_buckets(
            rbx_start,
            rdi_end,
            rd::<u64>(param_1 + 0x50),
            rd::<u64>(param_1 + 0x38),
            rd::<u64>(param_1 + 0x30),
        )
    } else {
        rbx_start
    };

    let rsi_end =
        rd::<usize>(param_1 + 0x68).wrapping_add((rd::<u64>(param_1 + 0x48) as usize).wrapping_mul(0x18));
    while rbx_it != rsi_end {
        exe::call_va!(
            0x1_41f5_9270,
            (param_1, rbx_it + 0x08, param_3, param_2, param_3, param_4)
        );
        rbx_it = rbx_it.wrapping_add(0x18);
        if rbx_it != rdi_end {
            rbx_it = skip_map_buckets(
                rbx_it,
                rdi_end,
                rd::<u64>(param_1 + 0x50),
                rd::<u64>(param_1 + 0x38),
                rd::<u64>(param_1 + 0x30),
            );
        }
    }
    wr::<u32>(param_1, 0);
}

/// `unknown_141f5b080`
/// Original: 0x141f5b080 (501 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f5b080(
    param_1: usize,
    param_2: u8,
    param_3: usize,
    param_4: usize,
) -> u8 {
    let r13 = exe::call_va!(0x1_4021_7df0, (param_1) -> usize);
    let init_cap = rd::<u32>(r13 + 0x08) as u64;
    let ret: u8 = if init_cap == 0 {
        0
    } else {
        let mut cap: u64 = init_cap;
        let mut dst = exe::call_va!(0x1_4241_c0a0, (cap * 8, 8u32) -> usize);
        let mut count: u32 = 0;
        let mut i: u32 = 0;
        let mut arr_ptr = rd::<usize>(r13);
        if rd::<u32>(r13 + 0x08) > 0 {
            loop {
                let elem = rd::<u64>(arr_ptr.wrapping_add((i as usize) << 5));
                let valid = (elem & 0x3fff) != 0 && (elem & 0x3fff_ffff_ffff_c000) != 0;
                if valid {
                    let cur_cnt = count as u64;
                    let cur_cap = (cap as u32) as u64;
                    if cur_cnt * 8 == cur_cap * 8 {
                        let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                        let min_cap = cur_cnt.wrapping_add(1);
                        if new_cap < min_cap {
                            new_cap = min_cap;
                        }
                        cap = new_cap;
                        let new_dst = exe::call_va!(0x1_4241_c0a0, (cap * 8, 8u32) -> usize);
                        exe::call_va!(0x1_4183_acc8, (new_dst, dst, cur_cnt * 8));
                        if dst != 0 {
                            exe::call_va!(0x1_4241_c190, (dst));
                        }
                        dst = new_dst;
                    }
                    wr::<u64>(dst.wrapping_add((count as usize) * 8), elem);
                    count = count.wrapping_add(1);

                    let p3_0 = rd::<usize>(param_3);
                    let g_word = rd::<u16>(exe::ptr::<u16>(0x1_43cc_aec8) as usize) as usize;
                    let tab_20 = rd::<usize>(p3_0 + 0x20);
                    let slot = rd::<u16>(tab_20.wrapping_add(g_word * 2)) as usize;
                    let p3_8 = rd::<usize>(param_3 + 0x08);
                    let v48 = rd::<u32>(p3_8.wrapping_add(slot * 0x10 + 0x08)) as u64;
                    let v50 = rd::<u64>(p3_8.wrapping_add(slot * 0x10));
                    let mut local_40: [u64; 3] = [param_3 as u64, v48, v50];
                    exe::call_va!(
                        0x1_41f5_b370,
                        (elem, param_2, param_4, local_40.as_mut_ptr() as usize)
                    );
                }
                i = i.wrapping_add(1);
                let len_now = rd::<u32>(r13 + 0x08) as u64;
                arr_ptr = rd::<usize>(r13);
                if ((i as i32 as i64) as u64) >= len_now {
                    break;
                }
            }
        }
        let mut slice_30: [u64; 2] = [count as u64, dst as u64];
        exe::call_va!(0x1_41e5_c310, (param_3, slice_30.as_mut_ptr() as usize));
        if dst != 0 {
            exe::call_va!(0x1_4241_c190, (dst));
        }
        1
    };
    let _ = crate::simd_ops::atomic_xchg_u64((param_1 + 0x08) as u64, 0);
    ret
}

/// `unknown_141f5b370`
/// Original: 0x141f5b370 (243 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f5b370(
    param_1: u64,
    param_2: u8,
    param_3: usize,
    param_4: usize,
) {
    let mut key_slot: u64 = param_1;
    let end_node = ((rd::<u64>(param_3 + 0x40) as usize) << 4).wrapping_add(rd::<usize>(param_3 + 0x58));
    let r11_node = if rd::<u64>(param_3 + 0x38) == rd::<u64>(param_3 + 0x30) {
        end_node
    } else {
        let mut find_20 = [0u64; 2];
        exe::call_va!(
            0x1_401f_a730,
            (
                param_3,
                find_20.as_mut_ptr() as usize,
                (&mut key_slot as *mut u64) as usize
            )
        );
        if find_20[0] == u64::MAX {
            return;
        }
        ((find_20[0] as usize) << 4).wrapping_add(rd::<usize>(param_3 + 0x58))
    };

    let end_node_check =
        ((rd::<u64>(param_3 + 0x40) as usize) << 4).wrapping_add(rd::<usize>(param_3 + 0x58));
    if r11_node == end_node_check {
        return;
    }
    let r11_val = rd::<u64>(r11_node + 0x08);
    if (r11_val as u32) == u32::MAX {
        return;
    }

    let r9_obj = rd::<usize>(param_4);
    let rcx_idx = (r11_val as u32) as usize;
    let rax_tab = rd::<usize>(r9_obj + 0x30);
    let rdx_entry = rd::<u64>(rax_tab.wrapping_add(rcx_idx * 8));
    if (rdx_entry as u16) == 0xffff {
        exe::call_iat!(0x1_4264_67e8, ());
        return;
    }

    let r8_word = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d87c) as usize) as u32;
    let off = ((rdx_entry as u16) as usize) << 5;
    let edx_len = rd::<u32>(r9_obj + off + 0x180a8);
    let rcx_ptr = rd::<usize>(r9_obj + off + 0x18090);
    let found = exe::call_va!(0x1_4248_9ee0, (rcx_ptr, edx_len, r8_word) -> i32);
    if found != -1 {
        let flag_ptr = exe::call_va!(0x1_4035_b4a0, (param_4, r11_val) -> usize);
        let cur = rd::<u32>(flag_ptr);
        let next = if param_2 != 0 { cur | 2 } else { cur & !2u32 };
        wr::<u32>(flag_ptr, next);
    }
}

/// `coregame_141f732a0`
/// Original: 0x141f732a0 (168 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141f732a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
) {
    if param_4 < 0x10 {
        wr::<u16>(param_1, 4);
        exe::call_va!(0x1_4183_acbc, (param_1 + 0x08, param_3, param_4));
        exe::call_va!(
            0x1_4183_acc2,
            (
                param_1 + 0x08 + (param_4 as usize),
                0u32,
                0x10u64.wrapping_sub(param_4)
            )
        );
    } else {
        wr::<u16>(param_1, 5);
        let idx = exe::call_va!(0x1_41f7_2a00, (param_2, param_3, param_4) -> u32);
        wr::<u64>(param_1 + 0x08, idx as u64);
    }
    let edx = rd::<u32>(param_1 + 0x04) as u64;
    let shifted = edx >> 5;
    wr::<u8>(param_1 + 0x03, 0xff);
    let bit = shifted & 0x3f;
    let word_idx = (shifted >> 6) as usize;
    let slot_addr = param_2.wrapping_add(word_idx * 8 + 0x118);
    wr::<u64>(slot_addr, rd::<u64>(slot_addr) | (1u64 << bit));
}

/// `unknown_141fcfce0`
/// Original: 0x141fcfce0 (572 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141fcfce0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: u32,
    param_8: u8,
    param_9: u64,
    param_10: usize,
) {
    use crate::simd_ops::{vaddps, vblendps, vblendvps, vcmpss, vmulps, vpermilps, vshufps, vsubps};

    let xmm1_in = rd::<u128>(param_4);
    let xmm5_c = rd::<u128>(exe::ptr::<u128>(0x1_4342_83b0) as usize);
    let xmm0 = vmulps(xmm5_c, xmm1_in);
    let xmm4 = vmulps(xmm0, xmm1_in);
    let xmm0_e0 = vpermilps(xmm1_in, 0xe0);
    let xmm2_m = vmulps(xmm0_e0, xmm5_c);
    let xmm1_d9 = vpermilps(xmm1_in, 0xd9);
    let xmm7 = vmulps(xmm2_m, xmm1_d9);
    let xmm1_m = vmulps(xmm5_c, xmm1_in);
    let xmm0_ff = vpermilps(xmm1_in, 0xff);
    let xmm5_m = vmulps(xmm1_m, xmm0_ff);
    let xmm8_c = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);
    let xmm0_c1 = vpermilps(xmm4, 0xc1);
    let xmm2_s = vsubps(xmm8_c, xmm0_c1);

    let saved_74 = rd::<u32>(param_5 + 0x74);
    let saved_7c = rd::<u32>(param_5 + 0x7c);

    let xmm1_da = vpermilps(xmm4, 0xda);
    let xmm6 = vsubps(xmm2_s, xmm1_da);
    let xmm3_d2 = vpermilps(xmm5_m, 0xd2);
    let xmm1_c9 = vpermilps(xmm5_m, 0xc9);
    let xmm0_d8 = vpermilps(xmm7, 0xd8);
    let xmm4_a = vaddps(xmm3_d2, xmm0_d8);
    let xmm5_44 = vshufps(xmm6, xmm4_a, 0x44);
    let xmm0_e1 = vpermilps(xmm7, 0xe1);
    let xmm3_s = vsubps(xmm0_e1, xmm1_c9);
    let xmm1_ee = vshufps(xmm3_s, 0, 0xee);
    let xmm0_44 = vshufps(xmm3_s, 0, 0x44);
    let xmm2_88 = vshufps(xmm5_44, xmm0_44, 0x88);
    let xmm0_dd = vshufps(xmm5_44, xmm0_44, 0xdd);
    let xmm5_d2 = vpermilps(xmm0_dd, 0xd2);
    let xmm4_ee = vshufps(xmm6, xmm4_a, 0xee);
    let xmm0_88 = vshufps(xmm4_ee, xmm1_ee, 0x88);
    let xmm1_bl = vblendps(xmm8_c, rd::<u128>(param_4 + 0x10), 0x7);
    let xmm3_c9 = vpermilps(xmm0_88, 0xc9);
    let xmm4_out = vshufps(xmm5_d2, xmm3_c9, 0x49);
    let xmm0_4e = vshufps(xmm2_88, xmm5_d2, 0x4e);
    let xmm2_out = vshufps(xmm2_88, xmm0_4e, 0x84);
    let xmm0_4e2 = vshufps(xmm3_c9, xmm1_bl, 0x4e);
    let xmm5_out = vshufps(xmm0_4e2, xmm1_bl, 0x98);

    let mut local_40: [u128; 3] = [xmm2_out, xmm4_out, xmm5_out];
    let xmm0_call = ((xmm5_out >> 32) as u64) as u128;
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4201_42e0,
        param_1 as u64,
        param_2 as u64,
        local_40.as_mut_ptr() as usize as u64,
        param_4 as u64,
        param_5 as u64,
        param_8 as u64,
        (param_9 as u8) as u64,
        0,
        [xmm0_call, xmm1_bl, xmm2_out, param_7 as u128],
    );

    if rd::<u8>(param_6 + 0x18) != 0 {
        let f_190 = rd::<u32>(param_5 + 0x190);
        let f_18c = rd::<u32>(param_5 + 0x18c);
        let g_428 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8428) as usize);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_424c_ea10,
            g_428 as u64,
            param_6 as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, f_18c as u128, f_190 as u128],
        );
    }

    let g_428 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8428) as usize);
    exe::call_va!(0x1_424c_e980, (g_428, param_3, param_6));

    if rd::<u8>(param_10 + 0x10) != 0 {
        let c_neg1 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7080) as usize);
        let x1 = rd::<u32>(param_10);
        let m0 = vcmpss(0, x1, c_neg1, 0);
        let r0 = vblendvps(x1 as u128, saved_74 as u128, m0) as u32;
        let x2 = rd::<u32>(param_10 + 0x04);
        wr::<u32>(param_5 + 0x74, r0);
        let m1 = vcmpss(0, x2, c_neg1, 0);
        let r1 = vblendvps(x2 as u128, saved_7c as u128, m1) as u32;
        wr::<u32>(param_5 + 0x7c, r1);
    }
}

#[inline(always)]
unsafe fn skip_empty_ctrl(mut ctrl: usize, mut elem: usize, stride: usize) -> (usize, usize) {
    if rd::<i8>(ctrl) < -1 {
        let all_ff = u128::MAX;
        loop {
            let chunk = rd::<u128>(ctrl);
            let cmp = crate::simd_ops::vpcmpgtb(all_ff, chunk);
            let mask = (crate::simd_ops::vpmovmskb(cmp) as u32).wrapping_add(1);
            let step = if mask != 0 {
                mask.trailing_zeros() as usize
            } else {
                0
            };
            ctrl = ctrl.wrapping_add(step);
            elem = elem.wrapping_add(step.wrapping_mul(stride));
            if rd::<i8>(ctrl) >= -1 {
                break;
            }
        }
    }
    (ctrl, elem)
}

/// `unknown_14202ea90`
/// Original: 0x14202ea90 (715 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14202ea90(
    param_1: usize,
    _param_2_xmm1: usize,
    param_3: usize,
) {
    let [_xmm0, xmm1, _xmm2, _xmm3] = crate::simd_ops::take_xmm_args();
    let dt = f32::from_bits(xmm1 as u32);

    let r9_ctrl = rd::<usize>(param_1);
    let rdi_base = rd::<usize>(param_1 + 0x08);
    let (mut rbp, mut rdi) = skip_empty_ctrl(r9_ctrl, rdi_base, 0x48);
    let r12_end = r9_ctrl.wrapping_add(rd::<usize>(param_1 + 0x18));
    if rbp == r12_end {
        return;
    }

    loop {
        let rdx_ictrl = rd::<usize>(rdi + 0x18);
        let rsi_ielem = rd::<usize>(rdi + 0x20);
        let (mut rbx, mut rsi) = skip_empty_ctrl(rdx_ictrl, rsi_ielem, 8);
        let ictrl_end = rdx_ictrl.wrapping_add(rd::<usize>(rdi + 0x30));
        if rbx != ictrl_end {
            loop {
                let rem_time = rd::<f32>(rsi + 0x04) - dt;
                wr::<f32>(rsi + 0x04, rem_time);
                if !(0.0f32 >= rem_time) {
                    let (nb, ns) = skip_empty_ctrl(rbx + 1, rsi + 8, 8);
                    rbx = nb;
                    rsi = ns;
                } else {
                    let mut rcx_v = rd::<usize>(rdi + 0x08);
                    let mut edx_len = rd::<u32>(rdi + 0x10);
                    if rcx_v != rcx_v.wrapping_add((edx_len as usize) * 4) {
                        let mut r8_base = rcx_v;
                        loop {
                            let target = rd::<u32>(rsi);
                            if rd::<u32>(rcx_v) == target {
                                let last_ptr =
                                    r8_base.wrapping_add(((edx_len as usize).wrapping_sub(1)) * 4);
                                if rcx_v != last_ptr {
                                    wr::<u32>(rcx_v, rd::<u32>(last_ptr));
                                }
                                edx_len = rd::<u32>(rdi + 0x10).wrapping_sub(1);
                                wr::<u32>(rdi + 0x10, edx_len);
                            } else {
                                rcx_v = rcx_v.wrapping_add(4);
                            }
                            r8_base = rd::<usize>(rdi + 0x08);
                            if rcx_v == r8_base.wrapping_add((edx_len as usize) * 4) {
                                break;
                            }
                        }
                    }
                    let mut old_iter: [usize; 2] = [rbx, rsi];
                    let (nb, ns) = skip_empty_ctrl(rbx + 1, rsi + 8, 8);
                    rbx = nb;
                    rsi = ns;
                    exe::call_va!(0x1_411b_c750, (rdi + 0x18, old_iter.as_mut_ptr() as usize));
                }
                let cur_end = rd::<usize>(rdi + 0x18).wrapping_add(rd::<usize>(rdi + 0x30));
                if rbx == cur_end {
                    break;
                }
            }
        }

        let mut rbx_v = rd::<usize>(rdi + 0x08);
        let init_len = rd::<u32>(rdi + 0x10) as usize;
        if rbx_v != rbx_v.wrapping_add(init_len * 4) {
            loop {
                let fn_ptr = rd::<u64>(param_3 + 0x08);
                let edx_id = rd::<u32>(rbx_v);
                let rcx_ctx = rd::<usize>(param_3);
                let keep = exe::call_va!(fn_ptr, (rcx_ctx, edx_id) -> u8);
                if keep == 0 {
                    let cur_len = rd::<u32>(rdi + 0x10) as usize;
                    let base_v = rd::<usize>(rdi + 0x08);
                    let last_ptr = base_v.wrapping_add(cur_len * 4).wrapping_sub(4);
                    if rbx_v != last_ptr {
                        wr::<u32>(rbx_v, rd::<u32>(last_ptr));
                    }
                    wr::<u32>(rdi + 0x10, (cur_len as u32).wrapping_sub(1));
                } else {
                    rbx_v = rbx_v.wrapping_add(4);
                }
                let new_len = rd::<u32>(rdi + 0x10) as usize;
                let new_base = rd::<usize>(rdi + 0x08);
                if rbx_v == new_base.wrapping_add(new_len * 4) {
                    break;
                }
            }
        }

        let (np, nd) = skip_empty_ctrl(rbp + 1, rdi + 0x48, 0x48);
        rbp = np;
        rdi = nd;
        if rbp == r12_end {
            break;
        }
    }
}

#[inline(always)]
unsafe fn ensure_static_hash_init(
    tls_0: usize,
    guard_va: u64,
    val_va: u64,
    str_va: u64,
    str_len: u64,
) {
    let epoch = rd::<i32>(tls_0 + 0x74);
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > epoch {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let mut view: [u64; 2] = [exe::ptr::<u8>(str_va) as usize as u64, str_len];
            let val_ptr = exe::ptr::<u32>(val_va) as usize;
            exe::call_va!(0x1_4220_ea70, (val_ptr, view.as_mut_ptr() as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

/// `coreshared_14204a3b0`
/// Original: 0x14204a3b0 (1220 bytes, Throws::Caught)
pub unsafe extern "win64" fn coreshared_14204a3b0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let tls_arr = exe::tls_pointer();
    let tls_0 = rd::<usize>(tls_arr);

    ensure_static_hash_init(tls_0, 0x1_43ce_6870, 0x1_43ce_6868, 0x1_433d_b0f8, 0x18);
    ensure_static_hash_init(tls_0, 0x1_43ce_6854, 0x1_43ce_686c, 0x1_433d_b118, 0x19);
    ensure_static_hash_init(tls_0, 0x1_43ce_6860, 0x1_43ce_685c, 0x1_433d_b138, 0x11);
    ensure_static_hash_init(tls_0, 0x1_43ce_6864, 0x1_43ce_6858, 0x1_433d_b150, 0x15);

    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let mut xmm9 = c_one;
    let mut xmm8 = c_one;
    let mut xmm7 = c_one;
    let mut xmm6 = c_one;

    let rax_60 = rd::<usize>(param_2 + 0x60);
    let rcx_c0 = rd::<usize>(rax_60 + 0xc0);
    if rcx_c0 != 0 {
        let map_ptr = rd::<usize>(rcx_c0);
        let keys_ptr = rd::<usize>(map_ptr);
        let count = rd::<u32>(map_ptr + 0x08) as usize;
        let vals_ptr = rd::<usize>(map_ptr + 0x10);
        if count != 0 {
            let k_diff = rd::<u32>(exe::ptr::<u32>(0x1_43ce_6868) as usize);
            for i in 0..count {
                if rd::<u32>(keys_ptr + i * 4) == k_diff {
                    let entry = vals_ptr.wrapping_add(i * 0x18);
                    if entry != 0 {
                        xmm9 = rd::<f32>(entry + 0x08);
                    }
                    break;
                }
            }
            let k_spec = rd::<u32>(exe::ptr::<u32>(0x1_43ce_686c) as usize);
            for i in 0..count {
                if rd::<u32>(keys_ptr + i * 4) == k_spec {
                    let entry = vals_ptr.wrapping_add(i * 0x18);
                    if entry != 0 {
                        xmm8 = rd::<f32>(entry + 0x08);
                    }
                    break;
                }
            }
            let k_norm = rd::<u32>(exe::ptr::<u32>(0x1_43ce_685c) as usize);
            for i in 0..count {
                if rd::<u32>(keys_ptr + i * 4) == k_norm {
                    let entry = vals_ptr.wrapping_add(i * 0x18);
                    if entry != 0 {
                        xmm7 = rd::<f32>(entry + 0x08);
                    }
                    break;
                }
            }
            let k_smoo = rd::<u32>(exe::ptr::<u32>(0x1_43ce_6858) as usize);
            for i in 0..count {
                if rd::<u32>(keys_ptr + i * 4) == k_smoo {
                    let entry = vals_ptr.wrapping_add(i * 0x18);
                    if entry != 0 {
                        xmm6 = rd::<f32>(entry + 0x08);
                    }
                    break;
                }
            }
        }
    }

    wr::<u64>(param_1 + 0x48, 0);
    wr::<u64>(param_1 + 0x50, 0);
    wr::<u64>(param_1 + 0x58, 0);
    wr::<u32>(param_1 + 0x80, 0);

    wr::<u128>(param_1, rd::<u128>(param_3));
    wr::<u128>(param_1 + 0x10, rd::<u128>(param_3 + 0x10));
    wr::<u128>(param_1 + 0x20, rd::<u128>(param_3 + 0x20));
    wr::<u128>(param_1 + 0x30, rd::<u128>(param_3 + 0x30));
    wr::<u32>(param_1 + 0x40, rd::<u32>(param_2 + 0x48));

    let rsi_res = rd::<usize>(param_2 + 0x60);
    if rsi_res != 0 {
        let prev = crate::simd_ops::atomic_xadd_u32((rsi_res + 0x24) as u64, 1);
        if prev.wrapping_add(1) == 1 {
            let cb = rd::<usize>(rsi_res + 0x28);
            if cb != 0 {
                exe::call_va!(0x1_4241_d6f0, (cb, rsi_res));
            }
        }
    }
    let old_res = rd::<usize>(param_1 + 0x48);
    if old_res != 0 {
        let prev = crate::simd_ops::atomic_xadd_u32((old_res + 0x24) as u64, u32::MAX);
        if prev == 1 {
            let cb = rd::<usize>(old_res + 0x28);
            if cb != 0 {
                exe::call_va!(0x1_4241_d6f0, (cb, old_res));
            }
        }
    }
    wr::<usize>(param_1 + 0x48, rsi_res);

    wr::<u64>(param_1 + 0x50, rd::<u64>(param_2 + 0x34));
    wr::<u64>(param_1 + 0x58, rd::<u64>(param_2 + 0x3c));
    wr::<u32>(param_1 + 0x60, rd::<u32>(param_2 + 0x44));
    wr::<u32>(param_1 + 0x64, rd::<u32>(param_2 + 0x5c));
    wr::<u32>(param_1 + 0x68, rd::<u32>(param_2));
    wr::<f32>(param_1 + 0x6c, xmm9 * rd::<f32>(param_2 + 0x04));
    wr::<f32>(param_1 + 0x70, xmm8 * rd::<f32>(param_2 + 0x08));
    wr::<f32>(param_1 + 0x74, xmm7 * rd::<f32>(param_2 + 0x0c));
    wr::<f32>(param_1 + 0x78, xmm6 * rd::<f32>(param_2 + 0x10));
    wr::<u32>(param_1 + 0x7c, rd::<u32>(param_2 + 0x14));
    if rd::<u8>(param_2 + 0x58) != 0 {
        wr::<u32>(param_1 + 0x80, rd::<u32>(param_1 + 0x80) | 1);
    }
    param_1
}

/// `unknown_142058260`
/// Original: 0x142058260 (726 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142058260(
    param_1: u16,
    param_2: u16,
    _param_3: u64,
    param_4: u32,
    param_5: u8,
) {
    let g_9174_addr = exe::ptr::<u16>(0x1_43a2_9174) as usize;
    let g_91bc_addr = exe::ptr::<u16>(0x1_43a2_91bc) as usize;
    let g_b03c_addr = exe::ptr::<u32>(0x1_43a2_b03c) as usize;
    let g_8c3c_addr = exe::ptr::<u8>(0x1_43a2_8c3c) as usize;
    let g_5a98_addr = exe::ptr::<usize>(0x1_43a3_5a98) as usize;

    let mut rbx = rd::<usize>(g_5a98_addr);
    if param_1 != rd::<u16>(g_9174_addr)
        || param_2 != rd::<u16>(g_91bc_addr)
        || param_4 != rd::<u32>(g_b03c_addr)
        || param_5 != rd::<u8>(g_8c3c_addr)
        || rbx == 0
    {
        exe::call_va!(0x1_4205_8180, ());

        let mut cfg_28 = [0u8; 0x18];
        let c28p = cfg_28.as_mut_ptr() as usize;
        wr::<u32>(c28p, 1);
        wr::<u32>(c28p + 0x04, (param_5 as u32) * 2 + 6);
        wr::<u16>(c28p + 0x08, param_1);
        wr::<u16>(c28p + 0x0a, param_2);
        wr::<u32>(c28p + 0x0c, 2);
        wr::<u32>(c28p + 0x10, 0x10);
        wr::<u16>(c28p + 0x14, param_1);
        wr::<u16>(c28p + 0x16, param_2);

        let mut desc_40 = [0u8; 0x30];
        let d40p = desc_40.as_mut_ptr() as usize;
        wr::<usize>(d40p + 0x20, c28p);
        wr::<u64>(d40p + 0x28, 2);

        let mut local_20: usize = 0;
        let mut zero_byte: u8 = 0;
        let mut param_4_slot: u32 = param_4;
        let l20p = (&mut local_20 as *mut usize) as usize;
        let rax_ptr = exe::call_va!(
            0x1_411d_5fd0,
            (
                l20p,
                param_2 as u64,
                (&mut param_4_slot as *mut u32) as usize,
                (&mut zero_byte as *mut u8) as usize
            ) -> usize
        );
        let new_ctx = rd::<usize>(rax_ptr);
        wr::<usize>(rax_ptr, 0);
        exe::call_va!(0x1_411d_5f40, (rax_ptr, new_ctx));

        let leftover = rd::<usize>(l20p);
        if leftover != 0 {
            for off in [0xd0usize, 0xb8, 0xa0, 0x88, 0x70, 0x58] {
                exe::call_va!(0x1_4020_2a90, (leftover + off));
            }
            exe::call_va!(0x1_411d_5cd0, (leftover + 0x40));
            exe::call_va!(0x1_411d_5d50, (leftover + 0x28));
            exe::call_va!(0x1_411d_5e80, (leftover + 0x18));
            exe::call_va!(0x1_411d_5ee0, (leftover));
            exe::call_va!(0x1_4244_2de0, (leftover));
        }

        let rbx_ctx = rd::<usize>(g_5a98_addr);
        let r15_dev = rd::<usize>(exe::ptr::<usize>(0x1_43a3_59f8) as usize);
        let fn_48ad0 = rd::<u64>(exe::ptr::<u64>(0x1_43c4_8ad0) as usize);
        let info1 = exe::call_va!(fn_48ad0, (r15_dev) -> usize);
        if rd::<u16>(info1 + 0xa2) == 1 && rd::<u16>(info1 + 0xa4) == 0x63 {
            let info2 = exe::call_iat!(0x1_4264_5c70, () -> usize);
            if rd::<u8>(info2 + 0x1c) == 4 && rd::<u8>(info2 + 0x1d) == 3 {
                let status = exe::call_iat!(0x1_4264_5c88, (d40p, rbx_ctx + 0x110) -> i32);
                if status == 0 {
                    wr::<usize>(rbx_ctx + 0xf8, r15_dev);
                    wr::<usize>(
                        rbx_ctx + 0xe8,
                        exe::ptr::<u8>(0x1_43c4_8ad0) as usize,
                    );
                    wr::<usize>(
                        rbx_ctx + 0xf0,
                        exe::ptr::<u8>(0x1_43c4_8e20) as usize,
                    );
                    exe::call_va!(0x1_4205_9ea0, (rbx_ctx));
                    exe::call_va!(0x1_4205_97a0, (rbx_ctx));
                }
            }
        }

        wr::<u16>(g_9174_addr, param_1);
        wr::<u16>(g_91bc_addr, param_2);
        wr::<u32>(g_b03c_addr, param_4);
        wr::<u8>(g_8c3c_addr, param_5);
        rbx = rd::<usize>(g_5a98_addr);
    }

    let rem = rd::<u32>(rbx + 0x148) % rd::<u32>(rbx + 0x140);
    wr::<u32>(rbx + 0x144, rem);
    let vtbl_e8 = rd::<usize>(rbx + 0xe8);
    let arr_b8 = rd::<usize>(rbx + 0xb8);
    let elem_b8 = rd::<usize>(arr_b8.wrapping_add((rem as usize) * 8));
    let fn_290 = rd::<u64>(vtbl_e8 + 0x290);
    exe::call_va!(fn_290, (elem_b8));

    let cur_144 = rd::<u32>(rbx + 0x144) as usize;
    let arr_d0 = rd::<usize>(rbx + 0xd0);
    wr::<u64>(arr_d0.wrapping_add(cur_144 * 8), 0);

    if rd::<u8>(rbx + 0x14d) == 0 {
        let cur_idx = rd::<u32>(rbx + 0x144) as usize;
        let vec_base = rd::<usize>(rbx + 0x28).wrapping_add(cur_idx * 0x18);
        let rsi_end = rd::<usize>(vec_base + 0x08);
        let mut rdi_cur = rd::<usize>(vec_base);
        while rdi_cur != rsi_end {
            let vtbl = rd::<usize>(rbx + 0xe8);
            let fn_d8 = rd::<u64>(vtbl + 0xd8);
            let h = rd::<usize>(rdi_cur);
            exe::call_va!(fn_d8, (h));
            rdi_cur = rdi_cur.wrapping_add(8);
        }
        let cur_idx2 = rd::<u32>(rbx + 0x144) as usize;
        let vec_base2 = rd::<usize>(rbx + 0x28).wrapping_add(cur_idx2 * 0x18);
        let begin = rd::<usize>(vec_base2);
        if begin != rd::<usize>(vec_base2 + 0x08) {
            wr::<usize>(vec_base2 + 0x08, begin);
        }
    }

    wr::<u32>(rbx + 0x148, rd::<u32>(rbx + 0x148).wrapping_add(1));
}

#[inline(always)]
unsafe fn ring_push(q: usize, item: u64) {
    let cnt = rd::<u32>(q + 0x10);
    if cnt == rd::<u32>(q + 0x14) {
        exe::call_va!(0x1_411d_aad0, (q, cnt.wrapping_add(1)));
    }
    let tail = rd::<u32>(q + 0x0c) as usize;
    let buf = rd::<usize>(q);
    wr::<u64>(buf.wrapping_add(tail * 8), item);
    let next_tail = rd::<u32>(q + 0x0c).wrapping_add(1);
    wr::<u32>(q + 0x0c, next_tail);
    if next_tail == rd::<u32>(q + 0x14) {
        wr::<u32>(q + 0x0c, 0);
    }
    wr::<u32>(q + 0x10, rd::<u32>(q + 0x10).wrapping_add(1));
}

/// `d3d_142063760`
/// Original: 0x142063760 (839 bytes, Throws::Caught)
pub unsafe extern "win64" fn d3d_142063760(param_1: usize, param_2: i32) {
    let rdi_lock = param_1 + 0x08;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rdi_lock + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (rdi_lock));
        wr::<u32>(rdi_lock + 0x08, tid);
    }
    wr::<u32>(rdi_lock + 0x0c, rd::<u32>(rdi_lock + 0x0c).wrapping_add(1));

    let rsi_idx = param_2 as isize;
    let q3 = (param_1 as isize + (rsi_idx + 3) * 0x18) as usize;
    let active_slot = (param_1 as isize + rsi_idx * 8 + 0xa8) as usize;
    ring_push(q3, rd::<u64>(active_slot));

    let g_fc58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let rcx_38 = rd::<u64>(g_fc58 + 0x38);
    let mut r15_40 = rd::<u64>(g_fc58 + 0x40);
    let mut local_50: [u64; 2] = [r15_40, rcx_38];

    let rbp = (param_1 as isize + rsi_idx * 0x18) as usize;
    let q5 = (param_1 as isize + (rsi_idx + 5) * 0x18) as usize;
    if crate::simd_ops::rd_acq_u32(rbp + 0x58) != 0 {
        loop {
            let buf = rd::<usize>(q3);
            let cap = rd::<u32>(q3 + 0x14) as u64;
            let head = rd::<u32>(q3 + 0x08) as u64;
            let rem = (head % cap) as usize;
            let item = rd::<usize>(buf.wrapping_add(rem * 8));
            let cnt = rd::<u32>(q3 + 0x10);
            if cnt != 0 {
                let next_head = (head as u32).wrapping_add(1);
                wr::<u32>(q3 + 0x08, next_head);
                if next_head == (cap as u32) {
                    wr::<u32>(q3 + 0x08, 0);
                }
                wr::<u32>(q3 + 0x10, cnt.wrapping_sub(1));
            }

            let mut local_40: [usize; 2] = [0; 2];
            exe::call_va!(0x1_411d_5b00, (local_40.as_mut_ptr() as usize));
            let obj_48 = local_40[1];
            let vtbl = rd::<usize>(obj_48);
            let fn_1b0 = rd::<u64>(vtbl + 0x1b0);
            let arg_18 = rd::<u64>(item + 0x18);
            let arg_20 = rd::<u32>(item + 0x20);
            let arg_34 = rd::<u32>(item + 0x34);
            let arg_28 = rd::<u64>(item + 0x28);
            exe::call_va!(
                fn_1b0,
                (obj_48, arg_18, arg_20, 0u32, arg_34, arg_28, 0u64)
            );

            let lock_40 = local_40[0];
            if lock_40 != 0 {
                let new_c = rd::<u32>(lock_40 + 0x0c).wrapping_sub(1);
                wr::<u32>(lock_40 + 0x0c, new_c);
                if new_c == 0 {
                    let _ = crate::simd_ops::atomic_xchg_u32((lock_40 + 0x08) as u64, u32::MAX);
                    let lock_reload = local_40[0];
                    exe::call_iat!(0x1_4264_5598, (lock_reload));
                    r15_40 = local_50[0];
                }
            }

            wr::<u64>(item, r15_40);
            ring_push(q5, item as u64);

            if crate::simd_ops::rd_acq_u32(rbp + 0x58) == 0 {
                break;
            }
            core::hint::spin_loop();
        }
    }

    let mut param_2_slot: i32 = param_2;
    let q1_cnt = rd::<u32>(rbp + 0x28);
    let next_cmd: u64 = if q1_cnt == 0 {
        exe::call_va!(
            0x1_411d_b770,
            (
                param_1,
                0usize,
                (&mut param_2_slot as *mut i32) as usize,
                local_50.as_mut_ptr() as usize
            ) -> u64
        )
    } else {
        let buf = rd::<usize>(rbp + 0x18);
        let cap = rd::<u32>(rbp + 0x2c) as u64;
        let head = rd::<u32>(rbp + 0x20) as u64;
        let rem = (head % cap) as usize;
        let val = rd::<u64>(buf.wrapping_add(rem * 8));
        let next_head = (head as u32).wrapping_add(1);
        wr::<u32>(rbp + 0x20, next_head);
        if next_head == (cap as u32) {
            wr::<u32>(rbp + 0x20, 0);
        }
        wr::<u32>(rbp + 0x28, q1_cnt.wrapping_sub(1));
        val
    };
    wr::<u64>(active_slot, next_cmd);

    let mut r10_buf = crate::simd_ops::rd_acq_usize(rbp + 0x78);
    let mut r9_cap = crate::simd_ops::rd_acq_u32(rbp + 0x8c) as u64;
    let mut r8_head = crate::simd_ops::rd_acq_u32(rbp + 0x80) as u64;
    let rem0 = (r8_head % r9_cap) as usize;
    let front0 = rd::<usize>(r10_buf.wrapping_add(rem0 * 8));
    if rd::<u64>(front0).wrapping_add(6) < r15_40 {
        let q1 = (param_1 as isize + (rsi_idx + 1) * 0x18) as usize;
        loop {
            let rem = (r8_head % r9_cap) as usize;
            let item_rbp = rd::<usize>(r10_buf.wrapping_add(rem * 8));
            let q5_cnt = rd::<u32>(q5 + 0x10);
            if q5_cnt != 0 {
                let next_head = (r8_head as u32).wrapping_add(1);
                wr::<u32>(q5 + 0x08, next_head);
                if next_head == (r9_cap as u32) {
                    wr::<u32>(q5 + 0x08, 0);
                }
                wr::<u32>(q5 + 0x10, q5_cnt.wrapping_sub(1));
            }
            wr::<u32>(item_rbp + 0x30, rd::<u32>(item_rbp + 0x30).wrapping_add(1));
            wr::<u32>(item_rbp + 0x34, 0);
            ring_push(q1, item_rbp as u64);

            r10_buf = crate::simd_ops::rd_acq_usize(q5);
            r9_cap = crate::simd_ops::rd_acq_u32(q5 + 0x14) as u64;
            r8_head = crate::simd_ops::rd_acq_u32(q5 + 0x08) as u64;
            let next_rem = (r8_head % r9_cap) as usize;
            let next_front = rd::<usize>(r10_buf.wrapping_add(next_rem * 8));
            if rd::<u64>(next_front).wrapping_add(6) >= r15_40 {
                break;
            }
            core::hint::spin_loop();
        }
    }

    let new_depth = rd::<u32>(rdi_lock + 0x0c).wrapping_sub(1);
    wr::<u32>(rdi_lock + 0x0c, new_depth);
    if new_depth == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((rdi_lock + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rdi_lock));
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

pub unsafe fn diff_unknown_141f5a270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4384_1b24, 4);
    let _g2 = GlobalSave::new(0x1_43a0_8300, 8);

    static mut RESOLVED_NODE: [u8; 0x30] = [0; 0x30];
    static mut SUB_ELEMS: [u8; 0x40] = [0; 0x40];
    static mut ENTRY_70_MEM: [u8; 0x20] = [0; 0x20];
    static mut ENTRY_70_ARR: [u8; 0x50] = [0; 0x50];
    static mut LOG_9270: Vec<(u64, u64, u64)> = Vec::new();

    unsafe extern "win64" fn mock_3d20(_map: usize, out_pair: usize, _key_ptr: usize) {
        wr::<u64>(out_pair, 0);
        wr::<u64>(out_pair + 0x08, 0);
    }
    unsafe extern "win64" fn mock_b000(_mgr: usize, _l30: usize) -> usize {
        core::ptr::addr_of_mut!(RESOLVED_NODE) as usize
    }
    unsafe extern "win64" fn mock_8620(_l48: usize) {}
    unsafe extern "win64" fn mock_9270(
        _p1: usize,
        _rdx: usize,
        r8: u64,
        r9: u64,
        a5: u64,
        a6: u64,
    ) {
        LOG_9270.push((r8 ^ a5, r9, a6));
    }

    let _m1 = crate::difftest::Mock::new(0x1_410d_3d20, mock_3d20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_b000, mock_b000 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_8620, mock_8620 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41f5_9270, mock_9270 as *const ())?;

    let run = |f: F, dep_state: u32| -> (u32, u32, Vec<(u64, u64, u64)>) {
        LOG_9270.clear();
        RESOLVED_NODE = [0; 0x30];
        SUB_ELEMS = [0; 0x40];
        ENTRY_70_MEM = [0; 0x20];
        ENTRY_70_ARR = [0; 0x50];

        wr::<u32>(exe::ptr::<u32>(0x1_4384_1b24) as usize, 7);
        wr::<usize>(exe::ptr::<usize>(0x1_43a0_8300) as usize, 0x1234_0000);

        let sub_p = core::ptr::addr_of_mut!(SUB_ELEMS) as usize;
        wr::<u64>(sub_p, 0x999);
        let res_p = core::ptr::addr_of_mut!(RESOLVED_NODE) as usize;
        wr::<usize>(res_p + 0x10, sub_p);
        wr::<u32>(res_p + 0x18, 1);

        let e70_arr_p = core::ptr::addr_of_mut!(ENTRY_70_ARR) as usize;
        wr::<u64>(e70_arr_p, 0x999);
        wr::<u32>(e70_arr_p + 0x1c, dep_state);

        let e70_p = core::ptr::addr_of_mut!(ENTRY_70_MEM) as usize;
        wr::<u64>(e70_p, 0x111);
        wr::<usize>(e70_p + 0x08, e70_arr_p);
        wr::<u32>(e70_p + 0x10, 1);

        let mut elem_rsi = [0u8; 0x20];
        let rsip = elem_rsi.as_mut_ptr() as usize;
        wr::<u64>(rsip, 0x777);
        wr::<u32>(rsip + 0x18, 0);

        let mut bucket_e0 = [0u8; 0x18];
        let be0p = bucket_e0.as_mut_ptr() as usize;
        wr::<u64>(be0p, 0x222);
        wr::<usize>(be0p + 0x08, rsip);
        wr::<u32>(be0p + 0x10, 1);

        let mut p1 = [0u8; 0x100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p, 99);
        wr::<u64>(p1p + 0x48, 1);
        wr::<u64>(p1p + 0x50, u64::MAX);
        wr::<usize>(p1p + 0x68, e70_p);
        wr::<u64>(p1p + 0xc0, 1);
        wr::<u64>(p1p + 0xc8, u64::MAX);
        wr::<usize>(p1p + 0xe0, be0p);

        f(p1p, 0xaa, 0xbb, 0xcc);
        (rd::<u32>(p1p), rd::<u32>(rsip + 0x18), LOG_9270.clone())
    };

    for dep_state in [1u32, 2u32] {
        let o = run(f_o, dep_state);
        let n = run(f_n, dep_state);
        crate::diff_eq!(o, n, "141f5a270 dep_state={}", dep_state);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f5b080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43cc_aec8, 2);

    static mut VEC_HDR: [u64; 2] = [0; 2];
    static mut ELEMS_BUF: [u64; 16] = [0; 16];
    static mut ALLOC_BUF: [u64; 16] = [0; 16];
    static mut LOG_B370: Vec<(u64, u8, u64, u64)> = Vec::new();
    static mut LOG_C310: Vec<u64> = Vec::new();

    unsafe extern "win64" fn mock_7df0(_p1: usize) -> usize {
        core::ptr::addr_of_mut!(VEC_HDR) as usize
    }
    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}
    unsafe extern "win64" fn mock_b370(elem: u64, p2: u8, _p4: usize, l40: usize) {
        LOG_B370.push((elem, p2, rd::<u64>(l40 + 0x08), rd::<u64>(l40 + 0x10)));
    }
    unsafe extern "win64" fn mock_c310(_p3: usize, slice_ptr: usize) {
        let cnt = rd::<u64>(slice_ptr) as usize;
        let ptr = rd::<usize>(slice_ptr + 0x08);
        for i in 0..cnt {
            LOG_C310.push(rd::<u64>(ptr + i * 8));
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4021_7df0, mock_7df0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41f5_b370, mock_b370 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41e5_c310, mock_c310 as *const ())?;

    let run = |f: F, count: u32| -> (u8, u64, Vec<(u64, u8, u64, u64)>, Vec<u64>) {
        LOG_B370.clear();
        LOG_C310.clear();
        ELEMS_BUF = [0; 16];
        ALLOC_BUF = [0; 16];

        wr::<u16>(exe::ptr::<u16>(0x1_43cc_aec8) as usize, 1);

        let ep = core::ptr::addr_of_mut!(ELEMS_BUF) as usize;
        wr::<u64>(ep, 0x0001_4005);
        wr::<u64>(ep + 0x20, 0x0000_0005);
        wr::<u64>(ep + 0x40, 0x0002_8009);

        let vp = core::ptr::addr_of_mut!(VEC_HDR) as usize;
        wr::<usize>(vp, ep);
        wr::<u32>(vp + 0x08, count);

        let mut tab_20 = [0u16; 4];
        tab_20[1] = 2;
        let mut p3_0 = [0u8; 0x30];
        wr::<usize>(p3_0.as_mut_ptr() as usize + 0x20, tab_20.as_ptr() as usize);

        let mut p3_8 = [0u8; 0x40];
        let p3_8p = p3_8.as_mut_ptr() as usize;
        wr::<u64>(p3_8p + 2 * 0x10, 0xdead_beef);
        wr::<u32>(p3_8p + 2 * 0x10 + 0x08, 42);

        let mut p3: [usize; 2] = [p3_0.as_mut_ptr() as usize, p3_8p];
        let mut p1: [u64; 2] = [0x111, 0x999];

        let ret = f(p1.as_mut_ptr() as usize, 1, p3.as_mut_ptr() as usize, 0);
        (ret, p1[1], LOG_B370.clone(), LOG_C310.clone())
    };

    for cnt in [0u32, 3u32] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "141f5b080 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f5b370(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u8, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43c8_d87c, 2);

    static mut FLAG_VAL: u32 = 0;

    unsafe extern "win64" fn mock_a730(_p3: usize, out_20: usize, key_ptr: usize) {
        let k = rd::<u64>(key_ptr);
        wr::<u64>(out_20, if k == 0x100 { 0 } else { u64::MAX });
    }
    unsafe extern "win64" fn mock_9ee0(_ptr: usize, len: u32, word: u32) -> i32 {
        if len == word { 1 } else { -1 }
    }
    unsafe extern "win64" fn mock_b4a0(_p4: usize, _idx: u64) -> usize {
        core::ptr::addr_of_mut!(FLAG_VAL) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4035_b4a0, mock_b4a0 as *const ())?;

    static mut R9_BIG: [u8; 0x18100] = [0; 0x18100];

    let run = |f: F, key: u64, set_bit: u8| -> u32 {
        FLAG_VAL = 0x55;
        R9_BIG = [0; 0x18100];
        wr::<u16>(exe::ptr::<u16>(0x1_43c8_d87c) as usize, 7);

        let mut node_58 = [0u64; 4];
        let n58p = node_58.as_mut_ptr() as usize;
        wr::<u64>(n58p + 0x08, 0);

        let mut p3 = [0u8; 0x60];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x30, 0);
        wr::<u64>(p3p + 0x38, 1);
        wr::<u64>(p3p + 0x40, 1);
        wr::<usize>(p3p + 0x58, n58p);

        let mut tab_30: [u64; 2] = [1, 0];
        let r9p = core::ptr::addr_of_mut!(R9_BIG) as usize;
        wr::<usize>(r9p + 0x30, tab_30.as_mut_ptr() as usize);
        wr::<usize>(r9p + 0x20 + 0x18090, 0x1234);
        wr::<u32>(r9p + 0x20 + 0x180a8, 7);

        let mut p4: [usize; 1] = [r9p];
        f(key, set_bit, p3p, p4.as_mut_ptr() as usize);
        FLAG_VAL
    };

    for (key, set_bit) in [(0x100u64, 1u8), (0x100u64, 0u8), (0x200u64, 1u8)] {
        let o = run(f_o, key, set_bit);
        let n = run(f_n, key, set_bit);
        crate::diff_eq!(o, n, "141f5b370 key={} set_bit={}", key, set_bit);
    }
    Ok(3)
}

pub unsafe fn diff_coregame_141f732a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_2a00(_p2: usize, p3: usize, len: u64) -> u32 {
        (rd::<u8>(p3) as u32).wrapping_add(len as u32 * 100)
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f7_2a00, mock_2a00 as *const ())?;

    let run = |f: F, len: u64| -> (Vec<u8>, u64) {
        let mut p1 = vec![0xaa_u8; 0x20];
        let mut p2 = vec![0u8; 0x140];
        let src = b"0123456789abcdef0123456789abcdef";

        let p1p = p1.as_mut_ptr() as usize;
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x04, 5 << 5);

        f(p1p, p2p, src.as_ptr() as usize, len);
        (p1, rd::<u64>(p2p + 0x118))
    };

    for len in [0u64, 5, 15, 16, 24] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "141f732a0 len={}", len);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_141fcfce0(orig: usize, new: usize) -> Result<u32, String> {
    type F =
        unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, u32, u8, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43a0_8428, 8);

    static mut MAT_LOG: [u32; 12] = [0; 12];
    static mut CALL_LOG: (u32, u8, u8, u32, u32, usize) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_42e0(
        _p1: usize,
        _p2: usize,
        mat_ptr: usize,
        _p4: usize,
        _p5: usize,
        p8: u8,
        p9: u8,
    ) {
        let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
        MAT_LOG = rd::<[u32; 12]>(mat_ptr);
        CALL_LOG.0 = xmm3 as u32;
        CALL_LOG.1 = p8;
        CALL_LOG.2 = p9;
    }
    unsafe extern "win64" fn mock_ea10(_mgr: usize, _p6: usize, _a2: usize, _a3: usize) {
        let [_, _, xmm2, xmm3] = crate::simd_ops::take_xmm_args();
        CALL_LOG.3 = xmm2 as u32;
        CALL_LOG.4 = xmm3 as u32;
    }
    unsafe extern "win64" fn mock_e980(_mgr: usize, p3: usize, _p6: usize) {
        CALL_LOG.5 = p3;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4201_42e0, mock_42e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424c_ea10, mock_ea10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424c_e980, mock_e980 as *const ())?;

    let run = |f: F, p6_flag: u8, p10_first: f32| -> ([u32; 12], (u32, u8, u8, u32, u32, usize), u32, u32) {
        MAT_LOG = [0; 12];
        CALL_LOG = (0, 0, 0, 0, 0, 0);
        wr::<usize>(exe::ptr::<usize>(0x1_43a0_8428) as usize, 0x5555);

        let p4: [f32; 8] = [0.0, 0.70710677, 0.0, 0.70710677, 10.0, 20.0, 30.0, 1.0];
        let mut p5 = [0u8; 0x1a0];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<f32>(p5p + 0x74, 2.5);
        wr::<f32>(p5p + 0x7c, 3.5);
        wr::<f32>(p5p + 0x18c, 4.5);
        wr::<f32>(p5p + 0x190, 5.5);

        let mut p6 = [0u8; 0x20];
        let p6p = p6.as_mut_ptr() as usize;
        wr::<u8>(p6p + 0x18, p6_flag);

        let mut p10 = [0u8; 0x18];
        let p10p = p10.as_mut_ptr() as usize;
        wr::<f32>(p10p, p10_first);
        wr::<f32>(p10p + 0x04, 9.0);
        wr::<u8>(p10p + 0x10, 1);

        f(
            0x11,
            0x22,
            0x33,
            p4.as_ptr() as usize,
            p5p,
            p6p,
            1.25f32.to_bits(),
            1,
            0,
            p10p,
        );
        (MAT_LOG, CALL_LOG, rd::<u32>(p5p + 0x74), rd::<u32>(p5p + 0x7c))
    };

    for (p6_flag, p10_first) in [(0u8, -1.0f32), (1u8, 7.0f32)] {
        let o = run(f_o, p6_flag, p10_first);
        let n = run(f_n, p6_flag, p10_first);
        crate::diff_eq!(o, n, "141fcfce0 p6_flag={} p10_first={}", p6_flag, p10_first);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14202ea90(orig: usize, new: usize) -> Result<u32, String> {
    static mut ERASED_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_c750(_map: usize, _it: usize) {
        ERASED_COUNT += 1;
    }
    unsafe extern "win64" fn filter_cb(_ctx: usize, val: u32) -> u8 {
        u8::from(val != 20)
    }

    let _m1 = crate::difftest::Mock::new(0x1_411b_c750, mock_c750 as *const ())?;

    let run = |fn_addr: usize, dt: f32| -> (u32, u32, [u32; 3], [u32; 2]) {
        ERASED_COUNT = 0;

        let mut outer_ctrl = [-1i8; 32];
        outer_ctrl[0] = -128;
        outer_ctrl[1] = 0;

        let mut inner_ctrl = [-1i8; 32];
        inner_ctrl[0] = 0;
        inner_ctrl[1] = 0;

        let mut inner_elems: [(u32, f32); 2] = [(10, 0.5), (30, 2.0)];
        let mut vec_elems: [u32; 3] = [10, 20, 30];

        let mut outer_elems = [0u8; 0x90];
        let oep = outer_elems.as_mut_ptr() as usize;
        let b1 = oep + 0x48;
        wr::<usize>(b1 + 0x08, vec_elems.as_mut_ptr() as usize);
        wr::<u32>(b1 + 0x10, 3);
        wr::<usize>(b1 + 0x18, inner_ctrl.as_mut_ptr() as usize);
        wr::<usize>(b1 + 0x20, inner_elems.as_mut_ptr() as usize);
        wr::<usize>(b1 + 0x30, 2);

        let mut p1: [usize; 4] = [outer_ctrl.as_mut_ptr() as usize, oep, 0, 2];
        let mut p3: [usize; 2] = [0, filter_cb as *const () as usize];

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1.as_mut_ptr() as usize as u64,
            0,
            p3.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dt.to_bits() as u128, 0, 0],
        );
        (
            ERASED_COUNT,
            rd::<u32>(b1 + 0x10),
            vec_elems,
            [inner_elems[0].1.to_bits(), inner_elems[1].1.to_bits()],
        )
    };

    for dt in [0.25f32, 1.0f32] {
        let o = run(orig, dt);
        let n = run(new, dt);
        crate::diff_eq!(o, n, "14202ea90 dt={}", dt);
    }
    Ok(2)
}

pub unsafe fn diff_coreshared_14204a3b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43ce_6854, 0x24);

    let run = |f: F, flag_58: u8| -> (bool, Vec<u8>, u32) {
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_6870) as usize, i32::MIN);
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_6854) as usize, i32::MIN);
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_6860) as usize, i32::MIN);
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_6864) as usize, i32::MIN);

        wr::<u32>(exe::ptr::<u32>(0x1_43ce_6868) as usize, 101);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_686c) as usize, 102);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_685c) as usize, 103);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_6858) as usize, 104);

        let keys: [u32; 4] = [101, 102, 103, 104];
        let mut vals = [0u8; 4 * 0x18];
        let vp = vals.as_mut_ptr() as usize;
        wr::<f32>(vp + 0x08, 0.5);
        wr::<f32>(vp + 0x18 + 0x08, 2.0);
        wr::<f32>(vp + 0x30 + 0x08, 3.0);
        wr::<f32>(vp + 0x48 + 0x08, 4.0);

        let mut map_hdr = [0u8; 0x18];
        let mhp = map_hdr.as_mut_ptr() as usize;
        wr::<usize>(mhp, keys.as_ptr() as usize);
        wr::<u32>(mhp + 0x08, 4);
        wr::<usize>(mhp + 0x10, vp);

        let map_ptr_holder: [usize; 1] = [mhp];
        let mut res_60 = [0u8; 0xd0];
        let r60p = res_60.as_mut_ptr() as usize;
        wr::<u32>(r60p + 0x24, 5);
        wr::<usize>(r60p + 0xc0, map_ptr_holder.as_ptr() as usize);

        let mut p2 = [0u8; 0x70];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p, 0x11);
        wr::<f32>(p2p + 0x04, 2.0);
        wr::<f32>(p2p + 0x08, 3.0);
        wr::<f32>(p2p + 0x0c, 4.0);
        wr::<f32>(p2p + 0x10, 5.0);
        wr::<u32>(p2p + 0x14, 0x22);
        wr::<u64>(p2p + 0x34, 0xaaaa);
        wr::<u64>(p2p + 0x3c, 0xbbbb);
        wr::<u32>(p2p + 0x44, 0x33);
        wr::<u32>(p2p + 0x48, 0x44);
        wr::<u8>(p2p + 0x58, flag_58);
        wr::<u32>(p2p + 0x5c, 0x55);
        wr::<usize>(p2p + 0x60, r60p);

        let p3: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut p1 = vec![0u8; 0x88];
        let p1p = p1.as_mut_ptr() as usize;

        let ret = f(p1p, p2p, p3.as_ptr() as usize);
        wr::<usize>(p1p + 0x48, 0);
        (ret == p1p, p1, rd::<u32>(r60p + 0x24))
    };

    for flag_58 in [0u8, 1u8] {
        let o = run(f_o, flag_58);
        let n = run(f_n, flag_58);
        crate::diff_eq!(o, n, "14204a3b0 flag_58={}", flag_58);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142058260(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u16, u16, u64, u32, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43a2_9174, 2);
    let _g2 = GlobalSave::new(0x1_43a2_91bc, 2);
    let _g3 = GlobalSave::new(0x1_43a2_b03c, 4);
    let _g4 = GlobalSave::new(0x1_43a2_8c3c, 1);
    let _g5 = GlobalSave::new(0x1_43a3_5a98, 8);

    static mut LOG_CALLS: Vec<u64> = Vec::new();

    unsafe extern "win64" fn fn_290(arg: u64) {
        LOG_CALLS.push(arg ^ 0x290);
    }
    unsafe extern "win64" fn fn_d8(arg: u64) {
        LOG_CALLS.push(arg ^ 0xd8);
    }

    let run = |f: F, flag_14d: u8| -> (u32, u32, u64, usize, Vec<u64>) {
        LOG_CALLS.clear();

        let mut vtbl = [0u8; 0x2a0];
        let vp = vtbl.as_mut_ptr() as usize;
        wr::<usize>(vp + 0xd8, fn_d8 as *const () as usize);
        wr::<usize>(vp + 0x290, fn_290 as *const () as usize);

        let arr_b8: [u64; 2] = [0x1111, 0x2222];
        let mut arr_d0: [u64; 2] = [0x9999, 0x8888];
        let vec_items: [u64; 2] = [0xaaaa, 0xbbbb];
        let mut vec_28 = [0usize; 6];
        let v28p = vec_28.as_mut_ptr() as usize;
        wr::<usize>(v28p + 0x18, vec_items.as_ptr() as usize);
        wr::<usize>(v28p + 0x20, (vec_items.as_ptr() as usize) + 16);

        let mut ctx = [0u8; 0x160];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x28, v28p);
        wr::<usize>(cp + 0xb8, arr_b8.as_ptr() as usize);
        wr::<usize>(cp + 0xd0, arr_d0.as_mut_ptr() as usize);
        wr::<usize>(cp + 0xe8, vp);
        wr::<u32>(cp + 0x140, 2);
        wr::<u32>(cp + 0x148, 3);
        wr::<u8>(cp + 0x14d, flag_14d);

        wr::<u16>(exe::ptr::<u16>(0x1_43a2_9174) as usize, 1920);
        wr::<u16>(exe::ptr::<u16>(0x1_43a2_91bc) as usize, 1080);
        wr::<u32>(exe::ptr::<u32>(0x1_43a2_b03c) as usize, 7);
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8c3c) as usize, 1);
        wr::<usize>(exe::ptr::<usize>(0x1_43a3_5a98) as usize, cp);

        f(1920, 1080, 0, 7, 1);
        (
            rd::<u32>(cp + 0x144),
            rd::<u32>(cp + 0x148),
            arr_d0[1],
            rd::<usize>(v28p + 0x20).wrapping_sub(rd::<usize>(v28p + 0x18)),
            LOG_CALLS.clone(),
        )
    };

    for flag_14d in [0u8, 1u8] {
        let o = run(f_o, flag_14d);
        let n = run(f_n, flag_14d);
        crate::diff_eq!(o, n, "142058260 flag_14d={}", flag_14d);
    }
    Ok(2)
}

pub unsafe fn diff_d3d_142063760(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4397_fc58, 8);

    static mut DEV_OBJ: usize = 0;
    static mut LOG_1B0: Vec<(u64, u32, u32, u64)> = Vec::new();

    unsafe extern "win64" fn mock_5b00(out_40: usize) {
        wr::<usize>(out_40, 0);
        wr::<usize>(out_40 + 0x08, DEV_OBJ);
    }
    unsafe extern "win64" fn mock_b770(_c: usize, _d: usize, _r8: usize, _r9: usize) -> u64 {
        0x7777_0000
    }
    unsafe extern "win64" fn fn_1b0(
        _this: usize,
        a2: u64,
        a3: u32,
        _a4: u32,
        a5: u32,
        a6: u64,
        _a7: u64,
    ) {
        LOG_1B0.push((a2, a3, a5, a6));
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_5b00, mock_5b00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_b770, mock_b770 as *const ())?;

    let run = |f: F, free_avail: bool| -> (u64, u32, u32, u64, u32, Vec<(u64, u32, u32, u64)>) {
        LOG_1B0.clear();

        let mut vtbl = [0u8; 0x1c0];
        wr::<usize>(
            vtbl.as_mut_ptr() as usize + 0x1b0,
            fn_1b0 as *const () as usize,
        );
        let dev_holder: [usize; 1] = [vtbl.as_ptr() as usize];
        DEV_OBJ = dev_holder.as_ptr() as usize;

        let mut fc58 = [0u8; 0x50];
        let fc58p = fc58.as_mut_ptr() as usize;
        wr::<u64>(fc58p + 0x38, 100);
        wr::<u64>(fc58p + 0x40, 50);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize, fc58p);

        let mut cmd_active = [0u8; 0x40];
        let cap_ptr = cmd_active.as_mut_ptr() as usize;
        wr::<u64>(cap_ptr + 0x18, 0x111);
        wr::<u32>(cap_ptr + 0x20, 2);
        wr::<u64>(cap_ptr + 0x28, 0x333);
        wr::<u32>(cap_ptr + 0x34, 4);

        let mut cmd_old = [0u8; 0x40];
        let cold_ptr = cmd_old.as_mut_ptr() as usize;
        wr::<u64>(cold_ptr, 10);
        wr::<u32>(cold_ptr + 0x30, 5);
        wr::<u32>(cold_ptr + 0x34, 9);

        let mut q1_buf: [usize; 4] = [0x8888_0000, 0, 0, 0];
        let mut q3_buf: [usize; 4] = [0; 4];
        let mut q5_buf: [usize; 4] = [cold_ptr, 0, 0, 0];

        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        wr::<u32>(p1p + 0x10, tid);
        wr::<u32>(p1p + 0x14, 1);

        // q1 at +0x18
        wr::<usize>(p1p + 0x18, q1_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x20, 0);
        wr::<u32>(p1p + 0x24, u32::from(free_avail));
        wr::<u32>(p1p + 0x28, u32::from(free_avail));
        wr::<u32>(p1p + 0x2c, 4);

        // q3 at +0x48
        wr::<usize>(p1p + 0x48, q3_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x50, 0);
        wr::<u32>(p1p + 0x54, 0);
        wr::<u32>(p1p + 0x58, 0);
        wr::<u32>(p1p + 0x5c, 4);

        // q5 at +0x78
        wr::<usize>(p1p + 0x78, q5_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x80, 0);
        wr::<u32>(p1p + 0x84, 1);
        wr::<u32>(p1p + 0x88, 1);
        wr::<u32>(p1p + 0x8c, 4);

        wr::<usize>(p1p + 0xa8, cap_ptr);

        f(p1p, 0);
        (
            rd::<u64>(p1p + 0xa8),
            rd::<u32>(p1p + 0x28),
            rd::<u32>(p1p + 0x88),
            rd::<u64>(cap_ptr),
            rd::<u32>(cold_ptr + 0x30),
            LOG_1B0.clone(),
        )
    };

    for free_avail in [false, true] {
        let o = run(f_o, free_avail);
        let n = run(f_n, free_avail);
        crate::diff_eq!(o, n, "142063760 free_avail={}", free_avail);
    }
    Ok(2)
}
