//! Decompiled `rend` Cauldron / ECS / input / animation / streaming hot functions shard S:
//! - `0x1420b2330` (`r_1420b2330`)
//! - `0x1420c4950` (`unknown_1420c4950`)
//! - `0x1420d4ca0` (`unknown_1420d4ca0`)
//! - `0x1420d73d0` (`unknown_1420d73d0`)
//! - `0x1420db7c0` (`unknown_1420db7c0`)
//! - `0x1420e02c0` (`unknown_1420e02c0`)
//! - `0x1420e3d70` (`unknown_1420e3d70`)
//! - `0x1421c0320` (`unknown_1421c0320`)
//! - `0x1422075a0` (`unknown_1422075a0`)
//! - `0x14220ea70` (`unknown_14220ea70`)

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    vaddps, vaddsubps, vblendps, vdpps, vhaddps, vhsubps, vmulps, vpbroadcastb, vpcmpeqb,
    vpermilps, vpmovmskb, vpsignb, vshufps, vsubps,
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
unsafe fn push_u16_vec(vec_ptr: usize, val: u16) {
    let ebp = rd::<u32>(vec_ptr + 0x08);
    let r13 = (ebp as usize).wrapping_mul(2);
    let ecx = rd::<u32>(vec_ptr + 0x0c);
    if r13 == (ecx as usize).wrapping_mul(2) {
        let need = (ebp as u64).wrapping_add(1);
        let grown = ((ecx >> 1) as u64).wrapping_add(ecx as u64);
        let r12 = if grown < need { need } else { grown };
        let new_buf = exe::call_va!(0x1_4241_c0a0, (r12.wrapping_mul(2), 2u32) -> usize);
        let copy_bytes = (rd::<u32>(vec_ptr + 0x08) as usize).wrapping_mul(2);
        let old_buf = rd::<usize>(vec_ptr);
        exe::call_va!(0x1_4183_acc8, (new_buf, old_buf, copy_bytes));
        let old_buf2 = rd::<usize>(vec_ptr);
        if old_buf2 != 0 {
            exe::call_va!(0x1_4241_c190, (old_buf2));
        }
        wr::<usize>(vec_ptr, new_buf);
        wr::<u32>(vec_ptr + 0x08, ebp);
        wr::<u32>(vec_ptr + 0x0c, r12 as u32);
    }
    let buf = rd::<usize>(vec_ptr);
    wr::<u16>(buf.wrapping_add(r13), val);
    wr::<u32>(vec_ptr + 0x08, rd::<u32>(vec_ptr + 0x08).wrapping_add(1));
}

/// `r_1420b2330`
/// Original: `0x1420b2330` (1309 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_1420b2330(
    param_1: u16,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
    param_8: u64,
) {
    let count_groups = param_1 as usize;
    let mut local_58: usize = 0;
    if count_groups != 0 {
        local_58 = exe::call_va!(0x1_4241_c0a0, ((count_groups as u64) << 4, 8u32) -> usize);
        for i in 0..count_groups {
            let p = local_58.wrapping_add(i * 16);
            wr::<u64>(p, 0);
            wr::<u64>(p + 0x08, 0);
        }
    }

    let mut r14_span = rd::<usize>(param_3 + 0x08);
    let r14_end = r14_span.wrapping_add(rd::<usize>(param_3).wrapping_shl(4));
    while r14_span != r14_end {
        let mut rsi = rd::<usize>(r14_span);
        let rsi_end = rsi.wrapping_add((rd::<u32>(r14_span + 0x08) as usize).wrapping_mul(4));
        if rsi != rsi_end {
            let rdi = rd::<usize>(param_4 + 0x08);
            let p2_len = rd::<u64>(param_2);
            let p2_ptr = rd::<u64>(param_2 + 0x08);
            while rsi != rsi_end {
                let r11d = rd::<u32>(rsi);
                let lo16 = (r11d & 0xffff) as u32;
                let found = exe::call_va!(0x1_4248_9ee0, (p2_ptr, p2_len, lo16) -> i32);
                let vec_ptr = local_58.wrapping_add((found as isize as usize).wrapping_mul(16));
                let hi_off = ((r11d >> 16) as usize) << 5;
                let mut rbx = rd::<usize>(rdi.wrapping_add(hi_off).wrapping_add(0x18));
                let rbx_end = rbx.wrapping_add(
                    rd::<usize>(rdi.wrapping_add(hi_off).wrapping_add(0x10)).wrapping_mul(8),
                );
                while rbx != rbx_end {
                    let val = rd::<u64>(rbx) as u16;
                    push_u16_vec(vec_ptr, val);
                    rbx = rbx.wrapping_add(8);
                }
                rsi = rsi.wrapping_add(4);
            }
        }
        r14_span = r14_span.wrapping_add(0x10);
    }

    let mut r14_grp = rd::<usize>(param_2 + 0x08);
    let grp_end = r14_grp.wrapping_add(rd::<usize>(param_2).wrapping_mul(2));
    if r14_grp != grp_end {
        let p2_len = rd::<u64>(param_2);
        let p2_ptr = rd::<u64>(param_2 + 0x08);
        while r14_grp != grp_end {
            let grp_id = rd::<u16>(r14_grp) as u32;
            let grp_off = (grp_id as usize) << 5;
            let sub_cnt = rd::<u32>(param_5.wrapping_add(grp_off).wrapping_add(0x18068)) as usize;
            let mut rsi = rd::<usize>(param_5.wrapping_add(grp_off).wrapping_add(0x18050));
            let found = exe::call_va!(0x1_4248_9ee0, (p2_ptr, p2_len, grp_id) -> i32);
            let vec_ptr = local_58.wrapping_add((found as isize as usize).wrapping_shl(4));
            let rsi_end = rsi.wrapping_add(sub_cnt.wrapping_mul(2));
            while rsi != rsi_end {
                let comp_id = rd::<u16>(rsi) as usize;
                let tbl_10 = rd::<usize>(param_6 + 0x10);
                let mut rbx = rd::<usize>(tbl_10.wrapping_add(comp_id * 16).wrapping_add(0x08));
                let cnt = rd::<usize>(tbl_10.wrapping_add(comp_id * 16));
                let rbx_end = rbx.wrapping_add(cnt.wrapping_mul(2));
                while rbx != rbx_end {
                    let val = rd::<u16>(rbx);
                    push_u16_vec(vec_ptr, val);
                    rbx = rbx.wrapping_add(2);
                }
                rsi = rsi.wrapping_add(2);
            }
            r14_grp = r14_grp.wrapping_add(2);
        }
    }

    for i in 0..count_groups {
        let rbx = local_58.wrapping_add(i * 16);
        let r8d = rd::<u32>(rbx + 0x08) as u64;
        let mut ptr_begin = rd::<usize>(rbx);
        let mut ptr_end = ptr_begin.wrapping_add((r8d as usize).wrapping_mul(2));
        exe::call_va!(
            0x1_411e_ef20,
            (
                (&mut ptr_begin as *mut usize) as usize,
                (&mut ptr_end as *mut usize) as usize,
                r8d,
                0u32
            )
        );
    }

    let p2_ptr = rd::<usize>(param_2 + 0x08);
    let req_cap = param_8
        .wrapping_mul(rd::<u64>(param_7 + 0x38))
        .wrapping_add(rd::<u64>(param_7 + 0x10));
    exe::call_va!(0x1_40ff_f1e0, (param_7, req_cap));

    for i in 0..count_groups {
        let r15_vec = local_58.wrapping_add(i * 16);
        let esi = rd::<u32>(r15_vec + 0x08) as usize;
        let mut rbx = rd::<usize>(r15_vec);
        let grp_id = rd::<u16>(p2_ptr.wrapping_add(i * 2)) as usize;
        let words_per_row = rd::<usize>(param_7 + 0x40);
        let row_bytes = words_per_row.wrapping_mul(8);
        let r14_dst = rd::<usize>(param_7).wrapping_add(grp_id.wrapping_mul(row_bytes));
        exe::call_va!(0x1_4183_acbc, (r14_dst, rd::<usize>(param_7 + 0x18), row_bytes));
        let r8_end = rbx.wrapping_add(esi.wrapping_mul(2));
        while rbx != r8_end {
            let bit = rd::<u16>(rbx) as usize;
            let word_idx = bit >> 6;
            let bit_pos = (bit & 0x3f) as u32;
            let word_addr = r14_dst.wrapping_add(word_idx * 8);
            wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_pos));
            rbx = rbx.wrapping_add(2);
        }
    }

    for i in 0..count_groups {
        let rbx = local_58.wrapping_add(i * 16);
        let ptr = rd::<usize>(rbx);
        if ptr != 0 {
            exe::call_va!(0x1_4241_c190, (ptr));
        }
        wr::<u64>(rbx, 0);
        wr::<u64>(rbx + 0x08, 0);
    }
    if local_58 != 0 {
        exe::call_va!(0x1_4241_c190, (local_58));
    }
}

#[inline(always)]
unsafe fn upper_bound_u32(sorted_ptr: usize, count: u64, count_u32: u32, key: u32) -> usize {
    let mut rem = count;
    let mut base: usize = 0;
    if count_u32 != 0 {
        while rem != 0 {
            let half = rem >> 1;
            let mid = base.wrapping_add(half as usize);
            if key >= rd::<u32>(sorted_ptr.wrapping_add(mid * 4)) {
                base = base.wrapping_add((rem - half) as usize);
            }
            rem = half;
        }
    }
    base
}

