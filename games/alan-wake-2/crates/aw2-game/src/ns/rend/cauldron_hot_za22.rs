//! Decompiled `rend` hot functions shard ZA22 (`0x142334b20..0x1424858a0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn atomic_and_u64(addr: usize, mask: u64) {
    let mut cur = rd::<u64>(addr);
    loop {
        let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u64(addr as u64, cur, cur & mask);
        if ok { break; }
        cur = prev;
    }
}

#[inline(always)]
unsafe fn atomic_or_u64(addr: usize, mask: u64) {
    let mut cur = rd::<u64>(addr);
    loop {
        let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u64(addr as u64, cur, cur | mask);
        if ok { break; }
        cur = prev;
    }
}

#[inline(always)]
unsafe fn atomic_or_u8(addr: usize, mask: u8) {
    let mut cur = rd::<u8>(addr);
    loop {
        let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u8(addr as u64, cur, cur | mask);
        if ok { break; }
        cur = prev;
    }
}

/// `rend_142334b20`
/// Original: 0x142334b20 (520 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142334b20(param_1: usize, param_2: usize) {
    let r15 = rd::<u16>(param_2 + 0x20) as u64;
    let rdi = r15.wrapping_add(0x80) & !0x7fu64;

    let ebp1 = rd::<u32>(param_1 + 0x08) as u64;
    let mut rcx_p1 = rd::<usize>(param_1).wrapping_add((ebp1 as usize) * 8);
    if ebp1 < rdi {
        let cap1 = rd::<u32>(param_1 + 0x0c) as u64;
        if rdi > cap1 {
            let mut new_cap = (cap1 >> 1).wrapping_add(cap1);
            if new_cap < rdi {
                new_cap = rdi;
            }
            exe::call_va!(0x1_4087_46b0, (param_1, new_cap));
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            rcx_p1 = rd::<usize>(param_1).wrapping_add(cur_len * 8);
        }
        let diff1 = rdi.wrapping_sub(ebp1);
        if diff1 != 0 {
            exe::call_va!(0x1_4183_acc2, (rcx_p1, 0u32, diff1.wrapping_mul(8)));
        }
        wr::<u32>(param_1 + 0x08, rdi as u32);
    }

    let len2 = rd::<u32>(param_1 + 0x18) as u64;
    let r9_p2 = rd::<usize>(param_1 + 0x10).wrapping_add((len2 as usize) * 0x38);
    let rbp2 = len2;
    if rbp2 < rdi {
        let cap2 = rd::<u32>(param_1 + 0x1c) as u64;
        let dst2 = if rdi > cap2 {
            let mut new_cap = (cap2 >> 1).wrapping_add(cap2);
            if new_cap < rdi {
                new_cap = rdi;
            }
            exe::call_va!(0x1_411d_b640, (param_1 + 0x10, new_cap));
            let cur_len = rd::<u32>(param_1 + 0x18) as usize;
            rd::<usize>(param_1 + 0x10).wrapping_add(cur_len * 0x38)
        } else {
            r9_p2
        };
        let diff2 = rdi.wrapping_sub(rbp2);
        if diff2 != 0 {
            exe::call_va!(0x1_4183_acc2, (dst2, 0u32, diff2.wrapping_mul(0x38)));
        }
        wr::<u32>(param_1 + 0x18, rdi as u32);
    }

    let rbp3 = rd::<u32>(param_1 + 0x40) as u64;
    let mut ptr3 = rd::<usize>(param_1 + 0x38).wrapping_add((rbp3 as usize) << 7);
    if rbp3 < rdi {
        let cap3 = rd::<u32>(param_1 + 0x44) as u64;
        if rdi > cap3 {
            let mut new_cap = (cap3 >> 1).wrapping_add(cap3);
            if new_cap < rdi {
                new_cap = rdi;
            }
            exe::call_va!(0x1_4133_fed0, (param_1 + 0x38, new_cap));
            let cur_len = rd::<u32>(param_1 + 0x40) as usize;
            ptr3 = rd::<usize>(param_1 + 0x38).wrapping_add(cur_len << 7);
        }
        let diff3 = rdi.wrapping_sub(rbp3);
        for _ in 0..diff3 {
            wr::<[u8; 0x80]>(ptr3, [0u8; 0x80]);
            ptr3 = ptr3.wrapping_add(0x80);
        }
        wr::<u32>(param_1 + 0x40, rdi as u32);
    }

    let word_idx = (r15 >> 6) as usize;
    let bit_mask = 1u64 << (r15 & 0x3f);
    atomic_and_u64(param_1 + 0x78 + word_idx * 8, !bit_mask);
    atomic_or_u64(param_1 + 0x2078 + word_idx * 8, bit_mask);
}

#[inline(always)]
fn hash_material_flags(r8: u64) -> u64 {
    let r9: u64 = 0x9ae1_6a3b_2f90_404f;
    let r10: u64 = 0x9ae1_6a3b_2f90_405f;
    let mut rdx = r8.wrapping_add(r9);
    let rax = r8.rotate_right(0x25).wrapping_mul(r10).wrapping_add(rdx);
    rdx = rdx.rotate_right(0x19).wrapping_add(r8).wrapping_mul(r10);
    let mut rcx = (rdx ^ rax).wrapping_mul(r10);
    rcx = ((rcx ^ (rcx >> 0x2f)) ^ rdx).wrapping_mul(r10);
    (rcx ^ (rcx >> 0x2f)).wrapping_mul(r10)
}

