//! Decompiled hot functions shard X3 (`0x141600880..0x141f7a100`).

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vcmpps, vdpps, vhaddps, vhsubps, vinsertps, vmovmskps, vmulps,
    vpermilps, vshufps, vsubps,
};

#[inline(always)]
fn rsqrt_ss(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        f32::from_bits(crate::simd_ops::vrsqrtss(0, x.to_bits()) as u32)
    }
}

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
unsafe fn call_va_f32(va: u64, rcx: u64, rdx: u64) -> f32 {
    let (_, xmm0) = crate::simd_ops::call_va8_xmm(va, rcx, rdx, 0, 0, 0, 0, 0, 0, [0; 4]);
    f32::from_bits(xmm0 as u32)
}

/// `unknown_141600880`
/// Original: 0x141600880 (428 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141600880(param_1: usize) {
    let p_e8 = rd::<usize>(param_1 + 0xe8);
    let rsi = rd::<usize>(p_e8 + 0x18);
    let rcx1 = rsi + 0x18;
    let vtbl1 = rd::<usize>(rcx1);
    if (rd::<u8>(rsi + 0x210) & 1) == 0 {
        exe::call_va!(rd::<usize>(vtbl1 + 0x68), (rcx1, 0u32));
    } else {
        exe::call_va!(rd::<usize>(vtbl1 + 0x70), (rcx1));
        if (rd::<u8>(rsi + 0x210) & 0x10) != 0 {
            let vtbl1b = rd::<usize>(rcx1);
            exe::call_va!(rd::<usize>(vtbl1b + 0x78), (rcx1));
        }
    }

    let p_f0 = rd::<usize>(param_1 + 0xf0);
    if p_f0 != 0 {
        let rdi = rd::<usize>(p_f0 + 0x18) + 0x18;
        let vtbl2 = rd::<usize>(rdi);
        if (rd::<u8>(rdi + 0x1f8) & 1) == 0 {
            exe::call_va!(rd::<usize>(vtbl2 + 0x68), (rdi, 0u32));
        } else {
            exe::call_va!(rd::<usize>(vtbl2 + 0x70), (rdi));
            if (rd::<u8>(rdi + 0x1f8) & 0x10) != 0 {
                let vtbl2b = rd::<usize>(rdi);
                exe::call_va!(rd::<usize>(vtbl2b + 0x78), (rdi));
            }
        }
    }

    if (rd::<u8>(rsi + 0x211) & 3) == 0 {
        let rdi = rd::<usize>(param_1 + 0x08);
        let c_05 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6564) as usize);
        let c_m37 = rd::<f32>(exe::ptr::<f32>(0x1_4267_3708) as usize);
        let xmm0 = rd::<f32>(rdi + 0x88) * c_05;
        let xmm1 = if xmm0 >= c_m37 {
            let c_mul = rd::<f32>(exe::ptr::<f32>(0x1_4267_36fc) as usize);
            let c_add = rd::<f32>(exe::ptr::<f32>(0x1_4267_3700) as usize);
            let u = crate::simd_ops::cvt_f32_i64(xmm0 * c_mul + c_add) as u32;
            let mant_bits = (u & 0x007f_ffff).wrapping_add(0x3f80_0000);
            let exp_bits = u & 0xff80_0000;
            let m = f32::from_bits(mant_bits);
            let e = f32::from_bits(exp_bits);
            let c_p2 = rd::<f32>(exe::ptr::<f32>(0x1_4267_36ec) as usize);
            let c_p1 = rd::<f32>(exe::ptr::<f32>(0x1_4267_36e8) as usize);
            let c_p0 = rd::<f32>(exe::ptr::<f32>(0x1_4267_36f0) as usize);
            ((m * c_p2 + c_p1) * m + c_p0) * e
        } else {
            0.0f32
        };
        wr::<f32>(param_1 + 0x10, xmm1 * rd::<f32>(rdi + 0x100));
        if (rd::<u8>(rdi + 0x160) & 8) != 0 {
            let r = exe::call_va!(0x1_4158_fd60, (rdi) -> i32);
            if r != 0 {
                exe::call_va!(0x1_415d_7890, (rdi, rdi + 0x198));
            }
        }
        exe::call_va!(0x1_4164_5950, (param_1));
    } else {
        let c = exe::call_va!(0x1_4159_5e10, (rsi) -> u8);
        if c != 0 || rd::<u32>(param_1 + 0x11c) == 0 {
            wr::<u32>(param_1 + 0x11c, 2);
            wr::<u32>(param_1 + 0x118, 1);
        }
        wr::<u32>(param_1 + 0x10, 0);
        wr::<u32>(rsi + 0x1b8, 0);
        exe::call_va!(0x1_4159_6870, (rsi));
        wr::<u32>(param_1 + 0x48, 0);
    }
}

/// `unknown_141645fc0`
/// Original: 0x141645fc0 (1963 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141645fc0(param_1: usize, param_2: u8) {
    let mut rdi = rd::<usize>(param_1 + 0x08);
    let xmm13 = if (rd::<u8>(rdi + 0x13f) & 1) != 0 {
        let vtbl = rd::<usize>(param_1);
        call_va_f32(rd::<u64>(vtbl + 0x60), param_1 as u64, 0)
    } else {
        0.0f32
    };

    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    wr::<u32>(param_1 + 0x58, 0);
    wr::<u32>(param_1 + 0x14, 0xc580_0000);
    let mut local_140: u64 = 1;
    let rcx_78 = rd::<usize>(rdi + 0x78);
    let mut xmm12 = if rcx_78 != 0 {
        call_va_f32(
            0x1_415c_27c0,
            rcx_78 as u64,
            (&mut local_140 as *mut u64) as usize as u64,
        )
    } else {
        c_one
    };
    xmm12 *= rd::<f32>(param_1 + 0x10);

    let mut rsi = rd::<usize>(param_1 + 0x38);
    let mut r13: usize = 0;
    if rsi == 0 {
        return;
    }

    let xmm14 = rd::<f32>(exe::ptr::<f32>(0x1_4268_ebb0) as usize);
    let xmm15 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6564) as usize);

    while rsi != 0 {
        let ebp = rd::<u32>(rsi + 0x194);
        let mut xmm6 = 0.0f32;
        let r14 = rd::<u64>(rsi + 0x170);
        let r15 = rdi + 0x198;
        let rbx_arr = rd::<usize>(r15);
        let mut xmm7: f32;

        if ebp == 0 {
            let b4 = rd::<f32>(rdi + 0xb4);
            let cached_b4 = rd::<f32>(rdi + 0x200);
            if !b4.is_nan() && !cached_b4.is_nan() && b4 == cached_b4 {
                xmm7 = rd::<f32>(rdi + 0x1fc);
            } else {
                wr::<f32>(rdi + 0x200, b4);
                let scaled = b4 * xmm15;
                let c_m37 = rd::<f32>(exe::ptr::<f32>(0x1_4267_3708) as usize);
                if scaled >= c_m37 {
                    let c_mul = rd::<f32>(exe::ptr::<f32>(0x1_4267_36fc) as usize);
                    let c_add = rd::<f32>(exe::ptr::<f32>(0x1_4267_3700) as usize);
                    let u = crate::simd_ops::cvt_f32_i64(scaled * c_mul + c_add) as u32;
                    let exp_bits = u & 0xff80_0000;
                    let mant_bits = (u & 0x007f_ffff).wrapping_add(0x3f80_0000);
                    let m = f32::from_bits(mant_bits);
                    let e = f32::from_bits(exp_bits);
                    let c_p2 = rd::<f32>(exe::ptr::<f32>(0x1_4267_36ec) as usize);
                    let c_p1 = rd::<f32>(exe::ptr::<f32>(0x1_4267_36e8) as usize);
                    let c_p0 = rd::<f32>(exe::ptr::<f32>(0x1_4267_36f0) as usize);
                    xmm7 = ((m * c_p2 + c_p1) * m + c_p0) * e;
                } else {
                    xmm7 = 0.0f32;
                }
                wr::<f32>(rdi + 0x1fc, xmm7);
            }

            if (rd::<u8>(rdi + 0x160) & 8) != 0 {
                let count = rd::<u32>(r15 + 0x08) as usize;
                let end_arr = rbx_arr.wrapping_add(count * 0x88);
                let mut cur = rbx_arr;
                while cur != end_arr {
                    if r14 == rd::<u64>(cur + 0x70) {
                        let v = rd::<f32>(cur + 0x3c);
                        if v > xmm6 {
                            xmm6 = v;
                        }
                    }
                    cur = cur.wrapping_add(0x88);
                }
                let mut ctx_ptr = exe::call_va!(0x1_4160_5340, (rd::<u64>(rdi + 0x08)) -> usize);
                let mut key_arg: [u64; 2] = [r14, 0];
                if rd::<u8>(ctx_ptr + 0xe0) != 0 {
                    ctx_ptr = ctx_ptr.wrapping_add(0x20);
                } else {
                    let tmp = exe::call_va!(0x1_4158_c580, () -> u64);
                    ctx_ptr = exe::call_va!(0x1_4158_d2f0, (tmp) -> usize);
                }
                let mul_val = call_va_f32(
                    0x1_4160_38d0,
                    ctx_ptr as u64,
                    key_arg.as_mut_ptr() as usize as u64,
                );
                xmm7 *= mul_val;
            } else {
                xmm6 = c_one;
            }
            wr::<u32>(rsi + 0x178, rd::<u32>(rdi + 0xbc));
            wr::<u32>(rsi + 0x17c, rd::<u32>(rdi + 0xb8));
        } else {
            let rbp_obj = exe::call_va!(0x1_4162_ee80, (rsi) -> u64);
            let mut cur_p = rd::<usize>(param_1 + 0x40);
            let end_p = cur_p.wrapping_add((rd::<u32>(param_1 + 0x48) as usize) * 0x20);
            let mut found = false;
            xmm7 = 0.0f32;
            let mut xmm8 = 0.0f32;
            let mut xmm9 = 0.0f32;

            if cur_p != end_p {
                loop {
                    let id32 = exe::call_va!(0x1_4163_41b0, (rbp_obj) -> u32);
                    if rd::<u32>(cur_p + 0x08) == id32 {
                        let id64 = exe::call_va!(0x1_4163_4020, (rbp_obj) -> u64);
                        if rd::<u64>(cur_p) == id64 {
                            xmm7 += rd::<f32>(cur_p + 0x0c);
                            xmm8 += rd::<f32>(cur_p + 0x14);
                            xmm9 += rd::<f32>(cur_p + 0x18);
                            found = true;
                        }
                    }
                    cur_p = cur_p.wrapping_add(0x20);
                    let cur_end = rd::<usize>(param_1 + 0x40)
                        .wrapping_add((rd::<u32>(param_1 + 0x48) as usize) * 0x20);
                    if cur_p == cur_end {
                        break;
                    }
                }
            }

            if !found {
                xmm7 = 0.0f32;
                if (rd::<u8>(rsi + 0x198) & 1) != 0 {
                    let p8 = rd::<usize>(param_1 + 0x08);
                    if p8 != 0 {
                        wr::<u32>(p8 + 0x164, 0x42ca_0000);
                    }
                    let g_be9 = exe::ptr::<u8>(0x1_4393_1be9) as usize;
                    let mut v_be9 = rd::<u8>(g_be9);
                    if (rd::<u8>(param_1 + 0x60) & 4) != 0 {
                        v_be9 = 1;
                    }
                    wr::<u8>(g_be9, v_be9);

                    let owner = exe::call_va!(0x1_4162_ee70, (rsi) -> usize);
                    let head = rd::<usize>(owner + 0x20);
                    let mut prev: usize = 0;
                    let mut scan = head;
                    while scan != 0 {
                        if scan == rsi {
                            let next_18 = rd::<usize>(scan + 0x18);
                            if scan == head {
                                wr::<usize>(owner + 0x20, next_18);
                            } else {
                                wr::<usize>(prev + 0x18, next_18);
                            }
                            if scan == rd::<usize>(owner + 0x18) {
                                wr::<usize>(owner + 0x18, prev);
                            }
                            break;
                        }
                        prev = scan;
                        scan = rd::<usize>(scan + 0x18);
                    }

                    let owner2 = exe::call_va!(0x1_4162_ee70, (rsi) -> usize);
                    wr::<u32>(owner2 + 0x28, rd::<u32>(owner2 + 0x28).wrapping_sub(1));
                    let dead = rsi;
                    let next_20 = rd::<usize>(rsi + 0x20);
                    if rsi == rd::<usize>(param_1 + 0x38) {
                        wr::<usize>(param_1 + 0x38, next_20);
                    } else {
                        wr::<usize>(r13 + 0x20, next_20);
                    }
                    rsi = next_20;
                    if dead != 0 {
                        let vtbl = rd::<usize>(dead);
                        exe::call_va!(rd::<usize>(vtbl), (dead, 0u32));
                        exe::call_va!(0x1_4152_42e0, (5u32, dead));
                    }
                    rdi = rd::<usize>(param_1 + 0x08);
                    continue;
                }
            }

            wr::<f32>(rsi + 0x178, xmm8);
            wr::<f32>(rsi + 0x17c, xmm9);
            if (rd::<u8>(rdi + 0x160) & 8) == 0 {
                xmm6 = c_one;
            } else if ebp == 1 {
                let count = rd::<u32>(r15 + 0x08) as usize;
                let end_arr = rbx_arr.wrapping_add(count * 0x88);
                let mut cur = rbx_arr;
                while cur != end_arr {
                    if r14 == rd::<u64>(cur + 0x70) {
                        let v = rd::<f32>(cur + 0x40);
                        if v > xmm6 {
                            xmm6 = v;
                        }
                    }
                    cur = cur.wrapping_add(0x88);
                }
            } else if ebp == 2 {
                let count = rd::<u32>(r15 + 0x08) as usize;
                let end_arr = rbx_arr.wrapping_add(count * 0x88);
                let mut cur = rbx_arr;
                while cur != end_arr {
                    if r14 == rd::<u64>(cur + 0x70) {
                        let v = rd::<f32>(cur + 0x44);
                        if v > xmm6 {
                            xmm6 = v;
                        }
                    }
                    cur = cur.wrapping_add(0x88);
                }
            } else {
                xmm6 = c_one;
            }
        }

        let prod = (xmm7 * xmm12) * xmm6;
        if !(prod > 0.0f32) {
            let old_flags = rd::<u32>(rsi + 0x198);
            let mut flags = ((old_flags.wrapping_mul(2) ^ old_flags) & 2) ^ old_flags;
            wr::<u32>(rsi + 0x198, flags);
            let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4381_5b3c) as usize);
            wr::<f32>(rsi + 0x188, xmm7);
            let bit0 = (!thresh.is_nan() && !xmm14.is_nan() && thresh >= xmm14) as u32;
            flags = (flags & !1u32) | bit0;
            let prev_c = rd::<u32>(rsi + 0x0c);
            wr::<u32>(rsi + 0x0c, 0);
            wr::<u32>(rsi + 0x08, prev_c);
            let prev_14 = rd::<u32>(rsi + 0x14);
            wr::<f32>(rsi + 0x14, xmm6);
            wr::<u32>(rsi + 0x10, prev_14);
            wr::<u32>(rsi + 0x18c, 0xc580_0000);
            wr::<u32>(rsi + 0x198, flags);
        } else {
            let bits = prod.to_bits();
            let mant_f = f32::from_bits((bits & 0x007f_ffff).wrapping_add(0x3f80_0000));
            let exp_u = (bits >> 23) & 0xff;
            let p_158 = rd::<usize>(rsi + 0x158);
            let m_plus_1 = mant_f + c_one;
            let num = (mant_f * mant_f * mant_f - c_one)
                * rd::<f32>(exe::ptr::<f32>(0x1_4267_648c) as usize);
            let den = m_plus_1 * m_plus_1 * m_plus_1;
            let term1 = (num / den) * rd::<f32>(exe::ptr::<f32>(0x1_4342_6e1c) as usize);
            let term2 = ((exp_u as f32) - rd::<f32>(exe::ptr::<f32>(0x1_4342_6eb8) as usize))
                * rd::<f32>(exe::ptr::<f32>(0x1_4267_664c) as usize);
            let mut db = term1 + term2 + rd::<f32>(p_158 + 0x14);

            let old_flags = rd::<u32>(rsi + 0x198);
            let mut flags = ((old_flags.wrapping_mul(2) ^ old_flags) & 2) ^ old_flags;
            wr::<u32>(rsi + 0x198, flags);
            let thresh = rd::<f32>(exe::ptr::<f32>(0x1_4381_5b3c) as usize);
            let bit0 = if param_2 != 0 && !db.is_nan() && !thresh.is_nan() && db <= thresh {
                1u32
            } else {
                0u32
            };
            flags = (flags & !1u32) | bit0;
            wr::<f32>(rsi + 0x18c, db);
            wr::<f32>(rsi + 0x188, xmm7);
            let prev_c = rd::<u32>(rsi + 0x0c);
            wr::<u32>(rsi + 0x08, prev_c);
            let prev_14 = rd::<u32>(rsi + 0x14);
            wr::<f32>(rsi + 0x14, xmm6);
            wr::<u32>(rsi + 0x10, prev_14);
            wr::<u32>(rsi + 0x198, flags);
            wr::<u32>(rsi + 0x0c, 0);

            if db > rd::<f32>(param_1 + 0x14) {
                wr::<f32>(param_1 + 0x14, db);
            }

            let p_160 = rd::<usize>(rsi + 0x160);
            if p_160 != 0 && (rd::<i8>(rdi + 0x1f8) < 0) {
                let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
                let neg_x13 = f32::from_bits(xmm13.to_bits() ^ sign_mask);
                if neg_x13 < rd::<f32>(rdi + 0xc0) {
                    db += xmm13;
                    if db > rd::<f32>(p_160 + 0x1c8) {
                        wr::<f32>(p_160 + 0x1c8, db);
                    }
                }
                let rbx_160 = rd::<usize>(rsi + 0x160);
                let entry_5cf0 = exe::call_va!(
                    0x1_4164_5cf0,
                    (
                        param_1 + 0x50,
                        rbx_160,
                        (&mut local_140 as *mut u64) as usize
                    ) -> usize
                );
                if entry_5cf0 == 0 {
                    let old_f = rd::<u32>(rsi + 0x198);
                    let mut f2 = ((old_f.wrapping_mul(2) ^ old_f) & 2) ^ old_f;
                    wr::<u32>(rsi + 0x198, f2);
                    let thresh2 = rd::<f32>(exe::ptr::<f32>(0x1_4381_5b3c) as usize);
                    wr::<u32>(rsi + 0x188, 0);
                    let b0 = (!thresh2.is_nan() && !xmm14.is_nan() && thresh2 >= xmm14) as u32;
                    f2 = (f2 & !1u32) | b0;
                    let pc = rd::<u32>(rsi + 0x0c);
                    wr::<u32>(rsi + 0x0c, 0);
                    wr::<u32>(rsi + 0x08, pc);
                    let p14 = rd::<u32>(rsi + 0x14);
                    wr::<f32>(rsi + 0x14, xmm6);
                    wr::<u32>(rsi + 0x10, p14);
                    wr::<u32>(rsi + 0x18c, 0xc580_0000);
                    wr::<u32>(rsi + 0x198, f2);
                } else {
                    let mut xmm1 = rd::<f32>(rbx_160 + 0x1d8) + rd::<f32>(rbx_160 + 0x14);
                    let diff = rd::<f32>(rsi + 0x18c) - xmm1;
                    if diff > 0.0f32 {
                        xmm1 = diff * rd::<f32>(rbx_160 + 0x1dc) + xmm1;
                    }
                    if (local_140 as u8) == 0 || xmm1 > rd::<f32>(entry_5cf0 + 0x08) {
                        wr::<f32>(entry_5cf0 + 0x08, xmm1);
                    }
                }
            }
        }

        if ebp != 3 && (rd::<u32>(rsi + 0x198) & 3) != 3 && rd::<u32>(r15 + 0x08) != 0 {
            exe::call_va!(0x1_415d_5240, (rdi, r15, rsi));
        }
        r13 = rsi;
        rsi = rd::<usize>(rsi + 0x20);
    }
}

