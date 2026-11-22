//! Decompiled hot functions shard R (`0x141f57ce0..0x1420892a0`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vhaddps, vhsubps, vmulps, vpcmpgtb, vpermilps, vpmovmskb,
    vshufps, vsubps,
};

#[inline(always)]
unsafe fn rbtree_next(node: usize) -> usize {
    let right = rd::<usize>(node + 0x10);
    if rd::<u8>(right + 0x19) == 0 {
        let mut cur = right;
        loop {
            let left = rd::<usize>(cur);
            if rd::<u8>(left + 0x19) != 0 {
                return cur;
            }
            cur = left;
        }
    } else {
        let mut cur = node;
        let mut parent = rd::<usize>(node + 0x08);
        while rd::<u8>(parent + 0x19) == 0 && cur == rd::<usize>(parent + 0x10) {
            cur = parent;
            parent = rd::<usize>(parent + 0x08);
        }
        parent
    }
}

/// `unknown_141f57ce0`
/// Original: 0x141f57ce0 (1241 bytes, Throws::No)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_141f57ce0(param_1: usize, param_2: f32) {
    unknown_141f57ce0_impl(param_1, param_2);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_141f57ce0(param_1: usize, _xmm0: f32, param_2: f32) {
    unknown_141f57ce0_impl(param_1, param_2);
}

#[inline(always)]
unsafe fn unknown_141f57ce0_impl(param_1: usize, param_2: f32) {
    let head0 = rd::<usize>(param_1);
    let mut rbx = rd::<usize>(head0);
    if rbx != head0 {
        loop {
            let rdi = rd::<usize>(rbx + 0x30);
            let mut has_any = false;
            for i in 0..4usize {
                if rd::<usize>(rdi + 0xa8 + i * 8) != 0 {
                    has_any = true;
                    break;
                }
            }
            if !has_any {
                if rdi != 0 {
                    exe::call_va!(0x1_410d_2360, (rdi));
                    exe::call_va!(0x1_4244_2de0, (rdi));
                }
                let next_node = rbtree_next(rbx);
                let extracted = exe::call_va!(0x1_403f_7860, (param_1, rbx) -> usize);
                exe::call_va!(0x1_401e_66f0, (0usize, extracted));
                rbx = next_node;
            } else {
                rbx = rbtree_next(rbx);
            }
            let head = rd::<usize>(param_1);
            if rbx == head {
                rbx = rd::<usize>(head);
                break;
            }
        }
    }

    if rd::<u8>(rbx + 0x19) == 0 {
        let xmm6 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let dt_bits = param_2.to_bits() as u128;
        loop {
            let rbp = rd::<usize>(rbx + 0x30);
            let mut has_any = false;
            for i in 0..4usize {
                if rd::<usize>(rbp + 0xa8 + i * 8) != 0 {
                    has_any = true;
                    break;
                }
            }
            if has_any {
                for esi in 0..4usize {
                    let slot_addr = rbp + 0xa8 + esi * 8;
                    let rcx = rd::<usize>(slot_addr);
                    if rcx == 0 {
                        continue;
                    }
                    let vtbl = rd::<usize>(rcx);
                    let alive = exe::call_va!(rd::<usize>(vtbl + 0x20), (rcx) -> u8);
                    let r14 = rd::<usize>(slot_addr);
                    if alive == 0 {
                        if r14 != 0 {
                            let vtbl14 = rd::<usize>(r14);
                            exe::call_va!(rd::<usize>(vtbl14), (r14, 0u32));
                            exe::call_va!(0x1_4244_2de0, (r14));
                        }
                        wr::<usize>(slot_addr, 0);
                        continue;
                    }
                    let vtbl14 = rd::<usize>(r14);
                    let _ = crate::simd_ops::call_va8_xmm(
                        rd::<u64>(vtbl14 + 0x08),
                        r14 as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        [0, dt_bits, 0, 0],
                    );
                    if esi != 0 {
                        continue;
                    }
                    let rcx_cur = rd::<usize>(slot_addr);
                    let r8d = rd::<u32>(rcx_cur + 0x08);
                    if r8d == 1 && !(xmm6 > rd::<f32>(rcx_cur + 0x1c)) {
                        let v30 = rd::<u32>(rcx_cur + 0x30);
                        wr::<u64>(rbp + 0x98, (v30 as u64) | ((r8d as u8 as u64) << 32));
                        let r14_del = rd::<usize>(slot_addr);
                        if r14_del != 0 {
                            let vtbl_del = rd::<usize>(r14_del);
                            exe::call_va!(rd::<usize>(vtbl_del), (r14_del, 0u32));
                            exe::call_va!(0x1_4244_2de0, (r14_del));
                        }
                        wr::<usize>(slot_addr, 0);
                        continue;
                    }
                    let a0 = rd::<u64>(rbp + 0xa0);
                    if (a0 as u32) == u32::MAX && ((a0 >> 32) as u32) == u32::MAX {
                        continue;
                    }
                    if r8d == 3 {
                        let v30 = rd::<u32>(rcx_cur + 0x30);
                        wr::<u64>(rbp + 0x98, (v30 as u64) | (1u64 << 32));
                    }
                }
            }
            rbx = rbtree_next(rbx);
            if rd::<u8>(rbx + 0x19) != 0 {
                break;
            }
        }
    }

    let head2 = rd::<usize>(param_1);
    rbx = rd::<usize>(head2);
    while rd::<u8>(rbx + 0x19) == 0 {
        let rdi = rd::<usize>(rbx + 0x30);
        let a0 = rd::<u64>(rdi + 0xa0);
        if (a0 as u32) == u32::MAX && ((a0 >> 32) as u32) == u32::MAX {
            let mut xmm0_bits: u32 = if rd::<u8>(rdi + 0x9c) != 0 {
                rd::<u32>(rdi + 0x98)
            } else {
                0
            };
            for off in [0xa8usize, 0xb0, 0xb8, 0xc0] {
                let rcx = rd::<usize>(rdi + off);
                if rcx != 0 {
                    let fn_10 = rd::<u64>(rd::<usize>(rcx) + 0x10);
                    let (_, x0) = crate::simd_ops::call_va8_xmm(
                        fn_10,
                        rcx as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        [xmm0_bits as u128, xmm0_bits as u128, 0, 0],
                    );
                    xmm0_bits = x0 as u32;
                }
            }
            let out_ptr = rd::<usize>(rdi + 0x90);
            wr::<u32>(out_ptr, xmm0_bits);
        }
        rbx = rbtree_next(rbx);
    }

    let rdi_tbl = exe::call_va!(0x1_4248_a640, () -> usize);
    let mut rbx_it = rd::<usize>(rdi_tbl + 0x50);
    let rsi_end = rbx_it.wrapping_add(rd::<usize>(rdi_tbl + 0x38).wrapping_shl(4));
    if rbx_it != rsi_end {
        let ecx = rd::<u32>(rdi_tbl + 0x40);
        loop {
            let eax = rd::<u32>(rbx_it);
            if ecx != eax && (rd::<u64>(rdi_tbl + 0x28) == 0 || rd::<u32>(rdi_tbl + 0x24) != eax) {
                break;
            }
            rbx_it = rbx_it.wrapping_add(0x10);
            if rbx_it == rsi_end {
                break;
            }
        }
    }
    let rbp_end = rd::<usize>(rdi_tbl + 0x50)
        .wrapping_add(rd::<usize>(rdi_tbl + 0x38).wrapping_shl(4));
    while rbx_it != rbp_end {
        let rcx = rd::<usize>(rbx_it + 0x08);
        let vtbl = rd::<usize>(rcx);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (rcx));
        rbx_it = rbx_it.wrapping_add(0x10);
        if rbx_it != rsi_end {
            let ecx = rd::<u32>(rdi_tbl + 0x40);
            loop {
                let eax = rd::<u32>(rbx_it);
                if ecx != eax
                    && (rd::<u64>(rdi_tbl + 0x28) == 0 || rd::<u32>(rdi_tbl + 0x24) != eax)
                {
                    break;
                }
                rbx_it = rbx_it.wrapping_add(0x10);
                if rbx_it == rsi_end {
                    break;
                }
            }
        }
    }
}

