//! Decompiled `rend` Cauldron / ECS / animation hot functions shard L:
//! - `0x1420afda0` (`r_1420afda0`)
//! - `0x1420b1180` (`r_1420b1180`)
//! - `0x1420b2f80` (`r_1420b2f80`)
//! - `0x1420b2ff0` (`r_1420b2ff0`)
//! - `0x1420b33f0` (`r_1420b33f0`)
//! - `0x1420bd220` (`unknown_1420bd220`)
//! - `0x1420cd120` (`unknown_1420cd120`)
//! - `0x1420d8810` (`unknown_1420d8810`)
//! - `0x1420d9330` (`unknown_1420d9330`)
//! - `0x1420de930` (`unknown_1420de930`)
//! - `0x1420e3850` (`unknown_1420e3850`)
//! - `0x1421de220` (`unknown_1421de220`)

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vbroadcastss, vdpps, vhaddps, vhsubps, vmulps, vpermilps,
    vshufps, vsubps,
};

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
fn vfmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for lane in 0..4 {
        let fa = f32::from_bits(((a >> (lane * 32)) & 0xffff_ffff) as u32);
        let fb = f32::from_bits(((b >> (lane * 32)) & 0xffff_ffff) as u32);
        let fc = f32::from_bits(((acc >> (lane * 32)) & 0xffff_ffff) as u32);
        let r = fa.mul_add(fb, fc);
        out |= (r.to_bits() as u128) << (lane * 32);
    }
    out
}

#[inline(always)]
fn vfnmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for lane in 0..4 {
        let fa = f32::from_bits(((a >> (lane * 32)) & 0xffff_ffff) as u32);
        let fb = f32::from_bits(((b >> (lane * 32)) & 0xffff_ffff) as u32);
        let fc = f32::from_bits(((acc >> (lane * 32)) & 0xffff_ffff) as u32);
        let r = (-fa).mul_add(fb, fc);
        out |= (r.to_bits() as u128) << (lane * 32);
    }
    out
}

/// `r_1420afda0` — `_anon_4540FC47::sortIteration32`
/// Original: `0x1420afda0` (726 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_1420afda0(
    param_1: usize,
    param_2: usize,
    param_3: u32,
    param_4: i64,
    param_5: usize,
    param_6: usize,
) {
    let depth = rd::<u32>(param_1 + 0x10);
    let rsi = rd::<i64>(param_1);
    let r14 = rd::<i64>(param_1 + 0x08);

    if depth == param_3 || r14.wrapping_sub(rsi) <= param_4 {
        let sub_ptr = param_2.wrapping_add((rsi as usize).wrapping_mul(4));
        let count = (r14 as u32).wrapping_sub(rsi as u32);
        exe::call_va!(0x1_4103_a0c0, (sub_ptr, count));
        return;
    }

    let rbx = r14.wrapping_sub(1);
    exe::call_va!(param_6 as u64, (param_2, rsi, rbx));

    let r10 = param_2.wrapping_add((rbx as usize).wrapping_mul(4));
    let pivot = rd::<u32>(r10);
    let mut rdx = rsi.wrapping_sub(1);
    let mut rax = rsi;
    while rax < rbx {
        let addr_rax = param_2.wrapping_add((rax as usize).wrapping_mul(4));
        let r8d = rd::<u32>(addr_rax);
        if r8d <= pivot {
            rdx = rdx.wrapping_add(1);
            let addr_rdx = param_2.wrapping_add((rdx as usize).wrapping_mul(4));
            let ecx = rd::<u32>(addr_rdx);
            wr::<u32>(addr_rdx, r8d);
            wr::<u32>(addr_rax, ecx);
        }
        rax = rax.wrapping_add(1);
    }

    let r15 = rdx.wrapping_add(1);
    let addr_r15 = param_2.wrapping_add((r15 as usize).wrapping_mul(4));
    let ecx = rd::<u32>(addr_r15);
    let eax = rd::<u32>(r10);
    wr::<u32>(addr_r15, eax);
    wr::<u32>(r10, ecx);

    let g_job_va = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    if rsi < r15 {
        let next_depth = depth.wrapping_add(1);
        let g_job = rd::<usize>(g_job_va);
        let rbx_sched = rd::<usize>(g_job + 0x10);
        let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let mut job_ptr: usize = 0;
        let pool = rd::<usize>(rbx_sched + wid * 0x40 + 0x48).wrapping_add(0x80);
        exe::call_va!(0x1_4248_5aa0, (pool, (&mut job_ptr as *mut usize) as usize));
        crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
        let vftbl = exe::ptr::<u8>(0x1_433d_d700) as usize;
        wr::<u64>(job_ptr + 0x08, 0);
        wr::<i64>(job_ptr + 0x18, rsi);
        wr::<i64>(job_ptr + 0x20, r15);
        wr::<u64>(job_ptr + 0x28, next_depth as u64);
        wr::<usize>(job_ptr + 0x30, param_2);
        wr::<u64>(job_ptr + 0x38, param_3 as u64);
        wr::<i64>(job_ptr + 0x40, param_4);
        wr::<usize>(job_ptr + 0x48, param_5);
        wr::<usize>(job_ptr + 0x50, param_6);
        wr::<usize>(job_ptr + 0x10, vftbl);
        let handle = rd::<usize>(param_5);
        let _ = crate::simd_ops::atomic_add_u32(handle as u64, 1);
        wr::<usize>(job_ptr + 0x08, handle);
        if rd::<u32>(job_ptr) == 1 {
            let sched = rd::<usize>(g_job + 0x10);
            exe::call_va!(0x1_4248_6460, (sched, job_ptr));
            let sem = rd::<usize>(rd::<usize>(sched) + 0x1008);
            exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
        }
    }

    let r15_right = r15.wrapping_add(1);
    if r15_right < r14 {
        let next_depth = depth.wrapping_add(1);
        let g_job = rd::<usize>(g_job_va);
        let rbx_sched = rd::<usize>(g_job + 0x10);
        let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let mut job_ptr: usize = 0;
        let pool = rd::<usize>(rbx_sched + wid * 0x40 + 0x48).wrapping_add(0x80);
        exe::call_va!(0x1_4248_5aa0, (pool, (&mut job_ptr as *mut usize) as usize));
        crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
        let vftbl = exe::ptr::<u8>(0x1_433d_d6d0) as usize;
        wr::<u64>(job_ptr + 0x08, 0);
        wr::<i64>(job_ptr + 0x18, r15_right);
        wr::<i64>(job_ptr + 0x20, r14);
        wr::<u64>(job_ptr + 0x28, next_depth as u64);
        wr::<usize>(job_ptr + 0x30, param_2);
        wr::<u64>(job_ptr + 0x38, param_3 as u64);
        wr::<i64>(job_ptr + 0x40, param_4);
        wr::<usize>(job_ptr + 0x48, param_5);
        wr::<usize>(job_ptr + 0x50, param_6);
        wr::<usize>(job_ptr + 0x10, vftbl);
        let handle = rd::<usize>(param_5);
        let _ = crate::simd_ops::atomic_add_u32(handle as u64, 1);
        wr::<usize>(job_ptr + 0x08, handle);
        if rd::<u32>(job_ptr) == 1 {
            let sched = rd::<usize>(g_job + 0x10);
            exe::call_va!(0x1_4248_6460, (sched, job_ptr));
            let sem = rd::<usize>(rd::<usize>(sched) + 0x1008);
            exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
        }
    }
}

/// `r_1420b1180` — `r::JobCallable<ecs::core::bindGroups::<lambda_3>>::vfunc_1`
/// Original: `0x1420b1180` (98 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_1420b1180(param_1: usize) {
    let mut v10 = rd::<[u64; 2]>(param_1 + 0x10);
    let mut v20 = rd::<[u64; 2]>(param_1 + 0x20);
    let mut v30 = rd::<[u64; 2]>(param_1 + 0x30);
    let arg1 = rd::<u16>(param_1 + 0x08) as u32;
    let arg5 = rd::<u64>(param_1 + 0x40);
    let arg6 = rd::<u64>(param_1 + 0x48);
    let arg7 = rd::<u64>(param_1 + 0x50);
    let arg8 = rd::<u64>(param_1 + 0x58);
    exe::call_va!(
        0x1_420b_2330,
        (
            arg1,
            v10.as_mut_ptr() as usize,
            v20.as_mut_ptr() as usize,
            v30.as_mut_ptr() as usize,
            arg5,
            arg6,
            arg7,
            arg8
        )
    );
}

/// `r_1420b2f80` — `r::JobCallable<ecs::core::GroupManager::clean::<lambda_1>>::vfunc_1`
/// Original: `0x1420b2f80` (61 bytes, `Throws::No`)
pub unsafe extern "win64" fn r_1420b2f80(param_1: usize) {
    let mut v20 = rd::<[u64; 2]>(param_1 + 0x20);
    let arg1 = rd::<u64>(param_1 + 0x08);
    let arg3 = rd::<u64>(param_1 + 0x10);
    let arg4 = rd::<u64>(param_1 + 0x18);
    exe::call_va!(0x1_420b_33f0, (arg1, v20.as_mut_ptr() as usize, arg3, arg4));
}

/// `r_1420b2ff0` — `r::JobCallable<ecs::core::GroupManager::clean::<lambda_2>>::vfunc_1`
/// Original: `0x1420b2ff0` (61 bytes, `Throws::No`)
pub unsafe extern "win64" fn r_1420b2ff0(param_1: usize) {
    let mut v10 = rd::<[u64; 2]>(param_1 + 0x10);
    let arg1 = rd::<u64>(param_1 + 0x08).wrapping_add(8);
    let arg3 = rd::<u64>(param_1 + 0x20);
    exe::call_va!(0x1_411f_03b0, (arg1, v10.as_mut_ptr() as usize, arg3));
}