#[inline(always)]
unsafe fn sso_str_ptr(entry: usize) -> usize {
    let b3 = rd::<u8>(entry + 0x03) as u32;
    let data_field = entry + 0x08;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(entry) & 0x00ff_ffff) << 3;
    if len_bits <= cap_bits {
        data_field
    } else {
        rd::<usize>(data_field)
    }
}

/// `unknown_141acd4d0`
/// Original: 0x141acd4d0 (2749 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141acd4d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
) {
    if rd::<u8>(param_5 + 0x424) == 0 {
        return;
    }
    let rsi = rd::<usize>(param_1 + 0x38);
    let rdi_arr = rd::<usize>(param_1 + 0x28);
    if rd::<u32>(rdi_arr.wrapping_add(rsi * 4)) != 0 {
        return;
    }

    let mut key_a0: u64 = 0;
    let p1_base = rd::<usize>(param_1);
    let r8_init = rd::<u64>(p1_base.wrapping_add(rsi * 0x18 + 0x08));
    exe::call_va!(
        0x1_401f_9650,
        (param_6, (&mut key_a0 as *mut u64) as usize, r8_init)
    );

    let mut buf_100 = [0u64; 7];
    exe::call_va!(
        0x1_4058_ab00,
        (param_2, buf_100.as_mut_ptr() as usize, key_a0)
    );
    let r13 = buf_100[6];
    let r12 = (buf_100[0] as usize).wrapping_add((r13 as usize).wrapping_mul(0x630));

    if rd::<u8>(r12 + 0xa5) != 0 {
        let flag_ptr = rdi_arr.wrapping_add(rsi * 4);
        wr::<u32>(flag_ptr, rd::<u32>(flag_ptr) | 1);
        let mut copy_p3: [u64; 3] = [
            rd::<u64>(param_3),
            rd::<u64>(param_3 + 0x08),
            rd::<u64>(param_3 + 0x10),
        ];
        exe::call_va!(
            0x1_41ad_4390,
            (param_5, r12, copy_p3.as_mut_ptr() as usize, 0u32)
        );
        return;
    }

    let v_90: u64;
    let v_a0: u32;
    let v_60: u64;
    let v_68: u32;

    if rd::<u32>(rd::<usize>(param_1).wrapping_add(rsi * 0x18)) != 0 {
        let mut arg_b0: [u64; 6] = [
            buf_100[1], buf_100[2], buf_100[3], buf_100[4], buf_100[5], buf_100[6],
        ];
        let mut out1 = [0u64; 2];
        let ret1 = exe::call_va!(
            0x1_41ad_6850,
            (
                out1.as_mut_ptr() as usize,
                arg_b0.as_mut_ptr() as usize,
                rd::<u64>(r12 + 0x90),
                param_5,
                0u32
            ) -> usize
        );
        v_90 = rd::<u64>(ret1);
        v_a0 = rd::<u32>(ret1 + 0x08);

        let mut arg_b0_2: [u64; 6] = [
            buf_100[1], buf_100[2], buf_100[3], buf_100[4], buf_100[5], buf_100[6],
        ];
        let mut out2 = [0u64; 2];
        let ret2 = exe::call_va!(
            0x1_41ad_6850,
            (
                out2.as_mut_ptr() as usize,
                arg_b0_2.as_mut_ptr() as usize,
                rd::<u64>(r12 + 0x90),
                param_5,
                1u32
            ) -> usize
        );
        v_60 = rd::<u64>(ret2);
        v_68 = rd::<u32>(ret2 + 0x08);
    } else {
        let s318_ptr = sso_str_ptr(r12 + 0x318);
        let s318_len = rd::<u32>(r12 + 0x31c);
        let s48_ptr = sso_str_ptr(r12 + 0x48);
        let s48_len = rd::<u32>(r12 + 0x4c);

        let mut str1: [u64; 2] = [s48_ptr as u64, s48_len as u64];
        let found1 = exe::call_va!(
            0x1_41ad_cb20,
            (param_5, str1.as_mut_ptr() as usize) -> usize
        );
        if found1 == 0 {
            return;
        }
        let mut str2: [u64; 2] = [s318_ptr as u64, s318_len as u64];
        let found2 = exe::call_va!(
            0x1_41ac_8bc0,
            (found1, str2.as_mut_ptr() as usize) -> usize
        );
        if found2 == 0 {
            return;
        }

        let r9_raw = rd::<u64>(found2 + 0xc8);
        let rdx_id = if (r9_raw as u32) == u32::MAX {
            u64::MAX
        } else {
            r9_raw
        };
        if rdx_id == u64::MAX {
            return;
        }

        let p2_0 = rd::<usize>(param_2);
        let p2_len = rd::<usize>(param_2 + 0x08);
        let mut pu3 = rd::<usize>(param_2 + 0x10);
        let idx32 = (rdx_id as u32) as usize;
        if (idx32 as u64) >= (rd::<u32>(p2_0 + 0x20) as u64) {
            return;
        }
        let entry_id = rd::<u64>(rd::<usize>(p2_0 + 0x18).wrapping_add(idx32 * 8));
        if entry_id != rdx_id || (rdx_id as u32) == u32::MAX {
            return;
        }
        let u22 = rd::<u16>(rd::<usize>(p2_0 + 0x30).wrapping_add(idx32 * 8));
        if u22 == 0xffff {
            exe::call_iat!(0x1_4264_67e8, ());
            return;
        }
        let end_pu = pu3.wrapping_add(p2_len * 2);
        let mut rem = ((end_pu.wrapping_sub(pu3)) as isize) >> 1;
        while rem > 0 {
            let half = (rem as usize) >> 1;
            let mid = pu3.wrapping_add(half * 2);
            if rd::<u16>(mid) < u22 {
                pu3 = mid.wrapping_add(2);
                rem -= (half as isize) + 1;
            } else {
                rem = half as isize;
            }
        }
        if pu3 == end_pu || u22 < rd::<u16>(pu3) {
            return;
        }

        let mut buf_b0 = [0u64; 7];
        exe::call_va!(
            0x1_4058_ab00,
            (param_2, buf_b0.as_mut_ptr() as usize, r9_raw)
        );
        let mut arg_b0: [u64; 6] = [
            buf_b0[1], buf_b0[2], buf_b0[3], buf_b0[4], buf_b0[5], buf_b0[6],
        ];
        let mut out1 = [0u64; 2];
        let ret1 = exe::call_va!(
            0x1_41ad_6850,
            (
                out1.as_mut_ptr() as usize,
                arg_b0.as_mut_ptr() as usize,
                found2 as u64,
                param_5,
                1u32
            ) -> usize
        );
        v_90 = rd::<u64>(ret1);
        v_a0 = rd::<u32>(ret1 + 0x08);

        let mut arg_b0_2: [u64; 6] = [
            buf_100[1], buf_100[2], buf_100[3], buf_100[4], buf_100[5], buf_100[6],
        ];
        let mut out2 = [0u64; 2];
        let ret2 = exe::call_va!(
            0x1_41ad_6850,
            (
                out2.as_mut_ptr() as usize,
                arg_b0_2.as_mut_ptr() as usize,
                rd::<u64>(r12 + 0x90),
                param_5,
                0u32
            ) -> usize
        );
        v_60 = rd::<u64>(ret2);
        v_68 = rd::<u32>(ret2 + 0x08);
    }

    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let mut xmm11 = f32::from_bits(v_60 as u32);
    let mut xmm12 = f32::from_bits((v_60 >> 32) as u32);
    let mut xmm13 = f32::from_bits(v_68);
    let xmm14 = f32::from_bits(v_90 as u32);
    let xmm15 = f32::from_bits((v_90 >> 32) as u32);
    let xmm7 = f32::from_bits(v_a0);

    let rdi_slot = rd::<usize>(param_1 + 0x20).wrapping_add(rsi * 0x10);
    let rcx_obj = rd::<usize>(rdi_slot);
    let b6 = rd::<u8>(rcx_obj + 0x06) as usize;
    let r14_buf = rd::<usize>(rcx_obj.wrapping_add(b6 * 8 + 0x08));
    let w4 = rd::<u16>(rcx_obj + 0x04) as u32;
    let denom_250 = (w4.wrapping_sub(4) as i32) as f32;

    let xmm8_dx = xmm11 - xmm14;
    let xmm9_dy = xmm12 - xmm15;
    let xmm10_dz = xmm13 - xmm7;

    let pf17 = rd::<usize>(param_1).wrapping_add(rsi * 0x18 + 0x10);
    if rd::<f32>(pf17) < c_one {
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4058_ad10,
            pf17 as u64,
            param_5.wrapping_add(0x88) as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, c_one.to_bits() as u128, rd::<u32>(param_7) as u128],
        );
        let mut t = rd::<f32>(pf17);
        let eps = rd::<f32>(exe::ptr::<f32>(0x1_4342_64e4) as usize);
        if t + eps > c_one {
            wr::<f32>(pf17, c_one);
            t = c_one;
        }
        xmm11 = t * xmm8_dx + xmm14;
        xmm13 = t * xmm10_dz + xmm7;
        xmm12 = t * xmm9_dy + xmm15;
    }

    let len_sq = (xmm8_dx * xmm8_dx + xmm9_dy * xmm9_dy) + xmm10_dz * xmm10_dz;
    if len_sq.is_nan() || len_sq != 0.0f32 {
        let r4 = rsqrt_ss(len_sq);
        let c_3 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6bf8) as usize);
        let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
        let abs_mask = !rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
        let inv_len = (c_half * r4) * (c_3 - (len_sq * r4) * r4);
        let nx = xmm8_dx * inv_len;
        let ny = xmm9_dy * inv_len;
        let nz = xmm10_dz * inv_len;

        let mut abs_vec: [u32; 3] = [
            nx.to_bits() & abs_mask,
            ny.to_bits() & abs_mask,
            nz.to_bits() & abs_mask,
        ];
        let mut up_vec: [f32; 3] = [0.0, c_one, 0.0];
        let angle = call_va_f32(
            0x1_4028_4700,
            abs_vec.as_mut_ptr() as usize as u64,
            up_vec.as_mut_ptr() as usize as u64,
        );
        if angle > 0.0f32 {
            let sign_flip = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
            let neg_nx = (nx.to_bits() ^ sign_flip) as u128;
            let neg_ny = (ny.to_bits() ^ sign_flip) as u128;
            let neg_nz = (nz.to_bits() ^ sign_flip) as u128;

            let c_7730 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7730) as usize);
            let xmm0_p = vpermilps(c_7730, 0xd2);
            let xmm1_p = vpermilps(c_7730, 0xc9);

            let mut xmm8_v = vblendps(0, neg_nx, 0x01);
            xmm8_v = vinsertps(xmm8_v, neg_ny, 0x10);
            xmm8_v = vinsertps(xmm8_v, neg_nz, 0x20);

            let xmm7_v = vpermilps(xmm8_v, 0xd2);
            let xmm6_v = vpermilps(xmm8_v, 0xc9);
            let xmm5_v = vsubps(vmulps(xmm1_p, xmm7_v), vmulps(xmm0_p, xmm6_v));
            let dp = vdpps(xmm5_v, xmm5_v, 0x7f);
            let r_ps = vrsqrtps_128(dp);
            let c_8460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);
            let c_8120 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize);
            let r_nr = vmulps(
                vsubps(c_8460, vmulps(vmulps(r_ps, dp), r_ps)),
                vmulps(r_ps, c_8120),
            );
            let xmm4_v = vmulps(r_nr, xmm5_v);
            let xmm5_b = vsubps(
                vmulps(vpermilps(xmm4_v, 0xd2), xmm6_v),
                vmulps(vpermilps(xmm4_v, 0xc9), xmm7_v),
            );

            let mut mat_30: [u32; 12] = [
                xmm4_v as u32,
                (xmm4_v >> 32) as u32,
                (xmm4_v >> 64) as u32,
                xmm5_b as u32,
                (xmm5_b >> 32) as u32,
                (xmm5_b >> 64) as u32,
                xmm8_v as u32,
                (xmm8_v >> 32) as u32,
                (xmm8_v >> 64) as u32,
                0,
                0,
                0,
            ];
            let mut quat_70 = [0u32; 4];
            exe::call_va!(
                0x1_4027_5010,
                (
                    quat_70.as_mut_ptr() as usize,
                    mat_30.as_mut_ptr() as usize
                )
            );

            let mut transform_100: [u32; 8] = [
                quat_70[1],
                quat_70[2],
                quat_70[3],
                quat_70[0],
                xmm11.to_bits(),
                xmm12.to_bits(),
                xmm13.to_bits(),
                0,
            ];
            let mut ctx_30: [u64; 5] = [
                rd::<u64>(param_1 + 0x08),
                rd::<u64>(param_1 + 0x18),
                rd::<u64>(param_1 + 0x10),
                rd::<u64>(param_1 + 0x30),
                rsi as u64,
            ];
            let mut copy_p4: [u64; 3] = [
                rd::<u64>(param_4),
                rd::<u64>(param_4 + 0x08),
                rd::<u64>(param_4 + 0x10),
            ];
            exe::call_va!(
                0x1_41e2_34c0,
                (
                    ctx_30.as_mut_ptr() as usize,
                    copy_p4.as_mut_ptr() as usize,
                    transform_100.as_mut_ptr() as usize
                )
            );

            let dx2 = xmm14 - xmm11;
            let dy2 = xmm15 - xmm12;
            let dz2 = xmm7 - xmm13;
            let cur_rcx = rd::<usize>(rdi_slot);
            let dist = ((dy2 * dy2 + dx2 * dx2) + dz2 * dz2).sqrt();
            let step_len = dist / denom_250;
            if rd::<u16>(cur_rcx + 0x04) != 3 {
                let c_81d0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81d0) as usize);
                let mut r13d: u32 = 0;
                loop {
                    let scaled = ((r13d as f32) * step_len).to_bits() as u128;
                    let mut v = vblendps(c_81d0, scaled, 0x01);
                    v = vinsertps(v, rd::<u32>(param_5 + 0x18) as u128, 0x10);
                    let off = ((r13d.wrapping_add(3)) as usize) * 0x20 + 0x10;
                    wr::<u128>(r14_buf.wrapping_add(off), v);
                    r13d = r13d.wrapping_add(1);
                    let cur_w4 = rd::<u16>(rd::<usize>(rdi_slot) + 0x04) as u32;
                    if r13d >= cur_w4.wrapping_sub(3) {
                        break;
                    }
                }
            }
        }
    }
}

