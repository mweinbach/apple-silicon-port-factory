//! Decompiled `rend` hot functions shard ZA11 (`0x141e9a600..0x141f596e0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline]
#[allow(clippy::too_many_arguments)]
unsafe fn call_1421650f0(
    a1: usize,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: usize,
    a9: usize,
    a10: u64,
    a11: u64,
    a12: usize,
    a13: usize,
    a14: usize,
) {
    // Only safe on x86_64: on aarch64 a direct Rust call cannot pass stack
    // args 9+ to a lifted function (they arrive via `emu::set_outgoing_rsp`),
    // so it must take the `call_va8` path below. (The target, `t37::fn_1421650f0`,
    // is lifted; revisit if a full-signature Rust decompilation replaces it.)
    if let Some(addr) = exe::lookup_native_va(0x1_4216_50f0).filter(|_| cfg!(target_arch = "x86_64")) {
        let f: unsafe extern "win64" fn(
            usize,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            usize,
            usize,
            u64,
            u64,
            usize,
            usize,
            usize,
        ) -> u64 = core::mem::transmute(addr);
        let _ = f(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14);
        return;
    }
    #[cfg(target_arch = "x86_64")]
    {
        let addr = exe::to_mapped_addr(0x1_4216_50f0);
        let f: unsafe extern "win64" fn(
            usize,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            usize,
            usize,
            u64,
            u64,
            usize,
            usize,
            usize,
        ) -> u64 = core::mem::transmute(addr);
        let _ = f(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let buf: [u64; 16] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            a9 as u64,
            a10,
            a11,
            a12 as u64,
            a13 as u64,
            a14 as u64,
            0,
            0,
        ];
        let base = buf.as_ptr() as u64;
        crate::emu::set_outgoing_rsp(base, base + 128);
        let _ = crate::emu::call_va8(
            0x1_4216_50f0,
            a1 as u64,
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a8 as u64,
        );
    }
}

