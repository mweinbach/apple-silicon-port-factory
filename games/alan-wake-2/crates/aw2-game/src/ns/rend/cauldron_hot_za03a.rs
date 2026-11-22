//! Decompiled hot functions shard ZA03A (`0x140f64cf0..0x140f8b340`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn extract_inplace_string_view(s_ptr: usize) -> [u64; 2] {
    let mut data_ptr = s_ptr.wrapping_add(0x08);
    let b3 = rd::<u8>(s_ptr + 0x03) as u32;
    let inline_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap_bits = (rd::<u32>(s_ptr) & 0x00ff_ffff) << 3;
    if cap_bits > inline_bits {
        data_ptr = rd::<usize>(data_ptr);
    }
    let len = rd::<u32>(s_ptr + 0x04) as u64;
    [data_ptr as u64, len]
}

/// `unknown_140f64cf0`
/// Original: 0x140f64cf0 (223 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140f64cf0(param_1: usize, param_2: usize, param_3: usize) {
    let vtbl_1 = rd::<usize>(param_1);
    let fn_240 = rd::<u64>(vtbl_1 + 0x240);
    let fn_218 = rd::<u64>(vtbl_1 + 0x218);
    let rbx = exe::call_va!(fn_218, (param_1, param_2, param_3) -> usize);
    let mut u_var4: u64 = 0;
    if rbx != 0 {
        let fn_78 = rd::<u64>(rd::<usize>(rbx) + 0x78);
        let r15d = exe::call_va!(fn_78, (rbx) -> u32);
        let fn_70 = rd::<u64>(rd::<usize>(rbx) + 0x70);
        exe::call_va!(fn_70, (rbx, 2u32));
        let fn_28 = rd::<u64>(rd::<usize>(rbx) + 0x28);
        exe::call_va!(fn_28, (rbx));
        if param_3 != 0 {
            let fn_28_b = rd::<u64>(rd::<usize>(rbx) + 0x28);
            exe::call_va!(fn_28_b, (rbx));
            exe::call_va!(0x1_41e1_6720, (rbx, param_3));
            let fn_48 = rd::<u64>(rd::<usize>(rbx) + 0x48);
            exe::call_va!(fn_48, (rbx, param_3));
        } else {
            exe::call_va!(0x1_41e1_6720, (rbx, 0usize));
        }
        let fn_48_2 = rd::<u64>(rd::<usize>(rbx) + 0x48);
        exe::call_va!(fn_48_2, (rbx, param_3));
        let fn_70_2 = rd::<u64>(rd::<usize>(rbx) + 0x70);
        exe::call_va!(fn_70_2, (rbx, r15d));
        let fn_40 = rd::<u64>(rd::<usize>(rbx) + 0x40);
        u_var4 = exe::call_va!(fn_40, (rbx) -> u64);
    }
    let cb = exe::ptr::<u8>(0x1_40f6_ba40) as usize;
    exe::call_va!(fn_240, (param_1, param_2, param_3, u_var4, cb));
}

/// `unknown_140f65220`
/// Original: 0x140f65220 (466 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140f65220(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> usize {
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    let len = rd::<u32>(param_1 + 0x08) as u64;
    if len.wrapping_mul(0xd0) == cap.wrapping_mul(0xd0) {
        let req = len.wrapping_add(1);
        let mut new_cap = (cap >> 1).wrapping_add(cap);
        if new_cap < req {
            new_cap = req;
        }
        exe::call_va!(0x1_40f6_abe0, (param_1, new_cap));
    }

    let cur_len = rd::<u32>(param_1 + 0x08) as usize;
    let dst = rd::<usize>(param_1).wrapping_add(cur_len.wrapping_mul(0xd0));
    let r14d = rd::<u32>(param_6);

    let mut view_5 = extract_inplace_string_view(param_5);
    let mut view_4 = extract_inplace_string_view(param_4);
    let mut view_3 = extract_inplace_string_view(param_3);
    let mut copy_2 = rd::<[u64; 2]>(param_2);

    exe::call_va!(
        0x1_41e1_3a20,
        (
            dst,
            copy_2.as_mut_ptr() as usize,
            view_3.as_mut_ptr() as usize,
            view_4.as_mut_ptr() as usize,
            view_5.as_mut_ptr() as usize,
            r14d,
        )
    );

    let next_len = rd::<u32>(param_1 + 0x08).wrapping_add(1);
    wr::<u32>(param_1 + 0x08, next_len);
    rd::<usize>(param_1)
        .wrapping_sub(0xd0)
        .wrapping_add((next_len as usize).wrapping_mul(0xd0))
}

