//! Decompiled hot functions shard Y3 (`0x1420bb3a0..0x142213ae0`).

use crate::exe::{self, rd, wr};

/// `unknown_1420bb3a0`
/// Original: 0x1420bb3a0 (1660 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420bb3a0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: usize,
    param_5: usize,
    param_6: u64,
    param_7: usize,
) -> usize {
    let init_len = rd::<u32>(param_7 + 0x10) as u64;
    let mut vec_20: [u64; 3] = [
        rd::<u64>(param_7),
        rd::<u64>(param_7 + 0x08),
        (init_len & 0xffff_ffff) | ((rd::<u32>(param_7 + 0x14) as u64) << 32),
    ];
    let vec_ptr = vec_20.as_mut_ptr() as usize;
    wr::<u64>(param_7 + 0x08, 0);
    wr::<u64>(param_7 + 0x10, 0);

    let new_total = rd::<u64>(param_2 + 0x18).wrapping_add(init_len);
    let init_cap = rd::<u32>(vec_ptr + 0x14) as u64;
    if new_total > init_cap {
        exe::call_va!(0x1_411f_2570, (vec_ptr, new_total));
    }
    wr::<u32>(vec_ptr + 0x10, new_total as u32);

    let g_2f8_slot = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let g_2f8 = rd::<usize>(g_2f8_slot);
    let s_pair_parent = exe::ptr::<u8>(0x1_433d_d9c0) as usize;
    let sched_0 = rd::<usize>(g_2f8 + 0x10);
    let worker_0 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let mut job_counter_38: [u64; 2] = [0; 2];
    let jc_ptr = job_counter_38.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_4248_5aa0,
        (
            rd::<usize>(sched_0.wrapping_add(worker_0 * 0x40 + 0x48)).wrapping_add(0x40),
            jc_ptr
        )
    );
    let jc_obj = rd::<usize>(jc_ptr);
    let _ = crate::simd_ops::atomic_xchg_u32(jc_obj as u64, 1);
    let jc_obj = rd::<usize>(jc_ptr);
    wr::<u64>(jc_obj + 0x08, 0);
    wr::<usize>(jc_obj + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(jc_obj + 0x18, s_pair_parent);
    wr::<u64>(jc_obj + 0x20, 0x26);
    if crate::simd_ops::atomic_cmpxchg_u8((jc_obj + 0x04) as u64, 1, 3).0 {
        let jc_obj2 = rd::<usize>(jc_ptr);
        let _ = crate::simd_ops::atomic_xadd_u32(jc_obj2 as u64, 0xffff);
    }
    wr::<u8>(jc_ptr + 0x0a, 1);

    let init_byte_off = init_len.wrapping_mul(8);
    let r12_span_ptr = rd::<usize>(vec_ptr + 0x08).wrapping_add(init_byte_off as usize);
    let r15_span_len = (new_total as u32 as u64).wrapping_sub(init_len);
    let mut rdi_page = rd::<usize>(param_2 + 0x08);
    let mut r13_g2f8 = rd::<usize>(g_2f8_slot);
    let mut rsi_off: u64 = 0;
    let mut slot_60: u64 = 0;

    if rdi_page != 0 {
        let fn_bb2f0 = exe::ptr::<u8>(0x1_420b_b2f0) as usize as u64;
        let vtbl_c38 = exe::ptr::<u8>(0x1_433d_dc38) as usize;
        while rdi_page != 0 {
            slot_60 = rdi_page as u64;
            let sched = rd::<usize>(r13_g2f8 + 0x10);
            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let mut job_a8: [u64; 2] = [0; 2];
            exe::call_va!(
                0x1_4248_5aa0,
                (
                    rd::<usize>(sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x80),
                    job_a8.as_mut_ptr() as usize
                )
            );
            let job = job_a8[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(job as u64, 1);
            wr::<u64>(job + 0x08, 0);
            wr::<usize>(job + 0x10, vtbl_c38);
            wr::<u64>(job + 0x18, slot_60);
            wr::<u64>(job + 0x20, 3);
            wr::<u64>(job + 0x28, fn_bb2f0);
            wr::<u64>(job + 0x30, r15_span_len);
            wr::<u64>(job + 0x38, r12_span_ptr as u64);
            wr::<u64>(job + 0x40, rsi_off);
            let parent_jc = rd::<usize>(jc_ptr);
            let _ = crate::simd_ops::atomic_xadd_u32(parent_jc as u64, 1);
            let parent_jc = rd::<usize>(jc_ptr);
            wr::<usize>(job + 0x08, parent_jc);
            if rd::<u32>(job) == 1 {
                let sched2 = rd::<usize>(r13_g2f8 + 0x10);
                exe::call_va!(0x1_4248_6460, (sched2, job));
                let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
                let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
            }
            for _ in 0..3 {
                if rdi_page == 0 {
                    break;
                }
                rsi_off = rsi_off.wrapping_add(rd::<u64>(rdi_page + 0x08));
                rdi_page = rd::<usize>(rdi_page);
            }
        }
        r13_g2f8 = rd::<usize>(g_2f8_slot);
    }

    let span2_ptr = rd::<usize>(vec_ptr + 0x08).wrapping_add(init_byte_off as usize);
    let span2_len = (rd::<u32>(vec_ptr + 0x10) as u64).wrapping_sub(init_len);
    let mut rdi_page2 = rd::<usize>(param_2 + 0x20);
    let mut rsi_off2: u64 = 0;

    if rdi_page2 != 0 {
        let fn_bb260 = exe::ptr::<u8>(0x1_420b_b260) as usize as u64;
        let vtbl_ea8 = exe::ptr::<u8>(0x1_433d_dea8) as usize;
        while rdi_page2 != 0 {
            slot_60 = rdi_page2 as u64;
            let sched = rd::<usize>(r13_g2f8 + 0x10);
            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let mut job_110: [u64; 2] = [0; 2];
            exe::call_va!(
                0x1_4248_5aa0,
                (
                    rd::<usize>(sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x80),
                    job_110.as_mut_ptr() as usize
                )
            );
            let job = job_110[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(job as u64, 1);
            wr::<u64>(job + 0x08, 0);
            wr::<usize>(job + 0x10, vtbl_ea8);
            wr::<u64>(job + 0x18, slot_60);
            wr::<u64>(job + 0x20, 3);
            wr::<u64>(job + 0x28, fn_bb260);
            wr::<u64>(job + 0x30, span2_len);
            wr::<u64>(job + 0x38, span2_ptr as u64);
            wr::<u64>(job + 0x40, rsi_off2);
            let parent_jc = rd::<usize>(jc_ptr);
            let _ = crate::simd_ops::atomic_xadd_u32(parent_jc as u64, 1);
            let parent_jc = rd::<usize>(jc_ptr);
            wr::<usize>(job + 0x08, parent_jc);
            if rd::<u32>(job) == 1 {
                let sched2 = rd::<usize>(r13_g2f8 + 0x10);
                exe::call_va!(0x1_4248_6460, (sched2, job));
                let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
                let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
            }
            for _ in 0..3 {
                if rdi_page2 == 0 {
                    break;
                }
                rsi_off2 = rsi_off2.wrapping_add(rd::<u64>(rdi_page2 + 0x08));
                rdi_page2 = rd::<usize>(rdi_page2);
            }
        }
    }

    exe::call_va!(0x1_4248_5680, (g_2f8, jc_ptr));
    let mut cur_len = rd::<u32>(vec_ptr + 0x10);
    let mut data_ptr = rd::<usize>(vec_ptr + 0x08);
    let cmp_fn = exe::ptr::<u8>(0x1_420a_fc40) as usize;
    exe::call_va!(0x1_420a_f9a0, (data_ptr, cur_len, cmp_fn));

    let mut end_ptr = data_ptr.wrapping_add((cur_len as usize) * 8);
    if data_ptr != end_ptr && data_ptr.wrapping_add(8) != end_ptr {
        let mut prev_ptr = data_ptr;
        let mut scan_ptr = data_ptr.wrapping_add(8);
        let mut prev_val = rd::<u64>(data_ptr);
        loop {
            let cur_val = rd::<u64>(scan_ptr);
            if prev_val == cur_val {
                let mut read_ptr = scan_ptr.wrapping_add(8);
                while read_ptr != end_ptr {
                    let v = rd::<u64>(read_ptr);
                    if prev_val != v {
                        prev_ptr = prev_ptr.wrapping_add(8);
                        wr::<u64>(prev_ptr, v);
                        prev_val = v;
                    }
                    read_ptr = read_ptr.wrapping_add(8);
                }
                end_ptr = prev_ptr.wrapping_add(8);
                break;
            }
            prev_ptr = scan_ptr;
            prev_val = cur_val;
            scan_ptr = scan_ptr.wrapping_add(8);
            if scan_ptr == end_ptr {
                break;
            }
        }
    }

    let uniq_count = end_ptr.wrapping_sub(data_ptr) >> 3;
    let cur_cap = rd::<u32>(vec_ptr + 0x14) as usize;
    if uniq_count > cur_cap {
        exe::call_va!(0x1_411f_2570, (vec_ptr, uniq_count as u64));
        cur_len = rd::<u32>(vec_ptr + 0x10);
        data_ptr = rd::<usize>(vec_ptr + 0x08);
    }
    let old_len = cur_len as usize;
    if uniq_count > old_len {
        let diff = uniq_count - old_len;
        if diff != 0 {
            let _ = exe::call_va!(
                0x1_4183_acc2,
                (
                    data_ptr.wrapping_add(old_len * 8),
                    0u32,
                    (diff * 8) as u64
                ) -> usize
            );
        }
    }
    let u_len = uniq_count as u32;
    wr::<u32>(vec_ptr + 0x10, u_len);

    let span5_len = rd::<u32>(param_5 + 0x08) as u64;
    let span5_ptr = rd::<u64>(param_5);
    let bitset_ptr = rd::<usize>(param_4);
    let s_move_parent = exe::ptr::<u8>(0x1_433d_d998) as usize;
    let sched_m = rd::<usize>(g_2f8 + 0x10);
    let w_idx_m = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    exe::call_va!(
        0x1_4248_5aa0,
        (
            rd::<usize>(sched_m.wrapping_add(w_idx_m * 0x40 + 0x48)).wrapping_add(0x40),
            param_1
        )
    );
    let out_jc = rd::<usize>(param_1);
    let _ = crate::simd_ops::atomic_xchg_u32(out_jc as u64, 1);
    wr::<u64>(out_jc + 0x08, 0);
    wr::<usize>(out_jc + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(out_jc + 0x18, s_move_parent);
    wr::<u64>(out_jc + 0x20, 0x25);
    let out_jc = rd::<usize>(param_1);
    if crate::simd_ops::atomic_cmpxchg_u8((out_jc + 0x04) as u64, 1, 3).0 {
        let _ = crate::simd_ops::atomic_xadd_u32(out_jc as u64, 0xffff);
    }
    wr::<u8>(param_1 + 0x0a, 1);

    let mut ebx: u32 = 0;
    if u_len != 0 {
        let mut r8_data = rd::<usize>(vec_ptr + 0x08);
        let vtbl_cb0 = exe::ptr::<u8>(0x1_433d_dcb0) as usize;
        while ebx < u_len {
            let comp_id = rd::<u16>(r8_data.wrapping_add((ebx as usize) * 8 + 6));
            let run_start = ebx;
            ebx = ebx.wrapping_add(1);
            while ebx < u_len && rd::<u16>(r8_data.wrapping_add((ebx as usize) * 8 + 6)) == comp_id
            {
                ebx = ebx.wrapping_add(1);
            }
            let word = rd::<u64>(bitset_ptr.wrapping_add(((comp_id as usize) >> 6) * 8));
            if ((word >> (comp_id & 0x3f)) & 1) != 0 {
                slot_60 = (slot_60 & !0xffffu64) | (comp_id as u64);
                let g_cur = rd::<usize>(g_2f8_slot);
                let sched = rd::<usize>(g_cur + 0x10);
                let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                let mut job_98: [u64; 2] = [0; 2];
                exe::call_va!(
                    0x1_4248_5aa0,
                    (
                        rd::<usize>(sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x80),
                        job_98.as_mut_ptr() as usize
                    )
                );
                let job = job_98[0] as usize;
                let _ = crate::simd_ops::atomic_xchg_u32(job as u64, 1);
                wr::<u64>(job + 0x08, 0);
                wr::<usize>(job + 0x10, vtbl_cb0);
                wr::<u64>(job + 0x18, slot_60);
                wr::<u64>(job + 0x20, param_4 as u64);
                wr::<u64>(job + 0x28, param_3);
                wr::<u64>(job + 0x30, param_6);
                wr::<u32>(job + 0x38, run_start);
                wr::<u32>(job + 0x3c, ebx);
                wr::<u64>(job + 0x40, rd::<u32>(vec_ptr + 0x10) as u64);
                wr::<u64>(job + 0x48, r8_data as u64);
                wr::<u64>(job + 0x50, span5_len);
                wr::<u64>(job + 0x58, span5_ptr);
                let parent_jc = rd::<usize>(param_1);
                let _ = crate::simd_ops::atomic_xadd_u32(parent_jc as u64, 1);
                wr::<usize>(job + 0x08, parent_jc);
                if rd::<u32>(job) == 1 {
                    let sched2 = rd::<usize>(g_cur + 0x10);
                    exe::call_va!(0x1_4248_6460, (sched2, job));
                    let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
                    let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
                }
                r8_data = rd::<usize>(vec_ptr + 0x08);
            }
        }
    }
    param_1
}

#[inline]
unsafe fn advance_paged_cursor(mut page: usize, mut idx: u64, chunk: u64) -> (usize, u64) {
    if page != 0 {
        let mut rem = chunk;
        let mut avail = rd::<u64>(page + 0x08).wrapping_sub(idx);
        if (rem as i64) >= (avail as i64) {
            loop {
                rem = rem.wrapping_sub(avail);
                let next = rd::<usize>(page);
                if next == 0 {
                    return (page, rd::<u64>(page + 0x08));
                }
                page = next;
                avail = rd::<u64>(page + 0x08);
                idx = 0;
                if (rem as i64) < (avail as i64) {
                    break;
                }
            }
        }
        idx = idx.wrapping_add(rem);
    }
    (page, idx)
}

#[inline]
unsafe fn execute_add_items(
    param_1: usize,
    mut count: u64,
    mut page1: usize,
    mut idx1: u64,
    mut page2: usize,
    mut idx2: u64,
    mut page3: usize,
    mut idx3: u64,
) {
    let p1_08 = rd::<usize>(param_1 + 0x08);
    let p1_10 = rd::<u64>(param_1 + 0x10);
    let p1_18 = rd::<usize>(param_1 + 0x18);
    let p1_28 = rd::<usize>(param_1 + 0x28);
    let p1_38 = rd::<usize>(param_1 + 0x38);
    let p1_48 = rd::<usize>(param_1 + 0x48);
    let p1_50 = rd::<usize>(param_1 + 0x50);
    let p1_58 = rd::<usize>(param_1 + 0x58);

    while count != 0 {
        let cur_page2 = page2;
        let cur_idx2 = idx2;
        idx2 = idx2.wrapping_add(1);
        if idx2 == rd::<u64>(page2 + 0x08) {
            let next = rd::<usize>(page2);
            if next != 0 {
                page2 = next;
                idx2 = 0;
            }
        }
        let comp_id = rd::<u16>(cur_page2.wrapping_add((cur_idx2 as usize) * 2 + 0x10));

        let cur_page3 = page3;
        let cur_idx3 = idx3;
        idx3 = idx3.wrapping_add(1);
        if idx3 == rd::<u64>(page3 + 0x08) {
            let next = rd::<usize>(page3);
            if next != 0 {
                page3 = next;
                idx3 = 0;
            }
        }
        let elem3 = rd::<usize>(cur_page3.wrapping_add((cur_idx3 as usize) * 8 + 0x10));

        let cur_page1 = page1;
        let cur_idx1 = idx1;
        idx1 = idx1.wrapping_add(1);
        if idx1 == rd::<u64>(page1 + 0x08) {
            let next = rd::<usize>(page1);
            if next != 0 {
                page1 = next;
                idx1 = 0;
            }
        }

        if elem3 != 0 {
            let key = rd::<u32>(cur_page1.wrapping_add((cur_idx1 as usize) * 4 + 0x10));
            let mut r9: usize = 0;
            let mut rem = p1_10 as usize;
            while rem != 0 {
                let half = rem >> 1;
                let next_r9 = r9.wrapping_sub(half).wrapping_add(rem);
                let mid = r9.wrapping_add(half);
                if key >= rd::<u32>(p1_18.wrapping_add(mid * 4)) {
                    r9 = next_r9;
                }
                rem = half;
            }
            let u4 = rd::<u32>(p1_38.wrapping_add(r9 * 4).wrapping_sub(4));
            if u4 != 0xffff_ffff {
                let arch = rd::<u16>(p1_28.wrapping_add((u4 as usize) * 8 + 4));
                if arch != 0xffff {
                    let arch_usize = arch as usize;
                    let r11 = p1_08.wrapping_add(arch_usize * 0x30);
                    let search_len = rd::<u32>(p1_50.wrapping_add(arch_usize * 0x20 + 0x1806c));
                    let search_ptr = rd::<u64>(p1_50.wrapping_add(arch_usize * 0x20 + 0x18058));
                    let found = exe::call_va!(
                        0x1_4248_9ee0,
                        (search_ptr, search_len, comp_id) -> i32
                    );
                    let fn_ptr =
                        rd::<usize>(rd::<usize>(p1_58).wrapping_add((comp_id as usize) * 0x50 + 0x30));
                    let comp_off = rd::<u32>(
                        rd::<usize>(r11 + 0x18).wrapping_add((found as isize as usize) * 4),
                    ) as usize;
                    let dst_ptr = rd::<usize>(r11 + 0x08).wrapping_add(comp_off);
                    let idx_arg = rd::<u32>(p1_48.wrapping_add((u4 as usize) * 4));
                    exe::call_va!(fn_ptr, (elem3, dst_ptr, idx_arg));
                }
            }
        }
        count -= 1;
    }
}

/// `unknown_1420bbd40`
/// Original: 0x1420bbd40 (1812 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420bbd40(param_1: usize, param_2: usize) {
    let adds = rd::<usize>(param_1 + 0x60);
    let total = rd::<u64>(adds + 0x18);
    let mut page1 = rd::<usize>(adds + 0x08);
    let mut page2 = rd::<usize>(adds + 0x20);
    let mut page3 = rd::<usize>(adds + 0x38);
    let mut idx1: u64 = 0;
    let mut idx2: u64 = 0;
    let mut idx3: u64 = 0;

    if total <= 0x2710 {
        if total != 0 {
            execute_add_items(param_1, total, page1, idx1, page2, idx2, page3, idx3);
        }
    } else {
        let chunk = total >> 2;
        let g_2f8_slot = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
        let vtbl_cf8 = exe::ptr::<u8>(0x1_433d_dcf8) as usize;
        for _ in 0..4 {
            let g_2f8 = rd::<usize>(g_2f8_slot);
            let sched = rd::<usize>(g_2f8 + 0x10);
            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let mut job_118: [u64; 2] = [0; 2];
            exe::call_va!(
                0x1_4248_5aa0,
                (
                    rd::<usize>(sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x80),
                    job_118.as_mut_ptr() as usize
                )
            );
            let job = job_118[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(job as u64, 1);
            wr::<u64>(job + 0x08, 0);
            wr::<usize>(job + 0x10, vtbl_cf8);
            wr::<usize>(job + 0x18, param_1);
            wr::<u64>(job + 0x20, chunk);
            wr::<usize>(job + 0x28, page1);
            wr::<u64>(job + 0x30, idx1);
            wr::<usize>(job + 0x38, page2);
            wr::<u64>(job + 0x40, idx2);
            wr::<usize>(job + 0x48, page3);
            wr::<u64>(job + 0x50, idx3);
            let parent_jc = rd::<usize>(param_2);
            let _ = crate::simd_ops::atomic_xadd_u32(parent_jc as u64, 1);
            wr::<usize>(job + 0x08, parent_jc);
            if rd::<u32>(job) == 1 {
                let sched2 = rd::<usize>(g_2f8 + 0x10);
                exe::call_va!(0x1_4248_6460, (sched2, job));
                let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
                let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
            }
            (page1, idx1) = advance_paged_cursor(page1, idx1, chunk);
            (page2, idx2) = advance_paged_cursor(page2, idx2, chunk);
            (page3, idx3) = advance_paged_cursor(page3, idx3, chunk);
        }
        let rem = total & 3;
        if rem != 0 {
            execute_add_items(param_1, rem, page1, idx1, page2, idx2, page3, idx3);
        }
    }
}

#[inline]
unsafe fn call_1420be3c0(
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    a8: usize,
    a9: usize,
    a10: u64,
    a11: usize,
    a12: u64,
    a13: usize,
    a14: usize,
    a15: u64,
    a16: usize,
    a17: usize,
    a18: u64,
) {
    // The registered target is the 18-parameter Rust decompilation
    // (`cauldron_hot_z5a::unknown_1420be3c0`), so a direct call passes every arg.
    if let Some(addr) = exe::lookup_native_va(0x1_420b_e3c0) {
        let f: unsafe extern "win64" fn(
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            u64,
            usize,
            u64,
            usize,
            usize,
            u64,
            usize,
            usize,
            u64,
        ) -> u64 = core::mem::transmute(addr);
        let _ = f(
            a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18,
        );
        return;
    }
    #[cfg(target_arch = "x86_64")]
    {
        let addr = exe::to_mapped_addr(0x1_420b_e3c0);
        let f: unsafe extern "win64" fn(
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            u64,
            usize,
            u64,
            usize,
            usize,
            u64,
            usize,
            usize,
            u64,
        ) -> u64 = core::mem::transmute(addr);
        let _ = f(
            a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18,
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let buf: [u64; 20] = [
            0, 0, 0, 0, 0, 0, 0, 0, a9 as u64, a10, a11 as u64, a12, a13 as u64, a14 as u64, a15,
            a16 as u64, a17 as u64, a18, 0, 0,
        ];
        let base = buf.as_ptr() as u64;
        crate::emu::set_outgoing_rsp(base, base + 160);
        let _ = crate::emu::call_va8(
            0x1_420b_e3c0,
            a1 as u64,
            a2 as u64,
            a3 as u64,
            a4 as u64,
            a5 as u64,
            a6 as u64,
            a7 as u64,
            a8 as u64,
        );
    }
}

/// `unknown_1420bc770`
/// Original: 0x1420bc770 (1529 bytes, Throws::Caught)
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn unknown_1420bc770(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
    param_8: usize,
    param_9: u64,
    param_10: usize,
    param_11: u64,
    param_12: usize,
    param_13: usize,
    param_14: u64,
    param_15: usize,
    param_16: usize,
    param_17_in: u64,
) -> usize {
    let (in_rsp, in_limit) = crate::emu::take_incoming_rsp();
    let param_17 = if in_rsp != 0 && in_rsp.wrapping_add(0x88) <= in_limit {
        rd::<u64>((in_rsp + 0x80) as usize)
    } else {
        param_17_in
    };

    let s_name = exe::ptr::<u8>(0x1_433d_dae0) as usize;
    let g_2f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let sched = rd::<usize>(g_2f8 + 0x10);
    let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let mut job_handle: [u64; 2] = [0; 2];
    let jh_ptr = job_handle.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_4248_5aa0,
        (
            rd::<usize>(sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x40),
            jh_ptr
        )
    );
    let jc = rd::<usize>(jh_ptr);
    let _ = crate::simd_ops::atomic_xchg_u32(jc as u64, 1);
    wr::<u64>(jc + 0x08, 0);
    wr::<usize>(jc + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(jc + 0x18, s_name);
    wr::<u64>(jc + 0x20, 0x24);
    let jc = rd::<usize>(jh_ptr);
    if crate::simd_ops::atomic_cmpxchg_u8((jc + 0x04) as u64, 1, 3).0 {
        let _ = crate::simd_ops::atomic_xadd_u32(jc as u64, 0xffff);
    }
    wr::<u8>(jh_ptr + 0x0a, 1);

    let mut vec_m78: [u64; 3] = [param_2 as u64, 0, 0];
    let mut tmp_p7 = rd::<[u64; 2]>(param_7);
    let mut out_20: [u64; 4] = [0; 4];
    exe::call_va!(
        0x1_420b_dea0,
        (
            out_20.as_mut_ptr() as usize,
            param_2,
            param_11,
            0u64,
            0u64,
            tmp_p7.as_mut_ptr() as usize
        )
    );

    let r12 = rd::<u64>(param_6);
    let r13 = rd::<usize>(param_6 + 0x08);
    let dst = exe::call_va!(0x1_411f_11e0, (param_2, r12.wrapping_mul(2), 2u32) -> usize);
    let r12_u32 = (r12 as u32) as u64;
    let _ = exe::call_va!(0x1_4183_acc2, (dst, 0xffu32, r12_u32.wrapping_mul(2)) -> usize);
    if r12 != 0 {
        for i in 0..(r12 as usize) {
            let found = exe::call_va!(
                0x1_411f_1520,
                (
                    rd::<u64>(param_16),
                    rd::<u32>(param_16 + 0x08),
                    rd::<u64>(r13.wrapping_add(i * 8))
                ) -> i32
            );
            if found != -1 {
                let v = rd::<u16>(
                    rd::<usize>(param_16 + 0x10).wrapping_add((found as isize as usize) * 2),
                );
                wr::<u16>(dst.wrapping_add(i * 2), v);
            }
        }
    }

    let mut span_dst: [u64; 2] = [r12_u32, dst as u64];
    let mut span_p6: [u64; 2] = [r12, r13 as u64];
    let mut span_p10 = rd::<[u64; 2]>(param_10);
    let mut span_p8 = rd::<[u64; 2]>(param_8);
    let mut span_p7 = rd::<[u64; 2]>(param_7);
    let mut span_p5 = rd::<[u64; 2]>(param_5);
    let mut span_p4 = rd::<[u64; 2]>(param_4);
    let mut span_p3 = rd::<[u64; 2]>(param_3);
    let mut be3c0_out: [u64; 15] = [0; 15];

    call_1420be3c0(
        be3c0_out.as_mut_ptr() as usize,
        param_2,
        span_p3.as_mut_ptr() as usize,
        span_p4.as_mut_ptr() as usize,
        span_p5.as_mut_ptr() as usize,
        span_p6.as_mut_ptr() as usize,
        span_dst.as_mut_ptr() as usize,
        span_p7.as_mut_ptr() as usize,
        span_p8.as_mut_ptr() as usize,
        param_9,
        span_p10.as_mut_ptr() as usize,
        param_11,
        param_12,
        param_13,
        param_14,
        jh_ptr,
        vec_m78.as_mut_ptr() as usize,
        param_17,
    );

    let mut span_m78: [u64; 2] = [(vec_m78[2] as u32) as u64, vec_m78[1]];
    let mut span_dst2: [u64; 2] = [r12_u32, dst as u64];
    let mut span_p15 = rd::<[u64; 2]>(param_15);
    let mut bcd80_out: [u64; 3] = [0; 3];
    exe::call_va!(
        0x1_420b_cd80,
        (
            bcd80_out.as_mut_ptr() as usize,
            param_2,
            span_dst2.as_mut_ptr() as usize,
            param_13.wrapping_add(0x40),
            rd::<u64>(param_13),
            param_13.wrapping_add(0x18),
            param_13.wrapping_add(0x30),
            param_11,
            be3c0_out.as_mut_ptr() as usize,
            span_m78.as_mut_ptr() as usize,
            span_p15.as_mut_ptr() as usize
        )
    );

    let r10_len = be3c0_out[5] as u32;
    let r9_ptr = be3c0_out[4];
    let r14_m70 = vec_m78[1] as usize;
    let r12_cnt = (out_20[2] as u32) as usize;
    if r12_cnt != 0 {
        let rdx_base = rd::<usize>(rd::<usize>(param_13));
        let rcx_arr = out_20[1] as usize;
        let mut rdi: usize = 0;
        while rdi < r12_cnt {
            let entry_ptr = rcx_arr.wrapping_add(rdi * 16);
            let w0 = rd::<u64>(entry_ptr);
            let w1 = rd::<usize>(entry_ptr + 0x08);
            if w1 == 0 {
                rdi += 1;
                continue;
            }
            let comp_id = (w0 >> 32) as u16;
            let rsi_comp = rdx_base.wrapping_add((comp_id as usize) * 0x50);
            let r13_idx = (w0 as u32) as usize;
            let r8_arch = rd::<u16>(dst.wrapping_add(r13_idx * 2));
            let p15_hi = rd::<usize>(param_15 + 0x08);
            let found1 = exe::call_va!(0x1_4248_9ee0, (r9_ptr, r10_len, r8_arch) -> i32);
            let r11 = if (found1 as isize) == -1 {
                p15_hi.wrapping_add((r8_arch as usize) * 0x30)
            } else {
                r14_m70.wrapping_add((found1 as isize as usize) * 0x30)
            };
            let arch_off = (r8_arch as usize) * 0x20;
            let search_len = rd::<u32>(param_12.wrapping_add(arch_off + 0x1806c));
            let search_ptr = rd::<u64>(param_12.wrapping_add(arch_off + 0x18058));
            let found2 = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, comp_id) -> i32);
            let comp_off =
                rd::<u32>(rd::<usize>(r11 + 0x18).wrapping_add((found2 as isize as usize) * 4))
                    as usize;
            let arg2_ptr = rd::<usize>(r11 + 0x08).wrapping_add(comp_off);
            let arg3_val = rd::<u32>((bcd80_out[1] as usize).wrapping_add(r13_idx * 4));
            exe::call_va!(rd::<usize>(rsi_comp + 0x30), (w1, arg2_ptr, arg3_val));
            rdi += 1;
            while rdi < r12_cnt {
                let next_ptr = rcx_arr.wrapping_add(rdi * 16);
                if rd::<u32>(next_ptr) != (w0 as u32) {
                    break;
                }
                if rd::<u16>(next_ptr + 0x04) != comp_id {
                    break;
                }
                exe::call_va!(
                    rd::<usize>(rsi_comp + 0x40),
                    (rd::<usize>(next_ptr + 0x08), arg2_ptr, arg3_val)
                );
                rdi += 1;
            }
        }
    }

    wr::<u64>(param_1, vec_m78[0]);
    wr::<u64>(param_1 + 0x08, vec_m78[1]);
    wr::<u64>(param_1 + 0x10, vec_m78[2]);
    wr::<u64>(param_1 + 0x18, be3c0_out[6]);
    wr::<u64>(param_1 + 0x20, be3c0_out[7]);
    wr::<u64>(param_1 + 0x28, be3c0_out[8]);
    wr::<u64>(param_1 + 0x30, be3c0_out[9]);
    wr::<u64>(param_1 + 0x38, be3c0_out[10]);
    wr::<u64>(param_1 + 0x40, be3c0_out[11]);
    wr::<u64>(param_1 + 0x48, be3c0_out[3]);
    wr::<u64>(param_1 + 0x50, be3c0_out[4]);
    wr::<u64>(param_1 + 0x58, be3c0_out[5]);
    wr::<u64>(param_1 + 0x60, job_handle[0]);
    wr::<u64>(param_1 + 0x68, job_handle[1]);
    wr::<u64>(param_1 + 0x70, be3c0_out[12]);
    wr::<u64>(param_1 + 0x78, be3c0_out[13]);
    wr::<u64>(param_1 + 0x80, be3c0_out[14]);
    param_1
}

/// `unknown_1420c2c10`
/// Original: 0x1420c2c10 (305 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420c2c10(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) -> usize {
    let count = rd::<u64>(param_3);
    wr::<usize>(param_1, param_2);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);

    let mut cur_len: u64 = 0;
    let mut data_ptr: usize = 0;
    if count != 0 {
        let byte_len = count.wrapping_mul(4);
        data_ptr = exe::call_va!(0x1_411f_11e0, (param_2, byte_len, 4u32) -> usize);
        if data_ptr == 0 {
            data_ptr = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), byte_len, 4u32) -> usize);
            let old_len = rd::<u32>(param_1 + 0x10);
            let old_ptr = rd::<usize>(param_1 + 0x08);
            if old_len != 0 {
                for i in 0..(old_len as usize) {
                    wr::<u32>(data_ptr.wrapping_add(i * 4), rd::<u32>(old_ptr.wrapping_add(i * 4)));
                }
            }
            cur_len = rd::<u32>(param_1 + 0x10) as u64;
        } else {
            cur_len = rd::<u32>(param_1 + 0x10) as u64;
        }
        wr::<usize>(param_1 + 0x08, data_ptr);
        wr::<u32>(param_1 + 0x14, count as u32);
    }

    if count > cur_len {
        let diff = count - cur_len;
        if diff != 0 {
            let _ = exe::call_va!(
                0x1_4183_acc2,
                (
                    data_ptr.wrapping_add((cur_len as usize) * 4),
                    0u32,
                    diff.wrapping_mul(4)
                ) -> usize
            );
        }
    }
    wr::<u32>(param_1 + 0x10, count as u32);

    if count != 0 {
        let mut src = rd::<usize>(param_3 + 0x08);
        let base_4 = rd::<usize>(param_4 + 0x08);
        for i in 0..(count as usize) {
            let hi = (rd::<u64>(src) >> 32) as u16;
            if hi == 0xffff {
                wr::<u32>(rd::<usize>(param_1 + 0x08).wrapping_add(i * 4), 0xffff_ffff);
            } else {
                let entry = base_4.wrapping_add((hi as usize) * 0x30);
                let old = rd::<u32>(entry + 0x28);
                wr::<u32>(entry + 0x28, old.wrapping_add(1));
                wr::<u32>(rd::<usize>(param_1 + 0x08).wrapping_add(i * 4), old);
            }
            src = src.wrapping_add(8);
        }
    }
    param_1
}

