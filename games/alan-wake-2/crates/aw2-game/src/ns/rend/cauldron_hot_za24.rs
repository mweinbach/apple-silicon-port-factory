//! Decompiled `rend` hot functions shard ZA24 (`0x1424e0b10..0x14253a310`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn lock_spin_byte(lock_addr: usize) {
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
}

#[inline(always)]
fn fnv1a_u64(val: u64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in val.to_le_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[inline(always)]
unsafe fn find_hash_node(this: usize, key: u64) -> usize {
    let h = fnv1a_u64(key);
    let bucket_idx = (rd::<u64>(this + 0xe0) & h) as usize;
    let buckets = rd::<usize>(this + 0xc8);
    let sentinel = rd::<usize>(this + 0xb8);
    let mut node = rd::<usize>(buckets + bucket_idx * 16 + 8);
    if node == sentinel {
        return 0;
    }
    let bucket_end = rd::<usize>(buckets + bucket_idx * 16);
    loop {
        if rd::<u64>(node + 0x10) == key {
            return node;
        }
        if node == bucket_end {
            return 0;
        }
        node = rd::<usize>(node + 0x08);
    }
}

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

/// `anonymous_namespace_1424e0b10`
/// Original: 0x1424e0b10 (1385 bytes, Throws::Caught)
pub unsafe extern "win64" fn anonymous_namespace_1424e0b10(param_1: usize) {
    lock_spin_byte(param_1 + 0x110);

    let mut local_a8 = [0u8; 16];
    let la8p = local_a8.as_mut_ptr() as usize;
    let count = rd::<u32>(param_1 + 0x120) as usize;
    let total_bytes = (count as u64).wrapping_mul(0x30);
    let mut rbx = rd::<usize>(param_1 + 0x118);
    let r15_end = rbx.wrapping_add(total_bytes as usize);
    let rbp_cnt = count as i64;
    let mut rdi: usize = 0;
    if rbp_cnt > 0 {
        rdi = exe::call_va!(0x1_4024_a9e0, (total_bytes, rbp_cnt as u64) -> usize);
        wr::<usize>(la8p, rdi);
        wr::<u32>(la8p + 0x0c, rbp_cnt as u32);
    }
    let mut dst_copy = rdi;
    while rbx != r15_end {
        wr::<u128>(dst_copy, rd::<u128>(rbx));
        wr::<u128>(dst_copy + 0x10, rd::<u128>(rbx + 0x10));
        wr::<u128>(dst_copy + 0x20, rd::<u128>(rbx + 0x20));
        dst_copy = dst_copy.wrapping_add(0x30);
        rbx = rbx.wrapping_add(0x30);
    }
    wr::<u32>(la8p + 0x08, rbp_cnt as u32);
    wr::<u32>(param_1 + 0x120, 0);
    crate::simd_ops::wu8((param_1 + 0x110) as u64, 0); // spinlock release (x86 mov = release)

    let rbp_end = rdi.wrapping_add((rbp_cnt as u32 as usize).wrapping_mul(0x30));
    if rdi != rbp_end {
        let mut local_30 = [0u8; 0x50];
        let l30p = local_30.as_mut_ptr() as usize;
        let sentinel = rd::<usize>(param_1 + 0xb8);
        let mut cur = rdi;
        while cur != rbp_end {
            let kind = rd::<u32>(cur);
            if kind == 0 {
                let key = rd::<u64>(cur + 0x20);
                let node = find_hash_node(param_1, key);
                if node != 0 && node != sentinel {
                    let val_ptr = rd::<usize>(node + 0x18);
                    wr::<u128>(val_ptr + 0x10, rd::<u128>(cur + 0x10));
                    let val_ptr2 = rd::<usize>(node + 0x18);
                    wr::<u8>(val_ptr2 + 0x61, 1);
                } else {
                    wr::<u8>(l30p + 0x44, 1);
                    wr::<u128>(l30p + 0x10, rd::<u128>(cur + 0x10));
                    wr::<u32>(l30p + 0x40, 0);
                    let ok = exe::call_va!(0x1_424e_1830, (param_1, l30p) -> u8);
                    if ok == 0 {
                        lock_spin_byte(param_1 + 0x110);
                        let len = rd::<u32>(param_1 + 0x120) as u64;
                        let cap = rd::<u32>(param_1 + 0x124) as u64;
                        if len.wrapping_mul(0x30) == cap.wrapping_mul(0x30) {
                            exe::call_va!(0x1_403f_ec40, (param_1 + 0x118, len.wrapping_add(1)));
                        }
                        let idx = rd::<u32>(param_1 + 0x120) as usize;
                        let dst = rd::<usize>(param_1 + 0x118).wrapping_add(idx * 0x30);
                        wr::<u128>(dst, rd::<u128>(cur));
                        wr::<u128>(dst + 0x10, rd::<u128>(cur + 0x10));
                        wr::<u128>(dst + 0x20, rd::<u128>(cur + 0x20));
                        wr::<u32>(param_1 + 0x120, (idx as u32).wrapping_add(1));
                        crate::simd_ops::wu8((param_1 + 0x110) as u64, 0); // spinlock release (x86 mov = release)
                    }
                }
            } else if kind == 1 {
                let key = rd::<u64>(cur + 0x20);
                let node = find_hash_node(param_1, key);
                if node != 0 && node != sentinel {
                    let val_ptr = rd::<usize>(node + 0x18);
                    wr::<u8>(val_ptr + 0x61, 0);
                    let r10 = rd::<usize>(node + 0x18);
                    let r11 = rd::<usize>(param_1 + 0xf0);
                    let f8_len = rd::<u32>(param_1 + 0xf8) as u64;
                    let r9 = rd::<u32>(r10 + 0x58) as u64;
                    if r9 < f8_len {
                        let entry = r11.wrapping_add((r9 as usize) * 0x20);
                        if rd::<u8>(entry + 0x0c) != 0 {
                            let target_id = rd::<u32>(r10 + 0x54);
                            let mut rax = rd::<usize>(entry + 0x10);
                            let ecx = rd::<u32>(entry + 0x18) as usize;
                            let r8_end = rax.wrapping_add(ecx * 4);
                            if rax != r8_end {
                                while rax != r8_end {
                                    if rd::<u32>(rax) == target_id {
                                        break;
                                    }
                                    rax = rax.wrapping_add(4);
                                }
                                if rax != r8_end {
                                    let mut rcx = rax.wrapping_add(4);
                                    while rcx != r8_end {
                                        let d = rd::<u32>(rcx);
                                        if d != target_id {
                                            wr::<u32>(rax, d);
                                            rax = rax.wrapping_add(4);
                                        }
                                        rcx = rcx.wrapping_add(4);
                                    }
                                }
                            }
                            let rdx_begin = rd::<usize>(entry + 0x10);
                            let ecx_len = rd::<u32>(entry + 0x18) as usize;
                            let rcx_end = rdx_begin.wrapping_add(ecx_len * 4);
                            if rax != rcx_end {
                                if rax == rdx_begin {
                                    wr::<u32>(entry + 0x18, 0);
                                } else {
                                    let removed = ((rcx_end.wrapping_sub(rax)) >> 2) as u32;
                                    wr::<u32>(
                                        entry + 0x18,
                                        rd::<u32>(entry + 0x18).wrapping_sub(removed),
                                    );
                                }
                            }
                        }
                    }
                    let val_ptr3 = rd::<usize>(node + 0x18);
                    wr::<u32>(val_ptr3 + 0x58, 0);
                }
            }
            cur = cur.wrapping_add(0x30);
        }
    }

    let mut rbx_100 = rd::<usize>(param_1 + 0x100);
    let end_100 = rbx_100.wrapping_add((rd::<u32>(param_1 + 0x108) as usize).wrapping_mul(0x50));
    while rbx_100 != end_100 {
        exe::call_va!(0x1_424e_1830, (param_1, rbx_100));
        rbx_100 = rbx_100.wrapping_add(0x50);
    }

    exe::call_va!(0x1_401d_e830, (la8p));
}

/// `anonymous_namespace_1424e10a0`
/// Original: 0x1424e10a0 (817 bytes, Throws::Caught)
pub unsafe extern "win64" fn anonymous_namespace_1424e10a0(
    param_1: usize,
    param_2: usize,
    _param_3_xmm2: usize,
) {
    let [_, _, xmm2, _] = crate::simd_ops::take_xmm_args();
    let time_f64 = f64::from_bits(xmm2 as u64);

    let mut rdi_ptr: usize = 0;
    let mut esi_len: u32 = 0;
    let mut r14_cap: u32 = 0;
    let init_cap = rd::<u32>(param_1 + 0xf8);
    if init_cap != 0 {
        r14_cap = init_cap;
        rdi_ptr = exe::call_va!(0x1_4241_c0a0, ((init_cap as u64) * 8, 8u32) -> usize);
    }

    let mut rbx = rd::<usize>(param_1 + 0xf0);
    let r9_end = rbx.wrapping_add((rd::<u32>(param_1 + 0xf8) as usize).wrapping_mul(0x20));
    if rbx != r9_end {
        let f_one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        while rbx != r9_end {
            if rd::<u8>(rbx + 0x0d) != 0 {
                let count_18 = rd::<u32>(rbx + 0x18) as u64;
                let rcx_arr = rd::<usize>(rbx + 0x10);
                let rdx_end = rcx_arr.wrapping_add((count_18 as usize) * 4);
                if count_18 == 2 {
                    let count_a8 = rd::<u32>(param_1 + 0xa8);
                    let base_a0 = rd::<usize>(param_1 + 0xa0);
                    if rd::<u32>(rcx_arr) < count_a8 && rd::<u32>(rcx_arr + 4) < count_a8 {
                        let mut mult = f_one;
                        let mut cur = rcx_arr;
                        while cur != rdx_end {
                            let idx = rd::<u32>(cur) as usize;
                            if rd::<u8>(base_a0.wrapping_add(idx * 0x70 + 0x63)) != 0 {
                                mult = rd::<f32>(exe::ptr::<f32>(0x1_4387_2970) as usize);
                                break;
                            }
                            cur = cur.wrapping_add(4);
                        }
                        let dt = (time_f64 - rd::<f64>(rbx)) as f32;
                        wr::<f32>(rbx + 0x08, dt * mult);

                        if esi_len == r14_cap {
                            let old_len = esi_len as usize;
                            let old_cap = r14_cap as usize;
                            let new_cap = (old_cap + (old_cap >> 1)).max(old_len + 1);
                            let new_ptr = exe::call_va!(
                                0x1_4241_c0a0,
                                ((new_cap * 8) as u64, 8u32) -> usize
                            );
                            exe::call_va!(
                                0x1_4183_acc8,
                                (new_ptr, rdi_ptr, (old_len * 8) as u64)
                            );
                            if rdi_ptr != 0 {
                                exe::call_va!(0x1_4241_c190, (rdi_ptr));
                            }
                            rdi_ptr = new_ptr;
                            r14_cap = new_cap as u32;
                        }
                        wr::<usize>(rdi_ptr.wrapping_add((esi_len as usize) * 8), rbx);
                        esi_len = esi_len.wrapping_add(1);
                    }
                }
            }
            rbx = rbx.wrapping_add(0x20);
        }
    }

    let end_ptr = rdi_ptr.wrapping_add((esi_len as usize) * 8);
    let count = (esi_len as usize).min(3);
    let mid_ptr = rdi_ptr.wrapping_add(count * 8);
    let mut first_arg = rdi_ptr;
    let mut mid_arg = mid_ptr;
    let mut end_arg = end_ptr;
    exe::call_va!(
        0x1_413f_c170,
        (
            (&mut first_arg as *mut usize) as usize,
            (&mut mid_arg as *mut usize) as usize,
            (&mut end_arg as *mut usize) as usize
        )
    );

    if count != 0 {
        let vtbl_analyzer = exe::ptr::<u8>(0x1_4341_61b8) as usize;
        let vtbl_base = exe::ptr::<u8>(0x1_4319_5628) as usize;
        let mut ignore_obj: usize;
        let mut end_vec = [0u8; 16];
        let mut start_vec = [0u8; 16];
        let mut ray_params = [0u8; 0x18];
        let mut hit_out: [u8; 0x58];

        for i in 0..count {
            let item = rd::<usize>(rdi_ptr.wrapping_add(i * 8));
            ignore_obj = vtbl_analyzer;
            hit_out = [0u8; 0x58];

            let idx_pair = rd::<usize>(item + 0x10);
            let idx1 = rd::<u32>(idx_pair + 4) as usize;
            let base_a0 = rd::<usize>(param_1 + 0xa0);
            let src1 = base_a0.wrapping_add(idx1 * 0x70 + 0x10);
            let evp = end_vec.as_mut_ptr() as usize;
            wr::<u64>(evp, rd::<u64>(src1));
            wr::<u32>(evp + 0x08, rd::<u32>(src1 + 0x08));

            let idx0 = rd::<u32>(idx_pair) as usize;
            let src0 = base_a0.wrapping_add(idx0 * 0x70 + 0x10);
            let svp = start_vec.as_mut_ptr() as usize;
            wr::<u64>(svp, rd::<u64>(src0));
            wr::<u32>(svp + 0x08, rd::<u32>(src0 + 0x08));

            let rpp = ray_params.as_mut_ptr() as usize;
            wr::<u64>(rpp, 0x21);
            wr::<u8>(rpp + 0x08, 0);
            wr::<usize>(rpp + 0x10, (&mut ignore_obj as *mut usize) as usize);

            let hit = exe::call_va!(
                0x1_4214_f5d0,
                (
                    param_2,
                    svp,
                    evp,
                    rpp,
                    hit_out.as_mut_ptr() as usize,
                    0u32
                ) -> u8
            );
            wr::<u8>(item + 0x0e, hit ^ 1);
            wr::<f64>(item, time_f64);
            ignore_obj = vtbl_base;
        }
        let _ = ignore_obj;
    }

    if rdi_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (rdi_ptr));
    }
}