/// `unknown_140f69070`
/// Original: 0x140f69070 (524 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f69070(param_1: u64, param_2: usize, param_3: usize) {
    match param_1 as u32 {
        0 => {
            let rbx = rd::<usize>(param_2);
            let tag = rd::<i8>(rbx + 0x48);
            if tag == 2 {
                exe::call_va!(0x1_4240_ab10, (rbx));
            }
            wr::<u8>(rbx + 0x48, 0xff);
        }
        1 => {
            let rcx = rd::<usize>(param_2);
            if rd::<u8>(rcx + 0x48) == 0 {
                wr::<u8>(rcx, rd::<u8>(param_3));
            } else {
                exe::call_va!(0x1_40f4_5520, (rcx, param_2, param_3));
                let rcx2 = rd::<usize>(param_2);
                wr::<u8>(rcx2, rd::<u8>(param_3));
                let rax = rd::<usize>(param_2);
                wr::<u8>(rax + 0x48, 0);
            }
        }
        2 => {
            let rcx = rd::<usize>(param_2);
            if rd::<u8>(rcx + 0x48) == 1 {
                wr::<u64>(rcx, rd::<u64>(param_3));
            } else {
                exe::call_va!(0x1_40f4_5520, (rcx, param_2, param_3));
                let rcx2 = rd::<usize>(param_2);
                wr::<u64>(rcx2, rd::<u64>(param_3));
                wr::<u8>(rcx2 + 0x48, 1);
            }
        }
        3 => {
            let rcx = rd::<usize>(param_2);
            if rd::<u8>(rcx + 0x48) == 2 {
                exe::call_va!(0x1_4240_aa00, (rcx, param_3));
            } else {
                exe::call_va!(0x1_40f4_5520, (rcx, param_2, param_3));
                let rcx2 = rd::<usize>(param_2);
                let hdr = (rd::<u32>(rcx2) & 0xff00_0008) | 0x08;
                wr::<u32>(rcx2, hdr);
                wr::<u8>(rcx2 + 0x03, 0x08);
                wr::<u32>(rcx2 + 0x04, 0);
                wr::<u8>(rcx2 + 0x08, 0);
                exe::call_va!(0x1_4240_aa00, (rcx2, param_3));
                let rax = rd::<usize>(param_2);
                wr::<u8>(rax + 0x48, 2);
            }
        }
        4 => {
            let rcx = rd::<usize>(param_2);
            if rd::<u8>(rcx + 0x48) == 3 {
                wr::<u64>(rcx, rd::<u64>(param_3));
            } else {
                exe::call_va!(0x1_40f4_5520, (rcx, param_2, param_3));
                let rcx2 = rd::<usize>(param_2);
                wr::<u64>(rcx2, rd::<u64>(param_3));
                let rax = rd::<usize>(param_2);
                wr::<u8>(rax + 0x48, 3);
            }
        }
        5 => {
            let rcx = rd::<usize>(param_2);
            if rd::<u8>(rcx + 0x48) == 4 {
                wr::<u64>(rcx, rd::<u64>(param_3));
                wr::<u32>(rcx + 0x08, rd::<u32>(param_3 + 0x08));
            } else {
                exe::call_va!(0x1_40f4_5520, (rcx, param_2, param_3));
                let rcx2 = rd::<usize>(param_2);
                wr::<u64>(rcx2, rd::<u64>(param_3));
                wr::<u32>(rcx2 + 0x08, rd::<u32>(param_3 + 0x08));
                let rax = rd::<usize>(param_2);
                wr::<u8>(rax + 0x48, 4);
            }
        }
        6 => {
            let rcx = rd::<usize>(param_2);
            if rd::<u8>(rcx + 0x48) == 5 {
                wr::<[u64; 2]>(rcx, rd::<[u64; 2]>(param_3));
            } else {
                exe::call_va!(0x1_40f4_5520, (rcx, param_2, param_3));
                let rax = rd::<usize>(param_2);
                wr::<[u64; 2]>(rax, rd::<[u64; 2]>(param_3));
                let rax2 = rd::<usize>(param_2);
                wr::<u8>(rax2 + 0x48, 5);
            }
        }
        _ => {
            let rbx = rd::<usize>(param_2);
            wr::<u8>(rbx + 0x48, 0xff);
        }
    }
}

/// `unknown_140f7d7e0`
/// Original: 0x140f7d7e0 (157 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f7d7e0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut rdx = rd::<usize>(param_2);
    let r8 = rd::<usize>(param_3);
    if rdx == r8 {
        wr::<usize>(param_1, r8);
        return param_1;
    }
    let mut rax = rdx.wrapping_add(8);
    if rax == r8 {
        wr::<usize>(param_1, r8);
        return param_1;
    }
    let mut r9 = rd::<u64>(rdx);
    loop {
        let r10 = rd::<u64>(rax);
        if (r9 as u32) == (r10 as u32) && ((r9 >> 32) as u32) == ((r10 >> 32) as u32) {
            break;
        }
        rdx = rax;
        r9 = r10;
        rax = rax.wrapping_add(8);
        if rax == r8 {
            wr::<usize>(param_1, r8);
            return param_1;
        }
    }
    rax = rax.wrapping_add(8);
    while rax != r8 {
        let r10 = rd::<u64>(rax);
        let cur = rd::<u64>(rdx);
        if (cur as u32) != (r10 as u32) || ((cur >> 32) as u32) != ((r10 >> 32) as u32) {
            rdx = rdx.wrapping_add(8);
            wr::<u64>(rdx, r10);
        }
        rax = rax.wrapping_add(8);
    }
    wr::<usize>(param_1, rdx.wrapping_add(8));
    param_1
}