/// `unknown_1420c43d0`
/// Original: 0x1420c43d0 (1151 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420c43d0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: u64,
) {
    let count = rd::<u64>(param_1 + 0x60);
    if count == 0 {
        return;
    }

    let mut vec_c8: [u64; 3] = [param_5 as u64, 0, 0];
    let mut vec_b0: [u64; 3] = [param_5 as u64, 0, 0];
    let mut vec_88: [u64; 2] = [param_5 as u64, 0];

    let arr_80 = exe::call_va!(0x1_411f_11e0, (param_5, count.wrapping_mul(8), 8u32) -> usize);
    vec_88[1] = arr_80 as u64;

    let g_2f8_slot = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let g_2f8 = rd::<usize>(g_2f8_slot);
    let sched = rd::<usize>(g_2f8 + 0x10);
    let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let mut job_98: [u64; 2] = [0; 2];
    exe::call_va!(
        0x1_4248_5aa0,
        (
            rd::<usize>(sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x80),
            job_98.as_mut_ptr() as usize
        )
    );
    let job = job_98[0] as usize;
    let _ = crate::simd_ops::atomic_xchg_u32(job as u64, 1);
    wr::<u64>(job + 0x08, 0);
    wr::<usize>(job + 0x10, exe::ptr::<u8>(0x1_433d_dde8) as usize);
    wr::<u64>(job + 0x18, param_1 as u64);
    wr::<u64>(job + 0x20, rd::<u64>(param_2));
    wr::<u64>(job + 0x28, rd::<u64>(param_2 + 0x08));
    wr::<u64>(job + 0x30, param_3);
    wr::<usize>(job + 0x38, vec_c8.as_mut_ptr() as usize);
    wr::<usize>(job + 0x40, vec_b0.as_mut_ptr() as usize);
    wr::<u64>(job + 0x48, param_5 as u64);
    wr::<u64>(job + 0x50, param_7);
    if rd::<u32>(job) == 1 {
        let sched2 = rd::<usize>(g_2f8 + 0x10);
        exe::call_va!(0x1_4248_6460, (sched2, job));
        let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
        let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
    }

    let mut page_50 = rd::<usize>(param_1 + 0x50);
    let mut idx_50: u64 = 0;
    let mut page_68 = rd::<usize>(param_1 + 0x68);
    let mut idx_68: u64 = 0;

    for i in 0..(count as usize) {
        let cur_68 = page_68;
        let cur_i68 = idx_68;
        idx_68 = idx_68.wrapping_add(1);
        if idx_68 == rd::<u64>(page_68 + 0x08) {
            let next = rd::<usize>(page_68);
            if next != 0 {
                page_68 = next;
                idx_68 = 0;
            }
        }
        let lo16 = rd::<u16>(cur_68.wrapping_add((cur_i68 as usize) * 2 + 0x10)) as u64;

        let cur_50 = page_50;
        let cur_i50 = idx_50;
        idx_50 = idx_50.wrapping_add(1);
        if idx_50 == rd::<u64>(page_50 + 0x08) {
            let next = rd::<usize>(page_50);
            if next != 0 {
                page_50 = next;
                idx_50 = 0;
            }
        }
        let hi32 = rd::<u32>(cur_50.wrapping_add((cur_i50 as usize) * 4 + 0x10)) as u64;

        wr::<u64>(arr_80.wrapping_add(i * 8), (hi32 << 32) | lo16);
    }

    exe::call_va!(0x1_40fa_e550, (arr_80, count as u32));

    let mut dup_ptr: usize = 0;
    let mut dup_len: u32 = 0;
    let mut dup_cap: u32 = 0;
    let mut prev = rd::<u64>(arr_80);
    let mut scan = arr_80.wrapping_add(8);
    let end_scan = arr_80.wrapping_add((count as usize) * 8);
    while scan != end_scan {
        let val = rd::<u64>(scan);
        if val == prev {
            if dup_len == dup_cap {
                let old_bytes = (dup_len as u64) * 8;
                let min_cap = (dup_len as u64) + 1;
                let cur_cap = dup_cap as u64;
                let grown = cur_cap + (cur_cap >> 1);
                let new_cap = if grown < min_cap { min_cap } else { grown };
                let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 8u32) -> usize);
                let _ = exe::call_va!(0x1_4183_acc8, (new_buf, dup_ptr, old_bytes) -> usize);
                if dup_ptr != 0 {
                    exe::call_va!(0x1_4241_c190, (dup_ptr));
                }
                dup_ptr = new_buf;
                dup_cap = new_cap as u32;
            }
            wr::<u64>(dup_ptr.wrapping_add((dup_len as usize) * 8), val);
            dup_len = dup_len.wrapping_add(1);
        } else {
            prev = val;
        }
        scan = scan.wrapping_add(8);
    }

    exe::call_va!(
        0x1_4248_5680,
        (rd::<usize>(g_2f8_slot), job_98.as_mut_ptr() as usize)
    );

    let r15_cnt = dup_len as u64;
    wr::<usize>(param_4, param_5);
    exe::call_va!(0x1_411f_2320, (param_4, r15_cnt));
    let p4_data = rd::<usize>(param_4 + 0x08);
    let p4_len = rd::<u32>(param_4 + 0x10);
    for i in 0..(p4_len as usize) {
        wr::<u64>(
            p4_data.wrapping_add(i * 8),
            rd::<u64>(dup_ptr.wrapping_add(i * 8)),
        );
    }

    let c8_len = (vec_c8[2] as u32) as u64;
    let c8_ptr = vec_c8[1] as usize;
    let old_len1 = rd::<u32>(param_4 + 0x10) as u64;
    let new_len1 = old_len1.wrapping_add(c8_len);
    if new_len1 > (rd::<u32>(param_4 + 0x14) as u64) {
        exe::call_va!(0x1_411f_2380, (param_4, new_len1, 8u32));
    }
    wr::<u32>(param_4 + 0x10, new_len1 as u32);
    let dst1 = rd::<usize>(param_4 + 0x08).wrapping_add((old_len1 as usize) * 8);
    for i in 0..(c8_len as usize) {
        wr::<u64>(
            dst1.wrapping_add(i * 8),
            rd::<u64>(c8_ptr.wrapping_add(i * 8)),
        );
    }

    let b0_len = (vec_b0[2] as u32) as u64;
    let b0_ptr = vec_b0[1] as usize;
    let old_len2 = rd::<u32>(param_4 + 0x10) as u64;
    let new_len2 = old_len2.wrapping_add(b0_len);
    if new_len2 > (rd::<u32>(param_4 + 0x14) as u64) {
        exe::call_va!(0x1_411f_2380, (param_4, new_len2, 8u32));
    }
    wr::<u32>(param_4 + 0x10, new_len2 as u32);
    let dst2 = rd::<usize>(param_4 + 0x08).wrapping_add((old_len2 as usize) * 8);
    for i in 0..(b0_len as usize) {
        wr::<u64>(
            dst2.wrapping_add(i * 8),
            rd::<u64>(b0_ptr.wrapping_add(i * 8)),
        );
    }

    let final_data = rd::<usize>(param_4 + 0x08);
    let sum_18 = ((vec_c8[2] as u32) as u64).wrapping_add(r15_cnt);
    let val_18 = if sum_18 != u64::MAX {
        sum_18
    } else {
        rd::<u32>(param_4 + 0x10) as u64
    };
    wr::<u64>(param_4 + 0x18, val_18);
    wr::<usize>(param_4 + 0x20, final_data);
    wr::<u64>(param_4 + 0x28, (vec_b0[2] as u32) as u64);
    wr::<usize>(
        param_4 + 0x30,
        final_data.wrapping_add((val_18 as usize) * 8),
    );

    wr::<u64>(param_6, vec_88[0]);
    wr::<u64>(param_6 + 0x08, vec_88[1]);
    wr::<u32>(param_6 + 0x10, count as u32);
    wr::<u32>(param_6 + 0x14, count as u32);

    if dup_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (dup_ptr));
    }
}