/// `unknown_141e9a600`
/// Original: 0x141e9a600 (1456 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e9a600(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let p2_00 = rd::<usize>(param_2);
    let p2_08 = rd::<usize>(param_2 + 0x08);
    let p2_10 = rd::<usize>(param_2 + 0x10);
    let p2_18 = rd::<usize>(param_2 + 0x18);
    let p2_20 = rd::<usize>(param_2 + 0x20);
    let p2_28 = rd::<usize>(param_2 + 0x28);
    let p2_48 = rd::<usize>(param_2 + 0x48);
    let idx = rd::<u64>(param_2 + 0x58) as usize;

    let off_30 = idx.wrapping_mul(0x30);
    let off_20 = idx.wrapping_shl(5);
    let ptr_f8 = p2_28.wrapping_add(idx.wrapping_shl(4));

    let g_f63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
    let ptr_110 = rd::<usize>(rd::<usize>(param_3 + 0x580e0).wrapping_add(g_f63c * 8));

    let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let rdi_ctx = param_3.wrapping_add(0x580f0).wrapping_add(w_idx * 0x188);

    let state_addr = p2_00.wrapping_add(off_30);
    if rd::<u8>(state_addr) == 0 {
        wr::<u8>(state_addr, 1);
        let src_32 = rd::<[u8; 32]>(p2_10.wrapping_add(off_20));
        wr::<[u8; 32]>(state_addr + 0x10, src_32);
    } else {
        let a_lo = rd::<u128>(p2_10.wrapping_add(off_20));
        let a_hi = rd::<u128>(p2_10.wrapping_add(off_20 + 0x10));
        let b_lo = rd::<u128>(state_addr + 0x10);
        let b_hi = rd::<u128>(state_addr + 0x20);
        let cmp_lo = crate::simd_ops::vcmpps(a_lo, b_lo, 0);
        let cmp_hi = crate::simd_ops::vcmpps(b_hi, a_hi, 0);
        let mask = crate::simd_ops::vmovmskps(cmp_lo & cmp_hi);
        if mask != 0xf {
            wr::<u8>(p2_48.wrapping_add(idx), 1);
        }
    }

    let p2_40 = rd::<usize>(param_2 + 0x40);
    let r15_handle = if p2_40 != 0 {
        let cur_idx = rd::<u64>(param_2 + 0x58) as usize;
        rd::<usize>(p2_40.wrapping_add(cur_idx * 8))
    } else {
        let mut out_258: usize = 0;
        let cur_idx = rd::<u64>(param_2 + 0x58) as usize;
        let arr_50 = rd::<usize>(param_2 + 0x50);
        let key_val = rd::<u64>(arr_50.wrapping_add(cur_idx * 8 + 0x10));
        let mut key_d0: [u64; 2] = [key_val, 0];
        let slot_ptr = exe::call_va!(
            0x1_402d_3900,
            (
                rdi_ctx,
                key_d0.as_mut_ptr() as usize,
                (&mut out_258 as *mut usize) as usize
            ) -> usize
        );

        let rbx_freed = out_258;
        if rbx_freed != 0 {
            let p_d0 = rd::<usize>(rbx_freed + 0xd0);
            if p_d0 != 0 {
                exe::call_va!(0x1_4244_2de0, (p_d0));
            }
            let p_88 = rd::<usize>(rbx_freed + 0x88);
            if p_88 != 0 {
                exe::call_va!(0x1_4244_2de0, (p_88));
            }
            let seg_base = rbx_freed.wrapping_sub(1) & !0x01ff_ffffusize;
            let same_thread = exe::teb() == rd::<usize>(seg_base + 0x100);
            let r9_ptr = out_258;
            let seg_idx = (r9_ptr.wrapping_sub(seg_base)) >> 16;
            let mut rcx_hdr = seg_base.wrapping_add(0x108).wrapping_add(seg_idx.wrapping_mul(0x50));
            let back_off = rd::<u32>(rcx_hdr + 0x04) as usize;
            rcx_hdr = rcx_hdr.wrapping_sub(back_off);
            if same_thread {
                if rd::<u8>(rcx_hdr + 0x0e) == 0 {
                    wr::<u64>(r9_ptr, rd::<u64>(rcx_hdr + 0x20));
                    wr::<usize>(rcx_hdr + 0x20, r9_ptr);
                    let prev = rd::<u32>(rcx_hdr + 0x18).wrapping_sub(1);
                    wr::<u32>(rcx_hdr + 0x18, prev);
                    if prev == 0 {
                        exe::call_va!(0x1_4253_71a0, (rcx_hdr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (seg_base, rcx_hdr, 1u8, r9_ptr));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (seg_base, rcx_hdr, 0u8, r9_ptr));
            }
        }

        let new_h = exe::call_va!(0x1_4028_b4a0, () -> usize);
        let old_slot = rd::<usize>(slot_ptr);
        wr::<usize>(slot_ptr, new_h);
        if old_slot != 0 {
            let p_d0 = rd::<usize>(old_slot + 0xd0);
            if p_d0 != 0 {
                exe::call_va!(0x1_4244_2de0, (p_d0));
            }
            let p_88 = rd::<usize>(old_slot + 0x88);
            if p_88 != 0 {
                exe::call_va!(0x1_4244_2de0, (p_88));
            }
            exe::call_va!(0x1_4244_2de0, (old_slot));
        }
        new_h
    };

    let rbx_obj = exe::call_va!(0x1_4100_8330, (0usize, 0usize, r15_handle) -> usize);
    let rdx_ref = rd::<usize>(param_1);
    let mut buf_88 = [0u8; 0x20];
    let buf_88_ptr = buf_88.as_mut_ptr() as usize;
    wr::<usize>(buf_88_ptr, rbx_obj);
    wr::<usize>(buf_88_ptr + 0x08, rdx_ref);
    wr::<u32>(buf_88_ptr + 0x10, 0);
    wr::<u16>(buf_88_ptr + 0x16, 1);
    wr::<u8>(buf_88_ptr + 0x18, 0);

    if rdx_ref != 0 {
        let old = crate::simd_ops::atomic_xadd_u32((rdx_ref + 0x24) as u64, 1);
        if old.wrapping_add(1) == 1 {
            let owner = rd::<usize>(rdx_ref + 0x28);
            if owner != 0 {
                exe::call_va!(0x1_4241_d6f0, (owner, rdx_ref));
            }
        }
    }

    let r15_arr50 = rd::<usize>(param_2 + 0x50);
    let r14_idx58 = rd::<u64>(param_2 + 0x58) as usize;
    let key_120 = rd::<u64>(r15_arr50.wrapping_add(r14_idx58 * 8 + 0x10));
    let mut buf_120: [u64; 2] = [key_120, 0];
    let rdi_entry = exe::call_va!(
        0x1_4100_c160,
        (rdi_ctx, buf_120.as_mut_ptr() as usize, buf_88_ptr) -> usize
    );

    let rdx_after = rd::<usize>(buf_88_ptr + 0x08);
    if rdx_after != 0 {
        let old = crate::simd_ops::atomic_xadd_u32((rdx_after + 0x24) as u64, u32::MAX);
        if old == 1 {
            let rdx_reload = rd::<usize>(buf_88_ptr + 0x08);
            let owner = rd::<usize>(rdx_reload + 0x28);
            if owner != 0 {
                exe::call_va!(0x1_4241_d6f0, (owner, rdx_reload));
            }
        }
    }
    exe::call_va!(0x1_4027_3690, (buf_88_ptr));

    let word_val = rd::<u16>(p2_18.wrapping_add(idx * 2));
    wr::<u16>(rdi_entry + 0x14, word_val);

    let p2_38 = rd::<usize>(param_2 + 0x38);
    let arg6_rcx = if p2_38 != 0 {
        rd::<u64>(p2_38.wrapping_add(r14_idx58 * 8))
    } else {
        0
    };
    let raw_arg7 = rd::<u64>(r15_arr50.wrapping_add(r14_idx58 * 8 + 0x10));

    let mut buf_b0: [u64; 3] = [rd::<u64>(ptr_f8), rd::<u64>(ptr_f8 + 0x08), 1];
    let item_20 = p2_20.wrapping_add(off_30);
    let mut buf_d0_vec: [u32; 4] = [
        rd::<u32>(item_20 + 0x20),
        rd::<u32>(item_20 + 0x24),
        rd::<u32>(item_20 + 0x28),
        0,
    ];
    let mut buf_180: [u64; 5] = [
        rd::<u32>(item_20 + 0x08) as u64,
        rd::<u64>(item_20),
        rd::<u32>(item_20 + 0x18) as u64,
        rd::<u64>(item_20 + 0x10),
        1,
    ];
    let mut buf_130 = rd::<[u8; 32]>(p2_08.wrapping_add(off_20));
    let mut buf_150 = rd::<[u8; 32]>(state_addr + 0x10);

    let r9d = rd::<u32>(param_1 + 0x18);
    let arg7_rax = if r9d == 1 { 0 } else { raw_arg7 };

    call_1421650f0(
        rbx_obj,
        rd::<u64>(ptr_110),
        rd::<u64>(param_1 + 0x08),
        rd::<u64>(param_1 + 0x10),
        r9d as u64,
        arg6_rcx,
        arg7_rax,
        buf_150.as_mut_ptr() as usize,
        buf_130.as_mut_ptr() as usize,
        word_val as u64,
        0,
        buf_180.as_mut_ptr() as usize,
        buf_d0_vec.as_mut_ptr() as usize,
        buf_b0.as_mut_ptr() as usize,
    );

    let p2_30 = rd::<usize>(param_2 + 0x30);
    let rcx_30 = if p2_30 == 0 {
        0
    } else {
        p2_30.wrapping_add(r14_idx58 * 8)
    };
    if rcx_30 != 0 && ((rd::<u32>(rcx_30) >> 7) & 1) != 0 {
        wr::<u8>(rbx_obj + 0x114, rd::<u8>(rbx_obj + 0x114) & !0x10);
    }
}

/// `unknown_141eb29c0`
/// Original: 0x141eb29c0 (373 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eb29c0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    use crate::simd_ops::{vaddps, vblendps, vmulps, vpermilps, vshufps, vsubps};

    let xmm1_in = rd::<u128>(param_3);
    let xmm5_two = rd::<u128>(exe::ptr::<u128>(0x1_4342_83b0) as usize);
    let xmm8_one = rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize);

    let xmm0 = vmulps(xmm5_two, xmm1_in);
    let xmm4 = vmulps(xmm0, xmm1_in);
    let xmm0 = vpermilps(xmm1_in, 0xe0);
    let xmm2 = vmulps(xmm0, xmm5_two);
    let xmm3 = xmm1_in;
    let xmm1 = vpermilps(xmm1_in, 0xd9);
    let xmm0 = vpermilps(xmm3, 0xff);
    let xmm7 = vmulps(xmm2, xmm1);
    let xmm1 = vmulps(xmm5_two, xmm3);
    let xmm5 = vmulps(xmm1, xmm0);
    let xmm0 = vpermilps(xmm4, 0xc1);
    let xmm1 = vpermilps(xmm4, 0xda);
    let xmm3 = vpermilps(xmm5, 0xd2);
    let xmm2 = vsubps(xmm8_one, xmm0);
    let xmm6 = vsubps(xmm2, xmm1);
    let xmm1 = vpermilps(xmm5, 0xc9);
    let xmm2 = 0u128;
    let xmm0 = vpermilps(xmm7, 0xd8);
    let xmm4 = vaddps(xmm3, xmm0);
    let xmm5 = vshufps(xmm6, xmm4, 0x44);
    let xmm0 = vpermilps(xmm7, 0xe1);
    let xmm3 = vsubps(xmm0, xmm1);
    let xmm1 = vshufps(xmm3, xmm2, 0xee);
    let xmm0 = vshufps(xmm3, xmm2, 0x44);
    let xmm2 = vshufps(xmm5, xmm0, 0x88);
    let xmm0 = vshufps(xmm5, xmm0, 0xdd);
    let xmm5 = vpermilps(xmm0, 0xd2);
    let xmm4 = vshufps(xmm6, xmm4, 0xee);
    let xmm0 = vshufps(xmm4, xmm1, 0x88);
    let xmm1 = vblendps(xmm8_one, rd::<u128>(param_3 + 0x10), 0x7);
    let xmm3 = vpermilps(xmm0, 0xc9);
    let xmm4 = vshufps(xmm5, xmm3, 0x49);
    let xmm0 = vshufps(xmm2, xmm5, 0x4e);
    let xmm2 = vshufps(xmm2, xmm0, 0x84);
    let xmm0 = vshufps(xmm3, xmm1, 0x4e);
    let xmm5 = vshufps(xmm0, xmm1, 0x98);

    let mut mat_20: [u128; 3] = [xmm2, xmm4, xmm5];
    exe::call_va!(0x1_4103_8eb0, (param_1 + 0x10, mat_20.as_mut_ptr() as usize));

    let copy_32 = rd::<[u8; 32]>(param_3);
    wr::<[u8; 32]>(param_2, copy_32);
}

/// `unknown_141f2d380`
/// Original: 0x141f2d380 (148 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f2d380(param_1: usize, param_2: usize) {
    let arr0 = rd::<usize>(param_1);
    let idx = rd::<u64>(param_1 + 0x10) as usize;
    let handle = rd::<u64>(arr0.wrapping_add(idx * 8));
    let h_lo = handle as u32;
    let h_hi = (handle >> 32) as u32;

    let table = rd::<usize>(param_2 + 0x18);
    let cap = rd::<u32>(param_2 + 0x20);
    if h_lo < cap {
        let entry = rd::<u64>(table.wrapping_add((h_lo as usize) * 8));
        if (entry as u32) == h_lo && ((entry >> 32) as u32) == h_hi && h_lo != u32::MAX {
            return;
        }
    }

    let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let rcx_ctx = param_2.wrapping_add(0x580f0).wrapping_add(w_idx * 0x188);
    let idx2 = rd::<u64>(param_1 + 0x10) as usize;
    let arr1 = rd::<usize>(param_1 + 0x08);
    let arg2 = rd::<u64>(arr1.wrapping_add(idx2 * 8 + 0x10));
    exe::call_va!(0x1_4034_8930, (rcx_ctx, arg2));
}