/// `unknown_140f7d880`
/// Original: 0x140f7d880 (521 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f7d880(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let count_signed = (rd::<usize>(param_4).wrapping_sub(rd::<usize>(param_3)) as isize) >> 3;
    if count_signed == 0 {
        return;
    }
    let count = count_signed as usize;
    let old_base = rd::<usize>(param_1);
    let len_ptr = param_1 + 0x08;
    let is_push_one =
        count == 1 && param_2 == old_base.wrapping_add((rd::<u32>(len_ptr) as usize) * 8);

    let cur_len = rd::<u32>(len_ptr) as usize;
    let cur_cap = rd::<u32>(param_1 + 0x0c) as usize;
    let rem_cap = cur_cap.wrapping_sub(cur_len);
    let new_len = cur_len.wrapping_add(count);

    if count > rem_cap {
        let ins_idx = ((param_2.wrapping_sub(old_base) as isize) >> 3) as usize;
        let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
        if new_cap < new_len {
            new_cap = new_len;
        }
        let new_buf = exe::call_va!(0x1_401d_ec00, (cur_len as u64, new_cap as u64) -> usize);
        let end_src = rd::<usize>(param_4);
        let mut cur_src = rd::<usize>(param_3);
        let mut cur_dst = new_buf.wrapping_add(ins_idx * 8);
        while cur_src != end_src {
            wr::<u64>(cur_dst, rd::<u64>(cur_src));
            cur_src = cur_src.wrapping_add(8);
            cur_dst = cur_dst.wrapping_add(8);
        }

        let src_buf = rd::<usize>(param_1);
        if is_push_one {
            let byte_len = (rd::<u32>(len_ptr) as usize) * 8;
            let _ = exe::call_va!(0x1_4183_acc8, (new_buf, src_buf, byte_len) -> usize);
        } else {
            let prefix_bytes = param_2.wrapping_sub(src_buf);
            let _ = exe::call_va!(0x1_4183_acc8, (new_buf, src_buf, prefix_bytes) -> usize);
            let total_bytes = (rd::<u32>(len_ptr) as usize) * 8;
            let dst2 = new_buf.wrapping_add(ins_idx.wrapping_add(count) * 8);
            let suffix_bytes = total_bytes
                .wrapping_sub(param_2)
                .wrapping_add(rd::<usize>(param_1));
            let _ = exe::call_va!(0x1_4183_acc8, (dst2, param_2, suffix_bytes) -> usize);
        }
        exe::call_va!(0x1_401d_e830, (param_1));
        wr::<u32>(len_ptr, new_len as u32);
        wr::<u32>(param_1 + 0x0c, new_cap as u32);
        wr::<usize>(param_1, new_buf);
    } else {
        let end_ptr = old_base.wrapping_add(cur_len * 8);
        let ins_bytes = count * 8;
        let tail_bytes = end_ptr.wrapping_sub(param_2);
        let tail_elems = ((tail_bytes as isize) >> 3) as usize;
        if count < tail_elems {
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (end_ptr, end_ptr.wrapping_sub(ins_bytes), ins_bytes) -> usize
            );
            let move_bytes = end_ptr.wrapping_sub(ins_bytes).wrapping_sub(param_2);
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (end_ptr.wrapping_sub(move_bytes), param_2, move_bytes) -> usize
            );
        } else {
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (param_2.wrapping_add(ins_bytes), param_2, tail_bytes) -> usize
            );
        }
        let end_src = rd::<usize>(param_4);
        let mut cur_src = rd::<usize>(param_3);
        let mut dst = param_2;
        while cur_src != end_src {
            wr::<u64>(dst, rd::<u64>(cur_src));
            cur_src = cur_src.wrapping_add(8);
            dst = dst.wrapping_add(8);
        }
        wr::<u32>(len_ptr, new_len as u32);
    }
}

/// `unknown_140f7dc00`
/// Original: 0x140f7dc00 (396 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f7dc00(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) -> usize {
    let r10 = rd::<usize>(param_3);
    let idx = (param_2 as u32) as usize;
    let arr_30 = rd::<usize>(r10 + 0x30);
    let rdx_val = rd::<u64>(arr_30.wrapping_add(idx * 8));
    let dx = rdx_val as u16;
    if dx == 0xffff {
        exe::call_iat!(0x1_4264_67e8, ());
        return param_1;
    }

    let rcx_slice = rd::<usize>(param_3 + 0x10);
    let count_8 = rd::<u64>(param_3 + 0x08) as usize;
    let r11_end = rcx_slice.wrapping_add(count_8.wrapping_mul(2));
    let mut cur = rcx_slice;
    let mut rem = (r11_end.wrapping_sub(rcx_slice) as isize) >> 1;
    while rem > 0 {
        let half = ((rem as usize) >> 1) as isize;
        let mid = cur.wrapping_add((half as usize) * 2);
        if rd::<u16>(mid) < dx {
            cur = mid.wrapping_add(2);
            rem = rem - 1 - half;
        } else {
            rem = half;
        }
    }

    if cur != r11_end && dx >= rd::<u16>(cur) {
        let r9 = dx as usize;
        let r8 = r10.wrapping_add(0x40);
        let rbx = rd::<u64>(r8.wrapping_add(r9 * 8).wrapping_add(0x48));
        let mut out_buf = [0u64; 7];
        exe::call_va!(
            0x1_40f8_0cd0,
            (out_buf.as_mut_ptr() as usize, rdx_val, r8, dx as u32)
        );
        for slot in &mut out_buf {
            if *slot != 0 {
                *slot = slot.wrapping_add(rbx);
            }
        }
        for (i, &val) in out_buf.iter().enumerate() {
            wr::<u64>(param_1 + i * 8, val);
        }
        wr::<u64>(param_1 + 0x38, rbx);
        wr::<u64>(param_1 + 0x40, (rdx_val >> 32) as u32 as u64);
        wr::<u8>(param_1 + 0x48, 1);
        return param_1;
    }

    wr::<u8>(param_1 + 0x48, 0);
    param_1
}

