//! Decompiled `rend` hot functions shard ZA10 (`0x141e34de0..0x141e9a210`).
#![allow(static_mut_refs)]

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

#[inline(always)]
unsafe fn mul_mat3x4_48(src_a: usize, src_b: usize, dst: usize) {
    use crate::simd_ops::{vmulps, vpermilps, vshufps};
    let xmm11 = rd::<u128>(src_a);
    let xmm0_in = rd::<u128>(src_a + 0x10);
    let xmm9 = rd::<u128>(src_a + 0x20);
    let xmm6 = rd::<u128>(src_b);
    let xmm7 = rd::<u128>(src_b + 0x10);
    let xmm10 = vshufps(xmm11, xmm0_in, 0x4f);
    let xmm8 = vshufps(xmm0_in, xmm9, 0x0e);
    let xmm3_a = vshufps(xmm6, xmm7, 0x49);
    let xmm1_a = vpermilps(xmm8, 0x24);
    let xmm0_a = vpermilps(xmm3_a, 0xd5);
    let mut xmm4 = vmulps(xmm0_a, xmm1_a);
    let xmm2_a = vpermilps(xmm10, 0x38);
    let xmm0_b = vpermilps(xmm3_a, 0x80);
    xmm4 = vfmadd231ps_128(xmm4, xmm0_b, xmm2_a);
    let xmm1_b = vpermilps(xmm11, 0x24);
    let xmm0_c = vpermilps(xmm6, 0xc0);
    xmm4 = vfmadd231ps_128(xmm4, xmm0_c, xmm1_b);
    wr::<u128>(dst, xmm4);

    let xmm5 = rd::<u128>(src_b + 0x20);
    let xmm1_c = vpermilps(xmm8, 0x49);
    let xmm0_d = vshufps(xmm7, xmm5, 0x05);
    let mut xmm3 = vmulps(xmm0_d, xmm1_c);
    let xmm2_b = vpermilps(xmm10, 0x8e);
    let xmm0_e = vpermilps(xmm7, 0xf0);
    xmm3 = vfmadd231ps_128(xmm3, xmm0_e, xmm2_b);
    let xmm1_d = vpermilps(xmm11, 0x49);
    let xmm0_f = vshufps(xmm6, xmm7, 0xaf);
    xmm3 = vfmadd231ps_128(xmm3, xmm0_f, xmm1_d);
    wr::<u128>(dst + 0x10, xmm3);

    let xmm2_c = vshufps(xmm7, xmm5, 0x9e);
    let xmm0_g = vshufps(0, xmm9, 0x54);
    let mut xmm3_out = vshufps(xmm0_g, xmm9, 0xe8);
    let xmm1_e = vpermilps(xmm8, 0x92);
    let xmm0_h = vpermilps(xmm5, 0xfc);
    xmm3_out = vfmadd231ps_128(xmm3_out, xmm0_h, xmm1_e);
    let xmm1_f = vpermilps(xmm10, 0xe7);
    let xmm0_i = vpermilps(xmm2_c, 0xfd);
    xmm3_out = vfmadd231ps_128(xmm3_out, xmm0_i, xmm1_f);
    let xmm1_g = vpermilps(xmm11, 0x92);
    let xmm0_j = vpermilps(xmm2_c, 0xa8);
    xmm3_out = vfmadd231ps_128(xmm3_out, xmm0_j, xmm1_g);
    wr::<u128>(dst + 0x20, xmm3_out);
}

/// `unknown_141e34de0`
/// Original: 0x141e34de0 (598 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e34de0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    if rd::<i32>(param_2) != -1 {
        return;
    }

    let mut max_depth: i32 = 0;
    let mut rbx = rd::<usize>(param_3);
    let len = rd::<u32>(param_3 + 0x08) as usize;
    let rdi = rbx.wrapping_add(len * 0x20);

    while rbx != rdi {
        let r9_world = rd::<usize>(param_1);
        let count_u16 = rd::<usize>(param_1 + 0x08);
        let sorted_u16 = rd::<usize>(param_1 + 0x10);
        let handle = rd::<u64>(rbx + 0x18);
        let idx = (handle as u32) as usize;
        let cap_20 = rd::<u32>(r9_world + 0x20) as usize;

        if idx < cap_20 {
            let arr_18 = rd::<usize>(r9_world + 0x18);
            let entry = rd::<u64>(arr_18 + idx * 8);
            if (entry as u32) == (handle as u32)
                && ((entry >> 32) as u32) == ((handle >> 32) as u32)
                && (handle as u32) != u32::MAX
            {
                let arr_30 = rd::<usize>(r9_world + 0x30);
                let loc0 = rd::<u64>(arr_30 + idx * 8);
                let dx = loc0 as u16;
                if dx == 0xffff {
                    let abort_fn = rd::<u64>(exe::ptr::<u64>(0x1_4264_67e8) as usize);
                    exe::call_va!(abort_fn, ());
                    return;
                }

                let mut lo_ptr = sorted_u16;
                let end_ptr = sorted_u16.wrapping_add(count_u16 * 2);
                let mut rem = count_u16 as isize;
                while rem > 0 {
                    let half = (rem as usize) >> 1;
                    let mid_ptr = lo_ptr.wrapping_add(half * 2);
                    if rd::<u16>(mid_ptr) < dx {
                        lo_ptr = mid_ptr.wrapping_add(2);
                        rem = rem - 1 - (half as isize);
                    } else {
                        rem = half as isize;
                    }
                }

                if lo_ptr != end_ptr && dx >= rd::<u16>(lo_ptr) {
                    let r14_world = rd::<usize>(param_1);
                    let arr_30_b = rd::<usize>(r14_world + 0x30);
                    let r12_loc = rd::<u64>(arr_30_b + idx * 8);
                    let arch = r12_loc as u16;
                    if arch == 0xffff {
                        let abort_fn = rd::<u64>(exe::ptr::<u64>(0x1_4264_67e8) as usize);
                        exe::call_va!(abort_fn, ());
                        return;
                    }

                    let comp_d48 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_8d48) as usize) as u32;
                    let arch_usize = arch as usize;
                    let arch_off = arch_usize << 5;
                    let r15_chunk = rd::<usize>(r14_world + 0x88 + arch_usize * 8);
                    let r11d_cnt = rd::<u32>(r14_world + 0x180ac + arch_off);
                    let rbp_keys = rd::<usize>(r14_world + 0x18098 + arch_off);
                    let r14_offs = rd::<usize>(r14_world + 0x180a0 + arch_off);

                    let slot1 = exe::call_va!(
                        0x1_4248_9ee0,
                        (rbp_keys, r11d_cnt, comp_d48) -> i32
                    );
                    let mut local_ctx: [u64; 3] = [
                        rd::<u64>(param_1),
                        rd::<u64>(param_1 + 0x08),
                        rd::<u64>(param_1 + 0x10),
                    ];
                    let row = (r12_loc >> 32) as usize;
                    let off1 = rd::<u32>((r14_offs as isize + (slot1 as isize) * 4) as usize) as usize;
                    let rsi_depth = r15_chunk.wrapping_add(off1).wrapping_add(row * 4);

                    let comp_d4c = rd::<u16>(exe::ptr::<u16>(0x1_43cc_8d4c) as usize) as u32;
                    let slot2 = exe::call_va!(
                        0x1_4248_9ee0,
                        (rbp_keys, r11d_cnt, comp_d4c) -> i32
                    );
                    let off2 = rd::<u32>((r14_offs as isize + (slot2 as isize) * 4) as usize) as usize;
                    let r8_children = r15_chunk.wrapping_add(off2).wrapping_add(row * 0x10);

                    unknown_141e34de0(
                        local_ctx.as_mut_ptr() as usize,
                        rsi_depth,
                        r8_children,
                    );

                    let child_depth = rd::<i32>(rsi_depth).wrapping_add(1);
                    if max_depth < child_depth {
                        max_depth = child_depth;
                    }
                }
            }
        }
        rbx = rbx.wrapping_add(0x20);
    }

    wr::<i32>(param_2, max_depth);
}

