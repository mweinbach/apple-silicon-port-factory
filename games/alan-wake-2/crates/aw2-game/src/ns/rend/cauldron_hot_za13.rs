//! Decompiled `rend` hot functions shard ZA13 (`0x142065820..0x14212c3e0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn push_job_handle_to_rsi(rsi: usize, handle: [u64; 2]) {
    let edx_len = rd::<u32>(rsi + 0x08) as u64;
    let r8d_cap = rd::<u32>(rsi + 0x0c) as u64;
    if (edx_len << 4) == (r8d_cap << 4) {
        let min_cap = edx_len.wrapping_add(1);
        let mut new_cap = (r8d_cap >> 1).wrapping_add(r8d_cap);
        if new_cap < min_cap {
            new_cap = min_cap;
        }
        exe::call_va!(0x1_40a5_b830, (rsi, new_cap));
    }
    let cur_len = rd::<u32>(rsi + 0x08) as usize;
    let dst = rd::<usize>(rsi).wrapping_add(cur_len << 4);
    wr::<u64>(dst, handle[0]);
    wr::<u64>(dst + 0x08, handle[1]);
    wr::<u32>(rsi + 0x08, (cur_len as u32).wrapping_add(1));
}

#[inline(always)]
unsafe fn push_job_handle_to_local(
    rbx_ptr: &mut usize,
    r14_len: &mut u32,
    cap_7d: &mut u32,
    handle: [u64; 2],
) {
    let cur_end = (*rbx_ptr).wrapping_add((*r14_len as usize) << 4);
    let cap_end = (*rbx_ptr).wrapping_add((*cap_7d as usize) << 4);
    if cur_end == cap_end {
        let old_bytes = cur_end.wrapping_sub(*rbx_ptr);
        let old_len = ((old_bytes as isize) >> 4) as usize;
        let min_cap = old_len.wrapping_add(1);
        let old_cap = ((cap_end.wrapping_sub(*rbx_ptr) as isize) >> 4) as usize;
        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
        if new_cap < min_cap {
            new_cap = min_cap;
        }
        let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap << 4, 8u32) -> usize);
        exe::call_va!(0x1_4183_acc8, (new_buf, *rbx_ptr, old_bytes));
        if *rbx_ptr != 0 {
            exe::call_va!(0x1_4241_c190, (*rbx_ptr));
        }
        *rbx_ptr = new_buf;
        *cap_7d = new_cap as u32;
        *r14_len = old_len as u32;
    }
    let slot = (*rbx_ptr).wrapping_add((*r14_len as usize) << 4);
    wr::<u64>(slot, handle[0]);
    wr::<u64>(slot + 0x08, handle[1]);
    *r14_len = (*r14_len).wrapping_add(1);
}

#[inline(always)]
fn keep_pso_job(rax_elem: usize) -> bool {
    unsafe {
        let r8 = rd::<usize>(rax_elem);
        if r8 == 0 {
            return false;
        }
        let dx = rd::<u16>(r8 + 0x06) >> 3;
        let cx = rd::<u16>(rax_elem + 0x08);
        if cx != dx {
            return false;
        }
        rd::<u32>(r8) != 0
    }
}

/// `r_142065820`
/// Original: 0x142065820 (1529 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142065820(_param_1: usize) {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc50) as usize);
    let r13 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);

    let rax_m1 = rd::<u64>(r13).wrapping_sub(1);
    let r12_limit: u64 = if rax_m1 > 1 { rax_m1 } else { 1 };

    let mut rax_it = rd::<usize>(rsi);
    let r10_end = rax_it.wrapping_add((rd::<u32>(rsi + 0x08) as usize) << 4);
    if rax_it != r10_end {
        while rax_it != r10_end {
            if !keep_pso_job(rax_it) {
                break;
            }
            rax_it = rax_it.wrapping_add(0x10);
        }
        if rax_it != r10_end {
            let mut rcx_it = rax_it.wrapping_add(0x10);
            while rcx_it != r10_end {
                if keep_pso_job(rcx_it) {
                    wr::<u128>(rax_it, rd::<u128>(rcx_it));
                    rax_it = rax_it.wrapping_add(0x10);
                }
                rcx_it = rcx_it.wrapping_add(0x10);
            }
        }
    }

    let rdx_base = rd::<usize>(rsi);
    let rcx_end = rdx_base.wrapping_add((rd::<u32>(rsi + 0x08) as usize) << 4);
    if rax_it != rcx_end {
        if rax_it == rdx_base {
            wr::<u32>(rsi + 0x08, 0);
        } else {
            let removed = ((rcx_end.wrapping_sub(rax_it) as isize) >> 4) as u32;
            wr::<u32>(rsi + 0x08, rd::<u32>(rsi + 0x08).wrapping_sub(removed));
        }
    }

    let mut rbx_ptr: usize = 0;
    let mut r14_len: u32 = 0;
    let mut cap_7d: u32 = 0;

    let rdi_lock1 = rsi + 0x20;
    exe::call_iat!(0x1_4264_55a8, (rdi_lock1));
    let mut r8d = rd::<u32>(rsi + 0x30);
    let mut rdx_arr1 = rd::<usize>(rsi + 0x28);
    if rdx_arr1 != rdx_arr1.wrapping_add((r8d as usize) * 0x18) {
        let vtbl_lambda2 = exe::ptr::<u8>(0x1_433d_c968) as usize;
        loop {
            if (rd::<u32>(rsi + 0x08) as u64) >= r12_limit {
                break;
            }
            let top = rdx_arr1.wrapping_add((r8d as usize) * 0x18);
            let w0 = rd::<u64>(top - 0x18);
            let w1 = rd::<u64>(top - 0x10);
            let w2 = rd::<u64>(top - 0x08);
            wr::<u32>(rsi + 0x30, r8d.wrapping_sub(1));

            let rbx_sched = rd::<usize>(r13 + 0x10);
            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool =
                rd::<usize>(rbx_sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0xc0);
            let mut job_handle: [u64; 2] = [0; 2];
            exe::call_va!(0x1_4248_5aa0, (pool, job_handle.as_mut_ptr() as usize));
            let node = job_handle[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(node as u64, 1);
            wr::<u64>(node + 0x08, 0);
            wr::<usize>(node + 0x10, vtbl_lambda2);
            wr::<usize>(node + 0x18, rsi);
            wr::<u64>(node + 0x20, w0);
            wr::<u64>(node + 0x28, w1);
            wr::<u64>(node + 0x30, w2);

            push_job_handle_to_rsi(rsi, job_handle);
            push_job_handle_to_local(&mut rbx_ptr, &mut r14_len, &mut cap_7d, job_handle);

            r8d = rd::<u32>(rsi + 0x30);
            rdx_arr1 = rd::<usize>(rsi + 0x28);
            if rdx_arr1 == rdx_arr1.wrapping_add((r8d as usize) * 0x18) {
                break;
            }
        }
    }
    exe::call_iat!(0x1_4264_5598, (rdi_lock1));

    let rdi_lock2 = rsi + 0x68;
    exe::call_iat!(0x1_4264_55a8, (rdi_lock2));
    let mut edx_cnt2 = rd::<u32>(rsi + 0x78);
    let mut rcx_arr2 = rd::<usize>(rsi + 0x70);
    if rcx_arr2 != rcx_arr2.wrapping_add((edx_cnt2 as usize) * 0x90) {
        let vtbl_lambda3 = exe::ptr::<u8>(0x1_433d_c980) as usize;
        loop {
            if (rd::<u32>(rsi + 0x08) as u64) >= r12_limit {
                break;
            }
            let r8_elem = rcx_arr2
                .wrapping_add((edx_cnt2 as usize) * 0x90)
                .wrapping_sub(0x90);
            let mut moved_ptr: usize = 0;
            exe::call_va!(
                0x1_411d_b9b0,
                (
                    (&mut moved_ptr as *mut usize) as usize,
                    edx_cnt2 as u64,
                    r8_elem
                )
            );
            wr::<u32>(rsi + 0x78, rd::<u32>(rsi + 0x78).wrapping_sub(1));
            let rax_moved = moved_ptr;

            let rbx_sched = rd::<usize>(r13 + 0x10);
            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool =
                rd::<usize>(rbx_sched.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0xc0);
            let mut job_handle: [u64; 2] = [0; 2];
            exe::call_va!(0x1_4248_5aa0, (pool, job_handle.as_mut_ptr() as usize));
            let node = job_handle[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(node as u64, 1);
            wr::<u64>(node + 0x08, 0);
            wr::<usize>(node + 0x10, vtbl_lambda3);
            wr::<usize>(node + 0x18, rsi);
            wr::<usize>(node + 0x20, rax_moved);

            push_job_handle_to_rsi(rsi, job_handle);
            push_job_handle_to_local(&mut rbx_ptr, &mut r14_len, &mut cap_7d, job_handle);

            edx_cnt2 = rd::<u32>(rsi + 0x78);
            rcx_arr2 = rd::<usize>(rsi + 0x70);
            if rcx_arr2 == rcx_arr2.wrapping_add((edx_cnt2 as usize) * 0x90) {
                break;
            }
        }
    }
    exe::call_iat!(0x1_4264_5598, (rdi_lock2));

    let end_local = rbx_ptr.wrapping_add((r14_len as usize) << 4);
    if rbx_ptr != end_local {
        let mut cur = rbx_ptr;
        let mut r15_dispatched: u64 = 0;
        while cur != end_local {
            let node = rd::<usize>(cur);
            if rd::<u32>(node) == 1 {
                exe::call_va!(0x1_4248_6460, (rd::<usize>(r13 + 0x10), node));
                r15_dispatched = r15_dispatched.wrapping_add(1);
            }
            cur = cur.wrapping_add(0x10);
        }
        if r15_dispatched != 0 {
            let sched = rd::<usize>(r13 + 0x10);
            let sem_owner = rd::<usize>(sched);
            let sem_handle = rd::<usize>(sem_owner + 0x1008);
            exe::call_iat!(0x1_4264_5580, (sem_handle, r15_dispatched as u32, 0usize));
        }
    }

    if rbx_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (rbx_ptr));
    }
}

