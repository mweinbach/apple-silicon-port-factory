//! Decompiled `rend` hot functions shard Y1 (`0x1412dfdd0..0x141353a10`).

use crate::exe::{self, rd, wr};

/// `rend_1412dfdd0`
/// Original: 0x1412dfdd0 (198 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412dfdd0() -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let pool = rd::<usize>(heap + 0x88);
    let mut ptr = rd::<usize>(pool + 0x10);
    if ptr != 0 {
        wr::<u32>(pool + 0x18, rd::<u32>(pool + 0x18).wrapping_add(1));
        wr::<usize>(pool + 0x10, rd::<usize>(ptr));
    } else {
        ptr = exe::call_va!(0x1_4253_7280, (heap, 0x80u64, 0u64, 0u64) -> usize);
        if ptr == 0 {
            let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
            exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
            ptr = 0;
        }
    }

    wr::<[u64; 16]>(ptr, [0; 16]);
    let vftbl = exe::ptr::<u8>(0x1_433e_fd70) as usize;
    wr::<u8>(ptr + 0x08, 1);
    wr::<usize>(ptr, vftbl);
    wr::<u32>(ptr + 0x10, 0xffff_ffff);
    wr::<u64>(ptr + 0x14, 0);
    wr::<u32>(ptr + 0x1c, 0x0000_ff01);
    wr::<u64>(ptr + 0x60, 0);
    wr::<u32>(ptr + 0x68, 1);
    wr::<u64>(ptr + 0x6c, 1);
    wr::<u64>(ptr + 0x74, 0x3f80_0000);
    ptr
}

/// `rend_1412dff90`
/// Original: 0x1412dff90 (968 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412dff90(
    param_1: usize,
    param_2: usize,
    _param_3: u8,
    param_4: u64,
) -> u64 {
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4388_1950) as usize);
    exe::call_va!(
        0x1_4243_b0b0,
        (param_2, param_1, name_ptr, param_4, 8u32, 8u32)
    );

    let crc_tbl = exe::ptr::<u32>(0x1_426d_4580) as usize;
    let crc_step = |mut crc: u32, bytes: &[u8]| -> u32 {
        for &b in bytes {
            let idx = ((crc as u8) ^ b) as usize;
            crc = (crc >> 8) ^ rd::<u32>(crc_tbl + idx * 4);
        }
        crc
    };

    let mut r8d = !rd::<u32>(param_2 + 0x338);

    r8d = crc_step(r8d, &rd::<[u8; 4]>(param_1 + 0x10));
    wr::<u32>(param_2 + 0x338, !r8d);

    r8d = crc_step(r8d, &rd::<[u8; 2]>(param_1 + 0x14));
    wr::<u32>(param_2 + 0x338, !r8d);

    r8d = crc_step(r8d, &rd::<[u8; 2]>(param_1 + 0x16));
    wr::<u32>(param_2 + 0x338, !r8d);

    r8d = crc_step(r8d, &rd::<[u8; 2]>(param_1 + 0x18));
    wr::<u32>(param_2 + 0x338, !r8d);

    r8d = crc_step(r8d, &rd::<[u8; 2]>(param_1 + 0x1a));
    wr::<u32>(param_2 + 0x338, !r8d);

    for off in [0x1cu8, 0x1d, 0x1e, 0x1f] {
        r8d = crc_step(r8d, &[rd::<u8>(param_1 + off as usize)]);
        wr::<u32>(param_2 + 0x338, !r8d);
    }

    let count_60 = rd::<u64>(param_1 + 0x60);
    r8d = crc_step(r8d, &count_60.to_le_bytes());
    r8d = !r8d;
    wr::<u32>(param_2 + 0x338, r8d);

    let mut cur = param_1.wrapping_add(0x20);
    let end = cur.wrapping_add((rd::<u64>(param_1 + 0x60) as usize).wrapping_mul(8));
    while cur != end {
        let lo = rd::<u32>(cur);
        let diff = rd::<u32>(cur + 0x04).wrapping_sub(lo);
        r8d = !r8d;
        r8d = crc_step(r8d, &lo.to_le_bytes());
        r8d = crc_step(r8d, &diff.to_le_bytes());
        r8d = !r8d;
        wr::<u32>(param_2 + 0x338, r8d);
        cur = cur.wrapping_add(8);
    }

    r8d = !r8d;
    for off in [0x68usize, 0x6c, 0x70, 0x74, 0x78] {
        r8d = crc_step(r8d, &rd::<[u8; 4]>(param_1 + off));
        wr::<u32>(param_2 + 0x338, !r8d);
    }

    let stack_idx = rd::<u32>(param_2 + 0x08);
    let stack_base = rd::<usize>(param_2);
    let prev_ver = rd::<u32>(
        stack_base
            .wrapping_add((stack_idx as usize).wrapping_mul(0x30))
            .wrapping_sub(8),
    );
    wr::<u32>(param_2 + 0x08, stack_idx.wrapping_sub(1));
    wr::<u32>(param_2 + 0x328, prev_ver);
    0
}