/// `unknown_141e56a50`
/// Original: 0x141e56a50 (865 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e56a50(
    param_1: u32,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) {
    let rax = exe::call_va!(0x1_4021_7df0, (param_2) -> usize);
    let r11_base = rd::<usize>(rax + 0x20);
    let len_28 = rd::<u32>(rax + 0x28) as usize;
    let r10_end = r11_base.wrapping_add(len_28 * 0x30);

    if r11_base != r10_end {
        let mut rsi_buf: usize = 0;
        let mut r14_len: usize = 0;
        let mut r12_cap: usize = 0;

        if rd::<u32>(param_3 + 0x18) != 0 {
            let mut bx: u16 = 0;
            loop {
                let edi = bx as usize;
                let rcx_arr = rd::<usize>(param_3 + 0x10);
                if rd::<u64>(rcx_arr + edi * 0x80 + 0x60) == rd::<u64>(param_2) {
                    if r14_len * 2 == r12_cap * 2 {
                        let mut new_cap = (r12_cap >> 1) + r12_cap;
                        if new_cap < r14_len + 1 {
                            new_cap = r14_len + 1;
                        }
                        r12_cap = new_cap;
                        let new_ptr = exe::call_va!(
                            0x1_4241_c0a0,
                            ((new_cap * 2) as u64, 2u32) -> usize
                        );
                        exe::call_va!(
                            0x1_4183_acc8,
                            (new_ptr, rsi_buf, (r14_len * 2) as u64)
                        );
                        if rsi_buf != 0 {
                            exe::call_va!(0x1_4241_c190, (rsi_buf));
                        }
                        rsi_buf = new_ptr;
                    }
                    wr::<u16>(rsi_buf + r14_len * 2, bx);
                    r14_len += 1;
                    let flags_30 = rd::<usize>(param_3 + 0x30);
                    wr::<u8>(flags_30 + edi, u8::from(param_1 == 1));
                }
                bx = bx.wrapping_add(1);
                let next_edi = bx as usize;
                if next_edi == (rd::<u32>(param_3 + 0x18) as usize) {
                    break;
                }
            }
        }

        let rcx_p5_0 = rd::<usize>(param_5);
        let idx_10c = rd::<u16>(exe::ptr::<u16>(0x1_43cc_b10c) as usize) as usize;
        let map_20 = rd::<usize>(rcx_p5_0 + 0x20);
        let slot_idx = rd::<u16>(map_20 + idx_10c * 2) as usize;
        let arr_08 = rd::<usize>(param_5 + 0x08);
        let q0 = rd::<u32>(arr_08 + slot_idx * 0x10 + 0x08) as u64;
        let q1 = rd::<u64>(arr_08 + slot_idx * 0x10);
        let r11_tbl = param_5.wrapping_add(0x40);

        let mut query_pair: [u64; 2] = [q0, q1];
        let mut out_11: [u64; 3] = [0; 3];
        exe::call_va!(
            0x1_401e_7c10,
            (
                out_11.as_mut_ptr() as usize,
                r11_tbl,
                query_pair.as_mut_ptr() as usize
            )
        );

        let mut iter_buf: [u64; 10] = [0; 10];
        let iter_ptr = iter_buf.as_mut_ptr() as usize;
        wr::<usize>(iter_ptr, r11_tbl);
        wr::<u64>(iter_ptr + 0x08, out_11[0]);
        wr::<u64>(iter_ptr + 0x10, out_11[1]);
        wr::<u64>(iter_ptr + 0x18, out_11[2]);
        wr::<u64>(iter_ptr + 0x28, 0);
        wr::<u64>(iter_ptr + 0x30, 0);

        if out_11[1] != out_11[2] {
            let arch = rd::<u16>(out_11[1] as usize) as usize;
            let chunk = rd::<usize>(r11_tbl + arch * 8 + 0x48);
            wr::<usize>(iter_ptr + 0x20, chunk);
            let cnt = if chunk != 0 {
                rd::<u32>(r11_tbl + arch * 4 + 0x10048) as u64
            } else {
                0
            };
            wr::<u64>(iter_ptr + 0x28, cnt);
        }

        exe::call_va!(0x1_40fa_e190, (iter_ptr));
        let mut rdi = rd::<u64>(iter_ptr + 0x30) as usize;
        let mut r15 = rd::<u64>(iter_ptr + 0x28) as usize;

        while rdi < r15 {
            while rdi < r15 {
                let r9_mask_base = rd::<usize>(iter_ptr + 0x38).wrapping_add(rdi << 5);
                let r12_visited = rd::<usize>(iter_ptr + 0x40);
                if rd::<u8>(r12_visited + rdi) == 0 {
                    for k in 0..r14_len {
                        let bit_idx = rd::<u16>(rsi_buf + k * 2) as usize;
                        let word = rd::<u64>(r9_mask_base + (bit_idx >> 6) * 8);
                        if ((word >> (bit_idx & 0x3f)) & 1) != 0 {
                            let chunk = rd::<usize>(iter_ptr + 0x20);
                            let val = rd::<u64>(chunk + rdi * 8 + 0x10);
                            let cur_len = rd::<u32>(param_4 + 0x08) as u64;
                            let cur_cap = rd::<u32>(param_4 + 0x0c) as u64;
                            if cur_len * 8 == cur_cap * 8 {
                                let mut new_cap = (cur_cap >> 1) + cur_cap;
                                if new_cap < cur_len + 1 {
                                    new_cap = cur_len + 1;
                                }
                                exe::call_va!(0x1_401d_eba0, (param_4, new_cap));
                            }
                            let idx = rd::<u32>(param_4 + 0x08) as usize;
                            let dst_arr = rd::<usize>(param_4);
                            wr::<u64>(dst_arr + idx * 8, val);
                            wr::<u32>(param_4 + 0x08, (idx as u32).wrapping_add(1));
                            wr::<u8>(r12_visited + rdi, 1);
                            break;
                        }
                    }
                }
                rdi += 1;
                wr::<u64>(iter_ptr + 0x30, rdi as u64);
            }

            let mut cur_u16 = rd::<usize>(iter_ptr + 0x10).wrapping_add(2);
            wr::<usize>(iter_ptr + 0x10, cur_u16);
            let end_u16 = rd::<usize>(iter_ptr + 0x18);
            if cur_u16 != end_u16 {
                let rdx_tbl = rd::<usize>(iter_ptr + 0x08);
                while cur_u16 != end_u16 {
                    let arch = rd::<u16>(cur_u16) as usize;
                    if rd::<u64>(rdx_tbl + arch * 8 + 0x48) != 0
                        && rd::<u32>(rdx_tbl + arch * 4 + 0x10048) != 0
                    {
                        break;
                    }
                    cur_u16 = cur_u16.wrapping_add(2);
                }
                wr::<usize>(iter_ptr + 0x10, cur_u16);
                if cur_u16 != end_u16 {
                    let rdx_0 = rd::<usize>(iter_ptr);
                    let arch = rd::<u16>(cur_u16) as usize;
                    let chunk = rd::<usize>(rdx_0 + arch * 8 + 0x48);
                    wr::<usize>(iter_ptr + 0x20, chunk);
                    let cnt = if chunk != 0 {
                        rd::<u32>(rdx_0 + arch * 4 + 0x10048) as u64
                    } else {
                        0
                    };
                    wr::<u64>(iter_ptr + 0x28, cnt);
                    wr::<u64>(iter_ptr + 0x30, 0);
                }
            }
            exe::call_va!(0x1_40fa_e190, (iter_ptr));
            rdi = rd::<u64>(iter_ptr + 0x30) as usize;
            r15 = rd::<u64>(iter_ptr + 0x28) as usize;
        }

        if rsi_buf != 0 {
            exe::call_va!(0x1_4241_c190, (rsi_buf));
        }
    }

    crate::simd_ops::atomic_xchg_u64((param_2 + 0x08) as u64, 0);
}

/// `unknown_141e5c4f0`
/// Original: 0x141e5c4f0 (276 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e5c4f0(param_1: usize, param_2: u64) {
    let mut local_p1: usize = param_1;
    let mut local_p2: u64 = param_2;
    let mut idx_out: i64 = 0;

    let idx_08c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d08c) as usize) as usize;
    let tbl_580e0 = rd::<usize>(param_1 + 0x580e0);
    let rsi = rd::<usize>(tbl_580e0 + idx_08c * 8);

    let rbx: i64 = if rd::<u64>(rsi + 0x38) == rd::<u64>(rsi + 0x30) {
        rd::<i64>(rsi + 0x40)
    } else {
        exe::call_va!(
            0x1_401f_a730,
            (
                rsi,
                (&mut idx_out as *mut i64) as usize,
                (&mut local_p2 as *mut u64) as usize
            )
        );
        if idx_out == -1 {
            return;
        }
        idx_out
    };

    let base_58 = rd::<usize>(rsi + 0x58);
    let entry_ptr = base_58.wrapping_add((rbx as usize).wrapping_shl(4));
    let end_ptr = base_58.wrapping_add(rd::<usize>(rsi + 0x40).wrapping_shl(4));
    if entry_ptr == end_ptr {
        return;
    }

    let handle = rd::<u64>(entry_ptr + 0x08);
    if (handle as u32) == u32::MAX {
        return;
    }

    let ref_ptr = exe::call_va!(
        0x1_40fb_6eb0,
        ((&mut local_p1 as *mut usize) as usize, handle) -> usize
    );
    let cur = rd::<u8>(ref_ptr);
    if cur == 0 {
        return;
    }
    let next = cur.wrapping_sub(1);
    wr::<u8>(ref_ptr, next);
    if next != 0 {
        return;
    }

    let ent_idx = (handle as u32) as usize;
    let arr_30 = rd::<usize>(param_1 + 0x30);
    let loc = rd::<u64>(arr_30 + ent_idx * 8);
    let arch = loc as u16;
    if arch == 0xffff {
        let abort_fn = rd::<u64>(exe::ptr::<u64>(0x1_4264_67e8) as usize);
        exe::call_va!(abort_fn, ());
        return;
    }

    let arch_off = (arch as usize) << 5;
    let edx_cnt = rd::<u32>(param_1 + 0x180a8 + arch_off);
    let r8d_comp = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d694) as usize) as u32;
    let rcx_arr = rd::<usize>(param_1 + 0x18090 + arch_off);
    let found = exe::call_va!(0x1_4248_9ee0, (rcx_arr, edx_cnt, r8d_comp) -> i32);
    if found != -1 {
        let worker = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let queue_ptr = param_1.wrapping_add(0x580f0).wrapping_add(worker * 0x188);
        exe::call_va!(0x1_40fb_0db0, (queue_ptr, handle));
    }
}