/// `r_142068040`
/// Original: 0x142068040 (72 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_142068040(param_1: usize) {
    let idx = rd::<i8>(param_1 + 0xa0) as isize;
    let base = exe::ptr::<u8>(0x1_4000_0000) as usize;
    let rel = rd::<u32>((base as isize + idx * 4 + 0x0206_8070) as usize);
    if rel == 0x0206_805f {
        return;
    }
    exe::call_va!(0x1_4020_8a00, ());
}

/// `r_142068ed0`
/// Original: 0x142068ed0 (384 bytes, Throws::No)
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn r_142068ed0(
    param_1: usize,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u16,
    param_6: u8,
    param_7: u32,
    param_8: usize,
    param_9: usize,
) -> usize {
    wr::<u32>(param_8, 0);
    wr::<u32>(param_9, 0);

    if (param_7 & 0x20) == 0 && (param_7 & 0x10) == 0 {
        let fmt_idx = (param_6 as i8) as isize;
        let tbl_base = exe::ptr::<u8>(0x1_4297_eda0) as usize;
        let dxgi_fmt = rd::<u32>((tbl_base as isize + fmt_idx * 0x14) as usize);
        let flags = (((param_7 >> 2) & 1) << 2) | (param_7 & 1);

        wr::<u32>(param_1, 4);
        wr::<u32>(param_1 + 0x04, 0);
        wr::<u64>(param_1 + 0x08, 0);
        wr::<i64>(param_1 + 0x10, param_2 as i64);
        wr::<u32>(param_1 + 0x18, param_3 as u32);
        wr::<u16>(param_1 + 0x1c, param_4 as u16);
        wr::<u16>(param_1 + 0x1e, param_5);
        wr::<u32>(param_1 + 0x20, dxgi_fmt);
        wr::<u64>(param_1 + 0x24, 1);
        wr::<u32>(param_1 + 0x2c, 0);
        wr::<u32>(param_1 + 0x30, flags);
        wr::<u32>(param_1 + 0x34, 0);
    } else {
        let fmt_i32 = (param_6 as i8) as i32;
        let raw_row = exe::call_va!(0x1_4204_e7f0, (param_2 as u32, 1u32, param_6) -> u32);
        let row_pitch = raw_row.wrapping_add(0xff) & !0xffu32;
        wr::<u32>(param_8, row_pitch);

        let mut ebx = param_3;
        if (fmt_i32.wrapping_sub(0x1c) as u32) <= 9 {
            ebx >>= 2;
        }
        let slice_pitch = (row_pitch as i32).wrapping_mul(ebx);
        wr::<i32>(param_9, slice_pitch);
        let total = slice_pitch.wrapping_mul(param_4);

        wr::<u32>(param_1, 1);
        wr::<u32>(param_1 + 0x04, 0);
        wr::<u64>(param_1 + 0x08, 0);
        wr::<i64>(param_1 + 0x10, total as i64);
        wr::<u32>(param_1 + 0x18, 1);
        wr::<u64>(param_1 + 0x1c, 0x0001_0001);
        wr::<u64>(param_1 + 0x24, 1);
        wr::<u64>(param_1 + 0x2c, 1);
        wr::<u32>(param_1 + 0x34, 0);
    }
    param_1
}

/// `unknown_14207d6c0`
/// Original: 0x14207d6c0 (804 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14207d6c0(param_1: usize, param_2: u8) -> u64 {
    if param_2 < rd::<u8>(param_1 + 0x6c) {
        let r8 = rd::<u32>(param_1 + 0x68) as usize;
        let mut r11d: u32 = 0;
        let r9 = rd::<usize>(param_1 + 0x08);
        if r8 != 0 {
            let mut rax: usize = 0;
            let mut rcx = r9.wrapping_add(0x1c);
            while rd::<u8>(rcx) < param_2 {
                rax += 1;
                rcx = rcx.wrapping_add(0x20);
                if rax >= r8 {
                    return 0;
                }
            }
            let rem = r8 - rax;
            let start = r9.wrapping_add(rax << 5);
            let end = start.wrapping_add(rem << 5);
            if start != end {
                let mut ptr = start.wrapping_add(0x18);
                loop {
                    let w_p2 = rd::<u16>(ptr + 0x02) as u32;
                    let w_m2 = rd::<u16>(ptr - 0x02) as u32;
                    let w_0 = rd::<u16>(ptr) as u32;
                    let p1 = (w_m2.wrapping_mul(w_p2)) as u16 as u32;
                    let p2 = (p1.wrapping_mul(w_0)) as u16 as u32;
                    r11d = r11d.wrapping_add(p2);
                    ptr = ptr.wrapping_add(0x20);
                    if ptr.wrapping_sub(0x18) == end {
                        break;
                    }
                }
            }
        }
        return (r11d as u64) << 16;
    }

    let kind = rd::<i8>(param_1 + 0x80);
    let mut desc = [0u8; 0x38];
    let desc_ptr = desc.as_mut_ptr() as usize;
    let mut tmp = [0u8; 0x38];
    let tmp_ptr = tmp.as_mut_ptr() as usize;

    if kind == 0 {
        let raw_fmt = rd::<u8>(param_1 + 0x81);
        let mips = rd::<u8>(param_1 + 0x82);
        let shift = (param_2 as u32) & 0x1f;
        let mut r8d = rd::<u32>(param_1 + 0x74) >> shift;
        let mut edx = rd::<u32>(param_1 + 0x70) >> shift;
        let r10d = (mips as u32).wrapping_sub(param_2 as u32);
        if r8d < 1 {
            r8d = 1;
        }
        if edx < 1 {
            edx = 1;
        }
        let r9b = if raw_fmt == 0x11 { 0x12u8 } else { raw_fmt };
        if (((r9b as i8) as i32).wrapping_sub(0x1c) as u32) <= 9 {
            edx = edx.wrapping_add(3) & !3u32;
            r8d = r8d.wrapping_add(3) & !3u32;
        }
        let mut out_row: u32 = 0;
        let mut out_slice: u32 = 0;
        let src = exe::call_va!(
            0x1_4206_9110,
            (
                tmp_ptr,
                edx as i32,
                r8d,
                1u16,
                r10d as u16,
                r9b,
                0u32,
                (&mut out_row as *mut u32) as usize,
                (&mut out_slice as *mut u32) as usize
            ) -> usize
        );
        wr::<[u8; 0x38]>(desc_ptr, rd::<[u8; 0x38]>(src));
    } else if kind == 1 {
        let raw_fmt = rd::<u8>(param_1 + 0x81);
        let mips = rd::<u8>(param_1 + 0x82);
        let shift = (param_2 as u32) & 0x1f;
        let mut r9d = rd::<u32>(param_1 + 0x78) >> shift;
        let mut r8d = rd::<u32>(param_1 + 0x74) >> shift;
        let mut edx = rd::<u32>(param_1 + 0x70) >> shift;
        let r10d = (mips as u32).wrapping_sub(param_2 as u32);
        if r9d < 1 {
            r9d = 1;
        }
        if r8d < 1 {
            r8d = 1;
        }
        if edx < 1 {
            edx = 1;
        }
        let fmt = if raw_fmt == 0x11 { 0x12u8 } else { raw_fmt };
        let mut out_row: u32 = 0;
        let mut out_slice: u32 = 0;
        let src = exe::call_va!(
            0x1_4206_8ed0,
            (
                tmp_ptr,
                edx as i32,
                r8d as i32,
                r9d as i32,
                r10d as u16,
                fmt,
                0u32,
                (&mut out_row as *mut u32) as usize,
                (&mut out_slice as *mut u32) as usize
            ) -> usize
        );
        wr::<[u8; 0x38]>(desc_ptr, rd::<[u8; 0x38]>(src));
    } else if kind == 2 {
        let raw_fmt = rd::<u8>(param_1 + 0x81);
        let mips = rd::<u8>(param_1 + 0x82);
        let shift = (param_2 as u32) & 0x1f;
        let mut r8d = rd::<u32>(param_1 + 0x70) >> shift;
        let r9w = (mips as u16).wrapping_sub(param_2 as u16);
        if r8d < 1 {
            r8d = 1;
        }
        let fmt = if raw_fmt == 0x11 { 0x12u8 } else { raw_fmt };
        let tbl_base = exe::ptr::<u8>(0x1_4297_eda0) as usize;
        let dxgi_fmt = rd::<u32>((tbl_base as isize + ((fmt as i8) as isize) * 0x14) as usize);

        wr::<u32>(desc_ptr, 3);
        wr::<u32>(desc_ptr + 0x04, 0);
        wr::<u64>(desc_ptr + 0x08, 0);
        wr::<i64>(desc_ptr + 0x10, (r8d as i32) as i64);
        wr::<u32>(desc_ptr + 0x18, r8d);
        wr::<u16>(desc_ptr + 0x1c, 6);
        wr::<u16>(desc_ptr + 0x1e, r9w);
        wr::<u32>(desc_ptr + 0x20, dxgi_fmt);
        wr::<u64>(desc_ptr + 0x24, 1);
        wr::<u64>(desc_ptr + 0x2c, 0);
        wr::<u32>(desc_ptr + 0x34, 0);
    } else {
        return 0;
    }

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let vtbl = rd::<usize>(dev);
    let fn_c8 = rd::<u64>(vtbl + 0xc8);
    let mut out_info: [u64; 2] = [0; 2];
    exe::call_va!(
        fn_c8,
        (dev, out_info.as_mut_ptr() as usize, 0u32, 1u32, desc_ptr)
    );
    out_info[0]
}