/// `unknown_141f5b690`
/// Original: 0x141f5b690 (776 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f5b690(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: u8,
) {
    let r8_arr = exe::call_va!(0x1_410d_32e0, (param_1.wrapping_add(0x80), param_2) -> usize);
    let mut r10 = rd::<usize>(r8_arr);
    let r11 = r10.wrapping_add((rd::<u32>(r8_arr + 0x08) as usize).wrapping_shl(5));
    while r10 != r11 {
        let _ = crate::simd_ops::atomic_xchg_u64((r10 + 0x08) as u64, 0);
        r10 = r10.wrapping_add(0x20);
    }
    wr::<u32>(r8_arr + 0x08, 0);

    let mut vec_60 = [0u64; 2];
    let vec_60_ptr = vec_60.as_mut_ptr() as usize;
    let rdi_cnt = rd::<u64>(param_3);
    if rdi_cnt != 0 {
        exe::call_va!(0x1_410d_6540, (vec_60_ptr, rdi_cnt));
    }

    let mut rbx = rd::<usize>(param_3 + 0x08);
    let r14 = rbx.wrapping_add((rdi_cnt as usize).wrapping_shl(5));
    let mut item_buf = [0u8; 0x28];
    let item = item_buf.as_mut_ptr() as usize;

    while rbx != r14 {
        wr::<u64>(item, 0);
        wr::<u64>(item + 0x08, 0);
        wr::<u32>(item + 0x10, 0);
        wr::<u8>(item + 0x14, 0);
        exe::call_va!(0x1_401e_8620, (item));
        wr::<u64>(item, 0x100);
        wr::<u16>(item + 0x18, 0);
        wr::<u64>(item + 0x1c, 0);
        wr::<u32>(item + 0x24, 0);

        let mut wait_40: u64 = 0x64;
        loop {
            while core::sync::atomic::AtomicU8::from_ptr((item + 0x14) as *mut u8)
                .load(core::sync::atomic::Ordering::Acquire)
                != 0
            {
                core::hint::spin_loop();
            }
            if crate::simd_ops::atomic_xchg_u8((item + 0x14) as u64, 1) == 0 {
                break;
            }
            exe::call_va!(0x1_4241_7470, ((&mut wait_40 as *mut u64) as usize));
        }
        let _ = crate::simd_ops::atomic_xchg_u64((item + 0x08) as u64, 0);
        let _ = crate::simd_ops::atomic_xchg_u32((item + 0x10) as u64, 0);
        core::sync::atomic::AtomicU8::from_ptr((item + 0x14) as *mut u8)
            .store(0, core::sync::atomic::Ordering::Release);

        wr::<u64>(item, rd::<u64>(rbx));
        let mut wait_50: u64 = 0x64;
        loop {
            while core::sync::atomic::AtomicU8::from_ptr((rbx + 0x14) as *mut u8)
                .load(core::sync::atomic::Ordering::Acquire)
                != 0
            {
                core::hint::spin_loop();
            }
            if crate::simd_ops::atomic_xchg_u8((rbx + 0x14) as u64, 1) == 0 {
                break;
            }
            exe::call_va!(0x1_4241_7470, ((&mut wait_50 as *mut u64) as usize));
        }
        let _ = crate::simd_ops::atomic_xchg_u64((item + 0x08) as u64, rd::<u64>(rbx + 0x08));
        let _ = crate::simd_ops::atomic_xchg_u32((item + 0x10) as u64, rd::<u32>(rbx + 0x10));
        core::sync::atomic::AtomicU8::from_ptr((rbx + 0x14) as *mut u8)
            .store(0, core::sync::atomic::Ordering::Release);

        let r11b = rd::<u8>(rbx + 0x18);
        wr::<u8>(item + 0x18, r11b);

        let mut esi_len = rd::<u32>(vec_60_ptr + 0x08);
        let mut r10_base = rd::<usize>(vec_60_ptr);
        let rdx_cnt = esi_len as usize;
        let want_key = rd::<u64>(item);
        let mut found_idx: isize = -1;
        for i in 0..rdx_cnt {
            if rd::<u64>(r10_base.wrapping_add(i * 0x28)) == want_key {
                found_idx = i as isize;
                break;
            }
        }

        if found_idx != -1 {
            let off = (found_idx as usize) * 0x28 + 0x18;
            wr::<u8>(r10_base + off, rd::<u8>(r10_base + off) & r11b);
        } else {
            let cap = rd::<u32>(vec_60_ptr + 0x0c);
            if esi_len == cap {
                exe::call_va!(0x1_410d_6540, (vec_60_ptr, (rdx_cnt + 1) as u64));
                esi_len = rd::<u32>(vec_60_ptr + 0x08);
                r10_base = rd::<usize>(vec_60_ptr);
            }
            let rdi_slot = r10_base.wrapping_add((esi_len as usize) * 0x28);
            wr::<u64>(rdi_slot, 0);
            wr::<u64>(rdi_slot + 0x08, 0);
            wr::<u32>(rdi_slot + 0x10, 0);
            wr::<u8>(rdi_slot + 0x14, 0);
            exe::call_va!(0x1_401e_84a0, (rdi_slot, item));
            wr::<u8>(rdi_slot + 0x18, rd::<u8>(item + 0x18));
            wr::<u8>(rdi_slot + 0x19, rd::<u8>(item + 0x19));
            wr::<u32>(rdi_slot + 0x1c, rd::<u32>(item + 0x1c));
            wr::<u32>(rdi_slot + 0x20, rd::<u32>(item + 0x20));
            wr::<u32>(rdi_slot + 0x24, rd::<u32>(item + 0x24));
            wr::<u32>(vec_60_ptr + 0x08, esi_len.wrapping_add(1));
        }

        let _ = crate::simd_ops::atomic_xchg_u64((item + 0x08) as u64, 0);
        rbx = rbx.wrapping_add(0x20);
    }

    let rdx_slot = exe::call_va!(0x1_410d_38c0, (param_1.wrapping_add(0x08), param_2) -> usize);
    exe::call_va!(0x1_41f5_aa00, (0usize, rdx_slot, vec_60_ptr, param_4));

    let final_len = rd::<u32>(vec_60_ptr + 0x08) as usize;
    let final_ptr = rd::<usize>(vec_60_ptr);
    let final_end = final_ptr.wrapping_add(final_len * 0x28);
    let mut cur = final_ptr;
    while cur != final_end {
        let _ = crate::simd_ops::atomic_xchg_u64((cur + 0x08) as u64, 0);
        cur = cur.wrapping_add(0x28);
    }
    let free_ptr = rd::<usize>(vec_60_ptr);
    if free_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (free_ptr));
    }
}

/// `unknown_141fd6a00`
/// Original: 0x141fd6a00 (1056 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fd6a00(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let rsi = rd::<usize>(param_1 + 0x38);
    let r15 = rd::<usize>(param_1 + 0x08);
    let mut local_res8: u64 = u64::MAX;
    let mut rdx_id = rd::<u64>(r15.wrapping_add(rsi.wrapping_mul(8)));
    if (rdx_id as u32) == u32::MAX {
        rdx_id = u64::MAX;
    }

    let mut buf_20 = [0u8; 0x20];
    let b20 = buf_20.as_mut_ptr() as usize;
    wr::<[u64; 3]>(b20, rd::<[u64; 3]>(param_2));

    exe::call_va!(
        0x1_4020_d4a0,
        ((&mut local_res8 as *mut u64) as usize, rdx_id, b20)
    );
    if (local_res8 as u32) == u32::MAX {
        return;
    }

    let rbx = rd::<usize>(param_1).wrapping_add(rsi.wrapping_mul(0x30));
    exe::call_va!(0x1_4103_9060, (param_2, b20, local_res8));

    let rax_20 = rd::<usize>(b20);
    let rcx_30 = rd::<usize>(b20 + 0x10);
    let r14 = rd::<usize>(rax_20.wrapping_add(rcx_30.wrapping_mul(0x10)));
    let r11 = rd::<usize>(r14 + 0x18);
    let r12d = exe::call_va!(
        0x1_4248_9d30,
        (
            rd::<u64>(r11 + 0x18),
            rd::<u16>(r14 + 0x04) as u32,
            rd::<u32>(rbx)
        ) -> u32
    );
    if r12d == u32::MAX {
        return;
    }

    let idx6 = rd::<u8>(r14 + 0x06) as usize;
    exe::call_va!(
        0x1_421c_0320,
        (
            b20,
            rd::<u64>(r14.wrapping_add(0x08).wrapping_add(idx6 * 8)),
            rd::<u64>(r11 + 0x10),
            r12d
        )
    );

    let c_ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);

    let xmm10_1 = rd::<u128>(b20);
    let ymm4_lo_1 = rd::<u128>(rbx + 0x10);
    let xmm8_1 = rd::<u128>(rbx + 0x20);
    let idx7 = rd::<u8>(r14 + 0x07) as usize;
    let arg_r8_2 = rd::<u64>(rd::<usize>(r14 + 0x18) + 0x10);
    let arg_rdx_2 = rd::<u64>(r14.wrapping_add(0x08).wrapping_add(idx7 * 8));

    let xmm0 = vpermilps(xmm8_1, 0xc9);
    let xmm1 = vpermilps(xmm8_1, 0xd2);
    let xmm9 = vpermilps(xmm10_1, 0xc9);
    let xmm2 = vmulps(xmm1, xmm9);
    let xmm7 = vpermilps(xmm10_1, 0xd2);
    let xmm3 = vmulps(xmm0, xmm7);
    let xmm0 = vsubps(xmm2, xmm3);
    let xmm6 = vmulps(xmm0, c_ab0);
    let xmm3 = vpermilps(ymm4_lo_1, 0x11);
    let xmm1 = vmulps(xmm10_1, xmm3);
    let xmm2 = vpermilps(ymm4_lo_1, 0xbb);
    let xmm5 = vpermilps(xmm10_1, 0x1b);
    let xmm0 = vmulps(xmm2, xmm5);
    let xmm4 = vhsubps(xmm1, xmm0);
    let xmm0 = vmulps(xmm3, xmm5);
    let xmm2 = vmulps(xmm10_1, xmm2);
    let xmm3 = vhaddps(xmm2, xmm0);
    let xmm0 = vshufps(xmm4, xmm3, 0xb1);
    let xmm1 = vblendps(xmm3, xmm4, 0x0c);
    let xmm1 = vaddsubps(xmm1, xmm0);
    let out70_lo = vpermilps(xmm1, 0x9c);
    let xmm0 = vpermilps(xmm6, 0xc9);
    let xmm3 = vmulps(xmm0, xmm7);
    let xmm1 = vpermilps(xmm6, 0xd2);
    let xmm2 = vmulps(xmm1, xmm9);
    let xmm0 = vpermilps(xmm10_1, 0xff);
    let xmm1 = vmulps(xmm0, xmm6);
    let xmm4 = vsubps(xmm2, xmm3);
    let xmm2 = vaddps(xmm1, xmm8_1);
    let xmm3 = vaddps(xmm4, xmm2);
    let out80_hi = vaddps(xmm3, rd::<u128>(b20 + 0x10));

    exe::call_va!(0x1_421c_0320, (b20, arg_rdx_2, arg_r8_2, r12d));

    let xmm10_2 = rd::<u128>(b20);
    let ymm4_lo_2 = rd::<u128>(rbx + 0x10);
    let xmm9_2 = rd::<u128>(rbx + 0x20);
    let xmm0 = vpermilps(xmm9_2, 0xd2);
    let xmm1 = vpermilps(xmm9_2, 0xc9);
    let xmm5 = vpermilps(xmm10_2, 0x1b);
    let xmm8 = vpermilps(xmm10_2, 0xd2);
    let xmm2 = vmulps(xmm1, xmm8);
    let xmm7 = vpermilps(xmm10_2, 0xc9);
    let xmm3 = vmulps(xmm0, xmm7);
    let xmm0 = vsubps(xmm3, xmm2);
    let xmm6 = vmulps(xmm0, c_ab0);
    let xmm3 = vpermilps(ymm4_lo_2, 0x11);
    let xmm2 = vpermilps(ymm4_lo_2, 0xbb);
    let xmm0 = vmulps(xmm2, xmm5);
    let xmm1 = vmulps(xmm10_2, xmm3);
    let xmm4 = vhsubps(xmm1, xmm0);
    let xmm0 = vmulps(xmm3, xmm5);
    let xmm2 = vmulps(xmm10_2, xmm2);
    let xmm3 = vhaddps(xmm2, xmm0);
    let xmm1 = vblendps(xmm3, xmm4, 0x0c);
    let xmm0 = vshufps(xmm4, xmm3, 0xb1);
    let xmm1 = vaddsubps(xmm1, xmm0);
    let out90_lo = vpermilps(xmm1, 0x9c);
    let xmm0 = vpermilps(xmm6, 0xd2);
    let xmm3 = vmulps(xmm0, xmm7);
    let xmm1 = vpermilps(xmm6, 0xc9);
    let xmm2 = vmulps(xmm1, xmm8);
    let xmm0 = vpermilps(xmm10_2, 0xff);
    let xmm1 = vmulps(xmm0, xmm6);
    let xmm4 = vsubps(xmm3, xmm2);
    let xmm2 = vaddps(xmm1, xmm9_2);
    let xmm3 = vaddps(xmm4, xmm2);
    let outa0_hi = vaddps(xmm3, rd::<u128>(b20 + 0x10));

    let p30 = rd::<u64>(param_1 + 0x30);
    let mut arg1_a: [u64; 5] = [
        r15 as u64,
        rd::<u64>(param_1 + 0x10),
        rd::<u64>(param_1 + 0x18),
        p30,
        rsi as u64,
    ];
    let mut arg2_a = rd::<[u64; 3]>(param_3);
    let mut arg3_a: [u128; 2] = [out70_lo, out80_hi];
    exe::call_va!(
        0x1_41e2_3170,
        (
            arg1_a.as_mut_ptr() as usize,
            arg2_a.as_mut_ptr() as usize,
            arg3_a.as_mut_ptr() as usize
        )
    );

    let mut arg1_b: [u64; 5] = [
        r15 as u64,
        rd::<u64>(param_1 + 0x20),
        rd::<u64>(param_1 + 0x28),
        p30,
        rsi as u64,
    ];
    let mut arg2_b = rd::<[u64; 3]>(param_4);
    let mut arg3_b: [u128; 2] = [out90_lo, outa0_hi];
    exe::call_va!(
        0x1_41e2_2e20,
        (
            arg1_b.as_mut_ptr() as usize,
            arg2_b.as_mut_ptr() as usize,
            arg3_b.as_mut_ptr() as usize
        )
    );
}