#[inline(always)]
unsafe fn find_u16_idx(base_ptr: usize, count: usize, needle: u16) -> isize {
    for i in 0..count {
        if rd::<u16>(base_ptr + i * 2) == needle {
            return i as isize;
        }
    }
    -1
}

/// `r_1420b33f0`
/// Original: `0x1420b33f0` (905 bytes, `Throws::No`)
pub unsafe extern "win64" fn r_1420b33f0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u64,
) {
    let base_shl6 = param_3.wrapping_shl(6);
    let r15_words = param_4.wrapping_add(0x3f) >> 6;
    let span_len = rd::<u64>(param_2) as usize;
    let mut rdi = rd::<usize>(param_2 + 0x08);
    let rsi_end = rdi.wrapping_add(span_len.wrapping_mul(2));

    while rdi != rsi_end {
        let r14w = rd::<u16>(rdi);
        let ptr_70 = rd::<usize>(param_1 + 0x70);
        let stride_b0 = rd::<u64>(param_1 + 0xb0);
        let word_off = (r14w as u64).wrapping_mul(stride_b0).wrapping_add(param_3);
        let rbx = ptr_70.wrapping_add((word_off as usize).wrapping_mul(8));

        let mut span_40: [u64; 3] = [r15_words, rbx as u64, param_4];
        let mut span_60: [u64; 3] = [r15_words, rbx as u64, param_4];
        let mut out_iter = [0u64; 2];
        let ret_ptr = exe::call_va!(
            0x1_411d_3b20,
            (
                span_40.as_mut_ptr() as usize,
                out_iter.as_mut_ptr() as usize,
                0u64
            ) -> usize
        );
        let mut cur_bit = rd::<u64>(ret_ptr);
        let mut found = rd::<u8>(ret_ptr + 0x08);

        while found != 0 {
            let slot_idx = ((cur_bit.wrapping_add(base_shl6)) & 0xffff) as usize;
            let r11 = rd::<usize>(param_1 + 0x10).wrapping_add(slot_idx * 0x30);

            let len0 = rd::<u32>(r11 + 0x08) as usize;
            let ptr0 = rd::<usize>(r11);
            let idx0 = find_u16_idx(ptr0, len0, r14w);
            if idx0 != -1 {
                let p_elem = ptr0.wrapping_add((idx0 as usize) * 2);
                let p_last = ptr0.wrapping_add((len0 - 1) * 2);
                if p_elem != p_last {
                    wr::<u16>(p_elem, rd::<u16>(p_last));
                }
                wr::<u32>(r11 + 0x08, (len0 as u32).wrapping_sub(1));
            }

            let len1_init = rd::<u32>(r11 + 0x18) as usize;
            let ptr1_init = rd::<usize>(r11 + 0x10);
            let mut r8_idx = find_u16_idx(ptr1_init, len1_init, r14w);
            while r8_idx != -1 {
                let idx_u = r8_idx as usize;
                let ptr1 = rd::<usize>(r11 + 0x10);
                let last1 = rd::<u32>(r11 + 0x18).wrapping_sub(1) as usize;
                let p_elem1 = ptr1.wrapping_add(idx_u * 2);
                let p_last1 = ptr1.wrapping_add(last1 * 2);
                if p_elem1 != p_last1 {
                    wr::<u16>(p_elem1, rd::<u16>(p_last1));
                }
                wr::<u32>(r11 + 0x18, last1 as u32);

                let ptr2 = rd::<usize>(r11 + 0x20);
                let last2 = rd::<u32>(r11 + 0x28).wrapping_sub(1) as usize;
                let p_elem2 = ptr2.wrapping_add(idx_u * 4);
                let p_last2 = ptr2.wrapping_add(last2 * 4);
                if p_elem2 != p_last2 {
                    wr::<u32>(p_elem2, rd::<u32>(p_last2));
                }
                wr::<u32>(r11 + 0x28, last2 as u32);

                let ptr1_after = rd::<usize>(r11 + 0x10);
                let len1_after = rd::<u32>(r11 + 0x18) as usize;
                let rem_len = len1_after.wrapping_sub(idx_u);
                let sub_base = ptr1_after.wrapping_add(idx_u * 2);
                r8_idx = find_u16_idx(sub_base, rem_len, r14w);
            }

            let mut out_next = [0u64; 2];
            let next_ptr = exe::call_va!(
                0x1_411d_3b20,
                (
                    span_60.as_mut_ptr() as usize,
                    out_next.as_mut_ptr() as usize,
                    cur_bit.wrapping_add(1)
                ) -> usize
            );
            cur_bit = rd::<u64>(next_ptr);
            found = rd::<u8>(next_ptr + 0x08);
        }

        if param_4 != 0 {
            let last_word_ptr = rbx.wrapping_add(((r15_words - 1) as usize) * 8);
            let mut p = rbx;
            while p != last_word_ptr {
                wr::<u64>(p, 0);
                p = p.wrapping_add(8);
            }
            let shift = (!(param_4.wrapping_sub(1)) as u32) & 0x3f;
            let mask = !(u64::MAX >> shift);
            wr::<u64>(last_word_ptr, rd::<u64>(last_word_ptr) & mask);
        }

        rdi = rdi.wrapping_add(2);
    }
}

/// `unknown_1420bd220`
/// Original: `0x1420bd220` (676 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1420bd220(param_1: usize) {
    let ctx = rd::<usize>(param_1);
    let mut rbx = rd::<u64>(param_1 + 0x08);
    if rbx == 0 {
        return;
    }

    let f_10 = rd::<usize>(ctx + 0x10);
    let f_18 = rd::<usize>(ctx + 0x18);
    let f_20 = rd::<u64>(ctx + 0x20);
    let f_28 = rd::<u64>(ctx + 0x28);
    let f_38 = rd::<usize>(ctx + 0x38);
    let f_48 = rd::<usize>(ctx + 0x48);
    let f_50 = rd::<usize>(ctx + 0x50);
    let f_60 = rd::<usize>(ctx + 0x60);

    let mut node0 = rd::<usize>(param_1 + 0x10);
    let mut idx0 = rd::<u64>(param_1 + 0x18);
    let mut node2 = rd::<usize>(param_1 + 0x20);
    let mut idx2 = rd::<u64>(param_1 + 0x28);
    let mut node1 = rd::<usize>(param_1 + 0x30);
    let mut idx1 = rd::<u64>(param_1 + 0x38);

    while rbx != 0 {
        let cur_node1 = node1;
        let cur_idx1 = idx1;
        idx1 = cur_idx1.wrapping_add(1);
        if idx1 == rd::<u64>(cur_node1 + 0x08) {
            let next1 = rd::<usize>(cur_node1);
            if next1 != 0 {
                idx1 = 0;
                node1 = next1;
            }
        }

        let cur_node0 = node0;
        let cur_idx0 = idx0;
        let cur_node2 = node2;
        let cur_idx2 = idx2;
        let val_rax = rd::<usize>(cur_node1 + (cur_idx1 as usize) * 8 + 0x10);

        idx0 = cur_idx0.wrapping_add(1);
        if idx0 == rd::<u64>(cur_node0 + 0x08) {
            let next0 = rd::<usize>(cur_node0);
            if next0 != 0 {
                idx0 = 0;
                node0 = next0;
            }
        }

        idx2 = cur_idx2.wrapping_add(1);
        if idx2 == rd::<u64>(cur_node2 + 0x08) {
            let next2 = rd::<usize>(cur_node2);
            if next2 != 0 {
                idx2 = 0;
                node2 = next2;
            }
        }

        if val_rax != 0 {
            let idx_u32 = rd::<u32>(cur_node0 + (cur_idx0 as usize) * 4 + 0x10) as usize;
            let r8_u16 = rd::<u16>(f_10 + idx_u32 * 2) as usize;
            let r11_u16 = rd::<u16>(cur_node2 + (cur_idx2 as usize) * 2 + 0x10) as usize;
            let vtbl_slot = rd::<usize>(f_18).wrapping_add(r11_u16 * 0x50);

            let found1 = exe::call_va!(0x1_4248_9ee0, (f_28, f_20, r8_u16 as u32) -> i32) as isize;
            let rdi = if found1 == -1 {
                f_48.wrapping_add(r8_u16 * 0x30)
            } else {
                f_38.wrapping_add((found1 as usize) * 0x30)
            };

            let sub_hdr = f_50.wrapping_add(r8_u16 << 5);
            let search_ptr = rd::<u64>(sub_hdr + 0x18058);
            let search_len = rd::<u32>(sub_hdr + 0x1806c) as u64;
            let found2 = exe::call_va!(
                0x1_4248_9ee0,
                (search_ptr, search_len, r11_u16 as u32) -> i32
            ) as isize;

            let off_arr = rd::<usize>(rdi + 0x18);
            let elem_off = rd::<u32>((off_arr as isize + found2 * 4) as usize) as u64;
            let arg2 = rd::<u64>(rdi + 0x08).wrapping_add(elem_off);
            let arg3 = rd::<u32>(f_60 + idx_u32 * 4);
            let fn_ptr = rd::<u64>(vtbl_slot + 0x30);
            exe::call_va!(fn_ptr, (val_rax, arg2, arg3));
        }

        rbx -= 1;
    }
}