/// `rend_1412e0930`
/// Original: 0x1412e0930 (1030 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_1412e0930(param_1: usize, param_2: usize) -> u64 {
    let crc_val = rd::<u32>(exe::ptr::<u32>(0x1_4388_1958) as usize);
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4388_1950) as usize);
    exe::call_va!(
        0x1_401e_9b60,
        (param_2, param_1, name_ptr, crc_val, 8u32, 8u32)
    );

    let read_u32 = |p2: usize| -> u32 {
        let s = rd::<usize>(p2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        let v = rd::<u32>(p);
        wr::<usize>(s + 0x120, p.wrapping_add(4));
        v
    };
    let read_u16 = |p2: usize| -> u16 {
        let s = rd::<usize>(p2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        let v = rd::<u16>(p);
        wr::<usize>(s + 0x120, p.wrapping_add(2));
        v
    };
    let read_u8 = |p2: usize| -> u8 {
        let s = rd::<usize>(p2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        let v = rd::<u8>(p);
        wr::<usize>(s + 0x120, p.wrapping_add(1));
        v
    };

    if rd::<i32>(param_2 + 0x328) >= 8 {
        wr::<u32>(param_1 + 0x10, read_u32(param_2));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x10, 0xffff_ffff);
    }

    for off in [0x14usize, 0x16, 0x18, 0x1a] {
        if rd::<i32>(param_2 + 0x328) >= 8 {
            wr::<u16>(param_1 + off, read_u16(param_2));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u16>(param_1 + off, 0);
        }
    }

    for (off, def_val) in [(0x1cusize, 1u8), (0x1d, 0xff), (0x1e, 0), (0x1f, 0)] {
        if rd::<i32>(param_2 + 0x328) >= 8 {
            wr::<u8>(param_1 + off, read_u8(param_2));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u8>(param_1 + off, def_val);
        }
    }

    let new_len_u32 = if rd::<i32>(param_2 + 0x328) >= 8 {
        read_u32(param_2)
    } else {
        0
    };
    let old_len = rd::<u64>(param_1 + 0x60);
    let new_len = new_len_u32 as u64;
    if new_len > old_len {
        let mut cur = param_1.wrapping_add((old_len.wrapping_add(4) as usize).wrapping_mul(8));
        let end = (param_1 + 0x20).wrapping_add((new_len as usize).wrapping_mul(8));
        while cur != end {
            wr::<u64>(cur, 0);
            cur = cur.wrapping_add(8);
        }
    }
    wr::<u64>(param_1 + 0x60, new_len);

    let count_i32 = new_len_u32 as i32;
    if count_i32 > 0 {
        for i in 0..(count_i32 as usize) {
            let a = if rd::<i32>(param_2 + 0x328) >= 8 {
                read_u32(param_2)
            } else {
                0
            };
            let b = if rd::<i32>(param_2 + 0x328) >= 8 {
                read_u32(param_2)
            } else {
                0
            };
            let hi = b.wrapping_add(a);
            let packed = (a as u64) | ((hi as u64) << 32);
            wr::<u64>(param_1 + 0x20 + i * 8, packed);
        }
    }

    for (off, min_ver, def_val) in [
        (0x68usize, 2i32, 1u32),
        (0x6c, 2, 1),
        (0x70, 5, 0),
        (0x74, 5, 0x3f80_0000),
        (0x78, 7, 0),
    ] {
        if rd::<i32>(param_2 + 0x328) >= min_ver {
            wr::<u32>(param_1 + off, read_u32(param_2));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u32>(param_1 + off, def_val);
        }
    }

    exe::call_va!(0x1_401e_9a20, (param_2));
    0
}

/// `rend_1412e8930`
/// Original: 0x1412e8930 (1084 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412e8930(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: usize,
) {
    let mut r13 = rd::<usize>(param_2);
    let mut r14 = rd::<usize>(param_1);
    let mut rsi = param_3;

    while ((r13.wrapping_sub(r14) as i64) & -0x40) > 0x800 {
        if rsi <= 0 {
            let count: i64 = (r13.wrapping_sub(r14) as i64) >> 6;
            let mut hole_top: i64 = count >> 1;
            if hole_top > 0 {
                let max_non_leaf: i64 = (count - 1) >> 1;
                let mut cur_ptr = r14.wrapping_add((hole_top as usize) << 6);
                loop {
                    let mut val = rd::<[u64; 8]>(cur_ptr.wrapping_sub(0x40));
                    cur_ptr = cur_ptr.wrapping_sub(0x40);
                    hole_top -= 1;
                    let mut rdi_idx = hole_top;
                    let mut rcx_idx = hole_top;
                    while rcx_idx < max_non_leaf {
                        let rbx_idx = rcx_idx * 2;
                        let right_idx = rbx_idx + 2;
                        let right_ptr = r14.wrapping_add((right_idx as usize) << 6);
                        let left_ptr = right_ptr.wrapping_sub(0x40);
                        let c = exe::call_va!(param_4, (right_ptr, left_ptr) -> u8);
                        rcx_idx = rbx_idx + if c != 0 { 1 } else { 2 };
                        let child_val = rd::<[u64; 8]>(r14.wrapping_add((rcx_idx as usize) << 6));
                        wr::<[u64; 8]>(r14.wrapping_add((rdi_idx as usize) << 6), child_val);
                        rdi_idx = rcx_idx;
                    }
                    if rdi_idx == max_non_leaf && (count & 1) == 0 {
                        let last_val =
                            rd::<[u64; 8]>(r14.wrapping_add((count as usize) << 6).wrapping_sub(0x40));
                        wr::<[u64; 8]>(r14.wrapping_add((rdi_idx as usize) << 6), last_val);
                        rdi_idx = count - 1;
                    }
                    let mut parent_idx = (rdi_idx - 1) >> 1;
                    while hole_top < rdi_idx {
                        let parent_ptr = r14.wrapping_add((parent_idx as usize) << 6);
                        let c = exe::call_va!(param_4, (parent_ptr, val.as_mut_ptr() as usize) -> u8);
                        if c == 0 {
                            break;
                        }
                        wr::<[u64; 8]>(
                            r14.wrapping_add((rdi_idx as usize) << 6),
                            rd::<[u64; 8]>(parent_ptr),
                        );
                        rdi_idx = parent_idx;
                        parent_idx = (parent_idx - 1) >> 1;
                    }
                    wr::<[u64; 8]>(r14.wrapping_add((rdi_idx as usize) << 6), val);
                    if hole_top <= 0 {
                        break;
                    }
                }
            }

            let mut rbx_end = rd::<usize>(param_2);
            let rdi_beg = rd::<usize>(param_1);
            while ((rbx_end.wrapping_sub(rdi_beg) as i64) & -0x40) >= 0x80 {
                let rax_last = rbx_end.wrapping_sub(0x40);
                let diff = (rbx_end.wrapping_sub(rdi_beg) as i64) & -0x40;
                rbx_end = rax_last;
                if diff >= 0x80 {
                    let mut val = rd::<[u64; 8]>(rax_last);
                    wr::<[u64; 8]>(rax_last, rd::<[u64; 8]>(rdi_beg));
                    let mut first_slot: usize = rdi_beg;
                    let heap_len: i64 = (rax_last.wrapping_sub(rdi_beg) as i64) >> 6;
                    exe::call_va!(
                        0x1_412e_9cd0,
                        (
                            (&mut first_slot as *mut usize) as usize,
                            0i64,
                            heap_len,
                            val.as_mut_ptr() as usize,
                            param_4
                        )
                    );
                }
            }
            return;
        }

        let mut local_last: usize = r13;
        let mut local_first: usize = r14;
        let mut mid_pair = [0usize; 2];
        exe::call_va!(
            0x1_412e_9310,
            (
                mid_pair.as_mut_ptr() as usize,
                (&mut local_first as *mut usize) as usize,
                (&mut local_last as *mut usize) as usize,
                param_4
            )
        );
        let cur_last = rd::<usize>(param_2);
        let mid_lo = mid_pair[0];
        let mid_hi = mid_pair[1];
        let cur_first = rd::<usize>(param_1);
        rsi = (rsi >> 1).wrapping_add(rsi >> 2);
        let right_bytes = (cur_last.wrapping_sub(mid_hi) as i64) & -0x40;
        let left_bytes = (mid_lo.wrapping_sub(cur_first) as i64) & -0x40;
        if left_bytes < right_bytes {
            local_last = cur_first;
            local_first = mid_lo;
            exe::call_va!(
                0x1_412e_8930,
                (
                    (&mut local_last as *mut usize) as usize,
                    (&mut local_first as *mut usize) as usize,
                    rsi,
                    param_4
                )
            );
            wr::<usize>(param_1, mid_hi);
        } else {
            local_first = cur_last;
            local_last = mid_hi;
            exe::call_va!(
                0x1_412e_8930,
                (
                    (&mut local_last as *mut usize) as usize,
                    (&mut local_first as *mut usize) as usize,
                    rsi,
                    param_4
                )
            );
            wr::<usize>(param_2, mid_lo);
        }
        r13 = rd::<usize>(param_2);
        r14 = rd::<usize>(param_1);
    }

    if r14 == r13 {
        return;
    }
    let mut rsi_ptr = r14.wrapping_add(0x40);
    if rsi_ptr == r13 {
        return;
    }
    let mut r12_ptr = r14;
    loop {
        let mut val = rd::<[u64; 8]>(rsi_ptr);
        let mut rdi_ptr = rsi_ptr;
        let c = exe::call_va!(param_4, (val.as_mut_ptr() as usize, r14) -> u8);
        if c != 0 {
            if r14 != rsi_ptr {
                let mut rax_ptr = rsi_ptr;
                let mut rcx_ptr = r12_ptr.wrapping_add(0x80);
                loop {
                    let tmp = rd::<[u64; 8]>(rax_ptr.wrapping_sub(0x40));
                    rax_ptr = rax_ptr.wrapping_sub(0x40);
                    rcx_ptr = rcx_ptr.wrapping_sub(0x40);
                    wr::<[u64; 8]>(rcx_ptr, tmp);
                    if rax_ptr == r14 {
                        break;
                    }
                }
            }
            wr::<[u64; 8]>(r14, val);
        } else {
            let mut rbx_ptr = r12_ptr;
            let mut c2 = exe::call_va!(param_4, (val.as_mut_ptr() as usize, r12_ptr) -> u8);
            while c2 != 0 {
                wr::<[u64; 8]>(rdi_ptr, rd::<[u64; 8]>(rbx_ptr));
                rdi_ptr = rbx_ptr;
                rbx_ptr = rbx_ptr.wrapping_sub(0x40);
                c2 = exe::call_va!(param_4, (val.as_mut_ptr() as usize, rbx_ptr) -> u8);
            }
            wr::<[u64; 8]>(rdi_ptr, val);
        }
        rsi_ptr = rsi_ptr.wrapping_add(0x40);
        r12_ptr = r12_ptr.wrapping_add(0x40);
        if rsi_ptr == r13 {
            break;
        }
    }
}