/// `anonymous_namespace_1424e1430`
/// Original: 0x1424e1430 (836 bytes, Throws::Caught)
pub unsafe extern "win64" fn anonymous_namespace_1424e1430(
    param_1: usize,
    param_2: usize,
    _param_3_xmm2: usize,
) {
    let [_, _, xmm7, _] = crate::simd_ops::take_xmm_args();
    let param_3_u64 = xmm7 as u64;

    let mut rsi_ptr: usize = 0;
    let mut len: u32 = 0;
    let mut edi_cap: u32 = 0;
    let init_cap = rd::<u32>(param_1 + 0xa8);
    if init_cap != 0 {
        edi_cap = init_cap;
        rsi_ptr = exe::call_va!(0x1_4241_c0a0, ((init_cap as u64) * 8, 8u32) -> usize);
    }

    let mut rbx = rd::<usize>(param_1 + 0xa0);
    let r13_end = rbx.wrapping_add((rd::<u32>(param_1 + 0xa8) as usize).wrapping_mul(0x70));
    while rbx != r13_end {
        if len == edi_cap {
            let old_len = len as usize;
            let old_cap = edi_cap as usize;
            let new_cap = (old_cap + (old_cap >> 1)).max(old_len + 1);
            let new_ptr = exe::call_va!(0x1_4241_c0a0, ((new_cap * 8) as u64, 8u32) -> usize);
            exe::call_va!(
                0x1_4183_acc8,
                (new_ptr, rsi_ptr, (old_len * 8) as u64)
            );
            if rsi_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (rsi_ptr));
            }
            rsi_ptr = new_ptr;
            edi_cap = new_cap as u32;
        }
        wr::<usize>(rsi_ptr.wrapping_add((len as usize) * 8), rbx);
        len = len.wrapping_add(1);
        rbx = rbx.wrapping_add(0x70);
    }

    let rdi_end = rsi_ptr.wrapping_add((len as usize) * 8);
    let mut first_arg = rsi_ptr;
    let mut last_arg = rdi_end;
    exe::call_va!(
        0x1_413f_cf40,
        (
            (&mut first_arg as *mut usize) as usize,
            (&mut last_arg as *mut usize) as usize,
            len as u64,
            param_3_u64
        )
    );

    let mut r13w: u16 = 0;
    let limits: [u8; 4] = [0x10, 0x0a, 0x03, 0x01];
    let mut counts: [u8; 4] = [0; 4];

    if rsi_ptr != rdi_end {
        let vtbl_probe = exe::ptr::<u8>(0x1_4341_61a0) as usize;
        let mut ignore_obj: usize;
        let mut end_vec = [0u8; 16];
        let mut start_vec = [0u8; 16];
        let mut ray_params = [0u8; 0x18];
        let mut hit_out: [u8; 0x58];

        let mut cur = rsi_ptr;
        while cur != rdi_end {
            let r14 = rd::<usize>(cur);
            if r13w >= 0x1e {
                break;
            }
            let rounded = crate::simd_ops::vroundss(0, rd::<u32>(r14 + 0x48), 0xc) as u32;
            let eax = crate::simd_ops::cvt_f32_i32(f32::from_bits(rounded)) as i32;
            let mut ecx = (rd::<u8>(r14 + 0x60) as i32).wrapping_add(eax);
            if ecx < 0 {
                ecx = 0;
            } else if ecx > 4 {
                ecx = 4;
            }
            if rd::<i32>(r14 + 0x4c) >= 0 && (ecx as u32) < 4 {
                let r12 = ecx as usize;
                let r15b = counts[r12];
                if r15b < limits[r12] {
                    let pos_vec = rd::<u128>(r14 + 0x10);
                    let mask = crate::simd_ops::vmovmskps(crate::simd_ops::vcmpps(pos_vec, 0, 0));
                    if mask != 0xf {
                        let (ret_dfb90, _) = crate::simd_ops::call_va8_xmm(
                            0x1_424d_fb90,
                            r14 as u64,
                            param_2 as u64,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            [0, 0, xmm7, 0],
                        );
                        let eax_idx = ret_dfb90 as i32;

                        ignore_obj = vtbl_probe;
                        let evp = end_vec.as_mut_ptr() as usize;
                        wr::<u64>(evp, rd::<u64>(param_1 + 0x40));
                        wr::<u32>(evp + 0x08, rd::<u32>(param_1 + 0x48));

                        let svp = start_vec.as_mut_ptr() as usize;
                        wr::<u64>(svp, rd::<u64>(r14 + 0x10));
                        wr::<u32>(svp + 0x08, rd::<u32>(r14 + 0x18));

                        let rpp = ray_params.as_mut_ptr() as usize;
                        wr::<u64>(rpp, 0x21);
                        wr::<u8>(rpp + 0x08, 1);
                        wr::<usize>(rpp + 0x10, (&mut ignore_obj as *mut usize) as usize);

                        hit_out = [0u8; 0x58];
                        let hit = exe::call_va!(
                            0x1_4214_f5d0,
                            (
                                param_2,
                                svp,
                                evp,
                                rpp,
                                hit_out.as_mut_ptr() as usize,
                                0u32
                            ) -> u8
                        );
                        wr::<u8>(r14 + 0x63, hit ^ 1);
                        r13w = r13w.wrapping_add(1);
                        if eax_idx >= 0 {
                            exe::call_va!(0x1_424e_08e0, (param_1, r14, eax_idx as u32));
                            counts[r12] = r15b.wrapping_add(1);
                            r13w = r13w.wrapping_add(1);
                        }
                    }
                }
            }
            cur = cur.wrapping_add(8);
        }
    }

    if rsi_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (rsi_ptr));
    }
}

