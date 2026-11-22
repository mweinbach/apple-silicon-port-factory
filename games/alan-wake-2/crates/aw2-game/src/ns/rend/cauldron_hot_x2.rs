//! Decompiled hot functions shard X2 (`0x14104ea10..0x1415ef8b0`).

use crate::exe::{self, rd, wr};

/// `unknown_14104ea10`
/// Original: 0x14104ea10 (257 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14104ea10(param_1: usize, param_2: u64, param_3: u32) {
    let mut key: u64 = param_2;
    let mut it_a = [0u64; 9];
    let mut it_b = [0u64; 9];

    let _ = exe::call_va!(
        0x1_4104_ebe0,
        (
            param_1,
            it_a.as_mut_ptr() as usize,
            (&mut key as *mut u64) as usize
        ) -> usize
    );
    let _ = exe::call_va!(0x1_4104_eeb0, (param_1, it_b.as_mut_ptr() as usize) -> usize);

    if it_a[1] != it_b[1]
        || it_a[2] != it_b[2]
        || it_a[3] != it_b[3]
        || (it_a[3] != it_a[2] && it_a[4] != it_b[4])
    {
        let r10 = it_a[4] as usize;
        let mut cur = rd::<u32>(r10 + 0x08);
        if cur != 0xffff_ffff {
            let mut prev: u32 = 0xffff_ffff;
            let nodes_base = rd::<usize>(param_1 + 0xc0);
            loop {
                let idx = (cur & 0x00ff_ffff) as usize;
                let next = rd::<u32>(nodes_base.wrapping_add(idx << 5).wrapping_add(0x08));
                if cur == param_3 {
                    if prev != 0xffff_ffff {
                        let prev_idx = (prev & 0x00ff_ffff) as usize;
                        wr::<u32>(
                            nodes_base.wrapping_add(prev_idx << 5).wrapping_add(0x08),
                            next,
                        );
                    } else {
                        let head_ptr = exe::call_va!(
                            0x1_4104_ee00,
                            (param_1, (&mut key as *mut u64) as usize) -> usize
                        );
                        wr::<u32>(head_ptr, next);
                    }
                    return;
                }
                prev = cur;
                cur = next;
                if next == 0xffff_ffff {
                    return;
                }
            }
        }
    }
}

/// `unknown_141092240`
/// Original: 0x141092240 (213 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141092240(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let mut buf = [0u64; 6];
        exe::call_va!(
            0x1_4109_3a50,
            (
                buf.as_mut_ptr() as usize,
                param_2,
                block as u64,
                count as u64,
                world as u64
            )
        );
        let r14 = buf[3];
        if r14 != 0 {
            let rbp = buf[1];
            let rsi = buf[4];
            let rdi = buf[5];
            for idx in 0..r14 {
                buf[0] = rsi;
                buf[1] = rdi;
                buf[2] = rbp;
                buf[3] = idx;
                exe::call_va!(0x1_41f2_c190, (buf.as_mut_ptr() as usize));
            }
        }
    }
}

/// `unknown_1411f36a0`
/// Original: 0x1411f36a0 (122 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f36a0(
    param_1: usize,
    param_2: u64,
    param_3: u32,
) -> i32 {
    let mut r10: u64 = 0;
    let mut rax: u64 = param_2;
    if param_2 != 0 {
        loop {
            let r9 = rax >> 1;
            let r8 = r10.wrapping_sub(r9).wrapping_add(rax);
            let mid = r9.wrapping_add(r10) as usize;
            let elem = rd::<u32>(param_1.wrapping_add(mid * 4));
            rax = r9;
            if param_3 >= elem {
                r10 = r8;
            }
            if r9 == 0 {
                break;
            }
        }
    }
    let cand = r10.wrapping_sub(1);
    let edi: i32 = if cand < param_2 { cand as i32 } else { 0 };
    let dummy: u32 = !param_3;
    let base_ptr = if param_2 != 0 {
        param_1
    } else {
        (&dummy as *const u32) as usize
    };
    let idx_off = ((edi as isize) * 4) as usize;
    if param_3 == rd::<u32>(base_ptr.wrapping_add(idx_off)) {
        edi
    } else {
        -1
    }
}

/// `rend_1412ebcc0`
/// Original: 0x1412ebcc0 (327 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412ebcc0(param_1: usize, param_2: u32) {
    let old_cap = rd::<u32>(param_1 + 0x14);
    if param_2 <= old_cap {
        return;
    }

    let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
    let old_ptr = rd::<usize>(param_1);
    if new_cap < 8 {
        new_cap = 8;
    }
    if new_cap < param_2 {
        new_cap = param_2;
    }

    let new_ptr = exe::call_va!(0x1_401d_ecf0, (param_1, new_cap as u64) -> usize);
    let mut i: u32 = 0;
    if rd::<u32>(param_1 + 0x10) > 0 {
        loop {
            let cap = rd::<u32>(param_1 + 0x14) as u64;
            let head = rd::<u32>(param_1 + 0x08) as u64;
            let src_idx = (head.wrapping_add(i as u64) % cap) as usize;
            let src_base = rd::<usize>(param_1);
            let val = rd::<u32>(src_base.wrapping_add(src_idx * 4));
            wr::<u32>(new_ptr.wrapping_add((i as usize) * 4), val);
            i = i.wrapping_add(1);
            if i >= rd::<u32>(param_1 + 0x10) {
                break;
            }
        }
    }

    if old_ptr != 0 {
        let rcx = old_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = old_ptr.wrapping_sub(rcx) >> 16;
        let hdr_raw = rcx
            .wrapping_add(0x108)
            .wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if exe::teb() == rd::<usize>(rcx + 0x100) {
            if rd::<u8>(rdx + 0x0e) == 0 {
                wr::<u64>(old_ptr, rd::<u64>(rdx + 0x20));
                let new_cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
                wr::<u32>(rdx + 0x18, new_cnt);
                wr::<usize>(rdx + 0x20, old_ptr);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (rdx));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (rcx, rdx, 1u8, old_ptr));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (rcx, rdx, 0u8, old_ptr));
        }
    }

    let count = rd::<u32>(param_1 + 0x10);
    wr::<usize>(param_1, new_ptr);
    wr::<u32>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x14, new_cap);
    wr::<u32>(param_1 + 0x0c, count);
}

/// `rend_1412f7fb0`
/// Original: 0x1412f7fb0 (94 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412f7fb0(param_1: usize) -> usize {
    let r10 = rd::<u64>(param_1 + 0x10);
    let r8 = rd::<usize>(param_1);
    let r9 = r10.wrapping_add(0x3f) >> 6;
    if r10 != 0 {
        let rdx = r8.wrapping_sub(8).wrapping_add((r9 as usize).wrapping_mul(8));
        if r8 != rdx {
            let mut rax = r8;
            while rax != rdx {
                wr::<u64>(rax, 0);
                rax = rax.wrapping_add(8);
            }
        }
        let shift = (!(r10.wrapping_sub(1)) as u32) & 0x3f;
        let mask = !(u64::MAX >> shift);
        let last_ptr = r8.wrapping_add((r9 as usize).wrapping_mul(8)).wrapping_sub(8);
        wr::<u64>(last_ptr, rd::<u64>(last_ptr) & mask);
    }
    param_1
}