#[inline(always)]
unsafe fn call_f32_diff_helper(va: u64, a: usize, b: usize, c_bits: u32, sign_mask: u32) -> f32 {
    let (_, x0) = crate::simd_ops::call_va8_xmm(
        va,
        a as u64,
        b as u64,
        c_bits as u64,
        0,
        0,
        0,
        0,
        0,
        [0, 0, c_bits as u128, 0],
    );
    f32::from_bits((x0 as u32) & !sign_mask)
}

/// `unknown_1420cd120`
/// Original: `0x1420cd120` (1364 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1420cd120(
    param_1: usize,
    param_2: usize,
    param_3: u16,
) -> u8 {
    let fn_get_tid = rd::<usize>(exe::ptr::<usize>(0x1_4264_5790) as usize);
    let tid = exe::call_va!(fn_get_tid, () -> u32);
    if rd::<u32>(param_2 + 0x40) != tid {
        let fn_acq = rd::<usize>(exe::ptr::<usize>(0x1_4264_55a8) as usize);
        exe::call_va!(fn_acq, (param_2 + 0x38));
        wr::<u32>(param_2 + 0x40, tid);
    }
    wr::<u32>(param_2 + 0x44, rd::<u32>(param_2 + 0x44).wrapping_add(1));

    let idx0 = (rd::<u32>(param_2 + 0x138) & 3) as usize;
    let rsi = param_2 + 0x48 + idx0 * 0x28;

    let idx1 = if rd::<u8>(param_1 + 0x4fd8) != 0 {
        (((rd::<u64>(param_2 + 0x140).wrapping_sub(1)) as u32 & 1) as usize) + 4
    } else {
        ((rd::<u64>(param_2 + 0x138).wrapping_sub(1)) as u32 & 3) as usize
    };
    let rbp = param_2 + 0x48 + idx1 * 0x28;

    let idx2 = if rd::<u8>(param_1 + 0x4fd8) != 0 {
        ((rd::<u32>(param_2 + 0x140) & 1) as usize) + 4
    } else {
        ((rd::<u64>(param_2 + 0x138).wrapping_sub(2)) as u32 & 3) as usize
    };
    let r14 = param_2 + 0x48 + idx2 * 0x28;

    let eps_pos = rd::<f32>(exe::ptr::<f32>(0x1_4342_6460) as usize);
    let eps_neg = rd::<f32>(exe::ptr::<f32>(0x1_4342_7000) as usize);
    let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_6ff0) as usize);
    let c_7080 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7080) as usize);

    let res: u8 = match param_3 {
        0x83 => {
            let v1 = call_f32_diff_helper(0x1_420c_e940, rsi, rbp, c_7080, sign_mask);
            let v2 = call_f32_diff_helper(0x1_420c_e940, rbp, r14, c_7080, sign_mask);
            if (eps_pos > v1) != (eps_pos > v2) {
                1
            } else {
                let w1 = call_f32_diff_helper(0x1_420c_e8a0, rsi, rbp, c_7080, sign_mask);
                let w2 = call_f32_diff_helper(0x1_420c_e8a0, rbp, r14, c_7080, sign_mask);
                ((eps_pos > w1) != (eps_pos > w2)) as u8
            }
        }
        0x84 => {
            let v1 = call_f32_diff_helper(0x1_420c_e940, rsi, rbp, c_7080, sign_mask);
            let v2 = call_f32_diff_helper(0x1_420c_e940, rbp, r14, c_7080, sign_mask);
            ((eps_pos > v2) as u8) ^ ((eps_pos > v1) as u8)
        }
        0x85 => {
            let w1 = call_f32_diff_helper(0x1_420c_e8a0, rsi, rbp, c_7080, sign_mask);
            let w2 = call_f32_diff_helper(0x1_420c_e8a0, rbp, r14, c_7080, sign_mask);
            ((eps_pos > w2) as u8) ^ ((eps_pos > w1) as u8)
        }
        0x86 => {
            (rd::<f32>(rsi + 0x10) != rd::<f32>(rbp + 0x10)
                || rd::<f32>(rsi + 0x14) != rd::<f32>(rbp + 0x14)) as u8
        }
        0x87 => (rd::<f32>(rsi + 0x10) != rd::<f32>(rbp + 0x10)) as u8,
        0x88 => (rd::<f32>(rsi + 0x14) != rd::<f32>(rbp + 0x14)) as u8,
        0x89 => (rd::<u8>(rsi + 0x20) != rd::<u8>(rbp + 0x20)) as u8,
        0x8a => (rd::<u8>(rsi + 0x21) != rd::<u8>(rbp + 0x21)) as u8,
        0x8b => (rd::<u8>(rsi + 0x22) != rd::<u8>(rbp + 0x22)) as u8,
        0x8c => (rd::<u8>(rsi + 0x23) != rd::<u8>(rbp + 0x23)) as u8,
        0x8d => (rd::<u8>(rsi + 0x24) != rd::<u8>(rbp + 0x24)) as u8,
        0x8e => {
            let v_rbp = rd::<f32>(rbp + 0x18);
            let d1 = f32::from_bits((v_rbp - rd::<f32>(r14 + 0x18)).to_bits() & !sign_mask);
            let d2 = f32::from_bits((rd::<f32>(rsi + 0x18) - v_rbp).to_bits() & !sign_mask);
            ((eps_pos > d1) as u8) ^ ((eps_pos > d2) as u8)
        }
        0x8f => {
            let v_rbp = rd::<f32>(rbp + 0x18);
            let d1 = v_rbp - rd::<f32>(r14 + 0x18);
            let d2 = rd::<f32>(rsi + 0x18) - v_rbp;
            ((eps_pos > d1) as u8) ^ ((eps_pos > d2) as u8)
        }
        0x90 => {
            let v_rbp = rd::<f32>(rbp + 0x18);
            let d1 = v_rbp - rd::<f32>(r14 + 0x18);
            let d2 = rd::<f32>(rsi + 0x18) - v_rbp;
            ((d1 > eps_neg) as u8) ^ ((d2 > eps_neg) as u8)
        }
        0x91 => {
            let v_rbp = rd::<f32>(rbp + 0x1c);
            let d1 = f32::from_bits((v_rbp - rd::<f32>(r14 + 0x1c)).to_bits() & !sign_mask);
            let d2 = f32::from_bits((rd::<f32>(rsi + 0x1c) - v_rbp).to_bits() & !sign_mask);
            ((eps_pos > d1) as u8) ^ ((eps_pos > d2) as u8)
        }
        0x92 => {
            let v_rbp = rd::<f32>(rbp + 0x1c);
            let d1 = v_rbp - rd::<f32>(r14 + 0x1c);
            let d2 = rd::<f32>(rsi + 0x1c) - v_rbp;
            ((d1 > eps_neg) as u8) ^ ((d2 > eps_neg) as u8)
        }
        0x93 => {
            let v_rbp = rd::<f32>(rbp + 0x1c);
            let d1 = v_rbp - rd::<f32>(r14 + 0x1c);
            let d2 = rd::<f32>(rsi + 0x1c) - v_rbp;
            ((eps_pos > d1) as u8) ^ ((eps_pos > d2) as u8)
        }
        _ => 0,
    };

    let new_cnt = rd::<u32>(param_2 + 0x44).wrapping_sub(1);
    wr::<u32>(param_2 + 0x44, new_cnt);
    if new_cnt == 0 {
        crate::simd_ops::atomic_xchg_u32((param_2 + 0x40) as u64, u32::MAX);
        let fn_rel = rd::<usize>(exe::ptr::<usize>(0x1_4264_5598) as usize);
        exe::call_va!(fn_rel, (param_2 + 0x38));
    }
    res
}

/// `unknown_1420d8810`
/// Original: `0x1420d8810` (480 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420d8810(param_1: usize, param_2: u16) -> u8 {
    if (param_2 & 0x4000) != 0 {
        let r14 = rd::<usize>(param_1 + 0x08);
        let bx = param_2 & 0xbfff;
        let r12 = rd::<u32>(r14 + 0x5888) as u64;
        let rdi = r14 + 0x5aa0;
        let r15 = rd::<usize>(r14 + 0x5880);

        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rdi + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (rdi));
            wr::<u32>(rdi + 0x08, tid);
        }
        let ecx = rd::<u32>(rdi + 0x0c);
        let idx = bx.wrapping_sub(0xce) as u64;
        let (bpl, entry_ptr) = if idx < r12 {
            (true, r15.wrapping_add((idx as usize) * 0x28))
        } else {
            (false, 0)
        };
        wr::<u32>(rdi + 0x0c, ecx);
        if ecx == 0 {
            crate::simd_ops::atomic_xchg_u32((rdi + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (rdi));
        }

        if bpl {
            exe::call_va!(0x1_420e_0380, (r14, entry_ptr));
            for off in [0x18usize, 0x1a, 0x1c, 0x1e, 0x20, 0x22] {
                let child = rd::<u16>(entry_ptr + off);
                if unknown_1420d8810(param_1, child) != 0 {
                    return 1;
                }
            }
        }
        return 0;
    }

    if param_2 < 0xce {
        return exe::call_va!(0x1_420d_98f0, (param_1, param_2 as u32) -> u8);
    }

    let rdi_base = rd::<usize>(param_1 + 0x08);
    let r13 = rd::<u32>(rdi_base + 0x5a78) as u64;
    let r12 = rd::<usize>(rdi_base + 0x5a70);
    let rdi = rdi_base + 0x5af0;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rdi + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (rdi));
        wr::<u32>(rdi + 0x08, tid);
    }
    let edx = rd::<u32>(rdi + 0x0c);
    let idx = param_2.wrapping_sub(0xce) as u64;
    let (bpl, r14) = if idx < r13 {
        (true, r12.wrapping_add((idx as usize) * 0x28))
    } else {
        (false, 0)
    };
    wr::<u32>(rdi + 0x0c, edx);
    if edx == 0 {
        crate::simd_ops::atomic_xchg_u32((rdi + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rdi));
    }

    if bpl {
        return exe::call_va!(0x1_420d_b7c0, (r14, param_2 as u32, param_1) -> u8);
    }
    0
}

