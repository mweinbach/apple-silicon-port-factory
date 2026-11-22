//! Decompiled hot functions shard W4 (`0x141da1810..0x1420abdc0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn rcp_ss(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        1.0f32 / x
    }
}

#[inline(always)]
fn lerp_rcp(x: f32, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let cond_le_x0 = x <= x0;
    let dx = x1 - x0;
    let denom = if dx == 0.0f32 { 1.0f32 } else { dx };
    let cond_ge_x1 = x1 <= x;
    let max_x = if x > x0 { x } else { x0 };
    let clamped_x = if max_x < x1 { max_x } else { x1 };
    let r = rcp_ss(denom);
    let r_sq_d = (r * r) * denom;
    let inv_denom = (r + r) - r_sq_d;
    let t = (clamped_x - x0) * inv_denom;
    let interp = y0 + (t * (y1 - y0));
    let sel1 = if cond_le_x0 { y0 } else { interp };
    if cond_ge_x1 { y1 } else { sel1 }
}

#[inline(always)]
unsafe fn call_std_func_f32(func_obj: usize, elem_ptr: usize) -> f32 {
    let callable = rd::<usize>(func_obj + 0x38);
    if callable == 0 {
        exe::call_iat!(0x1_4264_5878, ());
        unreachable!();
    }
    let vtbl = rd::<usize>(callable);
    let fn_va = rd::<u64>(vtbl + 0x10);
    let (_, xmm0) = crate::simd_ops::call_va8_xmm(
        fn_va,
        callable as u64,
        elem_ptr as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0; 4],
    );
    f32::from_bits(xmm0 as u32)
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

/// `unknown_141da1810`
/// Original: 0x141da1810 (576 bytes, Throws::Caught, FLOAT_RETURN)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_141da1810(
    param_1: f32,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) -> f32 {
    unknown_141da1810_impl(param_1, param_2, param_3, param_4, param_5)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_141da1810(
    param_1: f32,
    _rcx: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) -> f32 {
    unknown_141da1810_impl(param_1, param_2, param_3, param_4, param_5)
}

#[inline(always)]
unsafe fn unknown_141da1810_impl(
    param_1: f32,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) -> f32 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let count = rd::<u64>(param_2);
    let res = if count == 0 {
        let def_val = rd::<f32>(param_5);
        destroy_std_func_inline(param_3);
        destroy_std_func_inline(param_4);
        def_val
    } else {
        let base_ptr = rd::<usize>(param_2 + 0x08);
        if count == 1 {
            let v = call_std_func_f32(param_4, base_ptr);
            destroy_std_func_inline(param_3);
            destroy_std_func_inline(param_4);
            v
        } else {
            let end_ptr = base_ptr.wrapping_add((count as usize) * 8);
            let x_first = call_std_func_f32(param_3, base_ptr);
            if !(x_first < param_1) {
                let v = call_std_func_f32(param_4, base_ptr);
                destroy_std_func_inline(param_3);
                destroy_std_func_inline(param_4);
                v
            } else {
                let last_ptr = end_ptr.wrapping_sub(8);
                let x_last = call_std_func_f32(param_3, last_ptr);
                if !(param_1 < x_last) {
                    let v = call_std_func_f32(param_4, last_ptr);
                    exe::call_va!(0x1_401e_1560, (param_3));
                    exe::call_va!(0x1_401e_1560, (param_4));
                    v
                } else {
                    let mut idx: usize = 0;
                    if count > 1 {
                        let mut step: u64 = 1;
                        let mut cur = base_ptr.wrapping_add(8);
                        loop {
                            let x_cur = call_std_func_f32(param_3, cur);
                            if param_1 <= x_cur {
                                break;
                            }
                            idx += 1;
                            cur = cur.wrapping_add(8);
                            step += 1;
                            if step >= count {
                                break;
                            }
                        }
                    }
                    let seg_ptr = base_ptr.wrapping_add(idx * 8);
                    let x0_a = call_std_func_f32(param_3, seg_ptr);
                    let x1 = call_std_func_f32(param_3, seg_ptr.wrapping_add(8));
                    let x0_b = call_std_func_f32(param_3, seg_ptr);
                    let dx = x1 - x0_b;
                    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
                    let inv_dx = if dx != 0.0f32 { one / dx } else { 0.0f32 };
                    let t = (param_1 - x0_a) * inv_dx;
                    let y1 = call_std_func_f32(param_4, seg_ptr.wrapping_add(8));
                    let y0 = call_std_func_f32(param_4, seg_ptr);
                    let v = (one - t) * y0 + y1 * t;
                    exe::call_va!(0x1_401e_1560, (param_3));
                    exe::call_va!(0x1_401e_1560, (param_4));
                    v
                }
            }
        }
    };
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

#[inline(always)]
unsafe fn rec_srw_lock(lock_base: usize) {
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_base + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_base));
        wr::<u32>(lock_base + 0x08, tid);
    }
    wr::<u32>(lock_base + 0x0c, rd::<u32>(lock_base + 0x0c).wrapping_add(1));
}

#[inline(always)]
unsafe fn rec_srw_unlock(lock_base: usize) {
    let cnt = rd::<u32>(lock_base + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_base + 0x0c, cnt);
    if cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((lock_base + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_base));
    }
}

#[inline(always)]
unsafe fn swap_slot_16(a: usize, b: usize) {
    if a != b {
        let q0 = rd::<u64>(a);
        let q1 = rd::<u64>(b);
        wr::<u64>(a, q1);
        wr::<u64>(b, q0);
        let d2a = rd::<u32>(a + 0x08);
        let d2b = rd::<u32>(b + 0x08);
        wr::<u32>(a + 0x08, d2b);
        wr::<u32>(b + 0x08, d2a);
        let d3a = rd::<u32>(a + 0x0c);
        let d3b = rd::<u32>(b + 0x0c);
        wr::<u32>(a + 0x0c, d3b);
        wr::<u32>(b + 0x0c, d3a);
    }
}

#[inline(always)]
unsafe fn release_ref_count_base(ctrl: usize) {
    if ctrl != 0 {
        if crate::simd_ops::atomic_xadd_u32((ctrl + 0x08) as u64, u32::MAX) == 1 {
            let vtbl0 = rd::<usize>(ctrl);
            exe::call_va!(rd::<usize>(vtbl0), (ctrl));
            if crate::simd_ops::atomic_xadd_u32((ctrl + 0x0c) as u64, u32::MAX) == 1 {
                let vtbl1 = rd::<usize>(ctrl);
                exe::call_va!(rd::<usize>(vtbl1 + 0x08), (ctrl));
            }
        }
    }
}