/// `anonymous_namespace_1424e1e80`
/// Original: 0x1424e1e80 (841 bytes, Throws::No)
pub unsafe extern "win64" fn anonymous_namespace_1424e1e80(
    param_1: usize,
    param_2: usize,
    _param_3_xmm2: usize,
    param_4: u8,
    param_5: i32,
) -> u64 {
    let [_, _, xmm2, _] = crate::simd_ops::take_xmm_args();
    let param_3 = f32::from_bits(xmm2 as u32);

    let v0 = rd::<u128>(param_2);
    let xmm4 = crate::simd_ops::vsubps(v0, rd::<u128>(param_1));
    let xmm3 = crate::simd_ops::vpermilps(xmm4, 0x55);

    let table = (exe::ptr::<u8>(0x1_43c4_7ce0) as usize)
        .wrapping_add(((param_5 as isize) * 0x70) as usize);

    let scaled_y = f32::from_bits(xmm3 as u32) * rd::<f32>(table + 0x58);
    let xmm0_bcast = crate::simd_ops::vbroadcastss(scaled_y.to_bits());
    let xmm2_vec = crate::simd_ops::vblendps(xmm4, xmm0_bcast, 0x2);
    let dp = crate::simd_ops::vdpps(xmm2_vec, xmm2_vec, 0x7f);
    let dist = f32::from_bits(crate::simd_ops::vsqrtps(dp) as u32);

    let diff_44 = rd::<f32>(table + 0x44) - dist;
    let abs_44 = f32::from_bits(diff_44.to_bits() & 0x7fff_ffff);
    let f48 = rd::<f32>(table + 0x48);
    let xmm7 = f48 - abs_44;
    let xmm14 = lerp_rcp(xmm7, 0.0f32, 0.0f32, f48, 100.0f32);

    let f4c = rd::<f32>(table + 0x4c);
    let xmm4_factor = if f4c > param_3 {
        let f54 = rd::<f32>(table + 0x54);
        let d = f32::from_bits((f4c - param_3).to_bits() & 0x7fff_ffff);
        lerp_rcp(f54 - d, 0.0f32, 0.0f32, f54, 1.0f32)
    } else {
        let f50 = rd::<f32>(table + 0x50);
        if param_3 > f50 {
            let f54 = rd::<f32>(table + 0x54);
            let d = f32::from_bits((f50 - param_3).to_bits() & 0x7fff_ffff);
            lerp_rcp(f54 - d, 0.0f32, 0.0f32, f54, 1.0f32)
        } else {
            1.0f32
        }
    };

    let vis_weight = if param_4 == 0 {
        rd::<f32>(table + 0x64)
    } else {
        1.0f32
    };
    let f60 = rd::<f32>(table + 0x60);
    let f5c = rd::<f32>(table + 0x5c);
    let ratio = f5c / (vis_weight * f60 + f5c);
    let xmm6 = xmm4_factor * xmm14;
    let xmm3_out = lerp_rcp(ratio, 0.0f32, xmm14, 1.0f32, xmm6);
    let clamped_lo = if xmm3_out > 0.0f32 { xmm3_out } else { 0.0f32 };
    let res = if clamped_lo < 100.0f32 { clamped_lo } else { 100.0f32 };
    let bits = res.to_bits();
    crate::simd_ops::set_xmm_ret(bits as u128);
    bits as u64
}