/// `unknown_141e8f2d0`
/// Original: 0x141e8f2d0 (2107 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e8f2d0(param_1: usize, param_2: usize) {
    let p0 = rd::<usize>(param_1);
    let p1 = rd::<usize>(param_1 + 0x08);
    let p2 = rd::<usize>(param_1 + 0x10);
    let p3 = rd::<usize>(param_1 + 0x18);
    let p4 = rd::<usize>(param_1 + 0x20);
    let p5 = rd::<usize>(param_1 + 0x28);
    let p6 = rd::<usize>(param_1 + 0x30);
    let p7 = rd::<usize>(param_1 + 0x38);
    let p8 = rd::<usize>(param_1 + 0x40);
    let p10 = rd::<usize>(param_1 + 0x50);
    let r14 = rd::<usize>(param_1 + 0x68);

    let off_20 = r14.wrapping_shl(5);
    let rsi = p1.wrapping_add(r14.wrapping_shl(7));
    let r13 = p3.wrapping_add(r14 * 0x18);
    let mut local_60 = p4.wrapping_add(off_20);
    let r15 = p5.wrapping_add(r14 * 0x28);
    let rsp_238 = p6.wrapping_add(r14.wrapping_shl(4));

    wr::<u8>(rsi + 0x01, 1);
    if rd::<u8>(rsi) == 0 {
        return;
    }

    if rd::<u8>(rsi + 0x03) != rd::<u8>(rsi + 0x02) {
        let mut tmp_2: u32 = 2;
        exe::call_va!(
            0x1_40ff_f3d0,
            (
                r13 + 0x10,
                (&mut tmp_2 as *mut u32) as usize,
                rsi + 0x02
            )
        );
        wr::<u8>(rsi + 0x03, rd::<u8>(rsi + 0x02));
    }

    let r8_arg = rd::<usize>(p0.wrapping_add(off_20));
    let idx_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
    let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
    let float_ptr = rd::<usize>(tbl_580e0 + idx_0d8 * 8);
    let xmm6_bits = rd::<u32>(float_ptr);

    let obj_a = rd::<usize>(rsp_238);
    let obj_b = rd::<usize>(obj_a + 0x30);
    let r12_sub = rd::<usize>(obj_b + 0x18);
    let r9_arg = if r12_sub == 0 {
        0usize
    } else {
        let target_id = rd::<u32>(exe::ptr::<u32>(0x1_4388_1700) as usize);
        let vtbl = rd::<usize>(r12_sub);
        let got_id = exe::call_va!(rd::<u64>(vtbl + 0x98), (r12_sub) -> u32);
        if got_id == target_id {
            r12_sub
        } else {
            0
        }
    };

    let mut buf_a0 = rd::<[u8; 32]>(p8.wrapping_add(off_20));
    let mut buf_c0 = rd::<[u8; 32]>(p7.wrapping_add(off_20));
    exe::call_va!(
        0x1_41e8_fd10,
        (
            buf_c0.as_mut_ptr() as usize,
            buf_a0.as_mut_ptr() as usize,
            r8_arg,
            r9_arg,
            rsi,
            xmm6_bits
        )
    );

    let cur_flags = rd::<u32>(p2 + r14 * 4);
    let new_flags = if rd::<u8>(rsi + 0x0c) != 0 {
        cur_flags & !1u32
    } else {
        cur_flags | 1u32
    };
    wr::<u32>(p2 + r14 * 4, new_flags);

    let mut buf_e0 = rd::<[u8; 32]>(rsi + 0x10);
    exe::call_va!(
        0x1_41eb_29c0,
        (r13, local_60, buf_e0.as_mut_ptr() as usize)
    );

    if rd::<u8>(rsi + 0x0e) != 0 {
        let r8b = rd::<u8>(rsi + 0x0d);
        let rdx_val = rd::<u64>(p10 + r14 * 8);
        exe::call_va!(0x1_41e8_fb30, (r13, rdx_val, r8b));
    }

    let ebx_38 = rd::<u32>(rsi + 0x38) as usize;
    let r14_obj = rd::<usize>(r13 + 0x08);

    if ebx_38 != 0 {
        let b272 = rd::<u8>(r14_obj + 0x272);
        if b272 != 0 {
            let sub1 = (b272 as i32) - 1;
            let idx0 = if sub1 < 0 { sub1 as isize } else { 0 };
            let cnt_1c8 = rd::<u32>((r14_obj as isize + idx0 * 4 + 0x1c8) as usize);
            if cnt_1c8 != 0 {
                let toggle = rd::<u8>(r15).wrapping_sub(1) & 1;
                wr::<u8>(r15, toggle);
                let r12_slot = r15.wrapping_add((toggle as usize) * 0x10);
                let r15_vec = rd::<usize>(r12_slot + 0x08);

                let b272_2 = rd::<u8>(r14_obj + 0x272);
                let req_cnt: i32 = if b272_2 != 0 {
                    let s = (b272_2 as i32) - 1;
                    let idx = if s < 0 { s as isize } else { 0 };
                    rd::<i32>((r14_obj as isize + idx * 4 + 0x1c8) as usize)
                } else {
                    0
                };
                exe::call_va!(0x1_40a0_2480, (r15_vec, req_cnt as i64 as u64));

                let off_1e8 = (rd::<u32>(r14_obj + 0x1e8) as usize) * 0x30;
                let r14_mats = rd::<usize>(r14_obj + 0x1b0).wrapping_add(off_1e8);

                if rd::<u64>(rsi + 0x60) == 0 {
                    for i in 0..ebx_38 {
                        let off = i * 0x30;
                        let src_a = rd::<usize>(rsi + 0x30).wrapping_add(off);
                        let dst = rd::<usize>(r15_vec).wrapping_add(off);
                        mul_mat3x4_48(src_a, r14_mats.wrapping_add(off), dst);
                    }
                } else {
                    for i in 0..ebx_38 {
                        let off = i * 0x30;
                        let mask_ptr = rd::<usize>(rsi + 0x50);
                        let word = rd::<u64>(mask_ptr + (i >> 6) * 8);
                        let dst = rd::<usize>(r15_vec).wrapping_add(off);
                        if ((word >> (i & 0x3f)) & 1) == 0 {
                            wr::<[u8; 0x30]>(dst, [0u8; 0x30]);
                        } else {
                            let src_a = rd::<usize>(rsi + 0x30).wrapping_add(off);
                            mul_mat3x4_48(src_a, r14_mats.wrapping_add(off), dst);
                        }
                    }
                }

                let ebx_48 = rd::<u32>(rsi + 0x48) as usize;
                if ebx_48 != 0 {
                    let rax_sh = exe::call_va!(
                        0x1_4100_04c0,
                        ((&mut local_60 as *mut usize) as usize) -> usize
                    );
                    let rcx_ctrl = rd::<usize>(rax_sh + 0x08);
                    if rcx_ctrl != 0 {
                        crate::simd_ops::atomic_add_u32((rcx_ctrl + 0x08) as u64, 1);
                    }
                    let rdi_vec = rd::<usize>(rax_sh);
                    let mut shared_pair: [usize; 2] = [rdi_vec, rd::<usize>(rax_sh + 0x08)];
                    exe::call_va!(0x1_4020_0d90, (rax_sh));
                    exe::call_va!(0x1_40a0_2480, (rdi_vec, ebx_48 as u64));

                    for i in 0..ebx_48 {
                        let off = i * 0x30;
                        let src_a = rd::<usize>(rsi + 0x40).wrapping_add(off);
                        let dst = rd::<usize>(rdi_vec).wrapping_add(off);
                        mul_mat3x4_48(src_a, r14_mats.wrapping_add(off), dst);
                    }

                    exe::call_va!(
                        0x1_40ff_f6c0,
                        (r13 + 0x10, shared_pair.as_mut_ptr() as usize)
                    );
                    let ctrl = shared_pair[1];
                    if ctrl != 0 {
                        let old_use =
                            crate::simd_ops::atomic_xadd_u32((ctrl + 0x08) as u64, u32::MAX);
                        if old_use == 1 {
                            let vtbl = rd::<usize>(ctrl);
                            exe::call_va!(rd::<u64>(vtbl), (ctrl));
                            let old_weak =
                                crate::simd_ops::atomic_xadd_u32((ctrl + 0x0c) as u64, u32::MAX);
                            if old_weak == 1 {
                                let ctrl2 = shared_pair[1];
                                let vtbl2 = rd::<usize>(ctrl2);
                                exe::call_va!(rd::<u64>(vtbl2 + 0x08), (ctrl2));
                            }
                        }
                    }
                }

                exe::call_va!(0x1_40ff_f850, (r13 + 0x10, r12_slot + 0x08));
                let mut zero_u32: u32 = 0;
                exe::call_va!(
                    0x1_40ff_f9e0,
                    (r13 + 0x10, (&mut zero_u32 as *mut u32) as usize)
                );
                return;
            }
        }
    }

    let v0 = rd::<usize>(r15 + 0x08);
    wr::<u32>(v0 + 0x08, 0);
    let v1 = rd::<usize>(r15 + 0x18);
    wr::<u32>(v1 + 0x08, 0);
    exe::call_va!(0x1_40ff_f550, (r13 + 0x10));
}

/// `unknown_141e8fb30`
/// Original: 0x141e8fb30 (314 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141e8fb30(
    param_1: usize,
    param_2: u64,
    param_3: u8,
) {
    let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
    let rsi = exe::call_va!(0x1_403b_4090, (g_2b8, param_2) -> usize);
    let rdx_30 = rd::<usize>(rsi + 0x30);
    let rdi = rd::<usize>(rdx_30 + 0x18);
    let r9 = if rdi == 0 {
        0usize
    } else {
        let target_id = rd::<u32>(exe::ptr::<u32>(0x1_4388_1bf8) as usize);
        let vtbl = rd::<usize>(rdi);
        let got_id = exe::call_va!(rd::<u64>(vtbl + 0x98), (rdi) -> u32);
        if got_id == target_id {
            rdi
        } else {
            0
        }
    };

    let rdx_handle = rd::<u64>(r9 + (param_3 as usize) * 8 + 0x248);
    let g_2b8_2 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
    let rax_res = exe::call_va!(0x1_403b_4090, (g_2b8_2, rdx_handle) -> usize);
    let rbx = if rax_res == 0 { rsi } else { rax_res };

    if rbx != rd::<usize>(param_1) {
        let old_ref = crate::simd_ops::atomic_xadd_u32((rbx + 0x24) as u64, 1);
        if old_ref.wrapping_add(1) == 1 {
            let rcx_cb = rd::<usize>(rbx + 0x28);
            if rcx_cb != 0 {
                exe::call_va!(0x1_4241_d6f0, (rcx_cb, rbx));
            }
        }
        let old_ptr = rd::<usize>(param_1);
        wr::<usize>(param_1, rbx);
        if old_ptr != 0 {
            let prev_ref = crate::simd_ops::atomic_xadd_u32((old_ptr + 0x24) as u64, u32::MAX);
            if prev_ref == 1 {
                let rcx_cb = rd::<usize>(old_ptr + 0x28);
                if rcx_cb != 0 {
                    exe::call_va!(0x1_4241_d6f0, (rcx_cb, old_ptr));
                }
            }
        }
    }

    wr::<u64>(param_1 + 0x08, rd::<u64>(rbx + 0xc0));
    let mut local_40: usize = rbx;
    let prev = crate::simd_ops::atomic_xadd_u32((rbx + 0x24) as u64, 1);
    if prev == 0 {
        let rcx_cb = rd::<usize>(rbx + 0x28);
        if rcx_cb != 0 {
            exe::call_va!(0x1_4241_d6f0, (rcx_cb, rbx));
        }
    }

    exe::call_va!(
        0x1_40ff_f250,
        (param_1 + 0x10, (&mut local_40 as *mut usize) as usize)
    );
    let rem = local_40;
    if rem != 0 {
        let prev_rem = crate::simd_ops::atomic_xadd_u32((rem + 0x24) as u64, u32::MAX);
        if prev_rem == 1 {
            let rcx_cb = rd::<usize>(rem + 0x28);
            if rcx_cb != 0 {
                exe::call_va!(0x1_4241_d6f0, (rcx_cb, rem));
            }
        }
    }
}