/// `unknown_142080120`
/// Original: 0x142080120 (904 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142080120(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
) -> usize {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_8020) as usize) == 0 {
        wr::<usize>(param_1, 0);
        return param_1;
    }

    let mut bl = rd::<u8>(param_2 + 0x10);
    if bl == 0 {
        let fmt = rd::<i8>(param_2 + 0x11);
        let eax = exe::call_va!(0x1_4204_e7f0, (4u32, 4u32, fmt as u8) -> u32);
        let shift_sub = if eax <= 1 {
            0u32
        } else {
            let dec = eax - 1;
            if dec == 0 {
                0
            } else {
                (31 - dec.leading_zeros()) + 1
            }
        };
        let ecx = 0xcu32.wrapping_sub(shift_sub);
        let half = ecx >> 1;
        let rem = ecx.wrapping_sub(half);
        let edx_tile_h = 4u32 << (half & 0x1f);
        let r8d_tile_w = 4u32 << (rem & 0x1f);

        let mut r12_total: u64 = 0;
        let mut r10d_w = rd::<u32>(param_2);
        let mut r11d_h = rd::<u32>(param_2 + 0x04);
        let mips = rd::<u8>(param_2 + 0x12);
        if mips != 0 {
            let neg_h = edx_tile_h.wrapping_neg();
            let neg_w = r8d_tile_w.wrapping_neg();
            let is_bc = ((fmt as i32).wrapping_sub(0x1c) as u32) <= 9;
            for _ in 0..(mips as usize) {
                if is_bc {
                    r10d_w = r10d_w.wrapping_add(3) & !3u32;
                    r11d_h = r11d_h.wrapping_add(3) & !3u32;
                }
                let aligned_h = edx_tile_h.wrapping_sub(1).wrapping_add(r11d_h) & neg_h;
                let aligned_w = r10d_w.wrapping_sub(1).wrapping_add(r8d_tile_w) & neg_w;
                let mip_bytes =
                    exe::call_va!(0x1_4204_e7f0, (aligned_w, aligned_h, fmt as u8) -> i32);
                r12_total = r12_total.wrapping_add((mip_bytes as i64) as u64);
                r10d_w >>= 1;
                if r10d_w < 1 {
                    r10d_w = 1;
                }
                r11d_h >>= 1;
                if r11d_h < 1 {
                    r11d_h = 1;
                }
            }
            bl = rd::<u8>(param_2 + 0x10);
        }
        if r12_total <= 0x10000 {
            wr::<usize>(param_1, 0);
            return param_1;
        }
    }

    let mut desc = [0u8; 0x38];
    let dp = desc.as_mut_ptr() as usize;

    match bl as i8 {
        0 | 2 | 5 | 6 => {
            let g_698 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
            if rd::<u8>(g_698 + 0x20) == 0 {
                wr::<usize>(param_1, 0);
                return param_1;
            }
            wr::<u32>(dp, 3);
        }
        1 => {
            let g_698 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
            if rd::<u8>(g_698 + 0x21) == 0 {
                wr::<usize>(param_1, 0);
                return param_1;
            }
            wr::<u32>(dp, 4);
        }
        3 | 4 => {
            wr::<usize>(param_1, 0);
            return param_1;
        }
        _ => {}
    }

    wr::<u64>(dp + 0x08, 0x10000);
    let r8_w = rd::<u32>(param_2) as u64;
    wr::<u64>(dp + 0x10, r8_w);
    let r9d_h = rd::<u32>(param_2 + 0x04);
    wr::<u32>(dp + 0x18, r9d_h);
    wr::<u16>(dp + 0x1c, rd::<u16>(param_2 + 0x08));
    wr::<u16>(dp + 0x1e, rd::<u8>(param_2 + 0x12) as u16);

    let fmt_i8 = rd::<i8>(param_2 + 0x11);
    let tbl_base = exe::ptr::<u8>(0x1_4297_eda0) as usize;
    let dxgi_fmt = rd::<u32>((tbl_base as isize + (fmt_i8 as isize) * 0x14) as usize);
    wr::<u32>(dp + 0x20, dxgi_fmt);
    wr::<u64>(dp + 0x24, 1);
    wr::<u64>(dp + 0x2c, 2);

    let is_bc = exe::call_va!(0x1_4204_ebd0, (fmt_i8 as u8) -> u8);
    if is_bc != 0 {
        wr::<u64>(dp + 0x10, (r8_w + 3) & !3u64);
        wr::<u32>(dp + 0x18, r9d_h.wrapping_add(3) & !3u32);
    }

    let mut d3d_res: usize = 0;
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let dev_vtbl = rd::<usize>(dev);
    let fn_f0 = rd::<u64>(dev_vtbl + 0xf0);
    let iid = exe::ptr::<u8>(0x1_433d_c7e8) as usize;
    let hr = exe::call_va!(
        fn_f0,
        (
            dev,
            dp,
            0u32,
            0usize,
            iid,
            (&mut d3d_res as *mut usize) as usize
        ) -> u32
    );
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 1 || hr == 0x887a_0007 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }

    let mut z1: u64 = 0;
    let mut z2: u64 = 0;
    let mut z3: u64 = 0;
    let mut z4: u64 = 0;
    let mut out_obj: usize = 0;
    exe::call_va!(
        0x1_411d_cc60,
        (
            (&mut out_obj as *mut usize) as usize,
            0usize,
            d3d_res,
            0usize,
            (&mut z1 as *mut u64) as usize,
            (&mut z2 as *mut u64) as usize,
            (&mut z3 as *mut u64) as usize,
            (&mut z4 as *mut u64) as usize
        )
    );

    let rbx = out_obj;
    wr::<u8>(rbx + 0x51, rd::<u8>(param_2 + 0x10));
    wr::<u8>(rbx + 0x18, rd::<u8>(param_2 + 0x11));
    wr::<u32>(rbx, rd::<u32>(param_2));
    wr::<u32>(rbx + 0x04, rd::<u32>(param_2 + 0x04));
    wr::<u32>(rbx + 0x08, rd::<u32>(param_2 + 0x08));
    wr::<u32>(rbx + 0x1c, 1);
    wr::<u32>(rbx + 0x0c, rd::<u8>(param_2 + 0x12) as u32);
    exe::call_va!(0x1_4206_8040, (rbx));
    wr::<u8>(rbx + 0x50, 1);
    wr::<usize>(param_1, rbx);

    if d3d_res != 0 {
        let vtbl = rd::<usize>(d3d_res);
        let fn_rel = rd::<u64>(vtbl + 0x10);
        exe::call_va!(fn_rel, (d3d_res));
    }
    param_1
}

#[inline(always)]
fn u64_to_f32_msvc(v: u64) -> f32 {
    if (v as i64) >= 0 {
        (v as i64) as f32
    } else {
        let half = (((v >> 1) | (v & 1)) as i64) as f32;
        half + half
    }
}

