//! Decompiled hot functions shard Y8B (`0x14253b6b0..0x1425412f0`).

use crate::exe::{self, rd, wr};

/// `unknown_14253b6b0`
/// Original: 0x14253b6b0 (380 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253b6b0(param_1: usize, param_2: usize) -> usize {
    let mut rax = rd::<u32>(param_1 + 0x1c) as u64;
    let rsi = param_1.wrapping_sub(1) & !0x01ff_ffffusize;
    if rax >= 0x8000_0000 {
        rax = (rd::<u32>(param_1) as u64) << 16;
    }

    let rdx_neg = ((rd::<u16>(param_1 + 0x0a) as u64).wrapping_mul(rax)).wrapping_neg();
    let stats_0 = rd::<usize>(param_2 + 0x380);
    exe::call_va!(0x1_4254_08f0, (stats_0.wrapping_add(0xc0), rdx_neg));

    let stats_1 = rd::<usize>(param_2 + 0x380);
    exe::call_va!(0x1_4254_08f0, (stats_1.wrapping_add(0x20), u64::MAX));

    if rd::<u8>(rsi + 0x18) != 0 {
        let g_d50 = exe::ptr::<u32>(0x1_4387_2d50) as usize;
        let g_d54 = exe::ptr::<u32>(0x1_4387_2d54) as usize;
        if rd::<u32>(g_d54) == 0 {
            exe::call_va!(0x1_4253_75a0, (g_d50));
        }
        if rd::<u32>(g_d50) != 0 {
            let rdi = rd::<u32>(param_1 + 0x1c) as u64;
            let idx = (((param_1.wrapping_sub(rsi).wrapping_sub(0x108)) as i64) / 0x50) as u64;
            let span_bytes = (rd::<u32>(param_1) as u64) << 16;
            let off = if rdi < 8 {
                0
            } else if rdi <= 0x40 {
                rdi.wrapping_mul(3)
            } else if rdi <= 0x200 {
                rdi
            } else {
                0
            };
            let r8 = rd::<u64>(param_2 + 0x380);
            let addr = (idx << 16).wrapping_add(off).wrapping_add(rsi as u64);
            let sz = span_bytes.wrapping_sub(off);
            exe::call_va!(0x1_4253_91a0, (addr as usize, sz, r8));
        }
    }

    wr::<u8>(param_1 + 0x08, rd::<u8>(param_1 + 0x08) & 0xfd);
    let g_580 = exe::ptr::<u8>(0x1_43a0_9580) as usize;
    if rd::<u8>(g_580) != 0 {
        for i in 0..0x46usize {
            wr::<u8>(param_1 + 0x0a + i, 0);
        }
    } else {
        wr::<u64>(param_1 + 0x0a, 0);
        wr::<u64>(param_1 + 0x12, 0);
        wr::<u16>(param_1 + 0x1a, 0);
        wr::<u64>(param_1 + 0x20, 0);
        wr::<u64>(param_1 + 0x28, 0);
        wr::<u64>(param_1 + 0x30, 0);
        wr::<u64>(param_1 + 0x38, 0);
        wr::<u64>(param_1 + 0x40, 0);
        wr::<u64>(param_1 + 0x48, 0);
    }

    wr::<u32>(param_1 + 0x1c, 1);
    let ret = exe::call_va!(0x1_4253_c400, (param_1, param_2) -> usize);
    wr::<u64>(rsi + 0xd0, rd::<u64>(rsi + 0xd0).wrapping_sub(1));
    ret
}

/// `unknown_14253bf60`
/// Original: 0x14253bf60 (572 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253bf60(
    param_1: u64,
    param_2: u32,
    param_3: usize,
) -> usize {
    let bin_idx = if param_1 <= 1 {
        param_1
    } else {
        let u6 = param_1.wrapping_sub(1);
        if u6 == 0 {
            let r8: u64 = 0x40;
            (((u6 >> ((r8 - 2) & 0x3f)) & 3) | (r8 * 4)).wrapping_sub(4)
        } else {
            let b = (63 - u6.leading_zeros()) as u64;
            if b <= 2 {
                param_1
            } else {
                (((u6 >> ((b - 2) & 0x3f)) & 3) | (b * 4)).wrapping_sub(4)
            }
        }
    };

    let want_slices = if param_1 != 0 { param_1 } else { 1 };
    let mut bin_ptr = param_3.wrapping_add((bin_idx as usize).wrapping_mul(0x18));
    let max_bin_ptr = param_3.wrapping_add(0x348);

    while bin_ptr <= max_bin_ptr {
        let mut rbx = rd::<usize>(bin_ptr);
        while rbx != 0 {
            if (rd::<u32>(rbx) as u64) >= want_slices {
                let rdi = rbx.wrapping_sub(1) & !0x01ff_ffffusize;
                let memid_kind = (rd::<u64>(rdi + 0x10) >> 32) as u32;
                let matched = if memid_kind == 6 {
                    (rd::<u8>(rdi + 0x0c) == 0 && param_2 == 0)
                        || rd::<u32>(rdi + 0x08) == param_2
                } else {
                    param_2 == 0
                };

                if matched {
                    let next_40 = rd::<usize>(rbx + 0x40);
                    if next_40 != 0 {
                        wr::<u64>(next_40 + 0x38, rd::<u64>(rbx + 0x38));
                    }
                    if rbx == rd::<usize>(bin_ptr) {
                        wr::<u64>(bin_ptr, rd::<u64>(rbx + 0x38));
                    }
                    let prev_38 = rd::<usize>(rbx + 0x38);
                    if prev_38 != 0 {
                        wr::<u64>(prev_38 + 0x40, rd::<u64>(rbx + 0x40));
                    }
                    if rbx == rd::<usize>(bin_ptr + 0x08) {
                        wr::<u64>(bin_ptr + 0x08, rd::<u64>(rbx + 0x40));
                    }

                    let mut r8 = rd::<u32>(rbx) as u64;
                    wr::<u64>(rbx + 0x40, 0);
                    wr::<u64>(rbx + 0x38, 0);
                    wr::<u32>(rbx + 0x1c, 1);

                    if r8 > want_slices {
                        let seg = rbx.wrapping_sub(1) & !0x01ff_ffffusize;
                        let idx = (((rbx.wrapping_sub(seg).wrapping_sub(0x108)) as i64) / 0x50)
                            as u64;
                        exe::call_va!(
                            0x1_4253_c600,
                            (
                                rdi,
                                idx.wrapping_add(want_slices),
                                r8.wrapping_sub(want_slices),
                                0u8,
                                param_3,
                            )
                        );
                        r8 = want_slices & 0xffff_ffff;
                        wr::<u32>(rbx, want_slices as u32);
                    }

                    let seg = rbx.wrapping_sub(1) & !0x01ff_ffffusize;
                    let idx = (((rbx.wrapping_sub(seg).wrapping_sub(0x108)) as i64) / 0x50) as u64;
                    let page = exe::call_va!(0x1_4253_c220, (rdi, idx, r8) -> usize);
                    if page == 0 {
                        let _ = exe::call_va!(0x1_4253_c400, (rbx, param_3) -> usize);
                        return 0;
                    }
                    return page;
                }
            }
            rbx = rd::<usize>(rbx + 0x38);
        }
        bin_ptr = bin_ptr.wrapping_add(0x18);
    }
    0
}