/// `unknown_141e8fd10`
/// Original: 0x141e8fd10 (1448 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e8fd10(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6_bits: u32,
) {
    use crate::simd_ops::{vaddps, vbroadcastss, vdpps, vmulps, vpermilps, vsubps};

    let al = exe::call_va!(0x1_4216_69e0, (param_3) -> u8);
    let mut use_slerp = al != 0;
    let mut r10_arr: usize = 0;
    if !use_slerp {
        let rax_48 = rd::<usize>(param_3 + 0x48);
        r10_arr = rd::<usize>(rax_48 + 0x40);
        let cnt_48 = rd::<u32>(rax_48 + 0x48) as usize;
        if r10_arr == r10_arr.wrapping_add(cnt_48 * 0xb0) || param_4 == 0 {
            use_slerp = true;
        }
    }

    if use_slerp {
        let xmm10_lo = rd::<u128>(param_2);
        let xmm10_hi = rd::<u128>(param_2 + 0x10);
        let xmm11_lo = rd::<u128>(param_1);
        let xmm11_hi = rd::<u128>(param_1 + 0x10);

        let mut xmm3 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8840) as usize);
        let mut xmm6 = rd::<u128>(exe::ptr::<u128>(0x1_4342_81b0) as usize);
        let xmm1 = vdpps(xmm10_lo, xmm11_lo, 0xff);
        let xmm8 = xmm1 & rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
        let xmm5 = xmm8 ^ xmm1;
        xmm3 = vfmadd231ps_128(
            xmm3,
            xmm5,
            rd::<u128>(exe::ptr::<u128>(0x1_4342_80c0) as usize),
        );
        xmm6 = vfmadd231ps_128(xmm6, xmm5, xmm3);
        xmm3 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8280) as usize);
        let xmm12 = vbroadcastss(param_6_bits);
        let xmm4 = vsubps(xmm12, rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize));
        let xmm2 = vsubps(xmm12, rd::<u128>(exe::ptr::<u128>(0x1_4342_8240) as usize));
        let xmm1_t = vmulps(xmm12, xmm4);
        let xmm7 = vmulps(xmm2, xmm1_t);
        let mut xmm1_p = rd::<u128>(exe::ptr::<u128>(0x1_4342_84c0) as usize);
        xmm1_p = vfnmadd231ps_128(
            xmm1_p,
            xmm5,
            rd::<u128>(exe::ptr::<u128>(0x1_4342_82c0) as usize),
        );
        let mut xmm2_p = rd::<u128>(exe::ptr::<u128>(0x1_4342_88d0) as usize);
        xmm2_p = vfmadd231ps_128(xmm2_p, xmm5, xmm1_p);
        let xmm1_460 = rd::<u128>(exe::ptr::<u128>(0x1_4342_8460) as usize);
        xmm3 = vfmadd231ps_128(xmm3, xmm5, xmm2_p);
        let mut xmm0 = xmm12;
        let xmm4_sq = vmulps(xmm4, xmm4);
        xmm6 = vfmadd231ps_128(xmm6, xmm3, xmm4_sq);
        xmm0 = vfmadd231ps_128(xmm0, xmm6, xmm7);

        wr::<u8>(param_5 + 0x0c, 1);
        wr::<u32>(param_5 + 0x48, 0);
        wr::<u32>(param_5 + 0x38, 0);

        let mut xmm5_q = xmm10_lo;
        xmm5_q = vfnmadd231ps_128(xmm5_q, xmm0, xmm10_lo);
        let xmm0_signed = xmm0 ^ xmm8;
        xmm5_q = vfmadd231ps_128(xmm5_q, xmm0_signed, xmm11_lo);
        let xmm0_dp = vdpps(xmm5_q, xmm5_q, 0xff);
        let xmm4_rsq = vrsqrtps_128(xmm0_dp);
        let xmm0_r1 = vmulps(xmm4_rsq, xmm0_dp);
        let xmm2_r2 = vmulps(xmm0_r1, xmm4_rsq);
        let xmm0_half = vmulps(
            xmm4_rsq,
            rd::<u128>(exe::ptr::<u128>(0x1_4342_8120) as usize),
        );
        let xmm3_nr = vsubps(xmm1_460, xmm2_r2);
        let xmm2_scale = vmulps(xmm3_nr, xmm0_half);

        let mut xmm3_pos = xmm10_hi;
        xmm3_pos = vfnmadd231ps_128(xmm3_pos, xmm3_pos, xmm12);
        xmm3_pos = vfmadd231ps_128(xmm3_pos, xmm11_hi, xmm12);
        let xmm1_q = vmulps(xmm2_scale, xmm5_q);

        wr::<u128>(param_5 + 0x10, xmm1_q);
        wr::<u128>(param_5 + 0x20, xmm3_pos);
        wr::<u32>(param_5 + 0x58, 0);
        wr::<u64>(param_5 + 0x60, 0);
        return;
    }

    let rax_20 = rd::<usize>(param_3 + 0x20);
    if rd::<u16>(rax_20 + 0x90) == 0 {
        wr::<u8>(param_5 + 0x0c, 0);
        wr::<[u8; 32]>(param_5 + 0x10, rd::<[u8; 32]>(param_1));
        wr::<u32>(param_5 + 0x48, 0);
        wr::<u32>(param_5 + 0x38, 0);
        wr::<u32>(param_5 + 0x58, 0);
        wr::<u64>(param_5 + 0x60, 0);
        return;
    }

    let mut rcx_p = rd::<usize>(rax_20 + 0x80);
    let cnt_92 = rd::<u16>(rax_20 + 0x92) as usize;
    let rdx_end = rcx_p.wrapping_add(cnt_92 * 2);
    let mut r9_off: usize = 0x18;
    let mut r12b: u8 = 0;
    let mut r10_off: usize = 0x10;
    while rcx_p != rdx_end {
        let idx = rd::<u16>(rcx_p) as usize;
        if rd::<u16>(r10_arr + idx * 0xb0 + 0x40) != 0xffff {
            r9_off = 0x28;
            r12b = 1;
            r10_off = 0x20;
            break;
        }
        rcx_p = rcx_p.wrapping_add(2);
    }

    let bl = rd::<u8>(param_5 + 0x0d);
    let r13b = rd::<u8>(param_5 + 0x0c);
    wr::<u8>(param_5 + 0x0d, r12b);
    wr::<u8>(param_5 + 0x0c, 1);
    wr::<u8>(param_5 + 0x0e, u8::from(bl != r12b));

    let r9w = rd::<u16>(param_4 + r9_off);
    let mut di: u16 = 0;
    if r9w != 0 {
        let arr_ptr = rd::<usize>(param_4 + r10_off);
        for k in 0..(r9w as usize) {
            let cx = rd::<u16>(arr_ptr + k * 2).wrapping_add(1);
            if di < cx {
                di = cx;
            }
        }
    }

    let mut buf_80 = [0u128; 2];
    let rax_src = if di == 0 {
        let (ret_ptr, _) = crate::simd_ops::call_va8_xmm(
            0x1_4216_88a0,
            param_3 as u64,
            buf_80.as_mut_ptr() as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, param_6_bits as u128, 0],
        );
        ret_ptr as usize
    } else {
        let p20 = rd::<usize>(param_3 + 0x20);
        buf_80[0] = rd::<u128>(p20);
        buf_80[1] = rd::<u128>(p20 + 0x10);
        buf_80.as_mut_ptr() as usize
    };

    let ymm9_lo = rd::<u128>(rax_src);
    let ymm9_hi = rd::<u128>(rax_src + 0x10);
    let eax_38 = rd::<u32>(param_5 + 0x38);

    let xmm8 = ymm9_lo ^ rd::<u128>(exe::ptr::<u128>(0x1_4342_7b00) as usize);
    let xmm4 = vpermilps(xmm8, 0xd2);
    let xmm7 = ymm9_hi ^ rd::<u128>(exe::ptr::<u128>(0x1_4342_8760) as usize);
    let xmm0_a = vpermilps(xmm7, 0xc9);
    let xmm3_a = vmulps(xmm0_a, xmm4);
    let xmm1_a = vpermilps(xmm7, 0xd2);
    let xmm6 = vpermilps(xmm8, 0xc9);
    let xmm2_a = vmulps(xmm1_a, xmm6);
    let xmm3_b = vsubps(xmm2_a, xmm3_a);
    let xmm5 = vmulps(xmm3_b, rd::<u128>(exe::ptr::<u128>(0x1_4342_7ab0) as usize));
    let xmm0_b = vpermilps(xmm5, 0xc9);
    let xmm3_c = vmulps(xmm0_b, xmm4);
    let xmm1_b = vpermilps(xmm5, 0xd2);
    let xmm2_b = vmulps(xmm1_b, xmm6);
    let xmm4_b = vsubps(xmm2_b, xmm3_c);
    let xmm0_c = vpermilps(xmm8, 0xff);
    let xmm1_c = vmulps(xmm0_c, xmm5);
    let xmm2_c = vaddps(xmm1_c, xmm7);
    let xmm3_d = vaddps(xmm4_b, xmm2_c);

    buf_80[0] = xmm8;
    buf_80[1] = xmm3_d;
    wr::<u128>(param_5 + 0x10, ymm9_lo);
    wr::<u128>(param_5 + 0x20, ymm9_hi);

    if di == 0 {
        if eax_38 == 0 {
            return;
        }
        if r13b != 0 && bl == r12b {
            exe::call_va!(0x1_40e9_7680, (param_5 + 0x40, param_5 + 0x30));
        } else {
            exe::call_va!(0x1_40e9_8030, (param_5 + 0x40, di as u32));
        }
    } else if eax_38 == 0 {
        wr::<u32>(param_5 + 0x48, 0);
    } else if r13b != 0 && bl == r12b {
        exe::call_va!(0x1_40e9_7680, (param_5 + 0x40, param_5 + 0x30));
    } else {
        exe::call_va!(0x1_40e9_8030, (param_5 + 0x40, di as u32));
    }

    exe::call_va!(0x1_403d_79a0, (param_5 + 0x68, param_5 + 0x50));
    wr::<u64>(param_5 + 0x78, rd::<u64>(param_5 + 0x60));

    if r13b != 0 && eax_38 != 0 {
        exe::call_va!(
            0x1_41e9_07e0,
            (
                param_5,
                param_4,
                rd::<usize>(param_3 + 0x20),
                param_3,
                bl,
                r12b,
                di
            )
        );
    }

    exe::call_va!(0x1_40e9_8030, (param_5 + 0x30, di as u32));
    wr::<u32>(param_5 + 0x58, 0);
    wr::<u64>(param_5 + 0x60, 0);
    exe::call_va!(0x1_40ff_f1e0, (param_5 + 0x50, di as u32));

    let mut rdx_ptr = rd::<usize>(param_3 + 0xc0);
    let mut r8d: u32 = 0;
    let rcx_0 = rd::<usize>(rdx_ptr);
    let rax_0 = if rd::<u16>(rcx_0 + 0x08) == 7 {
        rcx_0
    } else {
        0
    };
    if rax_0 == 0 {
        let p20 = rd::<usize>(param_3 + 0x20);
        let mut r11_cnt = rd::<u16>(p20 + 0x90) as usize;
        if r11_cnt != 0 {
            let mut r10_ptr = rd::<usize>(param_3 + 0xb8);
            let mut r9d: u32 = 0;
            let mut xmm1_best: f32 = 0.0;
            while r11_cnt != 0 {
                let entry = rd::<usize>(rdx_ptr);
                let xmm0_val = rd::<f32>(r10_ptr);
                let matched = if rd::<u16>(entry + 0x08) == 7 {
                    entry
                } else {
                    0
                };
                if matched != 0 || xmm0_val > xmm1_best {
                    r8d = r9d;
                    xmm1_best = xmm0_val;
                }
                r9d = r9d.wrapping_add(1);
                r10_ptr = r10_ptr.wrapping_add(0x14);
                rdx_ptr = rdx_ptr.wrapping_add(0x08);
                r11_cnt -= 1;
            }
        }
    }

    let mut buf_60 = [0u128; 2];
    let (rax_700, _) = crate::simd_ops::call_va8_xmm(
        0x1_4216_6700,
        param_3 as u64,
        buf_60.as_mut_ptr() as u64,
        r8d as u64,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 0, param_6_bits as u128],
    );
    buf_60[0] = rd::<u128>((rax_700 as usize) + 0x10);

    exe::call_va!(
        0x1_41e9_02c0,
        (
            param_5,
            param_4,
            rd::<usize>(param_3 + 0x20),
            param_3,
            param_6_bits,
            buf_80.as_mut_ptr() as usize,
            buf_60.as_mut_ptr() as usize,
            r12b,
            di as u64
        )
    );

    if r13b == 0 {
        exe::call_va!(0x1_40e9_7680, (param_5 + 0x40, param_5 + 0x30));
    }
}