/// `unknown_141e791b0`
/// Original: 0x141e791b0 (2278 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e791b0(param_1: usize) {
    let mut local_460 = rd::<u32>(exe::ptr::<u32>(0x1_4387_969c) as usize);
    exe::call_va!(0x1_40fe_5ef0, ((&mut local_460 as *mut u32) as usize));

    if rd::<u8>(param_1 + 0x379) != 0 && rd::<u8>(param_1 + 0x11) != 0 {
        exe::call_va!(0x1_41e7_8c80, (param_1));
        wr::<u8>(param_1 + 0x11, 0);
    }

    let g_fef8_slot = exe::ptr::<usize>(0x1_4397_fef8) as usize;
    let mut g_fef8 = rd::<usize>(g_fef8_slot);
    for idx in 0..0x13u32 {
        let off = (idx as usize) * 0xd8;
        if rd::<u8>(g_fef8.wrapping_add(off)) == 0 {
            exe::call_va!(0x1_4224_0e40, (g_fef8, idx));
            g_fef8 = rd::<usize>(g_fef8_slot);
        }
    }

    let tbl_6980 = exe::ptr::<u8>(0x1_43c1_6980) as usize;
    let g_698_slot = exe::ptr::<u32>(0x1_4387_9698) as usize;
    for bucket in 0..0x40usize {
        let stride = bucket * 0x158;
        let lock_addr = tbl_6980 + 0x140 + stride;
        let s0 = tbl_6980 + 0x40 + stride;
        let s1 = tbl_6980 + 0x50 + stride;
        let s2 = tbl_6980 + 0x60 + stride;
        let s3 = tbl_6980 + 0x70 + stride;
        let d0 = tbl_6980 + 0xc0 + stride;
        let d1 = tbl_6980 + 0xd0 + stride;
        let d2 = tbl_6980 + 0xe0 + stride;
        let d3 = tbl_6980 + 0xf0 + stride;

        rec_srw_lock(lock_addr);
        swap_slot_16(s0, d0);
        swap_slot_16(s1, d1);
        swap_slot_16(s2, d2);
        swap_slot_16(s3, d3);
        rec_srw_unlock(lock_addr);

        rec_srw_lock(lock_addr);
        exe::call_va!(0x1_4249_d670, (s0));
        exe::call_va!(0x1_4249_d670, (s1));
        exe::call_va!(0x1_4249_d670, (s2));
        exe::call_va!(0x1_4249_d670, (s3));
        rec_srw_unlock(lock_addr);

        let dt_bits = rd::<u32>(g_698_slot);
        rec_srw_lock(lock_addr);
        for (src, dst) in [(d0, s0), (d1, s1), (d2, s2), (d3, s3)] {
            let cnt = rd::<u32>(src + 0x08) as u64;
            let ptr = rd::<u64>(src);
            let mut pair: [u64; 2] = [cnt, ptr];
            let _ = crate::simd_ops::call_va8_xmm(
                0x1_4249_d710,
                ptr,
                pair.as_mut_ptr() as usize as u64,
                dst as u64,
                dt_bits as u64,
                0,
                0,
                0,
                0,
                [0, 0, 0, dt_bits as u128],
            );
        }
        rec_srw_unlock(lock_addr);
    }

    let mut vec_a0 = [0u64; 12];
    let va0 = vec_a0.as_mut_ptr() as usize;
    wr::<usize>(va0, va0 + 0x10);
    wr::<u32>(va0 + 0x08, 0);
    wr::<u32>(va0 + 0x0c, 4);

    let e80_ptr = rd::<usize>(exe::ptr::<usize>(0x1_43a4_8e80) as usize);
    let e88_cnt = rd::<u32>(exe::ptr::<u32>(0x1_43a4_8e88) as usize) as usize;
    let mut end_e80 = e80_ptr.wrapping_add(e88_cnt * 0x14);
    let mut beg_e80 = e80_ptr;
    exe::call_va!(
        0x1_40fe_6af0,
        (
            va0,
            (&mut beg_e80 as *mut usize) as usize,
            (&mut end_e80 as *mut usize) as usize
        )
    );
    exe::call_va!(0x1_40fe_6060, (param_1 + 0x0c, va0));

    wr::<u32>(
        param_1 + 0x370,
        rd::<u32>(exe::ptr::<u32>(0x1_4385_49a8) as usize),
    );
    wr::<u32>(
        param_1 + 0x374,
        rd::<u32>(exe::ptr::<u32>(0x1_4387_9670) as usize),
    );

    let mut sp_ptr: usize = 0;
    let mut sp_ctrl: usize = 0;
    let lock_fa30 = exe::ptr::<u8>(0x1_4384_fa30) as usize;
    let g_d9b8 = exe::ptr::<u32>(0x1_4397_d9b8) as usize;
    let g_8460 = exe::ptr::<usize>(0x1_43c4_8460) as usize;

    rec_srw_lock(lock_fa30);
    let mut tmp_50 = [0usize; 2];
    if rd::<u32>(g_d9b8) == 1 {
        let c460 = rd::<usize>(g_8460);
        if c460 != 0 && ((rd::<u32>(c460 + 0x28) >> 1) & 1) != 0 {
            exe::call_va!(0x1_41dc_8340, (c460, tmp_50.as_mut_ptr() as usize));
        }
    }
    rec_srw_unlock(lock_fa30);

    if tmp_50[0] != 0 {
        sp_ptr = tmp_50[0];
        sp_ctrl = tmp_50[1];
        tmp_50 = [0, 0];
    }
    release_ref_count_base(tmp_50[1]);

    if sp_ptr != 0 {
        let vec_680 = param_1 + 0x680;
        let len = rd::<u32>(vec_680 + 0x08) as u64;
        let cap = rd::<u32>(vec_680 + 0x0c) as u64;
        if (len << 5) == (cap << 5) {
            let next_len = len.wrapping_add(1);
            let grown = (cap >> 1).wrapping_add(cap);
            let new_cap = if grown < next_len { next_len } else { grown };
            exe::call_va!(0x1_40fe_6fa0, (vec_680, new_cap));
        }
        let slot = rd::<usize>(vec_680).wrapping_add((rd::<u32>(vec_680 + 0x08) as usize) * 0x20);
        wr::<[u64; 4]>(slot, [0; 4]);
        if sp_ctrl != 0 {
            let _ = crate::simd_ops::atomic_xadd_u32((sp_ctrl + 0x08) as u64, 1);
        }
        wr::<usize>(slot + 0x10, sp_ptr);
        wr::<usize>(slot + 0x18, sp_ctrl);
        wr::<u32>(vec_680 + 0x08, rd::<u32>(vec_680 + 0x08).wrapping_add(1));
    }

    let mut sp_arg: [u64; 4] = [0, 0, sp_ptr as u64, sp_ctrl as u64];
    if sp_ctrl != 0 {
        let _ = crate::simd_ops::atomic_xadd_u32((sp_ctrl + 0x08) as u64, 1);
    }

    let mut frame_100 = [0u64; 0x308 / 8];
    let f100 = frame_100.as_mut_ptr() as usize;
    wr::<u32>(f100, rd::<u32>(param_1 + 0x370));
    wr::<u32>(f100 + 0x04, rd::<u32>(param_1 + 0x374));
    wr::<u8>(f100 + 0x08, rd::<u8>(param_1 + 0x378));
    wr::<u8>(f100 + 0x09, rd::<u8>(param_1 + 0x379));
    exe::call_va!(0x1_40f3_3c50, (f100 + 0x10, param_1 + 0x380));
    exe::call_va!(
        0x1_422a_5dc0,
        (
            rd::<u32>(param_1 + 0x0c),
            f100,
            sp_arg.as_mut_ptr() as usize
        )
    );

    let g_ff58_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    exe::call_va!(0x1_40fe_6ca0, (rd::<usize>(g_ff58_slot)));

    rec_srw_lock(lock_fa30);
    if rd::<u32>(g_d9b8) == 1 {
        let g_8450 = exe::ptr::<u64>(0x1_43c4_8450) as usize;
        let g_8458 = exe::ptr::<usize>(0x1_43c4_8458) as usize;
        let g_8468 = exe::ptr::<usize>(0x1_43c4_8468) as usize;
        let g_8470 = exe::ptr::<u64>(0x1_43c4_8470) as usize;
        let g_8478 = exe::ptr::<usize>(0x1_43c4_8478) as usize;

        let c58 = rd::<usize>(g_8458);
        if c58 != 0 {
            let _ = crate::simd_ops::atomic_xadd_u32((c58 + 0x08) as u64, 1);
        }
        let c58_after = rd::<usize>(g_8458) as u64;
        let p50 = rd::<u64>(g_8450);

        let c68 = rd::<usize>(g_8468);
        if c68 != 0 {
            let _ = crate::simd_ops::atomic_xadd_u32((c68 + 0x08) as u64, 1);
        }
        let c68_after = rd::<usize>(g_8468) as u64;
        let p60 = rd::<usize>(g_8460) as u64;

        let c78 = rd::<usize>(g_8478);
        if c78 != 0 {
            let _ = crate::simd_ops::atomic_xadd_u32((c78 + 0x08) as u64, 1);
        }
        let c78_after = rd::<usize>(g_8478) as u64;
        let p70 = rd::<u64>(g_8470);

        let mut pkt_70: [u64; 6] = [p50, c58_after, p60, c68_after, p70, c78_after];
        exe::call_va!(
            0x1_40f3_5c10,
            (rd::<usize>(g_ff58_slot), pkt_70.as_mut_ptr() as usize)
        );
    }
    rec_srw_unlock(lock_fa30);

    if rd::<u32>(param_1 + 0x264) != 0 {
        let data_field = param_1 + 0x268;
        let b263 = rd::<u8>(param_1 + 0x263) as u32;
        let cap_bits = if b263 < 0x80 {
            b263 * 8
        } else {
            (b263 << 8).wrapping_sub(0x7c00)
        };
        let len_bits = (rd::<u32>(param_1 + 0x260) & 0x00ff_ffff) << 3;
        let mut str_ptr = if len_bits <= cap_bits {
            data_field
        } else {
            rd::<usize>(data_field)
        };
        exe::call_va!(
            0x1_40fe_62c0,
            (param_1 + 0x0c, (&mut str_ptr as *mut usize) as usize)
        );
        wr::<u32>(param_1 + 0x264, 0);

        let b263_2 = rd::<u8>(param_1 + 0x263) as u32;
        let cap_bits_2 = if b263_2 < 0x80 {
            b263_2 * 8
        } else {
            (b263_2 << 8).wrapping_sub(0x7c00)
        };
        let len_bits_2 = (rd::<u32>(param_1 + 0x260) & 0x00ff_ffff) << 3;
        let zero_ptr = if len_bits_2 <= cap_bits_2 {
            data_field
        } else {
            rd::<usize>(data_field)
        };
        wr::<u8>(zero_ptr, 0);
    }

    exe::call_va!(0x1_40fb_d7e0, (param_1 + 0x0c));
    exe::call_va!(0x1_40fe_6de0, (rd::<usize>(g_ff58_slot), param_1));
    exe::call_va!(0x1_40fe_64b0, (rd::<usize>(g_ff58_slot)));

    let g_690_slot = exe::ptr::<u64>(0x1_4387_9690) as usize;
    let mut next_id = rd::<u64>(g_690_slot).wrapping_add(1);
    wr::<u64>(g_690_slot, next_id);
    exe::call_va!(
        0x1_40fb_d950,
        (param_1 + 0x0c, (&mut next_id as *mut u64) as usize)
    );
    wr::<u8>(param_1 + 0x10, 0);

    release_ref_count_base(sp_ctrl);
    exe::call_va!(0x1_403b_8050, (va0));
}

/// `unknown_141e7b330`
/// Original: 0x141e7b330 (728 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e7b330(param_1: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_0f40) as usize) == 0 {
        return;
    }
    if rd::<u64>(param_1 + 0x18) == 0 {
        return;
    }

    let mut h_soft: u32 = 0x811c_9dc5;
    for &b in b"SoftEyesDisabled" {
        h_soft = (((b as i8 as i32 as u32) | 0x20) ^ h_soft).wrapping_mul(0x0100_0193);
    }
    let mut h_cine: u32 = 0x811c_9dc5;
    for &b in b"DATA_CINEMATIC_FACE" {
        h_cine = (((b as i8 as i32 as u32) | 0x20) ^ h_cine).wrapping_mul(0x0100_0193);
    }

    let mut idx: usize = 0;
    let mut off_30: usize = 0;
    let mut off_48: usize = 0;
    let mut off_0c: usize = 0;
    let mut off_10: usize = 0;

    loop {
        let p20 = rd::<usize>(param_1 + 0x20);
        let p28 = rd::<usize>(param_1 + 0x28);
        let p38 = rd::<usize>(param_1 + 0x38);
        let p40 = rd::<usize>(param_1 + 0x40);
        let p48 = rd::<usize>(param_1 + 0x48);
        let p50 = rd::<usize>(param_1 + 0x50);
        let p58 = rd::<usize>(param_1 + 0x58);

        let rbp_slot = p28.wrapping_add(off_30);
        if rd::<u32>(p58.wrapping_add(idx * 4)) == 0 {
            let mut r13d = rd::<u32>(p38.wrapping_add(idx * 8));
            let rdi = p40.wrapping_add(off_48);
            let r15d = rd::<u32>(rdi + 0x30);
            let r12 = rd::<u64>(rdi + 0x28);

            let idx_soft = exe::call_va!(0x1_4248_9e40, (r12, r15d, h_soft) -> u32);
            let idx_cine = exe::call_va!(0x1_4248_9e40, (r12, r15d, h_cine) -> u32);

            let mut disable_bit = false;
            if idx_soft != u32::MAX {
                let w0 = rd::<u16>(rdi) as usize;
                let floats = rd::<usize>(rdi.wrapping_add(w0 * 16 + 0x08));
                if rd::<f32>(floats.wrapping_add((idx_soft as usize) * 4)) > 0.0f32 {
                    disable_bit = true;
                }
            }
            if !disable_bit && idx_cine != u32::MAX {
                let w0 = rd::<u16>(rdi) as usize;
                let floats = rd::<usize>(rdi.wrapping_add(w0 * 16 + 0x08));
                if rd::<f32>(floats.wrapping_add((idx_cine as usize) * 4)) > 0.0f32 {
                    disable_bit = true;
                }
            }
            if disable_bit {
                r13d &= !2u32;
            }

            let mut dl = rd::<u8>(p48.wrapping_add(off_0c + 0x08));
            let r8b = rd::<u8>(p50.wrapping_add(idx * 4));
            if r8b >= dl {
                let cl = rd::<u8>(p48.wrapping_add(off_0c + 0x09));
                dl = if r8b > cl { cl } else { r8b };
            }

            let edi = dl as u32;
            let p30 = rd::<usize>(param_1 + 0x30);
            let mask_eax = if (edi as i32) < rd::<i32>(p48.wrapping_add(off_0c)) {
                r13d
            } else {
                0
            };
            let changed = (rd::<u32>(rbp_slot + 0x28) != edi) as u8;
            let arg_r9 = rd::<u32>(p38.wrapping_add(idx * 8 + 0x04)) | mask_eax;
            let arg_r8 = if p30 != 0 {
                rd::<u64>(p30.wrapping_add(off_10))
            } else {
                0
            };
            let arg_rdx = rd::<u64>(p20.wrapping_add(off_10));

            exe::call_va!(
                0x1_421b_c290,
                (rbp_slot, arg_rdx, arg_r8, arg_r9, changed)
            );
            wr::<u32>(rbp_slot + 0x28, edi);
        }

        off_30 = off_30.wrapping_add(0x30);
        off_48 = off_48.wrapping_add(0x48);
        idx += 1;
        off_0c = off_0c.wrapping_add(0x0c);
        off_10 = off_10.wrapping_add(0x10);
        if (idx as u64) >= rd::<u64>(param_1 + 0x18) {
            break;
        }
    }
}

/// `coregame_141e9fa80` (`coregame::PersistentEpisodeBlock::vfunc_1`)
/// Original: 0x141e9fa80 (63 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_141e9fa80(param_1: usize) -> u8 {
    if rd::<u8>(param_1 + 0xe8) == 0 {
        return 1;
    }
    if rd::<u32>(param_1 + 0xec).wrapping_sub(1) <= 1 {
        return 1;
    }
    if rd::<u32>(param_1 + 0xec) == 2 {
        return 1;
    }
    if rd::<u8>(param_1 + 0xe8) != 0 {
        return 0;
    }
    let sub = rd::<usize>(param_1 + 0x08);
    if rd::<u8>(sub + 0x2a0) != 0 {
        1
    } else {
        0
    }
}