/// `unknown_1420d9330`
/// Original: `0x1420d9330` (576 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1420d9330(
    param_1: usize,
    param_2: u16,
    param_3: u8,
) -> u8 {
    let r15 = rd::<usize>(param_1 + 0x08);
    if exe::call_va!(0x1_420d_2c00, (r15, param_2 as u32, param_3) -> u8) != 0 {
        return 0;
    }

    if (param_2 & 0x4000) != 0 {
        let bx = param_2 & 0xbfff;
        let rsi = r15 + 0x5aa0;
        let r12 = rd::<usize>(r15 + 0x5880);
        let r13 = rd::<u32>(r15 + 0x5888) as u64;

        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rsi + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (rsi));
            wr::<u32>(rsi + 0x08, tid);
        }
        let ecx = rd::<u32>(rsi + 0x0c);
        let idx = bx.wrapping_sub(0xce) as u64;
        let (bpl, entry_ptr) = if idx < r13 {
            (true, r12.wrapping_add((idx as usize) * 0x28))
        } else {
            (false, 0)
        };
        wr::<u32>(rsi + 0x0c, ecx);
        if ecx == 0 {
            crate::simd_ops::atomic_xchg_u32((rsi + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (rsi));
        }

        if bpl {
            exe::call_va!(0x1_420e_0380, (r15, entry_ptr));
            for off in [0x18usize, 0x1a, 0x1c, 0x1e, 0x20, 0x22] {
                let child = rd::<u16>(entry_ptr + off);
                if unknown_1420d9330(param_1, child, param_3) != 0 {
                    return 1;
                }
            }
        }
        return 0;
    }

    if param_2.wrapping_sub(0x95) <= 0x24 {
        let rax = rd::<usize>(param_1);
        let rdx = rd::<usize>(rax + 0x60);
        if rdx == 0 {
            return 0;
        }
        let rcx = rd::<usize>(rax);
        if rd::<u64>(rdx + 0x38) != 0 {
            return exe::call_va!(0x1_420d_0e90, (rcx, rdx, param_2 as u32, param_3) -> u8);
        }
        if rd::<u64>(rdx + 0x40) != 0 {
            return exe::call_va!(0x1_420d_7970, (rcx, rdx, param_2 as u32, param_3) -> u8);
        }
        return 0;
    }

    if param_2.wrapping_sub(1) <= 0x80 {
        let rcx = rd::<usize>(param_1);
        let rdx = rd::<usize>(rcx + 0x48);
        if rdx == 0 {
            return 0;
        }
        return exe::call_va!(0x1_420c_eef0, (rd::<usize>(rcx), rdx, param_2 as u32) -> u8);
    }

    if param_2.wrapping_sub(0x83) <= 0x10 {
        let rcx = rd::<usize>(param_1);
        let rdx = rd::<usize>(rcx + 0x50);
        if rdx == 0 {
            return 0;
        }
        return exe::call_va!(0x1_420c_d120, (rd::<usize>(rcx), rdx, param_2 as u32) -> u8);
    }

    0
}

/// `unknown_1420de930`
/// Original: `0x1420de930` (674 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420de930(param_1: usize) {
    let mut rdi = rd::<usize>(param_1 + 0x4fe0);
    let count = rd::<u32>(param_1 + 0x4fe8) as usize;
    let rsi = rdi.wrapping_add(count * 0x78);

    while rdi != rsi {
        let rbp_60 = rd::<usize>(rdi + 0x60);
        if rbp_60 != 0 {
            let mode_61: u8 = if rd::<u64>(rbp_60 + 0x38) != 0 {
                1
            } else if rd::<u64>(rbp_60 + 0x40) != 0 {
                2
            } else {
                0
            };
            if rd::<u8>(rbp_60 + 0x18) != 0 {
                for bx in 0x94u16..0xbau16 {
                    if bx.wrapping_sub(0xb2) > 2
                        && exe::call_va!(0x1_411f_4260, (rdi, bx as u32, 0u8) -> u8) != 0
                    {
                        crate::simd_ops::atomic_xchg_u8((param_1 + 0x5860) as u64, 0);
                        crate::simd_ops::atomic_xchg_u8((param_1 + 0x5861) as u64, mode_61);
                        let v_fc8 = rd::<u64>(param_1 + 0x4fc8);
                        crate::simd_ops::atomic_xchg_u64((param_1 + 0x4fd0) as u64, v_fc8);
                        let v_fd0 = rd::<u64>(param_1 + 0x4fd0);
                        crate::simd_ops::atomic_xchg_u64((rbp_60 + 0x20) as u64, v_fd0);
                        break;
                    }
                }
            }
        }

        let rbp_50 = rd::<usize>(rdi + 0x50);
        if rbp_50 != 0 && rd::<u8>(rbp_50 + 0x18) != 0 {
            for bx in 0x82u16..0x94u16 {
                if bx.wrapping_sub(0xb3) > 1
                    && exe::call_va!(0x1_411f_4260, (rdi, bx as u32, 0u8) -> u8) != 0
                {
                    crate::simd_ops::atomic_xchg_u8((param_1 + 0x5860) as u64, 1);
                    crate::simd_ops::atomic_xchg_u8((param_1 + 0x5861) as u64, 0);
                    let v_fc8 = rd::<u64>(param_1 + 0x4fc8);
                    crate::simd_ops::atomic_xchg_u64((param_1 + 0x4fd0) as u64, v_fc8);
                    let v_fd0 = rd::<u64>(param_1 + 0x4fd0);
                    crate::simd_ops::atomic_xchg_u64((rbp_50 + 0x20) as u64, v_fd0);
                    break;
                }
            }
        }

        let rbp_48 = rd::<usize>(rdi + 0x48);
        if rbp_48 != 0 && rd::<u8>(rbp_48 + 0x18) != 0 {
            for bx in 0x00u16..0x82u16 {
                if bx.wrapping_sub(0xb3) > 1
                    && exe::call_va!(0x1_411f_4260, (rdi, bx as u32, 0u8) -> u8) != 0
                {
                    crate::simd_ops::atomic_xchg_u8((param_1 + 0x5860) as u64, 1);
                    crate::simd_ops::atomic_xchg_u8((param_1 + 0x5861) as u64, 0);
                    let v_fc8 = rd::<u64>(param_1 + 0x4fc8);
                    crate::simd_ops::atomic_xchg_u64((param_1 + 0x4fd0) as u64, v_fc8);
                    let v_fd0 = rd::<u64>(param_1 + 0x4fd0);
                    crate::simd_ops::atomic_xchg_u64((rbp_48 + 0x20) as u64, v_fd0);
                    break;
                }
            }
        }

        let rbp_58 = rd::<usize>(rdi + 0x58);
        if rbp_58 != 0 && rd::<u8>(rbp_58 + 0x18) != 0 {
            for bx in 0xbau16..0xc3u16 {
                if bx.wrapping_sub(0xb2) > 2
                    && exe::call_va!(0x1_411f_4260, (rdi, bx as u32, 0u8) -> u8) != 0
                {
                    crate::simd_ops::atomic_xchg_u8((param_1 + 0x5860) as u64, 2);
                    crate::simd_ops::atomic_xchg_u8((param_1 + 0x5861) as u64, 0);
                    let v_fc8 = rd::<u64>(param_1 + 0x4fc8);
                    crate::simd_ops::atomic_xchg_u64((param_1 + 0x4fd0) as u64, v_fc8);
                    let v_fd0 = rd::<u64>(param_1 + 0x4fd0);
                    crate::simd_ops::atomic_xchg_u64((rbp_58 + 0x20) as u64, v_fd0);
                    break;
                }
            }
        }

        rdi = rdi.wrapping_add(0x78);
    }
}

/// `unknown_1420e3850`
/// Original: `0x1420e3850` (172 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420e3850(param_1: usize, param_2: u16) -> u8 {
    let p0 = rd::<usize>(param_1);
    if p0 == 0 {
        return 0;
    }

    let fn_70 = rd::<usize>(param_1 + 0x70);
    let fn_e0960 = exe::ptr::<u8>(0x1_420e_0960) as usize;
    if rd::<u64>(param_1 + 0x08) != rd::<u64>(p0 + 0x4fc0) && fn_70 != fn_e0960 {
        return 0;
    }

    if fn_70 != 0 {
        let arg_68 = rd::<u64>(param_1 + 0x68);
        if exe::call_va!(fn_70 as u64, (arg_68) -> u8) == 0 {
            return 0;
        }
    }

    if rd::<u64>(param_1 + 0x48) == 0 && rd::<u64>(param_1 + 0x50) == 0 {
        let p60 = rd::<usize>(param_1 + 0x60);
        if p60 == 0 || rd::<u8>(p60 + 0x18) == 0 {
            return 0;
        }
    }

    let mut resolved: u16 = 0;
    let mut pair: [usize; 2] = [param_1, rd::<usize>(param_1)];
    exe::call_va!(
        0x1_420d_cac0,
        (
            (&mut resolved as *mut u16) as usize,
            pair[1],
            param_2 as u32
        ) -> usize
    );
    exe::call_va!(0x1_420d_8810, (pair.as_mut_ptr() as usize, resolved as u32) -> u8)
}