/// `unknown_141e9a210`
/// Original: 0x141e9a210 (749 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141e9a210(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) {
    let mut rdi_limit: u64 = if rd::<u8>(exe::ptr::<u8>(0x1_43a2_8c0e) as usize) == 0 {
        let f18 = rd::<f32>(param_4 + 0x18);
        let mult = if f18 == 0.0f32 {
            1u64
        } else {
            rd::<u64>(param_4 + 0x08)
        };
        let base_c60 = rd::<i32>(exe::ptr::<i32>(0x1_4385_1c60) as usize) as i64 as u64;
        base_c60.wrapping_mul(mult)
    } else {
        rd::<i32>(exe::ptr::<i32>(0x1_4385_1c80) as usize) as i64 as u64
    };

    if rd::<u8>(param_3) != 0 {
        rdi_limit = u64::MAX;
    }

    let r11_tbl = rd::<usize>(param_1).wrapping_add(0x40);
    let mut query_pair: [u64; 2] = [rd::<u64>(param_1 + 0x08), rd::<u64>(param_1 + 0x10)];
    let mut out_20: [u64; 3] = [0; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            out_20.as_mut_ptr() as usize,
            r11_tbl,
            query_pair.as_mut_ptr() as usize
        )
    );

    let mut iter_buf: [u64; 18] = [0; 18];
    let iter_ptr = iter_buf.as_mut_ptr() as usize;
    wr::<usize>(iter_ptr, r11_tbl);
    wr::<u64>(iter_ptr + 0x08, out_20[0]);
    wr::<u64>(iter_ptr + 0x10, out_20[1]);
    wr::<u64>(iter_ptr + 0x18, out_20[2]);
    wr::<u64>(iter_ptr + 0x28, 0);
    wr::<u64>(iter_ptr + 0x30, 0);

    if out_20[1] != out_20[2] {
        let arch = rd::<u16>(out_20[1] as usize) as usize;
        let chunk = rd::<usize>(r11_tbl + arch * 8 + 0x48);
        wr::<usize>(iter_ptr + 0x20, chunk);
        let cnt = if chunk != 0 {
            rd::<u32>(r11_tbl + arch * 4 + 0x10048) as u64
        } else {
            0
        };
        wr::<u64>(iter_ptr + 0x28, cnt);
    }

    let mut r13_count: u64 = 0;
    exe::call_va!(0x1_4100_8850, (iter_ptr));
    let mut rsi = rd::<u64>(iter_ptr + 0x30);
    let mut r12 = rd::<u64>(iter_ptr + 0x28);

    while rsi < r12 {
        while rsi < r12 {
            if r13_count >= rdi_limit {
                return;
            }
            let r14_slot = rd::<usize>(iter_ptr + 0x88).wrapping_add((rsi as usize) * 0x10);
            let obj_ptr = rd::<usize>(r14_slot);
            if obj_ptr != 0 {
                r13_count = r13_count.wrapping_add(1);
                let p30 = rd::<usize>(obj_ptr + 0x30);
                let r15_sub = rd::<usize>(p30 + 0x18);
                let matched_sub = if r15_sub == 0 {
                    0usize
                } else {
                    let target_id = rd::<u32>(exe::ptr::<u32>(0x1_4388_1700) as usize);
                    let vtbl = rd::<usize>(r15_sub);
                    let got_id = exe::call_va!(rd::<u64>(vtbl + 0x98), (r15_sub) -> u32);
                    if got_id == target_id {
                        r15_sub
                    } else {
                        0
                    }
                };

                let cur_obj = rd::<usize>(r14_slot);
                let mut arg1: [u64; 4] = [
                    obj_ptr as u64,
                    matched_sub as u64,
                    cur_obj.wrapping_add(0xc0) as u64,
                    rd::<u32>(r14_slot + 0x08) as u64,
                ];
                let mut arg2: [u64; 12] = [
                    rd::<u64>(iter_ptr + 0x38),
                    rd::<u64>(iter_ptr + 0x40),
                    rd::<u64>(iter_ptr + 0x48),
                    rd::<u64>(iter_ptr + 0x50),
                    rd::<u64>(iter_ptr + 0x58),
                    rd::<u64>(iter_ptr + 0x60),
                    rd::<u64>(iter_ptr + 0x68),
                    rd::<u64>(iter_ptr + 0x70),
                    rd::<u64>(iter_ptr + 0x78),
                    rd::<u64>(iter_ptr + 0x80),
                    rd::<u64>(iter_ptr + 0x20),
                    rsi,
                ];
                exe::call_va!(
                    0x1_41e9_a600,
                    (
                        arg1.as_mut_ptr() as usize,
                        arg2.as_mut_ptr() as usize,
                        param_2
                    )
                );
            }
            rsi = rsi.wrapping_add(1);
            wr::<u64>(iter_ptr + 0x30, rsi);
        }

        let mut cur_u16 = rd::<usize>(iter_ptr + 0x10).wrapping_add(2);
        let end_u16 = rd::<usize>(iter_ptr + 0x18);
        wr::<usize>(iter_ptr + 0x10, cur_u16);
        if cur_u16 != end_u16 {
            let rdx_tbl = rd::<usize>(iter_ptr + 0x08);
            while cur_u16 != end_u16 {
                let arch = rd::<u16>(cur_u16) as usize;
                if rd::<u64>(rdx_tbl + arch * 8 + 0x48) != 0
                    && rd::<u32>(rdx_tbl + arch * 4 + 0x10048) != 0
                {
                    break;
                }
                cur_u16 = cur_u16.wrapping_add(2);
                wr::<usize>(iter_ptr + 0x10, cur_u16);
            }
            if cur_u16 != end_u16 {
                let arch = rd::<u16>(cur_u16) as usize;
                let rdx_0 = rd::<usize>(iter_ptr);
                let chunk = rd::<usize>(rdx_0 + arch * 8 + 0x48);
                wr::<usize>(iter_ptr + 0x20, chunk);
                let cnt = if chunk != 0 {
                    rd::<u32>(rdx_0 + arch * 4 + 0x10048) as u64
                } else {
                    0
                };
                wr::<u64>(iter_ptr + 0x28, cnt);
                wr::<u64>(iter_ptr + 0x30, 0);
            }
        }

        exe::call_va!(0x1_4100_8850, (iter_ptr));
        rsi = rd::<u64>(iter_ptr + 0x30);
        r12 = rd::<u64>(iter_ptr + 0x28);
    }

    wr::<u8>(param_3, 0);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141e34de0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_9ee0(_keys: usize, _cnt: u32, comp: u32) -> i32 {
        if comp == 10 {
            0
        } else {
            1
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;

    let g_d48 = exe::ptr::<u16>(0x1_43cc_8d48) as usize;
    let g_d4c = exe::ptr::<u16>(0x1_43cc_8d4c) as usize;
    let saved_d48 = rd::<u16>(g_d48);
    let saved_d4c = rd::<u16>(g_d4c);
    wr::<u16>(g_d48, 10);
    wr::<u16>(g_d4c, 20);

    let run = |f: F, preset_depth: i32| -> (i32, i32) {
        let mut world = vec![0u8; 0x18100];
        let wp = world.as_mut_ptr() as usize;

        let mut ent_table: [u64; 4] = [0, (2u64 << 32) | 1, 0, 0];
        let mut loc_table: [u64; 4] = [0, (0u64 << 32) | 1, 0, 0];
        let mut sorted_archs: [u16; 3] = [1, 3, 5];
        let mut offs_table: [u32; 2] = [0x00, 0x10];
        let mut chunk = vec![0u8; 0x40];
        let cp = chunk.as_mut_ptr() as usize;
        wr::<i32>(cp, -1); // child depth at offset 0x00, row 0
        // child list at offset 0x10, row 0 has len = 0

        wr::<usize>(wp + 0x18, ent_table.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x20, 4);
        wr::<usize>(wp + 0x30, loc_table.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x88 + 8, cp);
        wr::<usize>(wp + 0x18098 + 0x20, 0x1234);
        wr::<usize>(wp + 0x180a0 + 0x20, offs_table.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x180ac + 0x20, 2);

        let mut items = vec![0u8; 0x40];
        let ip = items.as_mut_ptr() as usize;
        wr::<u64>(ip + 0x18, (2u64 << 32) | 1); // valid handle at index 1
        wr::<u64>(ip + 0x38, u64::MAX); // invalid handle

        let mut p1: [usize; 3] = [wp, sorted_archs.len(), sorted_archs.as_mut_ptr() as usize];
        let mut p2: i32 = preset_depth;
        let mut p3: [u64; 2] = [ip as u64, 2];

        f(
            p1.as_mut_ptr() as usize,
            (&mut p2 as *mut i32) as usize,
            p3.as_mut_ptr() as usize,
        );
        (p2, rd::<i32>(cp))
    };

    for preset in [-1i32, 5] {
        let o = run(f_o, preset);
        let n = run(f_n, preset);
        if o != n {
            wr::<u16>(g_d48, saved_d48);
            wr::<u16>(g_d4c, saved_d4c);
        }
        crate::diff_eq!(o, n, "141e34de0 preset={}", preset);
    }

    wr::<u16>(g_d48, saved_d48);
    wr::<u16>(g_d4c, saved_d4c);
    Ok(2)
}

pub unsafe fn diff_unknown_141e56a50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DUMMY_7DF0: [u64; 8] = [0; 8];
    static mut ALLOC_U16: [u16; 16] = [0; 16];
    static mut ARCH_LIST: [u16; 2] = [0, 1];
    static mut BITMASKS: [u64; 8] = [0; 8];
    static mut VISITED: [u8; 8] = [0; 8];
    static mut FREED_CNT: u32 = 0;

    unsafe extern "win64" fn mock_7df0(_p2: usize) -> usize {
        core::ptr::addr_of_mut!(DUMMY_7DF0) as usize
    }
    unsafe extern "win64" fn mock_c0a0(_bytes: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ALLOC_U16) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREED_CNT += 1;
    }
    unsafe extern "win64" fn mock_7c10(out_ptr: usize, r11_tbl: usize, _q_ptr: usize) -> usize {
        let start = core::ptr::addr_of_mut!(ARCH_LIST) as usize;
        wr::<usize>(out_ptr, r11_tbl);
        wr::<usize>(out_ptr + 0x08, start);
        wr::<usize>(out_ptr + 0x10, start + 2);
        out_ptr
    }
    unsafe extern "win64" fn mock_e190(iter_ptr: usize) {
        wr::<usize>(iter_ptr + 0x38, core::ptr::addr_of_mut!(BITMASKS) as usize);
        wr::<usize>(iter_ptr + 0x40, core::ptr::addr_of_mut!(VISITED) as usize);
    }
    unsafe extern "win64" fn mock_eba0(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4021_7df0, mock_7df0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40fa_e190, mock_e190 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;

    let g_10c = exe::ptr::<u16>(0x1_43cc_b10c) as usize;
    let saved_10c = rd::<u16>(g_10c);
    wr::<u16>(g_10c, 0);

    let run = |f: F, p1: u32, non_empty: bool| -> (u64, Vec<u8>, Vec<u64>, u32, [u8; 8], u32) {
        FREED_CNT = 0;
        ALLOC_U16 = [0; 16];
        BITMASKS = [0; 8];
        BITMASKS[0] = 1; // bit 0 set on element 0
        VISITED = [0; 8];
        DUMMY_7DF0 = [0; 8];
        let dp = core::ptr::addr_of_mut!(DUMMY_7DF0) as usize;
        wr::<usize>(dp + 0x20, 0x1000);
        wr::<u32>(dp + 0x28, u32::from(non_empty));

        let mut p2: [u64; 2] = [0xaaaa_bbbb, 99];
        let mut p3_items = vec![0u8; 0x100];
        let mut p3_flags = vec![0u8; 2];
        wr::<u64>(p3_items.as_mut_ptr() as usize + 0x60, 0xaaaa_bbbb);
        wr::<u64>(p3_items.as_mut_ptr() as usize + 0x80 + 0x60, 0x1111_2222);

        let mut p3 = [0u8; 0x40];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x10, p3_items.as_mut_ptr() as usize);
        wr::<u32>(p3p + 0x18, 2);
        wr::<usize>(p3p + 0x30, p3_flags.as_mut_ptr() as usize);

        let mut out_items = vec![0u64; 4];
        let mut p4: [u64; 2] = [out_items.as_mut_ptr() as u64, 0];

        let mut p5_sub0 = [0u8; 0x30];
        let mut p5_map: [u16; 2] = [0, 0];
        let mut p5_arr08: [u64; 2] = [0x111, 0x222];
        wr::<usize>(p5_sub0.as_mut_ptr() as usize + 0x20, p5_map.as_mut_ptr() as usize);

        let mut chunk: [u64; 4] = [0, 0, 0xdead_beef_cafe, 0x9999];
        let mut p5 = vec![0u8; 0x10100];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<usize>(p5p, p5_sub0.as_mut_ptr() as usize);
        wr::<usize>(p5p + 0x08, p5_arr08.as_mut_ptr() as usize);
        wr::<usize>(p5p + 0x40 + 0x48, chunk.as_mut_ptr() as usize);
        wr::<u32>(p5p + 0x40 + 0x10048, 2);

        f(p1, p2.as_mut_ptr() as usize, p3p, p4.as_mut_ptr() as usize, p5p);
        (
            p2[1],
            p3_flags,
            out_items,
            p4[1] as u32,
            VISITED,
            FREED_CNT,
        )
    };

    for (p1, non_empty) in [(0u32, false), (1, true), (2, true)] {
        let o = run(f_o, p1, non_empty);
        let n = run(f_n, p1, non_empty);
        if o != n {
            wr::<u16>(g_10c, saved_10c);
        }
        crate::diff_eq!(o, n, "141e56a50 p1={} non_empty={}", p1, non_empty);
    }

    wr::<u16>(g_10c, saved_10c);
    Ok(3)
}

