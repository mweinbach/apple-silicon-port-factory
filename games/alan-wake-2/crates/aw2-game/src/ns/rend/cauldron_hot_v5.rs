//! Decompiled hot functions shard V5 (`0x1420c1320..0x1421cdbf0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vrsqrtps_128(a: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{__m128, _mm_rsqrt_ps};
        let v: __m128 = core::mem::transmute(a);
        core::mem::transmute(_mm_rsqrt_ps(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vrsqrtps(a)
    }
}

#[inline(always)]
fn quat_mul_simd(a: u128, b: u128) -> u128 {
    use crate::simd_ops::{
        vaddsubps, vblendps, vhaddps, vhsubps, vmulps, vpermilps, vshufps,
    };
    let a_rev = vpermilps(a, 0x1b);
    let b_11 = vpermilps(b, 0x11);
    let b_bb = vpermilps(b, 0xbb);
    let p1 = vmulps(b_bb, a_rev);
    let p0 = vmulps(a, b_11);
    let hsub = vhsubps(p0, p1);
    let p2 = vmulps(b_11, a_rev);
    let p3 = vmulps(a, b_bb);
    let hadd = vhaddps(p3, p2);
    let shuf = vshufps(hsub, hadd, 0xb1);
    let blend = vblendps(hadd, hsub, 0x0c);
    let addsub = vaddsubps(blend, shuf);
    vpermilps(addsub, 0x9c)
}

/// `unknown_1420c1320`
/// Original: 0x1420c1320 (226 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420c1320(param_1: usize, param_2: usize) {
    let count = rd::<u64>(param_1) as usize;
    let base = rd::<usize>(param_1 + 0x08);
    let end = base.wrapping_add(count.wrapping_mul(0x28));
    if base == end {
        return;
    }

    let spans_base = rd::<usize>(param_2 + 0x08);
    let mut r14 = base.wrapping_add(0x08);
    loop {
        let range = rd::<u64>(r14 + 0x10);
        let mut dst = rd::<usize>(r14);
        let start_idx = (range as u32) as usize;
        let mut rbx = spans_base.wrapping_add(start_idx.wrapping_mul(8));
        let span_cnt = ((range >> 32) as u32).wrapping_sub(range as u32) as usize;
        let rdi = rbx.wrapping_add(span_cnt.wrapping_mul(8));

        while rbx != rdi {
            let sub_range = rd::<u64>(rbx);
            let sub_start = (sub_range as u32) as u64;
            let sub_cnt = ((sub_range >> 32) as u32).wrapping_sub(sub_range as u32) as u64;
            let stride = rd::<u64>(r14 + 0x08);
            let src_off = sub_start.wrapping_mul(stride) as usize;
            let byte_len = sub_cnt.wrapping_mul(stride);
            let src = rd::<usize>(r14 - 0x08).wrapping_add(src_off);
            let _ = exe::call_va!(0x1_4183_acc8, (dst, src, byte_len) -> usize);
            dst = dst.wrapping_add(byte_len as usize);
            rbx = rbx.wrapping_add(8);
        }

        r14 = r14.wrapping_add(0x28);
        if r14.wrapping_sub(0x08) == end {
            break;
        }
    }
}

/// `unknown_1420d1690`
/// Original: 0x1420d1690 (776 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420d1690(param_1: usize, param_2: u8, param_3: u8) {
    let mut rbx = param_1.wrapping_add(0x55c);
    for _ in 0..16usize {
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rbx - 0x04) != tid {
            exe::call_iat!(0x1_4264_55a8, (rbx - 0x0c));
            wr::<u32>(rbx - 0x04, tid);
        }
        let prev_depth = rd::<u32>(rbx);
        wr::<u32>(rbx, prev_depth.wrapping_add(1));

        let dev = rd::<usize>(rbx + 0x24);
        if dev == 0 {
            wr::<u32>(rbx, prev_depth);
            if prev_depth == 0 {
                let _ = crate::simd_ops::atomic_xchg_u32((rbx - 0x04) as u64, u32::MAX);
                exe::call_iat!(0x1_4264_5598, (rbx - 0x0c));
            }
        } else {
            if param_2 != 0 {
                wr::<u64>(dev + 0xc8, rd::<u64>(dev + 0xc8).wrapping_add(1));
                let slot = (rd::<u64>(dev + 0xc0) % 3) as usize;
                let src = dev.wrapping_add(slot * 0x30);
                wr::<[u8; 0x30]>(dev + 0x90, rd::<[u8; 0x30]>(src));
            } else {
                if rd::<u8>(param_1 + 0x72) != 0
                    && rd::<u8>(rbx + 0x04) != 2
                    && (rd::<u8>(rbx + 0x04) != 0 || param_3 != 0)
                {
                    let dev_in = rd::<usize>(rbx + 0x24);
                    let next_slot = (rd::<u64>(dev_in + 0xc0).wrapping_add(1) % 3) as usize;
                    let rdi = dev_in.wrapping_add(next_slot * 0x30);
                    let mut xstate = [0u8; 16];
                    let user_idx = rd::<u32>(dev_in + 0xd0);
                    let err = exe::call_iat!(
                        0x1_4264_6478,
                        (user_idx, xstate.as_mut_ptr() as usize) -> u32
                    );
                    let connected: u8 = if err == 0 { 1 } else { 0 };
                    let _ = crate::simd_ops::atomic_xchg_u8((rbx + 0x04) as u64, connected);
                    if err == 0 {
                        let b4 = xstate[4];
                        wr::<u8>(rdi + 0x06, b4 & 1);
                        wr::<u8>(rdi + 0x07, (b4 >> 1) & 1);
                        wr::<u8>(rdi + 0x08, (b4 >> 2) & 1);
                        wr::<u8>(rdi + 0x09, (b4 >> 3) & 1);
                        wr::<u8>(rdi + 0x18, (b4 >> 5) & 1);
                        wr::<u8>(rdi + 0x1b, (b4 >> 4) & 1);
                        wr::<u8>(rdi + 0x10, (b4 >> 6) & 1);
                        wr::<u8>(rdi + 0x13, (b4 >> 7) & 1);

                        let b5 = xstate[5];
                        wr::<u8>(rdi + 0x0e, b5 & 1);
                        wr::<u8>(rdi + 0x11, (b5 >> 1) & 1);
                        wr::<u8>(rdi + 0x0a, (b5 >> 4) & 1);
                        wr::<u8>(rdi + 0x0b, (b5 >> 5) & 1);
                        wr::<u8>(rdi + 0x0c, (b5 >> 6) & 1);
                        wr::<u8>(rdi + 0x0d, (b5 >> 7) & 1);

                        wr::<u16>(rdi + 0x28, u16::from_le_bytes([xstate[8], xstate[9]]));
                        wr::<u16>(rdi + 0x2a, u16::from_le_bytes([xstate[10], xstate[11]]));
                        wr::<u16>(rdi + 0x2c, u16::from_le_bytes([xstate[12], xstate[13]]));
                        wr::<u16>(rdi + 0x2e, u16::from_le_bytes([xstate[14], xstate[15]]));
                        wr::<u8>(rdi + 0x25, xstate[6]);
                        wr::<u8>(rdi + 0x26, xstate[7]);
                    } else {
                        wr::<[u8; 0x20]>(rdi, [0u8; 0x20]);
                        wr::<u32>(rdi + 0x20, 0);
                        wr::<u16>(rdi + 0x24, 0);
                        wr::<u64>(rdi + 0x28, 0);
                        wr::<u8>(rdi + 0x26, 0);
                    }
                }

                let r9 = rd::<usize>(rbx + 0x24);
                let new_c0 = rd::<u64>(r9 + 0xc0).wrapping_add(1);
                wr::<u64>(r9 + 0xc0, new_c0);
                let src_slot = (new_c0 % 3) as usize;
                let dst_slot = (new_c0.wrapping_add(1) % 3) as usize;
                let block = rd::<[u8; 0x30]>(r9.wrapping_add(src_slot * 0x30));
                wr::<[u8; 0x30]>(r9.wrapping_add(dst_slot * 0x30), block);
            }

            let new_depth = rd::<u32>(rbx).wrapping_sub(1);
            wr::<u32>(rbx, new_depth);
            if new_depth == 0 {
                let _ = crate::simd_ops::atomic_xchg_u32((rbx - 0x04) as u64, u32::MAX);
                exe::call_iat!(0x1_4264_5598, (rbx - 0x0c));
            }
        }

        rbx = rbx.wrapping_add(0x330);
    }
}

/// `unknown_1420d2440`
/// Original: 0x1420d2440 (530 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420d2440(param_1: usize, param_2: u8) -> usize {
    let old_4f88 = crate::simd_ops::atomic_xchg_u8((param_1 + 0x4f88) as u64, 0);
    exe::call_va!(0x1_420d_1690, (param_1, param_2, old_4f88));
    exe::call_va!(0x1_420d_73d0, (param_1, param_2));

    let saved_dil = rd::<u8>(param_1 + 0x4f89);
    let mut dil = saved_dil;
    let _ = crate::simd_ops::atomic_xchg_u8((param_1 + 0x4f89) as u64, 0);

    let r13 = param_1.wrapping_add(0x548 + 0x3300);
    let mut rsi = param_1.wrapping_add(0x55c);
    let mut scratch = [0u8; 0x100];
    let sp = scratch.as_mut_ptr() as usize;

    loop {
        let r12 = rsi - 0x0c;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rsi - 0x04) != tid {
            exe::call_iat!(0x1_4264_55a8, (r12));
            wr::<u32>(rsi - 0x04, tid);
        }
        wr::<u32>(rsi, rd::<u32>(rsi).wrapping_add(1));

        let al = rd::<u8>(rsi + 0x04);
        let has_314 = rd::<u8>(rsi + 0x314) != 0;
        let run_body = if !has_314 {
            al != 0
        } else if al == 0 {
            exe::call_va!(
                0x1_4218_4b70,
                (sp + 0x20, rd::<u64>(param_1), rd::<u64>(rsi - 0x14))
            );
            wr::<u8>(sp + 0x40, 0);
            wr::<u128>(rsi + 0x30c, rd::<u128>(sp + 0x38));
            false
        } else {
            dil != 0
        };

        if run_body {
            let r14 = rd::<usize>(param_1);
            let rdi_handle = if r14 == 0 {
                0u64
            } else {
                let vtbl = rd::<usize>(r14);
                let mgr = exe::call_va!(rd::<usize>(vtbl + 0x28), (r14) -> usize);
                let lock = mgr + 0x2d40;
                exe::call_iat!(0x1_4264_55a8, (lock));
                let val = rd::<u64>(mgr + 0x2d58);
                exe::call_iat!(0x1_4264_5598, (lock));
                val
            };

            exe::call_va!(0x1_4218_49f0, (sp + 0x58, r14, rdi_handle, 1u8));
            if rd::<u8>(sp + 0xe8) != 0 {
                exe::call_va!(
                    0x1_4218_4cb0,
                    (
                        sp + 0x28,
                        rd::<u64>(param_1),
                        rdi_handle,
                        rd::<u64>(rsi - 0x14)
                    )
                );
                wr::<u128>(rsi + 0x30c, rd::<u128>(sp + 0x68));
            } else if has_314 {
                exe::call_va!(
                    0x1_4218_4b70,
                    (sp + 0x30, rd::<u64>(param_1), rd::<u64>(rsi - 0x14))
                );
                wr::<u8>(sp + 0x50, 0);
                wr::<u128>(rsi + 0x30c, rd::<u128>(sp + 0x48));
            }

            if rd::<u8>(sp + 0xe8) != 0 {
                exe::call_va!(0x1_4240_ab10, (sp + 0xa0));
                wr::<u64>(sp + 0x98, 0);
            }
            dil = saved_dil;
        }

        let new_depth = rd::<u32>(rsi).wrapping_sub(1);
        wr::<u32>(rsi, new_depth);
        if new_depth == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((rsi - 0x04) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (r12));
        }

        rsi = rsi.wrapping_add(0x330);
        if rsi.wrapping_sub(0x14) == r13 {
            break;
        }
    }

    r13
}