#[inline(always)]
unsafe fn call_std_func_f32(func_obj: usize, elem_ptr: usize) -> f32 {
    let callable = rd::<usize>(func_obj + 0x38);
    if callable == 0 {
        exe::call_iat!(0x1_4264_5878, ());
        return 0.0;
    }
    let vtbl = rd::<usize>(callable);
    call_va_f32(rd::<u64>(vtbl + 0x10), callable as u64, elem_ptr as u64)
}

#[inline(always)]
unsafe fn call_std_func_vec3(func_obj: usize, out_ptr: usize, elem_ptr: usize) {
    let callable = rd::<usize>(func_obj + 0x38);
    if callable == 0 {
        exe::call_iat!(0x1_4264_5878, ());
        return;
    }
    let vtbl = rd::<usize>(callable);
    exe::call_va!(rd::<usize>(vtbl + 0x10), (callable, out_ptr, elem_ptr));
}

#[inline(always)]
unsafe fn destroy_std_func_inline(func_obj: usize) {
    let callable = rd::<usize>(func_obj + 0x38);
    if callable != 0 {
        let vtbl = rd::<usize>(callable);
        let is_heap = (callable != func_obj) as u8;
        exe::call_va!(rd::<usize>(vtbl + 0x20), (callable, is_heap));
        wr::<usize>(func_obj + 0x38, 0);
    }
}

/// `unknown_141da1a60`
/// Original: 0x141da1a60 (805 bytes, Throws::Caught)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_141da1a60(
    param_1: usize,
    param_2: f32,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> usize {
    unknown_141da1a60_impl(param_1, param_2, param_3, param_4, param_5, param_6)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_141da1a60(
    param_1: usize,
    _rdx: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    _s0: f32,
    param_2: f32,
) -> usize {
    unknown_141da1a60_impl(param_1, param_2, param_3, param_4, param_5, param_6)
}

#[inline(always)]
unsafe fn unknown_141da1a60_impl(
    param_1: usize,
    param_2: f32,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> usize {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let count = rd::<u64>(param_3);
    if count == 0 {
        wr::<u64>(param_1, rd::<u64>(param_6));
        wr::<u32>(param_1 + 0x08, rd::<u32>(param_6 + 0x08));
        destroy_std_func_inline(param_4);
        destroy_std_func_inline(param_5);
        return param_1;
    }

    let base_ptr = rd::<usize>(param_3 + 0x08);
    if count == 1 {
        call_std_func_vec3(param_5, param_1, base_ptr);
        destroy_std_func_inline(param_4);
        destroy_std_func_inline(param_5);
        return param_1;
    }

    let x_first = call_std_func_f32(param_4, base_ptr);
    if !(x_first < param_2) {
        call_std_func_vec3(param_5, param_1, base_ptr);
        destroy_std_func_inline(param_4);
        destroy_std_func_inline(param_5);
        return param_1;
    }

    let last_ptr = base_ptr
        .wrapping_sub(0x10)
        .wrapping_add((count as usize).wrapping_mul(0x10));
    let x_last = call_std_func_f32(param_4, last_ptr);
    if !(param_2 < x_last) {
        call_std_func_vec3(param_5, param_1, last_ptr);
        exe::call_va!(0x1_401e_1560, (param_4));
        exe::call_va!(0x1_401e_1560, (param_5));
        return param_1;
    }

    let mut idx: usize = 0;
    if count > 1 {
        let mut step: u64 = 1;
        let mut cur = base_ptr.wrapping_add(0x10);
        loop {
            let x_cur = call_std_func_f32(param_4, cur);
            if param_2 <= x_cur {
                break;
            }
            idx += 1;
            cur = cur.wrapping_add(0x10);
            step += 1;
            if step >= count {
                break;
            }
        }
    }

    let seg_ptr = base_ptr.wrapping_add(idx * 0x10);
    let x0_a = call_std_func_f32(param_4, seg_ptr);
    let x1 = call_std_func_f32(param_4, seg_ptr.wrapping_add(0x10));
    let x0_b = call_std_func_f32(param_4, seg_ptr);
    let dx = x1 - x0_b;
    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let inv_dx = if dx.is_nan() || dx != 0.0f32 {
        c_one / dx
    } else {
        0.0f32
    };
    let t = (param_2 - x0_a) * inv_dx;

    let mut v1 = [0u8; 16];
    let mut v0 = [0u8; 16];
    let v1_ptr = v1.as_mut_ptr() as usize;
    let v0_ptr = v0.as_mut_ptr() as usize;
    call_std_func_vec3(param_5, v1_ptr, seg_ptr.wrapping_add(0x10));
    call_std_func_vec3(param_5, v0_ptr, seg_ptr);

    wr::<u64>(param_1, rd::<u64>(v1_ptr));
    wr::<u32>(param_1 + 0x08, rd::<u32>(v1_ptr + 0x08));
    for k in 0..3usize {
        let c1 = rd::<f32>(param_1 + k * 4);
        let c0 = rd::<f32>(v0_ptr + k * 4);
        wr::<f32>(param_1 + k * 4, (c1 - c0) * t + c0);
    }

    exe::call_va!(0x1_401e_1560, (param_4));
    exe::call_va!(0x1_401e_1560, (param_5));
    param_1
}

/// `unknown_141e1eac0`
/// Original: 0x141e1eac0 (280 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e1eac0(param_1: usize) {
    let count = rd::<u64>(param_1 + 0x18);
    if count == 0 {
        return;
    }
    let mut r13: usize = 0;
    let mut r12: usize = 0;
    for ebp in 0..count {
        let p20 = rd::<usize>(param_1 + 0x20);
        let p28 = rd::<usize>(param_1 + 0x28);
        let p30 = rd::<usize>(param_1 + 0x30);
        let r15 = p20.wrapping_add(r13);
        if rd::<u8>(p28.wrapping_add(r12 + 0x0f)) != 0
            && rd::<u8>(p28.wrapping_add(r12 + 0x0e)) != 0
            && rd::<u32>(p30.wrapping_add((ebp as usize) * 4)) != 0
        {
            let mut rbx = rd::<usize>(r15);
            let len = rd::<u32>(r15 + 0x08) as usize;
            let rsi = rbx.wrapping_add(len * 8);
            while rbx != rsi {
                let rdi = rd::<u64>(rbx);
                let edx = rd::<u32>(r15 + 0x68) as u64;
                let ecx = rd::<u32>(r15 + 0x6c) as u64;
                if (edx << 3) == (ecx << 3) {
                    exe::call_va!(0x1_40f7_8430, (r15 + 0x60, edx.wrapping_add(1)));
                }
                let idx = rd::<u32>(r15 + 0x68) as usize;
                let dst = rd::<usize>(r15 + 0x60);
                wr::<u64>(dst.wrapping_add(idx * 8), rdi);
                wr::<u32>(r15 + 0x68, (idx as u32).wrapping_add(1));
                rbx = rbx.wrapping_add(8);
            }
            wr::<u32>(r15 + 0x08, 0);
        }
        r13 = r13.wrapping_add(0x90);
        r12 = r12.wrapping_add(0x18);
    }
}

/// `unknown_141e1f040`
/// Original: 0x141e1f040 (293 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e1f040(param_1: usize) {
    let count = rd::<u64>(param_1 + 0x18);
    if count == 0 {
        return;
    }
    let mut off: usize = 0;
    for rbx in 0..count {
        let p30 = rd::<usize>(param_1 + 0x30);
        let p20 = rd::<usize>(param_1 + 0x20);
        let p28 = rd::<usize>(param_1 + 0x28);
        let a_lo = rd::<u128>(p28.wrapping_add(off));
        let a_hi = rd::<u128>(p28.wrapping_add(off + 0x10));
        let b_lo = rd::<u128>(p30.wrapping_add(off));
        let b_hi = rd::<u128>(p30.wrapping_add(off + 0x10));

        let eq_lo = vcmpps(b_lo, a_lo, 0);
        let eq_hi = vcmpps(a_hi, b_hi, 0);
        let mask = vmovmskps(eq_lo & eq_hi) as u32;
        if mask != 0x0f {
            let mut arg_buf: [u32; 8] = [
                b_lo as u32,
                (b_lo >> 32) as u32,
                (b_lo >> 64) as u32,
                (b_lo >> 96) as u32,
                b_hi as u32,
                (b_hi >> 32) as u32,
                (b_hi >> 64) as u32,
                0,
            ];
            let elem = rd::<usize>(p20.wrapping_add((rbx as usize) * 8));
            let obj = rd::<usize>(elem + 0xa8);
            let vtbl = rd::<usize>(obj);
            exe::call_va!(
                rd::<usize>(vtbl + 0xa8),
                (obj, arg_buf.as_mut_ptr() as usize, 1u8)
            );
            let c_lo = rd::<u128>(p30.wrapping_add(off));
            let c_hi = rd::<u128>(p30.wrapping_add(off + 0x10));
            wr::<u128>(p28.wrapping_add(off), c_lo);
            wr::<u128>(p28.wrapping_add(off + 0x10), c_hi);
        }
        off = off.wrapping_add(0x20);
    }
}