/// `unknown_141fe4aa0`
/// Original: 0x141fe4aa0 (337 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fe4aa0(param_1: usize, param_2: usize) {
    let r12 = rd::<usize>(param_1 + 0x18);
    if r12 == 0 {
        return;
    }

    let vftbl = exe::ptr::<u8>(0x1_433c_d390) as u64;
    let mut raycaster = [0u64; 3];
    let rc_ptr = raycaster.as_mut_ptr() as usize;

    let mut rbp = 0usize;
    let mut r15 = 0usize;
    let mut r14 = 0usize;
    while rbp < r12 {
        let rbx_arr = rd::<usize>(param_1 + 0x28);
        let rdx_slot = rd::<usize>(param_1 + 0x30).wrapping_add(r14);
        let flags_arr = rd::<usize>(param_1 + 0x38);
        if rd::<u32>(flags_arr.wrapping_add(rbp * 4)) == 0 {
            wr::<u64>(rc_ptr, vftbl);
            let p48 = rd::<usize>(param_1 + 0x48);
            if p48 != 0 {
                let ecx = rd::<u32>(p48.wrapping_add(rbp * 4));
                wr::<u64>(rc_ptr + 0x08, rd::<u64>(param_2));
                wr::<u32>(rc_ptr + 0x10, ecx);
            }
            let obj_hi = rd::<usize>(rbx_arr.wrapping_add(r15).wrapping_add(0x08));
            wr::<usize>(obj_hi + 0x68, rc_ptr);
            let rbx = rd::<usize>(rbx_arr.wrapping_add(r15));
            wr::<usize>(rbx + 0xb0, rdx_slot);
            wr::<u8>(rbx + 0x3d, 0);
            if rd::<u8>(rbx + 0x3c) != 0 {
                exe::call_va!(
                    0x1_421c_b380,
                    (rbx, rd::<u64>(rbx + 0x28), rd::<u32>(rbx + 0x38))
                );
            }
            let rax_start = rd::<u64>(rbx + 0x50);
            let esi_end = rd::<u32>(rbx + 0x48) as u64;
            if rax_start < esi_end {
                let mut rdi_off = (rax_start as usize).wrapping_shl(4);
                let mut left = esi_end - rax_start;
                while left != 0 {
                    let arr_40 = rd::<usize>(rbx + 0x40);
                    let fn_ptr = rd::<usize>(arr_40.wrapping_add(rdi_off).wrapping_add(0x08));
                    let arg_edx = rd::<u32>(arr_40.wrapping_add(rdi_off).wrapping_add(0x04));
                    exe::call_va!(fn_ptr, (rbx, arg_edx));
                    rdi_off = rdi_off.wrapping_add(0x10);
                    left -= 1;
                }
            }
        }
        rbp += 1;
        r15 = r15.wrapping_add(0x10);
        r14 = r14.wrapping_add(0x48);
    }
}

/// `unknown_141fe4c30`
/// Original: 0x141fe4c30 (1295 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fe4c30(param_1: usize, param_2: usize, param_3: usize) {
    let xmm9_default = rd::<u32>(param_3);
    if rd::<u64>(param_1 + 0x18) == 0 {
        return;
    }

    let c_b00 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7b00) as usize);
    let c_760 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
    let xmm8 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let vftbl = exe::ptr::<u8>(0x1_433c_d390) as u64;

    let mut raycaster = [0u64; 3];
    let rc_ptr = raycaster.as_mut_ptr() as usize;

    let mut rdi = 0usize;
    let mut r15 = 0usize;
    let mut r9 = 0usize;
    let mut rcx_off = 0usize;
    let mut rdx_off = 0usize;
    let mut r14 = 0usize;
    let mut r8_off = 0usize;

    loop {
        let p20 = rd::<usize>(param_1 + 0x20);
        let rbx_28 = rd::<usize>(param_1 + 0x28);
        let r12 = rd::<usize>(param_1 + 0x30).wrapping_add(r9);
        let rsi_38 = rd::<usize>(param_1 + 0x38);
        let p58 = rd::<usize>(param_1 + 0x58);
        let p60 = rd::<usize>(param_1 + 0x60);
        let p70 = rd::<usize>(param_1 + 0x70);
        let r11_78 = rd::<usize>(param_1 + 0x78);
        let r10_88 = rd::<usize>(param_1 + 0x88);
        let rax_98 = rd::<usize>(param_1 + 0x98);
        let p_a0 = rd::<usize>(param_1 + 0xa0);
        let p_a8 = rd::<usize>(param_1 + 0xa8);

        if rd::<u32>(rax_98.wrapping_add(rdi * 4)) == 0 {
            let src_88 = r10_88.wrapping_add(rcx_off);
            let ymm2_lo = rd::<u128>(src_88);
            let ymm2_hi = rd::<u128>(src_88 + 0x10);
            let obj_rcx = rd::<usize>(rbx_28.wrapping_add(r14).wrapping_add(0x08));

            if p58 != 0 {
                wr::<usize>(obj_rcx + 0x60, p58.wrapping_add(rdx_off));
            }
            if p70 != 0 {
                let x = rd::<u128>(p70.wrapping_add(r14)) ^ c_b00;
                wr::<u128>(obj_rcx + 0xd0, x);
            }

            wr::<u64>(rc_ptr, vftbl);
            let edx = if p60 != 0 {
                rd::<u32>(p60.wrapping_add(rdi * 4))
            } else {
                0
            };
            wr::<u64>(rc_ptr + 0x08, rd::<u64>(param_2));
            wr::<u32>(rc_ptr + 0x10, edx);

            let mut xmm0_48 = xmm9_default;
            if p_a8 != 0 {
                let elem_a8 = p_a8.wrapping_add(rdi * 8);
                if elem_a8 != 0 {
                    xmm0_48 = rd::<u32>(elem_a8);
                }
            }
            wr::<u32>(obj_rcx + 0x48, xmm0_48);
            wr::<u128>(obj_rcx + 0x70, ymm2_lo);
            wr::<u128>(obj_rcx + 0x80, ymm2_hi);

            let xmm7 = ymm2_lo ^ c_b00;
            let xmm6 = ymm2_hi ^ c_760;
            let xmm0 = vpermilps(xmm6, 0xc9);
            let xmm4 = vpermilps(xmm7, 0xd2);
            let xmm2 = vmulps(xmm0, xmm4);
            let xmm1 = vpermilps(xmm6, 0xd2);
            let xmm5 = vpermilps(xmm7, 0xc9);
            let xmm0 = vmulps(xmm1, xmm5);
            let xmm0 = vsubps(xmm0, xmm2);
            let xmm3 = vmulps(xmm0, xmm8);
            let xmm1 = vpermilps(xmm3, 0xc9);
            let xmm4 = vmulps(xmm1, xmm4);
            let xmm0 = vpermilps(xmm3, 0xd2);
            let xmm2 = vmulps(xmm0, xmm5);
            let xmm0 = vpermilps(xmm7, 0xff);
            let xmm1 = vmulps(xmm0, xmm3);
            let xmm3 = vaddps(xmm1, xmm6);
            let xmm2 = vsubps(xmm2, xmm4);
            let xmm0 = vaddps(xmm3, xmm2);
            wr::<u128>(obj_rcx + 0x90, xmm7);
            wr::<u128>(obj_rcx + 0xa0, xmm0);

            let p20_cur = p20.wrapping_add(r15);
            wr::<u128>(obj_rcx + 0xe0, rd::<u128>(p20_cur + 0x20));
            wr::<u128>(obj_rcx + 0xb0, rd::<u128>(p20_cur));
            wr::<u128>(obj_rcx + 0xc0, rd::<u128>(p20_cur + 0x10));

            let p78_cur = r11_78.wrapping_add(r8_off);
            let xmm1_140 = rd::<u32>(p78_cur + 0x20);
            wr::<u128>(obj_rcx + 0x120, rd::<u128>(p78_cur));
            wr::<u128>(obj_rcx + 0x130, rd::<u128>(p78_cur + 0x10));
            wr::<u32>(obj_rcx + 0x140, xmm1_140);

            wr::<usize>(obj_rcx + 0x68, rc_ptr);
            wr::<u8>(obj_rcx + 0x15a, rd::<u8>(obj_rcx + 0x15a) & 0xef);

            let rsi = rd::<usize>(rsi_38.wrapping_add(r14));
            let old_b6 = rd::<u8>(rsi + 0x06);
            wr::<u8>(rsi + 0x07, old_b6);
            let new_b6 = old_b6 ^ 1;
            wr::<u8>(rsi + 0x06, new_b6);
            wr::<u32>(rsi, rd::<u32>(rsi).wrapping_add(1));

            let copy_len = (rd::<u16>(rsi + 0x04) as usize).wrapping_shl(5);
            let src_ptr = rd::<usize>(rd::<usize>(rsi + 0x18) + 0x08);
            let dst_ptr = rd::<usize>(rsi.wrapping_add((new_b6 as usize) * 8).wrapping_add(0x08));
            exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, copy_len));

            let old_w0 = rd::<u16>(r12);
            wr::<u16>(r12 + 0x02, old_w0);
            let new_w0 = old_w0 ^ 1;
            wr::<u16>(r12, new_w0);
            let r12_slot = r12.wrapping_add((new_w0 as usize) * 0x10);
            let memset_cnt = rd::<u32>(r12_slot + 0x10) as usize;
            if memset_cnt != 0 {
                exe::call_va!(0x1_4183_acc2, (rd::<usize>(r12_slot + 0x08), 0u32, memset_cnt * 4));
            }

            let rbx = rd::<usize>(rbx_28.wrapping_add(r14));
            let rsi_18 = rd::<usize>(rsi + 0x18);
            wr::<u64>(rbx + 0x98, rd::<u64>(rsi_18 + 0x08));
            let b7 = rd::<u8>(rsi + 0x07) as usize;
            wr::<u64>(rbx + 0xa0, rd::<u64>(rsi.wrapping_add(b7 * 8).wrapping_add(0x08)));
            let b6 = rd::<u8>(rsi + 0x06) as usize;
            wr::<u64>(rbx + 0x90, rd::<u64>(rsi.wrapping_add(b6 * 8).wrapping_add(0x08)));
            wr::<usize>(rbx + 0xb0, r12);
            let rsi_18_2 = rd::<usize>(rsi + 0x18);
            wr::<u64>(rbx + 0xa8, rd::<u64>(rsi_18_2 + 0x10));
            wr::<u8>(rbx + 0x3d, 1);

            let p68 = rd::<usize>(param_1 + 0x68);
            let al = if p68 != 0 {
                rd::<u8>(p68.wrapping_add(r8_off).wrapping_add(0x22))
            } else {
                0
            };
            if rd::<u8>(p_a0.wrapping_add(rdi)) != 0 && al == 0 {
                wr::<u8>(rbx + 0x103, 1);
            }

            exe::call_va!(0x1_421c_dbf0, (rbx));

            let cl = rd::<u8>(rbx + 0x103);
            wr::<f32>(p20_cur + 0x30, (cl as u32) as f32);
            if cl != 0 {
                wr::<u8>(rsi + 0x07, rd::<u8>(rsi + 0x06));
                wr::<u16>(r12 + 0x02, rd::<u16>(r12));
                wr::<u8>(rbx + 0x103, 0);
            }

            if rd::<u8>(rbx + 0x102) != 0 {
                wr::<u128>(p20_cur, rd::<u128>(rbx + 0xd0));
                wr::<u128>(p20_cur + 0x10, rd::<u128>(rbx + 0xe0));
                let xmm6_f0 = rd::<u128>(rbx + 0xf0);
                let xmm7_q = ymm2_lo;
                let xmm0 = vshufps(xmm6_f0, xmm6_f0, 0xc9);
                let xmm4 = vshufps(xmm7_q, xmm7_q, 0xd2);
                let xmm2 = vmulps(xmm4, xmm0);
                let xmm1 = vshufps(xmm6_f0, xmm6_f0, 0xd2);
                let xmm5 = vshufps(xmm7_q, xmm7_q, 0xc9);
                let xmm0 = vmulps(xmm5, xmm1);
                let xmm0 = vsubps(xmm0, xmm2);
                let xmm3 = vmulps(xmm0, xmm8);
                let xmm1 = vpermilps(xmm3, 0xc9);
                let xmm4 = vmulps(xmm4, xmm1);
                let xmm0 = vpermilps(xmm3, 0xd2);
                let xmm2 = vmulps(xmm5, xmm0);
                let xmm0 = vshufps(xmm7_q, xmm7_q, 0xff);
                let xmm1 = vmulps(xmm0, xmm3);
                let xmm3 = vaddps(xmm1, xmm6_f0);
                let xmm2 = vsubps(xmm2, xmm4);
                let xmm0 = vaddps(xmm3, xmm2);
                wr::<u128>(p20_cur + 0x20, xmm0);
            }

            let p40 = rd::<usize>(param_1 + 0x40);
            if p40 != 0 {
                wr::<u64>(p40.wrapping_add(rdi * 8), rd::<u64>(rbx + 0xc0));
            }
            let p48 = rd::<usize>(param_1 + 0x48);
            if p48 != 0 {
                wr::<u64>(p48.wrapping_add(rdi * 8), rd::<u64>(rbx + 0xb8));
            }
            let p50 = rd::<usize>(param_1 + 0x50);
            if p50 != 0 {
                wr::<u32>(p50.wrapping_add(rdi * 8), rd::<u8>(rbx + 0x100) as u32);
                wr::<u32>(p50.wrapping_add(rdi * 8 + 4), rd::<u8>(rbx + 0x101) as u32);
            }
        }

        rdi += 1;
        r15 = r15.wrapping_add(0x40);
        r9 = r9.wrapping_add(0x48);
        rcx_off = rcx_off.wrapping_add(0x20);
        rdx_off = rdx_off.wrapping_add(0x1010);
        r14 = r14.wrapping_add(0x10);
        r8_off = r8_off.wrapping_add(0x30);
        if rdi >= rd::<usize>(param_1 + 0x18) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn skip_ctrl_neg_40(mut ctrl: usize, mut slot: usize) -> (usize, usize) {
    if rd::<i8>(ctrl) < -1 {
        let all_ff = u128::MAX;
        loop {
            let cmp = vpcmpgtb(all_ff, rd::<u128>(ctrl));
            let mask = (vpmovmskb(cmp) as u32).wrapping_add(1);
            let step = if mask != 0 {
                mask.trailing_zeros() as usize
            } else {
                0
            };
            ctrl = ctrl.wrapping_add(step);
            slot = slot.wrapping_add(step.wrapping_mul(0x40));
            if rd::<i8>(ctrl) >= -1 {
                break;
            }
        }
    }
    (ctrl, slot)
}