#[inline(always)]
unsafe fn rbtree_lower_bound(head: usize, key: u64) -> usize {
    let mut best = head;
    let mut cur = rd::<usize>(head + 0x08);
    while rd::<u8>(cur + 0x19) == 0 {
        if rd::<u64>(cur + 0x20) < key {
            cur = rd::<usize>(cur + 0x10);
        } else {
            best = cur;
            cur = rd::<usize>(cur);
        }
    }
    if rd::<u8>(best + 0x19) != 0 || key < rd::<u64>(best + 0x20) {
        head
    } else {
        best
    }
}

#[inline(always)]
unsafe fn push_node_pair(node: usize, val_a: u64, val_b: u64) {
    let vec1 = node + 0x28;
    let len1 = rd::<u32>(vec1 + 0x08) as u64;
    let cap1 = rd::<u32>(vec1 + 0x0c) as u64;
    if (len1 << 3) == (cap1 << 3) {
        let next_len = len1.wrapping_add(1);
        let grown = (cap1 >> 1).wrapping_add(cap1);
        let new_cap = if grown < next_len { next_len } else { grown };
        exe::call_va!(0x1_401d_eba0, (vec1, new_cap));
    }
    let idx1 = rd::<u32>(vec1 + 0x08) as usize;
    wr::<u64>(rd::<usize>(vec1).wrapping_add(idx1 * 8), val_a);
    wr::<u32>(vec1 + 0x08, (idx1 as u32).wrapping_add(1));

    let vec2 = node + 0x38;
    let len2 = rd::<u32>(vec2 + 0x08) as u64;
    let cap2 = rd::<u32>(vec2 + 0x0c) as u64;
    if (len2 << 3) == (cap2 << 3) {
        exe::call_va!(0x1_401e_6d50, (vec2, len2.wrapping_add(1)));
    }
    let idx2 = rd::<u32>(vec2 + 0x08) as usize;
    wr::<u64>(rd::<usize>(vec2).wrapping_add(idx2 * 8), val_b);
    wr::<u32>(vec2 + 0x08, (idx2 as u32).wrapping_add(1));
}

#[inline(always)]
unsafe fn advance_arch_iter(iter_ptr: usize) {
    let mut cur_arch = rd::<usize>(iter_ptr + 0x10).wrapping_add(2);
    wr::<usize>(iter_ptr + 0x10, cur_arch);
    let end_arch = rd::<usize>(iter_ptr + 0x18);
    if cur_arch != end_arch {
        let w40 = rd::<usize>(iter_ptr + 0x08);
        loop {
            let arch = rd::<u16>(cur_arch) as usize;
            if rd::<usize>(w40.wrapping_add(arch * 8 + 0x48)) != 0
                && rd::<u32>(w40.wrapping_add(arch * 4 + 0x10048)) != 0
            {
                break;
            }
            cur_arch = cur_arch.wrapping_add(2);
            wr::<usize>(iter_ptr + 0x10, cur_arch);
            if cur_arch == end_arch {
                break;
            }
        }
        if cur_arch != end_arch {
            let arch = rd::<u16>(cur_arch) as usize;
            let w0 = rd::<usize>(iter_ptr);
            let blk = rd::<usize>(w0.wrapping_add(arch * 8 + 0x48));
            wr::<usize>(iter_ptr + 0x20, blk);
            let cnt = if blk != 0 {
                rd::<u32>(w0.wrapping_add(arch * 4 + 0x10048)) as u64
            } else {
                0
            };
            wr::<u64>(iter_ptr + 0x28, cnt);
            wr::<u64>(iter_ptr + 0x30, 0);
        }
    }
}

/// `unknown_141eac250`
/// Original: 0x141eac250 (621 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eac250(param_1: usize, param_2: usize) {
    if rd::<u64>(param_2 + 0x08) == 0 {
        return;
    }

    let r11 = rd::<usize>(param_1).wrapping_add(0x40);
    let mut span_in: [u64; 2] = [rd::<u64>(param_1 + 0x08), rd::<u64>(param_1 + 0x10)];
    let mut span_out: [u64; 3] = [0; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            span_out.as_mut_ptr() as usize,
            r11,
            span_in.as_mut_ptr() as usize
        )
    );

    let mut iter: [u64; 9] = [0; 9];
    let iter_ptr = iter.as_mut_ptr() as usize;
    wr::<usize>(iter_ptr, r11);
    wr::<u64>(iter_ptr + 0x08, span_out[0]);
    wr::<u64>(iter_ptr + 0x10, span_out[1]);
    wr::<u64>(iter_ptr + 0x18, span_out[2]);
    wr::<u64>(iter_ptr + 0x28, 0);
    wr::<u64>(iter_ptr + 0x30, 0);
    if span_out[1] != span_out[2] {
        let arch = rd::<u16>(span_out[1] as usize) as usize;
        let blk = rd::<usize>(r11.wrapping_add(arch * 8 + 0x48));
        wr::<usize>(iter_ptr + 0x20, blk);
        let cnt = if blk != 0 {
            rd::<u32>(r11.wrapping_add(arch * 4 + 0x10048)) as u64
        } else {
            0
        };
        wr::<u64>(iter_ptr + 0x28, cnt);
    }

    exe::call_va!(0x1_4103_83a0, (iter_ptr));
    let mut rdi = rd::<usize>(iter_ptr + 0x30);
    let mut r14 = rd::<usize>(iter_ptr + 0x28);
    while rdi < r14 {
        while rdi < r14 {
            let col_a = rd::<usize>(iter_ptr + 0x38);
            let col_b = rd::<usize>(iter_ptr + 0x40);
            let key = rd::<u64>(col_a.wrapping_add(rdi * 8));
            if key != 0 {
                let head = rd::<usize>(param_2);
                let node = rbtree_lower_bound(head, key);
                let val_b = rd::<u64>(col_b.wrapping_add(rdi * 8));
                if val_b.wrapping_sub(1) <= u64::MAX - 3 {
                    let blk = rd::<usize>(iter_ptr + 0x20);
                    let ent_id = rd::<u64>(blk.wrapping_add(rdi * 8 + 0x10));
                    push_node_pair(node, val_b, ent_id);
                }
            }
            rdi += 1;
            wr::<u64>(iter_ptr + 0x30, rdi as u64);
        }
        advance_arch_iter(iter_ptr);
        exe::call_va!(0x1_4103_83a0, (iter_ptr));
        rdi = rd::<usize>(iter_ptr + 0x30);
        r14 = rd::<usize>(iter_ptr + 0x28);
    }
}

/// `unknown_141eac8c0`
/// Original: 0x141eac8c0 (207 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eac8c0(param_1: usize) {
    let mut count = rd::<u64>(param_1 + 0x18);
    if count != 0 {
        let mut off_18: usize = 0;
        let mut off_10: usize = 0;
        loop {
            let mut local_20: [u64; 3] = [
                rd::<u64>(param_1 + 0x20),
                rd::<u64>(param_1 + 0x28),
                rd::<u64>(param_1 + 0x30),
            ];
            let r8 = (local_20[0] as usize).wrapping_add(off_18);
            let rdx = rd::<usize>(param_1 + 0x28).wrapping_add(off_10);
            let r14 = (local_20[2] as usize).wrapping_add(off_18);
            let r9 = rd::<usize>(r8 + 0x08);
            let rax = rd::<usize>(rdx);
            let rcx = rd::<usize>(rax + 0x18);
            let tag = rd::<u32>(rcx + 0x04) ^ rd::<u32>(r9 + 0x208);
            if tag != rd::<u32>(r14 + 0x10) {
                let rdi = exe::call_va!(
                    0x1_41eb_25e0,
                    (local_20.as_mut_ptr() as usize, rdx, r8) -> usize
                );
                exe::call_va!(0x1_4033_0e60, (r14, rdi));
                wr::<u32>(r14 + 0x10, rd::<u32>(rdi + 0x10));
                exe::call_va!(0x1_401f_2bc0, (local_20.as_mut_ptr() as usize));
            }
            off_10 = off_10.wrapping_add(0x10);
            off_18 = off_18.wrapping_add(0x18);
            count -= 1;
            if count == 0 {
                break;
            }
        }
    }
}

/// `unknown_141ec6230`
/// Original: 0x141ec6230 (870 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ec6230(param_1: usize, param_2: usize) {
    if rd::<u64>(param_2 + 0x08) == 0 {
        return;
    }

    let r11 = rd::<usize>(param_1).wrapping_add(0x40);
    let mut span_in: [u64; 2] = [rd::<u64>(param_1 + 0x08), rd::<u64>(param_1 + 0x10)];
    let mut span_out: [u64; 3] = [0; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            span_out.as_mut_ptr() as usize,
            r11,
            span_in.as_mut_ptr() as usize
        )
    );

    let mut iter: [u64; 9] = [0; 9];
    let iter_ptr = iter.as_mut_ptr() as usize;
    wr::<usize>(iter_ptr, r11);
    wr::<u64>(iter_ptr + 0x08, span_out[0]);
    wr::<u64>(iter_ptr + 0x10, span_out[1]);
    wr::<u64>(iter_ptr + 0x18, span_out[2]);
    wr::<u64>(iter_ptr + 0x28, 0);
    wr::<u64>(iter_ptr + 0x30, 0);
    if span_out[1] != span_out[2] {
        let arch = rd::<u16>(span_out[1] as usize) as usize;
        let blk = rd::<usize>(r11.wrapping_add(arch * 8 + 0x48));
        wr::<usize>(iter_ptr + 0x20, blk);
        let cnt = if blk != 0 {
            rd::<u32>(r11.wrapping_add(arch * 4 + 0x10048)) as u64
        } else {
            0
        };
        wr::<u64>(iter_ptr + 0x28, cnt);
    }

    exe::call_va!(0x1_4105_7ed0, (iter_ptr));
    let mut r13 = rd::<usize>(iter_ptr + 0x30);
    while r13 < rd::<usize>(iter_ptr + 0x28) {
        while r13 < rd::<usize>(iter_ptr + 0x28) {
            let col_a = rd::<usize>(iter_ptr + 0x38);
            let col_b = rd::<usize>(iter_ptr + 0x40);
            let key = rd::<u64>(col_a.wrapping_add(r13 * 8));
            if key != 0 {
                let comp = rd::<usize>(col_b.wrapping_add(r13 * 8));
                let cnt_a = rd::<u32>(comp + 0x24) as usize;
                let ptr_a = comp.wrapping_add(rd::<u32>(comp + 0x20) as usize);
                let cnt_b = rd::<u32>(comp + 0x2c) as usize;
                let ptr_b = comp.wrapping_add(rd::<u32>(comp + 0x28) as usize);
                if cnt_a != 0 || cnt_b != 0 {
                    let head = rd::<usize>(param_2);
                    let node = rbtree_lower_bound(head, key);
                    let end_a = ptr_a.wrapping_add(cnt_a * 8);
                    let mut cur_a = ptr_a;
                    if cur_a != end_a {
                        let ent_slot = rd::<usize>(iter_ptr + 0x20).wrapping_add(r13 * 8);
                        while cur_a != end_a {
                            let val_a = rd::<u64>(cur_a);
                            let ent_id = rd::<u64>(ent_slot + 0x10);
                            push_node_pair(node, val_a, ent_id);
                            cur_a = cur_a.wrapping_add(8);
                        }
                    }
                    let end_b = ptr_b.wrapping_add(cnt_b * 8);
                    let mut cur_b = ptr_b;
                    while cur_b != end_b {
                        let val_b = rd::<u64>(cur_b);
                        let ent_id =
                            rd::<u64>(rd::<usize>(iter_ptr + 0x20).wrapping_add(r13 * 8 + 0x10));
                        push_node_pair(node, val_b, ent_id);
                        cur_b = cur_b.wrapping_add(8);
                    }
                }
            }
            r13 += 1;
            wr::<u64>(iter_ptr + 0x30, r13 as u64);
        }
        advance_arch_iter(iter_ptr);
        exe::call_va!(0x1_4105_7ed0, (iter_ptr));
        r13 = rd::<usize>(iter_ptr + 0x30);
    }
}

