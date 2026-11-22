//! Decompiled `rend` hot functions shard ZA18 (`0x14220c7e0..0x142245590`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// `unknown_14220c7e0`
/// Original: 0x14220c7e0 (300 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14220c7e0(_param_1: usize, param_2: usize) {
    let rbx = rd::<usize>(exe::ptr::<usize>(0x1_43a0_0188) as usize);
    if rd::<u32>(rbx + 0x20) == rd::<u32>(param_2)
        && rd::<u32>(rbx + 0x24) == rd::<u32>(param_2 + 0x04)
    {
        return;
    }

    let rdi = rd::<usize>(rbx + 0x40);
    wr::<usize>(rbx + 0x40, 0);
    if rdi != 0 {
        exe::call_va!(0x1_4204_f860, (rdi));
        exe::call_va!(0x1_4244_2de0, (rdi));
    }

    wr::<u64>(rbx + 0x20, rd::<u64>(param_2));
    exe::call_va!(0x1_4220_ca30, (rbx));

    let mut rax = rd::<usize>(rbx + 0xc0e0);
    let r9 = rax.wrapping_add((rd::<u64>(rbx + 0xc0a0) as usize).wrapping_mul(0x38));
    while rax != r9 {
        let rdx_empty = rd::<u64>(rbx + 0xc0a8);
        while rax != r9 {
            let rcx_key = rd::<u64>(rax);
            if rdx_empty == rcx_key
                || (rd::<u64>(rbx + 0xc090) > 0 && rd::<u64>(rbx + 0xc088) == rcx_key)
            {
                rax = rax.wrapping_add(0x38);
            } else {
                break;
            }
        }
        if rax == r9 {
            break;
        }
        let r8 = rd::<usize>(rax + 0x18);
        let rdx_val = rd::<u64>(rbx + 0x40);
        let cl = u8::from(rd::<u64>(r8) != rdx_val);
        wr::<u8>(r8 + 0x08, rd::<u8>(r8 + 0x08) | cl);
        wr::<u64>(r8, rdx_val);
        let rcx_ptr = rd::<usize>(rax + 0x18);
        wr::<u8>(rcx_ptr + 0x08, 1);
        rax = rax.wrapping_add(0x38);
    }

    let rcx_48 = rd::<usize>(rbx + 0x48);
    wr::<u32>(rcx_48 + 0x04, rd::<u32>(rbx + 0x20));
    let rcx_48_b = rd::<usize>(rbx + 0x48);
    wr::<u32>(rcx_48_b, rd::<u32>(rbx + 0x24));
}

/// `unknown_14222ec60`
/// Original: 0x14222ec60 (123 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14222ec60(param_1: usize, param_2: u32) -> u8 {
    let r8 = rd::<usize>(param_1 + 0x90);
    let rax_g = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let rcx_tbl = rd::<usize>(rax_g + 0x1a8);
    let edx_val = rd::<u8>(rcx_tbl.wrapping_add(param_2 as usize)) as u32;
    if rd::<u32>(r8 + 0x34) != 0 {
        let eax = exe::call_va!(0x1_4222_ed20, (param_1, edx_val) -> u32);
        return u8::from(eax == param_2);
    }

    let b272 = rd::<u8>(r8 + 0x272);
    if b272 != 0 {
        let ecx_m1 = (b272 as i32) - 1;
        let edx_i32 = edx_val as i32;
        let idx = (if ecx_m1 < edx_i32 { ecx_m1 } else { edx_i32 }) as isize;
        let cnt = rd::<i32>((r8 as isize + idx * 4 + 0x1c8) as usize);
        if cnt > 0 && edx_val < (rd::<u8>(r8 + 0x270) as u32) {
            return 1;
        }
    }
    0
}

/// `unknown_14222ed20`
/// Original: 0x14222ed20 (166 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14222ed20(param_1: usize, param_2: u32) -> u32 {
    let rdx = rd::<usize>(param_1 + 0x90);
    let b272 = rd::<u8>(rdx + 0x272);
    if b272 != 0 {
        let r8d = (b272 as i32) - 1;
        let ebx_i32 = param_2 as i32;
        let idx = (if r8d < ebx_i32 { r8d } else { ebx_i32 }) as isize;
        if rd::<u32>((rdx as isize + idx * 4 + 0x1c8) as usize) != 0
            && param_2 < (rd::<u8>(rdx + 0x270) as u32)
        {
            let mut rax = rd::<usize>(param_1 + 0x80);
            let len = rd::<u32>(param_1 + 0x88) as usize;
            let r11 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
            let rcx_end = rax.wrapping_add(len * 4);
            while rax != rcx_end {
                let r9d = rd::<u32>(rax);
                if r9d != u32::MAX {
                    let rdx_1a8 = rd::<usize>(r11 + 0x1a8);
                    let r8d_b = rd::<u8>(rdx_1a8.wrapping_add(r9d as usize)) as u32;
                    if r8d_b == param_2 {
                        let rdx_18 = rd::<usize>(r11 + 0x18);
                        let r8_obj = rd::<usize>(rdx_18.wrapping_add((r9d as usize) * 8));
                        if (rd::<u8>(r8_obj + 0x4c) & 1) == 0 {
                            return r9d;
                        }
                    }
                }
                rax = rax.wrapping_add(4);
            }
        }
    }
    u32::MAX
}

