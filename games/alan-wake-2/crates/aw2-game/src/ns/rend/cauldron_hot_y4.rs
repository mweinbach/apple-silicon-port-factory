//! Decompiled hot functions shard Y4 (`0x142213400..0x1423d9450`).

use crate::exe::{self, rd, wr};

/// `unknown_142213400`
/// Original: 0x142213400 (1393 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142213400(param_1: usize, param_2: usize, param_3: u64) {
    let ebp = param_3 as i32;
    let r9_init = (ebp as i64) as u64;
    let count_p2 = rd::<u32>(param_2 + 0x08) as u64;
    let total = count_p2.wrapping_sub(1).wrapping_add(r9_init);
    let quot = total / r9_init;
    let rem = total % r9_init;
    let mut num_groups = (quot as i32) as i64;

    let groups_ptr = exe::call_va!(
        0x1_4221_3360,
        (param_1, rem, param_3, num_groups as u64) -> usize
    );

    let mut r12 = rd::<usize>(param_2);
    if rd::<i32>(param_2 + 0x08) > 0 {
        let mut r14d: i32 = 0;
        loop {
            let grp_idx = (r14d / ebp) as isize;
            let rdi = (groups_ptr as isize + (grp_idx << 4)) as usize;
            let r15_off = ((r14d as isize) << 6) as usize;
            let len = rd::<u32>(rdi + 0x08) as u64;
            let cap = rd::<u32>(rdi + 0x0c) as u64;
            if (len << 6) == (cap << 6) {
                let req = (((len << 6) as i64) >> 6).wrapping_add(1) as u64;
                exe::call_va!(0x1_411a_2010, (rdi, req));
            }
            let cur_len = rd::<u32>(rdi + 0x08) as usize;
            let dst = rd::<usize>(rdi).wrapping_add(cur_len << 6);
            let src = r12.wrapping_add(r15_off);
            wr::<[u8; 64]>(dst, rd::<[u8; 64]>(src));
            wr::<u32>(rdi + 0x08, (cur_len as u32).wrapping_add(1));

            r14d = r14d.wrapping_add(1);
            r12 = rd::<usize>(param_2);
            if r14d >= rd::<i32>(param_2 + 0x08) {
                break;
            }
        }
    }

    if num_groups <= 0 {
        wr::<u32>(param_2 + 0x08, 0);
    } else {
        let mut r14: i64 = 0;
        let mut rdi = groups_ptr;
        while r14 < num_groups {
            let len = rd::<u32>(rdi + 0x08) as u64;
            let mut local_beg = rd::<u64>(rdi);
            let mut local_end = local_beg.wrapping_add(len << 6);
            let cmp_fn = exe::ptr::<u8>(0x1_4221_32f0) as usize;
            exe::call_va!(
                0x1_412e_8930,
                (
                    (&mut local_beg as *mut u64) as usize,
                    (&mut local_end as *mut u64) as usize,
                    len,
                    cmp_fn
                )
            );

            let mut ecx: i32 = 1;
            if ebp > 1 {
                let r8 = rd::<usize>(rdi);
                while ecx < ebp {
                    let rdx = ((ecx as isize) << 6) as usize;
                    if rd::<u32>(r8.wrapping_add(rdx) + 0x24) == 0 {
                        break;
                    }
                    let prev_34 = rd::<u32>(r8.wrapping_add(rdx).wrapping_sub(0x0c));
                    if prev_34.wrapping_add(1) != rd::<u32>(r8.wrapping_add(rdx) + 0x34) {
                        let mut err_buf = [0u64; 4];
                        let msg = exe::ptr::<u8>(0x1_433f_04a0) as usize;
                        let err_obj = exe::call_va!(
                            0x1_412e_8690,
                            (err_buf.as_mut_ptr() as usize, msg) -> usize
                        );
                        exe::call_va!(0x1_412e_88c0, (err_obj));
                        return;
                    }
                    ecx = ecx.wrapping_add(1);
                }
            }

            r14 += 1;
            rdi = rdi.wrapping_add(0x10);
        }

        let mut r9 = groups_ptr;
        for _ in 0..num_groups {
            let mut r8d: i32 = 1_000_000;
            if ebp > 0 {
                let rdx = rd::<usize>(r9);
                for j in 0..ebp {
                    let off = ((j as isize) << 6) as usize;
                    if rd::<i32>(rdx.wrapping_add(off) + 0x24) == 0 {
                        break;
                    }
                    let v30 = rd::<i32>(rdx.wrapping_add(off) + 0x30);
                    if v30 < r8d {
                        r8d = v30;
                    }
                }
            }

            if ebp > 0 {
                for j in 0..ebp {
                    let rdx = ((j as isize) << 6) as usize;
                    let base = rd::<usize>(r9);
                    let eax = rd::<i32>(base.wrapping_add(rdx) + 0x24);
                    if eax == 0 {
                        break;
                    }
                    let ecx = rd::<i32>(base.wrapping_add(rdx) + 0x30).wrapping_sub(r8d);
                    wr::<i32>(base.wrapping_add(rdx) + 0x24, eax.wrapping_add(ecx));

                    let base2 = rd::<usize>(r9);
                    let elem = base2.wrapping_add(rdx);
                    let d_ecx = ecx as f64;
                    let d10 = rd::<f64>(elem + 0x10);
                    let d18 = rd::<f64>(elem + 0x18);
                    let d00 = rd::<f64>(elem);
                    let d08 = rd::<f64>(elem + 0x08);
                    wr::<f64>(elem, d00 - d10 * d_ecx);
                    wr::<f64>(elem + 0x08, d08 - d_ecx * d18);

                    let base3 = rd::<usize>(r9);
                    let p30 = base3.wrapping_add(rdx) + 0x30;
                    wr::<i32>(p30, rd::<i32>(p30).wrapping_sub(ecx));
                }
            }

            r9 = r9.wrapping_add(0x10);
        }

        wr::<u32>(param_2 + 0x08, 0);
        let mut r14 = groups_ptr;
        while num_groups != 0 {
            if ebp > 0 {
                let mut rdi_off: usize = 0;
                for _ in 0..(ebp as u32) {
                    let p1_0 = rd::<usize>(param_1);
                    let div20 = rd::<i32>(p1_0 + 0x20) as f64;
                    let g0 = rd::<usize>(r14).wrapping_add(rdi_off);
                    wr::<f64>(g0, rd::<f64>(g0) / div20);

                    let div24 = rd::<i32>(p1_0 + 0x24) as f64;
                    let g1 = rd::<usize>(r14).wrapping_add(rdi_off);
                    wr::<f64>(g1 + 0x08, rd::<f64>(g1 + 0x08) / div24);

                    let div20_b = rd::<i32>(p1_0 + 0x20) as f64;
                    let g2 = rd::<usize>(r14).wrapping_add(rdi_off);
                    wr::<f64>(g2 + 0x10, rd::<f64>(g2 + 0x10) / div20_b);

                    let div24_b = rd::<i32>(p1_0 + 0x24) as f64;
                    let g3 = rd::<usize>(r14).wrapping_add(rdi_off);
                    wr::<f64>(g3 + 0x18, rd::<f64>(g3 + 0x18) / div24_b);

                    let r15_buf = rd::<usize>(r14);
                    let len = rd::<u32>(param_2 + 0x08) as u64;
                    let cap = rd::<u32>(param_2 + 0x0c) as u64;
                    if (len << 6) == (cap << 6) {
                        let req = (((len << 6) as i64) >> 6).wrapping_add(1) as u64;
                        exe::call_va!(0x1_411a_2010, (param_2, req));
                    }
                    let cur_len = rd::<u32>(param_2 + 0x08) as usize;
                    let dst = rd::<usize>(param_2).wrapping_add(cur_len << 6);
                    let src = r15_buf.wrapping_add(rdi_off);
                    wr::<[u8; 64]>(dst, rd::<[u8; 64]>(src));
                    wr::<u32>(param_2 + 0x08, (cur_len as u32).wrapping_add(1));

                    rdi_off = rdi_off.wrapping_add(0x40);
                }
            }
            r14 = r14.wrapping_add(0x10);
            num_groups -= 1;
        }
    }

    if groups_ptr != 0 {
        if rd::<u32>(groups_ptr - 0x08) != 0 {
            let mut idx: u64 = 0;
            loop {
                let stride = rd::<u64>(groups_ptr - 0x10);
                let elem_ptr = groups_ptr.wrapping_add(idx.wrapping_mul(stride) as usize);
                exe::call_va!(0x1_401d_e830, (elem_ptr));
                idx = idx.wrapping_add(1);
                if idx >= rd::<u32>(groups_ptr - 0x08) as u64 {
                    break;
                }
            }
        }
        exe::call_va!(0x1_4244_2c80, (groups_ptr));
    }
}

/// `unknown_1422170a0`
/// Original: 0x1422170a0 (536 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1422170a0(param_1: usize, param_2: usize) -> u64 {
    let g_eb8_slot = exe::ptr::<usize>(0x1_4397_feb8) as usize;
    let mut ctx = [0u64; 24];
    let ctx_ptr = ctx.as_mut_ptr() as usize;
    wr::<usize>(ctx_ptr, rd::<usize>(g_eb8_slot));

    exe::call_va!(0x1_4221_5330, (ctx_ptr));
    let mut byte_len: u64 = 0;
    let res_ptr = exe::call_va!(
        0x1_4221_3f60,
        (ctx_ptr, (&mut byte_len as *mut u64) as usize) -> u64
    );
    wr::<u64>(param_1, res_ptr);

    let mut r9 = rd::<usize>(g_eb8_slot);
    let rbx = r9.wrapping_add(0x48);
    if byte_len / 0x28 != 0 {
        let mut ebp: i32 = 0;
        let mut rdi_off: usize = 0;
        loop {
            let rsi = rd::<usize>(param_1);
            let len = rd::<u32>(rbx + 0x08) as u64;
            let cap = rd::<u32>(rbx + 0x0c) as u64;
            if len.wrapping_mul(0x28) == cap.wrapping_mul(0x28) {
                exe::call_va!(0x1_411e_9730, (rbx, len.wrapping_add(1)));
            }
            let cur_len = rd::<u32>(rbx + 0x08) as usize;
            let dst = rd::<usize>(rbx).wrapping_add(cur_len * 0x28);
            let src = rsi.wrapping_add(rdi_off);
            wr::<[u8; 0x28]>(dst, rd::<[u8; 0x28]>(src));
            wr::<u32>(rbx + 0x08, (cur_len as u32).wrapping_add(1));

            ebp = ebp.wrapping_add(1);
            rdi_off = rdi_off.wrapping_add(0x28);
            if ((ebp as i64) as u64) >= byte_len / 0x28 {
                break;
            }
        }
        r9 = rd::<usize>(g_eb8_slot);
    }

    let prod = rd::<i32>(r9 + 0x04)
        .wrapping_mul(rd::<i32>(r9))
        .wrapping_add(0x1f4) as i64;
    wr::<i64>(param_2, prod);

    let count = byte_len / 0x28;
    let buf_28 = rd::<usize>(ctx_ptr + 0x08);
    if buf_28 != 0 {
        let r10 = buf_28.wrapping_sub(1) & !0x01ff_ffffusize;
        let is_local = exe::teb() == rd::<usize>(r10 + 0x100);
        let page_idx = buf_28.wrapping_sub(r10) >> 16;
        let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if is_local {
            if rd::<u8>(rdx + 0x0e) == 0 {
                wr::<u64>(buf_28, rd::<u64>(rdx + 0x20));
                wr::<usize>(rdx + 0x20, buf_28);
                let new_cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
                wr::<u32>(rdx + 0x18, new_cnt);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (rdx));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (r10, rdx, 1u8, buf_28));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (r10, rdx, 0u8, buf_28));
        }
    }
    count
}