/// `rend_1412e8d70`
/// Original: 0x1412e8d70 (1116 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412e8d70(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: usize,
) {
    let mut r13 = rd::<usize>(param_2);
    let mut r14 = rd::<usize>(param_1);
    let mut rsi = param_3;

    while (r13.wrapping_sub(r14) as i64) >= 0x39c {
        if rsi <= 0 {
            let count: i64 = (r13.wrapping_sub(r14) as i64) / 0x1c;
            let mut hole_top: i64 = count >> 1;
            if hole_top > 0 {
                let mut rbx_ptr = r14.wrapping_add((hole_top as usize).wrapping_mul(0x1c));
                let mut first_slot: usize = r14;
                loop {
                    rbx_ptr = rbx_ptr.wrapping_sub(0x1c);
                    let mut val = rd::<[u8; 0x1c]>(rbx_ptr);
                    hole_top -= 1;
                    exe::call_va!(
                        0x1_412e_9e40,
                        (
                            (&mut first_slot as *mut usize) as usize,
                            hole_top,
                            count,
                            val.as_mut_ptr() as usize,
                            param_4
                        )
                    );
                    if hole_top <= 0 {
                        break;
                    }
                }
            }

            let rdi_beg = rd::<usize>(param_1);
            let mut rcx_end = rd::<usize>(param_2);
            while (rcx_end.wrapping_sub(rdi_beg) as i64) >= 0x38 {
                let rbx_last = rcx_end.wrapping_sub(0x1c);
                let cur_cnt = (rcx_end.wrapping_sub(rdi_beg) as i64) / 0x1c;
                rcx_end = rbx_last;
                if cur_cnt >= 2 {
                    let mut val = rd::<[u8; 0x1c]>(rbx_last);
                    wr::<[u8; 0x1c]>(rbx_last, rd::<[u8; 0x1c]>(rdi_beg));
                    let mut first_slot: usize = rdi_beg;
                    let heap_len: i64 = (rbx_last.wrapping_sub(rdi_beg) as i64) / 0x1c;
                    exe::call_va!(
                        0x1_412e_9e40,
                        (
                            (&mut first_slot as *mut usize) as usize,
                            0i64,
                            heap_len,
                            val.as_mut_ptr() as usize,
                            param_4
                        )
                    );
                }
            }
            return;
        }

        let mut local_last: usize = r13;
        let mut local_first: usize = r14;
        let mut mid_pair = [0usize; 2];
        exe::call_va!(
            0x1_412e_96c0,
            (
                mid_pair.as_mut_ptr() as usize,
                (&mut local_first as *mut usize) as usize,
                (&mut local_last as *mut usize) as usize,
                param_4
            )
        );
        let cur_last = rd::<usize>(param_2);
        let mid_lo = mid_pair[0];
        let mid_hi = mid_pair[1];
        let cur_first = rd::<usize>(param_1);
        rsi = (rsi >> 1).wrapping_add(rsi >> 2);
        let right_cnt = (cur_last.wrapping_sub(mid_hi) as i64) / 0x1c;
        let left_cnt = (mid_lo.wrapping_sub(cur_first) as i64) / 0x1c;
        if left_cnt < right_cnt {
            local_first = cur_first;
            local_last = mid_lo;
            exe::call_va!(
                0x1_412e_8d70,
                (
                    (&mut local_first as *mut usize) as usize,
                    (&mut local_last as *mut usize) as usize,
                    rsi,
                    param_4
                )
            );
            wr::<usize>(param_1, mid_hi);
        } else {
            local_last = cur_last;
            local_first = mid_hi;
            exe::call_va!(
                0x1_412e_8d70,
                (
                    (&mut local_first as *mut usize) as usize,
                    (&mut local_last as *mut usize) as usize,
                    rsi,
                    param_4
                )
            );
            wr::<usize>(param_2, mid_lo);
        }
        r13 = rd::<usize>(param_2);
        r14 = rd::<usize>(param_1);
    }

    if r14 == r13 {
        return;
    }
    let mut rbx_ptr = r14.wrapping_add(0x1c);
    if rbx_ptr == r13 {
        return;
    }
    let mut r15_ptr = r14;
    loop {
        let val = rd::<[u8; 0x1c]>(rbx_ptr);
        let mut arg_a = val;
        let mut arg_b = rd::<[u8; 0x1c]>(r14);
        let mut rsi_ptr = rbx_ptr;
        let c = exe::call_va!(
            param_4,
            (arg_a.as_mut_ptr() as usize, arg_b.as_mut_ptr() as usize) -> u8
        );
        if c != 0 {
            if r14 != rbx_ptr {
                let mut rax_ptr = rbx_ptr;
                let mut rdx_ptr = r15_ptr.wrapping_add(0x38);
                loop {
                    let tmp = rd::<[u8; 0x1c]>(rax_ptr.wrapping_sub(0x1c));
                    rdx_ptr = rdx_ptr.wrapping_sub(0x1c);
                    rax_ptr = rax_ptr.wrapping_sub(0x1c);
                    wr::<[u8; 0x1c]>(rdx_ptr, tmp);
                    if rax_ptr == r14 {
                        break;
                    }
                }
            }
            wr::<[u8; 0x1c]>(r14, val);
        } else {
            let mut rdi_ptr = r15_ptr;
            arg_a = rd::<[u8; 0x1c]>(r15_ptr);
            arg_b = val;
            let mut c2 = exe::call_va!(
                param_4,
                (arg_b.as_mut_ptr() as usize, arg_a.as_mut_ptr() as usize) -> u8
            );
            while c2 != 0 {
                wr::<[u8; 0x1c]>(rsi_ptr, rd::<[u8; 0x1c]>(rdi_ptr));
                let next_prev = rd::<[u8; 0x1c]>(rdi_ptr.wrapping_sub(0x1c));
                rsi_ptr = rdi_ptr;
                rdi_ptr = rdi_ptr.wrapping_sub(0x1c);
                arg_a = next_prev;
                arg_b = val;
                c2 = exe::call_va!(
                    param_4,
                    (arg_b.as_mut_ptr() as usize, arg_a.as_mut_ptr() as usize) -> u8
                );
            }
            wr::<[u8; 0x1c]>(rsi_ptr, val);
        }
        rbx_ptr = rbx_ptr.wrapping_add(0x1c);
        r15_ptr = r15_ptr.wrapping_add(0x1c);
        if rbx_ptr == r13 {
            break;
        }
    }
}

/// `rend_141313d50`
/// Original: 0x141313d50 (697 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_141313d50(param_1: usize, param_2: usize) -> u64 {
    let crc_val = rd::<u32>(exe::ptr::<u32>(0x1_4388_1bd0) as usize);
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4388_1bf0) as usize);
    exe::call_va!(
        0x1_401e_9b60,
        (param_2, param_1, name_ptr, crc_val, 5u32, 5u32)
    );

    const FIELDS: [(usize, i32); 13] = [
        (0x00, 0),
        (0x04, 0),
        (0x08, 0),
        (0x0c, 0),
        (0x10, 0),
        (0x14, 0),
        (0x18, 4),
        (0x1c, 5),
        (0x20, 3),
        (0x24, 3),
        (0x28, 3),
        (0x2c, 1),
        (0x30, 2),
    ];

    for (off, min_ver) in FIELDS {
        if rd::<i32>(param_2 + 0x328) >= min_ver {
            let s = rd::<usize>(param_2 + 0x338);
            let p = rd::<usize>(s + 0x120);
            wr::<u32>(param_1 + off, rd::<u32>(p));
            wr::<usize>(s + 0x120, p.wrapping_add(4));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u32>(param_1 + off, 0);
        }
    }

    exe::call_va!(0x1_401e_9a20, (param_2));
    0
}

/// `rend_1413157f0`
/// Original: 0x1413157f0 (261 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_1413157f0(param_1: usize, param_2: usize) -> u64 {
    let crc_val = rd::<u32>(exe::ptr::<u32>(0x1_4388_1be8) as usize);
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4388_1bb8) as usize);
    exe::call_va!(
        0x1_401e_9b60,
        (param_2, param_1, name_ptr, crc_val, 3u32, 2u32)
    );

    for (off, min_ver) in [(0x00usize, 0i32), (0x04, 3)] {
        if rd::<i32>(param_2 + 0x328) >= min_ver {
            let s = rd::<usize>(param_2 + 0x338);
            let p = rd::<usize>(s + 0x120);
            wr::<u32>(param_1 + off, rd::<u32>(p));
            wr::<usize>(s + 0x120, p.wrapping_add(4));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u32>(param_1 + off, 0);
        }
    }

    exe::call_va!(0x1_4131_e6b0, (param_2, param_1 + 0x08));
    exe::call_va!(0x1_4131_e870, (param_2, param_1 + 0x18));

    if rd::<i32>(param_2 + 0x328) >= 0 {
        let s = rd::<usize>(param_2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        wr::<u8>(param_1 + 0x4c, rd::<u8>(p));
        wr::<usize>(s + 0x120, p.wrapping_add(1));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u8>(param_1 + 0x4c, 0);
    }

    exe::call_va!(0x1_401e_9a20, (param_2));
    0
}