/// `unknown_14253c220`
/// Original: 0x14253c220 (467 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253c220(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> usize {
    let rdi = param_1
        .wrapping_add(0x108)
        .wrapping_add((param_2 as usize).wrapping_mul(0x50));
    let idx = (((rdi.wrapping_sub(param_1).wrapping_sub(0x108)) as i64) / 0x50) as u64;

    let mut need_commit = false;
    for i in 0..8usize {
        if rd::<u64>(param_1 + 0x70 + i * 8) != u64::MAX {
            need_commit = true;
            break;
        }
    }
    if !need_commit {
        for i in 0..8usize {
            if rd::<u64>(param_1 + 0x30 + i * 8) != 0 {
                need_commit = true;
                break;
            }
        }
    }

    if need_commit {
        let ok = exe::call_va!(
            0x1_4253_d110,
            (
                param_1,
                (idx << 16).wrapping_add(param_1 as u64),
                param_3 << 16,
            ) -> u8
        );
        if ok == 0 {
            return 0;
        }
    }

    wr::<u32>(rdi + 0x04, 0);
    wr::<u32>(rdi, param_3 as u32);
    let shifted = param_3 << 16;
    let bsize = if shifted >= 0x8000_0000 {
        0x8000_0000u32
    } else {
        shifted as u32
    };
    wr::<u32>(rdi + 0x1c, bsize);

    let mut r8 = param_3.wrapping_sub(1).min(0xff);
    let cap_f8 = rd::<u64>(param_1 + 0xf8);
    if r8.wrapping_add(param_2) >= cap_f8 {
        r8 = cap_f8.wrapping_sub(param_2).wrapping_sub(1);
    }

    let mut rcx: u64 = 1;
    let mut rdx_ptr = rdi.wrapping_add(0x50);
    while rcx <= r8 {
        wr::<u32>(rdx_ptr, 0);
        wr::<u32>(rdx_ptr + 0x04, (rcx as u32).wrapping_mul(0x50));
        wr::<u32>(rdx_ptr + 0x1c, 1);
        rcx = rcx.wrapping_add(1);
        rdx_ptr = rdx_ptr.wrapping_add(0x50);
    }

    let last_in_span = rdi
        .wrapping_add((param_3 as usize).wrapping_mul(0x50))
        .wrapping_sub(0x50);
    let seg_end = param_1
        .wrapping_add(0x108)
        .wrapping_add((rd::<u64>(param_1 + 0xf8) as usize).wrapping_mul(0x50));
    let tail_ptr = if last_in_span <= seg_end {
        last_in_span
    } else {
        seg_end
    };
    if tail_ptr > rdi {
        wr::<u32>(tail_ptr, 0);
        wr::<u32>(tail_ptr + 0x1c, 1);
        let q = ((tail_ptr.wrapping_sub(rdi) as i64) / 0x50) as u32;
        wr::<u32>(tail_ptr + 0x04, q.wrapping_mul(0x50));
    }

    wr::<u8>(rdi + 0x08, rd::<u8>(rdi + 0x08) | 1);
    wr::<u64>(param_1 + 0xd0, rd::<u64>(param_1 + 0xd0).wrapping_add(1));
    rdi
}

/// `unknown_14253c400`
/// Original: 0x14253c400 (292 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253c400(param_1: usize, param_2: usize) -> usize {
    let rsi = param_1.wrapping_sub(1) & !0x01ff_ffffusize;
    let r15 = rd::<u64>(rsi + 0x100);
    if rd::<u32>(rsi + 0xf0) == 1 {
        wr::<u32>(param_1 + 0x1c, 0);
        return param_1;
    }

    let mut rbx = param_1;
    let mut r14 = rd::<u32>(param_1) as u64;
    let seg_end = rsi
        .wrapping_add(0x108)
        .wrapping_add((rd::<u64>(rsi + 0xf8) as usize).wrapping_mul(0x50));
    let next_slice = rbx.wrapping_add((r14 as usize).wrapping_mul(0x50));
    if next_slice < seg_end && rd::<u32>(next_slice + 0x1c) == 0 {
        r14 = r14.wrapping_add(rd::<u32>(next_slice) as u64);
        if r15 != 0 {
            exe::call_va!(0x1_4253_c530, (next_slice, param_2));
        }
    }

    if rbx > rsi.wrapping_add(0x108) {
        let back_off = rd::<u32>(rbx.wrapping_sub(0x4c)) as usize;
        let rdi = rbx.wrapping_sub(0x50).wrapping_sub(back_off);
        if rd::<u32>(rdi + 0x1c) == 0 {
            r14 = r14.wrapping_add(rd::<u32>(rdi) as u64);
            if r15 != 0 {
                exe::call_va!(0x1_4253_c530, (rdi, param_2));
            }
            rbx = rdi;
        }
    }

    let seg = rbx.wrapping_sub(1) & !0x01ff_ffffusize;
    let idx = (((rbx.wrapping_sub(seg).wrapping_sub(0x108)) as i64) / 0x50) as u64;
    exe::call_va!(0x1_4253_c600, (rsi, idx, r14, 1u8, param_2));
    rbx
}

/// `unknown_14253c600`
/// Original: 0x14253c600 (320 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253c600(
    param_1: usize,
    param_2: u64,
    param_3: u64,
    param_4: u8,
    param_5: usize,
) {
    let rdi = if rd::<u32>(param_1 + 0xf0) != 1 && rd::<u64>(param_1 + 0x100) != 0 {
        exe::call_va!(0x1_4253_db60, (param_3, param_5) -> usize)
    } else {
        0
    };

    let r8 = if param_3 != 0 { param_3 } else { 1 };
    let rbx = param_1
        .wrapping_add(0x108)
        .wrapping_add((param_2 as usize).wrapping_mul(0x50));
    wr::<u32>(rbx, r8 as u32);
    wr::<u32>(rbx + 0x04, 0);

    if r8 > 1 {
        let end_idx = r8.wrapping_add(param_2) as usize;
        let base_off = param_1.wrapping_add(end_idx.wrapping_mul(0x50));
        wr::<u32>(base_off.wrapping_add(0xb8), 0);
        wr::<u32>(
            base_off.wrapping_add(0xbc),
            (r8 as u32).wrapping_mul(0x50).wrapping_sub(0x50),
        );
        wr::<u32>(base_off.wrapping_add(0xd4), 0);
    }

    if param_4 != 0 {
        let r10 = rbx.wrapping_sub(1) & !0x01ff_ffffusize;
        let idx = (((rbx.wrapping_sub(r10).wrapping_sub(0x108)) as i64) / 0x50) as u64;
        let addr = (idx << 16).wrapping_add(r10 as u64);
        let sz = r8 << 16;
        let stats = rd::<u64>(param_5 + 0x380);
        exe::call_va!(0x1_4253_c8d0, (param_1, addr, sz, stats));
    }

    if rdi != 0 {
        wr::<u64>(rbx + 0x40, 0);
        let head = rd::<usize>(rdi);
        wr::<usize>(rbx + 0x38, head);
        wr::<usize>(rdi, rbx);
        let next = rd::<usize>(rbx + 0x38);
        if next != 0 {
            wr::<usize>(next + 0x40, rbx);
        } else {
            wr::<usize>(rdi + 0x08, rbx);
        }
    }
    wr::<u32>(rbx + 0x1c, 0);
}