/// `rend_14130b910`
/// Original: 0x14130b910 (316 bytes, Throws::No)
pub unsafe extern "win64" fn rend_14130b910(param_1: usize, param_2: u32) -> u64 {
    let alloc_res = exe::call_va!(0x1_4222_9a00, (param_1 + 0xd0, param_2) -> u32);
    if alloc_res != 0xffff_ffff {
        return alloc_res as u64;
    }

    let mut rsi = rd::<u64>(param_1 + 0x58);
    let r14d = rd::<u32>(param_1 + 0xd0);
    let mut edi = r14d.wrapping_add(param_2);
    let ebp: u32 = if (edi as u64) < rsi {
        rsi as u32
    } else {
        let rbp_full = rsi.wrapping_sub(1).wrapping_add(edi as u64);
        rbp_full.wrapping_sub(rbp_full % rsi) as u32
    };

    let mut r12d = ebp.wrapping_sub(r14d);
    let r8 = (r12d as u64).wrapping_add(rd::<u64>(param_1 + 0x90));
    if r8 >= rsi {
        let t = rsi.wrapping_add(r8).wrapping_sub(1);
        rsi = t.wrapping_sub(t % rsi);
    }

    wr::<u64>(param_1 + 0x90, r8);
    exe::call_va!(0x1_4130_ca90, (param_1 + 0x98, rsi));
    exe::call_va!(0x1_4130_c930, (param_1 + 0x98));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0xa8, rsi));
    wr::<u32>(param_1 + 0xd0, ebp);

    r12d = r12d.wrapping_sub(param_2);
    if r12d != 0 {
        let rax = exe::call_va!(0x1_412f_8d90, (param_1 + 0xd8) -> usize);
        wr::<u32>(rax, edi);
        wr::<u32>(rax + 0x04, r12d);
        if edi < ebp {
            loop {
                let r8_ptr = rd::<usize>(param_1 + 0xa8);
                let bit = edi & 0x3f;
                let word_idx = (edi >> 6) as usize;
                edi = edi.wrapping_add(1);
                let word_ptr = r8_ptr.wrapping_add(word_idx * 8);
                wr::<u64>(word_ptr, rd::<u64>(word_ptr) | (1u64 << bit));
                let end = rd::<u32>(rax).wrapping_add(rd::<u32>(rax + 0x04));
                if edi >= end {
                    break;
                }
            }
        }
    }
    r14d as u64
}