/// `rend_1423387b0`
/// Original: 0x1423387b0 (1498 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_1423387b0(param_1: usize, param_2: usize) -> usize {
    wr::<usize>(param_1, param_2);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    wr::<u64>(param_1 + 0x18, 0);
    wr::<u16>(param_1 + 0x20, 0);
    wr::<u128>(param_1 + 0x28, 0);
    wr::<u8>(param_1 + 0x38, 0);

    let r8_p0 = rd::<usize>(param_1);
    if rd::<u32>(r8_p0 + 0x54) != 0 {
        let str_ptr = exe::call_va!(0x1_401d_f680, (r8_p0 + 0x50) -> u64);
        let str_len = rd::<u32>(r8_p0 + 0x54) as u64;
        let mut view: [u64; 2] = [str_ptr, str_len];
        let g_210 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_0210) as usize);
        let h_18 = exe::call_va!(0x1_4239_70b0, (g_210, view.as_mut_ptr() as usize) -> u64);
        wr::<u64>(param_1 + 0x18, h_18);
    }

    let mut local_a0: usize = 0;
    let rax_bc0 = exe::call_va!(
        0x1_4134_0bc0,
        ((&mut local_a0 as *mut usize) as usize, param_2, param_1) -> usize
    );
    let mut rdx_p1 = rd::<usize>(rax_bc0);
    wr::<usize>(rax_bc0, 0);
    let old_p1 = rd::<usize>(param_1 + 0x08);
    wr::<usize>(param_1 + 0x08, rdx_p1);
    if old_p1 != 0 {
        let vfn = rd::<u64>(rd::<usize>(old_p1));
        exe::call_va!(vfn, (old_p1, 0u32));
        exe::call_va!(0x1_4244_2de0, (old_p1));
        rdx_p1 = rd::<usize>(param_1 + 0x08);
    }
    if local_a0 != 0 {
        let vfn = rd::<u64>(rd::<usize>(local_a0));
        exe::call_va!(vfn, (local_a0, 0u32));
        exe::call_va!(0x1_4244_2de0, (local_a0));
        rdx_p1 = rd::<usize>(param_1 + 0x08);
    }

    let req1 = rd::<u32>(param_2 + 0x28) as u64;
    let cap1 = rd::<u32>(rdx_p1 + 0x1c) as u64;
    if req1 > cap1 {
        let mut new_cap = (cap1 >> 1).wrapping_add(cap1);
        if new_cap < req1 {
            new_cap = req1;
        }
        exe::call_va!(0x1_411c_8030, (rdx_p1 + 0x10, new_cap));
    }

    let mut rbx_it = rd::<usize>(param_2 + 0x20);
    let rsi_end = rbx_it.wrapping_add((rd::<u32>(param_2 + 0x28) as usize) * 8);
    while rbx_it != rsi_end {
        let rdi = rd::<usize>(param_1 + 0x08);
        let mut rax_cur = rd::<usize>(rdi + 0x10);
        let r8_end = rax_cur.wrapping_add((rd::<u32>(rdi + 0x18) as usize) * 8);
        let mut exists = false;
        if rax_cur != r8_end {
            let r9_val = rd::<u64>(rbx_it);
            while rax_cur != r8_end {
                if rd::<u64>(rax_cur) == r9_val {
                    exists = true;
                    break;
                }
                rax_cur = rax_cur.wrapping_add(8);
            }
        }
        if !exists {
            let pushed = exe::call_va!(0x1_411c_2820, (rdi + 0x10, rbx_it) -> usize);
            exe::call_va!(0x1_4242_82e0, (rd::<usize>(pushed), rdi));
        }
        rbx_it = rbx_it.wrapping_add(8);
    }

    let mut local_a8: usize = 0;
    let rax_c60 = exe::call_va!(
        0x1_4134_0c60,
        ((&mut local_a8 as *mut usize) as usize, 0usize, param_1) -> usize
    );
    let mut rdx_p2 = rd::<usize>(rax_c60);
    wr::<usize>(rax_c60, 0);
    let old_p2 = rd::<usize>(param_1 + 0x10);
    wr::<usize>(param_1 + 0x10, rdx_p2);
    if old_p2 != 0 {
        let vfn = rd::<u64>(rd::<usize>(old_p2));
        exe::call_va!(vfn, (old_p2, 0u32));
        exe::call_va!(0x1_4244_2de0, (old_p2));
        rdx_p2 = rd::<usize>(param_1 + 0x10);
    }
    if local_a8 != 0 {
        let vfn = rd::<u64>(rd::<usize>(local_a8));
        exe::call_va!(vfn, (local_a8, 0u32));
        exe::call_va!(0x1_4244_2de0, (local_a8));
        rdx_p2 = rd::<usize>(param_1 + 0x10);
    }

    let req2 = rd::<u32>(param_2 + 0x38) as u64;
    let cap2 = rd::<u32>(rdx_p2 + 0x1c) as u64;
    if req2 > cap2 {
        let mut new_cap = (cap2 >> 1).wrapping_add(cap2);
        if new_cap < req2 {
            new_cap = req2;
        }
        exe::call_va!(0x1_412d_f620, (rdx_p2 + 0x10, new_cap));
    }

    let mut rdi_it = rd::<usize>(param_2 + 0x30);
    let r15_end = rdi_it.wrapping_add((rd::<u32>(param_2 + 0x38) as usize) * 8);
    let cb_fn = exe::ptr::<u8>(0x1_4134_0a20) as usize as u64;
    while rdi_it != r15_end {
        let rbp = rd::<usize>(param_1 + 0x10);
        let mut rax_cur = rd::<usize>(rbp + 0x10);
        let rdx_end = rax_cur.wrapping_add((rd::<u32>(rbp + 0x18) as usize) << 4);
        let mut exists = false;
        if rax_cur != rdx_end {
            let rcx_val = rd::<u64>(rdi_it);
            while rax_cur != rdx_end {
                if rd::<u64>(rax_cur) == rcx_val {
                    exists = true;
                    break;
                }
                rax_cur = rax_cur.wrapping_add(0x10);
            }
        }
        if !exists {
            let rbx_item = rd::<usize>(rdi_it);
            if rbx_item != 0 {
                let prev = crate::simd_ops::atomic_xadd_u32((rbx_item + 0x24) as u64, 1);
                if prev.wrapping_add(1) == 1 {
                    let rcx_28 = rd::<usize>(rbx_item + 0x28);
                    if rcx_28 != 0 {
                        exe::call_va!(0x1_4241_d6f0, (rcx_28, rbx_item));
                    }
                }
            }
            let len_18 = rd::<u32>(rbp + 0x18) as u64;
            let cap_1c = rd::<u32>(rbp + 0x1c) as u64;
            if (len_18 << 4) == (cap_1c << 4) {
                let min_cap = len_18.wrapping_add(1);
                let mut new_cap = (cap_1c >> 1).wrapping_add(cap_1c);
                if new_cap < min_cap {
                    new_cap = min_cap;
                }
                exe::call_va!(0x1_412d_f620, (rbp + 0x10, new_cap));
            }
            let cur_len = rd::<u32>(rbp + 0x18) as usize;
            let slot_ptr = rd::<usize>(rbp + 0x10).wrapping_add(cur_len << 4);
            wr::<usize>(slot_ptr, rbx_item);
            wr::<u32>(slot_ptr + 0x08, 0xffff_ffff);
            wr::<u32>(rbp + 0x18, (cur_len as u32).wrapping_add(1));
            let new_end = rd::<usize>(rbp + 0x10).wrapping_add((rd::<u32>(rbp + 0x18) as usize) << 4);
            if rd::<i32>(new_end - 0x08) < 0 {
                let mut local_30: [u64; 2] = [rbp as u64, cb_fn];
                let idx_val = exe::call_va!(
                    0x1_4220_9780,
                    (rd::<usize>(new_end - 0x10), local_30.as_mut_ptr() as usize) -> u32
                );
                wr::<u32>(new_end - 0x08, idx_val);
            }
            exe::call_va!(0x1_4242_82e0, (rd::<usize>(new_end - 0x10), rbp));
        }
        rdi_it = rdi_it.wrapping_add(8);
    }

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let lock_addr = exe::ptr::<u8>(0x1_4386_c790) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4386_c798) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4386_c79c) as usize;
    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_addr));
        wr::<u32>(owner_addr, tid);
    }
    wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));

    let vec_base_addr = exe::ptr::<usize>(0x1_43bb_d988) as usize;
    let vec_len_addr = exe::ptr::<u32>(0x1_43bb_d990) as usize;
    let vec_cap_addr = exe::ptr::<u32>(0x1_43bb_d994) as usize;
    let init_flag_addr = exe::ptr::<u8>(0x1_43c8_cb00) as usize;
    let mut edi_len = rd::<u32>(vec_len_addr);
    if rd::<u8>(init_flag_addr) == 0 {
        for ebp_i in 0..0x10000u32 {
            let r15w = !(ebp_i as u16);
            let mut rsi_base = rd::<usize>(vec_base_addr);
            let rdx_end = rsi_base.wrapping_add((edi_len as usize) * 2);
            let rcx_cap_end = rsi_base.wrapping_add((rd::<u32>(vec_cap_addr) as usize) * 2);
            if rdx_end == rcx_cap_end {
                let cur_len = ((rdx_end.wrapping_sub(rsi_base) as isize) >> 1) as u64;
                let cur_cap = ((rcx_cap_end.wrapping_sub(rsi_base) as isize) >> 1) as u64;
                let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                let min_cap = cur_len.wrapping_add(1);
                if new_cap < min_cap {
                    new_cap = min_cap;
                }
                let new_buf = exe::call_va!(0x1_4028_e1c0, (cur_cap, new_cap) -> usize);
                let copy_bytes = (rd::<u32>(vec_len_addr) as u64) * 2;
                exe::call_va!(0x1_4183_acc8, (new_buf, rd::<usize>(vec_base_addr), copy_bytes));
                exe::call_va!(0x1_401d_e830, (vec_base_addr));
                wr::<usize>(vec_base_addr, new_buf);
                wr::<u32>(vec_len_addr, cur_len as u32);
                wr::<u32>(vec_cap_addr, new_cap as u32);
                rsi_base = new_buf;
                edi_len = cur_len as u32;
            }
            wr::<u16>(rsi_base.wrapping_add((edi_len as usize) * 2), r15w);
            edi_len = rd::<u32>(vec_len_addr).wrapping_add(1);
            wr::<u32>(vec_len_addr, edi_len);
        }
        wr::<u8>(init_flag_addr, 1);
    }

    let vec_ptr = rd::<usize>(vec_base_addr);
    let bx_id = rd::<u16>(vec_ptr.wrapping_add((edi_len as usize) * 2).wrapping_sub(2));
    wr::<u32>(vec_len_addr, edi_len.wrapping_sub(1));
    let new_depth = rd::<u32>(depth_addr).wrapping_sub(1);
    wr::<u32>(depth_addr, new_depth);
    if new_depth == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32(owner_addr as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_addr));
    }

    wr::<u16>(param_1 + 0x20, bx_id);
    let tbl_0e0 = exe::ptr::<usize>(0x1_4398_00e0) as usize;
    wr::<usize>(tbl_0e0.wrapping_add((bx_id as usize) * 8), param_1);

    let g_c98_addr = exe::ptr::<usize>(0x1_43a3_6c98) as usize;
    let r9_mgr = rd::<usize>(g_c98_addr);
    if r9_mgr != 0 {
        exe::call_va!(0x1_4233_4b20, (r9_mgr, param_1));
        let r9_after = rd::<usize>(g_c98_addr);
        if r9_after != 0 {
            let r8d = rd::<u16>(param_1 + 0x20) as u32;
            atomic_or_u8(param_1 + 0x38, 3);
            let bit = 1u64 << (r8d & 0x3f);
            let word_idx = (r8d >> 6) as usize;
            atomic_or_u64(r9_after + 0x78 + word_idx * 8, bit);
        }
    }

    let rax_0 = rd::<usize>(param_1);
    let b_81 = rd::<u8>(rax_0 + 0x81);
    let b_80 = rd::<u8>(rax_0 + 0x80);
    let edi_bit = (rd::<u32>(rax_0 + 0x7c) >> 31) as u64;
    let rbx_0 = rd::<usize>(param_2);
    let str_emission = exe::ptr::<u8>(0x1_433f_48f8) as usize;
    let p_em = exe::call_va!(0x1_411a_5000, (rbx_0, str_emission) -> usize);
    let sil = if p_em == 0 { 0u8 } else { rd::<u8>(p_em + 0x08) };
    let str_retro = exe::ptr::<u8>(0x1_433f_d900) as usize;
    let p_ret = exe::call_va!(0x1_411a_5000, (rbx_0, str_retro) -> usize);
    let al_ret = if p_ret == 0 { 0u8 } else { rd::<u8>(p_ret + 0x08) };

    let mut r8_flags = (((((al_ret as u64) * 2) | (sil as u64)) * 2) | edi_bit) << 3;
    if b_80 == 1 {
        r8_flags |= 4;
    }
    if b_81 == 0 {
        r8_flags |= 2;
    }

    wr::<u64>(param_1 + 0x28, hash_material_flags(r8_flags));
    wr::<u64>(param_1 + 0x30, hash_material_flags(r8_flags | 1));
    param_1
}

#[inline(always)]
fn quantize_u8(v: f32, scale: f32) -> u8 {
    let r = (v * scale).round_ties_even();
    let i = crate::simd_ops::cvt_f32_i32(r) as i32;
    if i < 0 {
        0
    } else if i > 0xff {
        0xff
    } else {
        i as u8
    }
}

