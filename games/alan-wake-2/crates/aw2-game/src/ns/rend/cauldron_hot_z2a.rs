//! Decompiled `rend` hot functions shard Z2A (`0x1411eef20..0x1412eb9d0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn move_elem_50(src: usize, dst: usize) {
    wr::<u64>(dst, rd::<u64>(src));
    wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08));
    wr::<u32>(dst + 0x10, rd::<u32>(src + 0x10));
    wr::<u32>(dst + 0x14, rd::<u32>(src + 0x14));
    wr::<u64>(src + 0x08, 0);
    wr::<u64>(src + 0x10, 0);
    wr::<u64>(dst + 0x18, rd::<u64>(src + 0x18));
    wr::<u64>(dst + 0x20, rd::<u64>(src + 0x20));
    wr::<u32>(dst + 0x28, rd::<u32>(src + 0x28));
    wr::<u32>(dst + 0x2c, rd::<u32>(src + 0x2c));
    wr::<u64>(src + 0x20, 0);
    wr::<u64>(src + 0x28, 0);
    wr::<u64>(dst + 0x30, rd::<u64>(src + 0x30));
    wr::<u64>(dst + 0x38, rd::<u64>(src + 0x38));
    wr::<u32>(dst + 0x40, rd::<u32>(src + 0x40));
    wr::<u32>(dst + 0x44, rd::<u32>(src + 0x44));
    wr::<u64>(src + 0x38, 0);
    wr::<u64>(src + 0x40, 0);
    wr::<u16>(dst + 0x48, rd::<u16>(src + 0x48));
}

#[inline(always)]
unsafe fn arena_try_grow_in_place(
    arena: usize,
    old_data: usize,
    old_bytes: u64,
    new_bytes: u64,
) -> bool {
    let blocks_ptr = rd::<usize>(arena);
    let blocks_len = rd::<u32>(arena + 0x08) as usize;
    let blocks_end = blocks_ptr.wrapping_add(blocks_len.wrapping_mul(8));
    if blocks_ptr == blocks_end {
        return false;
    }
    let cur_off = rd::<u64>(arena + 0x10);
    let last_blk = rd::<u64>(blocks_end.wrapping_sub(8));
    let base_off = cur_off.wrapping_sub(old_bytes);
    if last_blk.wrapping_add(base_off) != old_data as u64 {
        return false;
    }
    let new_off = new_bytes.wrapping_add(base_off);
    if new_off > rd::<u64>(arena + 0x18) {
        return false;
    }
    wr::<u64>(arena + 0x10, new_off);
    true
}

/// `sparsehash_1411eef20`
/// Original: 0x1411eef20 (731 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1411eef20(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: u8,
) {
    let mut r11 = rd::<usize>(param_2);
    let mut r9 = rd::<usize>(param_1);
    let mut rbp = param_3;
    let mut span = (r11.wrapping_sub(r9) & !1usize) as i64;

    while span > 0x40 {
        if rbp <= 0 {
            let count = (r11.wrapping_sub(r9) as i64) >> 1;
            let mut r10 = count >> 1;
            if r10 > 0 {
                let rbx = (count - 1) >> 1;
                loop {
                    let ax = rd::<u16>(r9.wrapping_add((r10 as usize) * 2).wrapping_sub(2));
                    r10 -= 1;
                    let mut r8 = r10;
                    let mut rdx = r10;
                    if r10 < rbx {
                        loop {
                            rdx = rdx.wrapping_mul(2);
                            let cx = rd::<u16>(r9.wrapping_add((rdx as usize) * 2 + 2));
                            let right = rd::<u16>(r9.wrapping_add((rdx as usize) * 2 + 4));
                            let step = if right < cx { 1i64 } else { 2i64 };
                            rdx = rdx.wrapping_add(step);
                            let child_val = rd::<u16>(r9.wrapping_add((rdx as usize) * 2));
                            wr::<u16>(r9.wrapping_add((r8 as usize) * 2), child_val);
                            r8 = rdx;
                            if rdx >= rbx {
                                break;
                            }
                        }
                    }
                    if r8 == rbx && (count & 1) == 0 {
                        let last_val = rd::<u16>(r9.wrapping_add((count as usize) * 2 - 2));
                        wr::<u16>(r9.wrapping_add((r8 as usize) * 2), last_val);
                        r8 = count - 1;
                    }
                    let mut rcx = (r8 - 1) >> 1;
                    while r10 < r8 {
                        let dx = rd::<u16>(r9.wrapping_add((rcx as usize) * 2));
                        if dx >= ax {
                            break;
                        }
                        wr::<u16>(r9.wrapping_add((r8 as usize) * 2), dx);
                        r8 = rcx;
                        rcx = (rcx - 1) >> 1;
                    }
                    wr::<u16>(r9.wrapping_add((r8 as usize) * 2), ax);
                    if r10 <= 0 {
                        break;
                    }
                }
            }

            let mut rcx = rd::<usize>(param_2);
            let rdi = rd::<usize>(param_1);
            if ((rcx.wrapping_sub(rdi) & !1usize) as i64) >= 4 {
                let mut val_buf: u64 = 0;
                let mut ptr_buf: usize;
                loop {
                    let rbx = rcx.wrapping_sub(2);
                    if ((rcx.wrapping_sub(rdi) & !1usize) as i64) >= 4 {
                        let ax = rd::<u16>(rbx);
                        val_buf = (val_buf & !0xffffu64) | (ax as u64);
                        wr::<u16>(rbx, rd::<u16>(rdi));
                        ptr_buf = rdi;
                        let len = (rbx.wrapping_sub(rdi) as i64) >> 1;
                        exe::call_va!(
                            0x1_40fa_6780,
                            (
                                core::ptr::addr_of_mut!(ptr_buf) as usize,
                                0i64,
                                len,
                                core::ptr::addr_of_mut!(val_buf) as usize,
                                param_4
                            )
                        );
                    }
                    rcx = rbx;
                    if ((rbx.wrapping_sub(rdi) & !1usize) as i64) < 4 {
                        break;
                    }
                }
            }
            return;
        }

        let mut local_78 = r9;
        let mut local_70 = r11;
        let mut split = [0usize; 2];
        let _ = exe::call_va!(
            0x1_411e_f200,
            (
                split.as_mut_ptr() as usize,
                core::ptr::addr_of_mut!(local_78) as usize,
                core::ptr::addr_of_mut!(local_70) as usize,
                param_4
            ) -> usize
        );
        let r8_end = rd::<usize>(param_2);
        let rdx_beg = rd::<usize>(param_1);
        let rbx_mid0 = split[0];
        let rdi_mid1 = split[1];
        rbp = (rbp >> 1).wrapping_add(rbp >> 2);
        let right_span = (r8_end.wrapping_sub(rdi_mid1) & !1usize) as i64;
        let left_span = (rbx_mid0.wrapping_sub(rdx_beg) & !1usize) as i64;
        if left_span < right_span {
            local_78 = rdx_beg;
            local_70 = rbx_mid0;
            sparsehash_1411eef20(
                core::ptr::addr_of_mut!(local_78) as usize,
                core::ptr::addr_of_mut!(local_70) as usize,
                rbp,
                param_4,
            );
            wr::<usize>(param_1, rdi_mid1);
        } else {
            local_78 = rdi_mid1;
            local_70 = r8_end;
            sparsehash_1411eef20(
                core::ptr::addr_of_mut!(local_78) as usize,
                core::ptr::addr_of_mut!(local_70) as usize,
                rbp,
                param_4,
            );
            wr::<usize>(param_2, rbx_mid0);
        }
        r11 = rd::<usize>(param_2);
        r9 = rd::<usize>(param_1);
        span = (r11.wrapping_sub(r9) & !1usize) as i64;
    }

    if r9 == r11 {
        return;
    }
    let mut rcx = r9.wrapping_add(2);
    if rcx == r11 {
        return;
    }
    loop {
        let r8w = rd::<u16>(rcx);
        let mut rdx = rcx;
        if r8w < rd::<u16>(r9) {
            if r9 != rcx {
                let mut rax = rcx;
                let mut r10 = rcx.wrapping_add(2);
                loop {
                    let dx = rd::<u16>(rax.wrapping_sub(2));
                    r10 = r10.wrapping_sub(2);
                    rax = rax.wrapping_sub(2);
                    wr::<u16>(r10, dx);
                    if rax == r9 {
                        break;
                    }
                }
            }
            wr::<u16>(r9, r8w);
        } else {
            let mut r10w = rd::<u16>(rcx.wrapping_sub(2));
            let mut rax = rcx.wrapping_sub(2);
            while r8w < r10w {
                wr::<u16>(rdx, r10w);
                rdx = rax;
                r10w = rd::<u16>(rax.wrapping_sub(2));
                rax = rax.wrapping_sub(2);
            }
            wr::<u16>(rdx, r8w);
        }
        rcx = rcx.wrapping_add(2);
        if rcx == r11 {
            break;
        }
    }
}