/// `unknown_1420d4180`
/// Original: 0x1420d4180 (2387 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420d4180(_param_1: usize, param_2: usize) {
    if rd::<usize>(param_2) != 0 {
        if rd::<u8>(param_2 + 0x18) == 2 {
            return;
        }
        let rcx_40 = rd::<usize>(param_2 + 0x40);
        let rdi = exe::ptr::<u8>(0x1_43ce_6880) as usize;
        let count = exe::call_va!(
            0x1_417a_1f20,
            (rd::<u32>(rcx_40 + 0x208), rdi, 0x40u32) -> i32
        );
        if count <= 0 {
            return;
        }

        let dl = rd::<u8>(exe::ptr::<u8>(0x1_43ce_68cc) as usize);
        let cl: u8 = if dl != 0 { 1 } else { 0 };
        let _ = crate::simd_ops::atomic_xchg_u8((param_2 + 0x18) as u64, cl);
        if dl != 0 {
            let r8 = rd::<usize>(param_2 + 0x40);
            let g_e8_ptr = exe::ptr::<u8>(0x1_43ce_68e8) as usize;
            if rd::<u8>(g_e8_ptr) != rd::<u8>(r8 + 0x206) {
                let u = exe::call_va!(0x1_420d_4ca0, (param_2) -> u64);
                exe::call_va!(0x1_420d_4b90, (u, param_2));
                let rcx_after = rd::<usize>(param_2 + 0x40);
                wr::<u8>(rcx_after + 0x206, rd::<u8>(g_e8_ptr));
                return;
            }

            let r10d = (count as u32).wrapping_sub(1);
            let idx_last = r10d as usize;
            let r12 = idx_last.wrapping_mul(0x78);
            let last_entry = rdi.wrapping_add(r12);
            if rd::<u32>(last_entry) != 0x8000_0000 {
                let slot = (rd::<u64>(r8 + 0x1f0).wrapping_add(1) % 3) as usize;
                let r14 = r8.wrapping_add(slot.wrapping_mul(0x7c));

                wr::<u8>(r14 + 0x27, rd::<u8>(last_entry + 0x04));
                wr::<u8>(r14 + 0x28, !rd::<u8>(last_entry + 0x05));
                wr::<u8>(r14 + 0x29, rd::<u8>(last_entry + 0x06));
                wr::<u8>(r14 + 0x2a, !rd::<u8>(last_entry + 0x07));
                wr::<u8>(r14 + 0x25, rd::<u8>(last_entry + 0x08));
                wr::<u8>(r14 + 0x26, rd::<u8>(last_entry + 0x09));

                let d_0c = rd::<u32>(last_entry + 0x0c);
                let d_10 = rd::<u32>(last_entry + 0x10);
                let d_14 = rd::<u32>(last_entry + 0x14);
                let d_18 = rd::<u32>(last_entry + 0x18);
                wr::<u32>(r14 + 0x2c, d_18);
                wr::<u32>(r14 + 0x30, d_0c);
                wr::<u32>(r14 + 0x34, d_10);
                wr::<u32>(r14 + 0x38, d_14);

                let d_1c = rd::<u32>(last_entry + 0x1c);
                let d_20 = rd::<u32>(last_entry + 0x20);
                let d_24 = rd::<u32>(last_entry + 0x24);
                wr::<u32>(r14 + 0x3c, d_1c);
                wr::<u32>(r14 + 0x40, d_20);
                wr::<u32>(r14 + 0x44, d_24);

                let d_28 = rd::<u32>(last_entry + 0x28);
                let d_2c = rd::<u32>(last_entry + 0x2c);
                let d_30 = rd::<u32>(last_entry + 0x30);
                wr::<u32>(r14 + 0x48, d_28);
                wr::<u32>(r14 + 0x4c, d_2c);
                wr::<u32>(r14 + 0x50, d_30);

                let first_u32 = rd::<u32>(rdi);
                let rdx_40 = rd::<usize>(param_2 + 0x40);
                let prev_200 = rd::<u32>(rdx_40 + 0x200);
                let mut r13d = (!prev_200) & first_u32;
                let mut r11d = (!first_u32) & prev_200;
                wr::<u32>(rdx_40 + 0x200, rd::<u32>(last_entry));

                for k in 0..idx_last {
                    let cur = rd::<u32>(rdi.wrapping_add(k * 0x78));
                    let next = rd::<u32>(rdi.wrapping_add((k + 1) * 0x78));
                    r13d |= (!cur) & next;
                    r11d |= (!next) & cur;
                }

                let tbl_base = exe::ptr::<u8>(0x1_4298_3e00) as usize;
                let last_u32 = rd::<u32>(last_entry);
                for i in 0..16usize {
                    let entry = tbl_base.wrapping_add(i * 8);
                    let off_w = rd::<u16>(entry) as usize;
                    let mask = rd::<u32>(entry + 0x04);
                    let target = r14.wrapping_add(off_w).wrapping_sub(0x95);

                    let mut b = rd::<u8>(target);
                    if (mask & last_u32) != 0 {
                        b |= 1;
                    } else {
                        b &= !1;
                    }
                    wr::<u8>(target, b);

                    let mut b = rd::<u8>(target);
                    if (mask & r13d) != 0 {
                        b |= 2;
                    } else {
                        b &= !2;
                    }
                    wr::<u8>(target, b);

                    let mut b = rd::<u8>(target);
                    if (mask & r11d) != 0 {
                        b |= 4;
                    } else {
                        b &= !4;
                    }
                    wr::<u8>(target, b);
                }

                let touch_cnt = rd::<u8>(last_entry + 0x34) as u32;
                let mut indices: [i32; 4] = [-1, -1, -1, -1];
                let r8_2 = rd::<usize>(param_2 + 0x40);
                let slot_2 = (rd::<u64>(r8_2 + 0x1f0).wrapping_add(1) % 3) as usize;
                let rsi = r8_2.wrapping_add(slot_2.wrapping_mul(0x7c));

                if touch_cnt != 0 {
                    let mut rbx_ptr = last_entry.wrapping_add(0x40);
                    for r10_idx in 0..touch_cnt {
                        let r11b = rd::<u8>(rbx_ptr);
                        let mut rdx_ptr = rsi.wrapping_add(0x54);
                        let mut first_zero: usize = usize::MAX;
                        let mut matched: Option<usize> = None;
                        for c in 0..4usize {
                            let r9b = rd::<u8>(rdx_ptr);
                            if r11b == r9b {
                                matched = Some(c);
                                break;
                            }
                            if r9b == 0 && first_zero == usize::MAX {
                                first_zero = c;
                            }
                            rdx_ptr = rdx_ptr.wrapping_add(0x0a);
                        }
                        if let Some(c) = matched {
                            indices[c] = r10_idx as i32;
                        } else if first_zero < 4 {
                            indices[first_zero] = r10_idx as i32;
                        }
                        rbx_ptr = rbx_ptr.wrapping_add(8);
                    }
                }

                for (slot_i, &idx) in indices.iter().enumerate() {
                    let dst = r14.wrapping_add(0x54 + slot_i * 0x0a);
                    if idx == -1 {
                        wr::<u8>(dst, 0);
                        wr::<u16>(dst + 0x02, 0);
                        wr::<u16>(dst + 0x04, 0);
                    } else {
                        let src = last_entry.wrapping_add((idx as isize as usize).wrapping_mul(8));
                        wr::<u8>(dst, rd::<u8>(src + 0x40));
                        wr::<u16>(dst + 0x02, rd::<u16>(src + 0x3c));
                        wr::<u16>(dst + 0x04, rd::<u16>(src + 0x3e));
                    }
                }
                return;
            }
        }
    }

    exe::call_va!(0x1_420d_4ca0, (param_2));
}

/// `unknown_1420e93c0`
/// Original: 0x1420e93c0 (579 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420e93c0(param_1: usize, param_2: usize) -> u8 {
    let wstr_len = rd::<u32>(param_1 + 0x10);
    let alloc_size = (wstr_len as u64).wrapping_mul(4).wrapping_add(1);
    let dst_buf = exe::call_va!(0x1_4241_c0a0, (alloc_size, 1u32) -> usize);
    let _ = exe::call_va!(0x1_4183_acc2, (dst_buf, 0u32, alloc_size) -> usize);

    let mut r10 = rd::<usize>(param_1);
    let end_wstr = r10.wrapping_add((rd::<u32>(param_1 + 0x10) as usize).wrapping_mul(2));
    let max_dst = dst_buf.wrapping_add((wstr_len as usize).wrapping_mul(4));
    let mut err_code: u32 = 0;
    let mut rdx = dst_buf;
    let tbl_142986390 = exe::ptr::<u8>(0x1_4298_6390) as usize;

    while r10 < end_wstr {
        let r11 = rdx;
        let mut cp = rd::<u16>(r10) as u32;
        r10 = r10.wrapping_add(2);

        if cp.wrapping_sub(0xd800) <= 0x3ff {
            if r10 >= end_wstr {
                err_code = 1;
                break;
            }
            let low = rd::<u16>(r10) as u32;
            if low.wrapping_sub(0xdc00) > 0x3ff {
                err_code = 3;
                break;
            }
            cp = (cp << 10).wrapping_add(0xfca0_2400).wrapping_add(low);
            r10 = r10.wrapping_add(2);
        } else if cp.wrapping_sub(0xdc00) <= 0x3ff {
            err_code = 3;
            break;
        }

        let seq_len: usize = if cp < 0x80 {
            1
        } else if cp < 0x800 {
            2
        } else if cp < 0x10000 {
            3
        } else if cp < 0x110000 {
            4
        } else {
            cp = 0xfffd;
            3
        };

        rdx = rdx.wrapping_add(seq_len);
        if rdx > max_dst {
            rdx = r11;
            err_code = 2;
            break;
        }

        for _ in 1..seq_len {
            rdx -= 1;
            wr::<u8>(rdx, ((cp as u8) & 0x3f) | 0x80);
            cp >>= 6;
        }
        rdx -= 1;
        let prefix = rd::<u8>(tbl_142986390.wrapping_add(seq_len));
        wr::<u8>(rdx, (cp as u8) | prefix);
        rdx = rdx.wrapping_add(seq_len);
    }

    wr::<u8>(rdx, 0);
    if dst_buf == 0 {
        wr::<u32>(param_2 + 0x04, 0);
        let mut p = param_2.wrapping_add(0x08);
        let b3 = rd::<u8>(param_2 + 0x03) as u32;
        let cap_bits = if b3 < 0x80 {
            b3 * 8
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let len_bits = (rd::<u32>(param_2) & 0x00ff_ffff) << 3;
        if len_bits > cap_bits {
            p = rd::<usize>(p);
        }
        wr::<u8>(p, 0);
    } else {
        let mut utf8_len: u64 = 0;
        while rd::<u8>(dst_buf.wrapping_add(utf8_len as usize)) != 0 {
            utf8_len = utf8_len.wrapping_add(1);
        }
        let mut span: [u64; 2] = [dst_buf as u64, utf8_len];
        exe::call_va!(
            0x1_4240_9b50,
            (param_2, span.as_mut_ptr() as usize, 0u32, utf8_len)
        );
    }

    let ok: u8 = if err_code == 0 { 1 } else { 0 };
    if dst_buf != 0 {
        exe::call_va!(0x1_4241_c190, (dst_buf));
    }
    ok
}