/// `unknown_1420dae80`
/// Original: 0x1420dae80 (102 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420dae80(param_1: usize, param_2: usize) {
    let gen = rd::<u64>(param_2 + 0x5868);
    if gen != rd::<u64>(param_1 + 0x10) {
        let mut out1: u16 = 0;
        let p1 = exe::call_va!(
            0x1_420d_cac0,
            (
                (&mut out1 as *mut u16) as usize,
                param_2,
                rd::<u16>(param_1 + 0x08)
            ) -> usize
        );
        wr::<u16>(param_1 + 0x18, rd::<u16>(p1));

        let mut out2: u16 = 0;
        let p2 = exe::call_va!(
            0x1_420d_cac0,
            (
                (&mut out2 as *mut u16) as usize,
                param_2,
                rd::<u16>(param_1 + 0x0a)
            ) -> usize
        );
        wr::<u16>(param_1 + 0x1a, rd::<u16>(p2));
        wr::<u64>(param_1 + 0x10, gen);
    }
}

/// `unknown_1420dcc30`
/// Original: 0x1420dcc30 (975 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420dcc30(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u8,
) -> usize {
    if param_2 == 0 || param_3 == 0 {
        wr::<u16>(param_1, 0);
        return param_1;
    }

    let mut len: usize = 0;
    while rd::<u8>(param_3.wrapping_add(len)) != 0 {
        len += 1;
    }

    let lut = exe::ptr::<i8>(0x1_426d_01e0) as usize;
    let mut entry = exe::ptr::<u8>(0x1_4386_1ff0) as usize;
    let end_entry = exe::ptr::<u8>(0x1_4386_3250) as usize;
    while entry != end_entry {
        let s_ptr = rd::<usize>(entry + 0x08);
        let mut s_len: usize = 0;
        while rd::<u8>(s_ptr.wrapping_add(s_len)) != 0 {
            s_len += 1;
        }
        if len == s_len {
            let mut eq = true;
            for i in 0..len {
                let c1 = rd::<i8>(lut.wrapping_add(rd::<u8>(param_3.wrapping_add(i)) as usize));
                let c2 = rd::<i8>(lut.wrapping_add(rd::<u8>(s_ptr.wrapping_add(i)) as usize));
                if c1 != c2 {
                    eq = false;
                    break;
                }
            }
            if eq {
                wr::<u16>(param_1, rd::<u16>(entry));
                return param_1;
            }
        }
        entry = entry.wrapping_add(0x18);
    }

    let lock_base = param_2.wrapping_add(0x5a90);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_base + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_base));
        wr::<u32>(lock_base + 0x08, tid);
    }
    wr::<u32>(lock_base + 0x0c, rd::<u32>(lock_base + 0x0c).wrapping_add(1));

    let vec_base = param_2.wrapping_add(0x5870);
    let dyn_count = rd::<u32>(vec_base + 0x08) as usize;
    if dyn_count != 0 {
        let mut s_len: usize = 0;
        while rd::<u8>(param_3.wrapping_add(s_len)) != 0 {
            s_len += 1;
        }
        for idx in 0..dyn_count {
            let elem = rd::<usize>(vec_base).wrapping_add(idx * 0x38);
            let b3 = rd::<u8>(elem + 0x03) as u32;
            let cap_bits = if b3 < 0x80 {
                b3 * 8
            } else {
                (b3 << 8).wrapping_sub(0x7c00)
            };
            let cur_bits = (rd::<u32>(elem) & 0x00ff_ffff) << 3;
            let str_ptr = if cur_bits > cap_bits {
                rd::<usize>(elem + 0x08)
            } else {
                elem.wrapping_add(0x08)
            };
            let elem_len = rd::<u32>(elem + 0x04) as usize;
            if elem_len == s_len {
                let mut eq = true;
                for i in 0..s_len {
                    let c1 = rd::<i8>(lut.wrapping_add(rd::<u8>(str_ptr.wrapping_add(i)) as usize));
                    let c2 = rd::<i8>(lut.wrapping_add(rd::<u8>(param_3.wrapping_add(i)) as usize));
                    if c1 != c2 {
                        eq = false;
                        break;
                    }
                }
                if eq {
                    wr::<u16>(param_1, (idx as u16).wrapping_add(0xce) | 0x8000);
                    let new_cnt = rd::<u32>(lock_base + 0x0c).wrapping_sub(1);
                    wr::<u32>(lock_base + 0x0c, new_cnt);
                    if new_cnt == 0 {
                        let _ =
                            crate::simd_ops::atomic_xchg_u32((lock_base + 0x08) as u64, u32::MAX);
                        exe::call_iat!(0x1_4264_5598, (lock_base));
                    }
                    return param_1;
                }
            }
        }
    }

    if param_4 == 0 {
        wr::<u16>(param_1, 0);
        let new_cnt = rd::<u32>(lock_base + 0x0c).wrapping_sub(1);
        wr::<u32>(lock_base + 0x0c, new_cnt);
        if new_cnt == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((lock_base + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (lock_base));
        }
        return param_1;
    }

    let mut temp_entry = [0u64; 8];
    let tp = temp_entry.as_mut_ptr() as usize;
    wr::<u64>(tp, 0x0400_0004);
    wr::<u8>(tp + 0x08, 0);
    let mut s_len: usize = 0;
    while rd::<u8>(param_3.wrapping_add(s_len)) != 0 {
        s_len += 1;
    }
    let mut str_view: [u64; 2] = [param_3 as u64, s_len as u64];
    exe::call_va!(
        0x1_4240_9b50,
        (tp, str_view.as_mut_ptr() as usize, 0u32, s_len as u64)
    );

    let val_50 = rd::<u16>(param_4) as u64;
    let val_48 = param_5;
    let cur_len = rd::<u32>(vec_base + 0x08) as usize;
    let mut out_id: u16 = 0;
    if cur_len + 0xce <= 0x3fff {
        let cur_cap = rd::<u32>(vec_base + 0x0c) as usize;
        if cur_len * 0x38 == cur_cap * 0x38 {
            let min_cap = cur_len + 1;
            let grown = cur_cap + (cur_cap >> 1);
            let new_cap = if grown < min_cap { min_cap } else { grown };
            exe::call_va!(0x1_411f_6570, (vec_base, new_cap as u64));
        }
        let slot_idx = rd::<u32>(vec_base + 0x08) as usize;
        let rsi = rd::<usize>(vec_base).wrapping_add(slot_idx * 0x38);
        let hdr = (rd::<u32>(rsi) & 0xff00_0004) | 0x0400_0004;
        wr::<u32>(rsi, hdr);
        wr::<u32>(rsi + 0x04, 0);
        wr::<u8>(rsi + 0x08, 0);
        exe::call_va!(0x1_4240_aa00, (rsi, tp));
        wr::<u64>(rsi + 0x28, val_50);
        wr::<u8>(rsi + 0x30, val_48);
        wr::<u32>(vec_base + 0x08, (slot_idx as u32).wrapping_add(1));
        out_id = (cur_len as u16).wrapping_add(0xce);
    }
    wr::<u16>(param_1, out_id | 0x8000);
    exe::call_va!(0x1_4240_ab10, (tp));

    let new_cnt = rd::<u32>(lock_base + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_base + 0x0c, new_cnt);
    if new_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((lock_base + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_base));
    }
    param_1
}