/// `sparsehash_1411ef200`
/// Original: 0x1411ef200 (668 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1411ef200(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: u8,
) -> usize {
    let end_init = rd::<usize>(param_3);
    let beg_init = rd::<usize>(param_2);
    let r9_quarter = (end_init.wrapping_sub(beg_init) as isize) >> 2;
    let rdi_last = end_init.wrapping_sub(2);
    let r14_len = (rdi_last.wrapping_sub(beg_init) as isize) >> 1;
    let mut rbx = beg_init.wrapping_add((r9_quarter * 2) as usize);

    if r14_len > 0x28 {
        let step = (((r14_len + 1) >> 2) & !1isize) as usize;
        let r15 = beg_init.wrapping_add(step);
        let two_step = step.wrapping_mul(2);

        let mut a = beg_init;
        let mut b = r15;
        let mut c = beg_init.wrapping_add(two_step);
        exe::call_va!(
            0x1_40fa_69b0,
            (
                core::ptr::addr_of_mut!(a) as usize,
                core::ptr::addr_of_mut!(b) as usize,
                core::ptr::addr_of_mut!(c) as usize
            )
        );

        a = rbx.wrapping_sub(step);
        b = rbx;
        c = rbx.wrapping_add(step);
        exe::call_va!(
            0x1_40fa_69b0,
            (
                core::ptr::addr_of_mut!(a) as usize,
                core::ptr::addr_of_mut!(b) as usize,
                core::ptr::addr_of_mut!(c) as usize
            )
        );

        let rdi_minus_step = rdi_last.wrapping_sub(step);
        a = rdi_last.wrapping_sub(two_step);
        b = rdi_minus_step;
        c = rdi_last;
        exe::call_va!(
            0x1_40fa_69b0,
            (
                core::ptr::addr_of_mut!(a) as usize,
                core::ptr::addr_of_mut!(b) as usize,
                core::ptr::addr_of_mut!(c) as usize
            )
        );

        a = r15;
        b = rbx;
        c = rdi_minus_step;
        exe::call_va!(
            0x1_40fa_69b0,
            (
                core::ptr::addr_of_mut!(a) as usize,
                core::ptr::addr_of_mut!(b) as usize,
                core::ptr::addr_of_mut!(c) as usize
            )
        );
    } else {
        let mut a = beg_init;
        let mut b = rbx;
        let mut c = rdi_last;
        exe::call_va!(
            0x1_40fa_69b0,
            (
                core::ptr::addr_of_mut!(a) as usize,
                core::ptr::addr_of_mut!(b) as usize,
                core::ptr::addr_of_mut!(c) as usize
            )
        );
    }

    let r9_beg = rd::<usize>(param_2);
    let mut rcx = rbx.wrapping_add(2);
    if r9_beg < rbx {
        let mut ax = rd::<u16>(rbx);
        loop {
            let dx = ax;
            let r8 = rbx.wrapping_sub(2);
            ax = rd::<u16>(r8);
            if ax != dx {
                break;
            }
            rbx = r8;
            if r9_beg >= r8 {
                break;
            }
        }
    }

    let r8_end = rd::<usize>(param_3);
    if rcx < r8_end {
        let dx = rd::<u16>(rbx);
        while rcx < r8_end && rd::<u16>(rcx) == dx {
            rcx = rcx.wrapping_add(2);
        }
    }

    let mut rdx = rcx;
    let mut r8 = rbx;
    loop {
        while rdx < rd::<usize>(param_3) {
            let r9w = rd::<u16>(rdx);
            let pivot = rd::<u16>(rbx);
            if pivot < r9w {
                rdx = rdx.wrapping_add(2);
            } else if pivot > r9w {
                break;
            } else {
                if rcx != rdx {
                    let ax = rd::<u16>(rcx);
                    wr::<u16>(rcx, r9w);
                    wr::<u16>(rdx, ax);
                }
                rcx = rcx.wrapping_add(2);
                rdx = rdx.wrapping_add(2);
            }
        }

        let mut rax_beg = rd::<usize>(param_2);
        if rax_beg < r8 {
            let mut r9 = r8.wrapping_sub(2);
            loop {
                let r10w = rd::<u16>(r9);
                let pivot = rd::<u16>(rbx);
                if r10w > pivot {
                    break;
                }
                if r10w == pivot {
                    rbx = rbx.wrapping_sub(2);
                    if rbx != r9 {
                        let ax = rd::<u16>(rbx);
                        wr::<u16>(rbx, r10w);
                        wr::<u16>(r9, ax);
                    }
                }
                rax_beg = rd::<usize>(param_2);
                r8 = r8.wrapping_sub(2);
                r9 = r9.wrapping_sub(2);
                if rax_beg >= r8 {
                    break;
                }
            }
        }

        let r9_end = rd::<usize>(param_3);
        if r8 == rax_beg {
            if rdx == r9_end {
                wr::<usize>(param_1, rbx);
                wr::<usize>(param_1 + 0x08, rcx);
                return param_1;
            }
            if rcx != rdx {
                let r9w = rd::<u16>(rcx);
                let ax = rd::<u16>(rbx);
                wr::<u16>(rbx, r9w);
                wr::<u16>(rcx, ax);
            }
            let r9w = rd::<u16>(rdx);
            rcx = rcx.wrapping_add(2);
            let ax = rd::<u16>(rbx);
            wr::<u16>(rbx, r9w);
            rbx = rbx.wrapping_add(2);
            wr::<u16>(rdx, ax);
            rdx = rdx.wrapping_add(2);
        } else {
            r8 = r8.wrapping_sub(2);
            if rdx == r9_end {
                rbx = rbx.wrapping_sub(2);
                if r8 != rbx {
                    let r9w = rd::<u16>(rbx);
                    let ax = rd::<u16>(r8);
                    wr::<u16>(r8, r9w);
                    wr::<u16>(rbx, ax);
                }
                let r9w = rd::<u16>(rcx.wrapping_sub(2));
                rcx = rcx.wrapping_sub(2);
                let ax = rd::<u16>(rbx);
                wr::<u16>(rbx, r9w);
                wr::<u16>(rcx, ax);
            } else {
                let ax = rd::<u16>(rdx);
                let r9w = rd::<u16>(r8);
                wr::<u16>(rdx, r9w);
                rdx = rdx.wrapping_add(2);
                wr::<u16>(r8, ax);
            }
        }
    }
}