/// `unknown_141f59270`
/// Original: 0x141f59270 (493 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f59270(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) {
    let mut rsi_buf: usize = 0;
    let mut edi_len: u32 = 0;
    let mut r14_cap: u32 = 0;
    let mut bp: u16 = 0;

    if bp < rd::<u16>(param_2 + 0x08) {
        loop {
            let batch_ptr = rd::<usize>(param_2).wrapping_add((bp as usize) * 0x28);
            let done = exe::call_va!(
                0x1_41f5_96e0,
                (param_1, batch_ptr, param_4, param_5, param_6) -> u8
            );
            if done == 1 {
                let cur_end = rsi_buf.wrapping_add((edi_len as usize) * 2);
                let cap_end = rsi_buf.wrapping_add((r14_cap as usize) * 2);
                if cur_end == cap_end {
                    let old_bytes = cur_end.wrapping_sub(rsi_buf);
                    edi_len = ((old_bytes as isize) >> 1) as u32;
                    let min_cap = ((old_bytes as isize) >> 1).wrapping_add(1) as usize;
                    let old_cap = ((cap_end.wrapping_sub(rsi_buf) as isize) >> 1) as usize;
                    let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
                    if new_cap < min_cap {
                        new_cap = min_cap;
                    }
                    r14_cap = new_cap as u32;
                    let new_ptr = exe::call_va!(0x1_4241_c0a0, (new_cap.wrapping_mul(2), 2u32) -> usize);
                    exe::call_va!(0x1_4183_acc8, (new_ptr, rsi_buf, old_bytes));
                    if rsi_buf != 0 {
                        exe::call_va!(0x1_4241_c190, (rsi_buf));
                    }
                    rsi_buf = new_ptr;
                }
                wr::<u16>(rsi_buf.wrapping_add((edi_len as usize) * 2), bp);
                edi_len = edi_len.wrapping_add(1);
            }
            bp = bp.wrapping_add(1);
            if bp >= rd::<u16>(param_2 + 0x08) {
                break;
            }
        }
    }

    let end_ptr = rsi_buf.wrapping_add((edi_len as usize) * 2);
    let count = (end_ptr.wrapping_sub(rsi_buf) as isize) >> 1;
    let mut local_88: usize = rsi_buf;
    let mut local_90: usize = end_ptr;
    exe::call_va!(
        0x1_40fa_5d90,
        (
            (&mut local_88 as *mut usize) as usize,
            (&mut local_90 as *mut usize) as usize,
            count as usize,
            0u32
        )
    );

    let mut ebx = (count as i32).wrapping_sub(1);
    if ebx >= 0 {
        let mut rdi_ptr = rsi_buf.wrapping_add((ebx as isize as usize) * 2);
        loop {
            let idx = rd::<u16>(rdi_ptr) as usize;
            exe::call_va!(
                0x1_410d_3ee0,
                (
                    param_2,
                    (&mut local_90 as *mut usize) as usize,
                    idx,
                    idx + 1
                )
            );
            rdi_ptr = rdi_ptr.wrapping_sub(2);
            ebx -= 1;
            if ebx < 0 {
                break;
            }
        }
    }

    if rsi_buf != 0 {
        exe::call_va!(0x1_4241_c190, (rsi_buf));
    }
}

/// `unknown_141f594f0`
/// Original: 0x141f594f0 (484 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f594f0(
    _param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut r14_len: u32 = 0;
    let mut r15_buf: usize = 0;
    let mut r12_cap: usize = 0;

    let init_cnt = rd::<u64>(param_2) as usize;
    if init_cnt != 0 {
        r12_cap = init_cnt;
        r15_buf = exe::call_va!(0x1_4241_c0a0, (init_cnt.wrapping_mul(8), 4u32) -> usize);
    }

    let mut rbx_cur = rd::<usize>(param_2 + 0x08);
    let end_in = rbx_cur.wrapping_add(init_cnt.wrapping_shl(5));
    while rbx_cur != end_in {
        let mut key_70: u64 = rd::<u64>(rbx_cur);
        let mut idx_20: i64 = 0;
        let idx = if rd::<u64>(param_3 + 0x38) == rd::<u64>(param_3 + 0x30) {
            rd::<u64>(param_3 + 0x40)
        } else {
            exe::call_va!(
                0x1_401f_a730,
                (
                    param_3,
                    (&mut idx_20 as *mut i64) as usize,
                    (&mut key_70 as *mut u64) as usize
                )
            );
            if idx_20 == -1 {
                rd::<u64>(param_3 + 0x40)
            } else {
                idx_20 as u64
            }
        };

        let base_58 = rd::<usize>(param_3 + 0x58);
        let entry_ptr = base_58.wrapping_add((idx as usize).wrapping_shl(4));
        let end_entry = base_58.wrapping_add((rd::<u64>(param_3 + 0x40) as usize).wrapping_shl(4));
        if entry_ptr != end_entry {
            let val = rd::<u64>(entry_ptr + 0x08);
            if (val as u32) != u32::MAX {
                let cur_bytes = (r14_len as usize).wrapping_mul(8);
                let cap_bytes = (r12_cap as u32 as usize).wrapping_mul(8);
                if cur_bytes == cap_bytes {
                    let min_cap = (r14_len as usize).wrapping_add(1);
                    let old_cap = r12_cap as u32 as usize;
                    let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
                    if new_cap < min_cap {
                        new_cap = min_cap;
                    }
                    r12_cap = new_cap;
                    let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap.wrapping_mul(8), 4u32) -> usize);
                    exe::call_va!(0x1_4183_acc8, (new_buf, r15_buf, cur_bytes));
                    if r15_buf != 0 {
                        exe::call_va!(0x1_4241_c190, (r15_buf));
                    }
                    r15_buf = new_buf;
                }
                wr::<u64>(r15_buf.wrapping_add((r14_len as usize) * 8), val);
                r14_len = r14_len.wrapping_add(1);
            }
        }
        rbx_cur = rbx_cur.wrapping_add(0x20);
    }

    let n = r14_len as usize;
    let end_buf = r15_buf.wrapping_add(n * 8);
    let mut cur = r15_buf;
    while cur != end_buf {
        let ebp = rd::<u32>(cur);
        let chunk = rd::<usize>(param_4 + 0x80);
        if chunk == 0 || rd::<u64>(chunk + 0x08).wrapping_add(1) > 0x4000 {
            exe::call_va!(0x1_4020_e8f0, (param_4 + 0x78));
        }
        let chunk = rd::<usize>(param_4 + 0x80);
        let slot_idx = rd::<u64>(chunk + 0x08) as usize;
        wr::<u32>(chunk.wrapping_add(0x10 + slot_idx * 4), ebp);
        let chunk = rd::<usize>(param_4 + 0x80);
        wr::<u64>(chunk + 0x08, rd::<u64>(chunk + 0x08).wrapping_add(1));
        wr::<u64>(param_4 + 0x88, rd::<u64>(param_4 + 0x88).wrapping_add(1));
        cur = cur.wrapping_add(8);
    }

    wr::<u64>(param_4 + 0x60, rd::<u64>(param_4 + 0x60).wrapping_add(n as u64));
    if n != 0 {
        wr::<u64>(param_4 + 0x58, rd::<u64>(end_buf - 8));
    }
    if r15_buf != 0 {
        exe::call_va!(0x1_4241_c190, (r15_buf));
    }
}