/// `rend_141317230`
/// Original: 0x141317230 (1303 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_141317230(param_1: usize, param_2: usize) -> u64 {
    let crc_val = rd::<u32>(exe::ptr::<u32>(0x1_4388_1bf8) as usize);
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4388_1c00) as usize);
    exe::call_va!(
        0x1_401e_9b60,
        (param_2, param_1, name_ptr, crc_val, 0x20u32, 0x20u32)
    );

    exe::call_va!(0x1_406d_b670, (param_2, param_1 + 0x10, 0x1au32));
    exe::call_va!(0x1_4131_f010, (param_2, param_1 + 0x28, 0x1au32));

    for (off, min_ver) in [(0x38usize, 0i32), (0x39, 5), (0x3a, 6)] {
        if rd::<i32>(param_2 + 0x328) >= min_ver {
            let s = rd::<usize>(param_2 + 0x338);
            let p = rd::<usize>(s + 0x120);
            wr::<u8>(param_1 + off, rd::<u8>(p));
            wr::<usize>(s + 0x120, p.wrapping_add(1));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u8>(param_1 + off, 0);
        }
    }

    exe::call_va!(0x1_4055_75f0, (param_2, param_1 + 0x40, 0xdu32));

    let empty_str_ptr = exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64;
    if rd::<i32>(param_2 + 0x328) >= 7 {
        exe::call_va!(0x1_424b_4da0, (rd::<usize>(param_2 + 0x338), param_1 + 0x48));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut sv: [u64; 2] = [empty_str_ptr, 0];
        exe::call_va!(
            0x1_4240_9b50,
            (param_1 + 0x48, sv.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    if rd::<i32>(param_2 + 0x328) >= 9 {
        let s = rd::<usize>(param_2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        wr::<u32>(param_1 + 0x150, rd::<u32>(p));
        wr::<usize>(s + 0x120, p.wrapping_add(4));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x150, 0xffff_ffff);
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0b {
        exe::call_va!(
            0x1_424b_4da0,
            (rd::<usize>(param_2 + 0x338), param_1 + 0x154)
        );
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut sv: [u64; 2] = [empty_str_ptr, 0];
        exe::call_va!(
            0x1_4240_9b50,
            (param_1 + 0x154, sv.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    for (off, min_ver, def_val) in [
        (0x1dcusize, 0x12i32, 0u32),
        (0x1e0, 0x0e, 0),
        (0x1e4, 0x0e, 0),
        (0x1e8, 0x1f, 8),
        (0x1ec, 0x13, 0),
        (0x1f0, 0x13, 0),
    ] {
        if rd::<i32>(param_2 + 0x328) >= min_ver {
            let s = rd::<usize>(param_2 + 0x338);
            let p = rd::<usize>(s + 0x120);
            wr::<u32>(param_1 + off, rd::<u32>(p));
            wr::<usize>(s + 0x120, p.wrapping_add(4));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u32>(param_1 + off, def_val);
        }
    }

    if rd::<i32>(param_2 + 0x328) >= 0x19 {
        exe::call_va!(0x1_4131_57f0, (param_1 + 0x1f8, param_2));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut ctx = [0u64; 0x340 / 8];
        let base = ctx.as_mut_ptr() as usize;
        let _ = exe::call_va!(0x1_4183_acc2, (base, 0u32, 0x310u64) -> usize);
        wr::<usize>(base, base + 0x10);
        wr::<u32>(base + 0x0c, 0x10);
        wr::<u32>(base + 0x318, 6);
        wr::<u64>(base + 0x320, 0);
        wr::<u32>(base + 0x328, 0xffff_ffff);
        wr::<u64>(base + 0x330, 0);
        wr::<u64>(base + 0x310, 0);
        wr::<[u64; 6]>(base + 0x10, [0; 6]);
        wr::<u32>(base + 0x08, 1);

        let slot_end = (base + 0x10).wrapping_add(0x30);
        wr::<u64>(slot_end - 0x30, 0);
        wr::<usize>(slot_end - 0x28, exe::ptr::<u8>(0x1_4309_f824) as usize);
        wr::<u64>(slot_end - 0x20, 0);
        wr::<u64>(slot_end - 0x18, 0);
        wr::<u32>(slot_end - 0x08, 0xffff_ffff);
        wr::<u64>(slot_end - 0x10, 0);

        let p320 = rd::<usize>(base + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            if inner != 0 {
                let vtbl = rd::<usize>(inner);
                let pos = exe::call_va!(rd::<usize>(vtbl + 0x10), (inner) -> u64);
                wr::<u64>(slot_end - 0x18, pos);
            }
        }

        wr::<u32>(base + 0x338, 0);
        exe::call_va!(0x1_4131_5e90, (param_1 + 0x1f8, base));
        exe::call_va!(0x1_401e_e280, (base));
    }

    if rd::<i32>(param_2 + 0x328) >= 0x11 {
        let s = rd::<usize>(param_2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        let count = rd::<i32>(p);
        wr::<usize>(s + 0x120, p.wrapping_add(4));
        if count > 2 {
            let p330 = rd::<u64>(param_2 + 0x330);
            let a1 = exe::call_va!(0x1_401e_7b40, (param_2) -> usize);
            let r9_val = rd::<u64>(a1 + 0x08);
            let a2 = exe::call_va!(0x1_401e_7b20, (param_2) -> usize);
            let r8_val = rd::<u64>(a2 + 0x08);
            let mut tmp_40 = [0u64; 2];
            let s_sub = exe::ptr::<u8>(0x1_433f_ab38) as usize;
            let exc_ptr = exe::call_va!(
                0x1_40d3_7180,
                (tmp_40.as_mut_ptr() as usize, s_sub, r8_val, r9_val, p330) -> usize
            );
            exe::call_va!(0x1_40d3_8160, (exc_ptr));
            return 0;
        }
        exe::call_va!(
            0x1_4085_3b90,
            (param_2, param_1 + 0x248, count as u32, 0x11u32)
        );
    }

    exe::call_va!(0x1_4055_75f0, (param_2, param_1 + 0x258, 0x17u32));

    if rd::<i32>(param_2 + 0x328) >= 0x1e {
        let s = rd::<usize>(param_2 + 0x338);
        let p = rd::<usize>(s + 0x120);
        wr::<u32>(param_1 + 0x260, rd::<u32>(p));
        wr::<usize>(s + 0x120, p.wrapping_add(4));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x260, 0);
    }

    exe::call_va!(0x1_401e_9a20, (param_2));
    0
}

/// `rend_141341820`
/// Original: 0x141341820 (236 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_141341820(
    param_1: usize,
    param_2: u64,
    param_3: u64,
    param_4: usize,
    param_5: u64,
) -> usize {
    let mut arg_desc: [u64; 2] = [param_5, exe::ptr::<u8>(0x1_401f_90f0) as usize as u64];
    let mut format_args: [u64; 2] = [0x0f, arg_desc.as_mut_ptr() as usize as u64];
    let mut fmt_str = rd::<[u64; 2]>(param_4);

    let mut iter_buf = [0u64; 40];
    let buf_base = iter_buf.as_mut_ptr() as usize;
    wr::<usize>(buf_base, exe::ptr::<u8>(0x1_4307_9330) as usize);
    wr::<u64>(buf_base + 0x08, param_2);
    wr::<u64>(buf_base + 0x10, 0);
    wr::<u64>(buf_base + 0x18, param_3);
    wr::<u64>(buf_base + 0x20, 0);
    wr::<u64>(buf_base + 0x28, param_3);
    wr::<u64>(buf_base + 0x30, param_2);

    exe::call_va!(
        0x1_4146_1fb0,
        (
            buf_base,
            fmt_str.as_mut_ptr() as usize,
            format_args.as_mut_ptr() as usize,
            0u32
        )
    );

    let r8_size = rd::<u64>(buf_base + 0x10);
    let rcx_limit = rd::<u64>(buf_base + 0x28);
    let rdx_count = rd::<u64>(buf_base + 0x20);
    let mut rdi_rem = if rcx_limit > rdx_count {
        rcx_limit.wrapping_sub(rdx_count)
    } else {
        0
    };
    let total_size = r8_size.wrapping_add(rdx_count);
    if r8_size < rdi_rem {
        rdi_rem = r8_size;
    }
    let mut out_ptr = rd::<u64>(buf_base + 0x30);
    if rd::<u64>(buf_base + 0x08) == out_ptr {
        out_ptr = out_ptr.wrapping_add(rdi_rem);
    }
    wr::<u64>(param_1, out_ptr);
    wr::<u64>(param_1 + 0x08, total_size);
    param_1
}

/// `rend_14134d0f0`
/// Original: 0x14134d0f0 (665 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_14134d0f0(param_1: usize) {
    let mut r14_ptr = rd::<usize>(param_1 + 0xa8);
    let word_count = (rd::<u64>(param_1 + 0xb8).wrapping_add(0x3f) & !0x3fu64) >> 6;
    if word_count != 0 {
        for word_idx in 0..word_count {
            let mut word = rd::<u64>(r14_ptr);
            if word != 0 {
                let base_bit = (word_idx as u32).wrapping_shl(6);
                loop {
                    let bit = word.trailing_zeros();
                    let elem_idx = base_bit.wrapping_add(bit);
                    if (elem_idx as u64) < rd::<u64>(param_1 + 0x90) {
                        let vec_hdr = param_1 + 0xc0;
                        let cur_len = rd::<u32>(vec_hdr + 0x08);
                        let cur_cap = rd::<u32>(vec_hdr + 0x0c);
                        if (cur_len as u64).wrapping_mul(0x14) == (cur_cap as u64).wrapping_mul(0x14)
                        {
                            let need = (cur_len as u64).wrapping_add(1);
                            let grow = ((cur_cap >> 1) as u64).wrapping_add(cur_cap as u64);
                            let new_cap = if grow < need { need } else { grow };
                            exe::call_va!(0x1_4134_dfb0, (vec_hdr, new_cap));
                        }
                        let idx0 = rd::<u32>(vec_hdr + 0x08) as usize;
                        let data_ptr = rd::<usize>(vec_hdr);
                        let dst0 = data_ptr.wrapping_add(idx0.wrapping_mul(0x14));
                        wr::<u64>(dst0, 0);
                        wr::<u64>(dst0 + 0x0c, 0);
                        wr::<u64>(dst0 + 0x08, u64::MAX);
                        wr::<u32>(dst0 + 0x10, u32::MAX);
                        wr::<u32>(vec_hdr + 0x08, rd::<u32>(vec_hdr + 0x08).wrapping_add(1));

                        let idx1 = rd::<u32>(vec_hdr + 0x08) as usize;
                        let data_ptr2 = rd::<usize>(vec_hdr);
                        let dst1 = data_ptr2.wrapping_add(idx1.wrapping_mul(0x14)).wrapping_sub(0x14);
                        wr::<u32>(dst1, elem_idx);
                        let src_arr = rd::<usize>(param_1 + 0x98);
                        let src_elem = src_arr.wrapping_add((elem_idx as usize).wrapping_mul(0x0c));
                        wr::<u64>(dst1 + 0x08, rd::<u64>(src_elem));
                        wr::<u32>(dst1 + 0x10, rd::<u32>(src_elem + 0x08));
                    }
                    let cleared = rd::<u64>(r14_ptr) ^ (1u64 << (bit & 0x3f));
                    wr::<u64>(r14_ptr, cleared);
                    word = cleared;
                    if word == 0 {
                        break;
                    }
                }
            }
            r14_ptr = r14_ptr.wrapping_add(8);
        }
    }

    exe::call_va!(
        0x1_412e_c110,
        (param_1 + 0x08, rd::<u32>(param_1 + 0xa0), param_1 + 0x58)
    );

    let esi = rd::<u32>(param_1 + 0xc8);
    if esi != 0 {
        exe::call_va!(
            0x1_412f_9440,
            (param_1 + 0x10, esi, param_1 + 0x70, param_1 + 0x88)
        );
        let byte_len = (esi as u64).wrapping_mul(0x14);
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<u64>(param_1 + 0x10), 0u64, byte_len, 2u32) -> u64
        );
        wr::<u64>(param_1 + 0x38, mapped);
        wr::<u8>(param_1 + 0x30, 1);
        wr::<u32>(param_1 + 0x20, esi);

        let old_40 = rd::<u32>(param_1 + 0x40);
        wr::<u32>(param_1 + 0x40, esi.wrapping_add(old_40));
        let dst = rd::<usize>(param_1 + 0x38).wrapping_add((old_40 as usize).wrapping_mul(0x14));
        let src = rd::<usize>(param_1 + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst, src, byte_len) -> usize);
        if rd::<u32>(param_1 + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<u64>(param_1 + 0x10), 0u32));
            wr::<u64>(param_1 + 0x38, 0);
            wr::<u8>(param_1 + 0x30, 0);
            wr::<u32>(param_1 + 0x40, 0);
        }
    }

    let r8d = rd::<u32>(param_1 + 0x20);
    if r8d != 0 && rd::<u8>(param_1 + 0x29) == 0 {
        exe::call_va!(
            0x1_4235_0fd0,
            (
                rd::<u64>(param_1 + 0x08),
                rd::<u64>(param_1 + 0x10),
                r8d,
                0x0cu32,
                0u8
            )
        );
        wr::<u32>(param_1 + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let len_c8 = rd::<u32>(param_1 + 0xc8) as u64;
        let r9_bytes = len_c8.wrapping_mul(0x14);
        let rdx_cnt = ((r9_bytes as i64) / 0x14) as u64;
        if rdx_cnt.wrapping_mul(0x14) > 0x10_0000
            && r9_bytes != (rd::<u32>(param_1 + 0xcc) as u64).wrapping_mul(0x14)
        {
            exe::call_va!(0x1_4134_dfb0, (param_1 + 0xc0, rdx_cnt));
        }
    }

    wr::<u32>(param_1 + 0xc8, 0);
}