/// `sparsehash_1411f03b0`
/// Original: 0x1411f03b0 (322 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1411f03b0(param_1: usize, param_2: usize, param_3: u64) {
    let lo = param_3 as u32;
    let hi = (param_3 >> 32) as u32;
    let count_entries = hi.wrapping_sub(lo) as usize;
    let mut rbp = rd::<usize>(param_1).wrapping_add((lo as usize).wrapping_mul(0x10));
    let rsi = rbp.wrapping_add(count_entries.wrapping_mul(0x10));
    if rbp == rsi {
        return;
    }

    let key_cnt = rd::<usize>(param_2);
    rbp = rbp.wrapping_add(0x08);
    let keys_beg = rd::<usize>(param_2 + 0x08);
    let keys_end = keys_beg.wrapping_add(key_cnt.wrapping_mul(2));

    loop {
        let mut rbx = keys_beg;
        while rbx != keys_end {
            let r10 = rd::<u32>(rbp) as usize;
            let mut rax: usize = 0;
            let r11w = rd::<u16>(rbx);
            let mut rcx = r10;
            let r9 = rd::<usize>(rbp.wrapping_sub(0x08));
            let r14 = r10.wrapping_mul(2);
            if r10 != 0 {
                loop {
                    let r8 = rcx >> 1;
                    let rdx = rax.wrapping_sub(r8).wrapping_add(rcx);
                    let mid = r8.wrapping_add(rax);
                    let elem = rd::<u16>(r9.wrapping_add(mid.wrapping_mul(2)));
                    rcx = r8;
                    if r11w >= elem {
                        rax = rdx;
                    }
                    if r8 == 0 {
                        break;
                    }
                }
            }
            rax = rax.wrapping_sub(1);
            let mut dummy: u16 = !r11w;
            let edx: u32 = if rax < r10 { rax as u32 } else { 0 };
            let rcx_ptr = if r10 != 0 {
                r9
            } else {
                core::ptr::addr_of_mut!(dummy) as usize
            };
            let idx_off = ((edx as i32) as isize as usize).wrapping_mul(2);
            let r8d: i32 = if r11w == rd::<u16>(rcx_ptr.wrapping_add(idx_off)) {
                edx as i32
            } else {
                -1
            };
            if r8d != -1 {
                let pos = ((r8d as isize) as usize).wrapping_mul(2);
                let dst = r9.wrapping_add(pos);
                let src = dst.wrapping_add(2);
                let move_len = r14.wrapping_add(r9).wrapping_sub(src);
                let _ = exe::call_va!(0x1_4183_acc8, (dst, src, move_len) -> usize);
                wr::<u32>(rbp, rd::<u32>(rbp).wrapping_sub(1));
            }
            rbx = rbx.wrapping_add(2);
        }
        rbp = rbp.wrapping_add(0x10);
        if rbp.wrapping_sub(0x08) == rsi {
            break;
        }
    }
}

/// `unknown_1411f12b0`
/// Original: 0x1411f12b0 (142 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f12b0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u64,
    param_5: u64,
) -> usize {
    if param_2 == 0 {
        return exe::call_va!(0x1_411f_11e0, (param_1, param_4, param_5) -> usize);
    }
    let blocks_ptr = rd::<usize>(param_1);
    let blocks_len = rd::<u32>(param_1 + 0x08) as usize;
    let blocks_end = blocks_ptr.wrapping_add(blocks_len.wrapping_mul(8));
    if blocks_ptr != blocks_end {
        let last_blk = rd::<u64>(blocks_end.wrapping_sub(8));
        let align_m1 = param_5.wrapping_sub(1);
        let mask = !align_m1;
        let aligned_old = align_m1.wrapping_add(param_3) & mask;
        let cur_off = rd::<u64>(param_1 + 0x10);
        let base_off = cur_off.wrapping_sub(aligned_old);
        if last_blk.wrapping_add(base_off) == param_2 as u64 {
            let aligned_new = align_m1.wrapping_add(param_4) & mask;
            let new_off = aligned_new.wrapping_add(base_off);
            if new_off <= rd::<u64>(param_1 + 0x18) {
                wr::<u64>(param_1 + 0x10, new_off);
                return param_2;
            }
        }
    }
    0
}

/// `unknown_1411f13d0`
/// Original: 0x1411f13d0 (129 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f13d0(param_1: usize) {
    let mut rdx = rd::<usize>(param_1 + 0x08);
    let len = rd::<u32>(param_1 + 0x10) as usize;
    let mut r8 = rdx.wrapping_add(len.wrapping_mul(8));
    if rdx != r8 {
        let mut r9 = rdx.wrapping_add(8);
        if r9 != r8 {
            let mut r10 = rd::<u64>(rdx);
            loop {
                let rcx_val = rd::<u64>(r9);
                let mut rax = r9.wrapping_add(8);
                if r10 == rcx_val {
                    while rax != r8 {
                        let v = rd::<u64>(rax);
                        if rd::<u64>(rdx) != v {
                            rdx = rdx.wrapping_add(8);
                            wr::<u64>(rdx, v);
                        }
                        rax = rax.wrapping_add(8);
                    }
                    r8 = rdx.wrapping_add(8);
                    break;
                }
                rdx = r9;
                r9 = rax;
                r10 = rcx_val;
                if rax == r8 {
                    let new_len = ((r8.wrapping_sub(rd::<usize>(param_1 + 0x08))) as isize) >> 3;
                    exe::call_va!(0x1_411f_2320, (param_1, new_len as u64));
                    return;
                }
            }
        }
    }
    let new_len = ((r8.wrapping_sub(rd::<usize>(param_1 + 0x08))) as isize) >> 3;
    exe::call_va!(0x1_411f_2320, (param_1, new_len as u64));
}

/// `unknown_1411f15a0`
/// Original: 0x1411f15a0 (283 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f15a0(param_1: usize, param_2: u64) {
    let cap = rd::<u32>(param_1 + 0x14) as u64;
    if param_2 > cap {
        let mut r8_data = rd::<usize>(param_1 + 0x08);
        let old_bytes = cap.wrapping_mul(2);
        let arena = rd::<usize>(param_1);
        let new_bytes = param_2.wrapping_mul(2);
        let mut in_place = false;
        if r8_data == 0 {
            r8_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 2u32) -> usize);
            if r8_data != 0 {
                in_place = true;
            }
        } else if arena_try_grow_in_place(arena, r8_data, old_bytes, new_bytes) {
            in_place = true;
        }
        if !in_place {
            r8_data = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 2u32) -> usize);
            let len = rd::<u32>(param_1 + 0x10) as usize;
            let old_base = rd::<usize>(param_1 + 0x08);
            for i in 0..len {
                wr::<u16>(r8_data + i * 2, rd::<u16>(old_base + i * 2));
            }
        }
        wr::<usize>(param_1 + 0x08, r8_data);
        wr::<u32>(param_1 + 0x14, param_2 as u32);
    }
    let cur_len = rd::<u32>(param_1 + 0x10) as u64;
    if param_2 > cur_len {
        let mut dst = rd::<usize>(param_1 + 0x08).wrapping_add((cur_len as usize).wrapping_mul(2));
        for _ in 0..param_2.wrapping_sub(cur_len) {
            wr::<u16>(dst, 0xffff);
            dst = dst.wrapping_add(2);
        }
    }
    wr::<u32>(param_1 + 0x10, param_2 as u32);
}

/// `unknown_1411f1750`
/// Original: 0x1411f1750 (800 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f1750(param_1: usize, param_2: u64, param_3: usize) {
    let len = rd::<u32>(param_1 + 0x10);
    let cap = rd::<u32>(param_1 + 0x14);
    if len != cap && rd::<usize>(param_1 + 0x08) != 0 {
        exe::call_va!(0x1_411f_2c00, (param_1, param_2, param_3));
        return;
    }
    let arena = rd::<usize>(param_1);
    let need = len.wrapping_add(1) as u64;
    let old_bytes = (cap as u64).wrapping_mul(0x50);
    let mut new_cap = (cap as u64).wrapping_mul(3);
    if new_cap < need {
        new_cap = need;
    }
    let new_bytes = new_cap.wrapping_mul(0x50);
    let mut rdx_data = rd::<usize>(param_1 + 0x08);
    let mut in_place = false;
    if rdx_data == 0 {
        rdx_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 8u32) -> usize);
        if rdx_data != 0 {
            in_place = true;
        }
    } else if arena_try_grow_in_place(arena, rdx_data, old_bytes, new_bytes) {
        in_place = true;
    }
    if in_place {
        wr::<usize>(param_1 + 0x08, rdx_data);
        exe::call_va!(0x1_411f_2c00, (param_1, param_2, param_3));
        wr::<u32>(param_1 + 0x14, new_cap as u32);
        return;
    }

    let new_buf = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 8u32) -> usize);
    if param_2 != 0 {
        let old_base = rd::<usize>(param_1 + 0x08);
        for i in 0..(param_2 as usize) {
            move_elem_50(old_base + i * 0x50, new_buf + i * 0x50);
        }
    }
    let old_base = rd::<usize>(param_1 + 0x08);
    let cur_len = rd::<u32>(param_1 + 0x10) as usize;
    let ins_off = (param_2 as usize).wrapping_mul(0x50);
    let mut src = old_base.wrapping_add(ins_off);
    let end = old_base.wrapping_add(cur_len.wrapping_mul(0x50));
    let mut dst = new_buf.wrapping_add(ins_off).wrapping_add(0x50);
    while src != end {
        move_elem_50(src, dst);
        src = src.wrapping_add(0x50);
        dst = dst.wrapping_add(0x50);
    }
    move_elem_50(param_3, new_buf.wrapping_add(ins_off));
    wr::<u32>(param_1 + 0x10, rd::<u32>(param_1 + 0x10).wrapping_add(1));
    wr::<usize>(param_1 + 0x08, new_buf);
    wr::<u32>(param_1 + 0x14, new_cap as u32);
}