/// `unknown_1421de220`
/// Original: `0x1421de220` (823 bytes, `Throws::No`)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn unknown_1421de220(
    param_1: usize,
    param_2: usize,
    param_3: f32,
) {
    unknown_1421de220_impl(param_1, param_2, param_3);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn unknown_1421de220(
    param_1: usize,
    param_2: usize,
    _r8: usize,
    _r9: usize,
    _xmm0: f32,
    _xmm1: f32,
    param_3: f32,
) {
    unknown_1421de220_impl(param_1, param_2, param_3);
}

#[inline(always)]
unsafe fn unknown_1421de220_impl(param_1: usize, param_2: usize, param_3: f32) {
    let c_81d0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_81d0) as usize);
    let c_80c0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_80c0) as usize);
    let c_84c0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_84c0) as usize);
    let c_8120 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize);
    let c_8240 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8240) as usize);
    let c_82c0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_82c0) as usize);
    let c_8840 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8840) as usize);
    let c_8760 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
    let c_8460 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8460) as usize);
    let c_7ab0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let c_8280 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8280) as usize);
    let c_81b0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_81b0) as usize);
    let c_88d0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_88d0) as usize);

    let idx7 = rd::<u8>(param_1 + 0x07) as usize;
    let mut r10 = rd::<usize>(param_1 + 0x08 + idx7 * 8);
    let idx6 = rd::<u8>(param_1 + 0x06) as usize;
    let mut r9 = rd::<usize>(param_1 + 0x08 + idx6 * 8);
    let rax_18 = rd::<usize>(param_1 + 0x18);
    let count = rd::<u32>(rax_18) as usize;
    let mut r14 = rd::<usize>(rax_18 + 0x10);
    let r8_end = r9.wrapping_add(count << 5);

    let xmm10 = vbroadcastss(param_3.to_bits());
    let xmm2_sub = vsubps(xmm10, c_8240);
    let xmm1_sub = vsubps(xmm10, c_8120);
    let xmm15 = vmulps(xmm1_sub, xmm1_sub);
    let xmm1_mul = vmulps(xmm1_sub, xmm10);
    let xmm8_saved = vmulps(xmm2_sub, xmm1_mul);

    if r9 >= r8_end {
        return;
    }

    let mut rbx = r9.wrapping_add(0x10);
    let mut rdi = r10.wrapping_add(0x10);
    let mut rsi = param_2;
    let default_parent: [u128; 2] = [c_81d0, 0];

    while r9 < r8_end {
        let mut xmm7 = rd::<u128>(r10);
        let r9_q = rd::<u128>(r9);
        let xmm0_dp = vdpps(xmm7, r9_q, 0xff);
        let ecx = rd::<u16>(r14);
        let xmm5_sign = c_8760 & xmm0_dp;
        let xmm3_abs = xmm5_sign ^ xmm0_dp;

        let mut xmm1 = vfmadd231ps_128(c_8840, xmm3_abs, c_80c0);
        let mut xmm4 = vfmadd231ps_128(c_81b0, xmm3_abs, xmm1);
        let xmm9_poly = vfnmadd231ps_128(c_84c0, xmm3_abs, c_82c0);
        xmm1 = vfmadd231ps_128(c_88d0, xmm3_abs, xmm9_poly);
        let xmm0_poly = vfmadd231ps_128(c_8280, xmm3_abs, xmm1);
        xmm4 = vfmadd231ps_128(xmm4, xmm0_poly, xmm15);

        let rdx_parent = if ecx != 0xffff {
            param_2.wrapping_add((ecx as usize) << 5)
        } else {
            default_parent.as_ptr() as usize
        };
        r14 = r14.wrapping_add(2);
        r10 = r10.wrapping_add(0x20);

        let mut xmm0_w = vfmadd231ps_128(xmm10, xmm4, xmm8_saved);
        let mut xmm8_pos = rd::<u128>(rdi);
        rdi = rdi.wrapping_add(0x20);
        xmm7 = vfnmadd231ps_128(xmm7, xmm0_w, xmm7);
        xmm0_w ^= xmm5_sign;
        xmm7 = vfmadd231ps_128(xmm7, xmm0_w, r9_q);

        let xmm3 = vpermilps(xmm7, 0x11);
        let xmm2 = vpermilps(xmm7, 0xbb);
        xmm8_pos = vfnmadd231ps_128(xmm8_pos, xmm8_pos, xmm10);
        let xmm9_p = rd::<u128>(rdx_parent);
        xmm8_pos = vfmadd231ps_128(xmm8_pos, xmm10, rd::<u128>(rbx));
        rbx = rbx.wrapping_add(0x20);

        let xmm1_q = vmulps(xmm3, xmm9_p);
        let xmm5_q = vpermilps(xmm9_p, 0x1b);
        let xmm0_q = vmulps(xmm2, xmm5_q);
        let xmm4_q = vhsubps(xmm1_q, xmm0_q);
        let xmm0_q2 = vmulps(xmm3, xmm5_q);
        let xmm2_q = vmulps(xmm2, xmm9_p);
        let xmm3_q = vhaddps(xmm2_q, xmm0_q2);
        let xmm0_sh = vshufps(xmm4_q, xmm3_q, 0xb1);
        let xmm1_bl = vblendps(xmm3_q, xmm4_q, 0x0c);
        let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);
        let xmm5 = vpermilps(xmm1_as, 0x9c);
        let xmm4_p = vpermilps(xmm9_p, 0xc9);
        let xmm0_n = vdpps(xmm5, xmm5, 0xff);
        let xmm2_rsq = vrsqrtps_128(xmm0_n);
        let xmm0_m = vmulps(xmm2_rsq, xmm0_n);
        let xmm1_m = vmulps(xmm0_m, xmm2_rsq);
        let xmm3_s = vsubps(c_8460, xmm1_m);
        let xmm2_h = vmulps(xmm2_rsq, c_8120);
        let xmm0_sc = vmulps(xmm3_s, xmm2_h);
        let xmm1_out = vmulps(xmm0_sc, xmm5);
        wr::<u128>(rsi, xmm1_out);

        let xmm1_t = vpermilps(xmm8_pos, 0xd2);
        let xmm2_t = vmulps(xmm1_t, xmm4_p);
        let xmm6_p = vpermilps(xmm9_p, 0xd2);
        let xmm0_t = vpermilps(xmm8_pos, 0xc9);
        let xmm3_t = vmulps(xmm0_t, xmm6_p);
        let xmm0_c = vsubps(xmm2_t, xmm3_t);
        let xmm5_c = vmulps(xmm0_c, c_7ab0);
        let xmm1_c = vpermilps(xmm5_c, 0xd2);
        let xmm3_c = vmulps(xmm1_c, xmm4_p);
        let xmm1_w = vpermilps(xmm9_p, 0xff);
        let xmm0_c2 = vpermilps(xmm5_c, 0xc9);
        let xmm2_c = vmulps(xmm0_c2, xmm6_p);
        let xmm4_c = vsubps(xmm3_c, xmm2_c);
        let xmm0_w2 = vmulps(xmm1_w, xmm5_c);
        let xmm2_a = vaddps(xmm0_w2, xmm8_pos);
        let xmm3_a = vaddps(xmm4_c, xmm2_a);
        let xmm1_pos_out = vaddps(xmm3_a, rd::<u128>(rdx_parent + 0x10));
        wr::<u128>(rsi + 0x10, xmm1_pos_out);

        rsi = rsi.wrapping_add(0x20);
        r9 = r9.wrapping_add(0x20);
    }
}

// ============================================================================
// Differential tests
// ============================================================================