/// `unknown_1424e2e60`
/// Original: 0x1424e2e60 (642 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1424e2e60(
    param_1: usize,
    param_2: usize,
    _param_3_xmm2: usize,
) {
    let [_, _, xmm6, _] = crate::simd_ops::take_xmm_args();
    if rd::<u8>(exe::ptr::<u8>(0x1_4387_2958) as usize) != 0
        || rd::<u64>(exe::ptr::<u64>(0x1_43a4_6568) as usize) == 0
    {
        return;
    }

    let cam = exe::call_va!(0x1_424d_4a00, () -> usize);
    let mut xmm0 = rd::<u64>(cam + 0xc4) as u128;
    xmm0 = crate::simd_ops::vinsertps(xmm0, rd::<u32>(cam + 0xcc) as u128, 0x20);
    wr::<u128>(param_1, xmm0);

    let mut xmm2 = rd::<u64>(cam + 0x30) as u128;
    xmm2 = crate::simd_ops::vinsertps(xmm2, rd::<u32>(cam + 0x38) as u128, 0x20);
    wr::<u128>(param_1 + 0x10, xmm2);

    let scale = crate::simd_ops::vbroadcastss(rd::<u32>(exe::ptr::<u32>(0x1_4387_2a38) as usize));
    let scaled = crate::simd_ops::vmulps(scale, xmm2);
    let pos_20 = crate::simd_ops::vaddps(scaled, rd::<u128>(param_1 + 0x40));
    let cur_0 = rd::<u128>(param_1);
    wr::<u128>(param_1 + 0x20, pos_20);
    wr::<u32>(param_1 + 0x108, 0);
    wr::<u128>(param_1 + 0x40, cur_0);

    for _ in 0..6 {
        let (ret_dfb90, _) = crate::simd_ops::call_va8_xmm(
            0x1_424d_fb90,
            (param_1 + 0x30) as u64,
            param_2 as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, xmm6, 0],
        );
        let eax = ret_dfb90 as i32;
        if eax >= 0 {
            exe::call_va!(0x1_424e_08e0, (param_1, param_1 + 0x30, eax as u32));
        }
    }

    exe::call_va!(0x1_424e_0030, (param_1));
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_424e_1430,
        param_1 as u64,
        param_2 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm6, 0],
    );
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_424e_10a0,
        param_1 as u64,
        param_2 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, xmm6, 0],
    );

    let mut r10 = rd::<usize>(param_1 + 0xa0);
    let rbx_end = r10.wrapping_add((rd::<u32>(param_1 + 0xa8) as usize).wrapping_mul(0x70));
    if r10 != rbx_end {
        let thresh_900 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6f30) as usize);
        let thresh_3600 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6f64) as usize);
        let thresh_10000 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6f78) as usize);
        let thresh_225 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ee4) as usize);

        while r10 != rbx_end {
            if rd::<i32>(r10 + 0x4c) >= 0 {
                let mut local_30 = rd::<u128>(param_1 + 0x40);
                let mut local_40 = rd::<u128>(r10 + 0x10);
                let f_44 = rd::<u32>(r10 + 0x44);
                let b_63 = rd::<u8>(r10 + 0x63);
                let i_5c = rd::<u32>(r10 + 0x5c);
                let (_, out_xmm0) = crate::simd_ops::call_va8_xmm(
                    0x1_424e_1e80,
                    (&mut local_40 as *mut u128) as usize as u64,
                    (&mut local_30 as *mut u128) as usize as u64,
                    0,
                    b_63 as u64,
                    i_5c as u64,
                    0,
                    0,
                    0,
                    [0, 0, f_44 as u128, 0],
                );
                wr::<u32>(r10 + 0x40, out_xmm0 as u32);
            }
            let diff = crate::simd_ops::vsubps(rd::<u128>(r10 + 0x10), rd::<u128>(param_1 + 0x20));
            let dp = crate::simd_ops::vdpps(diff, diff, 0x7f);
            let dist_sq = f32::from_bits(dp as u32);
            let bucket: u8 = if !(thresh_225 <= dist_sq) {
                0
            } else if !(thresh_900 <= dist_sq) {
                1
            } else if !(thresh_3600 <= dist_sq) {
                2
            } else if thresh_10000 > dist_sq {
                3
            } else {
                4
            };
            wr::<u8>(r10 + 0x60, bucket);
            r10 = r10.wrapping_add(0x70);
        }
    }

    exe::call_va!(0x1_424e_0b10, (param_1));

    let g_29e0_addr = exe::ptr::<u8>(0x1_4387_29e0) as usize;
    if rd::<u8>(g_29e0_addr) != 0 {
        let mut rax = rd::<usize>(param_1 + 0xa0);
        let rcx_end = rax.wrapping_add((rd::<u32>(param_1 + 0xa8) as usize).wrapping_mul(0x70));
        while rax != rcx_end {
            if rd::<i32>(rax + 0x4c) != -1 {
                wr::<u32>(rax + 0x4c, u32::MAX);
                wr::<u32>(rax + 0x50, 0);
                wr::<u32>(rax + 0x40, 0);
                wr::<u8>(rax + 0x60, 0);
                wr::<u32>(rax + 0x48, 0);
            }
            rax = rax.wrapping_add(0x70);
        }
        wr::<u8>(g_29e0_addr, 0);
        wr::<u8>(exe::ptr::<u8>(0x1_4387_29dc) as usize, 1);
        exe::call_va!(0x1_4119_f940, (param_1 + 0xb0));
    }
}

/// `unknown_142517660`
/// Original: 0x142517660 (748 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142517660(param_1: usize) {
    let len_10 = rd::<u64>(param_1 + 0x10);
    loop {
        let pos = rd::<u32>(param_1 + 0x18);
        if (pos as u64) >= len_10 {
            break;
        }
        let base_8 = rd::<usize>(param_1 + 0x08);
        let ch = rd::<u8>(base_8.wrapping_add(pos as usize));
        if ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n' || ch == 0x0b || ch == 0x0c {
            if ch == b'\n' {
                wr::<u32>(param_1 + 0x1c, rd::<u32>(param_1 + 0x1c).wrapping_add(1));
                wr::<u32>(param_1 + 0x20, pos.wrapping_add(1));
            }
            wr::<u32>(param_1 + 0x18, pos.wrapping_add(1));
        } else {
            break;
        }
    }

    wr::<u128>(param_1 + 0x48, rd::<u128>(param_1 + 0x2c));
    let mut local_20 = [0u8; 0x40];
    let l20p = local_20.as_mut_ptr() as usize;
    let mut tok_ptr = exe::call_va!(0x1_4252_79d0, (param_1 + 0x08, l20p) -> usize);

    loop {
        wr::<u128>(param_1 + 0x28, rd::<u128>(tok_ptr));
        wr::<u128>(param_1 + 0x38, rd::<u128>(tok_ptr + 0x10));
        let kind = rd::<u32>(param_1 + 0x28);
        if (kind.wrapping_sub(0x118) & 0xffff_fffc) != 0 || kind == 0x11a {
            return;
        }
        if rd::<u8>(param_1 + 0x03) != 0 {
            wr::<u32>(l20p, rd::<u32>(param_1 + 0x28));
            let xmm1 = rd::<u128>(param_1 + 0x2c);
            wr::<u128>(l20p + 0x04, xmm1);
            let rdx_90 = rd::<usize>(param_1 + 0x90);
            if rdx_90 == rd::<usize>(param_1 + 0x98) {
                exe::call_va!(0x1_4146_ff90, (param_1 + 0x88, rdx_90, l20p));
            } else {
                wr::<u128>(rdx_90, rd::<u128>(l20p));
                wr::<u32>(rdx_90 + 0x10, (xmm1 >> 96) as u32);
                wr::<usize>(param_1 + 0x90, rdx_90.wrapping_add(0x14));
            }
        }
        let kind2 = rd::<u32>(param_1 + 0x28);
        if kind2 == 0x11b {
            return;
        }
        if kind2 == 0x118 {
            let mut edx_len = rd::<u32>(param_1 + 0x3c);
            if edx_len != 0 {
                let r8_str = rd::<usize>(param_1 + 0x40);
                if rd::<u8>(r8_str) == b'!' {
                    loop {
                        let prev = rd::<u8>(r8_str.wrapping_add((edx_len as usize) - 1));
                        if prev != b' ' && prev.wrapping_sub(9) > 4 {
                            break;
                        }
                        edx_len -= 1;
                        if edx_len == 0 {
                            break;
                        }
                    }
                    let mut cl_b8 = rd::<u8>(param_1 + 0xb8);
                    wr::<u8>(l20p, cl_b8);
                    let mut xmm1 = rd::<u128>(param_1 + 0x2c);
                    wr::<u128>(l20p + 0x04, xmm1);
                    let rax_end = r8_str.wrapping_add(edx_len as usize);
                    let rdx_start = r8_str.wrapping_add(1);
                    wr::<u128>(l20p + 0x18, 0);
                    wr::<u64>(l20p + 0x28, 0);
                    wr::<u64>(l20p + 0x30, 0);
                    if rdx_start == rax_end {
                        wr::<u64>(l20p + 0x28, 0);
                        wr::<u64>(l20p + 0x30, 0x0f);
                        wr::<u8>(l20p + 0x18, 0);
                    } else {
                        exe::call_va!(
                            0x1_4020_50d0,
                            (l20p + 0x18, rdx_start, (rax_end - rdx_start) as u64)
                        );
                        xmm1 = rd::<u128>(l20p + 0x04);
                        cl_b8 = rd::<u8>(l20p);
                    }
                    let rdx_a8 = rd::<usize>(param_1 + 0xa8);
                    let cap_str: u64;
                    if rdx_a8 == rd::<usize>(param_1 + 0xb0) {
                        exe::call_va!(0x1_4147_0170, (param_1 + 0xa0, rdx_a8, l20p));
                        cap_str = rd::<u64>(l20p + 0x30);
                    } else {
                        wr::<u8>(rdx_a8, cl_b8);
                        wr::<u128>(rdx_a8 + 0x04, xmm1);
                        wr::<u128>(rdx_a8 + 0x18, rd::<u128>(l20p + 0x18));
                        wr::<u128>(rdx_a8 + 0x28, rd::<u128>(l20p + 0x28));
                        wr::<u8>(l20p + 0x18, 0);
                        wr::<usize>(param_1 + 0xa8, rdx_a8.wrapping_add(0x38));
                        cap_str = 0x0f;
                    }
                    if cap_str > 0x0f {
                        let mut rcx_mem = rd::<usize>(l20p + 0x18);
                        if cap_str.wrapping_add(1) >= 0x1000 {
                            let raw_ptr = rd::<usize>(rcx_mem - 8);
                            if rcx_mem.wrapping_sub(raw_ptr).wrapping_sub(8) > 0x1f {
                                exe::call_iat!(0x1_4264_67f0, ());
                                unreachable!();
                            }
                            rcx_mem = raw_ptr;
                        }
                        exe::call_va!(0x1_4183_4d30, (rcx_mem));
                    }
                }
            }
        }
        let r9_len = rd::<u64>(param_1 + 0x10);
        loop {
            let r8d = rd::<u32>(param_1 + 0x18);
            if (r8d as u64) >= r9_len {
                break;
            }
            let rax_base = rd::<usize>(param_1 + 0x08);
            let ch = rd::<u8>(rax_base.wrapping_add(r8d as usize));
            if ch != b' ' && ch != b'\t' && ch != b'\r' && ch != b'\n' && ch != 0x0b && ch != 0x0c
            {
                break;
            }
            let rcx_base = rd::<usize>(param_1 + 0x08);
            if rd::<u8>(rcx_base.wrapping_add(r8d as usize)) == b'\n' {
                wr::<u32>(param_1 + 0x1c, rd::<u32>(param_1 + 0x1c).wrapping_add(1));
                wr::<u32>(param_1 + 0x20, r8d.wrapping_add(1));
            }
            wr::<u32>(param_1 + 0x18, r8d.wrapping_add(1));
        }
        tok_ptr = exe::call_va!(0x1_4252_79d0, (param_1 + 0x08, l20p) -> usize);
    }
}