/// `unknown_1420c4950`
/// Original: `0x1420c4950` (2299 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420c4950(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
) -> usize {
    let r14 = rd::<u64>(param_3 + 0x40);
    let r15 = rd::<u64>(param_3 + 0x100);
    let r13 = rd::<u64>(param_3 + 0x60);
    let total_u64 = rd::<u64>(param_3 + 0x20)
        .wrapping_add(r13)
        .wrapping_add(r15)
        .wrapping_add(r14);
    let total_u32 = total_u64 as u32;

    let rsi_buf = exe::call_va!(
        0x1_411f_11e0,
        (param_2, total_u64.wrapping_mul(4), 4u32) -> usize
    );
    let mut vec_50: [u64; 3] = [
        param_2 as u64,
        rsi_buf as u64,
        (total_u32 as u64) | ((total_u32 as u64) << 32),
    ];
    let vec_50_ptr = vec_50.as_mut_ptr() as usize;

    let r12 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let r12_0 = rd::<u64>(r12);

    let rbx_sched = rd::<usize>(r12 + 0x10);
    let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let mut handle_28 = [0u64; 2];
    let h28_ptr = handle_28.as_mut_ptr() as usize;
    let pool40 = rd::<usize>(rbx_sched + wid * 0x40 + 0x48).wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool40, h28_ptr));

    let rdi_jc = rd::<usize>(h28_ptr);
    crate::simd_ops::atomic_xchg_u32(rdi_jc as u64, 1);
    wr::<u64>(rdi_jc + 0x08, 0);
    wr::<usize>(rdi_jc + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(rdi_jc + 0x18, exe::ptr::<u8>(0x1_433d_dbe0) as usize);
    wr::<u64>(rdi_jc + 0x20, 0x36);
    let (ok_init, _) = crate::simd_ops::atomic_cmpxchg_u8((rdi_jc + 0x04) as u64, 1, 3);
    if ok_init {
        let _ = crate::simd_ops::atomic_add_u32(rd::<usize>(h28_ptr) as u64, 0xffff);
    }
    wr::<u8>(h28_ptr + 0x0a, 1);

    let mut span_30 = rd::<[u64; 3]>(param_3 + 0x30);
    exe::call_va!(0x1_420c_5830, (rsi_buf, span_30.as_mut_ptr() as usize, h28_ptr));

    let mut rbx_off = rsi_buf.wrapping_add((r14 as usize).wrapping_mul(4));
    let mut span_f0 = rd::<[u64; 3]>(param_3 + 0xf0);
    exe::call_va!(0x1_420c_5830, (rbx_off, span_f0.as_mut_ptr() as usize, h28_ptr));

    rbx_off = rbx_off.wrapping_add((r15 as usize).wrapping_mul(4));
    let mut span_50 = rd::<[u64; 3]>(param_3 + 0x50);
    exe::call_va!(0x1_420c_5830, (rbx_off, span_50.as_mut_ptr() as usize, h28_ptr));

    let rcx_off = rbx_off.wrapping_add((r13 as usize).wrapping_mul(4));
    let mut span_10 = rd::<[u64; 3]>(param_3 + 0x10);
    exe::call_va!(0x1_420c_5830, (rcx_off, span_10.as_mut_ptr() as usize, h28_ptr));

    if rd::<u8>(h28_ptr + 0x0b) == 0 {
        if rd::<u8>(rdi_jc + 0x04) == 3 {
            let rcx_jc = rd::<usize>(h28_ptr);
            loop {
                let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((rdi_jc + 0x04) as u64, 3, 4);
                if ok {
                    let _ = crate::simd_ops::atomic_add_u32(
                        rd::<usize>(h28_ptr) as u64,
                        0xffff_0001,
                    );
                }
                if rd::<u8>(rcx_jc + 0x04) != 3 {
                    break;
                }
            }
        }
        let rbx_jc = rd::<usize>(h28_ptr);
        while (rd::<u16>(rbx_jc + 0x06) >> 3) == rd::<u16>(h28_ptr + 0x08)
            && rd::<u32>(rd::<usize>(h28_ptr)) != 0
        {
            if rd::<u32>(rd::<usize>(h28_ptr)) == 1 {
                let sched = rd::<usize>(r12 + 0x10);
                let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((rbx_jc + 0x04) as u64, 4, 2);
                if ok {
                    let _ = crate::simd_ops::atomic_add_u32(rd::<usize>(h28_ptr) as u64, u32::MAX);
                    let rdx_parent = rd::<usize>(rbx_jc + 0x08);
                    let _ = crate::simd_ops::atomic_cmpxchg_u8((rbx_jc + 0x04) as u64, 2, 0);
                    let w6 = rd::<u16>(rbx_jc + 0x06);
                    wr::<u16>(rbx_jc + 0x06, ((w6 & 0xfff8).wrapping_add(8)) | (w6 & 7));
                    if rdx_parent != 0 {
                        let old = crate::simd_ops::atomic_xadd_u32(rdx_parent as u64, u32::MAX);
                        if old == 2 {
                            exe::call_va!(0x1_4248_6460, (sched, rdx_parent));
                            let sem = rd::<usize>(rd::<usize>(sched) + 0x1008);
                            exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
                        }
                    }
                }
            } else if exe::call_va!(0x1_4248_54f0, (r12) -> u64) == 0 {
                exe::call_iat!(0x1_4264_5868, ());
            }
        }
    }

    let len_before = rd::<u32>(vec_50_ptr + 0x10) as i32 as i64;
    let buf_before = rd::<usize>(vec_50_ptr + 0x08);
    exe::call_va!(0x1_420a_f720, (buf_before, len_before));
    exe::call_va!(0x1_411f_1460, (vec_50_ptr));

    let edi = rd::<u32>(vec_50_ptr + 0x10);
    let rdi_cnt = edi as usize;
    let r13_out = exe::call_va!(
        0x1_411f_11e0,
        (param_2, (rdi_cnt as u64) << 4, 8u32) -> usize
    );

    if rdi_cnt != 0 {
        let r10_loc = rd::<usize>(param_4 + 0x08);
        let r11_lut = rd::<usize>(param_5 + 0x20);
        let rbx_sorted = rd::<usize>(vec_50_ptr + 0x08);
        for i in 0..rdi_cnt {
            let ent_idx = rd::<u32>(rbx_sorted.wrapping_add(i * 4)) as usize;
            let grp_idx = rd::<u16>(r10_loc.wrapping_add(ent_idx * 8)) as usize;
            let val = rd::<u64>(r11_lut.wrapping_add(grp_idx * 8));
            let dst = r13_out.wrapping_add(i * 16);
            wr::<u64>(dst, val);
            wr::<u64>(dst + 0x08, val);
        }
    }

    exe::call_va!(0x1_4248_5780, (r12, param_7.wrapping_add(0x18)));

    let r15_rem = rd::<u32>(param_7 + 0x10) as u64;
    let rem_base = rd::<usize>(param_7 + 0x08);

    let rbx_sched2 = rd::<usize>(r12 + 0x10);
    let wid2 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let mut handle_40 = [0u64; 2];
    let h40_ptr = handle_40.as_mut_ptr() as usize;
    let pool40_2 = rd::<usize>(rbx_sched2 + wid2 * 0x40 + 0x48).wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool40_2, h40_ptr));

    let ptr_40 = rd::<usize>(h40_ptr);
    crate::simd_ops::atomic_xchg_u32(ptr_40 as u64, 1);
    wr::<u64>(ptr_40 + 0x08, 0);
    wr::<usize>(ptr_40 + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(ptr_40 + 0x18, exe::ptr::<u8>(0x1_433d_dba8) as usize);
    wr::<u64>(ptr_40 + 0x20, 0x35);
    let (ok_init2, _) = crate::simd_ops::atomic_cmpxchg_u8((ptr_40 + 0x04) as u64, 1, 3);
    if ok_init2 {
        let _ = crate::simd_ops::atomic_add_u32(rd::<usize>(h40_ptr) as u64, 0xffff);
    }
    wr::<u8>(h40_ptr + 0x0a, 1);

    let r14_chunk = r15_rem / r12_0;
    let mut rsi_idx: u64 = 0;
    let mut rdi_rem: i64 = r15_rem as i64;

    if (r15_rem as i64) > (r14_chunk as i64) {
        let sorted_len_u64 = rd::<u32>(vec_50_ptr + 0x10) as u64;
        let sorted_buf_ptr = rd::<usize>(vec_50_ptr + 0x08);
        let mut handle_78 = [0u64; 2];
        let h78_ptr = handle_78.as_mut_ptr() as usize;

        loop {
            let mut rdx_cnt = r14_chunk;
            let r8_start = rsi_idx;
            rsi_idx = rsi_idx.wrapping_add(r14_chunk);
            rdi_rem -= r14_chunk as i64;
            let mut rax_p = rem_base.wrapping_add((rsi_idx as usize) * 8 + 4);
            let ecx_key = rd::<u32>(rax_p);
            while rdi_rem > 0 && rd::<u32>(rax_p) == ecx_key {
                rsi_idx = rsi_idx.wrapping_add(1);
                rax_p = rax_p.wrapping_add(8);
                rdx_cnt = rdx_cnt.wrapping_add(1);
                rdi_rem -= 1;
            }
            let chunk_ptr = rem_base.wrapping_add((r8_start as usize) * 8);
            if rdx_cnt == u64::MAX {
                rdx_cnt = r15_rem.wrapping_sub(r8_start);
            }

            let sched = rd::<usize>(r12 + 0x10);
            let w = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool80 = rd::<usize>(sched + w * 0x40 + 0x48).wrapping_add(0x80);
            exe::call_va!(0x1_4248_5aa0, (pool80, h78_ptr));

            let job = rd::<usize>(h78_ptr);
            crate::simd_ops::atomic_xchg_u32(job as u64, 1);
            wr::<u64>(job + 0x08, 0);
            wr::<usize>(job + 0x10, exe::ptr::<u8>(0x1_433d_de30) as usize);
            wr::<u64>(job + 0x18, edi as u64);
            wr::<usize>(job + 0x20, r13_out);
            wr::<u64>(job + 0x28, sorted_len_u64);
            wr::<usize>(job + 0x30, sorted_buf_ptr);
            wr::<usize>(job + 0x38, param_6);
            wr::<u64>(job + 0x40, rdx_cnt);
            wr::<usize>(job + 0x48, chunk_ptr);

            let jc = rd::<usize>(h40_ptr);
            let _ = crate::simd_ops::atomic_add_u32(jc as u64, 1);
            wr::<usize>(job + 0x08, jc);

            let job_after = rd::<usize>(h78_ptr);
            if rd::<u32>(job_after) == 1 {
                let sched2 = rd::<usize>(r12 + 0x10);
                exe::call_va!(0x1_4248_6460, (sched2, job_after));
                let sem = rd::<usize>(rd::<usize>(sched2) + 0x1008);
                exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
            }

            if rdi_rem <= (r14_chunk as i64) {
                break;
            }
        }
    }

    let mut rax_cur = rem_base.wrapping_add((rsi_idx as usize) * 8);
    let r15_left = r15_rem.wrapping_sub(rsi_idx);
    let rdi_tail = if rdi_rem == -1 {
        r15_left
    } else {
        rdi_rem as u64
    };
    let rsi_end = rax_cur.wrapping_add((rdi_tail as usize) * 8);
    let r15d = rd::<u32>(vec_50_ptr + 0x10);
    let r15_u64 = r15d as u64;
    let rbx_sorted = rd::<usize>(vec_50_ptr + 0x08);

    while rax_cur != rsi_end {
        let pair = rd::<u64>(rax_cur);
        let key = (pair >> 32) as u32;
        let ub = upper_bound_u32(rbx_sorted, r15_u64, r15d, key);
        let idx_m1 = ub.wrapping_sub(1);
        let comp_id = (pair as u16) as usize;
        let tbl_10 = rd::<usize>(param_6 + 0x10);
        let hash_val = rd::<u64>(tbl_10.wrapping_add(comp_id * 8));
        let slot_addr = r13_out.wrapping_add(idx_m1.wrapping_mul(16)).wrapping_add(0x08);
        wr::<u64>(slot_addr, rd::<u64>(slot_addr) ^ hash_val);
        rax_cur = rax_cur.wrapping_add(8);
    }

    exe::call_va!(0x1_4248_5680, (r12, h40_ptr));

    let mut r10_node = rd::<usize>(param_3 + 0x30);
    let mut r9_idx: u64 = 0;
    let rdi_end_node = rd::<usize>(param_3 + 0x38);
    let rsi_end_idx = if rdi_end_node != 0 {
        rd::<u64>(rdi_end_node + 0x08)
    } else {
        0
    };
    while r10_node != rdi_end_node || r9_idx != rsi_end_idx {
        let key = rd::<u32>(r10_node.wrapping_add((r9_idx as usize) * 4 + 0x10));
        let ub = upper_bound_u32(rbx_sorted, r15_u64, r15d, key);
        let slot_addr = r13_out.wrapping_add(ub.wrapping_mul(16)).wrapping_sub(8);
        wr::<u64>(slot_addr, 0);
        r9_idx = r9_idx.wrapping_add(1);
        if r9_idx == rd::<u64>(r10_node + 0x08) {
            let next = rd::<usize>(r10_node);
            if next != 0 {
                r10_node = next;
                r9_idx = 0;
            }
        }
    }

    let mut rsi_node = rd::<usize>(param_3 + 0x68);
    let mut r9_comp_idx: u64 = 0;
    let mut r11_node = rd::<usize>(param_3 + 0x50);
    let mut r10_ent_idx: u64 = 0;
    let r14_end_node = rd::<usize>(param_3 + 0x58);
    let r12_end_idx = if r14_end_node != 0 {
        rd::<u64>(r14_end_node + 0x08)
    } else {
        0
    };
    while r11_node != r14_end_node || r10_ent_idx != r12_end_idx {
        let key = rd::<u32>(r11_node.wrapping_add((r10_ent_idx as usize) * 4 + 0x10));
        let ub = upper_bound_u32(rbx_sorted, r15_u64, r15d, key);
        let cur_rsi_node = rsi_node;
        let cur_r9_idx = r9_comp_idx;
        r9_comp_idx = r9_comp_idx.wrapping_add(1);
        if r9_comp_idx == rd::<u64>(rsi_node + 0x08) {
            let next_rsi = rd::<usize>(rsi_node);
            if next_rsi != 0 {
                rsi_node = next_rsi;
                r9_comp_idx = 0;
            }
        }
        let comp_id = rd::<u16>(cur_rsi_node.wrapping_add((cur_r9_idx as usize) * 2 + 0x10)) as usize;
        let tbl_10 = rd::<usize>(param_6 + 0x10);
        let hash_val = rd::<u64>(tbl_10.wrapping_add(comp_id * 8));
        let slot_addr = r13_out
            .wrapping_add(ub.wrapping_sub(1).wrapping_mul(16))
            .wrapping_add(0x08);
        wr::<u64>(slot_addr, rd::<u64>(slot_addr) ^ hash_val);

        r10_ent_idx = r10_ent_idx.wrapping_add(1);
        if r10_ent_idx == rd::<u64>(r11_node + 0x08) {
            let next_r11 = rd::<usize>(r11_node);
            if next_r11 != 0 {
                r11_node = next_r11;
                r10_ent_idx = 0;
            }
        }
    }

    let mut ctx_80: [u64; 4] = [r15_u64, rbx_sorted as u64, edi as u64, r13_out as u64];
    exe::call_va!(
        0x1_411f_33a0,
        (
            rd::<u64>(param_3 + 0x10),
            0u64,
            0u64,
            ctx_80.as_mut_ptr() as usize
        )
    );

    wr::<usize>(param_1, param_2);
    wr::<usize>(param_1 + 0x08, r13_out);
    wr::<u32>(param_1 + 0x10, edi);
    wr::<u32>(param_1 + 0x14, edi);
    wr::<u64>(param_1 + 0x18, rd::<u64>(vec_50_ptr));
    wr::<usize>(param_1 + 0x20, rbx_sorted);
    wr::<u32>(param_1 + 0x28, r15d);
    wr::<u32>(param_1 + 0x2c, rd::<u32>(vec_50_ptr + 0x14));
    param_1
}