/// `r_142252b40`
/// Original: 0x142252b40 (418 bytes, Throws::No)
pub unsafe extern "win64" fn r_142252b40(
    param_1: usize,
    param_2: usize,
    param_3: i32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: usize,
) -> usize {
    let count = rd::<i32>(param_1 + 0x60);
    let mut matched = false;
    if count > 0 {
        let arr = rd::<usize>(param_1 + 0x58);
        let x = rd::<f32>(param_7);
        let mut ebx: i32 = 0;
        while ebx < count {
            let r10 = rd::<usize>(arr.wrapping_add((ebx as usize) * 8));
            let min_x = rd::<f32>(r10);
            if !(x < min_x) {
                let y = rd::<f32>(param_7 + 0x04);
                if !(y < rd::<f32>(r10 + 0x04)) {
                    let z = rd::<f32>(param_7 + 0x08);
                    if !(z < rd::<f32>(r10 + 0x08))
                        && !(rd::<f32>(r10 + 0x0c) < x)
                        && !(rd::<f32>(r10 + 0x10) < y)
                        && !(rd::<f32>(r10 + 0x14) < z)
                    {
                        if ebx != -1 && r10 != 0 {
                            let sub_off = (param_3 as isize) * 0x30;
                            let ptr_28 = rd::<u64>((r10 as isize + sub_off + 0x28) as usize);
                            let len_30 =
                                rd::<u32>((r10 as isize + sub_off + 0x30) as usize) as u64;
                            if ptr_28 != ptr_28.wrapping_add(len_30.wrapping_mul(0x0c))
                                && !(x < min_x)
                                && !(y < rd::<f32>(r10 + 0x04))
                            {
                                let z2 = rd::<f32>(param_7 + 0x08);
                                if !(z2 < rd::<f32>(r10 + 0x08))
                                    && !(rd::<f32>(r10 + 0x0c) < x)
                                    && !(rd::<f32>(r10 + 0x10) < y)
                                    && !(rd::<f32>(r10 + 0x14) < z2)
                                {
                                    let mut local_28 = [0u32; 3];
                                    exe::call_va!(
                                        0x1_4239_ada0,
                                        (
                                            r10,
                                            local_28.as_mut_ptr() as usize,
                                            param_3,
                                            param_7
                                        )
                                    );
                                    if local_28[0] != 0 || local_28[1] != 0 || local_28[2] != 0 {
                                        wr::<u32>(param_2 + 0x04, local_28[0]);
                                        wr::<u32>(param_2 + 0x08, local_28[1]);
                                        wr::<u32>(param_2 + 0x0c, local_28[2]);
                                        wr::<u32>(param_2, (ebx + 1) as u32);
                                        matched = true;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
            ebx += 1;
        }
    }

    if !matched {
        wr::<u64>(param_2, 0);
        wr::<u64>(param_2 + 0x08, 0);
    }
    wr::<u32>(param_2 + 0x10, param_4);
    wr::<u32>(param_2 + 0x14, param_5);
    wr::<u32>(param_2 + 0x18, param_6);
    8
}

/// `unknown_1422df580`
/// Original: 0x1422df580 (722 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1422df580(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let src0 = rd::<usize>(param_2);
    wr::<[u8; 0x68]>(param_1, rd::<[u8; 0x68]>(src0));
    wr::<u64>(param_1 + 0x68, u64::MAX);
    wr::<u64>(param_1 + 0x70, u64::MAX);
    wr::<[u64; 12]>(param_1 + 0x78, [0; 12]);
    wr::<u32>(param_1 + 0xd8, 0);
    wr::<u128>(param_1 + 0xdc, 0);
    wr::<u64>(param_1 + 0xec, 0);
    wr::<u64>(param_1 + 0xf4, 0);
    wr::<u64>(param_1 + 0xfc, 0);

    let c_a38 = rd::<u64>(exe::ptr::<u64>(0x1_4315_9a38) as usize);
    let c_a40 = rd::<u32>(exe::ptr::<u32>(0x1_4315_9a40) as usize);
    let c_178 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7178) as usize);
    wr::<u64>(param_1 + 0x104, c_a38);
    wr::<u32>(param_1 + 0x10c, c_a40);
    wr::<u32>(param_1 + 0x110, c_178);
    wr::<u32>(param_1 + 0x114, c_178);
    wr::<u32>(param_1 + 0x118, c_178);

    wr::<u64>(param_1 + 0x11c, 0);
    wr::<u64>(param_1 + 0x124, 0);
    wr::<u64>(param_1 + 0x12c, 0);
    wr::<u64>(param_1 + 0x134, 0);
    wr::<u8>(param_1 + 0x13c, 0);
    wr::<u64>(param_1 + 0x140, 0);
    wr::<u64>(param_1 + 0x148, 0);
    let r14 = param_1 + 0x150;
    wr::<u64>(param_1 + 0x158, 0);
    wr::<u64>(r14, 0);
    wr::<u64>(r14 + 0x08, 0);

    wr::<u32>(param_1 + 0xa8, rd::<u32>(param_2 + 0x20));
    wr::<u32>(param_1 + 0xac, rd::<u32>(param_2 + 0x24));
    wr::<u32>(param_1 + 0xb0, rd::<u32>(param_2 + 0x28));
    wr::<u32>(param_1 + 0xb4, rd::<u32>(param_2 + 0x2c));
    wr::<u64>(param_1 + 0xb8, rd::<u64>(param_2 + 0x30));
    wr::<u64>(param_1 + 0xc0, rd::<u64>(param_2 + 0x38));
    wr::<u32>(param_1 + 0xc8, rd::<u32>(param_2 + 0x40));
    wr::<u32>(param_1 + 0x11c, rd::<u32>(param_2 + 0x44));
    wr::<u32>(param_1 + 0x120, rd::<u32>(param_2 + 0x48));
    wr::<u32>(param_1 + 0x124, rd::<u32>(param_2 + 0x4c));
    wr::<u32>(param_1 + 0xcc, rd::<u32>(param_2 + 0x70));
    wr::<u32>(param_1 + 0xd0, rd::<u32>(param_2 + 0x74));
    wr::<u32>(param_1 + 0xd4, rd::<u32>(param_2 + 0x78));

    let id = exe::call_va!(0x1_422d_f920, () -> u32);
    wr::<u32>(param_1 + 0xd8, id);

    wr::<u128>(param_1 + 0xdc, rd::<u128>(param_2 + 0x08));
    wr::<u64>(param_1 + 0xec, rd::<u64>(param_2 + 0x18));

    let p60 = rd::<usize>(param_2 + 0x60);
    wr::<u128>(param_1 + 0xf4, rd::<u128>(p60));

    let p68 = rd::<usize>(param_2 + 0x68);
    wr::<u128>(param_1 + 0x104, rd::<u128>(p68));
    wr::<u64>(param_1 + 0x114, rd::<u64>(p68 + 0x10));

    wr::<u8>(param_1 + 0x13c, rd::<u8>(param_2 + 0x7c));

    let p80 = rd::<usize>(param_2 + 0x80);
    wr::<u128>(param_1 + 0x140, rd::<u128>(p80));

    wr::<u32>(param_1 + 0x12c, rd::<u32>(param_2 + 0x50));
    wr::<u32>(param_1 + 0x130, rd::<u32>(param_2 + 0x54));
    wr::<u32>(param_1 + 0x134, rd::<u32>(param_2 + 0x58));
    wr::<u32>(param_1 + 0x138, rd::<u32>(param_2 + 0x5c));

    let count_88 = rd::<u32>(param_2 + 0x88);
    exe::call_va!(0x1_4130_b240, (r14, count_88 as u64, param_3, r14));

    if rd::<u32>(param_2 + 0x88) != 0 {
        let mut idx: u32 = 0;
        loop {
            let src = rd::<usize>(param_2 + 0x90).wrapping_add((idx as usize) * 0x28);
            let dst = rd::<usize>(r14).wrapping_add((idx as usize) * 0x30);
            wr::<[u8; 0x28]>(dst, rd::<[u8; 0x28]>(src));
            idx = idx.wrapping_add(1);
            if idx >= rd::<u32>(param_2 + 0x88) {
                break;
            }
        }
    }

    param_1
}

/// `rend_14230c9e0`
/// Original: 0x14230c9e0 (653 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_14230c9e0(param_1: usize, param_2: usize) {
    let p1_0 = rd::<usize>(param_1);
    let p30 = rd::<usize>(p1_0 + 0x30);
    let rdi = rd::<usize>(p30 + 0x18);
    let rbp = if rdi == 0 {
        0
    } else {
        let want_id = rd::<u32>(exe::ptr::<u32>(0x1_4388_1bf8) as usize);
        let vtbl = rd::<usize>(rdi);
        let got_id = exe::call_va!(rd::<usize>(vtbl + 0x98), (rdi) -> u32);
        if got_id == want_id {
            rdi
        } else {
            0
        }
    };

    let mut local_res8: usize = 0;
    let mut arg_30: [u64; 2] = [rd::<u32>(rbp + 0x1fc) as u64, rd::<u64>(param_2 + 0x20)];
    let out_ptr = exe::call_va!(
        0x1_4232_87d0,
        (
            (&mut local_res8 as *mut usize) as usize,
            arg_30.as_mut_ptr() as usize
        ) -> usize
    );

    let obj = rd::<usize>(param_1);
    let new_b8 = rd::<usize>(out_ptr);
    wr::<usize>(out_ptr, 0);
    let old_b8 = rd::<usize>(obj + 0xb8);
    wr::<usize>(obj + 0xb8, new_b8);

    if old_b8 != 0 {
        let p28 = rd::<usize>(old_b8 + 0x28);
        if p28 != 0 {
            exe::call_va!(0x1_4244_2de0, (p28));
        }
        let p20 = rd::<usize>(old_b8 + 0x20);
        if p20 != 0 {
            exe::call_va!(0x1_4244_2de0, (p20));
        }
        exe::call_va!(0x1_4244_2de0, (old_b8));
    }

    if local_res8 != 0 {
        let p28 = rd::<usize>(local_res8 + 0x28);
        if p28 != 0 {
            exe::call_va!(0x1_4244_2de0, (p28));
        }
        let p20 = rd::<usize>(local_res8 + 0x20);
        if p20 != 0 {
            exe::call_va!(0x1_4244_2de0, (p20));
        }
        exe::call_va!(0x1_4244_2de0, (local_res8));
    }

    let obj2 = rd::<usize>(param_1);
    let mut val_208: u64 = rd::<u32>(rbp + 0x208) as u64;
    let new_c0 = exe::call_va!(
        0x1_4131_3370,
        (0usize, 0usize, (&mut val_208 as *mut u64) as usize) -> usize
    );
    let old_c0 = rd::<usize>(obj2 + 0xc0);
    wr::<usize>(obj2 + 0xc0, new_c0);
    if old_c0 != 0 {
        let g_f58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        exe::call_va!(0x1_4132_0320, (g_f58, old_c0));
    }

    let obj3 = rd::<usize>(param_1);
    let c0_ptr = rd::<usize>(obj3 + 0xc0);
    let v_p2_18 = rd::<u64>(param_2 + 0x18);
    let v_obj_08 = rd::<u64>(obj3 + 0x08);
    let sub_30 = rd::<usize>(obj3 + 0x30);
    let mut pair_28: [u64; 2] = [rd::<u64>(sub_30 + 0x28), rd::<u64>(sub_30 + 0x30)];
    exe::call_va!(
        0x1_4240_9b50,
        (
            c0_ptr + 0x80,
            pair_28.as_mut_ptr() as usize,
            0u32,
            pair_28[1]
        )
    );
    wr::<u64>(c0_ptr + 0x108, v_obj_08);
    wr::<u64>(c0_ptr + 0x110, v_p2_18);

    let c0_a = rd::<usize>(rd::<usize>(param_1) + 0xc0);
    wr::<u64>(c0_a + 0x228, rd::<u64>(rbp + 0x258));
    let c0_b = rd::<usize>(rd::<usize>(param_1) + 0xc0);
    wr::<u32>(c0_b + 0x240, rd::<u32>(rbp + 0x260));

    let c0_c = rd::<usize>(rd::<usize>(param_1) + 0xc0);
    let mut arg_c7a0: [u64; 2] = [
        rd::<u32>(rbp + 0x1f8) as u64,
        (rd::<u32>(rbp + 0x1fc) as u64).wrapping_add(rd::<u64>(param_2 + 0x20)),
    ];
    exe::call_va!(0x1_4232_c7a0, (c0_c, arg_c7a0.as_mut_ptr() as usize));

    let c0_d = rd::<usize>(rd::<usize>(param_1) + 0xc0);
    let sum_u32 = rd::<u32>(rbp + 0x238)
        .wrapping_add(rd::<u32>(rbp + 0x234))
        .wrapping_add(rd::<u32>(rbp + 0x230))
        .wrapping_add(rd::<u32>(rbp + 0x22c))
        .wrapping_add(rd::<u32>(rbp + 0x228))
        .wrapping_add(rd::<u32>(rbp + 0x224))
        .wrapping_add(rd::<u32>(rbp + 0x220))
        .wrapping_add(rd::<u32>(rbp + 0x21c))
        .wrapping_add(rd::<u32>(rbp + 0x218))
        .wrapping_add(rd::<u32>(rbp + 0x214));
    let mut arg_c480: [u64; 2] = [
        sum_u32 as u64,
        (rd::<u32>(rbp + 0x210) as u64).wrapping_add(rd::<u64>(param_2 + 0x20)),
    ];
    exe::call_va!(
        0x1_4232_c480,
        (c0_d, arg_c480.as_mut_ptr() as usize, rbp + 0x1f8)
    );

    let obj_end = rd::<usize>(param_1);
    let _ = crate::simd_ops::atomic_xchg_u32((obj_end + 0x80) as u64, 4);
    let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
    exe::call_va!(0x1_4242_0ed0, (g_2b8, rd::<usize>(param_1)));
}

/// `rend_142326630`
/// Original: 0x142326630 (409 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142326630(param_1: usize) {
    let sub_f8 = param_1 + 0xf8;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_f8) + 0x38), (sub_f8));
    exe::call_va!(rd::<usize>(rd::<usize>(sub_f8) + 0x08), (sub_f8));
    exe::call_va!(rd::<usize>(rd::<usize>(param_1) + 0x08), (param_1));

    let rbx = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
    exe::call_va!(rd::<usize>(rd::<usize>(rbx) + 0x08), (rbx));
    let sub_d0 = rbx + 0xd0;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_d0) + 0x08), (sub_d0));
    let sub_1a0 = rbx + 0x1a0;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_1a0) + 0x08), (sub_1a0));
    let sub_340 = rbx + 0x340;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_340) + 0x08), (sub_340));

    let g_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    if rd::<u8>(g_c58 + 0xaf) != 0 {
        let sub_270 = rbx + 0x270;
        exe::call_va!(rd::<usize>(rd::<usize>(sub_270) + 0x08), (sub_270));
    }

    let g_1e8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_01e8) as usize);
    exe::call_va!(0x1_4238_a420, (g_1e8));

    let lock_ptr = exe::ptr::<u8>(0x1_4386_c5a8) as usize;
    let owner_ptr = exe::ptr::<u32>(0x1_4386_c5b0) as usize;
    let depth_ptr = exe::ptr::<u32>(0x1_4386_c5b4) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_ptr) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_ptr));
        wr::<u32>(owner_ptr, tid);
    }
    wr::<u32>(depth_ptr, rd::<u32>(depth_ptr).wrapping_add(1));

    let mut cur = rd::<usize>(param_1 + 0xd0);
    let end = cur.wrapping_add((rd::<u32>(param_1 + 0xd8) as usize) * 4);
    let q_base = exe::ptr::<u8>(0x1_43bb_d838) as usize;
    while cur != end {
        let val = rd::<u32>(cur);
        let cnt = rd::<u32>(q_base + 0x10);
        let cap = rd::<u32>(q_base + 0x14);
        if cnt == cap {
            exe::call_va!(0x1_412e_bcc0, (q_base, cnt.wrapping_add(1)));
        }
        let wr_idx = rd::<u32>(q_base + 0x0c) as usize;
        let buf = rd::<usize>(q_base);
        wr::<u32>(buf.wrapping_add(wr_idx * 4), val);
        let next_wr = rd::<u32>(q_base + 0x0c).wrapping_add(1);
        wr::<u32>(q_base + 0x0c, next_wr);
        if next_wr == rd::<u32>(q_base + 0x14) {
            wr::<u32>(q_base + 0x0c, 0);
        }
        wr::<u32>(q_base + 0x10, rd::<u32>(q_base + 0x10).wrapping_add(1));
        cur = cur.wrapping_add(4);
    }

    wr::<u32>(param_1 + 0xd8, 0);
    let new_depth = rd::<u32>(depth_ptr).wrapping_sub(1);
    wr::<u32>(depth_ptr, new_depth);
    if new_depth == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32(owner_ptr as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_ptr));
    }
}