/// `unknown_1422391f0`
/// Original: 0x1422391f0 (2221 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1422391f0(param_1: usize) {
    let src_lock = rd::<usize>(param_1 + 0x08).wrapping_add(0xa8);
    let tid1 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(src_lock + 0x08) != tid1 {
        exe::call_iat!(0x1_4264_55a8, (src_lock));
        wr::<u32>(src_lock + 0x08, tid1);
    }
    wr::<u32>(src_lock + 0x0c, rd::<u32>(src_lock + 0x0c).wrapping_add(1));

    let dst_base = rd::<usize>(param_1);
    let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let slot_ptr =
        rd::<usize>((exe::ptr::<usize>(0x1_43c4_aa00) as usize).wrapping_add(worker_idx * 8));
    let dst_idx = if slot_ptr != 0 {
        rd::<u32>(slot_ptr) as usize
    } else {
        0
    };
    let dst = dst_base.wrapping_add(dst_idx.wrapping_mul(0xd8));

    let dst_lock = dst.wrapping_add(0xa8);
    let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(dst_lock + 0x08) != tid2 {
        exe::call_iat!(0x1_4264_55a8, (dst_lock));
        wr::<u32>(dst_lock + 0x08, tid2);
    }
    wr::<u32>(dst_lock + 0x0c, rd::<u32>(dst_lock + 0x0c).wrapping_add(1));

    let vec_88 = dst.wrapping_add(0x88);
    let orig_len_88 = rd::<u32>(vec_88 + 0x08) as u64;
    let orig_off_88 = (orig_len_88 as usize) << 5;
    let r9_end_88 = rd::<usize>(vec_88).wrapping_add(orig_off_88);

    let vec_98 = dst.wrapping_add(0x98);
    let orig_len_98 = rd::<u32>(vec_98 + 0x08) as u64;
    let orig_len_68 = rd::<u32>(dst + 0x70) as u64;
    let orig_len_78 = rd::<u32>(dst + 0x80) as u64;

    let cnt_10 = rd::<u64>(param_1 + 0x10);
    if cnt_10 != 0 {
        let new_len = cnt_10.wrapping_add(orig_len_88);
        let cap = rd::<u32>(vec_88 + 0x0c) as u64;
        if new_len > cap {
            let mut grow = (cap >> 1) + cap;
            if grow < new_len {
                grow = new_len;
            }
            exe::call_va!(0x1_40fb_1b60, (vec_88, grow));
            let cur_len = rd::<u32>(vec_88 + 0x08) as usize;
            let zero_dst = rd::<usize>(vec_88).wrapping_add(cur_len << 5);
            exe::call_va!(0x1_4183_acc2, (zero_dst, 0u32, cnt_10 << 5));
            wr::<u32>(vec_88 + 0x08, new_len as u32);
        } else if new_len > orig_len_88 {
            exe::call_va!(0x1_4183_acc2, (r9_end_88, 0u32, cnt_10 << 5));
            wr::<u32>(vec_88 + 0x08, new_len as u32);
        } else if new_len != orig_len_88 {
            wr::<u32>(vec_88 + 0x08, new_len as u32);
        }
        let src_obj = rd::<usize>(param_1 + 0x08);
        let copy_len = rd::<u64>(param_1 + 0x10) << 5;
        let copy_dst = rd::<usize>(vec_88).wrapping_add(orig_off_88);
        let copy_src = rd::<usize>(src_obj + 0x88);
        exe::call_va!(0x1_4183_acbc, (copy_dst, copy_src, copy_len));
    }

    let cnt_18 = rd::<u64>(param_1 + 0x18);
    if cnt_18 != 0 {
        exe::call_va!(0x1_4086_b290, (vec_98, cnt_18.wrapping_add(orig_len_98)));
        let src_obj = rd::<usize>(param_1 + 0x08);
        let copy_len = rd::<u64>(param_1 + 0x18).wrapping_mul(0x0c);
        let copy_dst = rd::<usize>(vec_98).wrapping_add((orig_len_98 as usize).wrapping_mul(0x0c));
        let copy_src = rd::<usize>(src_obj + 0x98);
        exe::call_va!(0x1_4183_acbc, (copy_dst, copy_src, copy_len));
    }

    let cnt_20 = rd::<u64>(param_1 + 0x20);
    if cnt_20 != 0 {
        let vec_68 = dst.wrapping_add(0x68);
        let new_len = cnt_20.wrapping_add(orig_len_68);
        let cur_len = rd::<u32>(dst + 0x70) as u64;
        let r8_end = rd::<usize>(vec_68).wrapping_add((cur_len as usize).wrapping_mul(0x50));
        let cap = rd::<u32>(dst + 0x74) as u64;
        if new_len > cap {
            let mut grow = (cap >> 1) + cap;
            if grow < new_len {
                grow = new_len;
            }
            exe::call_va!(0x1_412e_e6f0, (vec_68, grow));
            let diff = new_len.wrapping_sub(cur_len);
            if diff != 0 {
                let cur_len2 = rd::<u32>(dst + 0x70) as usize;
                let mut p = rd::<usize>(vec_68).wrapping_add(cur_len2.wrapping_mul(0x50));
                for _ in 0..diff {
                    wr::<[u8; 0x50]>(p, [0u8; 0x50]);
                    p = p.wrapping_add(0x50);
                }
            }
            wr::<u32>(dst + 0x70, new_len as u32);
        } else if new_len > cur_len {
            let diff = new_len.wrapping_sub(cur_len);
            let mut p = r8_end;
            for _ in 0..diff {
                wr::<[u8; 0x50]>(p, [0u8; 0x50]);
                p = p.wrapping_add(0x50);
            }
            wr::<u32>(dst + 0x70, new_len as u32);
        } else if new_len != cur_len {
            wr::<u32>(dst + 0x70, new_len as u32);
        }
        let src_obj = rd::<usize>(param_1 + 0x08);
        let copy_len = rd::<u64>(param_1 + 0x20).wrapping_mul(0x50);
        let copy_dst = rd::<usize>(vec_68).wrapping_add((orig_len_68 as usize).wrapping_mul(0x50));
        let copy_src = rd::<usize>(src_obj + 0x68);
        exe::call_va!(0x1_4183_acbc, (copy_dst, copy_src, copy_len));
    }

    let cnt_28 = rd::<u64>(param_1 + 0x28);
    if cnt_28 != 0 {
        let vec_78 = dst.wrapping_add(0x78);
        let new_len = cnt_28.wrapping_add(orig_len_78);
        let cur_len = rd::<u32>(dst + 0x80) as u64;
        let rdi_end = rd::<usize>(vec_78).wrapping_add((cur_len as usize).wrapping_mul(0xd0));
        let cap = rd::<u32>(dst + 0x84) as u64;
        if new_len > cap {
            let mut grow = (cap >> 1) + cap;
            if grow < new_len {
                grow = new_len;
            }
            exe::call_va!(0x1_412e_e880, (vec_78, grow));
            let diff = new_len.wrapping_sub(cur_len);
            if diff != 0 {
                let cur_len2 = rd::<u32>(dst + 0x80) as usize;
                let mut p = rd::<usize>(vec_78).wrapping_add(cur_len2.wrapping_mul(0xd0));
                for _ in 0..diff {
                    exe::call_va!(0x1_4183_acc2, (p, 0u32, 0xc0u64));
                    wr::<u64>(p + 0xc0, 0);
                    wr::<u64>(p + 0xc8, 0);
                    p = p.wrapping_add(0xd0);
                }
            }
            wr::<u32>(dst + 0x80, new_len as u32);
        } else if new_len > cur_len {
            let diff = new_len.wrapping_sub(cur_len);
            let mut p = rdi_end;
            for _ in 0..diff {
                exe::call_va!(0x1_4183_acc2, (p, 0u32, 0xc0u64));
                wr::<u64>(p + 0xc0, 0);
                wr::<u64>(p + 0xc8, 0);
                p = p.wrapping_add(0xd0);
            }
            wr::<u32>(dst + 0x80, new_len as u32);
        } else if new_len != cur_len {
            wr::<u32>(dst + 0x80, new_len as u32);
        }

        let src_obj = rd::<usize>(param_1 + 0x08);
        let rdi_byte_off = (orig_len_78 as usize).wrapping_mul(0xd0);
        let copy_len = rd::<u64>(param_1 + 0x28).wrapping_mul(0xd0);
        let copy_dst = rd::<usize>(vec_78).wrapping_add(rdi_byte_off);
        let copy_src = rd::<usize>(src_obj + 0x78);
        exe::call_va!(0x1_4183_acbc, (copy_dst, copy_src, copy_len));

        let mut idx = orig_len_78;
        let mut byte_off = rdi_byte_off;
        while idx < rd::<u64>(param_1 + 0x28).wrapping_add(orig_len_78) {
            let arr_base = rd::<usize>(vec_78);
            let elem = arr_base.wrapping_add(byte_off);
            if rd::<u32>(elem) == 5 {
                let add_10 = if rd::<u64>(elem + 0xb8) != 0 {
                    orig_len_98 as u32
                } else {
                    orig_len_88 as u32
                };
                wr::<u32>(elem + 0x10, rd::<u32>(elem + 0x10).wrapping_add(add_10));
                wr::<u32>(
                    elem + 0x18,
                    rd::<u32>(elem + 0x18).wrapping_add(orig_len_68 as u32),
                );
            }
            idx = idx.wrapping_add(1);
            byte_off = byte_off.wrapping_add(0xd0);
        }
    }

    // Remove transferred prefix from src + 0x88 (0x20-byte elements)
    let src_88_obj = rd::<usize>(param_1 + 0x08);
    let rcx_88 = rd::<usize>(src_88_obj + 0x88);
    let rdi_cnt_10 = rd::<u64>(param_1 + 0x10);
    let rdx_88 = rcx_88.wrapping_add((rdi_cnt_10 as usize) << 5);
    if rcx_88 != rdx_88 {
        let len_88 = rd::<u32>(src_88_obj + 0x90) as usize;
        let end_88 = rcx_88.wrapping_add(len_88 << 5);
        if rdx_88 == end_88 {
            wr::<u32>(src_88_obj + 0x90, 0);
        } else {
            let rem_bytes = end_88.wrapping_sub(rdx_88) as u64;
            exe::call_va!(0x1_4183_acc8, (rcx_88, rdx_88, rem_bytes));
            wr::<u32>(
                src_88_obj + 0x90,
                rd::<u32>(src_88_obj + 0x90).wrapping_sub(rdi_cnt_10 as u32),
            );
        }
    }

    // Remove transferred prefix from src + 0x98 (0x0c-byte elements)
    let src_98_obj = rd::<usize>(param_1 + 0x08);
    let rcx_98 = rd::<usize>(src_98_obj + 0x98);
    let rdi_bytes_18 = (rd::<u64>(param_1 + 0x18) as usize).wrapping_mul(0x0c);
    let rdx_98 = rcx_98.wrapping_add(rdi_bytes_18);
    if rcx_98 != rdx_98 {
        let len_98 = rd::<u32>(src_98_obj + 0xa0) as usize;
        let end_98 = rcx_98.wrapping_add(len_98.wrapping_mul(0x0c));
        if rdx_98 == end_98 {
            wr::<u32>(src_98_obj + 0xa0, 0);
        } else {
            let rem_bytes = end_98.wrapping_sub(rdx_98) as u64;
            exe::call_va!(0x1_4183_acc8, (rcx_98, rdx_98, rem_bytes));
            let sub_cnt = ((rdi_bytes_18 as isize) / 0x0c) as u32;
            wr::<u32>(
                src_98_obj + 0xa0,
                rd::<u32>(src_98_obj + 0xa0).wrapping_sub(sub_cnt),
            );
        }
    }

    // Remove transferred prefix from src + 0x68 (0x50-byte elements)
    let src_68_obj = rd::<usize>(param_1 + 0x08);
    let rcx_68 = rd::<usize>(src_68_obj + 0x68);
    let r10_bytes_20 = (rd::<u64>(param_1 + 0x20) as usize).wrapping_mul(0x50);
    let rdx_68 = rcx_68.wrapping_add(r10_bytes_20);
    if rcx_68 != rdx_68 {
        let len_68 = rd::<u32>(src_68_obj + 0x70) as usize;
        let end_68 = rcx_68.wrapping_add(len_68.wrapping_mul(0x50));
        if rdx_68 == end_68 {
            wr::<u32>(src_68_obj + 0x70, 0);
        } else {
            let mut src_p = rdx_68;
            let mut dst_p = rcx_68;
            while src_p != end_68 {
                let chunk = rd::<[u8; 0x50]>(src_p);
                wr::<[u8; 0x50]>(dst_p, chunk);
                src_p = src_p.wrapping_add(0x50);
                dst_p = dst_p.wrapping_add(0x50);
            }
            let sub_cnt = ((rdx_68.wrapping_sub(rcx_68) as isize) / 0x50) as u32;
            wr::<u32>(
                src_68_obj + 0x70,
                rd::<u32>(src_68_obj + 0x70).wrapping_sub(sub_cnt),
            );
        }
    }

    // Remove transferred prefix from src + 0x78 (0xd0-byte elements)
    let src_78_obj = rd::<usize>(param_1 + 0x08);
    let rcx_78 = rd::<usize>(src_78_obj + 0x78);
    let rdi_bytes_28 = (rd::<u64>(param_1 + 0x28) as usize).wrapping_mul(0xd0);
    let rdx_78 = rcx_78.wrapping_add(rdi_bytes_28);
    if rcx_78 != rdx_78 {
        let len_78 = rd::<u32>(src_78_obj + 0x80) as usize;
        let end_78 = rcx_78.wrapping_add(len_78.wrapping_mul(0xd0));
        if rdx_78 == end_78 {
            wr::<u32>(src_78_obj + 0x80, 0);
        } else {
            let mut src_p = rdx_78;
            let mut dst_p = rcx_78;
            while src_p != end_78 {
                let tag = rd::<u32>(src_p);
                let tail = rd::<[u8; 0xc8]>(src_p + 0x08);
                wr::<u32>(dst_p, tag);
                wr::<[u8; 0xc8]>(dst_p + 0x08, tail);
                src_p = src_p.wrapping_add(0xd0);
                dst_p = dst_p.wrapping_add(0xd0);
            }
            let sub_cnt = ((rdx_78.wrapping_sub(rcx_78) as isize) / 0xd0) as u32;
            wr::<u32>(
                src_78_obj + 0x80,
                rd::<u32>(src_78_obj + 0x80).wrapping_sub(sub_cnt),
            );
        }
    }

    let s0 = rd::<usize>(param_1 + 0x08);
    wr::<u64>(s0 + 0xb8, rd::<u64>(s0 + 0xb8).wrapping_sub(rd::<u64>(param_1 + 0x10)));
    let s1 = rd::<usize>(param_1 + 0x08);
    wr::<u64>(s1 + 0xc0, rd::<u64>(s1 + 0xc0).wrapping_sub(rd::<u64>(param_1 + 0x18)));
    let s2 = rd::<usize>(param_1 + 0x08);
    wr::<u64>(s2 + 0xc8, rd::<u64>(s2 + 0xc8).wrapping_sub(rd::<u64>(param_1 + 0x20)));
    let s3 = rd::<usize>(param_1 + 0x08);
    wr::<u64>(s3 + 0xd0, rd::<u64>(s3 + 0xd0).wrapping_sub(rd::<u64>(param_1 + 0x28)));

    let dst_depth = rd::<u32>(dst_lock + 0x0c).wrapping_sub(1);
    wr::<u32>(dst_lock + 0x0c, dst_depth);
    if dst_depth == 0 {
        crate::simd_ops::atomic_xchg_u32((dst_lock + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (dst_lock));
    }

    let src_depth = rd::<u32>(src_lock + 0x0c).wrapping_sub(1);
    wr::<u32>(src_lock + 0x0c, src_depth);
    if src_depth == 0 {
        crate::simd_ops::atomic_xchg_u32((src_lock + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (src_lock));
    }
}

/// `unknown_142245590`
/// Original: 0x142245590 (293 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_142245590(param_1: usize) {
    let cur_b0 = rd::<f32>(param_1 + 0xb0);
    let g_b0_ptr = exe::ptr::<u32>(0x1_4386_84b0) as usize;
    let g_b0 = rd::<f32>(g_b0_ptr);
    if cur_b0 != g_b0 {
        wr::<u32>(param_1 + 0xb0, rd::<u32>(g_b0_ptr));
        for i in 0..4usize {
            let xmm3_bits = rd::<u32>(param_1 + 0xb0) as u128;
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_4224_5740,
                param_1 as u64,
                0x10,
                i as u64,
                0,
                param_1.wrapping_add(0x50 + i * 8) as u64,
                0,
                0,
                0,
                [0, 0, 0, xmm3_bits],
            );
        }
    }

    let cur_b4 = rd::<f32>(param_1 + 0xb4);
    let g_b4_ptr = exe::ptr::<u32>(0x1_4386_8470) as usize;
    let g_b4 = rd::<f32>(g_b4_ptr);
    if cur_b4 != g_b4 {
        wr::<u32>(param_1 + 0xb4, rd::<u32>(g_b4_ptr));
        for i in 0..4usize {
            let xmm3_bits = rd::<u32>(param_1 + 0xb4) as u128;
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_4224_5740,
                param_1 as u64,
                0x06,
                i as u64,
                0,
                param_1.wrapping_add(0x70 + i * 8) as u64,
                0,
                0,
                0,
                [0, 0, 0, xmm3_bits],
            );
        }
    }

    let cur_b8 = rd::<f32>(param_1 + 0xb8);
    let g_b8_ptr = exe::ptr::<u32>(0x1_4386_8490) as usize;
    let g_b8 = rd::<f32>(g_b8_ptr);
    if cur_b8 != g_b8 {
        wr::<u32>(param_1 + 0xb8, rd::<u32>(g_b8_ptr));
        for i in 0..4usize {
            let xmm3_bits = rd::<u32>(param_1 + 0xb8) as u128;
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_4224_5740,
                param_1 as u64,
                0x04,
                i as u64,
                0,
                param_1.wrapping_add(0x90 + i * 8) as u64,
                0,
                0,
                0,
                [0, 0, 0, xmm3_bits],
            );
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14220c7e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DESTROYED: (usize, usize, u32) = (0, 0, 0);

    unsafe extern "win64" fn mock_f860(ptr: usize) {
        DESTROYED.0 = ptr;
    }
    unsafe extern "win64" fn mock_2de0(ptr: usize) {
        DESTROYED.1 = ptr;
    }
    unsafe extern "win64" fn mock_ca30(rbx: usize) {
        DESTROYED.2 += 1;
        wr::<u64>(rbx + 0x40, 0x5555_6666);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_f860, mock_f860 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4220_ca30, mock_ca30 as *const ())?;

    let g_ptr = exe::ptr::<usize>(0x1_43a0_0188) as usize;
    let saved_g = rd::<usize>(g_ptr);

    let run = |f: F, changed: bool, has_old: bool| -> (
        (usize, usize, u32),
        u64,
        u64,
        [u32; 2],
        [(u64, u8); 2],
    ) {
        DESTROYED = (0, 0, 0);
        let mut state = vec![0u8; 0xc100];
        let mut table = vec![0u8; 0x38 * 4];
        let mut out_dims: [u32; 2] = [0, 0];
        let mut sub0: [u64; 2] = [0x1111, 0];
        let mut sub1: [u64; 2] = [0x5555_6666, 0];

        let sp = state.as_mut_ptr() as usize;
        let tp = table.as_mut_ptr() as usize;

        wr::<u32>(sp + 0x20, 1920);
        wr::<u32>(sp + 0x24, 1080);
        wr::<usize>(sp + 0x40, if has_old { 0xdead_beef } else { 0 });
        wr::<usize>(sp + 0x48, out_dims.as_mut_ptr() as usize);
        wr::<u64>(sp + 0xc088, 0xdddd);
        wr::<u64>(sp + 0xc090, 1);
        wr::<u64>(sp + 0xc0a0, 4);
        wr::<u64>(sp + 0xc0a8, 0xeeee);
        wr::<usize>(sp + 0xc0e0, tp);

        // Slot 0: empty
        wr::<u64>(tp, 0xeeee);
        // Slot 1: valid -> sub0
        wr::<u64>(tp + 0x38, 10);
        wr::<usize>(tp + 0x38 + 0x18, sub0.as_mut_ptr() as usize);
        // Slot 2: deleted
        wr::<u64>(tp + 0x70, 0xdddd);
        // Slot 3: valid -> sub1
        wr::<u64>(tp + 0xa8, 20);
        wr::<usize>(tp + 0xa8 + 0x18, sub1.as_mut_ptr() as usize);

        let new_dims: [u32; 2] = if changed { [2560, 1440] } else { [1920, 1080] };
        wr::<usize>(g_ptr, sp);
        f(0, new_dims.as_ptr() as usize);
        wr::<usize>(g_ptr, saved_g);

        (
            DESTROYED,
            rd::<u64>(sp + 0x20),
            rd::<u64>(sp + 0x40),
            out_dims,
            [
                (sub0[0], rd::<u8>((sub0.as_ptr() as usize) + 0x08)),
                (sub1[0], rd::<u8>((sub1.as_ptr() as usize) + 0x08)),
            ],
        )
    };

    for (changed, has_old) in [(false, true), (true, false), (true, true)] {
        let o = run(f_o, changed, has_old);
        let n = run(f_n, changed, has_old);
        crate::diff_eq!(o, n, "14220c7e0 changed={} has_old={}", changed, has_old);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14222ec60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ED20_RET: u32 = 0;
    static mut ED20_ARG: u32 = 0;

    unsafe extern "win64" fn mock_ed20(_p1: usize, p2: u32) -> u32 {
        ED20_ARG = p2;
        ED20_RET
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_ed20, mock_ed20 as *const ())?;

    let g_ptr = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let saved_g = rd::<usize>(g_ptr);

    let run = |f: F, mode: u8| -> (u8, u32) {
        ED20_ARG = 0xff;
        ED20_RET = if mode == 0 { 2 } else { 99 };

        let mut g_obj = [0u8; 0x1c0];
        let map_1a8: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut r8_obj = [0u8; 0x280];
        let mut p1_obj = [0u8; 0xa0];

        let gp = g_obj.as_mut_ptr() as usize;
        let r8p = r8_obj.as_mut_ptr() as usize;
        let p1p = p1_obj.as_mut_ptr() as usize;

        wr::<usize>(gp + 0x1a8, map_1a8.as_ptr() as usize);
        wr::<usize>(p1p + 0x90, r8p);
        wr::<u32>(r8p + 0x34, u32::from(mode < 2));
        wr::<u8>(r8p + 0x272, if mode == 2 { 0 } else { 3 });
        wr::<u8>(r8p + 0x270, if mode == 4 { 2 } else { 5 });
        wr::<i32>(r8p + 0x1c8 + 2 * 4, if mode == 3 { 0 } else { 4 });

        wr::<usize>(g_ptr, gp);
        let ret = f(p1p, 2);
        wr::<usize>(g_ptr, saved_g);
        (ret, ED20_ARG)
    };

    for mode in 0..6u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14222ec60 mode={}", mode);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_14222ed20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_ptr = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let saved_g = rd::<usize>(g_ptr);

    let run = |f: F, target: u32, flag_4c: u8| -> u32 {
        let mut g_obj = [0u8; 0x1c0];
        let map_1a8: [u8; 8] = [10, 20, 30, 20, 40, 0, 0, 0];
        let mut item1 = [0u8; 0x60];
        let mut item3 = [0u8; 0x60];
        wr::<u8>(item1.as_mut_ptr() as usize + 0x4c, flag_4c);
        wr::<u8>(item3.as_mut_ptr() as usize + 0x4c, 0);

        let ptrs_18: [usize; 5] = [
            0,
            item1.as_mut_ptr() as usize,
            0,
            item3.as_mut_ptr() as usize,
            0,
        ];
        let ids_80: [u32; 4] = [u32::MAX, 0, 1, 3];

        let mut rdx_obj = [0u8; 0x280];
        let mut p1_obj = [0u8; 0xa0];

        let gp = g_obj.as_mut_ptr() as usize;
        let rdxp = rdx_obj.as_mut_ptr() as usize;
        let p1p = p1_obj.as_mut_ptr() as usize;

        wr::<usize>(gp + 0x18, ptrs_18.as_ptr() as usize);
        wr::<usize>(gp + 0x1a8, map_1a8.as_ptr() as usize);

        wr::<u8>(rdxp + 0x272, 2);
        wr::<u8>(rdxp + 0x270, 25);
        wr::<i32>(rdxp + 0x1c8, 1);
        wr::<i32>(rdxp + 0x1cc, 1);

        wr::<usize>(p1p + 0x80, ids_80.as_ptr() as usize);
        wr::<u32>(p1p + 0x88, 4);
        wr::<usize>(p1p + 0x90, rdxp);

        wr::<usize>(g_ptr, gp);
        let ret = f(p1p, target);
        wr::<usize>(g_ptr, saved_g);
        ret
    };

    for (target, flag_4c) in [(20u32, 0u8), (20, 1), (30, 0), (99, 0)] {
        let o = run(f_o, target, flag_4c);
        let n = run(f_n, target, flag_4c);
        crate::diff_eq!(o, n, "14222ed20 target={} flag={}", target, flag_4c);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1422391f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOCK_COUNTS: (u32, u32) = (0, 0);

    unsafe extern "win64" fn mock_tid() -> u32 {
        0x1234
    }
    unsafe extern "win64" fn mock_acq(_lock: usize) {
        LOCK_COUNTS.0 += 1;
    }
    unsafe extern "win64" fn mock_rel(_lock: usize) {
        LOCK_COUNTS.1 += 1;
    }
    unsafe extern "win64" fn mock_worker() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_grow_88(vec_ptr: usize, cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
    }
    unsafe extern "win64" fn mock_resize_98(vec_ptr: usize, new_len: u64) {
        wr::<u32>(vec_ptr + 0x08, new_len as u32);
    }
    unsafe extern "win64" fn mock_grow_68(vec_ptr: usize, cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
    }
    unsafe extern "win64" fn mock_grow_78(vec_ptr: usize, cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
    }

    let _i1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _i3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_worker as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40fb_1b60, mock_grow_88 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4086_b290, mock_resize_98 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412e_e6f0, mock_grow_68 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_412e_e880, mock_grow_78 as *const ())?;

    let g_aa00 = exe::ptr::<usize>(0x1_43c4_aa00) as usize;
    let saved_aa00 = rd::<usize>(g_aa00);
    wr::<usize>(g_aa00, 0);

    let run = |f: F, small_cap: bool| -> (
        (u32, u32),
        [u32; 8],
        [u64; 4],
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
    ) {
        LOCK_COUNTS = (0, 0);

        let mut dst_obj = vec![0u8; 0xd8];
        let mut src_obj = vec![0u8; 0xd8];
        let mut dst_88 = vec![0xaa_u8; 0x20 * 4];
        let mut src_88 = vec![0x11_u8; 0x20 * 3];
        let mut dst_98 = vec![0xbb_u8; 0x0c * 4];
        let mut src_98 = vec![0x22_u8; 0x0c * 3];
        let mut dst_68 = vec![0xcc_u8; 0x50 * 4];
        let mut src_68 = vec![0x33_u8; 0x50 * 3];
        let mut dst_78 = vec![0xdd_u8; 0xd0 * 4];
        let mut src_78 = vec![0x44_u8; 0xd0 * 3];

        for i in 0..3usize {
            src_88[i * 0x20] = (0x10 + i) as u8;
            src_98[i * 0x0c] = (0x20 + i) as u8;
            src_68[i * 0x50] = (0x30 + i) as u8;
            let e = src_78.as_mut_ptr() as usize + i * 0xd0;
            wr::<u32>(e, if i == 0 { 5 } else { 2 });
            wr::<u32>(e + 0x04, 0x7777_0000 + i as u32);
            wr::<u32>(e + 0x10, 100 + i as u32);
            wr::<u32>(e + 0x18, 200 + i as u32);
            wr::<u64>(e + 0xb8, if small_cap { 1 } else { 0 });
        }

        let dp = dst_obj.as_mut_ptr() as usize;
        let sp = src_obj.as_mut_ptr() as usize;
        let cap = if small_cap { 1u32 } else { 4u32 };

        wr::<usize>(dp + 0x68, dst_68.as_mut_ptr() as usize);
        wr::<u32>(dp + 0x70, 1);
        wr::<u32>(dp + 0x74, cap);
        wr::<usize>(dp + 0x78, dst_78.as_mut_ptr() as usize);
        wr::<u32>(dp + 0x80, 1);
        wr::<u32>(dp + 0x84, cap);
        wr::<usize>(dp + 0x88, dst_88.as_mut_ptr() as usize);
        wr::<u32>(dp + 0x90, 1);
        wr::<u32>(dp + 0x94, cap);
        wr::<usize>(dp + 0x98, dst_98.as_mut_ptr() as usize);
        wr::<u32>(dp + 0xa0, 2);
        wr::<u32>(dp + 0xa4, 4);
        wr::<u32>(dp + 0xb0, u32::MAX);

        wr::<usize>(sp + 0x68, src_68.as_mut_ptr() as usize);
        wr::<u32>(sp + 0x70, 2);
        wr::<usize>(sp + 0x78, src_78.as_mut_ptr() as usize);
        wr::<u32>(sp + 0x80, 2);
        wr::<usize>(sp + 0x88, src_88.as_mut_ptr() as usize);
        wr::<u32>(sp + 0x90, 2);
        wr::<usize>(sp + 0x98, src_98.as_mut_ptr() as usize);
        wr::<u32>(sp + 0xa0, 2);
        wr::<u32>(sp + 0xb0, u32::MAX);
        wr::<u64>(sp + 0xb8, 10);
        wr::<u64>(sp + 0xc0, 20);
        wr::<u64>(sp + 0xc8, 30);
        wr::<u64>(sp + 0xd0, 40);

        let mut p1: [u64; 6] = [dp as u64, sp as u64, 1, 1, 1, 1];
        f(p1.as_mut_ptr() as usize);

        (
            LOCK_COUNTS,
            [
                rd::<u32>(dp + 0x70),
                rd::<u32>(dp + 0x80),
                rd::<u32>(dp + 0x90),
                rd::<u32>(dp + 0xa0),
                rd::<u32>(sp + 0x70),
                rd::<u32>(sp + 0x80),
                rd::<u32>(sp + 0x90),
                rd::<u32>(sp + 0xa0),
            ],
            [
                rd::<u64>(sp + 0xb8),
                rd::<u64>(sp + 0xc0),
                rd::<u64>(sp + 0xc8),
                rd::<u64>(sp + 0xd0),
            ],
            dst_78,
            src_78,
            dst_68,
            src_68,
        )
    };

    for small_cap in [false, true] {
        let o = run(f_o, small_cap);
        let n = run(f_n, small_cap);
        if o != n {
            wr::<usize>(g_aa00, saved_aa00);
        }
        crate::diff_eq!(o, n, "1422391f0 small_cap={}", small_cap);
    }
    wr::<usize>(g_aa00, saved_aa00);
    Ok(2)
}

pub unsafe fn diff_unknown_142245590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLS: Vec<(u32, u32, u32, usize)> = Vec::new();
    static mut BASE_PTR: usize = 0;

    unsafe extern "win64" fn mock_5740(
        _rcx: usize,
        edx: u32,
        r8d: u32,
        _r9: usize,
        arg5: usize,
    ) {
        let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
        CALLS.push((edx, r8d, xmm3 as u32, arg5.wrapping_sub(BASE_PTR)));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_5740, mock_5740 as *const ())?;

    let g_b0 = exe::ptr::<u32>(0x1_4386_84b0) as usize;
    let g_b4 = exe::ptr::<u32>(0x1_4386_8470) as usize;
    let g_b8 = exe::ptr::<u32>(0x1_4386_8490) as usize;
    let saved = (rd::<u32>(g_b0), rd::<u32>(g_b4), rd::<u32>(g_b8));
    wr::<f32>(g_b0, 1.5);
    wr::<f32>(g_b4, 2.5);
    wr::<f32>(g_b8, 3.5);

    let run = |f: F, same: bool| -> ([u32; 3], Vec<(u32, u32, u32, usize)>) {
        CALLS.clear();
        let mut obj = [0u8; 0xc0];
        let op = obj.as_mut_ptr() as usize;
        BASE_PTR = op;
        if same {
            wr::<f32>(op + 0xb0, 1.5);
            wr::<f32>(op + 0xb4, 2.5);
            wr::<f32>(op + 0xb8, 3.5);
        } else {
            wr::<f32>(op + 0xb0, 0.5);
            wr::<f32>(op + 0xb4, 2.5);
            wr::<f32>(op + 0xb8, -1.0);
        }
        f(op);
        (
            [
                rd::<u32>(op + 0xb0),
                rd::<u32>(op + 0xb4),
                rd::<u32>(op + 0xb8),
            ],
            CALLS.clone(),
        )
    };

    for same in [true, false] {
        let o = run(f_o, same);
        let n = run(f_n, same);
        if o != n {
            wr::<u32>(g_b0, saved.0);
            wr::<u32>(g_b4, saved.1);
            wr::<u32>(g_b8, saved.2);
        }
        crate::diff_eq!(o, n, "142245590 same={}", same);
    }

    wr::<u32>(g_b0, saved.0);
    wr::<u32>(g_b4, saved.1);
    wr::<u32>(g_b8, saved.2);
    Ok(2)
}