/// `unknown_1420d4ca0`
/// Original: `0x1420d4ca0` (1051 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1420d4ca0(param_1: usize) {
    let r9 = rd::<usize>(param_1 + 0x40);
    let idx = ((rd::<u64>(r9 + 0x1f0).wrapping_add(1)) % 3) as usize;
    let base = r9.wrapping_add(idx * 0x7c);

    for off in 0x06usize..=0x13usize {
        wr::<u8>(base + off, 0);
    }
    wr::<u8>(base + 0x1b, 0);
    wr::<u8>(base + 0x20, 0);
    wr::<u16>(base + 0x27, 0x8080);
    wr::<u16>(base + 0x29, 0x8080);
    wr::<u8>(base + 0x25, 0);
    wr::<u8>(base + 0x26, 0);
    wr::<u64>(base + 0x3c, 0);
    wr::<u32>(base + 0x44, 0);
    wr::<u64>(base + 0x48, 0);
    wr::<u32>(base + 0x50, 0);
    wr::<u64>(base + 0x2c, 0x3f80_0000);
    wr::<u64>(base + 0x34, 0);

    for i in 0..4usize {
        let entry = base.wrapping_add(0x54 + i * 10);
        wr::<u8>(entry, 0);
        wr::<u32>(entry + 0x02, 0);
    }

    wr::<u32>(r9 + 0x200, 0);
}

/// `unknown_1420d73d0`
/// Original: `0x1420d73d0` (331 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1420d73d0(param_1: usize, param_2: u8) {
    let mut rbx = param_1.wrapping_add(0x55c);
    for _ in 0..16usize {
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rbx - 0x04) != tid {
            exe::call_iat!(0x1_4264_55a8, (rbx - 0x0c));
            wr::<u32>(rbx - 0x04, tid);
        }
        let ecx = rd::<u32>(rbx);
        wr::<u32>(rbx, ecx.wrapping_add(1));
        let r8 = rd::<usize>(rbx + 0x2c);
        let new_cnt = if r8 == 0 {
            wr::<u32>(rbx, ecx);
            ecx
        } else {
            let (rcx_dst, rdx_src) = if param_2 != 0 {
                let idx_src = (rd::<u64>(r8 + 0x1f0) % 3) as usize;
                wr::<u64>(r8 + 0x1f8, rd::<u64>(r8 + 0x1f8).wrapping_add(1));
                (r8.wrapping_add(0x174), r8.wrapping_add(idx_src * 0x7c))
            } else {
                if rd::<u8>(param_1 + 0x72) != 0 {
                    exe::call_va!(0x1_420d_4180, (ecx, rbx - 0x14));
                }
                let r9 = rd::<usize>(rbx + 0x2c);
                let next_1f0 = rd::<u64>(r9 + 0x1f0).wrapping_add(1);
                wr::<u64>(r9 + 0x1f0, next_1f0);
                let idx_dst = (next_1f0.wrapping_add(1) % 3) as usize;
                let idx_src = (next_1f0 % 3) as usize;
                (
                    r9.wrapping_add(idx_dst * 0x7c),
                    r9.wrapping_add(idx_src * 0x7c),
                )
            };
            exe::call_va!(0x1_411f_3f00, (rcx_dst, rdx_src));
            let after = rd::<u32>(rbx).wrapping_sub(1);
            wr::<u32>(rbx, after);
            after
        };
        if new_cnt == 0 {
            crate::simd_ops::atomic_xchg_u32((rbx - 0x04) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (rbx - 0x0c));
        }
        rbx = rbx.wrapping_add(0x330);
    }
}

/// `unknown_1420db7c0`
/// Original: `0x1420db7c0` (525 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_1420db7c0(
    param_1: usize,
    param_2: u16,
    param_3: usize,
) -> u8 {
    let pred_fn = rd::<usize>(param_1);
    if pred_fn != 0 {
        let p3_0 = rd::<usize>(param_3);
        let arg2 = rd::<u64>(p3_0 + 0x68);
        if exe::call_va!(pred_fn as u64, (param_2 as u32, arg2) -> u8) == 0 {
            return 0;
        }
    }
    if rd::<u8>(param_1 + 0x0c) == 0 {
        return 1;
    }

    exe::call_va!(0x1_420d_ae80, (param_1, rd::<u64>(param_3 + 0x08)));
    let kind = rd::<u8>(param_1 + 0x0c);
    if kind == 4 {
        return exe::call_va!(
            0x1_420e_02c0,
            (
                param_3,
                rd::<u8>(param_1 + 0x1c) as u32,
                rd::<u16>(param_1 + 0x18) as u32
            ) -> u8
        );
    }
    if kind.wrapping_sub(5) <= 1 {
        return exe::call_va!(0x1_420d_b190, (param_1, param_3, param_2 as u32) -> u8);
    }
    if kind == 7 {
        return exe::call_va!(
            0x1_420d_aff0,
            (
                7u32,
                param_3,
                param_2 as u32,
                rd::<u16>(param_1 + 0x18) as u32,
                rd::<u8>(param_1 + 0x1c) as u32
            ) -> u8
        );
    }
    if kind == 8 {
        let rbx_lock = rd::<usize>(param_3 + 0x08).wrapping_add(0x5ad0);
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rbx_lock + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (rbx_lock));
            wr::<u32>(rbx_lock + 0x08, tid);
        }
        wr::<u32>(rbx_lock + 0x0c, rd::<u32>(rbx_lock + 0x0c).wrapping_add(1));

        let user_idx = exe::call_va!(0x1_420d_f900, (rd::<usize>(param_3)) -> u8);
        if user_idx == 0xff {
            let new_cnt = rd::<u32>(rbx_lock + 0x0c).wrapping_sub(1);
            wr::<u32>(rbx_lock + 0x0c, new_cnt);
            if new_cnt == 0 {
                crate::simd_ops::atomic_xchg_u32((rbx_lock + 0x08) as u64, u32::MAX);
                exe::call_iat!(0x1_4264_5598, (rbx_lock));
            }
            return 0;
        }

        let p3_08 = rd::<usize>(param_3 + 0x08);
        let mut cur = rd::<usize>(p3_08 + 0x58d0);
        let count = rd::<u32>(p3_08 + 0x58d8) as usize;
        let end = cur.wrapping_add(count * 0x22);
        let found = loop {
            if cur == end {
                break 0usize;
            }
            if rd::<u16>(cur) == param_2 {
                break cur;
            }
            cur = cur.wrapping_add(0x22);
        };

        if (user_idx as u64) >= 0x10 {
            let s_loc = exe::ptr::<u8>(0x1_433d_efa0) as usize;
            let s_msg = exe::ptr::<u8>(0x1_433d_f690) as usize;
            exe::call_va!(0x1_4249_9280, (s_loc, s_msg));
            wr::<u32>(0, 0);
        }

        let val = rd::<u8>(found.wrapping_add(2).wrapping_add(user_idx as usize));
        let new_cnt = rd::<u32>(rbx_lock + 0x0c).wrapping_sub(1);
        wr::<u32>(rbx_lock + 0x0c, new_cnt);
        if new_cnt == 0 {
            crate::simd_ops::atomic_xchg_u32((rbx_lock + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (rbx_lock));
        }
        return val;
    }

    let child0 = rd::<u16>(param_1 + 0x18);
    if kind == 3 {
        return exe::call_va!(0x1_420d_8810, (param_3, child0 as u32) -> u8) ^ 1;
    }
    if kind == 9 {
        return exe::call_va!(0x1_420d_8810, (param_3, child0 as u32) -> u8);
    }

    let r0 = exe::call_va!(0x1_420d_8810, (param_3, child0 as u32) -> u8);
    let child1 = rd::<u16>(param_1 + 0x1a);
    let r1 = exe::call_va!(0x1_420d_8810, (param_3, child1 as u32) -> u8);
    if rd::<u8>(param_1 + 0x0c) == 1 {
        ((r0 != 0) && (r1 != 0)) as u8
    } else {
        ((r0 != 0) || (r1 != 0)) as u8
    }
}

/// `unknown_1420e02c0`
/// Original: `0x1420e02c0` (184 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420e02c0(
    param_1: usize,
    param_2: u8,
    param_3: u16,
) -> u8 {
    match param_2.wrapping_sub(7) {
        0 => exe::call_va!(0x1_420d_98f0, (param_1, param_3 as u32) -> u8),
        1 => {
            if exe::call_va!(0x1_420d_95d0, (param_1, param_3 as u32) -> u8) != 0
                && exe::call_va!(0x1_420d_98f0, (param_1, param_3 as u32) -> u8) != 0
            {
                1
            } else {
                0
            }
        }
        2 => {
            if exe::call_va!(0x1_420d_95d0, (param_1, param_3 as u32) -> u8) != 0
                && exe::call_va!(0x1_420d_98f0, (param_1, param_3 as u32) -> u8) == 0
            {
                1
            } else {
                0
            }
        }
        3..=6 => 1,
        _ => 0,
    }
}