/// `unknown_1411f1a70`
/// Original: 0x1411f1a70 (452 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f1a70(param_1: usize, param_2: u64) {
    let cap = rd::<u32>(param_1 + 0x14) as u64;
    if param_2 > cap {
        let mut r8_data = rd::<usize>(param_1 + 0x08);
        let old_bytes = cap.wrapping_mul(0x30);
        let arena = rd::<usize>(param_1);
        let new_bytes = param_2.wrapping_mul(0x30);
        let mut in_place = false;
        if r8_data == 0 {
            r8_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 8u32) -> usize);
            if r8_data != 0 {
                in_place = true;
            }
        } else if arena_try_grow_in_place(arena, r8_data, old_bytes, new_bytes) {
            in_place = true;
        }
        if !in_place {
            r8_data = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 8u32) -> usize);
            let cnt = rd::<u32>(param_1 + 0x10) as usize;
            let old_base = rd::<usize>(param_1 + 0x08);
            for i in 0..cnt {
                let src = old_base + i * 0x30;
                let dst = r8_data + i * 0x30;
                wr::<u16>(dst, rd::<u16>(src));
                wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08));
                wr::<u64>(dst + 0x10, rd::<u64>(src + 0x10));
                wr::<u64>(dst + 0x18, rd::<u64>(src + 0x18));
                wr::<u32>(dst + 0x20, rd::<u32>(src + 0x20));
                wr::<u32>(dst + 0x24, rd::<u32>(src + 0x24));
                wr::<u64>(src + 0x18, 0);
                wr::<u64>(src + 0x20, 0);
                wr::<u32>(dst + 0x28, rd::<u32>(src + 0x28));
            }
        }
        wr::<usize>(param_1 + 0x08, r8_data);
        wr::<u32>(param_1 + 0x14, param_2 as u32);
    }
    let cur_len = rd::<u32>(param_1 + 0x10) as u64;
    if param_2 > cur_len {
        let base = rd::<usize>(param_1 + 0x08).wrapping_add((cur_len as usize).wrapping_mul(0x30));
        for i in 0..(param_2.wrapping_sub(cur_len) as usize) {
            let p = base.wrapping_add(i.wrapping_mul(0x30));
            wr::<u64>(p + 0x02, 0);
            wr::<u32>(p + 0x0a, 0);
            wr::<u16>(p + 0x0e, 0);
            wr::<u64>(p + 0x28, 0);
            wr::<u16>(p, 0xffff);
            wr::<u64>(p + 0x10, 0);
            wr::<u64>(p + 0x18, 0);
            wr::<u64>(p + 0x20, 0);
        }
    }
    wr::<u32>(param_1 + 0x10, param_2 as u32);
}

/// `unknown_1411f2180`
/// Original: 0x1411f2180 (405 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f2180(param_1: usize, param_2: u64, param_3: usize) {
    let len = rd::<u32>(param_1 + 0x10);
    let cap = rd::<u32>(param_1 + 0x14);
    if len != cap && rd::<usize>(param_1 + 0x08) != 0 {
        exe::call_va!(0x1_411f_3a10, (param_1, param_2, param_3));
        return;
    }
    let arena = rd::<usize>(param_1);
    let need = len.wrapping_add(1) as u64;
    let old_bytes = (cap as u64).wrapping_mul(8);
    let mut new_cap = (cap as u64).wrapping_mul(3);
    if new_cap < need {
        new_cap = need;
    }
    let new_bytes = new_cap.wrapping_mul(8);
    let mut rdx_data = rd::<usize>(param_1 + 0x08);
    let mut in_place = false;
    if rdx_data == 0 {
        rdx_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 8u32) -> usize);
        if rdx_data != 0 {
            in_place = true;
        }
    } else if arena_try_grow_in_place(arena, rdx_data, old_bytes, new_bytes) {
        in_place = true;
    }
    if in_place {
        wr::<usize>(param_1 + 0x08, rdx_data);
        exe::call_va!(0x1_411f_3a10, (param_1, param_2, param_3));
        wr::<u32>(param_1 + 0x14, new_cap as u32);
        return;
    }

    let new_buf = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 8u32) -> usize);
    if param_2 != 0 {
        let old_base = rd::<usize>(param_1 + 0x08);
        for i in 0..(param_2 as usize) {
            wr::<u64>(new_buf + i * 8, rd::<u64>(old_base + i * 8));
        }
    }
    let old_base = rd::<usize>(param_1 + 0x08);
    let cur_len = rd::<u32>(param_1 + 0x10) as usize;
    let ins_off = (param_2 as usize).wrapping_mul(8);
    let mut src = old_base.wrapping_add(ins_off);
    let end = old_base.wrapping_add(cur_len.wrapping_mul(8));
    let mut dst = new_buf.wrapping_add(ins_off).wrapping_add(8);
    while src != end {
        wr::<u64>(dst, rd::<u64>(src));
        src = src.wrapping_add(8);
        dst = dst.wrapping_add(8);
    }
    wr::<u64>(new_buf.wrapping_add(ins_off), rd::<u64>(param_3));
    wr::<u32>(param_1 + 0x10, rd::<u32>(param_1 + 0x10).wrapping_add(1));
    wr::<usize>(param_1 + 0x08, new_buf);
    wr::<u32>(param_1 + 0x14, new_cap as u32);
}

/// `unknown_1411f2490`
/// Original: 0x1411f2490 (223 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f2490(param_1: usize, param_2: u64) {
    let old_bytes = (rd::<u32>(param_1 + 0x14) as u64).wrapping_mul(4);
    let new_bytes = param_2.wrapping_mul(4);
    let mut r8_data = rd::<usize>(param_1 + 0x08);
    let arena = rd::<usize>(param_1);
    if r8_data == 0 {
        r8_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 4u32) -> usize);
        if r8_data != 0 {
            wr::<usize>(param_1 + 0x08, r8_data);
            wr::<u32>(param_1 + 0x14, param_2 as u32);
            return;
        }
    } else if arena_try_grow_in_place(arena, r8_data, old_bytes, new_bytes) {
        wr::<usize>(param_1 + 0x08, r8_data);
        wr::<u32>(param_1 + 0x14, param_2 as u32);
        return;
    }
    let new_buf = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 4u32) -> usize);
    let len = rd::<u32>(param_1 + 0x10) as usize;
    let old_base = rd::<usize>(param_1 + 0x08);
    for i in 0..len {
        wr::<u32>(new_buf + i * 4, rd::<u32>(old_base + i * 4));
    }
    wr::<usize>(param_1 + 0x08, new_buf);
    wr::<u32>(param_1 + 0x14, param_2 as u32);
}