/// `unknown_141f596e0`
/// Original: 0x141f596e0 (2940 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f596e0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) -> u8 {
    let g_d08c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d08c) as usize) as usize;
    let r15_map = rd::<usize>(rd::<usize>(param_3 + 0x580e0).wrapping_add(g_d08c * 8));

    if exe::call_va!(0x1_4021_7df0, (param_2) -> usize) == 0 {
        return 1;
    }

    let state_1c = rd::<u32>(param_2 + 0x1c);
    let mut run_state_1 = false;
    if state_1c == 0 {
        let mode_20 = rd::<u32>(param_2 + 0x20);
        if mode_20 != 1 && mode_20 != 3 {
            let bl = rd::<u8>(param_2 + 0x18);
            let mut buf_100 = [0u64; 10];
            let rax_str = exe::call_va!(0x1_4022_7100, (buf_100.as_mut_ptr() as usize, param_2) -> usize);
            let has_19 = exe::call_va!(0x1_41f5_b080, (rax_str, bl, param_3, r15_map) -> u8);
            wr::<u8>(param_2 + 0x19, has_19);
            wr::<u32>(param_2 + 0x24, 0);
            wr::<u32>(param_2 + 0x1c, 1);
            run_state_1 = true;
        }
    } else if state_1c == 1 {
        run_state_1 = true;
    }

    if run_state_1 {
        if rd::<u8>(param_2 + 0x19) != 0 {
            let mut esi: u32 = 0;
            let bd0 = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
            if rd::<u32>(bd0 + 0x08) > 0 {
                loop {
                    let bd = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
                    let r8_id = rd::<u64>(rd::<usize>(bd).wrapping_add((esi as usize) << 5));
                    if (r8_id & 0x3fff) != 0 && (r8_id & 0x3fff_ffff_ffff_c000) != 0 {
                        let mut out_m20: u64 = 0;
                        exe::call_va!(
                            0x1_401f_9650,
                            (r15_map, (&mut out_m20 as *mut u64) as usize, r8_id)
                        );
                        if (out_m20 as u32) != u32::MAX {
                            let ok = exe::call_va!(0x1_40f3_0800, (param_3, out_m20) -> u8);
                            if ok == 0 {
                                return 0;
                            }
                        }
                    }
                    esi = esi.wrapping_add(1);
                    let bd_next = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
                    if (esi as usize) >= (rd::<u32>(bd_next + 0x08) as usize) {
                        break;
                    }
                }
            }
        }

        let cnt_24 = rd::<i32>(param_2 + 0x24);
        if cnt_24 < 5 {
            wr::<i32>(param_2 + 0x24, cnt_24 + 1);
            let bd = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
            let mut rbx_ptr = rd::<usize>(bd + 0x10);
            let end_ptr = rbx_ptr.wrapping_add((rd::<u32>(bd + 0x18) as usize) << 5);
            while rbx_ptr != end_ptr {
                let r8_id = rd::<u64>(rbx_ptr);
                if (r8_id & 0x3fff) != 0 && (r8_id & 0x3fff_ffff_ffff_c000) != 0 {
                    let mut out_m18: u64 = 0;
                    exe::call_va!(
                        0x1_401f_9650,
                        (r15_map, (&mut out_m18 as *mut u64) as usize, r8_id)
                    );
                    if (out_m18 as u32) == u32::MAX {
                        return 0;
                    }
                }
                rbx_ptr = rbx_ptr.wrapping_add(0x20);
            }
        }

        let bd = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
        let mut rsi_ptr = rd::<usize>(bd + 0x10);
        let end_ptr = rsi_ptr.wrapping_add((rd::<u32>(bd + 0x18) as usize) << 5);
        while rsi_ptr != end_ptr {
            let rbx_id = rd::<u64>(rsi_ptr);
            let valid = (rbx_id & 0x3fff) != 0 && (rbx_id & 0x3fff_ffff_ffff_c000) != 0;
            if valid {
                let g_aec8 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_aec8) as usize) as usize;
                let map_20 = rd::<usize>(rd::<usize>(param_3) + 0x20);
                let slot = rd::<u16>(map_20.wrapping_add(g_aec8 * 2)) as usize;
                let p3_8 = rd::<usize>(param_3 + 0x08);
                let mut ctx_80: [u64; 3] = [
                    param_3 as u64,
                    rd::<u32>(p3_8.wrapping_add(slot * 0x10 + 0x08)) as u64,
                    rd::<u64>(p3_8.wrapping_add(slot * 0x10)),
                ];
                exe::call_va!(
                    0x1_41f5_b370,
                    (rbx_id, rd::<u8>(param_2 + 0x18), r15_map, ctx_80.as_mut_ptr() as usize)
                );

                let mut key_m10: u64 = rbx_id;
                let mut idx_a0: i64 = 0;
                let idx = if rd::<u64>(r15_map + 0x38) == rd::<u64>(r15_map + 0x30) {
                    rd::<u64>(r15_map + 0x40)
                } else {
                    exe::call_va!(
                        0x1_401f_a730,
                        (
                            r15_map,
                            (&mut idx_a0 as *mut i64) as usize,
                            (&mut key_m10 as *mut u64) as usize
                        )
                    );
                    if idx_a0 == -1 {
                        rd::<u64>(r15_map + 0x40)
                    } else {
                        idx_a0 as u64
                    }
                };
                let base_58 = rd::<usize>(r15_map + 0x58);
                let entry_ptr = base_58.wrapping_add((idx as usize) << 4);
                let end_entry = base_58.wrapping_add((rd::<u64>(r15_map + 0x40) as usize) << 4);
                if entry_ptr != end_entry {
                    let ent = rd::<u64>(entry_ptr + 0x08);
                    if (ent as u32) != u32::MAX {
                        let ent_lo = (ent as u32) as usize;
                        let rdx_val = rd::<u64>(rd::<usize>(param_3 + 0x30).wrapping_add(ent_lo * 8));
                        if (rdx_val as u16) == 0xffff {
                            exe::call_iat!(0x1_4264_67e8, ());
                            unreachable!();
                        }
                        let arch_off = ((rdx_val as u16) as usize) << 5;
                        let comp_cnt = rd::<u32>(param_3.wrapping_add(arch_off + 0x180a8));
                        let comp_id = rd::<u16>(exe::ptr::<u16>(0x1_43cc_5590) as usize);
                        let comp_arr = rd::<usize>(param_3.wrapping_add(arch_off + 0x18090));
                        let found = exe::call_va!(0x1_4248_9ee0, (comp_arr, comp_cnt, comp_id) -> i32);
                        if found == -1 {
                            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                            let rcx_buf = param_3.wrapping_add(0x580f0).wrapping_add(w_idx * 0x188);
                            let mut item_10: [u64; 2] = [ent, 0];
                            exe::call_va!(0x1_40fa_1370, (rcx_buf, item_10.as_mut_ptr() as usize));
                        }
                    }
                }
            }
            rsi_ptr = rsi_ptr.wrapping_add(0x20);
        }

        wr::<u32>(param_2 + 0x1c, 2);
        let vec_70 = param_1.wrapping_add(0x70);
        let len_70 = rd::<u32>(vec_70 + 0x08);
        let elem_ptr = rd::<usize>(vec_70).wrapping_add((len_70 as usize) * 0x18);
        let cap_70 = rd::<u32>(vec_70 + 0x0c) & 0x3fff_ffff;
        if len_70 == cap_70 {
            let mut pos_m8: usize = elem_ptr;
            exe::call_va!(
                0x1_410d_6660,
                (vec_70, (&mut pos_m8 as *mut usize) as usize, param_2)
            );
        } else {
            wr::<u32>(vec_70 + 0x08, len_70.wrapping_add(1));
            wr::<u64>(elem_ptr, 0);
            wr::<u64>(elem_ptr + 0x08, 0);
            wr::<u32>(elem_ptr + 0x10, 0);
            wr::<u8>(elem_ptr + 0x14, 0);
            exe::call_va!(0x1_401e_84a0, (elem_ptr, param_2));
        }

        let mut tmp_m50 = [0u8; 24];
        exe::call_va!(0x1_401e_84a0, (tmp_m50.as_mut_ptr() as usize, param_2));
        exe::call_va!(
            0x1_41e5_6a50,
            (1u32, tmp_m50.as_mut_ptr() as usize, param_4, param_5, param_3)
        );
    }

    let cur_state = rd::<u32>(param_2 + 0x1c);
    if cur_state != 2 {
        if cur_state == 4 {
            if rd::<u8>(param_2 + 0x19) != 0 {
                let mut esi: u32 = 0;
                let bd0 = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
                if rd::<u32>(bd0 + 0x08) > 0 {
                    loop {
                        let bd = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
                        let rdx_id = rd::<u64>(rd::<usize>(bd).wrapping_add((esi as usize) << 5));
                        if (rdx_id & 0x3fff) != 0 && (rdx_id & 0x3fff_ffff_ffff_c000) != 0 {
                            exe::call_va!(0x1_41e5_c4f0, (param_3, rdx_id));
                        }
                        esi = esi.wrapping_add(1);
                        let bd_next = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
                        if (esi as usize) >= (rd::<u32>(bd_next + 0x08) as usize) {
                            break;
                        }
                    }
                }
                wr::<u8>(param_2 + 0x19, 0);
            }
            wr::<u32>(param_2 + 0x1c, 0);
        }
        return 1;
    }

    let mode_20 = rd::<u32>(param_2 + 0x20);
    if mode_20 == 1 {
        let bd = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
        let mut rsi_ptr = rd::<usize>(bd + 0x10);
        let end_ptr = rsi_ptr.wrapping_add((rd::<u32>(bd + 0x18) as usize) << 5);
        while rsi_ptr != end_ptr {
            let mut key_30: u64 = rd::<u64>(rsi_ptr);
            let mut idx_40: i64 = 0;
            let idx = if rd::<u64>(r15_map + 0x38) == rd::<u64>(r15_map + 0x30) {
                rd::<u64>(r15_map + 0x40)
            } else {
                exe::call_va!(
                    0x1_401f_a730,
                    (
                        r15_map,
                        (&mut idx_40 as *mut i64) as usize,
                        (&mut key_30 as *mut u64) as usize
                    )
                );
                if idx_40 == -1 {
                    rd::<u64>(r15_map + 0x40)
                } else {
                    idx_40 as u64
                }
            };
            let base_58 = rd::<usize>(r15_map + 0x58);
            let entry_ptr = base_58.wrapping_add((idx as usize) << 4);
            let end_entry = base_58.wrapping_add((rd::<u64>(r15_map + 0x40) as usize) << 4);
            if entry_ptr != end_entry {
                let ent = rd::<u64>(entry_ptr + 0x08);
                if (ent as u32) != u32::MAX {
                    let ent_lo = (ent as u32) as usize;
                    let rdx_val = rd::<u64>(rd::<usize>(param_3 + 0x30).wrapping_add(ent_lo * 8));
                    if (rdx_val as u16) == 0xffff {
                        exe::call_iat!(0x1_4264_67e8, ());
                        unreachable!();
                    }
                    let arch_off = ((rdx_val as u16) as usize) << 5;
                    let comp_cnt = rd::<u32>(param_3.wrapping_add(arch_off + 0x180a8));
                    let comp_id = rd::<u16>(exe::ptr::<u16>(0x1_43cc_5590) as usize);
                    let comp_arr = rd::<usize>(param_3.wrapping_add(arch_off + 0x18090));
                    let found = exe::call_va!(0x1_4248_9ee0, (comp_arr, comp_cnt, comp_id) -> i32);
                    if found != -1 {
                        let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                        let r8_off = w_idx * 0x188;
                        let prev_ent = rd::<u64>(param_3.wrapping_add(r8_off + 0x58148));
                        let inc: u64 = if (ent as u32) == (prev_ent as u32)
                            && ((ent >> 32) as u32) == ((prev_ent >> 32) as u32)
                        {
                            0
                        } else {
                            1
                        };
                        wr::<u64>(
                            param_3.wrapping_add(r8_off + 0x58150),
                            rd::<u64>(param_3.wrapping_add(r8_off + 0x58150)).wrapping_add(inc),
                        );
                        wr::<u64>(param_3.wrapping_add(r8_off + 0x58148), ent);

                        let comp_bx = rd::<u16>(exe::ptr::<u16>(0x1_43cc_5590) as usize);
                        let list1 = param_3.wrapping_add(r8_off + 0x58248);
                        let c1 = rd::<usize>(list1 + 0x08);
                        if c1 == 0 || rd::<u64>(c1 + 0x08).wrapping_add(1) > 0x4000 {
                            exe::call_va!(0x1_4020_e8f0, (list1));
                        }
                        let c1 = rd::<usize>(list1 + 0x08);
                        let idx1 = rd::<u64>(c1 + 0x08) as usize;
                        wr::<u32>(c1.wrapping_add(0x10 + idx1 * 4), ent as u32);
                        let c1 = rd::<usize>(list1 + 0x08);
                        wr::<u64>(c1 + 0x08, rd::<u64>(c1 + 0x08).wrapping_add(1));
                        wr::<u64>(list1 + 0x10, rd::<u64>(list1 + 0x10).wrapping_add(1));

                        let list2 = param_3.wrapping_add(r8_off + 0x58260);
                        let c2 = rd::<usize>(list2 + 0x08);
                        if c2 == 0 || rd::<u64>(c2 + 0x08).wrapping_add(1) > 0x8000 {
                            exe::call_va!(0x1_4020_e8f0, (list2));
                        }
                        let c2 = rd::<usize>(list2 + 0x08);
                        let idx2 = rd::<u64>(c2 + 0x08) as usize;
                        wr::<u16>(c2.wrapping_add(0x10 + idx2 * 2), comp_bx);
                        let c2 = rd::<usize>(list2 + 0x08);
                        wr::<u64>(c2 + 0x08, rd::<u64>(c2 + 0x08).wrapping_add(1));
                        wr::<u64>(list2 + 0x10, rd::<u64>(list2 + 0x10).wrapping_add(1));
                    }
                }
            }
            rsi_ptr = rsi_ptr.wrapping_add(0x20);
        }

        let vec_70 = param_1.wrapping_add(0x70);
        wr::<u32>(param_2 + 0x1c, 4);
        wr::<u32>(param_2 + 0x20, 0);
        let len_70 = rd::<u32>(vec_70 + 0x08) as usize;
        let base_70 = rd::<usize>(vec_70);
        let target = rd::<u64>(param_2);
        let mut found_ptr = base_70.wrapping_add(len_70 * 0x18);
        for i in 0..len_70 {
            let p = base_70.wrapping_add(i * 0x18);
            if rd::<u64>(p) == target {
                found_ptr = p;
                break;
            }
        }
        let mut iter_in: usize = found_ptr;
        let mut iter_out: usize = 0;
        exe::call_va!(
            0x1_410d_3570,
            (
                vec_70,
                (&mut iter_out as *mut usize) as usize,
                (&mut iter_in as *mut usize) as usize
            )
        );
        let mut tmp_50 = [0u8; 24];
        exe::call_va!(0x1_401e_84a0, (tmp_50.as_mut_ptr() as usize, param_2));
        exe::call_va!(
            0x1_41e5_6a50,
            (2u32, tmp_50.as_mut_ptr() as usize, param_4, param_5, param_3)
        );
    } else if mode_20 == 3 {
        let bd = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
        let mut rsi_cnt = rd::<u32>(bd + 0x18) as u64;
        if rsi_cnt <= 0x9c4 {
            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let bd2 = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
            let mut span_50: [u64; 2] = [rd::<u32>(bd2 + 0x18) as u64, rd::<u64>(bd2 + 0x10)];
            exe::call_va!(
                0x1_41f5_94f0,
                (
                    0usize,
                    span_50.as_mut_ptr() as usize,
                    r15_map,
                    param_3.wrapping_add(0x580f0 + w_idx * 0x188)
                )
            );
        } else {
            let g_2f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
            let sched = rd::<usize>(g_2f8 + 0x10);
            let s_parent = exe::ptr::<u8>(0x1_4339_1300) as usize;
            let mut name_span: [u64; 2] = [s_parent as u64, 0x54];
            let mut parent_job: [u64; 2] = [0; 2];
            exe::call_va!(
                0x1_4248_58a0,
                (
                    g_2f8,
                    parent_job.as_mut_ptr() as usize,
                    name_span.as_mut_ptr() as usize
                )
            );

            let chunk_size = rsi_cnt / rd::<u64>(g_2f8);
            let mut r13_done: u64 = 0;
            let mut jobs_queued: u64 = 0;
            if chunk_size != 0 && (rsi_cnt as i64) > (chunk_size as i64) {
                let mut byte_off: usize = 0;
                let stride_bytes = (chunk_size as usize) << 5;
                loop {
                    let bd_loop = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
                    let rem_cnt = (rd::<u32>(bd_loop + 0x18) as u64).wrapping_sub(r13_done);
                    let ptr_14 = rd::<usize>(bd_loop + 0x10).wrapping_add(byte_off);
                    let batch_cnt = if chunk_size == u64::MAX {
                        rem_cnt
                    } else {
                        chunk_size
                    };
                    let sched_inner = rd::<usize>(g_2f8 + 0x10);
                    let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                    let pool =
                        rd::<usize>(sched_inner.wrapping_add(w_idx * 0x40 + 0x48)).wrapping_add(0x80);
                    let mut child_job: [u64; 2] = [0; 2];
                    exe::call_va!(0x1_4248_5aa0, (pool, child_job.as_mut_ptr() as usize));
                    let node = child_job[0] as usize;
                    let _ = crate::simd_ops::atomic_xchg_u32(node as u64, 1);
                    wr::<u64>(node + 0x08, 0);
                    wr::<usize>(node + 0x10, exe::ptr::<u8>(0x1_4339_1818) as usize);
                    wr::<u64>(node + 0x18, batch_cnt);
                    wr::<usize>(node + 0x20, ptr_14);
                    wr::<usize>(node + 0x28, r15_map);
                    wr::<usize>(node + 0x30, param_3);
                    let parent_node = parent_job[0] as usize;
                    let _ = crate::simd_ops::atomic_add_u32(parent_node as u64, 1);
                    wr::<usize>(node + 0x08, parent_node);
                    exe::call_va!(0x1_4248_6460, (sched, node));
                    jobs_queued = jobs_queued.wrapping_add(1);
                    rsi_cnt = rsi_cnt.wrapping_sub(chunk_size);
                    r13_done = r13_done.wrapping_add(chunk_size);
                    byte_off = byte_off.wrapping_add(stride_bytes);
                    if (rsi_cnt as i64) <= (chunk_size as i64) {
                        break;
                    }
                }
                if jobs_queued != 0 {
                    let sem_owner = rd::<usize>(sched);
                    let sem_handle = rd::<usize>(sem_owner + 0x1008);
                    exe::call_iat!(0x1_4264_5580, (sem_handle, jobs_queued as u32, 0usize));
                }
            }

            let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let bd_rem = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
            let rem_cnt = (rd::<u32>(bd_rem + 0x18) as u64).wrapping_sub(r13_done);
            let rem_ptr = rd::<usize>(bd_rem + 0x10).wrapping_add((r13_done as usize) << 5);
            let mut span_60: [u64; 2] = [rem_cnt, rem_ptr as u64];
            exe::call_va!(
                0x1_41f5_94f0,
                (
                    0usize,
                    span_60.as_mut_ptr() as usize,
                    r15_map,
                    param_3.wrapping_add(0x580f0 + w_idx * 0x188)
                )
            );
            exe::call_va!(0x1_4248_5780, (g_2f8, parent_job.as_mut_ptr() as usize));
        }

        let w_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let bd_tail = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
        let mut span_b0: [u64; 2] = [rd::<u32>(bd_tail + 0x08) as u64, rd::<u64>(bd_tail)];
        exe::call_va!(
            0x1_41f5_94f0,
            (
                0usize,
                span_b0.as_mut_ptr() as usize,
                r15_map,
                param_3.wrapping_add(0x580f0 + w_idx * 0x188)
            )
        );
        wr::<u32>(param_2 + 0x1c, 0);
        wr::<u32>(param_2 + 0x20, 0);
        let vec_70 = param_1.wrapping_add(0x70);
        let mut out_find: usize = 0;
        let found_iter = exe::call_va!(
            0x1_410d_3510,
            (vec_70, (&mut out_find as *mut usize) as usize, param_2) -> usize
        );
        let mut out_erase: usize = 0;
        exe::call_va!(
            0x1_410d_3570,
            (vec_70, (&mut out_erase as *mut usize) as usize, found_iter)
        );
        let mut tmp_m38 = [0u8; 24];
        exe::call_va!(0x1_401e_84a0, (tmp_m38.as_mut_ptr() as usize, param_2));
        exe::call_va!(
            0x1_41e5_6a50,
            (2u32, tmp_m38.as_mut_ptr() as usize, param_4, param_5, param_3)
        );
    }

    if rd::<u32>(param_2 + 0x20) == 2 {
        let g_aec8 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_aec8) as usize) as usize;
        let map_20 = rd::<usize>(rd::<usize>(param_3) + 0x20);
        let slot = rd::<u16>(map_20.wrapping_add(g_aec8 * 2)) as usize;
        let entry_rcx = rd::<usize>(param_3 + 0x08).wrapping_add(slot << 4);
        let ctx_0 = param_3 as u64;
        let ctx_1 = rd::<u32>(entry_rcx + 0x08) as u64;
        let ctx_2 = rd::<u64>(entry_rcx);
        let sil = rd::<u8>(param_2 + 0x18);

        let mut tmp_70 = [0u8; 24];
        let tmp_70_ptr = tmp_70.as_mut_ptr() as usize;
        exe::call_va!(0x1_401e_84a0, (tmp_70_ptr, param_2));

        let bd1 = exe::call_va!(0x1_4021_7df0, (tmp_70_ptr) -> usize);
        let mut rbx1 = rd::<usize>(bd1);
        let end1 = rbx1.wrapping_add((rd::<u32>(bd1 + 0x08) as usize) << 5);
        while rbx1 != end1 {
            let mut ctx_c0: [u64; 3] = [ctx_0, ctx_1, ctx_2];
            exe::call_va!(
                0x1_41f5_b370,
                (rd::<u64>(rbx1), sil, r15_map, ctx_c0.as_mut_ptr() as usize)
            );
            rbx1 = rbx1.wrapping_add(0x20);
        }

        let bd2 = exe::call_va!(0x1_4021_7df0, (tmp_70_ptr) -> usize);
        let mut rbx2 = rd::<usize>(bd2 + 0x10);
        let end2 = rbx2.wrapping_add((rd::<u32>(bd2 + 0x18) as usize) << 5);
        while rbx2 != end2 {
            let mut ctx_e0: [u64; 3] = [ctx_0, ctx_1, ctx_2];
            exe::call_va!(
                0x1_41f5_b370,
                (rd::<u64>(rbx2), sil, r15_map, ctx_e0.as_mut_ptr() as usize)
            );
            rbx2 = rbx2.wrapping_add(0x20);
        }

        let _ = crate::simd_ops::atomic_xchg_u64((tmp_70_ptr + 0x08) as u64, 0);
        wr::<u32>(param_2 + 0x20, 0);
    }

    0
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