/// `unknown_1425353e0`
/// Original: 0x1425353e0 (423 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425353e0(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> usize {
    if param_2 > 0x20000 || param_3 != 0 {
        if param_2 > 0x20000 && param_2 > 0x7fff_ffff_ffff_ffff {
            let msg = exe::ptr::<u8>(0x1_4342_54f8) as usize;
            exe::call_va!(0x1_4253_86a0, (0x84u32, msg, param_2));
            return 0;
        }
        let r11 = exe::call_va!(0x1_4253_a310, (param_2) -> u64);
        let bin_arg = if r11 > 0x100_0000 || param_3 != 0 {
            0x8000_0000u64
        } else {
            r11
        };
        let b = exe::call_va!(0x1_4253_6cb0, (bin_arg) -> u8) as usize;
        let pq = param_1.wrapping_add(b * 0x18 + 0x410);
        return exe::call_va!(0x1_4253_64e0, (param_1, pq, r11, param_3) -> usize);
    }

    let b = exe::call_va!(0x1_4253_6cb0, (param_2) -> u8) as usize;
    let rsi = param_1.wrapping_add(b * 0x18);
    let rbx = rd::<usize>(rsi + 0x410);
    if rbx != 0 {
        let mut rax = rd::<u64>(rbx + 0x28);
        if (rax & !3u64) != 0 {
            let rdx_tfree: usize;
            loop {
                let rdx_cand = (rax & !3u64) as usize;
                let rcx_new = rax & 3u64;
                let (ok, old) =
                    crate::simd_ops::atomic_cmpxchg_u64((rbx + 0x28) as u64, rax, rcx_new);
                if ok {
                    rdx_tfree = rdx_cand;
                    break;
                }
                rax = old;
            }
            if rdx_tfree != 0 {
                let mut cur = rd::<usize>(rdx_tfree);
                let mut ecx_cnt: u32 = 1;
                let r8d_cap = rd::<u16>(rbx + 0x0a) as u32;
                let mut r9_tail = rdx_tfree;
                let mut corrupted = false;
                while cur != 0 {
                    if ecx_cnt > r8d_cap {
                        corrupted = true;
                        break;
                    }
                    r9_tail = cur;
                    ecx_cnt = ecx_cnt.wrapping_add(1);
                    cur = rd::<usize>(cur);
                }
                if corrupted || ecx_cnt > r8d_cap {
                    let msg = exe::ptr::<u8>(0x1_4342_54d8) as usize;
                    exe::call_va!(0x1_4253_86a0, (0x0eu32, msg));
                } else {
                    let local_free = rd::<usize>(rbx + 0x20);
                    wr::<usize>(r9_tail, local_free);
                    wr::<u32>(rbx + 0x18, rd::<u32>(rbx + 0x18).wrapping_sub(ecx_cnt));
                    wr::<usize>(rbx + 0x20, rdx_tfree);
                }
            }
        }
        let rax_20 = rd::<usize>(rbx + 0x20);
        if rax_20 != 0 {
            if rd::<usize>(rbx + 0x10) != 0 {
                wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & 1);
                return rbx;
            }
            wr::<usize>(rbx + 0x10, rax_20);
            wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & !1u8);
            wr::<usize>(rbx + 0x20, 0);
        }
        if rd::<usize>(rbx + 0x10) != 0 {
            wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & 1);
            return rbx;
        }
    }
    exe::call_va!(0x1_4253_5770, (param_1, rsi + 0x410, 1u8) -> usize)
}

/// `unknown_1425386a0`
/// Original: 0x1425386a0 (186 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425386a0(
    param_1: u32,
    param_2: usize,
    param_3: u64,
    param_4: u64,
    param_5: u64,
    param_6: u64,
) {
    let g_c34 = exe::ptr::<u32>(0x1_4387_2c34) as usize;
    let g_c30 = exe::ptr::<u32>(0x1_4387_2c30) as usize;
    if rd::<u32>(g_c34) == 0 {
        exe::call_va!(0x1_4253_75a0, (g_c30));
    }
    let mut should_print = true;
    if rd::<u32>(g_c30) == 0 {
        let g_bf4 = exe::ptr::<u32>(0x1_4387_2bf4) as usize;
        let g_bf0 = exe::ptr::<u32>(0x1_4387_2bf0) as usize;
        if rd::<u32>(g_bf4) == 0 {
            exe::call_va!(0x1_4253_75a0, (g_bf0));
        }
        if rd::<u32>(g_bf0) == 0 {
            should_print = false;
        } else {
            let g_7bec = exe::ptr::<i32>(0x1_4385_7bec) as usize;
            if rd::<i32>(g_7bec) >= 0 {
                let g_d1e8 = exe::ptr::<u64>(0x1_43bb_d1e8) as usize;
                let old = crate::simd_ops::atomic_xadd_u64(g_d1e8 as u64, 1);
                if (old as i32) > rd::<i32>(g_7bec) {
                    should_print = false;
                }
            }
        }
    }
    if should_print {
        let prefix = exe::ptr::<u8>(0x1_4342_58a0) as usize;
        let mut va_buf: [u64; 4] = [param_3, param_4, param_5, param_6];
        exe::call_va!(
            0x1_4253_7b60,
            (
                0u64,
                0u64,
                prefix,
                param_2,
                va_buf.as_mut_ptr() as usize
            )
        );
    }
    let g_37a8 = exe::ptr::<u64>(0x1_43a6_37a8) as usize;
    let cb = rd::<u64>(g_37a8);
    if cb != 0 {
        let cb2 = rd::<u64>(g_37a8);
        exe::call_va!(cb2, (param_1, 0u32));
    }
}

/// `unknown_14253a310`
/// Original: 0x14253a310 (149 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253a310(mut param_1: u64) -> u64 {
    let r8: u64 = if param_1 < 0x8_0000 {
        rd::<u64>(exe::ptr::<u64>(0x1_4387_2f30) as usize)
    } else if param_1 < 0x20_0000 {
        return param_1.wrapping_add(0xffff) & !0xffffu64;
    } else if param_1 < 0x80_0000 {
        return param_1.wrapping_add(0x3_ffff) & !0x3_ffffu64;
    } else if param_1 < 0x200_0000 {
        return param_1.wrapping_add(0xf_ffff) & !0xf_ffffu64;
    } else {
        0x40_0000
    };
    if param_1 < !r8 {
        let mask = r8.wrapping_sub(1);
        param_1 = param_1.wrapping_add(mask);
        if (mask & r8) == 0 {
            return param_1 & !mask;
        }
        param_1 = param_1.wrapping_sub(param_1 % r8);
    }
    param_1
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