/// `unknown_1420ea540`
/// Original: 0x1420ea540 (903 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420ea540(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> u8 {
    if param_2 == 0 {
        return 0;
    }

    let mut local_str: [u64; 18] = [0; 18];
    local_str[0] = 0x1000_0010;
    let local_str_ptr = local_str.as_mut_ptr() as usize;

    let mut in_len: u64 = 0;
    while rd::<u8>(param_2.wrapping_add(in_len as usize)) != 0 {
        in_len = in_len.wrapping_add(1);
    }
    let mut in_span: [u64; 2] = [param_2 as u64, in_len];
    exe::call_va!(
        0x1_4240_9b50,
        (local_str_ptr, in_span.as_mut_ptr() as usize, 0u32, in_len)
    );

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(param_1 + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (param_1));
        wr::<u32>(param_1 + 0x08, tid);
    }
    wr::<u32>(param_1 + 0x0c, rd::<u32>(param_1 + 0x0c).wrapping_add(1));

    let res: u8 = 'lookup: {
        let mut pair_30 = [0usize; 2];
        exe::call_va!(
            0x1_411f_7760,
            (
                param_1 + 0x10,
                pair_30.as_mut_ptr() as usize,
                local_str_ptr
            )
        );
        let end1 = rd::<usize>(param_1 + 0x18)
            .wrapping_add((rd::<u32>(param_1 + 0x20) as usize).wrapping_mul(0x88));
        let (r14_1, rsi_1) = if pair_30[0] != end1
            && exe::call_va!(0x1_401e_6ff0, (pair_30[0], local_str_ptr) -> u8) == 0
        {
            let s = rd::<usize>(param_1 + 0x28)
                .wrapping_add((rd::<u32>(param_1 + 0x30) as usize).wrapping_mul(0x120));
            (end1, s)
        } else {
            (pair_30[0], pair_30[1])
        };
        exe::call_va!(0x1_411f_76e0, (param_1 + 0x10));
        let end1_after = rd::<usize>(param_1 + 0x18)
            .wrapping_add((rd::<u32>(param_1 + 0x20) as usize).wrapping_mul(0x88));
        if r14_1 != end1_after {
            break 'lookup exe::call_va!(0x1_420e_93c0, (rsi_1, param_3) -> u8);
        }

        let mut pair_40 = [0usize; 2];
        exe::call_va!(
            0x1_411f_7760,
            (
                param_1 + 0x38,
                pair_40.as_mut_ptr() as usize,
                local_str_ptr
            )
        );
        let end2 = rd::<usize>(param_1 + 0x40)
            .wrapping_add((rd::<u32>(param_1 + 0x48) as usize).wrapping_mul(0x88));
        let (r14_2, rsi_2) = if pair_40[0] != end2
            && exe::call_va!(0x1_401e_6ff0, (pair_40[0], local_str_ptr) -> u8) == 0
        {
            let s = rd::<usize>(param_1 + 0x50)
                .wrapping_add((rd::<u32>(param_1 + 0x58) as usize).wrapping_mul(0x120));
            (end2, s)
        } else {
            (pair_40[0], pair_40[1])
        };
        exe::call_va!(0x1_411f_76e0, (param_1 + 0x38));
        let end2_after = rd::<usize>(param_1 + 0x40)
            .wrapping_add((rd::<u32>(param_1 + 0x48) as usize).wrapping_mul(0x88));
        if r14_2 != end2_after {
            break 'lookup exe::call_va!(0x1_420e_93c0, (rsi_2, param_3) -> u8);
        }

        let head = rd::<usize>(param_1 + 0x60);
        let mut cur = rd::<usize>(head + 0x08);
        let mut cand = head;
        while rd::<u8>(cur + 0x19) == 0 {
            if exe::call_va!(0x1_4021_74f0, (cur + 0x20, local_str_ptr) -> u8) != 0 {
                cur = rd::<usize>(cur + 0x10);
            } else {
                cand = cur;
                cur = rd::<usize>(cur);
            }
        }
        if rd::<u8>(cand + 0x19) == 0
            && exe::call_va!(0x1_4021_74f0, (local_str_ptr, cand + 0x20) -> u8) == 0
            && cand != head
        {
            break 'lookup exe::call_va!(0x1_420e_93c0, (cand + 0xa8, param_3) -> u8);
        }

        let mut wstr_storage = [0u64; 0x120 / 8];
        let wstr_base = wstr_storage.as_mut_ptr() as usize;
        let inline_buf = wstr_base + 0x20;
        wr::<usize>(wstr_base, inline_buf);
        wr::<u32>(wstr_base + 0x08, 0x80);
        wr::<u64>(wstr_base + 0x0c, 0x80);
        wr::<usize>(wstr_base + 0x18, inline_buf);
        wr::<u16>(inline_buf, 0);

        exe::call_va!(0x1_420e_9610, (local_str_ptr, wstr_base));
        let rbx_val = exe::call_va!(0x1_411f_7570, (param_1 + 0x60, local_str_ptr) -> usize);
        let w_len = rd::<u32>(wstr_base + 0x10);
        let cur_len = rd::<u32>(rbx_val + 0x10);
        let req_len = if w_len >= cur_len { w_len } else { cur_len };
        exe::call_va!(0x1_423e_f2b0, (rbx_val, req_len));
        let _ = exe::call_iat!(
            0x1_4264_69d0,
            (
                rd::<usize>(rbx_val),
                rd::<u32>(rbx_val + 0x0c) as u64,
                rd::<usize>(wstr_base)
            )
        );
        wr::<u32>(rbx_val + 0x10, rd::<u32>(wstr_base + 0x10));
        let conv_ok = exe::call_va!(0x1_420e_93c0, (wstr_base, param_3) -> u8);

        let r10_ptr = rd::<usize>(wstr_base);
        if r10_ptr != 0 && r10_ptr != rd::<usize>(wstr_base + 0x18) {
            let r10_arena = r10_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
            let same_tid = exe::teb() == rd::<usize>(r10_arena + 0x100);
            let r9_ptr = rd::<usize>(wstr_base);
            let page_idx = r9_ptr.wrapping_sub(r10_arena) >> 16;
            let hdr_raw = r10_arena
                .wrapping_add(0x108)
                .wrapping_add(page_idx.wrapping_mul(0x50));
            let rdx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
            if same_tid {
                if rd::<u8>(rdx_hdr + 0x0e) == 0 {
                    wr::<u64>(r9_ptr, rd::<u64>(rdx_hdr + 0x20));
                    wr::<usize>(rdx_hdr + 0x20, r9_ptr);
                    let new_cnt = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
                    wr::<u32>(rdx_hdr + 0x18, new_cnt);
                    if new_cnt == 0 {
                        exe::call_va!(0x1_4253_71a0, (rdx_hdr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10_arena, rdx_hdr, 1u8, r9_ptr));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (r10_arena, rdx_hdr, 0u8, r9_ptr));
            }
        }

        conv_ok
    };

    let new_depth = rd::<u32>(param_1 + 0x0c).wrapping_sub(1);
    wr::<u32>(param_1 + 0x0c, new_depth);
    if new_depth == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((param_1 + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (param_1));
    }
    exe::call_va!(0x1_4240_ab10, (local_str_ptr));
    res
}