/// `rend_141353310`
/// Original: 0x141353310 (346 bytes, Throws::No)
pub unsafe extern "win64" fn rend_141353310(param_1: usize, param_2: u32) -> u32 {
    let found = exe::call_va!(0x1_4222_9a00, (param_1 + 0xd0, param_2) -> u32);
    if found != u32::MAX {
        return found;
    }

    let esi = rd::<u32>(param_1 + 0xd0);
    let ebx = esi.wrapping_add(param_2);
    let mut rdi = rd::<u64>(param_1 + 0x58);
    let ebp: u32 = if (ebx as u64) < rdi {
        rdi as u32
    } else {
        let sum = rdi.wrapping_sub(1).wrapping_add(ebx as u64);
        sum.wrapping_sub(sum % rdi) as u32
    };

    let r12d = ebp.wrapping_sub(esi);
    let r8 = (r12d as u64).wrapping_add(rd::<u64>(param_1 + 0x90));
    if r8 >= rdi {
        let top = rdi.wrapping_add(r8).wrapping_sub(1);
        rdi = top.wrapping_sub(top % rdi);
    }

    wr::<u64>(param_1 + 0x90, r8);
    exe::call_va!(0x1_4135_4750, (param_1 + 0x98, rdi));

    let len_a0 = rd::<u32>(param_1 + 0xa0) as u64;
    let cap_a4 = rd::<u32>(param_1 + 0xa4) as u64;
    if len_a0.wrapping_mul(12) != cap_a4.wrapping_mul(12) {
        exe::call_va!(0x1_412e_c2c0, (param_1 + 0x98, len_a0));
    }

    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0xa8, rdi));
    wr::<u32>(param_1 + 0xd0, ebp);

    let rem = r12d.wrapping_sub(param_2);
    if rem != 0 {
        let slot_ptr = exe::call_va!(0x1_412f_8d90, (param_1 + 0xd8) -> usize);
        wr::<u32>(slot_ptr, ebx);
        wr::<u32>(slot_ptr + 0x04, rem);
        let mut cur = ebx;
        if cur < ebp {
            loop {
                let bits_arr = rd::<usize>(param_1 + 0xa8);
                let word_addr = bits_arr.wrapping_add(((cur >> 6) as usize) * 8);
                wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << (cur & 0x3f)));
                cur = cur.wrapping_add(1);
                let end = rd::<u32>(slot_ptr).wrapping_add(rd::<u32>(slot_ptr + 0x04));
                if cur >= end {
                    break;
                }
            }
        }
    }
    esi
}