/// `unknown_140f817e0`
/// Original: 0x140f817e0 (584 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f817e0(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: u8,
) {
    let mut rsi = rd::<usize>(param_2);
    let mut rdi = rd::<usize>(param_1);
    let mut rbp = param_3;
    let bl = param_4;

    while ((rsi.wrapping_sub(rdi) as isize) & -8) > 0x100 {
        let mut local_70: usize = rsi;
        let mut local_78: usize = rdi;
        if rbp <= 0 {
            exe::call_va!(
                0x1_40f8_2a60,
                (
                    (&mut local_78 as *mut usize) as usize,
                    (&mut local_70 as *mut usize) as usize,
                    bl,
                )
            );
            let mut rcx = rd::<usize>(param_2);
            let base_rsi = rd::<usize>(param_1);
            if ((rcx.wrapping_sub(base_rsi) as isize) & -8) >= 0x10 {
                loop {
                    let prev_rdi = rcx.wrapping_sub(8);
                    if ((rcx.wrapping_sub(base_rsi) as isize) & -8) >= 0x10 {
                        let mut val_70 = rd::<u64>(prev_rdi);
                        wr::<u64>(prev_rdi, rd::<u64>(base_rsi));
                        let mut ptr_78 = base_rsi;
                        let count = (prev_rdi.wrapping_sub(base_rsi) as isize) >> 3;
                        exe::call_va!(
                            0x1_40f8_3e40,
                            (
                                (&mut ptr_78 as *mut usize) as usize,
                                0u64,
                                count as u64,
                                (&mut val_70 as *mut u64) as usize,
                                bl,
                            )
                        );
                    }
                    rcx = prev_rdi;
                    if ((rcx.wrapping_sub(base_rsi) as isize) & -8) < 0x10 {
                        break;
                    }
                }
            }
            return;
        }

        let mut part = [0usize; 2];
        exe::call_va!(
            0x1_40f8_2b80,
            (
                part.as_mut_ptr() as usize,
                (&mut local_78 as *mut usize) as usize,
                (&mut local_70 as *mut usize) as usize,
                bl,
            )
        );
        let r8 = rd::<usize>(param_2);
        let rdx = rd::<usize>(param_1);
        let p0 = part[0];
        let p1 = part[1];
        rbp = (rbp >> 1).wrapping_add(rbp >> 2);
        let left_bytes = (p0.wrapping_sub(rdx) as isize) & -8;
        let right_bytes = (r8.wrapping_sub(p1) as isize) & -8;
        if left_bytes < right_bytes {
            local_78 = rdx;
            local_70 = p0;
            unknown_140f817e0(
                (&mut local_78 as *mut usize) as usize,
                (&mut local_70 as *mut usize) as usize,
                rbp,
                bl,
            );
            wr::<usize>(param_1, p1);
        } else {
            local_78 = p1;
            local_70 = r8;
            unknown_140f817e0(
                (&mut local_78 as *mut usize) as usize,
                (&mut local_70 as *mut usize) as usize,
                rbp,
                bl,
            );
            wr::<usize>(param_2, p0);
        }
        rsi = rd::<usize>(param_2);
        rdi = rd::<usize>(param_1);
    }

    if rdi == rsi {
        return;
    }
    let mut rdx = rdi.wrapping_add(8);
    if rdx == rsi {
        return;
    }
    loop {
        let rax = rd::<u64>(rdx);
        let r9 = rd::<u64>(rdi);
        let rax_lo = rax as u32;
        let rax_hi = (rax >> 32) as u32;
        let r9_lo = r9 as u32;
        let r9_hi = (r9 >> 32) as u32;
        if rax_lo < r9_lo || (rax_lo == r9_lo && rax_hi < r9_hi) {
            if rdi != rdx {
                let mut rcx = rdx;
                let mut r9_ptr = rdx.wrapping_add(8);
                loop {
                    let v = rd::<u64>(rcx.wrapping_sub(8));
                    r9_ptr = r9_ptr.wrapping_sub(8);
                    rcx = rcx.wrapping_sub(8);
                    wr::<u64>(r9_ptr, v);
                    if rcx == rdi {
                        break;
                    }
                }
            }
            wr::<u64>(rdi, rax);
        } else {
            let mut r8 = rdx;
            let mut rcx = rdx;
            loop {
                let prev = rd::<u64>(rcx.wrapping_sub(8));
                rcx = rcx.wrapping_sub(8);
                let prev_lo = prev as u32;
                let prev_hi = (prev >> 32) as u32;
                if rax_lo < prev_lo || (rax_lo == prev_lo && rax_hi < prev_hi) {
                    wr::<u64>(r8, prev);
                    r8 = rcx;
                } else {
                    break;
                }
            }
            wr::<u64>(r8, rax);
        }
        rdx = rdx.wrapping_add(8);
        if rdx == rsi {
            break;
        }
    }
}