/// `rend_14233d270`
/// Original: 0x14233d270 (848 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233d270(param_1: usize) -> u64 {
    let rcx_top = rd::<usize>(param_1 + 0x08);
    let r8_base = rd::<usize>(param_1 + 0x10);
    let nargs = ((rcx_top as i64).wrapping_sub(r8_base as i64) / 0x18) as i32;
    let nil_obj = exe::ptr::<u8>(0x1_4298_68d0) as usize;

    let (xmm6, xmm8, xmm7, xmm4): (f32, f32, f32, f32);
    if nargs >= 3 {
        let mut local_98 = [0u64; 3];
        let mut p1 = if r8_base < rcx_top { r8_base } else { nil_obj };
        if rd::<u32>(p1 + 0x10) != 3 {
            p1 = exe::call_va!(0x1_420e_cd00, (p1, local_98.as_mut_ptr() as usize) -> usize);
            if p1 == 0 {
                exe::call_va!(0x1_4211_58a0, (param_1, 1u32, 3u32));
                return 0;
            }
        }
        xmm6 = rd::<f64>(p1) as f32;

        let mut local_80 = [0u64; 3];
        let raw_p2 = rd::<usize>(param_1 + 0x10).wrapping_add(0x18);
        let mut p2 = if raw_p2 < rd::<usize>(param_1 + 0x08) {
            raw_p2
        } else {
            nil_obj
        };
        if rd::<u32>(p2 + 0x10) != 3 {
            p2 = exe::call_va!(0x1_420e_cd00, (p2, local_80.as_mut_ptr() as usize) -> usize);
            if p2 == 0 {
                exe::call_va!(0x1_4211_58a0, (param_1, 2u32, 3u32));
                return 0;
            }
        }
        xmm8 = rd::<f64>(p2) as f32;

        let mut local_68 = [0u64; 3];
        let raw_p3 = rd::<usize>(param_1 + 0x10).wrapping_add(0x30);
        let mut p3 = if raw_p3 < rd::<usize>(param_1 + 0x08) {
            raw_p3
        } else {
            nil_obj
        };
        if rd::<u32>(p3 + 0x10) != 3 {
            p3 = exe::call_va!(0x1_420e_cd00, (p3, local_68.as_mut_ptr() as usize) -> usize);
            if p3 == 0 {
                exe::call_va!(0x1_4211_58a0, (param_1, 3u32, 3u32));
                return 0;
            }
        }
        xmm7 = rd::<f64>(p3) as f32;

        let mut local_50 = [0u64; 3];
        let raw_p4 = rd::<usize>(param_1 + 0x10).wrapping_add(0x48);
        if raw_p4 < rd::<usize>(param_1 + 0x08)
            && raw_p4 != nil_obj
            && rd::<i32>(raw_p4 + 0x10) > 0
        {
            let mut p4 = raw_p4;
            if rd::<u32>(p4 + 0x10) != 3 {
                p4 = exe::call_va!(0x1_420e_cd00, (p4, local_50.as_mut_ptr() as usize) -> usize);
                if p4 == 0 {
                    exe::call_va!(0x1_4211_58a0, (param_1, 4u32, 3u32));
                    return 0;
                }
            }
            xmm4 = rd::<f64>(p4) as f32;
        } else {
            xmm4 = 0.0f32;
        }
    } else {
        if r8_base >= rcx_top || r8_base == nil_obj || rd::<u32>(r8_base + 0x10) != 4 {
            let msg = exe::ptr::<u8>(0x1_433f_e660) as usize;
            exe::call_va!(0x1_4211_6bc0, (param_1, 1u32, msg));
            return 0;
        }
        if r8_base == 0 {
            exe::call_va!(0x1_4211_58a0, (param_1, 1u32, 4u32));
            return 0;
        }
        xmm6 = rd::<f32>(r8_base);
        xmm8 = rd::<f32>(r8_base + 0x04);
        xmm7 = rd::<f32>(r8_base + 0x08);
        xmm4 = rd::<f32>(r8_base + 0x0c);
    }

    let scale = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eec) as usize);
    let b0 = quantize_u8(xmm6, scale);
    let ptr_addr = exe::ptr::<usize>(0x1_43a3_6df8) as usize;
    let rem_addr = exe::ptr::<u64>(0x1_43a3_70e8) as usize;

    let rem0 = rd::<u64>(rem_addr);
    if rem0 >= 1 {
        let p0 = rd::<usize>(ptr_addr);
        let b1 = quantize_u8(xmm8, scale);
        wr::<u8>(p0, b0);
        let p1 = p0.wrapping_add(1);
        wr::<usize>(ptr_addr, p1);
        let mut rem = rem0.wrapping_sub(1);
        wr::<u64>(rem_addr, rem);
        if rem >= 1 {
            rem = rem.wrapping_sub(1);
            wr::<u8>(p1, b1);
            let b2 = quantize_u8(xmm7, scale);
            wr::<u64>(rem_addr, rem);
            let mut p2 = p1.wrapping_add(1);
            wr::<usize>(ptr_addr, p2);
            if rem >= 1 {
                wr::<u8>(p2, b2);
                p2 = p2.wrapping_add(1);
                rem = rem.wrapping_sub(1);
                wr::<usize>(ptr_addr, p2);
                let b3 = quantize_u8(xmm4, scale);
                wr::<u64>(rem_addr, rem);
                if rem >= 1 {
                    wr::<u8>(p2, b3);
                    wr::<usize>(ptr_addr, p2.wrapping_add(1));
                    wr::<u64>(rem_addr, rem.wrapping_sub(1));
                    return 0;
                }
            }
        }
    }

    let msg = exe::ptr::<u8>(0x1_433f_da00) as usize;
    exe::call_va!(0x1_4211_6b30, (param_1, msg));
    0
}

/// `anonymous_namespace_142394700`
/// Original: 0x142394700 (285 bytes, Throws::No)
pub unsafe extern "win64" fn anonymous_namespace_142394700(param_1: usize) {
    let c128 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7660) as usize);
    let w0 = c128 as u32;
    let r11d = (c128 >> 32) as u32;
    let xmm1_hi = (c128 >> 64) as u64;
    let mut r10d = rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize);
    let mut xmm7_lo: u64 = (w0 as u64) << 32;

    if rd::<u8>(param_1 + 0x34) != 0 {
        let s = rd::<u32>(param_1 + 0x30);
        r10d = s;
        xmm7_lo = (s as u64) << 32;
        let p38 = rd::<usize>(param_1 + 0x38);
        wr::<u32>(p38 + 0x90, s);
        let p40 = rd::<usize>(param_1 + 0x40);
        wr::<u32>(p40 + 0x90, rd::<u32>(param_1 + 0x30));
    }

    let p38 = rd::<usize>(param_1 + 0x38);
    let xmm3 = rd::<u32>(param_1 + 0x18);
    let xmm2 = rd::<u32>(param_1 + 0x1c);
    let xmm5 = rd::<u32>(param_1 + 0x20);
    let xmm6_lo: u64 = r10d as u64;
    let xmm0_lo: u64 = (xmm3 as u64) | ((xmm2 as u64) << 32);

    wr::<u64>(p38 + 0x5c, xmm7_lo);
    wr::<u64>(p38 + 0x68, xmm1_hi);
    wr::<u32>(p38 + 0x64, r11d);
    wr::<u32>(p38 + 0x70, r10d);
    wr::<u64>(p38 + 0x50, xmm6_lo);
    wr::<u32>(p38 + 0x58, 0);
    wr::<u64>(p38 + 0x74, xmm0_lo);
    wr::<u32>(p38 + 0x7c, xmm5);

    let p40 = rd::<usize>(param_1 + 0x40);
    wr::<u64>(p40 + 0x50, xmm6_lo);
    wr::<u64>(p40 + 0x5c, xmm7_lo);
    wr::<u32>(p40 + 0x58, 0);
    wr::<u64>(p40 + 0x68, xmm1_hi);
    wr::<u64>(p40 + 0x74, xmm0_lo);
    wr::<u32>(p40 + 0x7c, xmm5);
    wr::<u32>(p40 + 0x64, r11d);
    wr::<u32>(p40 + 0x70, r10d);
}

/// `unknown_1423c5090`
/// Original: 0x1423c5090 (118 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1423c5090(
    param_1: usize,
    param_2: u32,
    param_3: usize,
    _param_4_xmm3: usize,
) {
    let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
    let param_4 = f32::from_bits(xmm3 as u32);
    let rcx_val = rd::<u64>(param_3);
    if rcx_val != 0 && !(0.0f32 >= param_4) {
        let idx = param_2 as usize;
        let slot_ptr = param_1.wrapping_add((idx + 3) << 4);
        let f_ptr = param_1.wrapping_add(idx * 0x10 + 0x38);
        if rd::<u64>(slot_ptr) != rcx_val || rd::<f32>(f_ptr) != param_4 {
            exe::call_va!(0x1_403b_3290, (slot_ptr, param_3));
            wr::<u32>(f_ptr, xmm3 as u32);
            wr::<u8>(param_1 + 0x24, 1);
        }
    }
}

#[inline(always)]
unsafe fn select_tex_handle(esi_mode: u32, rdi_tex: usize, rbx_default: usize) -> u64 {
    if esi_mode == 0 {
        if rdi_tex != 0 { rd::<u64>(rdi_tex + 0x78) } else { rd::<u64>(rbx_default + 0x88) }
    } else if esi_mode == 5 {
        if rdi_tex != 0 { rd::<u64>(rdi_tex + 0x90) } else { rd::<u64>(rbx_default + 0x90) }
    } else {
        rd::<u64>(rbx_default + 0x88)
    }
}

#[inline(always)]
unsafe fn bind_env_tex_slot(
    r15_tls0: usize,
    tex_ptr: usize,
    guard_va: u64,
    ptr_va: u64,
    id_va: u64,
    val0_va: u64,
    str_va: u64,
    str_len: u64,
    atexit_va: u64,
) {
    let epoch = rd::<i32>(r15_tls0 + 0x74);
    let guard_addr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_addr) > epoch {
        exe::call_va!(0x1_4183_4a78, (guard_addr));
        if rd::<i32>(guard_addr) == -1 {
            let mut name_view: [u64; 2] = [exe::ptr::<u8>(str_va) as usize as u64, str_len];
            let id_addr = exe::ptr::<i32>(id_va) as usize;
            exe::call_va!(0x1_4220_ea70, (id_addr, name_view.as_mut_ptr() as usize));
            wr::<u32>(exe::ptr::<u32>(id_va + 4) as usize, 9);
            let val0_addr = exe::ptr::<u64>(val0_va) as usize;
            wr::<usize>(exe::ptr::<usize>(ptr_va) as usize, val0_addr);
            wr::<u64>(val0_addr, 0);
            wr::<u64>(exe::ptr::<u64>(val0_va + 8) as usize, 0);
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(atexit_va) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_addr));
        }
    }

    let rbx_default = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let rdi_tex = rd::<usize>(tex_ptr);
    let r14 = rd::<i32>(exe::ptr::<i32>(id_va) as usize) as isize;
    let base_590 = exe::ptr::<u8>(0x1_4385_8590) as usize;
    let esi_mode = rd::<u32>(base_590.wrapping_add_signed(r14 * 4));
    let handle1 = select_tex_handle(esi_mode, rdi_tex, rbx_default);

    let local_pair: [u64; 2] = [handle1, rdi_tex as u64];
    let tbl_7b0 = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let off_7b0 = rd::<i32>(tbl_7b0.wrapping_add_signed(r14 * 16));
    if off_7b0 >= 0 {
        let len_7b8 = rd::<i32>(tbl_7b0.wrapping_add_signed(r14 * 16 + 8)) as i64 as u64;
        let dst = r15_tls0.wrapping_add(0x63d0).wrapping_add_signed(off_7b0 as isize);
        exe::call_va!(0x1_4183_acbc, (dst, local_pair.as_ptr() as usize, len_7b8));
        let idx_7bc = rd::<i32>(tbl_7b0.wrapping_add_signed(r14 * 16 + 12));
        if idx_7bc != -1 {
            let ctr_addr = r15_tls0.wrapping_add(0x463e0).wrapping_add_signed((idx_7bc as isize) * 4);
            let next_ctr = rd::<i32>(ctr_addr).wrapping_add(1);
            wr::<i32>(ctr_addr, if next_ctr == -1 { 0 } else { next_ctr });
        }
    }

    wr::<usize>(exe::ptr::<usize>(val0_va + 8) as usize, rdi_tex);
    let handle2 = select_tex_handle(esi_mode, rdi_tex, rbx_default);
    wr::<u64>(exe::ptr::<u64>(val0_va) as usize, handle2);
}

