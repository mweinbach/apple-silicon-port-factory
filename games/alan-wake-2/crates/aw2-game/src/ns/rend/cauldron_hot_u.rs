//! Decompiled hot functions shard U (`0x141e07fb0`, `0x142364e00`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn rstr_cstr(s: usize) -> usize {
    let hdr = rd::<u32>(s);
    let top = hdr >> 24;
    let cap_bits = if top < 0x80 {
        top * 8
    } else {
        (top << 8).wrapping_sub(0x7c00)
    };
    let used_bits = (hdr & 0x00ff_ffff) << 3;
    if used_bits > cap_bits {
        rd::<usize>(s + 0x08)
    } else {
        s + 0x08
    }
}

#[inline(always)]
unsafe fn cstr_len(p: usize) -> usize {
    let mut len: usize = 0;
    while rd::<u8>(p.wrapping_add(len)) != 0 {
        len = len.wrapping_add(1);
    }
    len
}

/// `coregame::ui_data_transform::interpolate_localized_string::Transform::vfunc_1` (`coregame_141e07fb0`)
/// Original: `0x141e07fb0` (3782 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn coregame_141e07fb0(
    param_1: usize,
    _param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> u64 {
    let mut r14 = param_4.wrapping_add(0x08);
    if rd::<u16>(param_4) != 4 {
        let idx = rd::<usize>(r14);
        let entry = rd::<usize>(param_3 + 0x80).wrapping_add(idx * 0x48);
        r14 = rstr_cstr(entry);
    }

    let mut args_buf = [0u8; 0x50 * 5];
    let args_base = args_buf.as_mut_ptr() as usize;
    let arg4_ptr = args_base;
    let arg3_ptr = args_base + 0x50;
    let arg2_ptr = args_base + 0xa0;
    let arg1_ptr = args_base + 0xf0;
    let arg0_ptr = args_base + 0x140;

    exe::call_va!(0x1_41e0_8e80, (arg0_ptr, param_3, param_6, 0u32));
    exe::call_va!(0x1_41e0_8e80, (arg1_ptr, param_3, param_6, 1u32));
    exe::call_va!(0x1_41e0_8e80, (arg2_ptr, param_3, param_6, 2u32));
    exe::call_va!(0x1_41e0_8e80, (arg3_ptr, param_3, param_6, 3u32));
    exe::call_va!(0x1_41e0_8e80, (arg4_ptr, param_3, param_6, 4u32));

    let r12d = rd::<u32>(rd::<usize>(exe::ptr::<usize>(0x1_43a0_82e8) as usize) + 0x10);
    let r9_head = rd::<usize>(param_1 + 0x38);
    let mut rcx_node = rd::<usize>(r9_head + 0x08);
    let mut rdi_cand = r9_head;
    if rd::<u8>(rcx_node + 0x19) == 0 {
        let key_p5 = rd::<u64>(param_5 + 0x60) ^ rd::<u64>(param_5 + 0x68);
        loop {
            let key_node = rd::<u64>(rcx_node + 0x28) ^ rd::<u64>(rcx_node + 0x20);
            if key_node < key_p5 {
                rcx_node = rd::<usize>(rcx_node + 0x10);
            } else {
                rdi_cand = rcx_node;
                rcx_node = rd::<usize>(rcx_node);
            }
            if rd::<u8>(rcx_node + 0x19) != 0 {
                break;
            }
        }
    }

    let mut cache_hit = false;
    if rd::<u8>(rdi_cand + 0x19) == 0
        && (rd::<u64>(param_5 + 0x60) ^ rd::<u64>(param_5 + 0x68))
            >= (rd::<u64>(rdi_cand + 0x28) ^ rd::<u64>(rdi_cand + 0x20))
        && rdi_cand != r9_head
    {
        let rcx_len = cstr_len(r14);
        let r10_cstr = rstr_cstr(rdi_cand + 0x30);
        let edx_len = rd::<u32>(rdi_cand + 0x34) as usize;
        if edx_len == rcx_len {
            let mut str_eq = true;
            if edx_len != 0 {
                let tbl_1e0 = exe::ptr::<u8>(0x1_426d_01e0) as usize;
                for i in 0..edx_len {
                    let b0 = rd::<i8>(tbl_1e0 + (rd::<u8>(r10_cstr + i) as usize));
                    let b1 = rd::<i8>(tbl_1e0 + (rd::<u8>(r14 + i) as usize));
                    if b0 != b1 {
                        str_eq = false;
                        break;
                    }
                }
            }
            if str_eq
                && exe::call_va!(0x1_401e_6ff0, (arg0_ptr, rdi_cand + 0x78) -> u8) != 0
                && exe::call_va!(0x1_401e_6ff0, (arg1_ptr, rdi_cand + 0xc0) -> u8) != 0
                && exe::call_va!(0x1_401e_6ff0, (arg2_ptr, rdi_cand + 0x108) -> u8) != 0
                && exe::call_va!(0x1_401e_6ff0, (arg3_ptr, rdi_cand + 0x150) -> u8) != 0
                && exe::call_va!(0x1_401e_6ff0, (arg4_ptr, rdi_cand + 0x198) -> u8) != 0
                && r12d == rd::<u32>(rdi_cand + 0x1e0)
            {
                cache_hit = true;
            }
        }
    }

    let ret_bl: u64 = if cache_hit {
        0
    } else {
        let mut tmp_strs = [0u8; 0x48 * 6];
        let ts_base = tmp_strs.as_mut_ptr() as usize;
        let s_fmt = ts_base;
        let s_arg0 = ts_base + 0x48;
        let s_arg1 = ts_base + 0x90;
        let s_arg2 = ts_base + 0xd8;
        let s_arg3 = ts_base + 0x120;
        let s_arg4 = ts_base + 0x168;

        wr::<u64>(s_fmt, 0x0800_0008);
        wr::<u8>(s_fmt + 0x08, 0);
        if r14 == 0 {
            wr::<u32>(s_fmt + 0x04, 0);
            wr::<u8>(s_fmt + 0x08, 0);
        } else {
            let r9_len = cstr_len(r14);
            let mut view: [u64; 2] = [r14 as u64, r9_len as u64];
            exe::call_va!(
                0x1_4240_9b50,
                (s_fmt, view.as_mut_ptr() as usize, 0u32, r9_len as u64)
            );
        }

        for (dst_s, src_arg) in [
            (s_arg0, arg0_ptr),
            (s_arg1, arg1_ptr),
            (s_arg2, arg2_ptr),
            (s_arg3, arg3_ptr),
            (s_arg4, arg4_ptr),
        ] {
            wr::<u64>(dst_s, 0x0800_0008);
            wr::<u8>(dst_s + 0x08, 0);
            let c_ptr = rstr_cstr(src_arg);
            let c_len = rd::<u32>(src_arg + 0x04) as u64;
            let mut view: [u64; 2] = [c_ptr as u64, c_len];
            exe::call_va!(
                0x1_4240_9b50,
                (dst_s, view.as_mut_ptr() as usize, 0u32, c_len)
            );
        }

        let r12_map = param_1.wrapping_add(0x38);
        let r13_head = rd::<usize>(r12_map);
        let mut rcx_cur = rd::<usize>(r13_head + 0x08);
        let mut loc_parent = rcx_cur;
        let mut loc_child: u64 = 0;
        let mut rdi_node = r13_head;
        if rd::<u8>(rcx_cur + 0x19) == 0 {
            let key_p5 = rd::<u64>(param_5 + 0x68) ^ rd::<u64>(param_5 + 0x60);
            loop {
                loc_parent = rcx_cur;
                let key_node = rd::<u64>(rcx_cur + 0x28) ^ rd::<u64>(rcx_cur + 0x20);
                if key_node < key_p5 {
                    loc_child = 0;
                    rcx_cur = rd::<usize>(rcx_cur + 0x10);
                } else {
                    loc_child = 1;
                    rdi_node = rcx_cur;
                    rcx_cur = rd::<usize>(rcx_cur);
                }
                if rd::<u8>(rcx_cur + 0x19) != 0 {
                    break;
                }
            }
        }

        if rd::<u8>(rdi_node + 0x19) != 0
            || (rd::<u64>(param_5 + 0x68) ^ rd::<u64>(param_5 + 0x60))
                < (rd::<u64>(rdi_node + 0x28) ^ rd::<u64>(rdi_node + 0x20))
        {
            if rd::<u64>(r12_map + 0x08) == 0x0086_4b8a_7de6_d1d6 {
                exe::call_va!(0x1_4020_0130, ());
            }
            let new_node = exe::call_va!(0x1_40f5_c3f0, () -> usize);
            wr::<u128>(new_node + 0x20, rd::<u128>(param_5 + 0x60));
            let _ = exe::call_va!(0x1_4183_acc2, (new_node + 0x30, 0u32, 0x1b4u64) -> usize);
            exe::call_va!(0x1_40f5_c470, (new_node + 0x30));
            wr::<usize>(new_node, r13_head);
            wr::<usize>(new_node + 0x08, r13_head);
            wr::<usize>(new_node + 0x10, r13_head);
            wr::<u16>(new_node + 0x18, 0);
            let mut loc: [u64; 2] = [loc_parent as u64, loc_child];
            rdi_node = exe::call_va!(
                0x1_4020_8500,
                (r12_map, loc.as_mut_ptr() as usize, new_node) -> usize
            );
        }

        exe::call_va!(0x1_4240_a730, (rdi_node + 0x30, s_fmt));
        exe::call_va!(0x1_4240_a730, (rdi_node + 0x78, s_arg0));
        exe::call_va!(0x1_4240_a730, (rdi_node + 0xc0, s_arg1));
        exe::call_va!(0x1_4240_a730, (rdi_node + 0x108, s_arg2));
        exe::call_va!(0x1_4240_a730, (rdi_node + 0x150, s_arg3));
        exe::call_va!(0x1_4240_a730, (rdi_node + 0x198, s_arg4));
        wr::<u32>(rdi_node + 0x1e0, r12d);

        exe::call_va!(0x1_4240_ab10, (s_arg4));
        exe::call_va!(0x1_4240_ab10, (s_arg3));
        exe::call_va!(0x1_4240_ab10, (s_arg2));
        exe::call_va!(0x1_4240_ab10, (s_arg1));
        exe::call_va!(0x1_4240_ab10, (s_arg0));
        exe::call_va!(0x1_4240_ab10, (s_fmt));

        let mut loc_strs = [0u8; 0x50 * 6];
        let ls_base = loc_strs.as_mut_ptr() as usize;
        let loc_str0 = ls_base;
        let loc_str1 = ls_base + 0x50;
        let loc_str2 = ls_base + 0xa0;
        let loc_str3 = ls_base + 0xf0;
        let loc_str4 = ls_base + 0x140;
        let loc_fmt = ls_base + 0x190;

        let mut p_locs = [0usize; 5];
        for (idx_rev, (dst_loc, src_arg)) in [
            (4usize, (loc_str4, arg4_ptr)),
            (3usize, (loc_str3, arg3_ptr)),
            (2usize, (loc_str2, arg2_ptr)),
            (1usize, (loc_str1, arg1_ptr)),
            (0usize, (loc_str0, arg0_ptr)),
        ] {
            let src_cstr = rstr_cstr(src_arg);
            let src_len = rd::<u32>(src_arg + 0x04);
            wr::<u64>(dst_loc, 0x0800_0008);
            wr::<u8>(dst_loc + 0x08, 0);
            if src_len != 0 {
                let mgr = exe::call_va!(0x1_420e_af00, () -> usize);
                exe::call_va!(0x1_420e_a540, (mgr, src_cstr, dst_loc));
            }
            p_locs[idx_rev] = rstr_cstr(dst_loc);
        }

        let r14_len = cstr_len(r14);
        wr::<u64>(loc_fmt, 0x0800_0008);
        wr::<u8>(loc_fmt + 0x08, 0);
        if r14_len != 0 {
            let mgr = exe::call_va!(0x1_420e_af00, () -> usize);
            exe::call_va!(0x1_420e_a540, (mgr, r14, loc_fmt));
        }
        let fmt_cstr = exe::call_va!(0x1_401d_f680, (loc_fmt) -> usize);
        let fmt_len = cstr_len(fmt_cstr);
        let rdi_buf = exe::call_va!(0x1_423f_dfc0, () -> usize);

        let mut fmt_store = [0u64; 22];
        let fs_ptr = fmt_store.as_mut_ptr() as usize;
        wr::<usize>(fs_ptr, fs_ptr + 0x60);
        wr::<u64>(fs_ptr + 0x08, 5);
        wr::<usize>(fs_ptr + 0x10, p_locs[0]);
        wr::<usize>(fs_ptr + 0x20, p_locs[1]);
        wr::<usize>(fs_ptr + 0x30, p_locs[2]);
        wr::<usize>(fs_ptr + 0x40, p_locs[3]);
        wr::<usize>(fs_ptr + 0x50, p_locs[4]);

        let named_strs: [u64; 5] = [
            0x1_4331_332c,
            0x1_4331_3334,
            0x1_4331_3314,
            0x1_4331_331c,
            0x1_4331_3324,
        ];
        for i in 0..5usize {
            let entry_p = fs_ptr + 0x60 + i * 0x10;
            wr::<usize>(entry_p, exe::ptr::<u8>(named_strs[i]) as usize);
            wr::<u32>(entry_p + 0x08, i as u32);
        }

        let mut iter_buf = [0u8; 0x140];
        let ib_ptr = iter_buf.as_mut_ptr() as usize;
        let vtbl_fmt = exe::ptr::<u8>(0x1_4307_9330) as usize;
        wr::<usize>(ib_ptr, vtbl_fmt);
        wr::<usize>(ib_ptr + 0x08, rdi_buf);
        wr::<u64>(ib_ptr + 0x10, 0);
        wr::<u64>(ib_ptr + 0x18, 0x7ff);
        wr::<u64>(ib_ptr + 0x20, 0);
        wr::<u64>(ib_ptr + 0x28, 0x7ff);
        wr::<usize>(ib_ptr + 0x30, rdi_buf);

        let mut fmt_view: [u64; 2] = [fmt_cstr as u64, fmt_len as u64];
        let mut fmt_args: [u64; 2] = [0x4000_0000_000c_cccc, (fs_ptr + 0x10) as u64];
        exe::call_va!(
            0x1_4146_1fb0,
            (
                ib_ptr,
                fmt_view.as_mut_ptr() as usize,
                fmt_args.as_mut_ptr() as usize,
                0u64
            )
        );

        let rdx_sz = rd::<u64>(ib_ptr + 0x10);
        let r8_lim = rd::<u64>(ib_ptr + 0x28);
        let rcx_cnt = rd::<u64>(ib_ptr + 0x20);
        let mut r9_avail = if r8_lim > rcx_cnt {
            r8_lim.wrapping_sub(rcx_cnt)
        } else {
            0
        };
        let rax_new_cnt = rcx_cnt.wrapping_add(rdx_sz);
        if rdx_sz < r9_avail {
            r9_avail = rdx_sz;
        }
        let mut rdx_ptr = rd::<usize>(ib_ptr + 0x08);
        let mut rcx_out = rd::<usize>(ib_ptr + 0x30);
        if rdx_ptr == rcx_out {
            rcx_out = rcx_out.wrapping_add(r9_avail as usize);
            wr::<usize>(ib_ptr + 0x30, rcx_out);
            rdx_ptr = ib_ptr + 0x38;
            wr::<usize>(ib_ptr + 0x08, rdx_ptr);
            wr::<u64>(ib_ptr + 0x18, 0x100);
        }
        wr::<usize>(ib_ptr, vtbl_fmt);
        wr::<u64>(ib_ptr + 0x20, rax_new_cnt);
        if rdx_ptr == rcx_out {
            wr::<usize>(ib_ptr + 0x30, rcx_out);
            wr::<usize>(ib_ptr + 0x08, ib_ptr + 0x38);
            wr::<u64>(ib_ptr + 0x18, 0x100);
        }
        wr::<u64>(ib_ptr + 0x10, 0);
        wr::<u8>(rcx_out, 0);

        let mut res_str = [0u8; 0x50];
        let res_p = res_str.as_mut_ptr() as usize;
        wr::<u64>(res_p, 0x0800_0008);
        wr::<u8>(res_p + 0x08, 0);
        if rdi_buf == 0 {
            wr::<u32>(res_p + 0x04, 0);
            wr::<u8>(res_p + 0x08, 0);
        } else {
            let out_len = cstr_len(rdi_buf);
            let mut view: [u64; 2] = [rdi_buf as u64, out_len as u64];
            exe::call_va!(
                0x1_4240_9b50,
                (res_p, view.as_mut_ptr() as usize, 0u32, out_len as u64)
            );
        }

        exe::call_va!(0x1_4240_ab10, (loc_fmt));
        exe::call_va!(0x1_4240_ab10, (loc_str0));
        exe::call_va!(0x1_4240_ab10, (loc_str1));
        exe::call_va!(0x1_4240_ab10, (loc_str2));
        exe::call_va!(0x1_4240_ab10, (loc_str3));
        exe::call_va!(0x1_4240_ab10, (loc_str4));

        let dst_80 = param_5.wrapping_add(0x80);
        if rd::<u8>(dst_80 + 0x48) == 2 {
            exe::call_va!(0x1_4240_aa00, (dst_80, res_p));
        } else {
            exe::call_va!(0x1_40f4_5520, (dst_80));
            let v = (rd::<u32>(dst_80) & 0xff00_0008) | 8;
            wr::<u32>(dst_80, v);
            wr::<u8>(dst_80 + 0x03, 8);
            wr::<u32>(dst_80 + 0x04, 0);
            wr::<u8>(dst_80 + 0x08, 0);
            exe::call_va!(0x1_4240_aa00, (dst_80, res_p));
            wr::<u8>(dst_80 + 0x48, 2);
        }

        exe::call_va!(0x1_4240_ab10, (res_p));
        1
    };

    exe::call_va!(0x1_4240_ab10, (arg4_ptr));
    exe::call_va!(0x1_4240_ab10, (arg3_ptr));
    exe::call_va!(0x1_4240_ab10, (arg2_ptr));
    exe::call_va!(0x1_4240_ab10, (arg1_ptr));
    exe::call_va!(0x1_4240_ab10, (arg0_ptr));
    ret_bl
}

