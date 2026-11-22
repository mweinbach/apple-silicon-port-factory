//! Decompiled `rend` hot functions shard ZA09 (`0x141e330a0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline]
unsafe fn slab_free_ptr(ptr: usize) {
    if ptr != 0 {
        let seg = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = ptr.wrapping_sub(seg) >> 16;
        let hdr_raw = seg.wrapping_add(0x108).wrapping_add(page_idx * 0x50);
        let hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if exe::teb() == rd::<usize>(seg + 0x100) {
            if rd::<u8>(hdr + 0x0e) == 0 {
                wr::<u64>(ptr, rd::<u64>(hdr + 0x20));
                let new_cnt = rd::<u32>(hdr + 0x18).wrapping_sub(1);
                wr::<u32>(hdr + 0x18, new_cnt);
                wr::<usize>(hdr + 0x20, ptr);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (hdr));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (seg, hdr, 1u8, ptr));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (seg, hdr, 0u8, ptr));
        }
    }
}

#[inline]
unsafe fn advance_iter_e0(iter_e0: usize) {
    exe::call_va!(0x1_401e_7c70, (iter_e0 + 0x08));
    let cur = rd::<usize>(iter_e0 + 0x10);
    if cur != rd::<usize>(iter_e0 + 0x18) {
        let tbl = rd::<usize>(iter_e0);
        let idx = rd::<u16>(cur) as usize;
        let base = rd::<u64>(tbl + 0x48 + idx * 8);
        wr::<u64>(iter_e0 + 0x20, base);
        let cnt = if base != 0 {
            rd::<u32>(tbl + 0x10048 + idx * 4) as u64
        } else {
            0
        };
        wr::<u64>(iter_e0 + 0x28, cnt);
        wr::<u64>(iter_e0 + 0x30, 0);
    }
    exe::call_va!(0x1_40f8_aee0, (iter_e0));
}

#[inline]
unsafe fn push_item_40(vec_hdr: usize, item_ptr: usize) {
    let len = rd::<u32>(vec_hdr + 0x08);
    let mut dst = rd::<usize>(vec_hdr).wrapping_add((len as usize) * 0x28);
    let cap = rd::<u32>(vec_hdr + 0x0c) & 0x3fff_ffff;
    if len == cap {
        exe::call_va!(
            0x1_40f9_0730,
            (vec_hdr, (&mut dst as *mut usize) as usize, item_ptr)
        );
    } else {
        wr::<u32>(vec_hdr + 0x08, len.wrapping_add(1));
        wr::<[u8; 0x28]>(dst, rd::<[u8; 0x28]>(item_ptr));
    }
}

#[inline]
unsafe fn push_item_48(vec_hdr: usize, item_ptr: usize) {
    let len = rd::<u32>(vec_hdr + 0x08);
    let mut dst = rd::<usize>(vec_hdr).wrapping_add((len as usize) * 0x30);
    let cap = rd::<u32>(vec_hdr + 0x0c) & 0x3fff_ffff;
    if len == cap {
        exe::call_va!(
            0x1_4030_63b0,
            (vec_hdr, (&mut dst as *mut usize) as usize, item_ptr)
        );
    } else {
        wr::<u32>(vec_hdr + 0x08, len.wrapping_add(1));
        wr::<[u8; 0x30]>(dst, rd::<[u8; 0x30]>(item_ptr));
    }
}