/// `unknown_1423c7200`
/// Original: 0x1423c7200 (1323 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1423c7200(param_1: usize) {
    let r15_tls0 = rd::<usize>(exe::tls_pointer());
    bind_env_tex_slot(r15_tls0, rd::<usize>(param_1), 0x1_43cf_f754, 0x1_43cf_f758, 0x1_43cf_f760, 0x1_43cf_f768, 0x1_4340_58e0, 0x0a, 0x1_4263_d140);
    bind_env_tex_slot(r15_tls0, rd::<usize>(param_1 + 0x08), 0x1_43cf_f750, 0x1_43cf_f710, 0x1_43cf_f718, 0x1_43cf_f720, 0x1_4340_58f0, 0x0f, 0x1_4263_d130);
    bind_env_tex_slot(r15_tls0, rd::<usize>(param_1 + 0x10), 0x1_43cf_f708, 0x1_43cf_f730, 0x1_43cf_f738, 0x1_43cf_f740, 0x1_4340_5900, 0x13, 0x1_4263_d120);
}

/// `unknown_1423dfe40`
/// Original: 0x1423dfe40 (168 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1423dfe40(param_1: usize, param_2: u64) {
    if param_2 == 0 {
        return;
    }
    let mut rsi = param_2;
    let rbp_tbl = exe::ptr::<usize>(0x1_43be_60a8) as usize;
    let mut rdi = param_1.wrapping_add(0x10);
    loop {
        let eax = rd::<u32>(rdi - 0x10);
        let rdx_arg = rdi - 0x08;
        let hi = (eax >> 16) as usize;
        let lo = (eax as u16) as usize;
        let base = rd::<usize>(rbp_tbl.wrapping_add(hi * 8));
        let rcx_arg = base.wrapping_add(lo * 0x98);
        exe::call_va!(0x1_4230_2e60, (rcx_arg, rdx_arg));

        let rbx = rd::<usize>(rdi);
        if rbx != 0 {
            let old_use = crate::simd_ops::atomic_xadd_u32((rbx + 0x08) as u64, u32::MAX);
            if old_use == 1 {
                let vfn0 = rd::<u64>(rd::<usize>(rbx));
                exe::call_va!(vfn0, (rbx));
                let old_weak = crate::simd_ops::atomic_xadd_u32((rbx + 0x0c) as u64, u32::MAX);
                if old_weak == 1 {
                    let vfn1 = rd::<u64>(rd::<usize>(rbx) + 0x08);
                    exe::call_va!(vfn1, (rbx));
                }
            }
        }
        rdi = rdi.wrapping_add(0x18);
        rsi -= 1;
        if rsi == 0 {
            break;
        }
    }
}

#[inline(always)]
fn lehmer_step(seed: u32) -> u32 {
    let r9 = (seed as u64).wrapping_mul(0xbc8f);
    let rdx = (((r9 as u128).wrapping_mul(0x2000_00005u128)) >> 64) as u64;
    let rax = (((r9.wrapping_sub(rdx) >> 1).wrapping_add(rdx)) >> 0x1e) as u32;
    (r9 as u32).wrapping_sub(rax.wrapping_mul(0x7fff_ffff))
}

/// `r_142417470`
/// Original: 0x142417470 (392 bytes, Throws::No)
pub unsafe extern "win64" fn r_142417470(param_1: usize) {
    let rbx_tls0 = rd::<usize>(exe::tls_pointer());
    let mut edx_flags = rd::<u32>(rbx_tls0 + 0xa87f0);
    if (edx_flags & 1) == 0 {
        edx_flags |= 1;
        wr::<u32>(rbx_tls0 + 0xa87f0, edx_flags);
        wr::<u64>(rbx_tls0 + 0xa87f8, 0);
        wr::<u64>(rbx_tls0 + 0xa8800, u64::MAX);
    }
    if (edx_flags & 2) == 0 {
        edx_flags |= 2;
        wr::<u32>(rbx_tls0 + 0xa87f0, edx_flags);
        let mut r8d = exe::call_iat!(0x1_4264_5940, () -> u32);
        let prod = (r8d as u64) * 3;
        let edx_hi = (prod >> 32) as u32;
        let mut ecx = r8d.wrapping_sub(edx_hi);
        ecx = (ecx >> 1).wrapping_add(edx_hi);
        ecx >>= 0x1e;
        ecx = ecx.wrapping_mul(0x8000_0001);
        r8d = r8d.wrapping_add(ecx);
        if r8d == 0 {
            r8d = 1;
        }
        wr::<u32>(rbx_tls0 + 0xa8808, r8d);
    }

    let r11 = rbx_tls0 + 0xa8808;
    let rdx_val = rd::<u64>(param_1);
    let mut r8_spins: u64;
    if rdx_val == u64::MAX {
        let mut r9d: u32;
        loop {
            r9d = lehmer_step(rd::<u32>(r11));
            wr::<u32>(r11, r9d);
            let cand = r9d.wrapping_sub(1) as u64;
            if cand <= 0x3fff_ffff {
                r8_spins = cand;
                break;
            }
        }
        let mut ecx = r9d;
        for _ in 0..2 {
            r8_spins <<= 0x1e;
            let edx: u64;
            loop {
                ecx = lehmer_step(ecx);
                let cand = ecx.wrapping_sub(1) as u64;
                if cand <= 0x3fff_ffff {
                    edx = cand;
                    break;
                }
            }
            r8_spins |= edx;
        }
        wr::<u32>(r11, ecx);
    } else {
        let mut ptr_slot: usize = r11;
        r8_spins = exe::call_va!(
            0x1_4139_b4a0,
            ((&mut ptr_slot as *mut usize) as usize, rdx_val.wrapping_add(1)) -> u64
        );
    }

    let mut next_val = rd::<u64>(param_1).wrapping_mul(2);
    if next_val > 0x1388 {
        next_val = 0x1388;
    }
    wr::<u64>(param_1, next_val);

    while r8_spins != 0 {
        core::hint::spin_loop();
        r8_spins -= 1;
    }
}

/// `r_14241d6f0`
/// Original: 0x14241d6f0 (341 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14241d6f0(param_1: usize, param_2: usize) {
    let lock1 = param_1 + 0x140;
    let tid1 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock1 + 0x08) != tid1 {
        exe::call_iat!(0x1_4264_55a8, (lock1));
        wr::<u32>(lock1 + 0x08, tid1);
    }
    wr::<u32>(lock1 + 0x0c, rd::<u32>(lock1 + 0x0c).wrapping_add(1));

    let eax = rd::<u32>(param_2 + 0x24);
    let rdx_58 = rd::<usize>(param_2 + 0x58);
    if eax == 0 {
        if rdx_58 != param_1 + 0x40 {
            exe::call_va!(0x1_4139_e210, (param_1 + 0x40, param_2 + 0x40));
        }
        let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
        let vfn_40 = rd::<u64>(rd::<usize>(param_2) + 0x40);
        let idx = exe::call_va!(vfn_40, (param_2) -> i32) as isize;
        let rsi_bucket = g_2b8.wrapping_add(0xb0).wrapping_add_signed(idx * 0x40);
        let lock2 = rsi_bucket + 0x30;
        let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(lock2 + 0x08) != tid2 {
            exe::call_iat!(0x1_4264_55a8, (lock2));
            wr::<u32>(lock2 + 0x08, tid2);
        }
        wr::<u32>(lock2 + 0x0c, rd::<u32>(lock2 + 0x0c).wrapping_add(1));

        let mut moved = false;
        if rd::<usize>(param_2 + 0x78) != rsi_bucket {
            exe::call_va!(0x1_4139_e210, (rsi_bucket, param_2 + 0x60));
            moved = true;
        }

        let new_c2 = rd::<u32>(lock2 + 0x0c).wrapping_sub(1);
        wr::<u32>(lock2 + 0x0c, new_c2);
        if new_c2 == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((lock2 + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (lock2));
        }
        if moved {
            let vfn_50 = rd::<u64>(rd::<usize>(param_2) + 0x50);
            exe::call_va!(vfn_50, (param_2));
        }
    } else {
        if rdx_58 != param_1 {
            exe::call_va!(0x1_4139_e210, (param_1, param_2 + 0x40));
        }
        let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
        exe::call_va!(0x1_4242_0dd0, (g_2b8, param_2));
    }

    let new_c1 = rd::<u32>(lock1 + 0x0c).wrapping_sub(1);
    wr::<u32>(lock1 + 0x0c, new_c1);
    if new_c1 == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((lock1 + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock1));
    }
}

/// `r_1424282e0`
/// Original: 0x1424282e0 (307 bytes, Throws::No)
pub unsafe extern "win64" fn r_1424282e0(param_1: usize, param_2: u64) {
    let lock_addr = param_1 + 0x84;
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(lock_addr) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8(lock_addr as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }

    let mut key_slot: u64 = param_2;
    let hash = exe::call_va!(
        0x1_413a_3560,
        (0usize, (&mut key_slot as *mut u64) as usize) -> u64
    );
    let mask = rd::<u64>(param_1 + 0xa0);
    let mut r8 = (hash >> 7) & mask;
    let mut r11: u64 = 0;
    let ctrl_base = rd::<usize>(param_1 + 0x88);
    let h2 = (hash as u8) & 0x7f;
    let xmm2 = crate::simd_ops::vpbroadcastb(h2);

    loop {
        let xmm1 = rd::<u128>(ctrl_base.wrapping_add(r8 as usize));
        let eq_mask = crate::simd_ops::vpmovmskb(crate::simd_ops::vpcmpeqb(xmm1, xmm2)) as u32;
        if eq_mask != 0 {
            let keys_base = rd::<usize>(param_1 + 0x90);
            let mut m = eq_mask;
            while m != 0 {
                let bit = m.trailing_zeros() as u64;
                let idx = r8.wrapping_add(bit) & mask;
                if rd::<u64>(keys_base.wrapping_add((idx as usize) * 8)) == param_2 {
                    crate::simd_ops::wu8((lock_addr) as u64, 0); // spinlock release (x86 mov = release)
                    return;
                }
                m &= m - 1;
            }
        }
        let empty_mask = crate::simd_ops::vpmovmskb(crate::simd_ops::vpsignb(xmm1, xmm1)) as u32;
        if empty_mask != 0 {
            let slot = exe::call_va!(0x1_413a_3610, (param_1 + 0x88, hash) -> usize);
            let keys_base = rd::<usize>(param_1 + 0x90);
            wr::<u64>(keys_base.wrapping_add(slot * 8), param_2);
            break;
        }
        r11 = r11.wrapping_add(0x10);
        r8 = r8.wrapping_add(r11) & mask;
    }

    crate::simd_ops::wu8((lock_addr) as u64, 0); // spinlock release (x86 mov = release)
}