/// `unknown_14232a5c0`
/// Original: 0x14232a5c0 (1242 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14232a5c0(
    param_1: usize,
    param_2: usize,
    param_3: u32,
) -> usize {
    unsafe fn call_vfn(stream_slot: usize, voff: usize, out_ptr: usize) {
        let s = rd::<usize>(stream_slot);
        let vtbl = rd::<usize>(s);
        exe::call_va!(rd::<usize>(vtbl + voff), (s, out_ptr));
    }

    let mut v20: u32 = 0;
    let mut v24: u32 = 0;
    let mut v28: u32 = 0;
    let mut v30: i32 = 0;
    let mut v34: i32 = 0;
    let mut v2c: u32 = 0;
    let mut v38: u32 = 0;
    let mut res8: u32 = 0;
    let mut res10: usize = param_2;
    let mut res18: u32 = param_3;

    call_vfn(param_1, 0x48, (&mut v20 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v24 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v28 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v30 as *mut i32) as usize);
    call_vfn(param_1, 0x48, (&mut v34 as *mut i32) as usize);
    call_vfn(param_1, 0x48, (&mut v2c as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v38 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut res8 as *mut u32) as usize);

    let mut local_108 = [0u32; 4];
    let p_108 = local_108.as_mut_ptr() as usize;
    let s1 = rd::<usize>(param_1);
    for i in 0..4usize {
        let vtbl = rd::<usize>(s1);
        exe::call_va!(rd::<usize>(vtbl + 0x38), (s1, p_108 + i * 4));
    }

    let mut local_120 = [0u32; 6];
    let p_120 = local_120.as_mut_ptr() as usize;
    wr::<u64>(p_120, rd::<u64>(exe::ptr::<u64>(0x1_4315_9a38) as usize));
    wr::<u32>(
        p_120 + 0x08,
        rd::<u32>(exe::ptr::<u32>(0x1_4315_9a40) as usize),
    );
    let c_178 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7178) as usize);
    wr::<u32>(p_120 + 0x0c, c_178);
    wr::<u32>(p_120 + 0x10, c_178);
    wr::<u32>(p_120 + 0x14, c_178);
    let s2 = rd::<usize>(param_1);
    for i in 0..6usize {
        let vtbl = rd::<usize>(s2);
        exe::call_va!(rd::<usize>(vtbl + 0x38), (s2, p_120 + i * 4));
    }

    let mut local_198 = [0u64; 13];
    let p_198 = local_198.as_mut_ptr() as usize;
    exe::call_va!(0x1_4204_dcc0, (rd::<usize>(param_1), p_198));

    let mut v_m30: u32 = 0;
    let mut v_m2c: u32 = 0;
    call_vfn(param_1, 0x48, (&mut v_m30 as *mut u32) as usize);
    call_vfn(param_1, 0x38, (&mut v_m2c as *mut u32) as usize);
    call_vfn(param_1, 0x80, (&mut res18 as *mut u32) as usize);
    if (res18 as u8) != 0 {
        res8 = 1;
    }

    let mut v3c: u32 = 0;
    let mut v40: u32 = 0;
    let mut v44: u32 = 0;
    call_vfn(param_1, 0x48, (&mut v3c as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v40 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v44 as *mut u32) as usize);

    let mut v48: u32 = 0;
    let mut v4c: u32 = 0;
    let mut v50: u32 = 0;
    let mut v54: u32 = 0;
    call_vfn(param_1, 0x48, (&mut v48 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v4c as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v50 as *mut u32) as usize);
    call_vfn(param_1, 0x48, (&mut v54 as *mut u32) as usize);
    call_vfn(param_1, 0x80, (&mut res10 as *mut usize) as usize);

    let mut res20: u32 = 0;
    call_vfn(param_1, 0x40, (&mut res20 as *mut u32) as usize);

    let mut local_f0 = [0u64; 2];
    let p_f0 = local_f0.as_mut_ptr() as usize;
    exe::call_va!(0x1_4133_cd80, (p_f0, res20));

    let r12 = rd::<usize>(p_f0);
    let len_f0 = rd::<u32>(p_f0 + 0x08) as usize;
    let rsi = r12.wrapping_add(len_f0 * 0x28);
    let mut rbx = r12;
    while rbx != rsi {
        let s = rd::<usize>(param_1);
        for off in [0x04usize, 0x08, 0x00, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
            let vtbl = rd::<usize>(s);
            exe::call_va!(rd::<usize>(vtbl + 0x40), (s, rbx + off));
        }
        rbx = rbx.wrapping_add(0x28);
    }

    let p1_8 = rd::<usize>(param_1 + 0x08);
    let r8_idx = (v20 as i32) as i64 as u64;
    let lim_120 = rd::<u32>(p1_8 + 0x120) as u64;
    let entry_ptr = if r8_idx < lim_120 {
        rd::<usize>(p1_8 + 0x118).wrapping_add((r8_idx as usize).wrapping_mul(0x338))
    } else {
        rd::<usize>(p1_8 + 0x128)
    };

    let elem_count = ((rsi as isize - r12 as isize) / 0x28) as i64;
    let mut desc = [0u64; 19];
    let dp = desc.as_mut_ptr() as usize;
    wr::<usize>(dp, p_198);
    wr::<u128>(dp + 0x08, rd::<u128>(entry_ptr + 0x320));
    wr::<u64>(dp + 0x18, rd::<u64>(entry_ptr + 0x330));
    wr::<u32>(dp + 0x20, v20);
    wr::<u32>(dp + 0x24, v24);
    wr::<u32>(dp + 0x28, v28);
    wr::<u32>(dp + 0x2c, v2c);
    wr::<i64>(dp + 0x30, v30 as i64);
    wr::<i64>(dp + 0x38, v34 as i64);
    wr::<u32>(dp + 0x40, v38);
    wr::<u32>(dp + 0x44, v3c);
    wr::<u32>(dp + 0x48, v40);
    wr::<u32>(dp + 0x4c, v44);
    wr::<u32>(dp + 0x50, v48);
    wr::<u32>(dp + 0x54, v4c);
    wr::<u32>(dp + 0x58, v50);
    wr::<u32>(dp + 0x5c, v54);
    wr::<usize>(dp + 0x60, p_108);
    wr::<usize>(dp + 0x68, p_120);
    wr::<u32>(dp + 0x70, v_m30);
    wr::<u32>(dp + 0x74, res8);
    wr::<u32>(dp + 0x78, v_m2c);
    wr::<u8>(dp + 0x7c, res10 as u8);
    wr::<usize>(
        dp + 0x80,
        p1_8.wrapping_add((r8_idx.wrapping_add(0x30) << 4) as usize),
    );
    wr::<u32>(dp + 0x88, elem_count as u32);
    wr::<usize>(dp + 0x90, r12);

    exe::call_va!(0x1_4133_cee0, (param_2, elem_count as u64, dp));
    exe::call_va!(0x1_422d_e420, (rd::<usize>(param_2)));
    exe::call_va!(0x1_401e_59a0, (p_f0));
    param_2
}