pub unsafe fn diff_anonymous_namespace_1424e0b10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUF: [u8; 0xc0] = [0; 0xc0];
    static mut RET_1830: u8 = 0;
    static mut LOG_1830: Vec<(u8, u32)> = Vec::new();
    static mut LOG_E830: Vec<(u32, u32)> = Vec::new();

    unsafe extern "win64" fn mock_a9e0(_bytes: u64, _cnt: u64) -> usize {
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_1830(_p1: usize, item: usize) -> u8 {
        LOG_1830.push((rd::<u8>(item + 0x44), rd::<u32>(item + 0x10)));
        RET_1830
    }
    unsafe extern "win64" fn mock_ec40(_vec: usize, _new_cap: u64) {}
    unsafe extern "win64" fn mock_e830(vec_ptr: usize) {
        LOG_E830.push((rd::<u32>(vec_ptr + 0x08), rd::<u32>(vec_ptr + 0x0c)));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4024_a9e0, mock_a9e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424e_1830, mock_1830 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_403f_ec40, mock_ec40 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, found: bool, ret_1830: u8| -> (u32, u8, u32, u32, Vec<u32>, Vec<(u8, u32)>, Vec<(u32, u32)>) {
        ALLOC_BUF = [0; 0xc0];
        RET_1830 = ret_1830;
        LOG_1830.clear();
        LOG_E830.clear();

        let mut items_118 = [0u8; 0x90];
        let i118p = items_118.as_mut_ptr() as usize;
        wr::<u32>(i118p, 0);
        wr::<u32>(i118p + 0x10, 0xaaaa_bbbb);
        wr::<u64>(i118p + 0x20, 0x1122_3344_5566_7788);
        wr::<u32>(i118p + 0x30, 1);
        wr::<u64>(i118p + 0x50, 0x1122_3344_5566_7788);

        let mut val_obj = [0u8; 0x70];
        let vp = val_obj.as_mut_ptr() as usize;
        wr::<u32>(vp + 0x54, 77);
        wr::<u32>(vp + 0x58, 0);
        wr::<u8>(vp + 0x61, 1);

        let mut node = [0u8; 0x20];
        let np = node.as_mut_ptr() as usize;
        wr::<u64>(np + 0x10, if found { 0x1122_3344_5566_7788 } else { 0x9999 });
        wr::<usize>(np + 0x18, vp);

        let sentinel: u64 = 0xdead_beef;
        let buckets: [usize; 2] = [np, np];

        let mut ids: [u32; 4] = [10, 77, 20, 77];
        let mut f0_entry = [0u8; 0x20];
        let f0p = f0_entry.as_mut_ptr() as usize;
        wr::<u8>(f0p + 0x0c, 1);
        wr::<usize>(f0p + 0x10, ids.as_mut_ptr() as usize);
        wr::<u32>(f0p + 0x18, 4);

        let mut item_100 = [0u8; 0x50];
        let i100p = item_100.as_mut_ptr() as usize;
        wr::<u32>(i100p + 0x10, 0x1234);
        wr::<u8>(i100p + 0x44, 9);

        let mut p1 = [0u8; 0x130];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xb8, sentinel as usize);
        wr::<usize>(p1p + 0xc8, buckets.as_ptr() as usize);
        wr::<u64>(p1p + 0xe0, 0);
        wr::<usize>(p1p + 0xf0, f0p);
        wr::<u32>(p1p + 0xf8, 1);
        wr::<usize>(p1p + 0x100, i100p);
        wr::<u32>(p1p + 0x108, 1);
        wr::<usize>(p1p + 0x118, i118p);
        wr::<u32>(p1p + 0x120, 2);
        wr::<u32>(p1p + 0x124, 3);

        f(p1p);
        let kept_len = rd::<u32>(f0p + 0x18) as usize;
        (
            rd::<u32>(p1p + 0x120),
            rd::<u8>(vp + 0x61),
            rd::<u32>(vp + 0x10),
            rd::<u32>(vp + 0x58),
            ids[..kept_len].to_vec(),
            LOG_1830.clone(),
            LOG_E830.clone(),
        )
    };

    for (found, ret_1830) in [(true, 1u8), (false, 0u8), (false, 1u8)] {
        let o = run(f_o, found, ret_1830);
        let n = run(f_n, found, ret_1830);
        crate::diff_eq!(o, n, "1424e0b10 found={} ret_1830={}", found, ret_1830);
    }
    Ok(3)
}

pub unsafe fn diff_anonymous_namespace_1424e10a0(orig: usize, new: usize) -> Result<u32, String> {
    let _g1 = GlobalSave::new(0x1_4387_2970, 4);

    static mut ALLOC_BUF: [usize; 8] = [0; 8];
    static mut HIT_RET: u8 = 0;
    static mut LOG_RAY: Vec<(u32, u32, u64, u8, u8)> = Vec::new();
    static mut SORT_COUNTS: (usize, usize) = (0, 0);

    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}
    unsafe extern "win64" fn mock_c170(first: usize, mid: usize, last: usize) {
        let f = rd::<usize>(first);
        let m = rd::<usize>(mid);
        let l = rd::<usize>(last);
        SORT_COUNTS = ((m - f) / 8, (l - f) / 8);
    }
    unsafe extern "win64" fn mock_f5d0(
        _p2: usize,
        start_v: usize,
        end_v: usize,
        params: usize,
        _hit_out: usize,
        flag: u8,
    ) -> u8 {
        LOG_RAY.push((
            rd::<u32>(start_v),
            rd::<u32>(end_v),
            rd::<u64>(params),
            rd::<u8>(params + 0x08),
            flag,
        ));
        HIT_RET
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413f_c170, mock_c170 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4214_f5d0, mock_f5d0 as *const ())?;

    let run = |fn_addr: usize, vis_flag: u8, hit_ret: u8| -> (u32, u8, u64, (usize, usize), Vec<(u32, u32, u64, u8, u8)>) {
        ALLOC_BUF = [0; 8];
        HIT_RET = hit_ret;
        LOG_RAY.clear();
        SORT_COUNTS = (0, 0);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_2970) as usize, 3.0);

        let mut probes = [0u8; 2 * 0x70];
        let pp = probes.as_mut_ptr() as usize;
        wr::<f32>(pp + 0x10, 1.5);
        wr::<f32>(pp + 0x70 + 0x10, 4.5);
        wr::<u8>(pp + 0x70 + 0x63, vis_flag);

        let pair_indices: [u32; 2] = [0, 1];
        let mut edge = [0u8; 0x20];
        let ep = edge.as_mut_ptr() as usize;
        wr::<f64>(ep, 2.0);
        wr::<u8>(ep + 0x0d, 1);
        wr::<usize>(ep + 0x10, pair_indices.as_ptr() as usize);
        wr::<u32>(ep + 0x18, 2);

        let mut p1 = [0u8; 0x100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xa0, pp);
        wr::<u32>(p1p + 0xa8, 2);
        wr::<usize>(p1p + 0xf0, ep);
        wr::<u32>(p1p + 0xf8, 1);

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            0x55,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 6.0f64.to_bits() as u128, 0],
        );
        (
            rd::<u32>(ep + 0x08),
            rd::<u8>(ep + 0x0e),
            rd::<u64>(ep),
            SORT_COUNTS,
            LOG_RAY.clone(),
        )
    };

    for (vis_flag, hit_ret) in [(0u8, 0u8), (1u8, 1u8)] {
        let o = run(orig, vis_flag, hit_ret);
        let n = run(new, vis_flag, hit_ret);
        crate::diff_eq!(o, n, "1424e10a0 vis={} hit={}", vis_flag, hit_ret);
    }
    Ok(2)
}