/// `unknown_141f2b970`
/// Original: 0x141f2b970 (528 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f2b970(param_1: usize, param_2: usize, param_3: usize) {
    let p0 = rd::<usize>(param_1);
    let p1 = rd::<usize>(param_1 + 0x08);
    let p2 = rd::<usize>(param_1 + 0x10);
    let idx = rd::<usize>(param_1 + 0x20);

    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let half = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);

    let acc_addr = p1.wrapping_add(idx * 4);
    let acc = rd::<f32>(param_2) * rd::<f32>(param_3) + rd::<f32>(acc_addr);
    wr::<f32>(acc_addr, acc);

    let cfg = p0.wrapping_add(idx.wrapping_shl(5));
    let center = rd::<f32>(cfg + 0x0c);
    let width = rd::<f32>(cfg + 0x10);
    let half_w = width * half;
    let arg_xmm0 = acc * rd::<f32>(cfg + 0x08);

    let hi_raw = half_w + center;
    let hi = if one < hi_raw { one } else { hi_raw };
    let lo_raw = center - half_w;
    let lo = if 0.0f32 > lo_raw { 0.0f32 } else { lo_raw };

    let (_, ret_xmm0) = crate::simd_ops::call_va8_xmm(
        0x1_4243_be50,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [arg_xmm0.to_bits() as u128, 0, 0, 0],
    );
    let noise_val = f32::from_bits(ret_xmm0 as u32);

    let mut out = lerp_rcp(noise_val, lo, 0.0f32, hi, one);
    let out_min = rd::<f32>(cfg + 0x14);
    if out_min >= 0.0f32 {
        let out_max = rd::<f32>(cfg + 0x18);
        if out_max > 0.0f32 {
            out = lerp_rcp(out, 0.0f32, out_min, one, out_max);
        }
    }
    wr::<f32>(p2.wrapping_add(idx * 4), out);
}

/// `unknown_141f2c080`
/// Original: 0x141f2c080 (250 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f2c080(param_1: usize, param_2: usize, param_3: usize) {
    let p0 = rd::<usize>(param_1);
    let p1 = rd::<usize>(param_1 + 0x08);
    let p2 = rd::<usize>(param_1 + 0x10);
    let p3 = rd::<usize>(param_1 + 0x18);
    let r11 = rd::<usize>(param_1 + 0x20);
    let idx = rd::<usize>(param_1 + 0x30);

    let off_14 = idx.wrapping_mul(0x14);
    let off_20 = idx.wrapping_shl(5);
    let off_40 = idx.wrapping_shl(6);

    let rbx = p0.wrapping_add(off_14);
    let rsi = p1.wrapping_add(off_40);
    let rdi = p2.wrapping_add(off_14);
    let r9 = p3.wrapping_add(off_20);

    let dt = rd::<f32>(param_2) * rd::<f32>(param_3);
    let dt_xmm1 = dt.to_bits() as u128;

    if rd::<u8>(rsi) != 0 {
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4108_4f10,
            rsi as u64,
            0,
            r9 as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dt_xmm1, 0, 0],
        );
        wr::<u128>(r11.wrapping_add(off_20), rd::<u128>(r9));
    }
    if rd::<u8>(rdi) != 0 {
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4103_93f0,
            rdi as u64,
            0,
            (r9 + 0x10) as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dt_xmm1, 0, 0],
        );
        wr::<u32>(r11.wrapping_add(off_20 + 0x10), rd::<u32>(r9 + 0x10));
    }
    if rd::<u8>(rbx) != 0 {
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_4103_93f0,
            rbx as u64,
            0,
            (r9 + 0x14) as u64,
            0,
            0,
            0,
            0,
            0,
            [0, dt_xmm1, 0, 0],
        );
        wr::<u32>(r11.wrapping_add(off_20 + 0x14), rd::<u32>(r9 + 0x14));
    }
}

#[inline(always)]
unsafe fn copy_construct_entry_28(dst: usize, src: usize) {
    wr::<u64>(dst, 0);
    wr::<u64>(dst + 0x08, 0);
    wr::<u32>(dst + 0x10, 0);
    wr::<u8>(dst + 0x14, 0);
    exe::call_va!(0x1_401e_84a0, (dst, src));
    wr::<u8>(dst + 0x18, rd::<u8>(src + 0x18));
    wr::<u8>(dst + 0x19, rd::<u8>(src + 0x19));
    wr::<u32>(dst + 0x1c, rd::<u32>(src + 0x1c));
    wr::<u32>(dst + 0x20, rd::<u32>(src + 0x20));
    wr::<u32>(dst + 0x24, rd::<u32>(src + 0x24));
}

#[inline(always)]
unsafe fn push_entry_28(vec_hdr: usize, src: usize) {
    let mut base = rd::<usize>(vec_hdr);
    let mut len = rd::<u32>(vec_hdr + 0x08);
    let cap = rd::<u32>(vec_hdr + 0x0c);
    if len == cap {
        exe::call_va!(0x1_410d_6540, (vec_hdr, (len as u64).wrapping_add(1)));
        base = rd::<usize>(vec_hdr);
        len = rd::<u32>(vec_hdr + 0x08);
    }
    let dst = base.wrapping_add((len as usize) * 0x28);
    copy_construct_entry_28(dst, src);
    wr::<u32>(vec_hdr + 0x08, len.wrapping_add(1));
}

/// `unknown_141f5aa00`
/// Original: 0x141f5aa00 (1330 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f5aa00(
    _param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u8,
) {
    let mut vec_28: [u64; 2] = [0; 2];
    let mut vec_18: [u64; 2] = [0; 2];
    let v28 = vec_28.as_mut_ptr() as usize;
    let v18 = vec_18.as_mut_ptr() as usize;

    let cnt3 = rd::<u32>(param_3 + 0x08) as usize;
    let base3 = rd::<usize>(param_3);
    let cnt2 = rd::<u32>(param_2 + 0x08) as usize;
    let base2 = rd::<usize>(param_2);

    if cnt2 != 0 {
        exe::call_va!(0x1_410d_6540, (v28, cnt2 as u64));
    }

    let end2 = base2.wrapping_add(cnt2 * 0x28);
    let mut cur2 = base2;
    while cur2 != end2 {
        let want = rd::<u64>(cur2);
        let mut found = false;
        for i in 0..cnt3 {
            if rd::<u64>(base3.wrapping_add(i * 0x28)) == want {
                found = true;
                break;
            }
        }
        if !found {
            push_entry_28(v28, cur2);
        }
        cur2 = cur2.wrapping_add(0x28);
    }

    if cnt3 != 0 {
        exe::call_va!(0x1_410d_6540, (v18, cnt3 as u64));
    }

    let end3 = base3.wrapping_add(cnt3 * 0x28);
    let mut cur3 = base3;
    while cur3 != end3 {
        let want = rd::<u64>(cur3);
        let mut found = false;
        for i in 0..cnt2 {
            if rd::<u64>(base2.wrapping_add(i * 0x28)) == want {
                found = true;
                break;
            }
        }
        if !found {
            push_entry_28(v18, cur3);
        }
        cur3 = cur3.wrapping_add(0x28);
    }

    let p3_base = rd::<usize>(param_3);
    let p3_cnt = rd::<u32>(param_3 + 0x08) as usize;
    let p3_end = p3_base.wrapping_add(p3_cnt * 0x28);
    let mut it3 = p3_base;
    while it3 != p3_end {
        let p2_cnt = rd::<u32>(param_2 + 0x08) as usize;
        let p2_base = rd::<usize>(param_2);
        let want = rd::<u64>(it3);
        for i in 0..p2_cnt {
            let off = i * 0x28;
            if rd::<u64>(p2_base.wrapping_add(off)) == want {
                let state = rd::<u32>(p2_base.wrapping_add(off + 0x20));
                if (state.wrapping_sub(1) & 0xffff_fffd) == 0 {
                    wr::<u32>(p2_base.wrapping_add(off + 0x20), 0);
                }
                let p2_cur = rd::<usize>(param_2);
                let new_b18 = rd::<u8>(it3 + 0x18);
                if rd::<u8>(p2_cur.wrapping_add(off + 0x18)) != new_b18 {
                    wr::<u8>(p2_cur.wrapping_add(off + 0x18), new_b18);
                    let p2_cur2 = rd::<usize>(param_2);
                    if rd::<u32>(p2_cur2.wrapping_add(off + 0x20)) == 0 {
                        wr::<u32>(p2_cur2.wrapping_add(off + 0x20), 2);
                    }
                }
                break;
            }
        }
        it3 = it3.wrapping_add(0x28);
    }

    let v28_base = rd::<usize>(v28);
    let v28_cnt = rd::<u32>(v28 + 0x08) as usize;
    let v28_end = v28_base.wrapping_add(v28_cnt * 0x28);
    let mut it28 = v28_base;
    while it28 != v28_end {
        let p2_cnt = rd::<u32>(param_2 + 0x08) as usize;
        let p2_base = rd::<usize>(param_2);
        let want = rd::<u64>(it28);
        for i in 0..p2_cnt {
            let elem = p2_base.wrapping_add(i * 0x28);
            if rd::<u64>(elem) == want {
                if rd::<u32>(elem + 0x1c) <= 2 {
                    wr::<u32>(elem + 0x20, (param_4 as u32) * 2 + 1);
                }
                break;
            }
        }
        it28 = it28.wrapping_add(0x28);
    }

    let v18_base = rd::<usize>(v18);
    let v18_cnt = rd::<u32>(v18 + 0x08) as usize;
    let v18_end = v18_base.wrapping_add(v18_cnt * 0x28);
    let old_p2_cnt = rd::<u32>(param_2 + 0x08) as usize;
    let mut end_pos = rd::<usize>(param_2).wrapping_add(old_p2_cnt * 0x28);
    exe::call_va!(
        0x1_410d_6090,
        (
            param_2,
            (&mut end_pos as *mut usize) as usize,
            v18_cnt as u64
        )
    );
    if v18_base != v18_end {
        let mut dst_off = old_p2_cnt * 0x28;
        let mut src = v18_base;
        while src != v18_end {
            let dst = rd::<usize>(param_2).wrapping_add(dst_off);
            copy_construct_entry_28(dst, src);
            dst_off += 0x28;
            src = src.wrapping_add(0x28);
        }
    }

    let mut clean28 = rd::<usize>(v28);
    let end_clean28 = clean28.wrapping_add((rd::<u32>(v28 + 0x08) as usize) * 0x28);
    while clean28 != end_clean28 {
        let _ = crate::simd_ops::atomic_xchg_u64((clean28 + 0x08) as u64, 0);
        clean28 = clean28.wrapping_add(0x28);
    }
    let free28 = rd::<usize>(v28);
    if free28 != 0 {
        exe::call_va!(0x1_4241_c190, (free28));
    }

    let mut clean18 = rd::<usize>(v18);
    while clean18 != v18_end {
        let _ = crate::simd_ops::atomic_xchg_u64((clean18 + 0x08) as u64, 0);
        clean18 = clean18.wrapping_add(0x28);
    }
    let free18 = rd::<usize>(v18);
    if free18 != 0 {
        exe::call_va!(0x1_4241_c190, (free18));
    }
}

#[inline(always)]
unsafe fn cleanup_5b3f0_args(param_3: usize, param_6: usize) {
    let p3 = rd::<usize>(param_3);
    if p3 != 0 {
        exe::call_va!(0x1_4244_2de0, (p3));
    }
    let p6 = rd::<usize>(param_6);
    if p6 != 0 {
        exe::call_va!(0x1_411d_7a20, (p6 + 0x98));
        exe::call_va!(0x1_411d_7a20, (p6 + 0x88));
        let dtor = exe::ptr::<u8>(0x1_401d_e400) as usize;
        exe::call_va!(0x1_4183_4ee8, (p6 + 0x08, 0x10u32, 0x08u32, dtor));
        exe::call_va!(0x1_4244_2de0, (p6));
    }
}