/// `unknown_141e7c940`
/// Original: 0x141e7c940 (210 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e7c940(
    param_1: usize,
    param_2: u32,
    param_3: usize,
    param_4: usize,
) {
    let mut r12d = param_2;
    let mut rbx = rd::<usize>(param_1 + 0x08);
    let r10 = rd::<usize>(param_1);
    let rdi_end = rbx.wrapping_add(r10.wrapping_mul(8));
    while rbx != rdi_end {
        let rsi = rd::<u64>(rbx);
        let r15d = (rsi >> 32) as u32;
        let mut esi = rsi as u32;
        while esi != r15d {
            let rbp = param_3.wrapping_add((esi as usize) * 0x20);
            let r14 = param_4.wrapping_add((r12d as usize) * 0x20);
            r12d = r12d.wrapping_add(1);
            wr::<u64>(r14, 0);
            wr::<u64>(r14 + 0x08, 0);
            exe::call_va!(0x1_4022_57b0, (r14, rbp));
            wr::<u64>(r14 + 0x10, rd::<u64>(rbp + 0x10));
            wr::<u32>(r14 + 0x18, rd::<u32>(rbp + 0x18));
            wr::<u32>(r14 + 0x1c, rd::<u32>(rbp + 0x1c));
            esi = esi.wrapping_add(1);
        }
        rbx = rbx.wrapping_add(8);
    }
}

#[inline(always)]
unsafe fn free_tls_block_x3(ptr: usize) {
    let r10 = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
    let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
    let page_idx = ptr.wrapping_sub(r10) >> 16;
    let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
    let page_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
    if same_thread {
        if rd::<u8>(page_hdr + 0x0e) == 0 {
            wr::<u64>(ptr, rd::<u64>(page_hdr + 0x20));
            wr::<usize>(page_hdr + 0x20, ptr);
            let new_cnt = rd::<u32>(page_hdr + 0x18).wrapping_sub(1);
            wr::<u32>(page_hdr + 0x18, new_cnt);
            if new_cnt == 0 {
                exe::call_va!(0x1_4253_71a0, (page_hdr));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (r10, page_hdr, 1u8, ptr));
        }
    } else {
        exe::call_va!(0x1_4253_3200, (r10, page_hdr, 0u8, ptr));
    }
}

/// `unknown_141e8d380`
/// Original: 0x141e8d380 (1033 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e8d380(param_1: usize, param_2: usize, param_3: usize) {
    let rsi = rd::<usize>(param_1 + 0x30).wrapping_add(rd::<u32>(param_1 + 0x14) as usize);
    let rdi = rd::<u32>(param_1 + 0x24) as u64;

    let mut pkt = [0u64; 10];
    let pkt_ptr = pkt.as_mut_ptr() as usize;
    wr::<usize>(pkt_ptr, 0);
    exe::call_va!(0x1_4215_9140, (pkt_ptr + 0x08));
    wr::<usize>(pkt_ptr + 0x40, 0);

    let tls0 = rd::<usize>(exe::tls_pointer());
    let r10 = rd::<usize>(tls0 + 0x6348);
    let mut alloc_ptr: usize;

    if (rdi.wrapping_sub(1) & rdi) == 0 && rdi.wrapping_sub(0x80) <= 0x380 {
        let bin_idx = ((rdi.wrapping_add(7)) >> 3) as usize;
        let r8_bin = rd::<usize>(r10.wrapping_add(bin_idx * 8 + 0x08));
        let head = rd::<usize>(r8_bin + 0x10);
        if head == 0 {
            alloc_ptr = exe::call_va!(0x1_4253_7280, (r10, rdi as u32, 0u32, 0u32) -> usize);
        } else {
            wr::<u32>(r8_bin + 0x18, rd::<u32>(r8_bin + 0x18).wrapping_add(1));
            wr::<usize>(r8_bin + 0x10, rd::<usize>(head));
            alloc_ptr = head;
        }
    } else {
        alloc_ptr = 0;
        if rdi.wrapping_sub(0x80) <= 0x380 {
            let bin_idx = ((rdi.wrapping_add(7)) >> 3) as usize;
            let rcx_bin = rd::<usize>(r10.wrapping_add(bin_idx * 8 + 0x08));
            let head = rd::<usize>(rcx_bin + 0x10);
            if head != 0 && (head & 0x7f) == 0 {
                wr::<u32>(rcx_bin + 0x18, rd::<u32>(rcx_bin + 0x18).wrapping_add(1));
                wr::<usize>(rcx_bin + 0x10, rd::<usize>(head));
                alloc_ptr = head;
            }
        }
        if alloc_ptr == 0 {
            alloc_ptr = exe::call_va!(0x1_4253_5070, (r10, rdi, 0x80u32) -> usize);
        }
    }

    if alloc_ptr == 0 {
        let hproc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (hproc, 0xdead_d00du32));
    }

    let old_28 = rd::<usize>(pkt_ptr + 0x40);
    wr::<usize>(pkt_ptr + 0x40, alloc_ptr);
    if old_28 != 0 {
        free_tls_block_x3(old_28);
    }

    let cur_28 = rd::<usize>(pkt_ptr + 0x40);
    let _ = exe::call_va!(0x1_4183_acbc, (cur_28, rsi, rdi) -> usize);

    let new_68 = exe::call_va!(0x1_40ff_b860, () -> usize);
    let old_68 = rd::<usize>(pkt_ptr);
    wr::<usize>(pkt_ptr, new_68);
    if old_68 != 0 {
        exe::call_va!(0x1_4215_4920, (old_68));
        free_tls_block_x3(old_68);
    }

    let cur_68 = rd::<usize>(pkt_ptr);
    exe::call_va!(0x1_4215_36c0, (cur_68, rd::<usize>(pkt_ptr + 0x40), rdi));

    let cur_68_b = rd::<usize>(pkt_ptr);
    let mut pair_19: [u64; 2] = [
        rd::<u32>(cur_68_b + 0x130) as u64,
        rd::<u64>(cur_68_b + 0x128),
    ];
    exe::call_va!(
        0x1_4215_3950,
        (cur_68_b, pair_19.as_mut_ptr() as usize)
    );

    wr::<u32>(pkt_ptr + 0x48, rd::<u32>(param_1 + 0x08));
    let mut key_29 = [0u64; 2];
    let kp = key_29.as_mut_ptr() as usize;
    wr::<u64>(kp, rd::<u64>(param_2));
    wr::<u32>(kp + 0x08, rd::<u32>(param_2 + 0x08));
    exe::call_va!(0x1_40ff_c030, (rd::<usize>(param_3), kp, pkt_ptr));

    let end_28 = rd::<usize>(pkt_ptr + 0x40);
    if end_28 != 0 {
        free_tls_block_x3(end_28);
    }

    wr::<usize>(pkt_ptr + 0x08, exe::ptr::<u8>(0x1_430b_85d8) as usize);
    exe::call_va!(
        0x1_4183_4ee8,
        (
            pkt_ptr + 0x18,
            0x10u32,
            2u32,
            exe::ptr::<u8>(0x1_401d_e400) as usize
        )
    );
    wr::<usize>(pkt_ptr + 0x08, exe::ptr::<u8>(0x1_430b_6bd0) as usize);

    let end_68 = rd::<usize>(pkt_ptr);
    if end_68 != 0 {
        exe::call_va!(0x1_4215_4920, (end_68));
        free_tls_block_x3(end_68);
    }
}

#[inline(always)]
unsafe fn compose_transform_x3(local_ptr: usize, parent_ptr: usize, c_7ab0: u128) -> [u128; 2] {
    let ymm4_lo = rd::<u128>(local_ptr);
    let xmm8 = rd::<u128>(local_ptr + 0x10);
    let xmm10 = rd::<u128>(parent_ptr);
    let ymm10_hi = rd::<u128>(parent_ptr + 0x10);

    let xmm0_a = vpermilps(xmm8, 0xc9);
    let xmm1_a = vpermilps(xmm8, 0xd2);
    let xmm9 = vshufps(xmm10, xmm10, 0xc9);
    let xmm2_a = vmulps(xmm1_a, xmm9);
    let xmm7 = vshufps(xmm10, xmm10, 0xd2);
    let xmm3_a = vmulps(xmm0_a, xmm7);
    let xmm6 = vmulps(vsubps(xmm2_a, xmm3_a), c_7ab0);

    let xmm3_b = vpermilps(ymm4_lo, 0x11);
    let xmm1_b = vmulps(xmm10, xmm3_b);
    let xmm2_b = vpermilps(ymm4_lo, 0xbb);
    let xmm5 = vpermilps(xmm10, 0x1b);
    let xmm0_b = vmulps(xmm2_b, xmm5);
    let xmm4_q = vhsubps(xmm1_b, xmm0_b);
    let xmm0_c = vmulps(xmm3_b, xmm5);
    let xmm2_c = vmulps(xmm10, xmm2_b);
    let xmm3_q = vhaddps(xmm2_c, xmm0_c);
    let xmm0_d = vshufps(xmm4_q, xmm3_q, 0xb1);
    let xmm1_d = vblendps(xmm3_q, xmm4_q, 0x0c);
    let xmm1_e = vaddsubps(xmm1_d, xmm0_d);
    let out_lo = vpermilps(xmm1_e, 0x9c);

    let xmm0_f = vpermilps(xmm6, 0xc9);
    let xmm3_f = vmulps(xmm0_f, xmm7);
    let xmm1_f = vpermilps(xmm6, 0xd2);
    let xmm2_f = vmulps(xmm1_f, xmm9);
    let xmm0_w = vshufps(xmm10, xmm10, 0xff);
    let xmm1_w = vmulps(xmm0_w, xmm6);
    let xmm4_t = vsubps(xmm2_f, xmm3_f);
    let xmm2_t = vaddps(xmm1_w, xmm8);
    let xmm3_t = vaddps(xmm4_t, xmm2_t);
    let out_hi = vaddps(xmm3_t, ymm10_hi);

    [out_lo, out_hi]
}

/// `unknown_141e99010`
/// Original: 0x141e99010 (1076 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e99010(param_1: usize, param_2: usize, param_3: usize) {
    let r12 = rd::<u64>(param_1 + 0x18);
    if r12 == 0 {
        return;
    }
    let c_7ab0 = rd::<u128>(exe::ptr::<u128>(0x1_4342_7ab0) as usize);
    let mut r14: usize = 0;

    for rdi in 0..r12 {
        let p20 = rd::<usize>(param_1 + 0x20);
        let p28 = rd::<usize>(param_1 + 0x28);
        let p30 = rd::<usize>(param_1 + 0x30);
        let p38 = rd::<usize>(param_1 + 0x38);
        let p40 = rd::<usize>(param_1 + 0x40);
        let p48 = rd::<u64>(param_1 + 0x48);
        let p60 = rd::<u64>(param_1 + 0x60);
        let p68 = rd::<u64>(param_1 + 0x68);
        let p70 = rd::<usize>(param_1 + 0x70);
        let r13 = rd::<u64>(param_1 + 0x08);
        let idx = rdi as usize;

        if (rd::<u8>(p38.wrapping_add(idx)) != 0
            || rd::<u8>(p40.wrapping_add(idx)) != 0
            || rd::<u8>(p70.wrapping_add(idx)) != 0)
            && rd::<u32>(p20.wrapping_add(idx * 4)) != 1
            && rd::<u8>(p28.wrapping_add(idx)) == 0
        {
            let r15 = rd::<usize>(p30.wrapping_add(r14));
            let rcx_node = rd::<usize>(r15 + 0x20);
            if rd::<u16>(rcx_node + 0x90) != 0 {
                let mut arg_50 =
                    compose_transform_x3(rcx_node + 0x20, rd::<usize>(rcx_node + 0x58), c_7ab0);
                let mut arg_70: [u64; 3] = [
                    rd::<u64>(param_3),
                    rd::<u64>(param_3 + 0x08),
                    rd::<u64>(param_3 + 0x10),
                ];
                let mut arg_90: [u64; 5] = [p48, p60, p68, r13, rdi];
                exe::call_va!(
                    0x1_41e2_2a40,
                    (
                        arg_90.as_mut_ptr() as usize,
                        arg_70.as_mut_ptr() as usize,
                        arg_50.as_mut_ptr() as usize
                    )
                );

                let rcx_node2 = rd::<usize>(r15 + 0x20);
                let mut arg_50_2 =
                    compose_transform_x3(rcx_node2 + 0x20, rd::<usize>(rcx_node2 + 0x50), c_7ab0);
                let p50 = rd::<u64>(param_1 + 0x50);
                let p58 = rd::<u64>(param_1 + 0x58);
                let mut arg_70_2: [u64; 3] = [
                    rd::<u64>(param_2),
                    rd::<u64>(param_2 + 0x08),
                    rd::<u64>(param_2 + 0x10),
                ];
                let mut arg_90_2: [u64; 5] = [p48, p50, p58, r13, rdi];
                exe::call_va!(
                    0x1_41e2_34c0,
                    (
                        arg_90_2.as_mut_ptr() as usize,
                        arg_70_2.as_mut_ptr() as usize,
                        arg_50_2.as_mut_ptr() as usize
                    )
                );
            }
        }
        r14 = r14.wrapping_add(0x20);
    }
}