/// `unknown_14253c770`
/// Original: 0x14253c770 (334 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253c770(param_1: usize, param_2: u8, param_3: u64) {
    if rd::<u8>(param_1 + 0x19) == 0 {
        return;
    }

    let mut any_set = false;
    for i in 0..8usize {
        if rd::<u64>(param_1 + 0x30 + i * 8) != 0 {
            any_set = true;
            break;
        }
    }
    if !any_set {
        return;
    }

    let now = exe::call_va!(0x1_4254_12f0, () -> i64);
    if param_2 == 0 && now < rd::<i64>(param_1 + 0x28) {
        return;
    }

    let local_mask = rd::<[u64; 8]>(param_1 + 0x30);
    wr::<u64>(param_1 + 0x28, 0);
    wr::<[u64; 8]>(param_1 + 0x30, [0u64; 8]);

    let mut rbx: u64 = 0;
    loop {
        let mut rcx = (rbx >> 6) as usize;
        rbx &= 0x3f;
        if rcx >= 8 {
            return;
        }
        let mut rax = loop {
            let v = local_mask[rcx] >> (rbx as u32);
            if v != 0 {
                break v;
            }
            rcx += 1;
            rbx = 0;
            if rcx >= 8 {
                return;
            }
        };

        while (rax & 1) == 0 {
            rax >>= 1;
            rbx += 1;
        }
        if rcx >= 8 {
            return;
        }

        let rdx = ((rcx as u64) << 6).wrapping_add(rbx);
        let mut r8: u64 = 0;
        loop {
            loop {
                r8 = r8.wrapping_add(1);
                rax >>= 1;
                if (rax & 1) == 0 {
                    break;
                }
            }
            rbx = r8.wrapping_add(rdx);
            if (rbx & 0x3f) == 0 {
                rcx += 1;
                if rcx >= 8 {
                    break;
                }
                rax = local_mask[rcx];
            }
            if (rax & 1) == 0 {
                break;
            }
        }

        if r8 == 0 {
            return;
        }
        exe::call_va!(
            0x1_4253_cca0,
            (
                param_1,
                (rdx << 16).wrapping_add(param_1 as u64),
                r8 << 16,
                param_3,
            )
        );
    }
}

/// `unknown_14253c8d0`
/// Original: 0x14253c8d0 (962 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253c8d0(
    param_1: usize,
    param_2: u64,
    param_3: u64,
    param_4: u64,
) {
    if rd::<u8>(param_1 + 0x19) == 0 {
        return;
    }

    let g_dd0 = exe::ptr::<i32>(0x1_4387_2dd0) as usize;
    let g_dd4 = exe::ptr::<u32>(0x1_4387_2dd4) as usize;
    if rd::<u32>(g_dd4) == 0 {
        exe::call_va!(0x1_4253_75a0, (g_dd0));
    }
    if rd::<u32>(g_dd0) == 0 {
        exe::call_va!(0x1_4253_cca0, (param_1, param_2, param_3, param_4));
        return;
    }

    let mut mask = [0u64; 8];
    let mut fullsize: u64 = 0;
    if param_3.wrapping_sub(1) <= 0x01ff_ffff && rd::<u32>(param_1 + 0xf0) != 1 {
        let mut rcx_end = rd::<u64>(param_1 + 0xe0) << 16;
        let rdx_start = rd::<u64>(param_1 + 0xe8) << 16;
        if param_2 < rcx_end.wrapping_add(param_1 as u64) {
            let offset = param_2.wrapping_sub(param_1 as u64);
            let mut start = offset.wrapping_add(0xffff) & !0xffffu64;
            let end = offset.wrapping_add(param_3) & !0xffffu64;
            if offset >= rdx_start && start < rdx_start {
                start = rdx_start;
            }
            if end <= rcx_end {
                rcx_end = end;
            }
            fullsize = if rcx_end <= start {
                0
            } else {
                rcx_end.wrapping_sub(start)
            };
            if fullsize != 0 {
                let idx = start >> 16;
                let count = fullsize >> 16;
                if count.wrapping_add(idx) > 0x200 {
                    let fmt = exe::ptr::<u8>(0x1_4342_5cb0) as usize;
                    exe::call_va!(
                        0x1_4253_7a30,
                        (fmt, idx, count, start, rcx_end, param_2, param_3, fullsize)
                    );
                }
                if count == 0x200 {
                    mask = [u64::MAX; 8];
                } else if count != 0 {
                    let mut word_idx = (idx >> 6) as usize;
                    let mut bit_off = idx & 0x3f;
                    let mut rem = count;
                    while rem != 0 {
                        let n = rem.min(0x40 - bit_off);
                        let w = if n < 0x40 {
                            ((1u64 << (n as u32)) - 1) << (bit_off as u32)
                        } else {
                            u64::MAX
                        };
                        mask[word_idx] = w;
                        word_idx += 1;
                        bit_off = 0;
                        rem -= n;
                    }
                }
            }
        }
    }

    let mut any_nonzero = false;
    for i in 0..8usize {
        if mask[i] != 0 {
            any_nonzero = true;
            break;
        }
    }
    if !any_nonzero || fullsize == 0 {
        return;
    }

    for i in 0..8usize {
        let cur_30 = rd::<u64>(param_1 + 0x30 + i * 8);
        let cur_70 = rd::<u64>(param_1 + 0x70 + i * 8);
        wr::<u64>(param_1 + 0x30 + i * 8, cur_30 | (cur_70 & mask[i]));
    }

    let now = exe::call_va!(0x1_4254_12f0, () -> i64);
    let expire = rd::<i64>(param_1 + 0x28);
    if expire == 0 {
        if rd::<u32>(g_dd4) == 0 {
            exe::call_va!(0x1_4253_75a0, (g_dd0));
        }
        let delay = rd::<i32>(g_dd0) as i64;
        wr::<i64>(param_1 + 0x28, delay.wrapping_add(now));
        return;
    }

    let g_f10 = exe::ptr::<i32>(0x1_4387_2f10) as usize;
    let g_f14 = exe::ptr::<u32>(0x1_4387_2f14) as usize;
    if expire > now {
        if rd::<u32>(g_f14) == 0 {
            exe::call_va!(0x1_4253_75a0, (g_f10));
        }
        let step = rd::<i32>(g_f10) as i64;
        wr::<i64>(param_1 + 0x28, rd::<i64>(param_1 + 0x28).wrapping_add(step));
        return;
    }

    if rd::<u32>(g_f14) == 0 {
        exe::call_va!(0x1_4253_75a0, (g_f10));
    }
    let step = rd::<i32>(g_f10) as i64;
    if step.wrapping_add(rd::<i64>(param_1 + 0x28)) <= now {
        exe::call_va!(0x1_4253_c770, (param_1, 1u8, param_4));
    } else {
        if rd::<u32>(g_f14) == 0 {
            exe::call_va!(0x1_4253_75a0, (g_f10));
        }
        let step2 = rd::<i32>(g_f10) as i64;
        wr::<i64>(param_1 + 0x28, step2.wrapping_add(now));
    }
}