pub unsafe fn diff_unknown_141e5c4f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut REF_BYTE: u8 = 0;
    static mut PUSHED_HANDLE: u64 = 0;

    unsafe extern "win64" fn mock_a730(_rsi: usize, out_idx: usize, key_ptr: usize) {
        let k = rd::<u64>(key_ptr);
        wr::<i64>(out_idx, if k == 0xbad { -1 } else { 0 });
    }
    unsafe extern "win64" fn mock_6eb0(_p1_ptr: usize, _handle: u64) -> usize {
        core::ptr::addr_of_mut!(REF_BYTE) as usize
    }
    unsafe extern "win64" fn mock_9ee0(_arr: usize, _cnt: u32, _comp: u32) -> i32 {
        2
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_0db0(_q_ptr: usize, handle: u64) {
        PUSHED_HANDLE = handle;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40fb_6eb0, mock_6eb0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40fb_0db0, mock_0db0 as *const ())?;

    let g_08c = exe::ptr::<u16>(0x1_43c8_d08c) as usize;
    let g_694 = exe::ptr::<u16>(0x1_43c8_d694) as usize;
    let saved_08c = rd::<u16>(g_08c);
    let saved_694 = rd::<u16>(g_694);
    wr::<u16>(g_08c, 0);
    wr::<u16>(g_694, 5);

    let run = |f: F, ref_init: u8, key: u64| -> (u8, u64) {
        REF_BYTE = ref_init;
        PUSHED_HANDLE = 0;

        let mut world = vec![0u8; 0x58300];
        let wp = world.as_mut_ptr() as usize;

        let mut loc_arr: [u64; 2] = [1, 1];
        wr::<usize>(wp + 0x30, loc_arr.as_mut_ptr() as usize);

        let mut entries: [u64; 4] = [0x10, (0x2000u64 << 32) | 1, 0, 0];
        let mut map_rsi = [0u8; 0x70];
        let rp = map_rsi.as_mut_ptr() as usize;
        wr::<u64>(rp + 0x30, 1);
        wr::<u64>(rp + 0x38, 2);
        wr::<u64>(rp + 0x40, 1);
        wr::<usize>(rp + 0x58, entries.as_mut_ptr() as usize);

        let mut tbl_580e0: [usize; 2] = [rp, 0];
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_mut_ptr() as usize);

        f(wp, key);
        (REF_BYTE, PUSHED_HANDLE)
    };

    for (ref_init, key) in [(1u8, 0x10u64), (2, 0x10), (0, 0x10), (1, 0xbad)] {
        let o = run(f_o, ref_init, key);
        let n = run(f_n, ref_init, key);
        if o != n {
            wr::<u16>(g_08c, saved_08c);
            wr::<u16>(g_694, saved_694);
        }
        crate::diff_eq!(o, n, "141e5c4f0 ref={} key={:#x}", ref_init, key);
    }

    wr::<u16>(g_08c, saved_08c);
    wr::<u16>(g_694, saved_694);
    Ok(4)
}