/// `rend_14130ca90`
/// Original: 0x14130ca90 (262 bytes, Throws::No)
pub unsafe extern "win64" fn rend_14130ca90(param_1: usize, param_2: u64) {
    let rsi = rd::<u32>(param_1 + 0x08) as u64;
    let r8 = rsi.wrapping_mul(0x18);
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > cap {
        exe::call_va!(0x1_40a5_b960, (param_1, param_2));
        let mut count = param_2.wrapping_sub(rsi);
        if count != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let mut ptr = rd::<usize>(param_1).wrapping_add(cur_len.wrapping_mul(0x18));
            while count != 0 {
                wr::<u128>(ptr, 0);
                wr::<u64>(ptr + 0x10, 0);
                wr::<u32>(ptr, 0xffff_ffff);
                ptr = ptr.wrapping_add(0x18);
                count -= 1;
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else {
        if param_2 <= rsi {
            if param_2 != rsi {
                wr::<u32>(param_1 + 0x08, param_2 as u32);
            }
            return;
        }
        let mut count = param_2.wrapping_sub(rsi);
        if count != 0 {
            let mut ptr = rd::<usize>(param_1).wrapping_add(r8 as usize);
            while count != 0 {
                wr::<u128>(ptr, 0);
                wr::<u64>(ptr + 0x10, 0);
                wr::<u32>(ptr, 0xffff_ffff);
                ptr = ptr.wrapping_add(0x18);
                count -= 1;
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `rend_1413388f0`
/// Original: 0x1413388f0 (148 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1413388f0(param_1: usize, param_2: u64) {
    let r8 = rd::<u64>(param_1 + 0x90).wrapping_add(param_2);
    let mut rbx = rd::<u64>(param_1 + 0x58);
    if r8 >= rbx {
        let t = rbx.wrapping_add(r8).wrapping_sub(1);
        rbx = t.wrapping_sub(t % rbx);
    }
    wr::<u64>(param_1 + 0x90, r8);
    exe::call_va!(0x1_4133_9a60, (param_1 + 0x98, rbx));
    let len_a0 = rd::<u32>(param_1 + 0xa0) as u64;
    let cap_a4 = rd::<u32>(param_1 + 0xa4) as u64;
    if len_a0.wrapping_mul(0x34) != cap_a4.wrapping_mul(0x34) {
        exe::call_va!(0x1_4133_a670, (param_1 + 0x98, len_a0));
    }
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0xa8, rbx));
}

/// `unknown_1413a0440`
/// Original: 0x1413a0440 (1075 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1413a0440(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: usize,
) {
    let r11 = rd::<usize>(param_2);
    let mut r10 = rd::<usize>(param_1);
    if param_3 > 0x20 {
        let mut rdi = (((param_3 - 0x21) as u64) >> 5).wrapping_add(1);
        while rdi != 0 {
            let r8 = r10.wrapping_add(0x100);
            if r10 != r8 {
                let mut rcx = r10.wrapping_add(8);
                while rcx != r8 {
                    let rbx_val = rd::<u64>(rcx);
                    let mut rdx = rcx;
                    if rbx_val < rd::<u64>(r10) {
                        if r10 != rcx {
                            let mut rax = rcx;
                            let mut r9 = rcx.wrapping_add(8);
                            loop {
                                let v = rd::<u64>(rax.wrapping_sub(8));
                                r9 = r9.wrapping_sub(8);
                                rax = rax.wrapping_sub(8);
                                wr::<u64>(r9, v);
                                if rax == r10 {
                                    break;
                                }
                            }
                        }
                        wr::<u64>(r10, rbx_val);
                    } else {
                        let mut r9_val = rd::<u64>(rcx.wrapping_sub(8));
                        let mut rax = rcx.wrapping_sub(8);
                        while rbx_val < r9_val {
                            wr::<u64>(rdx, r9_val);
                            rdx = rax;
                            r9_val = rd::<u64>(rax.wrapping_sub(8));
                            rax = rax.wrapping_sub(8);
                        }
                        wr::<u64>(rdx, rbx_val);
                    }
                    rcx = rcx.wrapping_add(8);
                }
            }
            r10 = r8;
            rdi -= 1;
        }
    }

    let mut out_dummy: u64 = 0;
    let mut arg_r10: usize = r10;
    let mut arg_r11: usize = r11;
    let _ = exe::call_va!(
        0x1_413a_00c0,
        (
            (&mut out_dummy as *mut u64) as usize,
            (&mut arg_r10 as *mut usize) as usize,
            (&mut arg_r11 as *mut usize) as usize
        ) -> usize
    );
    if param_3 <= 0x20 {
        return;
    }

    let r8_end = rd::<usize>(param_2);
    let mut r9_dst = param_4;
    let mut r10_src = rd::<usize>(param_1);
    let mut rbp_rem: i64 = param_3;
    let mut r15_width: i64 = 0x20;

    loop {
        rbp_rem -= 0x20;
        let rcx_mid = r10_src.wrapping_add(0x100);
        let mut rax_right_len = r15_width;
        let mut rbx_out = r9_dst;
        let mut r11_left = r10_src;
        if rbp_rem < rax_right_len {
            rax_right_len = rbp_rem;
        }
        rbp_rem -= rax_right_len;
        let rdx_right_end = rcx_mid.wrapping_add((rax_right_len as usize).wrapping_mul(8));
        let mut rax_right = rcx_mid;

        loop {
            let rsi_val = rd::<u64>(r11_left);
            r9_dst = rbx_out.wrapping_add(8);
            let rdi_val = rd::<u64>(rax_right);
            if rdi_val < rsi_val {
                rax_right = rax_right.wrapping_add(8);
                wr::<u64>(rbx_out, rdi_val);
                rbx_out = r9_dst;
                if rax_right == rdx_right_end {
                    while r10_src != rcx_mid {
                        let v = rd::<u64>(r10_src);
                        r10_src = r10_src.wrapping_add(8);
                        wr::<u64>(r9_dst, v);
                        r9_dst = r9_dst.wrapping_add(8);
                    }
                    break;
                }
            } else {
                r10_src = r11_left.wrapping_add(8);
                wr::<u64>(rbx_out, rsi_val);
                rbx_out = r9_dst;
                r11_left = r10_src;
                if r10_src == rcx_mid {
                    while rax_right != rdx_right_end {
                        let v = rd::<u64>(rax_right);
                        rax_right = rax_right.wrapping_add(8);
                        wr::<u64>(r9_dst, v);
                        r9_dst = r9_dst.wrapping_add(8);
                    }
                    break;
                }
            }
        }
        r10_src = rdx_right_end;
        if rbp_rem <= r15_width {
            break;
        }
    }
    while r10_src != r8_end {
        let v = rd::<u64>(r10_src);
        r10_src = r10_src.wrapping_add(8);
        wr::<u64>(r9_dst, v);
        r9_dst = r9_dst.wrapping_add(8);
    }

    let r12_temp_end = param_4.wrapping_add((param_3 as usize).wrapping_mul(8));
    loop {
        let mut rax_dst = rd::<usize>(param_1);
        let rsi_width = r15_width * 2;
        let mut rdi_rem = param_3;
        let mut rdx_src = param_4;
        if rsi_width < param_3 {
            loop {
                rdi_rem -= rsi_width;
                let r11_mid = rdx_src.wrapping_add((rsi_width as usize).wrapping_mul(8));
                let mut rcx_right_len = rsi_width;
                let mut r8_right = r11_mid;
                if rdi_rem < rsi_width {
                    rcx_right_len = rdi_rem;
                }
                rdi_rem -= rcx_right_len;
                let rbx_right_end = r11_mid.wrapping_add((rcx_right_len as usize).wrapping_mul(8));
                let mut rcx_dst = rax_dst;

                loop {
                    let r9_val = rd::<u64>(r8_right);
                    let r10_val = rd::<u64>(rdx_src);
                    if r9_val < r10_val {
                        wr::<u64>(rax_dst, r9_val);
                        r8_right = r8_right.wrapping_add(8);
                        rax_dst = rax_dst.wrapping_add(8);
                        if r8_right == rbx_right_end {
                            while rdx_src != r11_mid {
                                let v = rd::<u64>(rdx_src);
                                rdx_src = rdx_src.wrapping_add(8);
                                wr::<u64>(rax_dst, v);
                                rax_dst = rax_dst.wrapping_add(8);
                            }
                            break;
                        }
                        rcx_dst = rax_dst;
                    } else {
                        rdx_src = rdx_src.wrapping_add(8);
                        wr::<u64>(rcx_dst, r10_val);
                        rax_dst = rcx_dst.wrapping_add(8);
                        rcx_dst = rax_dst;
                        if rdx_src == r11_mid {
                            while r8_right != rbx_right_end {
                                let v = rd::<u64>(r8_right);
                                r8_right = r8_right.wrapping_add(8);
                                wr::<u64>(rax_dst, v);
                                rax_dst = rax_dst.wrapping_add(8);
                            }
                            break;
                        }
                    }
                }
                rdx_src = rbx_right_end;
                if rsi_width >= rdi_rem {
                    break;
                }
            }
        }
        while rdx_src != r12_temp_end {
            wr::<u64>(rax_dst, rd::<u64>(rdx_src));
            rax_dst = rax_dst.wrapping_add(8);
            rdx_src = rdx_src.wrapping_add(8);
        }

        r15_width = rsi_width * 2;
        if param_3 <= r15_width {
            return;
        }

        let mut r10_dst = param_4;
        let r9_end = rd::<usize>(param_2);
        let mut rax_src = rd::<usize>(param_1);
        let mut r14_rem = param_3;
        loop {
            r14_rem -= r15_width;
            let rdx_mid = rax_src.wrapping_add((r15_width as usize).wrapping_mul(8));
            let mut rcx_right_len = r15_width;
            let mut r11_left = rax_src;
            if r14_rem < r15_width {
                rcx_right_len = r14_rem;
            }
            r14_rem -= rcx_right_len;
            let r8_right_end = rdx_mid.wrapping_add((rcx_right_len as usize).wrapping_mul(8));
            let mut rcx_right = rdx_mid;

            loop {
                let rsi_val = rd::<u64>(r11_left);
                let rbx_out = r10_dst;
                let rdi_val = rd::<u64>(rcx_right);
                r10_dst = rbx_out.wrapping_add(8);
                if rdi_val < rsi_val {
                    rcx_right = rcx_right.wrapping_add(8);
                    wr::<u64>(rbx_out, rdi_val);
                    if rcx_right == r8_right_end {
                        while rax_src != rdx_mid {
                            let v = rd::<u64>(rax_src);
                            rax_src = rax_src.wrapping_add(8);
                            wr::<u64>(r10_dst, v);
                            r10_dst = r10_dst.wrapping_add(8);
                        }
                        break;
                    }
                } else {
                    rax_src = r11_left.wrapping_add(8);
                    wr::<u64>(rbx_out, rsi_val);
                    r11_left = rax_src;
                    if rax_src == rdx_mid {
                        while rcx_right != r8_right_end {
                            let v = rd::<u64>(rcx_right);
                            rcx_right = rcx_right.wrapping_add(8);
                            wr::<u64>(r10_dst, v);
                            r10_dst = r10_dst.wrapping_add(8);
                        }
                        break;
                    }
                }
            }
            rax_src = r8_right_end;
            if r15_width >= r14_rem {
                break;
            }
        }
        while rax_src != r9_end {
            let v = rd::<u64>(rax_src);
            rax_src = rax_src.wrapping_add(8);
            wr::<u64>(r10_dst, v);
            r10_dst = r10_dst.wrapping_add(8);
        }
    }
}

/// `unknown_1413a0c80`
/// Original: 0x1413a0c80 (218 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1413a0c80(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
) -> usize {
    let mask = rd::<u64>(param_1 + 0x18);
    let ctrl_base = rd::<usize>(param_1);
    let mut r10 = (param_4 >> 7) & mask;
    let h2 = (param_4 as u8) & 0x7f;
    let xmm2 = crate::simd_ops::vpbroadcastb(h2);
    let mut step: u64 = 0;

    loop {
        let xmm1 = rd::<u128>(ctrl_base.wrapping_add(r10 as usize));
        let mut eq_mask =
            crate::simd_ops::vpmovmskb(crate::simd_ops::vpcmpeqb(xmm1, xmm2)) as u32;
        if eq_mask != 0 {
            let slots_base = rd::<usize>(param_1 + 0x08);
            let want_key = rd::<u64>(param_3);
            while eq_mask != 0 {
                let bit = eq_mask.trailing_zeros() as u64;
                let idx = r10.wrapping_add(bit) & mask;
                if rd::<u64>(slots_base.wrapping_add((idx as usize) * 16)) == want_key {
                    wr::<u64>(param_2, idx);
                    wr::<u8>(param_2 + 0x08, 0);
                    return param_2;
                }
                eq_mask &= eq_mask.wrapping_sub(1);
            }
        }

        let empty_mask =
            crate::simd_ops::vpmovmskb(crate::simd_ops::vpsignb(xmm1, xmm1)) as u32;
        if empty_mask != 0 {
            let idx = exe::call_va!(0x1_413a_0fc0, (param_1, param_4) -> u64);
            wr::<u64>(param_2, idx);
            wr::<u8>(param_2 + 0x08, 1);
            return param_2;
        }

        step = step.wrapping_add(0x10);
        r10 = r10.wrapping_add(step) & mask;
    }
}

/// `r_1413aa840`
/// Original: 0x1413aa840 (244 bytes, Throws::No)
pub unsafe extern "win64" fn r_1413aa840(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    exe::call_iat!(0x1_4264_55a8, (param_1));
    let mut found: u8 = 0;
    let num_deleted = rd::<u64>(param_1 + 0x38);
    if rd::<u64>(param_1 + 0x40) != num_deleted {
        let mask = rd::<u64>(param_1 + 0x48).wrapping_sub(1);
        let mut step: u64 = 0;
        let buckets = rd::<usize>(param_1 + 0x68);
        let empty_key = rd::<u64>(param_1 + 0x50);
        let mut idx = mask & param_3;
        let mut entry_off = (idx as usize).wrapping_mul(0x18);
        while rd::<u64>(buckets.wrapping_add(entry_off)) != empty_key {
            let cur_key = rd::<u64>(buckets.wrapping_add(entry_off));
            let is_deleted = num_deleted != 0 && rd::<u64>(param_1 + 0x30) == cur_key;
            if !is_deleted && param_3 == cur_key {
                if idx != u64::MAX {
                    let cap = rd::<u64>(param_1 + 0x48) as usize;
                    let entry_ptr = buckets.wrapping_add((idx as usize).wrapping_mul(0x18));
                    let end_ptr = buckets.wrapping_add(cap.wrapping_mul(0x18));
                    if entry_ptr != end_ptr {
                        let val = rd::<u128>(entry_ptr + 0x08);
                        wr::<u128>(param_2, val);
                        found = 1;
                    }
                }
                break;
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
            entry_off = (idx as usize).wrapping_mul(0x18);
        }
    }
    wr::<u8>(param_2 + 0x10, found);
    exe::call_iat!(0x1_4264_5598, (param_1));
    param_2
}

/// `unknown_1415ef8b0`
/// Original: 0x1415ef8b0 (671 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1415ef8b0(param_1: usize, param_2: usize) -> usize {
    let esi = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c08) as usize);
    let be9 = rd::<u8>(exe::ptr::<u8>(0x1_4393_1be9) as usize);
    if esi == 0 || be9 != 0 {
        exe::call_va!(0x1_415b_1780, (param_1, param_2));
        return param_1;
    }

    let mut task_handles = vec![0u64; (esi as usize).max(2)];
    let mut out_task_group = [0u64; 2];
    let mut graph_ctx = [0u64; 4];
    let mut iter_cur = [0u64; 4];
    let mut iter_end = [0u64; 4];
    let mut slot_c0: usize = 0;
    let mut slot_c8: usize = 0;

    let out_task_group_ptr = out_task_group.as_mut_ptr() as usize;
    let graph_ctx_ptr = graph_ctx.as_mut_ptr() as usize;
    let iter_cur_ptr = iter_cur.as_mut_ptr() as usize;
    let iter_end_ptr = iter_end.as_mut_ptr() as usize;

    let s_graph = exe::ptr::<u8>(0x1_4268_de60) as usize;
    exe::call_va!(
        0x1_415b_17e0,
        (
            graph_ctx_ptr,
            task_handles.as_mut_ptr() as usize,
            esi,
            s_graph,
            1u32
        )
    );

    if (esi as i32) > 0 {
        let s_bus_task = exe::ptr::<u8>(0x1_4268_de78) as usize;
        let vtbl_ded8 = exe::ptr::<u8>(0x1_4268_ded8) as usize;
        for ebx in 0..(esi as i32) {
            let off = (ebx as usize) * 8;
            let rdx_handle_slot = rd::<usize>(graph_ctx_ptr + 0x08).wrapping_add(off);
            exe::call_va!(
                0x1_415b_1e50,
                (
                    (&mut slot_c0 as *mut usize) as usize,
                    rdx_handle_slot,
                    0x10u32,
                    0x8u32,
                    s_bus_task,
                    0u32,
                    1u32
                )
            );
            if slot_c0 != 0 {
                wr::<usize>(slot_c0, vtbl_ded8);
                wr::<u32>(slot_c0 + 0x08, ebx as u32);
            }
            let handle_val = rd::<u64>(rd::<usize>(graph_ctx_ptr + 0x08).wrapping_add(off));
            exe::call_va!(0x1_415b_1c70, (graph_ctx_ptr, handle_val));
        }
    }

    let mut flags_buf = vec![0u8; (esi as usize).max(16)];
    let rdi = flags_buf.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_4183_acc2, (rdi, 0u32, esi as u64) -> usize);

    let g_c20 = exe::ptr::<u8>(0x1_4393_1c20) as usize;
    exe::call_va!(0x1_4164_74c0, (iter_cur_ptr, g_c20));
    loop {
        exe::call_va!(0x1_4164_7570, (iter_end_ptr, g_c20));
        let end_ptr_18 = rd::<u64>(iter_end_ptr + 0x18);
        let r8d = rd::<u32>(iter_cur_ptr + 0x04);
        let edx = rd::<u32>(iter_cur_ptr);
        if rd::<u64>(iter_cur_ptr + 0x18) == end_ptr_18
            && edx == rd::<u32>(iter_end_ptr)
            && r8d == rd::<u32>(iter_end_ptr + 0x04)
        {
            break;
        }
        exe::call_va!(0x1_415b_18f0, (graph_ctx_ptr, edx, r8d));
        let idx = rd::<u32>(iter_cur_ptr) as usize;
        wr::<u8>(rdi.wrapping_add(idx), 1);
        exe::call_va!(0x1_4164_7410, (iter_cur_ptr));
    }

    for ebx in 0..esi {
        if rd::<u8>(rdi.wrapping_add(ebx as usize)) == 0 {
            exe::call_va!(0x1_415b_18d0, (graph_ctx_ptr, ebx, param_2));
        }
    }

    exe::call_va!(0x1_415b_1910, (graph_ctx_ptr, out_task_group_ptr));

    let s_release = exe::ptr::<u8>(0x1_4268_de88) as usize;
    let vtbl_dee8 = exe::ptr::<u8>(0x1_4268_dee8) as usize;
    slot_c0 = usize::MAX;
    exe::call_va!(
        0x1_415b_1e50,
        (
            (&mut slot_c8 as *mut usize) as usize,
            (&mut slot_c0 as *mut usize) as usize,
            0x10u32,
            0x8u32,
            s_release,
            0u32,
            1u32
        )
    );
    if slot_c8 != 0 {
        wr::<usize>(slot_c8, vtbl_dee8);
    }
    slot_c8 = slot_c0;
    slot_c0 = usize::MAX;
    let _ = slot_c0;
    exe::call_va!(
        0x1_415b_1720,
        (
            param_1,
            (&mut slot_c8 as *mut usize) as usize,
            1u32,
            rd::<u64>(out_task_group_ptr + 0x08)
        )
    );
    exe::call_va!(0x1_415b_1860, (graph_ctx_ptr));
    param_1
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14104ea10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUCKET_HDR: [u32; 4] = [0; 4];
    static mut HEAD_SLOT: u32 = 0;
    static mut IT_EQUAL: bool = false;

    unsafe extern "win64" fn mock_ebe0(_p1: usize, out_it: usize, _key_ptr: usize) -> usize {
        wr::<u64>(out_it + 0x08, 10);
        wr::<u64>(out_it + 0x10, 20);
        wr::<u64>(out_it + 0x18, 30);
        wr::<usize>(out_it + 0x20, core::ptr::addr_of_mut!(BUCKET_HDR) as usize);
        out_it
    }
    unsafe extern "win64" fn mock_eeb0(_p1: usize, out_it: usize) -> usize {
        wr::<u64>(out_it + 0x08, 10);
        wr::<u64>(out_it + 0x10, 20);
        wr::<u64>(out_it + 0x18, 30);
        let ptr_val = if IT_EQUAL {
            core::ptr::addr_of_mut!(BUCKET_HDR) as usize
        } else {
            0x9999
        };
        wr::<usize>(out_it + 0x20, ptr_val);
        out_it
    }
    unsafe extern "win64" fn mock_ee00(_p1: usize, _key_ptr: usize) -> usize {
        core::ptr::addr_of_mut!(HEAD_SLOT) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4104_ebe0, mock_ebe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4104_eeb0, mock_eeb0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4104_ee00, mock_ee00 as *const ())?;

    let run = |f: F, it_eq: bool, target: u32| -> (u32, [u32; 24]) {
        IT_EQUAL = it_eq;
        HEAD_SLOT = 0x0100_0001;
        BUCKET_HDR = [0, 0, 0x0100_0001, 0];
        // 3 nodes of 0x20 bytes (8 u32s each): node 1 -> node 2 -> 0xffff_ffff
        let mut nodes = [0u32; 24];
        nodes[1 * 8 + 2] = 0x0200_0002;
        nodes[2 * 8 + 2] = 0xffff_ffff;

        let mut p1 = [0u8; 0xd0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xc0, nodes.as_mut_ptr() as usize);

        f(p1p, 0x1234_5678, target);
        (HEAD_SLOT, nodes)
    };

    for (it_eq, target) in [
        (true, 0x0100_0001u32),
        (false, 0x0100_0001),
        (false, 0x0200_0002),
        (false, 0x0300_0003),
    ] {
        let o = run(f_o, it_eq, target);
        let n = run(f_n, it_eq, target);
        crate::diff_eq!(o, n, "14104ea10 it_eq={} target={:#x}", it_eq, target);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_141092240(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_PKTS: [u64; 12] = [0; 12];
    static mut LOG_IDX: usize = 0;

    unsafe extern "win64" fn mock_3a50(
        out: usize,
        arch: u16,
        block: u64,
        count: u64,
        world: u64,
    ) -> usize {
        wr::<u64>(out, arch as u64);
        wr::<u64>(out + 0x08, block + 0x10);
        wr::<u64>(out + 0x10, world);
        wr::<u64>(out + 0x18, count);
        wr::<u64>(out + 0x20, block + 0x100);
        wr::<u64>(out + 0x28, block + 0x200);
        out
    }
    unsafe extern "win64" fn mock_c190(pkt: usize) {
        let i = LOG_IDX;
        if i + 4 <= 12 {
            let vals = rd::<[u64; 4]>(pkt);
            LOG_PKTS[i..i + 4].copy_from_slice(&vals);
            LOG_IDX = i + 4;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4109_3a50, mock_3a50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f2_c190, mock_c190 as *const ())?;

    let run = |f: F, arch: u16| -> ([u64; 12], usize) {
        LOG_PKTS = [0; 12];
        LOG_IDX = 0;
        let mut world = vec![0u8; 0x10200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x50_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 3);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        (LOG_PKTS, LOG_IDX)
    };

    for arch in [1u16, 2u16] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        crate::diff_eq!(o, n, "141092240 arch={}", arch);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411f36a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u32) -> i32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let arr: [u32; 7] = [10, 20, 20, 35, 50, 80, 100];
    let ap = arr.as_ptr() as usize;

    let o_empty = f_o(0, 0, 20);
    let n_empty = f_n(0, 0, 20);
    crate::diff_eq!(o_empty, n_empty, "1411f36a0 empty");

    for want in [5u32, 10, 20, 25, 35, 100, 150] {
        let o = f_o(ap, arr.len() as u64, want);
        let n = f_n(ap, arr.len() as u64, want);
        crate::diff_eq!(o, n, "1411f36a0 want={}", want);
    }
    Ok(8)
}

pub unsafe fn diff_rend_1412ebcc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SRC_BUF: [u32; 8] = [0; 8];
    static mut DST_BUF: [u32; 16] = [0; 16];
    static mut REQ_CAP: u64 = 0;

    unsafe extern "win64" fn mock_decf0(p1: usize, cap: u64) -> usize {
        REQ_CAP = cap;
        wr::<usize>(p1, core::ptr::addr_of!(SRC_BUF) as usize);
        core::ptr::addr_of_mut!(DST_BUF) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ecf0, mock_decf0 as *const ())?;

    let run = |f: F, init_cap: u32, head: u32, count: u32, req: u32| -> ([u32; 6], [u32; 16], u64) {
        SRC_BUF = [100, 200, 300, 400, 500, 600, 700, 800];
        DST_BUF = [0; 16];
        REQ_CAP = 0;
        let mut q = [0u32; 6];
        let qp = q.as_mut_ptr() as usize;
        wr::<usize>(qp, 0); // old_ptr = 0 so slab free is skipped while mock_decf0 supplies SRC_BUF
        wr::<u32>(qp + 0x08, head);
        wr::<u32>(qp + 0x0c, (head + count) % init_cap.max(1));
        wr::<u32>(qp + 0x10, count);
        wr::<u32>(qp + 0x14, init_cap);

        f(qp, req);
        wr::<usize>(qp, 0);
        (q, DST_BUF, REQ_CAP)
    };

    for (init_cap, head, count, req) in [(8u32, 2u32, 4u32, 6u32), (4, 3, 3, 5), (8, 5, 6, 15)] {
        let o = run(f_o, init_cap, head, count, req);
        let n = run(f_n, init_cap, head, count, req);
        crate::diff_eq!(
            o,
            n,
            "1412ebcc0 cap={} head={} count={} req={}",
            init_cap,
            head,
            count,
            req
        );
    }
    Ok(3)
}