/// `r_14242dcc0`
/// Original: 0x14242dcc0 (318 bytes, Throws::No)
pub unsafe extern "win64" fn r_14242dcc0(param_1: usize, param_2: u64) -> usize {
    let mut r8 = rd::<usize>(param_1 + 0x08);
    if r8 == 0 {
        let ebx = rd::<u32>(param_1 + 0x10);
        let tls0 = rd::<usize>(exe::tls_pointer());
        let alloc_ctx = rd::<usize>(tls0 + 0x6348);
        let mut rax = exe::call_va!(0x1_4253_5070, (alloc_ctx, 0x10000u32, 0x10000u32) -> usize);
        if rax == 0 {
            let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
            exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
            rax = 0;
        }
        let hdr = rax.wrapping_add(0xffe8);
        wr::<u64>(hdr, 0x7a63_f007);
        wr::<u32>(hdr + 0x08, ebx);
        wr::<u32>(hdr + 0x0c, 0);
        wr::<u64>(hdr + 0x10, 0);
        let old_p0 = rd::<usize>(param_1);
        wr::<usize>(param_1, hdr);
        if old_p0 != 0 {
            let rbx_block = old_p0.wrapping_sub(0xffe8);
            let sub_10 = rd::<usize>(old_p0 + 0x10);
            if sub_10 != 0 {
                exe::call_va!(0x1_4242_df10, (0usize, sub_10));
            }
            exe::call_va!(0x1_4244_2de0, (rbx_block));
        }
        r8 = rd::<usize>(param_1);
        wr::<usize>(param_1 + 0x08, r8);
    } else {
        let ecx = rd::<u32>(r8 + 0x0c) as u64;
        if 0xffe8u64.wrapping_sub(ecx) < param_2 {
            let edx = rd::<u32>(r8 + 0x04).wrapping_add(ecx as u32);
            let r8d = rd::<u32>(param_1 + 0x10);
            let mut local_40: usize = 0;
            let rax_new = exe::call_va!(
                0x1_413a_4120,
                ((&mut local_40 as *mut usize) as usize, edx, r8d) -> usize
            );
            exe::call_va!(0x1_413a_2140, (rd::<usize>(param_1 + 0x08) + 0x10, rax_new));
            if local_40 != 0 {
                let rbx_block = local_40.wrapping_sub(0xffe8);
                exe::call_va!(0x1_401e_17d0, (local_40 + 0x10));
                exe::call_va!(0x1_4244_2de0, (rbx_block));
            }
            let p8 = rd::<usize>(param_1 + 0x08);
            r8 = rd::<usize>(p8 + 0x10);
            wr::<usize>(param_1 + 0x08, r8);
        }
    }

    let ecx = rd::<u32>(r8 + 0x0c);
    wr::<u32>(r8 + 0x0c, ecx.wrapping_add(param_2 as u32));
    r8.wrapping_add(ecx as usize).wrapping_sub(0xffe8)
}

/// `r_14242df10`
/// Original: 0x14242df10 (52 bytes, Throws::No)
pub unsafe extern "win64" fn r_14242df10(_param_1: usize, param_2: usize) {
    if param_2 != 0 {
        let rbx = param_2.wrapping_sub(0xffe8);
        exe::call_va!(0x1_401e_17d0, (param_2 + 0x10));
        exe::call_va!(0x1_4244_2de0, (rbx));
    }
}

/// `unknown_14243bd60`
/// Original: 0x14243bd60 (149 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14243bd60(
    param_1: u32,
    _param_2_xmm1: usize,
    _param_3_xmm2: usize,
    _param_4_xmm3: usize,
) -> f32 {
    let [_, xmm1, xmm2, xmm3] = crate::simd_ops::take_xmm_args();
    let x_bits = xmm1 as u32;
    let y_bits = xmm2 as u32;
    let z_bits = xmm3 as u32;
    let h = param_1 & 0x0f;
    let u_bits = if h < 8 { x_bits } else { y_bits };
    let v_bits = if h < 4 {
        y_bits
    } else if (h.wrapping_sub(0x0c) & !2u32) == 0 {
        x_bits
    } else {
        z_bits
    };
    let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    let u_signed = if (param_1 & 1) != 0 {
        u_bits ^ sign_mask
    } else {
        u_bits
    };
    let v_signed = if (param_1 & 2) != 0 {
        v_bits ^ sign_mask
    } else {
        v_bits
    };
    let res = f32::from_bits(v_signed) + f32::from_bits(u_signed);
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

/// `unknown_14243be50`
/// Original: 0x14243be50 (195 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14243be50(_param_1_xmm0: usize) -> f32 {
    let [xmm0, _, _, _] = crate::simd_ops::take_xmm_args();
    let x = f32::from_bits(xmm0 as u32);
    let fl = x.floor();
    let eax = crate::simd_ops::cvt_f32_i32(fl);
    let r8 = (eax as u8) as usize;
    let xmm7 = x - fl;
    let r9_tbl = exe::ptr::<u8>(0x1_42cb_7dc0) as usize;

    let a0 = rd::<u8>(r9_tbl + r8) as usize;
    let c0 = rd::<u8>(r9_tbl + a0) as usize;
    let h0 = rd::<u8>(r9_tbl + c0) as u32;
    let (_, g0_xmm) = crate::simd_ops::call_va8_xmm(
        0x1_4243_bd60,
        h0 as u64,
        0, 0, 0, 0, 0, 0, 0,
        [0, xmm7.to_bits() as u128, 0, 0],
    );
    let xmm6 = f32::from_bits(g0_xmm as u32);

    let a1 = rd::<u8>(r9_tbl + r8 + 1) as usize;
    let c1 = rd::<u8>(r9_tbl + a1) as usize;
    let h1 = rd::<u8>(r9_tbl + c1) as u32;
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm1_arg = xmm7 - one;
    let (_, g1_xmm) = crate::simd_ops::call_va8_xmm(
        0x1_4243_bd60,
        h1 as u64,
        0, 0, 0, 0, 0, 0, 0,
        [0, xmm1_arg.to_bits() as u128, 0, 0],
    );
    let g1 = f32::from_bits(g1_xmm as u32);

    let c_6 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6d6c) as usize);
    let c_15 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e04) as usize);
    let c_10 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6dd4) as usize);
    let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);

    let t1 = xmm7 * c_6;
    let xmm4 = g1 - xmm6;
    let t2 = t1 - c_15;
    let t3 = t2 * xmm7;
    let xmm3 = t3 + c_10;
    let t4 = xmm7 * xmm7;
    let xmm1_cube = t4 * xmm7;
    let xmm2_fade = xmm3 * xmm1_cube;
    let xmm3_lerp = xmm4 * xmm2_fade;
    let t5 = xmm3_lerp + xmm6;
    let res = t5 + c_half;
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

/// `unknown_1424858a0`
/// Original: 0x1424858a0 (138 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1424858a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let rbx = rd::<usize>(param_1 + 0x10);
    let idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let rcx_alloc = rd::<usize>(rbx.wrapping_add((idx << 6) + 0x48)).wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (rcx_alloc, param_2));

    let rcx_node = rd::<usize>(param_2);
    let xmm0 = rd::<u128>(param_3);
    let _ = crate::simd_ops::atomic_xchg_u32(rcx_node as u64, 1);
    wr::<u64>(rcx_node + 0x08, 0);
    let vft = exe::ptr::<u8>(0x1_4340_e320) as usize as u64;
    wr::<u64>(rcx_node + 0x10, vft);
    wr::<u128>(rcx_node + 0x18, xmm0);

    let rcx_node2 = rd::<usize>(param_2);
    let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((rcx_node2 + 0x04) as u64, 1, 3);
    if ok {
        let _ = crate::simd_ops::atomic_add_u32(rcx_node2 as u64, 0xffff);
    }
    wr::<u8>(param_2 + 0x0a, 1);
    param_2
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
        Self::from_addr(addr, len)
    }

    unsafe fn from_addr(addr: usize, len: usize) -> Self {
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

pub unsafe fn diff_rend_142334b20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC1_MEM: [u64; 256] = [0; 256];
    static mut VEC2_MEM: [u8; 256 * 0x38] = [0; 256 * 0x38];
    static mut VEC3_MEM: [u8; 256 * 0x80] = [0; 256 * 0x80];
    static mut P1_MEM: [u8; 0x20a0] = [0; 0x20a0];
    static mut GROW_LOG: Vec<(u32, u64)> = Vec::new();

    unsafe extern "win64" fn mock_grow1(vec_p: usize, new_cap: u64) {
        wr::<u32>(vec_p + 0x0c, new_cap as u32);
        GROW_LOG.push((1, new_cap));
    }
    unsafe extern "win64" fn mock_grow2(vec_p: usize, new_cap: u64) {
        wr::<u32>(vec_p + 0x0c, new_cap as u32);
        GROW_LOG.push((2, new_cap));
    }
    unsafe extern "win64" fn mock_grow3(vec_p: usize, new_cap: u64) {
        wr::<u32>(vec_p + 0x0c, new_cap as u32);
        GROW_LOG.push((3, new_cap));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4087_46b0, mock_grow1 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_b640, mock_grow2 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4133_fed0, mock_grow3 as *const ())?;

    let run = |f: F, idx: u16, init_cap: u32| -> ([u32; 6], u64, u64, u64, u8, u8, Vec<(u32, u64)>) {
        GROW_LOG.clear();
        VEC1_MEM.fill(0xffff_ffff_ffff_ffff);
        VEC2_MEM.fill(0xaa);
        VEC3_MEM.fill(0xbb);
        P1_MEM.fill(0);

        let p1 = core::ptr::addr_of_mut!(P1_MEM) as usize;
        wr::<usize>(p1, core::ptr::addr_of_mut!(VEC1_MEM) as usize);
        wr::<u32>(p1 + 0x08, 4);
        wr::<u32>(p1 + 0x0c, init_cap);

        wr::<usize>(p1 + 0x10, core::ptr::addr_of_mut!(VEC2_MEM) as usize);
        wr::<u32>(p1 + 0x18, 4);
        wr::<u32>(p1 + 0x1c, init_cap);

        wr::<usize>(p1 + 0x38, core::ptr::addr_of_mut!(VEC3_MEM) as usize);
        wr::<u32>(p1 + 0x40, 4);
        wr::<u32>(p1 + 0x44, init_cap);

        let w_idx = (idx >> 6) as usize;
        wr::<u64>(p1 + 0x78 + w_idx * 8, u64::MAX);
        wr::<u64>(p1 + 0x2078 + w_idx * 8, 0);

        let mut p2 = [0u8; 0x30];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u16>(p2p + 0x20, idx);

        f(p1, p2p);
        let lens = [
            rd::<u32>(p1 + 0x08),
            rd::<u32>(p1 + 0x0c),
            rd::<u32>(p1 + 0x18),
            rd::<u32>(p1 + 0x1c),
            rd::<u32>(p1 + 0x40),
            rd::<u32>(p1 + 0x44),
        ];
        (
            lens,
            rd::<u64>(p1 + 0x78 + w_idx * 8),
            rd::<u64>(p1 + 0x2078 + w_idx * 8),
            VEC1_MEM[5],
            VEC2_MEM[5 * 0x38 + 7],
            VEC3_MEM[5 * 0x80 + 15],
            GROW_LOG.clone(),
        )
    };

    for (idx, cap) in [(5u16, 64u32), (5u16, 256u32), (130u16, 100u32)] {
        let o = run(f_o, idx, cap);
        let n = run(f_n, idx, cap);
        crate::diff_eq!(o, n, "142334b20 idx={} cap={}", idx, cap);
    }
    Ok(3)
}