#[inline(always)]
fn clamp01_eq(v: f32, hi: f32) -> bool {
    let max0 = if v > 0.0f32 { v } else { 0.0f32 };
    let cl = if max0 < hi { max0 } else { hi };
    !cl.is_nan() && !v.is_nan() && cl == v
}

/// `unknown_142364e00`
/// Original: `0x142364e00` (3867 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_142364e00(param_1: usize) {
    wr::<u64>(param_1 + 0x100, rd::<u64>(param_1 + 0x1c0));
    wr::<u32>(param_1 + 0x108, rd::<u32>(param_1 + 0x1c8));
    wr::<u64>(param_1 + 0x10c, rd::<u64>(param_1 + 0x1cc));
    wr::<u32>(param_1 + 0x114, rd::<u32>(param_1 + 0x1d4));
    wr::<u64>(param_1 + 0x118, rd::<u64>(param_1 + 0x1d8));
    wr::<u32>(param_1 + 0x120, rd::<u32>(param_1 + 0x1e0));
    wr::<u64>(param_1 + 0x124, rd::<u64>(param_1 + 0x1e4));
    wr::<u32>(param_1 + 0x12c, rd::<u32>(param_1 + 0x1ec));

    let mut mat_30 = [0u8; 0x30];
    let m30_ptr = mat_30.as_mut_ptr() as usize;
    wr::<u64>(m30_ptr, rd::<u64>(param_1 + 0x100));
    wr::<u32>(m30_ptr + 0x08, rd::<u32>(param_1 + 0x108));
    wr::<u64>(m30_ptr + 0x0c, rd::<u64>(param_1 + 0x10c));
    wr::<u32>(m30_ptr + 0x14, rd::<u32>(param_1 + 0x114));
    wr::<u64>(m30_ptr + 0x18, rd::<u64>(param_1 + 0x118));
    wr::<u32>(m30_ptr + 0x20, rd::<u32>(param_1 + 0x120));
    wr::<u64>(m30_ptr + 0x24, rd::<u64>(param_1 + 0x124));
    wr::<u32>(m30_ptr + 0x2c, rd::<u32>(param_1 + 0x12c));

    exe::call_va!(0x1_4024_1cf0, (m30_ptr));

    wr::<u64>(param_1 + 0x130, rd::<u64>(m30_ptr));
    wr::<u32>(param_1 + 0x138, rd::<u32>(m30_ptr + 0x08));
    wr::<u64>(param_1 + 0x13c, rd::<u64>(m30_ptr + 0x0c));
    wr::<u32>(param_1 + 0x144, rd::<u32>(m30_ptr + 0x14));
    wr::<u64>(param_1 + 0x148, rd::<u64>(m30_ptr + 0x18));
    wr::<u32>(param_1 + 0x150, rd::<u32>(m30_ptr + 0x20));
    wr::<u64>(param_1 + 0x154, rd::<u64>(m30_ptr + 0x24));
    wr::<u32>(param_1 + 0x15c, rd::<u32>(m30_ptr + 0x2c));

    let mut rbx = rd::<usize>(param_1);
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let rsi_end = rbx.wrapping_add(count * 8);
    if rbx == rsi_end {
        return;
    }

    let c_fd0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fd0) as usize);
    let c_174 = rd::<f32>(exe::ptr::<f32>(0x1_4342_7174) as usize);
    let c_8d0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let c_504 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6504) as usize);
    let c_e84 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e84) as usize);

    while rbx != rsi_end {
        let item = rd::<usize>(rbx);
        if rd::<u8>(item + 0x10e4) != 0 || rd::<u8>(param_1 + 0x1f3) != 0 {
            let rdx = rd::<usize>(item).wrapping_add(0xd0);
            let mut pts_80 = [0f32; 12];
            let pts_ptr = pts_80.as_mut_ptr() as usize;
            wr::<f32>(pts_ptr, 0.0f32);
            wr::<f32>(pts_ptr + 0x04, 0.0f32);
            wr::<f32>(pts_ptr + 0x08, c_8d0);
            exe::call_va!(0x1_4024_2a90, (pts_ptr, rdx));

            let z0 = 0.0f32;
            wr::<f32>(
                pts_ptr + 0x0c,
                ((z0 * rd::<f32>(rdx + 0x0c)) + rd::<f32>(rdx))
                    + rd::<f32>(rdx + 0x18)
                    + rd::<f32>(rdx + 0x24),
            );
            wr::<f32>(
                pts_ptr + 0x10,
                ((z0 * rd::<f32>(rdx + 0x10)) + rd::<f32>(rdx + 0x04))
                    + rd::<f32>(rdx + 0x1c)
                    + rd::<f32>(rdx + 0x28),
            );
            wr::<f32>(
                pts_ptr + 0x14,
                ((z0 * rd::<f32>(rdx + 0x14)) + rd::<f32>(rdx + 0x08))
                    + rd::<f32>(rdx + 0x20)
                    + rd::<f32>(rdx + 0x2c),
            );

            wr::<f32>(pts_ptr + 0x18, c_8d0);
            wr::<f32>(pts_ptr + 0x1c, 0.0f32);
            wr::<f32>(pts_ptr + 0x20, 0.0f32);
            exe::call_va!(0x1_4024_2a90, (pts_ptr + 0x18, rdx));

            wr::<f32>(
                pts_ptr + 0x24,
                ((z0 * rd::<f32>(rdx + 0x0c)) + (z0 * rd::<f32>(rdx)))
                    + (z0 * rd::<f32>(rdx + 0x18))
                    + rd::<f32>(rdx + 0x24),
            );
            wr::<f32>(
                pts_ptr + 0x28,
                ((z0 * rd::<f32>(rdx + 0x10)) + (z0 * rd::<f32>(rdx + 0x04)))
                    + (z0 * rd::<f32>(rdx + 0x1c))
                    + rd::<f32>(rdx + 0x28),
            );
            wr::<f32>(
                pts_ptr + 0x2c,
                ((z0 * rd::<f32>(rdx + 0x14)) + (z0 * rd::<f32>(rdx + 0x08)))
                    + (z0 * rd::<f32>(rdx + 0x20))
                    + rd::<f32>(rdx + 0x2c),
            );

            let mut proj_2d = [[0.0f32; 2]; 4];
            let mut min_v = c_fd0;
            let mut min_u = c_fd0;
            let mut max_u = c_174;
            let mut max_v = c_174;

            let m130 = rd::<f32>(param_1 + 0x130);
            let m138 = rd::<f32>(param_1 + 0x138);
            let m13c = rd::<f32>(param_1 + 0x13c);
            let m144 = rd::<f32>(param_1 + 0x144);
            let m148 = rd::<f32>(param_1 + 0x148);
            let m150 = rd::<f32>(param_1 + 0x150);
            let m154 = rd::<f32>(param_1 + 0x154);
            let m15c = rd::<f32>(param_1 + 0x15c);

            for pair_idx in 0..2usize {
                let rcx_p = pts_ptr + pair_idx * 0x18;
                let x0 = rd::<f32>(rcx_p);
                let y0 = rd::<f32>(rcx_p + 0x04);
                let z0_p = rd::<f32>(rcx_p + 0x08);
                let u0 = ((y0 * m13c) + (m130 * x0)) + (m148 * z0_p) + m154;
                let v0 = ((m138 * x0) + (y0 * m144)) + (z0_p * m150) + m15c;
                proj_2d[pair_idx * 2] = [u0, v0];

                min_v = if v0 < min_v { v0 } else { min_v };
                min_u = if u0 < min_u { u0 } else { min_u };
                let max_v_mid = if v0 > max_v { v0 } else { max_v };
                let max_u_mid = if u0 > max_u { u0 } else { max_u };

                let x1 = rd::<f32>(rcx_p + 0x0c);
                let y1 = rd::<f32>(rcx_p + 0x10);
                let z1_p = rd::<f32>(rcx_p + 0x14);
                let u1 = ((y1 * m13c) + (m130 * x1)) + (m148 * z1_p) + m154;
                let v1 = ((x1 * m138) + (y1 * m144)) + (z1_p * m150) + m15c;
                proj_2d[pair_idx * 2 + 1] = [u1, v1];

                min_v = if v1 < min_v { v1 } else { min_v };
                min_u = if u1 < min_u { u1 } else { min_u };
                max_v = if v1 > max_v_mid { v1 } else { max_v_mid };
                max_u = if u1 > max_u_mid { u1 } else { max_u_mid };
            }

            let px = ((z0 * rd::<f32>(rdx + 0x0c)) + (z0 * rd::<f32>(rdx)))
                + (z0 * rd::<f32>(rdx + 0x18))
                + rd::<f32>(rdx + 0x24);
            let py = ((z0 * rd::<f32>(rdx + 0x10)) + (z0 * rd::<f32>(rdx + 0x04)))
                + (z0 * rd::<f32>(rdx + 0x1c))
                + rd::<f32>(rdx + 0x28);
            let pz = ((z0 * rd::<f32>(rdx + 0x14)) + (z0 * rd::<f32>(rdx + 0x08)))
                + (z0 * rd::<f32>(rdx + 0x20))
                + rd::<f32>(rdx + 0x2c);
            let w_10e0 = ((py * rd::<f32>(param_1 + 0x140)) + (px * rd::<f32>(param_1 + 0x134)))
                + (pz * rd::<f32>(param_1 + 0x14c))
                + rd::<f32>(param_1 + 0x158);
            wr::<f32>(rd::<usize>(rbx) + 0x10e0, w_10e0);

            let xmm14 = proj_2d[0][0];
            let xmm13 = proj_2d[0][1];
            let xmm10 = proj_2d[1][0] - xmm14;
            let xmm9 = proj_2d[1][1] - xmm13;
            let xmm12 = proj_2d[3][0] - xmm14;
            let xmm11 = proj_2d[3][1] - xmm13;

            wr::<f32>(m30_ptr, xmm10);
            wr::<f32>(m30_ptr + 0x04, xmm9);
            wr::<f32>(m30_ptr + 0x08, 0.0f32);
            wr::<f32>(m30_ptr + 0x0c, xmm12);
            wr::<f32>(m30_ptr + 0x10, xmm11);
            wr::<f32>(m30_ptr + 0x14, 0.0f32);
            wr::<f32>(m30_ptr + 0x18, xmm14);
            wr::<f32>(m30_ptr + 0x1c, xmm13);
            wr::<f32>(m30_ptr + 0x20, c_8d0);

            exe::call_va!(0x1_4024_2c40, (m30_ptr));

            let item_cur = rd::<usize>(rbx);
            wr::<f32>(item_cur + 0x104c, xmm10);
            wr::<f32>(item_cur + 0x1050, xmm9);
            wr::<f32>(item_cur + 0x1054, 0.0f32);
            wr::<f32>(item_cur + 0x1058, xmm12);
            wr::<f32>(item_cur + 0x105c, xmm11);
            wr::<f32>(item_cur + 0x1060, 0.0f32);
            wr::<f32>(item_cur + 0x1064, xmm14);
            wr::<f32>(item_cur + 0x1068, xmm13);
            wr::<f32>(item_cur + 0x106c, c_8d0);

            let item_cur2 = rd::<usize>(rbx);
            wr::<u64>(item_cur2 + 0x1070, rd::<u64>(m30_ptr));
            wr::<u32>(item_cur2 + 0x1078, rd::<u32>(m30_ptr + 0x08));
            wr::<u64>(item_cur2 + 0x107c, rd::<u64>(m30_ptr + 0x0c));
            wr::<u32>(item_cur2 + 0x1084, rd::<u32>(m30_ptr + 0x14));
            wr::<u64>(item_cur2 + 0x1088, rd::<u64>(m30_ptr + 0x18));
            wr::<u32>(item_cur2 + 0x1090, rd::<u32>(m30_ptr + 0x20));

            let rdx_0 = rd::<usize>(rd::<usize>(rbx));
            let rcx_b8 = rd::<usize>(rd::<usize>(rdx_0 + 0xa0) + 0xb8);
            let xmm4_scale = rd::<f32>(rcx_b8 + 0x14) * rd::<f32>(rdx_0 + 0x74);

            let mut mat_80: [f32; 16] = [
                rd::<f32>(param_1 + 0x100),
                rd::<f32>(param_1 + 0x104),
                rd::<f32>(param_1 + 0x108),
                0.0f32,
                rd::<f32>(param_1 + 0x10c),
                rd::<f32>(param_1 + 0x110),
                rd::<f32>(param_1 + 0x114),
                0.0f32,
                rd::<f32>(param_1 + 0x118),
                rd::<f32>(param_1 + 0x11c),
                rd::<f32>(param_1 + 0x120),
                0.0f32,
                rd::<f32>(param_1 + 0x124),
                rd::<f32>(param_1 + 0x128),
                rd::<f32>(param_1 + 0x12c),
                c_8d0,
            ];
            let mut mat_120: [f32; 16] = [
                xmm10,
                0.0f32,
                xmm9,
                0.0f32,
                0.0f32,
                xmm4_scale,
                0.0f32,
                0.0f32,
                xmm12,
                0.0f32,
                xmm11,
                0.0f32,
                xmm14,
                0.0f32,
                xmm13,
                c_8d0,
            ];
            exe::call_va!(
                0x1_403b_7530,
                (
                    mat_120.as_mut_ptr() as usize,
                    mat_80.as_mut_ptr() as usize
                )
            );

            let item_cur3 = rd::<usize>(rbx);
            let m120_p = mat_120.as_ptr() as usize;
            wr::<u128>(item_cur3 + 0x10a0, rd::<u128>(m120_p));
            wr::<u128>(item_cur3 + 0x10b0, rd::<u128>(m120_p + 0x10));
            wr::<u128>(item_cur3 + 0x10c0, rd::<u128>(m120_p + 0x20));
            wr::<u128>(item_cur3 + 0x10d0, rd::<u128>(m120_p + 0x30));

            let r8d_min_v = crate::simd_ops::cvt_f32_i32((min_v * c_504).floor()) as i32;
            let ecx_min_u = crate::simd_ops::cvt_f32_i32((min_u * c_504).floor()) as i32;
            let eax_max_v = crate::simd_ops::cvt_f32_i32((max_v * c_504).ceil()) as i32;
            let r13d_max_u = crate::simd_ops::cvt_f32_i32((max_u * c_504).ceil()) as i32;

            let count_u = r13d_max_u.wrapping_sub(ecx_min_u);
            let count_v = eax_max_v.wrapping_sub(r8d_min_v);

            let item_cur4 = rd::<usize>(rbx);
            wr::<i32>(item_cur4 + 0x1108, ecx_min_u);
            wr::<i32>(item_cur4 + 0x110c, r8d_min_v);
            wr::<i32>(item_cur4 + 0x1110, r13d_max_u);
            wr::<i32>(item_cur4 + 0x1114, eax_max_v);
            wr::<u32>(item_cur4 + 0x10f0, 0);

            if count_v > 0 {
                let inv_m00 = rd::<f32>(m30_ptr);
                let inv_m01 = rd::<f32>(m30_ptr + 0x04);
                let inv_m10 = rd::<f32>(m30_ptr + 0x0c);
                let inv_m11 = rd::<f32>(m30_ptr + 0x10);
                let inv_m20 = rd::<f32>(m30_ptr + 0x18);
                let inv_m21 = rd::<f32>(m30_ptr + 0x1c);

                for r12d in 0..(count_v as u32) {
                    if count_u > 0 {
                        let v0 = (r8d_min_v.wrapping_add(r12d as i32) as f32) * c_e84;
                        let v0_m10 = inv_m10 * v0;
                        let v0_m11 = v0 * inv_m11;
                        let v1 =
                            (r8d_min_v.wrapping_add(1).wrapping_add(r12d as i32) as f32) * c_e84;
                        let v1_m10 = inv_m10 * v1;
                        let v1_m11 = v1 * inv_m11;

                        for r15d in 0..(count_u as u32) {
                            let ecx_u = ecx_min_u.wrapping_add(r15d as i32);
                            let u0 = (ecx_u as f32) * c_e84;
                            let u1 = (ecx_u.wrapping_add(1) as f32) * c_e84;

                            let u0_m00 = u0 * inv_m00;
                            let x00 = (v0_m10 + u0_m00) + inv_m20;
                            let u0_m01 = u0 * inv_m01;
                            let y00 = (v0_m11 + u0_m01) + inv_m21;

                            let u1_m00 = u1 * inv_m00;
                            let x11 = (v1_m10 + u1_m00) + inv_m20;
                            let u1_m01 = u1 * inv_m01;
                            let y11 = (v1_m11 + u1_m01) + inv_m21;

                            let x01 = (v1_m10 + u0_m00) + inv_m20;
                            let y01 = (v1_m11 + u0_m01) + inv_m21;

                            let x10 = (u1_m00 + v0_m10) + inv_m20;
                            let y10 = (v0_m11 + u1_m01) + inv_m21;

                            let in0 = x00 >= 0.0f32 && y00 >= 0.0f32 && c_8d0 > x00 && c_8d0 > y00;
                            let in1 = x11 >= 0.0f32 && y11 >= 0.0f32 && c_8d0 > x11 && c_8d0 > y11;
                            let in2 = x01 >= 0.0f32 && y01 >= 0.0f32 && c_8d0 > x01 && c_8d0 > y01;
                            let in3 = x10 >= 0.0f32 && y10 >= 0.0f32 && c_8d0 > x10 && c_8d0 > y10;

                            if in0 || in1 || in2 || in3 {
                                let vec_10e8 = rd::<usize>(rbx).wrapping_add(0x10e8);
                                let len = rd::<u32>(vec_10e8 + 0x08) as u64;
                                let cap = rd::<u32>(vec_10e8 + 0x0c) as u64;
                                if (len << 3) == (cap << 3) {
                                    exe::call_va!(0x1_401e_6d50, (vec_10e8, len.wrapping_add(1)));
                                }
                                let idx = rd::<u32>(vec_10e8 + 0x08) as usize;
                                let data = rd::<usize>(vec_10e8);
                                let packed = ((r12d as u64) << 32) | (r15d as u64);
                                wr::<u64>(data.wrapping_add(idx * 8), packed);
                                wr::<u32>(vec_10e8 + 0x08, (idx as u32).wrapping_add(1));
                            }
                        }
                    }
                }
            }

            wr::<u32>(rd::<usize>(rbx) + 0x1100, 0);
            let r12_obj = rd::<usize>(param_1 + 0x48);
            if r12_obj != 0 {
                let sub_188 = r12_obj.wrapping_add(0x188);
                let vtbl_188 = rd::<usize>(sub_188);
                exe::call_va!(rd::<usize>(vtbl_188 + 0x38), (sub_188));
                let verts_220 = rd::<usize>(r12_obj + 0x220);

                let rdi_obj = rd::<usize>(param_1 + 0x48);
                let sub_a8 = rdi_obj.wrapping_add(0xa8);
                let vtbl_a8 = rd::<usize>(sub_a8);
                exe::call_va!(rd::<usize>(vtbl_a8 + 0x38), (sub_a8));
                let indices_140 = rd::<usize>(rdi_obj + 0x140);

                let r9d_cnt = rd::<u32>(rd::<usize>(param_1 + 0x48) + 0x18);
                for r14d in 0..r9d_cnt {
                    let r15_off = (r14d as usize) * 0x0c;
                    let rdi_item = rd::<usize>(rbx);
                    let idx0 = rd::<u32>(indices_140.wrapping_add(r15_off)) as usize;
                    let v0_y = rd::<f32>(verts_220.wrapping_add(idx0 * 8 + 4));
                    let v0_x = rd::<f32>(verts_220.wrapping_add(idx0 * 8));

                    let m00 = rd::<f32>(rdi_item + 0x1070);
                    let m10 = rd::<f32>(rdi_item + 0x107c);
                    let m20 = rd::<f32>(rdi_item + 0x1088);
                    let p0_x = (m00 * v0_x + v0_y * m10) + m20;

                    let m01 = rd::<f32>(rdi_item + 0x1074);
                    let m11 = rd::<f32>(rdi_item + 0x1080);
                    let m21 = rd::<f32>(rdi_item + 0x108c);
                    let p0_y = (m01 * v0_x + v0_y * m11) + m21;

                    let m02 = rd::<f32>(rdi_item + 0x1078);
                    let m12 = rd::<f32>(rdi_item + 0x1084);
                    let m22 = rd::<f32>(rdi_item + 0x1090);
                    let p0_z = (m02 * v0_x + v0_y * m12) + m22;

                    let idx1 = rd::<u32>(indices_140.wrapping_add(r15_off + 4)) as usize;
                    let v1_y = rd::<f32>(verts_220.wrapping_add(idx1 * 8 + 4));
                    let v1_x = rd::<f32>(verts_220.wrapping_add(idx1 * 8));

                    let p1_x = (m00 * v1_x + v1_y * m10) + m20;
                    let p1_y = (m01 * v1_x + v1_y * m11) + m21;
                    let p1_z = (m02 * v1_x + v1_y * m12) + m22;

                    let ok0 = clamp01_eq(p0_x, c_8d0)
                        && clamp01_eq(p0_y, c_8d0)
                        && clamp01_eq(p0_z, c_8d0);
                    let ok1 = if ok0 {
                        false
                    } else {
                        clamp01_eq(p1_x, c_8d0)
                            && clamp01_eq(p1_y, c_8d0)
                            && clamp01_eq(p1_z, c_8d0)
                    };

                    if ok0 || ok1 {
                        let vec_10f8 = rdi_item.wrapping_add(0x10f8);
                        let len = rd::<u32>(vec_10f8 + 0x08) as u64;
                        let cap = rd::<u32>(vec_10f8 + 0x0c) as u64;
                        if (len << 2) == (cap << 2) {
                            let want = len.wrapping_add(1);
                            let mut new_cap = (cap >> 1).wrapping_add(cap);
                            if new_cap < want {
                                new_cap = want;
                            }
                            exe::call_va!(0x1_401d_eb40, (vec_10f8, new_cap));
                        }
                        let idx = rd::<u32>(vec_10f8 + 0x08) as usize;
                        wr::<u32>(rd::<usize>(vec_10f8).wrapping_add(idx * 4), r14d);
                        wr::<u32>(vec_10f8 + 0x08, (idx as u32).wrapping_add(1));
                    }
                }
            }

            exe::call_va!(0x1_4236_7240, (rd::<usize>(rbx), 1u32));
        }

        rbx = rbx.wrapping_add(8);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_coregame_141e07fb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;
    static mut ALLOC_NODE_PTR: usize = 0;
    static mut FMT_SCRATCH_PTR: usize = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    #[inline(always)]
    unsafe fn write_inline_rstr(dst: usize, bytes: &[u8]) {
        let len = bytes.len().min(30);
        wr::<u32>(dst, 0x0800_0008);
        wr::<u32>(dst + 0x04, len as u32);
        for (i, &b) in bytes.iter().take(len).enumerate() {
            wr::<u8>(dst + 0x08 + i, b);
        }
        wr::<u8>(dst + 0x08 + len, 0);
    }

    unsafe extern "win64" fn mock_8e80(out_rstr: usize, _p3: usize, _p6: usize, idx: u32) {
        unsafe { rec(0x8e80 ^ (idx as u64)) };
        let s: &[u8] = match idx {
            0 => b"A0",
            1 => b"A1",
            2 => b"",
            3 => b"A3",
            _ => b"A4",
        };
        unsafe { write_inline_rstr(out_rstr, s) };
    }
    unsafe extern "win64" fn mock_6ff0(lhs: usize, rhs: usize) -> u8 {
        let l_len = rd::<u32>(lhs + 0x04);
        let r_len = rd::<u32>(rhs + 0x04);
        unsafe { rec(0x6ff0 ^ (l_len as u64) ^ ((r_len as u64) << 8)) };
        if l_len == r_len { 1 } else { 0 }
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, view: usize, _flag: u32, _count: u64) {
        let p = rd::<usize>(view);
        let len = (rd::<u64>(view + 0x08) as usize).min(30);
        unsafe { rec(0x9b50 ^ (len as u64)) };
        wr::<u32>(dst, 0x0800_0008);
        wr::<u32>(dst + 0x04, len as u32);
        for i in 0..len {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(p + i));
        }
        wr::<u8>(dst + 0x08 + len, 0);
    }
    unsafe extern "win64" fn mock_c3f0() -> usize {
        unsafe {
            rec(0xc3f0);
            ALLOC_NODE_PTR
        }
    }
    unsafe extern "win64" fn mock_c470(val_ptr: usize) {
        unsafe { rec(0xc470 ^ ((val_ptr == ALLOC_NODE_PTR + 0x30) as u64)) };
    }
    unsafe extern "win64" fn mock_8500(map_ptr: usize, loc_ptr: usize, new_node: usize) -> usize {
        let parent = rd::<usize>(loc_ptr);
        let child = rd::<u32>(loc_ptr + 0x08);
        unsafe {
            rec(
                0x8500
                    ^ ((map_ptr != 0) as u64)
                    ^ ((parent != 0) as u64)
                    ^ ((child as u64) << 8)
                    ^ ((new_node == ALLOC_NODE_PTR) as u64),
            );
        }
        new_node
    }
    unsafe extern "win64" fn mock_a730(dst: usize, src: usize) {
        let len = (rd::<u32>(src + 0x04) as usize).min(30);
        unsafe { rec(0xa730 ^ (len as u64)) };
        wr::<u32>(dst, rd::<u32>(src));
        wr::<u32>(dst + 0x04, len as u32);
        for i in 0..=len {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(src + 0x08 + i));
        }
    }
    unsafe extern "win64" fn mock_ab10(s: usize) {
        unsafe { rec(0xab10 ^ (rd::<u32>(s + 0x04) as u64)) };
    }
    unsafe extern "win64" fn mock_af00() -> usize {
        unsafe { rec(0xaf00) };
        0x1234_5678
    }
    unsafe extern "win64" fn mock_a540(mgr: usize, cstr: usize, dst: usize) {
        let l = unsafe { cstr_len(cstr) }.min(28);
        unsafe { rec(0xa540 ^ (mgr as u64) ^ (l as u64)) };
        wr::<u32>(dst, 0x0800_0008);
        wr::<u32>(dst + 0x04, (l + 2) as u32);
        wr::<u8>(dst + 0x08, b'L');
        wr::<u8>(dst + 0x09, b':');
        for i in 0..l {
            wr::<u8>(dst + 0x0a + i, rd::<u8>(cstr + i));
        }
        wr::<u8>(dst + 0x0a + l, 0);
    }
    unsafe extern "win64" fn mock_f680(s: usize) -> usize {
        unsafe {
            rec(0xf680 ^ (rd::<u32>(s + 0x04) as u64));
            rstr_cstr(s)
        }
    }
    unsafe extern "win64" fn mock_dfc0() -> usize {
        unsafe {
            rec(0xdfc0);
            FMT_SCRATCH_PTR
        }
    }
    unsafe extern "win64" fn mock_1fb0(
        ib_ptr: usize,
        fmt_view: usize,
        fmt_args: usize,
        r9: u64,
    ) {
        let fmt_len = rd::<u64>(fmt_view + 0x08);
        let desc = rd::<u64>(fmt_args);
        let store_p = rd::<usize>(fmt_args + 0x08);
        let named_cnt = rd::<u64>(store_p - 0x08);
        let named_arr = rd::<usize>(store_p - 0x10);
        let id4 = rd::<u32>(named_arr + 4 * 0x10 + 0x08);
        let a0_ch = rd::<u8>(rd::<usize>(store_p));
        unsafe {
            rec(
                0x1fb0
                    ^ fmt_len
                    ^ (desc & 0xfffff)
                    ^ (named_cnt << 24)
                    ^ ((id4 as u64) << 32)
                    ^ ((a0_ch as u64) << 40)
                    ^ r9,
            );
        }
        let out = rd::<usize>(ib_ptr + 0x08);
        for (i, &b) in b"FORMATTED_OK".iter().enumerate() {
            wr::<u8>(out + i, b);
        }
        wr::<u64>(ib_ptr + 0x10, 12);
    }
    unsafe extern "win64" fn mock_aa00(dst: usize, src: usize) {
        let len = (rd::<u32>(src + 0x04) as usize).min(30);
        unsafe { rec(0xaa00 ^ (len as u64)) };
        wr::<u32>(dst + 0x04, len as u32);
        for i in 0..=len {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(src + 0x08 + i));
        }
    }
    unsafe extern "win64" fn mock_5520(_dst: usize) {
        unsafe { rec(0x5520) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e0_8e80, mock_8e80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40f5_c3f0, mock_c3f0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40f5_c470, mock_c470 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4020_8500, mock_8500 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4240_a730, mock_a730 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_420e_af00, mock_af00 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_420e_a540, mock_a540 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401d_f680, mock_f680 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_423f_dfc0, mock_dfc0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4146_1fb0, mock_1fb0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_40f4_5520, mock_5520 as *const ())?;

    let g_82e8 = exe::ptr::<usize>(0x1_43a0_82e8) as usize;
    let saved_82e8 = rd::<usize>(g_82e8);
    let mut g_obj = [0u8; 0x20];
    wr::<u32>(g_obj.as_mut_ptr() as usize + 0x10, 77);
    wr::<usize>(g_82e8, g_obj.as_ptr() as usize);

    let run = |f: F, mode: u32| -> (u64, u64, u32, u8, [u8; 16]) {
        let mut alloc_node = vec![0u8; 0x200];
        let mut scratch = vec![0xeeu8; 0x40];
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
            ALLOC_NODE_PTR = alloc_node.as_mut_ptr() as usize;
            FMT_SCRATCH_PTR = scratch.as_mut_ptr() as usize;
        }

        let mut head_node = vec![0u8; 0x200];
        let mut child_node = vec![0u8; 0x200];
        let hp = head_node.as_mut_ptr() as usize;
        let cp = child_node.as_mut_ptr() as usize;
        wr::<u8>(hp + 0x19, 1);

        if mode == 0 {
            wr::<usize>(hp + 0x08, cp);
            wr::<usize>(cp, hp);
            wr::<usize>(cp + 0x10, hp);
            wr::<u8>(cp + 0x19, 0);
            wr::<u64>(cp + 0x20, 0x1111_2222);
            wr::<u64>(cp + 0x28, 0x3333_4444);
            unsafe {
                write_inline_rstr(cp + 0x30, b"HELLO_KEY");
                write_inline_rstr(cp + 0x78, b"A0");
                write_inline_rstr(cp + 0xc0, b"A1");
                write_inline_rstr(cp + 0x108, b"");
                write_inline_rstr(cp + 0x150, b"A3");
                write_inline_rstr(cp + 0x198, b"A4");
            }
            wr::<u32>(cp + 0x1e0, 77);
        } else {
            wr::<usize>(hp + 0x08, hp);
        }

        let mut p1_buf = [0u8; 0x50];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1 + 0x38, hp);

        let mut p3_entry = [0u8; 0x90];
        unsafe { write_inline_rstr(p3_entry.as_mut_ptr() as usize + 0x48, b"hello_key") };
        let mut p3_buf = [0u8; 0x90];
        let p3 = p3_buf.as_mut_ptr() as usize;
        wr::<usize>(p3 + 0x80, p3_entry.as_ptr() as usize);

        let mut p4_buf = [0u8; 0x30];
        let p4 = p4_buf.as_mut_ptr() as usize;
        if mode == 0 {
            wr::<u16>(p4, 1);
            wr::<usize>(p4 + 0x08, 1);
        } else {
            wr::<u16>(p4, 4);
            for (i, &b) in b"FMT_KEY\0".iter().enumerate() {
                wr::<u8>(p4 + 0x08 + i, b);
            }
        }

        let mut p5_buf = vec![0u8; 0xe0];
        let p5 = p5_buf.as_mut_ptr() as usize;
        wr::<u64>(p5 + 0x60, 0x1111_2222);
        wr::<u64>(p5 + 0x68, 0x3333_4444);
        wr::<u8>(p5 + 0xc8, if mode == 2 { 2 } else { 0 });

        let ret = f(p1, 0, p3, p4, p5, 0x9999);
        let out_slice = rd::<[u8; 16]>(p5 + 0x88);
        (
            ret,
            unsafe { TRACE_HASH },
            rd::<u32>(alloc_node.as_ptr() as usize + 0x1e0),
            rd::<u8>(p5 + 0xc8),
            out_slice,
        )
    };

    for mode in 0..3 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141e07fb0 mode={mode}");
    }

    wr::<usize>(g_82e8, saved_82e8);
    Ok(3)
}