/// `unknown_1411f2570`
/// Original: 0x1411f2570 (224 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f2570(param_1: usize, param_2: u64) {
    let old_bytes = (rd::<u32>(param_1 + 0x14) as u64).wrapping_mul(8);
    let new_bytes = param_2.wrapping_mul(8);
    let mut r8_data = rd::<usize>(param_1 + 0x08);
    let arena = rd::<usize>(param_1);
    if r8_data == 0 {
        r8_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 8u32) -> usize);
        if r8_data != 0 {
            wr::<usize>(param_1 + 0x08, r8_data);
            wr::<u32>(param_1 + 0x14, param_2 as u32);
            return;
        }
    } else if arena_try_grow_in_place(arena, r8_data, old_bytes, new_bytes) {
        wr::<usize>(param_1 + 0x08, r8_data);
        wr::<u32>(param_1 + 0x14, param_2 as u32);
        return;
    }
    let new_buf = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 8u32) -> usize);
    let len = rd::<u32>(param_1 + 0x10) as usize;
    let old_base = rd::<usize>(param_1 + 0x08);
    for i in 0..len {
        wr::<u64>(new_buf + i * 8, rd::<u64>(old_base + i * 8));
    }
    wr::<usize>(param_1 + 0x08, new_buf);
    wr::<u32>(param_1 + 0x14, param_2 as u32);
}

/// `unknown_1411f2a80`
/// Original: 0x1411f2a80 (372 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f2a80(param_1: usize) {
    let old_bytes = (rd::<u32>(param_1 + 0x14) as u64).wrapping_mul(0x50);
    let mut rdx_data = rd::<usize>(param_1 + 0x08);
    let arena = rd::<usize>(param_1);
    if rdx_data == 0 {
        rdx_data = exe::call_va!(0x1_411f_11e0, (arena, 0x500u64, 8u32) -> usize);
        if rdx_data != 0 {
            wr::<usize>(param_1 + 0x08, rdx_data);
            wr::<u32>(param_1 + 0x14, 0x10);
            return;
        }
    } else if arena_try_grow_in_place(arena, rdx_data, old_bytes, 0x500) {
        wr::<usize>(param_1 + 0x08, rdx_data);
        wr::<u32>(param_1 + 0x14, 0x10);
        return;
    }
    let new_buf = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), 0x500u64, 8u32) -> usize);
    let len = rd::<u32>(param_1 + 0x10) as usize;
    let old_base = rd::<usize>(param_1 + 0x08);
    for i in 0..len {
        move_elem_50(old_base + i * 0x50, new_buf + i * 0x50);
    }
    wr::<usize>(param_1 + 0x08, new_buf);
    wr::<u32>(param_1 + 0x14, 0x10);
}

/// `unknown_1411f2eb0`
/// Original: 0x1411f2eb0 (226 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f2eb0(param_1: usize, param_2: u64) {
    let old_bytes = (rd::<u32>(param_1 + 0x14) as u64).wrapping_mul(0x10);
    let new_bytes = param_2.wrapping_mul(0x10);
    let mut rdx_data = rd::<usize>(param_1 + 0x08);
    let arena = rd::<usize>(param_1);
    if rdx_data == 0 {
        rdx_data = exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 8u32) -> usize);
        if rdx_data != 0 {
            wr::<usize>(param_1 + 0x08, rdx_data);
            wr::<u32>(param_1 + 0x14, param_2 as u32);
            return;
        }
    } else if arena_try_grow_in_place(arena, rdx_data, old_bytes, new_bytes) {
        wr::<usize>(param_1 + 0x08, rdx_data);
        wr::<u32>(param_1 + 0x14, param_2 as u32);
        return;
    }
    let new_buf = exe::call_va!(0x1_411f_11e0, (rd::<usize>(param_1), new_bytes, 8u32) -> usize);
    let len = rd::<u32>(param_1 + 0x10) as usize;
    let old_base = rd::<usize>(param_1 + 0x08);
    for i in 0..len {
        let src = old_base + i * 0x10;
        let dst = new_buf + i * 0x10;
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08));
    }
    wr::<usize>(param_1 + 0x08, new_buf);
    wr::<u32>(param_1 + 0x14, param_2 as u32);
}

/// `unknown_1411f3720`
/// Original: 0x1411f3720 (293 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1411f3720(
    param_1: usize,
    param_2: usize,
    _param_3: u64,
    param_4: usize,
) -> usize {
    let rbx = rd::<usize>(param_1 + 0x10);
    let tid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool = rd::<usize>(rbx.wrapping_add(tid.wrapping_mul(0x40)).wrapping_add(0x48))
        .wrapping_add(0x80);
    exe::call_va!(0x1_4248_5aa0, (pool, param_2));

    let mut local_20 = [0u64; 5];
    local_20[0] = rd::<u64>(param_4);
    local_20[1] = rd::<u64>(param_4 + 0x08);
    local_20[2] = rd::<u64>(param_4 + 0x10);
    let lo_18 = rd::<u32>(param_4 + 0x18) as u64;
    let hi_1c = rd::<u32>(param_4 + 0x1c) as u64;
    local_20[3] = lo_18 | (hi_1c << 32);
    wr::<u64>(param_4 + 0x10, 0);
    wr::<u64>(param_4 + 0x18, 0);
    local_20[4] = rd::<u64>(param_4 + 0x20);

    exe::call_va!(0x1_411f_3a90, (param_2, local_20.as_mut_ptr() as usize));

    let r9 = rd::<usize>(param_4 + 0x10);
    if r9 != 0 {
        let r10 = r9.wrapping_sub(1) & !0x01ff_ffffusize;
        let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
        let seg_idx = (r9.wrapping_sub(r10)) >> 16;
        let mut rcx = r10.wrapping_add(0x108).wrapping_add(seg_idx.wrapping_mul(0x50));
        let back_off = rd::<u32>(rcx + 0x04) as usize;
        rcx = rcx.wrapping_sub(back_off);
        if same_thread {
            if rd::<u8>(rcx + 0x0e) == 0 {
                wr::<u64>(r9, rd::<u64>(rcx + 0x20));
                wr::<usize>(rcx + 0x20, r9);
                let prev = rd::<u32>(rcx + 0x18).wrapping_sub(1);
                wr::<u32>(rcx + 0x18, prev);
                if prev == 0 {
                    exe::call_va!(0x1_4253_71a0, (rcx));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (r10, rcx, 1u8, r9));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (r10, rcx, 0u8, r9));
        }
    }
    wr::<u64>(param_4 + 0x10, 0);
    wr::<u64>(param_4 + 0x18, 0);
    param_2
}