/// `unknown_141f4d6d0`
/// Original: 0x141f4d6d0 (580 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f4d6d0(
    param_1: u64,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) {
    let mut stream_ctx = [0u64; 0x348 / 8];
    let base = stream_ctx.as_mut_ptr() as usize;

    let _ = exe::call_va!(0x1_4183_acc2, (base, 0u32, 0x310u64) -> usize);
    wr::<usize>(base, base + 0x10);
    wr::<u32>(base + 0x0c, 0x10);
    wr::<u32>(base + 0x318, 2);
    wr::<u64>(base + 0x320, 0);
    wr::<u32>(base + 0x328, 0xffff_ffff);
    wr::<u64>(base + 0x330, 0);
    wr::<u64>(base + 0x310, 0);
    wr::<[u64; 6]>(base + 0x10, [0; 6]);
    wr::<u32>(base + 0x08, 1);

    let rbx_slot = (base + 0x10).wrapping_add(0x30);
    wr::<u64>(rbx_slot - 0x30, 0);
    wr::<usize>(rbx_slot - 0x28, exe::ptr::<u8>(0x1_4309_f824) as usize);
    wr::<u64>(rbx_slot - 0x20, 0);
    wr::<u64>(rbx_slot - 0x18, 0);
    wr::<u32>(rbx_slot - 0x08, 0xffff_ffff);
    wr::<u64>(rbx_slot - 0x10, 0);

    let p320 = rd::<usize>(base + 0x320);
    if p320 != 0 {
        let inner = rd::<usize>(p320 + 0x08);
        if inner != 0 {
            let vtbl = rd::<usize>(inner);
            let pos = exe::call_va!(rd::<usize>(vtbl + 0x10), (inner) -> u64);
            wr::<u64>(rbx_slot - 0x18, pos);
        }
    }

    wr::<u64>(base + 0x338, param_1);
    wr::<u64>(base + 0x340, param_2);

    let mut cur3 = rd::<usize>(param_3 + 0x58);
    let end3 = cur3.wrapping_add((rd::<usize>(param_3 + 0x40)).wrapping_mul(16));
    while cur3 != end3 {
        let k = rd::<u64>(cur3);
        if k != rd::<u64>(param_3 + 0x48)
            && (rd::<u64>(param_3 + 0x30) == 0 || k != rd::<u64>(param_3 + 0x28))
        {
            break;
        }
        cur3 = cur3.wrapping_add(16);
    }
    while cur3 != end3 {
        let obj = rd::<usize>(cur3 + 0x08);
        let vtbl = rd::<usize>(obj);
        exe::call_va!(rd::<usize>(vtbl + 0x88), (obj, base, 0u32));
        loop {
            cur3 = cur3.wrapping_add(16);
            if cur3 == end3 {
                break;
            }
            let k = rd::<u64>(cur3);
            if k != rd::<u64>(param_3 + 0x48)
                && (rd::<u64>(param_3 + 0x30) == 0 || k != rd::<u64>(param_3 + 0x28))
            {
                break;
            }
        }
    }

    let mut cur4 = rd::<usize>(param_4 + 0x58);
    let end4_init = cur4.wrapping_add((rd::<usize>(param_4 + 0x40)).wrapping_mul(16));
    while cur4 != end4_init {
        let k = rd::<u64>(cur4);
        if k != rd::<u64>(param_4 + 0x48)
            && (rd::<u64>(param_4 + 0x30) == 0 || k != rd::<u64>(param_4 + 0x28))
        {
            break;
        }
        cur4 = cur4.wrapping_add(16);
    }
    let end4 = rd::<usize>(param_4 + 0x58)
        .wrapping_add((rd::<usize>(param_4 + 0x40)).wrapping_mul(16));
    while cur4 != end4 {
        let obj = rd::<usize>(cur4 + 0x08);
        let vtbl = rd::<usize>(obj);
        exe::call_va!(rd::<usize>(vtbl + 0x130), (obj, base, 0u32));
        loop {
            cur4 = cur4.wrapping_add(16);
            if cur4 == end4_init {
                break;
            }
            let k = rd::<u64>(cur4);
            if k != rd::<u64>(param_4 + 0x48)
                && (rd::<u64>(param_4 + 0x30) == 0 || k != rd::<u64>(param_4 + 0x28))
            {
                break;
            }
        }
    }

    exe::call_va!(0x1_401e_e280, (base));
}

#[inline(always)]
fn u64_to_f32_x86(rcx: u64) -> f32 {
    if (rcx as i64) >= 0 {
        (rcx as i64) as f32
    } else {
        let half = (rcx >> 1) | (rcx & 1);
        let f = (half as i64) as f32;
        f + f
    }
}

#[inline(always)]
fn f32_to_u64_x86(mut v: f32, two63: f32) -> u64 {
    let mut off: u64 = 0;
    if v >= two63 {
        v -= two63;
        if v < two63 {
            off = 0x8000_0000_0000_0000;
        }
    }
    crate::simd_ops::cvt_f32_i64(v).wrapping_add(off)
}

/// `unknown_141f79330`
/// Original: 0x141f79330 (810 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f79330(param_1: usize) {
    let g_300_addr = exe::ptr::<usize>(0x1_43a0_8300) as usize;

    let mut cur1 = rd::<usize>(param_1 + 0xb8);
    let end1 = cur1.wrapping_add((rd::<usize>(param_1 + 0xa0)).wrapping_mul(16));
    while cur1 != end1 {
        let k = rd::<u64>(cur1);
        if k != rd::<u64>(param_1 + 0xa8)
            && (rd::<u64>(param_1 + 0x90) == 0 || k != rd::<u64>(param_1 + 0x88))
        {
            break;
        }
        cur1 = cur1.wrapping_add(16);
    }
    while cur1 != end1 {
        exe::call_va!(0x1_4248_b0c0, (rd::<usize>(g_300_addr), cur1));
        loop {
            cur1 = cur1.wrapping_add(16);
            if cur1 == end1 {
                break;
            }
            let k = rd::<u64>(cur1);
            if k != rd::<u64>(param_1 + 0xa8)
                && (rd::<u64>(param_1 + 0x90) == 0 || k != rd::<u64>(param_1 + 0x88))
            {
                break;
            }
        }
    }

    let mut cur2 = rd::<usize>(param_1 + 0x58);
    let end2_init = cur2.wrapping_add((rd::<usize>(param_1 + 0x40)).wrapping_mul(16));
    while cur2 != end2_init {
        let k = rd::<u64>(cur2);
        if k != rd::<u64>(param_1 + 0x48)
            && (rd::<u64>(param_1 + 0x30) == 0 || k != rd::<u64>(param_1 + 0x28))
        {
            break;
        }
        cur2 = cur2.wrapping_add(16);
    }
    let end2 = rd::<usize>(param_1 + 0x58)
        .wrapping_add((rd::<usize>(param_1 + 0x40)).wrapping_mul(16));
    while cur2 != end2 {
        exe::call_va!(0x1_4248_b0c0, (rd::<usize>(g_300_addr), cur2));
        loop {
            cur2 = cur2.wrapping_add(16);
            if cur2 == end2_init {
                break;
            }
            let k = rd::<u64>(cur2);
            if k != rd::<u64>(param_1 + 0x48)
                && (rd::<u64>(param_1 + 0x30) == 0 || k != rd::<u64>(param_1 + 0x28))
            {
                break;
            }
        }
    }

    let rsi = rd::<usize>(g_300_addr);
    let rbx_lock = rsi + 0x60;
    exe::call_va!(0x1_4137_8fb0, (rbx_lock));

    let c_half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let c_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let two63 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fc8) as usize);

    let xmm1 = rd::<f32>(rsi + 0x10);
    let prod0 = xmm1 * c_half;
    let xmm2 = if prod0 < c_one { prod0 } else { c_one };
    wr::<f32>(rsi + 0x14, xmm2);
    wr::<f32>(rsi + 0x10, xmm1);
    let cap_f = u64_to_f32_x86(rd::<u64>(rsi + 0x40));
    wr::<u64>(rsi, f32_to_u64_x86(cap_f * xmm1, two63));
    wr::<u64>(rsi + 0x08, f32_to_u64_x86(cap_f * xmm2, two63));
    wr::<u8>(rsi + 0x18, 0);

    exe::call_va!(0x1_413c_fbe0, (rsi));

    let xmm2_b = rd::<f32>(rsi + 0x10);
    let prod1 = xmm2_b * c_half;
    let xmm3 = if prod1 < 0.0f32 { prod1 } else { 0.0f32 };
    wr::<f32>(rsi + 0x14, xmm3);
    wr::<f32>(rsi + 0x10, xmm2_b);
    let cap_f2 = u64_to_f32_x86(rd::<u64>(rsi + 0x40));
    wr::<u64>(rsi, f32_to_u64_x86(cap_f2 * xmm2_b, two63));
    wr::<u64>(rsi + 0x08, f32_to_u64_x86(cap_f2 * xmm3, two63));
    wr::<u8>(rsi + 0x18, 0);

    if (rd::<u64>(rbx_lock) & 0xffff_ffff_0000_0000) == 0x0000_0001_0000_0000 {
        wr::<u64>(rbx_lock + 0x08, 0);
    }
    let _ = crate::simd_ops::atomic_xadd_u64(rbx_lock as u64, 0xffff_ffff_0000_0000);

    exe::call_va!(0x1_410f_19a0, (param_1 + 0x60));
    exe::call_va!(0x1_410f_19a0, (param_1));

    let c0 = rd::<usize>(param_1 + 0xc0);
    if c0 != 0 {
        exe::call_va!(0x1_4244_2de0, (c0));
    }
    let c8 = rd::<usize>(param_1 + 0xc8);
    if c8 != 0 {
        exe::call_va!(0x1_4244_2de0, (c8));
    }
    wr::<u64>(param_1 + 0xc0, 0);
    wr::<u64>(param_1 + 0xc8, 0);
}