/// `physics_14217e2a0`
/// Original: 0x14217e2a0 (1963 bytes, Throws::Discard)
pub unsafe extern "win64" fn physics_14217e2a0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7_bits: u32,
    param_8: u64,
    param_9: u8,
    param_10: u64,
) {
    use crate::simd_ops::{
        vaddps, vblendps, vbroadcastss, vcvtdq2ps, vcvttps2dq, vdivps, vdpps, vinsertps, vmaxps,
        vminps, vmulps, vpaddd, vpcmpeqd, vpermilps, vpslld, vpsubd, vshufps, vsqrtps, vsubps,
        vunpcklpd,
    };

    let rax0 = rd::<usize>(param_1);
    let xmm8_init = rd::<u128>(rax0 + 0x10);
    let saved_90 = xmm8_init;
    let xmm6_q = rd::<u128>(rax0);
    let xmm7_v = rd::<u128>(param_4 + 0x10);

    let xmm0 = vshufps(xmm7_v, xmm7_v, 0xc9);
    let xmm4 = vpermilps(xmm6_q, 0xd2);
    let xmm2 = vmulps(xmm0, xmm4);
    let xmm1 = vshufps(xmm7_v, xmm7_v, 0xd2);
    let xmm5_p = vpermilps(xmm6_q, 0xc9);
    let xmm0 = vmulps(xmm1, xmm5_p);
    let xmm1 = vsubps(xmm0, xmm2);
    let c_7ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let xmm3 = vmulps(xmm1, c_7ab0);
    let xmm2 = vpermilps(xmm3, 0xc9);
    let xmm4 = vmulps(xmm2, xmm4);
    let xmm0 = vpermilps(xmm3, 0xd2);
    let xmm2 = vmulps(xmm0, xmm5_p);
    let xmm0 = vpermilps(xmm6_q, 0xff);
    let xmm1 = vmulps(xmm0, xmm3);
    let xmm3 = vaddps(xmm1, xmm7_v);
    let xmm2 = vsubps(xmm2, xmm4);
    let mut xmm5 = vaddps(xmm3, xmm2);

    let r15b = rd::<u8>(param_6 + 0x10);
    if r15b != 0 {
        xmm5 = rd::<u128>(param_6);
    }

    let cl = if param_9 != 0 {
        rd::<u8>(exe::ptr::<u8>(0x1_4386_51c8) as usize)
    } else {
        rd::<u8>(exe::ptr::<u8>(0x1_4386_4fc0) as usize)
    };
    let xmm14_f = rd::<f32>(exe::ptr::<u8>(0x1_4342_68d0) as usize);
    let xmm12_f = if cl != 0 && rd::<u8>(param_1 + 0x16) != 0 {
        rd::<f32>(exe::ptr::<u8>(0x1_4386_5080) as usize)
    } else {
        xmm14_f
    };

    let mut xmm9 = xmm8_init;
    let rsp_60 = rd::<u128>(param_5);
    let rsp_70 = rd::<u128>(param_1 + 0xa0);

    let sq = vsqrtps(vdpps(xmm7_v, xmm7_v, 0x7f));
    let mut div_f = f32::from_bits(sq as u32) / rd::<f32>(exe::ptr::<u8>(0x1_4386_4fd8) as usize);
    let two63 = rd::<f32>(exe::ptr::<u8>(0x1_4342_6fc8) as usize);
    let mut rcx_off: u64 = 0;
    if div_f >= two63 {
        div_f -= two63;
        if div_f < two63 {
            rcx_off = 0x8000_0000_0000_0000;
        }
    }
    let rax_cnt = crate::simd_ops::cvt_f32_i64(div_f).wrapping_add(rcx_off.wrapping_add(1));
    let mut rbx_cnt: u64 = if rax_cnt < 0x14 { rax_cnt } else { 0x14 };
    wr::<u8>(param_1 + 0x15, if rbx_cnt == 0x14 { 1 } else { 0 });

    let p68 = rd::<usize>(param_1 + 0x68);
    wr::<u64>(p68 + 0x30, 0x7fef_ffff_ffff_ffff);
    wr::<u64>(p68 + 0x38, 0x7fef_ffff_ffff_ffff);
    wr::<u64>(p68 + 0x40, 0x7fef_ffff_ffff_ffff);
    wr::<u64>(p68 + 0x48, 0xffef_ffff_ffff_ffff);
    wr::<u64>(p68 + 0x50, 0xffef_ffff_ffff_ffff);
    wr::<u64>(p68 + 0x58, 0xffef_ffff_ffff_ffff);
    wr::<u32>(p68 + 0x08, 0);
    wr::<u32>(p68 + 0x18, 0);
    wr::<u32>(p68 + 0x28, 0);

    wr::<u128>(
        param_1 + 0x50,
        rd::<u128>(exe::ptr::<u8>(0x1_43c8_ce70) as usize),
    );
    wr::<u32>(param_1 + 0x60, 0x3f80_0000);
    wr::<u8>(param_1 + 0x64, 0);
    wr::<u32>(param_1 + 0xb4, 0);

    let xmm8_step = vdivps(xmm5, vbroadcastss((rbx_cnt as f32).to_bits()));
    let xmm15_f = f32::from_bits(param_7_bits);

    if rbx_cnt != 0 {
        let xmm11 = vbroadcastss(param_7_bits);
        let xmm13_eps = rd::<f32>(exe::ptr::<u8>(0x1_4342_6468) as usize);
        let mut buf_a0 = [0u64; 8];
        while rbx_cnt != 0 {
            buf_a0[7] = 0;
            let rcx_38 = rd::<usize>(param_3 + 0x38);
            if rcx_38 != 0 {
                let vtbl = rd::<usize>(rcx_38);
                buf_a0[7] =
                    exe::call_va!(rd::<usize>(vtbl), (rcx_38, buf_a0.as_mut_ptr() as usize) -> u64);
            }
            let mut buf_e0: [u128; 2] = [rsp_60, rsp_70];
            let mut buf_80: u128 = xmm8_step;
            exe::call_va!(
                0x1_4217_ab60,
                (
                    param_1,
                    param_2,
                    buf_a0.as_mut_ptr() as usize,
                    (&mut buf_80 as *mut u128) as usize,
                    buf_e0.as_mut_ptr() as usize,
                    param_7_bits,
                    param_8,
                    xmm12_f.to_bits(),
                    param_9,
                    r15b,
                    param_10
                )
            );

            let xmm10 = vdivps(rd::<u128>(param_1 + 0x20), xmm11);
            if xmm12_f.is_nan() || xmm14_f.is_nan() || xmm12_f != xmm14_f {
                let xmm2_sq = vsqrtps(vdpps(xmm8_step, xmm8_step, 0x7f));
                let xmm2_f = f32::from_bits(xmm2_sq as u32);
                if xmm2_f > xmm13_eps && rd::<u8>(param_1 + 0xb0) == 0 {
                    let xmm7_mul = vmulps(xmm10, xmm11);
                    let xmm1_sub = vsubps(rd::<u128>(param_1 + 0xa0), xmm9);
                    let xmm6_sub = vsubps(xmm1_sub, xmm7_mul);
                    let xmm1_len = vsqrtps(vdpps(xmm6_sub, xmm6_sub, 0x7f));
                    let xmm5_ratio = f32::from_bits(xmm1_len as u32) / xmm2_f;
                    let c_5118 = rd::<f32>(exe::ptr::<u8>(0x1_4386_5118) as usize);
                    let c_5080 = rd::<f32>(exe::ptr::<u8>(0x1_4386_5080) as usize);
                    let xmm4_blend = (xmm14_f - c_5118) * c_5080 + xmm5_ratio * c_5118;
                    let max_f = if xmm5_ratio > xmm4_blend {
                        xmm5_ratio
                    } else {
                        xmm4_blend
                    };
                    let xmm0_div = vdivps(xmm6_sub, vbroadcastss(max_f.to_bits()));
                    let xmm1_add = vaddps(xmm0_div, xmm9);
                    let xmm3_add = vaddps(xmm1_add, xmm7_mul);
                    let c_8910 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8910) as usize);
                    let c_8620 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8620) as usize);
                    let clamped = vminps(vmaxps(xmm3_add, c_8910), c_8620);
                    wr::<u128>(param_1 + 0xa0, clamped);
                }
            }
            wr::<u128>(param_1 + 0x40, xmm10);
            xmm9 = rd::<u128>(param_1 + 0xa0);
            rbx_cnt -= 1;
        }
    }

    let xmm13_final = rd::<u128>(param_1 + 0xa0);
    let xmm8_p5 = rd::<u128>(param_5);
    let c_7b00 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7b00) as usize);
    let xmm6_conj = xmm8_p5 ^ c_7b00;
    let rbx_out = rd::<usize>(param_1);

    let q1 = quat_mul_simd(xmm6_conj, rd::<u128>(rbx_out));
    let q2 = quat_mul_simd(rd::<u128>(param_4), q1);
    let xmm4_q3 = quat_mul_simd(xmm8_p5, q2);

    let xmm0_dp = vdpps(xmm4_q3, xmm4_q3, 0xff);
    let c_8460 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8460) as usize);
    let xmm3_rsq = vrsqrtps_128(xmm0_dp);
    let xmm0_m1 = vmulps(xmm3_rsq, xmm0_dp);
    let xmm1_m2 = vmulps(xmm3_rsq, xmm0_m1);
    let xmm2_sub = vsubps(c_8460, xmm1_m2);
    let c_8120 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize);
    let xmm0_half = vmulps(xmm3_rsq, c_8120);
    let xmm1_nr = vmulps(xmm0_half, xmm2_sub);
    let xmm3_norm = vmulps(xmm1_nr, xmm4_q3);

    let xmm12_saved = xmm8_p5;
    let xmm6_conj2 = xmm8_p5 ^ c_7b00;
    let xmm4_q4 = quat_mul_simd(xmm6_conj2, xmm3_norm);

    let xmm0_44 = vpermilps(xmm4_q4, 0x44);
    let xmm1_sq = vmulps(xmm0_44, xmm4_q4);
    let c_83b0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_83b0) as usize);
    let xmm5_2 = vmulps(xmm1_sq, c_83b0);
    let xmm3_aa = vpermilps(xmm5_2, 0xaa);
    let xmm0_ff = vpermilps(xmm5_2, 0xff);
    let xmm2_55 = vpermilps(xmm5_2, 0x55);
    let atan_y = f32::from_bits(xmm0_ff as u32) + f32::from_bits(xmm3_aa as u32);
    let atan_x = (xmm14_f - f32::from_bits(xmm5_2 as u32)) - f32::from_bits(xmm2_55 as u32);
    let (_, atan_xmm0) = crate::simd_ops::call_va8_xmm(
        0x1_4183_ad94,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [atan_y.to_bits() as u128, atan_x.to_bits() as u128, 0, 0],
    );

    let xmm1_ins = vinsertps(0, atan_xmm0, 0x10);
    let xmm1_lh = vunpcklpd(xmm1_ins, xmm1_ins);
    let xmm10_half = vmulps(xmm1_lh, c_8120);
    let xmm3_abs = xmm10_half & rd::<u128>(exe::ptr::<u8>(0x1_426f_d060) as usize);
    let xmm0_scaled = vmulps(xmm3_abs, rd::<u128>(exe::ptr::<u8>(0x1_426f_caa0) as usize));
    let xmm1_dq = vcvttps2dq(xmm0_scaled);
    let xmm2_add = vpaddd(xmm1_dq, rd::<u128>(exe::ptr::<u8>(0x1_426f_ce00) as usize));
    let xmm7_and = xmm2_add & rd::<u128>(exe::ptr::<u8>(0x1_426f_cc90) as usize);
    let xmm5_f = vcvtdq2ps(xmm7_and);
    let c_fcfc0 = rd::<u128>(exe::ptr::<u8>(0x1_426f_cfc0) as usize);
    let xmm8_shl = vpslld(xmm7_and & c_fcfc0, 0x1d);
    let c_fd020 = rd::<u128>(exe::ptr::<u8>(0x1_426f_d020) as usize);
    let xmm11_eq = vpcmpeqd(xmm7_and & c_fd020, 0);

    let xmm0_c1 = vmulps(xmm5_f, rd::<u128>(exe::ptr::<u8>(0x1_426f_ccc0) as usize));
    let xmm2_r1 = vaddps(xmm0_c1, xmm3_abs);
    let xmm1_c2 = vmulps(xmm5_f, rd::<u128>(exe::ptr::<u8>(0x1_426f_cf60) as usize));
    let xmm3_r2 = vaddps(xmm2_r1, xmm1_c2);
    let xmm0_c3 = vmulps(xmm5_f, rd::<u128>(exe::ptr::<u8>(0x1_426f_cb90) as usize));
    let xmm9_rem = vaddps(xmm3_r2, xmm0_c3);

    let xmm1_subd = vpsubd(xmm7_and, c_fd020);
    let xmm0_andn = (!xmm1_subd) & c_fcfc0;
    let xmm7_shl = vpslld(xmm0_andn, 0x1d);
    let xmm0_sgn = xmm10_half & rd::<u128>(exe::ptr::<u8>(0x1_426f_d090) as usize);
    let xmm6_sgn = xmm0_sgn ^ xmm8_shl;

    let xmm4_r2 = vmulps(xmm9_rem, xmm9_rem);
    let xmm3_half_r2 = vmulps(xmm4_r2, rd::<u128>(exe::ptr::<u8>(0x1_426f_d070) as usize));
    let xmm0_p = vmulps(xmm4_r2, rd::<u128>(exe::ptr::<u8>(0x1_426f_ccd0) as usize));
    let xmm0_p = vaddps(xmm0_p, rd::<u128>(exe::ptr::<u8>(0x1_426f_cb00) as usize));
    let xmm1_p = vmulps(xmm0_p, xmm4_r2);
    let xmm1_p = vaddps(xmm1_p, rd::<u128>(exe::ptr::<u8>(0x1_426f_cd70) as usize));
    let xmm0_p = vmulps(xmm1_p, xmm4_r2);
    let xmm2_p = vmulps(xmm0_p, xmm4_r2);
    let xmm3_p = vsubps(xmm2_p, xmm3_half_r2);
    let xmm5_cos = vaddps(xmm3_p, rd::<u128>(exe::ptr::<u8>(0x1_426f_cd60) as usize));

    let xmm1_s = vmulps(xmm4_r2, rd::<u128>(exe::ptr::<u8>(0x1_426f_cce0) as usize));
    let xmm1_s = vaddps(xmm1_s, rd::<u128>(exe::ptr::<u8>(0x1_426f_cbf0) as usize));
    let xmm0_s = vmulps(xmm1_s, xmm4_r2);
    let xmm0_s = vaddps(xmm0_s, rd::<u128>(exe::ptr::<u8>(0x1_426f_cb70) as usize));
    let xmm1_s = vmulps(xmm0_s, xmm4_r2);
    let xmm2_s = vmulps(xmm1_s, xmm9_rem);
    let xmm3_sin = vaddps(xmm2_s, xmm9_rem);

    let xmm4_m = xmm3_sin & xmm11_eq;
    let xmm1_m = (!xmm11_eq) & xmm5_cos;
    let xmm2_d = vsubps(xmm3_sin, xmm4_m);
    let xmm0_d = vsubps(xmm5_cos, xmm1_m);
    let xmm1_a = vaddps(xmm1_m, xmm4_m);
    let xmm2_a = vaddps(xmm2_d, xmm0_d);
    let xmm1_x = xmm1_a ^ xmm6_sgn;
    let xmm0_x = xmm7_shl ^ xmm2_a;
    let xmm3_sc = vblendps(xmm0_x, xmm1_x, 0x07);

    let xmm2_final = quat_mul_simd(xmm12_saved, xmm3_sc);
    wr::<u128>(rbx_out, xmm2_final);
    wr::<u128>(rbx_out + 0x10, xmm13_final);

    let xmm1_diff = vsubps(xmm13_final, saved_90);
    let scale_bits = if xmm15_f.is_nan() || xmm15_f != 0.0 {
        (xmm14_f / xmm15_f).to_bits()
    } else {
        0u32
    };
    let xmm1_vel = vmulps(vbroadcastss(scale_bits), xmm1_diff);
    wr::<u128>(param_1 + 0x30, xmm1_vel);

    let rcx_end = rd::<usize>(param_3 + 0x38);
    if rcx_end != 0 {
        let vtbl = rd::<usize>(rcx_end);
        let dl: u8 = if rcx_end != param_3 { 1 } else { 0 };
        exe::call_va!(rd::<usize>(vtbl + 0x20), (rcx_end, dl));
        wr::<usize>(param_3 + 0x38, 0);
    }
}