/// `coregame_140f86a50`
/// Original: 0x140f86a50 (96 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_140f86a50(param_1: usize, param_2: u64) -> usize {
    let want_id = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d0a0) as usize);
    let rbx = param_1.wrapping_add(0x40);
    let r11 = (param_2 as u16) as usize;
    let off = r11 << 5;
    let count = rd::<u32>(rbx.wrapping_add(off).wrapping_add(0x1806c));
    let data = rd::<usize>(rbx.wrapping_add(off).wrapping_add(0x18058));
    let idx = exe::call_va!(0x1_4248_9ee0, (data, count, want_id) -> i32) as isize;
    let arr_ptr = rd::<usize>(rbx.wrapping_add((r11 + 0xc03) << 5));
    let elem_off = rd::<u32>(arr_ptr.wrapping_add((idx as usize).wrapping_mul(4))) as u64;
    let base_ptr = rd::<u64>(rbx.wrapping_add(r11 * 8).wrapping_add(0x48));
    let high_idx = (param_2 >> 32) as u32 as u64;
    elem_off
        .wrapping_add(base_ptr)
        .wrapping_add(high_idx.wrapping_mul(0x1c)) as usize
}

/// `coregame_140f8b340`
/// Original: 0x140f8b340 (295 bytes, Throws::Caught)
pub unsafe extern "win64" fn coregame_140f8b340(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> usize {
    let mut rbx = param_2;
    let rbp = param_3;
    let mut rsi = param_4;
    let rdi = param_6;
    if rbx != rbp {
        let r14 = param_5;
        while rsi != r14 {
            let k1 = rd::<u64>(rbx + 0x10);
            let k2 = rd::<u64>(rsi + 0x10);
            if k1 < k2 {
                let len = rd::<u32>(rdi + 0x08) as u64;
                let cap = rd::<u32>(rdi + 0x0c) as u64;
                if len.wrapping_mul(0x30) == cap.wrapping_mul(0x30) {
                    exe::call_va!(0x1_40f9_0b40, (rdi, len.wrapping_add(1)));
                }
                let cur_len = rd::<u32>(rdi + 0x08) as usize;
                let dst = rd::<usize>(rdi).wrapping_add(cur_len.wrapping_mul(0x30));
                wr::<[u64; 6]>(dst, rd::<[u64; 6]>(rbx));
                wr::<u32>(rdi + 0x08, rd::<u32>(rdi + 0x08).wrapping_add(1));
                rbx = rbx.wrapping_add(0x30);
            } else {
                if k1 == k2 {
                    rbx = rbx.wrapping_add(0x30);
                }
                rsi = rsi.wrapping_add(0x30);
            }
            if rbx == rbp {
                break;
            }
        }
    }
    while rbx != rbp {
        let len = rd::<u32>(rdi + 0x08) as u64;
        let cap = rd::<u32>(rdi + 0x0c) as u64;
        if len.wrapping_mul(0x30) == cap.wrapping_mul(0x30) {
            exe::call_va!(0x1_40f9_0b40, (rdi, len.wrapping_add(1)));
        }
        let cur_len = rd::<u32>(rdi + 0x08) as usize;
        let dst = rd::<usize>(rdi).wrapping_add(cur_len.wrapping_mul(0x30));
        wr::<[u64; 6]>(dst, rd::<[u64; 6]>(rbx));
        wr::<u32>(rdi + 0x08, rd::<u32>(rdi + 0x08).wrapping_add(1));
        rbx = rbx.wrapping_add(0x30);
    }
    wr::<usize>(param_1, rdi);
    param_1
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_unknown_140f64cf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG: Vec<(u32, u64, u64, u64)> = Vec::new();
    static mut SUB_OBJ: [usize; 2] = [0; 2];
    static mut RET_SUB: bool = false;

    unsafe extern "win64" fn mock_218(_this: usize, p2: usize, p3: usize) -> usize {
        LOG.push((1, p2 as u64, p3 as u64, 0));
        if RET_SUB {
            core::ptr::addr_of_mut!(SUB_OBJ) as usize
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_240(
        _this: usize,
        p2: usize,
        p3: usize,
        u4: u64,
        cb: usize,
    ) {
        LOG.push((2, p2 as u64, p3 as u64, u4 ^ (cb as u64)));
    }
    unsafe extern "win64" fn mock_78(_sub: usize) -> u32 {
        LOG.push((3, 0, 0, 0));
        7
    }
    unsafe extern "win64" fn mock_70(_sub: usize, val: u32) {
        LOG.push((4, val as u64, 0, 0));
    }
    unsafe extern "win64" fn mock_28(_sub: usize) {
        LOG.push((5, 0, 0, 0));
    }
    unsafe extern "win64" fn mock_48(_sub: usize, p3: usize) {
        LOG.push((6, p3 as u64, 0, 0));
    }
    unsafe extern "win64" fn mock_40(_sub: usize) -> u64 {
        LOG.push((7, 0, 0, 0));
        0xCAFE_BABE
    }
    unsafe extern "win64" fn mock_6720(_sub: usize, p3: usize) {
        LOG.push((8, p3 as u64, 0, 0));
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e1_6720, mock_6720 as *const ())?;

    let mut vtbl_main = [0usize; 0x50];
    vtbl_main[0x218 / 8] = mock_218 as *const () as usize;
    vtbl_main[0x240 / 8] = mock_240 as *const () as usize;

    let mut vtbl_sub = [0usize; 0x14];
    vtbl_sub[0x28 / 8] = mock_28 as *const () as usize;
    vtbl_sub[0x40 / 8] = mock_40 as *const () as usize;
    vtbl_sub[0x48 / 8] = mock_48 as *const () as usize;
    vtbl_sub[0x70 / 8] = mock_70 as *const () as usize;
    vtbl_sub[0x78 / 8] = mock_78 as *const () as usize;
    SUB_OBJ[0] = vtbl_sub.as_ptr() as usize;

    let main_obj: [usize; 2] = [vtbl_main.as_ptr() as usize, 0];

    let run = |f: F, has_sub: bool, p3: usize| -> Vec<(u32, u64, u64, u64)> {
        LOG.clear();
        RET_SUB = has_sub;
        f(main_obj.as_ptr() as usize, 0x1111, p3);
        LOG.clone()
    };

    for (has_sub, p3) in [(false, 0usize), (true, 0usize), (true, 0x2222usize)] {
        let o = run(f_o, has_sub, p3);
        let n = run(f_n, has_sub, p3);
        crate::diff_eq!(o, n, "140f64cf0 sub={} p3={}", has_sub, p3);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140f65220(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_BUF: [u8; 0xd0 * 8] = [0; 0xd0 * 8];
    static mut GROW_CALLED: Vec<u64> = Vec::new();
    static mut INIT_CALLED: Vec<(usize, [u64; 2], u64, u64, u64, u32)> = Vec::new();

    unsafe extern "win64" fn mock_abe0(vec_ptr: usize, new_cap: u64) {
        GROW_CALLED.push(new_cap);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_3a20(
        dst: usize,
        p2: usize,
        v3: usize,
        v4: usize,
        v5: usize,
        p6_val: u32,
    ) {
        let base = core::ptr::addr_of!(VEC_BUF) as usize;
        INIT_CALLED.push((
            dst.wrapping_sub(base),
            rd::<[u64; 2]>(p2),
            rd::<u64>(v3 + 0x08),
            rd::<u64>(v4 + 0x08),
            rd::<u64>(v5 + 0x08),
            p6_val,
        ));
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f6_abe0, mock_abe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e1_3a20, mock_3a20 as *const ())?;

    let run = |f: F,
               len: u32,
               cap: u32|
     -> (
        usize,
        u32,
        u32,
        Vec<u64>,
        Vec<(usize, [u64; 2], u64, u64, u64, u32)>,
    ) {
        GROW_CALLED.clear();
        INIT_CALLED.clear();
        let base = core::ptr::addr_of_mut!(VEC_BUF) as usize;
        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, base);
        wr::<u32>(vp + 0x08, len);
        wr::<u32>(vp + 0x0c, cap);

        let p2: [u64; 2] = [0x1122_3344_5566_7788, 0x99aa_bbcc_ddee_ff00];
        let mut s3 = [0u64; 10];
        let mut s4 = [0u64; 10];
        let mut s5 = [0u64; 10];
        let s3p = s3.as_mut_ptr() as usize;
        let s4p = s4.as_mut_ptr() as usize;
        let s5p = s5.as_mut_ptr() as usize;
        wr::<u32>(s3p, 0x0800_0008);
        wr::<u32>(s3p + 0x04, 5);
        wr::<u32>(s4p, 0x0800_0008);
        wr::<u32>(s4p + 0x04, 9);
        wr::<u32>(s5p, 0x8000_0100);
        wr::<u32>(s5p + 0x04, 14);
        wr::<usize>(s5p + 0x08, s3p + 0x08);

        let p6: u32 = 42;
        let ret = f(vp, p2.as_ptr() as usize, s3p, s4p, s5p, (&p6 as *const u32) as usize);
        (
            ret.wrapping_sub(base),
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            GROW_CALLED.clone(),
            INIT_CALLED.clone(),
        )
    };

    for (len, cap) in [(2u32, 2u32), (1u32, 4u32)] {
        let o = run(f_o, len, cap);
        let n = run(f_n, len, cap);
        crate::diff_eq!(o, n, "140f65220 len={} cap={}", len, cap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140f69070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG: Vec<(u32, u8)> = Vec::new();

    unsafe extern "win64" fn mock_ab10(s_ptr: usize) {
        LOG.push((1, rd::<u8>(s_ptr + 0x48)));
    }
    unsafe extern "win64" fn mock_5520(s_ptr: usize) {
        LOG.push((2, rd::<u8>(s_ptr + 0x48)));
        wr::<u8>(s_ptr + 0x48, 0xff);
    }
    unsafe extern "win64" fn mock_aa00(dst_str: usize, src_str: usize) {
        LOG.push((3, rd::<u8>(dst_str + 0x03)));
        wr::<u32>(dst_str + 0x04, rd::<u32>(src_str + 0x04));
        wr::<u64>(dst_str + 0x08, rd::<u64>(src_str + 0x08));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f4_5520, mock_5520 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;

    let run = |f: F, op: u64, init_tag: u8| -> (Vec<u8>, Vec<(u32, u8)>) {
        LOG.clear();
        let mut var_buf = vec![0x5au8; 0x50];
        let vp = var_buf.as_mut_ptr() as usize;
        wr::<u8>(vp + 0x48, init_tag);

        let mut src_buf = [0u64; 10];
        let sp = src_buf.as_mut_ptr() as usize;
        wr::<u32>(sp, 0x0800_0008);
        wr::<u32>(sp + 0x04, 6);
        wr::<u64>(sp + 0x08, 0x6655_4433_2211);

        let p2_slot: [usize; 1] = [vp];
        f(op, p2_slot.as_ptr() as usize, sp);
        (var_buf, LOG.clone())
    };

    for op in 0..=6u64 {
        for init_tag in [0xffu8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8] {
            let o = run(f_o, op, init_tag);
            let n = run(f_n, op, init_tag);
            crate::diff_eq!(o, n, "140f69070 op={} tag={}", op, init_tag);
        }
    }
    Ok(7)
}

pub unsafe fn diff_unknown_140f7d7e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let cases: [&[u64]; 4] = [
        &[],
        &[0x10],
        &[1, 2, 3, 4],
        &[1, 2, 2, 3, (1u64 << 32) | 3, (1u64 << 32) | 3, 4, 4, 5],
    ];

    let run = |f: F, data: &[u64]| -> (usize, Vec<u64>) {
        let mut buf = data.to_vec();
        let base = buf.as_mut_ptr() as usize;
        let end = base.wrapping_add(buf.len() * 8);
        let mut out_ptr: usize = 0;
        let ret = f(
            (&mut out_ptr as *mut usize) as usize,
            (&base as *const usize) as usize,
            (&end as *const usize) as usize,
        );
        assert_eq!(ret, (&mut out_ptr as *mut usize) as usize);
        let new_count = (out_ptr.wrapping_sub(base)) / 8;
        buf.truncate(new_count);
        (new_count, buf)
    };

    for (idx, case) in cases.iter().enumerate() {
        let o = run(f_o, case);
        let n = run(f_n, case);
        crate::diff_eq!(o, n, "140f7d7e0 idx={}", idx);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_140f7d880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u64; 32] = [0; 32];
    static mut FREED_CNT: u32 = 0;

    unsafe extern "win64" fn mock_ec00(_old_len: u64, _new_cap: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    unsafe extern "win64" fn mock_e830(_vec: usize) {
        FREED_CNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ec00, mock_ec00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F,
               init: &[u64],
               cap: u32,
               ins_pos: usize,
               ins: &[u64]|
     -> (bool, u32, u32, u32, Vec<u64>) {
        NEW_BUF = [0; 32];
        FREED_CNT = 0;
        let mut buf = vec![0u64; cap as usize];
        buf[..init.len()].copy_from_slice(init);
        let base = buf.as_mut_ptr() as usize;

        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, base);
        wr::<u32>(vp + 0x08, init.len() as u32);
        wr::<u32>(vp + 0x0c, cap);

        let src_first = ins.as_ptr() as usize;
        let src_last = src_first.wrapping_add(ins.len() * 8);
        let pos_ptr = base.wrapping_add(ins_pos * 8);

        f(
            vp,
            pos_ptr,
            (&src_first as *const usize) as usize,
            (&src_last as *const usize) as usize,
        );

        let final_ptr = rd::<usize>(vp);
        let final_len = rd::<u32>(vp + 0x08) as usize;
        let final_cap = rd::<u32>(vp + 0x0c);
        let reallocated = final_ptr == core::ptr::addr_of!(NEW_BUF) as usize;
        let mut out_elems = Vec::with_capacity(final_len);
        for i in 0..final_len {
            out_elems.push(rd::<u64>(final_ptr + i * 8));
        }
        (reallocated, final_len as u32, final_cap, FREED_CNT, out_elems)
    };

    let cases: [(&[u64], u32, usize, &[u64]); 5] = [
        (&[10, 20, 30], 4, 1, &[]),
        (&[10, 20, 30], 3, 3, &[40]),
        (&[10, 20, 30], 4, 1, &[99, 98, 97]),
        (&[10, 20, 30, 40, 50], 8, 1, &[77]),
        (&[10, 20, 30], 8, 2, &[88, 89, 90]),
    ];

    for (idx, &(init, cap, pos, ins)) in cases.iter().enumerate() {
        let o = run(f_o, init, cap, pos, ins);
        let n = run(f_n, init, cap, pos, ins);
        crate::diff_eq!(o, n, "140f7d880 case={}", idx);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_140f7dc00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_CD0: Vec<(u64, usize, u32)> = Vec::new();

    unsafe extern "win64" fn mock_0cd0(out_ptr: usize, rdx_val: u64, r8_ptr: usize, dx_val: u32) {
        LOG_CD0.push((rdx_val, r8_ptr & 0xfff, dx_val));
        for i in 0..7usize {
            wr::<u64>(
                out_ptr + i * 8,
                if i % 2 == 0 { (i as u64 + 1) * 0x100 } else { 0 },
            );
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f8_0cd0, mock_0cd0 as *const ())?;

    let run = |f: F, query_idx: u64| -> (Vec<u8>, Vec<(u64, usize, u32)>) {
        LOG_CD0.clear();
        let mut r10_buf = vec![0u8; 0x200];
        let r10 = r10_buf.as_mut_ptr() as usize;
        let arr_30: [u64; 4] = [
            (0x11u64 << 32) | 3,
            (0x22u64 << 32) | 7,
            (0x33u64 << 32) | 12,
            (0x44u64 << 32) | 20,
        ];
        wr::<usize>(r10 + 0x30, arr_30.as_ptr() as usize);
        for k in 0..24usize {
            wr::<u64>(r10 + 0x88 + k * 8, 0x5000 + (k as u64) * 0x10);
        }

        let slice_u16: [u16; 4] = [2, 3, 12, 19];
        let mut param3 = [0u64; 3];
        let p3p = param3.as_mut_ptr() as usize;
        wr::<usize>(p3p, r10);
        wr::<u64>(p3p + 0x08, slice_u16.len() as u64);
        wr::<usize>(p3p + 0x10, slice_u16.as_ptr() as usize);

        let mut out_struct = vec![0xAAu8; 0x50];
        let op = out_struct.as_mut_ptr() as usize;
        let ret = f(op, query_idx, p3p);
        assert_eq!(ret, op);
        (out_struct, LOG_CD0.clone())
    };

    for q in [0u64, 1u64, 2u64, 3u64] {
        let o = run(f_o, q);
        let n = run(f_n, q);
        crate::diff_eq!(o, n, "140f7dc00 q={}", q);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_140f817e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, input: &[u64]| -> Vec<u64> {
        let mut buf = input.to_vec();
        let mut first = buf.as_mut_ptr() as usize;
        let mut last = first.wrapping_add(buf.len() * 8);
        f(
            (&mut first as *mut usize) as usize,
            (&mut last as *mut usize) as usize,
            16,
            1,
        );
        buf
    };

    let cases: [&[u64]; 4] = [
        &[],
        &[42],
        &[
            (2u64 << 32) | 5,
            (1u64 << 32) | 5,
            (9u64 << 32) | 1,
            (3u64 << 32) | 5,
            (4u64 << 32) | 2,
            (1u64 << 32) | 2,
            (7u64 << 32) | 9,
            (2u64 << 32) | 1,
        ],
        &[
            30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9,
            8, 7, 6, 5, 4, 3, 2, 1,
        ],
    ];

    for (idx, case) in cases.iter().enumerate() {
        let o = run(f_o, case);
        let n = run(f_n, case);
        crate::diff_eq!(o, n, "140f817e0 idx={}", idx);
    }
    Ok(4)
}

pub unsafe fn diff_coregame_140f86a50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let want_id = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d0a0) as usize);
    let ids: [u16; 4] = [want_id.wrapping_add(1), want_id, want_id.wrapping_add(2), 0];
    let offs: [u32; 4] = [0x100, 0x240, 0x380, 0x4c0];

    let mut obj = vec![0u8; 0x20000];
    let p1 = obj.as_mut_ptr() as usize;
    let rbx = p1 + 0x40;

    for r11 in [0usize, 2usize] {
        let off = r11 << 5;
        wr::<usize>(rbx + off + 0x18058, ids.as_ptr() as usize);
        wr::<u32>(rbx + off + 0x1806c, ids.len() as u32);
        wr::<usize>(rbx + ((r11 + 0xc03) << 5), offs.as_ptr() as usize);
        wr::<u64>(rbx + r11 * 8 + 0x48, 0x8000_0000 + (r11 as u64) * 0x1000);
    }

    for param_2 in [0u64, (3u64 << 32) | 2, (5u64 << 32)] {
        let o = f_o(p1, param_2);
        let n = f_n(p1, param_2);
        crate::diff_eq!(o, n, "140f86a50 p2={:#x}", param_2);
    }
    Ok(3)
}

pub unsafe fn diff_coregame_140f8b340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut OUT_BUF: [[u64; 6]; 16] = [[0; 6]; 16];
    static mut GROW_REQS: Vec<u64> = Vec::new();

    unsafe extern "win64" fn mock_0b40(vec_ptr: usize, req: u64) {
        GROW_REQS.push(req);
        wr::<u32>(vec_ptr + 0x0c, (req as u32) + 2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f9_0b40, mock_0b40 as *const ())?;

    let make_elem = |k: u64, tag: u64| -> [u64; 6] { [tag, tag + 1, k, tag + 2, tag + 3, tag + 4] };

    let run = |f: F, k1s: &[u64], k2s: &[u64], init_cap: u32| -> (u32, u32, Vec<u64>, Vec<[u64; 6]>) {
        OUT_BUF = [[0; 6]; 16];
        GROW_REQS.clear();
        let r1: Vec<[u64; 6]> = k1s
            .iter()
            .enumerate()
            .map(|(i, &k)| make_elem(k, (i as u64 + 1) * 10))
            .collect();
        let r2: Vec<[u64; 6]> = k2s
            .iter()
            .enumerate()
            .map(|(i, &k)| make_elem(k, (i as u64 + 1) * 100))
            .collect();

        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, core::ptr::addr_of_mut!(OUT_BUF) as usize);
        wr::<u32>(vp + 0x08, 0);
        wr::<u32>(vp + 0x0c, init_cap);

        let mut out_ret: usize = 0;
        let p2 = r1.as_ptr() as usize;
        let p3 = p2.wrapping_add(r1.len() * 0x30);
        let p4 = r2.as_ptr() as usize;
        let p5 = p4.wrapping_add(r2.len() * 0x30);
        let ret = f((&mut out_ret as *mut usize) as usize, p2, p3, p4, p5, vp);
        assert_eq!(ret, (&mut out_ret as *mut usize) as usize);
        assert_eq!(out_ret, vp);

        let final_len = rd::<u32>(vp + 0x08);
        let final_cap = rd::<u32>(vp + 0x0c);
        (
            final_len,
            final_cap,
            GROW_REQS.clone(),
            OUT_BUF[..final_len as usize].to_vec(),
        )
    };

    let cases: [(&[u64], &[u64], u32); 3] = [
        (&[1, 3, 5, 7, 9], &[2, 3, 6, 7], 1),
        (&[2, 4, 6], &[1, 2, 3, 4, 5, 6, 7], 4),
        (&[10, 20], &[], 0),
    ];

    for (idx, &(k1s, k2s, cap)) in cases.iter().enumerate() {
        let o = run(f_o, k1s, k2s, cap);
        let n = run(f_n, k1s, k2s, cap);
        crate::diff_eq!(o, n, "140f8b340 idx={}", idx);
    }
    Ok(3)
}