/// `rend_14234b140` (`rend::IlluminationVolumeGIDataStreamer::vfunc_1`)
/// Original: 0x14234b140 (96 bytes, Throws::No)
pub unsafe extern "win64" fn rend_14234b140(param_1: usize) -> u8 {
    let p28 = rd::<u64>(param_1 + 0x28);
    let l30 = rd::<u32>(param_1 + 0x30) as u64;
    let p08 = rd::<u64>(param_1 + 0x08);
    let l10 = rd::<u32>(param_1 + 0x10) as u64;
    let p18 = rd::<u64>(param_1 + 0x18);
    let l20 = rd::<u32>(param_1 + 0x20) as u64;
    let has_items = p28 != p28.wrapping_add(l30 * 8)
        || p08 != p08.wrapping_add(l10 * 8)
        || p18 != p18.wrapping_add(l20 * 8);

    let g104 = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0104) as usize);
    let busy = if g104 > 0 {
        true
    } else {
        let g100 = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0100) as usize);
        g100 > 0
    };
    u8::from(has_items && !busy)
}

/// `unknown_14237cd80`
/// Original: 0x14237cd80 (748 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14237cd80(param_1: usize, param_2: u64) {
    let u10 = param_2 as u32;
    let r14 = rd::<usize>(param_1 + 0x578).wrapping_add((param_2 as usize) << 6);
    let rsi = rd::<usize>(param_1 + 0xb8).wrapping_add((param_2 as usize) << 5);
    let bm_c8 = rd::<usize>(param_1 + 0xc8).wrapping_add(((param_2 >> 6) as usize) * 8);
    wr::<u64>(bm_c8, rd::<u64>(bm_c8) | (1u64 << (u10 & 0x3f)));

    let r9d = rd::<u32>(param_1 + 0x5a0);
    wr::<u32>(param_1 + 0x5a0, r9d.wrapping_add(1));
    wr::<u32>(r14 + 0x2c, r9d);

    let bm_198 = rd::<usize>(param_1 + 0x198).wrapping_add(((r9d >> 6) as usize) * 8);
    wr::<u64>(bm_198, rd::<u64>(bm_198) | (1u64 << (r9d & 0x3f)));
    wr::<u32>(
        rd::<usize>(param_1 + 0x188).wrapping_add((r9d as usize) * 4),
        u10,
    );

    wr::<u32>(rsi + 0x14, rd::<u32>(r14 + 0x1c));
    let r14_0 = rd::<usize>(r14);
    let v10 = if rd::<u32>(r14_0 + 0x34) > 0 {
        u32::MAX
    } else {
        rd::<u32>(r14 + 0x20)
    };
    wr::<u32>(rsi + 0x10, v10);
    wr::<u32>(rsi + 0x1c, rd::<u32>(r14 + 0x24));
    wr::<u32>(rsi + 0x08, rd::<u32>(r14 + 0x08));

    let idx8 = rd::<u32>(r14 + 0x08) as usize;
    let arg_dx = rd::<u32>(r14 + 0x10);
    let g_ed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let arg_cx = rd::<usize>(g_ed0 + 0x120).wrapping_add(idx8 * 0xa0);
    let v0c = exe::call_va!(0x1_4222_f6d0, (arg_cx, arg_dx) -> u32);
    wr::<u32>(rsi + 0x0c, v0c);

    let rbx = param_1 + 0x250;
    let edi = rd::<u32>(r14 + 0x18);
    let mut r15d = exe::call_va!(0x1_4222_9a00, (rbx, edi) -> u32);
    if r15d == u32::MAX {
        r15d = rd::<u32>(rbx);
        let r9_new = r15d.wrapping_add(edi);
        let byte_size = (r9_new as u64).wrapping_mul(0x1c);
        wr::<u32>(rbx, r9_new);
        let mut r8_aligned = rd::<u64>(param_1 + 0x228);
        if byte_size >= r8_aligned {
            let sum = r8_aligned.wrapping_sub(1).wrapping_add(byte_size);
            r8_aligned = sum.wrapping_sub(sum % r8_aligned);
        }
        let step_220 = rd::<u64>(param_1 + 0x220);
        let new_limit = (r8_aligned / step_220) as u32;
        if r9_new < new_limit {
            let slot_ptr =
                exe::call_va!(0x1_412f_8d90, (param_1 + 0x258, r8_aligned % step_220) -> usize);
            let cur_rbx = rd::<u32>(rbx);
            wr::<u32>(slot_ptr, cur_rbx);
            wr::<u32>(slot_ptr + 0x04, new_limit.wrapping_sub(cur_rbx));
        }
        wr::<u32>(rbx, new_limit);
    }
    wr::<u32>(rsi, r15d);

    let r14_0_b = rd::<usize>(r14);
    let idx14 = rd::<u32>(r14 + 0x14) as usize;
    let idx48 = rd::<u32>(r14_0_b + 0x48) as usize;
    let tbl_base = rd::<usize>(rd::<usize>(param_1 + 0x538).wrapping_add(idx48 << 4));
    let r12 = tbl_base.wrapping_add(idx14 * 0x30);
    wr::<u32>(rsi + 0x04, rd::<u32>(r12));

    let cnt_c = rd::<u32>(r14 + 0x0c);
    if cnt_c != u32::MAX {
        wr::<u32>(rsi + 0x18, 0);
        if cnt_c > 0 {
            let mut acc: u32 = 0;
            for d in 0..cnt_c {
                let p670 = rd::<usize>(param_1 + 0x670);
                acc = acc.wrapping_add(rd::<u32>(p670.wrapping_add((d as usize) << 4) + 0x08));
                wr::<u32>(rsi + 0x18, acc);
            }
        }
    }

    let words = rd::<u64>(r12 + 0x28).wrapping_add(0x3f) >> 6;
    let bm_18 = rd::<usize>(r12 + 0x18);
    let mut pop_total: u64 = 0;
    for i in 0..words {
        let w = rd::<u64>(bm_18.wrapping_add((i as usize) * 8));
        if w != 0 {
            pop_total = pop_total.wrapping_add(w.count_ones() as u64);
        }
    }

    let sub_268 = param_1 + 0x268;
    let vtbl_268 = rd::<usize>(sub_268);
    let base_out =
        exe::call_va!(rd::<usize>(vtbl_268 + 0x40), (sub_268, pop_total as u32) -> u32);
    wr::<u32>(r14 + 0x28, base_out);

    let v320 = rd::<u64>(param_1 + 0x320);
    let mut span_20: [u64; 3] = [
        v320.wrapping_add(0x3f) >> 6,
        rd::<u64>(param_1 + 0x310),
        v320,
    ];
    exe::call_va!(
        0x1_40ff_f0a0,
        (
            span_20.as_mut_ptr() as usize,
            base_out,
            pop_total as u32,
            1u8
        )
    );

    let r11 = rd::<usize>(param_1 + 0x300).wrapping_add((base_out as usize) * 0x0c);
    let mut r10 = rd::<usize>(r12 + 0x18);
    let words2 = ((rd::<u64>(r12 + 0x28).wrapping_add(0x3f) >> 6) as u32) as usize;
    if words2 != 0 {
        let mut out_idx: u32 = 0;
        let mut bit_base: u32 = 0;
        for _ in 0..words2 {
            let mut w = rd::<u64>(r10);
            while w != 0 {
                let cur_out = out_idx as usize;
                out_idx = out_idx.wrapping_add(1);
                let bit = w.trailing_zeros();
                let dst = r11.wrapping_add(cur_out * 0x0c);
                wr::<u32>(dst + 0x08, u10);
                wr::<u32>(dst, bit_base.wrapping_add(bit));
                wr::<u32>(dst + 0x04, rd::<u32>(rsi + 0x04));
                w ^= 1u64 << bit;
            }
            bit_base = bit_base.wrapping_add(0x40);
            r10 = r10.wrapping_add(8);
        }
    }
}

/// `unknown_142387250`
/// Original: 0x142387250 (608 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142387250(param_1: usize) {
    let sub_20 = param_1 + 0x20;
    exe::call_va!(
        rd::<usize>(rd::<usize>(sub_20) + 0x28),
        (sub_20, rd::<u32>(param_1 + 0x580))
    );

    let sub_f0 = param_1 + 0xf0;
    let dat_1a8 = exe::ptr::<u8>(0x1_433f_11a8) as usize;
    exe::call_va!(
        rd::<usize>(rd::<usize>(sub_f0) + 0x20),
        (sub_f0, rd::<u32>(param_1 + 0x580), dat_1a8)
    );

    let g_f48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let g_ed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let words = (rd::<u32>(param_1 + 0x580) as u64).wrapping_add(0x3f) >> 6;
    let bm_548 = rd::<usize>(param_1 + 0x548);

    for word_idx in 0..words {
        let base_bit = word_idx * 0x40;
        let mut rdi = rd::<u64>(bm_548.wrapping_add((word_idx as usize) * 8));
        while rdi != 0 {
            let bit = rdi.trailing_zeros() as u64;
            let rbx = base_bit.wrapping_add(bit);
            let r13 = rd::<usize>(param_1 + 0x578).wrapping_add((rbx as usize) << 6);
            let idx8 = rd::<u32>(r13 + 0x08) as usize;
            let arg_dx = rd::<u32>(r13 + 0x10);
            let old_20 = rd::<u32>(r13 + 0x20);
            let arg_cx = rd::<usize>(g_ed0 + 0x120).wrapping_add(idx8 * 0xa0);
            let new_20 = exe::call_va!(0x1_4222_ed20, (arg_cx, arg_dx) -> u32);
            wr::<u32>(r13 + 0x20, new_20);

            let rbp = (rbx >> 6) as usize;
            let bit_in_word = (rbx as u32) & 0x3f;
            if new_20 == u32::MAX {
                exe::call_va!(0x1_4237_cb70, (param_1, rbx));
                let bm_560 = rd::<usize>(param_1 + 0x560).wrapping_add(rbp * 8);
                wr::<u64>(bm_560, rd::<u64>(bm_560) | (1u64 << bit_in_word));
            } else {
                if old_20 == u32::MAX {
                    exe::call_va!(0x1_4237_cd80, (param_1, rbx));
                    let bm_560 = rd::<usize>(param_1 + 0x560).wrapping_add(rbp * 8);
                    wr::<u64>(bm_560, rd::<u64>(bm_560) | (1u64 << bit_in_word));
                }
                let cur_20 = rd::<u32>(r13 + 0x20);
                let f48_bm = rd::<usize>(g_f48 + 0x180);
                let w_f48 = rd::<u64>(f48_bm.wrapping_add(((cur_20 >> 6) as usize) * 8));
                if ((w_f48 >> (cur_20 & 0x3f)) & 1) != 0 || old_20 != cur_20 {
                    let bm_c8 = rd::<usize>(param_1 + 0xc8).wrapping_add(rbp * 8);
                    wr::<u64>(bm_c8, rd::<u64>(bm_c8) | (1u64 << bit_in_word));
                    let elem_b8 = rd::<usize>(param_1 + 0xb8).wrapping_add((rbx as usize) << 5);
                    wr::<u32>(elem_b8 + 0x14, rd::<u32>(elem_b8 + 0x14) | 1);
                    wr::<u8>(param_1 + 0x6e6, 1);
                }
            }

            rdi ^= 1u64 << bit;
        }
    }
}