/// `unknown_14253d110`
/// Original: 0x14253d110 (1384 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253d110(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> u8 {
    let mut mask = [0u64; 8];
    let mut start_addr: u64 = 0;
    let mut fullsize: u64 = 0;

    if param_3.wrapping_sub(1) <= 0x01ff_ffff && rd::<u32>(param_1 + 0xf0) != 1 {
        let rdx_min = rd::<u64>(param_1 + 0xe8) << 16;
        let mut rcx_max = rd::<u64>(param_1 + 0xe0) << 16;
        if param_2 < rcx_max.wrapping_add(param_1 as u64) {
            let offset = param_2.wrapping_sub(param_1 as u64);
            let end = param_3.wrapping_add(0xffff).wrapping_add(offset) & !0xffffu64;
            let mut start = offset & !0xffffu64;
            if offset >= rdx_min && start < rdx_min {
                start = rdx_min;
            }
            start_addr = start.wrapping_add(param_1 as u64);
            if end <= rcx_max {
                rcx_max = end;
            }
            fullsize = if rcx_max <= start {
                0
            } else {
                rcx_max.wrapping_sub(start)
            };
            if fullsize != 0 {
                let idx = start >> 16;
                let count = fullsize >> 16;
                if count.wrapping_add(idx) > 0x200 {
                    let fmt = exe::ptr::<u8>(0x1_4342_5cb0) as usize;
                    exe::call_va!(
                        0x1_4253_7a30,
                        (fmt, idx, count, start, rcx_max, param_2, param_3, fullsize)
                    );
                }
                if count == 0x200 {
                    mask = [u64::MAX; 8];
                } else if count != 0 {
                    let mut word_idx = (idx >> 6) as usize;
                    let mut bit_off = idx & 0x3f;
                    let mut rem = count;
                    while rem != 0 {
                        let n = rem.min(0x40 - bit_off);
                        let w = if n < 0x40 {
                            ((1u64 << (n as u32)) - 1) << (bit_off as u32)
                        } else {
                            u64::MAX
                        };
                        mask[word_idx] = w;
                        word_idx += 1;
                        bit_off = 0;
                        rem -= n;
                    }
                }
            }
        }
    }

    let mut any_nonzero = false;
    for i in 0..8usize {
        if mask[i] != 0 {
            any_nonzero = true;
            break;
        }
    }
    if !any_nonzero || fullsize == 0 {
        return 1;
    }

    let mut all_committed = true;
    for i in 0..8usize {
        if (rd::<u64>(param_1 + 0x70 + i * 8) & mask[i]) != mask[i] {
            all_committed = false;
            break;
        }
    }

    if !all_committed {
        let mut already_committed = [0u64; 8];
        for i in 0..8usize {
            already_committed[i] = rd::<u64>(param_1 + 0x70 + i * 8) & mask[i];
        }
        let already_bytes = exe::call_va!(
            0x1_4253_ddf0,
            (already_committed.as_ptr() as usize, 0x0200_0000u64) -> u64
        );
        let g_620 = exe::ptr::<u64>(0x1_43a0_9620) as usize;
        exe::call_va!(0x1_4254_08f0, (g_620, already_bytes.wrapping_neg()));

        let g_638 = exe::ptr::<u64>(0x1_43a0_9638) as u64;
        let old_638 = crate::simd_ops::atomic_xadd_u64(g_638, fullsize);
        let new_638 = old_638.wrapping_add(fullsize);
        let g_630 = exe::ptr::<u64>(0x1_43a0_9630) as usize;
        loop {
            let cur_peak = rd::<i64>(g_630);
            if cur_peak >= (new_638 as i64) {
                break;
            }
            let (ok, _) =
                crate::simd_ops::atomic_cmpxchg_u64(g_630 as u64, cur_peak as u64, new_638);
            if ok {
                break;
            }
        }

        let (target_stat, add_val) = if (fullsize as i64) <= 0 {
            (
                exe::ptr::<u64>(0x1_43a0_9628) as u64,
                fullsize.wrapping_neg(),
            )
        } else {
            (exe::ptr::<u64>(0x1_43a0_9620) as u64, fullsize)
        };
        crate::simd_ops::atomic_add_u64(target_stat, add_val);
        crate::simd_ops::atomic_add_u64(exe::ptr::<u64>(0x1_43a0_97c8) as u64, 1);
        crate::simd_ops::atomic_add_u64(exe::ptr::<u64>(0x1_43a0_97c0) as u64, 1);

        if start_addr != 0 {
            let page_size = rd::<u64>(exe::ptr::<u64>(0x1_4387_2f30) as usize);
            let mask_ps = page_size.wrapping_sub(1);
            let (aligned_start, aligned_end) = if (page_size & mask_ps) == 0 {
                (
                    !mask_ps & start_addr,
                    !mask_ps & fullsize.wrapping_add(mask_ps).wrapping_add(start_addr),
                )
            } else {
                (
                    (start_addr / page_size).wrapping_mul(page_size),
                    (mask_ps.wrapping_add(start_addr).wrapping_add(fullsize) / page_size)
                        .wrapping_mul(page_size),
                )
            };
            let dw_size = aligned_end.wrapping_sub(aligned_start);
            if (dw_size as i64) > 0 {
                let pv = exe::call_iat!(
                    0x1_4264_5730,
                    (aligned_start as usize, dw_size, 0x1000u32, 4u32) -> usize
                );
                if pv == 0 {
                    let err = exe::call_iat!(0x1_4264_5708, () -> u32);
                    if err != 0 {
                        let fmt = exe::ptr::<u8>(0x1_4342_5b20) as usize;
                        exe::call_va!(
                            0x1_4253_7a30,
                            (fmt, err, err, aligned_start as usize, dw_size)
                        );
                        return 0;
                    }
                }
            }
        }

        for i in 0..8usize {
            wr::<u64>(
                param_1 + 0x70 + i * 8,
                rd::<u64>(param_1 + 0x70 + i * 8) | mask[i],
            );
        }
    }

    let mut any_decommit_overlap = false;
    for i in 0..8usize {
        if (rd::<u64>(param_1 + 0x30 + i * 8) & mask[i]) != 0 {
            any_decommit_overlap = true;
            break;
        }
    }
    if any_decommit_overlap {
        let now = exe::call_va!(0x1_4254_12f0, () -> i64);
        let g_dd0 = exe::ptr::<i32>(0x1_4387_2dd0) as usize;
        let g_dd4 = exe::ptr::<u32>(0x1_4387_2dd4) as usize;
        if rd::<u32>(g_dd4) == 0 {
            exe::call_va!(0x1_4253_75a0, (g_dd0));
        }
        let delay = rd::<i32>(g_dd0) as i64;
        wr::<i64>(param_1 + 0x28, delay.wrapping_add(now));
    }

    for i in 0..8usize {
        wr::<u64>(
            param_1 + 0x30 + i * 8,
            rd::<u64>(param_1 + 0x30 + i * 8) & !mask[i],
        );
    }
    1
}