/// `unknown_1420e3d70`
/// Original: `0x1420e3d70` (370 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1420e3d70(param_1: usize) {
    if param_1 == 0 {
        return;
    }

    let mut v30: u64 = rd::<u64>(param_1 + 0x10);
    let mut v38: u64 = 0x7fff_ffff_ffff_ffff;
    let rbx = exe::call_va!(0x1_4049_d170, ((&mut v38 as *mut u64) as usize) -> u64);
    let rax = exe::call_va!(0x1_4049_d170, ((&mut v30 as *mut u64) as usize) -> u64);
    if rax == rbx {
        let p = exe::call_va!(0x1_401f_2f00, ((&mut v38 as *mut u64) as usize) -> usize);
        let val = rd::<u64>(p);
        crate::simd_ops::atomic_xchg_u64((param_1 + 0x4fc8) as u64, val);
    } else {
        let prev = rd::<u64>(param_1 + 0x4fc8);
        let dt = exe::call_va!(0x1_4049_d170, (param_1 + 0x10) -> u64);
        let next = dt.wrapping_mul(1000).wrapping_add(prev);
        crate::simd_ops::atomic_xchg_u64((param_1 + 0x4fc8) as u64, next);
    }

    exe::call_va!(0x1_420c_f110, (param_1, 0u32));
    exe::call_va!(0x1_420c_da40, (param_1, 0u32));
    exe::call_va!(0x1_420d_2440, (param_1, 0u32));
    exe::call_va!(0x1_420d_e930, (param_1));
    exe::call_va!(0x1_420e_4e70, (param_1));
    exe::call_va!(0x1_420d_a540, (param_1));

    let mut cur = rd::<usize>(param_1 + 0x4fe0);
    let count = rd::<u32>(param_1 + 0x4fe8) as usize;
    let end = cur.wrapping_add(count * 0x78);
    let fn_e0960 = exe::ptr::<u8>(0x1_420e_0960) as usize;

    while cur != end {
        let p0 = rd::<usize>(cur);
        if p0 != 0 {
            let fn_70 = rd::<usize>(cur + 0x70);
            if rd::<u64>(cur + 0x08) == rd::<u64>(p0 + 0x4fc0) || fn_70 == fn_e0960 {
                let pred_ok = if fn_70 != 0 {
                    exe::call_va!(fn_70 as u64, (rd::<u64>(cur + 0x68)) -> u8) != 0
                } else {
                    true
                };
                if pred_ok {
                    let active = if rd::<u64>(cur + 0x48) != 0 || rd::<u64>(cur + 0x50) != 0 {
                        true
                    } else {
                        let p60 = rd::<usize>(cur + 0x60);
                        p60 != 0 && rd::<u8>(p60 + 0x18) != 0
                    };
                    if active {
                        exe::call_va!(0x1_411f_5380, (rd::<usize>(cur), rd::<usize>(cur + 0x60)));
                        exe::call_va!(0x1_411f_5520, (rd::<usize>(cur), rd::<usize>(cur + 0x48)));
                        exe::call_va!(0x1_411f_5690, (rd::<usize>(cur), rd::<usize>(cur + 0x50)));
                    }
                }
            }
        }
        cur = cur.wrapping_add(0x78);
    }
}

/// `unknown_1421c0320`
/// Original: `0x1421c0320` (663 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1421c0320(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u32,
) -> usize {
    let idx0 = param_4 as usize;
    let init_tf = rd::<[u128; 2]>(param_2.wrapping_add(idx0 << 5));
    let mut ax = rd::<u16>(param_3.wrapping_add(idx0 * 2));
    wr::<[u128; 2]>(param_1, init_tf);
    if ax == 0xffff {
        return param_1;
    }

    let xmm11 = rd::<u128>(exe::ptr::<u8>(0x1_4342_7ab0) as usize);
    let c_8460 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8460) as usize);
    let c_8120 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize);

    loop {
        let cur_tf = rd::<[u128; 2]>(param_1);
        let xmm4_init = cur_tf[0];
        let xmm9 = cur_tf[1];
        let ecx = ax as usize;
        let xmm0_a = vpermilps(xmm9, 0xc9);
        let xmm1_a = vpermilps(xmm9, 0xd2);
        let parent_off = ecx << 5;
        let xmm10_p = rd::<u128>(param_2.wrapping_add(parent_off));
        let xmm8 = vpermilps(xmm10_p, 0xc9);
        let xmm2_a = vmulps(xmm1_a, xmm8);
        let xmm7 = vpermilps(xmm10_p, 0xd2);
        let xmm3_a = vmulps(xmm0_a, xmm7);
        let xmm0_b = vsubps(xmm2_a, xmm3_a);
        let xmm6 = vmulps(xmm0_b, xmm11);

        let xmm3_q = vpermilps(xmm4_init, 0x11);
        let xmm1_q = vmulps(xmm3_q, xmm10_p);
        let xmm2_q = vpermilps(xmm4_init, 0xbb);
        let xmm5_q = vpermilps(xmm10_p, 0x1b);
        let xmm0_q = vmulps(xmm2_q, xmm5_q);
        let xmm4_q = vhsubps(xmm1_q, xmm0_q);
        let xmm0_q2 = vmulps(xmm3_q, xmm5_q);
        let xmm2_q2 = vmulps(xmm2_q, xmm10_p);
        let xmm3_q2 = vhaddps(xmm2_q2, xmm0_q2);
        let xmm0_sh = vshufps(xmm4_q, xmm3_q2, 0xb1);
        let xmm1_bl = vblendps(xmm3_q2, xmm4_q, 0x0c);
        let xmm1_as = vaddsubps(xmm1_bl, xmm0_sh);

        let xmm0_c = vpermilps(xmm6, 0xc9);
        let xmm3_c = vmulps(xmm0_c, xmm7);
        let xmm12 = vpermilps(xmm1_as, 0x9c);
        let xmm0_w = vpermilps(xmm10_p, 0xff);
        let xmm1_c = vpermilps(xmm6, 0xd2);
        let xmm2_c = vmulps(xmm1_c, xmm8);
        let xmm1_w = vmulps(xmm0_w, xmm6);
        let xmm4_c = vsubps(xmm2_c, xmm3_c);
        let xmm2_s = vaddps(xmm1_w, xmm9);
        let xmm3_s = vaddps(xmm4_c, xmm2_s);
        let xmm10_pos = vaddps(xmm3_s, rd::<u128>(param_2.wrapping_add(parent_off + 0x10)));

        ax = rd::<u16>(param_3.wrapping_add(ecx * 2));
        wr::<[u128; 2]>(param_1, [xmm12, xmm10_pos]);
        if ax == 0xffff {
            break;
        }

        let xmm1_a2 = vpermilps(xmm10_pos, 0xc9);
        let ecx2 = ax as usize;
        let parent_off2 = ecx2 << 5;
        let xmm0_a2 = vpermilps(xmm10_pos, 0xd2);
        let xmm9_p = rd::<u128>(param_2.wrapping_add(parent_off2));
        let xmm5_2 = vpermilps(xmm9_p, 0x1b);
        let xmm8_2 = vpermilps(xmm9_p, 0xd2);
        let xmm2_a2 = vmulps(xmm1_a2, xmm8_2);
        let xmm7_2 = vpermilps(xmm9_p, 0xc9);
        let xmm3_a2 = vmulps(xmm0_a2, xmm7_2);
        let xmm0_b2 = vsubps(xmm3_a2, xmm2_a2);
        let xmm6_2 = vmulps(xmm0_b2, xmm11);

        let xmm3_q3 = vpermilps(xmm12, 0x11);
        let xmm1_q3 = vmulps(xmm3_q3, xmm9_p);
        let xmm2_q3 = vpermilps(xmm12, 0xbb);
        let xmm0_q3 = vmulps(xmm2_q3, xmm5_2);
        let xmm4_q3 = vhsubps(xmm1_q3, xmm0_q3);
        let xmm0_q4 = vmulps(xmm3_q3, xmm5_2);
        let xmm2_q4 = vmulps(xmm2_q3, xmm9_p);
        let xmm3_q4 = vhaddps(xmm2_q4, xmm0_q4);
        let xmm1_bl2 = vblendps(xmm3_q4, xmm4_q3, 0x0c);
        let xmm0_sh2 = vshufps(xmm4_q3, xmm3_q4, 0xb1);
        let xmm1_as2 = vaddsubps(xmm1_bl2, xmm0_sh2);
        let xmm2_rot = vpermilps(xmm1_as2, 0x9c);

        let xmm1_c2 = vpermilps(xmm6_2, 0xc9);
        let xmm2_c2 = vmulps(xmm1_c2, xmm8_2);
        let xmm0_c2 = vpermilps(xmm6_2, 0xd2);
        let xmm3_c2 = vmulps(xmm0_c2, xmm7_2);
        let xmm4_c2 = vsubps(xmm3_c2, xmm2_c2);
        let xmm0_w2 = vpermilps(xmm9_p, 0xff);
        let xmm1_w2 = vmulps(xmm0_w2, xmm6_2);
        let xmm2_s2 = vaddps(xmm1_w2, xmm10_pos);
        let xmm3_s2 = vaddps(xmm4_c2, xmm2_s2);
        let xmm0_pos = vaddps(
            xmm3_s2,
            rd::<u128>(param_2.wrapping_add(parent_off2 + 0x10)),
        );

        ax = rd::<u16>(param_3.wrapping_add(ecx2 * 2));
        wr::<[u128; 2]>(param_1, [xmm2_rot, xmm0_pos]);

        let xmm5_norm = rd::<u128>(param_1);
        let xmm0_dp = vdpps(xmm5_norm, xmm5_norm, 0xff);
        let xmm4_rsq = vrsqrtps_128(xmm0_dp);
        let xmm0_m1 = vmulps(xmm4_rsq, xmm0_dp);
        let xmm2_m2 = vmulps(xmm0_m1, xmm4_rsq);
        let xmm0_half = vmulps(xmm4_rsq, c_8120);
        let xmm3_sub = vsubps(c_8460, xmm2_m2);
        let xmm1_sc = vmulps(xmm3_sub, xmm0_half);
        let xmm2_out = vmulps(xmm1_sc, xmm5_norm);
        wr::<u128>(param_1, xmm2_out);

        if ax == 0xffff {
            break;
        }
    }

    param_1
}

#[inline(always)]
fn u64_to_f32_msvc(v: u64) -> f32 {
    if (v as i64) >= 0 {
        (v as i64) as f32
    } else {
        let half = (((v >> 1) | (v & 1)) as i64) as f32;
        half + half
    }
}

#[inline(always)]
fn f32_to_u64_msvc(mut v: f32, two_pow_63: f32) -> u64 {
    let mut add: u64 = 0;
    if v >= two_pow_63 {
        v -= two_pow_63;
        if v < two_pow_63 {
            add = 0x8000_0000_0000_0000;
        }
    }
    crate::simd_ops::cvt_f32_i64(v).wrapping_add(add)
}

