//! Decompiled hot functions shard X6 (`0x14222c360..0x14241d5c0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn atomic_or_u8(addr: usize, mask: u8) {
    let mut cur = rd::<u8>(addr);
    loop {
        let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u8(addr as u64, cur, cur | mask);
        if ok {
            break;
        }
        cur = prev;
    }
}

#[inline(always)]
unsafe fn atomic_or_u64(addr: usize, mask: u64) {
    let mut cur = rd::<u64>(addr);
    loop {
        let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u64(addr as u64, cur, cur | mask);
        if ok {
            break;
        }
        cur = prev;
    }
}

/// `r_14222c360`
/// Original: 0x14222c360 (213 bytes, Throws::No)
pub unsafe extern "win64" fn r_14222c360(param_1: usize) {
    exe::call_va!(0x1_4238_7250, (rd::<usize>(param_1 + 0x08)));
    let rbx = rd::<usize>(param_1 + 0x08);
    if rd::<u32>(rbx + 0x520) == 0 {
        return;
    }

    let count = rd::<u32>(rbx + 0x5a0);
    let mut max_val: u32 = 0;
    wr::<u32>(rbx + 0x6e0, 0);
    if count != 0 {
        let mut off: usize = 0;
        for _ in 0..count {
            let idx0 = rd::<u32>(rd::<usize>(rbx + 0x188).wrapping_add(off)) as usize;
            off = off.wrapping_add(4);
            let entry_b8 = rd::<usize>(rbx + 0xb8).wrapping_add(idx0 << 5);
            let idx1 = rd::<u32>(entry_b8 + 0x04) as usize;
            let entry_5e8 = rd::<usize>(rbx + 0x5e8).wrapping_add(idx1 << 4);
            let v = rd::<u32>(entry_5e8 + 0x04);
            if max_val < v {
                max_val = v;
            }
            wr::<u32>(rbx + 0x6e0, max_val);
        }
    }

    let rdi = rbx.wrapping_add(0x250);
    let rdx_arr = rd::<usize>(rdi + 0x08);
    let len_260 = rd::<u32>(rdi + 0x10) as usize;
    let out_edx = if rdx_arr != rdx_arr.wrapping_add(len_260 * 8) {
        exe::call_va!(0x1_4222_9ab0, (rdi));
        let len2 = rd::<u32>(rdi + 0x10) as usize;
        let arr2 = rd::<usize>(rdi + 0x08);
        let last = arr2.wrapping_add(len2 * 8);
        let edx = rd::<u32>(last.wrapping_sub(8));
        let eax = rd::<u32>(last.wrapping_sub(4)).wrapping_add(edx);
        if eax == rd::<u32>(rdi) {
            edx
        } else {
            rd::<u32>(rdi)
        }
    } else {
        rd::<u32>(rdi)
    };

    let cur_6e0 = rd::<u32>(rbx + 0x6e0);
    wr::<u32>(rbx + 0x6e0, cur_6e0.wrapping_add(0x3f) >> 6);
    wr::<u32>(rbx + 0x5a4, out_edx);
}

/// `unknown_14222cad0`
/// Original: 0x14222cad0 (432 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14222cad0(param_1: usize) {
    let mut node = rd::<usize>(rd::<usize>(param_1));
    if rd::<u8>(node + 0x19) != 0 {
        return;
    }

    loop {
        let rbp = ((rd::<u32>(node + 0x5a8) as u64).wrapping_add(0x3f)) >> 6;
        let r14 = rd::<usize>(node + 0x570);
        if rbp != 0 {
            let mut rdi: u64 = 0;
            for rsi in 0..(rbp as usize) {
                let mut rcx = rd::<u64>(r14.wrapping_add(rsi * 8));
                while rcx != 0 {
                    let rdx_e0 = rd::<usize>(node + 0xe0);
                    let rbx = rcx.trailing_zeros() as u64;
                    let r11 = rdi.wrapping_add(rbx);
                    let r10 = (r11 << 5) as usize;
                    let flag_addr = rdx_e0.wrapping_add(r10).wrapping_add(0x14);
                    if (rd::<u8>(flag_addr) & 1) != 0 {
                        let r9 = rd::<usize>(node + 0xf0);
                        let r8 = (r11 >> 6) as usize;
                        let bit = (r11 as u32) & 0x3f;
                        let word_addr = r9.wrapping_add(r8 * 8);
                        wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit));
                        let rdx_e0_2 = rd::<usize>(node + 0xe0);
                        let flag_addr2 = rdx_e0_2.wrapping_add(r10).wrapping_add(0x14);
                        wr::<u32>(flag_addr2, rd::<u32>(flag_addr2) & !1u32);
                    }
                    rcx &= !(1u64 << rbx);
                }
                rdi = rdi.wrapping_add(0x40);
            }
        }

        let r10 = rd::<u64>(node + 0x580);
        let r8 = rd::<usize>(node + 0x570);
        let r9 = r10.wrapping_add(0x3f) >> 6;
        if r10 != 0 {
            let end = r8.wrapping_add(((r9 - 1) as usize) * 8);
            let mut cur = r8;
            while cur != end {
                wr::<u64>(cur, 0);
                cur = cur.wrapping_add(8);
            }
            let shift = (!(r10.wrapping_sub(1)) as u32) & 0x3f;
            let mask = !(u64::MAX >> shift);
            let last_addr = r8.wrapping_add((r9 as usize) * 8).wrapping_sub(8);
            wr::<u64>(last_addr, rd::<u64>(last_addr) & mask);
        }

        wr::<u8>(node + 0x70e, 0);

        let right = rd::<usize>(node + 0x10);
        if rd::<u8>(right + 0x19) == 0 {
            node = right;
            let mut left = rd::<usize>(node);
            while rd::<u8>(left + 0x19) == 0 {
                node = left;
                left = rd::<usize>(node);
            }
        } else {
            let mut parent = rd::<usize>(node + 0x08);
            while rd::<u8>(parent + 0x19) == 0 && node == rd::<usize>(parent + 0x10) {
                node = parent;
                parent = rd::<usize>(parent + 0x08);
            }
            node = parent;
        }

        if rd::<u8>(node + 0x19) != 0 {
            break;
        }
    }
}