/// `std_1412decf0`
/// Original: 0x1412decf0 (719 bytes, Throws::No)
pub unsafe extern "win64" fn std_1412decf0(param_1: usize, param_2: usize) {
    let xmm6 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d10) as usize);

    let count1 = rd::<u64>(param_1 + 0x1b8) as usize;
    let end1 = param_1.wrapping_add(count1.wrapping_mul(0x28));
    if param_1 != end1 {
        let mut rsi = param_1.wrapping_add(0x10);
        loop {
            exe::call_iat!(0x1_4264_55a8, (rsi));
            let mut rax = rd::<usize>(rsi + 0x08);
            let rcx = rax.wrapping_add((rd::<u32>(rsi + 0x10) as usize).wrapping_mul(0x20));
            while rax != rcx {
                let mut rdx_lim =
                    (rd::<u16>(rax + 0x10) as u64).wrapping_mul(rd::<u64>(rsi - 0x10));
                let mut r10 = rd::<usize>(rax + 0x18);
                let mut r11: u64 = 0;
                if rdx_lim != 0 {
                    loop {
                        let idx = rd::<u32>(r10 + 0x1000);
                        if idx < rd::<u32>(param_2 + 0x08) {
                            let xmm2 = rd::<f32>(r10);
                            if xmm2 > xmm6 {
                                let arr_2e0 = rd::<usize>(rd::<usize>(param_2) + 0x2e0);
                                let slot = arr_2e0.wrapping_add((idx as usize).wrapping_mul(4));
                                let xmm1 = rd::<f32>(slot);
                                wr::<f32>(slot, if xmm1 < xmm2 { xmm2 } else { xmm1 });
                            }
                        }
                        rdx_lim =
                            (rd::<u16>(rax + 0x10) as u64).wrapping_mul(rd::<u64>(rsi - 0x10));
                        r11 = r11.wrapping_add(1);
                        r10 = r10.wrapping_add(4);
                        if r11 >= rdx_lim {
                            break;
                        }
                    }
                }
                rax = rax.wrapping_add(0x20);
            }
            exe::call_iat!(0x1_4264_5598, (rsi));
            rsi = rsi.wrapping_add(0x28);
            if rsi.wrapping_sub(0x10) == end1 {
                break;
            }
        }
    }

    let count2 = rd::<u64>(param_1 + 0x300) as usize;
    let start2 = param_1.wrapping_add(0x1c0);
    let end2 = start2.wrapping_add(count2.wrapping_mul(0x28));
    if start2 != end2 {
        let mut rbp = start2.wrapping_add(0x10);
        loop {
            exe::call_iat!(0x1_4264_55a8, (rbp));
            let mut rax = rd::<usize>(rbp + 0x08);
            let rcx = rax.wrapping_add((rd::<u32>(rbp + 0x10) as usize).wrapping_mul(0x30));
            while rax != rcx {
                let mut r11: u64 = 0;
                if rd::<u16>(rax + 0x10) != 0 {
                    loop {
                        let stride = rd::<u64>(rbp - 0x10);
                        let mut r10 = stride.wrapping_mul(r11);
                        let rdx_end = r11.wrapping_add(1).wrapping_mul(stride);
                        if r10 < rdx_end {
                            loop {
                                let idx_arr = rd::<usize>(rax + 0x20);
                                let idx = rd::<u32>(idx_arr.wrapping_add((r10 as usize) * 4));
                                let val_arr = rd::<usize>(rax + 0x18);
                                if idx < rd::<u32>(param_2 + 0x08) {
                                    let xmm2 = rd::<f32>(val_arr.wrapping_add((r11 as usize) * 4));
                                    if xmm2 > xmm6 {
                                        let arr_2e0 = rd::<usize>(rd::<usize>(param_2) + 0x2e0);
                                        let slot =
                                            arr_2e0.wrapping_add((idx as usize).wrapping_mul(4));
                                        let xmm1 = rd::<f32>(slot);
                                        wr::<f32>(slot, if xmm1 < xmm2 { xmm2 } else { xmm1 });
                                    }
                                }
                                r10 = r10.wrapping_add(1);
                                let rdx_end2 =
                                    r11.wrapping_add(1).wrapping_mul(rd::<u64>(rbp - 0x10));
                                if r10 >= rdx_end2 {
                                    break;
                                }
                            }
                        }
                        let cnt16 = rd::<u16>(rax + 0x10) as u64;
                        r11 = r11.wrapping_add(1);
                        if r11 >= cnt16 {
                            break;
                        }
                    }
                }
                rax = rax.wrapping_add(0x30);
            }
            exe::call_iat!(0x1_4264_5598, (rbp));
            rbp = rbp.wrapping_add(0x28);
            if rbp.wrapping_sub(0x10) == end2 {
                break;
            }
        }
    }

    exe::call_iat!(0x1_4264_55a8, (param_1 + 0x308));
    let mut rbx = rd::<usize>(param_1 + 0x310);
    let rsi = rbx.wrapping_add((rd::<u32>(param_1 + 0x318) as usize).wrapping_mul(8));
    while rbx != rsi {
        let r15 = rd::<usize>(rbx);
        exe::call_iat!(0x1_4264_55a8, (r15 + 0x08));
        let mut rax = rd::<usize>(r15 + 0x10);
        let rdx = rax.wrapping_add((rd::<u32>(r15 + 0x18) as usize).wrapping_mul(4));
        while rax != rdx {
            let idx = rd::<u32>(rax);
            if idx < rd::<u32>(param_2 + 0x08) {
                let xmm2 = rd::<f32>(r15);
                if xmm2 > xmm6 {
                    let arr_2e0 = rd::<usize>(rd::<usize>(param_2) + 0x2e0);
                    let slot = arr_2e0.wrapping_add((idx as usize).wrapping_mul(4));
                    let xmm1 = rd::<f32>(slot);
                    wr::<f32>(slot, if xmm1 < xmm2 { xmm2 } else { xmm1 });
                }
            }
            rax = rax.wrapping_add(4);
        }
        exe::call_iat!(0x1_4264_5598, (r15 + 0x08));
        rbx = rbx.wrapping_add(8);
    }
    exe::call_iat!(0x1_4264_5598, (param_1 + 0x308));
}

/// `rend_1412eb9d0`
/// Original: 0x1412eb9d0 (294 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412eb9d0(param_1: usize, param_2: u64, param_3: usize) {
    let mut rbx = rd::<u64>(param_1 + 0x58);
    if param_2 >= rbx {
        let top = rbx.wrapping_add(param_2).wrapping_sub(1);
        rbx = top.wrapping_sub(top % rbx);
    }
    wr::<u64>(param_1 + 0x90, param_2);
    let rsi = param_1 + 0x98;
    let rbp = rd::<u32>(rsi + 0x08) as u64;
    let rcx_data = rd::<usize>(rsi);
    let rax_cap = rd::<u32>(rsi + 0x0c) as u64;
    if rbx > rax_cap {
        exe::call_va!(0x1_401d_eb40, (rsi, rbx));
        let cur_len = rd::<u32>(rsi + 0x08) as usize;
        let new_data = rd::<usize>(rsi);
        let mut r9 = new_data.wrapping_add(cur_len.wrapping_mul(4));
        let cnt = rbx.wrapping_sub(rbp);
        if cnt != 0 {
            if rd::<u32>(param_3) == 0 {
                let _ = exe::call_va!(0x1_4183_acc2, (r9, 0u32, cnt.wrapping_mul(4)) -> usize);
            } else {
                for _ in 0..cnt {
                    wr::<u32>(r9, rd::<u32>(param_3));
                    r9 = r9.wrapping_add(4);
                }
            }
        }
        wr::<u32>(rsi + 0x08, rbx as u32);
    } else if rbx > rbp {
        let mut dst = rcx_data.wrapping_add((rbp as usize).wrapping_mul(4));
        let cnt = rbx.wrapping_sub(rbp);
        if cnt != 0 {
            if rd::<u32>(param_3) == 0 {
                let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, cnt.wrapping_mul(4)) -> usize);
            } else {
                for _ in 0..cnt {
                    wr::<u32>(dst, rd::<u32>(param_3));
                    dst = dst.wrapping_add(4);
                }
            }
        }
        wr::<u32>(rsi + 0x08, rbx as u32);
    } else if rbx < rbp {
        wr::<u32>(rsi + 0x08, rbx as u32);
    }
    exe::call_va!(0x1_412e_be10, (rsi));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0xa8, rbx));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_sparsehash_1411eef20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, len: usize, depth: i64| -> Vec<u16> {
        let mut data: Vec<u16> = (0..len)
            .map(|i| ((i.wrapping_mul(73).wrapping_add(19) ^ (i >> 1)) & 0x7f) as u16)
            .collect();
        let mut beg = data.as_mut_ptr() as usize;
        let mut end = beg.wrapping_add(len * 2);
        f(
            core::ptr::addr_of_mut!(beg) as usize,
            core::ptr::addr_of_mut!(end) as usize,
            depth,
            0,
        );
        data
    };

    for (len, depth) in [(0, 4), (1, 4), (16, 4), (48, 0), (50, 1), (80, 8)] {
        let o = run(f_o, len, depth);
        let n = run(f_n, len, depth);
        crate::diff_eq!(o, n, "1411eef20 len={} depth={}", len, depth);
    }
    Ok(6)
}

pub unsafe fn diff_sparsehash_1411ef200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, len: usize| -> (Vec<u16>, usize, usize) {
        let mut data: Vec<u16> = (0..len)
            .map(|i| ((i.wrapping_mul(37).wrapping_add(11)) % 9) as u16)
            .collect();
        let base = data.as_mut_ptr() as usize;
        let mut beg = base;
        let mut end = base.wrapping_add(len * 2);
        let mut out = [0usize; 2];
        let _ = f(
            out.as_mut_ptr() as usize,
            core::ptr::addr_of_mut!(beg) as usize,
            core::ptr::addr_of_mut!(end) as usize,
            0,
        );
        (data, out[0].wrapping_sub(base), out[1].wrapping_sub(base))
    };

    for len in [12usize, 32, 60] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1411ef200 len={}", len);
    }
    Ok(3)
}