#[inline(always)]
unsafe fn destroy_5b3f0_slot_obj(obj: usize) {
    if obj != 0 {
        let p48 = rd::<usize>(obj + 0x48);
        if p48 != 0 {
            exe::call_va!(0x1_4244_2de0, (p48));
        }
        exe::call_va!(0x1_4240_ab10, (obj));
        exe::call_va!(0x1_4244_2de0, (obj));
    }
}

/// `unknown_14205b3f0`
/// Original: 0x14205b3f0 (759 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14205b3f0(
    param_1: u32,
    param_2: usize,
    param_3: usize,
    param_4: u64,
    param_5: u64,
    param_6: usize,
    param_7: usize,
    param_8: usize,
    param_9: u32,
) -> u64 {
    let lock_ptr = exe::ptr::<u8>(0x1_43a3_2f28) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_ptr));

    let vec_hdr = (exe::ptr::<u8>(0x1_43a6_3260) as usize)
        .wrapping_add(((param_1 as i32 as isize) * 16) as usize);
    let len = rd::<u32>(vec_hdr + 0x08) as usize;
    let mut cur = rd::<usize>(vec_hdr);
    let mut slot_idx = len;

    if len != 0 {
        for i in 0..len {
            let key = rd::<u64>(cur + 0x90);
            if key == param_5 {
                wr::<u32>(cur + 0x98, rd::<u32>(cur + 0x98).wrapping_add(1));
                let gen = rd::<u16>(cur + 0x9c) as u64;
                let handle =
                    (((gen << 16) | ((param_1 & 0xffff) as u64)) << 32) | (i as u32 as u64);
                exe::call_iat!(0x1_4264_5598, (lock_ptr));
                cleanup_5b3f0_args(param_3, param_6);
                return handle;
            }
            if key == 0 && rd::<u64>(cur + 0x08) == 0 {
                slot_idx = i;
            }
            cur = cur.wrapping_add(0xa0);
        }
        if slot_idx >= len {
            slot_idx = len;
            exe::call_va!(0x1_411d_7000, (vec_hdr));
        }
    } else {
        slot_idx = 0;
        exe::call_va!(0x1_411d_7000, (vec_hdr));
    }

    let slot = rd::<usize>(vec_hdr).wrapping_add(slot_idx * 0xa0);
    wr::<[u64; 13]>(slot + 0x18, rd::<[u64; 13]>(param_7));
    wr::<u64>(slot + 0x80, rd::<u64>(param_8));
    wr::<u8>(slot + 0x88, rd::<u8>(param_8 + 0x08));
    wr::<u64>(slot + 0x90, param_5);
    wr::<u32>(slot + 0x98, 1);

    let mut local_48: usize = 0;
    let mut local_40: u32 = 0;
    let mut p1_slot: u32 = param_1;
    let mut p4_slot: u64 = param_4;
    let mut p9_slot: u32 = param_9;

    let ret_ptr = exe::call_va!(
        0x1_411d_7140,
        (
            (&mut local_48 as *mut usize) as usize,
            0usize,
            param_3,
            (&mut p4_slot as *mut u64) as usize,
            (&mut p1_slot as *mut u32) as usize,
            (&mut local_40 as *mut u32) as usize,
            param_2,
            (&mut p9_slot as *mut u32) as usize
        ) -> usize
    );

    let new_obj = rd::<usize>(ret_ptr);
    wr::<usize>(ret_ptr, 0);
    let old_obj = rd::<usize>(slot);
    wr::<usize>(slot, new_obj);
    destroy_5b3f0_slot_obj(old_obj);
    destroy_5b3f0_slot_obj(local_48);

    exe::call_va!(0x1_411d_6f90, (slot + 0x10, param_6));
    exe::call_va!(0x1_4209_bb20, (rd::<usize>(slot + 0x10)));

    let gen = rd::<u16>(slot + 0x9c) as u64;
    let handle = (((gen << 16) | ((param_1 & 0xffff) as u64)) << 32) | (slot_idx as u32 as u64);
    exe::call_iat!(0x1_4264_5598, (lock_ptr));
    cleanup_5b3f0_args(param_3, param_6);
    handle
}