/// `r_14225abc0`
/// Original: 0x14225abc0 (2624 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14225abc0(param_1: usize) {
    let rbx = param_1;
    let g_ed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let cnt_128 = rd::<u32>(g_ed0 + 0x128);

    exe::call_va!(0x1_406d_cc50, (rbx + 0x6f8, cnt_128 as u64));
    let mut p_zero = rd::<usize>(rbx + 0x6f8);
    let p_zero_end = p_zero.wrapping_add((rd::<u32>(rbx + 0x700) as usize) * 4);
    while p_zero != p_zero_end {
        wr::<u32>(p_zero, 0);
        p_zero = p_zero.wrapping_add(4);
    }

    exe::call_va!(0x1_406d_cc50, (rbx + 0x6c8, cnt_128 as u64));
    let rsi_390 = rbx + 0x390;
    exe::call_va!(0x1_4222_9ab0, (rsi_390));
    exe::call_va!(0x1_4222_9ab0, (rbx + 0x438));

    let rdi_398 = rbx + 0x398;
    let arr_398 = rd::<usize>(rdi_398);
    let len_3a0 = rd::<u32>(rdi_398 + 0x08) as usize;
    if arr_398 == arr_398.wrapping_add(len_3a0 * 8) {
        return;
    }

    let r8d_init = if rd::<usize>(rsi_390 + 0x08)
        != rd::<usize>(rsi_390 + 0x08).wrapping_add((rd::<u32>(rsi_390 + 0x10) as usize) * 8)
    {
        exe::call_va!(0x1_4222_9ab0, (rsi_390));
        let ecx = rd::<u32>(rsi_390 + 0x10) as usize;
        let rax = rd::<usize>(rsi_390 + 0x08);
        let end = rax.wrapping_add(ecx * 8);
        let v_lo = rd::<u32>(end.wrapping_sub(8));
        let v_hi = rd::<u32>(end.wrapping_sub(4));
        if v_hi.wrapping_add(v_lo) == rd::<u32>(rsi_390) {
            v_lo
        } else {
            rd::<u32>(rsi_390)
        }
    } else {
        rd::<u32>(rsi_390)
    };

    let mut rsi_len = rd::<u32>(rdi_398 + 0x08) as u64;
    let rax_arr = rd::<usize>(rdi_398);
    let rcx_end = rax_arr.wrapping_add((rsi_len as usize) * 8);
    if rd::<u32>(rcx_end.wrapping_sub(8)) == r8d_init {
        rsi_len = rsi_len.wrapping_sub(1);
    }
    if rsi_len == 0 {
        return;
    }

    let mut sum_30: u32 = 0;
    let mut p_sum = rax_arr;
    while p_sum != rcx_end {
        sum_30 = sum_30.wrapping_add(rd::<u32>(p_sum + 0x04));
        p_sum = p_sum.wrapping_add(8);
    }

    let step = (r8d_init >> 6).max(4);
    let num_buckets = rd::<u32>(rbx + 0x390)
        .wrapping_sub(1)
        .wrapping_add(step)
        .wrapping_div(step)
        .wrapping_add(1);
    let r12 = num_buckets as u64;

    exe::call_va!(0x1_406d_cc50, (rbx + 0x6d8, r12));
    let mut p_6d8 = rd::<usize>(rbx + 0x6d8);
    let p_6d8_end = p_6d8.wrapping_add((rd::<u32>(rbx + 0x6e0) as usize) * 4);
    while p_6d8 != p_6d8_end {
        wr::<u32>(p_6d8, 0);
        p_6d8 = p_6d8.wrapping_add(4);
    }

    exe::call_va!(0x1_406d_cc50, (rbx + 0x6e8, r12));
    let mut p_6e8 = rd::<usize>(rbx + 0x6e8);
    let p_6e8_end = p_6e8.wrapping_add((rd::<u32>(rbx + 0x6f0) as usize) * 4);
    while p_6e8 != p_6e8_end {
        wr::<u32>(p_6e8, 0);
        p_6e8 = p_6e8.wrapping_add(4);
    }

    let esi_u32 = rsi_len as u32;
    for r9d in 0..esi_u32 {
        let r8_ptr = rd::<usize>(rdi_398).wrapping_add((r9d as usize) * 8);
        let b_idx = ((rd::<u32>(r8_ptr) / step).wrapping_add(1)) as usize;
        let edx = rd::<u32>(r8_ptr + 0x04);
        let dst_6d8 = rd::<usize>(rbx + 0x6d8).wrapping_add(b_idx * 4);
        wr::<u32>(dst_6d8, rd::<u32>(dst_6d8).wrapping_add(edx));
        let dst_6e8 = rd::<usize>(rbx + 0x6e8).wrapping_add(b_idx * 4);
        let cur = rd::<u32>(dst_6e8);
        wr::<u32>(dst_6e8, cur.max(r9d));
    }

    if r12 > 1 {
        for i in 1..(r12 as usize) {
            let base = rd::<usize>(rbx + 0x6d8);
            let prev = rd::<u32>(base.wrapping_add((i - 1) * 4));
            let cur_p = base.wrapping_add(i * 4);
            wr::<u32>(cur_p, rd::<u32>(cur_p).wrapping_add(prev));
        }
        for i in 1..(r12 as usize) {
            let base = rd::<usize>(rbx + 0x6e8);
            let prev = rd::<u32>(base.wrapping_add((i - 1) * 4));
            let cur_p = base.wrapping_add(i * 4);
            let cur = rd::<u32>(cur_p);
            wr::<u32>(cur_p, cur.max(prev));
        }
    }

    let first_val = rd::<u32>(rd::<usize>(rdi_398)) as u64;
    let mut b_idx = (first_val / (step as u64)).wrapping_add(1);
    while b_idx < r12 {
        let p = rd::<usize>(rbx + 0x6e8).wrapping_add((b_idx as usize) * 4);
        wr::<u32>(p, rd::<u32>(p).wrapping_add(1));
        b_idx = b_idx.wrapping_add(1);
    }

    let arr_98 = rd::<usize>(g_ed0 + 0x98);
    let arr_6d8 = rd::<usize>(rbx + 0x6d8);
    let arr_6e8 = rd::<usize>(rbx + 0x6e8);
    let arr_6f8 = rd::<usize>(rbx + 0x6f8);
    let mut r14d: u32 = 0;

    if cnt_128 != 0 {
        for i in 0..(cnt_128 as usize) {
            let r10d = rd::<u32>(arr_98.wrapping_add(i * 4));
            if r10d != u32::MAX {
                let bi = (r10d / step) as usize;
                let mut r8d = rd::<u32>(arr_6d8.wrapping_add(bi * 4));
                let out_p = arr_6f8.wrapping_add(i * 4);
                wr::<u32>(out_p, r8d);
                let mut edx = rd::<u32>(arr_6e8.wrapping_add(bi * 4));
                while edx < esi_u32 {
                    let elem_p = rd::<usize>(rdi_398).wrapping_add((edx as usize) * 8);
                    if rd::<u32>(elem_p) > r10d {
                        break;
                    }
                    r8d = r8d.wrapping_add(rd::<u32>(elem_p + 0x04));
                    wr::<u32>(out_p, r8d);
                    edx = edx.wrapping_add(1);
                }
            }
        }

        let arr_6f8_2 = rd::<usize>(rbx + 0x6f8);
        for edx in 0..cnt_128 {
            if rd::<u32>(arr_6f8_2.wrapping_add((edx as usize) * 4)) != 0 {
                let arr_6c8 = rd::<usize>(rbx + 0x6c8);
                wr::<u32>(arr_6c8.wrapping_add((r14d as usize) * 4), edx);
                r14d = r14d.wrapping_add(1);
            }
        }

        if r14d != 0 {
            for i in 0..(r14d as usize) {
                let arr_6c8 = rd::<usize>(rbx + 0x6c8);
                let idx = rd::<u32>(arr_6c8.wrapping_add(i * 4));
                let arr_a8 = rd::<usize>(g_ed0 + 0xa8);
                let word_p = arr_a8.wrapping_add(((idx >> 6) as usize) * 8);
                wr::<u64>(word_p, rd::<u64>(word_p) | (1u64 << (idx & 0x3f)));
                let p_98 = rd::<usize>(g_ed0 + 0x98).wrapping_add((idx as usize) * 4);
                let sub_v = rd::<u32>(arr_6f8_2.wrapping_add((idx as usize) * 4));
                wr::<u32>(p_98, rd::<u32>(p_98).wrapping_sub(sub_v));
            }
        }
    }

    let r15d = rd::<u32>(rbx + 0x390).wrapping_sub(sum_30);
    wr::<u32>(rdi_398 + 0x08, 0);
    let rcx_arr = rd::<usize>(rdi_398);
    let cap_3a4 = rd::<u32>(rdi_398 + 0x0c) as u64;
    if rcx_arr == rcx_arr.wrapping_add((cap_3a4 as usize) * 8) {
        let new_cap = (cap_3a4 + (cap_3a4 >> 1)).max(1);
        exe::call_va!(0x1_401d_eba0, (rdi_398, new_cap));
    }
    let idx0 = rd::<u32>(rdi_398 + 0x08) as usize;
    wr::<u64>(rd::<usize>(rdi_398).wrapping_add(idx0 * 8), 0);
    let new_len1 = (idx0 as u32).wrapping_add(1);
    wr::<u32>(rdi_398 + 0x08, new_len1);
    let end1 = rd::<usize>(rdi_398).wrapping_add((new_len1 as usize) * 8);
    wr::<u32>(end1.wrapping_sub(8), r15d);
    wr::<u32>(end1.wrapping_sub(4), sum_30);

    let rdi_440 = rbx + 0x440;
    wr::<u32>(rdi_440 + 0x08, 0);
    let rcx_arr2 = rd::<usize>(rdi_440);
    let cap_44c = rd::<u32>(rdi_440 + 0x0c) as u64;
    if rcx_arr2 == rcx_arr2.wrapping_add((cap_44c as usize) * 8) {
        let new_cap = (cap_44c + (cap_44c >> 1)).max(1);
        exe::call_va!(0x1_401d_eba0, (rdi_440, new_cap));
    }
    let idx0_2 = rd::<u32>(rdi_440 + 0x08) as usize;
    wr::<u64>(rd::<usize>(rdi_440).wrapping_add(idx0_2 * 8), 0);
    let new_len2 = (idx0_2 as u32).wrapping_add(1);
    wr::<u32>(rdi_440 + 0x08, new_len2);
    let end2 = rd::<usize>(rdi_440).wrapping_add((new_len2 as usize) * 8);
    wr::<u32>(end2.wrapping_sub(8), r15d);
    wr::<u32>(end2.wrapping_sub(4), sum_30);

    let rdi_bytes = (r15d as u64).wrapping_mul(0x30);
    let mut r8_368 = rd::<u64>(rbx + 0x368);
    if rdi_bytes >= r8_368 {
        let sum = r8_368.wrapping_add(rdi_bytes).wrapping_sub(1);
        r8_368 = sum.wrapping_sub(sum % r8_368);
    }
    let quot1 = (r8_368 / rd::<u64>(rbx + 0x360)) as u32;
    if quot1 != rd::<u32>(rbx + 0x390) {
        exe::call_va!(0x1_4222_9d10, (rbx + 0x390, quot1));
    }

    let mut r8_410 = rd::<u64>(rbx + 0x410);
    if rdi_bytes >= r8_410 {
        let sum = r8_410.wrapping_add(rdi_bytes).wrapping_sub(1);
        r8_410 = sum.wrapping_sub(sum % r8_410);
    }
    let quot2 = (r8_410 / rd::<u64>(rbx + 0x408)) as u32;
    if quot2 != rd::<u32>(rbx + 0x438) {
        exe::call_va!(0x1_4222_9d10, (rbx + 0x438, quot2));
    }

    if rd::<u64>(rd::<usize>(rbx + 0x308) + 0x10) == 0 {
        return;
    }
    if rd::<u64>(rd::<usize>(rbx + 0x3b0) + 0x10) == 0 {
        return;
    }

    let arr_6f8_saved = rd::<usize>(rbx + 0x6f8);
    let r12_e0 = rd::<usize>(g_ed0 + 0xe0);
    let rdi_98 = rd::<usize>(g_ed0 + 0x98);
    let r15_6c8 = rd::<usize>(rbx + 0x6c8);

    let mut r11d: u32 = 0;
    for i in 0..(r14d as usize) {
        let idx = rd::<u32>(r15_6c8.wrapping_add(i * 4)) as usize;
        let v = rd::<u32>(r12_e0.wrapping_add(idx * 4));
        r11d = r11d.wrapping_add(v.wrapping_add(0x3f) >> 6);
    }

    if r11d != 0 {
        exe::call_va!(0x1_412f_8460, (rbx + 0x300, r11d));
        for i in 0..(r14d as usize) {
            let idx = rd::<u32>(r15_6c8.wrapping_add(i * 4)) as usize;
            let v_98 = rd::<u32>(rdi_98.wrapping_add(idx * 4));
            let v_e0 = rd::<u32>(r12_e0.wrapping_add(idx * 4));
            let v_6f8 = rd::<u32>(arr_6f8_saved.wrapping_add(idx * 4));
            let mut r9d: u32 = 0;
            while r9d < v_e0 {
                let edx_cnt = rd::<u32>(rbx + 0x350);
                let r8_dst = rd::<usize>(rbx + 0x348).wrapping_add((edx_cnt as usize) * 12);
                wr::<u32>(rbx + 0x350, edx_cnt.wrapping_add(1));
                let ecx = r9d.wrapping_add(v_98);
                wr::<u32>(r8_dst, ecx.wrapping_add(v_6f8));
                wr::<u32>(r8_dst + 0x04, ecx);
                let rem = v_e0.wrapping_sub(r9d).min(0x40);
                wr::<u32>(r8_dst + 0x08, rem);
                r9d = r9d.wrapping_add(0x40);
            }
        }
        if rd::<u32>(rbx + 0x324) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<u64>(rbx + 0x318), 0u32));
            wr::<u64>(rbx + 0x348, 0);
            wr::<u8>(rbx + 0x344, 0);
            wr::<u32>(rbx + 0x350, 0);
        }
        if rd::<u32>(rbx + 0x324) != 0 {
            exe::call_va!(
                0x1_4225_fa10,
                (
                    rd::<u64>(rbx + 0x308),
                    rd::<u64>(rbx + 0x3b0),
                    rd::<u64>(rbx + 0x318)
                )
            );
            wr::<u32>(rbx + 0x324, 0);
        }
    }

    let r12_100 = rd::<usize>(g_ed0 + 0x100);
    let r15_e0 = rd::<usize>(g_ed0 + 0xe0);
    let r13_98 = rd::<usize>(g_ed0 + 0x98);
    let rdi_6c8 = rd::<usize>(rbx + 0x6c8);

    if r14d != 0 {
        let mut r10d: u32 = 0;
        for i in 0..(r14d as usize) {
            let idx = rd::<u32>(rdi_6c8.wrapping_add(i * 4)) as usize;
            let eax = rd::<u32>(r12_100.wrapping_add(idx * 4));
            if eax != u32::MAX {
                let bits_668 = rd::<usize>(rbx + 0x668);
                let word = rd::<u64>(bits_668.wrapping_add(((eax >> 6) as usize) * 8));
                if ((word >> (eax & 0x3f)) & 1) == 0 {
                    let v = rd::<u32>(r15_e0.wrapping_add(idx * 4));
                    r10d = r10d.wrapping_add(v.wrapping_add(0x3f) >> 6);
                }
            }
        }
        if r10d != 0 {
            exe::call_va!(0x1_412f_8460, (rbx + 0x3a8, r10d));
            for i in 0..(r14d as usize) {
                let idx = rd::<u32>(rdi_6c8.wrapping_add(i * 4)) as usize;
                let eax = rd::<u32>(r12_100.wrapping_add(idx * 4));
                if eax != u32::MAX {
                    let bits_668 = rd::<usize>(rbx + 0x668);
                    let word = rd::<u64>(bits_668.wrapping_add(((eax >> 6) as usize) * 8));
                    if ((word >> (eax & 0x3f)) & 1) == 0 {
                        let v_98 = rd::<u32>(r13_98.wrapping_add(idx * 4));
                        let v_e0 = rd::<u32>(r15_e0.wrapping_add(idx * 4));
                        let mut r9d: u32 = 0;
                        while r9d < v_e0 {
                            let edx_cnt = rd::<u32>(rbx + 0x3f8);
                            let r8_dst =
                                rd::<usize>(rbx + 0x3f0).wrapping_add((edx_cnt as usize) * 12);
                            wr::<u32>(rbx + 0x3f8, edx_cnt.wrapping_add(1));
                            let eax_v = r9d.wrapping_add(v_98);
                            wr::<u32>(r8_dst, eax_v);
                            wr::<u32>(r8_dst + 0x04, eax_v);
                            let rem = v_e0.wrapping_sub(r9d).min(0x40);
                            wr::<u32>(r8_dst + 0x08, rem);
                            r9d = r9d.wrapping_add(0x40);
                        }
                    }
                }
            }
            if rd::<u32>(rbx + 0x3cc) != 0 {
                exe::call_va!(0x1_4209_d6f0, (rd::<u64>(rbx + 0x3c0), 0u32));
                wr::<u64>(rbx + 0x3f0, 0);
                wr::<u8>(rbx + 0x3ec, 0);
                wr::<u32>(rbx + 0x3f8, 0);
            }
            if rd::<u32>(rbx + 0x3cc) != 0 {
                exe::call_va!(
                    0x1_4225_fa10,
                    (
                        rd::<u64>(rbx + 0x3b0),
                        rd::<u64>(rbx + 0x308),
                        rd::<u64>(rbx + 0x3c0)
                    )
                );
                wr::<u32>(rbx + 0x3cc, 0);
            }
        }
    }
}

/// Plain (non-TSO) load/store: `r_14225be60` fills a render-thread-private
/// draw list from per-frame instance arrays (no cross-thread handoff inside).
#[inline(always)]
unsafe fn pl<T: Copy>(addr: usize) -> T {
    std::ptr::read_unaligned(addr as *const T)
}

#[inline(always)]
unsafe fn ps<T: Copy>(addr: usize, v: T) {
    std::ptr::write_unaligned(addr as *mut T, v)
}

/// `r_14225be60`
/// Original: 0x14225be60 (370 bytes, Throws::No)
pub unsafe extern "win64" fn r_14225be60(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let rcx_bits = pl::<usize>(param_1);
    let r12 = pl::<usize>(param_2 + 0x08);
    let word_cnt = (pl::<u64>(param_2).wrapping_add(0x3f)) >> 6;
    if word_cnt == 0 {
        return;
    }

    let rbp = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let mut rsi: u64 = 0;
    for r15 in 0..(word_cnt as usize) {
        let mut rax = pl::<u64>(rcx_bits.wrapping_add(r15 * 8));
        while rax != 0 {
            let rbx = rax.trailing_zeros() as u64;
            let idx = rbx.wrapping_add(rsi) as usize;
            let edx = pl::<u32>(r12.wrapping_add(idx * 8)) as usize;
            let arr_f0 = pl::<usize>(rbp + 0xf0);
            let r10d = pl::<u32>(arr_f0.wrapping_add(edx * 4));
            if r10d != 0 {
                let arr_98 = pl::<usize>(rbp + 0x98);
                let edi = pl::<u32>(arr_98.wrapping_add(edx * 4));
                let arr_d0 = pl::<usize>(rbp + 0xd0);
                let mut r9d = pl::<u32>(arr_d0.wrapping_add(edx * 4));
                let old_40 = pl::<u32>(param_3 + 0x40);
                ps::<u32>(param_3 + 0x40, old_40.wrapping_add(r10d));
                let mut r8 = pl::<usize>(param_3 + 0x38)
                    .wrapping_add((old_40 as usize) * 0x38)
                    .wrapping_add(0x28);
                let diff_edi = edi.wrapping_sub(r9d);
                for _ in 0..r10d {
                    ps::<u32>(r8.wrapping_sub(0x28), r9d.wrapping_add(diff_edi));
                    r8 = r8.wrapping_add(0x38);
                    let src_idx = r9d as usize;
                    r9d = r9d.wrapping_add(1);
                    let src_base = pl::<usize>(param_4).wrapping_add(src_idx * 0x30);
                    let xmm0 = pl::<u128>(src_base + 0x10);
                    let xmm4 = pl::<u128>(src_base + 0x20);
                    let xmm3 = pl::<u128>(src_base);
                    let xmm1 = crate::simd_ops::vshufps(xmm0, xmm4, 0x9e);
                    let xmm2 = crate::simd_ops::vshufps(xmm3, xmm0, 0x49);
                    let out0 = crate::simd_ops::vshufps(xmm3, xmm1, 0x8c);
                    ps::<u128>(r8.wrapping_sub(0x58), out0);
                    let out2 = crate::simd_ops::vshufps(xmm2, xmm4, 0xcd);
                    let out1 = crate::simd_ops::vshufps(xmm2, xmm1, 0xd8);
                    ps::<u128>(r8.wrapping_sub(0x38), out2);
                    ps::<u128>(r8.wrapping_sub(0x48), out1);
                }
            }
            rax &= !(1u64 << rbx);
        }
        rsi = rsi.wrapping_add(0x40);
    }
}