/// `unknown_142088c20`
/// Original: 0x142088c20 (614 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_142088c20(param_1: usize) {
    if rd::<u64>(param_1 + 0xa8) == 0 {
        return;
    }

    let mut qpc_start: i64 = 0;
    exe::call_iat!(0x1_4264_56f8, ((&mut qpc_start as *mut i64) as usize));
    let freq0 = exe::call_va!(0x1_4240_6920, () -> i64);
    let r14 = param_1.wrapping_add(0x98);
    let rdx_ctrl = rd::<usize>(r14);
    let rbx_slots = rd::<usize>(r14 + 0x08);
    let rbp = qpc_start.wrapping_sub(((freq0 as f64) * 0.0f64) as i64);

    let (mut rdi, mut rbx) = skip_ctrl_neg_40(rdx_ctrl, rbx_slots);
    let ctrl_end = rdx_ctrl.wrapping_add(rd::<usize>(param_1 + 0xb0));

    if rdi != ctrl_end {
        loop {
            let mut qpc_now: i64 = 0;
            exe::call_iat!(0x1_4264_56f8, ((&mut qpc_now as *mut i64) as usize));
            let elapsed = qpc_now.wrapping_sub(rbp) as f64;
            let freq = exe::call_va!(0x1_4240_6920, () -> i64) as f64;
            let budget = rd::<f32>(exe::ptr::<f32>(0x1_4385_8038) as usize) as f64;
            let ratio = elapsed / freq;
            if !(budget > ratio) {
                break;
            }
            let d8_inc = rd::<u64>(param_1 + 0xd8).wrapping_add(1);
            let mut slot_18 = rd::<u64>(rbx + 0x18);
            wr::<u64>(rbx + 0x18, 0);
            exe::call_va!(
                0x1_4208_8af0,
                (
                    param_1,
                    (&mut slot_18 as *mut u64) as usize,
                    rbx.wrapping_add(0x20),
                    rbx.wrapping_add(0x30),
                    d8_inc
                )
            );
            let next = skip_ctrl_neg_40(rdi.wrapping_add(1), rbx.wrapping_add(0x40));
            rdi = next.0;
            rbx = next.1;
            let cur_end = rd::<usize>(r14).wrapping_add(rd::<usize>(r14 + 0x18));
            if rdi == cur_end {
                break;
            }
        }
    }

    let (mut cur_ctrl, mut cur_slot) = skip_ctrl_neg_40(rd::<usize>(r14), rd::<usize>(r14 + 0x08));
    if cur_ctrl != rdi {
        loop {
            let mut erase_it: [usize; 2] = [cur_ctrl, cur_slot];
            let next = skip_ctrl_neg_40(cur_ctrl.wrapping_add(1), cur_slot.wrapping_add(0x40));
            exe::call_va!(0x1_4208_4c50, (r14, erase_it.as_mut_ptr() as usize));
            if next.0 == rdi {
                break;
            }
            cur_ctrl = next.0;
            cur_slot = next.1;
        }
    }
}