/// `unknown_141e330a0`
/// Original: 0x141e330a0 (7449 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e330a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let idx_de0 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0de0) as usize) as usize;
    let local_d8 = rd::<usize>(rd::<usize>(param_3 + 0x580e0).wrapping_add(idx_de0 * 8));

    // Phase 1: transfer 25 vectors from `local_d8 + 0x4b0` to `local_d8`
    let mut rbx_v = local_d8.wrapping_add(0x4b0);
    for _ in 0..0x19usize {
        let rdi_v = rbx_v.wrapping_sub(0x4b0);
        if rdi_v != rbx_v {
            wr::<u32>(rdi_v + 0x08, 0);
            let req = rd::<u32>(rbx_v + 0x08);
            exe::call_va!(0x1_402d_6540, (rdi_v, req));
            let eax = rd::<u32>(rbx_v + 0x08);
            wr::<u32>(rdi_v + 0x08, eax);
            let ecx = rd::<u32>(rdi_v + 0x0c);
            if (ecx & 0xc000_0000) == 0x4000_0000 {
                wr::<u64>(rdi_v, rd::<u64>(rbx_v));
            } else if eax != 0 {
                let byte_len = (eax as u64).wrapping_mul(0x30);
                exe::call_va!(
                    0x1_4183_acc8,
                    (rd::<usize>(rdi_v), rd::<usize>(rbx_v), byte_len)
                );
            }
        }
        wr::<u32>(rbx_v + 0x08, 0);
        rbx_v = rbx_v.wrapping_add(0x10);
    }

    let mut iter_70_buf = [0u64; 8];
    let iter_70 = iter_70_buf.as_mut_ptr() as usize;
    let mut buf_b0 = [0u64; 6];
    let buf_b0_ptr = buf_b0.as_mut_ptr() as usize;
    let mut iter_e0_buf = [0u64; 12];
    let iter_e0 = iter_e0_buf.as_mut_ptr() as usize;
    let mut span_180 = [0u64; 2];
    let span_180_ptr = span_180.as_mut_ptr() as usize;

    // Phase 2: clear `+0x10` flag on all elements in `param_2`
    let tbl_p2 = rd::<usize>(param_2).wrapping_add(0x40);
    wr::<[u64; 2]>(span_180_ptr, rd::<[u64; 2]>(param_2 + 0x08));
    exe::call_va!(0x1_401e_7c10, (buf_b0_ptr, tbl_p2, span_180_ptr));
    wr::<usize>(iter_70, tbl_p2);
    wr::<u64>(iter_70 + 0x08, rd::<u64>(buf_b0_ptr));
    wr::<u64>(iter_70 + 0x10, rd::<u64>(buf_b0_ptr + 0x08));
    wr::<u64>(iter_70 + 0x18, rd::<u64>(buf_b0_ptr + 0x10));
    wr::<u64>(iter_70 + 0x28, 0);
    wr::<u64>(iter_70 + 0x30, 0);
    let cur_p2 = rd::<usize>(iter_70 + 0x10);
    if cur_p2 != rd::<usize>(iter_70 + 0x18) {
        let idx = rd::<u16>(cur_p2) as usize;
        let base = rd::<u64>(tbl_p2 + 0x48 + idx * 8);
        wr::<u64>(iter_70 + 0x20, base);
        let cnt = if base != 0 {
            rd::<u32>(tbl_p2 + 0x10048 + idx * 4) as u64
        } else {
            0
        };
        wr::<u64>(iter_70 + 0x28, cnt);
    }
    exe::call_va!(0x1_40f8_b0c0, (iter_70));
    while rd::<u64>(iter_70 + 0x30) < rd::<u64>(iter_70 + 0x28) {
        let mut idx = rd::<u64>(iter_70 + 0x30);
        let cnt = rd::<u64>(iter_70 + 0x28);
        while idx < cnt {
            let c0 = rd::<usize>(iter_70 + 0x38).wrapping_add((idx as usize) * 0x1c);
            wr::<u8>(c0 + 0x10, 0);
            idx += 1;
            wr::<u64>(iter_70 + 0x30, idx);
        }
        exe::call_va!(0x1_401e_7c70, (iter_70 + 0x08));
        let next_cur = rd::<usize>(iter_70 + 0x10);
        if next_cur != rd::<usize>(iter_70 + 0x18) {
            let tbl = rd::<usize>(iter_70);
            let b_idx = rd::<u16>(next_cur) as usize;
            let base = rd::<u64>(tbl + 0x48 + b_idx * 8);
            wr::<u64>(iter_70 + 0x20, base);
            let b_cnt = if base != 0 {
                rd::<u32>(tbl + 0x10048 + b_idx * 4) as u64
            } else {
                0
            };
            wr::<u64>(iter_70 + 0x28, b_cnt);
            wr::<u64>(iter_70 + 0x30, 0);
        }
        exe::call_va!(0x1_40f8_b0c0, (iter_70));
    }

    // Phase 3: initialize `c0 + 0x14 = 0` and `c2 = -1` in `param_1`
    exe::call_va!(0x1_40f8_a8d0, (param_1, iter_e0));
    while rd::<u64>(iter_e0 + 0x30) < rd::<u64>(iter_e0 + 0x28) {
        let mut idx = rd::<u64>(iter_e0 + 0x30);
        let cnt = rd::<u64>(iter_e0 + 0x28);
        while idx < cnt {
            let c0 = rd::<usize>(iter_e0 + 0x38).wrapping_add((idx as usize) * 0x1c);
            let c2 = rd::<usize>(iter_e0 + 0x48).wrapping_add((idx as usize) * 4);
            wr::<u16>(c0 + 0x14, 0);
            wr::<u32>(c2, 0xffff_ffff);
            idx += 1;
            wr::<u64>(iter_e0 + 0x30, idx);
        }
        advance_iter_e0(iter_e0);
    }

    // Phase 4: compute depth levels and `max_depth` (`r12d`)
    let mut max_depth: i32 = -1;
    exe::call_va!(0x1_40f8_a8d0, (param_1, iter_e0));
    while rd::<u64>(iter_e0 + 0x30) < rd::<u64>(iter_e0 + 0x28) {
        let mut idx = rd::<u64>(iter_e0 + 0x30);
        let cnt = rd::<u64>(iter_e0 + 0x28);
        while idx < cnt {
            let c2 = rd::<usize>(iter_e0 + 0x48).wrapping_add((idx as usize) * 4);
            let c3 = rd::<usize>(iter_e0 + 0x50).wrapping_add((idx as usize) * 0x10);
            if rd::<i32>(c2) == -1 && rd::<u32>(c3 + 0x08) == 0 {
                wr::<u32>(c2, 0);
            } else {
                wr::<[u64; 3]>(buf_b0_ptr, rd::<[u64; 3]>(param_1));
                exe::call_va!(0x1_41e3_4de0, (buf_b0_ptr, c2, c3));
            }
            let val = rd::<i32>(c2);
            if max_depth < val {
                max_depth = val;
            }
            idx += 1;
            wr::<u64>(iter_e0 + 0x30, idx);
        }
        advance_iter_e0(iter_e0);
    }

    // Phase 5: propagate from `max_depth` down to `0`
    if max_depth >= 0 {
        let ctx_rbx = rd::<usize>(param_1);
        let tbl_r11 = ctx_rbx.wrapping_add(0x40);
        let mut cur_depth = max_depth;
        while cur_depth >= 0 {
            wr::<[u64; 2]>(span_180_ptr, rd::<[u64; 2]>(param_1 + 0x08));
            exe::call_va!(0x1_401e_7c10, (buf_b0_ptr, tbl_r11, span_180_ptr));
            wr::<usize>(iter_e0, tbl_r11);
            wr::<u64>(iter_e0 + 0x08, rd::<u64>(buf_b0_ptr));
            wr::<u64>(iter_e0 + 0x10, rd::<u64>(buf_b0_ptr + 0x08));
            wr::<u64>(iter_e0 + 0x18, rd::<u64>(buf_b0_ptr + 0x10));
            wr::<u64>(iter_e0 + 0x28, 0);
            wr::<u64>(iter_e0 + 0x30, 0);
            let cur_s = rd::<usize>(buf_b0_ptr + 0x08);
            if cur_s != rd::<usize>(buf_b0_ptr + 0x10) {
                let b_idx = rd::<u16>(cur_s) as usize;
                let base = rd::<u64>(tbl_r11 + 0x48 + b_idx * 8);
                wr::<u64>(iter_e0 + 0x20, base);
                let b_cnt = if base != 0 {
                    rd::<u32>(tbl_r11 + 0x10048 + b_idx * 4) as u64
                } else {
                    0
                };
                wr::<u64>(iter_e0 + 0x28, b_cnt);
            }
            exe::call_va!(0x1_40f8_aee0, (iter_e0));

            if rd::<u64>(iter_e0 + 0x30) < rd::<u64>(iter_e0 + 0x28) {
                let mut r15 = rd::<u64>(iter_e0 + 0x30) as usize;
                let mut rdi_cnt = rd::<u64>(iter_e0 + 0x28) as usize;
                let mut r14_cur = rd::<usize>(iter_e0 + 0x10);
                while r15 < rdi_cnt {
                    let c2_ptr = rd::<usize>(iter_e0 + 0x48).wrapping_add(r15 * 4);
                    if rd::<i32>(c2_ptr) == cur_depth {
                        let r12_c0 = rd::<usize>(iter_e0 + 0x38).wrapping_add(r15 * 0x1c);
                        let r13_c3 = rd::<usize>(iter_e0 + 0x50).wrapping_add(r15 * 0x10);

                        if rd::<u8>(r12_c0 + 0x09) != 0 {
                            let mut sub_rbx = rd::<usize>(r13_c3);
                            let sub_end =
                                sub_rbx.wrapping_add((rd::<u32>(r13_c3 + 0x08) as usize) << 5);
                            while sub_rbx != sub_end {
                                wr::<[u64; 3]>(buf_b0_ptr, rd::<[u64; 3]>(param_1));
                                let mut out_198: u64 = 0;
                                exe::call_va!(
                                    0x1_403d_17d0,
                                    (
                                        (&mut out_198 as *mut u64) as usize,
                                        rd::<u64>(sub_rbx + 0x18),
                                        buf_b0_ptr
                                    )
                                );
                                if (out_198 as u32) != 0xffff_ffff {
                                    let idx32 = (out_198 as u32) as usize;
                                    let packed =
                                        rd::<u64>(rd::<usize>(ctx_rbx + 0x30) + idx32 * 8);
                                    if (packed as u16) == 0xffff {
                                        let abort_fn =
                                            rd::<usize>(exe::ptr::<usize>(0x1_4264_67e8) as usize);
                                        exe::call_va!(abort_fn, ());
                                    }
                                    let arg_148 = packed & 0xffff_0000_0000_ffff;
                                    let target = exe::call_va!(
                                        0x1_40f8_6a50,
                                        (ctx_rbx, arg_148) -> usize
                                    );
                                    wr::<u8>(target + 0x09, 1);
                                }
                                sub_rbx = sub_rbx.wrapping_add(0x20);
                            }
                        }

                        let mut sub_rdi = rd::<usize>(r13_c3);
                        let sub_end =
                            sub_rdi.wrapping_add((rd::<u32>(r13_c3 + 0x08) as usize) << 5);
                        if sub_rdi != sub_end {
                            let rdx_c4 = rd::<usize>(iter_e0 + 0x58).wrapping_add(r15 * 0xc0);
                            while sub_rdi != sub_end {
                                let xmm0 = rd::<f32>(r12_c0);
                                let ecx = rd::<i32>(sub_rdi);
                                let xmm3 = ecx as f32;
                                let xmm1 = (rd::<i32>(sub_rdi + 0x04)) as f32;
                                let xmm2 = rd::<f32>(r12_c0 + 0x04);
                                let gt = xmm0 > xmm2;

                                let flag_278: bool;
                                let cond_b: bool;
                                if gt && rd::<u8>(r12_c0 + 0x0e) == 0 {
                                    flag_278 = (xmm0 > xmm3) && (xmm1 >= xmm0);
                                    cond_b = ((xmm1 >= xmm2) && (xmm0 > xmm1))
                                        || (rd::<u8>(r12_c0 + 0x08) != 0
                                            && ecx == rd::<i32>(rdx_c4 + 0x88));
                                } else {
                                    flag_278 = (xmm0 >= xmm3) && (xmm1 > xmm0);
                                    if gt {
                                        cond_b = ((xmm1 >= xmm2) && (xmm0 > xmm1))
                                            || (rd::<u8>(r12_c0 + 0x08) != 0
                                                && ecx == rd::<i32>(rdx_c4 + 0x88));
                                    } else {
                                        cond_b = (xmm3 > xmm0) && (xmm2 >= xmm3);
                                    }
                                }

                                if flag_278 || cond_b {
                                    let r10_ctx = rd::<usize>(param_1);
                                    let span_cnt = rd::<u64>(param_1 + 0x08);
                                    let span_ptr = rd::<usize>(param_1 + 0x10);
                                    let handle = rd::<u64>(sub_rdi + 0x18);
                                    let edx = handle as u32;
                                    let r9d = (handle >> 32) as u32;
                                    let r8_arr = rd::<usize>(r10_ctx + 0x18);
                                    let ecx_len = rd::<u32>(r10_ctx + 0x20);
                                    if (edx as u64) < (ecx_len as u64) {
                                        let entry = rd::<u64>(r8_arr + (edx as usize) * 8);
                                        if (entry as u32) == edx
                                            && ((entry >> 32) as u32) == r9d
                                            && edx != 0xffff_ffff
                                        {
                                            let r11_idx = edx as usize;
                                            let dx = rd::<u64>(
                                                rd::<usize>(r10_ctx + 0x30) + r11_idx * 8,
                                            ) as u16;
                                            if dx == 0xffff {
                                                let abort_fn = rd::<usize>(
                                                    exe::ptr::<usize>(0x1_4264_67e8) as usize,
                                                );
                                                exe::call_va!(abort_fn, ());
                                            }
                                            let mut lo = span_ptr;
                                            let end_s =
                                                span_ptr.wrapping_add((span_cnt as usize) * 2);
                                            let mut rem = (end_s.wrapping_sub(lo) as isize) >> 1;
                                            while rem > 0 {
                                                let half = (rem as usize) >> 1;
                                                let mid = lo.wrapping_add(half * 2);
                                                if rd::<u16>(mid) < dx {
                                                    lo = mid.wrapping_add(2);
                                                    rem -= (half as isize) + 1;
                                                } else {
                                                    rem = half as isize;
                                                }
                                            }
                                            if lo != end_s && dx >= rd::<u16>(lo) {
                                                let rbx_entry = rd::<u64>(
                                                    rd::<usize>(ctx_rbx + 0x30) + r11_idx * 8,
                                                );
                                                if (rbx_entry as u16) == 0xffff {
                                                    let abort_fn = rd::<usize>(exe::ptr::<usize>(
                                                        0x1_4264_67e8,
                                                    )
                                                        as usize);
                                                    exe::call_va!(abort_fn, ());
                                                }
                                                let arg_150 = rbx_entry & 0xffff_0000_0000_ffff;
                                                let r13_target = exe::call_va!(
                                                    0x1_40f8_6a50,
                                                    (rd::<usize>(param_1), arg_150) -> usize
                                                );
                                                let elem_idx_50 = (rbx_entry >> 32) as usize;
                                                let bucket_r14 = (rbx_entry as u16) as usize;
                                                let off_r11 = bucket_r14 << 5;
                                                let r15d_cnt =
                                                    rd::<u32>(tbl_r11 + off_r11 + 0x1806c);
                                                let rcx_ids =
                                                    rd::<usize>(tbl_r11 + off_r11 + 0x18058);
                                                let id_d50 = rd::<u16>(
                                                    exe::ptr::<u16>(0x1_43cc_8d50) as usize,
                                                )
                                                    as u32;
                                                let pos1 = exe::call_va!(
                                                    0x1_4248_9ee0,
                                                    (rcx_ids, r15d_cnt, id_d50) -> i32
                                                )
                                                    as isize;
                                                let rbx_offs =
                                                    rd::<usize>(tbl_r11 + off_r11 + 0x18060);
                                                let comp_a_off = rd::<u32>(
                                                    rbx_offs.wrapping_add((pos1 * 4) as usize),
                                                )
                                                    as usize;
                                                let comp_a_ptr =
                                                    rd::<usize>(tbl_r11 + 0x48 + bucket_r14 * 8)
                                                        .wrapping_add(comp_a_off);

                                                let id_0b0 = rd::<u16>(
                                                    exe::ptr::<u16>(0x1_43c8_d0b0) as usize,
                                                )
                                                    as u32;
                                                let pos2 = exe::call_va!(
                                                    0x1_4248_9ee0,
                                                    (rcx_ids, r15d_cnt, id_0b0) -> i32
                                                )
                                                    as isize;
                                                let comp_b_off = rd::<u32>(
                                                    rbx_offs.wrapping_add((pos2 * 4) as usize),
                                                )
                                                    as usize;
                                                let ptr_1a0 =
                                                    rd::<usize>(tbl_r11 + 0x48 + bucket_r14 * 8)
                                                        .wrapping_add(comp_b_off)
                                                        .wrapping_add(elem_idx_50 * 0xc0);

                                                let r8_obj = rd::<usize>(sub_rdi + 0x10);
                                                wr::<u64>(iter_70, 0);
                                                wr::<u32>(iter_70 + 0x0c, 0);
                                                let cnt1 = rd::<u32>(r8_obj + 0x68) as usize;
                                                let src1 = rd::<usize>(r8_obj + 0x60);
                                                let mut dst1: usize = 0;
                                                if (cnt1 as isize) > 0 {
                                                    dst1 = exe::call_va!(
                                                        0x1_4040_9f80,
                                                        (0usize, cnt1 as u64) -> usize
                                                    );
                                                    wr::<usize>(iter_70, dst1);
                                                    wr::<u32>(iter_70 + 0x0c, cnt1 as u32);
                                                }
                                                for k in 0..cnt1 {
                                                    wr::<[u8; 0x54]>(
                                                        dst1.wrapping_add(k * 0x54),
                                                        rd::<[u8; 0x54]>(
                                                            src1.wrapping_add(k * 0x54),
                                                        ),
                                                    );
                                                }
                                                wr::<u32>(iter_70 + 0x08, cnt1 as u32);
                                                wr::<u64>(
                                                    iter_70 + 0x10,
                                                    rd::<u64>(r8_obj + 0x70),
                                                );
                                                wr::<u64>(
                                                    iter_70 + 0x18,
                                                    rd::<u64>(r8_obj + 0x78),
                                                );
                                                wr::<u128>(iter_70 + 0x20, 0);

                                                let cnt2 = rd::<u32>(r8_obj + 0x88);
                                                let src2 = rd::<usize>(r8_obj + 0x80);
                                                let mut dst2: usize = 0;
                                                if cnt2 != 0 {
                                                    dst2 = exe::call_va!(
                                                        0x1_401f_fc30,
                                                        (dst1.wrapping_add(cnt1 * 0x54), cnt2)
                                                            -> usize
                                                    );
                                                    wr::<usize>(iter_70 + 0x20, dst2);
                                                    wr::<u32>(iter_70 + 0x2c, cnt2);
                                                }
                                                for k in 0..(cnt2 as usize) {
                                                    wr::<[u8; 0x20]>(
                                                        dst2.wrapping_add(k * 0x20),
                                                        rd::<[u8; 0x20]>(
                                                            src2.wrapping_add(k * 0x20),
                                                        ),
                                                    );
                                                }
                                                wr::<u32>(iter_70 + 0x28, cnt2);
                                                exe::call_va!(0x1_41c6_9d30, (iter_70));

                                                let xmm8 = (rd::<i32>(sub_rdi)) as f32;
                                                let xmm9 = (rd::<i32>(sub_rdi + 0x08)) as f32;
                                                let mut xmm6 = (rd::<f32>(r12_c0) - xmm8) + xmm9;
                                                let r9_cnt = rd::<u32>(iter_70 + 0x28) as u64;
                                                if r9_cnt != 0 && r9_cnt.wrapping_sub(1) != 0 {
                                                    let (_, x0) = crate::simd_ops::call_va8_xmm(
                                                        0x1_41c6_a020,
                                                        iter_70 as u64,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        [0, xmm6.to_bits() as u128, 0, 0],
                                                    );
                                                    xmm6 = f32::from_bits(x0 as u32);
                                                }
                                                let xmm11 = (rd::<i32>(ptr_1a0 + 0x88)) as f32;
                                                let max_6 =
                                                    if xmm6 > 0.0f32 { xmm6 } else { 0.0f32 };
                                                let xmm7 =
                                                    if max_6 < xmm11 { max_6 } else { xmm11 };

                                                let mut xmm3_v =
                                                    (rd::<f32>(r12_c0 + 0x04) - xmm8) + xmm9;
                                                if r9_cnt != 0 && r9_cnt.wrapping_sub(1) != 0 {
                                                    let (_, x0) = crate::simd_ops::call_va8_xmm(
                                                        0x1_41c6_a020,
                                                        iter_70 as u64,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                        [0, xmm3_v.to_bits() as u128, 0, 0],
                                                    );
                                                    xmm3_v = f32::from_bits(x0 as u32);
                                                }
                                                let max_3 =
                                                    if xmm3_v > 0.0f32 { xmm3_v } else { 0.0f32 };
                                                let xmm2_v =
                                                    if max_3 < xmm11 { max_3 } else { xmm11 };

                                                wr::<f32>(r13_target, xmm6);
                                                wr::<f32>(r13_target + 0x04, xmm3_v);
                                                wr::<u8>(
                                                    r13_target + 0x08,
                                                    u8::from(xmm6 != xmm7 && xmm3_v == xmm2_v),
                                                );
                                                wr::<u8>(
                                                    r13_target + 0x0a,
                                                    rd::<u8>(r12_c0 + 0x0a),
                                                );
                                                wr::<u8>(
                                                    r13_target + 0x0b,
                                                    rd::<u8>(r12_c0 + 0x0b),
                                                );
                                                wr::<u8>(
                                                    r13_target + 0x0c,
                                                    rd::<u8>(r12_c0 + 0x0c),
                                                );
                                                wr::<u8>(
                                                    r13_target + 0x0d,
                                                    rd::<u8>(r12_c0 + 0x0d),
                                                );
                                                let b_0e = u8::from(
                                                    rd::<u8>(r12_c0 + 0x0e) != 0
                                                        || rd::<usize>(
                                                            comp_a_ptr + elem_idx_50 * 8,
                                                        ) != r8_obj,
                                                );
                                                wr::<u8>(r13_target + 0x0e, b_0e);
                                                wr::<u8>(
                                                    r13_target + 0x0f,
                                                    rd::<u8>(r13_target + 0x0f)
                                                        | rd::<u8>(r12_c0 + 0x0f),
                                                );
                                                wr::<u8>(r13_target + 0x14, 1);
                                                let b_15 = u8::from(
                                                    !(rd::<u8>(r12_c0 + 0x15) == 0 && flag_278),
                                                );
                                                wr::<u8>(r13_target + 0x15, b_15);
                                                wr::<usize>(comp_a_ptr + elem_idx_50 * 8, r8_obj);

                                                slab_free_ptr(rd::<usize>(iter_70));
                                                wr::<u64>(iter_70, 0);
                                                wr::<u64>(iter_70 + 0x08, 0);
                                                slab_free_ptr(rd::<usize>(iter_70 + 0x20));
                                                wr::<u64>(iter_70 + 0x20, 0);
                                                wr::<u64>(iter_70 + 0x28, 0);
                                                exe::call_va!(0x1_401d_e830, (iter_70));
                                            }
                                        }
                                    }
                                }
                                sub_rdi = sub_rdi.wrapping_add(0x20);
                            }
                        }
                    }

                    r15 += 1;
                    wr::<u64>(iter_e0 + 0x30, r15 as u64);
                    if r15 >= rdi_cnt {
                        r14_cur = r14_cur.wrapping_add(2);
                        wr::<usize>(iter_e0 + 0x10, r14_cur);
                        let end_s = rd::<usize>(iter_e0 + 0x18);
                        if r14_cur != end_s {
                            let span_tbl = rd::<usize>(iter_e0 + 0x08);
                            while r14_cur != end_s {
                                let idx = rd::<u16>(r14_cur) as usize;
                                if rd::<u64>(span_tbl + 0x48 + idx * 8) != 0
                                    && rd::<u32>(span_tbl + 0x10048 + idx * 4) != 0
                                {
                                    break;
                                }
                                r14_cur = r14_cur.wrapping_add(2);
                            }
                            wr::<usize>(iter_e0 + 0x10, r14_cur);
                        }
                        let at_end = r14_cur == end_s;
                        let tbl_e0 = rd::<usize>(iter_e0);
                        if !at_end {
                            let idx = rd::<u16>(r14_cur) as usize;
                            let base = rd::<usize>(tbl_e0 + 0x48 + idx * 8);
                            wr::<usize>(iter_e0 + 0x20, base);
                            rdi_cnt = if base != 0 {
                                rd::<u32>(tbl_e0 + 0x10048 + idx * 4) as usize
                            } else {
                                0
                            };
                            wr::<u64>(iter_e0 + 0x28, rdi_cnt as u64);
                            r15 = 0;
                            wr::<u64>(iter_e0 + 0x30, 0);
                            exe::call_va!(
                                0x1_40f8_af70,
                                (buf_b0_ptr, tbl_e0, tbl_e0, idx as u32)
                            );
                            let off0 = rd::<usize>(buf_b0_ptr);
                            wr::<usize>(
                                iter_e0 + 0x38,
                                if off0 != 0 { base.wrapping_add(off0) } else { 0 },
                            );
                            let off2 = rd::<usize>(buf_b0_ptr + 0x10);
                            wr::<usize>(
                                iter_e0 + 0x48,
                                if off2 != 0 { base.wrapping_add(off2) } else { 0 },
                            );
                            let off3 = rd::<usize>(buf_b0_ptr + 0x18);
                            wr::<usize>(
                                iter_e0 + 0x50,
                                if off3 != 0 { base.wrapping_add(off3) } else { 0 },
                            );
                            let off4 = rd::<usize>(buf_b0_ptr + 0x20);
                            wr::<usize>(
                                iter_e0 + 0x58,
                                if off4 != 0 { base.wrapping_add(off4) } else { 0 },
                            );
                        }
                    }
                }
            }
            cur_depth -= 1;
        }
    }

    // Phase 6: collect active events into `vec_ptr` and `local_d8 + (type + 0x19) * 0x10`
    let idx_fb8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0fb8) as usize) as usize;
    let ptr_fb8 = rd::<usize>(rd::<usize>(param_3 + 0x580e0).wrapping_add(idx_fb8 * 8));
    let flag_fb8 = rd::<u8>(ptr_fb8);

    let mut vec_ptr: usize = 0;
    let mut vec_len: u32 = 0;
    let mut vec_cap: u32 = 0;

    exe::call_va!(0x1_40f8_a8d0, (param_1, iter_e0));
    while rd::<u64>(iter_e0 + 0x30) < rd::<u64>(iter_e0 + 0x28) {
        let mut r11 = rd::<u64>(iter_e0 + 0x30) as usize;
        let rdx_cnt = rd::<u64>(iter_e0 + 0x28) as usize;
        while r11 < rdx_cnt {
            let rsi_base = rd::<usize>(iter_e0 + 0x20);
            let rdi_c0 = rd::<usize>(iter_e0 + 0x38).wrapping_add(r11 * 0x1c);
            let r8_c4 = rd::<usize>(iter_e0 + 0x58).wrapping_add(r11 * 0xc0);
            let mut cl_flag = rd::<u8>(rdi_c0 + 0x0f);

            if (rd::<u8>(rdi_c0 + 0x0a) != 0 || rd::<u8>(rdi_c0 + 0x08) != 0)
                && rd::<u32>(r8_c4 + 0x60) != 0
            {
                let mut r9 = 0usize;
                while r9 != (rd::<u32>(r8_c4 + 0x60) as usize) {
                    let r10 = rd::<usize>(r8_c4 + 0x58).wrapping_add(r9 * 0x20);
                    if rd::<u8>(r10 + 0x18) == 0 || cl_flag == 0 {
                        let mut rbx = rd::<usize>(r10);
                        let end_1a8 =
                            rbx.wrapping_add((rd::<u32>(r10 + 0x08) as usize) * 0x18);
                        while rbx != end_1a8 {
                            let r15_type =
                                rd::<i32>(rd::<usize>(rbx + 0x10) + 0x48) as isize;
                            let dl = if rd::<u8>(rdi_c0 + 0x15) != 0 {
                                false
                            } else {
                                let xmm0 = rd::<f32>(rdi_c0);
                                let xmm1 = (rd::<i32>(rbx)) as f32;
                                let xmm2 = (rd::<i32>(rbx + 0x04)) as f32;
                                let f04 = rd::<f32>(rdi_c0 + 0x04);
                                if xmm0 > f04 && rd::<u8>(rdi_c0 + 0x0e) == 0 {
                                    (xmm0 > xmm1) && (xmm2 >= xmm0)
                                } else {
                                    (xmm0 >= xmm1) && (xmm2 > xmm0)
                                }
                            };

                            let check_al = if rd::<u8>(rdi_c0 + 0x0e) == 0 {
                                ((r15_type as u32).wrapping_sub(8) & 0xffff_ffef) == 0
                            } else {
                                matches!(r15_type, 4 | 5 | 6 | 7 | 8 | 10 | 0x18)
                            };
                            let al = if check_al {
                                let xmm0 = rd::<f32>(rdi_c0 + 0x04);
                                let xmm1 = rd::<f32>(rdi_c0);
                                let ecx = rd::<i32>(rbx);
                                let xmm2 = ecx as f32;
                                let xmm3 = (rd::<i32>(rbx + 0x04)) as f32;
                                if xmm1 > xmm0 {
                                    ((xmm3 >= xmm0) && (xmm1 > xmm3))
                                        || (rd::<u8>(rdi_c0 + 0x08) != 0
                                            && ecx == rd::<i32>(r8_c4 + 0x88))
                                } else {
                                    (xmm2 > xmm1) && (xmm0 >= xmm2)
                                }
                            } else {
                                false
                            };

                            if dl || al {
                                let xmm2 = (rd::<i32>(rbx)) as f32;
                                let xmm7 = rd::<f32>(rdi_c0 + 0x04) - xmm2;
                                let xmm6 = rd::<f32>(rdi_c0) - xmm2;
                                let val_158 =
                                    rd::<u64>(rd::<usize>(r8_c4 + 0x68) + r9 * 8);
                                let val_1a0 = rd::<u64>(rsi_base + 0x10 + r11 * 8);

                                if vec_len == vec_cap {
                                    let old_len = vec_len as u64;
                                    let need = old_len + 1;
                                    let old_cap = vec_cap as u64;
                                    let grow = (old_cap >> 1) + old_cap;
                                    let new_cap = if grow < need { need } else { grow };
                                    vec_cap = new_cap as u32;
                                    let new_ptr = exe::call_va!(
                                        0x1_4241_c0a0,
                                        (new_cap * 0x30, 8u32) -> usize
                                    );
                                    exe::call_va!(
                                        0x1_4183_acc8,
                                        (new_ptr, vec_ptr, old_len * 0x30)
                                    );
                                    if vec_ptr != 0 {
                                        exe::call_va!(0x1_4241_c190, (vec_ptr));
                                    }
                                    vec_ptr = new_ptr;
                                }

                                let dst =
                                    vec_ptr.wrapping_add((vec_len as usize) * 0x30);
                                wr::<u64>(dst, val_158);
                                wr::<u64>(dst + 0x08, val_1a0);
                                wr::<usize>(dst + 0x10, rbx);
                                wr::<usize>(dst + 0x18, r10);
                                wr::<i64>(dst + 0x20, r15_type as i64);
                                wr::<f32>(dst + 0x28, xmm7);
                                wr::<f32>(dst + 0x2c, xmm6);
                                vec_len += 1;

                                let r9_vec = local_d8
                                    .wrapping_add(((r15_type + 0x19) as usize) * 0x10);
                                wr::<u64>(
                                    buf_b0_ptr,
                                    rd::<u64>(rd::<usize>(r8_c4 + 0x68) + r9 * 8),
                                );
                                wr::<u64>(
                                    buf_b0_ptr + 0x08,
                                    rd::<u64>(rsi_base + 0x10 + r11 * 8),
                                );
                                wr::<usize>(buf_b0_ptr + 0x10, rbx);
                                wr::<usize>(buf_b0_ptr + 0x18, r10);
                                wr::<f32>(buf_b0_ptr + 0x20, xmm6);
                                wr::<u8>(buf_b0_ptr + 0x24, rd::<u8>(rdi_c0 + 0x0e));
                                push_item_40(r9_vec, buf_b0_ptr);

                                cl_flag |= u8::from(r15_type == 0x0c) & flag_fb8;
                            }
                            rbx = rbx.wrapping_add(0x18);
                        }
                    }
                    r9 += 1;
                }
            }

            wr::<u8>(
                rdi_c0 + 0x10,
                u8::from(cl_flag != 0 && rd::<u8>(rdi_c0 + 0x0f) == 0),
            );
            wr::<u8>(
                rdi_c0 + 0x0f,
                u8::from(cl_flag != 0 && rd::<u8>(rdi_c0 + 0x0a) != 0),
            );
            r11 += 1;
            wr::<u64>(iter_e0 + 0x30, r11 as u64);
        }
        advance_iter_e0(iter_e0);
    }

    // Phase 7: collect `+0x09` flagged items into `local_d8 + (type + 0x32) * 0x10`
    exe::call_va!(0x1_40f8_a8d0, (param_1, iter_e0));
    let mut rbx_idx = rd::<u64>(iter_e0 + 0x30) as usize;
    let mut r13_cnt = rd::<u64>(iter_e0 + 0x28) as usize;
    if rbx_idx < r13_cnt {
        let r8_end = rd::<usize>(iter_e0 + 0x18);
        let mut rax_c4 = rd::<usize>(iter_e0 + 0x58);
        let mut rdi_c0 = rd::<usize>(iter_e0 + 0x38);
        let mut rdx_base = rd::<usize>(iter_e0 + 0x20);
        let mut r12_cur = rd::<usize>(iter_e0 + 0x10);
        while rbx_idx < r13_cnt {
            let r14_c0 = rdi_c0.wrapping_add(rbx_idx * 0x1c);
            let rsi_c4 = rax_c4.wrapping_add(rbx_idx * 0xc0);
            if rd::<u8>(r14_c0 + 0x09) != 0 && rd::<u32>(rsi_c4 + 0x60) != 0 {
                let mut r15_i = 0usize;
                while r15_i != (rd::<u32>(rsi_c4 + 0x60) as usize) {
                    let rdi_sub = rd::<usize>(rsi_c4 + 0x58).wrapping_add(r15_i * 0x20);
                    let mut rbx_sub = rd::<usize>(rdi_sub);
                    let r13_sub_end =
                        rbx_sub.wrapping_add((rd::<u32>(rdi_sub + 0x08) as usize) * 0x18);
                    while rbx_sub != r13_sub_end {
                        let rax_inner = rd::<usize>(rbx_sub + 0x10);
                        let xmm1 = (rd::<i32>(rbx_sub)) as f32;
                        let xmm0 = rd::<f32>(r14_c0);
                        let xmm2 = xmm0 - xmm1;
                        let r9_type = rd::<i32>(rax_inner + 0x48) as isize;
                        let r9_vec =
                            local_d8.wrapping_add(((r9_type + 0x32) as usize) * 0x10);
                        wr::<u64>(
                            buf_b0_ptr,
                            rd::<u64>(rd::<usize>(rsi_c4 + 0x68) + r15_i * 8),
                        );
                        wr::<u64>(
                            buf_b0_ptr + 0x08,
                            rd::<u64>(rdx_base + 0x10 + rbx_idx * 8),
                        );
                        wr::<usize>(buf_b0_ptr + 0x10, rbx_sub);
                        wr::<usize>(buf_b0_ptr + 0x18, rdi_sub);
                        wr::<f32>(buf_b0_ptr + 0x20, xmm2);
                        push_item_40(r9_vec, buf_b0_ptr);
                        rbx_sub = rbx_sub.wrapping_add(0x18);
                    }
                    r15_i += 1;
                }
            }
            rbx_idx += 1;
            if rbx_idx >= r13_cnt {
                r12_cur = r12_cur.wrapping_add(2);
                if r12_cur != r8_end {
                    let span_tbl = rd::<usize>(iter_e0 + 0x08);
                    while r12_cur != r8_end {
                        let idx = rd::<u16>(r12_cur) as usize;
                        if rd::<u64>(span_tbl + 0x48 + idx * 8) != 0
                            && rd::<u32>(span_tbl + 0x10048 + idx * 4) != 0
                        {
                            break;
                        }
                        r12_cur = r12_cur.wrapping_add(2);
                    }
                }
                let at_end = r12_cur == r8_end;
                let tbl_e0 = rd::<usize>(iter_e0);
                if !at_end {
                    let idx = rd::<u16>(r12_cur) as usize;
                    rdx_base = rd::<usize>(tbl_e0 + 0x48 + idx * 8);
                    r13_cnt = if rdx_base != 0 {
                        rd::<u32>(tbl_e0 + 0x10048 + idx * 4) as usize
                    } else {
                        0
                    };
                    rbx_idx = 0;
                    exe::call_va!(0x1_40f8_af70, (buf_b0_ptr, 0usize, tbl_e0, idx as u32));
                    let off0 = rd::<usize>(buf_b0_ptr);
                    rdi_c0 = if off0 != 0 {
                        rdx_base.wrapping_add(off0)
                    } else {
                        0
                    };
                    let off4 = rd::<usize>(buf_b0_ptr + 0x20);
                    rax_c4 = if off4 != 0 {
                        rdx_base.wrapping_add(off4)
                    } else {
                        0
                    };
                }
            }
        }
    }

    // Phase 8: sort, diff against `param_4`, dispatch to `local_d8`, update `param_4`, free
    let mut start_ptr: usize = vec_ptr;
    let mut end_ptr: usize = vec_ptr.wrapping_add((vec_len as usize) * 0x30);
    let count = vec_len as u64;
    exe::call_va!(
        0x1_40f8_cf30,
        (
            (&mut start_ptr as *mut usize) as usize,
            (&mut end_ptr as *mut usize) as usize,
            count,
            (end_ptr & 0xff) as u32
        )
    );

    let mut vec_170: [u64; 2] = [0; 2];
    let mut vec_160: [u64; 2] = [0; 2];
    let mut dummy_1b8: u64 = 0;
    let mut dummy_1c0: u64 = 0;
    let p4_start = rd::<usize>(param_4);
    let p4_end = p4_start.wrapping_add((rd::<u32>(param_4 + 0x08) as usize) * 0x30);
    let vec_end = vec_ptr.wrapping_add((count as usize) * 0x30);

    exe::call_va!(
        0x1_40f8_b340,
        (
            (&mut dummy_1b8 as *mut u64) as usize,
            vec_ptr,
            vec_end,
            p4_start,
            p4_end,
            vec_160.as_mut_ptr() as usize
        )
    );
    exe::call_va!(
        0x1_40f8_b340,
        (
            (&mut dummy_1c0 as *mut u64) as usize,
            p4_start,
            p4_end,
            vec_ptr,
            vec_end,
            vec_170.as_mut_ptr() as usize
        )
    );

    let v170_ptr = vec_170[0] as usize;
    let v170_len = (vec_170[1] as u32) as usize;
    for i in 0..v170_len {
        let rbx = v170_ptr.wrapping_add(i * 0x30);
        let rcx_vec = local_d8.wrapping_add((rd::<u64>(rbx + 0x20) as usize) * 0x10);
        wr::<u64>(iter_70, rd::<u64>(rbx));
        wr::<u64>(iter_70 + 0x08, rd::<u64>(rbx + 0x08));
        wr::<u64>(iter_70 + 0x10, rd::<u64>(rbx + 0x10));
        wr::<u64>(iter_70 + 0x18, rd::<u64>(rbx + 0x18));
        wr::<u32>(iter_70 + 0x20, rd::<u32>(rbx + 0x28));
        wr::<u32>(iter_70 + 0x24, rd::<u32>(rbx + 0x2c));
        wr::<u16>(iter_70 + 0x28, 1);
        push_item_48(rcx_vec, iter_70);
    }

    let v160_ptr = vec_160[0] as usize;
    let v160_len = (vec_160[1] as u32) as usize;
    for i in 0..v160_len {
        let rbx = v160_ptr.wrapping_add(i * 0x30);
        let rcx_vec = local_d8.wrapping_add((rd::<u64>(rbx + 0x20) as usize) * 0x10);
        wr::<u64>(iter_70, rd::<u64>(rbx));
        wr::<u64>(iter_70 + 0x08, rd::<u64>(rbx + 0x08));
        wr::<u64>(iter_70 + 0x10, rd::<u64>(rbx + 0x10));
        wr::<u64>(iter_70 + 0x18, rd::<u64>(rbx + 0x18));
        wr::<u32>(iter_70 + 0x20, rd::<u32>(rbx + 0x28));
        wr::<u32>(iter_70 + 0x24, rd::<u32>(rbx + 0x2c));
        wr::<u16>(iter_70 + 0x28, 0);
        push_item_48(rcx_vec, iter_70);
    }

    let new_p4_len = vec_len as u64;
    let p4_cap = rd::<u32>(param_4 + 0x0c) as u64;
    if new_p4_len > p4_cap {
        wr::<u32>(param_4 + 0x08, 0);
        exe::call_va!(0x1_403f_ec40, (param_4, new_p4_len));
    }
    let p4_dst = rd::<usize>(param_4);
    for k in 0..(vec_len as usize) {
        wr::<[u8; 0x30]>(
            p4_dst.wrapping_add(k * 0x30),
            rd::<[u8; 0x30]>(vec_ptr.wrapping_add(k * 0x30)),
        );
    }
    wr::<u32>(param_4 + 0x08, vec_len);

    if vec_160[0] != 0 {
        exe::call_va!(0x1_4241_c190, (vec_160[0] as usize));
    }
    if vec_170[0] != 0 {
        exe::call_va!(0x1_4241_c190, (vec_170[0] as usize));
    }
    if vec_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec_ptr));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141e330a0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static CALL_LOG: AtomicU64 = AtomicU64::new(0);
    static ARENA_BASE: AtomicUsize = AtomicUsize::new(0);
    static ARENA_OFF: AtomicUsize = AtomicUsize::new(0);
    static BUCKET_BASE: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn mock_6540(_rdi: usize, req: u32) {
        CALL_LOG.fetch_add((req as u64).wrapping_add(0x6540), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_4de0(_q_ptr: usize, c2_ptr: usize) {
        wr::<i32>(c2_ptr, 0);
        CALL_LOG.fetch_add(0x4de0, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_17d0(out_ptr: usize, _handle: u64, _q_ptr: usize) {
        wr::<u64>(out_ptr, 0);
        CALL_LOG.fetch_add(0x17d0, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_6a50(_ctx: usize, packed: u64) -> usize {
        let elem_idx = (packed >> 32) as usize;
        BUCKET_BASE.load(Ordering::Relaxed) + 0x100 + elem_idx * 0x1c
    }
    unsafe extern "win64" fn mock_9d30(_iter_70: usize) {
        CALL_LOG.fetch_add(0x9d30, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_e830(_iter_70: usize) {
        CALL_LOG.fetch_add(0xe830, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_c0a0(byte_sz: u64, _align: u32) -> usize {
        let off = ARENA_OFF.fetch_add((byte_sz as usize + 0x3f) & !0x3f, Ordering::Relaxed);
        ARENA_BASE.load(Ordering::Relaxed) + off
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        CALL_LOG.fetch_add(0xc190, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_cf30(
        _start_ref: usize,
        _end_ref: usize,
        cnt: u64,
        _low: u32,
    ) {
        CALL_LOG.fetch_add(cnt.wrapping_add(0xcf30), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_b340(
        _dummy: usize,
        a_start: usize,
        a_end: usize,
        _b_start: usize,
        _b_end: usize,
        out_vec: usize,
    ) {
        if a_end > a_start {
            let off = ARENA_OFF.fetch_add(0x40, Ordering::Relaxed);
            let buf = ARENA_BASE.load(Ordering::Relaxed) + off;
            wr::<[u8; 0x30]>(buf, rd::<[u8; 0x30]>(a_start));
            wr::<usize>(out_vec, buf);
            wr::<u32>(out_vec + 0x08, 1);
            wr::<u32>(out_vec + 0x0c, 1);
        } else {
            wr::<usize>(out_vec, 0);
            wr::<u32>(out_vec + 0x08, 0);
            wr::<u32>(out_vec + 0x0c, 0);
        }
    }
    unsafe extern "win64" fn mock_ec40(p4: usize, new_len: u64) {
        wr::<u32>(p4 + 0x0c, new_len as u32);
        CALL_LOG.fetch_add(new_len.wrapping_add(0xec40), Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_402d_6540, mock_6540 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e3_4de0, mock_4de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_403d_17d0, mock_17d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40f8_6a50, mock_6a50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41c6_9d30, mock_9d30 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_40f8_cf30, mock_cf30 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_40f8_b340, mock_b340 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_403f_ec40, mock_ec40 as *const ())?;

    let comp_vas: [u64; 5] = [
        0x1_43c8_d0a0,
        0x1_43cc_8d50,
        0x1_43cc_8d48,
        0x1_43cc_8d4c,
        0x1_43c8_d0b0,
    ];
    let mut saved_comp_ids = [0u16; 5];
    for i in 0..5 {
        let p = exe::ptr::<u16>(comp_vas[i]) as usize;
        saved_comp_ids[i] = rd::<u16>(p);
        wr::<u16>(p, (i as u16 + 1) * 10);
    }
    let p_de0 = exe::ptr::<u16>(0x1_43c9_0de0) as usize;
    let p_fb8 = exe::ptr::<u16>(0x1_43c9_0fb8) as usize;
    let saved_de0 = rd::<u16>(p_de0);
    let saved_fb8 = rd::<u16>(p_fb8);
    wr::<u16>(p_de0, 0);
    wr::<u16>(p_fb8, 1);

    static mut ARENA_BUF: [u8; 0x2000] = [0; 0x2000];
    static mut BUCKET_BUF: [u8; 0x1000] = [0; 0x1000];
    static mut SUB_C3_BUF: [u8; 0x40] = [0; 0x40];
    static mut R8_OBJ_BUF: [u8; 0xa0] = [0; 0xa0];
    static mut SUB_C4_ARR: [u8; 0x40] = [0; 0x40];
    static mut SUB_C4_ITEM: [u8; 0x30] = [0; 0x30];
    static mut INNER_48_BUF: [u8; 0x60] = [0; 0x60];

    let run = |f: F, invert_range: bool| -> ([u8; 0x1c], Vec<u8>, u32, u64) {
        CALL_LOG.store(0, Ordering::Relaxed);
        ARENA_BUF = [0u8; 0x2000];
        BUCKET_BUF = [0u8; 0x1000];
        SUB_C3_BUF = [0u8; 0x40];
        R8_OBJ_BUF = [0u8; 0xa0];
        SUB_C4_ARR = [0u8; 0x40];
        SUB_C4_ITEM = [0u8; 0x30];
        INNER_48_BUF = [0u8; 0x60];

        let arena_p = core::ptr::addr_of_mut!(ARENA_BUF) as usize;
        let bucket_p = core::ptr::addr_of_mut!(BUCKET_BUF) as usize;
        let sub_c3_p = core::ptr::addr_of_mut!(SUB_C3_BUF) as usize;
        let r8_obj_p = core::ptr::addr_of_mut!(R8_OBJ_BUF) as usize;
        let sub_c4_arr_p = core::ptr::addr_of_mut!(SUB_C4_ARR) as usize;
        let sub_c4_item_p = core::ptr::addr_of_mut!(SUB_C4_ITEM) as usize;
        let inner_48_p = core::ptr::addr_of_mut!(INNER_48_BUF) as usize;

        ARENA_BASE.store(arena_p, Ordering::Relaxed);
        ARENA_OFF.store(0, Ordering::Relaxed);
        BUCKET_BASE.store(bucket_p, Ordering::Relaxed);

        let mut ctx_buf = vec![0u8; 0x40 + 0x18100];
        let ctx_p = ctx_buf.as_mut_ptr() as usize;
        let tbl_p = ctx_p + 0x40;

        let comp_ids: [u16; 5] = [10, 20, 30, 40, 50];
        let comp_offs: [u32; 5] = [0x100, 0x200, 0x300, 0x400, 0x500];
        wr::<usize>(tbl_p + 0x48, bucket_p);
        wr::<u32>(tbl_p + 0x10048, 1);
        wr::<usize>(tbl_p + 0x18058, comp_ids.as_ptr() as usize);
        wr::<usize>(tbl_p + 0x18060, comp_offs.as_ptr() as usize);
        wr::<u32>(tbl_p + 0x1806c, 5);

        let ent_arr: [u64; 1] = [0];
        let map_arr: [u64; 1] = [0];
        wr::<usize>(ctx_p + 0x18, ent_arr.as_ptr() as usize);
        wr::<u32>(ctx_p + 0x20, 1);
        wr::<usize>(ctx_p + 0x30, map_arr.as_ptr() as usize);

        // Entity handle at bucket_p + 0x10
        wr::<u64>(bucket_p + 0x10, 0x1122_3344);

        // Component 0 (0x1c bytes at bucket_p + 0x100)
        let c0_p = bucket_p + 0x100;
        if invert_range {
            wr::<f32>(c0_p, 8.0);
            wr::<f32>(c0_p + 0x04, 2.0);
        } else {
            wr::<f32>(c0_p, 2.0);
            wr::<f32>(c0_p + 0x04, 8.0);
        }
        wr::<u8>(c0_p + 0x08, 1);
        wr::<u8>(c0_p + 0x09, 1);
        wr::<u8>(c0_p + 0x0a, 1);
        wr::<u8>(c0_p + 0x0e, 0);
        wr::<u8>(c0_p + 0x0f, 0);
        wr::<u8>(c0_p + 0x10, 1);
        wr::<u8>(c0_p + 0x15, 0);

        // Component 3 (0x10 bytes at bucket_p + 0x400)
        let c3_p = bucket_p + 0x400;
        wr::<usize>(c3_p, sub_c3_p);
        wr::<u32>(c3_p + 0x08, 1);
        wr::<i32>(sub_c3_p, 1);
        wr::<i32>(sub_c3_p + 0x04, 10);
        wr::<i32>(sub_c3_p + 0x08, 0);
        wr::<usize>(sub_c3_p + 0x10, r8_obj_p);
        wr::<u64>(sub_c3_p + 0x18, 0);
        wr::<f32>(r8_obj_p + 0x70, 0.0);
        wr::<f32>(r8_obj_p + 0x78, 20.0);

        // Component 4 (0xc0 bytes at bucket_p + 0x500)
        let c4_p = bucket_p + 0x500;
        let ptr_68: [u64; 1] = [0x5566_7788];
        wr::<usize>(c4_p + 0x58, sub_c4_arr_p);
        wr::<u32>(c4_p + 0x60, 1);
        wr::<usize>(c4_p + 0x68, ptr_68.as_ptr() as usize);
        wr::<i32>(c4_p + 0x88, 20);

        wr::<usize>(sub_c4_arr_p, sub_c4_item_p);
        wr::<u32>(sub_c4_arr_p + 0x08, 1);
        wr::<i32>(sub_c4_item_p, 1);
        wr::<i32>(sub_c4_item_p + 0x04, 10);
        wr::<usize>(sub_c4_item_p + 0x10, inner_48_p);
        wr::<i32>(inner_48_p + 0x48, 8);

        // Set up `local_d8` with 128 vectors (each with capacity 4 in `d8_storage`)
        let mut local_d8_buf = vec![0u8; 0x800];
        let mut d8_storage = vec![0u8; 128 * 4 * 0x30];
        let d8_p = local_d8_buf.as_mut_ptr() as usize;
        let st_p = d8_storage.as_mut_ptr() as usize;
        for i in 0..128usize {
            let vh = d8_p + i * 0x10;
            wr::<usize>(vh, st_p + i * 4 * 0x30);
            wr::<u32>(vh + 0x08, 0);
            wr::<u32>(vh + 0x0c, 4);
        }

        let flag_fb8_val: u8 = 1;
        let tbl_580e0: [usize; 2] = [d8_p, (&flag_fb8_val as *const u8) as usize];
        let mut p3_buf = vec![0u8; 0x580f0];
        let p3_p = p3_buf.as_mut_ptr() as usize;
        wr::<usize>(p3_p + 0x580e0, tbl_580e0.as_ptr() as usize);

        let span_indices: [u16; 1] = [0];
        let query: [u64; 3] = [ctx_p as u64, 1, span_indices.as_ptr() as u64];
        let q_p = query.as_ptr() as usize;

        let mut p4_backing = vec![0u8; 4 * 0x30];
        let mut p4_hdr: [u64; 2] = [p4_backing.as_mut_ptr() as u64, 4u64 << 32];
        let p4_p = p4_hdr.as_mut_ptr() as usize;

        f(q_p, q_p, p3_p, p4_p);

        let c0_bytes = rd::<[u8; 0x1c]>(bucket_p + 0x100);
        (
            c0_bytes,
            p4_backing,
            rd::<u32>(p4_p + 0x08),
            CALL_LOG.load(Ordering::Relaxed),
        )
    };

    let res = (|| -> Result<u32, String> {
        for invert in [false, true] {
            let o = run(f_o, invert);
            let n = run(f_n, invert);
            crate::diff_eq!(o, n, "141e330a0 invert={}", invert);
        }
        Ok(2)
    })();

    for i in 0..5 {
        let p = exe::ptr::<u16>(comp_vas[i]) as usize;
        wr::<u16>(p, saved_comp_ids[i]);
    }
    wr::<u16>(p_de0, saved_de0);
    wr::<u16>(p_fb8, saved_fb8);
    res
}

// =============================================================================
// Registry lines (for reference)
// =============================================================================
// Replacement::verified_throws(0x1_41e3_30a0, "unknown_141e330a0", rend::cauldron_hot_za09::unknown_141e330a0 as *const (), Throws::Caught, rend::cauldron_hot_za09::diff_unknown_141e330a0),