/// Copy a 4x3 `f32` matrix (12 dwords at `src`) transposed into three 4-dword rows at
/// `dst`, `dst + 0x10`, `dst + 0x20`: the original's five-`vshufps` sequence. Bit copies
/// only (no float ops), unaligned on both sides.
#[inline(always)]
unsafe fn transpose_4x3_rows(src: usize, dst: usize) {
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::{vld3q_u32, vst1q_u32};
        let v = vld3q_u32(src as *const u32);
        vst1q_u32(dst as *mut u32, v.0);
        vst1q_u32((dst + 0x20) as *mut u32, v.2);
        vst1q_u32((dst + 0x10) as *mut u32, v.1);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let xmm0 = pl::<u128>(src + 0x10);
        let xmm4 = pl::<u128>(src + 0x20);
        let xmm3 = pl::<u128>(src);
        let xmm1 = crate::simd_ops::vshufps(xmm0, xmm4, 0x9e);
        let xmm2 = crate::simd_ops::vshufps(xmm3, xmm0, 0x49);
        ps::<u128>(dst, crate::simd_ops::vshufps(xmm3, xmm1, 0x8c));
        ps::<u128>(dst + 0x20, crate::simd_ops::vshufps(xmm2, xmm4, 0xcd));
        ps::<u128>(dst + 0x10, crate::simd_ops::vshufps(xmm2, xmm1, 0xd8));
    }
}

/// `r_14225bfe0`: append one `{u32 index; pad; 3 x float4}` entry (0x38 bytes) per set bit
/// of a bitset to a render-thread draw list, the transform being the indexed 0x30-byte
/// 4x3 matrix stored transposed (as in [`r_14225be60`]).
///
/// `bits_holder` points at the bitset's word pointer, `bit_count` is its length in bits,
/// `list` is the draw list (`+0x38` entry array, `+0x40` entry count, `+0x20` entries this
/// call appends, added to the count up front), `mats_holder` points at the matrix array
/// pointer (re-read per entry, as the original does).
/// Original: 0x14225bfe0 (253 bytes, leaf, Throws::No); sole caller 0x1422620f0.
pub unsafe extern "win64" fn r_14225bfe0(bits_holder: usize, bit_count: u64, list: usize, mats_holder: usize) {
    let words = pl::<usize>(bits_holder);
    let word_cnt = bit_count.wrapping_add(0x3f) >> 6;
    let start = pl::<u32>(list + 0x40);
    let adding = pl::<u32>(list + 0x20);
    let mut entry = (start as usize).wrapping_mul(0x38).wrapping_add(pl::<usize>(list + 0x38));
    ps::<u32>(list + 0x40, adding.wrapping_add(start));
    for w in 0..word_cnt as usize {
        let mut bits = pl::<u64>(words.wrapping_add(w * 8));
        while bits != 0 {
            let b = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            let idx = (w * 64).wrapping_add(b);
            ps::<u32>(entry, idx as u32);
            let src = pl::<usize>(mats_holder).wrapping_add(idx.wrapping_mul(0x30));
            transpose_4x3_rows(src, entry + 8);
            entry = entry.wrapping_add(0x38);
        }
    }
}

/// `unknown_142303ab0`
/// Original: 0x142303ab0 (494 bytes, Throws::Caught)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_142303ab0(param_1: usize, param_2: f32) {
    unknown_142303ab0_impl(param_1, param_2);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_142303ab0(param_1: usize, _xmm0: f32, param_2: f32) {
    unknown_142303ab0_impl(param_1, param_2);
}

#[inline(always)]
unsafe fn unknown_142303ab0_impl(param_1: usize, param_2: f32) {
    let rdi = rd::<usize>(param_1 + 0x08);
    let mut ebx: i32 = 0;
    if rd::<i32>(rdi + 0x138) <= 0 {
        return;
    }

    let rbp = rd::<usize>(exe::tls_pointer());
    let g_b78 = exe::ptr::<i32>(0x1_43cf_8b78) as usize;
    let g_b80 = exe::ptr::<u32>(0x1_43cf_8b80) as usize;
    let s_emiss = exe::ptr::<u8>(0x1_433f_a688) as usize as u64;

    loop {
        let tls_epoch = rd::<i32>(rbp + 0x74);
        if rd::<i32>(g_b78) > tls_epoch {
            exe::call_va!(0x1_4183_4a78, (g_b78));
            if rd::<i32>(g_b78) == -1 {
                let mut sv: [u64; 2] = [s_emiss, 0x11];
                exe::call_va!(0x1_4220_ea70, (g_b80, sv.as_mut_ptr() as usize));
                exe::call_va!(0x1_4183_4a0c, (g_b78));
            }
        }

        let b4f = rd::<u8>(param_1 + 0x4f);
        let arr_base = if b4f != 0 && rd::<u32>(rdi + 0x178) != 0 {
            let p170 = rd::<usize>(rdi + 0x170);
            rd::<usize>(p170.wrapping_add((b4f as usize) * 0x18).wrapping_sub(0x10))
        } else {
            rd::<usize>(rdi + 0x160)
        };

        let elem_ptr = rd::<usize>(arr_base.wrapping_add((ebx as isize as usize) * 8));
        let rcx_c0 = rd::<usize>(elem_ptr + 0xc0);
        let r9 = rd::<usize>(rcx_c0);
        let r8 = rd::<usize>(r9);
        let edx_len = rd::<u32>(r9 + 0x08) as usize;
        let mut found_ptr: usize = 0;
        if edx_len != 0 {
            let want_id = rd::<u32>(g_b80);
            for i in 0..edx_len {
                if rd::<u32>(r8.wrapping_add(i * 4)) == want_id {
                    found_ptr = rd::<usize>(r9 + 0x10).wrapping_add(i * 0x18);
                    break;
                }
            }
        }

        let xmm6 = rd::<f32>(found_ptr + 0x08);
        let mut pair: [usize; 2] = [0; 2];
        exe::call_va!(
            0x1_4230_3160,
            (param_1, pair.as_mut_ptr() as usize, ebx as u32)
        );
        let rax_30 = pair[0];
        let want_id = rd::<u32>(g_b80);
        let rax_prop = exe::call_va!(0x1_4131_1210, (rd::<usize>(rax_30), want_id) -> usize);
        wr::<u32>(rax_prop, 0);
        wr::<f32>(rax_prop + 0x08, param_2 * xmm6);

        let r9_c98 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6c98) as usize);
        if r9_c98 != 0 {
            let rax_38 = pair[1];
            let r8d = rd::<u16>(rax_38 + 0x20) as usize;
            atomic_or_u8(rax_38 + 0x38, 1);
            let bit = (r8d as u32) & 0x3f;
            let word_idx = r8d >> 6;
            atomic_or_u64(
                r9_c98.wrapping_add(0x78).wrapping_add(word_idx * 8),
                1u64 << bit,
            );
        }

        ebx = ebx.wrapping_add(1);
        if ebx >= rd::<i32>(rdi + 0x138) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn run_upload_helper(handle: u64, src_data: u64, total_bytes: u64) {
    let mut ctx = [0u8; 0x80];
    let s = ctx.as_mut_ptr() as usize;
    wr::<u64>(s, handle);
    wr::<u64>(s + 0x08, 0);
    wr::<u64>(s + 0x10, total_bytes);
    wr::<u64>(s + 0x18, 0);
    wr::<u32>(s + 0x74, 0);
    wr::<u64>(s + 0x78, 0);

    exe::call_va!(0x1_411d_d860, (s, src_data, total_bytes));
    let pending = rd::<u64>(s + 0x78);
    if pending != 0 {
        let cur_off = rd::<u64>(s + 0x08);
        let rem = rd::<u64>(s + 0x10).wrapping_sub(cur_off);
        let chunk = pending.min(rem);
        exe::call_va!(0x1_4205_de40, ());
        wr::<u32>(s + 0x74, 0);
        wr::<u64>(s + 0x78, 0);
        let flag = rd::<u32>(s + 0x70);
        exe::call_va!(
            0x1_4207_5bc0,
            (rd::<u64>(s), cur_off, s + 0x18, flag, chunk)
        );
        wr::<u64>(s + 0x08, cur_off.wrapping_add(chunk));
    }
    wr::<u64>(s, 0);
}

/// `rend_142336540`
/// Original: 0x142336540 (1523 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_142336540(param_1: usize) {
    let edi = rd::<u32>(param_1 + 0x08);
    let r14 = edi as u64;
    let r8_bytes = r14.wrapping_mul(8);
    let r13_bytes = r14.wrapping_mul(0x38);

    run_upload_helper(rd::<u64>(param_1 + 0x50), rd::<u64>(param_1), r8_bytes);

    let r12_vec = param_1.wrapping_add(0x20);
    if rd::<u8>(param_1 + 0x30) != 0 {
        let mut rsi_buf: usize = 0;
        if r14 != 0 {
            rsi_buf = exe::call_va!(0x1_4241_c0a0, (r14 * 4, 4u32) -> usize);
            if r14 != 0 {
                let _ = exe::call_va!(0x1_4183_acc2, (rsi_buf, 0u32, r14 * 4) -> usize);
            }
        }
        for i in 0..edi {
            wr::<u32>(rsi_buf.wrapping_add((i as usize) * 4), i);
        }
        let mut p_end = rsi_buf.wrapping_add((edi as usize) * 4);
        let mut p_begin = rsi_buf;
        let diff_elems = ((p_end.wrapping_sub(rsi_buf)) as isize) >> 2;
        exe::call_va!(
            0x1_4133_fc70,
            (
                (&mut p_begin as *mut usize) as usize,
                (&mut p_end as *mut usize) as usize,
                diff_elems as u64,
                param_1
            )
        );

        let mut cur_off: u32 = 0;
        if edi != 0 {
            for i in 0..(r14 as usize) {
                let elem_idx = rd::<u32>(rsi_buf.wrapping_add(i * 4)) as usize;
                let rbx_entry = rd::<usize>(param_1 + 0x10).wrapping_add(elem_idx * 0x38);
                let len_34 = rd::<u32>(rbx_entry + 0x34) as u64;
                let off_30 = rd::<u32>(rbx_entry + 0x30) as usize;
                let vec_base = rd::<usize>(r12_vec);
                let _ = exe::call_va!(
                    0x1_4183_acc8,
                    (
                        vec_base.wrapping_add(cur_off as usize),
                        vec_base.wrapping_add(off_30),
                        len_34
                    ) -> usize
                );
                wr::<u32>(rbx_entry + 0x30, cur_off);
                cur_off = cur_off.wrapping_add(rd::<u32>(rbx_entry + 0x34));
            }
        }
        exe::call_va!(0x1_4028_e490, (r12_vec, cur_off));
        wr::<u8>(param_1 + 0x30, 0);
        if rsi_buf != 0 {
            exe::call_va!(0x1_4241_c190, (rsi_buf));
        }
    }

    let size_28 = rd::<u32>(param_1 + 0x28) as u64;
    run_upload_helper(
        rd::<u64>(param_1 + 0x58),
        rd::<u64>(r12_vec),
        size_28,
    );

    run_upload_helper(
        rd::<u64>(param_1 + 0x60),
        rd::<u64>(param_1 + 0x10),
        r13_bytes,
    );

    let r12_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let rsi_cb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let rdi_default = rd::<u64>(rsi_cb8 + 0x88);

    let tid1 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let g_lock = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let g_tid = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let g_cnt = exe::ptr::<u32>(0x1_4385_8484) as usize;
    if rd::<u32>(g_tid) != tid1 {
        exe::call_iat!(0x1_4264_55a8, (g_lock));
        wr::<u32>(g_tid, tid1);
    }
    wr::<u32>(g_cnt, rd::<u32>(g_cnt).wrapping_add(1));

    let rbx_lock = rsi_cb8.wrapping_add(0x20);
    let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx_lock + 0x08) != tid2 {
        exe::call_iat!(0x1_4264_55a8, (rbx_lock));
        wr::<u32>(rbx_lock + 0x08, tid2);
    }
    let old_c = rd::<u32>(rbx_lock + 0x0c);
    wr::<u32>(rbx_lock + 0x0c, old_c.wrapping_add(1));
    let g_ca8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fca8) as usize);
    let r14_val = rd::<u64>(g_ca8 + 0x08);
    let rsi_280 = rd::<u64>(rsi_cb8 + 0x280);
    wr::<u32>(rbx_lock + 0x0c, old_c);
    if old_c == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((rbx_lock + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx_lock));
    }

    let new_g_cnt = rd::<u32>(g_cnt).wrapping_sub(1);
    wr::<u32>(g_cnt, new_g_cnt);
    if new_g_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32(g_tid as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (g_lock));
    }

    let r10d_step = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize);
    let word_cnt = (edi >> 6) as usize;
    let r9b_force = rd::<u8>(exe::ptr::<u8>(0x1_4386_c758) as usize);

    if word_cnt != 0 {
        let mut r8_base: u64 = 0;
        for rax_idx in 0..word_cnt {
            let word_addr = param_1.wrapping_add(0x2078).wrapping_add(rax_idx * 8);
            let mut rdx_bits = crate::simd_ops::atomic_xchg_u64(word_addr as u64, 0);
            if r9b_force != 0 {
                rdx_bits = u64::MAX;
            }
            if rdx_bits != 0 {
                let step_i64 = (r10d_step as i32) as i64 as u64;
                let step_u64 = r10d_step as u64;
                while rdx_bits != 0 {
                    let bit = rdx_bits.trailing_zeros() as u64;
                    let idx = r8_base.wrapping_add(bit);
                    rdx_bits &= !(1u64 << bit);
                    let mut r13_ptr = rd::<usize>(param_1 + 0x38).wrapping_add((idx as usize) << 7);
                    let idx_shl4 = ((idx as u32) << 4) as i32 as i64 as u64;
                    let mut rbx_dst = rsi_280.wrapping_add(idx_shl4.wrapping_mul(step_u64));
                    for _ in 0..16 {
                        let elem_ptr = rd::<usize>(r13_ptr);
                        let vtbl = rd::<usize>(r12_dev);
                        let fn_c0 = rd::<usize>(vtbl + 0xc0);
                        let r9_arg = if elem_ptr != 0 {
                            rd::<u64>(elem_ptr + 0x78)
                        } else {
                            rdi_default
                        };
                        exe::call_va!(fn_c0, (r12_dev, 1u32, rbx_dst, r9_arg, 0u32));
                        rbx_dst = rbx_dst.wrapping_add(step_i64);
                        r13_ptr = r13_ptr.wrapping_add(8);
                    }
                }
            }
            r8_base = r8_base.wrapping_add(0x40);
        }
    }

    let vtbl = rd::<usize>(r12_dev);
    let fn_c0 = rd::<usize>(vtbl + 0xc0);
    exe::call_va!(fn_c0, (r12_dev, edi << 4, r14_val, rsi_280, 0u32));
    wr::<u8>(exe::ptr::<u8>(0x1_4386_c758) as usize, 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4386_c754) as usize, 1);
}