/// `unknown_141f7a100`
/// Original: 0x141f7a100 (290 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f7a100(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
) {
    let vtbl = rd::<usize>(param_3);
    let r11d = exe::call_va!(rd::<usize>(vtbl + 0x140), (param_3) -> u32);

    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const FNV_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h = FNV_BASIS;
    for &b in &r11d.to_le_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }

    let bucket_idx = (rd::<u64>(param_1 + 0x30) & h) as usize;
    let buckets = rd::<usize>(param_1 + 0x18);
    let bucket_ptr = buckets.wrapping_add(bucket_idx * 16);
    let mut rbx = rd::<usize>(bucket_ptr);
    if rbx == rd::<usize>(param_1 + 0x08) {
        return;
    }
    let bucket_last = rd::<usize>(bucket_ptr + 0x08);
    while rd::<u32>(rbx + 0x10) != r11d {
        if rbx == bucket_last {
            return;
        }
        rbx = rd::<usize>(rbx);
    }
    let mut rdi = rd::<usize>(rbx);
    if rbx != bucket_last {
        loop {
            let rcx_node = rdi;
            if rd::<u32>(rdi + 0x10) != r11d {
                break;
            }
            rdi = rd::<usize>(rdi);
            if rcx_node == bucket_last {
                break;
            }
        }
    }
    while rbx != rdi {
        let mut arg_20 = [0u64; 2];
        let ap = arg_20.as_mut_ptr() as usize;
        wr::<u64>(ap, rd::<u64>(param_2));
        wr::<u32>(ap + 0x08, rd::<u32>(param_2 + 0x08));
        exe::call_va!(rd::<usize>(rbx + 0x18), (param_3, ap, param_4));
        rbx = rd::<usize>(rbx);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141600880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_v68(_this: usize, arg: u32) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0x6800 + arg as u64);
    }
    unsafe extern "win64" fn mock_v70(_this: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0x7000);
    }
    unsafe extern "win64" fn mock_v78(_this: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0x7800);
    }
    unsafe extern "win64" fn mock_fd60(_rdi: usize) -> i32 {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0xfd60);
        1
    }
    unsafe extern "win64" fn mock_7890(_rdi: usize, rdx: usize) {
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(0x7890)
            .wrapping_add(rdx.wrapping_sub(_rdi) as u64);
    }
    unsafe extern "win64" fn mock_5950(_p1: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0x5950);
    }
    unsafe extern "win64" fn mock_5e10(_rsi: usize) -> u8 {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0x5e10);
        1
    }
    unsafe extern "win64" fn mock_6870(_rsi: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0x6870);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4158_fd60, mock_fd60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_415d_7890, mock_7890 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4164_5950, mock_5950 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4159_5e10, mock_5e10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4159_6870, mock_6870 as *const ())?;

    let mut vtbl = [0usize; 16];
    vtbl[0x68 / 8] = mock_v68 as *const () as usize;
    vtbl[0x70 / 8] = mock_v70 as *const () as usize;
    vtbl[0x78 / 8] = mock_v78 as *const () as usize;

    let run = |f: F, flags_210: u8, flags_211: u8, db_val: f32| -> (u32, u32, u32, u32, u32, u64) {
        CALL_LOG = 0;

        let mut rsi_obj = [0u8; 0x220];
        let rsi_p = rsi_obj.as_mut_ptr() as usize;
        wr::<usize>(rsi_p + 0x18, vtbl.as_ptr() as usize);
        wr::<u32>(rsi_p + 0x1b8, 0x9999);
        wr::<u8>(rsi_p + 0x210, flags_210);
        wr::<u8>(rsi_p + 0x211, flags_211);

        let mut rdi2_obj = [0u8; 0x220];
        let rdi2_p = rdi2_obj.as_mut_ptr() as usize;
        wr::<usize>(rdi2_p + 0x18, vtbl.as_ptr() as usize);
        wr::<u8>(rdi2_p + 0x210, flags_210 ^ 0x11);

        let mut wrap1 = [0usize; 4];
        wrap1[3] = rsi_p;
        let mut wrap2 = [0usize; 4];
        wrap2[3] = rdi2_p;

        let mut p8_obj = [0u8; 0x1b0];
        let p8_p = p8_obj.as_mut_ptr() as usize;
        wr::<f32>(p8_p + 0x88, db_val);
        wr::<f32>(p8_p + 0x100, 0.75);
        wr::<u8>(p8_p + 0x160, 8);

        let mut p1_obj = [0u8; 0x130];
        let p1_p = p1_obj.as_mut_ptr() as usize;
        wr::<usize>(p1_p + 0x08, p8_p);
        wr::<u32>(p1_p + 0x10, 0x1111);
        wr::<u32>(p1_p + 0x48, 0x2222);
        wr::<usize>(p1_p + 0xe8, wrap1.as_ptr() as usize);
        wr::<usize>(p1_p + 0xf0, wrap2.as_ptr() as usize);
        wr::<u32>(p1_p + 0x118, 7);
        wr::<u32>(p1_p + 0x11c, 0);

        f(p1_p);
        (
            rd::<u32>(p1_p + 0x10),
            rd::<u32>(p1_p + 0x48),
            rd::<u32>(p1_p + 0x118),
            rd::<u32>(p1_p + 0x11c),
            rd::<u32>(rsi_p + 0x1b8),
            CALL_LOG,
        )
    };

    for (f210, f211, db) in [(0u8, 0u8, -6.0f32), (0x11, 0, -900.0), (1, 1, 0.0)] {
        let o = run(f_o, f210, f211, db);
        let n = run(f_n, f210, f211, db);
        crate::diff_eq!(o, n, "141600880 f210={} f211={} db={}", f210, f211, db);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141645fc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;
    static mut ENTRY_5CF0: [u64; 2] = [0; 2];

    unsafe extern "win64" fn mock_v60(_this: usize) -> f32 {
        let v = 1.5f32;
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_27c0(_rcx: usize, rdx_flag: usize) -> f32 {
        wr::<u8>(rdx_flag, 1);
        let v = 0.8f32;
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_5340(rcx: usize) -> usize {
        rcx
    }
    unsafe extern "win64" fn mock_38d0(_rcx: usize, rdx_key: usize) -> f32 {
        let k = rd::<u64>(rdx_key);
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(k);
        let v = 0.5f32;
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_ee80(rsi: usize) -> u64 {
        rd::<u64>(rsi + 0x170)
    }
    unsafe extern "win64" fn mock_41b0(obj: u64) -> u32 {
        (obj as u32).wrapping_add(10)
    }
    unsafe extern "win64" fn mock_4020(obj: u64) -> u64 {
        obj.wrapping_add(100)
    }
    unsafe extern "win64" fn mock_5cf0(_map: usize, _rbx: usize, r8_flag: usize) -> usize {
        wr::<u8>(r8_flag, 1);
        core::ptr::addr_of_mut!(ENTRY_5CF0) as usize
    }
    unsafe extern "win64" fn mock_5240(_rdi: usize, _r15: usize, rsi: usize) {
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(rsi + 0x198) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_415c_27c0, mock_27c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4160_5340, mock_5340 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4160_38d0, mock_38d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4162_ee80, mock_ee80 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4163_41b0, mock_41b0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4163_4020, mock_4020 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4164_5cf0, mock_5cf0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_415d_5240, mock_5240 as *const ())?;

    let mut vtbl_p1 = [0usize; 14];
    vtbl_p1[0x60 / 8] = mock_v60 as *const () as usize;

    let run = |f: F, mode_ebp: u32, dl: u8| -> ([u32; 8], u32, u64) {
        CALL_LOG = 0;
        wr::<[u64; 2]>(
            core::ptr::addr_of_mut!(ENTRY_5CF0) as usize,
            [0, (-50.0f32).to_bits() as u64],
        );

        let mut ctx_5340 = [0u8; 0x100];
        wr::<u8>(ctx_5340.as_mut_ptr() as usize + 0xe0, 1);

        let mut arr_198 = [0u8; 0x88];
        let ap = arr_198.as_mut_ptr() as usize;
        wr::<f32>(ap + 0x3c, 0.9);
        wr::<f32>(ap + 0x40, 0.85);
        wr::<f32>(ap + 0x44, 0.75);
        wr::<u64>(ap + 0x70, 77);

        let mut rdi_obj = [0u8; 0x220];
        let rdi_p = rdi_obj.as_mut_ptr() as usize;
        wr::<usize>(rdi_p + 0x08, ctx_5340.as_mut_ptr() as usize);
        wr::<usize>(rdi_p + 0x78, 1);
        wr::<f32>(rdi_p + 0xb4, -6.0);
        wr::<f32>(rdi_p + 0xb8, 0.25);
        wr::<f32>(rdi_p + 0xbc, 0.75);
        wr::<f32>(rdi_p + 0xc0, 10.0);
        wr::<u8>(rdi_p + 0x13f, 1);
        wr::<u8>(rdi_p + 0x160, 8);
        wr::<usize>(rdi_p + 0x198, ap);
        wr::<u32>(rdi_p + 0x1a0, 1);
        wr::<u8>(rdi_p + 0x1f8, 0x80);
        wr::<f32>(rdi_p + 0x200, -12.0);

        let mut sub_158 = [0u8; 0x20];
        wr::<f32>(sub_158.as_mut_ptr() as usize + 0x14, -1.0);

        let mut sub_160 = [0u8; 0x200];
        let s160_p = sub_160.as_mut_ptr() as usize;
        wr::<f32>(s160_p + 0x14, -2.0);
        wr::<f32>(s160_p + 0x1c8, -100.0);
        wr::<f32>(s160_p + 0x1d8, -3.0);
        wr::<f32>(s160_p + 0x1dc, 0.5);

        let mut node = [0u8; 0x1b0];
        let np = node.as_mut_ptr() as usize;
        wr::<u32>(np + 0x0c, 0x1111);
        wr::<f32>(np + 0x14, 0.5);
        wr::<usize>(np + 0x158, sub_158.as_mut_ptr() as usize);
        wr::<usize>(np + 0x160, s160_p);
        wr::<u64>(np + 0x170, 77);
        wr::<u32>(np + 0x194, mode_ebp);
        wr::<u32>(np + 0x198, 1);

        let mut arr_40 = [0u8; 0x20];
        let a40_p = arr_40.as_mut_ptr() as usize;
        wr::<u64>(a40_p, 177);
        wr::<u32>(a40_p + 0x08, 87);
        wr::<f32>(a40_p + 0x0c, 0.6);
        wr::<f32>(a40_p + 0x14, 0.3);
        wr::<f32>(a40_p + 0x18, 0.4);

        let mut p1 = [0u8; 0x80];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1_p, vtbl_p1.as_ptr() as usize);
        wr::<usize>(p1_p + 0x08, rdi_p);
        wr::<f32>(p1_p + 0x10, 0.9);
        wr::<usize>(p1_p + 0x38, np);
        wr::<usize>(p1_p + 0x40, a40_p);
        wr::<u32>(p1_p + 0x48, 1);

        f(p1_p, dl);
        let out_words = [
            rd::<u32>(np + 0x08),
            rd::<u32>(np + 0x10),
            rd::<u32>(np + 0x14),
            rd::<u32>(np + 0x178),
            rd::<u32>(np + 0x17c),
            rd::<u32>(np + 0x188),
            rd::<u32>(np + 0x18c),
            rd::<u32>(np + 0x198),
        ];
        (
            out_words,
            rd::<u32>(core::ptr::addr_of!(ENTRY_5CF0) as usize + 0x08),
            CALL_LOG,
        )
    };

    for (ebp, dl) in [(0u32, 1u8), (1, 1), (2, 0)] {
        let o = run(f_o, ebp, dl);
        let n = run(f_n, ebp, dl);
        crate::diff_eq!(o, n, "141645fc0 ebp={} dl={}", ebp, dl);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141acd4d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;
    static mut LAST_TRANSFORM: [u32; 8] = [0; 8];
    static mut FOUND2_OBJ: [u8; 0xd0] = [0; 0xd0];

    unsafe extern "win64" fn mock_9650(p6: usize, out_key: usize, r8: u64) {
        wr::<u64>(out_key, (p6 as u64).wrapping_add(r8));
    }
    unsafe extern "win64" fn mock_ab00(p2: usize, out_buf: usize, key: u64) {
        let base_r12 = rd::<u64>(p2 + 0x18);
        wr::<u64>(out_buf, base_r12);
        for i in 1..6usize {
            wr::<u64>(out_buf + i * 8, key.wrapping_add(i as u64));
        }
        wr::<u64>(out_buf + 48, 0);
    }
    unsafe extern "win64" fn mock_4390(p5: usize, _r12: usize, r8_p3: usize, r9: u32) {
        CALL_LOG = CALL_LOG
            .wrapping_add(p5 as u64)
            .wrapping_add(rd::<u64>(r8_p3))
            .wrapping_add(r9 as u64);
    }
    unsafe extern "win64" fn mock_cb20(_p5: usize, _str1: usize) -> usize {
        0x1000
    }
    unsafe extern "win64" fn mock_8bc0(_found1: usize, _str2: usize) -> usize {
        core::ptr::addr_of_mut!(FOUND2_OBJ) as usize
    }
    unsafe extern "win64" fn mock_6850(
        out_ptr: usize,
        _in_b0: usize,
        _r8: u64,
        _r9_p5: usize,
        flag: u32,
    ) -> usize {
        if flag == 0 {
            wr::<f32>(out_ptr, 1.0);
            wr::<f32>(out_ptr + 0x04, 2.0);
            wr::<f32>(out_ptr + 0x08, 3.0);
        } else {
            wr::<f32>(out_ptr, 4.0);
            wr::<f32>(out_ptr + 0x04, 6.0);
            wr::<f32>(out_ptr + 0x08, 9.0);
        }
        out_ptr
    }
    unsafe extern "win64" fn mock_34c0(_ctx_30: usize, _p4_copy: usize, r8_xform: usize) {
        wr::<[u32; 8]>(
            core::ptr::addr_of_mut!(LAST_TRANSFORM) as usize,
            rd::<[u32; 8]>(r8_xform),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_401f_9650, mock_9650 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4058_ab00, mock_ab00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41ad_4390, mock_4390 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41ad_cb20, mock_cb20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41ac_8bc0, mock_8bc0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41ad_6850, mock_6850 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41e2_34c0, mock_34c0 as *const ())?;

    let run = |f: F, early_a5: u8, mode_p1: u32| -> ([u32; 8], [u32; 8], u32, u64) {
        CALL_LOG = 0;
        wr::<[u32; 8]>(core::ptr::addr_of_mut!(LAST_TRANSFORM) as usize, [0; 8]);
        wr::<u64>(core::ptr::addr_of_mut!(FOUND2_OBJ) as usize + 0xc8, 0);

        let mut r12_buf = [0u8; 0x340];
        let r12_p = r12_buf.as_mut_ptr() as usize;
        wr::<u32>(r12_p + 0x48, (4u32 << 24) | 2);
        wr::<u32>(r12_p + 0x4c, 11);
        wr::<u64>(r12_p + 0x90, 0x1234);
        wr::<u8>(r12_p + 0xa5, early_a5);
        wr::<u32>(r12_p + 0x318, (4u32 << 24) | 2);
        wr::<u32>(r12_p + 0x31c, 22);

        let mut r14_buf = [0u32; 40];
        let mut rcx_obj = [0u8; 0x20];
        let rcx_p = rcx_obj.as_mut_ptr() as usize;
        wr::<u16>(rcx_p + 0x04, 5);
        wr::<u8>(rcx_p + 0x06, 0);
        wr::<usize>(rcx_p + 0x08, r14_buf.as_mut_ptr() as usize);

        let slot_arr: [usize; 2] = [rcx_p, 0];
        let mut flags_arr: [u32; 2] = [0, 0];

        let mut p1_inner = [0u8; 0x20];
        let p1i_p = p1_inner.as_mut_ptr() as usize;
        wr::<u32>(p1i_p, mode_p1);
        wr::<u64>(p1i_p + 0x08, 10);
        wr::<f32>(p1i_p + 0x10, 0.5);
        wr::<f32>(p1i_p + 0x14, 0.1);

        let mut p1 = [0u64; 8];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1_p, p1i_p);
        wr::<u64>(p1_p + 0x08, 0xaa01);
        wr::<u64>(p1_p + 0x10, 0xaa02);
        wr::<u64>(p1_p + 0x18, 0xaa03);
        wr::<usize>(p1_p + 0x20, slot_arr.as_ptr() as usize);
        wr::<usize>(p1_p + 0x28, flags_arr.as_mut_ptr() as usize);
        wr::<u64>(p1_p + 0x30, 0xaa04);
        wr::<usize>(p1_p + 0x38, 0);

        let id_table: [u64; 1] = [0];
        let u16_table: [u64; 1] = [5];
        let sorted_u16: [u16; 2] = [3, 5];
        let mut p2_0 = [0u8; 0x40];
        let p2_0_p = p2_0.as_mut_ptr() as usize;
        wr::<usize>(p2_0_p + 0x18, id_table.as_ptr() as usize);
        wr::<u32>(p2_0_p + 0x20, 1);
        wr::<usize>(p2_0_p + 0x30, u16_table.as_ptr() as usize);

        let mut p2 = [0u64; 4];
        let p2_p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2_p, p2_0_p);
        wr::<usize>(p2_p + 0x08, 2);
        wr::<usize>(p2_p + 0x10, sorted_u16.as_ptr() as usize);
        wr::<usize>(p2_p + 0x18, r12_p);

        let p3: [u64; 3] = [0x111, 0x222, 0x333];
        let p4: [u64; 3] = [0x444, 0x555, 0x666];

        let mut p5 = [0u8; 0x430];
        let p5_p = p5.as_mut_ptr() as usize;
        wr::<f32>(p5_p + 0x18, 2.5);
        wr::<f32>(p5_p + 0x88, 10.0);
        wr::<f32>(p5_p + 0x8c, 2.0);
        wr::<u8>(p5_p + 0x424, 1);

        let dt: f32 = 0.016;
        f(
            p1_p,
            p2_p,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5_p,
            0x900,
            (&dt as *const f32) as usize,
        );

        let tail_slice: [u32; 8] = rd::<[u32; 8]>(r14_buf.as_ptr() as usize + 3 * 0x20 + 0x10);
        let xform = rd::<[u32; 8]>(core::ptr::addr_of!(LAST_TRANSFORM) as usize);
        (xform, tail_slice, flags_arr[0], CALL_LOG)
    };

    for (early, mode_p1) in [(1u8, 1u32), (0, 1), (0, 0)] {
        let o = run(f_o, early, mode_p1);
        let n = run(f_n, early, mode_p1);
        crate::diff_eq!(o, n, "141acd4d0 early={} mode_p1={}", early, mode_p1);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141da1a60(orig: usize, new: usize) -> Result<u32, String> {
    static mut DTOR_LOG: u64 = 0;

    unsafe extern "win64" fn mock_get_x(_this: usize, elem: usize) -> f32 {
        let v = rd::<f32>(elem);
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_get_v(_this: usize, out_v: usize, elem: usize) {
        wr::<u64>(out_v, rd::<u64>(elem + 0x04));
        wr::<u32>(out_v + 0x08, rd::<u32>(elem + 0x0c));
    }
    unsafe extern "win64" fn mock_dtor_vfunc(this: usize, is_heap: u8) {
        let tag = rd::<u32>(this + 0x08) as u64;
        DTOR_LOG = DTOR_LOG
            .wrapping_mul(131)
            .wrapping_add((tag << 8) | (is_heap as u64));
    }
    unsafe extern "win64" fn mock_1560(func_obj: usize) {
        destroy_std_func_inline(func_obj);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_1560, mock_1560 as *const ())?;

    let mut vtbl_x = [0usize; 6];
    vtbl_x[2] = mock_get_x as *const () as usize;
    vtbl_x[4] = mock_dtor_vfunc as *const () as usize;

    let mut vtbl_v = [0usize; 6];
    vtbl_v[2] = mock_get_v as *const () as usize;
    vtbl_v[4] = mock_dtor_vfunc as *const () as usize;

    let pts: [[f32; 4]; 3] = [
        [1.0, 10.0, 20.0, 30.0],
        [3.0, 30.0, 60.0, 90.0],
        [5.0, 70.0, 100.0, 130.0],
    ];

    let invoke = |fn_addr: usize, x: f32, cnt: u64| -> ([u32; 3], u64, usize, usize) {
        DTOR_LOG = 0;
        let span: [u64; 2] = [cnt, pts.as_ptr() as usize as u64];
        let def_val: [f32; 3] = [7.0, 8.0, 9.0];
        let mut out_vec = [0u32; 3];
        let out_p = out_vec.as_mut_ptr() as usize;

        let mut fn4 = [0u64; 8];
        let f4p = fn4.as_mut_ptr() as usize;
        wr::<usize>(f4p, vtbl_x.as_ptr() as usize);
        wr::<u32>(f4p + 0x08, 1);
        wr::<usize>(f4p + 0x38, f4p);

        let mut fn5 = [0u64; 8];
        let f5p = fn5.as_mut_ptr() as usize;
        wr::<usize>(f5p, vtbl_v.as_ptr() as usize);
        wr::<u32>(f5p + 0x08, 2);
        wr::<usize>(f5p + 0x38, f5p);

        #[cfg(not(target_arch = "aarch64"))]
        let ret = {
            type F = unsafe extern "win64" fn(usize, f32, usize, usize, usize, usize) -> usize;
            let f: F = core::mem::transmute(fn_addr);
            f(
                out_p,
                x,
                span.as_ptr() as usize,
                f4p,
                f5p,
                def_val.as_ptr() as usize,
            )
        };
        #[cfg(target_arch = "aarch64")]
        let ret = {
            crate::simd_ops::call_va8_xmm(
                fn_addr as u64,
                out_p as u64,
                0,
                span.as_ptr() as usize as u64,
                f4p as u64,
                f5p as u64,
                def_val.as_ptr() as usize as u64,
                0,
                0,
                [0, x.to_bits() as u128, 0, 0],
            )
            .0 as usize
        };
        assert_eq!(ret, out_p);
        (
            out_vec,
            DTOR_LOG,
            rd::<usize>(f4p + 0x38),
            rd::<usize>(f5p + 0x38),
        )
    };

    for (x, cnt) in [(2.0f32, 0u64), (2.0, 1), (0.5, 3), (6.0, 3), (2.0, 3), (4.0, 3)] {
        let o = invoke(orig, x, cnt);
        let n = invoke(new, x, cnt);
        crate::diff_eq!(o, n, "141da1a60 x={} cnt={}", x, cnt);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_141e1eac0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_8430(vec_ptr: usize, min_cap: u64) {
        GROW_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, (min_cap as u32) + 4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f7_8430, mock_8430 as *const ())?;

    let run = |f: F| -> ([u64; 4], u32, u32, u32) {
        GROW_CALLS = 0;
        let src0: [u64; 2] = [0x1111_2222, 0x3333_4444];
        let mut dst0: [u64; 4] = [0xaaaa, 0, 0, 0];

        let mut entries = [0u8; 0x90 * 2];
        let ep = entries.as_mut_ptr() as usize;
        wr::<usize>(ep, src0.as_ptr() as usize);
        wr::<u32>(ep + 0x08, 2);
        wr::<usize>(ep + 0x60, dst0.as_mut_ptr() as usize);
        wr::<u32>(ep + 0x68, 1);
        wr::<u32>(ep + 0x6c, 1); // triggers grow on first push

        let mut flags_28 = [0u8; 0x18 * 2];
        let f28_p = flags_28.as_mut_ptr() as usize;
        wr::<u8>(f28_p + 0x0e, 1);
        wr::<u8>(f28_p + 0x0f, 1);

        let flags_30: [u32; 2] = [1, 0];

        let mut p1 = [0u64; 8];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1_p + 0x18, 2);
        wr::<usize>(p1_p + 0x20, ep);
        wr::<usize>(p1_p + 0x28, f28_p);
        wr::<usize>(p1_p + 0x30, flags_30.as_ptr() as usize);

        f(p1_p);
        (
            dst0,
            rd::<u32>(ep + 0x08),
            rd::<u32>(ep + 0x68),
            GROW_CALLS,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e1eac0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e1f040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CB_ARGS: [u32; 7] = [0; 7];
    static mut CB_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_va8(_this: usize, arg_ptr: usize, r8b: u8) {
        CB_COUNT += r8b as u32;
        CB_ARGS = rd::<[u32; 7]>(arg_ptr);
    }

    let mut vtbl = [0usize; 24];
    vtbl[0xa8 / 8] = mock_va8 as *const () as usize;

    let run = |f: F| -> ([f32; 16], [u32; 7], u32) {
        CB_ARGS = [0; 7];
        CB_COUNT = 0;

        let obj_a8: [usize; 2] = [vtbl.as_ptr() as usize, 0];
        let mut elem0 = [0u8; 0xb0];
        wr::<usize>(elem0.as_mut_ptr() as usize + 0xa8, obj_a8.as_ptr() as usize);
        let elems: [usize; 2] = [elem0.as_ptr() as usize, elem0.as_ptr() as usize];

        let mut arr_28: [f32; 16] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        let arr_30: [f32; 16] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, // slot 0 equal
            9.0, 10.0, 99.0, 12.0, 13.0, 14.0, 15.0, 16.0, // slot 1 differs
        ];

        let mut p1 = [0u64; 8];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1_p + 0x18, 2);
        wr::<usize>(p1_p + 0x20, elems.as_ptr() as usize);
        wr::<usize>(p1_p + 0x28, arr_28.as_mut_ptr() as usize);
        wr::<usize>(p1_p + 0x30, arr_30.as_ptr() as usize);

        f(p1_p);
        (arr_28, CB_ARGS, CB_COUNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e1f040 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e7c940(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_57b0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src) ^ 0x5555);
        wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08) ^ 0xaaaa);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4022_57b0, mock_57b0 as *const ())?;

    let run = |f: F| -> [u64; 16] {
        let ranges: [u64; 2] = [(2u64 << 32) | 0, (3u64 << 32) | 2];
        let p1: [usize; 2] = [2, ranges.as_ptr() as usize];

        let mut src = [0u64; 12];
        for i in 0..12 {
            src[i] = 0x1000 + i as u64;
        }
        let mut dst = [0u64; 16];

        f(
            p1.as_ptr() as usize,
            1,
            src.as_ptr() as usize,
            dst.as_mut_ptr() as usize,
        );
        dst
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e7c940 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e8d380(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;
    static mut BUF_28: [u8; 256] = [0; 256];
    static mut OBJ_68: [u8; 0x140] = [0; 0x140];

    unsafe extern "win64" fn mock_9140(rcx: usize) {
        for off in (0..0x38).step_by(8) {
            wr::<u64>(rcx + off, 0);
        }
        wr::<u64>(rcx, 0x9140);
    }
    unsafe extern "win64" fn mock_7280(_r10: usize, size: u32, _r8: u32, _r9: u32) -> usize {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(size as u64);
        core::ptr::addr_of_mut!(BUF_28) as usize
    }
    unsafe extern "win64" fn mock_b860() -> usize {
        let p = core::ptr::addr_of_mut!(OBJ_68) as usize;
        wr::<u64>(p + 0x128, 0x7788_99aa);
        wr::<u32>(p + 0x130, 0x42);
        p
    }
    unsafe extern "win64" fn mock_36c0(_obj: usize, _buf: usize, len: u64) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(len);
    }
    unsafe extern "win64" fn mock_3950(_obj: usize, pair_ptr: usize) {
        let a = rd::<u64>(pair_ptr);
        let b = rd::<u64>(pair_ptr + 0x08);
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(a).wrapping_add(b);
    }
    unsafe extern "win64" fn mock_c030(rcx: usize, key_ptr: usize, pkt_ptr: usize) {
        let k0 = rd::<u64>(key_ptr);
        let k1 = rd::<u32>(key_ptr + 0x08) as u64;
        let tag = rd::<u32>(pkt_ptr + 0x48) as u64;
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(rcx as u64)
            .wrapping_add(k0)
            .wrapping_add(k1)
            .wrapping_add(tag);
        // Clear pointers at pkt_ptr and pkt_ptr + 0x40 so cleanup skips TLS arena free
        wr::<usize>(pkt_ptr, 0);
        wr::<usize>(pkt_ptr + 0x40, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4215_9140, mock_9140 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_7280, mock_7280 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40ff_b860, mock_b860 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4215_36c0, mock_36c0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4215_3950, mock_3950 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40ff_c030, mock_c030 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_6348 = rd::<usize>(tls0 + 0x6348);

    let run = |f: F| -> ([u8; 16], u64) {
        CALL_LOG = 0;
        BUF_28 = [0; 256];
        OBJ_68 = [0; 0x140];

        let mut bin_hdr = [0u8; 0x20];
        wr::<usize>(bin_hdr.as_mut_ptr() as usize + 0x10, 0);
        let mut tls_heap = [0u8; 0x100];
        wr::<usize>(
            tls_heap.as_mut_ptr() as usize + 0x08 + 0x10 * 8,
            bin_hdr.as_mut_ptr() as usize,
        );
        wr::<usize>(tls0 + 0x6348, tls_heap.as_mut_ptr() as usize);

        let mut src_data = [0u8; 0x90];
        for i in 0..16 {
            src_data[0x10 + i] = (i as u8) + 1;
        }

        let mut p1 = [0u8; 0x40];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1_p + 0x08, 0x777);
        wr::<u32>(p1_p + 0x14, 0x10);
        wr::<u32>(p1_p + 0x24, 0x80);
        wr::<usize>(p1_p + 0x30, src_data.as_ptr() as usize);

        let p2: [u64; 2] = [0x1122_3344_5566_7788, 0x99aa];
        let p3: [u64; 1] = [0x5000];

        f(p1_p, p2.as_ptr() as usize, p3.as_ptr() as usize);
        let copied = rd::<[u8; 16]>(core::ptr::addr_of!(BUF_28) as usize);
        (copied, CALL_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(tls0 + 0x6348, saved_6348);
    crate::diff_eq!(o, n, "141e8d380 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e99010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut OUT_2A40: ([u64; 5], [u64; 3], [u32; 8]) = ([0; 5], [0; 3], [0; 8]);
    static mut OUT_34C0: ([u64; 5], [u64; 3], [u32; 8]) = ([0; 5], [0; 3], [0; 8]);

    unsafe extern "win64" fn mock_2a40(rcx: usize, rdx: usize, r8: usize) {
        OUT_2A40 = (
            rd::<[u64; 5]>(rcx),
            rd::<[u64; 3]>(rdx),
            rd::<[u32; 8]>(r8),
        );
    }
    unsafe extern "win64" fn mock_34c0(rcx: usize, rdx: usize, r8: usize) {
        OUT_34C0 = (
            rd::<[u64; 5]>(rcx),
            rd::<[u64; 3]>(rdx),
            rd::<[u32; 8]>(r8),
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e2_2a40, mock_2a40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e2_34c0, mock_34c0 as *const ())?;

    let run = |f: F| -> (([u64; 5], [u64; 3], [u32; 8]), ([u64; 5], [u64; 3], [u32; 8])) {
        OUT_2A40 = ([0; 5], [0; 3], [0; 8]);
        OUT_34C0 = ([0; 5], [0; 3], [0; 8]);

        let parent_50: [f32; 8] = [0.0, 0.70710677, 0.0, 0.70710677, 10.0, 20.0, 30.0, 0.0];
        let parent_58: [f32; 8] = [0.5, 0.5, 0.5, 0.5, -5.0, 4.0, 12.0, 0.0];

        let mut node = [0u8; 0xa0];
        let np = node.as_mut_ptr() as usize;
        let local_xform: [f32; 8] = [0.0, 0.0, 0.70710677, 0.70710677, 1.0, 2.0, 3.0, 0.0];
        wr::<[f32; 8]>(np + 0x20, local_xform);
        wr::<usize>(np + 0x50, parent_50.as_ptr() as usize);
        wr::<usize>(np + 0x58, parent_58.as_ptr() as usize);
        wr::<u16>(np + 0x90, 1);

        let mut r15_obj = [0u8; 0x30];
        wr::<usize>(r15_obj.as_mut_ptr() as usize + 0x20, np);

        let arr_30: [usize; 4] = [r15_obj.as_ptr() as usize, 0, 0, 0];
        let arr_20: [u32; 1] = [0];
        let arr_28: [u8; 1] = [0];
        let arr_38: [u8; 1] = [1];
        let arr_40: [u8; 1] = [0];
        let arr_70: [u8; 1] = [0];

        let mut p1 = [0u8; 0x80];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1_p + 0x08, 0xaa00);
        wr::<u64>(p1_p + 0x18, 1);
        wr::<usize>(p1_p + 0x20, arr_20.as_ptr() as usize);
        wr::<usize>(p1_p + 0x28, arr_28.as_ptr() as usize);
        wr::<usize>(p1_p + 0x30, arr_30.as_ptr() as usize);
        wr::<usize>(p1_p + 0x38, arr_38.as_ptr() as usize);
        wr::<usize>(p1_p + 0x40, arr_40.as_ptr() as usize);
        wr::<u64>(p1_p + 0x48, 0xbb01);
        wr::<u64>(p1_p + 0x50, 0xbb02);
        wr::<u64>(p1_p + 0x58, 0xbb03);
        wr::<u64>(p1_p + 0x60, 0xbb04);
        wr::<u64>(p1_p + 0x68, 0xbb05);
        wr::<usize>(p1_p + 0x70, arr_70.as_ptr() as usize);

        let p2: [u64; 3] = [11, 22, 33];
        let p3: [u64; 3] = [44, 55, 66];

        f(p1_p, p2.as_ptr() as usize, p3.as_ptr() as usize);
        (OUT_2A40, OUT_34C0)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e99010 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f4d6d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_v88(this: usize, stream_ctx: usize, flag: u32) {
        let tag = rd::<u64>(this + 0x08);
        let mode = rd::<u32>(stream_ctx + 0x318) as u64;
        let p1 = rd::<u64>(stream_ctx + 0x338);
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(tag)
            .wrapping_add(mode)
            .wrapping_add(p1)
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_v130(this: usize, stream_ctx: usize, flag: u32) {
        let tag = rd::<u64>(this + 0x08);
        let p2 = rd::<u64>(stream_ctx + 0x340);
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(tag)
            .wrapping_add(p2)
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_e280(_stream_ctx: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(0xe280);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;

    let mut vtbl = [0usize; 40];
    vtbl[0x88 / 8] = mock_v88 as *const () as usize;
    vtbl[0x130 / 8] = mock_v130 as *const () as usize;

    let run = |f: F| -> u64 {
        CALL_LOG = 0;
        let obj1: [u64; 2] = [vtbl.as_ptr() as u64, 0x111];
        let obj2: [u64; 2] = [vtbl.as_ptr() as u64, 0x222];

        let buckets3: [u64; 4] = [u64::MAX, 0, 10, obj1.as_ptr() as u64];
        let buckets4: [u64; 4] = [20, obj2.as_ptr() as u64, u64::MAX - 1, 0];

        let mut map3 = [0u64; 12];
        let m3p = map3.as_mut_ptr() as usize;
        wr::<u64>(m3p + 0x40, 2);
        wr::<u64>(m3p + 0x48, u64::MAX);
        wr::<usize>(m3p + 0x58, buckets3.as_ptr() as usize);

        let mut map4 = [0u64; 12];
        let m4p = map4.as_mut_ptr() as usize;
        wr::<u64>(m4p + 0x28, u64::MAX - 1);
        wr::<u64>(m4p + 0x30, 1);
        wr::<u64>(m4p + 0x40, 2);
        wr::<u64>(m4p + 0x48, u64::MAX);
        wr::<usize>(m4p + 0x58, buckets4.as_ptr() as usize);

        f(0xaaaa, 0xbbbb, m3p, m4p);
        CALL_LOG
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141f4d6d0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f79330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_b0c0(_g300: usize, slot_ptr: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(rd::<u64>(slot_ptr));
    }
    unsafe extern "win64" fn mock_8fb0(lock_ptr: usize) {
        wr::<u64>(lock_ptr, 0x0000_0001_0000_0000);
        wr::<u64>(lock_ptr + 0x08, 0x7777);
    }
    unsafe extern "win64" fn mock_fbe0(rsi: usize) {
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(rsi))
            .wrapping_add(rd::<u64>(rsi + 0x08));
    }
    unsafe extern "win64" fn mock_19a0(map_ptr: usize) {
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(map_ptr + 0x40));
    }
    unsafe extern "win64" fn mock_2de0(ptr: usize) {
        CALL_LOG = CALL_LOG.wrapping_mul(31).wrapping_add(ptr as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_b0c0, mock_b0c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4137_8fb0, mock_8fb0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413c_fbe0, mock_fbe0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_410f_19a0, mock_19a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let g_300 = exe::ptr::<usize>(0x1_43a0_8300) as usize;
    let saved_300 = rd::<usize>(g_300);

    let run = |f: F| -> ([u64; 14], u64, u64, u64) {
        CALL_LOG = 0;
        let mut g_obj = [0u64; 14];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<f32>(gp + 0x10, 0.75);
        wr::<u64>(gp + 0x40, 64);
        wr::<usize>(g_300, gp);

        let b1: [u64; 4] = [u64::MAX, 0, 101, 202];
        let b2: [u64; 4] = [303, 404, u64::MAX, 0];

        let mut p1 = [0u8; 0xd0];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1_p + 0x40, 2);
        wr::<u64>(p1_p + 0x48, u64::MAX);
        wr::<usize>(p1_p + 0x58, b1.as_ptr() as usize);
        wr::<u64>(p1_p + 0xa0, 2);
        wr::<u64>(p1_p + 0xa8, u64::MAX);
        wr::<usize>(p1_p + 0xb8, b2.as_ptr() as usize);
        wr::<u64>(p1_p + 0xc0, 0x1234);
        wr::<u64>(p1_p + 0xc8, 0x5678);

        f(p1_p);
        (
            g_obj,
            rd::<u64>(p1_p + 0xc0),
            rd::<u64>(p1_p + 0xc8),
            CALL_LOG,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_300, saved_300);
    crate::diff_eq!(o, n, "141f79330 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f7a100(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_get_id(this: usize) -> u32 {
        rd::<u32>(this + 0x08)
    }
    unsafe extern "win64" fn mock_cb1(_p3: usize, arg_20: usize, p4: u64) {
        let a = rd::<u64>(arg_20);
        let b = rd::<u32>(arg_20 + 0x08) as u64;
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(a)
            .wrapping_add(b)
            .wrapping_add(p4)
            .wrapping_add(1);
    }
    unsafe extern "win64" fn mock_cb2(_p3: usize, arg_20: usize, p4: u64) {
        let a = rd::<u64>(arg_20);
        let b = rd::<u32>(arg_20 + 0x08) as u64;
        CALL_LOG = CALL_LOG
            .wrapping_mul(31)
            .wrapping_add(a)
            .wrapping_add(b)
            .wrapping_add(p4)
            .wrapping_add(2);
    }

    let mut vtbl = [0usize; 42];
    vtbl[0x140 / 8] = mock_get_id as *const () as usize;

    let run = |f: F, want_id: u32| -> u64 {
        CALL_LOG = 0;

        let mut n_end = [0u64; 4];
        let mut n2 = [0u64; 4];
        let mut n1 = [0u64; 4];
        let mut n0 = [0u64; 4];

        let p_end = n_end.as_mut_ptr() as usize;
        let p2 = n2.as_mut_ptr() as usize;
        let p1_node = n1.as_mut_ptr() as usize;
        let p0 = n0.as_mut_ptr() as usize;

        wr::<usize>(p0, p1_node);
        wr::<u32>(p0 + 0x10, 999);
        wr::<usize>(p0 + 0x18, mock_cb1 as *const () as usize);

        wr::<usize>(p1_node, p2);
        wr::<u32>(p1_node + 0x10, 42);
        wr::<usize>(p1_node + 0x18, mock_cb1 as *const () as usize);

        wr::<usize>(p2, p_end);
        wr::<u32>(p2 + 0x10, 42);
        wr::<usize>(p2 + 0x18, mock_cb2 as *const () as usize);

        let buckets: [usize; 2] = [p0, p2];

        let mut p1 = [0u64; 8];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1_p + 0x08, p_end);
        wr::<usize>(p1_p + 0x18, buckets.as_ptr() as usize);
        wr::<u64>(p1_p + 0x30, 0); // mask = 0 -> always bucket 0

        let p2_arg: [u64; 2] = [0x1234_5678, 0x9abc];
        let p3_obj: [u64; 2] = [vtbl.as_ptr() as u64, want_id as u64];

        f(
            p1_p,
            p2_arg.as_ptr() as usize,
            p3_obj.as_ptr() as usize,
            0x7777,
        );
        CALL_LOG
    };

    for id in [42u32, 999, 123] {
        let o = run(f_o, id);
        let n = run(f_n, id);
        crate::diff_eq!(o, n, "141f7a100 id={}", id);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_4160_0880, "unknown_141600880", rend::cauldron_hot_x3::unknown_141600880 as *const (), rend::cauldron_hot_x3::diff_unknown_141600880),
// Replacement::verified(0x1_4164_5fc0, "unknown_141645fc0", rend::cauldron_hot_x3::unknown_141645fc0 as *const (), rend::cauldron_hot_x3::diff_unknown_141645fc0),
// Replacement::verified(0x1_41ac_d4d0, "unknown_141acd4d0", rend::cauldron_hot_x3::unknown_141acd4d0 as *const (), rend::cauldron_hot_x3::diff_unknown_141acd4d0),
// Replacement::verified_throws(0x1_41da_1a60, "unknown_141da1a60", rend::cauldron_hot_x3::unknown_141da1a60 as *const (), Throws::Caught, rend::cauldron_hot_x3::diff_unknown_141da1a60),
// Replacement::verified(0x1_41e1_eac0, "unknown_141e1eac0", rend::cauldron_hot_x3::unknown_141e1eac0 as *const (), rend::cauldron_hot_x3::diff_unknown_141e1eac0),
// Replacement::verified(0x1_41e1_f040, "unknown_141e1f040", rend::cauldron_hot_x3::unknown_141e1f040 as *const (), rend::cauldron_hot_x3::diff_unknown_141e1f040),
// Replacement::verified(0x1_41e7_c940, "unknown_141e7c940", rend::cauldron_hot_x3::unknown_141e7c940 as *const (), rend::cauldron_hot_x3::diff_unknown_141e7c940),
// Replacement::verified_throws(0x1_41e8_d380, "unknown_141e8d380", rend::cauldron_hot_x3::unknown_141e8d380 as *const (), Throws::Caught, rend::cauldron_hot_x3::diff_unknown_141e8d380),
// Replacement::verified(0x1_41e9_9010, "unknown_141e99010", rend::cauldron_hot_x3::unknown_141e99010 as *const (), rend::cauldron_hot_x3::diff_unknown_141e99010),
// Replacement::verified(0x1_41f4_d6d0, "unknown_141f4d6d0", rend::cauldron_hot_x3::unknown_141f4d6d0 as *const (), rend::cauldron_hot_x3::diff_unknown_141f4d6d0),
// Replacement::verified_throws(0x1_41f7_9330, "unknown_141f79330", rend::cauldron_hot_x3::unknown_141f79330 as *const (), Throws::Caught, rend::cauldron_hot_x3::diff_unknown_141f79330),
// Replacement::verified(0x1_41f7_a100, "unknown_141f7a100", rend::cauldron_hot_x3::unknown_141f7a100 as *const (), rend::cauldron_hot_x3::diff_unknown_141f7a100),