/// `unknown_1422075a0`
/// Original: `0x1422075a0` (625 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_1422075a0(param_1: usize) {
    let ptr_698 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
    let raw_budget = rd::<u64>(ptr_698);
    let xmm1 = u64_to_f32_msvc(raw_budget);

    let xmm4 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let xmm2 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fc8) as usize);
    let c_c78 = rd::<f32>(exe::ptr::<f32>(0x1_4386_7c78) as usize);
    let c_c98 = rd::<f32>(exe::ptr::<f32>(0x1_4386_7c98) as usize);

    let rbp = f32_to_u64_msvc((xmm4 - c_c78) * xmm1, xmm2);
    let rdi = f32_to_u64_msvc((xmm4 - c_c98) * xmm1, xmm2);

    let xmm7 = rd::<f32>(param_1 + 0x10);
    let cur_08 = rd::<u64>(param_1 + 0x08);
    let step_cd8 = rd::<f32>(exe::ptr::<f32>(0x1_4386_7cd8) as usize);
    let xmm0 = if cur_08 < rbp {
        xmm7 - step_cd8
    } else if cur_08 > rdi {
        xmm7 + step_cd8
    } else {
        xmm7
    };

    let max0 = if xmm0 > 0.0f32 { xmm0 } else { 0.0f32 };
    let lim_d50 = rd::<f32>(exe::ptr::<f32>(0x1_4386_7d50) as usize);
    let xmm6 = if max0 < lim_d50 { max0 } else { lim_d50 };

    let lock_2d8 = param_1.wrapping_add(0x2d8);
    exe::call_iat!(0x1_4264_55a8, (lock_2d8));
    let g_82b8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
    let rax_ctx = exe::call_va!(0x1_410d_80d0, (g_82b8) -> usize);
    let mut pkt: [u32; 4] = [
        param_1 as u32,
        (param_1 >> 32) as u32,
        xmm7.to_bits(),
        xmm6.to_bits(),
    ];
    exe::call_va!(0x1_412d_efc0, (rax_ctx, pkt.as_mut_ptr() as usize));
    exe::call_iat!(0x1_4264_5598, (lock_2d8));

    let mut r8 = rd::<u64>(param_1 + 0x40);
    let cl = r8 > rdi && xmm6 > rd::<f32>(param_1 + 0x10);
    let al = r8 < rbp && rd::<f32>(param_1 + 0x10) > xmm6 && rd::<u64>(param_1 + 0x48) < rdi;
    let rdx_arg = if cl || al {
        wr::<f32>(param_1 + 0x10, xmm6);
        r8 = rd::<u64>(param_1 + 0x48);
        param_1.wrapping_add(0x170)
    } else {
        param_1.wrapping_add(0x60)
    };

    wr::<u64>(param_1 + 0x08, r8);
    let mut rcx_pass = exe::call_va!(0x1_4220_6560, (param_1, rdx_arg) -> u64);
    wr::<u64>(param_1 + 0x40, 0);
    wr::<u64>(param_1 + 0x48, 0);

    let p50 = rd::<usize>(param_1 + 0x50);
    let len58 = rd::<u32>(param_1 + 0x58) as usize;
    rcx_pass = exe::call_va!(
        0x1_410c_3eb0,
        (rcx_pass, p50, p50.wrapping_add(len58 * 8)) -> u64
    );
    wr::<u32>(param_1 + 0x58, 0);

    let mut cur_a = param_1.wrapping_add(0x60);
    let end_a = param_1.wrapping_add(0x170);
    while cur_a != end_a {
        let p = rd::<usize>(cur_a);
        let n = rd::<u32>(cur_a + 0x08) as usize;
        rcx_pass = exe::call_va!(0x1_412d_f250, (rcx_pass, p, p.wrapping_add(n << 4)) -> u64);
        wr::<u32>(cur_a + 0x08, 0);
        cur_a = cur_a.wrapping_add(0x10);
    }

    let mut cur_b = param_1.wrapping_add(0x170);
    let end_b = param_1.wrapping_add(0x280);
    while cur_b != end_b {
        let p = rd::<usize>(cur_b);
        let n = rd::<u32>(cur_b + 0x08) as usize;
        rcx_pass = exe::call_va!(0x1_412d_f250, (rcx_pass, p, p.wrapping_add(n << 4)) -> u64);
        wr::<u32>(cur_b + 0x08, 0);
        cur_b = cur_b.wrapping_add(0x10);
    }
}

/// `unknown_14220ea70`
/// Original: `0x14220ea70` (588 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_14220ea70(param_1: usize, param_2: usize) -> usize {
    wr::<u32>(param_1, 0);
    let rdi_len = rd::<u64>(param_2 + 0x08);
    if rdi_len == 0 {
        return param_1;
    }

    let r12_lock = exe::ptr::<u8>(0x1_4386_7e00) as usize;
    let g_tid_addr = exe::ptr::<u32>(0x1_4386_7e08) as usize;
    let g_cnt_addr = exe::ptr::<u32>(0x1_4386_7e0c) as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(g_tid_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (r12_lock));
        wr::<u32>(g_tid_addr, tid);
    }
    wr::<u32>(g_cnt_addr, rd::<u32>(g_cnt_addr).wrapping_add(1));

    let r11_hash = exe::call_va!(0x1_424b_16a0, (rd::<u64>(param_2), rdi_len) -> u64);
    let prime: u64 = 0x0000_0100_0000_01b3;
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for shift in [0u32, 8, 16, 24, 32, 40, 48, 56] {
        h ^= (r11_hash >> shift) & 0xff;
        h = h.wrapping_mul(prime);
    }
    let prod: u128 = (h as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u128);
    let rcx_mix: u64 = ((prod >> 64) as u64).wrapping_add(prod as u64);

    let r10_mask = rd::<u64>(exe::ptr::<u64>(0x1_4386_7de8) as usize);
    let mut r8_pos = (rcx_mix >> 7) & r10_mask;
    let mut rdi_step: u64 = 0;
    let h2: u8 = (rcx_mix as u8) & 0x7f;
    let xmm2 = vpbroadcastb(h2);
    let rbx_slots = rd::<usize>(exe::ptr::<usize>(0x1_4386_7dd8) as usize);
    let r14_ctrl = rd::<usize>(exe::ptr::<usize>(0x1_4386_7dd0) as usize);

    let found_existing = loop {
        let xmm1 = rd::<u128>(r14_ctrl.wrapping_add(r8_pos as usize));
        let mut m = vpmovmskb(vpcmpeqb(xmm1, xmm2)) as u32;
        let mut matched_idx: Option<u64> = None;
        while m != 0 {
            let bit = m.trailing_zeros() as u64;
            let idx = r8_pos.wrapping_add(bit) & r10_mask;
            if rd::<u64>(rbx_slots.wrapping_add((idx as usize) * 16)) == r11_hash {
                matched_idx = Some(idx);
                break;
            }
            m &= m.wrapping_sub(1);
        }
        if let Some(idx) = matched_idx {
            if r14_ctrl.wrapping_add(idx as usize) != r14_ctrl.wrapping_add(r10_mask as usize) {
                let val = rd::<u32>(rbx_slots.wrapping_add((idx as usize) * 16 + 0x08));
                wr::<u32>(param_1, val);
                break true;
            } else {
                break false;
            }
        }
        let empty_mask = vpmovmskb(vpsignb(xmm1, xmm1)) as u32;
        if empty_mask != 0 {
            break false;
        }
        rdi_step = rdi_step.wrapping_add(0x10);
        r8_pos = r8_pos.wrapping_add(rdi_step) & r10_mask;
    };

    if !found_existing {
        let g_next_id = exe::ptr::<i32>(0x1_4384_15c4) as usize;
        let cur_id = rd::<i32>(g_next_id);
        wr::<i32>(g_next_id, cur_id.wrapping_add(1));
        wr::<i32>(param_1, cur_id);

        let dst_str = (exe::ptr::<u8>(0x1_43a8_ae10) as isize)
            .wrapping_add((cur_id as isize).wrapping_mul(0x88)) as usize;
        let mut str_copy = rd::<[u64; 2]>(param_2);
        let rax_ret = exe::call_va!(
            0x1_4240_9b50,
            (
                dst_str,
                str_copy.as_mut_ptr() as usize,
                0u32,
                rd::<u64>(param_2 + 0x08)
            ) -> u64
        );
        let ebx = rd::<u32>(param_1);
        let mut hash_slot: u64 = r11_hash;
        let mut iter_out = [0u64; 3];
        exe::call_va!(
            0x1_412e_7bc0,
            (
                rax_ret,
                iter_out.as_mut_ptr() as usize,
                (&mut hash_slot as *mut u64) as usize
            )
        );
        let slot_ptr = iter_out[1] as usize;
        wr::<u32>(slot_ptr + 0x08, ebx);
    }

    let new_cnt = rd::<u32>(g_cnt_addr).wrapping_sub(1);
    wr::<u32>(g_cnt_addr, new_cnt);
    if new_cnt == 0 {
        crate::simd_ops::atomic_xchg_u32(g_tid_addr as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (r12_lock));
    }

    param_1
}

// ============================================================================
// Differential tests
// ============================================================================