pub unsafe fn diff_rend_1423387b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43a0_0210, 8);
    let _g2 = GlobalSave::new(0x1_4386_c790, 16);
    let _g3 = GlobalSave::new(0x1_43bb_d988, 16);
    let _g4 = GlobalSave::new(0x1_43c8_cb00, 1);
    let _g5 = GlobalSave::new(0x1_4398_00e0, 64);
    let _g6 = GlobalSave::new(0x1_43a3_6c98, 8);

    static mut SUB1_MEM: [u8; 0x30] = [0; 0x30];
    static mut SUB1_ARR: [u64; 8] = [0; 8];
    static mut SUB2_MEM: [u8; 0x30] = [0; 0x30];
    static mut SUB2_ARR: [u64; 16] = [0; 16];
    static mut ID_STACK: [u16; 8] = [0; 8];
    static mut MGR_C98: [u8; 0x100] = [0; 0x100];
    static mut PROP_EM: [u8; 16] = [0; 16];
    static mut PROP_RET: [u8; 16] = [0; 16];
    static mut P1_OUT: [u8; 0x40] = [0; 0x40];
    static mut CALL_LOG: Vec<(u32, u64)> = Vec::new();

    unsafe extern "win64" fn mock_f680(_p: usize) -> u64 {
        0x7777_0000
    }
    unsafe extern "win64" fn mock_70b0(mgr: usize, view: usize) -> u64 {
        (mgr as u64) ^ rd::<u64>(view) ^ rd::<u64>(view + 8)
    }
    unsafe extern "win64" fn mock_0bc0(out: usize, _rdx: usize, _r8: usize) -> usize {
        wr::<usize>(out, core::ptr::addr_of_mut!(SUB1_MEM) as usize);
        out
    }
    unsafe extern "win64" fn mock_0c60(out: usize, _rdx: usize, _r8: usize) -> usize {
        wr::<usize>(out, core::ptr::addr_of_mut!(SUB2_MEM) as usize);
        out
    }
    unsafe extern "win64" fn mock_2de0(_p: usize) {}
    unsafe extern "win64" fn mock_8030(vec_p: usize, cap: u64) {
        wr::<u32>(vec_p + 0x0c, cap as u32);
    }
    unsafe extern "win64" fn mock_2820(vec_p: usize, item_p: usize) -> usize {
        let len = rd::<u32>(vec_p + 0x08) as usize;
        let base = rd::<usize>(vec_p);
        let slot = base + len * 8;
        wr::<u64>(slot, rd::<u64>(item_p));
        wr::<u32>(vec_p + 0x08, (len + 1) as u32);
        slot
    }
    unsafe extern "win64" fn mock_82e0(obj: usize, owner: usize) {
        CALL_LOG.push((1, (obj as u64) ^ (owner as u64)));
    }
    unsafe extern "win64" fn mock_f620(vec_p: usize, cap: u64) {
        wr::<u32>(vec_p + 0x0c, cap as u32);
    }
    unsafe extern "win64" fn mock_d6f0(ctx: usize, item: usize) {
        CALL_LOG.push((2, (ctx as u64) ^ (item as u64)));
    }
    unsafe extern "win64" fn mock_9780(_item: usize, _cb: usize) -> u32 {
        42
    }
    unsafe extern "win64" fn mock_4b20(mgr: usize, _p1: usize) {
        CALL_LOG.push((3, mgr as u64));
    }
    unsafe extern "win64" fn mock_5000(_p2_0: usize, name_ptr: usize) -> usize {
        if rd::<u8>(name_ptr) == b'E' {
            core::ptr::addr_of_mut!(PROP_EM) as usize
        } else {
            core::ptr::addr_of_mut!(PROP_RET) as usize
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_f680, mock_f680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4239_70b0, mock_70b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4134_0bc0, mock_0bc0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4134_0c60, mock_0c60 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411c_8030, mock_8030 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_411c_2820, mock_2820 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4242_82e0, mock_82e0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_412d_f620, mock_f620 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4220_9780, mock_9780 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4233_4b20, mock_4b20 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_411a_5000, mock_5000 as *const ())?;

    static mut ITEM1: [u8; 0x30] = [0; 0x30];
    static mut ARR20: [usize; 2] = [0; 2];
    static mut ARR30: [usize; 2] = [0; 2];
    static mut P2_MEM: [u8; 0x90] = [0; 0x90];

    let run = |f: F, b80: u8, b81: u8, em: u8, ret_b: u8| -> ([u64; 4], u16, u8, u64, Vec<(u32, u64)>) {
        CALL_LOG.clear();
        SUB1_MEM = [0; 0x30];
        SUB1_ARR = [0; 8];
        SUB2_MEM = [0; 0x30];
        SUB2_ARR = [0; 16];
        ID_STACK = [0, 3, 5, 0, 0, 0, 0, 0];
        MGR_C98 = [0; 0x100];
        PROP_EM = [0; 16];
        PROP_RET = [0; 16];
        P1_OUT = [0; 0x40];
        ITEM1 = [0; 0x30];
        P2_MEM = [0; 0x90];

        PROP_EM[8] = em;
        PROP_RET[8] = ret_b;

        let s1 = core::ptr::addr_of_mut!(SUB1_MEM) as usize;
        wr::<usize>(s1 + 0x10, core::ptr::addr_of_mut!(SUB1_ARR) as usize);
        let s2 = core::ptr::addr_of_mut!(SUB2_MEM) as usize;
        wr::<usize>(s2 + 0x10, core::ptr::addr_of_mut!(SUB2_ARR) as usize);

        let item1_p = core::ptr::addr_of_mut!(ITEM1) as usize;
        wr::<u32>(item1_p + 0x24, 0);
        wr::<usize>(item1_p + 0x28, 0x9999);

        ARR20 = [0x1111, 0x1111];
        ARR30 = [item1_p, item1_p];

        let p2 = core::ptr::addr_of_mut!(P2_MEM) as usize;
        wr::<usize>(p2, 0x2222);
        wr::<usize>(p2 + 0x20, core::ptr::addr_of_mut!(ARR20) as usize);
        wr::<u32>(p2 + 0x28, 2);
        wr::<usize>(p2 + 0x30, core::ptr::addr_of_mut!(ARR30) as usize);
        wr::<u32>(p2 + 0x38, 2);
        wr::<u32>(p2 + 0x54, 6);
        wr::<u32>(p2 + 0x7c, 0x8000_0000);
        wr::<u8>(p2 + 0x80, b80);
        wr::<u8>(p2 + 0x81, b81);

        wr::<usize>(exe::ptr::<usize>(0x1_43a0_0210) as usize, 0x1234);
        wr::<u64>(exe::ptr::<u64>(0x1_4386_c790) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_c798) as usize, u32::MAX);
        wr::<u32>(exe::ptr::<u32>(0x1_4386_c79c) as usize, 0);
        wr::<usize>(
            exe::ptr::<usize>(0x1_43bb_d988) as usize,
            core::ptr::addr_of_mut!(ID_STACK) as usize,
        );
        wr::<u32>(exe::ptr::<u32>(0x1_43bb_d990) as usize, 3);
        wr::<u32>(exe::ptr::<u32>(0x1_43bb_d994) as usize, 8);
        wr::<u8>(exe::ptr::<u8>(0x1_43c8_cb00) as usize, 1);
        let mgr_p = core::ptr::addr_of_mut!(MGR_C98) as usize;
        wr::<usize>(exe::ptr::<usize>(0x1_43a3_6c98) as usize, mgr_p);

        let p1 = core::ptr::addr_of_mut!(P1_OUT) as usize;
        let ret = f(p1, p2);
        assert_eq!(ret, p1);
        (
            [
                rd::<u64>(p1 + 0x18),
                rd::<u64>(p1 + 0x28),
                rd::<u64>(p1 + 0x30),
                rd::<u64>(mgr_p + 0x78),
            ],
            rd::<u16>(p1 + 0x20),
            rd::<u8>(p1 + 0x38),
            SUB2_ARR[1],
            CALL_LOG.clone(),
        )
    };

    for (b80, b81, em, ret_b) in [(1u8, 0u8, 1u8, 1u8), (0u8, 1u8, 0u8, 1u8)] {
        let o = run(f_o, b80, b81, em, ret_b);
        let n = run(f_n, b80, b81, em, ret_b);
        crate::diff_eq!(o, n, "1423387b0 b80={} b81={}", b80, b81);
    }
    Ok(2)
}

pub unsafe fn diff_rend_14233d270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43a3_6df8, 8);
    let _g2 = GlobalSave::new(0x1_43a3_70e8, 8);

    static mut OUT_BUF: [u8; 16] = [0; 16];

    let run = |f: F, use_vec: bool, nargs: usize| -> (u64, [u8; 8], usize, u64) {
        OUT_BUF = [0; 16];
        let out_base = core::ptr::addr_of_mut!(OUT_BUF) as usize;
        wr::<usize>(exe::ptr::<usize>(0x1_43a3_6df8) as usize, out_base);
        wr::<u64>(exe::ptr::<u64>(0x1_43a3_70e8) as usize, 10);

        let mut stack_vals = [0u8; 4 * 0x18];
        let sp = stack_vals.as_mut_ptr() as usize;
        if use_vec {
            wr::<f32>(sp, 0.25);
            wr::<f32>(sp + 0x04, 0.5);
            wr::<f32>(sp + 0x08, 0.75);
            wr::<f32>(sp + 0x0c, 1.0);
            wr::<u32>(sp + 0x10, 4);
        } else {
            let vals = [0.1f64, 0.5f64, 0.9f64, 0.4f64];
            for i in 0..nargs {
                wr::<f64>(sp + i * 0x18, vals[i]);
                wr::<u32>(sp + i * 0x18 + 0x10, 3);
            }
        }

        let mut l_state = [0u8; 0x20];
        let lp = l_state.as_mut_ptr() as usize;
        wr::<usize>(lp + 0x08, sp + nargs * 0x18);
        wr::<usize>(lp + 0x10, sp);

        let ret = f(lp);
        let new_ptr = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6df8) as usize);
        let new_rem = rd::<u64>(exe::ptr::<u64>(0x1_43a3_70e8) as usize);
        let mut out8 = [0u8; 8];
        out8.copy_from_slice(&OUT_BUF[..8]);
        (ret, out8, new_ptr.wrapping_sub(out_base), new_rem)
    };

    for (use_vec, nargs) in [(true, 1usize), (false, 3usize), (false, 4usize)] {
        let o = run(f_o, use_vec, nargs);
        let n = run(f_n, use_vec, nargs);
        crate::diff_eq!(o, n, "14233d270 use_vec={} nargs={}", use_vec, nargs);
    }
    Ok(3)
}