/// `std_1420a46f0`
/// Original: 0x1420a46f0 (984 bytes, Throws::Caught)
pub unsafe extern "win64" fn std_1420a46f0(
    param_1: usize,
    param_2: usize,
    param_3: u16,
    param_4: u32,
) -> usize {
    let p1_8 = rd::<usize>(param_1 + 0x08);
    let init_w = rd::<u16>(p1_8 + 0x48);
    wr::<u16>(param_2, init_w);
    wr::<u16>(param_2 + 0x02, init_w);

    let end_idx = (param_4 >> 16) as u16;
    let mut cur_idx = param_4 as u16;
    if cur_idx == end_idx {
        return param_2;
    }

    let c_2p63 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fc8) as usize);

    let mut iter_buf = [0u8; 0x10];
    let iter_ptr = iter_buf.as_mut_ptr() as usize;
    let mut range_buf = [0u8; 0x20];
    let range_ptr = range_buf.as_mut_ptr() as usize;

    loop {
        let mesh_base = rd::<usize>(rd::<usize>(param_1));
        let range_dword = rd::<u32>(mesh_base.wrapping_add((cur_idx as usize) * 0x40 + 0x3a));
        wr::<u32>(range_ptr, range_dword);
        wr::<usize>(range_ptr + 0x08, param_1);
        wr::<u64>(range_ptr + 0x10, param_3 as u64);
        wr::<u8>(range_ptr + 0x18, 1);

        wr::<usize>(iter_ptr, range_ptr);
        wr::<u16>(iter_ptr + 0x08, range_dword as u16);
        wr::<u8>(iter_ptr + 0x0c, 0);
        exe::call_va!(0x1_420a_3310, (iter_ptr));

        let mut bx = rd::<u16>(iter_ptr + 0x08);
        if bx != rd::<u16>(rd::<usize>(iter_ptr) + 0x02) {
            loop {
                let ebp = rd::<u16>(iter_ptr + 0x0a) as usize;
                let r15 = rd::<usize>(rd::<usize>(param_1) + 0x10);
                wr::<u16>(r15.wrapping_add(ebp * 8 + 0x02), param_3);

                let rsi = rd::<usize>(param_1 + 0x08);
                let edx_len = rd::<u32>(rsi + 0x48) as u64;
                let r8d_cap = rd::<u32>(rsi + 0x4c) as u64;
                if (edx_len * 2) == (r8d_cap * 2) {
                    let need = edx_len.wrapping_add(1);
                    let grow = (r8d_cap >> 1).wrapping_add(r8d_cap);
                    let new_cap = if grow < need { need } else { grow };
                    exe::call_va!(0x1_40fb_3a60, (rsi + 0x40, new_cap));
                }
                let cur_len = rd::<u32>(rsi + 0x48) as usize;
                let arr_40 = rd::<usize>(rsi + 0x40);
                let val_u16 = rd::<u16>(r15.wrapping_add(ebp * 8));
                wr::<u16>(arr_40.wrapping_add(cur_len * 2), val_u16);
                wr::<u32>(rsi + 0x48, (cur_len as u32).wrapping_add(1));

                let rsi_map = rd::<usize>(param_1 + 0x08);
                if rd::<u64>(rsi_map + 0x10) == 0x0aaa_aaaa_aaaa_aaaau64 {
                    let msg = exe::ptr::<u8>(0x1_4307_e950) as usize;
                    exe::call_iat!(0x1_4264_5870, (msg));
                    return param_2;
                }

                let node = exe::call_va!(0x1_4241_c0a0, (0x18u64, 8u32) -> usize);
                let key_cx = rd::<u16>(r15.wrapping_add(ebp * 8));
                wr::<u16>(node + 0x10, key_cx);
                wr::<u16>(node + 0x12, param_3);

                let lo = (rd::<u8>(node + 0x10) as u64) ^ 0xcbf2_9ce4_8422_2325u64;
                let h1 = lo.wrapping_mul(0x0100_0000_01b3u64);
                let hi = (key_cx >> 8) as u64;
                let hash = (h1 ^ hi).wrapping_mul(0x0100_0000_01b3u64);

                let new_size = rd::<u64>(rsi_map + 0x10).wrapping_add(1);
                let max_load = rd::<f32>(rsi_map);
                let xmm1_size = u64_to_f32_msvc(new_size);
                let rdx_buckets = rd::<u64>(rsi_map + 0x38);
                let xmm0_buckets = u64_to_f32_msvc(rdx_buckets);
                if (xmm1_size / xmm0_buckets) > max_load {
                    let raw_req = xmm1_size / max_load;
                    let ceil_req =
                        f32::from_bits(crate::simd_ops::vroundss(0, raw_req.to_bits(), 2) as u32);
                    let mut adj: i64 = 0;
                    let mut f_conv = ceil_req;
                    if f_conv >= c_2p63 {
                        f_conv -= c_2p63;
                        if f_conv < c_2p63 {
                            adj = i64::MIN;
                        }
                    }
                    let req_int = ((f_conv as i64).wrapping_add(adj)) as u64;
                    let min_b = if req_int > 8 { req_int } else { 8 };
                    let mut target_b = rdx_buckets;
                    if target_b < min_b {
                        if target_b >= 0x200 || target_b.wrapping_mul(8) < min_b {
                            target_b = min_b;
                        } else {
                            target_b = target_b.wrapping_mul(8);
                        }
                    }
                    exe::call_va!(0x1_411e_bfc0, (rsi_map, target_b));
                }

                let mask = rd::<u64>(rsi_map + 0x30);
                let bucket_idx = (mask & hash) as usize;
                let buckets_base = rd::<usize>(rsi_map + 0x18);
                let bucket_hi = rd::<usize>(buckets_base.wrapping_add(bucket_idx * 0x10 + 0x08));
                let head_sentinel = rd::<usize>(rsi_map + 0x08);
                let insert_pos: usize = if bucket_hi == head_sentinel {
                    head_sentinel
                } else {
                    let bucket_lo = rd::<usize>(buckets_base.wrapping_add(bucket_idx * 0x10));
                    let k_new = rd::<u16>(node + 0x10);
                    let mut cur = bucket_hi;
                    loop {
                        if rd::<u16>(cur + 0x10) == k_new {
                            break rd::<usize>(cur);
                        }
                        if cur == bucket_lo {
                            break cur;
                        }
                        cur = rd::<usize>(cur + 0x08);
                    }
                };

                let r8_prev = rd::<usize>(insert_pos + 0x08);
                wr::<u64>(rsi_map + 0x10, rd::<u64>(rsi_map + 0x10).wrapping_add(1));
                wr::<usize>(node, insert_pos);
                wr::<usize>(node + 0x08, r8_prev);
                wr::<usize>(r8_prev, node);
                wr::<usize>(insert_pos + 0x08, node);

                let b_base = rd::<usize>(rsi_map + 0x18);
                let b_idx = ((rd::<u64>(rsi_map + 0x30) & hash) as usize) * 0x10;
                let b_slot = b_base.wrapping_add(b_idx);
                let r9_lo = rd::<usize>(b_slot);
                if r9_lo == rd::<usize>(rsi_map + 0x08) {
                    wr::<usize>(b_slot, node);
                    wr::<usize>(b_slot + 0x08, node);
                } else if r9_lo == insert_pos {
                    wr::<usize>(b_slot, node);
                } else if rd::<usize>(b_slot + 0x08) == r8_prev {
                    wr::<usize>(b_slot + 0x08, node);
                }

                let lo16 = rd::<u16>(param_2);
                let hi16 = rd::<u16>(param_2 + 0x02).wrapping_add(1);
                wr::<u32>(param_2, (lo16 as u32) | ((hi16 as u32) << 16));

                bx = bx.wrapping_add(1);
                wr::<u16>(iter_ptr + 0x08, bx);
                exe::call_va!(0x1_420a_3310, (iter_ptr));
                bx = rd::<u16>(iter_ptr + 0x08);
                if bx == rd::<u16>(rd::<usize>(iter_ptr) + 0x02) {
                    break;
                }
            }
        }

        cur_idx = cur_idx.wrapping_add(1);
        if cur_idx == end_idx {
            break;
        }
    }
    param_2
}

/// `unknown_1420a4b10`
/// Original: 0x1420a4b10 (583 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420a4b10(param_1: usize, param_2: u16, param_3: u32) {
    let mut vec_c8 = [0u64; 2];
    let vec_ptr = vec_c8.as_mut_ptr() as usize;

    let rax0 = rd::<usize>(param_1);
    let r8d_init_cap = rd::<u32>(rax0 + 0x08);
    let mut rsi_ptr: usize = 0;
    let mut r15d_cap: u32 = 0;

    if r8d_init_cap != 0 {
        r15d_cap = r8d_init_cap;
        rsi_ptr = exe::call_va!(0x1_4241_c0a0, ((r8d_init_cap as u64) * 2, 2u32) -> usize);
        exe::call_va!(0x1_4183_acc8, (rsi_ptr, 0usize, 0usize));
        wr::<usize>(vec_ptr, rsi_ptr);
        wr::<u32>(vec_ptr + 0x08, 0);
        wr::<u32>(vec_ptr + 0x0c, r15d_cap);
    }

    if r15d_cap == 0 {
        let old_ptr = rd::<usize>(vec_ptr);
        let old_len = rd::<u32>(vec_ptr + 0x08) as usize;
        rsi_ptr = exe::call_va!(0x1_4241_c0a0, (2u64, 2u32) -> usize);
        exe::call_va!(0x1_4183_acc8, (rsi_ptr, old_ptr, old_len * 2));
        if old_ptr != 0 {
            exe::call_va!(0x1_4241_c190, (old_ptr));
        }
        wr::<usize>(vec_ptr, rsi_ptr);
        wr::<u32>(vec_ptr + 0x08, 0);
        wr::<u32>(vec_ptr + 0x0c, 1);
    }

    wr::<u16>(rsi_ptr, param_2);
    wr::<u32>(vec_ptr + 0x08, 1);

    let mut view_ctx = [0u8; 0x20];
    let view_ctx_ptr = view_ctx.as_mut_ptr() as usize;
    let mut arg2_buf = [0u8; 0x28];
    let arg2_ptr = arg2_buf.as_mut_ptr() as usize;
    let mut arg3_buf = [0u8; 0x28];
    let arg3_ptr = arg3_buf.as_mut_ptr() as usize;

    while rd::<u32>(vec_ptr + 0x08) != 0 {
        let edx_len = rd::<u32>(vec_ptr + 0x08);
        let rcx_buf = rd::<usize>(vec_ptr);
        let r8w_pop = rd::<u16>(rcx_buf.wrapping_add((edx_len as usize) * 2 - 2));
        wr::<u32>(vec_ptr + 0x08, edx_len - 1);

        let rdi_arr50 = rd::<usize>(rd::<usize>(param_1 + 0x08) + 0x50);
        let mut out_range: u32 = 0;
        let ret_ptr = exe::call_va!(
            0x1_420a_46f0,
            (
                param_1,
                (&mut out_range as *mut u32) as usize,
                r8w_pop,
                param_3
            ) -> usize
        );
        let r8d_val = rd::<u32>(ret_ptr);
        wr::<u32>(rdi_arr50.wrapping_add((r8w_pop as usize) * 4), r8d_val);

        let rdx_p1_8 = rd::<usize>(param_1 + 0x08);
        let count_u16 = ((r8d_val >> 16) as u16).wrapping_sub(r8d_val as u16);
        let base_ptr =
            rd::<usize>(rdx_p1_8 + 0x40).wrapping_add(((r8d_val as u16) as usize) * 2);

        wr::<u64>(view_ctx_ptr, count_u16 as u64);
        wr::<usize>(view_ctx_ptr + 0x08, base_ptr);
        wr::<usize>(view_ctx_ptr + 0x10, param_1);
        wr::<u8>(view_ctx_ptr + 0x18, 1);

        wr::<u8>(arg3_ptr + 0x20, 1);

        let mut rcx_it = base_ptr;
        let r8_end = base_ptr.wrapping_add((count_u16 as usize) * 2);
        let mut last_ptr: usize = 0;
        let mut has_last: u8 = 0;
        if rcx_it != r8_end {
            let r9_mesh = rd::<usize>(rd::<usize>(param_1));
            let r10_tab = rd::<usize>(rd::<usize>(param_1 + 0x10) + 0x10);
            loop {
                last_ptr = rcx_it;
                has_last = 1;
                let idx_u16 = rd::<u16>(rcx_it) as usize;
                let sub_u16 = rd::<u16>(r9_mesh.wrapping_add(idx_u16 * 0x40 + 0x22)) as usize;
                if rd::<u8>(r10_tab.wrapping_add(sub_u16 * 0x30 + 0x20)) != 0 {
                    break;
                }
                rcx_it = rcx_it.wrapping_add(2);
                if rcx_it == r8_end {
                    break;
                }
            }
        }

        wr::<usize>(arg2_ptr, view_ctx_ptr);
        wr::<usize>(arg2_ptr + 0x08, rcx_it);
        wr::<usize>(arg2_ptr + 0x10, last_ptr);
        wr::<u8>(arg2_ptr + 0x18, has_last);
        wr::<u8>(arg2_ptr + 0x20, 0);

        let mut out_dummy: usize = 0;
        exe::call_va!(
            0x1_411e_c200,
            (
                (&mut out_dummy as *mut usize) as usize,
                arg2_ptr,
                arg3_ptr,
                vec_ptr
            )
        );
    }

    let final_buf = rd::<usize>(vec_ptr);
    if final_buf != 0 {
        exe::call_va!(0x1_4241_c190, (final_buf));
    }
}