/// `rend_142338090`
/// Original: 0x142338090 (117 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142338090(_param_1: usize, param_2: usize) -> usize {
    let rax_ed8 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6ed8) as usize);
    let r10 = rd::<usize>(rax_ed8);
    let want = rd::<u32>(param_2);
    let rcx = rd::<usize>(r10);
    let r8 = rd::<u32>(rcx + 0x08) as usize;
    let r9 = rd::<usize>(rcx);
    for rdx in 0..r8 {
        if rd::<u32>(r9.wrapping_add(rdx * 4)) == want {
            let entry = rd::<usize>(rcx + 0x10).wrapping_add(rdx * 0x18);
            if entry == 0 || rd::<u32>(entry) != 4 {
                return 0;
            }
            let idx = rd::<u32>(entry + 0x08) as usize;
            if idx < (rd::<u32>(r10 + 0x48) as usize) {
                let v = rd::<usize>(rd::<usize>(r10 + 0x40).wrapping_add(idx * 8));
                if v != 0 {
                    return v;
                }
            }
            let v2 = rd::<usize>(rd::<usize>(r10 + 0x30).wrapping_add(idx * 8));
            if v2 == 0 {
                return 0;
            }
            return rd::<usize>(v2 + 0x1a0);
        }
    }
    0
}

/// `rend_14233e9d0`
/// Original: 0x14233e9d0 (273 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233e9d0(param_1: usize) -> u64 {
    let mut local_res10 = exe::call_va!(0x1_4233_f070, (param_1) -> u64);
    let rdi = ((local_res10 as i32) as isize as usize).wrapping_shl(5);
    let mut rbp: usize = 0;
    let want_id = (local_res10 >> 32) as u32;
    let rax_fa8 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6fa8) as usize);
    let rsi = rd::<usize>(rax_fa8);
    let table = rsi.wrapping_add(rdi);
    let rdx_len = rd::<u32>(table + 0x08) as usize;
    let r8_keys = rd::<usize>(table);
    let mut found_first = false;
    for i in 0..rdx_len {
        if rd::<u32>(r8_keys.wrapping_add(i * 4)) == want_id {
            if rd::<usize>(table + 0x10).wrapping_add(i * 0x18) != 0 {
                found_first = true;
            }
            break;
        }
    }
    if !found_first {
        let local_ptr = (&mut local_res10 as *mut u64) as usize;
        let name_ptr = exe::call_va!(0x1_411a_4fb0, (local_ptr + 0x04) -> u64);
        let fmt = exe::ptr::<u8>(0x1_433f_e540) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, fmt, name_ptr));
        return 0;
    }

    exe::call_va!(
        0x1_4233_eec0,
        (param_1, (&mut local_res10 as *mut u64) as usize, 0u32)
    );
    let rdx_len2 = rd::<u32>(table + 0x08) as usize;
    let r8_keys2 = rd::<usize>(table);
    if rdx_len2 != 0 {
        let want_id2 = (local_res10 >> 32) as u32;
        for i in 0..rdx_len2 {
            if rd::<u32>(r8_keys2.wrapping_add(i * 4)) == want_id2 {
                rbp = rd::<usize>(table + 0x10).wrapping_add(i * 0x18);
                break;
            }
        }
    }

    let out_slot = rd::<usize>(param_1 + 0x08);
    let fval = rd::<f32>(rbp + 0x08);
    wr::<u32>(out_slot + 0x10, 3);
    wr::<f64>(out_slot, fval as f64);
    wr::<usize>(param_1 + 0x08, rd::<usize>(param_1 + 0x08).wrapping_add(0x18));
    1
}

/// `unknown_142350560`
/// Original: 0x142350560 (1797 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142350560(param_1: usize, param_2: usize) -> u8 {
    wr::<u32>(param_1 + 0x18, 0);
    wr::<u32>(param_1 + 0x28, 0);
    wr::<u64>(param_1 + 0x30, 0);
    wr::<u32>(param_1 + 0x40, 0);
    wr::<u64>(param_1 + 0x48, 0);
    wr::<u32>(param_1 + 0x58, 0);
    wr::<u64>(param_1 + 0x60, 0);
    wr::<u32>(param_1 + 0x70, 0);
    wr::<u32>(param_1 + 0x80, 0);
    wr::<u32>(param_1 + 0x90, 0);
    wr::<u32>(param_1 + 0xa0, 0);

    let sub_a8 = param_1 + 0xa8;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_a8) + 0x18), (sub_a8));

    wr::<u32>(param_1 + 0x180, 0);
    let r13_188 = param_1 + 0x188;
    exe::call_va!(rd::<usize>(rd::<usize>(r13_188) + 0x18), (r13_188));

    wr::<u32>(param_1 + 0x260, 0);
    wr::<u32>(param_1 + 0x270, 0);
    wr::<u32>(param_1 + 0x280, 0);
    wr::<u32>(param_1 + 0x290, 0);
    wr::<u16>(param_1 + 0x298, 0);

    wr::<u128>(param_1, rd::<u128>(param_2));
    wr::<u64>(param_1 + 0x10, rd::<u64>(param_2 + 0x10));

    let esi = rd::<i32>(param_1);
    if esi == 0 {
        wr::<u64>(param_1, 0);
        wr::<u64>(param_1 + 0x08, 0);
        wr::<u64>(param_1 + 0x10, 0);
        return 0;
    }
    let ebp = rd::<i32>(param_1 + 0x04);
    if ebp == 0 {
        wr::<u64>(param_1, 0);
        wr::<u64>(param_1 + 0x08, 0);
        wr::<u64>(param_1 + 0x10, 0);
        return 0;
    }

    let num_tris = (ebp.wrapping_mul(esi) as u32).wrapping_mul(2);
    let r14d = num_tris.wrapping_add(0x3ff) & 0xffff_fc00;
    wr::<u32>(param_1 + 0x18, num_tris);

    let sub_178 = param_1 + 0x178;
    let rbx_u64 = r14d as u64;
    exe::call_va!(0x1_4134_d670, (sub_178, rbx_u64));
    exe::call_va!(rd::<usize>(rd::<usize>(sub_a8) + 0x28), (sub_a8, rbx_u64));

    let mut init_f32: u32 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6f78) as usize);
    exe::call_va!(
        0x1_412d_ebd0,
        (param_1 + 0x68, rbx_u64, (&mut init_f32 as *mut u32) as usize)
    );
    init_f32 = 0;
    exe::call_va!(
        0x1_412d_ebd0,
        (param_1 + 0x78, rbx_u64, (&mut init_f32 as *mut u32) as usize)
    );

    let r15_88 = param_1 + 0x88;
    let r12_len = rd::<u32>(r15_88 + 0x08) as u64;
    let rcx_buf = rd::<usize>(r15_88);
    let cap_88 = rd::<u32>(r15_88 + 0x0c) as u64;
    if rbx_u64 > cap_88 {
        exe::call_va!(0x1_402d_7910, (r15_88, rbx_u64));
        let dst = rd::<usize>(r15_88).wrapping_add(rd::<u32>(r15_88 + 0x08) as usize);
        let diff = rbx_u64.wrapping_sub(r12_len);
        if diff != 0 {
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff) -> usize);
        }
        wr::<u32>(r15_88 + 0x08, r14d);
    } else if rbx_u64 > r12_len {
        let dst = rcx_buf.wrapping_add(r12_len as usize);
        let diff = rbx_u64.wrapping_sub(r12_len);
        if diff != 0 {
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff) -> usize);
        }
        wr::<u32>(r15_88 + 0x08, r14d);
    } else if rbx_u64 != r12_len {
        wr::<u32>(r15_88 + 0x08, r14d);
    }

    let g_f728 = exe::ptr::<u8>(0x1_42c1_f728) as usize;
    exe::call_va!(0x1_412e_b5b0, (param_1 + 0x98, rbx_u64, g_f728));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x20, rbx_u64));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x38, rbx_u64));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x50, rbx_u64));

    let num_verts = ((ebp + 1).wrapping_mul(esi + 1)) as i64 as u64;
    exe::call_va!(rd::<usize>(rd::<usize>(r13_188) + 0x28), (r13_188, num_verts));
    wr::<u16>(param_1 + 0x298, 0x0101);

    let c30 = rd::<u64>(param_1 + 0x30);
    let mut desc1: [u64; 3] = [c30.wrapping_add(0x3f) >> 6, rd::<u64>(param_1 + 0x20), c30];
    exe::call_va!(
        0x1_40ff_f0a0,
        (desc1.as_mut_ptr() as usize, 0u32, num_tris as u64, 1u8)
    );

    let extra = (r14d.wrapping_sub(num_tris)) >> 1;
    exe::call_va!(0x1_4134_dcc0, (param_1 + 0x268, extra as u64));
    if extra != 0 {
        let mut r8d: u32 = 0;
        for i in 0..(extra as usize) {
            let edx = r14d.wrapping_sub(r8d);
            let lo = edx.wrapping_sub(2);
            let hi = edx.wrapping_sub(1);
            let val = (lo as u64) | ((hi as u64) << 32);
            wr::<u64>(rd::<usize>(param_1 + 0x268).wrapping_add(i * 8), val);
            r8d = r8d.wrapping_add(2);
        }
    }

    let v_cnt = exe::call_va!(rd::<usize>(rd::<usize>(r13_188) + 0x38), (r13_188) -> u64);
    let c240 = rd::<u64>(param_1 + 0x240);
    let mut desc2: [u64; 3] = [
        c240.wrapping_add(0x3f) >> 6,
        rd::<u64>(param_1 + 0x230),
        c240,
    ];
    exe::call_va!(
        0x1_40ff_f0a0,
        (desc2.as_mut_ptr() as usize, 0u32, v_cnt, 1u8)
    );

    let pos_arr = rd::<usize>(param_1 + 0x220);
    let t_cnt = exe::call_va!(rd::<usize>(rd::<usize>(sub_a8) + 0x38), (sub_a8) -> u64);
    let c160 = rd::<u64>(param_1 + 0x160);
    let mut desc3: [u64; 3] = [
        c160.wrapping_add(0x3f) >> 6,
        rd::<u64>(param_1 + 0x150),
        c160,
    ];
    exe::call_va!(
        0x1_40ff_f0a0,
        (desc3.as_mut_ptr() as usize, 0u32, t_cnt, 1u8)
    );

    let idx_arr = rd::<usize>(param_1 + 0x140);
    if ebp > 0 {
        let one_f32 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        for r11d in 0..ebp {
            let next_y = r11d + 1;
            if esi > 0 {
                let row_tri_base = r11d.wrapping_mul(esi);
                let row0_v_base = (esi + 1).wrapping_mul(r11d);
                let row1_v_base = (esi + 1).wrapping_mul(next_y);
                let xmm9 = esi as f32;
                let xmm2 = ebp as f32;
                let xmm7 = (r11d as f32) / xmm2;
                let xmm8 = (next_y as f32) / xmm2;
                let xmm11 = one_f32 - xmm7;
                let xmm10 = one_f32 - xmm8;
                let y_parity = (r11d as u32) & 1;

                for r9d in 0..esi {
                    let r14_tri0 = (row_tri_base.wrapping_add(r9d) as u32).wrapping_mul(2);
                    let tri1 = r14_tri0.wrapping_add(1);
                    let adj_arr = rd::<usize>(sub_178);
                    let r15_adj1 = adj_arr.wrapping_add((tri1 as usize) * 0x10);

                    let v00 = (row0_v_base.wrapping_add(r9d)) as u32;
                    let v10 = v00.wrapping_add(1);
                    let v01 = (row1_v_base.wrapping_add(r9d)) as u32;
                    let v11 = v01.wrapping_add(1);

                    let xmm4 = (r9d as f32) / xmm9;
                    let next_x = r9d + 1;
                    let xmm5 = (next_x as f32) / xmm9;
                    let xmm3 = one_f32 - xmm4;
                    let xmm2_x = one_f32 - xmm5;

                    let min_x = rd::<f32>(param_1 + 0x08);
                    let max_x = rd::<f32>(param_1 + 0x10);
                    wr::<f32>(
                        pos_arr.wrapping_add((v00 as usize) * 8),
                        xmm3 * min_x + xmm4 * max_x,
                    );
                    wr::<f32>(
                        pos_arr.wrapping_add((v10 as usize) * 8),
                        xmm2_x * min_x + xmm5 * max_x,
                    );
                    wr::<f32>(
                        pos_arr.wrapping_add((v01 as usize) * 8),
                        xmm3 * min_x + xmm4 * max_x,
                    );
                    wr::<f32>(
                        pos_arr.wrapping_add((v11 as usize) * 8),
                        xmm2_x * min_x + xmm5 * max_x,
                    );

                    let min_y = rd::<f32>(param_1 + 0x0c);
                    let max_y = rd::<f32>(param_1 + 0x14);
                    wr::<f32>(
                        pos_arr.wrapping_add((v00 as usize) * 8 + 4),
                        xmm11 * max_y + xmm7 * min_y,
                    );
                    wr::<f32>(
                        pos_arr.wrapping_add((v10 as usize) * 8 + 4),
                        xmm7 * min_y + xmm11 * max_y,
                    );
                    wr::<f32>(
                        pos_arr.wrapping_add((v01 as usize) * 8 + 4),
                        xmm10 * max_y + xmm8 * min_y,
                    );
                    wr::<f32>(
                        pos_arr.wrapping_add((v11 as usize) * 8 + 4),
                        xmm8 * min_y + xmm10 * max_y,
                    );

                    let tri0_idx_p = idx_arr.wrapping_add((r14_tri0 as usize) * 12);
                    let tri1_idx_p = idx_arr.wrapping_add((tri1 as usize) * 12);
                    let tri0_adj_p = adj_arr.wrapping_add((r14_tri0 as usize) * 0x10);

                    let (c_adj1_0, a_adj1_2) = if ((r9d as u32) & 1) == y_parity {
                        wr::<u32>(tri0_idx_p, v00);
                        wr::<u32>(tri0_idx_p + 0x04, v10);
                        wr::<u32>(tri0_idx_p + 0x08, v01);
                        wr::<u32>(tri1_idx_p, v11);
                        wr::<u32>(tri1_idx_p + 0x04, v01);
                        wr::<u32>(tri1_idx_p + 0x08, v10);

                        let a0 = if r11d == 0 {
                            0xffff_fffe
                        } else {
                            (((r11d - 1).wrapping_mul(esi).wrapping_add(r9d)) as u32)
                                .wrapping_mul(2)
                        };
                        let c0 = if r9d == 0 {
                            0xffff_fffe
                        } else {
                            r14_tri0.wrapping_sub(1)
                        };
                        wr::<u32>(tri0_adj_p, a0);
                        wr::<u32>(tri0_adj_p + 0x04, tri1);
                        wr::<u32>(tri0_adj_p + 0x08, c0);

                        let c1 = if r11d == ebp - 1 {
                            0xffff_fffe
                        } else {
                            ((next_y.wrapping_mul(esi).wrapping_add(r9d)) as u32)
                                .wrapping_mul(2)
                                .wrapping_add(1)
                        };
                        let a1 = if r9d == esi - 1 {
                            0xffff_fffe
                        } else {
                            r14_tri0.wrapping_add(2)
                        };
                        (c1, a1)
                    } else {
                        wr::<u32>(tri0_idx_p, v01);
                        wr::<u32>(tri0_idx_p + 0x04, v00);
                        wr::<u32>(tri0_idx_p + 0x08, v11);
                        wr::<u32>(tri1_idx_p, v10);
                        wr::<u32>(tri1_idx_p + 0x04, v11);
                        wr::<u32>(tri1_idx_p + 0x08, v00);

                        let c0 = if r9d == 0 {
                            0xffff_fffe
                        } else {
                            r14_tri0.wrapping_sub(1)
                        };
                        let a0 = if r11d == ebp - 1 {
                            0xffff_fffe
                        } else {
                            ((next_y.wrapping_mul(esi).wrapping_add(r9d)) as u32).wrapping_mul(2)
                        };
                        wr::<u32>(tri0_adj_p, c0);
                        wr::<u32>(tri0_adj_p + 0x04, tri1);
                        wr::<u32>(tri0_adj_p + 0x08, a0);

                        let c1 = if r9d == esi - 1 {
                            0xffff_fffe
                        } else {
                            r14_tri0.wrapping_add(2)
                        };
                        let a1 = if r11d == 0 {
                            0xffff_fffe
                        } else {
                            (((r11d - 1).wrapping_mul(esi).wrapping_add(r9d)) as u32)
                                .wrapping_mul(2)
                                .wrapping_add(1)
                        };
                        (c1, a1)
                    };

                    wr::<u32>(r15_adj1, c_adj1_0);
                    wr::<u32>(r15_adj1 + 0x04, r14_tri0);
                    wr::<u32>(r15_adj1 + 0x08, a_adj1_2);
                }
            }
        }
    }
    1
}