pub unsafe fn diff_r_1420afda0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32, i64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SORT_BASE_ARGS: (usize, u32) = (0, 0);
    static mut JOB_SLOTS: [usize; 2] = [0; 2];
    static mut JOB_IDX: usize = 0;
    static mut ENQUEUED: [usize; 2] = [0; 2];
    static mut SEM_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_base_sort(ptr: usize, cnt: u32) {
        unsafe { SORT_BASE_ARGS = (ptr, cnt) };
    }
    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_alloc_job(_pool: usize, out_ptr: usize) {
        unsafe {
            let idx = JOB_IDX & 1;
            JOB_IDX += 1;
            wr::<usize>(out_ptr, JOB_SLOTS[idx]);
        }
    }
    unsafe extern "win64" fn mock_enqueue(_sched: usize, job: usize) {
        unsafe {
            if ENQUEUED[0] == 0 {
                ENQUEUED[0] = job;
            } else {
                ENQUEUED[1] = job;
            }
        }
    }
    unsafe extern "win64" fn mock_rel_sem(_h: usize, _cnt: u32, _prev: usize) -> i32 {
        unsafe { SEM_COUNT += 1 };
        1
    }
    unsafe extern "win64" fn mock_pivot(arr: usize, lo: i64, hi: i64) {
        let _ = (arr, lo, hi);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4103_a0c0, mock_base_sort as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc_job as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_6460, mock_enqueue as *const ())?;
    let _m5 = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;

    let g_job_va = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_job = rd::<usize>(g_job_va);

    let run = |f: F, max_depth: u32| -> ([u32; 6], (usize, u32), [u8; 0x58], [u8; 0x58], u32, u32) {
        let mut arr: [u32; 6] = [40, 10, 50, 20, 60, 30];
        let mut job0 = [0u8; 0x58];
        let mut job1 = [0u8; 0x58];
        unsafe {
            SORT_BASE_ARGS = (0, 0);
            JOB_SLOTS = [job0.as_mut_ptr() as usize, job1.as_mut_ptr() as usize];
            JOB_IDX = 0;
            ENQUEUED = [0, 0];
            SEM_COUNT = 0;
        }

        let mut sched_inner = vec![0u8; 0x1020];
        wr::<usize>(sched_inner.as_mut_ptr() as usize + 0x1008, 0x7777);
        let mut sched = [0u8; 0x90];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sched_inner.as_mut_ptr() as usize);
        wr::<usize>(sp + 0x48, 0x2000);

        let mut g_obj = [0u8; 0x20];
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sp);
        wr::<usize>(g_job_va, g_obj.as_mut_ptr() as usize);

        let mut handle_cnt: u32 = 1;
        let handle_ptr: usize = (&mut handle_cnt as *mut u32) as usize;
        let seg: [u64; 3] = [0, 6, 1];
        let pivot_fn = mock_pivot as *const () as usize;

        f(
            seg.as_ptr() as usize,
            arr.as_mut_ptr() as usize,
            max_depth,
            2,
            (&handle_ptr as *const usize) as usize,
            pivot_fn,
        );

        let base_off = if unsafe { SORT_BASE_ARGS.0 } != 0 {
            (
                unsafe { SORT_BASE_ARGS.0 }.wrapping_sub(arr.as_ptr() as usize),
                unsafe { SORT_BASE_ARGS.1 },
            )
        } else {
            (0, 0)
        };
        // Zero struct tail-padding u32 holes (+0x2c and +0x3c) that hold uninitialized stack bytes in orig
        wr::<u32>(job0.as_mut_ptr() as usize + 0x2c, 0);
        wr::<u32>(job0.as_mut_ptr() as usize + 0x3c, 0);
        wr::<usize>(job0.as_mut_ptr() as usize + 0x08, 0);
        wr::<usize>(job0.as_mut_ptr() as usize + 0x30, 0);
        wr::<usize>(job0.as_mut_ptr() as usize + 0x48, 0);
        wr::<u32>(job1.as_mut_ptr() as usize + 0x2c, 0);
        wr::<u32>(job1.as_mut_ptr() as usize + 0x3c, 0);
        wr::<usize>(job1.as_mut_ptr() as usize + 0x08, 0);
        wr::<usize>(job1.as_mut_ptr() as usize + 0x30, 0);
        wr::<usize>(job1.as_mut_ptr() as usize + 0x48, 0);

        (arr, base_off, job0, job1, handle_cnt, unsafe { SEM_COUNT })
    };

    for max_depth in [1u32, 5u32] {
        let o = run(f_o, max_depth);
        let n = run(f_n, max_depth);
        wr::<usize>(g_job_va, s_job);
        crate::diff_eq!(o, n, "1420afda0 max_depth={max_depth}");
    }
    Ok(2)
}