pub unsafe fn diff_anonymous_namespace_1424e1430(orig: usize, new: usize) -> Result<u32, String> {
    static mut ALLOC_BUF: [usize; 8] = [0; 8];
    static mut DFB90_RET: i32 = 0;
    static mut LOG_08E0: Vec<u32> = Vec::new();
    static mut LOG_SORT: (u64, u64) = (0, 0);

    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {}
    unsafe extern "win64" fn mock_cf40(_first: usize, _last: usize, cnt: u64, p3: u64) {
        LOG_SORT = (cnt, p3);
    }
    unsafe extern "win64" fn mock_dfb90(_probe: usize, _p2: usize, _p3: usize) -> i32 {
        DFB90_RET
    }
    unsafe extern "win64" fn mock_f5d0(
        _p2: usize,
        _start_v: usize,
        _end_v: usize,
        _params: usize,
        _hit_out: usize,
        _flag: u8,
    ) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_08e0(_p1: usize, _probe: usize, idx: u32) {
        LOG_08E0.push(idx);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413f_cf40, mock_cf40 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_424d_fb90, mock_dfb90 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4214_f5d0, mock_f5d0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_424e_08e0, mock_08e0 as *const ())?;

    let run = |fn_addr: usize, dfb_ret: i32| -> (u8, (u64, u64), Vec<u32>) {
        ALLOC_BUF = [0; 8];
        DFB90_RET = dfb_ret;
        LOG_08E0.clear();
        LOG_SORT = (0, 0);

        let mut probe = [0u8; 0x70];
        let pp = probe.as_mut_ptr() as usize;
        wr::<f32>(pp + 0x10, 2.0);
        wr::<f32>(pp + 0x48, 1.2);
        wr::<i32>(pp + 0x4c, 1);
        wr::<u8>(pp + 0x60, 1);

        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<f32>(p1p + 0x40, 5.0);
        wr::<usize>(p1p + 0xa0, pp);
        wr::<u32>(p1p + 0xa8, 1);

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            0x77,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 4.25f64.to_bits() as u128, 0],
        );
        (rd::<u8>(pp + 0x63), LOG_SORT, LOG_08E0.clone())
    };

    for dfb_ret in [-1i32, 5i32] {
        let o = run(orig, dfb_ret);
        let n = run(new, dfb_ret);
        crate::diff_eq!(o, n, "1424e1430 dfb_ret={}", dfb_ret);
    }
    Ok(2)
}

pub unsafe fn diff_anonymous_namespace_1424e1e80(orig: usize, new: usize) -> Result<u32, String> {
    let _g1 = GlobalSave::new(0x1_43c4_7ce0, 0x70);

    let tbl = exe::ptr::<u8>(0x1_43c4_7ce0) as usize;
    wr::<f32>(tbl + 0x44, 10.0);
    wr::<f32>(tbl + 0x48, 8.0);
    wr::<f32>(tbl + 0x4c, 2.0);
    wr::<f32>(tbl + 0x50, 6.0);
    wr::<f32>(tbl + 0x54, 4.0);
    wr::<f32>(tbl + 0x58, 0.5);
    wr::<f32>(tbl + 0x5c, 3.0);
    wr::<f32>(tbl + 0x60, 2.0);
    wr::<f32>(tbl + 0x64, 0.25);

    let v1: [f32; 4] = [1.0, 2.0, 3.0, 0.0];
    let v2: [f32; 4] = [4.0, 6.0, 3.0, 0.0];

    let mut tests = 0u32;
    for p3 in [1.0f32, 4.0f32, 8.0f32] {
        for p4 in [0u8, 1u8] {
            let r_o = crate::simd_ops::call_va8_xmm(
                orig as u64,
                v1.as_ptr() as usize as u64,
                v2.as_ptr() as usize as u64,
                0,
                p4 as u64,
                0,
                0,
                0,
                0,
                [0, 0, p3.to_bits() as u128, 0],
            )
            .1 as u32;
            let r_n = crate::simd_ops::call_va8_xmm(
                new as u64,
                v1.as_ptr() as usize as u64,
                v2.as_ptr() as usize as u64,
                0,
                p4 as u64,
                0,
                0,
                0,
                0,
                [0, 0, p3.to_bits() as u128, 0],
            )
            .1 as u32;
            crate::diff_eq!(r_o, r_n, "1424e1e80 p3={} p4={}", p3, p4);
            tests += 1;
        }
    }
    Ok(tests)
}