pub unsafe fn diff_unknown_142364e00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;
    static mut TILE_STORAGE_PTR: usize = 0;
    static mut SEG_STORAGE_PTR: usize = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_6d50(vec_ptr: usize, min_cap: u64) {
        unsafe {
            rec(0x6d50 ^ min_cap);
            wr::<usize>(vec_ptr, TILE_STORAGE_PTR);
            wr::<u32>(vec_ptr + 0x0c, 16);
        }
    }
    unsafe extern "win64" fn mock_eb40(vec_ptr: usize, new_cap: u64) {
        unsafe {
            rec(0xeb40 ^ new_cap);
            wr::<usize>(vec_ptr, SEG_STORAGE_PTR);
            wr::<u32>(vec_ptr + 0x0c, 16);
        }
    }
    unsafe extern "win64" fn mock_7240(item: usize, flag: u32) {
        unsafe {
            rec(
                0x7240
                    ^ (flag as u64)
                    ^ (rd::<u32>(item + 0x10f0) as u64)
                    ^ ((rd::<u32>(item + 0x1100) as u64) << 16),
            );
        }
    }
    unsafe extern "win64" fn mock_v188_38(_this: usize) {
        unsafe { rec(0x18838) };
    }
    unsafe extern "win64" fn mock_va8_38(_this: usize) {
        unsafe { rec(0xa838) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_6d50, mock_6d50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_eb40, mock_eb40 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4236_7240, mock_7240 as *const ())?;

    let run = |f: F| -> (
        u64,
        [u32; 12],
        [u32; 18],
        [u32; 16],
        u32,
        [i32; 4],
        u32,
        [u64; 9],
        u32,
        [u32; 3],
    ) {
        let mut tile_storage = [0u64; 16];
        let mut seg_storage = [0u32; 16];
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
            TILE_STORAGE_PTR = tile_storage.as_mut_ptr() as usize;
            SEG_STORAGE_PTR = seg_storage.as_mut_ptr() as usize;
        }

        let mut b8_buf = [0u8; 0x20];
        wr::<f32>(b8_buf.as_mut_ptr() as usize + 0x14, 2.0);
        let mut a0_buf = [0u8; 0xc8];
        wr::<usize>(a0_buf.as_mut_ptr() as usize + 0xb8, b8_buf.as_ptr() as usize);

        let mut inner_rdx0 = vec![0u8; 0x110];
        let rdx0_p = inner_rdx0.as_mut_ptr() as usize;
        wr::<f32>(rdx0_p + 0x74, 3.0);
        wr::<usize>(rdx0_p + 0xa0, a0_buf.as_ptr() as usize);
        let rdx_mat = rdx0_p + 0xd0;
        wr::<f32>(rdx_mat, 128.0);
        wr::<f32>(rdx_mat + 0x04, 0.0);
        wr::<f32>(rdx_mat + 0x08, 0.0);
        wr::<f32>(rdx_mat + 0x0c, 0.0);
        wr::<f32>(rdx_mat + 0x10, 0.0);
        wr::<f32>(rdx_mat + 0x14, 128.0);
        wr::<f32>(rdx_mat + 0x18, 0.0);
        wr::<f32>(rdx_mat + 0x1c, -128.0);
        wr::<f32>(rdx_mat + 0x20, 0.0);
        wr::<f32>(rdx_mat + 0x24, 16.0);
        wr::<f32>(rdx_mat + 0x28, 160.0);
        wr::<f32>(rdx_mat + 0x2c, 8.0);

        let mut item0 = vec![0u8; 0x1120];
        let i0_p = item0.as_mut_ptr() as usize;
        wr::<usize>(i0_p, rdx0_p);
        wr::<u8>(i0_p + 0x10e4, 1);

        let items: [usize; 1] = [i0_p];

        let mut vtbl_188 = [0usize; 8];
        vtbl_188[7] = mock_v188_38 as *const () as usize;
        let mut vtbl_a8 = [0usize; 8];
        vtbl_a8[7] = mock_va8_38 as *const () as usize;

        let verts: [[f32; 2]; 3] = [[64.0, 64.0], [-100.0, -100.0], [80.0, 96.0]];
        let indices: [[u32; 3]; 3] = [[0, 1, 0], [1, 2, 0], [1, 1, 0]];

        let mut p48_buf = vec![0u8; 0x230];
        let p48_p = p48_buf.as_mut_ptr() as usize;
        wr::<u32>(p48_p + 0x18, 3);
        wr::<usize>(p48_p + 0xa8, vtbl_a8.as_ptr() as usize);
        wr::<usize>(p48_p + 0x140, indices.as_ptr() as usize);
        wr::<usize>(p48_p + 0x188, vtbl_188.as_ptr() as usize);
        wr::<usize>(p48_p + 0x220, verts.as_ptr() as usize);

        let mut p1_buf = vec![0u8; 0x200];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1, items.as_ptr() as usize);
        wr::<u32>(p1 + 0x08, 1);
        wr::<usize>(p1 + 0x48, p48_p);
        wr::<f32>(p1 + 0x1c0, 1.0);
        wr::<f32>(p1 + 0x1d0, 1.0);
        wr::<f32>(p1 + 0x1e0, 1.0);

        f(p1);

        let mut tiles_out = [0u64; 9];
        tiles_out.copy_from_slice(&tile_storage[..9]);
        let mut segs_out = [0u32; 3];
        segs_out.copy_from_slice(&seg_storage[..3]);

        (
            unsafe { TRACE_HASH },
            rd::<[u32; 12]>(p1 + 0x130),
            rd::<[u32; 18]>(i0_p + 0x104c),
            rd::<[u32; 16]>(i0_p + 0x10a0),
            rd::<u32>(i0_p + 0x10e0),
            rd::<[i32; 4]>(i0_p + 0x1108),
            rd::<u32>(i0_p + 0x10f0),
            tiles_out,
            rd::<u32>(i0_p + 0x1100),
            segs_out,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142364e00 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_41e0_7fb0, "coregame_141e07fb0", rend::cauldron_hot_u::coregame_141e07fb0 as *const (), Throws::Caught, rend::cauldron_hot_u::diff_coregame_141e07fb0),
// Replacement::verified_throws(0x1_4236_4e00, "unknown_142364e00", rend::cauldron_hot_u::unknown_142364e00 as *const (), Throws::Caught, rend::cauldron_hot_u::diff_unknown_142364e00),