pub unsafe fn diff_rend_1412f7fb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, num_bits: u64| -> [u64; 4] {
        let mut words = [0xffff_ffff_ffff_ffffu64; 4];
        let mut bs = [0u64; 3];
        let bsp = bs.as_mut_ptr() as usize;
        wr::<usize>(bsp, words.as_mut_ptr() as usize);
        wr::<u64>(bsp + 0x10, num_bits);
        let ret = f(bsp);
        assert_eq!(ret, bsp);
        words
    };

    for bits in [0u64, 1, 13, 63, 64, 65, 135, 192] {
        let o = run(f_o, bits);
        let n = run(f_n, bits);
        crate::diff_eq!(o, n, "1412f7fb0 bits={}", bits);
    }
    Ok(8)
}

pub unsafe fn diff_rend_14130b910(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FAST_RET: u32 = 0xffff_ffff;
    static mut LOG_CALLS: [u64; 4] = [0; 4];
    static mut FREE_SLOT: [u32; 2] = [0; 2];

    unsafe extern "win64" fn mock_9a00(_rcx: usize, rdx: u32) -> u32 {
        LOG_CALLS[0] = rdx as u64;
        FAST_RET
    }
    unsafe extern "win64" fn mock_ca90(_rcx: usize, rdx: u64) {
        LOG_CALLS[1] = rdx;
    }
    unsafe extern "win64" fn mock_c930(_rcx: usize) {
        LOG_CALLS[2] += 1;
    }
    unsafe extern "win64" fn mock_f1e0(_rcx: usize, rdx: u64) {
        LOG_CALLS[3] = rdx;
    }
    unsafe extern "win64" fn mock_8d90(_rcx: usize) -> usize {
        core::ptr::addr_of_mut!(FREE_SLOT) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_9a00, mock_9a00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4130_ca90, mock_ca90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4130_c930, mock_c930 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_412f_8d90, mock_8d90 as *const ())?;

    let run = |f: F, fast: u32, init_d0: u32, req: u32| -> (u64, u64, u32, [u64; 4], [u32; 2], [u64; 2]) {
        FAST_RET = fast;
        LOG_CALLS = [0; 4];
        FREE_SLOT = [0; 2];
        let mut bit_words = [0u64; 2];
        let mut obj = [0u8; 0xf0];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x58, 64);
        wr::<u64>(op + 0x90, 50);
        wr::<usize>(op + 0xa8, bit_words.as_mut_ptr() as usize);
        wr::<u32>(op + 0xd0, init_d0);

        let ret = f(op, req);
        (
            ret,
            rd::<u64>(op + 0x90),
            rd::<u32>(op + 0xd0),
            LOG_CALLS,
            FREE_SLOT,
            bit_words,
        )
    };

    for (fast, init_d0, req) in [(7u32, 10u32, 5u32), (0xffff_ffff, 10, 5), (0xffff_ffff, 60, 20)] {
        let o = run(f_o, fast, init_d0, req);
        let n = run(f_n, fast, init_d0, req);
        crate::diff_eq!(o, n, "14130b910 fast={:#x} d0={} req={}", fast, init_d0, req);
    }
    Ok(3)
}