pub unsafe fn diff_sparsehash_1411f03b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, range: u64| -> (Vec<u16>, Vec<u16>, u32, u32) {
        let mut v0: Vec<u16> = vec![10, 20, 30, 40, 50];
        let mut v1: Vec<u16> = vec![5, 20, 25, 50];
        let mut entries = [0u64; 4];
        let ep = entries.as_mut_ptr() as usize;
        wr::<usize>(ep, v0.as_mut_ptr() as usize);
        wr::<u32>(ep + 0x08, v0.len() as u32);
        wr::<u32>(ep + 0x0c, 8);
        wr::<usize>(ep + 0x10, v1.as_mut_ptr() as usize);
        wr::<u32>(ep + 0x18, v1.len() as u32);
        wr::<u32>(ep + 0x1c, 8);

        let mut p1 = ep;
        let mut keys: Vec<u16> = vec![20, 35, 50];
        let p2: [u64; 2] = [keys.len() as u64, keys.as_mut_ptr() as usize as u64];
        f(
            core::ptr::addr_of_mut!(p1) as usize,
            p2.as_ptr() as usize,
            range,
        );
        (v0, v1, rd::<u32>(ep + 0x08), rd::<u32>(ep + 0x18))
    };

    for range in [0u64, 0x0000_0002_0000_0000u64] {
        let o = run(f_o, range);
        let n = run(f_n, range);
        crate::diff_eq!(o, n, "1411f03b0 range={:#x}", range);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f12b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_CALL: (u64, u64) = (0, 0);
    unsafe extern "win64" fn mock_11e0(_a: usize, sz: u64, al: u64) -> usize {
        ALLOC_CALL = (sz, al);
        0x5555_0000
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, mode: u8| -> (usize, u64, (u64, u64)) {
        ALLOC_CALL = (0, 0);
        let blk_base = 0x1000_0000usize;
        let mut blk_arr = [blk_base as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if mode == 3 { 0 } else { 1 });
        wr::<u64>(ap + 0x10, 0x40);
        wr::<u64>(ap + 0x18, if mode == 2 { 0x50 } else { 0x100 });

        let p2 = match mode {
            0 => 0,
            _ => blk_base + 0x20,
        };
        let ret = f(ap, p2, 0x20, 0x40, 8);
        (ret, rd::<u64>(ap + 0x10), ALLOC_CALL)
    };

    for mode in 0..4 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1411f12b0 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1411f13d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESIZE_LEN: u64 = 0;
    unsafe extern "win64" fn mock_2320(_p1: usize, len: u64) {
        RESIZE_LEN = len;
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_2320, mock_2320 as *const ())?;

    let run = |f: F, vals: &[u64]| -> (Vec<u64>, u64) {
        RESIZE_LEN = 999;
        let mut arr = vals.to_vec();
        let mut hdr = [0u64; 3];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x08, arr.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x10, arr.len() as u32);
        f(hp);
        (arr, RESIZE_LEN)
    };

    for vals in [
        &[][..],
        &[10u64, 20, 30][..],
        &[10u64, 20, 20, 30, 30, 30, 40][..],
    ] {
        let o = run(f_o, vals);
        let n = run(f_n, vals);
        crate::diff_eq!(o, n, "1411f13d0 vals={:?}", vals);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1411f15a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u16; 32] = [0; 32];
    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, in_place: bool, new_len: u64| -> ([u16; 16], u32, u32, u64) {
        NEW_BUF = [0x1111; 32];
        let mut old_buf = [0x2222u16; 16];
        old_buf[0] = 0x1234;
        old_buf[1] = 0x5678;
        let obp = old_buf.as_mut_ptr() as usize;

        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 8);
        wr::<u64>(ap + 0x18, 64);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 2);
        wr::<u32>(vp + 0x14, 4);

        f(vp, new_len);
        let out_ptr = rd::<usize>(vp + 0x08);
        (
            rd::<[u16; 16]>(out_ptr),
            rd::<u32>(vp + 0x10),
            rd::<u32>(vp + 0x14),
            rd::<u64>(ap + 0x10),
        )
    };

    for (in_place, new_len) in [(true, 3u64), (true, 8), (false, 8)] {
        let o = run(f_o, in_place, new_len);
        let n = run(f_n, in_place, new_len);
        crate::diff_eq!(o, n, "1411f15a0 in_place={} new_len={}", in_place, new_len);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1411f1750(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u8; 0x50 * 8] = [0; 0x50 * 8];
    static mut C2C00_CALLED: u64 = 0;

    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    unsafe extern "win64" fn mock_2c00(_p1: usize, idx: u64, _p3: usize) {
        C2C00_CALLED = idx + 100;
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411f_2c00, mock_2c00 as *const ())?;

    let run = |f: F, in_place: bool| -> (Vec<u8>, Vec<u8>, Vec<u8>, u32, u32, u64) {
        NEW_BUF = [0; 0x50 * 8];
        C2C00_CALLED = 0;
        let mut old_buf = vec![0u8; 0x50 * 4];
        for (i, b) in old_buf[..0x50 * 2].iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        let mut elem = vec![0u8; 0x50];
        for (i, b) in elem.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(0x80);
        }
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 0xa0);
        wr::<u64>(ap + 0x18, 0x500);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 2);
        wr::<u32>(vp + 0x14, 2);

        f(vp, 1, elem.as_mut_ptr() as usize);
        (
            unsafe { NEW_BUF.to_vec() },
            old_buf,
            elem,
            rd::<u32>(vp + 0x10),
            rd::<u32>(vp + 0x14),
            C2C00_CALLED,
        )
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f1750 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f1a70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u8; 0x30 * 6] = [0; 0x30 * 6];
    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, in_place: bool| -> (Vec<u8>, Vec<u8>, u32, u32, u64) {
        NEW_BUF = [0xaa; 0x30 * 6];
        let mut old_buf = vec![0x55u8; 0x30 * 4];
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 0x30);
        wr::<u64>(ap + 0x18, 0x300);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 1);
        wr::<u32>(vp + 0x14, 1);

        f(vp, 3);
        (
            unsafe { NEW_BUF.to_vec() },
            old_buf,
            rd::<u32>(vp + 0x10),
            rd::<u32>(vp + 0x14),
            rd::<u64>(ap + 0x10),
        )
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f1a70 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f2180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u64; 16] = [0; 16];
    static mut C3A10_CALLED: u64 = 0;

    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    unsafe extern "win64" fn mock_3a10(_p1: usize, idx: u64, _p3: usize) {
        C3A10_CALLED = idx + 200;
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411f_3a10, mock_3a10 as *const ())?;

    let run = |f: F, in_place: bool| -> ([u64; 16], u32, u32, u64) {
        NEW_BUF = [0; 16];
        C3A10_CALLED = 0;
        let mut old_buf = [10u64, 20, 30, 0, 0, 0, 0, 0];
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 24);
        wr::<u64>(ap + 0x18, 256);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 3);
        wr::<u32>(vp + 0x14, 3);

        let val: u64 = 99;
        f(vp, 1, core::ptr::addr_of!(val) as usize);
        (
            unsafe { NEW_BUF },
            rd::<u32>(vp + 0x10),
            rd::<u32>(vp + 0x14),
            C3A10_CALLED,
        )
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f2180 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f2490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u32; 16] = [0; 16];
    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, in_place: bool| -> ([u32; 16], u32, u64) {
        NEW_BUF = [0; 16];
        let mut old_buf = [11u32, 22, 33, 44];
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 16);
        wr::<u64>(ap + 0x18, 128);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 3);
        wr::<u32>(vp + 0x14, 4);

        f(vp, 8);
        (unsafe { NEW_BUF }, rd::<u32>(vp + 0x14), rd::<u64>(ap + 0x10))
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f2490 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f2570(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u64; 16] = [0; 16];
    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, in_place: bool| -> ([u64; 16], u32, u64) {
        NEW_BUF = [0; 16];
        let mut old_buf = [110u64, 220, 330, 440];
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 32);
        wr::<u64>(ap + 0x18, 256);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 3);
        wr::<u32>(vp + 0x14, 4);

        f(vp, 8);
        (unsafe { NEW_BUF }, rd::<u32>(vp + 0x14), rd::<u64>(ap + 0x10))
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f2570 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f2a80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u8; 0x500] = [0; 0x500];
    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, in_place: bool| -> (Vec<u8>, Vec<u8>, u32, u64) {
        NEW_BUF = [0; 0x500];
        let mut old_buf = vec![0u8; 0x100];
        for (i, b) in old_buf[..0xa0].iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(3);
        }
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 0xa0);
        wr::<u64>(ap + 0x18, 0x800);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 2);
        wr::<u32>(vp + 0x14, 2);

        f(vp);
        (
            unsafe { NEW_BUF[..0x100].to_vec() },
            old_buf,
            rd::<u32>(vp + 0x14),
            rd::<u64>(ap + 0x10),
        )
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f2a80 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f2eb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u64; 32] = [0; 32];
    unsafe extern "win64" fn mock_11e0(_a: usize, _sz: u64, _al: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;

    let run = |f: F, in_place: bool| -> ([u64; 32], u32, u64) {
        NEW_BUF = [0; 32];
        let mut old_buf = [1u64, 2, 3, 4, 5, 6, 7, 8];
        let obp = old_buf.as_mut_ptr() as usize;
        let mut blk_arr = [obp as u64];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, blk_arr.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, if in_place { 1 } else { 0 });
        wr::<u64>(ap + 0x10, 64);
        wr::<u64>(ap + 0x18, 512);

        let mut vec_hdr = [0u64; 3];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, ap);
        wr::<usize>(vp + 0x08, obp);
        wr::<u32>(vp + 0x10, 3);
        wr::<u32>(vp + 0x14, 4);

        f(vp, 8);
        (unsafe { NEW_BUF }, rd::<u32>(vp + 0x14), rd::<u64>(ap + 0x10))
    };

    for in_place in [false, true] {
        let o = run(f_o, in_place);
        let n = run(f_n, in_place);
        crate::diff_eq!(o, n, "1411f2eb0 in_place={}", in_place);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f3720(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut POOL_ARG: usize = 0;
    static mut CAPTURED_20: [u64; 5] = [0; 5];

    unsafe extern "win64" fn mock_7c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_5aa0(pool: usize, out_h: usize) {
        POOL_ARG = pool;
        wr::<[u64; 2]>(out_h, [0xaaaa_1111, 0xbbbb_2222]);
    }
    unsafe extern "win64" fn mock_3a90(_p2: usize, src: usize) {
        CAPTURED_20 = rd::<[u64; 5]>(src);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411f_3a90, mock_3a90 as *const ())?;

    let run = |f: F| -> (usize, [u64; 2], [u64; 5], [u64; 5]) {
        POOL_ARG = 0;
        CAPTURED_20 = [0; 5];
        let mut rbx_tbl = [0u8; 0xc0];
        let rbp = rbx_tbl.as_mut_ptr() as usize;
        wr::<usize>(rbp + 0x40 + 0x48, 0x3000_0000);

        let mut p1 = [0u64; 4];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x10, rbp);

        let mut p2_handle = [0u64; 2];
        let mut p4 = [10u64, 20, 30, 0x0000_0005_0000_0004, 50];
        let ret = f(p1p, p2_handle.as_mut_ptr() as usize, 0, p4.as_mut_ptr() as usize);
        let _ = ret;
        (unsafe { POOL_ARG }, p2_handle, unsafe { CAPTURED_20 }, p4)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1411f3720");
    Ok(1)
}

pub unsafe fn diff_std_1412decf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> [u32; 8] {
        let mut out_floats = [1.0f32, 5.0, 2.0, 9.0, 3.0, 6.0, 0.0, 4.0];
        let mut p2_inner = [0u8; 0x300];
        let p2ip = p2_inner.as_mut_ptr() as usize;
        wr::<usize>(p2ip + 0x2e0, out_floats.as_mut_ptr() as usize);

        let mut p2 = [0u64; 2];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, p2ip);
        wr::<u32>(p2p + 0x08, 8);

        let mut buf1 = vec![0u8; 0x1010];
        let b1p = buf1.as_mut_ptr() as usize;
        wr::<f32>(b1p, 7.5);
        wr::<f32>(b1p + 0x04, 3.5);
        wr::<u32>(b1p + 0x1000, 1);
        wr::<u32>(b1p + 0x1004, 2);

        let mut item1 = [0u8; 0x20];
        let i1p = item1.as_mut_ptr() as usize;
        wr::<u16>(i1p + 0x10, 1);
        wr::<usize>(i1p + 0x18, b1p);

        let mut vals2 = [8.25f32, 2.5f32];
        let mut idxs2 = [4u32, 5u32];
        let mut item2 = [0u8; 0x30];
        let i2p = item2.as_mut_ptr() as usize;
        wr::<u16>(i2p + 0x10, 2);
        wr::<usize>(i2p + 0x18, vals2.as_mut_ptr() as usize);
        wr::<usize>(i2p + 0x20, idxs2.as_mut_ptr() as usize);

        let mut idxs3 = [0u32, 6u32];
        let mut item3 = [0u8; 0x20];
        let i3p = item3.as_mut_ptr() as usize;
        wr::<f32>(i3p, 6.75);
        wr::<usize>(i3p + 0x10, idxs3.as_mut_ptr() as usize);
        wr::<u32>(i3p + 0x18, 2);
        let mut ptrs3 = [i3p];

        let mut p1 = vec![0u8; 0x330];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p, 2);
        wr::<usize>(p1p + 0x18, i1p);
        wr::<u32>(p1p + 0x20, 1);
        wr::<u64>(p1p + 0x1b8, 1);

        wr::<u64>(p1p + 0x1c0, 1);
        wr::<usize>(p1p + 0x1d8, i2p);
        wr::<u32>(p1p + 0x1e0, 1);
        wr::<u64>(p1p + 0x300, 1);

        wr::<usize>(p1p + 0x310, ptrs3.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x318, 1);

        f(p1p, p2p);
        out_floats.map(f32::to_bits)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412decf0");
    Ok(1)
}