/// `unknown_14212c3e0`
/// Original: 0x14212c3e0 (1099 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14212c3e0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    exe::call_va!(0x1_4213_b7a0, ());
    let rdi = rd::<usize>(param_3);

    let g_d18 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3d18) as usize);
    let xmm6 = if 0.0f32 > g_d18 {
        rd::<f32>(rdi + 0x44)
    } else {
        g_d18
    };

    let g_b60 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3b60) as usize);
    let g_170 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize);
    let xmm1 = rd::<f32>(rdi + 0x40) - (g_b60 * g_170);
    let mut xmm3 = if xmm1 > 0.0f32 { xmm1 } else { 0.0f32 };
    let g_e10 = rd::<f32>(exe::ptr::<f32>(0x1_4386_3e10) as usize);
    if !(0.0f32 > g_e10) {
        xmm3 = g_e10;
    }

    let g_078 = rd::<f32>(exe::ptr::<f32>(0x1_4386_4078) as usize);
    let xmm0 = if 0.0f32 > g_078 {
        rd::<f32>(rdi + 0x48)
    } else {
        g_078
    };

    let mut f_40: [f32; 3] = [xmm3, xmm6, xmm0];
    let f40_ptr = f_40.as_mut_ptr() as usize;

    let mut rbx = rd::<usize>(param_2);
    if rbx == 0 {
        let new_obj = exe::call_va!(0x1_4123_dc30, () -> usize);
        let old_obj = rd::<usize>(param_2);
        wr::<usize>(param_2, new_obj);
        if old_obj != 0 {
            let vtbl = rd::<usize>(old_obj);
            exe::call_va!(rd::<u64>(vtbl), (old_obj, 0u32));
            exe::call_va!(0x1_4244_2de0, (old_obj));
        }
        rbx = rd::<usize>(param_2);
    }

    exe::call_va!(0x1_4212_e240, (rbx, f40_ptr));

    let g_cc8 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3cc8) as usize);
    let g_d00 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3d00) as usize);
    let g_190 = rd::<u8>(exe::ptr::<u8>(0x1_4386_4190) as usize);
    let g_e60 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3e60) as usize);
    let g_ba0 = rd::<u8>(exe::ptr::<u8>(0x1_4386_3ba0) as usize);

    if g_cc8 != 0 || g_d00 != 0 || g_190 != 0 || g_e60 != 0 || g_ba0 != 0 {
        exe::call_va!(0x1_4212_cc20, (rbx, f40_ptr, rdi));
    }

    let mut buf_170 = [0u8; 0x40];
    let b170_ptr = buf_170.as_mut_ptr() as usize;
    let mut buf_a0 = [0u8; 0x38];
    let a0_ptr = buf_a0.as_mut_ptr() as usize;
    let mut buf_50 = [0u64; 4];
    let b50_ptr = buf_50.as_mut_ptr() as usize;
    let mut buf_70 = [0u8; 0x30];
    let b70_ptr = buf_70.as_mut_ptr() as usize;

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_3d00) as usize) != 0 {
        let rax_src = exe::call_va!(0x1_4213_a660, (b170_ptr, rdi) -> usize);
        wr::<[u8; 0x34]>(a0_ptr, rd::<[u8; 0x34]>(rax_src));
        exe::call_va!(0x1_4213_b1f0, (b50_ptr, a0_ptr));

        let eax_len = rd::<u32>(b50_ptr + 0x08) as usize;
        let r8_ptr = rd::<usize>(b50_ptr);
        let cnt0 = (((eax_len * 12) as i64) / 12) as u64;
        wr::<u64>(b70_ptr, cnt0);
        wr::<usize>(b70_ptr + 0x08, r8_ptr);
        wr::<u64>(b70_ptr + 0x10, rd::<u32>(b50_ptr + 0x18) as u64);
        wr::<usize>(b70_ptr + 0x18, rd::<usize>(b50_ptr + 0x10));
        wr::<u8>(b70_ptr + 0x20, 0);
        wr::<usize>(b70_ptr + 0x28, a0_ptr);

        exe::call_va!(0x1_4212_c860, (rbx, f40_ptr, b70_ptr));
        exe::call_va!(0x1_4212_ea10, (param_1, b70_ptr, f40_ptr, rdi));

        let p60 = rd::<usize>(b50_ptr + 0x10);
        if p60 != 0 {
            exe::call_va!(0x1_4241_c190, (p60));
        }
        wr::<u64>(b50_ptr + 0x10, 0);
        wr::<u64>(b50_ptr + 0x18, 0);
        let p50 = rd::<usize>(b50_ptr);
        if p50 != 0 {
            exe::call_va!(0x1_4241_c190, (p50));
        }
    } else {
        let rax_src = exe::call_va!(0x1_4213_a860, (b170_ptr, rdi, f40_ptr) -> usize);
        wr::<[u8; 0x34]>(a0_ptr, rd::<[u8; 0x34]>(rax_src));

        let mut buf_1b0 = [0u64; 12];
        let b1b0_ptr = buf_1b0.as_mut_ptr() as usize;
        exe::call_va!(0x1_4213_a510, (b1b0_ptr, a0_ptr, rdi));

        let mut buf_f0 = [0u64; 16];
        let bf0_ptr = buf_f0.as_mut_ptr() as usize;
        exe::call_va!(0x1_4216_fe00, (bf0_ptr));

        let mut buf_d8: [u64; 2] = [rd::<u64>(rdi + 0x08), rd::<u64>(rdi + 0x10)];
        exe::call_va!(
            0x1_4216_f470,
            (
                bf0_ptr,
                b1b0_ptr,
                b1b0_ptr + 0x30,
                rd::<u64>(rdi),
                buf_d8.as_mut_ptr() as usize,
                rdi + 0xf8,
                1u8,
                1u8
            )
        );

        exe::call_va!(0x1_4213_b1f0, (b50_ptr, a0_ptr));
        let eax_len = rd::<u32>(b50_ptr + 0x08) as usize;
        let r8_ptr = rd::<usize>(b50_ptr);
        let cnt0 = (((eax_len * 12) as i64) / 12) as u64;
        wr::<u64>(b70_ptr, cnt0);
        wr::<usize>(b70_ptr + 0x08, r8_ptr);
        wr::<u64>(b70_ptr + 0x10, rd::<u32>(b50_ptr + 0x18) as u64);
        wr::<usize>(b70_ptr + 0x18, rd::<usize>(b50_ptr + 0x10));
        wr::<u8>(b70_ptr + 0x20, 0);
        wr::<usize>(b70_ptr + 0x28, a0_ptr);

        exe::call_va!(0x1_4213_9350, (b70_ptr, bf0_ptr));
        exe::call_va!(0x1_4213_9f10, (b70_ptr, bf0_ptr));
        exe::call_va!(0x1_4213_81f0, (b70_ptr, f40_ptr));
        exe::call_va!(0x1_4213_8050, (b70_ptr));
        exe::call_va!(0x1_4212_ea10, (param_1, b70_ptr, f40_ptr, rdi));

        let p60 = rd::<usize>(b50_ptr + 0x10);
        if p60 != 0 {
            exe::call_va!(0x1_4241_c190, (p60));
        }
        wr::<u64>(b50_ptr + 0x10, 0);
        wr::<u64>(b50_ptr + 0x18, 0);
        let p50 = rd::<usize>(b50_ptr);
        if p50 != 0 {
            exe::call_va!(0x1_4241_c190, (p50));
        }
        wr::<u64>(b50_ptr, 0);
        wr::<u64>(b50_ptr + 0x08, 0);

        exe::call_va!(0x1_401d_e830, (bf0_ptr + 0x20));
        exe::call_va!(0x1_401d_e830, (bf0_ptr + 0x10));
        exe::call_va!(0x1_401d_e830, (bf0_ptr));
    }

    param_1
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