pub unsafe fn diff_unknown_141e9a600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43c8_f63c, 2);

    static mut SLOT_CELL: usize = 0;
    static mut OBJ_8330: [u8; 0x120] = [0; 0x120];
    static mut ENTRY_C160: [u8; 0x20] = [0; 0x20];
    static mut LOG_50F0: (u64, u64, u64, u64, u64, u64, u64, [u8; 32], [u8; 32], [u64; 5], [u32; 4], [u64; 3]) =
        (0, 0, 0, 0, 0, 0, 0, [0; 32], [0; 32], [0; 5], [0; 4], [0; 3]);
    static mut D6F0_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_3900(_ctx: usize, _key: usize, out_freed: usize) -> usize {
        wr::<usize>(out_freed, 0);
        core::ptr::addr_of_mut!(SLOT_CELL) as usize
    }
    unsafe extern "win64" fn mock_b4a0() -> usize {
        0x7777
    }
    unsafe extern "win64" fn mock_2de0(_p: usize) {}
    unsafe extern "win64" fn mock_8330(_a: usize, _b: usize, r8_h: usize) -> usize {
        let p = core::ptr::addr_of_mut!(OBJ_8330) as usize;
        wr::<u64>(p, r8_h as u64);
        p
    }
    unsafe extern "win64" fn mock_d6f0(_owner: usize, _obj: usize) {
        D6F0_COUNT += 1;
    }
    unsafe extern "win64" fn mock_c160(_ctx: usize, _key: usize, _ref_s: usize) -> usize {
        core::ptr::addr_of_mut!(ENTRY_C160) as usize
    }
    unsafe extern "win64" fn mock_3690(_p: usize) {}
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn mock_50f0(
        _a1: usize,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: usize,
        a9: usize,
        a10: u64,
        _a11: u64,
        a12: usize,
        a13: usize,
        a14: usize,
    ) {
        LOG_50F0 = (
            a2,
            a3,
            a4,
            a5,
            a6,
            a7,
            a10,
            rd::<[u8; 32]>(a8),
            rd::<[u8; 32]>(a9),
            [
                rd::<u64>(a12),
                rd::<u64>(a12 + 0x08),
                rd::<u64>(a12 + 0x10),
                rd::<u64>(a12 + 0x18),
                rd::<u8>(a12 + 0x20) as u64,
            ],
            rd::<[u32; 4]>(a13),
            [
                rd::<u64>(a14),
                rd::<u64>(a14 + 0x08),
                rd::<u8>(a14 + 0x10) as u64,
            ],
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_402d_3900, mock_3900 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4028_b4a0, mock_b4a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4100_8330, mock_8330 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4100_c160, mock_c160 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4027_3690, mock_3690 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4216_50f0, mock_50f0 as *const ())?;

    let run = |f: F, mode: u8| -> (u8, u8, u8, u16, u32, [u64; 5], [u32; 4], [u64; 3]) {
        SLOT_CELL = 0;
        OBJ_8330 = [0; 0x120];
        ENTRY_C160 = [0; 0x20];
        D6F0_COUNT = 0;

        let obj_p = core::ptr::addr_of_mut!(OBJ_8330) as usize;
        wr::<u8>(obj_p + 0x114, 0xff);

        wr::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize, 0);

        let mut ref_obj = [0u8; 0x30];
        let ref_p = ref_obj.as_mut_ptr() as usize;
        wr::<u32>(ref_p + 0x24, 0);
        wr::<usize>(ref_p + 0x28, 0x1234);

        let mut p1: [u64; 4] = [ref_p as u64, 0x11, 0x22, if mode == 1 { 1 } else { 2 }];

        let mut a00 = [0u8; 0x60];
        let mut a08 = [0x33u8; 0x40];
        let mut a10 = [0u8; 0x40];
        let mut a18: [u16; 4] = [0x4455, 0x6677, 0, 0];
        let mut a20 = [0u8; 0x60];
        let mut a28: [u64; 4] = [0xaaaa, 0xbbbb, 0xcccc, 0xdddd];
        let mut a30: [u64; 2] = [if mode == 1 { 0x80 } else { 0 }, 0];
        let mut a38: [u64; 2] = [0x9999, 0x8888];
        let mut a40: [u64; 2] = [0x5555, 0x6666];
        let mut a48 = [0u8; 4];
        let mut a50: [u64; 4] = [0, 0, 0x1234_5678, 0];

        if mode == 1 {
            a00[0] = 1;
            wr::<f32>(a00.as_mut_ptr() as usize + 0x10, 1.0);
            wr::<f32>(a10.as_mut_ptr() as usize, 2.0);
        }

        let item20_p = a20.as_mut_ptr() as usize;
        wr::<u64>(item20_p, 0x101);
        wr::<u32>(item20_p + 0x08, 0x202);
        wr::<u64>(item20_p + 0x10, 0x303);
        wr::<u32>(item20_p + 0x18, 0x404);
        wr::<u32>(item20_p + 0x20, 0x505);
        wr::<u32>(item20_p + 0x24, 0x606);
        wr::<u32>(item20_p + 0x28, 0x707);

        let mut p2: [usize; 12] = [
            a00.as_mut_ptr() as usize,
            a08.as_mut_ptr() as usize,
            a10.as_mut_ptr() as usize,
            a18.as_mut_ptr() as usize,
            a20.as_mut_ptr() as usize,
            a28.as_mut_ptr() as usize,
            a30.as_mut_ptr() as usize,
            a38.as_mut_ptr() as usize,
            if mode == 1 { a40.as_mut_ptr() as usize } else { 0 },
            a48.as_mut_ptr() as usize,
            a50.as_mut_ptr() as usize,
            0,
        ];

        let mut d110: [u64; 1] = [0x4242];
        let mut arr_580e0: [usize; 1] = [d110.as_mut_ptr() as usize];
        let mut p3 = vec![0u8; 0x58300];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x580e0, arr_580e0.as_mut_ptr() as usize);

        f(p1.as_mut_ptr() as usize, p2.as_mut_ptr() as usize, p3p);
        let ep = core::ptr::addr_of!(ENTRY_C160) as usize;
        (
            a00[0],
            a48[0],
            rd::<u8>(obj_p + 0x114),
            rd::<u16>(ep + 0x14),
            D6F0_COUNT,
            LOG_50F0.9,
            LOG_50F0.10,
            LOG_50F0.11,
        )
    };

    for mode in [0u8, 1u8] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141e9a600 mode={}", mode);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141eb29c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_8eb0(dst: usize, src_mat: usize) {
        wr::<[u32; 12]>(dst, rd::<[u32; 12]>(src_mat));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4103_8eb0, mock_8eb0 as *const ())?;

    let run = |f: F, q_and_t: [f32; 8]| -> ([u32; 12], [u32; 8]) {
        let mut p1 = [0u8; 0x40];
        let mut p2 = [0u32; 8];
        let mut p3 = q_and_t;
        f(
            p1.as_mut_ptr() as usize,
            p2.as_mut_ptr() as usize,
            p3.as_mut_ptr() as usize,
        );
        (rd::<[u32; 12]>(p1.as_ptr() as usize + 0x10), p2)
    };

    for input in [
        [0.0f32, 0.0, 0.0, 1.0, 10.0, -20.0, 30.0, 1.0],
        [0.5f32, -0.5, 0.5, 0.5, -3.5, 7.25, 11.0, 0.0],
    ] {
        let o = run(f_o, input);
        let n = run(f_n, input);
        crate::diff_eq!(o, n, "141eb29c0 input={:?}", input);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f2d380(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_8930: (usize, u64) = (0, 0);

    unsafe extern "win64" fn mock_7c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_8930(rcx_ctx: usize, rdx_val: u64) {
        LOG_8930 = (rcx_ctx, rdx_val);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4034_8930, mock_8930 as *const ())?;

    let run = |f: F, hit: bool| -> (usize, u64) {
        LOG_8930 = (0, 0);
        let mut handles: [u64; 2] = [0x0002_0000_0000_0000, 0];
        let mut keys: [u64; 4] = [0, 0, 0xdead_beef, 0];
        let mut p1: [usize; 3] = [
            handles.as_mut_ptr() as usize,
            keys.as_mut_ptr() as usize,
            0,
        ];

        let mut table: [u64; 2] = [
            if hit {
                0x0002_0000_0000_0000
            } else {
                0x0003_0000_0000_0000
            },
            0,
        ];
        let mut p2 = vec![0u8; 0x58400];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x18, table.as_mut_ptr() as usize);
        wr::<u32>(p2p + 0x20, 2);

        f(p1.as_mut_ptr() as usize, p2p);
        let off = if LOG_8930.0 == 0 {
            0
        } else {
            LOG_8930.0.wrapping_sub(p2p)
        };
        (off, LOG_8930.1)
    };

    for hit in [true, false] {
        let o = run(f_o, hit);
        let n = run(f_n, hit);
        crate::diff_eq!(o, n, "141f2d380 hit={}", hit);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f59270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA_9270: [u8; 0x200] = [0; 0x200];
    static mut ARENA_OFF: usize = 0;
    static mut ERASED_9270: Vec<(usize, usize)> = Vec::new();
    static mut SORTED_CNT: usize = 0;

    unsafe extern "win64" fn mock_96e0(
        _p1: usize,
        batch_ptr: usize,
        _p4: usize,
        _p5: usize,
        _p6: usize,
    ) -> u8 {
        rd::<u8>(batch_ptr)
    }
    unsafe extern "win64" fn mock_c0a0(bytes: usize, _align: u32) -> usize {
        let base = (core::ptr::addr_of_mut!(ARENA_9270) as usize).wrapping_add(ARENA_OFF);
        ARENA_OFF += (bytes + 15) & !15;
        base
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}
    unsafe extern "win64" fn mock_5d90(_b: usize, _e: usize, cnt: usize, _z: u32) {
        SORTED_CNT = cnt;
    }
    unsafe extern "win64" fn mock_3ee0(_p2: usize, _out: usize, i0: usize, i1: usize) {
        ERASED_9270.push((i0, i1));
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f5_96e0, mock_96e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40fa_5d90, mock_5d90 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_410d_3ee0, mock_3ee0 as *const ())?;

    let run = |f: F, count: u16| -> (usize, Vec<(usize, usize)>) {
        ARENA_9270 = [0; 0x200];
        ARENA_OFF = 0;
        ERASED_9270.clear();
        SORTED_CNT = 0;

        let mut batches = [0u8; 0x28 * 4];
        let bp = batches.as_mut_ptr() as usize;
        wr::<u8>(bp, 1);
        wr::<u8>(bp + 0x28, 0);
        wr::<u8>(bp + 0x50, 1);

        let mut p2 = [0u8; 0x10];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, bp);
        wr::<u16>(p2p + 0x08, count);

        f(0x10, p2p, 0, 0x20, 0x30, 0x40);
        (SORTED_CNT, ERASED_9270.clone())
    };

    for count in [0u16, 3u16] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "141f59270 count={}", count);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f594f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA_94F0: [u8; 0x200] = [0; 0x200];

    unsafe extern "win64" fn mock_c0a0(_bytes: usize, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ARENA_94F0) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}
    unsafe extern "win64" fn mock_a730(_map: usize, out_idx: usize, key_ptr: usize) {
        let k = rd::<u64>(key_ptr);
        wr::<i64>(out_idx, if k == 0x111 { 0 } else { -1 });
    }
    unsafe extern "win64" fn mock_e8f0(_list: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4020_e8f0, mock_e8f0 as *const ())?;

    let run = |f: F, count: u64| -> (u64, u64, u64, u64, u32) {
        ARENA_94F0 = [0; 0x200];

        let mut items = [0u8; 0x40];
        let ip = items.as_mut_ptr() as usize;
        wr::<u64>(ip, 0x111);
        wr::<u64>(ip + 0x20, 0x222);

        let mut p2: [u64; 2] = [count, ip as u64];

        let mut map_entries: [u64; 4] = [0x111, 0x0005_0000_0000_002a, 0, 0];
        let mut p3 = [0u8; 0x60];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x30, 0);
        wr::<u64>(p3p + 0x38, 1);
        wr::<u64>(p3p + 0x40, 1);
        wr::<usize>(p3p + 0x58, map_entries.as_mut_ptr() as usize);

        let mut chunk = [0u8; 0x30];
        let cp = chunk.as_mut_ptr() as usize;
        let mut p4 = [0u8; 0x90];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x80, cp);

        f(0, p2.as_mut_ptr() as usize, p3p, p4p);
        (
            rd::<u64>(p4p + 0x58),
            rd::<u64>(p4p + 0x60),
            rd::<u64>(p4p + 0x88),
            rd::<u64>(cp + 0x08),
            rd::<u32>(cp + 0x10),
        )
    };

    for count in [0u64, 2u64] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "141f594f0 count={}", count);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141f596e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_43c8_d08c, 2);
    let _g2 = GlobalSave::new(0x1_43cc_aec8, 2);
    let _g3 = GlobalSave::new(0x1_43cc_5590, 2);

    static mut BD_MEM: [u8; 0x20] = [0; 0x20];
    static mut BD_ELEMS: [u64; 8] = [0; 8];
    static mut LOG_6A50: Vec<u32> = Vec::new();
    static mut LOG_C4F0: Vec<u64> = Vec::new();

    unsafe extern "win64" fn mock_7df0(p2: usize) -> usize {
        if rd::<u64>(p2) == 0 {
            0
        } else {
            core::ptr::addr_of_mut!(BD_MEM) as usize
        }
    }
    unsafe extern "win64" fn mock_7100(out_buf: usize, _p2: usize) -> usize {
        out_buf
    }
    unsafe extern "win64" fn mock_b080(_s: usize, _b: u8, _p3: usize, _map: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_9650(_map: usize, out_ptr: usize, _id: u64) {
        wr::<u64>(out_ptr, 1);
    }
    unsafe extern "win64" fn mock_0800(_p3: usize, _ent: u64) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_b370(_id: u64, _b: u8, _map: usize, _ctx: usize) {}
    unsafe extern "win64" fn mock_a730(_map: usize, out_idx: usize, _key: usize) {
        wr::<i64>(out_idx, -1);
    }
    unsafe extern "win64" fn mock_6660(_vec: usize, _pos: usize, _p2: usize) {}
    unsafe extern "win64" fn mock_84a0(_dst: usize, _src: usize) {}
    unsafe extern "win64" fn mock_6a50(tag: u32, _tmp: usize, _p4: usize, _p5: usize, _p3: usize) {
        LOG_6A50.push(tag);
    }
    unsafe extern "win64" fn mock_c4f0(_p3: usize, id: u64) {
        LOG_C4F0.push(id);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4021_7df0, mock_7df0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4022_7100, mock_7100 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41f5_b080, mock_b080 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401f_9650, mock_9650 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40f3_0800, mock_0800 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41f5_b370, mock_b370 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_410d_6660, mock_6660 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_41e5_6a50, mock_6a50 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_41e5_c4f0, mock_c4f0 as *const ())?;

    let run = |f: F, init_state: u32| -> (u8, u32, u8, Vec<u32>, Vec<u64>) {
        BD_MEM = [0; 0x20];
        BD_ELEMS = [0; 8];
        LOG_6A50.clear();
        LOG_C4F0.clear();

        wr::<u16>(exe::ptr::<u16>(0x1_43c8_d08c) as usize, 0);
        wr::<u16>(exe::ptr::<u16>(0x1_43cc_aec8) as usize, 0);
        wr::<u16>(exe::ptr::<u16>(0x1_43cc_5590) as usize, 1);

        let ep = core::ptr::addr_of_mut!(BD_ELEMS) as usize;
        wr::<u64>(ep, 0x0001_4005);
        let bdp = core::ptr::addr_of_mut!(BD_MEM) as usize;
        wr::<usize>(bdp, ep);
        wr::<u32>(bdp + 0x08, 1);
        wr::<usize>(bdp + 0x10, ep);
        wr::<u32>(bdp + 0x18, 1);

        let mut p1 = [0u8; 0x90];
        let mut p2 = [0u8; 0x30];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, if init_state == 99 { 0 } else { 0x1234 });
        wr::<u8>(p2p + 0x18, 1);
        wr::<u8>(p2p + 0x19, 1);
        wr::<u32>(p2p + 0x1c, if init_state == 99 { 0 } else { init_state });
        wr::<u32>(p2p + 0x20, 0);

        let mut map_obj = [0u8; 0x60];
        let mut arr_580e0: [usize; 1] = [map_obj.as_mut_ptr() as usize];
        let mut tab_20: [u16; 2] = [0, 0];
        let mut p3_0 = [0u8; 0x30];
        wr::<usize>(p3_0.as_mut_ptr() as usize + 0x20, tab_20.as_mut_ptr() as usize);
        let mut p3_8 = [0u8; 0x20];

        let mut p3 = vec![0u8; 0x58200];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p, p3_0.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x08, p3_8.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x580e0, arr_580e0.as_mut_ptr() as usize);

        let ret = f(p1.as_mut_ptr() as usize, p2p, p3p, 0xaa, 0xbb);
        (
            ret,
            rd::<u32>(p2p + 0x1c),
            rd::<u8>(p2p + 0x19),
            LOG_6A50.clone(),
            LOG_C4F0.clone(),
        )
    };

    for init_state in [99u32, 0, 4] {
        let o = run(f_o, init_state);
        let n = run(f_n, init_state);
        crate::diff_eq!(o, n, "141f596e0 init_state={}", init_state);
    }
    Ok(3)
}