/// `rend_141353a10`
/// Original: 0x141353a10 (346 bytes, Throws::No)
pub unsafe extern "win64" fn rend_141353a10(param_1: usize, param_2: u32) -> u32 {
    let found = exe::call_va!(0x1_4222_9a00, (param_1 + 0xd0, param_2) -> u32);
    if found != u32::MAX {
        return found;
    }

    let esi = rd::<u32>(param_1 + 0xd0);
    let ebx = esi.wrapping_add(param_2);
    let mut rdi = rd::<u64>(param_1 + 0x58);
    let ebp: u32 = if (ebx as u64) < rdi {
        rdi as u32
    } else {
        let sum = rdi.wrapping_sub(1).wrapping_add(ebx as u64);
        sum.wrapping_sub(sum % rdi) as u32
    };

    let r12d = ebp.wrapping_sub(esi);
    let r8 = (r12d as u64).wrapping_add(rd::<u64>(param_1 + 0x90));
    if r8 >= rdi {
        let top = rdi.wrapping_add(r8).wrapping_sub(1);
        rdi = top.wrapping_sub(top % rdi);
    }

    wr::<u64>(param_1 + 0x90, r8);
    exe::call_va!(0x1_4135_48e0, (param_1 + 0x98, rdi));

    let len_a0 = rd::<u32>(param_1 + 0xa0) as u64;
    let cap_a4 = rd::<u32>(param_1 + 0xa4) as u64;
    if len_a0.wrapping_mul(8) != cap_a4.wrapping_mul(8) {
        exe::call_va!(0x1_4087_46b0, (param_1 + 0x98, len_a0));
    }

    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0xa8, rdi));
    wr::<u32>(param_1 + 0xd0, ebp);

    let rem = r12d.wrapping_sub(param_2);
    if rem != 0 {
        let slot_ptr = exe::call_va!(0x1_412f_8d90, (param_1 + 0xd8) -> usize);
        wr::<u32>(slot_ptr, ebx);
        wr::<u32>(slot_ptr + 0x04, rem);
        let mut cur = ebx;
        if cur < ebp {
            loop {
                let bits_arr = rd::<usize>(param_1 + 0xa8);
                let word_addr = bits_arr.wrapping_add(((cur >> 6) as usize) * 8);
                wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << (cur & 0x3f)));
                cur = cur.wrapping_add(1);
                let end = rd::<u32>(slot_ptr).wrapping_add(rd::<u32>(slot_ptr + 0x04));
                if cur >= end {
                    break;
                }
            }
        }
    }
    esi
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_1412dfdd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUF: [u64; 16] = [0; 16];
    static mut ALLOC_ARGS: (usize, u64, u64, u64) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_7280(heap: usize, sz: u64, a3: u64, a4: u64) -> usize {
        ALLOC_ARGS = (heap, sz, a3, a4);
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_7280, mock_7280 as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let run = |f: F, use_freelist: bool| -> ([u64; 16], usize, u32, (usize, u64, u64, u64)) {
        ALLOC_BUF = [0xaaaa_bbbb_cccc_ddddu64; 16];
        ALLOC_ARGS = (0, 0, 0, 0);

        let mut free_obj = [0x1111_2222_3333_4444u64; 16];
        let fop = free_obj.as_mut_ptr() as usize;
        wr::<usize>(fop, 0x9999_8888);

        let mut pool = [0u8; 0x20];
        let pp = pool.as_mut_ptr() as usize;
        wr::<usize>(pp + 0x10, if use_freelist { fop } else { 0 });
        wr::<u32>(pp + 0x18, 7);

        let mut heap = [0u8; 0x90];
        let hp = heap.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x88, pp);

        let mut tls_blk = vec![0u8; 0x6360];
        let tp = tls_blk.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x6348, hp);
        wr::<usize>(tls_slot, tp);

        let ret = f();
        wr::<usize>(tls_slot, saved_tls0);

        let out_obj = rd::<[u64; 16]>(ret);
        let mut norm_args = ALLOC_ARGS;
        if norm_args.0 != 0 {
            norm_args.0 = norm_args.0.wrapping_sub(hp);
        }
        (out_obj, rd::<usize>(pp + 0x10), rd::<u32>(pp + 0x18), norm_args)
    };

    for freelist in [true, false] {
        let o = run(f_o, freelist);
        let n = run(f_n, freelist);
        crate::diff_eq!(o, n, "1412dfdd0 freelist={}", freelist);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412dff90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HDR_CALL: (u64, u64, u32, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_b0b0(
        _p2: usize,
        _p1: usize,
        name_ptr: u64,
        p4: u64,
        v1: u32,
        v2: u32,
    ) {
        HDR_CALL = (name_ptr, p4, v1, v2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_b0b0, mock_b0b0 as *const ())?;

    let run = |f: F, pair_cnt: u64| -> (u64, u32, u32, u32, (u64, u64, u32, u32)) {
        HDR_CALL = (0, 0, 0, 0);

        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        for (i, b) in p1.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(13).wrapping_add(5);
        }
        wr::<u64>(p1p + 0x60, pair_cnt);
        wr::<u32>(p1p + 0x20, 100);
        wr::<u32>(p1p + 0x24, 250);
        wr::<u32>(p1p + 0x28, 300);
        wr::<u32>(p1p + 0x2c, 900);

        let mut stack_frames = [0u8; 0x90];
        let sfp = stack_frames.as_mut_ptr() as usize;
        wr::<u32>(sfp + 2 * 0x30 - 8, 0x42);

        let mut p2 = [0u8; 0x350];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, sfp);
        wr::<u32>(p2p + 0x08, 2);
        wr::<u32>(p2p + 0x328, 8);
        wr::<u32>(p2p + 0x338, 0x1234_5678);

        let ret = f(p1p, p2p, 1, 0xfeed_cafe_1234);
        (
            ret,
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x328),
            rd::<u32>(p2p + 0x338),
            HDR_CALL,
        )
    };

    for cnt in [0u64, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1412dff90 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412e0930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: (u64, u32, u32, u32, u32) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_9b60(
        _p2: usize,
        _p1: usize,
        name: u64,
        crc: u32,
        v1: u32,
        v2: u32,
    ) {
        CALL_LOG = (name, crc, v1, v2, 0);
    }
    unsafe extern "win64" fn mock_9a20(_p2: usize) {
        CALL_LOG.4 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9b60, mock_9b60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_9a20, mock_9a20 as *const ())?;

    let run = |f: F, ver: i32| -> (u64, [u64; 16], usize, (u64, u32, u32, u32, u32)) {
        CALL_LOG = (0, 0, 0, 0, 0);

        let mut stream_bytes = [0u8; 128];
        let sbp = stream_bytes.as_mut_ptr() as usize;
        for (i, b) in stream_bytes.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        // At offset 16 (after 4 + 2*4 + 4 = 16 bytes), place array length = 2
        wr::<u32>(sbp + 16, 2);
        wr::<u32>(sbp + 20, 100);
        wr::<u32>(sbp + 24, 50);
        wr::<u32>(sbp + 28, 200);
        wr::<u32>(sbp + 32, 75);

        let mut sub_338 = [0u8; 0x130];
        let sp = sub_338.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x120, sbp);

        let mut p1 = [0x5555_5555_5555_5555u64; 16];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x60, 0);

        let mut p2 = [0u8; 0x380];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, sp);
        wr::<u8>(p2p + 0x369, 1);

        let ret = f(p1p, p2p);
        let consumed = rd::<usize>(sp + 0x120).wrapping_sub(sbp);
        (ret, p1, consumed, CALL_LOG)
    };

    for ver in [8i32, 1] {
        let o = run(f_o, ver);
        let n = run(f_n, ver);
        crate::diff_eq!(o, n, "1412e0930 ver={}", ver);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412e8930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn cmp_u64_64(a: usize, b: usize) -> u8 {
        if rd::<u64>(a) < rd::<u64>(b) {
            1
        } else {
            0
        }
    }

    let run = |f: F, count: usize, ideal: i64| -> Vec<u64> {
        let mut data = vec![0u64; count * 8];
        for i in 0..count {
            let key = ((i * 17 + 11) % count) as u64;
            for j in 0..8 {
                data[i * 8 + j] = key * 100 + j as u64;
            }
        }
        let mut first = data.as_mut_ptr() as usize;
        let mut last = first.wrapping_add(count * 64);
        f(
            (&mut first as *mut usize) as usize,
            (&mut last as *mut usize) as usize,
            ideal,
            cmp_u64_64 as *const () as usize,
        );
        data
    };

    // Test insertion sort (<= 32 elements) and heapsort (> 32 elements with ideal = 0)
    for (cnt, ideal) in [(6usize, 10i64), (36, 0)] {
        let o = run(f_o, cnt, ideal);
        let n = run(f_n, cnt, ideal);
        crate::diff_eq!(o, n, "1412e8930 cnt={} ideal={}", cnt, ideal);
    }
    Ok(2)
}

pub unsafe fn diff_rend_1412e8d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn cmp_28(a: usize, b: usize) -> u8 {
        if rd::<u32>(a) < rd::<u32>(b) {
            1
        } else {
            0
        }
    }

    let run = |f: F, count: usize, ideal: i64| -> Vec<u32> {
        let mut data = vec![0u32; count * 7];
        for i in 0..count {
            let key = ((i * 19 + 7) % count) as u32;
            for j in 0..7 {
                data[i * 7 + j] = key * 10 + j as u32;
            }
        }
        let mut first = data.as_mut_ptr() as usize;
        let mut last = first.wrapping_add(count * 0x1c);
        f(
            (&mut first as *mut usize) as usize,
            (&mut last as *mut usize) as usize,
            ideal,
            cmp_28 as *const () as usize,
        );
        data
    };

    for (cnt, ideal) in [(6usize, 10i64), (36, 0)] {
        let o = run(f_o, cnt, ideal);
        let n = run(f_n, cnt, ideal);
        crate::diff_eq!(o, n, "1412e8d70 cnt={} ideal={}", cnt, ideal);
    }
    Ok(2)
}