/// `unknown_1423e6ef0`
/// Original: 0x1423e6ef0 (314 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1423e6ef0(param_1: usize, param_2: u64) {
    if param_2 == 0 {
        return;
    }

    let mut rsi = param_1;
    let mut rem = param_2;
    let rdi = rd::<usize>(exe::ptr::<usize>(0x1_43a0_0230) as usize).wrapping_add(0x100);
    let r13 = exe::ptr::<usize>(0x1_43be_60a8) as usize;
    let mut r8d = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize);

    loop {
        let r15 = rd::<u64>(rsi + 0x08);
        let eax = rd::<u32>(rsi);
        let hi = (eax >> 16) as usize;
        let lo = (eax as u16) as usize;
        let rbx = rd::<usize>(r13.wrapping_add(hi * 8)).wrapping_add(lo * 0x98);

        let r9 = rd::<usize>(rdi);
        let len = rd::<u32>(rdi + 0x08) as usize;
        let r10 = (len * 0x18) as i64;
        let end = r9.wrapping_add(len * 0x18);
        let mut cur = r9;
        let mut matched = false;
        while cur != end {
            if rd::<usize>(cur) == rbx {
                wr::<i64>(cur + 0x10, r8d as i64);
                wr::<u64>(cur + 0x08, r15);
                matched = true;
                break;
            }
            cur = cur.wrapping_add(0x18);
        }

        if !matched && rbx != 0 {
            let rbp = r8d as i64;
            let cap = rd::<u32>(rdi + 0x0c) as usize;
            if end == r9.wrapping_add(cap * 0x18) {
                let new_cap = ((r10 / 0x18) + 1) as u64;
                exe::call_va!(0x1_411d_ba80, (rdi, new_cap));
                r8d = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize);
            }
            let idx = rd::<u32>(rdi + 0x08) as usize;
            let dst = rd::<usize>(rdi).wrapping_add(idx * 0x18);
            wr::<usize>(dst, rbx);
            wr::<u64>(dst + 0x08, r15);
            wr::<i64>(dst + 0x10, rbp);
            wr::<u32>(rdi + 0x08, (idx as u32).wrapping_add(1));
            wr::<u32>(rbx + 0x4c, rd::<u32>(rbx + 0x4c) | 0x2000);
        }

        rsi = rsi.wrapping_add(0x10);
        rem -= 1;
        if rem == 0 {
            break;
        }
    }
}

/// `r_14241d340`
/// Original: 0x14241d340 (385 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14241d340(param_1: usize, param_2: u64, param_3: u64) -> usize {
    let mut key_slot: u64 = param_2;
    let rbx_lock = param_1.wrapping_add(0x140);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx_lock + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx_lock));
        wr::<u32>(rbx_lock + 0x08, tid);
    }
    wr::<u32>(rbx_lock + 0x0c, rd::<u32>(rbx_lock + 0x0c).wrapping_add(1));

    let eax_a8 = rd::<i32>(param_1 + 0xa8);
    let mut can_proceed = true;
    if eax_a8 <= 0 {
        if rd::<i32>(param_1 + 0x68) <= 0 {
            can_proceed = false;
        } else if eax_a8 == 0 {
            if exe::call_va!(0x1_4241_df90, (param_1) -> u8) == 0 {
                can_proceed = false;
            }
        }
    }

    let rsi_out = if can_proceed {
        let rax_80 = param_1.wrapping_add(0x80);
        let rcx_98 = rd::<usize>(rax_80 + 0x18);
        let ptr_node = if rcx_98 == 0 || rcx_98 == rax_80 {
            rd::<usize>(rax_80 + 0x10)
        } else {
            exe::call_va!(0x1_411e_3360, (rcx_98) -> usize)
        };
        let r14 = rd::<usize>(ptr_node);
        let rbx_40 = r14.wrapping_add(0x40);
        let rdx_owner = rd::<usize>(rbx_40 + 0x18);
        if rdx_owner != 0 && rdx_owner != rbx_40 {
            let prev = rd::<usize>(rbx_40 + 0x08);
            let next = rd::<usize>(rbx_40 + 0x10);
            wr::<usize>(prev + 0x10, next);
            let next2 = rd::<usize>(rbx_40 + 0x10);
            let prev2 = rd::<usize>(rbx_40 + 0x08);
            wr::<usize>(next2 + 0x08, prev2);
            wr::<usize>(rbx_40 + 0x08, rbx_40);
            wr::<usize>(rbx_40 + 0x10, rbx_40);
            wr::<usize>(rbx_40 + 0x18, 0);
            wr::<u32>(rdx_owner + 0x28, rd::<u32>(rdx_owner + 0x28).wrapping_sub(1));
        }

        let slot_ptr = exe::call_va!(
            0x1_4139_ea50,
            (param_1 + 0xd8, (&mut key_slot as *mut u64) as usize) -> usize
        );
        wr::<usize>(slot_ptr + 0x08, r14);

        let r11d = rd::<u32>(param_1 + 0x68).wrapping_add(rd::<u32>(param_1 + 0x28));
        let r8_off = ((r11d as i32) as isize as usize).wrapping_mul(8);
        let arr_c8 = rd::<usize>(param_1 + 0xc8);
        let elem_at_r8 = rd::<usize>(arr_c8.wrapping_add(r8_off));
        let idx_r14 = rd::<u32>(r14 + 0x20);
        wr::<u32>(elem_at_r8 + 0x20, idx_r14);

        let arr_c8_2 = rd::<usize>(param_1 + 0xc8);
        let r9_ptr = arr_c8_2.wrapping_add(r8_off);
        let idx_r14_2 = (rd::<i32>(r14 + 0x20) as isize as usize).wrapping_mul(8);
        let r8_ptr = arr_c8_2.wrapping_add(idx_r14_2);
        let v_r8 = rd::<u64>(r8_ptr);
        let v_r9 = rd::<u64>(r9_ptr);
        wr::<u64>(r8_ptr, v_r9);
        wr::<u64>(r9_ptr, v_r8);

        exe::call_va!(0x1_4139_e210, (param_1, rbx_40));
        let vtbl = rd::<usize>(r14);
        let fn_60 = rd::<usize>(vtbl + 0x60);
        exe::call_va!(fn_60, (r14, r11d, key_slot, param_3, param_1));
        r14
    } else {
        0
    };

    let new_cnt = rd::<u32>(rbx_lock + 0x0c).wrapping_sub(1);
    wr::<u32>(rbx_lock + 0x0c, new_cnt);
    if new_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((rbx_lock + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx_lock));
    }
    rsi_out
}