pub unsafe fn diff_rend_1412eb9d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLS: (u64, u64) = (0, 0);
    unsafe extern "win64" fn mock_eb40(_v: usize, sz: u64) {
        CALLS.0 = sz;
    }
    unsafe extern "win64" fn mock_be10(_v: usize) {}
    unsafe extern "win64" fn mock_f1e0(_v: usize, sz: u64) {
        CALLS.1 = sz;
    }
    let _m1 = crate::difftest::Mock::new(0x1_401d_eb40, mock_eb40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_be10, mock_be10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;

    let run = |f: F, new_sz: u64, cap: u32, fill: u32| -> ([u32; 8], u64, u32, (u64, u64)) {
        CALLS = (0, 0);
        let mut data = [0x1111_2222u32; 8];
        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x58, 4);
        wr::<usize>(p1p + 0x98, data.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0xa0, 2);
        wr::<u32>(p1p + 0xa4, cap);

        f(p1p, new_sz, core::ptr::addr_of!(fill) as usize);
        (data, rd::<u64>(p1p + 0x90), rd::<u32>(p1p + 0xa0), unsafe { CALLS })
    };

    for (new_sz, cap, fill) in [(5u64, 2u32, 0u32), (5, 8, 0xdead_beef), (2, 8, 7)] {
        let o = run(f_o, new_sz, cap, fill);
        let n = run(f_n, new_sz, cap, fill);
        crate::diff_eq!(o, n, "1412eb9d0 sz={} cap={} fill={}", new_sz, cap, fill);
    }
    Ok(3)
}