pub unsafe fn diff_rend_141313d50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: (u64, u32, u32, u32, u32) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_9b60(
        _p2: usize,
        _p1: usize,
        name: u64,
        crc: u32,
        v1: u32,
        v2: u32,
    ) {
        CALL_LOG = (name, crc, v1, v2, 0);
    }
    unsafe extern "win64" fn mock_9a20(_p2: usize) {
        CALL_LOG.4 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9b60, mock_9b60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_9a20, mock_9a20 as *const ())?;

    let run = |f: F, ver: i32| -> (u64, [u32; 13], usize, (u64, u32, u32, u32, u32)) {
        CALL_LOG = (0, 0, 0, 0, 0);

        let mut words = [0u32; 16];
        for (i, w) in words.iter_mut().enumerate() {
            *w = 0x100 + i as u32;
        }
        let wp = words.as_mut_ptr() as usize;

        let mut sub_338 = [0u8; 0x130];
        let sp = sub_338.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x120, wp);

        let mut p1 = [0xdead_beefu32; 13];
        let mut p2 = [0u8; 0x380];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, sp);
        wr::<u8>(p2p + 0x369, 1);

        let ret = f(p1.as_mut_ptr() as usize, p2p);
        let consumed = rd::<usize>(sp + 0x120).wrapping_sub(wp);
        (ret, p1, consumed, CALL_LOG)
    };

    for ver in [5i32, 2, -1] {
        let o = run(f_o, ver);
        let n = run(f_n, ver);
        crate::diff_eq!(o, n, "141313d50 ver={}", ver);
    }
    Ok(3)
}

pub unsafe fn diff_rend_1413157f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SUB_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_9b60(
        _p2: usize,
        _p1: usize,
        _name: u64,
        _crc: u32,
        _v1: u32,
        _v2: u32,
    ) {
        SUB_CALLS += 1;
    }
    unsafe extern "win64" fn mock_e6b0(_p2: usize, out: usize) {
        SUB_CALLS += 10;
        wr::<u64>(out, 0xaaaa_1111);
    }
    unsafe extern "win64" fn mock_e870(_p2: usize, out: usize) {
        SUB_CALLS += 100;
        wr::<u64>(out, 0xbbbb_2222);
    }
    unsafe extern "win64" fn mock_9a20(_p2: usize) {
        SUB_CALLS += 1000;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9b60, mock_9b60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4131_e6b0, mock_e6b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4131_e870, mock_e870 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_9a20, mock_9a20 as *const ())?;

    let run = |f: F, ver: i32| -> (u64, [u8; 0x50], usize, u32) {
        SUB_CALLS = 0;

        let stream_bytes: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let sbp = stream_bytes.as_ptr() as usize;

        let mut sub_338 = [0u8; 0x130];
        let sp = sub_338.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x120, sbp);

        let mut p1 = [0xffu8; 0x50];
        let mut p2 = [0u8; 0x380];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, sp);
        wr::<u8>(p2p + 0x369, 1);

        let ret = f(p1.as_mut_ptr() as usize, p2p);
        let consumed = rd::<usize>(sp + 0x120).wrapping_sub(sbp);
        (ret, p1, consumed, SUB_CALLS)
    };

    for ver in [3i32, -1] {
        let o = run(f_o, ver);
        let n = run(f_n, ver);
        crate::diff_eq!(o, n, "1413157f0 ver={}", ver);
    }
    Ok(2)
}

pub unsafe fn diff_rend_141317230(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: u64 = 0;

    unsafe extern "win64" fn mock_9b60(
        _p2: usize,
        _p1: usize,
        _name: u64,
        _crc: u32,
        v1: u32,
        v2: u32,
    ) {
        CALL_TRACE = CALL_TRACE.wrapping_add((v1 + v2) as u64);
    }
    unsafe extern "win64" fn mock_b670(_p2: usize, dst: usize, ver: u32) {
        wr::<u32>(dst, ver);
    }
    unsafe extern "win64" fn mock_f010(_p2: usize, dst: usize, ver: u32) {
        wr::<u32>(dst, ver + 0x100);
    }
    unsafe extern "win64" fn mock_75f0(_p2: usize, dst: usize, ver: u32) {
        wr::<u32>(dst, ver + 0x200);
    }
    unsafe extern "win64" fn mock_4da0(_s: usize, dst: usize) {
        wr::<u32>(dst, 0x7777);
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, _sv: usize, _a3: u64, _a4: u64) {
        wr::<u32>(dst, 0x8888);
    }
    unsafe extern "win64" fn mock_57f0(dst: usize, _p2: usize) -> u64 {
        wr::<u32>(dst, 0x9999);
        0
    }
    unsafe extern "win64" fn mock_5e90(dst: usize, ctx: usize) {
        wr::<u32>(dst, rd::<u32>(ctx + 0x318));
    }
    unsafe extern "win64" fn mock_e280(_ctx: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_add(0x1000);
    }
    unsafe extern "win64" fn mock_3b90(_p2: usize, dst: usize, cnt: u32, ver: u32) {
        wr::<u32>(dst, cnt * 100 + ver);
    }
    unsafe extern "win64" fn mock_9a20(_p2: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9b60, mock_9b60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_406d_b670, mock_b670 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4131_f010, mock_f010 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4055_75f0, mock_75f0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_424b_4da0, mock_4da0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4131_57f0, mock_57f0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4131_5e90, mock_5e90 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4085_3b90, mock_3b90 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401e_9a20, mock_9a20 as *const ())?;

    let run = |f: F, ver: i32| -> (u64, Vec<u8>, usize, u64) {
        CALL_TRACE = 0;

        let mut stream_data = [0u8; 64];
        for (i, b) in stream_data.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        let sdp = stream_data.as_mut_ptr() as usize;
        // At offset 31 (after 3 u8s + 7 u32s = 31 bytes), place count = 2 for 0x140853b90
        wr::<i32>(sdp + 31, 2);
        wr::<u32>(sdp + 35, 0x1234_5678);

        let mut sub_338 = [0u8; 0x130];
        let sp = sub_338.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x120, sdp);

        let mut p1 = vec![0xeeu8; 0x270];
        let mut p2 = [0u8; 0x380];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, sp);
        wr::<u8>(p2p + 0x369, 1);

        let ret = f(p1.as_mut_ptr() as usize, p2p);
        let consumed = rd::<usize>(sp + 0x120).wrapping_sub(sdp);
        (ret, p1, consumed, CALL_TRACE)
    };

    for ver in [0x20i32, 4] {
        let o = run(f_o, ver);
        let n = run(f_n, ver);
        crate::diff_eq!(o, n, "141317230 ver={}", ver);
    }
    Ok(2)
}

pub unsafe fn diff_rend_141341820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: (u64, u64, u64, u64, u32) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_1fb0(buf: usize, fmt_ptr: usize, args_ptr: usize, flag: u32) {
        let f0 = rd::<u64>(fmt_ptr);
        let f1 = rd::<u64>(fmt_ptr + 0x08);
        let a0 = rd::<u64>(args_ptr);
        let desc_ptr = rd::<usize>(args_ptr + 0x08);
        let p5 = rd::<u64>(desc_ptr);
        CAPTURED = (f0, f1, a0, p5, flag);
        // Simulate formatting 5 chars with 2 already counted
        wr::<u64>(buf + 0x10, 5);
        wr::<u64>(buf + 0x20, 2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4146_1fb0, mock_1fb0 as *const ())?;

    let run = |f: F, cap: u64| -> ([u64; 2], (u64, u64, u64, u64, u32)) {
        CAPTURED = (0, 0, 0, 0, 0);
        let mut out = [0u64; 2];
        let fmt_str: [u64; 2] = [0x1111_2222, 8];
        let ret = f(
            out.as_mut_ptr() as usize,
            0x5000_0000,
            cap,
            fmt_str.as_ptr() as usize,
            0x9999_aaaa,
        );
        assert_eq!(ret, out.as_ptr() as usize);
        (out, CAPTURED)
    };

    for cap in [10u64, 4, 1] {
        let o = run(f_o, cap);
        let n = run(f_n, cap);
        crate::diff_eq!(o, n, "141341820 cap={}", cap);
    }
    Ok(3)
}