/// `unknown_1421cc210`
/// Original: 0x1421cc210 (1200 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421cc210(param_1: usize, param_2: u32) {
    let r14 = rd::<usize>(param_1 + 0x70).wrapping_add((param_2 as usize).wrapping_mul(0x30));
    let rcx_20 = rd::<usize>(param_1 + 0x20);
    let r13 = rd::<usize>(r14);
    let rbp = rd::<usize>(r14 + 0x08);
    let mut rdi = rd::<usize>(r14 + 0x10);
    let r12d = rd::<u32>(r14 + 0x24);
    let rax_c8 = rd::<usize>(r13 + 0xc8);
    let r15d = rd::<u32>(rax_c8);

    wr::<u64>(rcx_20 + 0x58, rd::<u64>(param_1 + 0xa8));
    wr::<usize>(rcx_20 + 0x110, rdi);
    let rcx_20 = rd::<usize>(param_1 + 0x20);
    let val_98 = rd::<u64>(param_1 + 0x98);
    wr::<u64>(rcx_20 + 0xf0, 0);
    wr::<u64>(rcx_20 + 0xf8, val_98);
    wr::<u64>(rd::<usize>(param_1 + 0x20) + 0x108, rd::<u64>(param_1 + 0x90));
    wr::<u64>(rd::<usize>(param_1 + 0x20) + 0x100, rd::<u64>(param_1 + 0xa0));

    let r8_20 = rd::<usize>(param_1 + 0x20);
    wr::<u32>(r8_20 + 0x4c, rd::<u32>(rd::<usize>(r13 + 0xc8)));
    let val_c8_10 = rd::<u64>(rd::<usize>(r13 + 0xc8) + 0x10);
    wr::<u8>(r8_20 + 0x15a, rd::<u8>(r8_20 + 0x15a) & !2u8);
    wr::<u64>(r8_20 + 0x50, val_c8_10);
    wr::<usize>(r8_20 + 0x160, r13);
    wr::<usize>(r8_20 + 0x168, rbp);
    wr::<u64>(r8_20 + 0x148, 0);
    wr::<u64>(r8_20 + 0x150, 0);
    wr::<u16>(r8_20 + 0x158, 0x00ff);
    wr::<u16>(r8_20 + 0x1b4, 0);
    wr::<u16>(r8_20 + 0x1b6, rd::<u16>(r13 + 0x120));
    wr::<u16>(r8_20 + 0x1b8, rd::<u8>(rbp + 0x06) as u16);

    if rd::<u8>(r14 + 0x28) != 0 {
        exe::call_va!(0x1_421c_cef0, (param_1, r14));
        exe::call_va!(0x1_421c_ce30, (param_1, r14));
        let rdx_20 = rd::<usize>(param_1 + 0x20);
        let rcx_160 = rd::<usize>(rdx_20 + 0x160);
        if rd::<u32>(rcx_160 + 0x9c) != 0 {
            let rdx_168 = rd::<usize>(rdx_20 + 0x168);
            exe::call_va!(0x1_420c_ba20, (rcx_160 + 0x90, rd::<u64>(rdx_168 + 0xb0)));
        }
        let rbp_20 = rd::<usize>(param_1 + 0x20);
        let r14_168 = rd::<usize>(rbp_20 + 0x168);
        let r8_d0 = rd::<usize>(r14_168 + 0xd0);
        let edx_idx: u32 = if r8_d0 != 0 {
            let rcx_160_2 = rd::<usize>(rbp_20 + 0x160);
            let found = exe::call_va!(
                0x1_4248_9e40,
                (
                    rd::<u64>(rcx_160_2 + 0x118),
                    rd::<u64>(rcx_160_2 + 0x120),
                    rd::<u32>(r8_d0)
                ) -> i32
            );
            if found < 0 {
                0
            } else {
                found as u32
            }
        } else {
            0
        };
        wr::<u8>(r14_168 + 0x06, edx_idx as u8);
        let prod =
            (rd::<u16>(rbp_20 + 0x1b6) as u32).wrapping_mul(rd::<u16>(rbp_20 + 0x1b4) as u32);
        wr::<u16>(rbp_20 + 0x1b8, (prod as u16).wrapping_add(edx_idx as u16));
        exe::call_va!(0x1_421d_5f70, (rd::<usize>(param_1 + 0x20)));
    }

    exe::call_va!(0x1_421d_5c20, (rd::<usize>(param_1 + 0x20)));
    let u = exe::call_va!(0x1_421d_5a70, (rd::<usize>(param_1 + 0x20)) -> u64);
    exe::call_va!(0x1_421d_a330, (rd::<u64>(param_1 + 0xb0), u));
    let mut rcx_src = exe::call_va!(0x1_421d_5b40, (rd::<usize>(param_1 + 0x20)) -> usize);
    let r8_90 = rd::<usize>(param_1 + 0x90);
    if r15d != 0 {
        for _ in 0..(r15d as usize) {
            let ax = rd::<u16>(rdi);
            if ax != 0xffff {
                let dst = r8_90.wrapping_add((ax as usize) << 5);
                wr::<[u8; 32]>(dst, rd::<[u8; 32]>(rcx_src));
            }
            rdi = rdi.wrapping_add(2);
            rcx_src = rcx_src.wrapping_add(0x20);
        }
    }

    if rd::<u8>(param_1 + 0x3d) != 0 {
        let rdi_20 = rd::<usize>(param_1 + 0x20);
        let rdx_70 = rd::<u64>(rd::<usize>(rdi_20 + 0x160) + 0x70);
        let g_5bf0 = exe::ptr::<u8>(0x1_4386_5bf0) as usize;
        exe::call_va!(0x1_421d_4c70, (rdi_20, rdx_70, g_5bf0));
        let eax_cnt = rd::<i32>(rdi_20 + 0x14);
        if eax_cnt > 0 {
            let new_top = rd::<usize>(rdi_20 + 0x20).wrapping_sub(0x30);
            wr::<usize>(rdi_20 + 0x20, new_top);
            wr::<i32>(rdi_20 + 0x14, eax_cnt - 1);
            wr::<[u8; 0x30]>(param_1 + 0xd0, rd::<[u8; 0x30]>(new_top));
            wr::<u8>(param_1 + 0x102, 1);
        }
    }

    let rcx_20 = rd::<usize>(param_1 + 0x20);
    let mut v_148 = rd::<u64>(rcx_20 + 0x148);
    if v_148 == 0 {
        v_148 = rd::<u64>(param_1 + 0xb8);
    }
    wr::<u64>(param_1 + 0xb8, v_148);

    let mut v_150 = rd::<u64>(rcx_20 + 0x150);
    if v_150 == 0 {
        v_150 = rd::<u64>(param_1 + 0xc0);
    }
    wr::<u64>(param_1 + 0xc0, v_150);

    let mut b_158 = rd::<u8>(rcx_20 + 0x158);
    if b_158 == 0xff {
        b_158 = rd::<u8>(param_1 + 0x100);
    }
    wr::<u8>(param_1 + 0x100, b_158);
    wr::<u8>(
        param_1 + 0x101,
        rd::<u8>(param_1 + 0x101) | rd::<u8>(rcx_20 + 0x159),
    );

    wr::<u32>(rcx_20 + 0x4c, 0);
    wr::<u64>(rcx_20 + 0x160, 0);
    wr::<u64>(rcx_20 + 0x168, 0);
    wr::<u64>(rcx_20 + 0x148, 0);
    wr::<u64>(rcx_20 + 0x150, 0);
    wr::<u16>(rcx_20 + 0x158, 0x00ff);
    wr::<u32>(rcx_20 + 0x1b4, 0);
    wr::<u16>(rcx_20 + 0x1b8, 0xffff);

    if rd::<u8>(param_1 + 0x3c) == 0 {
        wr::<u8>(rbp + 0x07, 1);
    } else {
        let mut ctx_40 = [0u64; 3];
        let mut ctx_58 = [0u64; 4];
        let p40 = ctx_40.as_mut_ptr() as usize;
        let p58 = ctx_58.as_mut_ptr() as usize;

        let mut r15_ptr = rd::<usize>(rbp + 0x08);
        wr::<u64>(p40, rd::<u64>(param_1 + 0x28));
        wr::<u32>(p40 + 0x08, r12d >> 6);
        wr::<u32>(p40 + 0x0c, r12d & 0x3f);
        wr::<u32>(p40 + 0x10, r12d);
        wr::<usize>(p58, r13);

        exe::call_va!(
            0x1_421d_1d50,
            (
                rcx_20,
                rd::<u64>(rbp + 0x20),
                rd::<u64>(r13 + 0x50),
                p40
            )
        );

        let xmm1_c0 = rd::<u64>(rbp + 0xc0);
        let r9_58 = rd::<usize>(p58);
        let mut words_rem = rd::<u16>(rbp + 0x02) as usize;
        let r13_30 = rd::<usize>(r13 + 0x30);
        let xmm0_10 = rd::<u64>(r13 + 0x10);
        let r12_8 = rd::<usize>(r9_58 + 0x08);
        let mut r14_ptr = rd::<usize>(r13 + 0x80);
        let mut esi_base: u32 = 0;
        let fn_tbl = exe::ptr::<u8>(0x1_4386_5e50) as usize;

        while words_rem != 0 {
            let mut mask = rd::<u64>(r15_ptr) & rd::<u64>(r14_ptr);
            r15_ptr = r15_ptr.wrapping_add(8);
            r14_ptr = r14_ptr.wrapping_add(8);
            words_rem -= 1;

            while mask != 0 {
                let bit = mask.trailing_zeros();
                let edx_bit = bit.wrapping_add(esi_base);
                let entry_rcx = r12_8.wrapping_add((edx_bit as usize).wrapping_mul(12));
                let op_tag = rd::<u8>(entry_rcx) as usize;
                let fn_ptr = rd::<usize>(fn_tbl.wrapping_add(op_tag * 8));

                wr::<u64>(
                    p58 + 0x08,
                    (rd::<u32>(entry_rcx + 0x04) as u64).wrapping_add(xmm0_10),
                );
                wr::<u64>(
                    p58 + 0x10,
                    (rd::<u32>(entry_rcx + 0x08) as u64).wrapping_add(xmm1_c0),
                );
                wr::<u32>(p58 + 0x18, edx_bit);

                let bit_off = rd::<u32>(p40 + 0x10)
                    .wrapping_add(rd::<u32>(r13_30.wrapping_add((edx_bit as usize) * 4)));
                wr::<u32>(p40 + 0x0c, bit_off & 0x3f);
                wr::<u32>(p40 + 0x08, bit_off >> 6);

                exe::call_va!(fn_ptr, (p58, p40));
                mask ^= 1u64 << bit;
            }
            esi_base = esi_base.wrapping_add(0x40);
        }

        wr::<u8>(rbp + 0x07, 1);
    }
}