/// `r_14241d5c0`
/// Original: 0x14241d5c0 (291 bytes, Throws::No)
pub unsafe extern "win64" fn r_14241d5c0(param_1: usize, param_2: u64) -> u64 {
    let rbx_lock = param_1.wrapping_add(0x140);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx_lock + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx_lock));
        wr::<u32>(rbx_lock + 0x08, tid);
    }
    wr::<u32>(rbx_lock + 0x0c, rd::<u32>(rbx_lock + 0x0c).wrapping_add(1));

    let mut rbp_res: u64 = 0;
    let r11_del = rd::<u64>(param_1 + 0x108);
    if rd::<u64>(param_1 + 0x110) != r11_del {
        let r8_buckets = rd::<usize>(param_1 + 0x130);
        let mut r10_step: u64 = 0;
        let mut r9_tomb: u64 = u64::MAX;
        let r12_empty = rd::<u64>(param_1 + 0x120);
        let r15_mask = rd::<u64>(param_1 + 0x118).wrapping_sub(1);
        let mut rcx_idx = r15_mask & param_2;
        loop {
            let bucket_p = r8_buckets.wrapping_add((rcx_idx as usize) * 16);
            let k = rd::<u64>(bucket_p);
            if k == r12_empty {
                break;
            }
            if r11_del != 0 && rd::<u64>(param_1 + 0x100) == k {
                if r9_tomb == u64::MAX {
                    r9_tomb = rcx_idx;
                }
            } else if param_2 == k {
                if rcx_idx != u64::MAX {
                    let cap = rd::<u64>(param_1 + 0x118) as usize;
                    let end_p = r8_buckets.wrapping_add(cap * 16);
                    if bucket_p != end_p {
                        rbp_res = rd::<u64>(bucket_p + 0x08);
                    }
                }
                break;
            }
            r10_step = r10_step.wrapping_add(1);
            rcx_idx = rcx_idx.wrapping_add(r10_step) & r15_mask;
        }
    }

    let new_cnt = rd::<u32>(rbx_lock + 0x0c).wrapping_sub(1);
    wr::<u32>(rbx_lock + 0x0c, new_cnt);
    if new_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((rbx_lock + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx_lock));
    }
    rbp_res
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_r_14222c360(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_7250: u32 = 0;
    static mut CALL_9AB0: u32 = 0;

    unsafe extern "win64" fn mock_7250(mgr: usize) {
        CALL_7250 += 1;
        wr::<u32>(mgr + 0x500, rd::<u32>(mgr + 0x500).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_9ab0(_p: usize) {
        CALL_9AB0 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4238_7250, mock_7250 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4222_9ab0, mock_9ab0 as *const ())?;

    let run = |f: F, active: bool, match_tail: bool| -> (u32, u32, u32, u32, u32) {
        CALL_7250 = 0;
        CALL_9AB0 = 0;

        let idx_188: [u32; 3] = [1, 0, 2];
        let mut arr_b8 = [0u32; 3 * 8];
        arr_b8[0 * 8 + 1] = 2;
        arr_b8[1 * 8 + 1] = 0;
        arr_b8[2 * 8 + 1] = 1;

        let mut arr_5e8 = [0u32; 3 * 4];
        arr_5e8[0 * 4 + 1] = 65;
        arr_5e8[1 * 4 + 1] = 190;
        arr_5e8[2 * 4 + 1] = 128;

        let tail_pair: [u32; 2] = if match_tail { [30, 70] } else { [30, 50] };

        let mut mgr = vec![0u8; 0x700];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0xb8, arr_b8.as_ptr() as usize);
        wr::<usize>(mp + 0x188, idx_188.as_ptr() as usize);
        wr::<u32>(mp + 0x250, 100);
        wr::<usize>(mp + 0x258, tail_pair.as_ptr() as usize);
        wr::<u32>(mp + 0x260, 1);
        wr::<u32>(mp + 0x520, if active { 1 } else { 0 });
        wr::<u32>(mp + 0x5a0, 3);
        wr::<usize>(mp + 0x5e8, arr_5e8.as_ptr() as usize);

        let callable: [usize; 2] = [0, mp];
        f(callable.as_ptr() as usize);

        (
            rd::<u32>(mp + 0x500),
            rd::<u32>(mp + 0x6e0),
            rd::<u32>(mp + 0x5a4),
            CALL_7250,
            CALL_9AB0,
        )
    };

    for (active, match_tail) in [(false, false), (true, true), (true, false)] {
        let o = run(f_o, active, match_tail);
        let n = run(f_n, active, match_tail);
        crate::diff_eq!(o, n, "14222c360 active={active} match_tail={match_tail}");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14222cad0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u64; 2], [u64; 2], [u32; 32], u8, u8) {
        let mut head = vec![0u8; 0x40];
        let mut node1 = vec![0u8; 0x720];
        let mut node2 = vec![0u8; 0x720];
        let hp = head.as_mut_ptr() as usize;
        let n1 = node1.as_mut_ptr() as usize;
        let n2 = node2.as_mut_ptr() as usize;

        // Sentinel head (is_nil = 1 at +0x19); [head + 0] = leftmost = n1
        wr::<usize>(hp, n1);
        wr::<usize>(hp + 0x08, n1);
        wr::<usize>(hp + 0x10, n2);
        wr::<u8>(hp + 0x19, 1);

        // Node 1: right child = n2, parent = head
        wr::<usize>(n1, hp);
        wr::<usize>(n1 + 0x08, hp);
        wr::<usize>(n1 + 0x10, n2);
        wr::<u8>(n1 + 0x19, 0);

        // Node 2: leaf child of n1, parent = n1
        wr::<usize>(n2, hp);
        wr::<usize>(n2 + 0x08, n1);
        wr::<usize>(n2 + 0x10, hp);
        wr::<u8>(n2 + 0x19, 0);

        let mut e0_arr = [0u32; 32]; // 4 elements of 0x20 bytes (8 u32s each)
        e0_arr[1 * 8 + 5] = 0x13; // bit 0 set at +0x14
        e0_arr[2 * 8 + 5] = 0x12; // bit 0 clear at +0x14
        e0_arr[3 * 8 + 5] = 0x01; // bit 0 set at +0x14

        let mut f0_bits: [u64; 2] = [0, 0];
        let mut bits_n1: [u64; 2] = [0b1110, 0xffff_ffff_ffff_ffff];
        let mut bits_n2: [u64; 2] = [0b1000, 0];

        wr::<usize>(n1 + 0xe0, e0_arr.as_mut_ptr() as usize);
        wr::<usize>(n1 + 0xf0, f0_bits.as_mut_ptr() as usize);
        wr::<usize>(n1 + 0x570, bits_n1.as_mut_ptr() as usize);
        wr::<u64>(n1 + 0x580, 70);
        wr::<u32>(n1 + 0x5a8, 4);
        wr::<u8>(n1 + 0x70e, 1);

        wr::<usize>(n2 + 0xe0, e0_arr.as_mut_ptr() as usize);
        wr::<usize>(n2 + 0xf0, f0_bits.as_mut_ptr() as usize);
        wr::<usize>(n2 + 0x570, bits_n2.as_mut_ptr() as usize);
        wr::<u64>(n2 + 0x580, 4);
        wr::<u32>(n2 + 0x5a8, 4);
        wr::<u8>(n2 + 0x70e, 1);

        let p1 = [hp];
        f(p1.as_ptr() as usize);

        (
            f0_bits,
            bits_n1,
            e0_arr,
            rd::<u8>(n1 + 0x70e),
            rd::<u8>(n2 + 0x70e),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14222cad0 out");
    Ok(1)
}

pub unsafe fn diff_r_14225abc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;

    unsafe extern "win64" fn mock_cc50(vec_ptr: usize, count: u64) {
        wr::<u32>(vec_ptr + 0x08, count as u32);
    }
    unsafe extern "win64" fn mock_9ab0(_p: usize) {}
    unsafe extern "win64" fn mock_eba0(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_9d10(p: usize, new_val: u32) {
        wr::<u32>(p, new_val);
        LOG_HASH = LOG_HASH.wrapping_mul(131).wrapping_add(new_val as u64);
    }
    unsafe extern "win64" fn mock_8460(_p: usize, count: u32) {
        LOG_HASH = LOG_HASH.wrapping_mul(131).wrapping_add(count as u64);
    }
    unsafe extern "win64" fn mock_d6f0(h: u64, flag: u32) {
        LOG_HASH = LOG_HASH
            .wrapping_mul(131)
            .wrapping_add(h)
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_fa10(a: u64, b: u64, c: u64) {
        LOG_HASH = LOG_HASH
            .wrapping_mul(131)
            .wrapping_add(a ^ (b << 1) ^ (c << 2));
    }

    let _m1 = crate::difftest::Mock::new(0x1_406d_cc50, mock_cc50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4222_9ab0, mock_9ab0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4222_9d10, mock_9d10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_412f_8460, mock_8460 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4225_fa10, mock_fa10 as *const ())?;

    let g_ed0_ptr = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let saved_ed0 = rd::<usize>(g_ed0_ptr);

    let run = |f: F| -> (u64, [u32; 4], [u64; 2], [u32; 12], [u32; 12], [u32; 4]) {
        LOG_HASH = 0x1234;

        let mut arr_98: [u32; 4] = [10, u32::MAX, 20, 5];
        let mut arr_a8: [u64; 2] = [0, 0];
        let arr_e0: [u32; 4] = [70, 0, 50, 30];
        let arr_100: [u32; 4] = [0, u32::MAX, 1, 2];

        let mut ed0 = vec![0u8; 0x140];
        let ep = ed0.as_mut_ptr() as usize;
        wr::<usize>(ep + 0x98, arr_98.as_mut_ptr() as usize);
        wr::<usize>(ep + 0xa8, arr_a8.as_mut_ptr() as usize);
        wr::<usize>(ep + 0xe0, arr_e0.as_ptr() as usize);
        wr::<usize>(ep + 0x100, arr_100.as_ptr() as usize);
        wr::<u32>(ep + 0x128, 4);
        wr::<usize>(g_ed0_ptr, ep);

        let mut buf_398: [u32; 4] = [4, 2, 12, 3];
        let mut buf_440: [u32; 4] = [0; 4];
        let mut buf_6c8 = [0u32; 16];
        let mut buf_6d8 = [0u32; 32];
        let mut buf_6e8 = [0u32; 32];
        let mut buf_6f8 = [0u32; 16];
        let mut out_348 = [0u32; 12];
        let mut out_3f0 = [0u32; 12];
        let bits_668: [u64; 1] = [0b0010]; // bit 1 set, bits 0 and 2 clear

        let mut sub_308 = [0u64; 4];
        sub_308[2] = 1;
        let mut sub_3b0 = [0u64; 4];
        sub_3b0[2] = 1;

        let mut p1 = vec![0u8; 0x720];
        let p = p1.as_mut_ptr() as usize;
        wr::<usize>(p + 0x308, sub_308.as_mut_ptr() as usize);
        wr::<u64>(p + 0x318, 0xaaaa);
        wr::<u32>(p + 0x324, 1);
        wr::<usize>(p + 0x348, out_348.as_mut_ptr() as usize);
        wr::<u64>(p + 0x360, 0x30);
        wr::<u64>(p + 0x368, 0x300);
        wr::<u32>(p + 0x390, 20);
        wr::<usize>(p + 0x398, buf_398.as_mut_ptr() as usize);
        wr::<u32>(p + 0x3a0, 2);
        wr::<u32>(p + 0x3a4, 2);

        wr::<usize>(p + 0x3b0, sub_3b0.as_mut_ptr() as usize);
        wr::<u64>(p + 0x3c0, 0xbbbb);
        wr::<u32>(p + 0x3cc, 1);
        wr::<usize>(p + 0x3f0, out_3f0.as_mut_ptr() as usize);
        wr::<u64>(p + 0x408, 0x30);
        wr::<u64>(p + 0x410, 0x300);
        wr::<u32>(p + 0x438, 20);
        wr::<usize>(p + 0x440, buf_440.as_mut_ptr() as usize);
        wr::<u32>(p + 0x448, 0);
        wr::<u32>(p + 0x44c, 2);

        wr::<usize>(p + 0x668, bits_668.as_ptr() as usize);
        wr::<usize>(p + 0x6c8, buf_6c8.as_mut_ptr() as usize);
        wr::<usize>(p + 0x6d8, buf_6d8.as_mut_ptr() as usize);
        wr::<usize>(p + 0x6e8, buf_6e8.as_mut_ptr() as usize);
        wr::<usize>(p + 0x6f8, buf_6f8.as_mut_ptr() as usize);

        f(p);

        (LOG_HASH, arr_98, arr_a8, out_348, out_3f0, buf_398)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_ed0_ptr, saved_ed0);
    crate::diff_eq!(o, n, "14225abc0 out");
    Ok(1)
}

pub unsafe fn diff_r_14225be60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_ed0_ptr = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let saved_ed0 = rd::<usize>(g_ed0_ptr);

    let run = |f: F| -> (u32, [u32; 28]) {
        let arr_98: [u32; 2] = [10, 20];
        let arr_d0: [u32; 2] = [0, 1];
        let arr_f0: [u32; 2] = [1, 1];

        let mut ed0 = vec![0u8; 0x100];
        let ep = ed0.as_mut_ptr() as usize;
        wr::<usize>(ep + 0x98, arr_98.as_ptr() as usize);
        wr::<usize>(ep + 0xd0, arr_d0.as_ptr() as usize);
        wr::<usize>(ep + 0xf0, arr_f0.as_ptr() as usize);
        wr::<usize>(g_ed0_ptr, ep);

        let bits: [u64; 1] = [0b11];
        let p1: [usize; 1] = [bits.as_ptr() as usize];

        let idx_table: [u64; 2] = [0, 1];
        let mut p2 = [0u64; 2];
        p2[0] = 2;
        p2[1] = idx_table.as_ptr() as usize as u64;

        let mut out_buf = [0u32; 28]; // 2 * 0x38 bytes
        let mut p3 = [0u8; 0x50];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x38, out_buf.as_mut_ptr() as usize);
        wr::<u32>(p3p + 0x40, 0);

        let mut mats = [0u32; 24]; // 2 * 0x30 bytes
        for (i, slot) in mats.iter_mut().enumerate() {
            *slot = (i as u32 + 1) * 0x101;
        }
        let p4: [usize; 1] = [mats.as_ptr() as usize];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3p,
            p4.as_ptr() as usize,
        );

        (rd::<u32>(p3p + 0x40), out_buf)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_ed0_ptr, saved_ed0);
    crate::diff_eq!(o, n, "14225be60 out");
    Ok(1)
}