pub unsafe fn diff_r_1420b2330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u16, usize, usize, usize, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HEAP_BUF: [u8; 4096] = [0; 4096];
    static mut HEAP_OFF: usize = 0;
    static mut FREE_CNT: u32 = 0;
    static mut SORT_CNT: u32 = 0;
    static mut RESERVE_ARG: u64 = 0;

    unsafe extern "win64" fn mock_alloc(size: usize, _align: u32) -> usize {
        unsafe {
            let aligned = (HEAP_OFF + 15) & !15;
            let ptr = (core::ptr::addr_of_mut!(HEAP_BUF) as *mut u8).add(aligned) as usize;
            HEAP_OFF = aligned + size.max(16);
            ptr
        }
    }
    unsafe extern "win64" fn mock_free(_ptr: usize) {
        unsafe { FREE_CNT += 1 };
    }
    unsafe extern "win64" fn mock_find(ptr: usize, len: u64, needle: u16) -> i32 {
        for i in 0..(len as usize) {
            if rd::<u16>(ptr + i * 2) == needle {
                return i as i32;
            }
        }
        -1
    }
    unsafe extern "win64" fn mock_sort_dedup(
        _p_begin: usize,
        _p_end: usize,
        _len: u64,
        _flag: u32,
    ) {
        unsafe { SORT_CNT += 1 };
    }
    unsafe extern "win64" fn mock_reserve(_p7: usize, cap: u64) {
        unsafe { RESERVE_ARG = cap };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411e_ef20, mock_sort_dedup as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_reserve as *const ())?;

    let run = |f: F| -> ([u64; 4], u32, u32, u64) {
        unsafe {
            HEAP_BUF = [0; 4096];
            HEAP_OFF = 0;
            FREE_CNT = 0;
            SORT_CNT = 0;
            RESERVE_ARG = 0;
        }

        let grps: [u16; 2] = [0, 1];
        let p2: [u64; 2] = [2, grps.as_ptr() as u64];

        let packed_items: [u32; 2] = [(0u32 << 16) | 0, (1u32 << 16) | 1];
        let mut span_entry = [0u8; 0x10];
        wr::<usize>(span_entry.as_mut_ptr() as usize, packed_items.as_ptr() as usize);
        wr::<u32>(span_entry.as_mut_ptr() as usize + 0x08, 2);
        let p3: [u64; 2] = [1, span_entry.as_ptr() as u64];

        let bits_a: [u64; 2] = [3, 65];
        let bits_b: [u64; 1] = [12];
        let mut p4_arr = [0u8; 0x40];
        let p4a = p4_arr.as_mut_ptr() as usize;
        wr::<u64>(p4a + 0x10, 2);
        wr::<usize>(p4a + 0x18, bits_a.as_ptr() as usize);
        wr::<u64>(p4a + 0x30, 1);
        wr::<usize>(p4a + 0x38, bits_b.as_ptr() as usize);
        let p4: [u64; 2] = [0, p4a as u64];

        let comp_list0: [u16; 1] = [0];
        let comp_list1: [u16; 1] = [1];
        let mut p5 = vec![0u8; 0x180a0];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<usize>(p5p + 0x18050, comp_list0.as_ptr() as usize);
        wr::<u32>(p5p + 0x18068, 1);
        wr::<usize>(p5p + 0x20 + 0x18050, comp_list1.as_ptr() as usize);
        wr::<u32>(p5p + 0x20 + 0x18068, 1);

        let extra_bits0: [u16; 1] = [7];
        let extra_bits1: [u16; 1] = [70];
        let mut p6_tbl = [0u8; 0x20];
        let p6t = p6_tbl.as_mut_ptr() as usize;
        wr::<u64>(p6t, 1);
        wr::<usize>(p6t + 0x08, extra_bits0.as_ptr() as usize);
        wr::<u64>(p6t + 0x10, 1);
        wr::<usize>(p6t + 0x18, extra_bits1.as_ptr() as usize);
        let mut p6 = [0u8; 0x20];
        wr::<usize>(p6.as_mut_ptr() as usize + 0x10, p6t);

        let mut out_words: [u64; 4] = [0; 4];
        let tmpl_words: [u64; 2] = [1, 2];
        let mut p7 = [0u8; 0x48];
        let p7p = p7.as_mut_ptr() as usize;
        wr::<usize>(p7p, out_words.as_mut_ptr() as usize);
        wr::<u64>(p7p + 0x10, 10);
        wr::<usize>(p7p + 0x18, tmpl_words.as_ptr() as usize);
        wr::<u64>(p7p + 0x38, 4);
        wr::<u64>(p7p + 0x40, 2);

        f(
            2,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5p,
            p6.as_ptr() as usize,
            p7p,
            3,
        );

        (out_words, unsafe { FREE_CNT }, unsafe { SORT_CNT }, unsafe { RESERVE_ARG })
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b2330 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420c4950(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUMP_MEM: [u8; 4096] = [0; 4096];
    static mut BUMP_OFF: usize = 0;
    static mut JC_SLOTS: [usize; 3] = [0; 3];
    static mut JC_IDX: usize = 0;
    static mut COPY_CALLS: u32 = 0;
    static mut PER_PAGE_CTX: [u64; 4] = [0; 4];

    unsafe extern "win64" fn mock_bump(_arena: usize, bytes: u64, _align: u32) -> usize {
        unsafe {
            let aligned = (BUMP_OFF + 15) & !15;
            let ptr = (core::ptr::addr_of_mut!(BUMP_MEM) as *mut u8).add(aligned) as usize;
            BUMP_OFF = aligned + (bytes as usize).max(16);
            ptr
        }
    }
    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_alloc_job(_pool: usize, out_handle: usize) {
        unsafe {
            let slot = JC_SLOTS[JC_IDX % 3];
            JC_IDX += 1;
            wr::<u8>(slot + 0x04, 1);
            wr::<usize>(out_handle, slot);
            wr::<u16>(out_handle + 0x08, rd::<u16>(slot + 0x06) >> 3);
            wr::<u16>(out_handle + 0x0a, 0);
            wr::<u8>(out_handle + 0x0c, 0);
        }
    }
    unsafe extern "win64" fn mock_copy(dst: usize, _span: usize, _handle: usize) {
        unsafe {
            if COPY_CALLS == 0 {
                wr::<u32>(dst, 2);
                wr::<u32>(dst + 4, 0);
                wr::<u32>(dst + 8, 1);
            }
            COPY_CALLS += 1;
        }
    }
    unsafe extern "win64" fn mock_sort(ptr: usize, count: i64) {
        let n = count as usize;
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(rd::<u32>(ptr + i * 4));
        }
        v.sort_unstable();
        for (i, val) in v.into_iter().enumerate() {
            wr::<u32>(ptr + i * 4, val);
        }
    }
    unsafe extern "win64" fn mock_unique(_vec_ptr: usize) {}
    unsafe extern "win64" fn mock_wait_fut(_g: usize, _fut: usize) {}
    unsafe extern "win64" fn mock_enqueue(_sched: usize, _job: usize) {}
    unsafe extern "win64" fn mock_wait_cnt(_g: usize, _handle: usize) {}
    unsafe extern "win64" fn mock_per_page(_page: usize, _rdx: u64, _r8: u64, ctx: usize) {
        let c = rd::<[u64; 4]>(ctx);
        unsafe {
            PER_PAGE_CTX = [c[0], 0, c[2], 0];
        }
    }
    unsafe extern "win64" fn mock_rel_sem(_h: usize, _c: u32, _p: usize) -> i32 {
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_bump as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc_job as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420c_5830, mock_copy as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420a_f720, mock_sort as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411f_1460, mock_unique as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4248_5780, mock_wait_fut as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4248_6460, mock_enqueue as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4248_5680, mock_wait_cnt as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_411f_33a0, mock_per_page as *const ())?;
    let _m11 = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;

    let g_82f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_82f8 = rd::<usize>(g_82f8);

    let run = |f: F| -> ([u64; 6], [u64; 6], u32, [u64; 4], u16) {
        let mut jc0 = [0u8; 0x60];
        let mut jc1 = [0u8; 0x60];
        let mut jc2 = [0u8; 0x60];
        wr::<u8>(jc0.as_mut_ptr() as usize + 0x04, 3);
        wr::<u16>(jc0.as_mut_ptr() as usize + 0x06, 0x18);
        wr::<u8>(jc1.as_mut_ptr() as usize + 0x04, 1);

        unsafe {
            BUMP_MEM = [0; 4096];
            BUMP_OFF = 0;
            JC_SLOTS = [
                jc0.as_mut_ptr() as usize,
                jc1.as_mut_ptr() as usize,
                jc2.as_mut_ptr() as usize,
            ];
            JC_IDX = 0;
            COPY_CALLS = 0;
            PER_PAGE_CTX = [0; 4];
        }

        let mut sched_inner = vec![0u8; 0x1020];
        wr::<usize>(sched_inner.as_mut_ptr() as usize + 0x1008, 0x5555);
        let mut sched = [0u8; 0x90];
        wr::<usize>(sched.as_mut_ptr() as usize, sched_inner.as_mut_ptr() as usize);
        wr::<usize>(sched.as_mut_ptr() as usize + 0x48, 0x1000);

        let mut g_obj = [0u8; 0x20];
        wr::<u64>(g_obj.as_mut_ptr() as usize, 2);
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sched.as_mut_ptr() as usize);
        wr::<usize>(g_82f8, g_obj.as_mut_ptr() as usize);

        let mut node_30 = [0u8; 0x20];
        wr::<u64>(node_30.as_mut_ptr() as usize + 0x08, 1);
        wr::<u32>(node_30.as_mut_ptr() as usize + 0x10, 1);

        let mut node_50 = [0u8; 0x20];
        wr::<u64>(node_50.as_mut_ptr() as usize + 0x08, 1);
        wr::<u32>(node_50.as_mut_ptr() as usize + 0x10, 2);

        let mut node_68 = [0u8; 0x20];
        wr::<u64>(node_68.as_mut_ptr() as usize + 0x08, 1);
        wr::<u16>(node_68.as_mut_ptr() as usize + 0x10, 1);

        let mut p3 = vec![0u8; 0x110];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x10, 0x9999);
        wr::<u64>(p3p + 0x20, 1);
        wr::<usize>(p3p + 0x30, node_30.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x38, node_30.as_mut_ptr() as usize);
        wr::<u64>(p3p + 0x40, 1);
        wr::<usize>(p3p + 0x50, node_50.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x58, node_50.as_mut_ptr() as usize);
        wr::<u64>(p3p + 0x60, 1);
        wr::<usize>(p3p + 0x68, node_68.as_mut_ptr() as usize);
        wr::<u64>(p3p + 0x100, 0);

        let ent_locs: [u64; 3] = [0, 1, 2];
        let p4: [u64; 2] = [3, ent_locs.as_ptr() as u64];

        let grp_hashes: [u64; 3] = [0x1111_0000, 0x2222_0000, 0x3333_0000];
        let mut p5 = [0u8; 0x30];
        wr::<usize>(p5.as_mut_ptr() as usize + 0x20, grp_hashes.as_ptr() as usize);

        let comp_hashes: [u64; 2] = [0x0000_00aa, 0x0000_00bb];
        let mut p6 = [0u8; 0x20];
        wr::<usize>(p6.as_mut_ptr() as usize + 0x10, comp_hashes.as_ptr() as usize);

        let rem_pairs: [u64; 2] = [(0u64 << 32) | 0, (2u64 << 32) | 1];
        let mut p7 = [0u8; 0x30];
        wr::<usize>(p7.as_mut_ptr() as usize + 0x08, rem_pairs.as_ptr() as usize);
        wr::<u32>(p7.as_mut_ptr() as usize + 0x10, 2);

        let mut out_res: [u64; 6] = [0; 6];
        let ret = f(
            out_res.as_mut_ptr() as usize,
            0xabcd_0000,
            p3p,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            p6.as_ptr() as usize,
            p7.as_ptr() as usize,
        );
        assert_eq!(ret, out_res.as_mut_ptr() as usize);

        let pairs_out = rd::<[u64; 6]>(out_res[1] as usize);
        // Zero buffer pointers inside out_res before comparing
        out_res[1] = 0;
        out_res[4] = 0;

        (
            out_res,
            pairs_out,
            unsafe { COPY_CALLS },
            unsafe { PER_PAGE_CTX },
            rd::<u16>(jc0.as_ptr() as usize + 0x06),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_82f8, s_82f8);

    crate::diff_eq!(o, n, "1420c4950 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420d4ca0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, idx: u64| -> Vec<u8> {
        let mut state = vec![0xaau8; 0x210];
        let sp = state.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x1f0, idx);
        let mut outer = [0u8; 0x50];
        wr::<usize>(outer.as_mut_ptr() as usize + 0x40, sp);
        f(outer.as_mut_ptr() as usize);
        state
    };

    for idx in [0u64, 1, 2] {
        let o = run(f_o, idx);
        let n = run(f_n, idx);
        crate::diff_eq!(o, n, "1420d4ca0 idx={idx}");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420d73d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut D4180_CNT: u32 = 0;
    static mut COPY_OFFS: [(usize, usize); 2] = [(0, 0); 2];
    static mut COPY_IDX: usize = 0;

    unsafe extern "win64" fn mock_tid() -> u32 {
        0x1234
    }
    unsafe extern "win64" fn mock_acq(_l: usize) {}
    unsafe extern "win64" fn mock_rel(_l: usize) {}
    unsafe extern "win64" fn mock_d4180(_rcx: u32, _slot: usize) {
        unsafe { D4180_CNT += 1 };
    }
    unsafe extern "win64" fn mock_copy(dst: usize, src: usize) {
        unsafe {
            let i = COPY_IDX & 1;
            COPY_IDX += 1;
            COPY_OFFS[i] = (dst, src);
        }
    }

    let _m1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420d_4180, mock_d4180 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_3f00, mock_copy as *const ())?;

    let run = |f: F, p2: u8, flag72: u8| -> (u32, [(usize, usize); 2], u64, u64, u32) {
        unsafe {
            D4180_CNT = 0;
            COPY_OFFS = [(0, 0); 2];
            COPY_IDX = 0;
        }
        let mut st0 = vec![0u8; 0x210];
        let s0p = st0.as_mut_ptr() as usize;
        wr::<u64>(s0p + 0x1f0, 4);
        wr::<u64>(s0p + 0x1f8, 10);

        let mut obj = vec![0u8; 0x55c + 16 * 0x330];
        let op = obj.as_mut_ptr() as usize;
        wr::<u8>(op + 0x72, flag72);
        wr::<usize>(op + 0x55c + 0x2c, s0p);

        f(op, p2);
        let c0 = unsafe {
            (
                COPY_OFFS[0].0.wrapping_sub(s0p),
                COPY_OFFS[0].1.wrapping_sub(s0p),
            )
        };
        (
            unsafe { D4180_CNT },
            [c0, (0, 0)],
            rd::<u64>(s0p + 0x1f0),
            rd::<u64>(s0p + 0x1f8),
            rd::<u32>(op + 0x55c - 0x04),
        )
    };

    for (p2, f72) in [(0u8, 1u8), (0u8, 0u8), (1u8, 1u8)] {
        let o = run(f_o, p2, f72);
        let n = run(f_n, p2, f72);
        crate::diff_eq!(o, n, "1420d73d0 p2={p2} f72={f72}");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420db7c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_pred(id: u16, ctx: u64) -> u8 {
        (id != 0x9999 && ctx == 0x1234) as u8
    }
    unsafe extern "win64" fn mock_dae80(_p1: usize, _p3_8: u64) {}
    unsafe extern "win64" fn mock_e02c0(_p3: usize, b1c: u8, w18: u16) -> u8 {
        b1c.wrapping_add(w18 as u8)
    }
    unsafe extern "win64" fn mock_db190(_p1: usize, _p3: usize, p2: u16) -> u8 {
        (p2 as u8) ^ 0x55
    }
    unsafe extern "win64" fn mock_daff0(
        k: u8,
        _p3: usize,
        p2: u16,
        w18: u16,
        b1c: u8,
    ) -> u8 {
        k.wrapping_add(p2 as u8).wrapping_add(w18 as u8).wrapping_add(b1c)
    }
    unsafe extern "win64" fn mock_df900(p3_0: usize) -> u8 {
        rd::<u8>(p3_0 + 0x10)
    }
    unsafe extern "win64" fn mock_d8810(_p3: usize, child: u16) -> u8 {
        (child & 1) as u8
    }
    unsafe extern "win64" fn mock_tid() -> u32 {
        77
    }
    unsafe extern "win64" fn mock_acq(_l: usize) {}
    unsafe extern "win64" fn mock_rel(_l: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_420d_ae80, mock_dae80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_02c0, mock_e02c0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420d_b190, mock_db190 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420d_aff0, mock_daff0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420d_f900, mock_df900 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420d_8810, mock_d8810 as *const ())?;
    let _m7 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _m8 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _m9 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;

    let run = |f: F, kind: u8, id: u16, uidx: u8, c0: u16, c1: u16| -> u8 {
        let mut p1 = [0u8; 0x28];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, mock_pred as *const () as usize);
        wr::<u8>(p1p + 0x0c, kind);
        wr::<u16>(p1p + 0x18, c0);
        wr::<u16>(p1p + 0x1a, c1);
        wr::<u8>(p1p + 0x1c, 9);

        let mut p3_0 = [0u8; 0x80];
        wr::<u8>(p3_0.as_mut_ptr() as usize + 0x10, uidx);
        wr::<u64>(p3_0.as_mut_ptr() as usize + 0x68, 0x1234);

        let mut tbl_58d0 = [0u8; 0x44];
        wr::<u16>(tbl_58d0.as_mut_ptr() as usize + 0x22, 0x42);
        wr::<u8>(tbl_58d0.as_mut_ptr() as usize + 0x22 + 2 + 2, 0xab);

        let mut p3_8 = vec![0u8; 0x5b00];
        let p38p = p3_8.as_mut_ptr() as usize;
        wr::<usize>(p38p + 0x58d0, tbl_58d0.as_mut_ptr() as usize);
        wr::<u32>(p38p + 0x58d8, 2);

        let p3: [usize; 2] = [p3_0.as_mut_ptr() as usize, p38p];
        f(p1p, id, p3.as_ptr() as usize)
    };

    let cases = [
        (4u8, 0x9999u16, 0u8, 1u16, 0u16),
        (0u8, 0x42u16, 0u8, 1u16, 0u16),
        (4u8, 0x42u16, 0u8, 3u16, 0u16),
        (5u8, 0x42u16, 0u8, 3u16, 0u16),
        (7u8, 0x42u16, 0u8, 3u16, 0u16),
        (8u8, 0x42u16, 0xffu8, 0u16, 0u16),
        (8u8, 0x42u16, 2u8, 0u16, 0u16),
        (3u8, 0x42u16, 0u8, 1u16, 0u16),
        (9u8, 0x42u16, 0u8, 1u16, 0u16),
        (1u8, 0x42u16, 0u8, 1u16, 0u16),
        (1u8, 0x42u16, 0u8, 1u16, 1u16),
        (2u8, 0x42u16, 0u8, 0u16, 1u16),
    ];

    for (kind, id, uidx, c0, c1) in cases {
        let o = run(f_o, kind, id, uidx, c0, c1);
        let n = run(f_n, kind, id, uidx, c0, c1);
        crate::diff_eq!(o, n, "1420db7c0 kind={kind} id={id}");
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_unknown_1420e02c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_95d0(_p1: usize, p3: u16) -> u8 {
        (p3 & 1) as u8
    }
    unsafe extern "win64" fn mock_98f0(_p1: usize, p3: u16) -> u8 {
        ((p3 >> 1) & 1) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_420d_95d0, mock_95d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420d_98f0, mock_98f0 as *const ())?;

    let mut count = 0u32;
    for p2 in 6u8..=14u8 {
        for p3 in 0u16..4u16 {
            let o = f_o(0x1000, p2, p3);
            let n = f_n(0x1000, p2, p3);
            crate::diff_eq!(o, n, "1420e02c0 p2={p2} p3={p3}");
            count += 1;
        }
    }
    Ok(count)
}