/// `unknown_1420e9790`
/// Original: 0x1420e9790 (864 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420e9790(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> u8 {
    let mut param_1_slot: usize = param_1;
    let mut len: usize = 0;
    while rd::<u8>(param_1.wrapping_add(len)) != 0 {
        len += 1;
    }
    let mut span_path: [u64; 2] = [param_1 as u64, len as u64];
    let mut path_obj: [u64; 2] = [0; 2];
    exe::call_va!(
        0x1_401d_f850,
        (
            path_obj.as_mut_ptr() as usize,
            span_path.as_mut_ptr() as usize
        )
    );
    let exists = exe::call_va!(0x1_4249_1120, (path_obj.as_mut_ptr() as usize) -> u8);
    if exists == 0 {
        return 0;
    }

    let r14 = if param_3 != 0 {
        let vtbl = rd::<usize>(param_3);
        exe::call_va!(rd::<usize>(vtbl), (param_3, param_1) -> usize)
    } else {
        let mut zero_u32: u32 = 0;
        exe::call_va!(
            0x1_411f_7460,
            (
                0usize,
                0usize,
                (&mut param_1_slot as *mut usize) as usize,
                (&mut zero_u32 as *mut u32) as usize
            ) -> usize
        )
    };

    let mut fs: [usize; 2] = [exe::ptr::<u8>(0x1_4340_e9e0) as usize, r14];
    let mut count: i32 = 0;
    exe::call_va!(
        0x1_4249_5ac0,
        (
            fs.as_mut_ptr() as usize,
            (&mut count as *mut i32) as usize
        )
    );

    let rbx_need = (rd::<u32>(param_2 + 0x10) as u64).wrapping_add(count as i64 as u64);
    let cap1 = rd::<u32>(param_2 + 0x14) as u64;
    let rsi_vec = param_2.wrapping_add(0x18);
    if rbx_need > cap1 {
        let grown1 = (cap1 + (cap1 >> 1)).max(rbx_need);
        exe::call_va!(0x1_401f_22d0, (param_2 + 0x08, grown1));
        let cap2 = rd::<u32>(rsi_vec + 0x0c) as u64;
        if rbx_need > cap2 {
            let grown2 = (cap2 + (cap2 >> 1)).max(rbx_need);
            exe::call_va!(0x1_411f_7fc0, (rsi_vec, grown2));
        }
    }

    if count > 0 {
        for _ in 0..count {
            let mut str_c8 = [0u64; 18];
            let c8_ptr = str_c8.as_mut_ptr() as usize;
            wr::<u64>(c8_ptr, 0x1000_0010);
            wr::<u8>(c8_ptr + 0x08, 0);

            let mut wstr_buf = [0u64; 0x120 / 8];
            let wstr_ptr = wstr_buf.as_mut_ptr() as usize;
            let inline_wbuf = wstr_ptr.wrapping_add(0x20);
            wr::<usize>(wstr_ptr, inline_wbuf);
            wr::<u32>(wstr_ptr + 0x08, 0x80);
            wr::<u64>(wstr_ptr + 0x0c, 0x80);
            wr::<usize>(wstr_ptr + 0x18, inline_wbuf);
            wr::<u16>(inline_wbuf, 0);

            let vtbl = fs[0];
            exe::call_va!(
                rd::<usize>(vtbl + 0x20),
                (fs.as_mut_ptr() as usize, c8_ptr)
            );
            let vtbl2 = fs[0];
            exe::call_va!(
                rd::<usize>(vtbl2 + 0x18),
                (fs.as_mut_ptr() as usize, wstr_ptr)
            );
            exe::call_va!(0x1_411f_7ca0, (param_2 + 0x08, c8_ptr));

            let cur_len = rd::<u32>(rsi_vec + 0x08) as u64;
            let cur_cap = rd::<u32>(rsi_vec + 0x0c) as u64;
            if cur_len * 0x120 == cur_cap * 0x120 {
                let new_cap = (cur_cap + (cur_cap >> 1)).max(cur_len + 1);
                exe::call_va!(0x1_411f_7fc0, (rsi_vec, new_cap));
            }
            let slot_idx = rd::<u32>(rsi_vec + 0x08) as usize;
            let dst_wstr = rd::<usize>(rsi_vec).wrapping_add(slot_idx * 0x120);
            let dst_inline = dst_wstr.wrapping_add(0x20);
            wr::<usize>(dst_wstr, dst_inline);
            wr::<u32>(dst_wstr + 0x08, 0x80);
            wr::<u64>(dst_wstr + 0x0c, 0x80);
            wr::<usize>(dst_wstr + 0x18, dst_inline);
            wr::<u16>(dst_inline, 0);

            let w_len = rd::<u32>(wstr_ptr + 0x10);
            exe::call_va!(0x1_423e_f2b0, (dst_wstr, w_len));
            exe::call_iat!(
                0x1_4264_69d0,
                (
                    rd::<usize>(dst_wstr),
                    rd::<u32>(dst_wstr + 0x0c),
                    rd::<usize>(wstr_ptr)
                )
            );
            wr::<u32>(dst_wstr + 0x10, rd::<u32>(wstr_ptr + 0x10));
            wr::<u32>(rsi_vec + 0x08, rd::<u32>(rsi_vec + 0x08).wrapping_add(1));
            wr::<u8>(param_2, 1);

            let cur_wptr = rd::<usize>(wstr_ptr);
            if cur_wptr != 0 && cur_wptr != rd::<usize>(wstr_ptr + 0x18) {
                let r10 = cur_wptr.wrapping_sub(1) & !0x01ff_ffffusize;
                let same_teb = exe::teb() == rd::<usize>(r10 + 0x100);
                let page_idx = cur_wptr.wrapping_sub(r10) >> 16;
                let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx * 0x50);
                let rdx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
                if same_teb {
                    if rd::<u8>(rdx + 0x0e) == 0 {
                        wr::<u64>(cur_wptr, rd::<u64>(rdx + 0x20));
                        wr::<usize>(rdx + 0x20, cur_wptr);
                        let new_cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
                        wr::<u32>(rdx + 0x18, new_cnt);
                        if new_cnt == 0 {
                            exe::call_va!(0x1_4253_71a0, (rdx));
                        }
                    } else {
                        exe::call_va!(0x1_4253_3200, (r10, rdx, 1u8, cur_wptr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10, rdx, 0u8, cur_wptr));
                }
            }

            let reset_ptr = rd::<usize>(wstr_ptr + 0x18);
            wr::<usize>(wstr_ptr, reset_ptr);
            wr::<u32>(wstr_ptr + 0x0c, rd::<u32>(wstr_ptr + 0x08));
            if reset_ptr != 0 {
                wr::<u16>(reset_ptr, 0);
            }
            wr::<u32>(wstr_ptr + 0x10, 0);
            exe::call_va!(0x1_4240_ab10, (c8_ptr));
        }
    }

    if param_3 != 0 {
        let vtbl = rd::<usize>(param_3);
        exe::call_va!(rd::<usize>(vtbl + 0x08), (param_3, r14));
    } else if r14 != 0 {
        let vtbl = rd::<usize>(r14);
        exe::call_va!(rd::<usize>(vtbl), (r14, 0u32));
        exe::call_va!(0x1_4244_2de0, (r14));
    }

    u8::from(count > 0)
}