pub unsafe fn diff_unknown_1424e2e60(orig: usize, new: usize) -> Result<u32, String> {
    let _g1 = GlobalSave::new(0x1_4387_2958, 1);
    let _g2 = GlobalSave::new(0x1_43a4_6568, 8);
    let _g3 = GlobalSave::new(0x1_4387_2a38, 4);
    let _g4 = GlobalSave::new(0x1_4387_29e0, 1);
    let _g5 = GlobalSave::new(0x1_4387_29dc, 1);

    static mut CAM_BUF: [u8; 0xe0] = [0; 0xe0];
    static mut CALL_MASK: u32 = 0;

    unsafe extern "win64" fn mock_4a00() -> usize {
        core::ptr::addr_of_mut!(CAM_BUF) as usize
    }
    unsafe extern "win64" fn mock_dfb90(_p1: usize, _p2: usize, _p3: usize) -> i32 {
        -1
    }
    unsafe extern "win64" fn mock_0030(_p1: usize) {
        CALL_MASK |= 1;
    }
    unsafe extern "win64" fn mock_1430(_p1: usize, _p2: usize, _p3: usize) {
        CALL_MASK |= 2;
    }
    unsafe extern "win64" fn mock_10a0(_p1: usize, _p2: usize, _p3: usize) {
        CALL_MASK |= 4;
    }
    unsafe extern "win64" fn mock_1e80(
        _p1: usize,
        _p2: usize,
        _p3: usize,
        _p4: u8,
        _p5: i32,
    ) -> u64 {
        let bits = 77.5f32.to_bits();
        crate::simd_ops::set_xmm_ret(bits as u128);
        bits as u64
    }
    unsafe extern "win64" fn mock_0b10(_p1: usize) {
        CALL_MASK |= 8;
    }
    unsafe extern "win64" fn mock_f940(_p1: usize) {
        CALL_MASK |= 16;
    }

    let _m1 = crate::difftest::Mock::new(0x1_424d_4a00, mock_4a00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424d_fb90, mock_dfb90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424e_0030, mock_0030 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_424e_1430, mock_1430 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_424e_10a0, mock_10a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_424e_1e80, mock_1e80 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_424e_0b10, mock_0b10 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4119_f940, mock_f940 as *const ())?;

    let run = |fn_addr: usize, reset_flag: u8| -> (u32, u32, u8, i32, u8, u8) {
        CAM_BUF = [0; 0xe0];
        CALL_MASK = 0;
        let cp = core::ptr::addr_of_mut!(CAM_BUF) as usize;
        wr::<f32>(cp + 0x30, 1.0);
        wr::<f32>(cp + 0xc4, 2.0);

        wr::<u8>(exe::ptr::<u8>(0x1_4387_2958) as usize, 0);
        wr::<u64>(exe::ptr::<u64>(0x1_43a4_6568) as usize, 1);
        wr::<f32>(exe::ptr::<f32>(0x1_4387_2a38) as usize, 2.0);
        wr::<u8>(exe::ptr::<u8>(0x1_4387_29e0) as usize, reset_flag);
        wr::<u8>(exe::ptr::<u8>(0x1_4387_29dc) as usize, 0);

        let mut probe = [0u8; 0x70];
        let pp = probe.as_mut_ptr() as usize;
        wr::<f32>(pp + 0x10, 30.0);
        wr::<i32>(pp + 0x4c, 1);

        let mut p1 = [0u8; 0x120];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xa0, pp);
        wr::<u32>(p1p + 0xa8, 1);

        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            p1p as u64,
            0x99,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 1.0f64.to_bits() as u128, 0],
        );
        (
            CALL_MASK,
            rd::<u32>(pp + 0x40),
            rd::<u8>(pp + 0x60),
            rd::<i32>(pp + 0x4c),
            rd::<u8>(exe::ptr::<u8>(0x1_4387_29e0) as usize),
            rd::<u8>(exe::ptr::<u8>(0x1_4387_29dc) as usize),
        )
    };

    for reset_flag in [0u8, 1u8] {
        let o = run(orig, reset_flag);
        let n = run(new, reset_flag);
        crate::diff_eq!(o, n, "1424e2e60 reset={}", reset_flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142517660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TOK_CALL: u32 = 0;
    static mut BANG_TEXT: [u8; 8] = *b"!hello  ";
    static mut TOK_BUF: [u8; 0x20] = [0; 0x20];

    unsafe extern "win64" fn mock_79d0(_lex: usize, _out: usize) -> usize {
        let p = core::ptr::addr_of_mut!(TOK_BUF) as usize;
        TOK_BUF = [0; 0x20];
        if TOK_CALL == 0 {
            TOK_CALL += 1;
            wr::<u32>(p, 0x118);
            wr::<u32>(p + 0x04, 0x11);
            wr::<u32>(p + 0x14, 8);
            wr::<usize>(p + 0x18, core::ptr::addr_of!(BANG_TEXT) as usize);
        } else {
            wr::<u32>(p, 0x11b);
            wr::<u32>(p + 0x04, 0x22);
        }
        p
    }
    unsafe extern "win64" fn mock_50d0(dst_str: usize, src_ptr: usize, len: u64) {
        for i in 0..(len as usize) {
            wr::<u8>(dst_str + i, rd::<u8>(src_ptr + i));
        }
        wr::<u64>(dst_str + 0x10, len);
        wr::<u64>(dst_str + 0x18, 0x0f);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4252_79d0, mock_79d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4020_50d0, mock_50d0 as *const ())?;

    let run = |f: F, rec_flag: u8| -> (u32, u32, u32, usize, usize, u8, u64) {
        TOK_CALL = 0;
        let text = b" \n\tcode";
        let mut buf_90 = [0u8; 0x40];
        let b90p = buf_90.as_mut_ptr() as usize;
        let mut buf_a8 = [0u8; 0x80];
        let ba8p = buf_a8.as_mut_ptr() as usize;

        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p + 0x03, rec_flag);
        wr::<usize>(p1p + 0x08, text.as_ptr() as usize);
        wr::<u64>(p1p + 0x10, text.len() as u64);
        wr::<usize>(p1p + 0x90, b90p);
        wr::<usize>(p1p + 0x98, b90p + 0x40);
        wr::<usize>(p1p + 0xa8, ba8p);
        wr::<usize>(p1p + 0xb0, ba8p + 0x70);
        wr::<u8>(p1p + 0xb8, 7);

        f(p1p);
        (
            rd::<u32>(p1p + 0x18),
            rd::<u32>(p1p + 0x1c),
            rd::<u32>(p1p + 0x20),
            rd::<usize>(p1p + 0x90) - b90p,
            rd::<usize>(p1p + 0xa8) - ba8p,
            rd::<u8>(ba8p),
            rd::<u64>(ba8p + 0x28),
        )
    };

    for rec_flag in [0u8, 1u8] {
        let o = run(f_o, rec_flag);
        let n = run(f_n, rec_flag);
        crate::diff_eq!(o, n, "142517660 rec_flag={}", rec_flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1425353e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ERR_CODE: u32 = 0;

    unsafe extern "win64" fn mock_86a0(err: u32, _fmt: usize, _a3: u64) {
        ERR_CODE = err;
    }
    unsafe extern "win64" fn mock_a310(sz: u64) -> u64 {
        (sz + 0xffff) & !0xffff
    }
    unsafe extern "win64" fn mock_6cb0(_sz: u64) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_64e0(_h: usize, _pq: usize, r11: u64, p3: u64) -> usize {
        (r11 ^ p3) as usize
    }
    unsafe extern "win64" fn mock_5770(_h: usize, _pq: usize, _one: u8) -> usize {
        0x7777
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_86a0, mock_86a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_a310, mock_a310 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_6cb0, mock_6cb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_64e0, mock_64e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4253_5770, mock_5770 as *const ())?;

    static mut NODE1: usize = 0;

    let run = |f: F, sz: u64, align: u64| -> (bool, u32, u32, u8, u64) {
        ERR_CODE = 0;
        NODE1 = 0;
        let n1p = core::ptr::addr_of_mut!(NODE1) as usize;

        let mut page = [0u8; 0x38];
        let pgp = page.as_mut_ptr() as usize;
        wr::<u16>(pgp + 0x0a, 4);
        wr::<u8>(pgp + 0x0f, 3);
        wr::<u32>(pgp + 0x18, 5);
        wr::<u64>(pgp + 0x28, (n1p as u64) | 1);

        let mut heap = [0u8; 0x450];
        let hp = heap.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x18 + 0x410, pgp);

        let ret = f(hp, sz, align);
        let ret_norm = if ret == pgp { 1 } else { ret };
        (
            ret_norm == 1,
            ERR_CODE,
            rd::<u32>(pgp + 0x18),
            rd::<u8>(pgp + 0x0f),
            rd::<u64>(pgp + 0x28),
        )
    };

    for (sz, align) in [(64u64, 0u64), (0x4_0000u64, 0u64), (u64::MAX, 0u64)] {
        let o = run(f_o, sz, align);
        let n = run(f_n, sz, align);
        crate::diff_eq!(o, n, "1425353e0 sz={} align={}", sz, align);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1425386a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, usize, u64, u64, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4387_2c30, 8);
    let _g2 = GlobalSave::new(0x1_4387_2bf0, 8);
    let _g3 = GlobalSave::new(0x1_4385_7bec, 4);
    let _g4 = GlobalSave::new(0x1_43bb_d1e8, 8);
    let _g5 = GlobalSave::new(0x1_43a6_37a8, 8);

    static mut PRINT_LOG: Vec<(usize, u64)> = Vec::new();
    static mut CB_LOG: Vec<(u32, u32)> = Vec::new();

    unsafe extern "win64" fn mock_75a0(ptr: usize) {
        wr::<u32>(ptr, 1);
        wr::<u32>(ptr + 4, 1);
    }
    unsafe extern "win64" fn mock_7b60(
        _a1: u64,
        _a2: u64,
        _prefix: usize,
        fmt: usize,
        va_ptr: usize,
    ) {
        PRINT_LOG.push((fmt, rd::<u64>(va_ptr)));
    }
    unsafe extern "win64" fn mock_cb(err: u32, arg: u32) {
        CB_LOG.push((err, arg));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_75a0, mock_75a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_7b60, mock_7b60 as *const ())?;

    let run = |f: F, c30: u32, max_errs: i32| -> (Vec<(usize, u64)>, Vec<(u32, u32)>, u64) {
        PRINT_LOG.clear();
        CB_LOG.clear();
        wr::<u32>(exe::ptr::<u32>(0x1_4387_2c34) as usize, 1);
        wr::<u32>(exe::ptr::<u32>(0x1_4387_2c30) as usize, c30);
        wr::<u32>(exe::ptr::<u32>(0x1_4387_2bf4) as usize, 1);
        wr::<u32>(exe::ptr::<u32>(0x1_4387_2bf0) as usize, 1);
        wr::<i32>(exe::ptr::<i32>(0x1_4385_7bec) as usize, max_errs);
        wr::<u64>(exe::ptr::<u64>(0x1_43bb_d1e8) as usize, 2);
        wr::<usize>(
            exe::ptr::<usize>(0x1_43a6_37a8) as usize,
            mock_cb as *const () as usize,
        );

        f(0x84, 0x1234, 0x9999, 0, 0, 0);
        (
            PRINT_LOG.clone(),
            CB_LOG.clone(),
            rd::<u64>(exe::ptr::<u64>(0x1_43bb_d1e8) as usize),
        )
    };

    for (c30, max_errs) in [(1u32, 0i32), (0u32, 5i32), (0u32, 1i32)] {
        let o = run(f_o, c30, max_errs);
        let n = run(f_n, c30, max_errs);
        crate::diff_eq!(o, n, "1425386a0 c30={} max_errs={}", c30, max_errs);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14253a310(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _g1 = GlobalSave::new(0x1_4387_2f30, 8);

    let mut tests = 0u32;
    for gran in [0x1_0000u64, 0x3_0000u64] {
        wr::<u64>(exe::ptr::<u64>(0x1_4387_2f30) as usize, gran);
        for sz in [
            1u64,
            0x7_ffff,
            0x8_0001,
            0x20_0001,
            0x80_0001,
            0x200_0001,
            u64::MAX - 10,
        ] {
            let o = f_o(sz);
            let n = f_n(sz);
            crate::diff_eq!(o, n, "14253a310 gran={} sz={}", gran, sz);
            tests += 1;
        }
    }
    Ok(tests)
}