pub unsafe fn diff_r_14225bfe0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    const WORDS: usize = 4;
    const MAX_E: usize = WORDS * 64 + 8;
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut cases = 0u32;
    for (ci, &bit_count) in [0u64, 1, 37, 63, 64, 65, 130, 200, 256, 256].iter().enumerate() {
        let mut bits = [0u64; WORDS];
        for (i, w) in bits.iter_mut().enumerate() {
            *w = match (ci + i) % 4 {
                0 => rnd(),
                1 => rnd() & rnd() & rnd(),
                2 => 0,
                _ => 1u64 << 63 | 1,
            };
        }
        if ci == 9 {
            bits = [u64::MAX; WORDS];
        }
        let mut mats = vec![0u32; (WORDS * 64) * 12 + 4];
        for m in mats.iter_mut() {
            *m = rnd() as u32;
        }
        let start = [0u32, 3][ci % 2];
        let adding = rnd() as u32 & 0xff;
        let mis = if ci % 3 == 1 { 4 } else { 0 };
        let run = |f: F| -> (Vec<u8>, Vec<u8>) {
            let mut wbuf = vec![0u8; WORDS * 8 + 8];
            let wp = wbuf.as_mut_ptr() as usize + mis;
            for (i, w) in bits.iter().enumerate() {
                wr::<u64>(wp + i * 8, *w);
            }
            let holder: [usize; 1] = [wp];
            let mp = mats.as_ptr() as usize + mis;
            let mholder: [usize; 1] = [mp];
            let mut out = vec![0xa5u8; MAX_E * 0x38 + 0x40];
            let op = out.as_mut_ptr() as usize + mis;
            let mut list = vec![0x5au8; 0x60];
            let lp = list.as_mut_ptr() as usize + mis;
            wr::<usize>(lp + 0x38, op);
            wr::<u32>(lp + 0x40, start);
            wr::<u32>(lp + 0x20, adding);
            f(holder.as_ptr() as usize, bit_count, lp, mholder.as_ptr() as usize);
            wr::<usize>(lp + 0x38, 0); // per-run buffer address
            (out, list)
        };
        let o = run(f_o);
        let n = run(f_n);
        for (what, a, b) in [("out", &o.0, &n.0), ("list", &o.1, &n.1)] {
            if let Some(i) = (0..a.len()).find(|&i| a[i] != b[i]) {
                return Err(format!(
                    "mismatch 14225bfe0 case {ci} bits {bit_count}: {what}[{i:#x}] orig={:#x} new={:#x}",
                    a[i], b[i]
                ));
            }
        }
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_unknown_142303ab0(orig: usize, new: usize) -> Result<u32, String> {
    static mut PROP_BUF: [u32; 4] = [0; 4];
    static mut SUB_OBJ: [u8; 0x40] = [0; 0x40];
    static mut HOLDER_PTR: usize = 0;

    unsafe extern "win64" fn mock_3160(_p1: usize, out_pair: usize, _idx: u32) {
        wr::<usize>(out_pair, core::ptr::addr_of_mut!(HOLDER_PTR) as usize);
        wr::<usize>(out_pair + 0x08, core::ptr::addr_of_mut!(SUB_OBJ) as usize);
    }
    unsafe extern "win64" fn mock_1210(_rcx: usize, _id: u32) -> usize {
        core::ptr::addr_of_mut!(PROP_BUF) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4230_3160, mock_3160 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4131_1210, mock_1210 as *const ())?;

    let tls_arr = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_arr);
    let mut tls_buf = [0u8; 0x100];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    wr::<usize>(tls_arr, tls0);

    let g_b78 = exe::ptr::<i32>(0x1_43cf_8b78) as usize;
    let g_b80 = exe::ptr::<u32>(0x1_43cf_8b80) as usize;
    let g_c98 = exe::ptr::<usize>(0x1_43a3_6c98) as usize;
    let s_b78 = rd::<i32>(g_b78);
    let s_b80 = rd::<u32>(g_b80);
    let s_c98 = rd::<usize>(g_c98);

    let run = |fn_addr: usize, scale: f32| -> ([u32; 4], u8, u64) {
        PROP_BUF = [0xdead; 4];
        SUB_OBJ = [0; 0x40];
        HOLDER_PTR = 0x5555;
        let sub_p = core::ptr::addr_of_mut!(SUB_OBJ) as usize;
        wr::<u16>(sub_p + 0x20, 5);

        wr::<i32>(tls0 + 0x74, 10);
        wr::<i32>(g_b78, 0); // already initialized (<= tls_epoch)
        wr::<u32>(g_b80, 0x7777);

        let mut c98_buf = [0u8; 0x90];
        let c98_p = c98_buf.as_mut_ptr() as usize;
        wr::<usize>(g_c98, c98_p);

        let keys: [u32; 2] = [0x1111, 0x7777];
        let mut vals = [0u8; 0x30];
        wr::<f32>(vals.as_mut_ptr() as usize + 0x18 + 0x08, 3.5);

        let mut r9_table = [0u8; 0x20];
        let r9p = r9_table.as_mut_ptr() as usize;
        wr::<usize>(r9p, keys.as_ptr() as usize);
        wr::<u32>(r9p + 0x08, 2);
        wr::<usize>(r9p + 0x10, vals.as_ptr() as usize);

        let rcx_c0: [usize; 1] = [r9p];
        let mut elem = [0u8; 0xd0];
        wr::<usize>(elem.as_mut_ptr() as usize + 0xc0, rcx_c0.as_ptr() as usize);
        let elem_list: [usize; 1] = [elem.as_ptr() as usize];

        let mut rdi_obj = [0u8; 0x180];
        let rdi_p = rdi_obj.as_mut_ptr() as usize;
        wr::<i32>(rdi_p + 0x138, 1);
        wr::<usize>(rdi_p + 0x160, elem_list.as_ptr() as usize);
        wr::<u32>(rdi_p + 0x178, 0);

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, rdi_p);
        wr::<u8>(p1p + 0x4f, 0);

        #[cfg(not(target_arch = "aarch64"))]
        {
            type F = unsafe extern "win64" fn(usize, f32);
            let f: F = core::mem::transmute(fn_addr);
            f(p1p, scale);
        }
        #[cfg(target_arch = "aarch64")]
        {
            let _ = crate::simd_ops::call_va8_xmm(
                fn_addr as u64,
                p1p as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, scale.to_bits() as u128, 0, 0],
            );
        }

        (PROP_BUF, rd::<u8>(sub_p + 0x38), rd::<u64>(c98_p + 0x78))
    };

    let o = run(orig, 2.0);
    let n = run(new, 2.0);
    wr::<i32>(g_b78, s_b78);
    wr::<u32>(g_b80, s_b80);
    wr::<usize>(g_c98, s_c98);
    wr::<usize>(tls_arr, saved_tls0);

    crate::diff_eq!(o, n, "142303ab0 out");
    Ok(1)
}

pub unsafe fn diff_rend_142336540(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut UPLOAD_LOG: u64 = 0;
    static mut VTBL_CALLS: u64 = 0;

    unsafe extern "win64" fn mock_d860(ctx: usize, src: u64, size: u64) {
        wr::<u32>(ctx + 0x70, 7);
        wr::<u64>(ctx + 0x78, size);
        UPLOAD_LOG = UPLOAD_LOG
            .wrapping_mul(131)
            .wrapping_add(rd::<u64>(ctx))
            .wrapping_add((src != 0) as u64)
            .wrapping_add(size);
    }
    unsafe extern "win64" fn mock_de40() {
        UPLOAD_LOG = UPLOAD_LOG.wrapping_add(0x1000);
    }
    unsafe extern "win64" fn mock_5bc0(h: u64, off: u64, _s18: usize, flag: u32, chunk: u64) {
        UPLOAD_LOG = UPLOAD_LOG
            .wrapping_mul(131)
            .wrapping_add(h)
            .wrapping_add(off)
            .wrapping_add(flag as u64)
            .wrapping_add(chunk);
    }
    unsafe extern "win64" fn mock_e490(_vec: usize, new_len: u32) {
        UPLOAD_LOG = UPLOAD_LOG.wrapping_add(new_len as u64);
    }
    unsafe extern "win64" fn mock_vtbl_c0(
        _this: usize,
        cnt: u32,
        dst: u64,
        src: u64,
        arg5: u32,
    ) {
        VTBL_CALLS = VTBL_CALLS
            .wrapping_mul(131)
            .wrapping_add(cnt as u64)
            .wrapping_add(dst)
            .wrapping_add(src)
            .wrapping_add(arg5 as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_d860, mock_d860 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_de40, mock_de40 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_5bc0, mock_5bc0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4028_e490, mock_e490 as *const ())?;

    let s_fc80 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fc80) as *mut u8, 8);
    let s_fca8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fca8) as *mut u8, 8);
    let s_fcb0 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fcb0) as *mut u8, 4);
    let s_fcb8 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fcb8) as *mut u8, 8);
    let s_8478 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_8478) as *mut u8, 16);
    let s_c754 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4386_c754) as *mut u8, 8);

    let run = |f: F| -> (u64, u64, u64, u8, u8) {
        UPLOAD_LOG = 1;
        VTBL_CALLS = 1;

        let mut vtbl = [0usize; 32];
        vtbl[0xc0 / 8] = mock_vtbl_c0 as *const () as usize;
        let dev_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fc80) as usize,
            dev_obj.as_ptr() as usize,
        );

        let ca8_obj: [u64; 2] = [0, 0x9999_0000];
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fca8) as usize,
            ca8_obj.as_ptr() as usize,
        );
        wr::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize, 16);

        let mut cb8_obj = vec![0u8; 0x290];
        let cb8_p = cb8_obj.as_mut_ptr() as usize;
        wr::<u32>(cb8_p + 0x28, u32::MAX);
        wr::<u64>(cb8_p + 0x88, 0xdef0_1234);
        wr::<u64>(cb8_p + 0x280, 0x2000_0000);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize, cb8_p);

        wr::<u64>(exe::ptr::<u64>(0x1_4385_8478) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4385_8480) as usize, u32::MAX);
        wr::<u32>(exe::ptr::<u32>(0x1_4385_8484) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_c758) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_c754) as usize, 0);

        let mut sub_elem = [0u8; 0x80];
        wr::<u64>(sub_elem.as_mut_ptr() as usize + 0x78, 0x7777_8888);

        let mut entries_38 = [0usize; 16];
        entries_38[0] = sub_elem.as_ptr() as usize;

        let mut p1 = vec![0u8; 0x2090];
        let p = p1.as_mut_ptr() as usize;
        wr::<u64>(p, 0x100);
        wr::<u32>(p + 0x08, 64); // 1 word in bitmask
        wr::<u64>(p + 0x10, 0x200);
        wr::<u64>(p + 0x20, 0x300);
        wr::<u32>(p + 0x28, 32);
        wr::<u8>(p + 0x30, 0);
        wr::<usize>(p + 0x38, entries_38.as_ptr() as usize);
        wr::<u64>(p + 0x50, 0xa001);
        wr::<u64>(p + 0x58, 0xa002);
        wr::<u64>(p + 0x60, 0xa003);
        wr::<u64>(p + 0x2078, 1);

        f(p);

        (
            UPLOAD_LOG,
            VTBL_CALLS,
            rd::<u64>(p + 0x2078),
            rd::<u8>(exe::ptr::<u8>(0x1_4386_c758) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4386_c754) as usize),
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    s_c754.restore();
    s_8478.restore();
    s_fcb8.restore();
    s_fcb0.restore();
    s_fca8.restore();
    s_fc80.restore();

    crate::diff_eq!(o, n, "142336540 out");
    Ok(1)
}

pub unsafe fn diff_rend_142338090(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_ed8 = exe::ptr::<usize>(0x1_43a3_6ed8) as usize;
    let saved_ed8 = rd::<usize>(g_ed8);

    let keys: [u32; 3] = [10, 20, 30];
    let mut entries = [0u32; 3 * 6]; // 3 * 0x18 bytes
    // Entry 0 (key 10): type = 4, idx = 0 -> fast array at +0x40 has non-zero
    entries[0 * 6] = 4;
    entries[0 * 6 + 2] = 0;
    // Entry 1 (key 20): type = 4, idx = 1 -> fast array at +0x40 is 0, fallback at +0x30 -> +0x1a0
    entries[1 * 6] = 4;
    entries[1 * 6 + 2] = 1;
    // Entry 2 (key 30): type = 2 (!= 4) -> returns 0
    entries[2 * 6] = 2;
    entries[2 * 6 + 2] = 0;

    let mut rcx_tbl = [0u8; 0x20];
    let rcx_p = rcx_tbl.as_mut_ptr() as usize;
    wr::<usize>(rcx_p, keys.as_ptr() as usize);
    wr::<u32>(rcx_p + 0x08, 3);
    wr::<usize>(rcx_p + 0x10, entries.as_ptr() as usize);

    let mut fallback_obj = [0u8; 0x1b0];
    wr::<usize>(fallback_obj.as_mut_ptr() as usize + 0x1a0, 0xbbbb_2222);

    let arr_40: [usize; 2] = [0xaaaa_1111, 0];
    let arr_30: [usize; 2] = [0, fallback_obj.as_ptr() as usize];

    let mut r10_obj = [0u8; 0x50];
    let r10_p = r10_obj.as_mut_ptr() as usize;
    wr::<usize>(r10_p, rcx_p);
    wr::<usize>(r10_p + 0x30, arr_30.as_ptr() as usize);
    wr::<usize>(r10_p + 0x40, arr_40.as_ptr() as usize);
    wr::<u32>(r10_p + 0x48, 2);

    let r10_holder: [usize; 1] = [r10_p];
    wr::<usize>(g_ed8, r10_holder.as_ptr() as usize);

    for want in [10u32, 20, 30, 99] {
        let o = f_o(0, (&want as *const u32) as usize);
        let n = f_n(0, (&want as *const u32) as usize);
        wr::<usize>(g_ed8, r10_holder.as_ptr() as usize);
        crate::diff_eq!(o, n, "142338090 want={want}");
    }

    wr::<usize>(g_ed8, saved_ed8);
    Ok(4)
}