pub unsafe fn diff_unknown_1420e3d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SUB_LOG: u32 = 0;
    static mut FLUSH_LOG: [(usize, usize, usize); 2] = [(0, 0, 0); 2];
    static mut FLUSH_CNT: usize = 0;
    static mut NOW_VAL: u64 = 0x5555_6666;

    unsafe extern "win64" fn mock_d170(p: usize) -> u64 {
        rd::<u64>(p)
    }
    unsafe extern "win64" fn mock_2f00(_p: usize) -> usize {
        core::ptr::addr_of!(NOW_VAL) as usize
    }
    unsafe extern "win64" fn mock_step1(_p: usize, _z: u32) {
        unsafe { SUB_LOG |= 1 };
    }
    unsafe extern "win64" fn mock_step2(_p: usize, _z: u32) {
        unsafe { SUB_LOG |= 2 };
    }
    unsafe extern "win64" fn mock_step3(_p: usize, _z: u32) {
        unsafe { SUB_LOG |= 4 };
    }
    unsafe extern "win64" fn mock_step4(_p: usize) {
        unsafe { SUB_LOG |= 8 };
    }
    unsafe extern "win64" fn mock_step5(_p: usize) {
        unsafe { SUB_LOG |= 16 };
    }
    unsafe extern "win64" fn mock_step6(_p: usize) {
        unsafe { SUB_LOG |= 32 };
    }
    unsafe extern "win64" fn mock_f5380(_p0: usize, a: usize) {
        unsafe { FLUSH_LOG[FLUSH_CNT & 1].0 = a };
    }
    unsafe extern "win64" fn mock_f5520(_p0: usize, b: usize) {
        unsafe { FLUSH_LOG[FLUSH_CNT & 1].1 = b };
    }
    unsafe extern "win64" fn mock_f5690(_p0: usize, c: usize) {
        unsafe {
            FLUSH_LOG[FLUSH_CNT & 1].2 = c;
            FLUSH_CNT += 1;
        }
    }
    unsafe extern "win64" fn mock_pred(ctx: u64) -> u8 {
        (ctx == 1) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_4049_d170, mock_d170 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401f_2f00, mock_2f00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_420c_f110, mock_step1 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_420c_da40, mock_step2 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420d_2440, mock_step3 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420d_e930, mock_step4 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_420e_4e70, mock_step5 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_420d_a540, mock_step6 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_411f_5380, mock_f5380 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_411f_5520, mock_f5520 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_411f_5690, mock_f5690 as *const ())?;

    let run = |f: F, t10: u64| -> (u64, u32, [(usize, usize, usize); 2], usize) {
        unsafe {
            SUB_LOG = 0;
            FLUSH_LOG = [(0, 0, 0); 2];
            FLUSH_CNT = 0;
        }

        let mut p1 = vec![0u8; 0x5000];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x10, t10);
        wr::<u64>(p1p + 0x4fc0, 77);
        wr::<u64>(p1p + 0x4fc8, 1000);

        let mut entries = vec![0u8; 2 * 0x78];
        let ep = entries.as_mut_ptr() as usize;
        wr::<usize>(ep, p1p);
        wr::<u64>(ep + 0x08, 77);
        wr::<usize>(ep + 0x48, 0x111);
        wr::<usize>(ep + 0x50, 0x222);
        wr::<usize>(ep + 0x60, 0x333);
        wr::<u64>(ep + 0x68, 1);
        wr::<usize>(ep + 0x70, mock_pred as *const () as usize);

        wr::<usize>(p1p + 0x4fe0, ep);
        wr::<u32>(p1p + 0x4fe8, 2);

        f(p1p);
        (
            rd::<u64>(p1p + 0x4fc8),
            unsafe { SUB_LOG },
            unsafe { FLUSH_LOG },
            unsafe { FLUSH_CNT },
        )
    };

    for t10 in [0x7fff_ffff_ffff_ffffu64, 5u64] {
        let o = run(f_o, t10);
        let n = run(f_n, t10);
        crate::diff_eq!(o, n, "1420e3d70 t10={t10}");
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421c0320(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let pack4 = |x: f32, y: f32, z: f32, w: f32| -> u128 {
        (x.to_bits() as u128)
            | ((y.to_bits() as u128) << 32)
            | ((z.to_bits() as u128) << 64)
            | ((w.to_bits() as u128) << 96)
    };

    let tfs: [[u128; 2]; 4] = [
        [pack4(0.0, 0.0, 0.0, 1.0), pack4(1.0, 2.0, 3.0, 0.0)],
        [
            pack4(0.0, 0.70710677, 0.0, 0.70710677),
            pack4(4.0, 5.0, 6.0, 0.0),
        ],
        [
            pack4(0.5, 0.5, 0.5, 0.5),
            pack4(-1.0, 0.5, 2.0, 0.0),
        ],
        [
            pack4(0.0, 0.0, 0.70710677, 0.70710677),
            pack4(0.25, -0.75, 1.5, 0.0),
        ],
    ];
    let parents: [u16; 4] = [0xffff, 0, 1, 2];

    for start_idx in 0u32..4u32 {
        let mut out_o = [0u128; 2];
        let mut out_n = [0u128; 2];
        let ro = f_o(
            out_o.as_mut_ptr() as usize,
            tfs.as_ptr() as usize,
            parents.as_ptr() as usize,
            start_idx,
        );
        let rn = f_n(
            out_n.as_mut_ptr() as usize,
            tfs.as_ptr() as usize,
            parents.as_ptr() as usize,
            start_idx,
        );
        crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "1421c0320 ret o");
        crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "1421c0320 ret n");
        crate::diff_eq!(out_o, out_n, "1421c0320 idx={start_idx}");
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1422075a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PKT_CAPTURED: [u32; 2] = [0; 2];
    static mut RDX_OFF: usize = 0;
    static mut C3EB0_SPAN: usize = 0;
    static mut DF250_SUM: usize = 0;

    unsafe extern "win64" fn mock_acq(_l: usize) {}
    unsafe extern "win64" fn mock_rel(_l: usize) {}
    unsafe extern "win64" fn mock_80d0(_g: u64) -> usize {
        0x7777
    }
    unsafe extern "win64" fn mock_efc0(_ctx: usize, pkt_ptr: usize) {
        unsafe {
            PKT_CAPTURED = [rd::<u32>(pkt_ptr + 0x08), rd::<u32>(pkt_ptr + 0x0c)];
        }
    }
    unsafe extern "win64" fn mock_6560(p1: usize, rdx: usize) -> u64 {
        unsafe { RDX_OFF = rdx.wrapping_sub(p1) };
        0
    }
    unsafe extern "win64" fn mock_3eb0(_rcx: u64, b: usize, e: usize) -> u64 {
        unsafe { C3EB0_SPAN = e.wrapping_sub(b) };
        0
    }
    unsafe extern "win64" fn mock_f250(_rcx: u64, b: usize, e: usize) -> u64 {
        unsafe { DF250_SUM += e.wrapping_sub(b) };
        0
    }

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_410d_80d0, mock_80d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412d_efc0, mock_efc0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4220_6560, mock_6560 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_410c_3eb0, mock_3eb0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_412d_f250, mock_f250 as *const ())?;

    let g_698 = exe::ptr::<usize>(0x1_43a3_4698) as usize;
    let g_c78 = exe::ptr::<f32>(0x1_4386_7c78) as usize;
    let g_c98 = exe::ptr::<f32>(0x1_4386_7c98) as usize;
    let g_cd8 = exe::ptr::<f32>(0x1_4386_7cd8) as usize;
    let g_d50 = exe::ptr::<f32>(0x1_4386_7d50) as usize;

    let s_698 = rd::<usize>(g_698);
    let s_c78 = rd::<f32>(g_c78);
    let s_c98 = rd::<f32>(g_c98);
    let s_cd8 = rd::<f32>(g_cd8);
    let s_d50 = rd::<f32>(g_d50);

    let budget_val: u64 = 1000;
    wr::<usize>(g_698, (&budget_val as *const u64) as usize);
    wr::<f32>(g_c78, 0.2);
    wr::<f32>(g_c98, 0.1);
    wr::<f32>(g_cd8, 0.5);
    wr::<f32>(g_d50, 10.0);

    let run = |f: F, cur08: u64, v40: u64, v48: u64| -> (u64, u32, [u32; 2], usize, usize, usize) {
        unsafe {
            PKT_CAPTURED = [0; 2];
            RDX_OFF = 0;
            C3EB0_SPAN = 0;
            DF250_SUM = 0;
        }
        let mut p1 = vec![0u8; 0x300];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x08, cur08);
        wr::<f32>(p1p + 0x10, 2.0);
        wr::<u64>(p1p + 0x40, v40);
        wr::<u64>(p1p + 0x48, v48);
        wr::<usize>(p1p + 0x50, 0x1000);
        wr::<u32>(p1p + 0x58, 3);
        wr::<usize>(p1p + 0x60, 0x2000);
        wr::<u32>(p1p + 0x68, 2);
        wr::<usize>(p1p + 0x170, 0x3000);
        wr::<u32>(p1p + 0x178, 4);

        f(p1p);
        (
            rd::<u64>(p1p + 0x08),
            rd::<u32>(p1p + 0x10),
            unsafe { PKT_CAPTURED },
            unsafe { RDX_OFF },
            unsafe { C3EB0_SPAN },
            unsafe { DF250_SUM },
        )
    };

    for (cur08, v40, v48) in [(500u64, 700u64, 600u64), (950u64, 950u64, 920u64)] {
        let o = run(f_o, cur08, v40, v48);
        let n = run(f_n, cur08, v40, v48);
        crate::diff_eq!(o, n, "1422075a0 cur08={cur08}");
    }

    wr::<usize>(g_698, s_698);
    wr::<f32>(g_c78, s_c78);
    wr::<f32>(g_c98, s_c98);
    wr::<f32>(g_cd8, s_cd8);
    wr::<f32>(g_d50, s_d50);

    Ok(2)
}