pub unsafe fn diff_unknown_141e8f2d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_MASK: u32 = 0;
    static mut SHARED_PAIR_STATIC: [usize; 2] = [0; 2];

    unsafe extern "win64" fn mock_f3d0(_c: usize, _d: usize, _r8: usize) {
        CALL_MASK |= 1;
    }
    unsafe extern "win64" fn mock_vtbl_98(_this: usize) -> u32 {
        0x1234_5678
    }
    unsafe extern "win64" fn mock_fd10(
        _c0: usize,
        _a0: usize,
        _r8: usize,
        _r9: usize,
        rsi: usize,
        _xmm6: u32,
    ) {
        CALL_MASK |= 2;
        wr::<u8>(rsi + 0x0c, 1);
        wr::<u8>(rsi + 0x0d, 2);
        wr::<u8>(rsi + 0x0e, 1);
    }
    unsafe extern "win64" fn mock_29c0(_r13: usize, _l60: usize, _e0: usize) {
        CALL_MASK |= 4;
    }
    unsafe extern "win64" fn mock_fb30(_r13: usize, _rdx: u64, _r8b: u8) {
        CALL_MASK |= 8;
    }
    unsafe extern "win64" fn mock_2480(_vec: usize, _cnt: u64) {
        CALL_MASK |= 16;
    }
    unsafe extern "win64" fn mock_04c0(_l60_ptr: usize) -> usize {
        core::ptr::addr_of_mut!(SHARED_PAIR_STATIC) as usize
    }
    unsafe extern "win64" fn mock_0d90(_p: usize) {
        CALL_MASK |= 32;
    }
    unsafe extern "win64" fn mock_f6c0(_c: usize, _pair: usize) {
        CALL_MASK |= 64;
    }
    unsafe extern "win64" fn mock_f850(_c: usize, _slot: usize) {
        CALL_MASK |= 128;
    }
    unsafe extern "win64" fn mock_f9e0(_c: usize, _zero: usize) {
        CALL_MASK |= 256;
    }
    unsafe extern "win64" fn mock_f550(_c: usize) {
        CALL_MASK |= 512;
    }

    let _m1 = crate::difftest::Mock::new(0x1_40ff_f3d0, mock_f3d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e8_fd10, mock_fd10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41eb_29c0, mock_29c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_41e8_fb30, mock_fb30 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40a0_2480, mock_2480 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4100_04c0, mock_04c0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4020_0d90, mock_0d90 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40ff_f6c0, mock_f6c0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_40ff_f850, mock_f850 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_40ff_f9e0, mock_f9e0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_40ff_f550, mock_f550 as *const ())?;

    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let g_700 = exe::ptr::<u32>(0x1_4388_1700) as usize;
    let saved_0d8 = rd::<u16>(g_0d8);
    let saved_700 = rd::<u32>(g_700);
    wr::<u16>(g_0d8, 0);
    wr::<u32>(g_700, 0x1234_5678);

    let run = |f: F, has_bones: bool, use_mask: bool| -> (u32, u32, [f32; 12], [f32; 12]) {
        CALL_MASK = 0;

        let mut vtbl = [0usize; 24];
        vtbl[0x98 / 8] = mock_vtbl_98 as *const () as usize;
        let mut r12_obj: [usize; 2] = [vtbl.as_ptr() as usize, 0];
        let mut obj_b = [0u8; 0x30];
        wr::<usize>(obj_b.as_mut_ptr() as usize + 0x18, r12_obj.as_mut_ptr() as usize);
        let mut obj_a = [0u8; 0x40];
        wr::<usize>(obj_a.as_mut_ptr() as usize + 0x30, obj_b.as_mut_ptr() as usize);
        let mut p6_arr: [usize; 2] = [obj_a.as_mut_ptr() as usize, 0];

        let mut src_a: [f32; 12] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        let mut src_b: [f32; 12] = [
            1.0, 0.0, 0.0, 2.0, 0.0, 1.0, 0.0, 3.0, 0.0, 0.0, 1.0, 4.0,
        ];
        let mut dst_38 = [0.0f32; 12];
        let mut dst_48 = [0.0f32; 12];
        let mut vec_38: [usize; 2] = [dst_38.as_mut_ptr() as usize, 1];
        let mut vec_48: [usize; 2] = [dst_48.as_mut_ptr() as usize, 1];
        SHARED_PAIR_STATIC = [vec_48.as_mut_ptr() as usize, 0];

        let mut mask_bits: [u64; 1] = [if use_mask { 0 } else { 1 }];
        let mut rsi = [0u8; 0x80];
        let rsip = rsi.as_mut_ptr() as usize;
        wr::<u8>(rsip, 1);
        wr::<u8>(rsip + 0x02, 5);
        wr::<u8>(rsip + 0x03, 4);
        wr::<usize>(rsip + 0x30, src_a.as_mut_ptr() as usize);
        wr::<u32>(rsip + 0x38, u32::from(has_bones));
        wr::<usize>(rsip + 0x40, src_a.as_mut_ptr() as usize);
        wr::<u32>(rsip + 0x48, u32::from(has_bones));
        wr::<usize>(rsip + 0x50, mask_bits.as_mut_ptr() as usize);
        wr::<u64>(rsip + 0x60, u64::from(use_mask));

        let mut r14_obj = vec![0u8; 0x280];
        let r14p = r14_obj.as_mut_ptr() as usize;
        wr::<usize>(r14p + 0x1b0, src_b.as_mut_ptr() as usize);
        wr::<u32>(r14p + 0x1c8, 1);
        wr::<u8>(r14p + 0x272, 1);

        let mut r13 = [0u8; 0x20];
        wr::<usize>(r13.as_mut_ptr() as usize + 0x08, r14p);

        let mut r15 = [0u8; 0x28];
        let r15p = r15.as_mut_ptr() as usize;
        wr::<u8>(r15p, 1);
        wr::<usize>(r15p + 0x08, vec_38.as_mut_ptr() as usize);
        wr::<usize>(r15p + 0x18, vec_38.as_mut_ptr() as usize);

        let mut p0_arr: [u64; 4] = [0x1111, 0, 0, 0];
        let mut p2_flags: [u32; 2] = [3, 0];
        let mut p4_arr = [0u8; 0x20];
        let mut p7_arr = [0u8; 0x20];
        let mut p8_arr = [0u8; 0x20];
        let mut p10_arr: [u64; 2] = [0x2222, 0];

        let mut param1: [usize; 14] = [
            p0_arr.as_mut_ptr() as usize,
            rsip,
            p2_flags.as_mut_ptr() as usize,
            r13.as_mut_ptr() as usize,
            p4_arr.as_mut_ptr() as usize,
            r15p,
            p6_arr.as_mut_ptr() as usize,
            p7_arr.as_mut_ptr() as usize,
            p8_arr.as_mut_ptr() as usize,
            0,
            p10_arr.as_mut_ptr() as usize,
            0,
            0,
            0,
        ];

        let mut float_val: f32 = 0.5;
        let mut tbl_580e0: [usize; 1] = [(&mut float_val as *mut f32) as usize];
        let mut world = vec![0u8; 0x58100];
        wr::<usize>(world.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_mut_ptr() as usize);

        f(param1.as_mut_ptr() as usize, world.as_mut_ptr() as usize);
        (CALL_MASK, p2_flags[0], dst_38, dst_48)
    };

    for (has_bones, use_mask) in [(false, false), (true, false), (true, true)] {
        let o = run(f_o, has_bones, use_mask);
        let n = run(f_n, has_bones, use_mask);
        if o != n {
            wr::<u16>(g_0d8, saved_0d8);
            wr::<u32>(g_700, saved_700);
        }
        crate::diff_eq!(o, n, "141e8f2d0 bones={} mask={}", has_bones, use_mask);
    }

    wr::<u16>(g_0d8, saved_0d8);
    wr::<u32>(g_700, saved_700);
    Ok(3)
}