pub unsafe fn diff_r_1420b1180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: (u16, [u64; 2], [u64; 2], [u64; 2], [u64; 4]) =
        (0, [0; 2], [0; 2], [0; 2], [0; 4]);

    unsafe extern "win64" fn mock_b2330(
        a1: u16,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) {
        unsafe {
            CAPTURED = (
                a1,
                rd::<[u64; 2]>(a2),
                rd::<[u64; 2]>(a3),
                rd::<[u64; 2]>(a4),
                [a5, a6, a7, a8],
            );
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_420b_2330, mock_b2330 as *const ())?;

    let mut obj = [0u8; 0x60];
    let p = obj.as_mut_ptr() as usize;
    wr::<u16>(p + 0x08, 0x1234);
    wr::<[u64; 2]>(p + 0x10, [11, 22]);
    wr::<[u64; 2]>(p + 0x20, [33, 44]);
    wr::<[u64; 2]>(p + 0x30, [55, 66]);
    wr::<[u64; 4]>(p + 0x40, [77, 88, 99, 111]);

    unsafe { CAPTURED = (0, [0; 2], [0; 2], [0; 2], [0; 4]) };
    f_o(p);
    let o = unsafe { CAPTURED };

    unsafe { CAPTURED = (0, [0; 2], [0; 2], [0; 2], [0; 4]) };
    f_n(p);
    let n = unsafe { CAPTURED };

    crate::diff_eq!(o, n, "1420b1180 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b2f80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: (u64, [u64; 2], u64, u64) = (0, [0; 2], 0, 0);

    unsafe extern "win64" fn mock_b33f0(a1: u64, a2: usize, a3: u64, a4: u64) {
        unsafe {
            CAPTURED = (a1, rd::<[u64; 2]>(a2), a3, a4);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_420b_33f0, mock_b33f0 as *const ())?;

    let mut obj = [0u8; 0x30];
    let p = obj.as_mut_ptr() as usize;
    wr::<u64>(p + 0x08, 0xaaaa);
    wr::<u64>(p + 0x10, 0xbbbb);
    wr::<u64>(p + 0x18, 0xcccc);
    wr::<[u64; 2]>(p + 0x20, [0x1111, 0x2222]);

    unsafe { CAPTURED = (0, [0; 2], 0, 0) };
    f_o(p);
    let o = unsafe { CAPTURED };

    unsafe { CAPTURED = (0, [0; 2], 0, 0) };
    f_n(p);
    let n = unsafe { CAPTURED };

    crate::diff_eq!(o, n, "1420b2f80 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b2ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: (u64, [u64; 2], u64) = (0, [0; 2], 0);

    unsafe extern "win64" fn mock_f03b0(a1: u64, a2: usize, a3: u64) {
        unsafe {
            CAPTURED = (a1, rd::<[u64; 2]>(a2), a3);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_03b0, mock_f03b0 as *const ())?;

    let mut obj = [0u8; 0x30];
    let p = obj.as_mut_ptr() as usize;
    wr::<u64>(p + 0x08, 0x1000);
    wr::<[u64; 2]>(p + 0x10, [0x3333, 0x4444]);
    wr::<u64>(p + 0x20, 0x5555);

    unsafe { CAPTURED = (0, [0; 2], 0) };
    f_o(p);
    let o = unsafe { CAPTURED };

    unsafe { CAPTURED = (0, [0; 2], 0) };
    f_n(p);
    let n = unsafe { CAPTURED };

    crate::diff_eq!(o, n, "1420b2ff0 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b33f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_d3b20(_span: usize, out: usize, start_bit: u64) -> usize {
        if start_bit == 0 {
            wr::<u64>(out, 1);
            wr::<u8>(out + 0x08, 1);
        } else {
            wr::<u64>(out, 0);
            wr::<u8>(out + 0x08, 0);
        }
        out
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_3b20, mock_d3b20 as *const ())?;

    let run = |f: F| -> ([u16; 20], u32, [u16; 20], u32, [u32; 20], u32, [u64; 2]) {
        let mut arr0: [u16; 20] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 42, 19, 20,
        ];
        let mut arr1: [u16; 20] = [
            1, 42, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 42, 20,
        ];
        let mut arr2: [u32; 20] = [
            101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
            118, 119, 120,
        ];

        let mut slots = vec![0u8; 2 * 0x30];
        let s1 = slots.as_mut_ptr() as usize + 0x30;
        wr::<usize>(s1, arr0.as_mut_ptr() as usize);
        wr::<u32>(s1 + 0x08, 20);
        wr::<usize>(s1 + 0x10, arr1.as_mut_ptr() as usize);
        wr::<u32>(s1 + 0x18, 20);
        wr::<usize>(s1 + 0x20, arr2.as_mut_ptr() as usize);
        wr::<u32>(s1 + 0x28, 20);

        let mut bit_words: [u64; 2] = [u64::MAX, u64::MAX];
        let mut p1 = vec![0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x10, slots.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x70, bit_words.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0xb0, 0);

        let ids: [u16; 1] = [42];
        let span: [u64; 2] = [1, ids.as_ptr() as u64];
        f(p1p, span.as_ptr() as usize, 0, 70);

        (
            arr0,
            rd::<u32>(s1 + 0x08),
            arr1,
            rd::<u32>(s1 + 0x18),
            arr2,
            rd::<u32>(s1 + 0x28),
            bit_words,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b33f0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420bd220(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: [(usize, u64, u32); 2] = [(0, 0, 0); 2];
    static mut CALL_CNT: usize = 0;

    unsafe extern "win64" fn mock_cb(a1: usize, a2: u64, a3: u32) {
        unsafe {
            let idx = CALL_CNT & 1;
            CALL_CNT += 1;
            CALL_LOG[idx] = (a1, a2, a3);
        }
    }

    let run = |f: F| -> ([(usize, u64, u32); 2], usize) {
        unsafe {
            CALL_LOG = [(0, 0, 0); 2];
            CALL_CNT = 0;
        }

        let mut vtbl = [0u8; 0xa0];
        let vp = vtbl.as_mut_ptr() as usize;
        wr::<usize>(vp + 0x30, mock_cb as *const () as usize);
        wr::<usize>(vp + 0x80, mock_cb as *const () as usize);
        let f_18_ptr = vp;

        let f_10: [u16; 2] = [0, 1];
        let f_28: [u16; 1] = [1];
        let f_60: [u32; 2] = [0x1111, 0x2222];

        let off_arr0: [u32; 2] = [0x10, 0x20];
        let off_arr1: [u32; 2] = [0x30, 0x40];
        let mut f_48 = [0u8; 0x60];
        wr::<u64>(f_48.as_mut_ptr() as usize + 0x08, 0x1000);
        wr::<usize>(f_48.as_mut_ptr() as usize + 0x18, off_arr0.as_ptr() as usize);
        let mut f_38 = [0u8; 0x30];
        wr::<u64>(f_38.as_mut_ptr() as usize + 0x08, 0x2000);
        wr::<usize>(f_38.as_mut_ptr() as usize + 0x18, off_arr1.as_ptr() as usize);

        let sub_u16: [u16; 2] = [0, 1];
        let mut f_50 = vec![0u8; 0x180a0];
        let f50p = f_50.as_mut_ptr() as usize;
        wr::<usize>(f50p + 0x18058, sub_u16.as_ptr() as usize);
        wr::<u32>(f50p + 0x1806c, 2);
        wr::<usize>(f50p + 0x20 + 0x18058, sub_u16.as_ptr() as usize);
        wr::<u32>(f50p + 0x20 + 0x1806c, 2);

        let mut ctx = [0u8; 0x70];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x10, f_10.as_ptr() as usize);
        wr::<usize>(cp + 0x18, (&f_18_ptr as *const usize) as usize);
        wr::<u64>(cp + 0x20, 1);
        wr::<usize>(cp + 0x28, f_28.as_ptr() as usize);
        wr::<usize>(cp + 0x38, f_38.as_mut_ptr() as usize);
        wr::<usize>(cp + 0x48, f_48.as_mut_ptr() as usize);
        wr::<usize>(cp + 0x50, f50p);
        wr::<usize>(cp + 0x60, f_60.as_ptr() as usize);

        let mut node0 = [0u8; 0x20];
        wr::<u64>(node0.as_mut_ptr() as usize + 0x08, 3);
        wr::<u32>(node0.as_mut_ptr() as usize + 0x10, 0);
        wr::<u32>(node0.as_mut_ptr() as usize + 0x14, 0);
        wr::<u32>(node0.as_mut_ptr() as usize + 0x18, 1);

        let mut node2 = [0u8; 0x20];
        wr::<u64>(node2.as_mut_ptr() as usize + 0x08, 3);
        wr::<u16>(node2.as_mut_ptr() as usize + 0x10, 0);
        wr::<u16>(node2.as_mut_ptr() as usize + 0x12, 0);
        wr::<u16>(node2.as_mut_ptr() as usize + 0x14, 1);

        let mut node1 = [0u8; 0x28];
        wr::<u64>(node1.as_mut_ptr() as usize + 0x08, 3);
        wr::<u64>(node1.as_mut_ptr() as usize + 0x10, 0);
        wr::<u64>(node1.as_mut_ptr() as usize + 0x18, 0xaaaa);
        wr::<u64>(node1.as_mut_ptr() as usize + 0x20, 0xbbbb);

        let p1: [u64; 8] = [
            cp as u64,
            3,
            node0.as_mut_ptr() as u64,
            0,
            node2.as_mut_ptr() as u64,
            0,
            node1.as_mut_ptr() as u64,
            0,
        ];
        f(p1.as_ptr() as usize);
        (unsafe { CALL_LOG }, unsafe { CALL_CNT })
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420bd220 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420cd120(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, flag_4fd8: u8, op: u16| -> (u8, u32, u32) {
        let mut p1 = vec![0u8; 0x5000];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p + 0x4fd8, flag_4fd8);

        let mut p2 = vec![0u8; 0x200];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x40, u32::MAX);
        wr::<u32>(p2p + 0x44, 0);
        wr::<u64>(p2p + 0x138, 2);
        wr::<u64>(p2p + 0x140, 1);

        let vals_x: [f32; 6] = [1.0, 1.0, 2.5, 0.0, 3.0, 3.0];
        let vals_y: [f32; 6] = [2.0, 5.0, 5.0, 0.0, 4.0, 1.0];
        let vals_10: [f32; 6] = [1.0, 1.0, 1.0, 2.0, 5.0, 5.0];
        let vals_14: [f32; 6] = [2.0, 2.0, 3.0, 2.0, 6.0, 7.0];
        let vals_18: [f32; 6] = [0.0, 0.5, 0.5, 1.0, 2.0, 1.0];
        let vals_1c: [f32; 6] = [1.0, 0.2, 0.8, 0.0, -1.0, -1.0];

        for slot in 0..6usize {
            let base = p2p + 0x48 + slot * 0x28;
            wr::<f32>(base, vals_x[slot]);
            wr::<f32>(base + 0x04, vals_y[slot]);
            wr::<f32>(base + 0x10, vals_10[slot]);
            wr::<f32>(base + 0x14, vals_14[slot]);
            wr::<f32>(base + 0x18, vals_18[slot]);
            wr::<f32>(base + 0x1c, vals_1c[slot]);
            wr::<u8>(base + 0x20, (slot as u8) & 1);
            wr::<u8>(base + 0x21, ((slot + 1) as u8) & 1);
            wr::<u8>(base + 0x22, (slot as u8) + 10);
            wr::<u8>(base + 0x23, 5);
            wr::<u8>(base + 0x24, (slot as u8) * 3);
        }

        let r = f(p1p, p2p, op);
        (r, rd::<u32>(p2p + 0x40), rd::<u32>(p2p + 0x44))
    };

    for flag in [0u8, 1u8] {
        for op in 0x82u16..=0x94u16 {
            let o = run(f_o, flag, op);
            let n = run(f_n, flag, op);
            crate::diff_eq!(o, n, "1420cd120 flag={flag} op=0x{op:x}");
        }
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420d8810(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_e0380(_r14: usize, _entry: usize) {}
    unsafe extern "win64" fn mock_d98f0(_p1: usize, code: u16) -> u8 {
        (code == 0x12) as u8
    }
    unsafe extern "win64" fn mock_db7c0(entry: usize, code: u16, _p1: usize) -> u8 {
        (rd::<u16>(entry) == code) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_420e_0380, mock_e0380 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_98f0, mock_d98f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420d_b7c0, mock_db7c0 as *const ())?;

    let run = |f: F, code: u16| -> u8 {
        let mut node_5880 = [0u8; 0x28];
        let np = node_5880.as_mut_ptr() as usize;
        wr::<u16>(np + 0x18, 0x10);
        wr::<u16>(np + 0x1a, 0xce);

        let mut node_5a70 = [0u8; 0x28];
        wr::<u16>(node_5a70.as_mut_ptr() as usize, 0xce);

        let mut inner = vec![0u8; 0x5b20];
        let ip = inner.as_mut_ptr() as usize;
        wr::<usize>(ip + 0x5880, np);
        wr::<u32>(ip + 0x5888, 1);
        wr::<u32>(ip + 0x5aa8, u32::MAX);
        wr::<u32>(ip + 0x5aac, 0);
        wr::<usize>(ip + 0x5a70, node_5a70.as_mut_ptr() as usize);
        wr::<u32>(ip + 0x5a78, 1);
        wr::<u32>(ip + 0x5af8, u32::MAX);
        wr::<u32>(ip + 0x5afc, 0);

        let p1: [usize; 2] = [0, ip];
        f(p1.as_ptr() as usize, code)
    };

    for code in [0x10u16, 0x12, 0xce, 0xcf, 0x40ce, 0x40cf] {
        let o = run(f_o, code);
        let n = run(f_n, code);
        crate::diff_eq!(o, n, "1420d8810 code=0x{code:x}");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420d9330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u8) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_d2c00(_r15: usize, _code: u16, p3: u8) -> u8 {
        (p3 == 0xff) as u8
    }
    unsafe extern "win64" fn mock_e0380(_r15: usize, _entry: usize) {}
    unsafe extern "win64" fn mock_d0e90(_rcx: usize, _rdx: usize, code: u16, p3: u8) -> u8 {
        ((code == 0x95) as u8) ^ (p3 & 1)
    }
    unsafe extern "win64" fn mock_d7970(_rcx: usize, _rdx: usize, code: u16, _p3: u8) -> u8 {
        (code == 0x96) as u8
    }
    unsafe extern "win64" fn mock_ceef0(_rcx: usize, _rdx: usize, code: u16) -> u8 {
        (code == 0x02) as u8
    }
    unsafe extern "win64" fn mock_cd120(_rcx: usize, _rdx: usize, code: u16) -> u8 {
        (code == 0x84) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_420d_2c00, mock_d2c00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_0380, mock_e0380 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420d_0e90, mock_d0e90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420d_7970, mock_d7970 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420c_eef0, mock_ceef0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420c_d120, mock_cd120 as *const ())?;

    let run = |f: F, code: u16, p3: u8, use_40: bool| -> u8 {
        let mut sub_60 = [0u8; 0x50];
        if use_40 {
            wr::<u64>(sub_60.as_mut_ptr() as usize + 0x40, 1);
        } else {
            wr::<u64>(sub_60.as_mut_ptr() as usize + 0x38, 1);
        }
        let mut sub_48 = [0u8; 0x20];
        let mut sub_50 = [0u8; 0x20];

        let mut inner0 = [0u8; 0x70];
        let i0p = inner0.as_mut_ptr() as usize;
        wr::<usize>(i0p, 0x1234);
        wr::<usize>(i0p + 0x48, sub_48.as_mut_ptr() as usize);
        wr::<usize>(i0p + 0x50, sub_50.as_mut_ptr() as usize);
        wr::<usize>(i0p + 0x60, sub_60.as_mut_ptr() as usize);

        let mut node_5880 = [0u8; 0x28];
        wr::<u16>(node_5880.as_mut_ptr() as usize + 0x18, 0x01);
        wr::<u16>(node_5880.as_mut_ptr() as usize + 0x1a, 0x84);

        let mut inner1 = vec![0u8; 0x5b00];
        let i1p = inner1.as_mut_ptr() as usize;
        wr::<usize>(i1p + 0x5880, node_5880.as_mut_ptr() as usize);
        wr::<u32>(i1p + 0x5888, 1);
        wr::<u32>(i1p + 0x5aa8, u32::MAX);
        wr::<u32>(i1p + 0x5aac, 0);

        let p1: [usize; 2] = [i0p, i1p];
        f(p1.as_ptr() as usize, code, p3)
    };

    for (code, p3, use_40) in [
        (0x95u16, 0u8, false),
        (0x96u16, 0u8, true),
        (0x02u16, 0u8, false),
        (0x84u16, 0u8, false),
        (0x40ceu16, 0u8, false),
        (0x40cfu16, 0u8, false),
        (0x95u16, 0xffu8, false),
    ] {
        let o = run(f_o, code, p3, use_40);
        let n = run(f_n, code, p3, use_40);
        crate::diff_eq!(o, n, "1420d9330 code=0x{code:x}");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420de930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TARGET_CODE: u16 = 0;

    unsafe extern "win64" fn mock_f4260(_rdi: usize, code: u16, _zero: u8) -> u8 {
        (code == unsafe { TARGET_CODE }) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_4260, mock_f4260 as *const ())?;

    let run = |f: F, target: u16, has_38: bool| -> (u8, u8, u64, [u64; 4]) {
        unsafe { TARGET_CODE = target };

        let mut s48 = [0u8; 0x30];
        let mut s50 = [0u8; 0x30];
        let mut s58 = [0u8; 0x30];
        let mut s60 = [0u8; 0x50];
        wr::<u8>(s48.as_mut_ptr() as usize + 0x18, 1);
        wr::<u8>(s50.as_mut_ptr() as usize + 0x18, 1);
        wr::<u8>(s58.as_mut_ptr() as usize + 0x18, 1);
        wr::<u8>(s60.as_mut_ptr() as usize + 0x18, 1);
        if has_38 {
            wr::<u64>(s60.as_mut_ptr() as usize + 0x38, 1);
        } else {
            wr::<u64>(s60.as_mut_ptr() as usize + 0x40, 1);
        }

        let mut elem = [0u8; 0x78];
        let ep = elem.as_mut_ptr() as usize;
        wr::<usize>(ep + 0x48, s48.as_mut_ptr() as usize);
        wr::<usize>(ep + 0x50, s50.as_mut_ptr() as usize);
        wr::<usize>(ep + 0x58, s58.as_mut_ptr() as usize);
        wr::<usize>(ep + 0x60, s60.as_mut_ptr() as usize);

        let mut p1 = vec![0u8; 0x5900];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x4fc8, 0x1234_5678_9abc_def0);
        wr::<usize>(p1p + 0x4fe0, ep);
        wr::<u32>(p1p + 0x4fe8, 1);
        wr::<u8>(p1p + 0x5860, 9);
        wr::<u8>(p1p + 0x5861, 9);

        f(p1p);
        (
            rd::<u8>(p1p + 0x5860),
            rd::<u8>(p1p + 0x5861),
            rd::<u64>(p1p + 0x4fd0),
            [
                rd::<u64>(s48.as_ptr() as usize + 0x20),
                rd::<u64>(s50.as_ptr() as usize + 0x20),
                rd::<u64>(s58.as_ptr() as usize + 0x20),
                rd::<u64>(s60.as_ptr() as usize + 0x20),
            ],
        )
    };

    for (target, has_38) in [
        (0x95u16, true),
        (0x95u16, false),
        (0xb3u16, true),
        (0x85u16, false),
        (0x10u16, false),
        (0xbbu16, false),
    ] {
        let o = run(f_o, target, has_38);
        let n = run(f_n, target, has_38);
        crate::diff_eq!(o, n, "1420de930 target=0x{target:x}");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1420e3850(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_pred(arg: u64) -> u8 {
        (arg == 1) as u8
    }
    unsafe extern "win64" fn mock_dcac0(out_u16: usize, _p0: usize, code: u16) -> usize {
        wr::<u16>(out_u16, code.wrapping_add(10));
        out_u16
    }
    unsafe extern "win64" fn mock_d8810(pair_ptr: usize, resolved: u16) -> u8 {
        ((rd::<usize>(pair_ptr) != 0) as u8) & ((resolved == 30) as u8)
    }

    let _m1 = crate::difftest::Mock::new(0x1_420d_cac0, mock_dcac0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_8810, mock_d8810 as *const ())?;

    let run = |f: F, match_fc0: bool, pred_arg: u64, code: u16| -> u8 {
        let mut p0 = vec![0u8; 0x5000];
        let p0p = p0.as_mut_ptr() as usize;
        wr::<u64>(p0p + 0x4fc0, 0x7777);

        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, p0p);
        wr::<u64>(p1p + 0x08, if match_fc0 { 0x7777 } else { 0x8888 });
        wr::<u64>(p1p + 0x48, 1);
        wr::<u64>(p1p + 0x68, pred_arg);
        wr::<usize>(p1p + 0x70, mock_pred as *const () as usize);

        f(p1p, code)
    };

    for (match_fc0, pred_arg, code) in [
        (true, 1u64, 20u16),
        (true, 1u64, 21u16),
        (true, 0u64, 20u16),
        (false, 1u64, 20u16),
    ] {
        let o = run(f_o, match_fc0, pred_arg, code);
        let n = run(f_n, match_fc0, pred_arg, code);
        crate::diff_eq!(o, n, "1420e3850 out");
    }
    Ok(1)
}

pub unsafe fn diff_unknown_1421de220(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, t: f32| -> [u32; 16] {
        let pose_a: [f32; 16] = [
            0.0,
            0.0,
            0.0,
            1.0,
            1.0,
            2.0,
            3.0,
            0.0,
            0.0,
            0.707_106_77,
            0.0,
            0.707_106_77,
            -1.0,
            0.5,
            2.0,
            0.0,
        ];
        let pose_b: [f32; 16] = [
            0.0,
            0.0,
            0.707_106_77,
            0.707_106_77,
            3.0,
            4.0,
            5.0,
            0.0,
            0.5,
            0.5,
            0.5,
            0.5,
            2.0,
            -1.5,
            0.25,
            0.0,
        ];
        let parents: [u16; 2] = [0xffff, 0];

        let mut skel_hdr = [0u8; 0x20];
        let shp = skel_hdr.as_mut_ptr() as usize;
        wr::<u32>(shp, 2);
        wr::<usize>(shp + 0x10, parents.as_ptr() as usize);

        let mut p1 = [0u8; 0x28];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p + 0x06, 0);
        wr::<u8>(p1p + 0x07, 1);
        wr::<usize>(p1p + 0x08, pose_a.as_ptr() as usize);
        wr::<usize>(p1p + 0x10, pose_b.as_ptr() as usize);
        wr::<usize>(p1p + 0x18, shp);

        let mut out_pose = [0u32; 16];
        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            out_pose.as_mut_ptr() as usize as u64,
            t.to_bits() as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, t.to_bits() as u128, 0],
        );
        out_pose
    };

    for t in [0.25f32, 0.5f32, 0.75f32] {
        let o = run(orig, t);
        let n = run(new, t);
        crate::diff_eq!(o, n, "1421de220 t={t}");
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified_throws(0x1_420a_fda0, "r_1420afda0", rend::cauldron_hot_l::r_1420afda0 as *const (), Throws::Discard, rend::cauldron_hot_l::diff_r_1420afda0),
// Replacement::verified_throws(0x1_420b_1180, "r_1420b1180", rend::cauldron_hot_l::r_1420b1180 as *const (), Throws::Discard, rend::cauldron_hot_l::diff_r_1420b1180),
// Replacement::verified(0x1_420b_2f80, "r_1420b2f80", rend::cauldron_hot_l::r_1420b2f80 as *const (), rend::cauldron_hot_l::diff_r_1420b2f80),
// Replacement::verified(0x1_420b_2ff0, "r_1420b2ff0", rend::cauldron_hot_l::r_1420b2ff0 as *const (), rend::cauldron_hot_l::diff_r_1420b2ff0),
// Replacement::verified(0x1_420b_33f0, "r_1420b33f0", rend::cauldron_hot_l::r_1420b33f0 as *const (), rend::cauldron_hot_l::diff_r_1420b33f0),
// Replacement::verified(0x1_420b_d220, "unknown_1420bd220", rend::cauldron_hot_l::unknown_1420bd220 as *const (), rend::cauldron_hot_l::diff_unknown_1420bd220),
// Replacement::verified(0x1_420c_d120, "unknown_1420cd120", rend::cauldron_hot_l::unknown_1420cd120 as *const (), rend::cauldron_hot_l::diff_unknown_1420cd120),
// Replacement::verified_throws(0x1_420d_8810, "unknown_1420d8810", rend::cauldron_hot_l::unknown_1420d8810 as *const (), Throws::Discard, rend::cauldron_hot_l::diff_unknown_1420d8810),
// Replacement::verified(0x1_420d_9330, "unknown_1420d9330", rend::cauldron_hot_l::unknown_1420d9330 as *const (), rend::cauldron_hot_l::diff_unknown_1420d9330),
// Replacement::verified_throws(0x1_420d_e930, "unknown_1420de930", rend::cauldron_hot_l::unknown_1420de930 as *const (), Throws::Discard, rend::cauldron_hot_l::diff_unknown_1420de930),
// Replacement::verified_throws(0x1_420e_3850, "unknown_1420e3850", rend::cauldron_hot_l::unknown_1420e3850 as *const (), Throws::Discard, rend::cauldron_hot_l::diff_unknown_1420e3850),
// Replacement::verified(0x1_421d_e220, "unknown_1421de220", rend::cauldron_hot_l::unknown_1421de220 as *const (), rend::cauldron_hot_l::diff_unknown_1421de220),