pub unsafe fn diff_anonymous_namespace_142394700(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, has_scale: u8| -> (Vec<u8>, Vec<u8>) {
        let mut t38 = vec![0xaa_u8; 0xa0];
        let mut t40 = vec![0xbb_u8; 0xa0];
        let mut p1 = [0u8; 0x50];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<f32>(p1p + 0x18, 3.0);
        wr::<f32>(p1p + 0x1c, 4.0);
        wr::<f32>(p1p + 0x20, 5.0);
        wr::<f32>(p1p + 0x30, 2.5);
        wr::<u8>(p1p + 0x34, has_scale);
        wr::<usize>(p1p + 0x38, t38.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x40, t40.as_mut_ptr() as usize);
        f(p1p);
        (t38, t40)
    };

    for has_scale in [0u8, 1u8] {
        let o = run(f_o, has_scale);
        let n = run(f_n, has_scale);
        crate::diff_eq!(o, n, "142394700 has_scale={}", has_scale);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1423c5090(orig: usize, new: usize) -> Result<u32, String> {
    static mut COPY_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_3290(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        COPY_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_403b_3290, mock_3290 as *const ())?;

    let run = |fn_addr: usize, ptr_val: u64, val_f: f32, init_f: f32| -> (Vec<u8>, u32) {
        COPY_COUNT = 0;
        let mut p1 = vec![0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x40, 0x1111);
        wr::<f32>(p1p + 0x48, init_f);
        let mut p3: u64 = ptr_val;
        let p3_addr = (&mut p3 as *mut u64) as usize as u64;
        let xmm_in = [0, 0, 0, val_f.to_bits() as u128];
        let _ = crate::simd_ops::call_va8_xmm(fn_addr as u64, p1p as u64, 1, p3_addr, 0, 0, 0, 0, 0, xmm_in);
        (p1, COPY_COUNT)
    };

    let cases = [
        (0u64, 1.5f32, 0.5f32),
        (0x1111u64, -1.0f32, 0.5f32),
        (0x1111u64, 0.5f32, 0.5f32),
        (0x1111u64, 2.5f32, 0.5f32),
        (0x2222u64, 0.5f32, 0.5f32),
    ];
    for (pv, vf, inf) in cases {
        let o = run(orig, pv, vf, inf);
        let n = run(new, pv, vf, inf);
        crate::diff_eq!(o, n, "1423c5090 pv={} vf={} inf={}", pv, vf, inf);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1423c7200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43cf_f708, 0x70);
    let _g2 = GlobalSave::new(0x1_4397_fcb8, 8);
    let _g3 = GlobalSave::new(0x1_4385_8590, 16);
    let _g4 = GlobalSave::new(0x1_43a6_37b0, 48);

    let tls0 = rd::<usize>(exe::tls_pointer());
    let _t1 = GlobalSave::from_addr(tls0 + 0x74, 4);
    let _t2 = GlobalSave::from_addr(tls0 + 0x63d0, 0x40);
    let _t3 = GlobalSave::from_addr(tls0 + 0x463e0, 16);

    let run = |f: F, has_tex: bool| -> ([u64; 6], [u64; 6], [i32; 3]) {
        wr::<i32>(tls0 + 0x74, 0);
        wr::<[u8; 0x40]>(tls0 + 0x63d0, [0; 0x40]);
        wr::<[i32; 3]>(tls0 + 0x463e0, [10, -2, 5]);
        for va in [0x1_43cf_f754u64, 0x1_43cf_f750, 0x1_43cf_f708] {
            wr::<i32>(exe::ptr::<i32>(va) as usize, i32::MIN);
        }
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_f760) as usize, 0);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_f718) as usize, 1);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_f738) as usize, 2);
        wr::<[u32; 3]>(exe::ptr::<u32>(0x1_4385_8590) as usize, [0, 5, 2]);

        let base_7b0 = exe::ptr::<i32>(0x1_43a6_37b0) as usize;
        wr::<[i32; 4]>(base_7b0, [0x00, 0, 16, 0]);
        wr::<[i32; 4]>(base_7b0 + 16, [0x10, 0, 16, 1]);
        wr::<[i32; 4]>(base_7b0 + 32, [0x20, 0, 16, 2]);

        let mut def_tex = [0u8; 0xa0];
        let dtp = def_tex.as_mut_ptr() as usize;
        wr::<u64>(dtp + 0x88, 0xdef0_88);
        wr::<u64>(dtp + 0x90, 0xdef0_90);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize, dtp);

        static mut TEX_A: [u8; 0xa0] = [0; 0xa0];
        let tap = core::ptr::addr_of_mut!(TEX_A) as usize;
        wr::<u64>(tap + 0x78, 0xaaaa_78);
        wr::<u64>(tap + 0x90, 0xaaaa_90);

        let t_val = if has_tex { tap } else { 0usize };
        let slots: [usize; 3] = [t_val, t_val, t_val];
        let p1: [usize; 3] = [
            (&slots[0] as *const usize) as usize,
            (&slots[1] as *const usize) as usize,
            (&slots[2] as *const usize) as usize,
        ];

        f(p1.as_ptr() as usize);
        let globals_out = [
            rd::<u64>(exe::ptr::<u64>(0x1_43cf_f768) as usize),
            rd::<u64>(exe::ptr::<u64>(0x1_43cf_f770) as usize),
            rd::<u64>(exe::ptr::<u64>(0x1_43cf_f720) as usize),
            rd::<u64>(exe::ptr::<u64>(0x1_43cf_f728) as usize),
            rd::<u64>(exe::ptr::<u64>(0x1_43cf_f740) as usize),
            rd::<u64>(exe::ptr::<u64>(0x1_43cf_f748) as usize),
        ];
        (globals_out, rd::<[u64; 6]>(tls0 + 0x63d0), rd::<[i32; 3]>(tls0 + 0x463e0))
    };

    for has_tex in [true, false] {
        let o = run(f_o, has_tex);
        let n = run(f_n, has_tex);
        crate::diff_eq!(o, n, "1423c7200 has_tex={}", has_tex);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1423dfe40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43be_60a8, 16);

    static mut LOG_E40: Vec<(u64, u64)> = Vec::new();
    static mut VTBL: [usize; 2] = [0; 2];

    unsafe extern "win64" fn mock_2e60(rcx: usize, rdx: usize) {
        LOG_E40.push((rcx as u64, rd::<u64>(rdx)));
    }
    unsafe extern "win64" fn mock_vfn0(this: usize) {
        LOG_E40.push((0xdead_0, rd::<u32>(this + 0x08) as u64));
    }
    unsafe extern "win64" fn mock_vfn1(this: usize) {
        LOG_E40.push((0xdead_1, rd::<u32>(this + 0x0c) as u64));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4230_2e60, mock_2e60 as *const ())?;

    let run = |f: F, count: u64| -> (Vec<(u64, u64)>, u32, u32, u32, u32) {
        LOG_E40.clear();
        VTBL = [
            mock_vfn0 as *const () as usize,
            mock_vfn1 as *const () as usize,
        ];

        let tbl_addr = exe::ptr::<usize>(0x1_43be_60a8) as usize;
        wr::<usize>(tbl_addr, 0x1000_0000);
        wr::<usize>(tbl_addr + 8, 0x2000_0000);

        let mut rc1 = [0u64; 2];
        let rc1p = rc1.as_mut_ptr() as usize;
        wr::<usize>(rc1p, core::ptr::addr_of!(VTBL) as usize);
        wr::<u32>(rc1p + 0x08, 1);
        wr::<u32>(rc1p + 0x0c, 1);

        let mut rc2 = [0u64; 2];
        let rc2p = rc2.as_mut_ptr() as usize;
        wr::<usize>(rc2p, core::ptr::addr_of!(VTBL) as usize);
        wr::<u32>(rc2p + 0x08, 2);
        wr::<u32>(rc2p + 0x0c, 1);

        let mut arr = [0u8; 2 * 0x18];
        let ap = arr.as_mut_ptr() as usize;
        wr::<u32>(ap, 0x0001_0002);
        wr::<u64>(ap + 0x08, 0xaaaa);
        wr::<usize>(ap + 0x10, rc1p);

        wr::<u32>(ap + 0x18, 0x0000_0003);
        wr::<u64>(ap + 0x20, 0xbbbb);
        wr::<usize>(ap + 0x28, rc2p);

        f(ap, count);
        (
            LOG_E40.clone(),
            rd::<u32>(rc1p + 0x08),
            rd::<u32>(rc1p + 0x0c),
            rd::<u32>(rc2p + 0x08),
            rd::<u32>(rc2p + 0x0c),
        )
    };

    for cnt in [0u64, 2u64] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1423dfe40 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_r_142417470(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let tls0 = rd::<usize>(exe::tls_pointer());
    let _t1 = GlobalSave::from_addr(tls0 + 0xa87f0, 0x20);

    static mut DIST_LOG: (u32, u64) = (0, 0);

    unsafe extern "win64" fn mock_b4a0(rng_ptr_slot: usize, bound: u64) -> u64 {
        let rng_ptr = rd::<usize>(rng_ptr_slot);
        DIST_LOG = (rd::<u32>(rng_ptr), bound);
        2
    }

    let _m1 = crate::difftest::Mock::new(0x1_4139_b4a0, mock_b4a0 as *const ())?;

    let run = |f: F, init_val: u64| -> (u64, u32, u64, u64, (u32, u64)) {
        DIST_LOG = (0, 0);
        wr::<u32>(tls0 + 0xa87f0, 2);
        wr::<u64>(tls0 + 0xa87f8, 0x999);
        wr::<u64>(tls0 + 0xa8800, 0);
        wr::<u32>(tls0 + 0xa8808, 0x1234_5678);

        let mut p1: u64 = init_val;
        f((&mut p1 as *mut u64) as usize);
        (
            p1,
            rd::<u32>(tls0 + 0xa87f0),
            rd::<u64>(tls0 + 0xa87f8),
            rd::<u64>(tls0 + 0xa8800),
            DIST_LOG,
        )
    };

    for init_val in [100u64, 3000u64] {
        let o = run(f_o, init_val);
        let n = run(f_n, init_val);
        crate::diff_eq!(o, n, "142417470 init_val={}", init_val);
    }
    Ok(2)
}