/// `unknown_142211f20`
/// Original: 0x142211f20 (8 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142211f20(param_1: usize, param_2: usize) -> u8 {
    let b = rd::<i32>(param_2);
    let a = rd::<i32>(param_1);
    u8::from(a < b)
}

/// `unknown_142212fa0`
/// Original: 0x142212fa0 (453 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142212fa0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut xmm4 = rd::<f64>(param_3);
    let xmm6 = rd::<f64>(param_1 + 0x28);

    loop {
        let x0 = rd::<f64>(param_3);
        let x1 = rd::<f64>(param_3 + 0x08);
        if !(x0 >= xmm6) {
            break;
        }
        if !(x1 >= rd::<f64>(param_1 + 0x30)) {
            break;
        }
        if !(rd::<f64>(param_1 + 0x48) >= x0) {
            break;
        }
        if !(rd::<f64>(param_1 + 0x50) >= x1) {
            break;
        }
        xmm4 += rd::<f64>(param_3 + 0x10);
        wr::<f64>(param_3, xmm4);
        let new_y = rd::<f64>(param_3 + 0x18) + rd::<f64>(param_3 + 0x08);
        wr::<f64>(param_3 + 0x08, new_y);
        wr::<u32>(param_3 + 0x30, rd::<u32>(param_3 + 0x30).wrapping_add(1));
    }

    let dx = rd::<f64>(param_3 + 0x10);
    let y = rd::<f64>(param_3 + 0x08);
    wr::<f64>(param_3, xmm4 - dx);
    let dy_orig = rd::<f64>(param_3 + 0x18);
    let sign_mask = rd::<u64>(exe::ptr::<u64>(0x1_4342_8740) as usize);
    wr::<f64>(param_3 + 0x08, y - dy_orig);
    wr::<u32>(param_3 + 0x30, rd::<u32>(param_3 + 0x30).wrapping_sub(1));
    let dy2 = rd::<u64>(param_3 + 0x18);
    wr::<u64>(param_3 + 0x10, dx.to_bits() ^ sign_mask);
    wr::<u64>(param_3 + 0x18, dy2 ^ sign_mask);

    let mut xmm2 = rd::<f64>(param_3);
    let mut xmm1 = rd::<f64>(param_3 + 0x08);
    let xmm5 = rd::<f64>(param_3 + 0x10);
    let xmm4_step = rd::<f64>(param_3 + 0x18);
    let xmm3 = rd::<f64>(param_1 + 0x68);
    let mut eax: u32 = 0;

    loop {
        if xmm2 >= xmm3
            && xmm1 >= rd::<f64>(param_1 + 0x70)
            && rd::<f64>(param_1 + 0x88) >= xmm2
            && rd::<f64>(param_1 + 0x90) >= xmm1
        {
            while xmm2 >= xmm3
                && xmm1 >= rd::<f64>(param_1 + 0x70)
                && rd::<f64>(param_1 + 0x88) >= xmm2
                && rd::<f64>(param_1 + 0x90) >= xmm1
            {
                xmm2 += xmm5;
                xmm1 += xmm4_step;
                eax = eax.wrapping_add(1);
            }
            break;
        }
        xmm2 += xmm5;
        eax = eax.wrapping_add(1);
        xmm1 += xmm4_step;
        if !(xmm2 >= xmm6
            && xmm1 >= rd::<f64>(param_1 + 0x30)
            && rd::<f64>(param_1 + 0x48) >= xmm2
            && rd::<f64>(param_1 + 0x50) >= xmm1)
        {
            eax = 0;
            break;
        }
    }

    wr::<u32>(param_3 + 0x24, eax);
    wr::<i32>(param_3 + 0x30, rd::<i32>(param_3 + 0x30).wrapping_neg());
    let half = rd::<i32>(rd::<usize>(param_1) + 0x08) / 2;
    wr::<i32>(param_3 + 0x20, rd::<i32>(param_3 + 0x20).wrapping_add(half));

    let data = rd::<[u64; 8]>(param_3);
    wr::<[u64; 8]>(param_2, data);
    param_2
}

/// `unknown_1422132f0`
/// Original: 0x1422132f0 (79 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1422132f0(param_1: usize, param_2: usize) -> u8 {
    if rd::<u32>(param_1 + 0x24) == 0 {
        return 0;
    }
    if rd::<u32>(param_2 + 0x24) == 0 {
        return 1;
    }
    if rd::<u32>(param_1 + 0x20) == rd::<u32>(param_2 + 0x20) {
        return u8::from(rd::<i32>(param_1 + 0x34) < rd::<i32>(param_2 + 0x34));
    }
    let mut err_buf = [0u64; 4];
    let s_msg = exe::ptr::<u8>(0x1_433f_0470) as usize;
    let err_ptr = exe::call_va!(0x1_412e_8690, (err_buf.as_mut_ptr() as usize, s_msg) -> usize);
    exe::call_va!(0x1_412e_88c0, (err_ptr));
    0
}