/// `r_1420abdc0`
/// Original: 0x1420abdc0 (368 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420abdc0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u8,
) {
    let r10 = rd::<usize>(param_2);
    let r11_off = (param_4 as usize) * 0x28;
    let mut r9 = rd::<usize>(r10 + r11_off + 0x18);
    let outer_cnt = rd::<u32>(r10 + r11_off + 0x20) as usize;
    let outer_end = r9.wrapping_add(outer_cnt.wrapping_shl(6));

    let mut rbx = r9;
    let mut r14: usize = 0;
    let mut rdx: usize = 0;
    if r9 != outer_end {
        loop {
            rdx = rd::<usize>(r9);
            r14 = r9;
            let cnt = rd::<u32>(r9 + 0x08) as usize;
            if rdx != rdx.wrapping_add(cnt * 2) {
                break;
            }
            r9 = r9.wrapping_add(0x40);
            rbx = r9;
            if r9 == outer_end {
                break;
            }
        }
    }

    if rbx != outer_end {
        loop {
            let item_idx = rd::<u16>(rdx) as usize;
            let entry = rd::<usize>(param_3 + 0x08).wrapping_add(item_idx * 0x130);
            let mut sub_cur = rd::<usize>(entry + 0x100);
            let sub_end = sub_cur.wrapping_add((rd::<u32>(entry + 0x108) as usize).wrapping_shl(4));
            while sub_cur != sub_end {
                let slot_id = rd::<u16>(sub_cur + 0x08) as usize;
                let ref_addr = rd::<usize>(param_1 + 0x5e308)
                    .wrapping_add(slot_id.wrapping_shl(4))
                    .wrapping_add(0x08);
                let next_ref = rd::<u64>(ref_addr).wrapping_sub(1);
                wr::<u64>(ref_addr, next_ref);
                if next_ref == 0 {
                    let bit_id = rd::<u16>(sub_cur + 0x08) as usize;
                    let word_addr =
                        rd::<usize>(param_1 + 0x5e2f0).wrapping_add((bit_id >> 6) * 8);
                    let mask = !(1u64 << (bit_id & 0x3f));
                    wr::<u64>(word_addr, rd::<u64>(word_addr) & mask);
                }
                sub_cur = sub_cur.wrapping_add(0x10);
            }

            let cur_cnt = rd::<u32>(r14 + 0x08) as usize;
            rdx = rdx.wrapping_add(2);
            let inner_end = rd::<usize>(r14).wrapping_add(cur_cnt * 2);
            if rdx == inner_end {
                let cur_outer_end = rd::<usize>(r10 + r11_off + 0x18)
                    .wrapping_add((rd::<u32>(r10 + r11_off + 0x20) as usize).wrapping_shl(6));
                r9 = r9.wrapping_add(0x40);
                if r9 != cur_outer_end {
                    loop {
                        rdx = rd::<usize>(r9);
                        r14 = r9;
                        let cnt2 = rd::<u32>(r9 + 0x08) as usize;
                        if rdx != rdx.wrapping_add(cnt2 * 2) {
                            break;
                        }
                        r9 = r9.wrapping_add(0x40);
                        if r9 == cur_outer_end {
                            break;
                        }
                    }
                }
            }

            let end_check = rd::<usize>(r10 + r11_off + 0x18)
                .wrapping_add((rd::<u32>(r10 + r11_off + 0x20) as usize).wrapping_shl(6));
            if r9 == end_check {
                break;
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141da1810(orig: usize, new: usize) -> Result<u32, String> {
    static mut DTOR_LOG: u64 = 0;

    unsafe extern "win64" fn mock_get_x(_this: usize, elem: usize) -> f32 {
        let v = rd::<f32>(elem);
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_get_y(_this: usize, elem: usize) -> f32 {
        let v = rd::<f32>(elem + 0x04);
        crate::simd_ops::set_xmm_ret(v.to_bits() as u128);
        v
    }
    unsafe extern "win64" fn mock_dtor_vfunc(this: usize, is_heap: u8) {
        let tag = rd::<u32>(this + 0x08) as u64;
        DTOR_LOG = DTOR_LOG
            .wrapping_mul(131)
            .wrapping_add((tag << 8) | (is_heap as u64));
    }

    let mut vtbl_x = [0usize; 6];
    vtbl_x[2] = mock_get_x as *const () as usize;
    vtbl_x[4] = mock_dtor_vfunc as *const () as usize;

    let mut vtbl_y = [0usize; 6];
    vtbl_y[2] = mock_get_y as *const () as usize;
    vtbl_y[4] = mock_dtor_vfunc as *const () as usize;

    let pts: [[f32; 2]; 3] = [[1.0, 10.0], [3.0, 30.0], [5.0, 70.0]];

    let invoke = |fn_addr: usize, x: f32, cnt: u64| -> (u32, u64, usize, usize) {
        DTOR_LOG = 0;
        let span: [u64; 2] = [cnt, pts.as_ptr() as usize as u64];
        let def_val: f32 = 42.5;

        let mut fn3 = [0u64; 8];
        let f3p = fn3.as_mut_ptr() as usize;
        wr::<usize>(f3p, vtbl_x.as_ptr() as usize);
        wr::<u32>(f3p + 0x08, 1);
        wr::<usize>(f3p + 0x38, f3p);

        let mut fn4 = [0u64; 8];
        let f4p = fn4.as_mut_ptr() as usize;
        wr::<usize>(f4p, vtbl_y.as_ptr() as usize);
        wr::<u32>(f4p + 0x08, 2);
        wr::<usize>(f4p + 0x38, f4p);

        #[cfg(not(target_arch = "aarch64"))]
        let ret_bits = {
            type F = unsafe extern "win64" fn(f32, usize, usize, usize, usize) -> f32;
            let f: F = core::mem::transmute(fn_addr);
            f(x, span.as_ptr() as usize, f3p, f4p, (&def_val as *const f32) as usize).to_bits()
        };
        #[cfg(target_arch = "aarch64")]
        let ret_bits = {
            crate::simd_ops::call_va8_xmm(
                fn_addr as u64,
                0,
                span.as_ptr() as usize as u64,
                f3p as u64,
                f4p as u64,
                (&def_val as *const f32) as usize as u64,
                0,
                0,
                0,
                [x.to_bits() as u128, 0, 0, 0],
            )
            .1 as u32
        };

        (ret_bits, DTOR_LOG, rd::<usize>(f3p + 0x38), rd::<usize>(f4p + 0x38))
    };

    for (x, cnt) in [(2.0f32, 0u64), (2.0, 1), (0.5, 3), (6.0, 3), (2.0, 3), (4.0, 3)] {
        let o = invoke(orig, x, cnt);
        let n = invoke(new, x, cnt);
        crate::diff_eq!(o, n, "141da1810 x={} cnt={}", x, cnt);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_141e791b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    fn mix(v: u64) {
        unsafe {
            TRACE_HASH = TRACE_HASH.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_5ef0(p: usize) {
        mix(0x5ef0 ^ (rd::<u32>(p) as u64));
    }
    unsafe extern "win64" fn mock_8c80(_p1: usize) {
        mix(0x8c80);
    }
    unsafe extern "win64" fn mock_0e40(_base: usize, idx: u32) {
        mix(0x0e40 ^ (idx as u64));
    }
    unsafe extern "win64" fn mock_d670(slot: usize) {
        mix(0xd670 ^ (rd::<u32>(slot + 0x08) as u64));
    }
    unsafe extern "win64" fn mock_d710(ptr: u64, pair_ptr: usize, _dst: usize) {
        let cnt = rd::<u64>(pair_ptr);
        mix(0xd710 ^ ptr ^ cnt);
    }
    unsafe extern "win64" fn mock_6af0(_v: usize, beg_p: usize, end_p: usize) {
        let diff = rd::<usize>(end_p).wrapping_sub(rd::<usize>(beg_p));
        mix(0x6af0 ^ (diff as u64));
    }
    unsafe extern "win64" fn mock_6060(_c: usize, _v: usize) {
        mix(0x6060);
    }
    unsafe extern "win64" fn mock_8340(_c460: usize, out_pair: usize) {
        wr::<usize>(out_pair, 0x7777);
        wr::<usize>(out_pair + 0x08, 0);
        mix(0x8340);
    }
    unsafe extern "win64" fn mock_6fa0(_vec: usize, new_cap: u64) {
        mix(0x6fa0 ^ new_cap);
    }
    unsafe extern "win64" fn mock_3c50(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        mix(0x3c50 ^ rd::<u64>(src));
    }
    unsafe extern "win64" fn mock_5dc0(id: u32, f100: usize, sp_arg: usize) {
        mix(
            0x5dc0
                ^ (id as u64)
                ^ (rd::<u32>(f100) as u64)
                ^ (rd::<u32>(f100 + 0x04) as u64)
                ^ (rd::<u16>(f100 + 0x08) as u64)
                ^ rd::<u64>(sp_arg + 0x10),
        );
    }
    unsafe extern "win64" fn mock_6ca0(ff58: usize) {
        mix(0x6ca0 ^ (ff58 as u64));
    }
    unsafe extern "win64" fn mock_5c10(ff58: usize, pkt: usize) {
        mix(0x5c10 ^ (ff58 as u64) ^ rd::<u64>(pkt) ^ rd::<u64>(pkt + 0x20));
    }
    unsafe extern "win64" fn mock_62c0(_c: usize, str_p: usize) {
        let s = rd::<usize>(str_p);
        mix(0x62c0 ^ (rd::<u8>(s) as u64));
    }
    unsafe extern "win64" fn mock_d7e0(_c: usize) {
        mix(0xd7e0);
    }
    unsafe extern "win64" fn mock_6de0(ff58: usize, _p1: usize) {
        mix(0x6de0 ^ (ff58 as u64));
    }
    unsafe extern "win64" fn mock_64b0(ff58: usize) {
        mix(0x64b0 ^ (ff58 as u64));
    }
    unsafe extern "win64" fn mock_d950(_c: usize, id_p: usize) {
        mix(0xd950 ^ rd::<u64>(id_p));
    }
    unsafe extern "win64" fn mock_8050(_v: usize) {
        mix(0x8050);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fe_5ef0, mock_5ef0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e7_8c80, mock_8c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4224_0e40, mock_0e40 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4249_d670, mock_d670 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4249_d710, mock_d710 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40fe_6af0, mock_6af0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40fe_6060, mock_6060 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41dc_8340, mock_8340 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_40fe_6fa0, mock_6fa0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_40f3_3c50, mock_3c50 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_422a_5dc0, mock_5dc0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_40fe_6ca0, mock_6ca0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_40f3_5c10, mock_5c10 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_40fe_62c0, mock_62c0 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_40fb_d7e0, mock_d7e0 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_40fe_6de0, mock_6de0 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_40fe_64b0, mock_64b0 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_40fb_d950, mock_d950 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_403b_8050, mock_8050 as *const ())?;

    let g_fef8 = exe::ptr::<usize>(0x1_4397_fef8) as usize;
    let g_d9b8 = exe::ptr::<u32>(0x1_4397_d9b8) as usize;
    let g_8460 = exe::ptr::<usize>(0x1_43c4_8460) as usize;
    let g_690 = exe::ptr::<u64>(0x1_4387_9690) as usize;
    let saved = (
        rd::<usize>(g_fef8),
        rd::<u32>(g_d9b8),
        rd::<usize>(g_8460),
        rd::<u64>(g_690),
    );

    let mut fef8_buf = vec![1u8; 0x13 * 0xd8];
    fef8_buf[2 * 0xd8] = 0;
    wr::<usize>(g_fef8, fef8_buf.as_ptr() as usize);

    let mut c460_obj = [0u8; 0x40];
    wr::<u32>(c460_obj.as_mut_ptr() as usize + 0x28, 2);
    wr::<usize>(g_8460, c460_obj.as_ptr() as usize);
    wr::<u32>(g_d9b8, 1);

    let run = |f: F| -> (u64, u8, u32, u8, u32, u64) {
        TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        wr::<u64>(g_690, 100);

        let mut vec_slots = [0u64; 8];
        let mut p1 = vec![0u8; 0x700];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x0c, 7);
        wr::<u8>(p1p + 0x10, 1);
        wr::<u8>(p1p + 0x11, 1);
        wr::<u32>(p1p + 0x260, (4u32 << 24) | 2);
        wr::<u32>(p1p + 0x264, 2);
        wr::<u8>(p1p + 0x268, b'A');
        wr::<u8>(p1p + 0x378, 5);
        wr::<u8>(p1p + 0x379, 1);
        wr::<u64>(p1p + 0x380, 0x1234_5678);
        wr::<usize>(p1p + 0x680, vec_slots.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x688, 0);
        wr::<u32>(p1p + 0x68c, 2);

        f(p1p);
        (
            TRACE_HASH,
            rd::<u8>(p1p + 0x11),
            rd::<u32>(p1p + 0x264),
            rd::<u8>(p1p + 0x268),
            rd::<u32>(p1p + 0x688),
            vec_slots[2],
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<usize>(g_fef8, saved.0);
    wr::<u32>(g_d9b8, saved.1);
    wr::<usize>(g_8460, saved.2);
    wr::<u64>(g_690, saved.3);

    crate::diff_eq!(o, n, "141e791b0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e7b330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_9e40(ptr: u64, len: u32, hash: u32) -> u32 {
        let mut h_soft: u32 = 0x811c_9dc5;
        for &b in b"SoftEyesDisabled" {
            h_soft = (((b as i8 as i32 as u32) | 0x20) ^ h_soft).wrapping_mul(0x0100_0193);
        }
        if ptr == 0xaaa0 && len == 4 && hash == h_soft {
            1
        } else {
            u32::MAX
        }
    }
    unsafe extern "win64" fn mock_c290(
        slot: usize,
        arg_rdx: u64,
        arg_r8: u64,
        arg_r9: u32,
        changed: u8,
    ) {
        let idx = rd::<u32>(slot) as usize;
        CALL_LOG[idx * 4] = arg_rdx;
        CALL_LOG[idx * 4 + 1] = arg_r8;
        CALL_LOG[idx * 4 + 2] = arg_r9 as u64;
        CALL_LOG[idx * 4 + 3] = changed as u64;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9e40, mock_9e40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421b_c290, mock_c290 as *const ())?;

    let g_f40 = exe::ptr::<u8>(0x1_4385_0f40) as usize;
    let s_f40 = rd::<u8>(g_f40);
    wr::<u8>(g_f40, 1);

    let run = |f: F| -> ([u64; 8], u32, u32) {
        CALL_LOG = [0; 8];

        let p20: [u64; 4] = [0x1111, 0, 0x2222, 0];
        let p30: [u64; 4] = [0x3333, 0, 0x4444, 0];
        let mut p28 = [0u8; 0x60];
        let p28p = p28.as_mut_ptr() as usize;
        wr::<u32>(p28p, 0);
        wr::<u32>(p28p + 0x28, 3);
        wr::<u32>(p28p + 0x30, 1);
        wr::<u32>(p28p + 0x58, 9);

        let p38: [u32; 4] = [0x0f, 0x10, 0x0f, 0x20];
        let floats: [f32; 2] = [0.0, 1.5];

        let mut p40 = [0u8; 0x90];
        let p40p = p40.as_mut_ptr() as usize;
        wr::<u16>(p40p, 1);
        wr::<usize>(p40p + 0x18, floats.as_ptr() as usize);
        wr::<u64>(p40p + 0x28, 0xaaa0);
        wr::<u32>(p40p + 0x30, 4);

        wr::<u16>(p40p + 0x48, 1);
        wr::<usize>(p40p + 0x48 + 0x18, floats.as_ptr() as usize);
        wr::<u64>(p40p + 0x48 + 0x28, 0xbbb0);
        wr::<u32>(p40p + 0x48 + 0x30, 4);

        let mut p48 = [0u8; 0x18];
        let p48p = p48.as_mut_ptr() as usize;
        wr::<i32>(p48p, 10);
        wr::<u8>(p48p + 0x08, 2);
        wr::<u8>(p48p + 0x09, 6);
        wr::<i32>(p48p + 0x0c, 10);
        wr::<u8>(p48p + 0x14, 2);
        wr::<u8>(p48p + 0x15, 6);

        let p50: [u32; 2] = [3, 8];
        let p58: [u32; 2] = [0, 0];

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 2);
        wr::<usize>(p1p + 0x20, p20.as_ptr() as usize);
        wr::<usize>(p1p + 0x28, p28p);
        wr::<usize>(p1p + 0x30, p30.as_ptr() as usize);
        wr::<usize>(p1p + 0x38, p38.as_ptr() as usize);
        wr::<usize>(p1p + 0x40, p40p);
        wr::<usize>(p1p + 0x48, p48p);
        wr::<usize>(p1p + 0x50, p50.as_ptr() as usize);
        wr::<usize>(p1p + 0x58, p58.as_ptr() as usize);

        f(p1p);
        (CALL_LOG, rd::<u32>(p28p + 0x28), rd::<u32>(p28p + 0x58))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u8>(g_f40, s_f40);

    crate::diff_eq!(o, n, "141e7b330 out");
    Ok(1)
}

pub unsafe fn diff_coregame_141e9fa80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut sub = [0u8; 0x2b0];
    let sub_p = sub.as_mut_ptr() as usize;

    let mut tests = 0u32;
    for b_e8 in [0u8, 1] {
        for v_ec in [0u32, 1, 2, 3, 4] {
            for b_2a0 in [0u8, 1] {
                wr::<u8>(sub_p + 0x2a0, b_2a0);
                let mut obj = [0u8; 0x100];
                let p = obj.as_mut_ptr() as usize;
                wr::<usize>(p + 0x08, sub_p);
                wr::<u8>(p + 0xe8, b_e8);
                wr::<u32>(p + 0xec, v_ec);

                let o = f_o(p);
                let n = f_n(p);
                crate::diff_eq!(o, n, "141e9fa80 e8={} ec={} 2a0={}", b_e8, v_ec, b_2a0);
                tests += 1;
            }
        }
    }
    Ok(tests)
}

pub unsafe fn diff_unknown_141eac250(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_LIST: [u16; 2] = [1, 2];
    static mut COL_A: [u64; 2] = [10, 0];
    static mut COL_B: [u64; 2] = [0x5555, u64::MAX];

    unsafe extern "win64" fn mock_7c10(out: usize, w40: usize, _in_span: usize) -> usize {
        let p = core::ptr::addr_of!(ARCH_LIST) as usize;
        wr::<usize>(out, w40);
        wr::<usize>(out + 0x08, p);
        wr::<usize>(out + 0x10, p + 4);
        out
    }
    unsafe extern "win64" fn mock_83a0(iter: usize) {
        wr::<usize>(iter + 0x38, core::ptr::addr_of!(COL_A) as usize);
        wr::<usize>(iter + 0x40, core::ptr::addr_of!(COL_B) as usize);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_83a0, mock_83a0 as *const ())?;

    let run = |f: F| -> ([u64; 4], u32, [u64; 4], u32) {
        let mut blk1 = [0u64; 8];
        blk1[2] = 0xaaaa_0001;
        blk1[3] = 0xaaaa_0002;

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x40 + 8 + 0x48, blk1.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x40 + 4 + 0x10048, 2);

        let mut v1_buf = [0u64; 4];
        let mut v2_buf = [0u64; 4];

        let mut head = [0u8; 0x50];
        let mut node = [0u8; 0x50];
        let hp = head.as_mut_ptr() as usize;
        let np = node.as_mut_ptr() as usize;

        wr::<u8>(hp + 0x19, 1);
        wr::<usize>(hp + 0x08, np);

        wr::<usize>(np, hp);
        wr::<usize>(np + 0x08, hp);
        wr::<usize>(np + 0x10, hp);
        wr::<u8>(np + 0x19, 0);
        wr::<u64>(np + 0x20, 10);
        wr::<usize>(np + 0x28, v1_buf.as_mut_ptr() as usize);
        wr::<u32>(np + 0x30, 0);
        wr::<u32>(np + 0x34, 4);
        wr::<usize>(np + 0x38, v2_buf.as_mut_ptr() as usize);
        wr::<u32>(np + 0x40, 0);
        wr::<u32>(np + 0x44, 4);

        let p1: [u64; 3] = [wp as u64, 1, 2];
        let p2: [u64; 2] = [hp as u64, 1];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        (v1_buf, rd::<u32>(np + 0x30), v2_buf, rd::<u32>(np + 0x40))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141eac250 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eac8c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_25e0(out: usize, rdx: usize, r8: usize) -> usize {
        CALL_COUNT += 1;
        let r9 = rd::<usize>(r8 + 0x08);
        let rax = rd::<usize>(rdx);
        let rcx = rd::<usize>(rax + 0x18);
        let tag = rd::<u32>(rcx + 0x04) ^ rd::<u32>(r9 + 0x208);
        wr::<u64>(out, 0x1234);
        wr::<u32>(out + 0x08, 3);
        wr::<u32>(out + 0x10, tag);
        out
    }
    unsafe extern "win64" fn mock_0e60(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u32>(dst + 0x08, rd::<u32>(src + 0x08));
    }
    unsafe extern "win64" fn mock_2bc0(tmp: usize) {
        wr::<u64>(tmp, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41eb_25e0, mock_25e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4033_0e60, mock_0e60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401f_2bc0, mock_2bc0 as *const ())?;

    let run = |f: F| -> ([u8; 0x30], u32) {
        CALL_COUNT = 0;

        let mut c_obj = [0u8; 0x20];
        wr::<u32>(c_obj.as_mut_ptr() as usize + 0x04, 0xaa00);
        let mut a_obj = [0u8; 0x20];
        wr::<usize>(a_obj.as_mut_ptr() as usize + 0x18, c_obj.as_ptr() as usize);

        let mut r9_obj = [0u8; 0x220];
        wr::<u32>(r9_obj.as_mut_ptr() as usize + 0x208, 0x0055);

        let mut arr20 = [0u8; 0x30];
        let a20p = arr20.as_mut_ptr() as usize;
        wr::<usize>(a20p + 0x08, r9_obj.as_ptr() as usize);
        wr::<usize>(a20p + 0x18 + 0x08, r9_obj.as_ptr() as usize);

        let arr28: [usize; 4] = [a_obj.as_ptr() as usize, 0, a_obj.as_ptr() as usize, 0];

        let mut arr30 = [0u8; 0x30];
        let a30p = arr30.as_mut_ptr() as usize;
        wr::<u32>(a30p + 0x10, 0xaa55); // matches -> skip
        wr::<u32>(a30p + 0x18 + 0x10, 0x1111); // mismatch -> rebuild

        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 2);
        wr::<usize>(p1p + 0x20, a20p);
        wr::<usize>(p1p + 0x28, arr28.as_ptr() as usize);
        wr::<usize>(p1p + 0x30, a30p);

        f(p1p);
        (arr30, CALL_COUNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141eac8c0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141ec6230(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_LIST: [u16; 1] = [1];
    static mut COL_A: [u64; 1] = [20];
    static mut COL_B: [usize; 1] = [0];

    unsafe extern "win64" fn mock_7c10(out: usize, w40: usize, _in_span: usize) -> usize {
        let p = core::ptr::addr_of!(ARCH_LIST) as usize;
        wr::<usize>(out, w40);
        wr::<usize>(out + 0x08, p);
        wr::<usize>(out + 0x10, p + 2);
        out
    }
    unsafe extern "win64" fn mock_7ed0(iter: usize) {
        wr::<usize>(iter + 0x38, core::ptr::addr_of!(COL_A) as usize);
        wr::<usize>(iter + 0x40, core::ptr::addr_of!(COL_B) as usize);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4105_7ed0, mock_7ed0 as *const ())?;

    let run = |f: F| -> ([u64; 4], u32, [u64; 4], u32) {
        let mut comp = [0u8; 0x50];
        let cp = comp.as_mut_ptr() as usize;
        wr::<u32>(cp + 0x20, 0x30);
        wr::<u32>(cp + 0x24, 2);
        wr::<u32>(cp + 0x28, 0x40);
        wr::<u32>(cp + 0x2c, 1);
        wr::<u64>(cp + 0x30, 0x101);
        wr::<u64>(cp + 0x38, 0x102);
        wr::<u64>(cp + 0x40, 0x201);
        COL_B[0] = cp;

        let mut blk1 = [0u64; 4];
        blk1[2] = 0xbbbb_7777;

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x40 + 8 + 0x48, blk1.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x40 + 4 + 0x10048, 1);

        let mut v1_buf = [0u64; 4];
        let mut v2_buf = [0u64; 4];

        let mut head = [0u8; 0x50];
        let mut node = [0u8; 0x50];
        let hp = head.as_mut_ptr() as usize;
        let np = node.as_mut_ptr() as usize;

        wr::<u8>(hp + 0x19, 1);
        wr::<usize>(hp + 0x08, np);

        wr::<usize>(np, hp);
        wr::<usize>(np + 0x08, hp);
        wr::<usize>(np + 0x10, hp);
        wr::<u8>(np + 0x19, 0);
        wr::<u64>(np + 0x20, 20);
        wr::<usize>(np + 0x28, v1_buf.as_mut_ptr() as usize);
        wr::<u32>(np + 0x30, 0);
        wr::<u32>(np + 0x34, 4);
        wr::<usize>(np + 0x38, v2_buf.as_mut_ptr() as usize);
        wr::<u32>(np + 0x40, 0);
        wr::<u32>(np + 0x44, 4);

        let p1: [u64; 3] = [wp as u64, 1, 2];
        let p2: [u64; 2] = [hp as u64, 1];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        (v1_buf, rd::<u32>(np + 0x30), v2_buf, rd::<u32>(np + 0x40))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141ec6230 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f2b970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, dt: f32, scale: f32, range_min: f32, range_max: f32| -> ( u32, u32 ) {
        let mut cfg = [0u8; 0x40];
        let cp = cfg.as_mut_ptr() as usize;
        let c1 = cp + 0x20;
        wr::<f32>(c1 + 0x08, 1.25);
        wr::<f32>(c1 + 0x0c, 0.5);
        wr::<f32>(c1 + 0x10, 0.4);
        wr::<f32>(c1 + 0x14, range_min);
        wr::<f32>(c1 + 0x18, range_max);

        let mut acc: [f32; 2] = [0.0, 0.25];
        let mut out: [f32; 2] = [0.0, 0.0];

        let mut p1 = [0u64; 5];
        p1[0] = cp as u64;
        p1[1] = acc.as_mut_ptr() as usize as u64;
        p1[2] = out.as_mut_ptr() as usize as u64;
        p1[4] = 1;

        f(
            p1.as_ptr() as usize,
            (&dt as *const f32) as usize,
            (&scale as *const f32) as usize,
        );
        (acc[1].to_bits(), out[1].to_bits())
    };

    for (dt, sc, rmin, rmax) in [
        (0.1f32, 2.0f32, 2.0f32, 5.0f32),
        (0.5, 1.5, -1.0, 5.0),
        (0.3, 0.5, 0.0, 0.0),
    ] {
        let o = run(f_o, dt, sc, rmin, rmax);
        let n = run(f_n, dt, sc, rmin, rmax);
        crate::diff_eq!(o, n, "141f2b970 dt={} sc={} rmin={} rmax={}", dt, sc, rmin, rmax);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141f2c080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, active_mask: u8| -> [u32; 8] {
        let mut p0 = [0u8; 0x30];
        let mut p1 = [0u8; 0x80];
        let mut p2 = [0u8; 0x30];
        let mut p3 = [0u32; 16];
        let mut r11 = [0u32; 16];

        let b0 = p0.as_mut_ptr() as usize + 0x14;
        let b1 = p1.as_mut_ptr() as usize + 0x40;
        let b2 = p2.as_mut_ptr() as usize + 0x14;
        let b3 = p3.as_mut_ptr() as usize + 0x20;

        wr::<u8>(b1, active_mask & 1);
        wr::<f32>(b1 + 0x20, 10.0);
        wr::<f32>(b1 + 0x24, 20.0);
        wr::<f32>(b1 + 0x28, 30.0);
        wr::<f32>(b1 + 0x2c, 40.0);
        wr::<f32>(b1 + 0x30, 2.0);
        wr::<f32>(b1 + 0x34, 0.25);

        wr::<u8>(b2, (active_mask >> 1) & 1);
        wr::<f32>(b2 + 0x04, 1.0);
        wr::<f32>(b2 + 0x08, 5.0);
        wr::<f32>(b2 + 0x0c, 2.0);
        wr::<f32>(b2 + 0x10, 0.25);

        wr::<u8>(b0, (active_mask >> 2) & 1);
        wr::<f32>(b0 + 0x04, 2.0);
        wr::<f32>(b0 + 0x08, 8.0);
        wr::<f32>(b0 + 0x0c, 2.0);
        wr::<f32>(b0 + 0x10, 0.5);

        wr::<f32>(b3, 1.0);
        wr::<f32>(b3 + 0x04, 2.0);
        wr::<f32>(b3 + 0x08, 3.0);
        wr::<f32>(b3 + 0x0c, 4.0);
        wr::<f32>(b3 + 0x10, 1.0);
        wr::<f32>(b3 + 0x14, 2.0);

        let dt: f32 = 0.25;
        let scale: f32 = 0.5;
        let hdr: [u64; 7] = [
            p0.as_mut_ptr() as usize as u64,
            p1.as_mut_ptr() as usize as u64,
            p2.as_mut_ptr() as usize as u64,
            p3.as_mut_ptr() as usize as u64,
            r11.as_mut_ptr() as usize as u64,
            0,
            1,
        ];

        f(
            hdr.as_ptr() as usize,
            (&dt as *const f32) as usize,
            (&scale as *const f32) as usize,
        );
        rd::<[u32; 8]>(r11.as_ptr() as usize + 0x20)
    };

    for mask in [0u8, 1, 2, 4, 7] {
        let o = run(f_o, mask);
        let n = run(f_n, mask);
        crate::diff_eq!(o, n, "141f2c080 mask={}", mask);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_141f5aa00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF_POOL: [[u8; 0x28 * 8]; 2] = [[0; 0x28 * 8]; 2];
    static mut POOL_IDX: usize = 0;
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_6540(vec_hdr: usize, new_cap: u64) {
        if rd::<usize>(vec_hdr) == 0 {
            let p = core::ptr::addr_of_mut!(BUF_POOL[POOL_IDX & 1]) as usize;
            POOL_IDX += 1;
            wr::<usize>(vec_hdr, p);
        }
        wr::<u32>(vec_hdr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08));
        wr::<u32>(dst + 0x10, rd::<u32>(src + 0x10));
    }
    unsafe extern "win64" fn mock_6090(vec_hdr: usize, _end_pos: usize, add_cnt: u64) {
        let old = rd::<u32>(vec_hdr + 0x08);
        wr::<u32>(vec_hdr + 0x08, old.wrapping_add(add_cnt as u32));
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_410d_6540, mock_6540 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_410d_6090, mock_6090 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F, p4: u8| -> ([u8; 0x28 * 4], u32, u32) {
        BUF_POOL = [[0; 0x28 * 8]; 2];
        POOL_IDX = 0;
        FREE_COUNT = 0;

        let mut p2_items = [0u8; 0x28 * 4];
        let p2ip = p2_items.as_mut_ptr() as usize;
        // Item 0: key=100 (shared with p3, state=1 -> becomes 0 then 2 because b18 differs)
        wr::<u64>(p2ip, 100);
        wr::<u8>(p2ip + 0x18, 1);
        wr::<u32>(p2ip + 0x1c, 1);
        wr::<u32>(p2ip + 0x20, 1);
        // Item 1: key=200 (only in p2 -> goes into v28 -> state set to p4*2 + 1)
        let i1 = p2ip + 0x28;
        wr::<u64>(i1, 200);
        wr::<u8>(i1 + 0x18, 2);
        wr::<u32>(i1 + 0x1c, 2);
        wr::<u32>(i1 + 0x20, 0);

        let mut p3_items = [0u8; 0x28 * 2];
        let p3ip = p3_items.as_mut_ptr() as usize;
        // Item 0: key=100 (shared, b18=5)
        wr::<u64>(p3ip, 100);
        wr::<u8>(p3ip + 0x18, 5);
        // Item 1: key=300 (only in p3 -> appended to p2)
        let j1 = p3ip + 0x28;
        wr::<u64>(j1, 300);
        wr::<u8>(j1 + 0x18, 7);
        wr::<u8>(j1 + 0x19, 9);
        wr::<u32>(j1 + 0x1c, 1);
        wr::<u32>(j1 + 0x20, 4);
        wr::<u32>(j1 + 0x24, 8);

        let mut p2_hdr: [u64; 2] = [p2ip as u64, (4u64 << 32) | 2];
        let p3_hdr: [u64; 2] = [p3ip as u64, (2u64 << 32) | 2];

        f(
            0,
            p2_hdr.as_mut_ptr() as usize,
            p3_hdr.as_ptr() as usize,
            p4,
        );
        (
            p2_items,
            rd::<u32>((p2_hdr.as_ptr() as usize) + 0x08),
            FREE_COUNT,
        )
    };

    for p4 in [0u8, 1] {
        let o = run(f_o, p4);
        let n = run(f_n, p4);
        crate::diff_eq!(o, n, "141f5aa00 p4={}", p4);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14205b3f0(orig: usize, new: usize) -> Result<u32, String> {
    type F =
        unsafe extern "win64" fn(u32, usize, usize, u64, u64, usize, usize, usize, u32) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_OBJ_STORAGE: [u8; 0x80] = [0; 0x80];
    static mut CALL_TRACE: u64 = 0;

    fn trace(v: u64) {
        unsafe {
            CALL_TRACE = CALL_TRACE.wrapping_mul(131).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_7000(vec_hdr: usize) {
        let old = rd::<u32>(vec_hdr + 0x08);
        wr::<u32>(vec_hdr + 0x08, old + 1);
        trace(0x7000 ^ (old as u64));
    }
    unsafe extern "win64" fn mock_7140(
        out_ptr: usize,
        _rdx: usize,
        p3: usize,
        p4_ptr: usize,
        p1_ptr: usize,
        p0_ptr: usize,
        p2: usize,
        p9_ptr: usize,
    ) -> usize {
        let obj = core::ptr::addr_of_mut!(NEW_OBJ_STORAGE) as usize;
        wr::<usize>(out_ptr, obj);
        trace(
            0x7140
                ^ (rd::<usize>(p3) as u64)
                ^ rd::<u64>(p4_ptr)
                ^ (rd::<u32>(p1_ptr) as u64)
                ^ (rd::<u32>(p0_ptr) as u64)
                ^ (p2 as u64)
                ^ (rd::<u32>(p9_ptr) as u64),
        );
        out_ptr
    }
    unsafe extern "win64" fn mock_6f90(dst: usize, src: usize) {
        wr::<usize>(dst, rd::<usize>(src));
        wr::<usize>(src, 0);
        trace(0x6f90);
    }
    unsafe extern "win64" fn mock_bb20(val: usize) {
        trace(0xbb20 ^ (val as u64));
    }
    unsafe extern "win64" fn mock_2de0(ptr: usize) {
        trace(0x2de0 ^ (ptr as u64));
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_7000, mock_7000 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_7140, mock_7140 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_6f90, mock_6f90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_bb20, mock_bb20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let g_vec = exe::ptr::<u8>(0x1_43a6_3260) as usize;
    let saved_vec = rd::<[u64; 2]>(g_vec);

    let run = |f: F, hit_existing: bool| -> (u64, u64, u32, u64, u8) {
        CALL_TRACE = 0;
        NEW_OBJ_STORAGE = [0; 0x80];

        let mut slots = [0u8; 0xa0 * 2];
        let sp = slots.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x90, if hit_existing { 0x9999 } else { 0 });
        wr::<u32>(sp + 0x98, 2);
        wr::<u16>(sp + 0x9c, 7);

        wr::<usize>(g_vec, sp);
        wr::<u32>(g_vec + 0x08, 1);
        wr::<u32>(g_vec + 0x0c, 2);

        let mut p3_slot: usize = 0x1111;
        let mut p6_slot: usize = if hit_existing { 0 } else { 0x2222 };
        let mut p7 = [0u64; 13];
        for (i, v) in p7.iter_mut().enumerate() {
            *v = 0x5000 + i as u64;
        }
        let mut p8 = [0u8; 16];
        wr::<u64>(p8.as_mut_ptr() as usize, 0x8888_7777);
        wr::<u8>(p8.as_mut_ptr() as usize + 0x08, 0x3c);

        let ret = f(
            0,
            0x4444,
            (&mut p3_slot as *mut usize) as usize,
            0x6666,
            0x9999,
            (&mut p6_slot as *mut usize) as usize,
            p7.as_ptr() as usize,
            p8.as_ptr() as usize,
            0x77,
        );
        (
            ret,
            CALL_TRACE,
            rd::<u32>(sp + 0x98),
            rd::<u64>(sp + 0x80),
            rd::<u8>(sp + 0x88),
        )
    };

    for hit in [true, false] {
        let o = run(f_o, hit);
        let n = run(f_n, hit);
        wr::<[u64; 2]>(g_vec, saved_vec);
        crate::diff_eq!(o, n, "14205b3f0 hit={}", hit);
    }
    Ok(2)
}

pub unsafe fn diff_r_1420abdc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u64; 6], [u64; 2]) {
        let mut ref_table = [0u64; 6];
        // Slot 1 (+0x10..+0x20): refcount at +0x18 = 1 -> becomes 0 and clears bit 1
        ref_table[3] = 1;
        // Slot 2 (+0x20..+0x30): refcount at +0x28 = 2 -> becomes 1 (bit 2 stays set)
        ref_table[5] = 2;

        let mut bitmask = [0xffff_ffff_ffff_ffffu64; 2];

        let mut p1 = vec![0u8; 0x5e320];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x5e2f0, bitmask.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x5e308, ref_table.as_mut_ptr() as usize);

        let mut sub_entries = [0u8; 0x20];
        let sep = sub_entries.as_mut_ptr() as usize;
        wr::<u16>(sep + 0x08, 1);
        wr::<u16>(sep + 0x18, 2);

        let mut items = vec![0u8; 0x130 * 2];
        let ip = items.as_mut_ptr() as usize;
        wr::<usize>(ip + 0x130 + 0x100, sep);
        wr::<u32>(ip + 0x130 + 0x108, 2);

        let mut p3 = [0u8; 0x10];
        wr::<usize>(p3.as_mut_ptr() as usize + 0x08, ip);

        let u16_list: [u16; 1] = [1];
        let mut buckets = [0u8; 0x80];
        let bp = buckets.as_mut_ptr() as usize;
        // Bucket 0: empty
        wr::<usize>(bp, u16_list.as_ptr() as usize);
        wr::<u32>(bp + 0x08, 0);
        // Bucket 1 (+0x40): 1 item (index 1)
        wr::<usize>(bp + 0x40, u16_list.as_ptr() as usize);
        wr::<u32>(bp + 0x48, 1);

        let mut r10 = [0u8; 0x60];
        let r10p = r10.as_mut_ptr() as usize;
        wr::<usize>(r10p + 0x28 + 0x18, bp);
        wr::<u32>(r10p + 0x28 + 0x20, 2);

        let p2: [usize; 1] = [r10p];
        f(p1p, p2.as_ptr() as usize, p3.as_ptr() as usize, 1);
        (ref_table, bitmask)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420abdc0 out");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: 0x1_41da_1810 (`unknown_141da1810` returns `f32` in `xmm0`)
// Replacement::verified_throws(0x1_41da_1810, "unknown_141da1810", rend::cauldron_hot_w4::unknown_141da1810 as *const (), Throws::Caught, rend::cauldron_hot_w4::diff_unknown_141da1810),
// Replacement::verified_throws(0x1_41e7_91b0, "unknown_141e791b0", rend::cauldron_hot_w4::unknown_141e791b0 as *const (), Throws::Caught, rend::cauldron_hot_w4::diff_unknown_141e791b0),
// Replacement::verified(0x1_41e7_b330, "unknown_141e7b330", rend::cauldron_hot_w4::unknown_141e7b330 as *const (), rend::cauldron_hot_w4::diff_unknown_141e7b330),
// Replacement::verified(0x1_41e9_fa80, "coregame_141e9fa80", rend::cauldron_hot_w4::coregame_141e9fa80 as *const (), rend::cauldron_hot_w4::diff_coregame_141e9fa80),
// Replacement::verified(0x1_41ea_c250, "unknown_141eac250", rend::cauldron_hot_w4::unknown_141eac250 as *const (), rend::cauldron_hot_w4::diff_unknown_141eac250),
// Replacement::verified(0x1_41ea_c8c0, "unknown_141eac8c0", rend::cauldron_hot_w4::unknown_141eac8c0 as *const (), rend::cauldron_hot_w4::diff_unknown_141eac8c0),
// Replacement::verified(0x1_41ec_6230, "unknown_141ec6230", rend::cauldron_hot_w4::unknown_141ec6230 as *const (), rend::cauldron_hot_w4::diff_unknown_141ec6230),
// Replacement::verified(0x1_41f2_b970, "unknown_141f2b970", rend::cauldron_hot_w4::unknown_141f2b970 as *const (), rend::cauldron_hot_w4::diff_unknown_141f2b970),
// Replacement::verified(0x1_41f2_c080, "unknown_141f2c080", rend::cauldron_hot_w4::unknown_141f2c080 as *const (), rend::cauldron_hot_w4::diff_unknown_141f2c080),
// Replacement::verified_throws(0x1_41f5_aa00, "unknown_141f5aa00", rend::cauldron_hot_w4::unknown_141f5aa00 as *const (), Throws::Caught, rend::cauldron_hot_w4::diff_unknown_141f5aa00),
// Replacement::verified_throws(0x1_4205_b3f0, "unknown_14205b3f0", rend::cauldron_hot_w4::unknown_14205b3f0 as *const (), Throws::Caught, rend::cauldron_hot_w4::diff_unknown_14205b3f0),
// Replacement::verified(0x1_420a_bdc0, "r_1420abdc0", rend::cauldron_hot_w4::r_1420abdc0 as *const (), rend::cauldron_hot_w4::diff_r_1420abdc0),