/// `unknown_1420892a0`
/// Original: 0x1420892a0 (1219 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420892a0(param_1: usize, param_2: usize, param_3: u64) {
    exe::call_va!(0x1_4208_9d10, (param_1));

    if rd::<u8>(param_2 + 0x44) != 0 {
        let res_obj = rd::<usize>(param_2);
        let mut desc_80 = [0u8; 0x48];
        let d80_ptr = desc_80.as_mut_ptr() as usize;
        exe::call_va!(rd::<usize>(rd::<usize>(res_obj) + 0x50), (res_obj, d80_ptr));

        let mut vec_60 = [0u64; 2];
        let mut vec_70 = [0u64; 2];
        let num_mips = rd::<u8>(param_2 + 0x44) as u32;
        exe::call_va!(0x1_411d_fc30, (vec_60.as_mut_ptr() as usize, num_mips));
        exe::call_va!(0x1_411d_fce0, (vec_70.as_mut_ptr() as usize, num_mips));

        let mut total_bytes: u64 = 0;
        let r14 = vec_60[0] as usize;
        let rsi = vec_70[0] as usize;
        let dev1 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        exe::call_va!(
            rd::<usize>(rd::<usize>(dev1) + 0x130),
            (
                dev1,
                d80_ptr,
                rd::<u8>(param_2 + 0x48) as u32,
                rd::<u8>(param_2 + 0x44) as u32,
                0u64,
                r14,
                rsi,
                0u64,
                (&mut total_bytes as *mut u64) as usize
            )
        );

        let mut heap_props = [0u8; 0x18];
        let hp_ptr = heap_props.as_mut_ptr() as usize;
        wr::<u64>(hp_ptr, 2);
        wr::<u32>(hp_ptr + 0x08, 0);
        wr::<u32>(hp_ptr + 0x0c, 1);
        wr::<u32>(hp_ptr + 0x10, 1);

        let aligned_bytes = (total_bytes.wrapping_add(0x3ff)) & !0x3ffu64;
        let mut buf_desc = [0u8; 0x38];
        let bd_ptr = buf_desc.as_mut_ptr() as usize;
        wr::<u32>(bd_ptr, 1);
        wr::<u64>(bd_ptr + 0x08, 0);
        wr::<u64>(bd_ptr + 0x10, aligned_bytes);
        wr::<u32>(bd_ptr + 0x18, 1);
        wr::<u64>(bd_ptr + 0x1c, 0x0001_0001);
        wr::<u64>(bd_ptr + 0x24, 1);
        wr::<u64>(bd_ptr + 0x2c, 1);

        let mut upload_res: usize = 0;
        let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        let iid_res = exe::ptr::<u8>(0x1_433d_c7e8) as usize;
        let hr1 = exe::call_va!(
            rd::<usize>(rd::<usize>(dev2) + 0xd8),
            (
                dev2,
                hp_ptr,
                0u32,
                bd_ptr,
                0xac3u32,
                0usize,
                iid_res,
                (&mut upload_res as *mut usize) as usize
            ) -> u32
        );
        if hr1 == 0x8007_000e || hr1.wrapping_add(0x7785_fffb) <= 2 {
            exe::call_va!(0x1_4205_d410, (0usize, hr1));
        }

        let mut mapped_ptr: usize = 0;
        let hr2 = exe::call_va!(
            rd::<usize>(rd::<usize>(upload_res) + 0x40),
            (
                upload_res,
                0u32,
                0usize,
                (&mut mapped_ptr as *mut usize) as usize
            ) -> u32
        );
        if hr2 == 0x8007_000e || hr2.wrapping_add(0x7785_fffb) <= 2 {
            exe::call_va!(0x1_4205_d410, (0usize, hr2));
        }

        let mut src_offset: u64 = 0;
        let mut r12b: u8 = 0;
        if rd::<u8>(param_2 + 0x44) > 0 {
            loop {
                let shift = (r12b as u32) & 0x1f;
                let r11d = rd::<u32>(param_2 + 0x3c) >> shift;
                let edx = rd::<u32>(param_2 + 0x40) >> shift;
                let r8d = rd::<u8>(param_2 + 0x38) as u32;
                let ebx = exe::call_va!(0x1_4204_e7f0, (r11d, edx, r8d) -> u32);
                let r8d2 = rd::<u8>(param_2 + 0x38) as u32;
                let r10d = exe::call_va!(0x1_4204_e7f0, (r11d, 1u32, r8d2) -> u32);
                let rows_div = ebx / r10d;
                let next_src_offset = src_offset.wrapping_add(ebx as u64);
                if next_src_offset <= rd::<u64>(param_2 + 0x10) {
                    let r13_idx = r12b as usize;
                    let r15_num_rows_ptr = rsi.wrapping_add(r13_idx * 4);
                    let init_rows = rd::<u32>(r15_num_rows_ptr).min(rows_div);
                    if init_rows > 0 {
                        let fp_off = r13_idx.wrapping_shl(5);
                        let row_stride = r10d as usize;
                        let mut row_idx: usize = 0;
                        let mut row_src_off: usize = 0;
                        loop {
                            let r10_fp = vec_60[0] as usize;
                            let row_pitch =
                                rd::<u32>(r10_fp.wrapping_add(fp_off).wrapping_add(0x18)) as usize;
                            let src_base = rd::<usize>(param_2 + 0x08);
                            let copy_size = row_pitch.min(row_stride);
                            let src_addr = src_base
                                .wrapping_add(row_src_off)
                                .wrapping_add(src_offset as usize);
                            let dst_addr = row_pitch
                                .wrapping_mul(row_idx)
                                .wrapping_add(mapped_ptr)
                                .wrapping_add(rd::<usize>(r10_fp.wrapping_add(fp_off)));
                            exe::call_va!(0x1_4183_acbc, (dst_addr, src_addr, copy_size));
                            row_idx += 1;
                            row_src_off = row_src_off.wrapping_add(row_stride);
                            let cur_limit = rd::<u32>(r15_num_rows_ptr).min(rows_div) as usize;
                            if row_idx >= cur_limit {
                                break;
                            }
                        }
                    }
                }
                src_offset = next_src_offset;
                r12b = r12b.wrapping_add(1);
                if r12b >= rd::<u8>(param_2 + 0x44) {
                    break;
                }
            }
        }

        exe::call_va!(
            rd::<usize>(rd::<usize>(upload_res) + 0x48),
            (upload_res, 0u32, 0usize)
        );

        let mut src_loc = [0u8; 0x30];
        let mut dst_loc = [0u8; 0x30];
        let sl_ptr = src_loc.as_mut_ptr() as usize;
        let dl_ptr = dst_loc.as_mut_ptr() as usize;
        wr::<usize>(sl_ptr, upload_res);
        wr::<u32>(sl_ptr + 0x08, 1);
        wr::<u64>(dl_ptr, rd::<u64>(param_2));
        wr::<u32>(dl_ptr + 0x08, 0);

        let mut sil: u8 = 0;
        if rd::<u8>(param_2 + 0x44) > 0 {
            loop {
                let fp_src = r14.wrapping_add((sil as usize).wrapping_shl(5));
                wr::<[u8; 0x20]>(sl_ptr + 0x10, rd::<[u8; 0x20]>(fp_src));
                let subres = (rd::<u8>(param_2 + 0x48) as u32).wrapping_add(sil as u32);
                wr::<u32>(dl_ptr + 0x10, subres);
                let cmd_list = rd::<usize>(param_1 + 0x58);
                exe::call_va!(
                    rd::<usize>(rd::<usize>(cmd_list) + 0x80),
                    (cmd_list, dl_ptr, 0u32, 0u32, 0u32, sl_ptr, 0usize)
                );
                sil = sil.wrapping_add(1);
                if sil >= rd::<u8>(param_2 + 0x44) {
                    break;
                }
            }
        }

        let rsi_vec = param_1.wrapping_add(0x68);
        let len = rd::<u32>(rsi_vec + 0x08) as u64;
        let cap = rd::<u32>(rsi_vec + 0x0c) as u64;
        if (len << 4) == (cap << 4) {
            let new_cap = (cap + (cap >> 1)).max(len + 1);
            exe::call_va!(0x1_411e_1390, (rsi_vec, new_cap));
        }
        let idx = rd::<u32>(rsi_vec + 0x08) as usize;
        let slot = rd::<usize>(rsi_vec).wrapping_add(idx * 0x10);
        wr::<usize>(slot, upload_res);
        wr::<u64>(slot + 0x08, param_3);
        wr::<u32>(rsi_vec + 0x08, (idx as u32).wrapping_add(1));

        let p70_free = vec_70[0] as usize;
        if p70_free != 0 {
            exe::call_va!(0x1_4241_c190, (p70_free));
        }
        if r14 != 0 {
            exe::call_va!(0x1_4241_c190, (r14));
        }
    }

    let rsi_vec = param_1.wrapping_add(0x68);
    let rbx_res = rd::<u64>(param_2);
    wr::<u64>(param_2, 0);
    let len = rd::<u32>(rsi_vec + 0x08) as u64;
    let cap = rd::<u32>(rsi_vec + 0x0c) as u64;
    if (len << 4) == (cap << 4) {
        let new_cap = (cap + (cap >> 1)).max(len + 1);
        exe::call_va!(0x1_411e_1390, (rsi_vec, new_cap));
    }
    let idx = rd::<u32>(rsi_vec + 0x08) as usize;
    let slot = rd::<usize>(rsi_vec).wrapping_add(idx * 0x10);
    wr::<u64>(slot, rbx_res);
    wr::<u64>(slot + 0x08, param_3);
    wr::<u32>(rsi_vec + 0x08, (idx as u32).wrapping_add(1));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141f57ce0(orig: usize, new: usize) -> Result<u32, String> {
    static mut TRACE_HASH: u64 = 0;
    static mut HASH_TBL_PTR: usize = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_d2360(ptr: usize) {
        rec(0xd2360 ^ (rd::<u32>(ptr + 0x98) as u64));
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        rec(0x2de0 ^ (rd::<u32>(ptr + 0x08) as u64));
    }
    unsafe extern "win64" fn mock_extract(tree: usize, node: usize) -> usize {
        // Simple unlink from a 2-node list/tree where head is at [tree],
        // node is [head], and node's right child is node2.
        let head = rd::<usize>(tree);
        let next = rd::<usize>(node + 0x10);
        wr::<usize>(head, next);
        rec(0x7860);
        node
    }
    unsafe extern "win64" fn mock_free_node(_al: usize, _node: usize) {
        rec(0x66f0);
    }
    unsafe extern "win64" fn mock_get_tbl() -> usize {
        unsafe { HASH_TBL_PTR }
    }
    unsafe extern "win64" fn mock_slot_dtor(this: usize, flags: u32) {
        rec(0xdead_0000 ^ (rd::<u32>(this + 0x08) as u64) ^ (flags as u64));
    }
    #[cfg(not(target_arch = "aarch64"))]
    unsafe extern "win64" fn mock_slot_update(this: usize, dt: f32) {
        let next = rd::<f32>(this + 0x1c) + dt;
        wr::<f32>(this + 0x1c, next);
        rec(0x0808 ^ (next.to_bits() as u64));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe extern "win64" fn mock_slot_update(this: usize, _xmm0: f32, dt: f32) {
        let next = rd::<f32>(this + 0x1c) + dt;
        wr::<f32>(this + 0x1c, next);
        rec(0x0808 ^ (next.to_bits() as u64));
    }
    #[cfg(not(target_arch = "aarch64"))]
    unsafe extern "win64" fn mock_slot_apply(this: usize, val: f32) -> f32 {
        val + rd::<f32>(this + 0x30)
    }
    #[cfg(target_arch = "aarch64")]
    unsafe extern "win64" fn mock_slot_apply(this: usize, _xmm0: f32, val: f32) -> f32 {
        val + rd::<f32>(this + 0x30)
    }
    unsafe extern "win64" fn mock_slot_alive(this: usize) -> u8 {
        rd::<u8>(this + 0x10)
    }
    unsafe extern "win64" fn mock_hash_tick(this: usize) {
        let v = rd::<u32>(this + 0x08).wrapping_add(1);
        wr::<u32>(this + 0x08, v);
        rec(0x9999 ^ (v as u64));
    }

    let _m1 = crate::difftest::Mock::new(0x1_410d_2360, mock_d2360 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_403f_7860, mock_extract as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_66f0, mock_free_node as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4248_a640, mock_get_tbl as *const ())?;

    let run = |fn_addr: usize| -> (u64, u32, u32, u8, u32, usize, usize) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut slot_vtbl = [0usize; 6];
        slot_vtbl[0] = mock_slot_dtor as *const () as usize;
        slot_vtbl[1] = mock_slot_update as *const () as usize;
        slot_vtbl[2] = mock_slot_apply as *const () as usize;
        slot_vtbl[4] = mock_slot_alive as *const () as usize;

        // Slot 0: alive=1, type=1 (+0x08), progress=0.75 (+0x1c) -> +0.5 = 1.25 >= 1.0, val=2.5 (+0x30)
        let mut slot0 = [0u8; 0x40];
        let s0 = slot0.as_mut_ptr() as usize;
        wr::<usize>(s0, slot_vtbl.as_ptr() as usize);
        wr::<u32>(s0 + 0x08, 1);
        wr::<u8>(s0 + 0x10, 1);
        wr::<f32>(s0 + 0x1c, 0.75);
        wr::<f32>(s0 + 0x30, 2.5);

        // Slot 1: alive=0 (gets destroyed and zeroed)
        let mut slot1 = [0u8; 0x40];
        let s1 = slot1.as_mut_ptr() as usize;
        wr::<usize>(s1, slot_vtbl.as_ptr() as usize);
        wr::<u32>(s1 + 0x08, 2);
        wr::<u8>(s1 + 0x10, 0);
        wr::<f32>(s1 + 0x30, 10.0);

        // Slot 2: alive=1, adds +1.25 in apply
        let mut slot2 = [0u8; 0x40];
        let s2 = slot2.as_mut_ptr() as usize;
        wr::<usize>(s2, slot_vtbl.as_ptr() as usize);
        wr::<u32>(s2 + 0x08, 2);
        wr::<u8>(s2 + 0x10, 1);
        wr::<f32>(s2 + 0x1c, 0.1);
        wr::<f32>(s2 + 0x30, 1.25);

        let mut out_val: u32 = 0;

        // Payload 1: all 4 slots 0 -> erased in Loop 1
        let mut pay1 = [0u8; 0xd0];
        let p1_ptr = pay1.as_mut_ptr() as usize;
        wr::<u32>(p1_ptr + 0x08, 111);
        wr::<u32>(p1_ptr + 0x98, 222);

        // Payload 2: has slots s0, s1, s2, 0; a0 = u64::MAX
        let mut pay2 = [0u8; 0xd0];
        let p2_ptr = pay2.as_mut_ptr() as usize;
        wr::<usize>(p2_ptr + 0x90, (&mut out_val as *mut u32) as usize);
        wr::<f32>(p2_ptr + 0x98, 0.0);
        wr::<u8>(p2_ptr + 0x9c, 0);
        wr::<u64>(p2_ptr + 0xa0, u64::MAX);
        wr::<usize>(p2_ptr + 0xa8, s0);
        wr::<usize>(p2_ptr + 0xb0, s1);
        wr::<usize>(p2_ptr + 0xb8, s2);
        wr::<usize>(p2_ptr + 0xc0, 0);

        // RB-tree with head, node1 (erased in Loop 1), node2 (kept)
        let mut nodes = [[0u8; 0x40]; 3];
        let head = nodes[0].as_mut_ptr() as usize;
        let node1 = nodes[1].as_mut_ptr() as usize;
        let node2 = nodes[2].as_mut_ptr() as usize;

        wr::<usize>(head, node1);
        wr::<usize>(head + 0x08, node1);
        wr::<usize>(head + 0x10, node2);
        wr::<u8>(head + 0x19, 1);

        wr::<usize>(node1, head);
        wr::<usize>(node1 + 0x08, head);
        wr::<usize>(node1 + 0x10, node2);
        wr::<u8>(node1 + 0x19, 0);
        wr::<usize>(node1 + 0x30, p1_ptr);

        wr::<usize>(node2, head);
        wr::<usize>(node2 + 0x08, node1);
        wr::<usize>(node2 + 0x10, head);
        wr::<u8>(node2 + 0x19, 0);
        wr::<usize>(node2 + 0x30, p2_ptr);

        // Hash table for Loop 4
        let mut hash_vtbl = [0usize; 6];
        hash_vtbl[4] = mock_hash_tick as *const () as usize;
        let mut hash_obj = [hash_vtbl.as_ptr() as usize, 10usize];
        let mut buckets = [0u8; 0x30];
        let bp = buckets.as_mut_ptr() as usize;
        // Bucket 0: empty sentinel (0xffff_ffff)
        wr::<u32>(bp, 0xffff_ffff);
        // Bucket 1: active entry (key = 5)
        wr::<u32>(bp + 0x10, 5);
        wr::<usize>(bp + 0x18, hash_obj.as_mut_ptr() as usize);
        // Bucket 2: deleted sentinel (0xffff_fffe)
        wr::<u32>(bp + 0x20, 0xffff_fffe);

        let mut tbl = [0u8; 0x60];
        let tp = tbl.as_mut_ptr() as usize;
        wr::<u32>(tp + 0x24, 0xffff_fffe);
        wr::<u64>(tp + 0x28, 1);
        wr::<u64>(tp + 0x38, 3);
        wr::<u32>(tp + 0x40, 0xffff_ffff);
        wr::<usize>(tp + 0x50, bp);
        unsafe { HASH_TBL_PTR = tp };

        let tree_container = [head];
        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            tree_container.as_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0.5f32.to_bits() as u128, 0, 0],
        );

        (
            unsafe { TRACE_HASH },
            out_val,
            rd::<u32>(p2_ptr + 0x98),
            rd::<u8>(p2_ptr + 0x9c),
            rd::<u32>(hash_obj.as_ptr() as usize + 0x08),
            rd::<usize>(p2_ptr + 0xa8),
            rd::<usize>(p2_ptr + 0xb0),
        )
    };

    let o = run(orig);
    let n = run(new);
    crate::diff_eq!(o, n, "141f57ce0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f5b690(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_BUF: [u8; 0x28 * 8] = [0; 0x28 * 8];
    static mut AA00_CAPTURED: ([u8; 0x28 * 2], u32, usize, u8) = ([0; 0x28 * 2], 0, 0, 0);
    static mut FREE_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_32e0(rcx_plus_80: usize, p2: u64) -> usize {
        if p2 == 0x20 {
            rcx_plus_80
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_6540(vec_ptr: usize, new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC_BUF) as usize);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_8620(item: usize) {
        wr::<u32>(item + 0x10, 7);
    }
    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08));
        wr::<u32>(dst + 0x10, rd::<u32>(src + 0x10));
    }
    unsafe extern "win64" fn mock_38c0(rcx_plus_8: usize, key: u64) -> usize {
        rcx_plus_8.wrapping_add(key as usize)
    }
    unsafe extern "win64" fn mock_aa00(_unused: usize, rdx_slot: usize, vec_ptr: usize, flag: u8) {
        let ptr = rd::<usize>(vec_ptr);
        let len = rd::<u32>(vec_ptr + 0x08);
        unsafe {
            AA00_CAPTURED = (rd::<[u8; 0x28 * 2]>(ptr), len, rdx_slot, flag);
        }
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        if ptr == core::ptr::addr_of_mut!(VEC_BUF) as usize {
            unsafe { FREE_CALLS += 1 };
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_410d_32e0, mock_32e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_410d_6540, mock_6540 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_8620, mock_8620 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_410d_38c0, mock_38c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41f5_aa00, mock_aa00 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let run = |f: F| -> ([u64; 8], u32, [u8; 0x28 * 2], u32, usize, u8, u32, [u64; 2]) {
        unsafe {
            VEC_BUF = [0; 0x28 * 8];
            AA00_CAPTURED = ([0; 0x28 * 2], 0, 0, 0);
            FREE_CALLS = 0;
        }

        let mut clear_entries = [0x1111u64; 8];
        let mut p1 = [0u8; 0xa0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x80, clear_entries.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x88, 2);

        // 3 input items of 0x20 bytes each (items 0 and 2 share key 0xaaaa so duplicate branch runs)
        let mut items = [0u8; 0x20 * 3];
        let ip = items.as_mut_ptr() as usize;
        wr::<u64>(ip, 0xaaaa);
        wr::<u64>(ip + 0x08, 0x1001);
        wr::<u32>(ip + 0x10, 11);
        wr::<u8>(ip + 0x18, 0x0f);

        wr::<u64>(ip + 0x20, 0xbbbb);
        wr::<u64>(ip + 0x28, 0x1002);
        wr::<u32>(ip + 0x30, 22);
        wr::<u8>(ip + 0x38, 0x07);

        wr::<u64>(ip + 0x40, 0xaaaa);
        wr::<u64>(ip + 0x48, 0x1003);
        wr::<u32>(ip + 0x50, 33);
        wr::<u8>(ip + 0x58, 0x03);

        let p3: [u64; 2] = [3, ip as u64];
        f(p1p, 0x20, p3.as_ptr() as usize, 1);

        let (cap_arr, cap_len, cap_slot, cap_flag) = unsafe { AA00_CAPTURED };
        let vbuf_ptr = core::ptr::addr_of!(VEC_BUF) as usize;
        let cleared_ptrs = [rd::<u64>(vbuf_ptr + 0x08), rd::<u64>(vbuf_ptr + 0x28 + 0x08)];
        (
            clear_entries,
            rd::<u32>(p1p + 0x88),
            cap_arr,
            cap_len,
            cap_slot.wrapping_sub(p1p),
            cap_flag,
            unsafe { FREE_CALLS },
            cleared_ptrs,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141f5b690 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141fd6a00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut R14_PTR: usize = 0;
    static mut OUT_3170: ([u64; 5], [u64; 3], [u32; 8]) = ([0; 5], [0; 3], [0; 8]);
    static mut OUT_2E20: ([u64; 5], [u64; 3], [u32; 8]) = ([0; 5], [0; 3], [0; 8]);

    unsafe extern "win64" fn mock_d4a0(out_res: usize, rdx_id: u64, _b20: usize) {
        wr::<u64>(out_res, if rdx_id == u64::MAX { u64::MAX } else { 7 });
    }
    unsafe extern "win64" fn mock_9060(_p2: usize, b20: usize, res8: u64) {
        wr::<usize>(b20, core::ptr::addr_of!(R14_PTR) as usize);
        wr::<usize>(b20 + 0x10, if res8 == 7 { 0 } else { 1 });
    }
    unsafe extern "win64" fn mock_9d30(_arr: u64, _cnt: u16, want: u32) -> u32 {
        if want == 0x1234 {
            2
        } else {
            u32::MAX
        }
    }
    unsafe extern "win64" fn mock_0320(out_transform: usize, tag: u64, _info: u64, idx: u32) {
        let s = (tag as f32) * 0.1 + (idx as f32) * 0.25;
        // Normalized quaternion + translation
        wr::<[f32; 4]>(out_transform, [0.0, 0.707_106_77, 0.0, 0.707_106_77]);
        wr::<[f32; 4]>(out_transform + 0x10, [1.5 + s, -2.0 + s, 3.25 - s, 1.0]);
    }
    unsafe extern "win64" fn mock_3170(a1: usize, a2: usize, a3: usize) {
        unsafe {
            OUT_3170 = (rd::<[u64; 5]>(a1), rd::<[u64; 3]>(a2), rd::<[u32; 8]>(a3));
        }
    }
    unsafe extern "win64" fn mock_2e20(a1: usize, a2: usize, a3: usize) {
        unsafe {
            OUT_2E20 = (rd::<[u64; 5]>(a1), rd::<[u64; 3]>(a2), rd::<[u32; 8]>(a3));
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_d4a0, mock_d4a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_9060, mock_9060 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_9d30, mock_9d30 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_421c_0320, mock_0320 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41e2_3170, mock_3170 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e2_2e20, mock_2e20 as *const ())?;

    let run = |f: F, valid: bool| -> (([u64; 5], [u64; 3], [u32; 8]), ([u64; 5], [u64; 3], [u32; 8])) {
        unsafe {
            OUT_3170 = ([0; 5], [0; 3], [0; 8]);
            OUT_2E20 = ([0; 5], [0; 3], [0; 8]);
        }

        let mut r11_obj = [0u8; 0x20];
        wr::<u64>(r11_obj.as_mut_ptr() as usize + 0x10, 0x8888);
        wr::<u64>(r11_obj.as_mut_ptr() as usize + 0x18, 0x9999);

        let mut r14_obj = [0u8; 0x30];
        let r14p = r14_obj.as_mut_ptr() as usize;
        wr::<u16>(r14p + 0x04, 8);
        wr::<u8>(r14p + 0x06, 0);
        wr::<u8>(r14p + 0x07, 1);
        wr::<u64>(r14p + 0x08, 2);
        wr::<u64>(r14p + 0x10, 5);
        wr::<usize>(r14p + 0x18, r11_obj.as_mut_ptr() as usize);
        unsafe { R14_PTR = r14p };

        let mut rbx_arr = [0u8; 0x60];
        let rbxp = rbx_arr.as_mut_ptr() as usize + 0x30;
        wr::<u32>(rbxp, 0x1234);
        wr::<[f32; 4]>(rbxp + 0x10, [0.5, -0.5, 0.5, 0.5]);
        wr::<[f32; 4]>(rbxp + 0x20, [4.0, -1.5, 2.25, 0.0]);

        let ids: [u64; 2] = [0, if valid { 42 } else { u64::MAX }];
        let p1: [u64; 8] = [
            rbx_arr.as_mut_ptr() as u64,
            ids.as_ptr() as u64,
            0xaa01,
            0xaa02,
            0xbb01,
            0xbb02,
            0xcc00,
            1,
        ];
        let p2: [u64; 3] = [0x11, 0x22, 0x33];
        let p3: [u64; 3] = [0x101, 0x102, 0x103];
        let p4: [u64; 3] = [0x201, 0x202, 0x203];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
        );

        let (mut a1, a2, a3) = unsafe { OUT_3170 };
        let (mut b1, b2, b3) = unsafe { OUT_2E20 };
        a1[0] = 0;
        b1[0] = 0;
        ((a1, a2, a3), (b1, b2, b3))
    };

    for valid in [false, true] {
        let o = run(f_o, valid);
        let n = run(f_n, valid);
        crate::diff_eq!(o, n, "141fd6a00 valid={valid}");
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141fe4aa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_b380(_rbx: usize, rdx: u64, r8d: u32) {
        rec(0xb380 ^ rdx ^ (r8d as u64));
    }
    unsafe extern "win64" fn mock_cb(rbx: usize, edx: u32) {
        let obj_hi = rd::<usize>(rbx + 0x08);
        let rc_ptr = rd::<usize>(obj_hi + 0x68);
        rec(
            (edx as u64)
                ^ rd::<u64>(rc_ptr)
                ^ rd::<u64>(rc_ptr + 0x08)
                ^ (rd::<u32>(rc_ptr + 0x10) as u64),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_b380, mock_b380 as *const ())?;

    let run = |f: F| -> (u64, u8, usize) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };

        let mut cb_entries = [0u8; 0x20];
        let cbp = cb_entries.as_mut_ptr() as usize;
        wr::<u32>(cbp + 0x10 + 0x04, 0x77);
        wr::<usize>(cbp + 0x10 + 0x08, mock_cb as *const () as usize);

        let mut obj_hi = [0u8; 0x80];
        let mut obj_lo = [0u8; 0xc0];
        let olp = obj_lo.as_mut_ptr() as usize;
        let ohp = obj_hi.as_mut_ptr() as usize;
        wr::<usize>(olp + 0x08, ohp);
        wr::<u64>(olp + 0x28, 0x1234_5678);
        wr::<u32>(olp + 0x38, 0x9abc);
        wr::<u8>(olp + 0x3c, 1);
        wr::<u8>(olp + 0x3d, 1);
        wr::<usize>(olp + 0x40, cbp);
        wr::<u32>(olp + 0x48, 2);
        wr::<u64>(olp + 0x50, 1);

        let rbx_pairs: [usize; 4] = [olp, ohp, 0, 0];
        let flags: [u32; 2] = [0, 1];
        let p48_vals: [u32; 2] = [0x42, 0x99];

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 2);
        wr::<usize>(p1p + 0x28, rbx_pairs.as_ptr() as usize);
        wr::<usize>(p1p + 0x30, 0x50_0000);
        wr::<usize>(p1p + 0x38, flags.as_ptr() as usize);
        wr::<usize>(p1p + 0x48, p48_vals.as_ptr() as usize);

        let p2: u64 = 0xcafe_babe_1122;
        f(p1p, (&p2 as *const u64) as usize);

        (unsafe { TRACE_HASH }, rd::<u8>(olp + 0x3d), rd::<usize>(olp + 0xb0))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141fe4aa0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141fe4c30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RC_CAPTURED: (u64, u64, u32) = (0, 0, 0);

    unsafe extern "win64" fn mock_dbf0(rbx: usize) {
        let obj_rcx = rd::<usize>(rbx + 0x08);
        let rc_ptr = rd::<usize>(obj_rcx + 0x68);
        unsafe {
            RC_CAPTURED = (
                rd::<u64>(rc_ptr),
                rd::<u64>(rc_ptr + 0x08),
                rd::<u32>(rc_ptr + 0x10),
            );
        }
        wr::<u64>(rbx + 0xb8, 0x1111_2222);
        wr::<u64>(rbx + 0xc0, 0x3333_4444);
        wr::<[f32; 4]>(rbx + 0xd0, [1.0, 2.0, 3.0, 4.0]);
        wr::<[f32; 4]>(rbx + 0xe0, [5.0, 6.0, 7.0, 8.0]);
        wr::<[f32; 4]>(rbx + 0xf0, [2.0, -1.0, 0.5, 1.0]);
        wr::<u8>(rbx + 0x100, 7);
        wr::<u8>(rbx + 0x101, 9);
        wr::<u8>(rbx + 0x102, 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_dbf0, mock_dbf0 as *const ())?;

    let run = |f: F| -> (
        (u64, u64, u32),
        [u32; 16],
        [u32; 32],
        [u8; 32],
        [u32; 4],
        u64,
        u64,
        [u32; 2],
        u16,
        u16,
    ) {
        unsafe { RC_CAPTURED = (0, 0, 0) };

        let mut p20_buf = [0u32; 16];
        for i in 0..16 {
            p20_buf[i] = (0.25f32 * (i as f32 + 1.0)).to_bits();
        }

        let mut obj_rcx = [0u8; 0x160];
        let orp = obj_rcx.as_mut_ptr() as usize;
        wr::<u8>(orp + 0x15a, 0xff);

        let mut obj_rbx = [0u8; 0x120];
        let obp = obj_rbx.as_mut_ptr() as usize;
        wr::<usize>(obp + 0x08, orp);

        let rbx_pairs: [usize; 2] = [obp, orp];

        let mut memset_buf = [0xdead_beefu32; 4];
        let mut r12_buf = [0u8; 0x48];
        let r12p = r12_buf.as_mut_ptr() as usize;
        wr::<u16>(r12p, 0);
        wr::<usize>(r12p + 0x10 + 0x08, memset_buf.as_mut_ptr() as usize);
        wr::<u32>(r12p + 0x10 + 0x10, 4);

        let mut copy_src = [0x5au8; 32];
        for i in 0..32 {
            copy_src[i] = (i as u8).wrapping_mul(3);
        }
        let mut copy_dst0 = [0u8; 32];
        let mut copy_dst1 = [0u8; 32];
        let rsi_18: [usize; 3] = [0, copy_src.as_ptr() as usize, 0x7777];
        let mut rsi_obj = [0u8; 0x20];
        let rsip = rsi_obj.as_mut_ptr() as usize;
        wr::<u32>(rsip, 10);
        wr::<u16>(rsip + 0x04, 1);
        wr::<u8>(rsip + 0x06, 0);
        wr::<usize>(rsip + 0x08, copy_dst0.as_mut_ptr() as usize);
        wr::<usize>(rsip + 0x10, copy_dst1.as_mut_ptr() as usize);
        wr::<usize>(rsip + 0x18, rsi_18.as_ptr() as usize);
        let rsi_arr: [usize; 2] = [rsip, 0];

        let mut out_40: u64 = 0;
        let mut out_48: u64 = 0;
        let mut out_50: [u32; 2] = [0; 2];
        let p60_vals: [u32; 1] = [0x55];
        let mut p68_buf = [0u8; 0x30];
        wr::<u8>(p68_buf.as_mut_ptr() as usize + 0x22, 0);
        let p70_vals: [f32; 4] = [1.0, -2.0, 3.0, 4.0];
        let mut p78_vals = [0u32; 9];
        for i in 0..9 {
            p78_vals[i] = (1.5f32 + i as f32).to_bits();
        }
        let mut p88_vals: [f32; 8] = [
            0.0,
            0.707_106_77,
            0.0,
            0.707_106_77,
            10.0,
            -20.0,
            30.0,
            1.0,
        ];
        let flags_98: [u32; 1] = [0];
        let flags_a0: [u8; 1] = [1];
        let vals_a8: [f32; 2] = [3.5, 0.0];

        let mut p1 = [0u8; 0xb0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 1);
        wr::<usize>(p1p + 0x20, p20_buf.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, rbx_pairs.as_ptr() as usize);
        wr::<usize>(p1p + 0x30, r12p);
        wr::<usize>(p1p + 0x38, rsi_arr.as_ptr() as usize);
        wr::<usize>(p1p + 0x40, (&mut out_40 as *mut u64) as usize);
        wr::<usize>(p1p + 0x48, (&mut out_48 as *mut u64) as usize);
        wr::<usize>(p1p + 0x50, out_50.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x58, 0x40_0000);
        wr::<usize>(p1p + 0x60, p60_vals.as_ptr() as usize);
        wr::<usize>(p1p + 0x68, p68_buf.as_ptr() as usize);
        wr::<usize>(p1p + 0x70, p70_vals.as_ptr() as usize);
        wr::<usize>(p1p + 0x78, p78_vals.as_ptr() as usize);
        wr::<usize>(p1p + 0x88, p88_vals.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x98, flags_98.as_ptr() as usize);
        wr::<usize>(p1p + 0xa0, flags_a0.as_ptr() as usize);
        wr::<usize>(p1p + 0xa8, vals_a8.as_ptr() as usize);

        let p2: u64 = 0x9876_5432;
        let p3: f32 = 1.25;
        f(
            p1p,
            (&p2 as *const u64) as usize,
            (&p3 as *const f32) as usize,
        );

        // Zero stack pointer at +0x68 before reading obj_rcx snapshot
        wr::<usize>(orp + 0x68, 0);
        let rcx_words = rd::<[u32; 32]>(orp + 0x48);

        (
            unsafe { RC_CAPTURED },
            p20_buf,
            rcx_words,
            copy_dst1,
            memset_buf,
            out_40,
            out_48,
            out_50,
            rd::<u16>(r12p),
            rd::<u16>(r12p + 0x02),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141fe4c30 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142088c20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut AF0_CALLS: [(u64, u64, u64, u64); 4] = [(0, 0, 0, 0); 4];
    static mut AF0_CNT: usize = 0;
    static mut C50_CALLS: [(usize, usize); 4] = [(0, 0); 4];
    static mut C50_CNT: usize = 0;

    unsafe extern "win64" fn mock_freq() -> i64 {
        1_000_000_000
    }
    unsafe extern "win64" fn mock_8af0(
        _p1: usize,
        p_slot18: usize,
        slot20: usize,
        slot30: usize,
        d8_inc: u64,
    ) {
        unsafe {
            if AF0_CNT < 4 {
                AF0_CALLS[AF0_CNT] = (
                    rd::<u64>(p_slot18),
                    rd::<u64>(slot20),
                    rd::<u64>(slot30),
                    d8_inc,
                );
                AF0_CNT += 1;
            }
        }
    }
    unsafe extern "win64" fn mock_4c50(r14: usize, it_ptr: usize) {
        let ctrl_base = rd::<usize>(r14);
        let slot_base = rd::<usize>(r14 + 0x08);
        unsafe {
            if C50_CNT < 4 {
                C50_CALLS[C50_CNT] = (
                    rd::<usize>(it_ptr).wrapping_sub(ctrl_base),
                    rd::<usize>(it_ptr + 0x08).wrapping_sub(slot_base),
                );
                C50_CNT += 1;
            }
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_6920, mock_freq as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4208_8af0, mock_8af0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4208_4c50, mock_4c50 as *const ())?;

    let g_budget = exe::ptr::<f32>(0x1_4385_8038) as usize;
    let s_budget = rd::<f32>(g_budget);
    wr::<f32>(g_budget, 1000.0);

    let run = |f: F| -> ([(u64, u64, u64, u64); 4], usize, [(usize, usize); 4], usize, [u64; 2]) {
        unsafe {
            AF0_CALLS = [(0, 0, 0, 0); 4];
            AF0_CNT = 0;
            C50_CALLS = [(0, 0); 4];
            C50_CNT = 0;
        }

        let mut ctrl = [0x80u8; 32];
        ctrl[1] = 0;
        ctrl[3] = 0;
        ctrl[4] = 0xff; // end sentinel (-1i8)

        let mut slots = [0u8; 0x40 * 5];
        let sp = slots.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x40 + 0x18, 0x1111);
        wr::<u64>(sp + 0x40 + 0x20, 0x2222);
        wr::<u64>(sp + 0x40 + 0x30, 0x3333);

        wr::<u64>(sp + 0xc0 + 0x18, 0x4444);
        wr::<u64>(sp + 0xc0 + 0x20, 0x5555);
        wr::<u64>(sp + 0xc0 + 0x30, 0x6666);

        let mut p1 = [0u8; 0xf0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x98, ctrl.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0xa0, sp);
        wr::<u64>(p1p + 0xa8, 2);
        wr::<usize>(p1p + 0xb0, 4);
        wr::<u64>(p1p + 0xd8, 9);

        f(p1p);

        (
            unsafe { AF0_CALLS },
            unsafe { AF0_CNT },
            unsafe { C50_CALLS },
            unsafe { C50_CNT },
            [rd::<u64>(sp + 0x40 + 0x18), rd::<u64>(sp + 0xc0 + 0x18)],
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<f32>(g_budget, s_budget);

    crate::diff_eq!(o, n, "142088c20 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420892a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FP_BUF: [u8; 0x40] = [0; 0x40];
    static mut ROWS_BUF: [u32; 2] = [0; 2];
    static mut UPLOAD_MEM: [u8; 0x80] = [0; 0x80];
    static mut UPLOAD_OBJ: [usize; 2] = [0; 2];
    static mut COPY_CALLS: [([u8; 0x30], [u8; 0x30]); 2] = [([0; 0x30], [0; 0x30]); 2];
    static mut COPY_CNT: usize = 0;
    static mut UNMAP_CNT: u32 = 0;
    static mut FREE_CNT: u32 = 0;

    unsafe extern "win64" fn noop1(_a: usize) {}
    unsafe extern "win64" fn mock_fc30(vec_ptr: usize, cnt: u32) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(FP_BUF) as usize);
        wr::<u32>(vec_ptr + 0x08, cnt);
    }
    unsafe extern "win64" fn mock_fce0(vec_ptr: usize, cnt: u32) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(ROWS_BUF) as usize);
        wr::<u32>(vec_ptr + 0x08, cnt);
    }
    unsafe extern "win64" fn mock_e7f0(w: u32, h: u32, _fmt: u8) -> u32 {
        w * h * 4
    }
    unsafe extern "win64" fn mock_free(_ptr: usize) {
        unsafe { FREE_CNT += 1 };
    }
    unsafe extern "win64" fn mock_get_desc(_this: usize, out_desc: usize) {
        wr::<u64>(out_desc, 0x1234);
    }
    unsafe extern "win64" fn mock_get_footprints(
        _dev: usize,
        _desc: usize,
        _first_sub: u32,
        _num_sub: u32,
        _base_off: u64,
        layouts: usize,
        num_rows: usize,
        _row_sizes: usize,
        out_total: usize,
    ) {
        // Mip 0: offset 0, row_pitch 16, 2 rows
        wr::<u64>(layouts, 0);
        wr::<u32>(layouts + 0x18, 16);
        wr::<u32>(num_rows, 2);
        // Mip 1: offset 32, row_pitch 8, 1 row
        wr::<u64>(layouts + 0x20, 32);
        wr::<u32>(layouts + 0x20 + 0x18, 8);
        wr::<u32>(num_rows + 4, 1);
        wr::<u64>(out_total, 64);
    }
    unsafe extern "win64" fn mock_create_res(
        _dev: usize,
        _hp: usize,
        _hf: u32,
        _desc: usize,
        _state: u32,
        _clear: usize,
        _iid: usize,
        out_res: usize,
    ) -> u32 {
        wr::<usize>(out_res, core::ptr::addr_of_mut!(UPLOAD_OBJ) as usize);
        0
    }
    unsafe extern "win64" fn mock_map(
        _res: usize,
        _sub: u32,
        _range: usize,
        out_ptr: usize,
    ) -> u32 {
        wr::<usize>(out_ptr, core::ptr::addr_of_mut!(UPLOAD_MEM) as usize);
        0
    }
    unsafe extern "win64" fn mock_unmap(_res: usize, _sub: u32, _range: usize) {
        unsafe { UNMAP_CNT += 1 };
    }
    unsafe extern "win64" fn mock_copy_tex(
        _cmd: usize,
        dst_loc: usize,
        _x: u32,
        _y: u32,
        _z: u32,
        src_loc: usize,
        _box_ptr: usize,
    ) {
        unsafe {
            if COPY_CNT < 2 {
                COPY_CALLS[COPY_CNT] = (rd::<[u8; 0x30]>(dst_loc), rd::<[u8; 0x30]>(src_loc));
                COPY_CNT += 1;
            }
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4208_9d10, noop1 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_fc30, mock_fc30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_fce0, mock_fce0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4204_e7f0, mock_e7f0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let s_dev = rd::<usize>(g_dev);

    let run = |f: F| -> (
        [u8; 0x80],
        [([u8; 0x30], [u8; 0x30]); 2],
        usize,
        u32,
        u32,
        [u64; 4],
        u32,
        u64,
    ) {
        unsafe {
            FP_BUF = [0; 0x40];
            ROWS_BUF = [0; 2];
            UPLOAD_MEM = [0; 0x80];
            COPY_CALLS = [([0; 0x30], [0; 0x30]); 2];
            COPY_CNT = 0;
            UNMAP_CNT = 0;
            FREE_CNT = 0;
        }

        let mut res_vtbl = [0usize; 12];
        res_vtbl[0x40 / 8] = mock_map as *const () as usize;
        res_vtbl[0x48 / 8] = mock_unmap as *const () as usize;
        res_vtbl[0x50 / 8] = mock_get_desc as *const () as usize;
        unsafe { UPLOAD_OBJ = [res_vtbl.as_ptr() as usize, 0] };
        let tex_obj = [res_vtbl.as_ptr() as usize];

        let mut dev_vtbl = [0usize; 40];
        dev_vtbl[0xd8 / 8] = mock_create_res as *const () as usize;
        dev_vtbl[0x130 / 8] = mock_get_footprints as *const () as usize;
        let dev_obj = [dev_vtbl.as_ptr() as usize];
        wr::<usize>(g_dev, dev_obj.as_ptr() as usize);

        let mut cmd_vtbl = [0usize; 20];
        cmd_vtbl[0x80 / 8] = mock_copy_tex as *const () as usize;
        let cmd_obj = [cmd_vtbl.as_ptr() as usize];

        let mut vec_slots = [0u64; 4];
        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x58, cmd_obj.as_ptr() as usize);
        wr::<usize>(p1p + 0x68, vec_slots.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x70, 0);
        wr::<u32>(p1p + 0x74, 4);

        let mut src_pixels = [0u8; 64];
        for i in 0..64 {
            src_pixels[i] = (i as u8).wrapping_add(1);
        }

        let mut p2 = [0u8; 0x50];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p, tex_obj.as_ptr() as usize);
        wr::<usize>(p2p + 0x08, src_pixels.as_ptr() as usize);
        wr::<u64>(p2p + 0x10, 64);
        wr::<u8>(p2p + 0x38, 1);
        wr::<u32>(p2p + 0x3c, 2);
        wr::<u32>(p2p + 0x40, 2);
        wr::<u8>(p2p + 0x44, 2);
        wr::<u8>(p2p + 0x48, 3);

        f(p1p, p2p, 0x7777_8888);

        let mut copies = unsafe { COPY_CALLS };
        for c in &mut copies {
            // Normalize tex_obj pointer in dst_loc (+0x00)
            wr::<u64>(c.0.as_mut_ptr() as usize, 0);
        }
        vec_slots[2] = 0; // Normalize tex_obj pointer in 2nd pushed pair

        (
            unsafe { UPLOAD_MEM },
            copies,
            unsafe { COPY_CNT },
            unsafe { UNMAP_CNT },
            unsafe { FREE_CNT },
            vec_slots,
            rd::<u32>(p1p + 0x70),
            rd::<u64>(p2p),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_dev, s_dev);

    crate::diff_eq!(o, n, "1420892a0 out");
    Ok(1)
}

// FLOAT_RETURN: none (no function in this module returns f32/f64 in xmm0)
// REGISTRY:
// Replacement::verified(0x1_41f5_7ce0, "unknown_141f57ce0", rend::cauldron_hot_r::unknown_141f57ce0 as *const (), rend::cauldron_hot_r::diff_unknown_141f57ce0),
// Replacement::verified_throws(0x1_41f5_b690, "unknown_141f5b690", rend::cauldron_hot_r::unknown_141f5b690 as *const (), Throws::Caught, rend::cauldron_hot_r::diff_unknown_141f5b690),
// Replacement::verified(0x1_41fd_6a00, "unknown_141fd6a00", rend::cauldron_hot_r::unknown_141fd6a00 as *const (), rend::cauldron_hot_r::diff_unknown_141fd6a00),
// Replacement::verified(0x1_41fe_4aa0, "unknown_141fe4aa0", rend::cauldron_hot_r::unknown_141fe4aa0 as *const (), rend::cauldron_hot_r::diff_unknown_141fe4aa0),
// Replacement::verified(0x1_41fe_4c30, "unknown_141fe4c30", rend::cauldron_hot_r::unknown_141fe4c30 as *const (), rend::cauldron_hot_r::diff_unknown_141fe4c30),
// Replacement::verified_throws(0x1_4208_8c20, "unknown_142088c20", rend::cauldron_hot_r::unknown_142088c20 as *const (), Throws::Discard, rend::cauldron_hot_r::diff_unknown_142088c20),
// Replacement::verified_throws(0x1_4208_92a0, "unknown_1420892a0", rend::cauldron_hot_r::unknown_1420892a0 as *const (), Throws::Discard, rend::cauldron_hot_r::diff_unknown_1420892a0),