pub unsafe fn diff_rend_14233e9d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut EEC0_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_f070(_p1: usize) -> u64 {
        (0x4242u64 << 32) | 1u64
    }
    unsafe extern "win64" fn mock_eec0(_p1: usize, _res_ptr: usize, _z: u32) {
        EEC0_CALLED += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4233_f070, mock_f070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4233_eec0, mock_eec0 as *const ())?;

    let g_fa8 = exe::ptr::<usize>(0x1_43a3_6fa8) as usize;
    let saved_fa8 = rd::<usize>(g_fa8);

    let run = |f: F| -> (u64, u64, u32, usize, u32) {
        EEC0_CALLED = 0;

        let keys: [u32; 2] = [0x1111, 0x4242];
        let mut vals = [0u8; 0x30];
        wr::<f32>(vals.as_mut_ptr() as usize + 0x18 + 0x08, 6.25);

        let mut rsi_tables = [0u8; 0x40];
        let tp = rsi_tables.as_mut_ptr() as usize + 0x20; // index 1 * 0x20
        wr::<usize>(tp, keys.as_ptr() as usize);
        wr::<u32>(tp + 0x08, 2);
        wr::<usize>(tp + 0x10, vals.as_ptr() as usize);
        let rsi_holder: [usize; 1] = [rsi_tables.as_ptr() as usize];
        wr::<usize>(g_fa8, rsi_holder.as_ptr() as usize);

        let mut stack_slot = [0u8; 0x20];
        let sp = stack_slot.as_mut_ptr() as usize;
        let mut state = [0usize; 2];
        state[1] = sp;

        let ret = f(state.as_mut_ptr() as usize);
        (
            ret,
            rd::<u64>(sp),
            rd::<u32>(sp + 0x10),
            state[1].wrapping_sub(sp),
            EEC0_CALLED,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_fa8, saved_fa8);

    crate::diff_eq!(o, n, "14233e9d0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142350560(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_vtbl_18(this: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(rd::<u32>(this + 8) as u64);
    }
    unsafe extern "win64" fn mock_vtbl_28(this: usize, cnt: u64) {
        wr::<u64>(this + 0x10, cnt);
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(cnt);
    }
    unsafe extern "win64" fn mock_vtbl_38(this: usize) -> u64 {
        rd::<u64>(this + 0x10)
    }
    unsafe extern "win64" fn mock_d670(_p: usize, cnt: u64) {
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(cnt);
    }
    unsafe extern "win64" fn mock_ebd0(_p: usize, cnt: u64, val_ptr: usize) {
        CALL_LOG = CALL_LOG
            .wrapping_mul(131)
            .wrapping_add(cnt)
            .wrapping_add(rd::<u32>(val_ptr) as u64);
    }
    unsafe extern "win64" fn mock_7910(_p: usize, cnt: u64) {
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(cnt);
    }
    unsafe extern "win64" fn mock_b5b0(_p: usize, cnt: u64, _def: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(cnt);
    }
    unsafe extern "win64" fn mock_f1e0(_p: usize, cnt: u64) {
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(cnt);
    }
    unsafe extern "win64" fn mock_f0a0(desc: usize, z: u32, cnt: u64, flag: u8) {
        let d0 = rd::<u64>(desc);
        let d2 = rd::<u64>(desc + 0x10);
        CALL_LOG = CALL_LOG
            .wrapping_mul(131)
            .wrapping_add(d0)
            .wrapping_add(d2)
            .wrapping_add(z as u64)
            .wrapping_add(cnt)
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_dcc0(_p: usize, cnt: u64) {
        CALL_LOG = CALL_LOG.wrapping_mul(131).wrapping_add(cnt);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4134_d670, mock_d670 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412d_ebd0, mock_ebd0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_402d_7910, mock_7910 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412e_b5b0, mock_b5b0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40ff_f0a0, mock_f0a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4134_dcc0, mock_dcc0 as *const ())?;

    let run = |f: F, nx: i32, ny: i32| -> (u8, u64, Vec<u32>, Vec<u32>, Vec<u32>, [u64; 4]) {
        CALL_LOG = 1;

        let mut vtbl = [0usize; 8];
        vtbl[0x18 / 8] = mock_vtbl_18 as *const () as usize;
        vtbl[0x28 / 8] = mock_vtbl_28 as *const () as usize;
        vtbl[0x38 / 8] = mock_vtbl_38 as *const () as usize;

        let mut buf_88 = vec![0xffu8; 2048];
        let mut buf_268 = vec![0u64; 1024];
        let mut pos_buf = vec![0u32; 64];
        let mut idx_buf = vec![0u32; 64];
        let mut adj_buf = vec![0u32; 128];

        let mut p1 = vec![0u8; 0x2b0];
        let p = p1.as_mut_ptr() as usize;
        wr::<usize>(p + 0x88, buf_88.as_mut_ptr() as usize);
        wr::<u32>(p + 0x90, 4);
        wr::<u32>(p + 0x94, 2048);

        wr::<usize>(p + 0xa8, vtbl.as_ptr() as usize);
        wr::<u32>(p + 0xb0, 11);
        wr::<usize>(p + 0x140, idx_buf.as_mut_ptr() as usize);
        wr::<u64>(p + 0x160, 128);
        wr::<usize>(p + 0x178, adj_buf.as_mut_ptr() as usize);

        wr::<usize>(p + 0x188, vtbl.as_ptr() as usize);
        wr::<u32>(p + 0x190, 22);
        wr::<usize>(p + 0x220, pos_buf.as_mut_ptr() as usize);
        wr::<u64>(p + 0x240, 64);
        wr::<usize>(p + 0x268, buf_268.as_mut_ptr() as usize);

        let mut p2 = [0u8; 24];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p, nx);
        wr::<i32>(p2p + 0x04, ny);
        wr::<f32>(p2p + 0x08, -1.0);
        wr::<f32>(p2p + 0x0c, -2.0);
        wr::<f32>(p2p + 0x10, 3.0);
        wr::<f32>(p2p + 0x14, 4.0);

        let ret = f(p, p2p);
        let sample_268 = [buf_268[0], buf_268[1], buf_268[500], buf_268[507]];
        (ret, CALL_LOG, pos_buf, idx_buf, adj_buf, sample_268)
    };

    for (nx, ny) in [(0, 2), (2, 2)] {
        let o = run(f_o, nx, ny);
        let n = run(f_n, nx, ny);
        crate::diff_eq!(o, n, "142350560 nx={nx} ny={ny}");
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1423e6ef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_ba80(vec_ptr: usize, new_cap: u64) {
        GROW_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_ba80, mock_ba80 as *const ())?;

    let g_230 = exe::ptr::<usize>(0x1_43a0_0230) as usize;
    let g_180 = exe::ptr::<i32>(0x1_43a0_0180) as usize;
    let g_0a8 = exe::ptr::<usize>(0x1_43be_60a8) as usize;
    let s_230 = rd::<usize>(g_230);
    let s_180 = rd::<i32>(g_180);
    let s_0a8 = rd::<usize>(g_0a8);

    let run = |f: F| -> ([u64; 6], u32, u32, u32, u32) {
        GROW_CALLS = 0;

        let mut target_items = vec![0u8; 0x98 * 2];
        let ti_base = target_items.as_mut_ptr() as usize;
        wr::<usize>(g_0a8, ti_base);
        wr::<i32>(g_180, 77);

        let mut vec_entries = [0u64; 6]; // 2 entries of 0x18 bytes (3 u64s each)
        vec_entries[0] = ti_base as u64; // entry 0 already points to item 0
        vec_entries[1] = 0x1111;
        vec_entries[2] = 10;

        let mut mgr = [0u8; 0x120];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x100, vec_entries.as_mut_ptr() as usize);
        wr::<u32>(mp + 0x108, 1); // len = 1
        wr::<u32>(mp + 0x10c, 1); // cap = 1 (forces grow on 2nd item)
        wr::<usize>(g_230, mp);

        // Input: 2 items (each 16 bytes: u32 handle at +0, u64 payload at +8)
        let mut inputs = [0u64; 4];
        let ip = inputs.as_mut_ptr() as usize;
        wr::<u32>(ip, 0); // hi=0, lo=0 -> matches existing entry 0
        wr::<u64>(ip + 0x08, 0xaaaa_0001);
        wr::<u32>(ip + 0x10, 1); // hi=0, lo=1 -> inserts new entry 1
        wr::<u64>(ip + 0x18, 0xbbbb_0002);

        f(ip, 2);

        // Normalize pointer fields in vec_entries relative to ti_base
        vec_entries[0] = (vec_entries[0] as usize).wrapping_sub(ti_base) as u64;
        vec_entries[3] = (vec_entries[3] as usize).wrapping_sub(ti_base) as u64;

        (
            vec_entries,
            rd::<u32>(mp + 0x108),
            rd::<u32>(ti_base + 0x4c),
            rd::<u32>(ti_base + 0x98 + 0x4c),
            GROW_CALLS,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_230, s_230);
    wr::<i32>(g_180, s_180);
    wr::<usize>(g_0a8, s_0a8);

    crate::diff_eq!(o, n, "1423e6ef0 out");
    Ok(1)
}

pub unsafe fn diff_r_14241d340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT_MEM: [u64; 2] = [0; 2];
    static mut E210_CALLED: u32 = 0;
    static mut VFN_LOG: u64 = 0;

    unsafe extern "win64" fn mock_ea50(_map: usize, key_ptr: usize) -> usize {
        SLOT_MEM[0] = rd::<u64>(key_ptr);
        core::ptr::addr_of_mut!(SLOT_MEM) as usize
    }
    unsafe extern "win64" fn mock_e210(_p1: usize, _rbx40: usize) {
        E210_CALLED += 1;
    }
    unsafe extern "win64" fn mock_vfn_60(
        _this: usize,
        idx: u32,
        key: u64,
        p3: u64,
        _p1: usize,
    ) {
        VFN_LOG = (idx as u64) ^ key ^ p3;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4139_ea50, mock_ea50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4139_e210, mock_e210 as *const ())?;

    let run = |f: F, can_alloc: bool| -> (bool, u32, u32, [usize; 2], u32, u64) {
        SLOT_MEM = [0; 2];
        E210_CALLED = 0;
        VFN_LOG = 0;

        let mut vtbl = [0usize; 16];
        vtbl[0x60 / 8] = mock_vfn_60 as *const () as usize;

        let mut owner = [0u8; 0x30];
        let owner_p = owner.as_mut_ptr() as usize;
        wr::<u32>(owner_p + 0x28, 5);

        let mut prev_node = [0u8; 0x20];
        let mut next_node = [0u8; 0x20];
        let prev_p = prev_node.as_mut_ptr() as usize;
        let next_p = next_node.as_mut_ptr() as usize;

        let mut r14_obj = [0u8; 0x70];
        let r14_p = r14_obj.as_mut_ptr() as usize;
        wr::<usize>(r14_p, vtbl.as_ptr() as usize);
        wr::<u32>(r14_p + 0x20, 0);
        wr::<usize>(r14_p + 0x48, prev_p);
        wr::<usize>(r14_p + 0x50, next_p);
        wr::<usize>(r14_p + 0x58, owner_p);

        let mut other_obj = [0u8; 0x30];
        let other_p = other_obj.as_mut_ptr() as usize;
        wr::<u32>(other_p + 0x20, 1);

        let mut arr_c8: [usize; 2] = [r14_p, other_p];
        let ptr_node: [usize; 1] = [r14_p];

        let mut p1 = vec![0u8; 0x160];
        let p = p1.as_mut_ptr() as usize;
        wr::<u32>(p + 0x28, 0);
        wr::<i32>(p + 0x68, if can_alloc { 1 } else { 0 });
        wr::<usize>(p + 0x90, ptr_node.as_ptr() as usize);
        wr::<usize>(p + 0x98, 0);
        wr::<i32>(p + 0xa8, if can_alloc { 1 } else { 0 });
        wr::<usize>(p + 0xc8, arr_c8.as_mut_ptr() as usize);
        wr::<u32>(p + 0x148, u32::MAX);

        let ret = f(p, 0x1234_5678, 0x9abc_def0);
        let rel_c8 = [
            if arr_c8[0] == r14_p { 1 } else { 2 },
            if arr_c8[1] == r14_p { 1 } else { 2 },
        ];
        (
            ret == r14_p,
            rd::<u32>(other_p + 0x20),
            rd::<u32>(owner_p + 0x28),
            rel_c8,
            E210_CALLED,
            VFN_LOG,
        )
    };

    for can_alloc in [false, true] {
        let o = run(f_o, can_alloc);
        let n = run(f_n, can_alloc);
        crate::diff_eq!(o, n, "14241d340 can_alloc={can_alloc}");
    }
    Ok(2)
}

pub unsafe fn diff_r_14241d5c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, query_key: u64| -> (u64, u32, u32) {
        // 4 buckets of 16 bytes each (key, value):
        // Bucket 1: tombstone (0xffff_fffe)
        // Bucket 2: valid key 0x101 (collides with bucket 1 -> probes to bucket 1 + 1 = 2)
        // Bucket 0, 3: empty (0xffff_ffff)
        let buckets: [u64; 8] = [
            u64::MAX,
            0,
            u64::MAX - 1,
            0xdead,
            0x101,
            0x7777_8888,
            u64::MAX,
            0,
        ];

        let mut p1 = vec![0u8; 0x160];
        let p = p1.as_mut_ptr() as usize;
        wr::<u64>(p + 0x100, u64::MAX - 1); // tombstone key
        wr::<u64>(p + 0x108, 1); // deleted count
        wr::<u64>(p + 0x110, 2); // used count
        wr::<u64>(p + 0x118, 4); // capacity
        wr::<u64>(p + 0x120, u64::MAX); // empty key
        wr::<usize>(p + 0x130, buckets.as_ptr() as usize);
        wr::<u32>(p + 0x148, u32::MAX);
        wr::<u32>(p + 0x14c, 0);

        let ret = f(p, query_key);
        (ret, rd::<u32>(p + 0x148), rd::<u32>(p + 0x14c))
    };

    for k in [0x101u64, 0x201, u64::MAX - 1] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "14241d5c0 k={k:#x}");
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_4222_c360, "r_14222c360", rend::cauldron_hot_x6::r_14222c360 as *const (), rend::cauldron_hot_x6::diff_r_14222c360),
// Replacement::verified(0x1_4222_cad0, "unknown_14222cad0", rend::cauldron_hot_x6::unknown_14222cad0 as *const (), rend::cauldron_hot_x6::diff_unknown_14222cad0),
// Replacement::verified_throws(0x1_4225_abc0, "r_14225abc0", rend::cauldron_hot_x6::r_14225abc0 as *const (), Throws::Caught, rend::cauldron_hot_x6::diff_r_14225abc0),
// Replacement::verified(0x1_4225_be60, "r_14225be60", rend::cauldron_hot_x6::r_14225be60 as *const (), rend::cauldron_hot_x6::diff_r_14225be60),
// Replacement::verified_throws(0x1_4230_3ab0, "unknown_142303ab0", rend::cauldron_hot_x6::unknown_142303ab0 as *const (), Throws::Caught, rend::cauldron_hot_x6::diff_unknown_142303ab0),
// Replacement::verified_throws(0x1_4233_6540, "rend_142336540", rend::cauldron_hot_x6::rend_142336540 as *const (), Throws::Caught, rend::cauldron_hot_x6::diff_rend_142336540),
// Replacement::verified(0x1_4233_8090, "rend_142338090", rend::cauldron_hot_x6::rend_142338090 as *const (), rend::cauldron_hot_x6::diff_rend_142338090),
// Replacement::verified_throws(0x1_4233_e9d0, "rend_14233e9d0", rend::cauldron_hot_x6::rend_14233e9d0 as *const (), Throws::Discard, rend::cauldron_hot_x6::diff_rend_14233e9d0),
// Replacement::verified(0x1_4235_0560, "unknown_142350560", rend::cauldron_hot_x6::unknown_142350560 as *const (), rend::cauldron_hot_x6::diff_unknown_142350560),
// Replacement::verified(0x1_423e_6ef0, "unknown_1423e6ef0", rend::cauldron_hot_x6::unknown_1423e6ef0 as *const (), rend::cauldron_hot_x6::diff_unknown_1423e6ef0),
// Replacement::verified_throws(0x1_4241_d340, "r_14241d340", rend::cauldron_hot_x6::r_14241d340 as *const (), Throws::Caught, rend::cauldron_hot_x6::diff_r_14241d340),
// Replacement::verified(0x1_4241_d5c0, "r_14241d5c0", rend::cauldron_hot_x6::r_14241d5c0 as *const (), rend::cauldron_hot_x6::diff_r_14241d5c0),