pub unsafe fn diff_unknown_14220ea70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_SLOT: [u64; 2] = [0; 2];
    static mut STR_LEN_CAPTURED: u64 = 0;

    unsafe extern "win64" fn mock_tid() -> u32 {
        0x4444
    }
    unsafe extern "win64" fn mock_acq(_l: usize) {}
    unsafe extern "win64" fn mock_rel(_l: usize) {}
    unsafe extern "win64" fn mock_hash(ptr: u64, len: u64) -> u64 {
        ptr.wrapping_add(len)
    }
    unsafe extern "win64" fn mock_9b50(_dst: usize, _src: usize, _z: u32, len: u64) -> u64 {
        unsafe { STR_LEN_CAPTURED = len };
        0
    }
    unsafe extern "win64" fn mock_7bc0(_rcx: u64, iter_out: usize, hash_ptr: usize) {
        let h = rd::<u64>(hash_ptr);
        unsafe {
            NEW_SLOT[0] = h;
            NEW_SLOT[1] = 0;
            wr::<usize>(iter_out, 0);
            wr::<usize>(iter_out + 0x08, core::ptr::addr_of_mut!(NEW_SLOT) as usize);
            wr::<u8>(iter_out + 0x10, 1);
        }
    }

    let _m1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_424b_16a0, mock_hash as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_412e_7bc0, mock_7bc0 as *const ())?;

    let g_tid = exe::ptr::<u32>(0x1_4386_7e08) as usize;
    let g_cnt = exe::ptr::<u32>(0x1_4386_7e0c) as usize;
    let g_mask = exe::ptr::<u64>(0x1_4386_7de8) as usize;
    let g_slots = exe::ptr::<usize>(0x1_4386_7dd8) as usize;
    let g_ctrl = exe::ptr::<usize>(0x1_4386_7dd0) as usize;
    let g_id = exe::ptr::<i32>(0x1_4384_15c4) as usize;

    let s_tid = rd::<u32>(g_tid);
    let s_cnt = rd::<u32>(g_cnt);
    let s_mask = rd::<u64>(g_mask);
    let s_slots = rd::<usize>(g_slots);
    let s_ctrl = rd::<usize>(g_ctrl);
    let s_id = rd::<i32>(g_id);

    let run = |f: F, str_len: u64, prefill_hit: bool| -> (u32, i32, [u64; 2], u64) {
        unsafe {
            NEW_SLOT = [0; 2];
            STR_LEN_CAPTURED = 0;
        }
        let r11_hash: u64 = 0x1000u64.wrapping_add(str_len);
        let prime: u64 = 0x0000_0100_0000_01b3;
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for shift in [0u32, 8, 16, 24, 32, 40, 48, 56] {
            h ^= (r11_hash >> shift) & 0xff;
            h = h.wrapping_mul(prime);
        }
        let prod: u128 = (h as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u128);
        let mix: u64 = ((prod >> 64) as u64).wrapping_add(prod as u64);
        let h2: u8 = (mix as u8) & 0x7f;

        let mut ctrl = [0x80u8; 32];
        let mut slots = [0u64; 32];
        if prefill_hit {
            let pos = ((mix >> 7) & 15) as usize;
            ctrl[pos] = h2;
            slots[pos * 2] = r11_hash;
            slots[pos * 2 + 1] = 42;
        }

        wr::<u32>(g_tid, u32::MAX);
        wr::<u32>(g_cnt, 0);
        wr::<u64>(g_mask, 15);
        wr::<usize>(g_ctrl, ctrl.as_mut_ptr() as usize);
        wr::<usize>(g_slots, slots.as_mut_ptr() as usize);
        wr::<i32>(g_id, 7);

        let mut out_id: u32 = 999;
        let p2: [u64; 2] = [0x1000, str_len];
        let ret = f((&mut out_id as *mut u32) as usize, p2.as_ptr() as usize);
        assert_eq!(ret, (&mut out_id as *mut u32) as usize);

        (out_id, rd::<i32>(g_id), unsafe { NEW_SLOT }, unsafe { STR_LEN_CAPTURED })
    };

    for (len, hit) in [(0u64, false), (5u64, true), (5u64, false)] {
        let o = run(f_o, len, hit);
        let n = run(f_n, len, hit);
        crate::diff_eq!(o, n, "14220ea70 len={len} hit={hit}");
    }

    wr::<u32>(g_tid, s_tid);
    wr::<u32>(g_cnt, s_cnt);
    wr::<u64>(g_mask, s_mask);
    wr::<usize>(g_slots, s_slots);
    wr::<usize>(g_ctrl, s_ctrl);
    wr::<i32>(g_id, s_id);

    Ok(3)
}

// ============================================================================
// REGISTRY (for `games/alan-wake-2/crates/aw2-game/src/registry/core.rs`)
// ============================================================================
// Replacement::verified_throws(0x1_420b_2330, "r_1420b2330", rend::cauldron_hot_s::r_1420b2330 as *const (), Throws::Discard, rend::cauldron_hot_s::diff_r_1420b2330),
// Replacement::verified_throws(0x1_420c_4950, "unknown_1420c4950", rend::cauldron_hot_s::unknown_1420c4950 as *const (), Throws::Discard, rend::cauldron_hot_s::diff_unknown_1420c4950),
// Replacement::verified(0x1_420d_4ca0, "unknown_1420d4ca0", rend::cauldron_hot_s::unknown_1420d4ca0 as *const (), rend::cauldron_hot_s::diff_unknown_1420d4ca0),
// Replacement::verified(0x1_420d_73d0, "unknown_1420d73d0", rend::cauldron_hot_s::unknown_1420d73d0 as *const (), rend::cauldron_hot_s::diff_unknown_1420d73d0),
// Replacement::verified_throws(0x1_420d_b7c0, "unknown_1420db7c0", rend::cauldron_hot_s::unknown_1420db7c0 as *const (), Throws::Caught, rend::cauldron_hot_s::diff_unknown_1420db7c0),
// Replacement::verified_throws(0x1_420e_02c0, "unknown_1420e02c0", rend::cauldron_hot_s::unknown_1420e02c0 as *const (), Throws::Discard, rend::cauldron_hot_s::diff_unknown_1420e02c0),
// Replacement::verified_throws(0x1_420e_3d70, "unknown_1420e3d70", rend::cauldron_hot_s::unknown_1420e3d70 as *const (), Throws::Discard, rend::cauldron_hot_s::diff_unknown_1420e3d70),
// Replacement::verified(0x1_421c_0320, "unknown_1421c0320", rend::cauldron_hot_s::unknown_1421c0320 as *const (), rend::cauldron_hot_s::diff_unknown_1421c0320),
// Replacement::verified_throws(0x1_4220_75a0, "unknown_1422075a0", rend::cauldron_hot_s::unknown_1422075a0 as *const (), Throws::Caught, rend::cauldron_hot_s::diff_unknown_1422075a0),
// Replacement::verified(0x1_4220_ea70, "unknown_14220ea70", rend::cauldron_hot_s::unknown_14220ea70 as *const (), rend::cauldron_hot_s::diff_unknown_14220ea70),