/// `unknown_1425412f0`
/// Original: 0x1425412f0 (112 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425412f0() -> i64 {
    let mut counter: i64 = 0;
    let _ = exe::call_iat!(0x1_4264_56f8, ((&mut counter as *mut i64) as usize) -> i32);
    let g_freq = exe::ptr::<i64>(0x1_43bd_6038) as usize;
    let mut freq = rd::<i64>(g_freq);
    if freq == 0 {
        let mut raw_freq: i64 = 0;
        let _ = exe::call_iat!(0x1_4264_5700, ((&mut raw_freq as *mut i64) as usize) -> i32);
        freq = raw_freq / 1000;
        wr::<i64>(g_freq, freq);
        if freq == 0 {
            freq = 1;
            wr::<i64>(g_freq, 1);
        }
    }
    counter.wrapping_div(freq)
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14253b6b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut STAT_LOG: u64 = 0;
    static mut FILL_LOG: (usize, u64, u64) = (0, 0, 0);
    static mut C400_LOG: (usize, usize) = (0, 0);

    unsafe extern "win64" fn mock_08f0(stat_ptr: usize, delta: u64) {
        STAT_LOG = STAT_LOG
            .wrapping_add(stat_ptr as u64)
            .wrapping_add(delta.wrapping_mul(3));
    }
    unsafe extern "win64" fn mock_75a0(opt_ptr: usize) {
        wr::<u32>(opt_ptr, 1);
        wr::<u32>(opt_ptr + 4, 1);
    }
    unsafe extern "win64" fn mock_91a0(addr: usize, sz: u64, r8: u64) {
        FILL_LOG = (addr, sz, r8);
    }
    unsafe extern "win64" fn mock_c400(page: usize, tld: usize) -> usize {
        C400_LOG = (page, tld);
        page.wrapping_add(0x50)
    }

    let _m1 = crate::difftest::Mock::new(0x1_4254_08f0, mock_08f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_75a0, mock_75a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_91a0, mock_91a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_c400, mock_c400 as *const ())?;

    let g_d50 = exe::ptr::<u32>(0x1_4387_2d50) as usize;
    let g_d54 = exe::ptr::<u32>(0x1_4387_2d54) as usize;
    let g_580 = exe::ptr::<u8>(0x1_43a0_9580) as usize;
    let s_d50 = rd::<u32>(g_d50);
    let s_d54 = rd::<u32>(g_d54);
    let s_580 = rd::<u8>(g_580);

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, bsize: u32, rep_stosb: u8, allow_reset: u8| -> (usize, u64, (usize, u64, u64), (usize, usize), u64, [u8; 0x50]) {
        STAT_LOG = 0;
        FILL_LOG = (0, 0, 0);
        C400_LOG = (0, 0);
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1000);
        wr::<u8>(seg + 0x18, allow_reset);
        wr::<u64>(seg + 0xd0, 5);

        wr::<u32>(g_d50, 0);
        wr::<u32>(g_d54, 0);
        wr::<u8>(g_580, rep_stosb);

        let page = seg + 0x108 + 2 * 0x50;
        for i in 0..0x50usize {
            wr::<u8>(page + i, 0xaa);
        }
        wr::<u32>(page, 3);
        wr::<u8>(page + 0x08, 0x07);
        wr::<u16>(page + 0x0a, 4);
        wr::<u32>(page + 0x1c, bsize);

        let mut tld = vec![0u8; 0x390];
        let tp = tld.as_mut_ptr() as usize;
        wr::<u64>(tp + 0x380, 0x9000_0000);

        let ret = f(page, tp);
        let fill_norm = (FILL_LOG.0.wrapping_sub(seg), FILL_LOG.1, FILL_LOG.2);
        let c400_norm = (C400_LOG.0.wrapping_sub(seg), C400_LOG.1.wrapping_sub(tp));
        (
            ret.wrapping_sub(seg),
            STAT_LOG,
            fill_norm,
            c400_norm,
            rd::<u64>(seg + 0xd0),
            rd::<[u8; 0x50]>(page),
        )
    };

    for (bsize, rs, ar) in [
        (16u32, 0u8, 1u8),
        (0x100, 1, 1),
        (0x8000_0000, 0, 0),
    ] {
        let o = run(f_o, bsize, rs, ar);
        let n = run(f_n, bsize, rs, ar);
        wr::<u32>(g_d50, s_d50);
        wr::<u32>(g_d54, s_d54);
        wr::<u8>(g_580, s_580);
        crate::diff_eq!(o, n, "14253b6b0 bsize={:#x} rs={} ar={}", bsize, rs, ar);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14253bf60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u32, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut C600_LOG: (usize, u64, u64, u8) = (0, 0, 0, 0);
    static mut C220_FAIL: bool = false;
    static mut C400_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_c600(
        seg: usize,
        idx: u64,
        rem: u64,
        flag: u8,
        _tld: usize,
    ) {
        C600_LOG = (seg, idx, rem, flag);
    }
    unsafe extern "win64" fn mock_c220(seg: usize, idx: u64, cnt: u64) -> usize {
        if C220_FAIL {
            0
        } else {
            seg + 0x108 + (idx as usize) * 0x50 + (cnt as usize)
        }
    }
    unsafe extern "win64" fn mock_c400(page: usize, _tld: usize) -> usize {
        C400_CALLED += 1;
        page
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_c600, mock_c600 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_c220, mock_c220 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_c400, mock_c400 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, req_slices: u64, numa: u32, fail_c220: bool| -> (usize, (usize, u64, u64, u8), u32, u32, u32, usize, usize) {
        C600_LOG = (0, 0, 0, 0);
        C220_FAIL = fail_c220;
        C400_CALLED = 0;
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1000);
        wr::<u32>(seg + 0x08, numa);
        wr::<u8>(seg + 0x0c, 1);
        wr::<u32>(seg + 0x14, 6);

        let slice = seg + 0x108 + 2 * 0x50;
        wr::<u32>(slice, 16);

        let mut tld = vec![0u8; 0x380];
        let tp = tld.as_mut_ptr() as usize;
        // Bin for 16 slices: u6 = 15, b = 3, ((15 >> 1) & 3 | 12) - 4 = 11
        let bin_ptr = tp + 11 * 0x18;
        wr::<usize>(bin_ptr, slice);
        wr::<usize>(bin_ptr + 0x08, slice);

        let ret = f(req_slices, numa, tp);
        let ret_norm = if ret != 0 { ret.wrapping_sub(seg) } else { 0 };
        let c600_norm = (
            if C600_LOG.0 != 0 { C600_LOG.0.wrapping_sub(seg) } else { 0 },
            C600_LOG.1,
            C600_LOG.2,
            C600_LOG.3,
        );
        (
            ret_norm,
            c600_norm,
            C400_CALLED,
            rd::<u32>(slice),
            rd::<u32>(slice + 0x1c),
            rd::<usize>(bin_ptr),
            rd::<usize>(bin_ptr + 0x08),
        )
    };

    for (req, numa, fail) in [(2u64, 0u32, false), (12, 3, false), (16, 3, true)] {
        let o = run(f_o, req, numa, fail);
        let n = run(f_n, req, numa, fail);
        crate::diff_eq!(o, n, "14253bf60 req={} numa={} fail={}", req, numa, fail);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14253c220(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut D110_CALL: (usize, u64, u64) = (0, 0, 0);
    static mut D110_RET: u8 = 1;

    unsafe extern "win64" fn mock_d110(seg: usize, addr: u64, sz: u64) -> u8 {
        D110_CALL = (seg, addr, sz);
        D110_RET
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_d110, mock_d110 as *const ())?;

    let run = |f: F, all_committed: bool, d110_ok: u8, idx: u64, cnt: u64| -> (usize, (u64, u64), u64, Vec<u32>) {
        D110_CALL = (0, 0, 0);
        D110_RET = d110_ok;
        let mut seg = vec![0u8; 0x108 + 10 * 0x50];
        let sp = seg.as_mut_ptr() as usize;
        wr::<u64>(sp + 0xd0, 2);
        wr::<u64>(sp + 0xf8, 8);
        for i in 0..8usize {
            wr::<u64>(sp + 0x70 + i * 8, if all_committed { u64::MAX } else { 0x0f });
            wr::<u64>(sp + 0x30 + i * 8, 0);
        }

        let ret = f(sp, idx, cnt);
        let ret_off = if ret != 0 { ret.wrapping_sub(sp) } else { 0 };
        let call_norm = if D110_CALL.0 != 0 {
            (D110_CALL.1.wrapping_sub(sp as u64), D110_CALL.2)
        } else {
            (0, 0)
        };
        let mut words = Vec::with_capacity(8 * 4);
        for s in 0..8usize {
            let p = sp + 0x108 + s * 0x50;
            words.push(rd::<u32>(p));
            words.push(rd::<u32>(p + 0x04));
            words.push(rd::<u32>(p + 0x08));
            words.push(rd::<u32>(p + 0x1c));
        }
        (ret_off, call_norm, rd::<u64>(sp + 0xd0), words)
    };

    for (ac, ok, idx, cnt) in [
        (true, 1u8, 1u64, 4u64),
        (false, 1, 2, 3),
        (false, 0, 1, 2),
    ] {
        let o = run(f_o, ac, ok, idx, cnt);
        let n = run(f_n, ac, ok, idx, cnt);
        crate::diff_eq!(o, n, "14253c220 ac={} ok={} idx={} cnt={}", ac, ok, idx, cnt);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14253c400(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut C530_CALLS: [usize; 4] = [0; 4];
    static mut C530_LEN: usize = 0;
    static mut C600_CALL: (usize, u64, u64, u8, usize) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_c530(slice: usize, _tld: usize) {
        let idx = C530_LEN;
        if idx < 4 {
            C530_CALLS[idx] = slice;
            C530_LEN = idx + 1;
        }
    }
    unsafe extern "win64" fn mock_c600(
        seg: usize,
        idx: u64,
        cnt: u64,
        flag: u8,
        tld: usize,
    ) {
        C600_CALL = (seg, idx, cnt, flag, tld);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_c530, mock_c530 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_c600, mock_c600 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, kind_f0: u32| -> (usize, u32, [usize; 4], usize, (usize, u64, u64, u8, usize)) {
        C530_CALLS = [0; 4];
        C530_LEN = 0;
        C600_CALL = (0, 0, 0, 0, 0);
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1000);
        wr::<u32>(seg + 0xf0, kind_f0);
        wr::<u64>(seg + 0xf8, 10);
        wr::<u64>(seg + 0x100, 0x1234);

        // Slice 0..2 free (count = 2), slice 2..4 being freed (count = 2), slice 4..7 free (count = 3)
        let s0 = seg + 0x108;
        let s1 = seg + 0x108 + 0x50;
        let s2 = seg + 0x108 + 2 * 0x50;
        let s4 = seg + 0x108 + 4 * 0x50;

        wr::<u32>(s0, 2);
        wr::<u32>(s0 + 0x1c, 0);
        wr::<u32>(s1 + 0x04, 0x50); // back_off from s1 to s0

        wr::<u32>(s2, 2);
        wr::<u32>(s2 + 0x1c, 1);

        wr::<u32>(s4, 3);
        wr::<u32>(s4 + 0x1c, 0);

        let ret = f(s2, 0x7777);
        let raw_calls = C530_CALLS;
        let len = C530_LEN;
        let mut c530_offs = [0usize; 4];
        for i in 0..len {
            c530_offs[i] = raw_calls[i].wrapping_sub(seg);
        }
        let c600_norm = (
            if C600_CALL.0 != 0 { C600_CALL.0.wrapping_sub(seg) } else { 0 },
            C600_CALL.1,
            C600_CALL.2,
            C600_CALL.3,
            C600_CALL.4,
        );
        (ret.wrapping_sub(seg), rd::<u32>(s2 + 0x1c), c530_offs, len, c600_norm)
    };

    for kind_f0 in [1u32, 0] {
        let o = run(f_o, kind_f0);
        let n = run(f_n, kind_f0);
        crate::diff_eq!(o, n, "14253c400 kind_f0={}", kind_f0);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14253c600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u8, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut C8D0_CALL: (usize, u64, u64, u64) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_c8d0(seg: usize, addr: u64, sz: u64, stats: u64) {
        C8D0_CALL = (seg, addr, sz, stats);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_c8d0, mock_c8d0 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, idx: u64, cnt: u64, decommit: u8, has_head: bool| -> ((u64, u64, u64), [u32; 6], usize, usize) {
        C8D0_CALL = (0, 0, 0, 0);
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1000);
        wr::<u32>(seg + 0xf0, 0);
        wr::<u64>(seg + 0x100, 0x1234);

        let mut tld = vec![0u8; 0x390];
        let tp = tld.as_mut_ptr() as usize;
        wr::<u64>(tp + 0x380, 0xabcd_0000);

        let bin_ptr = exe::call_va!(0x1_4253_db60, (cnt, tp) -> usize);
        let dummy_node = seg + 0x108 + 8 * 0x50;
        if has_head {
            wr::<usize>(bin_ptr, dummy_node);
            wr::<usize>(bin_ptr + 0x08, dummy_node);
        }

        f(seg, idx, cnt, decommit, tp);

        let first = seg + 0x108 + (idx as usize) * 0x50;
        let last = seg + 0x108 + ((idx + cnt.max(1) - 1) as usize) * 0x50;
        let c8d0_norm = if C8D0_CALL.0 != 0 {
            (C8D0_CALL.1.wrapping_sub(seg as u64), C8D0_CALL.2, C8D0_CALL.3)
        } else {
            (0, 0, 0)
        };
        let slice_state = [
            rd::<u32>(first),
            rd::<u32>(first + 0x04),
            rd::<u32>(first + 0x1c),
            rd::<u32>(last),
            rd::<u32>(last + 0x04),
            rd::<u32>(last + 0x1c),
        ];
        (
            c8d0_norm,
            slice_state,
            rd::<usize>(bin_ptr).wrapping_sub(seg),
            rd::<usize>(bin_ptr + 0x08).wrapping_sub(seg),
        )
    };

    for (idx, cnt, dec, hh) in [(2u64, 4u64, 1u8, false), (1, 1, 0, true)] {
        let o = run(f_o, idx, cnt, dec, hh);
        let n = run(f_n, idx, cnt, dec, hh);
        crate::diff_eq!(o, n, "14253c600 idx={} cnt={} dec={} hh={}", idx, cnt, dec, hh);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14253c770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CCA0_CALLS: [(u64, u64, u64); 4] = [(0, 0, 0); 4];
    static mut CCA0_LEN: usize = 0;

    unsafe extern "win64" fn mock_12f0() -> i64 {
        1000
    }
    unsafe extern "win64" fn mock_cca0(seg: usize, addr: u64, sz: u64, stats: u64) {
        let idx = CCA0_LEN;
        if idx < 4 {
            CCA0_CALLS[idx] = (addr.wrapping_sub(seg as u64), sz, stats);
            CCA0_LEN = idx + 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4254_12f0, mock_12f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_cca0, mock_cca0 as *const ())?;

    let run = |f: F, force: u8, expire: i64| -> ([(u64, u64, u64); 4], usize, u64, [u64; 8]) {
        CCA0_CALLS = [(0, 0, 0); 4];
        CCA0_LEN = 0;
        let mut seg = [0u8; 0x80];
        let sp = seg.as_mut_ptr() as usize;
        wr::<u8>(sp + 0x19, 1);
        wr::<i64>(sp + 0x28, expire);
        // Contiguous run across word 0 (bits 60..64) and word 1 (bits 0..4), plus bits 2..5 in word 3
        wr::<u64>(sp + 0x30, 0xf000_0000_0000_0000);
        wr::<u64>(sp + 0x38, 0x0000_0000_0000_000f);
        wr::<u64>(sp + 0x48, 0b0001_1100);

        f(sp, force, 0x5555);
        (
            CCA0_CALLS,
            CCA0_LEN,
            rd::<u64>(sp + 0x28),
            rd::<[u64; 8]>(sp + 0x30),
        )
    };

    for (force, exp) in [(0u8, 2000i64), (0, 500), (1, 2000)] {
        let o = run(f_o, force, exp);
        let n = run(f_n, force, exp);
        crate::diff_eq!(o, n, "14253c770 force={} exp={}", force, exp);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14253c8d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CCA0_LOG: (u64, u64, u64) = (0, 0, 0);
    static mut C770_LOG: (u8, u64) = (0, 0);

    unsafe extern "win64" fn mock_12f0() -> i64 {
        1000
    }
    unsafe extern "win64" fn mock_cca0(seg: usize, addr: u64, sz: u64, stats: u64) {
        CCA0_LOG = (addr.wrapping_sub(seg as u64), sz, stats);
    }
    unsafe extern "win64" fn mock_c770(_seg: usize, force: u8, stats: u64) {
        C770_LOG = (force, stats);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4254_12f0, mock_12f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_cca0, mock_cca0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_c770, mock_c770 as *const ())?;

    let g_dd0 = exe::ptr::<i32>(0x1_4387_2dd0) as usize;
    let g_dd4 = exe::ptr::<u32>(0x1_4387_2dd4) as usize;
    let g_f10 = exe::ptr::<i32>(0x1_4387_2f10) as usize;
    let g_f14 = exe::ptr::<u32>(0x1_4387_2f14) as usize;
    let s_dd0 = rd::<i32>(g_dd0);
    let s_dd4 = rd::<u32>(g_dd4);
    let s_f10 = rd::<i32>(g_f10);
    let s_f14 = rd::<u32>(g_f14);

    let run = |f: F, delay_dd0: i32, init_exp: i64| -> ((u64, u64, u64), (u8, u64), i64, [u64; 8]) {
        CCA0_LOG = (0, 0, 0);
        C770_LOG = (0, 0);
        wr::<i32>(g_dd0, delay_dd0);
        wr::<u32>(g_dd4, 1);
        wr::<i32>(g_f10, 100);
        wr::<u32>(g_f14, 1);

        let mut seg = [0u8; 0x110];
        let sp = seg.as_mut_ptr() as usize;
        wr::<u8>(sp + 0x19, 1);
        wr::<i64>(sp + 0x28, init_exp);
        wr::<u64>(sp + 0xe0, 0x200);
        wr::<u64>(sp + 0xe8, 1);
        wr::<u32>(sp + 0xf0, 0);
        wr::<[u64; 8]>(sp + 0x70, [u64::MAX; 8]);

        f(sp, (sp as u64).wrapping_add(0x2_0000), 0x3_0000, 0x9999);
        (
            CCA0_LOG,
            C770_LOG,
            rd::<i64>(sp + 0x28),
            rd::<[u64; 8]>(sp + 0x30),
        )
    };

    for (dd0, exp) in [(0i32, 0i64), (500, 0), (500, 1200), (500, 800), (500, 950)] {
        let o = run(f_o, dd0, exp);
        let n = run(f_n, dd0, exp);
        wr::<i32>(g_dd0, s_dd0);
        wr::<u32>(g_dd4, s_dd4);
        wr::<i32>(g_f10, s_f10);
        wr::<u32>(g_f14, s_f14);
        crate::diff_eq!(o, n, "14253c8d0 dd0={} exp={}", dd0, exp);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14253d110(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VALLOC_FAIL: bool = false;
    static mut VALLOC_LOG: (usize, u64, u32, u32) = (0, 0, 0, 0);
    static mut ERR_LOG: u32 = 0;

    unsafe extern "win64" fn mock_ddf0(mask_ptr: usize, _total: u64) -> u64 {
        let m = rd::<[u64; 8]>(mask_ptr);
        let mut bits = 0u64;
        for w in m {
            bits += w.count_ones() as u64;
        }
        bits << 16
    }
    unsafe extern "win64" fn mock_08f0(stat_ptr: usize, delta: u64) {
        wr::<u64>(stat_ptr, rd::<u64>(stat_ptr).wrapping_add(delta));
    }
    unsafe extern "win64" fn mock_12f0() -> i64 {
        2000
    }
    unsafe extern "win64" fn mock_7a30(_fmt: usize, a1: u32, _a2: u32, _a3: usize, _a4: u64) {
        ERR_LOG = a1;
    }
    unsafe extern "win64" fn mock_valloc(addr: usize, sz: u64, kind: u32, prot: u32) -> usize {
        VALLOC_LOG = (addr, sz, kind, prot);
        if VALLOC_FAIL {
            0
        } else {
            addr
        }
    }
    unsafe extern "win64" fn mock_get_last_error() -> u32 {
        0x5af
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_ddf0, mock_ddf0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4254_08f0, mock_08f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4254_12f0, mock_12f0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_7a30, mock_7a30 as *const ())?;
    let _i1 = crate::difftest::IatMock::new(0x1_4264_5730, mock_valloc as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_5708, mock_get_last_error as *const ())?;

    let g_620 = exe::ptr::<u64>(0x1_43a0_9620) as usize;
    let g_7c0 = exe::ptr::<u64>(0x1_43a0_97c0) as usize;
    let g_f30 = exe::ptr::<u64>(0x1_4387_2f30) as usize;
    let g_dd0 = exe::ptr::<i32>(0x1_4387_2dd0) as usize;
    let g_dd4 = exe::ptr::<u32>(0x1_4387_2dd4) as usize;

    let s_620 = rd::<[u64; 4]>(g_620);
    let s_7c0 = rd::<[u64; 2]>(g_7c0);
    let s_f30 = rd::<u64>(g_f30);
    let s_dd0 = rd::<i32>(g_dd0);
    let s_dd4 = rd::<u32>(g_dd4);

    let run = |f: F, fail_va: bool, pre_committed: bool| -> (u8, (u64, u32, u32), u32, [u64; 4], [u64; 2], i64, [u64; 8], [u64; 8]) {
        VALLOC_FAIL = fail_va;
        VALLOC_LOG = (0, 0, 0, 0);
        ERR_LOG = 0;

        wr::<[u64; 4]>(g_620, [0x10_0000, 0, 0x10_0000, 0x10_0000]);
        wr::<[u64; 2]>(g_7c0, [10, 20]);
        wr::<u64>(g_f30, 0x1000);
        wr::<i32>(g_dd0, 300);
        wr::<u32>(g_dd4, 1);

        let mut seg = [0u8; 0x110];
        let sp = seg.as_mut_ptr() as usize;
        wr::<u64>(sp + 0xe0, 0x200);
        wr::<u64>(sp + 0xe8, 1);
        wr::<u32>(sp + 0xf0, 0);
        wr::<u64>(sp + 0x30, 0b0000_1100);
        wr::<u64>(
            sp + 0x70,
            if pre_committed {
                u64::MAX
            } else {
                0b0000_0100
            },
        );

        let ret = f(sp, (sp as u64).wrapping_add(0x2_0000), 0x3_0000);
        let va_norm = (VALLOC_LOG.1, VALLOC_LOG.2, VALLOC_LOG.3);
        (
            ret,
            va_norm,
            ERR_LOG,
            rd::<[u64; 4]>(g_620),
            rd::<[u64; 2]>(g_7c0),
            rd::<i64>(sp + 0x28),
            rd::<[u64; 8]>(sp + 0x30),
            rd::<[u64; 8]>(sp + 0x70),
        )
    };

    for (fail, pre) in [(false, false), (false, true), (true, false)] {
        let o = run(f_o, fail, pre);
        let n = run(f_n, fail, pre);
        wr::<[u64; 4]>(g_620, s_620);
        wr::<[u64; 2]>(g_7c0, s_7c0);
        wr::<u64>(g_f30, s_f30);
        wr::<i32>(g_dd0, s_dd0);
        wr::<u32>(g_dd4, s_dd4);
        crate::diff_eq!(o, n, "14253d110 fail={} pre={}", fail, pre);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1425412f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> i64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MOCK_COUNTER: i64 = 0;
    static mut MOCK_FREQ: i64 = 0;

    unsafe extern "win64" fn mock_qpc(out: usize) -> i32 {
        wr::<i64>(out, MOCK_COUNTER);
        1
    }
    unsafe extern "win64" fn mock_qpf(out: usize) -> i32 {
        wr::<i64>(out, MOCK_FREQ);
        1
    }

    let _i1 = crate::difftest::IatMock::new(0x1_4264_56f8, mock_qpc as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_5700, mock_qpf as *const ())?;

    let g_freq = exe::ptr::<i64>(0x1_43bd_6038) as usize;
    let saved_freq = rd::<i64>(g_freq);

    let run = |f: F, init_freq: i64, qpf_val: i64, qpc_val: i64| -> (i64, i64) {
        MOCK_COUNTER = qpc_val;
        MOCK_FREQ = qpf_val;
        wr::<i64>(g_freq, init_freq);
        let ret = f();
        (ret, rd::<i64>(g_freq))
    };

    for (init_f, qpf, qpc) in [
        (0i64, 10_000_000i64, 123_450_000i64),
        (0, 500, 777),
        (2_000, 10_000_000, 50_000),
    ] {
        let o = run(f_o, init_f, qpf, qpc);
        let n = run(f_n, init_f, qpf, qpc);
        wr::<i64>(g_freq, saved_freq);
        crate::diff_eq!(o, n, "1425412f0 init_f={} qpf={} qpc={}", init_f, qpf, qpc);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_4253_b6b0, "unknown_14253b6b0", rend::cauldron_hot_y8b::unknown_14253b6b0 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253b6b0),
// Replacement::verified(0x1_4253_bf60, "unknown_14253bf60", rend::cauldron_hot_y8b::unknown_14253bf60 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253bf60),
// Replacement::verified(0x1_4253_c220, "unknown_14253c220", rend::cauldron_hot_y8b::unknown_14253c220 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253c220),
// Replacement::verified(0x1_4253_c400, "unknown_14253c400", rend::cauldron_hot_y8b::unknown_14253c400 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253c400),
// Replacement::verified(0x1_4253_c600, "unknown_14253c600", rend::cauldron_hot_y8b::unknown_14253c600 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253c600),
// Replacement::verified(0x1_4253_c770, "unknown_14253c770", rend::cauldron_hot_y8b::unknown_14253c770 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253c770),
// Replacement::verified(0x1_4253_c8d0, "unknown_14253c8d0", rend::cauldron_hot_y8b::unknown_14253c8d0 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253c8d0),
// Replacement::verified(0x1_4253_d110, "unknown_14253d110", rend::cauldron_hot_y8b::unknown_14253d110 as *const (), rend::cauldron_hot_y8b::diff_unknown_14253d110),
// Replacement::verified(0x1_4254_12f0, "unknown_1425412f0", rend::cauldron_hot_y8b::unknown_1425412f0 as *const (), rend::cauldron_hot_y8b::diff_unknown_1425412f0),