pub unsafe fn diff_r_142065820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4397_fc50, 8);
    let _g2 = GlobalSave::new(0x1_43a0_82f8, 8);

    static mut JOB_NODES: [[u8; 0x40]; 4] = [[0; 0x40]; 4];
    static mut NEXT_NODE: usize = 0;
    static mut LOCAL_BUF: [u8; 0x100] = [0; 0x100];
    static mut ENQUEUED: Vec<(u64, u64)> = Vec::new();
    static mut SEM_RELEASED: u32 = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_handle: usize) {
        let idx = NEXT_NODE & 3;
        NEXT_NODE += 1;
        let node_ptr = core::ptr::addr_of_mut!(JOB_NODES[idx]) as usize;
        wr::<[u8; 0x40]>(node_ptr, [0; 0x40]);
        wr::<u16>(node_ptr + 0x06, (idx as u16 + 1) << 3);
        wr::<usize>(out_handle, node_ptr);
        wr::<u64>(out_handle + 0x08, idx as u64 + 1);
    }
    unsafe extern "win64" fn mock_b9b0(out_ptr: usize, _cnt: u64, elem_ptr: usize) {
        wr::<u64>(out_ptr, rd::<u64>(elem_ptr));
    }
    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(LOCAL_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_p: usize) {}
    unsafe extern "win64" fn mock_6460(_sched: usize, node: usize) {
        ENQUEUED.push((rd::<u64>(node + 0x20), rd::<u64>(node + 0x28)));
    }
    unsafe extern "win64" fn mock_sem(_h: usize, count: u32, _prev: usize) -> i32 {
        SEM_RELEASED += count;
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_b9b0, mock_b9b0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5580, mock_sem as *const ())?;

    let run = |f: F, max_workers: u64| -> (u32, u32, u32, u32, Vec<(u64, u64)>) {
        NEXT_NODE = 0;
        ENQUEUED.clear();
        SEM_RELEASED = 0;

        let mut existing_node1 = [0u8; 0x10];
        let en1p = existing_node1.as_mut_ptr() as usize;
        wr::<u32>(en1p, 0); // finished -> should be compacted out
        wr::<u16>(en1p + 0x06, 2 << 3);

        let mut existing_node2 = [0u8; 0x10];
        let en2p = existing_node2.as_mut_ptr() as usize;
        wr::<u32>(en2p, 1); // still running -> should be kept
        wr::<u16>(en2p + 0x06, 5 << 3);

        let mut active_jobs: [[u64; 2]; 4] = [
            [en1p as u64, 2],
            [en2p as u64, 5],
            [0, 0],
            [0, 0],
        ];
        let mut q1_items: [u64; 3] = [0x1111, 0x2222, 0x3333];
        let mut q2_items = [0u8; 0x90];
        wr::<u64>(q2_items.as_mut_ptr() as usize, 0x7777);

        let mut rsi_mgr = [0u8; 0x90];
        let rsip = rsi_mgr.as_mut_ptr() as usize;
        wr::<usize>(rsip, active_jobs.as_mut_ptr() as usize);
        wr::<u32>(rsip + 0x08, 2);
        wr::<u32>(rsip + 0x0c, 4);
        wr::<usize>(rsip + 0x28, q1_items.as_mut_ptr() as usize);
        wr::<u32>(rsip + 0x30, 1);
        wr::<usize>(rsip + 0x70, q2_items.as_mut_ptr() as usize);
        wr::<u32>(rsip + 0x78, 1);

        let mut sem_owner = [0u8; 0x1020];
        wr::<usize>(sem_owner.as_mut_ptr() as usize + 0x1008, 0xbeef);
        let mut worker_slot = [0u8; 0x100];
        let mut sched = [0u8; 0x80];
        let schedp = sched.as_mut_ptr() as usize;
        wr::<usize>(schedp, sem_owner.as_mut_ptr() as usize);
        wr::<usize>(schedp + 0x48, worker_slot.as_mut_ptr() as usize);

        let mut r13_ctx: [u64; 4] = [max_workers, 0, schedp as u64, 0];

        wr::<usize>(exe::ptr::<usize>(0x1_4397_fc50) as usize, rsip);
        wr::<usize>(
            exe::ptr::<usize>(0x1_43a0_82f8) as usize,
            r13_ctx.as_mut_ptr() as usize,
        );

        f(0);
        (
            rd::<u32>(rsip + 0x08),
            rd::<u32>(rsip + 0x30),
            rd::<u32>(rsip + 0x78),
            SEM_RELEASED,
            ENQUEUED.clone(),
        )
    };

    for max_workers in [2u64, 4u64] {
        let o = run(f_o, max_workers);
        let n = run(f_n, max_workers);
        crate::diff_eq!(o, n, "142065820 max_workers={}", max_workers);
    }
    Ok(2)
}

pub unsafe fn diff_r_142068040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, tag: u8| -> u8 {
        let mut buf = [0u8; 0xb0];
        buf[0xa0] = tag;
        f(buf.as_mut_ptr() as usize);
        buf[0xa0]
    };

    for tag in 0u8..=5u8 {
        let o = run(f_o, tag);
        let n = run(f_n, tag);
        crate::diff_eq!(o, n, "142068040 tag={}", tag);
    }
    Ok(6)
}