pub unsafe fn diff_r_14241d6f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43a0_82b8, 8);

    static mut LOG_D6F0: Vec<u32> = Vec::new();
    static mut VTBL_P2: [usize; 12] = [0; 12];
    static mut G_2B8_MEM: [u8; 0x200] = [0; 0x200];

    unsafe extern "win64" fn mock_e210(_dst: usize, _src: usize) {
        LOG_D6F0.push(1);
    }
    unsafe extern "win64" fn mock_0dd0(_mgr: usize, _p2: usize) {
        LOG_D6F0.push(2);
    }
    unsafe extern "win64" fn mock_vfn40(_this: usize) -> i32 {
        LOG_D6F0.push(3);
        1
    }
    unsafe extern "win64" fn mock_vfn50(_this: usize) {
        LOG_D6F0.push(4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4139_e210, mock_e210 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4242_0dd0, mock_0dd0 as *const ())?;

    let run = |f: F, ref_cnt: u32| -> (Vec<u32>, u32, u32) {
        LOG_D6F0.clear();
        VTBL_P2 = [0; 12];
        VTBL_P2[8] = mock_vfn40 as *const () as usize;
        VTBL_P2[10] = mock_vfn50 as *const () as usize;
        G_2B8_MEM = [0; 0x200];

        let gp = core::ptr::addr_of_mut!(G_2B8_MEM) as usize;
        wr::<u32>(gp + 0xb0 + 0x40 + 0x38, u32::MAX);
        wr::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize, gp);

        let mut p1 = [0u8; 0x160];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x148, u32::MAX);

        let mut p2 = [0u8; 0x90];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, core::ptr::addr_of!(VTBL_P2) as usize);
        wr::<u32>(p2p + 0x24, ref_cnt);
        wr::<usize>(p2p + 0x58, 0);
        wr::<usize>(p2p + 0x78, 0);

        f(p1p, p2p);
        (
            LOG_D6F0.clone(),
            rd::<u32>(p1p + 0x148),
            rd::<u32>(p1p + 0x14c),
        )
    };

    for ref_cnt in [0u32, 1u32] {
        let o = run(f_o, ref_cnt);
        let n = run(f_n, ref_cnt);
        crate::diff_eq!(o, n, "14241d6f0 ref_cnt={}", ref_cnt);
    }
    Ok(2)
}

pub unsafe fn diff_r_1424282e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_3560(_rcx: usize, key_ptr: usize) -> u64 {
        rd::<u64>(key_ptr).wrapping_mul(0x105)
    }
    unsafe extern "win64" fn mock_3610(_table: usize, hash: u64) -> usize {
        (hash & 0x0f) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_3560, mock_3560 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_3610, mock_3610 as *const ())?;

    let run = |f: F, pre_present: bool| -> ([u64; 16], u8) {
        let mut ctrl = [0x80u8; 32];
        let mut keys = [0u64; 16];
        let target_key: u64 = 3;
        let hash = target_key.wrapping_mul(0x105);
        let h2 = (hash as u8) & 0x7f;
        if pre_present {
            ctrl[2] = h2;
            keys[2] = target_key;
        }

        let mut p1 = [0u8; 0xb0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x88, ctrl.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x90, keys.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0xa0, 0x0f);

        f(p1p, target_key);
        (keys, rd::<u8>(p1p + 0x84))
    };

    for pre_present in [false, true] {
        let o = run(f_o, pre_present);
        let n = run(f_n, pre_present);
        crate::diff_eq!(o, n, "1424282e0 pre_present={}", pre_present);
    }
    Ok(2)
}

pub unsafe fn diff_r_14242dcc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let tls0 = rd::<usize>(exe::tls_pointer());
    let _t1 = GlobalSave::from_addr(tls0 + 0x6348, 8);

    static mut BLOCK_A: [u8; 0x10000] = [0; 0x10000];
    static mut BLOCK_B: [u8; 0x10000] = [0; 0x10000];
    static mut LOG_DCC0: Vec<u32> = Vec::new();

    unsafe extern "win64" fn mock_5070(_ctx: usize, _sz: u32, _al: u32) -> usize {
        LOG_DCC0.push(1);
        core::ptr::addr_of_mut!(BLOCK_A) as usize
    }
    unsafe extern "win64" fn mock_2de0(_p: usize) {
        LOG_DCC0.push(2);
    }
    unsafe extern "win64" fn mock_4120(out: usize, _used: u32, tag: u32) -> usize {
        LOG_DCC0.push(3);
        let hdr = (core::ptr::addr_of_mut!(BLOCK_B) as usize) + 0xffe8;
        wr::<u64>(hdr, 0x7a63_f007);
        wr::<u32>(hdr + 0x08, tag);
        wr::<u32>(hdr + 0x0c, 0);
        wr::<u64>(hdr + 0x10, 0);
        wr::<usize>(out, hdr);
        out
    }
    unsafe extern "win64" fn mock_2140(dst_10: usize, src_out: usize) {
        LOG_DCC0.push(4);
        wr::<usize>(dst_10, rd::<usize>(src_out));
        wr::<usize>(src_out, 0);
    }
    unsafe extern "win64" fn mock_17d0(_p: usize) {
        LOG_DCC0.push(5);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413a_4120, mock_4120 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_413a_2140, mock_2140 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_17d0, mock_17d0 as *const ())?;

    let run = |f: F, mode: u32| -> (usize, u32, Vec<u32>) {
        LOG_DCC0.clear();
        BLOCK_A.fill(0);
        BLOCK_B.fill(0);
        wr::<usize>(tls0 + 0x6348, 0x5555);

        let base_a = core::ptr::addr_of_mut!(BLOCK_A) as usize;
        let hdr_a = base_a + 0xffe8;
        let mut p1 = [0usize; 3];
        p1[2] = 77;
        if mode == 1 {
            wr::<u32>(hdr_a + 0x0c, 0x100);
            p1[0] = hdr_a;
            p1[1] = hdr_a;
        } else if mode == 2 {
            wr::<u32>(hdr_a + 0x0c, 0xffe0);
            p1[0] = hdr_a;
            p1[1] = hdr_a;
        }

        let ret = f(p1.as_mut_ptr() as usize, 0x20);
        let cur_hdr = p1[1];
        let cur_base = cur_hdr - 0xffe8;
        (ret - cur_base, rd::<u32>(cur_hdr + 0x0c), LOG_DCC0.clone())
    };

    for mode in [0u32, 1u32, 2u32] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14242dcc0 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_r_14242df10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_DF10: Vec<usize> = Vec::new();

    unsafe extern "win64" fn mock_17d0(p: usize) {
        LOG_DF10.push(p);
    }
    unsafe extern "win64" fn mock_2de0(p: usize) {
        LOG_DF10.push(p);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_17d0, mock_17d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let run = |f: F, p2: usize| -> Vec<usize> {
        LOG_DF10.clear();
        f(0, p2);
        LOG_DF10.clone()
    };

    for p2 in [0usize, 0x20000usize] {
        let o = run(f_o, p2);
        let n = run(f_n, p2);
        crate::diff_eq!(o, n, "14242df10 p2={:#x}", p2);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14243bd60(orig: usize, new: usize) -> Result<u32, String> {
    let x = 1.25f32.to_bits() as u128;
    let y = (-2.5f32).to_bits() as u128;
    let z = 3.75f32.to_bits() as u128;

    for h in 0..16u64 {
        let (_, xo) = crate::simd_ops::call_va8_xmm(orig as u64, h, 0, 0, 0, 0, 0, 0, 0, [0, x, y, z]);
        let (_, xn) = crate::simd_ops::call_va8_xmm(new as u64, h, 0, 0, 0, 0, 0, 0, 0, [0, x, y, z]);
        crate::diff_eq!(xo as u32, xn as u32, "14243bd60 h={}", h);
    }
    Ok(16)
}

pub unsafe fn diff_unknown_14243be50(orig: usize, new: usize) -> Result<u32, String> {
    let inputs: [f32; 7] = [-2.3, -0.5, 0.0, 0.37, 1.0, 4.82, 12.34];
    for &v in &inputs {
        let xb = v.to_bits() as u128;
        let (_, xo) = crate::simd_ops::call_va8_xmm(orig as u64, 0, 0, 0, 0, 0, 0, 0, 0, [xb, 0, 0, 0]);
        let (_, xn) = crate::simd_ops::call_va8_xmm(new as u64, 0, 0, 0, 0, 0, 0, 0, 0, [xb, 0, 0, 0]);
        crate::diff_eq!(xo as u32, xn as u32, "14243be50 v={}", v);
    }
    Ok(inputs.len() as u32)
}

pub unsafe fn diff_unknown_1424858a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODE_MEM: [u8; 0x30] = [0; 0x30];
    static mut ALLOC_ARG: usize = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_5aa0(rcx: usize, rdx: usize) {
        ALLOC_ARG = rcx;
        wr::<[u64; 2]>(rdx, [core::ptr::addr_of_mut!(NODE_MEM) as usize as u64, 0]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;

    let run = |f: F, init_state_byte: u8| -> ([u8; 0x30], [u8; 16], usize) {
        NODE_MEM = [0; 0x30];
        NODE_MEM[4] = init_state_byte;
        ALLOC_ARG = 0;

        let mut worker_tbl = [0u8; 0x100];
        let wtp = worker_tbl.as_mut_ptr() as usize;
        wr::<usize>(wtp + 0x40 + 0x48, 0x3000);

        let mut p1 = [0u8; 0x20];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x10, wtp);

        let mut p2 = [0u8; 16];
        let p2p = p2.as_mut_ptr() as usize;
        let p3: [u64; 2] = [0x1122_3344_5566_7788, 0x99aa_bbcc_ddee_ff00];

        let ret = f(p1p, p2p, p3.as_ptr() as usize);
        assert_eq!(ret, p2p);
        (NODE_MEM, p2, ALLOC_ARG)
    };

    for state_b in [0u8, 1u8] {
        let o = run(f_o, state_b);
        let n = run(f_n, state_b);
        crate::diff_eq!(o, n, "1424858a0 state_b={}", state_b);
    }
    Ok(2)
}