pub unsafe fn diff_rend_14130ca90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CALLED: u64 = 0;

    unsafe extern "win64" fn mock_b960(vec_ptr: usize, new_cap: u64) {
        GROW_CALLED = new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40a5_b960, mock_b960 as *const ())?;

    let run = |f: F, init_len: u32, init_cap: u32, new_len: u64| -> (u32, u32, u64, [u32; 30]) {
        GROW_CALLED = 0;
        let mut storage = [0x1111_2222u32; 30]; // 5 elements of 0x18 bytes (6 u32s each)
        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, storage.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, init_len);
        wr::<u32>(vp + 0x0c, init_cap);

        f(vp, new_len);
        (
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            GROW_CALLED,
            storage,
        )
    };

    for (len, cap, req) in [(2u32, 4u32, 1u64), (2, 4, 2), (1, 4, 3), (1, 2, 4)] {
        let o = run(f_o, len, cap, req);
        let n = run(f_n, len, cap, req);
        crate::diff_eq!(o, n, "14130ca90 len={} cap={} req={}", len, cap, req);
    }
    Ok(4)
}

pub unsafe fn diff_rend_1413388f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_CALLS: [u64; 3] = [0; 3];

    unsafe extern "win64" fn mock_9a60(_rcx: usize, rdx: u64) {
        LOG_CALLS[0] = rdx;
    }
    unsafe extern "win64" fn mock_a670(_rcx: usize, rdx: u64) {
        LOG_CALLS[1] = rdx;
    }
    unsafe extern "win64" fn mock_f1e0(_rcx: usize, rdx: u64) {
        LOG_CALLS[2] = rdx;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4133_9a60, mock_9a60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4133_a670, mock_a670 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;

    let run = |f: F, add: u64, a0: u32, a4: u32| -> (u64, [u64; 3]) {
        LOG_CALLS = [0; 3];
        let mut obj = [0u8; 0xc0];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x58, 32);
        wr::<u64>(op + 0x90, 10);
        wr::<u32>(op + 0xa0, a0);
        wr::<u32>(op + 0xa4, a4);

        f(op, add);
        (rd::<u64>(op + 0x90), LOG_CALLS)
    };

    for (add, a0, a4) in [(5u64, 15u32, 15u32), (30, 40, 64)] {
        let o = run(f_o, add, a0, a4);
        let n = run(f_n, add, a0, a4);
        crate::diff_eq!(o, n, "1413388f0 add={} a0={} a4={}", add, a0, a4);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1413a0440(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, len: usize| -> (Vec<u64>, Vec<u64>) {
        let mut data: Vec<u64> = (0..len)
            .map(|i| ((i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)) ^ 0xdead_beef_cafe_babe)
            .collect();
        let mut scratch = vec![0u64; len];
        let p_begin = data.as_mut_ptr() as usize;
        let p_end = p_begin.wrapping_add(len * 8);
        f(
            (&p_begin as *const usize) as usize,
            (&p_end as *const usize) as usize,
            len as i64,
            scratch.as_mut_ptr() as usize,
        );
        (data, scratch)
    };

    for len in [12usize, 45, 150] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1413a0440 len={}", len);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1413a0c80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_0fc0(_tbl: usize, hash: u64) -> u64 {
        (hash >> 7) & 31
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_0fc0, mock_0fc0 as *const ())?;

    let run = |f: F, want_key: u64, hash: u64, prefill: bool| -> (u64, u8) {
        let mut ctrl = [0x80u8; 48];
        let mut slots = [0u64; 64];
        let mask: u64 = 31;

        if prefill {
            // Fill first group of 16 bytes as non-empty so it steps to second group
            for i in 0..16 {
                ctrl[i] = 0x11;
            }
            let h2 = (hash as u8) & 0x7f;
            let pos = (((hash >> 7) + 0x10) & mask) as usize;
            ctrl[pos] = h2;
            slots[pos * 2] = want_key ^ 1;
            let pos2 = ((pos + 1) as u64 & mask) as usize;
            ctrl[pos + 1] = h2;
            slots[pos2 * 2] = want_key;
        }

        let mut tbl = [0u64; 4];
        let tp = tbl.as_mut_ptr() as usize;
        wr::<usize>(tp, ctrl.as_mut_ptr() as usize);
        wr::<usize>(tp + 0x08, slots.as_mut_ptr() as usize);
        wr::<u64>(tp + 0x18, mask);

        let mut out = [0u64; 2];
        let op = out.as_mut_ptr() as usize;
        let ret = f(tp, op, (&want_key as *const u64) as usize, hash);
        assert_eq!(ret, op);
        (rd::<u64>(op), rd::<u8>(op + 0x08))
    };

    for (k, h, prefill) in [
        (0x1122_3344_5566_7788u64, 0x05u64 | (0u64 << 7), true),
        (0x99aa_bbcc_ddee_ff00u64, 0x2au64 | (7u64 << 7), false),
    ] {
        let o = run(f_o, k, h, prefill);
        let n = run(f_n, k, h, prefill);
        crate::diff_eq!(o, n, "1413a0c80 prefill={}", prefill);
    }
    Ok(2)
}