pub unsafe fn diff_unknown_141e8fb30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RES_A: [u8; 0x100] = [0; 0x100];
    static mut RES_B: [u8; 0x100] = [0; 0x100];
    static mut CB_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_vtbl_98(_this: usize) -> u32 {
        0x7777
    }
    unsafe extern "win64" fn mock_4090(_mgr: usize, handle: u64) -> usize {
        if handle == 0x111 {
            core::ptr::addr_of_mut!(RES_A) as usize
        } else if handle == 0x222 {
            core::ptr::addr_of_mut!(RES_B) as usize
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_d6f0(_cb: usize, _obj: usize) {
        CB_COUNT += 1;
    }
    unsafe extern "win64" fn mock_f250(_dst: usize, _ptr_slot: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_403b_4090, mock_4090 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40ff_f250, mock_f250 as *const ())?;

    let g_2b8 = exe::ptr::<usize>(0x1_43a0_82b8) as usize;
    let g_bf8 = exe::ptr::<u32>(0x1_4388_1bf8) as usize;
    let saved_2b8 = rd::<usize>(g_2b8);
    let saved_bf8 = rd::<u32>(g_bf8);
    wr::<usize>(g_2b8, 0x9000);
    wr::<u32>(g_bf8, 0x7777);

    let run = |f: F, idx: u8| -> (usize, u64, u32, u32, u32) {
        CB_COUNT = 0;
        RES_A = [0; 0x100];
        RES_B = [0; 0x100];
        let ra = core::ptr::addr_of_mut!(RES_A) as usize;
        let rb = core::ptr::addr_of_mut!(RES_B) as usize;

        let mut vtbl = [0usize; 24];
        vtbl[0x98 / 8] = mock_vtbl_98 as *const () as usize;
        let mut rdi_obj = [0u8; 0x260];
        let rdip = rdi_obj.as_mut_ptr() as usize;
        wr::<usize>(rdip, vtbl.as_ptr() as usize);
        wr::<u64>(rdip + 0x248, 0x222);
        wr::<u64>(rdip + 0x250, 0);

        let mut sub_30 = [0u8; 0x20];
        wr::<usize>(sub_30.as_mut_ptr() as usize + 0x18, rdip);

        wr::<u32>(ra + 0x24, 1);
        wr::<usize>(ra + 0x28, 0x5555);
        wr::<usize>(ra + 0x30, sub_30.as_mut_ptr() as usize);
        wr::<u64>(ra + 0xc0, 0xaaaa);

        wr::<u32>(rb + 0x24, 0);
        wr::<usize>(rb + 0x28, 0x5555);
        wr::<u64>(rb + 0xc0, 0xbbbb);

        let mut p1: [usize; 4] = [ra, 0, 0, 0];
        f(p1.as_mut_ptr() as usize, 0x111, idx);
        (
            p1[0] - ra,
            p1[1] as u64,
            rd::<u32>(ra + 0x24),
            rd::<u32>(rb + 0x24),
            CB_COUNT,
        )
    };

    for idx in [0u8, 1] {
        let o = run(f_o, idx);
        let n = run(f_n, idx);
        if o != n {
            wr::<usize>(g_2b8, saved_2b8);
            wr::<u32>(g_bf8, saved_bf8);
        }
        crate::diff_eq!(o, n, "141e8fb30 idx={}", idx);
    }

    wr::<usize>(g_2b8, saved_2b8);
    wr::<u32>(g_bf8, saved_bf8);
    Ok(2)
}

pub unsafe fn diff_unknown_141e8fd10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_2C0: ([u32; 8], [u32; 4], u8, u16) = ([0; 8], [0; 4], 0, 0);

    unsafe extern "win64" fn mock_69e0(p3: usize) -> u8 {
        rd::<u8>(p3)
    }
    unsafe extern "win64" fn mock_88a0(_p3: usize, buf_80: usize, _x2: usize) -> usize {
        wr::<[f32; 8]>(buf_80, [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0]);
        buf_80
    }
    unsafe extern "win64" fn mock_7680(_dst: usize, _src: usize) {}
    unsafe extern "win64" fn mock_8030(vec: usize, cnt: u32) {
        wr::<u32>(vec + 0x08, cnt);
    }
    unsafe extern "win64" fn mock_79a0(_dst: usize, _src: usize) {}
    unsafe extern "win64" fn mock_07e0(
        _p5: usize,
        _p4: usize,
        _p20: usize,
        _p3: usize,
        _bl: u8,
        _r12b: u8,
        _di: u16,
    ) {
    }
    unsafe extern "win64" fn mock_f1e0(_bits: usize, _cnt: u32) {}
    unsafe extern "win64" fn mock_6700(_p3: usize, buf_60: usize, r8d: u32, _x3: usize) -> usize {
        wr::<[u32; 4]>(buf_60 + 0x10, [r8d + 10, 20, 30, 40]);
        buf_60
    }
    unsafe extern "win64" fn mock_02c0(
        _p5: usize,
        _p4: usize,
        _p20: usize,
        _p3: usize,
        _t: u32,
        b80: usize,
        b60: usize,
        r12b: u8,
        di: u64,
    ) {
        LOG_2C0 = (
            rd::<[u32; 8]>(b80),
            rd::<[u32; 4]>(b60),
            r12b,
            di as u16,
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4216_69e0, mock_69e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4216_88a0, mock_88a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40e9_7680, mock_7680 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40e9_8030, mock_8030 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_403d_79a0, mock_79a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e9_07e0, mock_07e0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4216_6700, mock_6700 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_41e9_02c0, mock_02c0 as *const ())?;

    let run = |f: F, mode: u8| -> (Vec<u32>, ([u32; 8], [u32; 4], u8, u16)) {
        LOG_2C0 = ([0; 8], [0; 4], 0, 0);

        let p1: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 4.0, 8.0, 12.0, 0.0];
        let p2: [f32; 8] = [0.0, 0.0, 0.0, 1.0, 2.0, 4.0, 6.0, 0.0];

        let mut r10_items = vec![0u8; 0xb0];
        wr::<u16>(r10_items.as_mut_ptr() as usize + 0x40, 1);
        let mut sub_48 = [0u8; 0x50];
        wr::<usize>(sub_48.as_mut_ptr() as usize + 0x40, r10_items.as_mut_ptr() as usize);
        wr::<u32>(sub_48.as_mut_ptr() as usize + 0x48, 1);

        let mut idx_80: [u16; 1] = [0];
        let mut sub_20 = vec![0u8; 0xa0];
        let s20p = sub_20.as_mut_ptr() as usize;
        wr::<[f32; 8]>(s20p, [0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0]);
        wr::<usize>(s20p + 0x80, idx_80.as_mut_ptr() as usize);
        wr::<u16>(s20p + 0x90, u16::from(mode >= 1));
        wr::<u16>(s20p + 0x92, 1);

        let mut c0_ent = [0u8; 0x10];
        wr::<u16>(c0_ent.as_mut_ptr() as usize + 0x08, 7);
        let mut c0_arr: [usize; 1] = [c0_ent.as_mut_ptr() as usize];

        let mut p3 = vec![0u8; 0xd0];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u8>(p3p, u8::from(mode == 0));
        wr::<usize>(p3p + 0x20, s20p);
        wr::<usize>(p3p + 0x48, sub_48.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0xc0, c0_arr.as_mut_ptr() as usize);

        let mut p4_indices: [u16; 1] = [2];
        let mut p4 = [0u8; 0x30];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x20, p4_indices.as_mut_ptr() as usize);
        wr::<u16>(p4p + 0x28, u16::from(mode == 2));

        let mut p5 = vec![0u32; 0x80 / 4];
        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3p,
            p4p,
            p5.as_mut_ptr() as usize,
            0.5f32.to_bits(),
        );
        (p5, LOG_2C0)
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141e8fd10 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141e9a210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_LIST_7: [u16; 1] = [0];
    static mut SLOTS_88: [u64; 4] = [0; 4];
    static mut VTBL_7: [usize; 24] = [0; 24];
    static mut R15_SUB_7: [usize; 2] = [0; 2];
    static mut P30_7: [u8; 0x20] = [0; 0x20];
    static mut OBJ_7: [u8; 0x100] = [0; 0x100];
    static mut LOG_A600: Vec<([u64; 4], [u64; 12], u64)> = Vec::new();

    unsafe extern "win64" fn mock_vtbl_98(_this: usize) -> u32 {
        0x1234_5678
    }
    unsafe extern "win64" fn mock_7c10(out_ptr: usize, r11_tbl: usize, _q_ptr: usize) -> usize {
        let start = core::ptr::addr_of_mut!(ARCH_LIST_7) as usize;
        wr::<usize>(out_ptr, r11_tbl);
        wr::<usize>(out_ptr + 0x08, start);
        wr::<usize>(out_ptr + 0x10, start + 2);
        out_ptr
    }
    unsafe extern "win64" fn mock_8850(iter_ptr: usize) {
        for i in 0..10usize {
            wr::<u64>(iter_ptr + 0x38 + i * 8, (0x100 + i) as u64);
        }
        wr::<usize>(iter_ptr + 0x88, core::ptr::addr_of_mut!(SLOTS_88) as usize);
    }
    unsafe extern "win64" fn mock_a600(a1: usize, a2: usize, p2: u64) {
        LOG_A600.push((rd::<[u64; 4]>(a1), rd::<[u64; 12]>(a2), p2));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_7c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4100_8850, mock_8850 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41e9_a600, mock_a600 as *const ())?;

    let g_c0e = exe::ptr::<u8>(0x1_43a2_8c0e) as usize;
    let g_c60 = exe::ptr::<i32>(0x1_4385_1c60) as usize;
    let g_c80 = exe::ptr::<i32>(0x1_4385_1c80) as usize;
    let g_700 = exe::ptr::<u32>(0x1_4388_1700) as usize;
    let saved_c0e = rd::<u8>(g_c0e);
    let saved_c60 = rd::<i32>(g_c60);
    let saved_c80 = rd::<i32>(g_c80);
    let saved_700 = rd::<u32>(g_700);
    wr::<u8>(g_c0e, 0);
    wr::<i32>(g_c60, 1);
    wr::<i32>(g_c80, 10);
    wr::<u32>(g_700, 0x1234_5678);

    let run = |f: F, force_all: u8| -> (u8, Vec<([u64; 4], [u64; 12], u64)>) {
        LOG_A600.clear();

        VTBL_7 = [0usize; 24];
        VTBL_7[0x98 / 8] = mock_vtbl_98 as *const () as usize;
        R15_SUB_7 = [core::ptr::addr_of!(VTBL_7) as usize, 0];
        P30_7 = [0u8; 0x20];
        wr::<usize>(
            core::ptr::addr_of_mut!(P30_7) as usize + 0x18,
            core::ptr::addr_of_mut!(R15_SUB_7) as usize,
        );
        OBJ_7 = [0u8; 0x100];
        let obj_ptr = core::ptr::addr_of_mut!(OBJ_7) as usize;
        wr::<usize>(obj_ptr + 0x30, core::ptr::addr_of_mut!(P30_7) as usize);

        SLOTS_88 = [obj_ptr as u64, 42, obj_ptr as u64, 99];

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x40 + 0x48, 0x5000);
        wr::<u32>(wp + 0x40 + 0x10048, 2);

        let mut p1: [u64; 3] = [wp as u64, 1, 2];
        let mut p3: u8 = force_all;
        let mut p4 = [0u8; 0x20];
        wr::<u64>(p4.as_mut_ptr() as usize + 0x08, 1);
        wr::<f32>(p4.as_mut_ptr() as usize + 0x18, 0.0);

        f(
            p1.as_mut_ptr() as usize,
            0x777,
            (&mut p3 as *mut u8) as usize,
            p4.as_mut_ptr() as usize,
        );
        (p3, LOG_A600.clone())
    };

    for force_all in [0u8, 1] {
        let o = run(f_o, force_all);
        let n = run(f_n, force_all);
        if o != n {
            wr::<u8>(g_c0e, saved_c0e);
            wr::<i32>(g_c60, saved_c60);
            wr::<i32>(g_c80, saved_c80);
            wr::<u32>(g_700, saved_700);
        }
        crate::diff_eq!(o, n, "141e9a210 force_all={}", force_all);
    }

    wr::<u8>(g_c0e, saved_c0e);
    wr::<i32>(g_c60, saved_c60);
    wr::<i32>(g_c80, saved_c80);
    wr::<u32>(g_700, saved_700);
    Ok(2)
}