pub unsafe fn diff_r_142068ed0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32, i32, u16, u8, u32, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_e7f0(w: u32, h: u32, fmt: u8) -> u32 {
        w.wrapping_mul(h).wrapping_mul((fmt as u32) + 2)
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_e7f0, mock_e7f0 as *const ())?;

    let run = |f: F,
               w: i32,
               h: i32,
               d: i32,
               mips: u16,
               fmt: u8,
               flags: u32|
     -> ([u32; 14], u32, i32) {
        let mut desc = [0u32; 14];
        let mut out_row: u32 = 0xdead;
        let mut out_slice: i32 = 0x7ead;
        let dp = desc.as_mut_ptr() as usize;
        let ret = f(
            dp,
            w,
            h,
            d,
            mips,
            fmt,
            flags,
            (&mut out_row as *mut u32) as usize,
            (&mut out_slice as *mut i32) as usize,
        );
        assert_eq!(ret, dp);
        desc[1] = 0;
        desc[13] = 0;
        (desc, out_row, out_slice)
    };

    for (w, h, d, mips, fmt, flags) in [
        (32i32, 32i32, 8i32, 3u16, 1u8, 0u32),
        (64, 64, 4, 1, 2, 5),
        (128, 16, 4, 1, 2, 0x10),
        (128, 16, 4, 1, 0x1e, 0x20),
    ] {
        let o = run(f_o, w, h, d, mips, fmt, flags);
        let n = run(f_n, w, h, d, mips, fmt, flags);
        crate::diff_eq!(o, n, "142068ed0 fmt={} flags={:#x}", fmt, flags);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_14207d6c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4397_fc80, 8);

    static mut LAST_DESC: [u32; 14] = [0; 14];

    unsafe extern "win64" fn mock_9110(
        out: usize,
        w: i32,
        h: u32,
        d: u16,
        mips: u16,
        fmt: u8,
        flags: u32,
        _p8: usize,
        _p9: usize,
    ) -> usize {
        wr::<[u8; 0x38]>(out, [0; 0x38]);
        wr::<u32>(out, 3);
        wr::<i64>(out + 0x10, w as i64);
        wr::<u32>(out + 0x18, h);
        wr::<u16>(out + 0x1c, d);
        wr::<u16>(out + 0x1e, mips);
        wr::<u32>(out + 0x20, fmt as u32);
        wr::<u32>(out + 0x30, flags);
        out
    }

    unsafe extern "win64" fn mock_8ed0(
        out: usize,
        w: i32,
        h: i32,
        d: i32,
        mips: u16,
        fmt: u8,
        flags: u32,
        _p8: usize,
        _p9: usize,
    ) -> usize {
        wr::<[u8; 0x38]>(out, [0; 0x38]);
        wr::<u32>(out, 4);
        wr::<i64>(out + 0x10, w as i64);
        wr::<u32>(out + 0x18, h as u32);
        wr::<u16>(out + 0x1c, d as u16);
        wr::<u16>(out + 0x1e, mips);
        wr::<u32>(out + 0x20, fmt as u32);
        wr::<u32>(out + 0x30, flags);
        out
    }

    unsafe extern "win64" fn mock_get_alloc_info(
        _dev: usize,
        out_info: usize,
        _mask: u32,
        _cnt: u32,
        desc_ptr: usize,
    ) -> usize {
        let mut d = rd::<[u32; 14]>(desc_ptr);
        d[1] = 0;
        d[13] = 0;
        LAST_DESC = d;
        wr::<u64>(out_info, (d[0] as u64) * 0x10000 + (d[4] as u64));
        wr::<u64>(out_info + 0x08, 0x10000);
        out_info
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_9110, mock_9110 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_8ed0, mock_8ed0 as *const ())?;

    let run = |f: F, mode: u8| -> (u64, [u32; 14]) {
        LAST_DESC = [0; 14];
        let mut vtbl = [0u64; 32];
        vtbl[0xc8 / 8] = mock_get_alloc_info as *const () as usize as u64;
        let mut dev: usize = vtbl.as_mut_ptr() as usize;
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fc80) as usize,
            (&mut dev as *mut usize) as usize,
        );

        let mut entries = [0u8; 0x40];
        let ep = entries.as_mut_ptr() as usize;
        wr::<u16>(ep + 0x16, 2);
        wr::<u16>(ep + 0x18, 3);
        wr::<u16>(ep + 0x1a, 4);
        wr::<u8>(ep + 0x1c, 1);
        wr::<u16>(ep + 0x36, 5);
        wr::<u16>(ep + 0x38, 6);
        wr::<u16>(ep + 0x3a, 7);
        wr::<u8>(ep + 0x3c, 2);

        let mut p1 = [0u8; 0x90];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, ep);
        wr::<u32>(p1p + 0x68, 2);
        wr::<u8>(p1p + 0x6c, if mode == 0 { 3 } else { 1 });
        wr::<u32>(p1p + 0x70, 64);
        wr::<u32>(p1p + 0x74, 32);
        wr::<u32>(p1p + 0x78, 16);
        wr::<u8>(p1p + 0x80, if mode == 0 { 0 } else { mode - 1 });
        wr::<u8>(p1p + 0x81, if mode == 1 { 0x1e } else { 0x11 });
        wr::<u8>(p1p + 0x82, 4);

        let ret = f(p1p, 1);
        (ret, LAST_DESC)
    };

    for mode in [0u8, 1u8, 2u8, 3u8, 4u8] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14207d6c0 mode={}", mode);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_142080120(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4385_8020, 1);
    let _g2 = GlobalSave::new(0x1_43a3_4698, 8);
    let _g3 = GlobalSave::new(0x1_4397_fc80, 8);

    static mut ALLOC_OBJ: [u8; 0xc0] = [0; 0xc0];
    static mut RELEASE_COUNT: u32 = 0;
    static mut CREATE_DESC: [u32; 14] = [0; 14];

    unsafe extern "win64" fn mock_e7f0(w: u32, h: u32, _fmt: u8) -> u32 {
        w.wrapping_mul(h).wrapping_mul(4)
    }
    unsafe extern "win64" fn mock_ebd0(fmt: u8) -> u8 {
        if fmt == 0x1e { 1 } else { 0 }
    }
    unsafe extern "win64" fn mock_cc60(
        out_ptr: usize,
        _a2: usize,
        _res: usize,
        _a4: usize,
        _a5: usize,
        _a6: usize,
        _a7: usize,
        _a8: usize,
    ) -> usize {
        ALLOC_OBJ = [0; 0xc0];
        let p = core::ptr::addr_of_mut!(ALLOC_OBJ) as usize;
        wr::<usize>(out_ptr, p);
        out_ptr
    }
    unsafe extern "win64" fn mock_8040(_p: usize) {}
    unsafe extern "win64" fn mock_com_release(_this: usize) -> u32 {
        RELEASE_COUNT += 1;
        0
    }
    unsafe extern "win64" fn mock_create_reserved(
        _dev: usize,
        desc_ptr: usize,
        _layout: u32,
        _clear: usize,
        _iid: usize,
        out_res: usize,
    ) -> u32 {
        CREATE_DESC = rd::<[u32; 14]>(desc_ptr);
        wr::<usize>(out_res, 0);
        0
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_e7f0, mock_e7f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4204_ebd0, mock_ebd0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_cc60, mock_cc60 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4206_8040, mock_8040 as *const ())?;

    let run = |f: F, kind: u8, w: u32| -> (bool, [u32; 8], u8, u8, [u32; 14]) {
        ALLOC_OBJ = [0; 0xc0];
        RELEASE_COUNT = 0;
        CREATE_DESC = [0; 14];

        wr::<u8>(exe::ptr::<u8>(0x1_4385_8020) as usize, 1);
        let mut caps = [0u8; 0x30];
        caps[0x20] = 1;
        caps[0x21] = 1;
        wr::<usize>(
            exe::ptr::<usize>(0x1_43a3_4698) as usize,
            caps.as_mut_ptr() as usize,
        );

        let mut dev_vtbl = [0u64; 32];
        dev_vtbl[0x10 / 8] = mock_com_release as *const () as usize as u64;
        dev_vtbl[0xf0 / 8] = mock_create_reserved as *const () as usize as u64;
        let mut dev_obj: usize = dev_vtbl.as_mut_ptr() as usize;
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fc80) as usize,
            (&mut dev_obj as *mut usize) as usize,
        );

        let mut p2 = [0u8; 0x20];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p, w);
        wr::<u32>(p2p + 0x04, w);
        wr::<u32>(p2p + 0x08, 1);
        wr::<u8>(p2p + 0x10, kind);
        wr::<u8>(p2p + 0x11, 0x1e);
        wr::<u8>(p2p + 0x12, 2);

        let mut out_ptr: usize = 0xdead;
        let ret = f((&mut out_ptr as *mut usize) as usize, p2p, 0);
        assert_eq!(ret, (&mut out_ptr as *mut usize) as usize);
        let ap = core::ptr::addr_of!(ALLOC_OBJ) as usize;
        (
            out_ptr != 0,
            rd::<[u32; 8]>(ap),
            rd::<u8>(ap + 0x50),
            rd::<u8>(ap + 0x51),
            CREATE_DESC,
        )
    };

    for (kind, w) in [(0u8, 16u32), (0u8, 256u32), (1u8, 64u32), (3u8, 64u32)] {
        let o = run(f_o, kind, w);
        let n = run(f_n, kind, w);
        crate::diff_eq!(o, n, "142080120 kind={} w={}", kind, w);
    }
    Ok(4)
}