pub unsafe fn diff_r_1413aa840(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, search_key: u64| -> (u128, u8) {
        // 4 buckets of 0x18 bytes (3 u64s each)
        // key = 1 (mask & 1 == 1): bucket 1 has deleted tombstone, bucket 2 (step=1, 1+1=2) has key=1, bucket 3 has empty
        let buckets: [u64; 12] = [
            u64::MAX,
            0,
            0,
            u64::MAX - 1, // bucket 1: deleted
            0x1111,
            0x2222,
            1, // bucket 2: key = 1
            0xaaaa_bbbb_cccc_dddd,
            0x1234_5678_9abc_def0,
            u64::MAX, // bucket 3: empty
            0,
            0,
        ];

        let mut map = [0u8; 0x70];
        let mp = map.as_mut_ptr() as usize;
        wr::<u64>(mp + 0x30, u64::MAX - 1); // deleted key
        wr::<u64>(mp + 0x38, 1); // num_deleted = 1
        wr::<u64>(mp + 0x40, 2); // num_used = 2
        wr::<u64>(mp + 0x48, 4); // cap = 4
        wr::<u64>(mp + 0x50, u64::MAX); // empty key
        wr::<usize>(mp + 0x68, buckets.as_ptr() as usize);

        let mut out = [0u64; 3];
        let op = out.as_mut_ptr() as usize;
        let ret = f(mp, op, search_key);
        assert_eq!(ret, op);
        (rd::<u128>(op), rd::<u8>(op + 0x10))
    };

    for k in [1u64, 5u64, u64::MAX - 1] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "1413aa840 k={}", k);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1415ef8b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut TASK_MEM: [[u64; 2]; 4] = [[0; 2]; 4];
    static mut ALLOC_IDX: usize = 0;

    unsafe extern "win64" fn mock_1780(p1: usize, p2: usize) -> usize {
        LOG_SUM = LOG_SUM.wrapping_add(0x1780).wrapping_add(rd::<u64>(p2));
        wr::<u64>(p1, 0x1111);
        p1
    }
    unsafe extern "win64" fn mock_17e0(
        ctx: usize,
        handles_buf: usize,
        count: u32,
        _name: usize,
        flag: u32,
    ) -> usize {
        wr::<u64>(ctx, 0x7777);
        wr::<usize>(ctx + 0x08, handles_buf);
        wr::<u32>(ctx + 0x10, count);
        wr::<u32>(ctx + 0x14, flag);
        wr::<u8>(ctx + 0x18, 0);
        LOG_SUM = LOG_SUM.wrapping_add(count as u64).wrapping_add(flag as u64);
        ctx
    }
    unsafe extern "win64" fn mock_1e50(
        out_obj_ptr: usize,
        out_handle_ptr: usize,
        sz: u32,
        align: u32,
        _name: usize,
        arg6: u32,
        arg7: u32,
    ) {
        let idx = ALLOC_IDX;
        ALLOC_IDX = (idx + 1) & 3;
        let obj_addr = core::ptr::addr_of_mut!(TASK_MEM[idx]) as usize;
        wr::<usize>(out_obj_ptr, obj_addr);
        wr::<u64>(out_handle_ptr, 0x5000 + idx as u64);
        LOG_SUM = LOG_SUM
            .wrapping_add(sz as u64)
            .wrapping_add(align as u64)
            .wrapping_add(arg6 as u64)
            .wrapping_add(arg7 as u64);
    }
    unsafe extern "win64" fn mock_1c70(_ctx: usize, handle_val: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(handle_val);
    }
    unsafe extern "win64" fn mock_18f0(_ctx: usize, edx: u32, r8d: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(0x18f000)
            .wrapping_add((edx as u64) * 10 + r8d as u64);
    }
    unsafe extern "win64" fn mock_18d0(_ctx: usize, ebx: u32, p2: usize) {
        LOG_SUM = LOG_SUM
            .wrapping_add(0x18d000)
            .wrapping_add(ebx as u64)
            .wrapping_add(rd::<u64>(p2));
    }
    unsafe extern "win64" fn mock_1910(_ctx: usize, out_tg: usize) -> usize {
        wr::<u32>(out_tg, 1);
        wr::<u64>(out_tg + 0x08, 0xabcd_0001);
        out_tg
    }
    unsafe extern "win64" fn mock_1720(
        p1: usize,
        handle_ptr: usize,
        r8d: u32,
        r9_tg: u64,
    ) -> usize {
        let h = rd::<u64>(handle_ptr);
        wr::<u32>(p1, r8d);
        wr::<u64>(p1 + 0x08, h.wrapping_add(r9_tg));
        p1
    }
    unsafe extern "win64" fn mock_1860(_ctx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x1860);
    }

    let _m1 = crate::difftest::Mock::new(0x1_415b_1780, mock_1780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_415b_17e0, mock_17e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_415b_1e50, mock_1e50 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_415b_1c70, mock_1c70 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_415b_18f0, mock_18f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_415b_18d0, mock_18d0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_415b_1910, mock_1910 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_415b_1720, mock_1720 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_415b_1860, mock_1860 as *const ())?;

    let g_c08 = exe::ptr::<u32>(0x1_4393_1c08) as usize;
    let g_be9 = exe::ptr::<u8>(0x1_4393_1be9) as usize;
    let g_c20 = exe::ptr::<u8>(0x1_4393_1c20) as usize;
    let s_c08 = rd::<u32>(g_c08);
    let s_be9 = rd::<u8>(g_be9);
    let s_c20 = rd::<[u64; 2]>(g_c20);

    let run = |f: F, count: u32, flag_be9: u8| -> ([u64; 2], u64, [[u64; 2]; 4]) {
        LOG_SUM = 0;
        TASK_MEM = [[0; 2]; 4];
        ALLOC_IDX = 0;
        let bit_words: [u32; 1] = [0b0010]; // edge (0, 1) set in 2x2 bitmatrix
        wr::<u32>(g_c08, count);
        wr::<u8>(g_be9, flag_be9);
        wr::<u32>(g_c20, 2);
        wr::<u32>(g_c20 + 0x04, 2);
        wr::<usize>(g_c20 + 0x08, bit_words.as_ptr() as usize);

        let mut p1 = [0u64; 2];
        let p2: [u64; 2] = [0x42, 0x9999];
        let ret = f(p1.as_mut_ptr() as usize, p2.as_ptr() as usize);
        assert_eq!(ret, p1.as_mut_ptr() as usize);
        (p1, LOG_SUM, TASK_MEM)
    };

    for (cnt, be9) in [(0u32, 0u8), (2, 1), (2, 0)] {
        let o = run(f_o, cnt, be9);
        let n = run(f_n, cnt, be9);
        wr::<u32>(g_c08, s_c08);
        wr::<u8>(g_be9, s_be9);
        wr::<[u64; 2]>(g_c20, s_c20);
        crate::diff_eq!(o, n, "1415ef8b0 cnt={} be9={}", cnt, be9);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4104_ea10, "unknown_14104ea10", rend::cauldron_hot_x2::unknown_14104ea10 as *const (), Throws::Discard, rend::cauldron_hot_x2::diff_unknown_14104ea10),