/// `unknown_1421cdbf0`
/// Original: 0x1421cdbf0 (217 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1421cdbf0(param_1: usize) {
    if rd::<u8>(param_1 + 0x3c) != 0 {
        exe::call_va!(0x1_421c_b380, (param_1, rd::<u64>(param_1 + 0x28), 0u32));
    }

    let xmm0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_81d0) as usize);
    let rdx_20 = rd::<usize>(param_1 + 0x20);
    let al_103 = rd::<u8>(param_1 + 0x103);

    wr::<u64>(param_1 + 0xb8, 0);
    wr::<u64>(param_1 + 0xc0, 0);
    wr::<u128>(param_1 + 0xf0, 0);
    wr::<u16>(param_1 + 0x100, 0x00ff);
    wr::<u8>(param_1 + 0x102, 0);
    wr::<u128>(param_1 + 0xd0, xmm0);
    wr::<u128>(param_1 + 0xe0, 0);

    let mut cl = rd::<u8>(rdx_20 + 0x15a) & !1u8;
    wr::<u8>(rdx_20 + 0x15a, cl);
    if al_103 != 0 {
        cl |= 1;
        wr::<u8>(rdx_20 + 0x15a, cl);
    }

    exe::call_va!(0x1_421c_ca40, (param_1));

    let mut rsi = rd::<u64>(param_1 + 0x50);
    let mut rdi_off: usize = 0;
    while rsi != 0 {
        let rax_40 = rd::<usize>(param_1 + 0x40);
        let edx_arg = rd::<u32>(rax_40.wrapping_add(rdi_off).wrapping_add(0x04));
        let fn_ptr = rd::<usize>(rax_40.wrapping_add(rdi_off).wrapping_add(0x08));
        exe::call_va!(fn_ptr, (param_1, edx_arg));
        rdi_off = rdi_off.wrapping_add(0x10);
        rsi -= 1;
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1420c1320(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u64| -> [u32; 8] {
        let src0: [u32; 6] = [10, 20, 30, 40, 50, 60];
        let src1: [u32; 4] = [100, 200, 300, 400];
        let mut dst = [0u32; 8];
        let dp = dst.as_mut_ptr() as usize;

        let spans: [u64; 3] = [
            (3u64 << 32) | 1u64, // elements [1..3] => 20, 30
            (6u64 << 32) | 4u64, // elements [4..6] => 50, 60
            (3u64 << 32) | 0u64, // elements [0..3] => 100, 200, 300
        ];

        let mut entries = [0u64; 10];
        // Entry 0: copies 2 spans (spans[0..2]) of 4-byte elements into dst[0..4]
        entries[0] = src0.as_ptr() as u64;
        entries[1] = dp as u64;
        entries[2] = 4;
        entries[3] = (2u64 << 32) | 0u64;
        entries[4] = 0;

        // Entry 1: copies 1 span (spans[2..3]) of 4-byte elements into dst[4..7]
        entries[5] = src1.as_ptr() as u64;
        entries[6] = (dp + 16) as u64;
        entries[7] = 4;
        entries[8] = (3u64 << 32) | 2u64;
        entries[9] = 0;

        let p1: [u64; 2] = [count, entries.as_ptr() as u64];
        let p2: [u64; 2] = [3, spans.as_ptr() as u64];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        dst
    };

    for cnt in [0u64, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1420c1320 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420d1690(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_xinput(user_idx: u32, out_state: usize) -> u32 {
        if user_idx == 0 {
            wr::<u32>(out_state, 42);
            wr::<u8>(out_state + 0x04, 0b1010_0101);
            wr::<u8>(out_state + 0x05, 0b1101_0010);
            wr::<u8>(out_state + 0x06, 0x77);
            wr::<u8>(out_state + 0x07, 0x88);
            wr::<u16>(out_state + 0x08, 0x1111);
            wr::<u16>(out_state + 0x0a, 0x2222);
            wr::<u16>(out_state + 0x0c, 0x3333);
            wr::<u16>(out_state + 0x0e, 0x4444);
            0
        } else {
            1167 // ERROR_DEVICE_NOT_CONNECTED
        }
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_6478, mock_xinput as *const ())?;

    let run = |f: F, p2: u8, p3: u8| -> (Vec<u8>, Vec<u8>) {
        let mut dev0 = vec![0x5au8; 0xe0];
        let d0 = dev0.as_mut_ptr() as usize;
        wr::<u64>(d0 + 0xc0, 1);
        wr::<u64>(d0 + 0xc8, 10);
        wr::<u32>(d0 + 0xd0, 0);

        let mut dev1 = vec![0xa5u8; 0xe0];
        let d1 = dev1.as_mut_ptr() as usize;
        wr::<u64>(d1 + 0xc0, 2);
        wr::<u64>(d1 + 0xc8, 20);
        wr::<u32>(d1 + 0xd0, 1);

        let mut mgr = vec![0u8; 0x55c + 16 * 0x330];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<u8>(mp + 0x72, 1);
        for i in 0..16usize {
            let slot = mp + 0x55c + i * 0x330;
            wr::<u32>(slot - 0x04, u32::MAX);
        }
        // Slot 0: connected device (user_idx = 0)
        let s0 = mp + 0x55c;
        wr::<u8>(s0 + 0x04, 1);
        wr::<usize>(s0 + 0x24, d0);

        // Slot 1: disconnected device (user_idx = 1), only queried if p3 != 0
        let s1 = mp + 0x55c + 0x330;
        wr::<u8>(s1 + 0x04, 0);
        wr::<usize>(s1 + 0x24, d1);

        f(mp, p2, p3);
        (dev0, dev1)
    };

    for (p2, p3) in [(0u8, 1u8), (0, 0), (1, 0)] {
        let o = run(f_o, p2, p3);
        let n = run(f_n, p2, p3);
        crate::diff_eq!(o, n, "1420d1690 p2={} p3={}", p2, p3);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420d2440(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_1690(_p1: usize, p2: u8, p3: u8) {
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add((p2 as u64) << 8 | (p3 as u64));
    }
    unsafe extern "win64" fn mock_73d0(_p1: usize, p2: u8) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(p2 as u64);
    }
    unsafe extern "win64" fn mock_vfunc_28(this: usize) -> usize {
        rd::<usize>(this + 0x08)
    }
    unsafe extern "win64" fn mock_49f0(out: usize, _r14: usize, handle: u64, flag: u8) -> usize {
        core::ptr::write_bytes(out as *mut u8, 0, 0x98);
        if handle == 0x7777 {
            wr::<u64>(out + 0x10, 0x1234_5678_9abc_def0);
            wr::<u64>(out + 0x18, (flag as u64) | 0x0100);
            wr::<u8>(out + 0x90, 1);
        } else {
            wr::<u8>(out + 0x90, 0);
        }
        out
    }
    unsafe extern "win64" fn mock_4b70(out: usize, _p1: u64, id: u64) -> usize {
        core::ptr::write_bytes(out as *mut u8, 0, 0x28);
        wr::<u64>(out + 0x18, id ^ 0xdead_0000);
        wr::<u64>(out + 0x20, 0xfeed_face_0000_0001);
        out
    }
    unsafe extern "win64" fn mock_4cb0(out: usize, _p1: u64, handle: u64, id: u64) -> usize {
        CALL_LOG = CALL_LOG.wrapping_add(handle).wrapping_add(id);
        wr::<u64>(out, 0);
        out
    }
    unsafe extern "win64" fn mock_ab10(_rcx: usize) {
        CALL_LOG = CALL_LOG.wrapping_add(0x99);
    }

    let _m1 = crate::difftest::Mock::new(0x1_420d_1690, mock_1690 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_73d0, mock_73d0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4218_49f0, mock_49f0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4218_4b70, mock_4b70 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4218_4cb0, mock_4cb0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, handle: u64| -> (usize, u8, u8, [u128; 3], u64) {
        CALL_LOG = 0;

        let mut mgr_2d = vec![0u8; 0x2d70];
        let m2p = mgr_2d.as_mut_ptr() as usize;
        wr::<u64>(m2p + 0x2d58, handle);

        let mut vtbl = [0usize; 8];
        vtbl[0x28 / 8] = mock_vfunc_28 as *const () as usize;
        let r14_obj: [usize; 2] = [vtbl.as_ptr() as usize, m2p];

        let mut p1 = vec![0u8; 0x5000];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, r14_obj.as_ptr() as usize);
        wr::<u8>(p1p + 0x4f88, 3);
        wr::<u8>(p1p + 0x4f89, 1);

        for i in 0..16usize {
            let s = p1p + 0x55c + i * 0x330;
            wr::<u32>(s - 0x04, u32::MAX);
        }
        // Slot 0: has_314 = 1, al = 0 => calls 4b70 directly
        let s0 = p1p + 0x55c;
        wr::<u64>(s0 - 0x14, 0x101);
        wr::<u8>(s0 + 0x04, 0);
        wr::<u8>(s0 + 0x314, 1);

        // Slot 1: has_314 = 0, al = 1 => calls 49f0
        let s1 = p1p + 0x55c + 0x330;
        wr::<u64>(s1 - 0x14, 0x202);
        wr::<u8>(s1 + 0x04, 1);
        wr::<u8>(s1 + 0x314, 0);

        // Slot 2: has_314 = 1, al = 1, dil = 1 => calls 49f0 (and 4b70 if 49f0 returns empty)
        let s2 = p1p + 0x55c + 2 * 0x330;
        wr::<u64>(s2 - 0x14, 0x303);
        wr::<u8>(s2 + 0x04, 1);
        wr::<u8>(s2 + 0x314, 1);

        let ret = f(p1p, 1).wrapping_sub(p1p);
        let out_30c = [
            rd::<u128>(s0 + 0x30c),
            rd::<u128>(s1 + 0x30c),
            rd::<u128>(s2 + 0x30c),
        ];
        (
            ret,
            rd::<u8>(p1p + 0x4f88),
            rd::<u8>(p1p + 0x4f89),
            out_30c,
            CALL_LOG,
        )
    };

    for h in [0x7777u64, 0] {
        let o = run(f_o, h);
        let n = run(f_n, h);
        crate::diff_eq!(o, n, "1420d2440 h={:#x}", h);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420d4180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MOCK_COUNT: i32 = 2;
    static mut CA0_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_1f20(_id: u32, buf: usize, _max: u32) -> i32 {
        let cnt = MOCK_COUNT;
        if cnt > 0 {
            core::ptr::write_bytes(buf as *mut u8, 0, (cnt as usize) * 0x78);
            // Entry 0
            wr::<u32>(buf, 0x0000_0050);
            // Last entry (cnt - 1)
            let last = buf + ((cnt as usize) - 1) * 0x78;
            wr::<u32>(last, 0x0010_0490);
            wr::<u8>(last + 0x04, 11);
            wr::<u8>(last + 0x05, 22);
            wr::<u8>(last + 0x06, 33);
            wr::<u8>(last + 0x07, 44);
            wr::<u8>(last + 0x08, 55);
            wr::<u8>(last + 0x09, 66);
            wr::<f32>(last + 0x0c, 1.25);
            wr::<f32>(last + 0x10, 2.5);
            wr::<f32>(last + 0x14, 3.75);
            wr::<f32>(last + 0x18, 4.0);
            wr::<u32>(last + 0x1c, 0x1111_2222);
            wr::<f32>(last + 0x20, 5.5);
            wr::<f32>(last + 0x24, 6.5);
            wr::<u32>(last + 0x28, 0x3333_4444);
            wr::<f32>(last + 0x2c, 7.5);
            wr::<f32>(last + 0x30, 8.5);
            wr::<u8>(last + 0x34, 2); // 2 touch points
            wr::<u16>(last + 0x3c, 100);
            wr::<u16>(last + 0x3e, 200);
            wr::<u8>(last + 0x40, 7);
            wr::<u16>(last + 0x44, 300);
            wr::<u16>(last + 0x46, 400);
            wr::<u8>(last + 0x48, 9);
            wr::<u8>(buf + 0x4c, 1); // 0x143ce68cc = 1
            wr::<u8>(buf + 0x68, 5); // 0x143ce68e8 = 5
        }
        cnt
    }
    unsafe extern "win64" fn mock_4ca0(_p2: usize) -> u64 {
        CA0_CALLS += 1;
        0x55
    }
    unsafe extern "win64" fn mock_4b90(_u: u64, _p2: usize) {
        CA0_CALLS += 10;
    }

    let _m1 = crate::difftest::Mock::new(0x1_417a_1f20, mock_1f20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_4ca0, mock_4ca0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420d_4b90, mock_4b90 as *const ())?;

    let g_buf = exe::ptr::<u8>(0x1_43ce_6880) as usize;
    let snap = crate::difftest::Snapshot::take(g_buf as *mut u8, 20 * 0x78);

    let run = |f: F, cnt: i32, e8_val: u8| -> (Vec<u8>, u8, u32) {
        MOCK_COUNT = cnt;
        CA0_CALLS = 0;

        let mut state = vec![0u8; 0x220];
        let sp = state.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x1f0, 0); // next slot = 1 (offset 0x7c)
        wr::<u32>(sp + 0x200, 0x0000_0010);
        wr::<u8>(sp + 0x206, e8_val);
        wr::<u32>(sp + 0x208, 1);
        // Existing touch ID 7 in slot 1 (offset 0x7c + 0x5e)
        wr::<u8>(sp + 0x7c + 0x5e, 7);

        let mut p2 = [0u64; 10];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, 1);
        wr::<u8>(p2p + 0x18, 1);
        wr::<usize>(p2p + 0x40, sp);

        f(0, p2p);
        (state, rd::<u8>(p2p + 0x18), CA0_CALLS)
    };

    for (cnt, e8_val) in [(2i32, 5u8), (18, 5), (2, 4)] {
        let o = run(f_o, cnt, e8_val);
        let n = run(f_n, cnt, e8_val);
        snap.restore();
        crate::diff_eq!(o, n, "1420d4180 cnt={} e8={}", cnt, e8_val);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420e93c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HEAP_BUF: [u8; 64] = [0; 64];
    static mut OUT_BYTES: [u8; 32] = [0; 32];
    static mut FREED_PTR: usize = 0;

    unsafe extern "win64" fn mock_alloc(_size: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(HEAP_BUF) as usize
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        FREED_PTR = ptr;
    }
    unsafe extern "win64" fn mock_9b50(dst_str: usize, span_ptr: usize, _flag: u32) {
        let p = rd::<usize>(span_ptr);
        let len = rd::<u64>(span_ptr + 0x08) as usize;
        wr::<u32>(dst_str, len as u32);
        OUT_BYTES = [0; 32];
        for i in 0..len.min(32) {
            OUT_BYTES[i] = rd::<u8>(p + i);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let run = |f: F, wchars: &[u16]| -> (u8, u32, [u8; 32], bool) {
        HEAP_BUF = [0; 64];
        OUT_BYTES = [0; 32];
        FREED_PTR = 0;

        let mut wstr = [0u64; 3];
        let wp = wstr.as_mut_ptr() as usize;
        wr::<usize>(wp, wchars.as_ptr() as usize);
        wr::<u32>(wp + 0x10, wchars.len() as u32);

        let mut out_rstr = [0u64; 3];
        let rp = out_rstr.as_mut_ptr() as usize;
        let ok = f(wp, rp);
        let freed_ok = FREED_PTR == (core::ptr::addr_of!(HEAP_BUF) as usize);
        (ok, rd::<u32>(rp), OUT_BYTES, freed_ok)
    };

    let valid_utf16: [u16; 4] = [0x0041, 0x00e9, 0x20ac, 0xd83d]; // last is unpaired high surrogate
    let good_utf16: [u16; 5] = [0x0041, 0x00e9, 0x20ac, 0xd83d, 0xde00];
    for slice in [&good_utf16[..], &valid_utf16[..]] {
        let o = run(f_o, slice);
        let n = run(f_n, slice);
        crate::diff_eq!(o, n, "1420e93c0 len={}", slice.len());
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420ea540(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CONV_SOURCE: usize = 0;

    unsafe extern "win64" fn mock_9b50(dst: usize, span: usize, _flag: u32) {
        wr::<u64>(dst + 0x08, rd::<u64>(span));
        wr::<u32>(dst + 0x04, rd::<u32>(span + 0x08));
    }
    unsafe extern "win64" fn mock_7760(tbl: usize, out_pair: usize, _key: usize) {
        wr::<usize>(out_pair, rd::<usize>(tbl));
        wr::<usize>(out_pair + 0x08, rd::<usize>(tbl) + 0x100);
    }
    unsafe extern "win64" fn mock_6ff0(_entry: usize, _key: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_76e0(_tbl: usize) {}
    unsafe extern "win64" fn mock_74f0(a: usize, b: usize) -> u8 {
        let va = rd::<u32>(a + 0x04);
        let vb = rd::<u32>(b + 0x04);
        if va < vb { 1 } else { 0 }
    }
    unsafe extern "win64" fn mock_93c0(wstr: usize, out_rstr: usize) -> u8 {
        CONV_SOURCE = wstr;
        wr::<u32>(out_rstr, (wstr & 0xffff) as u32);
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411f_7760, mock_7760 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411f_76e0, mock_76e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4021_74f0, mock_74f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420e_93c0, mock_93c0 as *const ())?;

    let run = |f: F, hit_first: bool| -> (u8, u32, usize) {
        CONV_SOURCE = 0;

        let mut node = [0u8; 0xd0];
        let np = node.as_mut_ptr() as usize;
        let mut head = [0u8; 0x30];
        let hp = head.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x08, np);
        wr::<u8>(hp + 0x19, 1);

        wr::<usize>(np, hp);
        wr::<usize>(np + 0x08, hp);
        wr::<usize>(np + 0x10, hp);
        wr::<u8>(np + 0x19, 0);
        wr::<u32>(np + 0x20 + 0x04, 4); // key len == 4 ("test")
        wr::<usize>(np + 0xa8, 0x5555);
        wr::<u32>(np + 0xa8 + 0x10, 77);

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x08, u32::MAX);
        wr::<usize>(p1p + 0x10, if hit_first { 0x2000 } else { 0x1000 });
        wr::<usize>(p1p + 0x18, 0x1000);
        wr::<u32>(p1p + 0x20, 0);
        wr::<usize>(p1p + 0x38, 0x3000);
        wr::<usize>(p1p + 0x40, 0x3000);
        wr::<u32>(p1p + 0x48, 0);
        wr::<usize>(p1p + 0x60, hp);

        let cstr = b"test\0";
        let mut out_rstr = [0u64; 3];
        let rp = out_rstr.as_mut_ptr() as usize;
        let ret = f(p1p, cstr.as_ptr() as usize, rp);
        (ret, rd::<u32>(rp), CONV_SOURCE)
    };

    for hit_first in [true, false] {
        let o = run(f_o, hit_first);
        let n = run(f_n, hit_first);
        crate::diff_eq!(o, n, "1420ea540 hit_first={}", hit_first);
    }
    Ok(2)
}

pub unsafe fn diff_physics_14217e2a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        u64,
        usize,
        usize,
        usize,
        usize,
        u32,
        u64,
        u8,
        u64,
    );
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VFUNC20_ARG: u8 = 0;

    unsafe extern "win64" fn mock_vfunc0(_this: usize, buf: usize) -> u64 {
        wr::<u64>(buf, 0x1122_3344);
        0x9988
    }
    unsafe extern "win64" fn mock_vfunc20(_this: usize, dl: u8) {
        VFUNC20_ARG = dl;
    }
    unsafe extern "win64" fn mock_ab60(
        p1: usize,
        _p2: u64,
        _b_a0: usize,
        b_80: usize,
        _b_e0: usize,
        _p7: u32,
        _p8: u64,
        _xmm12: u32,
        _p9: u8,
        _r15b: u8,
        _p10: u64,
    ) {
        let step = rd::<u128>(b_80);
        let cur = rd::<u128>(p1 + 0xa0);
        wr::<u128>(p1 + 0x20, step);
        wr::<u128>(p1 + 0xa0, crate::simd_ops::vaddps(cur, step));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4217_ab60, mock_ab60 as *const ())?;

    let run = |f: F| -> ([u128; 2], [u8; 0xc0], [u64; 12], u8) {
        VFUNC20_ARG = 0;

        let mut pose: [u128; 2] = [
            f32::to_bits(0.0) as u128
                | ((f32::to_bits(0.0) as u128) << 32)
                | ((f32::to_bits(0.0) as u128) << 64)
                | ((f32::to_bits(1.0) as u128) << 96),
            f32::to_bits(1.0) as u128
                | ((f32::to_bits(2.0) as u128) << 32)
                | ((f32::to_bits(3.0) as u128) << 64),
        ];
        let mut p68 = [0u64; 12];
        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, pose.as_mut_ptr() as usize);
        wr::<u8>(p1p + 0x16, 1);
        wr::<usize>(p1p + 0x68, p68.as_mut_ptr() as usize);
        wr::<u128>(p1p + 0xa0, pose[1]);

        let mut vtbl = [0usize; 5];
        vtbl[0] = mock_vfunc0 as *const () as usize;
        vtbl[4] = mock_vfunc20 as *const () as usize;
        let sub_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        let mut p3 = [0usize; 8];
        p3[7] = sub_obj.as_ptr() as usize;

        let p4: [u128; 2] = [
            pose[0],
            f32::to_bits(0.5) as u128
                | ((f32::to_bits(0.25) as u128) << 32)
                | ((f32::to_bits(-0.5) as u128) << 64),
        ];
        let p5: [u128; 1] = [pose[0]];
        let p6: [u128; 2] = [0, 0];

        f(
            p1p,
            0x1234,
            p3.as_mut_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            p6.as_ptr() as usize,
            0.016f32.to_bits(),
            0,
            0,
            0,
        );
        wr::<usize>(p1p, 0);
        wr::<usize>(p1p + 0x68, 0);
        (pose, p1, p68, VFUNC20_ARG)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14217e2a0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1421cc210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut OP_LOG: u64 = 0;

    unsafe extern "win64" fn mock_nop1(_a: usize) {}
    unsafe extern "win64" fn mock_nop2(_a: usize, _b: usize) {}
    unsafe extern "win64" fn mock_5a70(_a: usize) -> u64 {
        0x77
    }
    unsafe extern "win64" fn mock_5b40(rcx_20: usize) -> usize {
        rd::<usize>(rcx_20 + 0x108) + 0x80
    }
    unsafe extern "win64" fn mock_9e40(_a: u64, _b: u64, _c: u32) -> i32 {
        3
    }
    unsafe extern "win64" fn mock_4c70(_a: usize, _b: u64, _c: usize) {}
    unsafe extern "win64" fn mock_1d50(_a: usize, _b: u64, _c: u64, _d: usize) {}
    unsafe extern "win64" fn mock_op(p58: usize, p40: usize) {
        let a = rd::<u64>(p58 + 0x08);
        let b = rd::<u64>(p58 + 0x10);
        let idx = rd::<u32>(p58 + 0x18) as u64;
        let w = rd::<u32>(p40 + 0x08) as u64;
        let rem = rd::<u32>(p40 + 0x0c) as u64;
        OP_LOG = OP_LOG
            .wrapping_mul(31)
            .wrapping_add(a ^ b ^ idx ^ (w << 8) ^ rem);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_cef0, mock_nop2 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421c_ce30, mock_nop2 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420c_ba20, mock_nop2 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_9e40, mock_9e40 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_421d_5f70, mock_nop1 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_421d_5c20, mock_nop1 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_421d_5a70, mock_5a70 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_421d_a330, mock_nop2 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_421d_5b40, mock_5b40 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_421d_4c70, mock_4c70 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_421d_1d50, mock_1d50 as *const ())?;

    let g_tbl = exe::ptr::<usize>(0x1_4386_5e50) as usize;
    let saved_fn0 = rd::<usize>(g_tbl);
    wr::<usize>(g_tbl, mock_op as *const () as usize);

    let run = |f: F| -> (Vec<u8>, Vec<u8>, u8, u64) {
        OP_LOG = 0;

        let c8_data: [u64; 3] = [2, 0, 0x5555];
        let mut r12_entries = [0u32; 6];
        r12_entries[0] = 0; // op_tag = 0
        r12_entries[1] = 0x10;
        r12_entries[2] = 0x20;
        r12_entries[3] = 0; // op_tag = 0
        r12_entries[4] = 0x30;
        r12_entries[5] = 0x40;

        let bit_offsets: [u32; 2] = [5, 70];
        let mask_a: [u64; 1] = [0b11];
        let mask_b: [u64; 1] = [0b10];

        let mut r13 = vec![0u8; 0x130];
        let r13p = r13.as_mut_ptr() as usize;
        wr::<usize>(r13p + 0x08, r12_entries.as_ptr() as usize);
        wr::<u64>(r13p + 0x10, 0x1000);
        wr::<usize>(r13p + 0x30, bit_offsets.as_ptr() as usize);
        wr::<usize>(r13p + 0x80, mask_a.as_ptr() as usize);
        wr::<usize>(r13p + 0xc8, c8_data.as_ptr() as usize);
        wr::<u16>(r13p + 0x120, 4);

        let d0_val: u32 = 99;
        let mut rbp = vec![0u8; 0xe0];
        let rbpp = rbp.as_mut_ptr() as usize;
        wr::<u16>(rbpp + 0x02, 1);
        wr::<u8>(rbpp + 0x06, 2);
        wr::<usize>(rbpp + 0x08, mask_b.as_ptr() as usize);
        wr::<u64>(rbpp + 0xc0, 0x2000);
        wr::<usize>(rbpp + 0xd0, (&d0_val as *const u32) as usize);

        let rdi_map: [u16; 2] = [1, 0xffff];
        let mut r90_buf = vec![0u8; 0x100];
        let r90p = r90_buf.as_mut_ptr() as usize;
        for i in 0..32usize {
            wr::<u8>(r90p + 0x80 + i, (i as u8) + 1);
        }

        let mut stack_30 = [0xaa_u8; 0x30];
        let mut rcx_20 = vec![0u8; 0x1c0];
        let r20p = rcx_20.as_mut_ptr() as usize;
        wr::<i32>(r20p + 0x14, 1);
        wr::<usize>(r20p + 0x20, stack_30.as_mut_ptr() as usize + 0x30);

        let mut item = [0u8; 0x30];
        let ip = item.as_mut_ptr() as usize;
        wr::<usize>(ip, r13p);
        wr::<usize>(ip + 0x08, rbpp);
        wr::<usize>(ip + 0x10, rdi_map.as_ptr() as usize);
        wr::<u32>(ip + 0x24, 10);
        wr::<u8>(ip + 0x28, 1);

        let mut p1 = vec![0u8; 0x110];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x20, r20p);
        wr::<u64>(p1p + 0x28, 0x8888);
        wr::<u8>(p1p + 0x3c, 1);
        wr::<u8>(p1p + 0x3d, 1);
        wr::<usize>(p1p + 0x70, ip);
        wr::<usize>(p1p + 0x90, r90p);

        f(p1p, 0);
        // Clear pointer fields in p1 before comparison
        wr::<usize>(p1p + 0x20, 0);
        wr::<usize>(p1p + 0x70, 0);
        wr::<usize>(p1p + 0x90, 0);
        (p1, r90_buf, rd::<u8>(rbpp + 0x07), OP_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_tbl, saved_fn0);
    crate::diff_eq!(o, n, "1421cc210 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1421cdbf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_ACC: u64 = 0;

    unsafe extern "win64" fn mock_b380(_p1: usize, rdx: u64, r8: u32) {
        CALL_ACC = CALL_ACC.wrapping_add(rdx).wrapping_add(r8 as u64);
    }
    unsafe extern "win64" fn mock_ca40(_p1: usize) {
        CALL_ACC = CALL_ACC.wrapping_mul(17).wrapping_add(3);
    }
    unsafe extern "win64" fn mock_cb(_p1: usize, arg: u32) {
        CALL_ACC = CALL_ACC.wrapping_mul(31).wrapping_add(arg as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421c_b380, mock_b380 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421c_ca40, mock_ca40 as *const ())?;

    let run = |f: F, b3c: u8, b103: u8| -> (Vec<u8>, u8, u64) {
        CALL_ACC = 0;
        let mut r20 = vec![0u8; 0x160];
        let r20p = r20.as_mut_ptr() as usize;
        wr::<u8>(r20p + 0x15a, 0x06);

        let mut cbs = [0u8; 0x20];
        let cbp = cbs.as_mut_ptr() as usize;
        wr::<u32>(cbp + 0x04, 111);
        wr::<usize>(cbp + 0x08, mock_cb as *const () as usize);
        wr::<u32>(cbp + 0x14, 222);
        wr::<usize>(cbp + 0x18, mock_cb as *const () as usize);

        let mut p1 = vec![0x5au8; 0x110];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x20, r20p);
        wr::<u64>(p1p + 0x28, 0x1234);
        wr::<u8>(p1p + 0x3c, b3c);
        wr::<usize>(p1p + 0x40, cbp);
        wr::<u64>(p1p + 0x50, 2);
        wr::<u8>(p1p + 0x103, b103);

        f(p1p);
        wr::<usize>(p1p + 0x20, 0);
        wr::<usize>(p1p + 0x40, 0);
        (p1, rd::<u8>(r20p + 0x15a), CALL_ACC)
    };

    for (b3c, b103) in [(1u8, 1u8), (0, 0)] {
        let o = run(f_o, b3c, b103);
        let n = run(f_n, b3c, b103);
        crate::diff_eq!(o, n, "1421cdbf0 b3c={} b103={}", b3c, b103);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified(0x1_420c_1320, "unknown_1420c1320", rend::cauldron_hot_v5::unknown_1420c1320 as *const (), rend::cauldron_hot_v5::diff_unknown_1420c1320),
// Replacement::verified(0x1_420d_1690, "unknown_1420d1690", rend::cauldron_hot_v5::unknown_1420d1690 as *const (), rend::cauldron_hot_v5::diff_unknown_1420d1690),
// Replacement::verified(0x1_420d_2440, "unknown_1420d2440", rend::cauldron_hot_v5::unknown_1420d2440 as *const (), rend::cauldron_hot_v5::diff_unknown_1420d2440),
// Replacement::verified(0x1_420d_4180, "unknown_1420d4180", rend::cauldron_hot_v5::unknown_1420d4180 as *const (), rend::cauldron_hot_v5::diff_unknown_1420d4180),
// Replacement::verified_throws(0x1_420e_93c0, "unknown_1420e93c0", rend::cauldron_hot_v5::unknown_1420e93c0 as *const (), Throws::Discard, rend::cauldron_hot_v5::diff_unknown_1420e93c0),
// Replacement::verified_throws(0x1_420e_a540, "unknown_1420ea540", rend::cauldron_hot_v5::unknown_1420ea540 as *const (), Throws::Discard, rend::cauldron_hot_v5::diff_unknown_1420ea540),
// Replacement::verified_throws(0x1_4217_e2a0, "physics_14217e2a0", rend::cauldron_hot_v5::physics_14217e2a0 as *const (), Throws::Discard, rend::cauldron_hot_v5::diff_physics_14217e2a0),
// Replacement::verified(0x1_421c_c210, "unknown_1421cc210", rend::cauldron_hot_v5::unknown_1421cc210 as *const (), rend::cauldron_hot_v5::diff_unknown_1421cc210),
// Replacement::verified(0x1_421c_dbf0, "unknown_1421cdbf0", rend::cauldron_hot_v5::unknown_1421cdbf0 as *const (), rend::cauldron_hot_v5::diff_unknown_1421cdbf0),