pub unsafe fn diff_std_1420a46f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODES: [[u8; 0x18]; 8] = [[0; 0x18]; 8];
    static mut NODE_IDX: usize = 0;

    unsafe extern "win64" fn mock_3310(iter: usize) {
        let c0 = rd::<usize>(iter);
        let idx = rd::<u16>(iter + 0x08);
        let end = rd::<u16>(c0 + 0x02);
        if idx < end {
            wr::<u16>(iter + 0x0a, idx);
            wr::<u8>(iter + 0x0c, 1);
        }
    }
    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        let i = NODE_IDX & 7;
        NODE_IDX += 1;
        let p = core::ptr::addr_of_mut!(NODES[i]) as usize;
        wr::<[u8; 0x18]>(p, [0; 0x18]);
        p
    }
    unsafe extern "win64" fn mock_bfc0(_map: usize, _buckets: u64) {}

    let _m1 = crate::difftest::Mock::new(0x1_420a_3310, mock_3310 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_bfc0, mock_bfc0 as *const ())?;

    let run = |f: F, range_arg: u32| -> (u32, u32, [u16; 4], [u16; 4], u64) {
        NODE_IDX = 0;
        let mut mesh_items = [0u8; 0x80];
        let mp = mesh_items.as_mut_ptr() as usize;
        wr::<u16>(mp + 0x3a, 0);
        wr::<u16>(mp + 0x3c, 2);

        let mut edges: [u16; 8] = [0x101, 0, 0, 0, 0x202, 0, 0, 0];
        let mut p1_0: [usize; 3] = [mp, 0, edges.as_mut_ptr() as usize];

        let mut sentinel = [0usize; 3];
        let sent_p = sentinel.as_mut_ptr() as usize;
        wr::<usize>(sent_p, sent_p);
        wr::<usize>(sent_p + 0x08, sent_p);

        let mut buckets = [sent_p; 16];
        let mut vec40: [u16; 4] = [0; 4];

        let mut rsi_map = [0u8; 0x60];
        let rsip = rsi_map.as_mut_ptr() as usize;
        wr::<f32>(rsip, 1.0);
        wr::<usize>(rsip + 0x08, sent_p);
        wr::<u64>(rsip + 0x10, 0);
        wr::<usize>(rsip + 0x18, buckets.as_mut_ptr() as usize);
        wr::<u64>(rsip + 0x30, 7);
        wr::<u64>(rsip + 0x38, 8);
        wr::<usize>(rsip + 0x40, vec40.as_mut_ptr() as usize);
        wr::<u32>(rsip + 0x48, 0);
        wr::<u32>(rsip + 0x4c, 4);

        let mut p1: [usize; 2] = [p1_0.as_mut_ptr() as usize, rsip];
        let mut out_u32: u32 = 0;
        let ret = f(
            p1.as_mut_ptr() as usize,
            (&mut out_u32 as *mut u32) as usize,
            7,
            range_arg,
        );
        assert_eq!(ret, (&mut out_u32 as *mut u32) as usize);
        (
            out_u32,
            rd::<u32>(rsip + 0x48),
            vec40,
            [edges[1], edges[5], 0, 0],
            rd::<u64>(rsip + 0x10),
        )
    };

    for range_arg in [0u32, 0x0001_0000u32] {
        let o = run(f_o, range_arg);
        let n = run(f_n, range_arg);
        crate::diff_eq!(o, n, "1420a46f0 range={:#x}", range_arg);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420a4b10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut STACK_BUF: [u16; 16] = [0; 16];
    static mut LOG_C200: Vec<(u64, usize, u8)> = Vec::new();

    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(STACK_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_p: usize) {}
    unsafe extern "win64" fn mock_46f0(
        _p1: usize,
        out_ptr: usize,
        idx: u16,
        _p4: u32,
    ) -> usize {
        let range = if idx == 0 { 0x0002_0000u32 } else { 0u32 };
        wr::<u32>(out_ptr, range);
        out_ptr
    }
    unsafe extern "win64" fn mock_c200(
        out_ret: usize,
        arg2: usize,
        _arg3: usize,
        vec_ptr: usize,
    ) -> usize {
        let vctx = rd::<usize>(arg2);
        let cnt = rd::<u64>(vctx);
        let base = rd::<usize>(vctx + 0x08);
        let it = rd::<usize>(arg2 + 0x08);
        let has_last = rd::<u8>(arg2 + 0x18);
        LOG_C200.push((cnt, it.wrapping_sub(base), has_last));
        wr::<usize>(out_ret, vec_ptr);
        out_ret
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420a_46f0, mock_46f0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411e_c200, mock_c200 as *const ())?;

    let run = |f: F, init_cap: u32, hit_second: u8| -> ([u32; 2], Vec<(u64, usize, u8)>) {
        STACK_BUF = [0; 16];
        LOG_C200.clear();

        let mut mesh_nodes = [0u8; 0x80];
        let mnp = mesh_nodes.as_mut_ptr() as usize;
        wr::<u16>(mnp + 0x22, 0);
        wr::<u16>(mnp + 0x40 + 0x22, 1);

        let mut p1_0 = [0u8; 0x10];
        wr::<usize>(p1_0.as_mut_ptr() as usize, mnp);
        wr::<u32>(p1_0.as_mut_ptr() as usize + 0x08, init_cap);

        let mut arr40: [u16; 4] = [0, 1, 0, 0];
        let mut arr50: [u32; 2] = [0; 2];
        let mut p1_8 = [0u8; 0x60];
        wr::<usize>(p1_8.as_mut_ptr() as usize + 0x40, arr40.as_mut_ptr() as usize);
        wr::<usize>(p1_8.as_mut_ptr() as usize + 0x50, arr50.as_mut_ptr() as usize);

        let mut tab_entries = [0u8; 0x60];
        tab_entries[0x20] = 0;
        tab_entries[0x30 + 0x20] = hit_second;
        let mut p1_10 = [0u8; 0x20];
        wr::<usize>(
            p1_10.as_mut_ptr() as usize + 0x10,
            tab_entries.as_mut_ptr() as usize,
        );

        let mut p1: [usize; 3] = [
            p1_0.as_mut_ptr() as usize,
            p1_8.as_mut_ptr() as usize,
            p1_10.as_mut_ptr() as usize,
        ];
        f(p1.as_mut_ptr() as usize, 0, 0x0001_0000);
        (arr50, LOG_C200.clone())
    };

    for (init_cap, hit) in [(0u32, 0u8), (4u32, 1u8)] {
        let o = run(f_o, init_cap, hit);
        let n = run(f_n, init_cap, hit);
        crate::diff_eq!(o, n, "1420a4b10 cap={} hit={}", init_cap, hit);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14212c3e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4386_3d18, 4);
    let _g2 = GlobalSave::new(0x1_4386_3b60, 4);
    let _g3 = GlobalSave::new(0x1_4386_4170, 4);
    let _g4 = GlobalSave::new(0x1_4386_3e10, 4);
    let _g5 = GlobalSave::new(0x1_4386_4078, 4);
    let _g6 = GlobalSave::new(0x1_4386_3cc8, 1);
    let _g7 = GlobalSave::new(0x1_4386_3d00, 1);
    let _g8 = GlobalSave::new(0x1_4386_4190, 1);
    let _g9 = GlobalSave::new(0x1_4386_3e60, 1);
    let _g10 = GlobalSave::new(0x1_4386_3ba0, 1);

    static mut LOG_STEPS: Vec<u32> = Vec::new();
    static mut LOG_FLOATS: [u32; 3] = [0; 3];
    static mut LOG_CNT0: u64 = 0;
    static mut LOG_F470_AABB: [u8; 0x30] = [0; 0x30];
    static mut LOG_F470_XFORM: [u8; 0x28] = [0; 0x28];
    static mut LOG_F470_DELTA: isize = 0;
    static mut LOG_F470_ARGS: (u64, [u64; 2], u8, u8) = (0, [0; 2], 0, 0);

    unsafe extern "win64" fn mock_b7a0() {
        LOG_STEPS.push(1);
    }
    unsafe extern "win64" fn mock_dc30() -> usize {
        LOG_STEPS.push(2);
        0x5555
    }
    unsafe extern "win64" fn mock_e240(_rbx: usize, f40: usize) {
        LOG_STEPS.push(3);
        LOG_FLOATS = rd::<[u32; 3]>(f40);
    }
    unsafe extern "win64" fn mock_cc20(_rbx: usize, _f40: usize, _rdi: usize) {
        LOG_STEPS.push(4);
    }
    unsafe extern "win64" fn mock_a660(out: usize, _rdi: usize) -> usize {
        LOG_STEPS.push(5);
        wr::<[u8; 0x34]>(out, [0x11; 0x34]);
        out
    }
    unsafe extern "win64" fn mock_a860(out: usize, _rdi: usize, _f40: usize) -> usize {
        LOG_STEPS.push(6);
        wr::<[u8; 0x34]>(out, [0x22; 0x34]);
        out
    }
    unsafe extern "win64" fn mock_a510(out: usize, _a0: usize, _rdi: usize) -> usize {
        LOG_STEPS.push(7);
        // Real `unknown_14213a510` writes 0x58 bytes: 0x30-byte AABB + 0x28-byte Transform.
        for i in 0..0x58usize {
            wr::<u8>(out + i, (i as u8).wrapping_add(0x41));
        }
        out
    }
    unsafe extern "win64" fn mock_fe00(_bf0: usize) {
        LOG_STEPS.push(8);
    }
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn mock_f470(
        _bf0: usize,
        b1b0: usize,
        b1e0: usize,
        r0: u64,
        bd8: usize,
        _rf8: usize,
        f1: u8,
        f2: u8,
    ) {
        LOG_STEPS.push(9);
        LOG_F470_DELTA = b1e0.wrapping_sub(b1b0) as isize;
        LOG_F470_AABB = rd::<[u8; 0x30]>(b1b0);
        LOG_F470_XFORM = rd::<[u8; 0x28]>(b1e0);
        LOG_F470_ARGS = (r0, rd::<[u64; 2]>(bd8), f1, f2);
    }
    unsafe extern "win64" fn mock_b1f0(b50: usize, _a0: usize) {
        LOG_STEPS.push(10);
        wr::<u64>(b50, 0x1000);
        wr::<u32>(b50 + 0x08, 5);
        wr::<u64>(b50 + 0x10, 0x2000);
        wr::<u32>(b50 + 0x18, 9);
    }
    unsafe extern "win64" fn mock_c860(_rbx: usize, _f40: usize, b70: usize) {
        LOG_STEPS.push(11);
        LOG_CNT0 = rd::<u64>(b70);
    }
    unsafe extern "win64" fn mock_9350(b70: usize, _bf0: usize) {
        LOG_STEPS.push(12);
        LOG_CNT0 = rd::<u64>(b70);
    }
    unsafe extern "win64" fn mock_9f10(_b70: usize, _bf0: usize) {
        LOG_STEPS.push(13);
    }
    unsafe extern "win64" fn mock_81f0(_b70: usize, _f40: usize) {
        LOG_STEPS.push(14);
    }
    unsafe extern "win64" fn mock_8050(_b70: usize) {
        LOG_STEPS.push(15);
    }
    unsafe extern "win64" fn mock_ea10(_p1: usize, _b70: usize, _f40: usize, _rdi: usize) {
        LOG_STEPS.push(16);
    }
    unsafe extern "win64" fn mock_c190(_p: usize) {
        LOG_STEPS.push(17);
    }
    unsafe extern "win64" fn mock_e830(_p: usize) {
        LOG_STEPS.push(18);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4213_b7a0, mock_b7a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4123_dc30, mock_dc30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4212_e240, mock_e240 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4212_cc20, mock_cc20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4213_a660, mock_a660 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4213_a860, mock_a860 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4213_a510, mock_a510 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4216_fe00, mock_fe00 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4216_f470, mock_f470 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4213_b1f0, mock_b1f0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4212_c860, mock_c860 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4213_9350, mock_9350 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4213_9f10, mock_9f10 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4213_81f0, mock_81f0 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4213_8050, mock_8050 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4212_ea10, mock_ea10 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, d00: u8| {
        LOG_STEPS.clear();
        LOG_FLOATS = [0; 3];
        LOG_CNT0 = 0;
        LOG_F470_AABB = [0; 0x30];
        LOG_F470_XFORM = [0; 0x28];
        LOG_F470_DELTA = 0;
        LOG_F470_ARGS = (0, [0; 2], 0, 0);

        wr::<f32>(exe::ptr::<f32>(0x1_4386_3d18) as usize, -1.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3b60) as usize, 2.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4170) as usize, 3.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_3e10) as usize, -1.0);
        wr::<f32>(exe::ptr::<f32>(0x1_4386_4078) as usize, -1.0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3cc8) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3d00) as usize, d00);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_4190) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3e60) as usize, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4386_3ba0) as usize, 0);

        let mut rdi_obj = [0u8; 0x120];
        let rdip = rdi_obj.as_mut_ptr() as usize;
        wr::<u64>(rdip, 0x1122_3344_5566_7788);
        wr::<u64>(rdip + 0x08, 0xaaaa_bbbb_cccc_dddd);
        wr::<u64>(rdip + 0x10, 0x1234_5678_9abc_def0);
        wr::<f32>(rdip + 0x40, 10.0);
        wr::<f32>(rdip + 0x44, 20.0);
        wr::<f32>(rdip + 0x48, 30.0);

        let mut p2_cell: usize = 0;
        let mut p3_cell: usize = rdip;
        let ret = f(
            0x1234,
            (&mut p2_cell as *mut usize) as usize,
            (&mut p3_cell as *mut usize) as usize,
        );
        assert_eq!(ret, 0x1234);
        (
            p2_cell,
            LOG_STEPS.clone(),
            LOG_FLOATS,
            LOG_CNT0,
            LOG_F470_DELTA,
            LOG_F470_AABB,
            LOG_F470_XFORM,
            LOG_F470_ARGS,
        )
    };

    for d00 in [0u8, 1u8] {
        let o = run(f_o, d00);
        let n = run(f_n, d00);
        crate::diff_eq!(o, n, "14212c3e0 d00={}", d00);
    }
    Ok(2)
}