// Replacement::verified(0x1_4109_2240, "unknown_141092240", rend::cauldron_hot_x2::unknown_141092240 as *const (), rend::cauldron_hot_x2::diff_unknown_141092240),
// Replacement::verified(0x1_411f_36a0, "unknown_1411f36a0", rend::cauldron_hot_x2::unknown_1411f36a0 as *const (), rend::cauldron_hot_x2::diff_unknown_1411f36a0),
// Replacement::verified(0x1_412e_bcc0, "rend_1412ebcc0", rend::cauldron_hot_x2::rend_1412ebcc0 as *const (), rend::cauldron_hot_x2::diff_rend_1412ebcc0),
// Replacement::verified(0x1_412f_7fb0, "rend_1412f7fb0", rend::cauldron_hot_x2::rend_1412f7fb0 as *const (), rend::cauldron_hot_x2::diff_rend_1412f7fb0),
// Replacement::verified(0x1_4130_b910, "rend_14130b910", rend::cauldron_hot_x2::rend_14130b910 as *const (), rend::cauldron_hot_x2::diff_rend_14130b910),
// Replacement::verified(0x1_4130_ca90, "rend_14130ca90", rend::cauldron_hot_x2::rend_14130ca90 as *const (), rend::cauldron_hot_x2::diff_rend_14130ca90),
// Replacement::verified(0x1_4133_88f0, "rend_1413388f0", rend::cauldron_hot_x2::rend_1413388f0 as *const (), rend::cauldron_hot_x2::diff_rend_1413388f0),
// Replacement::verified(0x1_413a_0440, "unknown_1413a0440", rend::cauldron_hot_x2::unknown_1413a0440 as *const (), rend::cauldron_hot_x2::diff_unknown_1413a0440),
// Replacement::verified_throws(0x1_413a_0c80, "unknown_1413a0c80", rend::cauldron_hot_x2::unknown_1413a0c80 as *const (), Throws::Discard, rend::cauldron_hot_x2::diff_unknown_1413a0c80),
// Replacement::verified(0x1_413a_a840, "r_1413aa840", rend::cauldron_hot_x2::r_1413aa840 as *const (), rend::cauldron_hot_x2::diff_r_1413aa840),
// Replacement::verified(0x1_415e_f8b0, "unknown_1415ef8b0", rend::cauldron_hot_x2::unknown_1415ef8b0 as *const (), rend::cauldron_hot_x2::diff_unknown_1415ef8b0),