pub unsafe fn diff_rend_14134d0f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MAPPED_BUF: [u8; 64] = [0; 64];
    static mut CALL_TRACE: (u32, u32, u64, u32, u32) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_c110(_rcx: usize, edx: u32, _r8: usize) {
        CALL_TRACE.0 = edx;
    }
    unsafe extern "win64" fn mock_9440(_rcx: usize, edx: u32, _r8: usize, _r9: usize) {
        CALL_TRACE.1 = edx;
    }
    unsafe extern "win64" fn mock_c560(_rcx: u64, _rdx: u64, r8: u64, r9: u32) -> u64 {
        CALL_TRACE.2 = r8;
        CALL_TRACE.3 = r9;
        core::ptr::addr_of_mut!(MAPPED_BUF) as usize as u64
    }
    unsafe extern "win64" fn mock_d6f0(_rcx: u64, edx: u32) {
        CALL_TRACE.4 = edx + 1;
    }
    unsafe extern "win64" fn mock_0fd0(_rcx: u64, _rdx: u64, _r8: u32, _r9: u32, _s5: u8) {}

    let _m1 = crate::difftest::Mock::new(0x1_412e_c110, mock_c110 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412f_9440, mock_9440 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4235_0fd0, mock_0fd0 as *const ())?;

    let run = |f: F| -> ([u64; 2], [u8; 40], [u8; 64], u32, (u32, u32, u64, u32, u32)) {
        MAPPED_BUF = [0; 64];
        CALL_TRACE = (0, 0, 0, 0, 0);

        let mut bits: [u64; 2] = [0b101, 0];
        let src_elems: [u32; 9] = [10, 11, 12, 20, 21, 22, 30, 31, 32];
        let mut vec_buf = [0u8; 40];

        let mut obj = [0u8; 0xe0];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x10, 0xabcd_0001);
        wr::<u64>(op + 0x90, 3);
        wr::<usize>(op + 0x98, src_elems.as_ptr() as usize);
        wr::<u32>(op + 0xa0, 77);
        wr::<usize>(op + 0xa8, bits.as_mut_ptr() as usize);
        wr::<u64>(op + 0xb8, 64);
        wr::<usize>(op + 0xc0, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(op + 0xc8, 0);
        wr::<u32>(op + 0xcc, 4);

        f(op);
        (
            bits,
            vec_buf,
            MAPPED_BUF,
            rd::<u32>(op + 0xc8),
            CALL_TRACE,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14134d0f0 out");
    Ok(1)
}

pub unsafe fn diff_rend_141353310(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FREE_SLOT: [u32; 2] = [0; 2];
    static mut GROW_CALLS: (u64, u64, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_9a00(_rcx: usize, req: u32) -> u32 {
        if req == 99 {
            42
        } else {
            u32::MAX
        }
    }
    unsafe extern "win64" fn mock_4750(_rcx: usize, cap: u64) {
        GROW_CALLS.0 = cap;
    }
    unsafe extern "win64" fn mock_c2c0(_rcx: usize, len: u64) {
        GROW_CALLS.1 = len;
    }
    unsafe extern "win64" fn mock_f1e0(_rcx: usize, cap: u64) {
        GROW_CALLS.2 = cap;
    }
    unsafe extern "win64" fn mock_8d90(_rcx: usize) -> usize {
        core::ptr::addr_of_mut!(FREE_SLOT) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_9a00, mock_9a00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4135_4750, mock_4750 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412e_c2c0, mock_c2c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_412f_8d90, mock_8d90 as *const ())?;

    let run = |f: F, req: u32| -> (u32, u64, u32, [u64; 2], [u32; 2], (u64, u64, u64)) {
        FREE_SLOT = [0; 2];
        GROW_CALLS = (0, 0, 0);
        let mut bits = [0u64; 2];
        let mut obj = [0u8; 0xf0];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x58, 8);
        wr::<u64>(op + 0x90, 2);
        wr::<u32>(op + 0xa0, 3);
        wr::<u32>(op + 0xa4, 5);
        wr::<usize>(op + 0xa8, bits.as_mut_ptr() as usize);
        wr::<u32>(op + 0xd0, 1);

        let ret = f(op, req);
        (
            ret,
            rd::<u64>(op + 0x90),
            rd::<u32>(op + 0xd0),
            bits,
            FREE_SLOT,
            GROW_CALLS,
        )
    };

    for req in [99u32, 3, 10] {
        let o = run(f_o, req);
        let n = run(f_n, req);
        crate::diff_eq!(o, n, "141353310 req={}", req);
    }
    Ok(3)
}

pub unsafe fn diff_rend_141353a10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FREE_SLOT: [u32; 2] = [0; 2];
    static mut GROW_CALLS: (u64, u64, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_9a00(_rcx: usize, req: u32) -> u32 {
        if req == 99 {
            42
        } else {
            u32::MAX
        }
    }
    unsafe extern "win64" fn mock_48e0(_rcx: usize, cap: u64) {
        GROW_CALLS.0 = cap;
    }
    unsafe extern "win64" fn mock_46b0(_rcx: usize, len: u64) {
        GROW_CALLS.1 = len;
    }
    unsafe extern "win64" fn mock_f1e0(_rcx: usize, cap: u64) {
        GROW_CALLS.2 = cap;
    }
    unsafe extern "win64" fn mock_8d90(_rcx: usize) -> usize {
        core::ptr::addr_of_mut!(FREE_SLOT) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_9a00, mock_9a00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4135_48e0, mock_48e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4087_46b0, mock_46b0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_412f_8d90, mock_8d90 as *const ())?;

    let run = |f: F, req: u32| -> (u32, u64, u32, [u64; 2], [u32; 2], (u64, u64, u64)) {
        FREE_SLOT = [0; 2];
        GROW_CALLS = (0, 0, 0);
        let mut bits = [0u64; 2];
        let mut obj = [0u8; 0xf0];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x58, 8);
        wr::<u64>(op + 0x90, 2);
        wr::<u32>(op + 0xa0, 3);
        wr::<u32>(op + 0xa4, 5);
        wr::<usize>(op + 0xa8, bits.as_mut_ptr() as usize);
        wr::<u32>(op + 0xd0, 1);

        let ret = f(op, req);
        (
            ret,
            rd::<u64>(op + 0x90),
            rd::<u32>(op + 0xd0),
            bits,
            FREE_SLOT,
            GROW_CALLS,
        )
    };

    for req in [99u32, 3, 10] {
        let o = run(f_o, req);
        let n = run(f_n, req);
        crate::diff_eq!(o, n, "141353a10 req={}", req);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_412d_fdd0, "rend_1412dfdd0", rend::cauldron_hot_y1::rend_1412dfdd0 as *const (), rend::cauldron_hot_y1::diff_rend_1412dfdd0),
// Replacement::verified(0x1_412d_ff90, "rend_1412dff90", rend::cauldron_hot_y1::rend_1412dff90 as *const (), rend::cauldron_hot_y1::diff_rend_1412dff90),
// Replacement::verified_throws(0x1_412e_0930, "rend_1412e0930", rend::cauldron_hot_y1::rend_1412e0930 as *const (), Throws::Discard, rend::cauldron_hot_y1::diff_rend_1412e0930),
// Replacement::verified(0x1_412e_8930, "rend_1412e8930", rend::cauldron_hot_y1::rend_1412e8930 as *const (), rend::cauldron_hot_y1::diff_rend_1412e8930),
// Replacement::verified(0x1_412e_8d70, "rend_1412e8d70", rend::cauldron_hot_y1::rend_1412e8d70 as *const (), rend::cauldron_hot_y1::diff_rend_1412e8d70),
// Replacement::verified_throws(0x1_4131_3d50, "rend_141313d50", rend::cauldron_hot_y1::rend_141313d50 as *const (), Throws::Discard, rend::cauldron_hot_y1::diff_rend_141313d50),
// Replacement::verified_throws(0x1_4131_57f0, "rend_1413157f0", rend::cauldron_hot_y1::rend_1413157f0 as *const (), Throws::Discard, rend::cauldron_hot_y1::diff_rend_1413157f0),
// Replacement::verified_throws(0x1_4131_7230, "rend_141317230", rend::cauldron_hot_y1::rend_141317230 as *const (), Throws::Caught, rend::cauldron_hot_y1::diff_rend_141317230),
// Replacement::verified_throws(0x1_4134_1820, "rend_141341820", rend::cauldron_hot_y1::rend_141341820 as *const (), Throws::Caught, rend::cauldron_hot_y1::diff_rend_141341820),
// Replacement::verified_throws(0x1_4134_d0f0, "rend_14134d0f0", rend::cauldron_hot_y1::rend_14134d0f0 as *const (), Throws::Caught, rend::cauldron_hot_y1::diff_rend_14134d0f0),
// Replacement::verified(0x1_4135_3310, "rend_141353310", rend::cauldron_hot_y1::rend_141353310 as *const (), rend::cauldron_hot_y1::diff_rend_141353310),
// Replacement::verified(0x1_4135_3a10, "rend_141353a10", rend::cauldron_hot_y1::rend_141353a10 as *const (), rend::cauldron_hot_y1::diff_rend_141353a10),