/// `unknown_1423d8710`
/// Original: 0x1423d8710 (8 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1423d8710(
    param_1: u64,
    param_2: u64,
    _param_3: u64,
    param_4: u64,
) -> u64 {
    exe::call_va!(0x1_423d_8720, (param_1, param_2, param_2, param_4) -> u64)
}

/// `unknown_1423d9450`
/// Original: 0x1423d9450 (408 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1423d9450(param_1: u64, param_2: usize, param_3: u64) {
    if param_3 != 0 {
        let tbl_048 = exe::ptr::<usize>(0x1_43bd_6048) as usize;
        let mut local_res8: u64 = param_1;
        for i in 0..param_3 {
            let slot_idx = rd::<u32>(param_2.wrapping_add((i as usize) * 4)) as usize;
            let entry = rd::<usize>(tbl_048.wrapping_add(slot_idx * 8));
            exe::call_va!(0x1_4207_a680, (rd::<usize>(entry + 0x48)));
            exe::call_va!(0x1_4206_e380, ());

            let rbx = rd::<usize>(entry + 0x48);
            let sc = rd::<usize>(rd::<usize>(rbx) + 0x08);
            let cur_buf_idx = exe::call_va!(rd::<usize>(rd::<usize>(sc) + 0x120), (sc) -> u32);
            wr::<u32>(rbx + 0x38, cur_buf_idx);

            let rbx_0 = rd::<usize>(rbx);
            let fence_val = rd::<u64>(rbx_0 + 0x28);
            let fence_obj = rd::<usize>(rbx_0 + 0x20);
            let g_c60 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
            let cmd_q = rd::<usize>(g_c60 + 0x10);
            let hr = exe::call_va!(
                rd::<usize>(rd::<usize>(cmd_q) + 0x70),
                (cmd_q, fence_obj, fence_val) -> u32
            );
            if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
                exe::call_va!(0x1_4205_d410, (0usize, hr));
            }

            if rd::<u8>(exe::ptr::<u8>(0x1_4385_7fd0) as usize) != 0 {
                // Frame-latency wait: spin (`_Thrd_yield`) until the GPU has completed the
                // previous frame's signal. Diagnostic span: jobprof `K_MARK`, id = the loop's
                // original address, `sys` = 0xfffd.
                let jp = crate::jobprof::begin();
                loop {
                    let fence = rd::<usize>(rd::<usize>(rbx) + 0x20);
                    let completed =
                        exe::call_va!(rd::<usize>(rd::<usize>(fence) + 0x40), (fence) -> u64);
                    if completed == u64::MAX {
                        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32));
                    }
                    if completed.wrapping_add(1) >= rd::<u64>(rd::<usize>(rbx) + 0x28) {
                        break;
                    }
                    exe::call_iat!(0x1_4264_5868, ());
                }
                crate::jobprof::end(jp, crate::jobprof::K_MARK, 0x1_423d_94f2, 0xfffd, 0);
            }

            let rbx_0_b = rd::<usize>(rbx);
            wr::<u64>(rbx_0_b + 0x28, rd::<u64>(rbx_0_b + 0x28).wrapping_add(1));

            let rbx_0_c = rd::<usize>(rbx);
            if rd::<u8>(rbx_0_c + 0x41) != 0 {
                let sc2 = rd::<usize>(rbx_0_c + 0x08);
                local_res8 &= 0xffff_ffff_0000_0000;
                let hr2 = exe::call_va!(
                    rd::<usize>(rd::<usize>(sc2) + 0x58),
                    (sc2, (&mut local_res8 as *mut u64) as usize, 0usize) -> u32
                );
                if hr2 == 0x8007_000e || hr2.wrapping_add(0x7785_fffb) <= 2 {
                    exe::call_va!(0x1_4205_d410, (0usize, hr2));
                }
                if (local_res8 as u32) != 1 {
                    let hwnd = rd::<usize>(rd::<usize>(rbx));
                    let iconic = exe::call_iat!(0x1_4264_61c0, (hwnd) -> i32);
                    if iconic == 0 {
                        let sc3 = rd::<usize>(rd::<usize>(rbx) + 0x08);
                        let b66 = rd::<u8>(rbx + 0x66);
                        let r8_ptr = rbx + 0x08;
                        let r9_40 = rd::<u32>(rbx + 0x40);
                        let arg5_44 = rd::<u32>(rbx + 0x44);
                        exe::call_va!(0x1_4207_be30, (sc3, b66, r8_ptr, r9_40, arg5_44, 1u8));
                        let sc4 = rd::<usize>(rd::<usize>(rbx) + 0x08);
                        let v42 = exe::call_va!(0x1_4207_bd60, (sc4) -> u8);
                        wr::<u8>(rd::<usize>(rbx) + 0x42, v42);
                    }
                }
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142213400(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GRP_ALLOC: [u64; 8] = [0; 8];
    static mut GRP_BUF0: [u8; 64 * 5] = [0; 64 * 5];
    static mut GRP_BUF1: [u8; 64 * 5] = [0; 64 * 5];
    static mut GROW_CALLS: u32 = 0;
    static mut SORT_SUM: u64 = 0;
    static mut DTOR_SUM: u64 = 0;
    static mut FREE_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_3360(_p1: usize, _rem: u64, _p3: u64, n_grps: u64) -> usize {
        let base = core::ptr::addr_of_mut!(GRP_ALLOC) as usize;
        wr::<u64>(base, 0x10);
        wr::<u32>(base + 0x08, n_grps as u32);
        wr::<u32>(base + 0x0c, 8);
        let arr = base + 0x10;
        let b0 = core::ptr::addr_of_mut!(GRP_BUF0) as usize;
        let b1 = core::ptr::addr_of_mut!(GRP_BUF1) as usize;
        wr::<usize>(arr, b0);
        wr::<u32>(arr + 0x08, 0);
        wr::<u32>(arr + 0x0c, 4); // triggers grow on 5th element
        wr::<usize>(arr + 0x10, b1);
        wr::<u32>(arr + 0x18, 0);
        wr::<u32>(arr + 0x1c, 5);
        arr
    }
    unsafe extern "win64" fn mock_2010(vec_ptr: usize, req: u64) {
        GROW_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, (req as u32) + 4);
    }
    unsafe extern "win64" fn mock_8930(beg_ptr: usize, end_ptr: usize, len: u64, cmp: usize) {
        let diff = rd::<u64>(end_ptr).wrapping_sub(rd::<u64>(beg_ptr));
        SORT_SUM = SORT_SUM
            .wrapping_add(diff)
            .wrapping_add(len)
            .wrapping_add(cmp as u64);
    }
    unsafe extern "win64" fn mock_e830(elem_ptr: usize) {
        let base = core::ptr::addr_of!(GRP_ALLOC) as usize + 0x10;
        DTOR_SUM = DTOR_SUM.wrapping_add(elem_ptr.wrapping_sub(base) as u64 + 1);
    }
    unsafe extern "win64" fn mock_2c80(_ptr: usize) {
        FREE_CALLS += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4221_3360, mock_3360 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411a_2010, mock_2010 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412e_8930, mock_8930 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4244_2c80, mock_2c80 as *const ())?;

    let run = |f: F| -> (Vec<u8>, u32, u32, u64, u64, u32) {
        GRP_ALLOC = [0; 8];
        GRP_BUF0 = [0; 64 * 5];
        GRP_BUF1 = [0; 64 * 5];
        GROW_CALLS = 0;
        SORT_SUM = 0;
        DTOR_SUM = 0;
        FREE_CALLS = 0;

        let mut p1_inner = [0u8; 0x30];
        let p1ip = p1_inner.as_mut_ptr() as usize;
        wr::<i32>(p1ip + 0x20, 2);
        wr::<i32>(p1ip + 0x24, 4);
        let p1 = [p1ip];

        let mut elems = vec![0u8; 64 * 10];
        let ep = elems.as_mut_ptr() as usize;
        for i in 0..10usize {
            let e = ep + i * 64;
            let grp_local = (i % 5) as i32;
            wr::<f64>(e, 100.0 + (i as f64) * 10.0);
            wr::<f64>(e + 0x08, 200.0 + (i as f64) * 20.0);
            wr::<f64>(e + 0x10, 2.0);
            wr::<f64>(e + 0x18, 4.0);
            wr::<i32>(e + 0x24, 10 + i as i32);
            wr::<i32>(e + 0x30, 50 + grp_local * 3);
            wr::<i32>(e + 0x34, 100 + grp_local);
        }

        let mut p2 = [0u64; 2];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, ep);
        wr::<u32>(p2p + 0x08, 10);
        wr::<u32>(p2p + 0x0c, 9);

        f(p1.as_ptr() as usize, p2p, 5);
        (
            elems,
            rd::<u32>(p2p + 0x08),
            GROW_CALLS,
            SORT_SUM,
            DTOR_SUM,
            FREE_CALLS,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142213400 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1422170a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SRC_ITEMS: [u64; 10] = [0; 10];
    static mut GROW_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_5330(ctx: usize) {
        wr::<u64>(ctx + 0x10, 0x1234);
    }
    unsafe extern "win64" fn mock_3f60(_ctx: usize, out_len: usize) -> u64 {
        wr::<u64>(out_len, 0x50); // 2 items of 0x28 bytes
        core::ptr::addr_of!(SRC_ITEMS) as u64
    }
    unsafe extern "win64" fn mock_9730(vec_ptr: usize, req: u64) {
        GROW_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, (req as u32) + 4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4221_5330, mock_5330 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4221_3f60, mock_3f60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_9730, mock_9730 as *const ())?;

    let g_eb8 = exe::ptr::<usize>(0x1_4397_feb8) as usize;
    let saved_eb8 = rd::<usize>(g_eb8);

    let run = |f: F| -> (u64, i64, [u64; 10], u32, u32) {
        for i in 0..10usize {
            SRC_ITEMS[i] = 0x1000 + i as u64;
        }
        GROW_CALLS = 0;

        let mut dst_buf = [0u64; 10];
        let mut mgr = [0u8; 0x60];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<i32>(mp, 6);
        wr::<i32>(mp + 0x04, 7);
        wr::<usize>(mp + 0x48, dst_buf.as_mut_ptr() as usize);
        wr::<u32>(mp + 0x50, 0);
        wr::<u32>(mp + 0x54, 1); // triggers grow on 2nd push
        wr::<usize>(g_eb8, mp);

        let mut p1: u64 = 0;
        let mut p2: i64 = 0;
        let ret = f(
            (&mut p1 as *mut u64) as usize,
            (&mut p2 as *mut i64) as usize,
        );
        (ret, p2, dst_buf, rd::<u32>(mp + 0x50), GROW_CALLS)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_eb8, saved_eb8);
    crate::diff_eq!(o, n, "1422170a0 out");
    Ok(1)
}

pub unsafe fn diff_r_142252b40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, u32, u32, u32, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_ada0(vol: usize, out: usize, idx: i32, pt: usize) {
        let tag = rd::<u32>(vol + 0x18);
        let px = rd::<f32>(pt) as u32;
        wr::<u32>(out, tag.wrapping_add(idx as u32));
        wr::<u32>(out + 0x04, px);
        wr::<u32>(out + 0x08, 0x77);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4239_ada0, mock_ada0 as *const ())?;

    let run = |f: F, pt: [f32; 3]| -> (usize, [u32; 7]) {
        let mut vol0 = [0u8; 0x90];
        let v0 = vol0.as_mut_ptr() as usize;
        wr::<[f32; 6]>(v0, [0.0, 0.0, 0.0, 10.0, 10.0, 10.0]);
        wr::<u32>(v0 + 0x18, 0x100);
        wr::<u64>(v0 + 0x30 + 0x28, 0x5000);
        wr::<u32>(v0 + 0x30 + 0x30, 2);

        let mut vol1 = [0u8; 0x90];
        let v1 = vol1.as_mut_ptr() as usize;
        wr::<[f32; 6]>(v1, [20.0, 20.0, 20.0, 30.0, 30.0, 30.0]);
        wr::<u32>(v1 + 0x18, 0x200);
        wr::<u64>(v1 + 0x30 + 0x28, 0x6000);
        wr::<u32>(v1 + 0x30 + 0x30, 3);

        let vols = [v0, v1];
        let mut p1 = [0u8; 0x68];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x58, vols.as_ptr() as usize);
        wr::<i32>(p1p + 0x60, 2);

        let mut out = [0xdead_beefu32; 7];
        let ret = f(
            p1p,
            out.as_mut_ptr() as usize,
            1,
            0xaa,
            0xbb,
            0xcc,
            pt.as_ptr() as usize,
        );
        (ret, out)
    };

    for pt in [[25.0f32, 25.0, 25.0], [50.0, 50.0, 50.0]] {
        let o = run(f_o, pt);
        let n = run(f_n, pt);
        crate::diff_eq!(o, n, "142252b40 pt={:?}", pt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1422df580(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_ELEMS: [u8; 0x30 * 2] = [0; 0x30 * 2];
    static mut GROW_ARG: u64 = 0;

    unsafe extern "win64" fn mock_f920() -> u32 {
        0x4242_1337
    }
    unsafe extern "win64" fn mock_b240(vec_ptr: usize, cnt: u64, arg3: u64, vec_ptr2: usize) {
        assert_eq!(vec_ptr, vec_ptr2);
        GROW_ARG = cnt.wrapping_add(arg3);
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(DST_ELEMS) as usize);
        wr::<u32>(vec_ptr + 0x08, cnt as u32);
        wr::<u32>(vec_ptr + 0x0c, cnt as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_422d_f920, mock_f920 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4130_b240, mock_b240 as *const ())?;

    let run = |f: F| -> (Vec<u8>, [u8; 0x30 * 2], u64) {
        DST_ELEMS = [0; 0x30 * 2];
        GROW_ARG = 0;

        let mut src0 = [0u8; 0x68];
        for (i, b) in src0.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        let b60: [u64; 2] = [0x1111_2222_3333_4444, 0x5555_6666_7777_8888];
        let b68: [u64; 3] = [0xaaaa_0001, 0xbbbb_0002, 0xcccc_0003];
        let b80: [u64; 2] = [0xdddd_0004, 0xeeee_0005];
        let mut src_elems = [0u8; 0x28 * 2];
        for (i, b) in src_elems.iter_mut().enumerate() {
            *b = (i as u8) ^ 0x5a;
        }

        let mut p2 = [0u8; 0x98];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, src0.as_ptr() as usize);
        wr::<u64>(p2p + 0x08, 0x101);
        wr::<u64>(p2p + 0x10, 0x102);
        wr::<u64>(p2p + 0x18, 0x103);
        for off in (0x20..0x60).step_by(4) {
            wr::<u32>(p2p + off, 0x2000 + off as u32);
        }
        wr::<usize>(p2p + 0x60, b60.as_ptr() as usize);
        wr::<usize>(p2p + 0x68, b68.as_ptr() as usize);
        wr::<u32>(p2p + 0x70, 0x701);
        wr::<u32>(p2p + 0x74, 0x741);
        wr::<u32>(p2p + 0x78, 0x781);
        wr::<u8>(p2p + 0x7c, 1);
        wr::<usize>(p2p + 0x80, b80.as_ptr() as usize);
        wr::<u32>(p2p + 0x88, 2);
        wr::<usize>(p2p + 0x90, src_elems.as_ptr() as usize);

        let mut p1 = vec![0xffu8; 0x160];
        let p1p = p1.as_mut_ptr() as usize;
        let ret = f(p1p, p2p, 99);
        assert_eq!(ret, p1p);
        wr::<usize>(p1p + 0x150, 0); // clear pointer for comparison
        (p1, DST_ELEMS, GROW_ARG)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1422df580 out");
    Ok(1)
}

pub unsafe fn diff_rend_14230c9e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_C0_BUF: [u8; 0x250] = [0; 0x250];
    static mut LOG_ACC: u64 = 0;

    unsafe extern "win64" fn mock_v98(_this: usize) -> u32 {
        0x777
    }
    unsafe extern "win64" fn mock_87d0(out_res8: usize, pair_ptr: usize) -> usize {
        let p = rd::<[u64; 2]>(pair_ptr);
        LOG_ACC = LOG_ACC.wrapping_add(p[0]).wrapping_add(p[1]);
        wr::<usize>(out_res8, p[0] as usize);
        out_res8
    }
    unsafe extern "win64" fn mock_2de0(ptr: usize) {
        LOG_ACC = LOG_ACC.wrapping_add(ptr as u64);
    }
    unsafe extern "win64" fn mock_3370(_rcx: usize, _rdx: usize, r8_ptr: usize) -> usize {
        LOG_ACC = LOG_ACC.wrapping_add(rd::<u64>(r8_ptr));
        core::ptr::addr_of_mut!(NEW_C0_BUF) as usize
    }
    unsafe extern "win64" fn mock_0320(rcx: usize, rdx: usize) {
        LOG_ACC = LOG_ACC.wrapping_add(rcx as u64).wrapping_add(rdx as u64);
    }
    unsafe extern "win64" fn mock_9b50(_rcx: usize, rdx: usize, r8: u32, r9: u64) {
        let p = rd::<[u64; 2]>(rdx);
        LOG_ACC = LOG_ACC
            .wrapping_add(p[0])
            .wrapping_add(p[1])
            .wrapping_add(r8 as u64)
            .wrapping_add(r9);
    }
    unsafe extern "win64" fn mock_c7a0(_rcx: usize, rdx: usize) {
        let p = rd::<[u64; 2]>(rdx);
        LOG_ACC = LOG_ACC.wrapping_add(p[0]).wrapping_add(p[1]);
    }
    unsafe extern "win64" fn mock_c480(_rcx: usize, rdx: usize, r8: usize) {
        let p = rd::<[u64; 2]>(rdx);
        LOG_ACC = LOG_ACC
            .wrapping_add(p[0])
            .wrapping_add(p[1])
            .wrapping_add(rd::<u32>(r8) as u64);
    }
    unsafe extern "win64" fn mock_0ed0(rcx: usize, _rdx: usize) {
        LOG_ACC = LOG_ACC.wrapping_add(rcx as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4232_87d0, mock_87d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4131_3370, mock_3370 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4132_0320, mock_0320 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4232_c7a0, mock_c7a0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4232_c480, mock_c480 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4242_0ed0, mock_0ed0 as *const ())?;

    let g_bf8 = exe::ptr::<u32>(0x1_4388_1bf8) as usize;
    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let g_2b8 = exe::ptr::<usize>(0x1_43a0_82b8) as usize;
    let saved = (rd::<u32>(g_bf8), rd::<usize>(g_f58), rd::<usize>(g_2b8));
    wr::<u32>(g_bf8, 0x777);
    wr::<usize>(g_f58, 0x5500);
    wr::<usize>(g_2b8, 0x6600);

    let run = |f: F| -> ([u8; 0x250], u32, usize, u64) {
        NEW_C0_BUF = [0; 0x250];
        LOG_ACC = 0;

        let mut vtbl = [0usize; 20];
        vtbl[0x98 / 8] = mock_v98 as *const () as usize;
        let mut rdi_obj = [0u8; 0x270];
        let rp = rdi_obj.as_mut_ptr() as usize;
        wr::<usize>(rp, vtbl.as_ptr() as usize);
        wr::<u32>(rp + 0x1f8, 10);
        wr::<u32>(rp + 0x1fc, 20);
        wr::<u32>(rp + 0x208, 30);
        wr::<u32>(rp + 0x210, 40);
        for i in 0..10usize {
            wr::<u32>(rp + 0x214 + i * 4, (i as u32) + 1);
        }
        wr::<u64>(rp + 0x258, 0x9999);
        wr::<u32>(rp + 0x260, 0x8888);

        let mut sub30 = [0u8; 0x40];
        let s30p = sub30.as_mut_ptr() as usize;
        wr::<usize>(s30p + 0x18, rp);
        wr::<u64>(s30p + 0x28, 0x111);
        wr::<u64>(s30p + 0x30, 0x222);

        let mut old_b8 = [0u64; 6];
        old_b8[4] = 0x300;
        old_b8[5] = 0x400;

        let mut obj = [0u8; 0xd0];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x08, 0xaaaa);
        wr::<usize>(op + 0x30, s30p);
        wr::<usize>(op + 0xb8, old_b8.as_mut_ptr() as usize);
        wr::<usize>(op + 0xc0, 0x1234);

        let mut p2 = [0u8; 0x30];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x18, 0xbbbb);
        wr::<u64>(p2p + 0x20, 0x1000);

        let p1 = [op];
        f(p1.as_ptr() as usize, p2p);
        let log_adjusted = LOG_ACC.wrapping_sub(old_b8.as_ptr() as u64);
        (
            NEW_C0_BUF,
            rd::<u32>(op + 0x80),
            rd::<usize>(op + 0xb8),
            log_adjusted,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u32>(g_bf8, saved.0);
    wr::<usize>(g_f58, saved.1);
    wr::<usize>(g_2b8, saved.2);
    crate::diff_eq!(o, n, "14230c9e0 out");
    Ok(1)
}

pub unsafe fn diff_rend_142326630(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VCALL_COUNT: u32 = 0;
    static mut GROW_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_vfn(_this: usize) {
        VCALL_COUNT += 1;
    }
    unsafe extern "win64" fn mock_a420(_rcx: usize) {
        VCALL_COUNT += 10;
    }
    unsafe extern "win64" fn mock_bcc0(q_base: usize, req: u32) {
        GROW_COUNT += 1;
        wr::<u32>(q_base + 0x14, req + 2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4238_a420, mock_a420 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_bcc0, mock_bcc0 as *const ())?;

    let g_1e8 = exe::ptr::<usize>(0x1_43a0_01e8) as usize;
    let g_c58 = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let q_base = exe::ptr::<u8>(0x1_43bb_d838) as usize;
    let saved_1e8 = rd::<usize>(g_1e8);
    let saved_c58 = rd::<usize>(g_c58);
    let saved_q = rd::<[u64; 3]>(q_base);

    let run = |f: F| -> ([u32; 4], u32, u32, u32, u32, u32) {
        VCALL_COUNT = 0;
        GROW_COUNT = 0;

        let mut vtbl = [0usize; 8];
        vtbl[0x08 / 8] = mock_vfn as *const () as usize;
        vtbl[0x38 / 8] = mock_vfn as *const () as usize;
        let vp = vtbl.as_ptr() as usize;

        let mut rbx_obj = [0u8; 0x350];
        let rbxp = rbx_obj.as_mut_ptr() as usize;
        for off in [0usize, 0xd0, 0x1a0, 0x270, 0x340] {
            wr::<usize>(rbxp + off, vp);
        }
        wr::<usize>(g_1e8, rbxp);

        let mut c58_obj = [0u8; 0xb8];
        wr::<u8>(c58_obj.as_mut_ptr() as usize + 0xaf, 1);
        wr::<usize>(g_c58, c58_obj.as_mut_ptr() as usize);

        let mut q_buf = [0u32; 4];
        wr::<usize>(q_base, q_buf.as_mut_ptr() as usize);
        wr::<u32>(q_base + 0x0c, 1); // wr_idx = 1
        wr::<u32>(q_base + 0x10, 2); // cnt = 2
        wr::<u32>(q_base + 0x14, 2); // cap = 2 (triggers grow on first item)

        let items: [u32; 2] = [0x1111, 0x2222];
        let mut p1 = [0u8; 0x108];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, vp);
        wr::<usize>(p1p + 0xd0, items.as_ptr() as usize);
        wr::<u32>(p1p + 0xd8, 2);
        wr::<usize>(p1p + 0xf8, vp);

        f(p1p);
        (
            q_buf,
            rd::<u32>(q_base + 0x0c),
            rd::<u32>(q_base + 0x10),
            rd::<u32>(p1p + 0xd8),
            VCALL_COUNT,
            GROW_COUNT,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_1e8, saved_1e8);
    wr::<usize>(g_c58, saved_c58);
    wr::<[u64; 3]>(q_base, saved_q);
    crate::diff_eq!(o, n, "142326630 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14232a5c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut STREAM_SEQ: u32 = 0;
    static mut ELEM_BUF: [u32; 10] = [0; 10];
    static mut CAPTURED_DESC: [u64; 19] = [0; 19];
    static mut E420_ARG: usize = 0;
    static mut FREE_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_v38(_s: usize, out: usize) {
        STREAM_SEQ += 1;
        wr::<u32>(out, 0x1000 + STREAM_SEQ);
    }
    unsafe extern "win64" fn mock_v40(_s: usize, out: usize) {
        STREAM_SEQ += 1;
        wr::<u32>(out, STREAM_SEQ);
    }
    unsafe extern "win64" fn mock_v48(_s: usize, out: usize) {
        STREAM_SEQ += 1;
        wr::<u32>(out, 1);
    }
    unsafe extern "win64" fn mock_v80(_s: usize, out: usize) {
        STREAM_SEQ += 1;
        wr::<u8>(out, 1);
    }
    unsafe extern "win64" fn mock_dcc0(_s: usize, out: usize) {
        wr::<u64>(out, 0x9999_8888);
    }
    unsafe extern "win64" fn mock_cd80(vec_ptr: usize, _cnt: u32) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(ELEM_BUF) as usize);
        wr::<u32>(vec_ptr + 0x08, 1);
        wr::<u32>(vec_ptr + 0x0c, 1);
    }
    unsafe extern "win64" fn mock_cee0(p2: usize, _cnt: u64, desc_ptr: usize) {
        let mut d = rd::<[u64; 19]>(desc_ptr);
        d[0] = rd::<u64>(d[0] as usize); // deref p_198
        d[12] = rd::<u64>(d[12] as usize); // deref p_108
        d[13] = rd::<u64>(d[13] as usize); // deref p_120
        d[15] &= 0x0000_00ff_ffff_ffff; // +0x7d..0x7f is 3-byte uninitialized stack padding
        d[17] &= 0x0000_0000_ffff_ffff; // +0x8c..0x8f is 4-byte uninitialized stack padding
        d[18] = 0; // clear pointer r12
        CAPTURED_DESC = d;
        wr::<usize>(p2, 0x7777_0000);
    }
    unsafe extern "win64" fn mock_e420(arg: usize) {
        E420_ARG = arg;
    }
    unsafe extern "win64" fn mock_59a0(_vec_ptr: usize) {
        FREE_CALLED += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_dcc0, mock_dcc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4133_cd80, mock_cd80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4133_cee0, mock_cee0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_422d_e420, mock_e420 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_59a0, mock_59a0 as *const ())?;

    let run = |f: F| -> ([u64; 19], [u32; 10], usize, u32) {
        STREAM_SEQ = 0;
        ELEM_BUF = [0; 10];
        CAPTURED_DESC = [0; 19];
        E420_ARG = 0;
        FREE_CALLED = 0;

        let mut vtbl = [0usize; 18];
        vtbl[0x38 / 8] = mock_v38 as *const () as usize;
        vtbl[0x40 / 8] = mock_v40 as *const () as usize;
        vtbl[0x48 / 8] = mock_v48 as *const () as usize;
        vtbl[0x80 / 8] = mock_v80 as *const () as usize;
        let stream_obj = [vtbl.as_ptr() as usize];

        let mut entries = vec![0u8; 0x338 * 2];
        let ep = entries.as_mut_ptr() as usize;
        wr::<u64>(ep + 0x338 + 0x320, 0xaaaa_1111);
        wr::<u64>(ep + 0x338 + 0x328, 0xbbbb_2222);
        wr::<u64>(ep + 0x338 + 0x330, 0xcccc_3333);

        let mut p1_8 = [0u8; 0x130];
        let p18p = p1_8.as_mut_ptr() as usize;
        wr::<usize>(p18p + 0x118, ep);
        wr::<u32>(p18p + 0x120, 2);

        let p1 = [stream_obj.as_ptr() as usize, p18p];
        let mut p2 = [0usize; 1];
        let ret = f(p1.as_ptr() as usize, p2.as_mut_ptr() as usize, 0);
        assert_eq!(ret, p2.as_mut_ptr() as usize);
        let mut desc = CAPTURED_DESC;
        desc[16] = (desc[16] as usize).wrapping_sub(p18p) as u64;
        (desc, ELEM_BUF, E420_ARG, FREE_CALLED)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14232a5c0 out");
    Ok(1)
}

pub unsafe fn diff_rend_14234b140(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g104 = exe::ptr::<i32>(0x1_43a0_0104) as usize;
    let g100 = exe::ptr::<i32>(0x1_43a0_0100) as usize;
    let s104 = rd::<i32>(g104);
    let s100 = rd::<i32>(g100);

    for (len30, len10, len20, v104, v100) in [
        (0u32, 0u32, 0u32, 0i32, 0i32),
        (1, 0, 0, 0, 0),
        (0, 2, 0, 1, 0),
        (0, 0, 3, 0, 2),
        (1, 0, 0, -1, -256),
    ] {
        wr::<i32>(g104, v104);
        wr::<i32>(g100, v100);
        let mut obj = [0u8; 0x38];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x08, 0x1000);
        wr::<u32>(op + 0x10, len10);
        wr::<u64>(op + 0x18, 0x2000);
        wr::<u32>(op + 0x20, len20);
        wr::<u64>(op + 0x28, 0x3000);
        wr::<u32>(op + 0x30, len30);

        let o = f_o(op);
        let n = f_n(op);
        wr::<i32>(g104, s104);
        wr::<i32>(g100, s100);
        crate::diff_eq!(o, n, "14234b140");
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14237cd80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT_OUT: [u32; 2] = [0; 2];
    static mut F0A0_SUM: u64 = 0;

    unsafe extern "win64" fn mock_f6d0(_rcx: usize, rdx: u32) -> u32 {
        rdx + 0x50
    }
    unsafe extern "win64" fn mock_9a00(_rcx: usize, _rdx: u32) -> u32 {
        u32::MAX
    }
    unsafe extern "win64" fn mock_8d90(_rcx: usize, _rdx: u64) -> usize {
        core::ptr::addr_of_mut!(SLOT_OUT) as usize
    }
    unsafe extern "win64" fn mock_v40(_this: usize, pop: u32) -> u32 {
        pop
    }
    unsafe extern "win64" fn mock_f0a0(span: usize, b: u32, pop: u32, flag: u8) {
        let s = rd::<[u64; 3]>(span);
        F0A0_SUM = s[0]
            .wrapping_add(s[1])
            .wrapping_add(s[2])
            .wrapping_add(b as u64)
            .wrapping_add(pop as u64)
            .wrapping_add(flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_f6d0, mock_f6d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4222_9a00, mock_9a00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_8d90, mock_8d90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40ff_f0a0, mock_f0a0 as *const ())?;

    let g_ed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let saved_ed0 = rd::<usize>(g_ed0);

    let run = |f: F| -> ([u32; 16], [u32; 24], [u64; 2], [u64; 2], [u32; 2], u64) {
        SLOT_OUT = [0; 2];
        F0A0_SUM = 0;

        let mut ed0_obj = [0u8; 0x130];
        wr::<usize>(ed0_obj.as_mut_ptr() as usize + 0x120, 0x10_0000);
        wr::<usize>(g_ed0, ed0_obj.as_mut_ptr() as usize);

        let mut r14_0 = [0u8; 0x50];
        wr::<u32>(r14_0.as_mut_ptr() as usize + 0x34, 0);
        wr::<u32>(r14_0.as_mut_ptr() as usize + 0x48, 0);

        let mut r14_arr = [0u8; 0x80];
        let r14p = r14_arr.as_mut_ptr() as usize + 0x40; // index 1
        wr::<usize>(r14p, r14_0.as_ptr() as usize);
        wr::<u32>(r14p + 0x08, 2);
        wr::<u32>(r14p + 0x0c, 2);
        wr::<u32>(r14p + 0x10, 7);
        wr::<u32>(r14p + 0x14, 0);
        wr::<u32>(r14p + 0x18, 3);
        wr::<u32>(r14p + 0x1c, 0xaa);
        wr::<u32>(r14p + 0x20, 0xbb);
        wr::<u32>(r14p + 0x24, 0xcc);

        let bm_18_words: [u64; 1] = [0b1011]; // 3 bits set (0, 1, 3)
        let mut r12_entry = [0u8; 0x30];
        let r12p = r12_entry.as_mut_ptr() as usize;
        wr::<u32>(r12p, 0x777);
        wr::<usize>(r12p + 0x18, bm_18_words.as_ptr() as usize);
        wr::<u64>(r12p + 0x28, 16);
        let tbl_538 = [r12p, 0usize];

        let mut arr_670 = [0u8; 0x20];
        wr::<u32>(arr_670.as_mut_ptr() as usize + 0x08, 10);
        wr::<u32>(arr_670.as_mut_ptr() as usize + 0x18, 25);

        let mut vtbl_268 = [0usize; 9];
        vtbl_268[0x40 / 8] = mock_v40 as *const () as usize;

        let mut bm_c8 = [0u64; 2];
        let mut bm_198 = [0u64; 2];
        let mut arr_188 = [0u32; 4];
        let mut rsi_arr = [0u32; 16];
        let mut out_300 = [0u32; 24];

        let mut p1 = vec![0u8; 0x680];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xb8, rsi_arr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xc8, bm_c8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x188, arr_188.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x198, bm_198.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x220, 0x1c);
        wr::<u64>(p1p + 0x228, 0x1c * 8);
        wr::<u32>(p1p + 0x250, 1);
        wr::<usize>(p1p + 0x268, vtbl_268.as_ptr() as usize);
        wr::<usize>(p1p + 0x300, out_300.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x310, 0x4000);
        wr::<u64>(p1p + 0x320, 128);
        wr::<usize>(p1p + 0x538, tbl_538.as_ptr() as usize);
        wr::<usize>(p1p + 0x578, r14_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x5a0, 2);
        wr::<usize>(p1p + 0x670, arr_670.as_ptr() as usize);

        f(p1p, 1);
        (rsi_arr, out_300, bm_c8, bm_198, SLOT_OUT, F0A0_SUM)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_ed0, saved_ed0);
    crate::diff_eq!(o, n, "14237cd80 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142387250(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_v28(_this: usize, cnt: u32) {
        CALL_LOG = CALL_LOG.wrapping_add(cnt as u64);
    }
    unsafe extern "win64" fn mock_v20(_this: usize, cnt: u32, _dat: usize) {
        CALL_LOG = CALL_LOG.wrapping_add((cnt as u64) * 10);
    }
    unsafe extern "win64" fn mock_ed20(_rcx: usize, rdx: u32) -> u32 {
        if rdx == 0 {
            u32::MAX
        } else {
            rdx
        }
    }
    unsafe extern "win64" fn mock_cb70(_p1: usize, idx: u64) {
        CALL_LOG = CALL_LOG.wrapping_add(0x100 + idx);
    }
    unsafe extern "win64" fn mock_cd80(_p1: usize, idx: u64) {
        CALL_LOG = CALL_LOG.wrapping_add(0x200 + idx);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_ed20, mock_ed20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4237_cb70, mock_cb70 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4237_cd80, mock_cd80 as *const ())?;

    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let g_ed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let s_f48 = rd::<usize>(g_f48);
    let s_ed0 = rd::<usize>(g_ed0);

    let run = |f: F| -> (u64, u64, [u32; 16], u8, u64) {
        CALL_LOG = 0;

        let f48_bm: [u64; 1] = [0];
        let mut f48_obj = [0u8; 0x190];
        wr::<usize>(f48_obj.as_mut_ptr() as usize + 0x180, f48_bm.as_ptr() as usize);
        wr::<usize>(g_f48, f48_obj.as_mut_ptr() as usize);

        let mut ed0_obj = [0u8; 0x130];
        wr::<usize>(ed0_obj.as_mut_ptr() as usize + 0x120, 0x20_0000);
        wr::<usize>(g_ed0, ed0_obj.as_mut_ptr() as usize);

        let mut vtbl20 = [0usize; 6];
        vtbl20[0x28 / 8] = mock_v28 as *const () as usize;
        let mut vtblf0 = [0usize; 5];
        vtblf0[0x20 / 8] = mock_v20 as *const () as usize;

        let bm_548: [u64; 1] = [0b11]; // bits 0 and 1
        let mut bm_560: [u64; 1] = [0];
        let mut bm_c8: [u64; 1] = [0];
        let mut arr_b8 = [0u32; 16];

        let mut arr_578 = [0u8; 0x80];
        let p578 = arr_578.as_mut_ptr() as usize;
        // Entry 0: rdx = 0 -> ed20 returns u32::MAX -> calls cb70
        wr::<u32>(p578 + 0x08, 1);
        wr::<u32>(p578 + 0x10, 0);
        wr::<u32>(p578 + 0x20, 5);
        // Entry 1: rdx = 3, old_20 = u32::MAX -> calls cd80 and marks dirty
        wr::<u32>(p578 + 0x40 + 0x08, 2);
        wr::<u32>(p578 + 0x40 + 0x10, 3);
        wr::<u32>(p578 + 0x40 + 0x20, u32::MAX);

        let mut p1 = vec![0u8; 0x700];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x20, vtbl20.as_ptr() as usize);
        wr::<usize>(p1p + 0xb8, arr_b8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xc8, bm_c8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xf0, vtblf0.as_ptr() as usize);
        wr::<usize>(p1p + 0x548, bm_548.as_ptr() as usize);
        wr::<usize>(p1p + 0x560, bm_560.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x578, p578);
        wr::<u32>(p1p + 0x580, 2);

        f(p1p);
        (bm_560[0], bm_c8[0], arr_b8, rd::<u8>(p1p + 0x6e6), CALL_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_f48, s_f48);
    wr::<usize>(g_ed0, s_ed0);
    crate::diff_eq!(o, n, "142387250 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1423d8710(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_8720(a: u64, b: u64, c: u64, _d: u64) -> u64 {
        a.wrapping_mul(3)
            .wrapping_add(b.wrapping_mul(5))
            .wrapping_add(c.wrapping_mul(7))
    }

    let _m1 = crate::difftest::Mock::new(0x1_423d_8720, mock_8720 as *const ())?;

    let o = f_o(0x1111, 0x2222, 0x9999, 0x3333);
    let n = f_n(0x1111, 0x2222, 0x9999, 0x3333);
    crate::diff_eq!(o, n, "1423d8710 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1423d9450(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: u64 = 0;

    unsafe extern "win64" fn mock_a680(rcx: usize) {
        CALL_TRACE = CALL_TRACE.wrapping_add(rd::<u32>(rcx + 0x40) as u64);
    }
    unsafe extern "win64" fn mock_e380() {
        CALL_TRACE = CALL_TRACE.wrapping_add(0x10);
    }
    unsafe extern "win64" fn mock_v120(_sc: usize) -> u32 {
        7
    }
    unsafe extern "win64" fn mock_v70(_q: usize, _fence: usize, val: u64) -> u32 {
        CALL_TRACE = CALL_TRACE.wrapping_add(val);
        0
    }
    unsafe extern "win64" fn mock_v40(_fence: usize) -> u64 {
        100
    }
    unsafe extern "win64" fn mock_v58(_sc: usize, out_status: usize, _r8: usize) -> u32 {
        wr::<u32>(out_status, 1); // status == 1 skips IsIconic/be30/bd60
        0
    }

    let _m1 = crate::difftest::Mock::new(0x1_4207_a680, mock_a680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_e380, mock_e380 as *const ())?;

    let tbl_048 = exe::ptr::<usize>(0x1_43bd_6048) as usize;
    let g_c60 = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let g_fd0 = exe::ptr::<u8>(0x1_4385_7fd0) as usize;
    let s_048 = rd::<usize>(tbl_048);
    let s_c60 = rd::<usize>(g_c60);
    let s_fd0 = rd::<u8>(g_fd0);

    let run = |f: F| -> (u32, u64, u64) {
        CALL_TRACE = 0;

        let mut sc_vtbl = [0usize; 38];
        sc_vtbl[0x58 / 8] = mock_v58 as *const () as usize;
        sc_vtbl[0x120 / 8] = mock_v120 as *const () as usize;
        let sc_obj = [sc_vtbl.as_ptr() as usize];

        let mut fence_vtbl = [0usize; 9];
        fence_vtbl[0x40 / 8] = mock_v40 as *const () as usize;
        let fence_obj = [fence_vtbl.as_ptr() as usize];

        let mut q_vtbl = [0usize; 15];
        q_vtbl[0x70 / 8] = mock_v70 as *const () as usize;
        let q_obj = [q_vtbl.as_ptr() as usize];
        let c60_obj = [0usize, 0usize, q_obj.as_ptr() as usize];
        wr::<usize>(g_c60, c60_obj.as_ptr() as usize);
        wr::<u8>(g_fd0, 1);

        let mut rbx_0 = [0u8; 0x50];
        let r0p = rbx_0.as_mut_ptr() as usize;
        wr::<usize>(r0p + 0x08, sc_obj.as_ptr() as usize);
        wr::<usize>(r0p + 0x20, fence_obj.as_ptr() as usize);
        wr::<u64>(r0p + 0x28, 50);
        wr::<u8>(r0p + 0x41, 1);

        let mut rbx_obj = [0u8; 0x70];
        let rbxp = rbx_obj.as_mut_ptr() as usize;
        wr::<usize>(rbxp, r0p);
        wr::<u32>(rbxp + 0x40, 0x33);

        let mut entry = [0u8; 0x50];
        wr::<usize>(entry.as_mut_ptr() as usize + 0x48, rbxp);
        wr::<usize>(tbl_048, entry.as_ptr() as usize);

        let indices: [u32; 1] = [0];
        f(0, indices.as_ptr() as usize, 1);
        (rd::<u32>(rbxp + 0x38), rd::<u64>(r0p + 0x28), CALL_TRACE)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(tbl_048, s_048);
    wr::<usize>(g_c60, s_c60);
    wr::<u8>(g_fd0, s_fd0);
    crate::diff_eq!(o, n, "1423d9450 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4221_3400, "unknown_142213400", rend::cauldron_hot_y4::unknown_142213400 as *const (), Throws::Caught, rend::cauldron_hot_y4::diff_unknown_142213400),
// Replacement::verified_throws(0x1_4221_70a0, "unknown_1422170a0", rend::cauldron_hot_y4::unknown_1422170a0 as *const (), Throws::Caught, rend::cauldron_hot_y4::diff_unknown_1422170a0),
// Replacement::verified(0x1_4225_2b40, "r_142252b40", rend::cauldron_hot_y4::r_142252b40 as *const (), rend::cauldron_hot_y4::diff_r_142252b40),
// Replacement::verified(0x1_422d_f580, "unknown_1422df580", rend::cauldron_hot_y4::unknown_1422df580 as *const (), rend::cauldron_hot_y4::diff_unknown_1422df580),
// Replacement::verified_throws(0x1_4230_c9e0, "rend_14230c9e0", rend::cauldron_hot_y4::rend_14230c9e0 as *const (), Throws::Caught, rend::cauldron_hot_y4::diff_rend_14230c9e0),
// Replacement::verified(0x1_4232_6630, "rend_142326630", rend::cauldron_hot_y4::rend_142326630 as *const (), rend::cauldron_hot_y4::diff_rend_142326630),
// Replacement::verified(0x1_4232_a5c0, "unknown_14232a5c0", rend::cauldron_hot_y4::unknown_14232a5c0 as *const (), rend::cauldron_hot_y4::diff_unknown_14232a5c0),
// Replacement::verified(0x1_4234_b140, "rend_14234b140", rend::cauldron_hot_y4::rend_14234b140 as *const (), rend::cauldron_hot_y4::diff_rend_14234b140),
// Replacement::verified(0x1_4237_cd80, "unknown_14237cd80", rend::cauldron_hot_y4::unknown_14237cd80 as *const (), rend::cauldron_hot_y4::diff_unknown_14237cd80),
// Replacement::verified(0x1_4238_7250, "unknown_142387250", rend::cauldron_hot_y4::unknown_142387250 as *const (), rend::cauldron_hot_y4::diff_unknown_142387250),
// Replacement::verified_throws(0x1_423d_8710, "unknown_1423d8710", rend::cauldron_hot_y4::unknown_1423d8710 as *const (), Throws::Discard, rend::cauldron_hot_y4::diff_unknown_1423d8710),
// Replacement::verified_throws(0x1_423d_9450, "unknown_1423d9450", rend::cauldron_hot_y4::unknown_1423d9450 as *const (), Throws::Discard, rend::cauldron_hot_y4::diff_unknown_1423d9450),