/// `unknown_142213ae0`
/// Original: 0x142213ae0 (862 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142213ae0(param_1: usize, param_2: usize) {
    let mut vec_30: [u64; 2] = [0; 2];
    let ebp = rd::<u32>(param_2 + 0x08);
    let r13_bytes = (ebp as usize) << 6;
    let mut rbx = rd::<usize>(param_2);
    let rsi_end = rbx.wrapping_add(r13_bytes);
    let mut r15: usize = 0;

    if ebp != 0 {
        r15 = exe::call_va!(0x1_4023_b7a0, (param_1, ebp as u64) -> usize);
        wr::<usize>(vec_30.as_mut_ptr() as usize, r15);
        wr::<u32>((vec_30.as_mut_ptr() as usize) + 0x0c, ebp);
    }

    let mut dst_copy = r15;
    while rbx != rsi_end {
        wr::<[u64; 8]>(dst_copy, rd::<[u64; 8]>(rbx));
        dst_copy = dst_copy.wrapping_add(0x40);
        rbx = rbx.wrapping_add(0x40);
    }
    wr::<u32>((vec_30.as_mut_ptr() as usize) + 0x08, ebp);

    let mut vec_20: [u64; 2] = [0; 2];
    let mut r12: usize = 0;
    let mut rsi_cap: u32 = 0;
    let mut r10d: i32 = 0;
    let r13_items = ebp as u64;
    let stride0 = rd::<i32>(rd::<usize>(param_1) + 0x30) as i64 as u64;

    if r13_items / stride0 != 0 {
        let mut r9d: u32 = 0;
        let mut edi_len: u32 = 0;
        let mut r11d: u32 = 0;
        loop {
            if (r9d as u64) == (rsi_cap as u64) {
                let cur_len = edi_len as u64;
                let min_cap = cur_len + 1;
                let cur_cap = rsi_cap as u64;
                let grown = cur_cap + (cur_cap >> 1);
                let new_cap = if grown < min_cap { min_cap } else { grown };
                let new_buf = exe::call_va!(
                    0x1_401d_ecf0,
                    (r12.wrapping_add((cur_cap as usize) * 4), new_cap) -> usize
                );
                let _ = exe::call_va!(0x1_4183_acc8, (new_buf, r12, cur_len * 4) -> usize);
                exe::call_va!(0x1_401d_e830, (vec_20.as_mut_ptr() as usize));
                r12 = new_buf;
                wr::<usize>(vec_20.as_mut_ptr() as usize, r12);
                rsi_cap = new_cap as u32;
                wr::<u32>((vec_20.as_mut_ptr() as usize) + 0x0c, rsi_cap);
                r11d = edi_len;
            }
            wr::<u32>(r12.wrapping_add((edi_len as usize) * 4), 0);
            r9d = r11d.wrapping_add(1);
            wr::<u32>((vec_20.as_mut_ptr() as usize) + 0x08, r9d);
            r10d = r10d.wrapping_add(1);
            edi_len = r9d;
            r11d = r9d;
            let cur_stride = rd::<i32>(rd::<usize>(param_1) + 0x30) as i64 as u64;
            if (r10d as i64 as u64) >= r13_items / cur_stride {
                break;
            }
        }
    }

    let mut r8_obj = rd::<usize>(param_1);
    if r13_items / (rd::<i32>(r8_obj + 0x30) as i64 as u64) != 0 {
        let mut r9d_grp: i32 = 0;
        let mut r10_grp: usize = 0;
        loop {
            let mut ecx_in: i32 = 0;
            if rd::<i32>(r8_obj + 0x30) > 0 {
                loop {
                    let item_idx = r9d_grp
                        .wrapping_mul(rd::<i32>(r8_obj + 0x30))
                        .wrapping_add(ecx_in) as i64 as usize;
                    let val = rd::<i32>(r15.wrapping_add(item_idx * 0x40 + 0x24));
                    let cur_max = rd::<i32>(r12.wrapping_add(r10_grp * 4));
                    if cur_max < val {
                        wr::<i32>(r12.wrapping_add(r10_grp * 4), val);
                    }
                    ecx_in += 1;
                    r8_obj = rd::<usize>(param_1);
                    if ecx_in >= rd::<i32>(r8_obj + 0x30) {
                        break;
                    }
                }
            }
            r9d_grp += 1;
            r10_grp = r9d_grp as isize as usize;
            let cur_stride = rd::<i32>(r8_obj + 0x30) as i64 as u64;
            if (r10_grp as u64) >= r13_items / cur_stride {
                break;
            }
        }
    }

    wr::<u32>(param_2 + 0x08, 0);
    if r13_bytes >= 0x40 {
        loop {
            let mut max_val: i32 = 0;
            let mut best_grp: i32 = 0;
            let num_grps = r13_items / (rd::<i32>(rd::<usize>(param_1) + 0x30) as i64 as u64);
            if num_grps != 0 {
                let mut g: i32 = 0;
                while (g as i64 as u64) < num_grps {
                    let v = rd::<i32>(r12.wrapping_add((g as isize as usize) * 4));
                    if v > max_val {
                        max_val = v;
                        best_grp = g;
                    }
                    g += 1;
                }
            }
            wr::<u32>(
                r12.wrapping_add((best_grp as isize as usize) * 4),
                0xffff_ffff,
            );
            let mut j: i32 = 0;
            let mut obj = rd::<usize>(param_1);
            if rd::<i32>(obj + 0x30) > 0 {
                loop {
                    let src_idx = best_grp
                        .wrapping_mul(rd::<i32>(obj + 0x30))
                        .wrapping_add(j) as i64 as usize;
                    let cur_len = rd::<u32>(param_2 + 0x08) as u64;
                    let cur_cap = rd::<u32>(param_2 + 0x0c) as u64;
                    if (cur_len << 6) == (cur_cap << 6) {
                        exe::call_va!(0x1_411a_2010, (param_2, cur_len + 1));
                    }
                    let dst_idx = rd::<u32>(param_2 + 0x08) as usize;
                    let dst_ptr = rd::<usize>(param_2).wrapping_add(dst_idx * 0x40);
                    let src_ptr = r15.wrapping_add(src_idx * 0x40);
                    wr::<[u64; 8]>(dst_ptr, rd::<[u64; 8]>(src_ptr));
                    wr::<u32>(param_2 + 0x08, (dst_idx as u32).wrapping_add(1));
                    j += 1;
                    obj = rd::<usize>(param_1);
                    if j >= rd::<i32>(obj + 0x30) {
                        break;
                    }
                }
            }
            if (rd::<u32>(param_2 + 0x08) as u64) == r13_items {
                break;
            }
        }
    }

    exe::call_va!(0x1_401d_e830, (vec_20.as_mut_ptr() as usize));
    exe::call_va!(0x1_401d_e830, (vec_30.as_mut_ptr() as usize));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1420bb3a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut JOB_BUF: [[u64; 16]; 8] = [[0; 16]; 8];
    static mut JOB_IDX: usize = 0;
    static mut QUEUED: [u64; 8] = [0; 8];
    static mut QUEUED_CNT: usize = 0;
    static mut WAIT_CNT: u32 = 0;
    static mut RESIZE_CNT: u32 = 0;
    static PAGE1: [u64; 2] = [0, 2];
    static PAGE2: [u64; 2] = [0, 2];
    static mut VEC_DATA: [u64; 6] = [0; 6];

    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_jh: usize) {
        let i = JOB_IDX;
        JOB_IDX = (i + 1) & 7;
        JOB_BUF[i] = [0; 16];
        wr::<u8>((core::ptr::addr_of_mut!(JOB_BUF[i]) as usize) + 0x04, 1);
        let p = core::ptr::addr_of_mut!(JOB_BUF[i]) as usize;
        wr::<usize>(out_jh, p);
    }
    unsafe extern "win64" fn mock_6460(_sched: usize, job: usize) {
        let i = QUEUED_CNT & 7;
        QUEUED[i] = rd::<u64>(job + 0x18)
            ^ rd::<u64>(job + 0x20)
            ^ rd::<u64>(job + 0x28)
            ^ rd::<u64>(job + 0x30)
            ^ rd::<u64>(job + 0x38)
            ^ rd::<u64>(job + 0x40)
            ^ rd::<u64>(job + 0x48)
            ^ rd::<u64>(job + 0x50)
            ^ rd::<u64>(job + 0x58);
        QUEUED_CNT += 1;
    }
    unsafe extern "win64" fn mock_5680(_g2f8: usize, _jh: usize) {
        WAIT_CNT += 1;
    }
    unsafe extern "win64" fn mock_f9a0(ptr: usize, count: u32, _cmp: usize) {
        let n = count as usize;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = rd::<u64>(ptr + i * 8);
                let b = rd::<u64>(ptr + j * 8);
                if a > b {
                    wr::<u64>(ptr + i * 8, b);
                    wr::<u64>(ptr + j * 8, a);
                }
            }
        }
    }
    unsafe extern "win64" fn mock_2570(vec_ptr: usize, new_cap: u64) {
        RESIZE_CNT += 1;
        wr::<u32>(vec_ptr + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_rel_sem(_h: u64, _c: u32, _prev: usize) -> i32 {
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420a_f9a0, mock_f9a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411f_2570, mock_2570 as *const ())?;

    let g_2f8_slot = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<usize>(g_2f8_slot);
    // The IAT lives in read-only `.rdata` once an earlier `IatMock` has restored
    // its registered protection, so the ReleaseSemaphore slot must be redirected
    // through `IatMock` (VirtualProtect round-trip), not a raw write.
    let _iat_sem = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;

    let run = |f: F| -> ([u64; 6], [u64; 8], usize, u32, u32, u32, u8) {
        JOB_BUF = [[0; 16]; 8];
        JOB_IDX = 0;
        QUEUED = [0; 8];
        QUEUED_CNT = 0;
        WAIT_CNT = 0;
        RESIZE_CNT = 0;
        VEC_DATA = [
            (2u64 << 48) | 10,
            (1u64 << 48) | 20,
            (2u64 << 48) | 10, // duplicate
            (1u64 << 48) | 30,
            0,
            0,
        ];

        let mut sched_inner = vec![0u8; 0x1020];
        let sip = sched_inner.as_mut_ptr() as usize;
        wr::<u64>(sip + 0x1008, 0x9999);
        let mut worker_pool = [0u8; 0x100];
        let mut sched = [0u8; 0x60];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sip);
        wr::<usize>(sp + 0x48, worker_pool.as_mut_ptr() as usize);
        let mut g_obj = [0u8; 0x20];
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sp);
        wr::<usize>(g_2f8_slot, g_obj.as_mut_ptr() as usize);

        let mut p2 = [0u8; 0x30];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x08, PAGE1.as_ptr() as usize);
        wr::<u64>(p2p + 0x18, 2);
        wr::<usize>(p2p + 0x20, PAGE2.as_ptr() as usize);

        let mut p7: [u64; 3] = [
            0xaaaa,
            core::ptr::addr_of_mut!(VEC_DATA) as usize as u64,
            2u64 | (3u64 << 32), // len=2, cap=3 (< new_total=4)
        ];

        let bitset: [u64; 2] = [0b0110, 0]; // bits 1 and 2 set
        let p4: [usize; 1] = [bitset.as_ptr() as usize];
        let p5: [u64; 2] = [0x5555_0000, 7];
        let mut out_jh: [u64; 2] = [0; 2];

        let ret = f(
            out_jh.as_mut_ptr() as usize,
            p2p,
            0x3333,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            0x6666,
            p7.as_mut_ptr() as usize,
        );
        assert_eq!(ret, out_jh.as_mut_ptr() as usize);
        let out_jc = out_jh[0] as usize;
        (
            VEC_DATA,
            QUEUED,
            QUEUED_CNT,
            WAIT_CNT,
            RESIZE_CNT,
            rd::<u32>(out_jc),
            rd::<u8>((out_jh.as_ptr() as usize) + 0x0a),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2f8_slot, saved_2f8);
    drop(_iat_sem);

    crate::diff_eq!(o, n, "1420bb3a0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420bbd40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: [u64; 8] = [0; 8];
    static mut CALL_CNT: usize = 0;

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x7770 && n == 4 && want == 2 {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_cb(src: u64, dst: u64, idx: u32) {
        let i = CALL_CNT & 7;
        CALL_LOG[i] = src ^ dst.wrapping_shl(8) ^ (idx as u64);
        CALL_CNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;

    let run = |f: F| -> ([u64; 8], usize) {
        CALL_LOG = [0; 8];
        CALL_CNT = 0;

        // Page 1 (u32 keys): 2 elements
        let mut page1 = [0u8; 0x20];
        let p1p = page1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x08, 2);
        wr::<u32>(p1p + 0x10, 15);
        wr::<u32>(p1p + 0x14, 35);

        // Page 2 (u16 comp_ids): 2 elements
        let mut page2 = [0u8; 0x20];
        let p2p = page2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x08, 2);
        wr::<u16>(p2p + 0x10, 1);
        wr::<u16>(p2p + 0x12, 2);

        // Page 3 (u64 src ptrs): 2 elements (first 0 -> skipped, second non-zero -> called)
        let mut page3 = [0u8; 0x28];
        let p3p = page3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x08, 2);
        wr::<u64>(p3p + 0x10, 0);
        wr::<u64>(p3p + 0x18, 0x1234_0000);

        let mut adds = [0u8; 0x48];
        let ap = adds.as_mut_ptr() as usize;
        wr::<usize>(ap + 0x08, p1p);
        wr::<u64>(ap + 0x18, 2);
        wr::<usize>(ap + 0x20, p2p);
        wr::<usize>(ap + 0x38, p3p);

        let off_tbl: [u32; 2] = [0x10, 0x40];
        let mut arch_arr = [0u8; 0x60];
        let aap = arch_arr.as_mut_ptr() as usize;
        wr::<u64>(aap + 0x30 + 0x08, 0x5000);
        wr::<usize>(aap + 0x30 + 0x18, off_tbl.as_ptr() as usize);

        let keys_sorted: [u32; 3] = [10, 20, 30];
        let u4_map: [u32; 3] = [0xffff_ffff, 0, 1];
        let ent_locs: [u64; 2] = [0xffff_0000_0000, 1u64 << 32]; // u4=1 -> arch=1 at +4
        let idx_map: [u32; 2] = [7, 9];

        let mut world_50 = vec![0u8; 0x18100];
        let wp = world_50.as_mut_ptr() as usize;
        wr::<u64>(wp + 0x20 + 0x18058, 0x7770);
        wr::<u32>(wp + 0x20 + 0x1806c, 4);

        let mut comp_tbl = [0u8; 0x120];
        let ctp = comp_tbl.as_mut_ptr() as usize;
        wr::<usize>(ctp + 2 * 0x50 + 0x30, mock_cb as *const () as usize);
        let p58_box: [usize; 1] = [ctp];

        let mut ctx: [u64; 13] = [0; 13];
        ctx[1] = aap as u64;
        ctx[2] = 3;
        ctx[3] = keys_sorted.as_ptr() as usize as u64;
        ctx[5] = ent_locs.as_ptr() as usize as u64;
        ctx[7] = u4_map.as_ptr() as usize as u64;
        ctx[9] = idx_map.as_ptr() as usize as u64;
        ctx[10] = wp as u64;
        ctx[11] = p58_box.as_ptr() as usize as u64;
        ctx[12] = ap as u64;

        let mut jh: [u64; 2] = [0; 2];
        f(ctx.as_mut_ptr() as usize, jh.as_mut_ptr() as usize);
        (CALL_LOG, CALL_CNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420bbd40 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420bc770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        u64,
        usize,
        u64,
        usize,
        usize,
        u64,
        usize,
        usize,
        u64,
    ) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut JC_MEM: [u64; 8] = [0; 8];
    static mut DST_U16: [u16; 8] = [0; 8];
    static mut CB_SUM: u64 = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_jh: usize) {
        JC_MEM = [0; 8];
        wr::<u8>((core::ptr::addr_of_mut!(JC_MEM) as usize) + 0x04, 1);
        wr::<usize>(out_jh, core::ptr::addr_of_mut!(JC_MEM) as usize);
    }
    unsafe extern "win64" fn mock_dea0(
        out_20: usize,
        arena: usize,
        _p11: u64,
        _r9: u64,
        _a5: u64,
        p7_copy: usize,
    ) {
        let p7_v0 = rd::<u64>(p7_copy);
        wr::<usize>(out_20, arena);
        wr::<u64>(out_20 + 0x08, p7_v0);
        wr::<u32>(out_20 + 0x10, 3);
        wr::<u32>(out_20 + 0x14, 4);
    }
    unsafe extern "win64" fn mock_11e0(_arena: usize, _bytes: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(DST_U16) as usize
    }
    unsafe extern "win64" fn mock_1520(_p0: u64, _len: u32, key: u64) -> i32 {
        if key == 0x2222 {
            1
        } else {
            -1
        }
    }
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn mock_be3c0(
        out_ptr: usize,
        arena: usize,
        _a3: usize,
        _a4: usize,
        _a5: usize,
        _a6: usize,
        _a7: usize,
        _a8: usize,
        _a9: usize,
        a10_p9: u64,
        _a11: usize,
        a12_p11: u64,
        _a13: usize,
        _a14: usize,
        a15_p14: u64,
        _a16: usize,
        a17_m78: usize,
        a18_in: u64,
    ) -> usize {
        let (in_rsp, in_limit) = crate::emu::take_incoming_rsp();
        let (m78_ptr, p17_val) = if in_rsp != 0 && in_rsp.wrapping_add(0x90) <= in_limit {
            (
                rd::<usize>((in_rsp + 0x80) as usize),
                rd::<u64>((in_rsp + 0x88) as usize),
            )
        } else {
            (a17_m78, a18_in)
        };
        wr::<u64>(m78_ptr, arena as u64);
        wr::<u64>(m78_ptr + 0x08, 0x4000_0000);
        wr::<u64>(m78_ptr + 0x10, 2u64 | (4u64 << 32));
        for i in 0..15usize {
            wr::<u64>(out_ptr + i * 8, 0x100 + i as u64);
        }
        wr::<u64>(out_ptr + 4 * 8, 0x8880); // local_f8
        wr::<u32>(out_ptr + 5 * 8, 3); // local_f0
        wr::<u64>(out_ptr + 14 * 8, a10_p9 ^ a12_p11 ^ a15_p14 ^ p17_val);
        out_ptr
    }
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn mock_bcd80(
        out_ptr: usize,
        arena: usize,
        _a3: usize,
        _a4: usize,
        _a5: u64,
        _a6: usize,
        _a7: usize,
        _a8: u64,
        _a9: usize,
        _a10: usize,
        a11_p15: usize,
    ) -> usize {
        let p15_v0 = rd::<u64>(a11_p15);
        wr::<u64>(out_ptr, arena as u64);
        wr::<u64>(out_ptr + 0x08, p15_v0);
        wr::<u64>(out_ptr + 0x10, 2);
        out_ptr
    }
    unsafe extern "win64" fn mock_9ee0(ptr: u64, _len: u32, _want: u16) -> i32 {
        if ptr == 0x8880 {
            -1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_cb30(w1: u64, arg2: u64, arg3: u32) {
        CB_SUM = CB_SUM
            .wrapping_add(w1)
            .wrapping_add(arg2)
            .wrapping_add(arg3 as u64);
    }
    unsafe extern "win64" fn mock_cb40(w1: u64, arg2: u64, arg3: u32) {
        CB_SUM = CB_SUM
            .wrapping_add(w1 * 3)
            .wrapping_add(arg2)
            .wrapping_add(arg3 as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420b_dea0, mock_dea0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_1520, mock_1520 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420b_e3c0, mock_be3c0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_420b_cd80, mock_bcd80 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;

    let g_2f8_slot = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<usize>(g_2f8_slot);

    let run = |f: F| -> ([u64; 17], [u16; 8], u64) {
        DST_U16 = [0; 8];
        CB_SUM = 0;

        let mut worker_pool = [0u8; 0x100];
        let mut sched = [0u8; 0x60];
        wr::<usize>(
            sched.as_mut_ptr() as usize + 0x48,
            worker_pool.as_mut_ptr() as usize,
        );
        let mut g_obj = [0u8; 0x20];
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sched.as_mut_ptr() as usize);
        wr::<usize>(g_2f8_slot, g_obj.as_mut_ptr() as usize);

        // 3 items in out_20: item 0 (ent=1, comp=1, w1=0x111), item 1 (ent=1, comp=1, w1=0x222 -> coalesced!), item 2 (w1=0 -> skipped)
        let items: [u64; 6] = [
            (1u64 << 32) | 1,
            0x111,
            (1u64 << 32) | 1,
            0x222,
            0,
            0,
        ];
        let idx_arr: [u32; 2] = [10, 20];

        let mut comp_tbl = [0u8; 0xa0];
        let ctp = comp_tbl.as_mut_ptr() as usize;
        wr::<usize>(ctp + 0x50 + 0x30, mock_cb30 as *const () as usize);
        wr::<usize>(ctp + 0x50 + 0x40, mock_cb40 as *const () as usize);
        let p13_inner: [usize; 1] = [ctp];
        let mut p13 = [0u64; 10];
        p13[0] = p13_inner.as_ptr() as usize as u64;

        let off_tbl: [u32; 1] = [0x80];
        let mut arch_arr = [0u8; 0x60];
        let aap = arch_arr.as_mut_ptr() as usize;
        wr::<u64>(aap + 0x30 + 0x08, 0x6000);
        wr::<usize>(aap + 0x30 + 0x18, off_tbl.as_ptr() as usize);

        let mut p12 = vec![0u8; 0x18100];
        let p12p = p12.as_mut_ptr() as usize;
        wr::<u64>(p12p + 0x20 + 0x18058, 0x9990);
        wr::<u32>(p12p + 0x20 + 0x1806c, 2);

        let p6_keys: [u64; 2] = [0x1111, 0x2222];
        let p6: [u64; 2] = [2, p6_keys.as_ptr() as usize as u64];
        let p7: [u64; 2] = [items.as_ptr() as usize as u64, 3];
        let p15: [u64; 2] = [idx_arr.as_ptr() as usize as u64, aap as u64];
        let p16_vals: [u16; 2] = [0, 1];
        let mut p16 = [0u8; 0x18];
        wr::<u64>(p16.as_mut_ptr() as usize, 0xaaaa);
        wr::<u32>(p16.as_mut_ptr() as usize + 0x08, 2);
        wr::<usize>(
            p16.as_mut_ptr() as usize + 0x10,
            p16_vals.as_ptr() as usize,
        );

        let dummy_span: [u64; 2] = [1, 2];
        let mut out = [0u64; 17];
        let p17_val: u64 = 0x7777_8888;
        let mut in_buf = [0u64; 18];
        in_buf[16] = p17_val;
        let ib = in_buf.as_ptr() as u64;
        crate::emu::set_incoming_rsp(ib, ib + 0x90);
        let ret = f(
            out.as_mut_ptr() as usize,
            0x5555,
            dummy_span.as_ptr() as usize,
            dummy_span.as_ptr() as usize,
            dummy_span.as_ptr() as usize,
            p6.as_ptr() as usize,
            p7.as_ptr() as usize,
            dummy_span.as_ptr() as usize,
            0x9001,
            dummy_span.as_ptr() as usize,
            0xb002,
            p12p,
            p13.as_ptr() as usize,
            0xe003,
            p15.as_ptr() as usize,
            p16.as_ptr() as usize,
            p17_val,
        );
        crate::emu::set_incoming_rsp(0, 0);
        assert_eq!(ret, out.as_mut_ptr() as usize);
        out[12] = 0; // clear job_handle[0] pointer
        (out, DST_U16, CB_SUM)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2f8_slot, saved_2f8);

    crate::diff_eq!(o, n, "1420bc770 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420c2c10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF: [u32; 8] = [0; 8];
    static mut FAIL_FIRST: bool = false;

    unsafe extern "win64" fn mock_11e0(_arena: usize, _bytes: u64, _align: u32) -> usize {
        if FAIL_FIRST {
            FAIL_FIRST = false;
            0
        } else {
            core::ptr::addr_of_mut!(BUF) as usize
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, count: u64, fail_first: bool| -> ([u64; 3], [u32; 8], [u32; 2]) {
        BUF = [0xdead_beef; 8];
        FAIL_FIRST = fail_first;

        let entries: [u64; 3] = [
            0xffff_0000_0001,
            (1u64 << 32) | 2,
            (1u64 << 32) | 3,
        ];
        let p3: [u64; 2] = [count, entries.as_ptr() as usize as u64];
        let mut arch_arr = [0u8; 0x60];
        let aap = arch_arr.as_mut_ptr() as usize;
        wr::<u32>(aap + 0x28, 10);
        wr::<u32>(aap + 0x30 + 0x28, 20);
        let p4: [usize; 2] = [0, aap];

        let mut out: [u64; 3] = [0; 3];
        let ret = f(
            out.as_mut_ptr() as usize,
            0x1234,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
        );
        assert_eq!(ret, out.as_mut_ptr() as usize);
        out[1] = 0; // clear pointer
        (
            out,
            BUF,
            [rd::<u32>(aap + 0x28), rd::<u32>(aap + 0x58)],
        )
    };

    for (cnt, ff) in [(0u64, false), (3, false), (3, true)] {
        let o = run(f_o, cnt, ff);
        let n = run(f_n, cnt, ff);
        crate::diff_eq!(o, n, "1420c2c10 cnt={} ff={}", cnt, ff);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420c43d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARR_80: [u64; 8] = [0; 8];
    static mut DUP_BUF: [u64; 8] = [0; 8];
    static mut OUT_BUF: [u64; 16] = [0; 16];
    static mut JOB_MEM: [u64; 16] = [0; 16];
    static C8_DATA: [u64; 2] = [0xaaaa_0001, 0xaaaa_0002];
    static B0_DATA: [u64; 1] = [0xbbbb_0001];
    static mut FREE_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_11e0(_arena: usize, _bytes: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ARR_80) as usize
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_jh: usize) {
        JOB_MEM = [0; 16];
        wr::<usize>(out_jh, core::ptr::addr_of_mut!(JOB_MEM) as usize);
    }
    unsafe extern "win64" fn mock_6460(_sched: usize, job: usize) {
        let vec_c8 = rd::<usize>(job + 0x38);
        let vec_b0 = rd::<usize>(job + 0x40);
        wr::<usize>(vec_c8 + 0x08, C8_DATA.as_ptr() as usize);
        wr::<u32>(vec_c8 + 0x10, 2);
        wr::<usize>(vec_b0 + 0x08, B0_DATA.as_ptr() as usize);
        wr::<u32>(vec_b0 + 0x10, 1);
    }
    unsafe extern "win64" fn mock_e550(ptr: usize, count: u32) {
        let n = count as usize;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = rd::<u64>(ptr + i * 8);
                let b = rd::<u64>(ptr + j * 8);
                if a > b {
                    wr::<u64>(ptr + i * 8, b);
                    wr::<u64>(ptr + j * 8, a);
                }
            }
        }
    }
    unsafe extern "win64" fn mock_c0a0(_bytes: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(DUP_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_CALLS += 1;
    }
    unsafe extern "win64" fn mock_5680(_g2f8: usize, _jh: usize) {}
    unsafe extern "win64" fn mock_2320(p4: usize, cnt: u64) {
        wr::<usize>(p4 + 0x08, core::ptr::addr_of_mut!(OUT_BUF) as usize);
        wr::<u32>(p4 + 0x10, cnt as u32);
        wr::<u32>(p4 + 0x14, 2);
    }
    unsafe extern "win64" fn mock_2380(p4: usize, new_cap: u64, _elem_sz: u32) {
        wr::<u32>(p4 + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_rel_sem(_h: u64, _c: u32, _prev: usize) -> i32 {
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40fa_e550, mock_e550 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_411f_2320, mock_2320 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_411f_2380, mock_2380 as *const ())?;

    let g_2f8_slot = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<usize>(g_2f8_slot);
    // Redirect the ReleaseSemaphore IAT slot through `IatMock` (the slot's page
    // is read-only `.rdata` after earlier tests restore its protection).
    let _iat_sem = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;

    let run = |f: F| -> ([u64; 7], [u64; 3], [u64; 8], [u64; 8], u32) {
        ARR_80 = [0; 8];
        DUP_BUF = [0; 8];
        OUT_BUF = [0; 16];
        FREE_CALLS = 0;

        let mut sched_inner = vec![0u8; 0x1020];
        let mut worker_pool = [0u8; 0x100];
        let mut sched = [0u8; 0x60];
        wr::<usize>(sched.as_mut_ptr() as usize, sched_inner.as_mut_ptr() as usize);
        wr::<usize>(
            sched.as_mut_ptr() as usize + 0x48,
            worker_pool.as_mut_ptr() as usize,
        );
        let mut g_obj = [0u8; 0x20];
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sched.as_mut_ptr() as usize);
        wr::<usize>(g_2f8_slot, g_obj.as_mut_ptr() as usize);

        let mut page_50 = [0u8; 0x20];
        let p50 = page_50.as_mut_ptr() as usize;
        wr::<u64>(p50 + 0x08, 3);
        wr::<u32>(p50 + 0x10, 10);
        wr::<u32>(p50 + 0x14, 20);
        wr::<u32>(p50 + 0x18, 10); // duplicate with index 0

        let mut page_68 = [0u8; 0x20];
        let p68 = page_68.as_mut_ptr() as usize;
        wr::<u64>(p68 + 0x08, 3);
        wr::<u16>(p68 + 0x10, 5);
        wr::<u16>(p68 + 0x12, 6);
        wr::<u16>(p68 + 0x14, 5); // duplicate (10, 5)

        let mut p1 = [0u8; 0x70];
        wr::<usize>(p1.as_mut_ptr() as usize + 0x50, p50);
        wr::<u64>(p1.as_mut_ptr() as usize + 0x60, 3);
        wr::<usize>(p1.as_mut_ptr() as usize + 0x68, p68);

        let p2: [u64; 2] = [0x1111, 0x2222];
        let mut p4: [u64; 7] = [0; 7];
        let mut p6: [u64; 3] = [0; 3];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            0x3333,
            p4.as_mut_ptr() as usize,
            0x5555,
            p6.as_mut_ptr() as usize,
            0x7777,
        );
        let base_out = core::ptr::addr_of!(OUT_BUF) as usize as u64;
        p4[1] = p4[1].wrapping_sub(base_out);
        p4[4] = p4[4].wrapping_sub(base_out);
        p4[6] = p4[6].wrapping_sub(base_out);
        p6[1] = 0;
        let mut out_slice = [0u64; 8];
        out_slice.copy_from_slice(&OUT_BUF[..8]);
        (p4, p6, ARR_80, out_slice, FREE_CALLS)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2f8_slot, saved_2f8);
    drop(_iat_sem);

    crate::diff_eq!(o, n, "1420c43d0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420dae80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_cac0(out: usize, _p2: usize, id: u16) -> usize {
        wr::<u16>(out, id.wrapping_mul(3).wrapping_add(7));
        out
    }

    let _m1 = crate::difftest::Mock::new(0x1_420d_cac0, mock_cac0 as *const ())?;

    let run = |f: F, same_gen: bool| -> [u8; 0x20] {
        let mut p2 = vec![0u8; 0x5870];
        wr::<u64>(p2.as_mut_ptr() as usize + 0x5868, 42);

        let mut p1 = [0u8; 0x20];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u16>(p1p + 0x08, 10);
        wr::<u16>(p1p + 0x0a, 20);
        wr::<u64>(p1p + 0x10, if same_gen { 42 } else { 41 });
        wr::<u16>(p1p + 0x18, 0x1111);
        wr::<u16>(p1p + 0x1a, 0x2222);

        f(p1p, p2.as_ptr() as usize);
        p1
    };

    for same in [true, false] {
        let o = run(f_o, same);
        let n = run(f_n, same);
        crate::diff_eq!(o, n, "1420dae80 same={}", same);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420dcc30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_9b50(tp: usize, sv: usize, _zero: u32) {
        let s_ptr = rd::<usize>(sv);
        let s_len = rd::<u64>(sv + 0x08) as u32;
        wr::<u32>(tp + 0x04, s_len);
        for i in 0..(s_len as usize).min(16) {
            wr::<u8>(tp + 0x08 + i, rd::<u8>(s_ptr + i));
        }
    }
    unsafe extern "win64" fn mock_aa00(dst: usize, src: usize) {
        let data = rd::<[u64; 5]>(src);
        wr::<[u64; 5]>(dst, data);
    }
    unsafe extern "win64" fn mock_ab10(_tp: usize) {}
    unsafe extern "win64" fn mock_6570(_vec: usize, new_cap: u64) {
        wr::<u32>(_vec + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411f_6570, mock_6570 as *const ())?;

    let run = |f: F, mode: u32| -> (u16, u32, u32, u64, u8) {
        let mut entries = vec![0u8; 0x38 * 4];
        let ep = entries.as_mut_ptr() as usize;
        // Entry 0: inline string "foo" (len=3)
        wr::<u32>(ep, 0x0400_0004);
        wr::<u32>(ep + 0x04, 3);
        wr::<[u8; 4]>(ep + 0x08, *b"foo\0");

        let mut p2 = vec![0u8; 0x5ab0];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x5870, ep);
        wr::<u32>(p2p + 0x5878, 1);
        wr::<u32>(p2p + 0x587c, 2);
        wr::<u32>(p2p + 0x5a98, 0xffff_ffff);

        let mut out_u16: u16 = 0x9999;
        let p4_val: u16 = 0x1234;
        let s_foo = b"foo\0";
        let s_bar = b"bar_custom_name\0";
        let static_str = rd::<usize>((exe::ptr::<u8>(0x1_4386_1ff0) as usize) + 0x08);

        let ret = match mode {
            0 => f((&mut out_u16 as *mut u16) as usize, 0, s_foo.as_ptr() as usize, 0, 0),
            1 => f((&mut out_u16 as *mut u16) as usize, p2p, static_str, 0, 0),
            2 => f(
                (&mut out_u16 as *mut u16) as usize,
                p2p,
                s_foo.as_ptr() as usize,
                0,
                0,
            ),
            3 => f(
                (&mut out_u16 as *mut u16) as usize,
                p2p,
                s_bar.as_ptr() as usize,
                0,
                0,
            ),
            _ => f(
                (&mut out_u16 as *mut u16) as usize,
                p2p,
                s_bar.as_ptr() as usize,
                (&p4_val as *const u16) as usize,
                7,
            ),
        };
        assert_eq!(ret, (&mut out_u16 as *mut u16) as usize);
        (
            out_u16,
            rd::<u32>(p2p + 0x5878),
            rd::<u32>(p2p + 0x5a9c),
            rd::<u64>(ep + 0x38 + 0x28),
            rd::<u8>(ep + 0x38 + 0x30),
        )
    };

    for mode in 0..5 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1420dcc30 mode={}", mode);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1420e9790(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FS_VTBL: [usize; 6] = [0; 6];
    static mut P3_VTBL: [usize; 2] = [0; 2];
    static mut LOG_HASH: u64 = 0;

    unsafe extern "win64" fn mock_f850(out: usize, span: usize) {
        wr::<u64>(out, rd::<u64>(span + 0x08));
    }
    unsafe extern "win64" fn mock_1120(path_obj: usize) -> u8 {
        u8::from(rd::<u64>(path_obj) > 0)
    }
    unsafe extern "win64" fn mock_p3_open(_p3: usize, _path: usize) -> usize {
        0x7777
    }
    unsafe extern "win64" fn mock_p3_close(_p3: usize, stream: usize) {
        LOG_HASH ^= stream as u64;
    }
    unsafe extern "win64" fn mock_5ac0(fs_ptr: usize, out_cnt: usize) {
        wr::<usize>(fs_ptr, core::ptr::addr_of!(FS_VTBL) as usize);
        wr::<i32>(out_cnt, 1);
    }
    unsafe extern "win64" fn mock_fs_read_str(_fs: usize, c8_ptr: usize) {
        wr::<u32>(c8_ptr + 0x04, 3);
    }
    unsafe extern "win64" fn mock_fs_read_wstr(_fs: usize, wstr_ptr: usize) {
        let buf = rd::<usize>(wstr_ptr);
        wr::<u16>(buf, b'A' as u16);
        wr::<u16>(buf + 2, b'B' as u16);
        wr::<u16>(buf + 4, 0);
        wr::<u32>(wstr_ptr + 0x10, 2);
    }
    unsafe extern "win64" fn mock_22d0(vec: usize, new_cap: u64) {
        wr::<u32>(vec + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_7fc0(vec: usize, new_cap: u64) {
        wr::<u32>(vec + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_7ca0(vec: usize, c8_ptr: usize) {
        wr::<u32>(vec + 0x08, rd::<u32>(vec + 0x08).wrapping_add(1));
        LOG_HASH = LOG_HASH.wrapping_add(rd::<u32>(c8_ptr + 0x04) as u64);
    }
    unsafe extern "win64" fn mock_f2b0(_wstr: usize, _len: u32) {}
    unsafe extern "win64" fn mock_ab10(_c8: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4249_1120, mock_1120 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4249_5ac0, mock_5ac0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401f_22d0, mock_22d0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_7fc0, mock_7fc0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411f_7ca0, mock_7ca0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_423e_f2b0, mock_f2b0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, empty_path: bool| -> (u8, u8, u32, u32, [u16; 4], u64) {
        FS_VTBL = [0; 6];
        FS_VTBL[3] = mock_fs_read_wstr as *const () as usize;
        FS_VTBL[4] = mock_fs_read_str as *const () as usize;
        P3_VTBL = [
            mock_p3_open as *const () as usize,
            mock_p3_close as *const () as usize,
        ];
        LOG_HASH = 0;

        let mut wstr_storage = vec![0u8; 0x120 * 2];
        let wsp = wstr_storage.as_mut_ptr() as usize;
        let mut p2 = [0u8; 0x30];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x18, wsp);

        let p3: [usize; 1] = [core::ptr::addr_of!(P3_VTBL) as usize];
        let path_bytes: &[u8] = if empty_path { b"\0" } else { b"test.bin\0" };
        let ret = f(path_bytes.as_ptr() as usize, p2p, p3.as_ptr() as usize);
        (
            ret,
            rd::<u8>(p2p),
            rd::<u32>(p2p + 0x20),
            rd::<u32>(wsp + 0x10),
            rd::<[u16; 4]>(wsp + 0x20),
            LOG_HASH,
        )
    };

    for empty in [true, false] {
        let o = run(f_o, empty);
        let n = run(f_n, empty);
        crate::diff_eq!(o, n, "1420e9790 empty={}", empty);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142211f20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    for (a, b) in [(-5i32, 10i32), (10, -5), (7, 7), (0x1234_5600, 0x1234_5601)] {
        let pa = (&a as *const i32) as usize;
        let pb = (&b as *const i32) as usize;
        let o = f_o(pa, pb);
        let n = f_n(pa, pb);
        crate::diff_eq!(o, n, "142211f20 a={} b={}", a, b);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_142212fa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, hit_inner: bool| -> ([u64; 8], [u64; 8]) {
        let mut sub0 = [0u8; 0x10];
        wr::<i32>(sub0.as_mut_ptr() as usize + 0x08, 14);

        let mut p1 = [0u8; 0xa0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, sub0.as_ptr() as usize);
        // Outer box: [0.0, 0.0] .. [10.0, 10.0]
        wr::<f64>(p1p + 0x28, 0.0);
        wr::<f64>(p1p + 0x30, 0.0);
        wr::<f64>(p1p + 0x48, 10.0);
        wr::<f64>(p1p + 0x50, 10.0);
        // Inner box: either [2.0, 2.0]..[6.0, 6.0] (hit) or [20.0, 20.0]..[30.0, 30.0] (miss)
        if hit_inner {
            wr::<f64>(p1p + 0x68, 2.0);
            wr::<f64>(p1p + 0x70, 2.0);
            wr::<f64>(p1p + 0x88, 6.0);
            wr::<f64>(p1p + 0x90, 6.0);
        } else {
            wr::<f64>(p1p + 0x68, 20.0);
            wr::<f64>(p1p + 0x70, 20.0);
            wr::<f64>(p1p + 0x88, 30.0);
            wr::<f64>(p1p + 0x90, 30.0);
        }

        let mut p3 = [0u64; 8];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<f64>(p3p, 5.0);
        wr::<f64>(p3p + 0x08, 5.0);
        wr::<f64>(p3p + 0x10, 2.0);
        wr::<f64>(p3p + 0x18, 2.0);
        wr::<i32>(p3p + 0x20, 100);
        wr::<i32>(p3p + 0x30, 3);

        let mut p2 = [0u64; 8];
        let p2p = p2.as_mut_ptr() as usize;
        let ret = f(p1p, p2p, p3p);
        assert_eq!(ret, p2p);
        (p2, p3)
    };

    for hit in [true, false] {
        let o = run(f_o, hit);
        let n = run(f_n, hit);
        crate::diff_eq!(o, n, "142212fa0 hit={}", hit);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1422132f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    for (a24, b24, a20, b20, a34, b34) in [
        (0u32, 1u32, 1u32, 1u32, 5i32, 10i32),
        (1, 0, 1, 1, 10, 5),
        (1, 1, 2, 2, 5, 10),
        (1, 1, 2, 2, 10, 5),
    ] {
        let mut p1 = [0u8; 0x40];
        let mut p2 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x20, a20);
        wr::<u32>(p1p + 0x24, a24);
        wr::<i32>(p1p + 0x34, a34);
        wr::<u32>(p2p + 0x20, b20);
        wr::<u32>(p2p + 0x24, b24);
        wr::<i32>(p2p + 0x34, b34);

        let o = f_o(p1p, p2p);
        let n = f_n(p1p, p2p);
        crate::diff_eq!(o, n, "1422132f0 a34={} b34={}", a34, b34);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_142213ae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut COPY_BUF: [u64; 8 * 4] = [0; 8 * 4];
    static mut MAX_BUF: [u32; 8] = [0; 8];
    static mut FREE_CNT: u32 = 0;

    unsafe extern "win64" fn mock_b7a0(_this: usize, _cnt: u64) -> usize {
        core::ptr::addr_of_mut!(COPY_BUF) as usize
    }
    unsafe extern "win64" fn mock_ecf0(_this: usize, _cnt: u64) -> usize {
        core::ptr::addr_of_mut!(MAX_BUF) as usize
    }
    unsafe extern "win64" fn mock_e830(vec_ptr: usize) {
        if rd::<usize>(vec_ptr) != 0 {
            FREE_CNT += 1;
        }
        wr::<u64>(vec_ptr, 0);
        wr::<u64>(vec_ptr + 0x08, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4023_b7a0, mock_b7a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_ecf0, mock_ecf0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F| -> ([u64; 32], u32, u32) {
        COPY_BUF = [0; 32];
        MAX_BUF = [0; 8];
        FREE_CNT = 0;

        let mut inner = [0u8; 0x40];
        wr::<i32>(inner.as_mut_ptr() as usize + 0x30, 2); // group size = 2
        let p1: [usize; 1] = [inner.as_ptr() as usize];

        // 4 items (2 groups of 2): group 0 has max +0x24 = 15, group 1 has max +0x24 = 50
        let mut items = [0u64; 32];
        let ip = items.as_mut_ptr() as usize;
        for g in 0..4usize {
            wr::<u64>(ip + g * 0x40, (g + 1) as u64);
        }
        wr::<i32>(ip + 0 * 0x40 + 0x24, 10);
        wr::<i32>(ip + 1 * 0x40 + 0x24, 15);
        wr::<i32>(ip + 2 * 0x40 + 0x24, 50);
        wr::<i32>(ip + 3 * 0x40 + 0x24, 20);

        let mut p2: [u64; 2] = [ip as u64, 4u64 | (4u64 << 32)];
        f(p1.as_ptr() as usize, p2.as_mut_ptr() as usize);
        (items, rd::<u32>((p2.as_ptr() as usize) + 0x08), FREE_CNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142213ae0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_420b_b3a0, "unknown_1420bb3a0", rend::cauldron_hot_y3::unknown_1420bb3a0 as *const (), Throws::Caught, rend::cauldron_hot_y3::diff_unknown_1420bb3a0),
// Replacement::verified(0x1_420b_bd40, "unknown_1420bbd40", rend::cauldron_hot_y3::unknown_1420bbd40 as *const (), rend::cauldron_hot_y3::diff_unknown_1420bbd40),
// Replacement::verified_throws(0x1_420b_c770, "unknown_1420bc770", rend::cauldron_hot_y3::unknown_1420bc770 as *const (), Throws::Caught, rend::cauldron_hot_y3::diff_unknown_1420bc770),
// Replacement::verified(0x1_420c_2c10, "unknown_1420c2c10", rend::cauldron_hot_y3::unknown_1420c2c10 as *const (), rend::cauldron_hot_y3::diff_unknown_1420c2c10),
// Replacement::verified_throws(0x1_420c_43d0, "unknown_1420c43d0", rend::cauldron_hot_y3::unknown_1420c43d0 as *const (), Throws::Caught, rend::cauldron_hot_y3::diff_unknown_1420c43d0),
// Replacement::verified(0x1_420d_ae80, "unknown_1420dae80", rend::cauldron_hot_y3::unknown_1420dae80 as *const (), rend::cauldron_hot_y3::diff_unknown_1420dae80),
// Replacement::verified(0x1_420d_cc30, "unknown_1420dcc30", rend::cauldron_hot_y3::unknown_1420dcc30 as *const (), rend::cauldron_hot_y3::diff_unknown_1420dcc30),
// Replacement::verified_throws(0x1_420e_9790, "unknown_1420e9790", rend::cauldron_hot_y3::unknown_1420e9790 as *const (), Throws::Caught, rend::cauldron_hot_y3::diff_unknown_1420e9790),
// Replacement::verified(0x1_4221_1f20, "unknown_142211f20", rend::cauldron_hot_y3::unknown_142211f20 as *const (), rend::cauldron_hot_y3::diff_unknown_142211f20),
// Replacement::verified(0x1_4221_2fa0, "unknown_142212fa0", rend::cauldron_hot_y3::unknown_142212fa0 as *const (), rend::cauldron_hot_y3::diff_unknown_142212fa0),
// Replacement::verified_throws(0x1_4221_32f0, "unknown_1422132f0", rend::cauldron_hot_y3::unknown_1422132f0 as *const (), Throws::Caught, rend::cauldron_hot_y3::diff_unknown_1422132f0),
// Replacement::verified(0x1_4221_3ae0, "unknown_142213ae0", rend::cauldron_hot_y3::unknown_142213ae0 as *const (), rend::cauldron_hot_y3::diff_unknown_142213ae0),
