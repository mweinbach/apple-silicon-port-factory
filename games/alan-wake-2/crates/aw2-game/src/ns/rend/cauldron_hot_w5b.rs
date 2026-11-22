//! Decompiled hot functions shard W5b (`0x1420c3310..0x1421319a0`).

use crate::exe::{self, rd, wr};

/// `unknown_1420c3310`
/// Original: 0x1420c3310 (2028 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420c3310(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: u64,
    param_7: u64,
) -> usize {
    let mut frame = [0u64; 0x240 / 8];
    let rsp = frame.as_mut_ptr() as usize;
    let rbp = rsp.wrapping_add(0x100);

    let rdi = param_3.wrapping_add(0x68);
    let storage_38 = rd::<u32>(param_4 + 0x38) as u64;
    let storage_30 = rd::<u64>(param_4 + 0x30);
    wr::<u64>(rbp + 0x50, storage_38);
    wr::<u64>(rbp + 0x58, storage_30);

    wr::<[u64; 7]>(rbp - 0x60, [0; 7]);

    let local_arena = rsp.wrapping_add(0x40);
    wr::<u64>(local_arena, 0);
    wr::<u64>(local_arena + 0x08, 0);
    wr::<u64>(local_arena + 0x10, 0);
    wr::<u64>(local_arena + 0x18, 0x4000);

    let vec_60 = rsp.wrapping_add(0x60);
    wr::<usize>(vec_60, local_arena);
    wr::<u64>(vec_60 + 0x08, 0);
    wr::<u64>(vec_60 + 0x10, 0);

    let vec_78 = rsp.wrapping_add(0x78);
    wr::<usize>(vec_78, local_arena);
    wr::<u64>(rbp - 0x80, 0);
    wr::<u64>(rbp - 0x78, 0);

    wr::<usize>(rbp + 0x90, rdi);
    wr::<u64>(rbp + 0x98, storage_38);
    wr::<u64>(rbp + 0xa0, storage_30);
    wr::<u64>(rbp + 0xa8, param_6);
    wr::<usize>(rbp + 0xb0, rbp - 0x60);
    wr::<usize>(rbp + 0xb8, local_arena);
    wr::<usize>(rbp + 0xc0, vec_60);
    wr::<usize>(rbp + 0xc8, vec_78);

    let g_job_va = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let g_job1 = rd::<usize>(g_job_va);
    let sched1 = rd::<usize>(g_job1 + 0x10);
    let wid1 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool1 = rd::<usize>(sched1.wrapping_add(wid1 * 0x40).wrapping_add(0x48)).wrapping_add(0x80);
    exe::call_va!(0x1_4248_5aa0, (pool1, rbp - 0x28));

    let job1 = rd::<usize>(rbp - 0x28);
    let _ = crate::simd_ops::atomic_xchg_u32(job1 as u64, 1);
    let job1_b = rd::<usize>(rbp - 0x28);
    wr::<u64>(job1_b + 0x08, 0);
    wr::<usize>(job1_b + 0x10, exe::ptr::<u8>(0x1_433d_dcc8) as usize);
    let payload1 = rd::<[u64; 8]>(rbp + 0x90);
    wr::<[u64; 8]>(job1_b + 0x18, payload1);
    if rd::<u32>(job1_b) == 1 {
        let sched1_b = rd::<usize>(g_job1 + 0x10);
        exe::call_va!(0x1_4248_6460, (sched1_b, rd::<usize>(rbp - 0x28)));
        let sem1 = rd::<u64>(rd::<usize>(sched1_b) + 0x1008);
        let _ = exe::call_iat!(0x1_4264_5580, (sem1, 1u32, 0usize) -> i32);
    }

    let v_170 = rd::<u64>(rdi + 0x108);
    let v_178 = rd::<u64>(rdi + 0x110);
    let v_180 = rd::<u64>(rdi + 0x118);
    let v_158 = rd::<u64>(rdi + 0xf0);
    let v_160 = rd::<u64>(rdi + 0xf8);
    let v_168 = rd::<u64>(rdi + 0x100);
    let esi_u32 = v_168 as u32;
    let r12_alloc = exe::call_va!(0x1_411f_11e0, (param_2, v_168.wrapping_mul(8), 8u32) -> usize);

    wr::<u64>(rbp - 0x10, esi_u32 as u64);
    wr::<usize>(rbp - 0x08, r12_alloc);
    wr::<u64>(rbp + 0x90, esi_u32 as u64);
    wr::<usize>(rbp + 0x98, r12_alloc);
    wr::<u64>(rbp + 0xa0, v_158);
    wr::<u64>(rbp + 0xa8, v_160);
    wr::<u64>(rbp + 0xb0, v_168);
    wr::<u64>(rbp + 0xb8, v_170);
    wr::<u64>(rbp + 0xc0, v_178);
    wr::<u64>(rbp + 0xc8, v_180);

    let g_job2 = rd::<usize>(g_job_va);
    let sched2 = rd::<usize>(g_job2 + 0x10);
    let wid2 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool2 = rd::<usize>(sched2.wrapping_add(wid2 * 0x40).wrapping_add(0x48)).wrapping_add(0x80);
    exe::call_va!(0x1_4248_5aa0, (pool2, rbp - 0x70));

    let job2 = rd::<usize>(rbp - 0x70);
    let _ = crate::simd_ops::atomic_xchg_u32(job2 as u64, 1);
    let job2_b = rd::<usize>(rbp - 0x70);
    wr::<u64>(job2_b + 0x08, 0);
    wr::<usize>(job2_b + 0x10, exe::ptr::<u8>(0x1_433d_dc50) as usize);
    let payload2 = rd::<[u64; 8]>(rbp + 0x90);
    wr::<[u64; 8]>(job2_b + 0x18, payload2);
    if rd::<u32>(job2_b) == 1 {
        let sched2_b = rd::<usize>(g_job2 + 0x10);
        exe::call_va!(0x1_4248_6460, (sched2_b, rd::<usize>(rbp - 0x70)));
        let sem2 = rd::<u64>(rd::<usize>(sched2_b) + 0x1008);
        let _ = exe::call_iat!(0x1_4264_5580, (sem2, 1u32, 0usize) -> i32);
    }

    wr::<usize>(rbp + 0x60, param_2);
    wr::<usize>(rbp + 0x68, r12_alloc);
    wr::<u32>(rbp + 0x70, esi_u32);
    wr::<u32>(rbp + 0x74, esi_u32);
    let jh2 = rd::<[u64; 2]>(rbp - 0x70);
    wr::<[u64; 2]>(rbp + 0x78, jh2);

    wr::<usize>(rbp, param_2);
    let ebx_u32 = param_7 as u32;
    wr::<u32>(rbp + 0x10, ebx_u32);
    wr::<u32>(rbp + 0x14, ebx_u32);
    let grp_cnt_ptr = exe::call_va!(0x1_411f_11e0, (param_2, param_7.wrapping_mul(8), 4u32) -> usize);
    wr::<usize>(rbp + 0x08, grp_cnt_ptr);
    if ebx_u32 != 0 {
        for i in 0..(ebx_u32 as usize) {
            wr::<u64>(grp_cnt_ptr.wrapping_add(i * 8), 0);
        }
    }

    let _ = exe::call_va!(
        0x1_420c_5de0,
        (
            rbp + 0x100,
            param_2,
            rbp,
            rdi,
            param_5,
            rd::<usize>(param_4)
        ) -> usize
    );

    let s50 = rd::<[u64; 2]>(rbp + 0x50);
    wr::<[u64; 2]>(rbp - 0x10, s50);
    let _ = exe::call_va!(
        0x1_420c_4950,
        (
            rbp + 0x18,
            param_2,
            rdi,
            rbp - 0x10,
            param_5,
            rd::<usize>(param_4),
            rbp + 0x60
        ) -> usize
    );

    exe::call_va!(0x1_4248_5680, (rd::<usize>(g_job_va), rbp - 0x28));

    let mut rax_ptr = rd::<usize>(rbp - 0x40);
    let rcx_len = rd::<usize>(rbp - 0x48);
    let rbx_end = rax_ptr.wrapping_add(rcx_len.wrapping_mul(8));
    let r15_arr = rd::<usize>(rbp + 0x38);
    if rax_ptr != rbx_end {
        let edi_len = rd::<u32>(rbp + 0x40) as usize;
        let rsi_arr = rd::<usize>(rbp + 0x20);
        while rax_ptr != rbx_end {
            let rcx_val = rd::<u64>(rax_ptr);
            let mut r9_cnt = edi_len;
            let mut rdx_idx: usize = 0;
            if edi_len != 0 {
                let r11d = (rcx_val >> 32) as u32;
                loop {
                    let r10_half = r9_cnt >> 1;
                    let r8_mid = r10_half.wrapping_add(rdx_idx);
                    let r9_next = r9_cnt.wrapping_sub(r10_half).wrapping_add(rdx_idx);
                    if r11d >= rd::<u32>(r15_arr.wrapping_add(r8_mid * 4)) {
                        rdx_idx = r9_next;
                    }
                    r9_cnt = r10_half;
                    if r10_half == 0 {
                        break;
                    }
                }
            }
            let slot_addr = rsi_arr
                .wrapping_add(rdx_idx.wrapping_sub(1).wrapping_mul(16))
                .wrapping_add(8);
            let r10_cur = rd::<u64>(slot_addr);
            if r10_cur != u64::MAX {
                let comp_ctx = rd::<usize>(param_4);
                let comp_id = (rcx_val as u16) as usize;
                let mask_tbl = rd::<usize>(comp_ctx + 0x10);
                let mask = rd::<u64>(mask_tbl.wrapping_add(comp_id * 8));
                wr::<u64>(slot_addr, mask ^ r10_cur);
            }
            rax_ptr = rax_ptr.wrapping_add(8);
        }
    }

    let local_len = rd::<u32>(local_arena + 0x08) as usize;
    let local_beg = rd::<usize>(local_arena);
    let local_end = local_beg.wrapping_add(local_len * 8);
    if local_beg != local_end {
        let rbx_off = (rd::<u32>(param_2 + 0x08) as usize).wrapping_mul(8);
        let insert_pos = rd::<usize>(param_2).wrapping_add(rbx_off);
        let mut arg_beg = local_beg;
        let mut arg_end = local_end;
        exe::call_va!(
            0x1_40f7_d880,
            (
                param_2,
                insert_pos,
                (&mut arg_beg as *mut usize) as usize,
                (&mut arg_end as *mut usize) as usize
            )
        );
        wr::<u32>(local_arena + 0x08, 0);
        let p2_rem = rd::<u64>(param_2 + 0x18).wrapping_sub(rd::<u64>(param_2 + 0x10));
        let loc_18 = rd::<u64>(local_arena + 0x18);
        let loc_10 = rd::<u64>(local_arena + 0x10);
        let loc_rem = loc_18.wrapping_sub(loc_10);
        if loc_rem > p2_rem {
            let p2_base = rd::<usize>(param_2);
            let p2_new_end = p2_base.wrapping_add((rd::<u32>(param_2 + 0x08) as usize) * 8);
            let slot_a = p2_base.wrapping_add(rbx_off).wrapping_sub(8);
            let slot_b = p2_new_end.wrapping_sub(8);
            let va = rd::<u64>(slot_a);
            let vb = rd::<u64>(slot_b);
            wr::<u64>(slot_a, vb);
            wr::<u64>(slot_b, va);
        } else {
            wr::<u64>(param_2 + 0x10, loc_10);
            wr::<u64>(param_2 + 0x18, loc_18);
        }
        wr::<u64>(local_arena + 0x10, 0);
        wr::<u64>(local_arena + 0x18, 0);
    }

    let edi_count = rd::<u32>(rbp + 0x28) as usize;
    let out_arr = exe::call_va!(0x1_411f_11e0, (param_2, (edi_count as u64) * 8, 8u32) -> usize);
    wr::<usize>(rbp - 0x70, out_arr);
    let _ = exe::call_va!(0x1_4183_acc2, (out_arr, 0xffu32, (edi_count as u64) * 8) -> usize);

    for r14 in 0..edi_count {
        let rsi_entry = rd::<usize>(rbp + 0x20).wrapping_add(r14 * 16);
        let eax_idx = rd::<u32>(r15_arr.wrapping_add(r14 * 4)) as usize;
        let r11_key = rd::<u64>(rsi_entry);
        let mut rcx_cnt = rd::<u32>(param_5 + 0x08) as usize;
        let r10_keys = rd::<usize>(param_5);
        let mut rdi_pos: usize = 0;
        if rcx_cnt != 0 {
            loop {
                let r9_half = rcx_cnt >> 1;
                let rax_mid = rdi_pos.wrapping_add(r9_half);
                let r8_next = rcx_cnt.wrapping_sub(r9_half).wrapping_add(rdi_pos);
                if r11_key >= rd::<u64>(r10_keys.wrapping_add(rax_mid * 8)) {
                    rdi_pos = r8_next;
                }
                rcx_cnt = r9_half;
                if r9_half == 0 {
                    break;
                }
            }
        }
        let sig_new = rd::<u64>(rsi_entry + 0x08);
        let found = exe::call_va!(
            0x1_411f_1520,
            (r10_keys as u64, rd::<u32>(param_5 + 0x08), sig_new) -> i32
        );
        let lut_10 = rd::<usize>(param_5 + 0x10);
        let old_grp = rd::<u16>(lut_10.wrapping_add(rdi_pos * 2).wrapping_sub(2)) as usize;
        let r9_cnts = rd::<usize>(rbp + 0x08);
        let old_cnt_addr = r9_cnts.wrapping_add(old_grp * 8).wrapping_add(4);
        wr::<u32>(old_cnt_addr, rd::<u32>(old_cnt_addr).wrapping_add(1));

        let ent_info = (storage_30 as usize).wrapping_add(eax_idx * 8);
        let new_ent_val = if sig_new == u64::MAX {
            u32::MAX
        } else {
            rd::<u32>(ent_info + 0x04)
        };
        let dst_elem = out_arr.wrapping_add(r14 * 8);
        wr::<u32>(dst_elem, new_ent_val);
        wr::<u16>(dst_elem + 0x06, rd::<u16>(ent_info));
        if found != -1 {
            let new_grp = rd::<u16>(lut_10.wrapping_add((found as isize as usize) * 2));
            let new_cnt_addr = r9_cnts.wrapping_add((new_grp as usize) * 8);
            wr::<u32>(new_cnt_addr, rd::<u32>(new_cnt_addr).wrapping_add(1));
            wr::<u16>(dst_elem + 0x04, new_grp);
        }
    }

    let final_cnts = rd::<usize>(rbp + 0x08);
    wr::<usize>(param_1, param_2);
    wr::<usize>(param_1 + 0x08, final_cnts);
    wr::<u32>(param_1 + 0x10, ebx_u32);
    wr::<u32>(param_1 + 0x14, ebx_u32);
    wr::<usize>(param_1 + 0x18, param_2);
    wr::<usize>(param_1 + 0x20, out_arr);
    wr::<u32>(param_1 + 0x28, edi_count as u32);
    wr::<u32>(param_1 + 0x2c, edi_count as u32);
    wr::<u64>(param_1 + 0x30, rd::<u64>(rbp + 0x18));
    wr::<u64>(param_1 + 0x38, rd::<u64>(rbp + 0x20));
    wr::<u32>(param_1 + 0x40, rd::<u32>(rbp + 0x28));
    wr::<u32>(param_1 + 0x44, rd::<u32>(rbp + 0x2c));
    wr::<u64>(param_1 + 0x48, rd::<u64>(rbp + 0x30));
    wr::<usize>(param_1 + 0x50, r15_arr);
    wr::<u32>(param_1 + 0x58, rd::<u32>(rbp + 0x40));
    wr::<u32>(param_1 + 0x5c, rd::<u32>(rbp + 0x44));
    wr::<u64>(param_1 + 0x60, rd::<u64>(rbp + 0x100));
    wr::<u64>(param_1 + 0x68, rd::<u64>(rbp + 0x108));
    wr::<u32>(param_1 + 0x70, rd::<u32>(rbp + 0x110));
    wr::<u32>(param_1 + 0x74, rd::<u32>(rbp + 0x114));
    wr::<[u64; 2]>(param_1 + 0x78, rd::<[u64; 2]>(rbp - 0x60));
    wr::<u32>(param_1 + 0x88, rd::<u32>(rbp - 0x50));
    wr::<u32>(param_1 + 0x8c, rd::<u32>(rbp - 0x4c));
    wr::<u64>(rbp - 0x58, 0);
    wr::<u64>(rbp - 0x50, 0);
    wr::<[u64; 4]>(param_1 + 0x90, rd::<[u64; 4]>(rbp - 0x48));
    wr::<u64>(param_1 + 0xb0, rd::<u64>(rbp + 0x118));
    wr::<u64>(param_1 + 0xb8, rd::<u64>(rbp + 0x120));
    wr::<u32>(param_1 + 0xc0, rd::<u32>(rbp + 0x128));
    wr::<u32>(param_1 + 0xc4, rd::<u32>(rbp + 0x12c));
    wr::<u64>(param_1 + 0xc8, rd::<u64>(vec_60));
    wr::<u64>(param_1 + 0xd0, rd::<u64>(vec_60 + 0x08));
    wr::<u32>(param_1 + 0xd8, rd::<u32>(vec_60 + 0x10));
    wr::<u32>(param_1 + 0xdc, rd::<u32>(vec_60 + 0x14));
    wr::<u64>(vec_60 + 0x08, 0);
    wr::<u64>(vec_60 + 0x10, 0);
    wr::<u64>(param_1 + 0xe0, rd::<u64>(vec_78));
    wr::<u64>(param_1 + 0xe8, rd::<u64>(rbp - 0x80));
    wr::<u32>(param_1 + 0xf0, rd::<u32>(rbp - 0x78));
    wr::<u32>(param_1 + 0xf4, rd::<u32>(rbp - 0x74));
    wr::<u64>(rbp - 0x80, 0);
    wr::<u64>(rbp - 0x78, 0);
    wr::<u64>(param_1 + 0xf8, rd::<u64>(rbp + 0x60));
    wr::<u64>(param_1 + 0x100, rd::<u64>(rbp + 0x68));
    wr::<u32>(param_1 + 0x108, rd::<u32>(rbp + 0x70));
    wr::<u32>(param_1 + 0x10c, rd::<u32>(rbp + 0x74));

    exe::call_va!(0x1_420a_a3d0, (local_arena));
    exe::call_va!(0x1_401d_e830, (local_arena));
    param_1
}

#[inline(always)]
unsafe fn init_job_counter(
    g_job_sys: usize,
    jc_ptr: usize,
    name_va: u64,
    name_len: u64,
) {
    let sched = rd::<usize>(g_job_sys + 0x10);
    let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool = rd::<usize>(sched.wrapping_add(wid * 0x40).wrapping_add(0x48)).wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool, jc_ptr));
    let job = rd::<usize>(jc_ptr);
    let _ = crate::simd_ops::atomic_xchg_u32(job as u64, 1);
    let job2 = rd::<usize>(jc_ptr);
    wr::<u64>(job2 + 0x08, 0);
    wr::<usize>(job2 + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(job2 + 0x18, exe::ptr::<u8>(name_va) as usize);
    wr::<u64>(job2 + 0x20, name_len);
    let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((job2 + 0x04) as u64, 1, 3);
    if ok {
        let _ = crate::simd_ops::atomic_xadd_u32(rd::<usize>(jc_ptr) as u64, 0xffff);
    }
    wr::<u8>(jc_ptr + 0x0a, 1);
}

/// `unknown_1420c5de0`
/// Original: 0x1420c5de0 (2041 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420c5de0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
) -> usize {
    let g_job_sys = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let workers_x3 = rd::<u64>(g_job_sys).wrapping_mul(3);

    let mut r14: usize = 0;
    let mut cap_40: u32 = 0;
    let r12 = rd::<u64>(param_4 + 0xb0);
    if r12 != 0 {
        let byte_size = r12.wrapping_mul(8);
        r14 = exe::call_va!(0x1_411f_11e0, (param_2, byte_size, 0x20u32) -> usize);
        if r14 == 0 {
            r14 = exe::call_va!(0x1_411f_11e0, (param_2, byte_size, 0x20u32) -> usize);
        }
        cap_40 = r12 as u32;
    }

    let mut jc1 = [0u64; 2];
    let jc1_ptr = jc1.as_mut_ptr() as usize;
    init_job_counter(g_job_sys, jc1_ptr, 0x1_433d_d8e8, 0x25);

    let rbx_u64 = (r12 as u32) as u64;
    let mut view1: [u64; 2] = [rbx_u64, r14 as u64];
    exe::call_va!(
        0x1_411f_30e0,
        (
            jc1_ptr,
            rd::<u64>(param_4 + 0xa0),
            exe::ptr::<u8>(0x1_420c_5d30) as usize,
            0usize,
            view1.as_mut_ptr() as usize
        )
    );
    let mut view2: [u64; 2] = [rbx_u64, r14 as u64];
    exe::call_va!(
        0x1_411f_3240,
        (
            jc1_ptr,
            rd::<u64>(param_4 + 0xb8),
            exe::ptr::<u8>(0x1_420c_5ca0) as usize,
            0usize,
            view2.as_mut_ptr() as usize
        )
    );
    exe::call_va!(0x1_4248_5680, (g_job_sys, jc1_ptr));
    exe::call_va!(
        0x1_420a_f9a0,
        (
            r14,
            r12 as u32,
            exe::ptr::<u8>(0x1_420c_5c40) as usize
        )
    );

    let mut buf_70: usize = 0;
    let mut cap_44: u32 = 0;
    let mut r13_bytes = rbx_u64.wrapping_mul(8);
    if (r12 as u32) != 0 {
        buf_70 = exe::call_va!(0x1_411f_11e0, (param_2, r13_bytes, 0x20u32) -> usize);
        if buf_70 == 0 {
            buf_70 = exe::call_va!(0x1_411f_11e0, (param_2, r13_bytes, 0x20u32) -> usize);
        }
        cap_44 = r12 as u32;
    }

    let chunk_size = ((rbx_u64 / workers_x3).wrapping_add(3)) & !3u64;
    let mut jc2 = [0u64; 2];
    let jc2_ptr = jc2.as_mut_ptr() as usize;
    init_job_counter(g_job_sys, jc2_ptr, 0x1_433d_d8b8, 0x29);

    let mut dst_cur = buf_70;
    let mut src_cur = r14;
    let mut rem_elems = rbx_u64;
    if chunk_size != 0 && (rem_elems as i64) > (chunk_size as i64) {
        let chunk_bytes = (chunk_size as usize).wrapping_mul(8);
        loop {
            let sched = rd::<usize>(g_job_sys + 0x10);
            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool =
                rd::<usize>(sched.wrapping_add(wid * 0x40).wrapping_add(0x48)).wrapping_add(0x80);
            let mut ch = [0u64; 2];
            let ch_ptr = ch.as_mut_ptr() as usize;
            exe::call_va!(0x1_4248_5aa0, (pool, ch_ptr));
            let c_job = rd::<usize>(ch_ptr);
            let _ = crate::simd_ops::atomic_xchg_u32(c_job as u64, 1);
            let c_job2 = rd::<usize>(ch_ptr);
            wr::<u64>(c_job2 + 0x08, 0);
            wr::<usize>(c_job2 + 0x10, exe::ptr::<u8>(0x1_433d_dd40) as usize);
            wr::<usize>(c_job2 + 0x18, dst_cur);
            wr::<usize>(c_job2 + 0x20, src_cur);
            wr::<u64>(c_job2 + 0x28, chunk_size);
            let _ = crate::simd_ops::atomic_xadd_u32(rd::<usize>(jc2_ptr) as u64, 1);
            let c_job3 = rd::<usize>(ch_ptr);
            wr::<usize>(c_job3 + 0x08, rd::<usize>(jc2_ptr));
            if rd::<u32>(c_job3) == 1 {
                let sched2 = rd::<usize>(g_job_sys + 0x10);
                exe::call_va!(0x1_4248_6460, (sched2, rd::<usize>(ch_ptr)));
                let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
                let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
            }
            rem_elems = rem_elems.wrapping_sub(chunk_size);
            dst_cur = dst_cur.wrapping_add(chunk_bytes);
            src_cur = src_cur.wrapping_add(chunk_bytes);
            if (rem_elems as i64) <= (chunk_size as i64) {
                break;
            }
        }
        r13_bytes = rbx_u64.wrapping_mul(8);
    }

    let _ = exe::call_va!(0x1_4183_acbc, (dst_cur, src_cur, rem_elems.wrapping_mul(8)) -> usize);
    exe::call_va!(0x1_4248_5680, (g_job_sys, jc2_ptr));

    let mut end_ptr = r14.wrapping_add(r13_bytes as usize);
    if r14 != end_ptr {
        let mut rcx = r14;
        let mut rdx = r14.wrapping_add(8);
        if rdx != end_ptr {
            let mut r9 = rd::<u64>(r14);
            loop {
                let r8 = rd::<u64>(rdx);
                let mut rax = rdx.wrapping_add(8);
                if r9 == r8 {
                    while rax != end_ptr {
                        let v = rd::<u64>(rax);
                        if rd::<u64>(rcx) != v {
                            rcx = rcx.wrapping_add(8);
                            wr::<u64>(rcx, v);
                        }
                        rax = rax.wrapping_add(8);
                    }
                    end_ptr = rcx.wrapping_add(8);
                    break;
                }
                rcx = rdx;
                rdx = rax;
                r9 = r8;
                if rax == end_ptr {
                    break;
                }
            }
        }
    }

    let unique_count = ((end_ptr.wrapping_sub(r14) as isize) >> 3) as u64;
    if unique_count > (cap_40 as u64) {
        let new_bytes = unique_count.wrapping_mul(8);
        let mut grown_in_place = false;
        if r14 == 0 {
            let p = exe::call_va!(0x1_411f_11e0, (param_2, new_bytes, 8u32) -> usize);
            if p != 0 {
                r14 = p;
                grown_in_place = true;
            }
        } else {
            let blocks_ptr = rd::<usize>(param_2);
            let blocks_len = rd::<u32>(param_2 + 0x08) as usize;
            let blocks_end = blocks_ptr.wrapping_add(blocks_len * 8);
            if blocks_ptr != blocks_end {
                let old_bytes = (cap_40 as u64).wrapping_mul(8);
                let prev_off = rd::<u64>(param_2 + 0x10).wrapping_sub(old_bytes);
                let last_block = rd::<usize>(blocks_end - 8);
                if last_block.wrapping_add(prev_off as usize) == r14 {
                    let aligned_new = new_bytes.wrapping_add(7) & !7u64;
                    let new_off = aligned_new.wrapping_add(prev_off);
                    if new_off <= rd::<u64>(param_2 + 0x18) {
                        wr::<u64>(param_2 + 0x10, new_off);
                        grown_in_place = true;
                    }
                }
            }
        }
        if !grown_in_place {
            let new_p = exe::call_va!(0x1_411f_11e0, (param_2, new_bytes, 8u32) -> usize);
            let old_len = r12 as u32;
            for i in 0..(old_len as usize) {
                wr::<u64>(new_p.wrapping_add(i * 8), rd::<u64>(r14.wrapping_add(i * 8)));
            }
            r14 = new_p;
        }
    }

    if unique_count > rbx_u64 {
        let mut p = r14.wrapping_add(r13_bytes as usize);
        let mut diff = unique_count.wrapping_sub(rbx_u64);
        while diff != 0 {
            wr::<u64>(p, 0);
            p = p.wrapping_add(8);
            diff -= 1;
        }
    }

    let num_entities = rd::<u32>(param_4);
    let sigs_ptr = exe::call_va!(0x1_411f_11e0, (param_2, (num_entities as u64) << 3, 8u32) -> usize);
    let mut vec_58: [u64; 3] = [
        param_2 as u64,
        sigs_ptr as u64,
        (num_entities as u64) | ((num_entities as u64) << 32),
    ];
    if num_entities != 0 {
        let _ = exe::call_va!(0x1_4183_acc2, (sigs_ptr, 0u32, (num_entities as u64) << 3) -> usize);
    }

    let mut jc3 = [0u64; 2];
    let jc3_ptr = jc3.as_mut_ptr() as usize;
    init_job_counter(g_job_sys, jc3_ptr, 0x1_433d_db70, 0x31);

    let r12_rem = (unique_count as u32) as u64;
    let r15_chunk = r12_rem / workers_x3;
    let mut rbx_rem = r12_rem as i64;
    let mut rsi_idx: u64 = 0;
    if (r12_rem as i64) > (r15_chunk as i64) {
        loop {
            let mut rdx_cnt = r15_chunk;
            let r8_start = rsi_idx;
            rsi_idx = rsi_idx.wrapping_add(r15_chunk);
            rbx_rem = rbx_rem.wrapping_sub(r15_chunk as i64);
            let mut rax_ptr = r14.wrapping_add((rsi_idx as usize) * 8).wrapping_add(4);
            let ecx_hi = rd::<u32>(rax_ptr);
            if rbx_rem > 0 {
                while rd::<u32>(rax_ptr) == ecx_hi {
                    rsi_idx = rsi_idx.wrapping_add(1);
                    rax_ptr = rax_ptr.wrapping_add(8);
                    rdx_cnt = rdx_cnt.wrapping_add(1);
                    rbx_rem -= 1;
                    if rbx_rem <= 0 {
                        break;
                    }
                }
            }
            let slice_ptr = r14.wrapping_add((r8_start as usize) * 8);
            if rdx_cnt == u64::MAX {
                rdx_cnt = r12_rem.wrapping_sub(r8_start);
            }
            let sched = rd::<usize>(g_job_sys + 0x10);
            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool =
                rd::<usize>(sched.wrapping_add(wid * 0x40).wrapping_add(0x48)).wrapping_add(0x80);
            let mut ch = [0u64; 2];
            let ch_ptr = ch.as_mut_ptr() as usize;
            exe::call_va!(0x1_4248_5aa0, (pool, ch_ptr));
            let c_job = rd::<usize>(ch_ptr);
            let _ = crate::simd_ops::atomic_xchg_u32(c_job as u64, 1);
            let c_job2 = rd::<usize>(ch_ptr);
            wr::<u64>(c_job2 + 0x08, 0);
            wr::<usize>(c_job2 + 0x10, exe::ptr::<u8>(0x1_433d_dc80) as usize);
            wr::<usize>(c_job2 + 0x18, vec_58.as_mut_ptr() as usize);
            wr::<usize>(c_job2 + 0x20, param_6);
            wr::<u64>(c_job2 + 0x28, rdx_cnt);
            wr::<usize>(c_job2 + 0x30, slice_ptr);
            let _ = crate::simd_ops::atomic_xadd_u32(rd::<usize>(jc3_ptr) as u64, 1);
            wr::<usize>(c_job2 + 0x08, rd::<usize>(jc3_ptr));
            let c_job3 = rd::<usize>(ch_ptr);
            if rd::<u32>(c_job3) == 1 {
                let sched2 = rd::<usize>(g_job_sys + 0x10);
                exe::call_va!(0x1_4248_6460, (sched2, c_job3));
                let sem = rd::<u64>(rd::<usize>(sched2) + 0x1008);
                let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
            }
            if rbx_rem <= (r15_chunk as i64) {
                break;
            }
        }
    }

    let mut rax_cur = r14.wrapping_add((rsi_idx as usize) * 8);
    let rem_diff = r12_rem.wrapping_sub(rsi_idx);
    let tail_cnt = if (rbx_rem as u64) == u64::MAX {
        rem_diff
    } else {
        rbx_rem as u64
    };
    let rdx_end = rax_cur.wrapping_add((tail_cnt as usize) * 8);
    while rax_cur != rdx_end {
        let pair = rd::<u64>(rax_cur);
        let comp_id = (pair & 0xffff) as usize;
        let ent_idx = (pair >> 32) as usize;
        let mask_tbl = rd::<usize>(param_6 + 0x10);
        let comp_mask = rd::<u64>(mask_tbl.wrapping_add(comp_id * 8));
        let cur_sigs_ptr = vec_58[1] as usize;
        let sig_addr = cur_sigs_ptr.wrapping_add(ent_idx * 8);
        wr::<u64>(sig_addr, rd::<u64>(sig_addr) ^ comp_mask);
        rax_cur = rax_cur.wrapping_add(8);
    }

    exe::call_va!(0x1_4248_5680, (g_job_sys, jc3_ptr));

    let final_sigs_ptr = vec_58[1] as usize;
    let final_sigs_len = (vec_58[2] as u32) as usize;
    let final_sigs_end = final_sigs_ptr.wrapping_add(final_sigs_len * 8);
    let mut cur_sig = final_sigs_ptr;
    while cur_sig != final_sigs_end {
        let found = exe::call_va!(
            0x1_411f_1520,
            (rd::<u64>(param_5), rd::<u32>(param_5 + 0x08), rd::<u64>(cur_sig)) -> i32
        );
        if found != -1 {
            let grp_idx = rd::<u16>(
                rd::<usize>(param_5 + 0x10).wrapping_add((found as isize as usize) * 2),
            ) as usize;
            let cnt_addr = rd::<usize>(param_3 + 0x08).wrapping_add(grp_idx * 8);
            wr::<u32>(cnt_addr, rd::<u32>(cnt_addr).wrapping_add(1));
        }
        cur_sig = cur_sig.wrapping_add(8);
    }

    wr::<u64>(param_1, vec_58[0]);
    wr::<u64>(param_1 + 0x08, vec_58[1]);
    wr::<u64>(param_1 + 0x10, vec_58[2]);
    wr::<usize>(param_1 + 0x18, param_2);
    wr::<usize>(param_1 + 0x20, buf_70);
    wr::<u32>(param_1 + 0x28, r12 as u32);
    wr::<u32>(param_1 + 0x2c, cap_44);
    param_1
}

/// `unknown_1420c7f40`
/// Original: 0x1420c7f40 (361 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420c7f40(param_1: usize, param_2: usize) {
    let u58 = rd::<u64>(param_2 + 0x58);
    wr::<u64>(param_2 + 0x58, u64::MAX);
    wr::<u64>(param_1 + 0x58, u58);

    let u60 = rd::<u64>(param_2 + 0x60);
    wr::<u64>(param_2 + 0x60, 0);
    wr::<u64>(param_1 + 0x60, rd::<u64>(param_1 + 0x60).wrapping_add(u60));

    exe::call_va!(
        0x1_420b_0730,
        (param_1.wrapping_add(0x08), param_2.wrapping_add(0x08))
    );

    let esi = rd::<u32>(param_1 + 0x68);
    let ecx = rd::<u32>(param_2 + 0x68).wrapping_add(esi);
    wr::<u32>(param_1 + 0x68, ecx);
    wr::<u32>(param_2 + 0x68, 0);

    for off in [0x78usize, 0x98, 0xb8, 0xd0, 0xe8] {
        exe::call_va!(
            0x1_411f_3d40,
            (param_1.wrapping_add(off), param_2.wrapping_add(off))
        );
    }

    let r9 = rd::<usize>(param_2 + 0x110);
    let mut rax: u64 = 0;
    let mut rcx = rd::<usize>(param_2 + 0x108);
    let r8: u64 = if r9 != 0 { rd::<u64>(r9 + 0x08) } else { 0 };

    while rcx != r9 || rax != r8 {
        let elem_addr = rcx.wrapping_add((rax as usize).wrapping_mul(4)).wrapping_add(0x10);
        wr::<u32>(elem_addr, rd::<u32>(elem_addr).wrapping_add(esi));
        rax = rax.wrapping_add(1);
        if rax == rd::<u64>(rcx + 0x08) {
            let next = rd::<usize>(rcx);
            if next != 0 {
                rcx = next;
                rax = 0;
            }
        }
    }

    for off in [0x108usize, 0x120, 0x138, 0x158, 0x170] {
        exe::call_va!(
            0x1_411f_3d40,
            (param_1.wrapping_add(off), param_2.wrapping_add(off))
        );
    }
}

/// `unknown_1420e9b00`
/// Original: 0x1420e9b00 (264 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420e9b00(param_1: usize, param_2: usize) {
    exe::call_va!(0x1_411f_76e0, (param_2));
    let rdi = rd::<usize>(param_2 + 0x08);
    let mut rbp = rd::<u64>(param_2 + 0x18);
    exe::call_va!(0x1_411f_76e0, (param_2));

    let count = rd::<u32>(param_2 + 0x10) as usize;
    let r14 = rd::<usize>(param_2 + 0x08).wrapping_add(count.wrapping_mul(0x88));
    if rdi != r14 {
        let mut rbx = rdi.wrapping_add(0x08);
        loop {
            let b3 = rd::<u8>(rbx - 0x05) as u32;
            let cap_bits = if b3 < 0x80 {
                b3.wrapping_mul(8)
            } else {
                (b3 << 8).wrapping_sub(0x7c00)
            };
            let len_bits = (rd::<u32>(rbx - 0x08) & 0x00ff_ffff) << 3;
            let mut str_ptr = if len_bits <= cap_bits {
                rbx
            } else {
                rd::<usize>(rbx)
            };

            let mut len: usize = 0;
            while rd::<u8>(str_ptr.wrapping_add(len)) != 0 {
                len = len.wrapping_add(1);
            }
            let str_end = str_ptr.wrapping_add(len);
            let mut hash: u32 = 0x811c_9dc5;
            while str_ptr != str_end {
                let ch = (rd::<i8>(str_ptr) as i32 as u32) | 0x20;
                str_ptr = str_ptr.wrapping_add(1);
                hash = (ch ^ hash).wrapping_mul(0x0100_0193);
            }

            let mut key: u32 = hash;
            let mut out_iter = [0u64; 4];
            exe::call_va!(
                0x1_411f_8170,
                (
                    param_1.wrapping_add(0x70),
                    out_iter.as_mut_ptr() as usize,
                    (&mut key as *mut u32) as usize
                )
            );
            let slot_ptr = out_iter[1] as usize;
            rbx = rbx.wrapping_add(0x88);
            wr::<u64>(slot_ptr.wrapping_add(0x08), rbp);
            rbp = rbp.wrapping_add(0x120);
            if rbx.wrapping_sub(0x08) == r14 {
                break;
            }
        }
    }
}

/// `unknown_1421319a0`
/// Original: 0x1421319a0 (1481 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1421319a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) {
    let mut buf_ptr: usize = 0;
    let mut len: u32 = 0;
    let mut cap: u64 = 0;

    let count = rd::<u64>(param_5);
    let mut init_cap = count.wrapping_mul(2);
    let max_cap = rd::<u64>(param_3 + 0x10);
    if max_cap < init_cap {
        init_cap = max_cap;
    }
    if init_cap != 0 {
        cap = init_cap;
        buf_ptr = exe::call_va!(0x1_4241_c0a0, (init_cap.wrapping_mul(4), 4u32) -> usize);
    }

    let mut rbx = rd::<usize>(param_5 + 0x08);
    let end_rbx = rbx.wrapping_add((count as usize).wrapping_mul(12));
    if rbx != end_rbx {
        let end_corners = exe::ptr::<u8>(0x1_433e_25d0) as usize;
        while rbx != end_rbx {
            let mut r13 = exe::ptr::<u8>(0x1_433e_2570) as usize;
            while r13 != end_corners {
                let x = rd::<u32>(rbx);
                let y = rd::<u32>(rbx + 0x04);
                let z = rd::<u32>(rbx + 0x08);
                let grid_info = rd::<usize>(param_1 + 0x28);
                let dim_x = rd::<u32>(grid_info + 0x08);
                let dim_y = rd::<u32>(grid_info + 0x0c);
                let dx = rd::<u32>(r13);
                let dy = rd::<u32>(r13 + 0x04);
                let dz = rd::<u32>(r13 + 0x08);

                let idx_u32 = dz
                    .wrapping_add(z)
                    .wrapping_mul(dim_x)
                    .wrapping_add(dx)
                    .wrapping_add(x)
                    .wrapping_mul(dim_y)
                    .wrapping_add(dy)
                    .wrapping_add(y);
                let idx_s64 = (idx_u32 as i32) as i64 as u64;
                let bitset_base = rd::<usize>(param_3);
                let word_idx = (idx_s64 >> 6) as usize;
                let bit_mask = 1u64 << (idx_u32 & 0x3f);
                let word_addr = bitset_base.wrapping_add(word_idx.wrapping_mul(8));
                let word_val = rd::<u64>(word_addr);
                if (word_val & bit_mask) == 0 {
                    wr::<u64>(word_addr, (word_val & !bit_mask) | bit_mask);
                    let cur_end = buf_ptr.wrapping_add((len as usize).wrapping_mul(4));
                    let cap_end = buf_ptr.wrapping_add((cap as u32 as usize).wrapping_mul(4));
                    if cur_end == cap_end {
                        let byte_size = cur_end.wrapping_sub(buf_ptr);
                        let cur_len = ((byte_size as isize) >> 2) as u64;
                        let old_cap = ((cap_end.wrapping_sub(buf_ptr) as isize) >> 2) as u64;
                        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
                        if new_cap < cur_len.wrapping_add(1) {
                            new_cap = cur_len.wrapping_add(1);
                        }
                        cap = new_cap;
                        let new_buf =
                            exe::call_va!(0x1_4241_c0a0, (new_cap.wrapping_mul(4), 4u32) -> usize);
                        let _ =
                            exe::call_va!(0x1_4183_acc8, (new_buf, buf_ptr, byte_size as u64) -> usize);
                        if buf_ptr != 0 {
                            exe::call_va!(0x1_4241_c190, (buf_ptr));
                        }
                        buf_ptr = new_buf;
                        len = cur_len as u32;
                    }
                    wr::<u32>(buf_ptr.wrapping_add((len as usize).wrapping_mul(4)), idx_u32);
                    len = len.wrapping_add(1);
                }
                r13 = r13.wrapping_add(12);
            }
            rbx = rbx.wrapping_add(12);
        }
    }

    let end_buf = buf_ptr.wrapping_add((len as usize).wrapping_mul(4));
    let sort_count = ((end_buf.wrapping_sub(buf_ptr) as isize) >> 2) as u64;
    let mut sort_begin: usize = buf_ptr;
    let mut sort_end: usize = end_buf;
    exe::call_va!(
        0x1_40f9_1150,
        (
            (&mut sort_begin as *mut usize) as usize,
            (&mut sort_end as *mut usize) as usize,
            sort_count,
            0u32
        )
    );

    if buf_ptr != end_buf {
        let sign_mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
        let c_100 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6ea4) as usize);
        let mut cur = buf_ptr;
        while cur != end_buf {
            let r8 = (rd::<i32>(cur) as i64) as u64;
            let elem_addr = rd::<usize>(param_1 + 0x08).wrapping_add((r8 as usize).wrapping_mul(12));
            let v0 = rd::<f32>(elem_addr);
            let v1 = rd::<f32>(elem_addr + 0x04);
            let v2 = rd::<f32>(elem_addr + 0x08);
            let g0 = rd::<f32>(rd::<usize>(param_1 + 0x28));

            let dist = if 0.0f32 > v1 {
                f32::from_bits(v1.to_bits() ^ sign_mask)
            } else {
                let p4_0 = rd::<f32>(param_4);
                let p4_1 = rd::<f32>(param_4 + 0x04);
                let max_d = ((p4_0 + p4_0) + p4_1) + g0;
                if v1 > max_d {
                    v1 - c_100
                } else {
                    let rad = (v2 * v2 + v0 * v0).sqrt();
                    let p4_2 = rd::<f32>(param_4 + 0x08);
                    let add_g0 = if p4_2 < v1 { g0 } else { 0.0f32 };
                    let min_p = if p4_0 < v1 { p4_0 } else { v1 };
                    rad - (min_p + add_g0)
                }
            };

            let bitset2 = rd::<usize>(param_2);
            let word_idx = (r8 >> 6) as usize;
            let bit = 1u64 << ((r8 as u32) & 0x3f);
            let word_addr = bitset2.wrapping_add(word_idx * 8);
            let old_word = rd::<u64>(word_addr);
            let mask = if dist >= 0.0f32 { u64::MAX } else { 0 };
            wr::<u64>(word_addr, (old_word & !bit) | (mask & bit));

            cur = cur.wrapping_add(4);
        }
    }

    let mut rax = rd::<usize>(param_5 + 0x08);
    let end_rax = rax.wrapping_add((count as usize).wrapping_mul(12));
    if rax != end_rax {
        let end_r11 = exe::ptr::<u8>(0x1_433e_25e4) as usize;
        while rax != end_rax {
            let mut ebx: i32 = 0;
            let mut r10d: i32 = 0;
            let mut edi: i32 = 0;
            let mut r9: usize = 0;
            let grid_info = rd::<usize>(param_1 + 0x28);
            let dim_y = rd::<u32>(grid_info + 0x0c);
            let dim_x = rd::<u32>(grid_info + 0x08);
            let bitset2 = rd::<usize>(param_2);
            let flags_arr = rd::<usize>(param_1 + 0x18);
            let mut r11 = exe::ptr::<u8>(0x1_433e_2584) as usize;
            let mut local_bytes = [0u8; 8];

            let x = rd::<u32>(rax);
            let y = rd::<u32>(rax + 0x04);
            let z = rd::<u32>(rax + 0x08);

            while r11 < end_r11 {
                for (off_dx, off_dy, off_dz, slot) in [
                    (r11 - 0x14, r11 - 0x10, r11 - 0x0c, r9),
                    (r11 - 0x08, r11 - 0x04, r11, r9 + 1),
                ] {
                    let dx = rd::<u32>(off_dx);
                    let dy = rd::<u32>(off_dy);
                    let dz = rd::<u32>(off_dz);
                    let ecx = dz
                        .wrapping_add(z)
                        .wrapping_mul(dim_x)
                        .wrapping_add(dx)
                        .wrapping_add(x)
                        .wrapping_mul(dim_y)
                        .wrapping_add(y.wrapping_add(dy));
                    let idx_s64 = (ecx as i32) as i64 as u64;
                    let mut b = rd::<u8>(flags_arr.wrapping_add(idx_s64 as usize));
                    local_bytes[slot] = b;
                    let bit_set = (rd::<u64>(bitset2.wrapping_add(((idx_s64 >> 6) as usize) * 8))
                        & (1u64 << (ecx & 0x3f)))
                        != 0;
                    if (b & 1) == 0 && bit_set {
                        b = (b & 0xf9) | 1;
                        local_bytes[slot] = b;
                    }
                    if (b & 2) != 0 {
                        ebx += 1;
                    }
                    if (b & 4) != 0 {
                        r10d += 1;
                    }
                    if (b & 8) != 0 {
                        edi += 1;
                    }
                }
                r9 += 2;
                r11 = r11.wrapping_add(0x18);
            }

            let mut dl = (local_bytes[0] & 0xf8) | 0x01;
            if r10d + ebx == 8 {
                if r10d == 8 {
                    dl = (dl & 0xfc) | 0x04;
                } else {
                    dl = (dl & 0xfa) | 0x02;
                }
            }
            let mut r9b = if edi > 0 { dl | 0x08 } else { dl };

            let grid_info2 = rd::<usize>(param_1 + 0x28);
            let dim_x2 = rd::<u32>(grid_info2 + 0x08);
            let dim_y2 = rd::<u32>(grid_info2 + 0x0c);
            let flags_arr2 = rd::<usize>(param_1 + 0x18);
            let base_idx = dim_x2.wrapping_mul(z).wrapping_add(x).wrapping_mul(dim_y2);
            let idx_p1 = (base_idx.wrapping_add(y.wrapping_add(1)) as i32) as i64 as usize;
            let idx_p2 = (base_idx.wrapping_add(y.wrapping_add(2)) as i32) as i64 as usize;
            if (r9b & 0x04) != 0
                && ((rd::<u8>(flags_arr2.wrapping_add(idx_p1)) & 0x02) != 0
                    || (rd::<u8>(flags_arr2.wrapping_add(idx_p2)) & 0x02) != 0)
            {
                r9b = (r9b & 0xfa) | 0x02;
            }
            let idx_self = (base_idx.wrapping_add(y) as i32) as i64 as usize;
            wr::<u8>(flags_arr2.wrapping_add(idx_self), r9b);

            rax = rax.wrapping_add(12);
        }
    }

    if buf_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (buf_ptr));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1420c3310(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut JOB_IDX: usize = 0;
    static mut JOB_SLOTS: [usize; 2] = [0; 2];
    static mut ALLOC_IDX: usize = 0;
    static mut ALLOC_BUFS: [usize; 3] = [0; 3];
    static mut EXTRA_PAIRS_PTR: usize = 0;
    static mut C4950_PAIRS_PTR: usize = 0;
    static mut C4950_IDS_PTR: usize = 0;
    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_alloc_job(_pool: usize, out_ptr: usize) {
        let idx = JOB_IDX & 1;
        JOB_IDX += 1;
        wr::<usize>(out_ptr, JOB_SLOTS[idx]);
        wr::<u64>(out_ptr + 0x08, 0x11);
    }
    unsafe extern "win64" fn mock_enqueue(_sched: usize, job: usize) {
        CALL_LOG = CALL_LOG.wrapping_add(rd::<u32>(job) as u64);
    }
    unsafe extern "win64" fn mock_rel_sem(h: usize, cnt: u32, _prev: usize) -> i32 {
        CALL_LOG = CALL_LOG.wrapping_add(h as u64).wrapping_add(cnt as u64);
        1
    }
    unsafe extern "win64" fn mock_11e0(_arena: usize, size: u64, align: u32) -> usize {
        let idx = ALLOC_IDX;
        ALLOC_IDX += 1;
        CALL_LOG = CALL_LOG.wrapping_add(size).wrapping_add(align as u64);
        ALLOC_BUFS[idx]
    }
    unsafe extern "win64" fn mock_5de0(
        out: usize,
        arena: usize,
        grp_cnts: usize,
        cmd_68: usize,
        lut: usize,
        ctx: usize,
    ) -> usize {
        let _ = (grp_cnts, cmd_68, lut, ctx);
        wr::<usize>(out, arena);
        wr::<u64>(out + 0x08, 0xaaaa_0001);
        wr::<u32>(out + 0x10, 2);
        wr::<u32>(out + 0x14, 2);
        wr::<usize>(out + 0x18, arena);
        wr::<u64>(out + 0x20, 0xaaaa_0002);
        wr::<u32>(out + 0x28, 3);
        wr::<u32>(out + 0x2c, 3);
        out
    }
    unsafe extern "win64" fn mock_4950(
        out: usize,
        arena: usize,
        _cmd_68: usize,
        s50_ptr: usize,
        _lut: usize,
        _ctx: usize,
        fut_ptr: usize,
    ) -> usize {
        CALL_LOG = CALL_LOG
            .wrapping_add(rd::<u64>(s50_ptr))
            .wrapping_add(rd::<u32>(fut_ptr + 0x10) as u64);
        wr::<usize>(out, arena);
        wr::<usize>(out + 0x08, C4950_PAIRS_PTR);
        wr::<u32>(out + 0x10, 2);
        wr::<u32>(out + 0x14, 2);
        wr::<usize>(out + 0x18, arena);
        wr::<usize>(out + 0x20, C4950_IDS_PTR);
        wr::<u32>(out + 0x28, 2);
        wr::<u32>(out + 0x2c, 2);
        out
    }
    unsafe extern "win64" fn mock_5680(_sys: usize, handle_ptr: usize) {
        let job = rd::<usize>(handle_ptr);
        let gather_ctx = rd::<usize>(job + 0x38);
        let local_arena = rd::<usize>(job + 0x40);
        // Populate 2 pairs in gather_ctx (+0x18 = len, +0x20 = ptr)
        wr::<u64>(gather_ctx + 0x18, 2);
        wr::<usize>(gather_ctx + 0x20, EXTRA_PAIRS_PTR);
        // Populate local_arena so 0x140f7d880 branch executes
        wr::<u64>(local_arena, 0x5000);
        wr::<u32>(local_arena + 0x08, 1);
        wr::<u64>(local_arena + 0x10, 0x100);
        wr::<u64>(local_arena + 0x18, 0x4000);
    }
    unsafe extern "win64" fn mock_d880(
        arena: usize,
        _insert_pos: usize,
        beg_ptr: usize,
        end_ptr: usize,
    ) {
        let beg = rd::<u64>(beg_ptr);
        let end = rd::<u64>(end_ptr);
        CALL_LOG = CALL_LOG.wrapping_add(end.wrapping_sub(beg));
        // Increment arena block count by 1
        wr::<u32>(arena + 0x08, rd::<u32>(arena + 0x08).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_1520(_keys: u64, _len: u32, want: u64) -> i32 {
        if want == (0x0011 ^ 0x1000) {
            1
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_a3d0(_arena: usize) {
        CALL_LOG = CALL_LOG.wrapping_add(0x10);
    }
    unsafe extern "win64" fn mock_e830(_arena: usize) {
        CALL_LOG = CALL_LOG.wrapping_add(0x20);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc_job as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_6460, mock_enqueue as *const ())?;
    let _m4 = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420c_5de0, mock_5de0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_420c_4950, mock_4950 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_40f7_d880, mock_d880 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_411f_1520, mock_1520 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_420a_a3d0, mock_a3d0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let g_job_va = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_job = rd::<usize>(g_job_va);

    let run = |f: F, swap_blocks: bool| -> ([u64; 34], [u64; 3], [u64; 2], [u64; 4], [u64; 2], u64) {
        let mut job0 = [0u64; 11];
        let mut job1 = [0u64; 11];
        let mut alloc0 = [0u64; 4];
        let mut alloc1 = [0u64; 3]; // grp_cnt_ptr (3 u64s)
        let mut alloc2 = [0u64; 2]; // out_arr (2 u64s)
        let extra_pairs: [u64; 2] = [(10u64 << 32) | 1, (20u64 << 32) | 2];
        let mut c4950_pairs: [u64; 4] = [0x0005, 0x0011, 0x0015, u64::MAX];
        let c4950_ids: [u32; 2] = [10, 20];

        JOB_IDX = 0;
        JOB_SLOTS = [job0.as_mut_ptr() as usize, job1.as_mut_ptr() as usize];
        ALLOC_IDX = 0;
        ALLOC_BUFS = [
            alloc0.as_mut_ptr() as usize,
            alloc1.as_mut_ptr() as usize,
            alloc2.as_mut_ptr() as usize,
        ];
        EXTRA_PAIRS_PTR = extra_pairs.as_ptr() as usize;
        C4950_PAIRS_PTR = c4950_pairs.as_mut_ptr() as usize;
        C4950_IDS_PTR = c4950_ids.as_ptr() as usize;
        CALL_LOG = 0;

        let mut sched_inner = vec![0u8; 0x1020];
        wr::<usize>(sched_inner.as_mut_ptr() as usize + 0x1008, 0x7777);
        let mut sched = [0u8; 0x90];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sched_inner.as_mut_ptr() as usize);
        wr::<usize>(sp + 0x48, 0x2000);
        let mut g_obj = [0u8; 0x20];
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sp);
        wr::<usize>(g_job_va, g_obj.as_mut_ptr() as usize);

        let mut arena_blocks: [u64; 2] = [0x1111, 0x2222];
        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;
        wr::<usize>(ap, arena_blocks.as_mut_ptr() as usize);
        wr::<u32>(ap + 0x08, 1);
        wr::<u64>(ap + 0x10, 0x100);
        wr::<u64>(ap + 0x18, if swap_blocks { 0x200 } else { 0x9000 });

        let mut cmd = [0u8; 0x190];
        let cp = cmd.as_mut_ptr() as usize;
        wr::<u64>(cp + 0x158, 0x11);
        wr::<u64>(cp + 0x160, 0x22);
        wr::<u64>(cp + 0x168, 2);
        wr::<u64>(cp + 0x170, 0x33);
        wr::<u64>(cp + 0x178, 0x44);
        wr::<u64>(cp + 0x180, 0x55);

        let comp_masks: [u64; 4] = [0, 0x1000, 0x2000, 0x3000];
        let mut comp_ctx = [0u64; 4];
        wr::<usize>(comp_ctx.as_mut_ptr() as usize + 0x10, comp_masks.as_ptr() as usize);

        let mut ent_table = [0u8; 25 * 8];
        let etp = ent_table.as_mut_ptr() as usize;
        wr::<u16>(etp + 10 * 8, 7);
        wr::<u32>(etp + 10 * 8 + 4, 101);
        wr::<u16>(etp + 20 * 8, 9);
        wr::<u32>(etp + 20 * 8 + 4, 202);

        let mut storage = [0u8; 0x40];
        let stp = storage.as_mut_ptr() as usize;
        wr::<usize>(stp, comp_ctx.as_ptr() as usize);
        wr::<usize>(stp + 0x30, etp);
        wr::<u32>(stp + 0x38, 25);

        let lut_keys: [u64; 3] = [0, 0x0010, 0x0020];
        let lut_grps: [u16; 3] = [0, 1, 2];
        let mut lut = [0u64; 3];
        let lp = lut.as_mut_ptr() as usize;
        wr::<usize>(lp, lut_keys.as_ptr() as usize);
        wr::<u32>(lp + 0x08, 3);
        wr::<usize>(lp + 0x10, lut_grps.as_ptr() as usize);

        let mut out = [0u64; 34]; // 0x110 bytes
        let op = out.as_mut_ptr() as usize;
        let ret = f(op, ap, cp, stp, lp, 0x9999, 3);
        assert_eq!(ret, op);

        // Normalize pointers in `out` before comparison
        let local_arena_addr = job0[8] as usize;
        out[0] = 0; // param_2
        out[1] = 0; // grp_cnt_ptr
        out[3] = 0; // param_2
        out[4] = 0; // out_arr
        out[6] = 0; // arena
        out[7] = 0; // C4950_PAIRS_PTR
        out[9] = 0; // arena
        out[10] = 0; // C4950_IDS_PTR
        out[12] = 0; // arena
        out[15] = 0; // arena
        out[19] = 0; // EXTRA_PAIRS_PTR
        out[22] = 0; // arena
        out[25] = (out[25] as usize).wrapping_sub(local_arena_addr) as u64;
        out[28] = (out[28] as usize).wrapping_sub(local_arena_addr) as u64;
        out[31] = 0; // param_2
        out[32] = 0; // r12_alloc

        (out, alloc1, alloc2, c4950_pairs, arena_blocks, CALL_LOG)
    };

    for swap_blocks in [false, true] {
        let o = run(f_o, swap_blocks);
        let n = run(f_n, swap_blocks);
        wr::<usize>(g_job_va, s_job);
        crate::diff_eq!(o, n, "1420c3310 swap_blocks={}", swap_blocks);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c5de0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut JOB_IDX: usize = 0;
    static mut JOB_SLOTS: [usize; 8] = [0; 8];
    static mut ALLOC_IDX: usize = 0;
    static mut ALLOC_BUFS: [usize; 4] = [0; 4];
    static mut CALL_LOG: u64 = 0;

    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_alloc_job(_pool: usize, out_ptr: usize) {
        let idx = JOB_IDX & 7;
        JOB_IDX += 1;
        let j = JOB_SLOTS[idx];
        wr::<u8>(j + 0x04, 1);
        wr::<usize>(out_ptr, j);
        wr::<u64>(out_ptr + 0x08, 0);
    }
    unsafe extern "win64" fn mock_enqueue(_sched: usize, job: usize) {
        let vtbl = rd::<usize>(job + 0x10);
        if vtbl == exe::ptr::<u8>(0x1_433d_dd40) as usize {
            // Parallel copy job: copy chunk_size u64s from [job+0x20] to [job+0x18]
            let dst = rd::<usize>(job + 0x18);
            let src = rd::<usize>(job + 0x20);
            let cnt = rd::<usize>(job + 0x28);
            for i in 0..cnt {
                wr::<u64>(dst + i * 8, rd::<u64>(src + i * 8));
            }
        } else if vtbl == exe::ptr::<u8>(0x1_433d_dc80) as usize {
            // Signature hashing job
            let vec_58_ptr = rd::<usize>(job + 0x18);
            let sigs_ptr = rd::<usize>(vec_58_ptr + 0x08);
            let ctx = rd::<usize>(job + 0x20);
            let cnt = rd::<usize>(job + 0x28);
            let slice_ptr = rd::<usize>(job + 0x30);
            let mask_tbl = rd::<usize>(ctx + 0x10);
            for i in 0..cnt {
                let pair = rd::<u64>(slice_ptr + i * 8);
                let comp_id = (pair & 0xffff) as usize;
                let ent_idx = (pair >> 32) as usize;
                let mask = rd::<u64>(mask_tbl + comp_id * 8);
                let addr = sigs_ptr + ent_idx * 8;
                wr::<u64>(addr, rd::<u64>(addr) ^ mask);
            }
        }
    }
    unsafe extern "win64" fn mock_rel_sem(_h: usize, cnt: u32, _prev: usize) -> i32 {
        CALL_LOG = CALL_LOG.wrapping_add(cnt as u64);
        1
    }
    unsafe extern "win64" fn mock_11e0(_arena: usize, size: u64, align: u32) -> usize {
        let idx = ALLOC_IDX;
        ALLOC_IDX += 1;
        CALL_LOG = CALL_LOG.wrapping_add(size).wrapping_add(align as u64);
        ALLOC_BUFS[idx]
    }
    unsafe extern "win64" fn mock_30e0(
        _jc: usize,
        cmd_a0: u64,
        _fn_ptr: usize,
        _r9: usize,
        view_ptr: usize,
    ) {
        let len = rd::<u64>(view_ptr) as usize;
        let ptr = rd::<usize>(view_ptr + 0x08);
        CALL_LOG = CALL_LOG.wrapping_add(cmd_a0).wrapping_add(len as u64);
        // Populate high 32 bits (entity index)
        let ents: [u32; 6] = [0, 0, 1, 1, 1, 2];
        for i in 0..len.min(6) {
            wr::<u32>(ptr + i * 8 + 4, ents[i]);
        }
    }
    unsafe extern "win64" fn mock_3240(
        _jc: usize,
        cmd_b8: u64,
        _fn_ptr: usize,
        _r9: usize,
        view_ptr: usize,
    ) {
        let len = rd::<u64>(view_ptr) as usize;
        let ptr = rd::<usize>(view_ptr + 0x08);
        CALL_LOG = CALL_LOG.wrapping_add(cmd_b8).wrapping_add(len as u64);
        // Populate low 16 bits (component id), including a duplicate at index 0 & 1
        let comps: [u16; 6] = [1, 1, 2, 3, 1, 2];
        for i in 0..len.min(6) {
            wr::<u16>(ptr + i * 8, comps[i]);
        }
    }
    unsafe extern "win64" fn mock_5680(_sys: usize, _jc: usize) {
        CALL_LOG = CALL_LOG.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_f9a0(_ptr: usize, len: u32, _cmp: usize) {
        CALL_LOG = CALL_LOG.wrapping_add(len as u64);
    }
    unsafe extern "win64" fn mock_1520(_keys: u64, _len: u32, sig: u64) -> i32 {
        match sig {
            0x0010 => 0,
            0x0111 => 1,
            0x0100 => 2,
            _ => -1,
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_alloc_job as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_6460, mock_enqueue as *const ())?;
    let _m4 = crate::difftest::IatMock::new(0x1_4264_5580, mock_rel_sem as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411f_30e0, mock_30e0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_411f_3240, mock_3240 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_420a_f9a0, mock_f9a0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_411f_1520, mock_1520 as *const ())?;

    let g_job_va = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_job = rd::<usize>(g_job_va);

    let run = |f: F, num_pairs: u64| -> ([u64; 6], [u64; 6], [u64; 6], [u64; 3], [u64; 3], u64) {
        let mut jobs = [[0u64; 10]; 8];
        let mut buf_r14 = [0u64; 6];
        let mut buf_70 = [0u64; 6];
        let mut buf_sigs = [0u64; 3];

        JOB_IDX = 0;
        for i in 0..8 {
            JOB_SLOTS[i] = jobs[i].as_mut_ptr() as usize;
        }
        ALLOC_IDX = 0;
        ALLOC_BUFS = [
            buf_r14.as_mut_ptr() as usize,
            buf_70.as_mut_ptr() as usize,
            buf_sigs.as_mut_ptr() as usize,
            0,
        ];
        CALL_LOG = 0;

        let mut sched_inner = vec![0u8; 0x1020];
        wr::<usize>(sched_inner.as_mut_ptr() as usize + 0x1008, 0x7777);
        let mut sched = [0u8; 0x90];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sched_inner.as_mut_ptr() as usize);
        wr::<usize>(sp + 0x48, 0x2000);
        let mut g_obj = [0u64; 4];
        g_obj[0] = 1; // 1 worker -> workers_x3 = 3
        wr::<usize>(g_obj.as_mut_ptr() as usize + 0x10, sp);
        wr::<usize>(g_job_va, g_obj.as_mut_ptr() as usize);

        let mut arena = [0u64; 4];
        let ap = arena.as_mut_ptr() as usize;

        let mut grp_cnt_arr = [0u64; 3];
        let mut p3 = [0u64; 3];
        wr::<usize>(p3.as_mut_ptr() as usize + 0x08, grp_cnt_arr.as_mut_ptr() as usize);

        let mut cmd = [0u8; 0xc0];
        let cp = cmd.as_mut_ptr() as usize;
        wr::<u32>(cp, 3); // 3 entities
        wr::<u64>(cp + 0xa0, 0x111);
        wr::<u64>(cp + 0xb0, num_pairs);
        wr::<u64>(cp + 0xb8, 0x222);

        let lut_grps: [u16; 3] = [0, 1, 2];
        let mut lut = [0u64; 3];
        let lp = lut.as_mut_ptr() as usize;
        wr::<u64>(lp, 0x8000);
        wr::<u32>(lp + 0x08, 3);
        wr::<usize>(lp + 0x10, lut_grps.as_ptr() as usize);

        let comp_masks: [u64; 4] = [0, 0x0010, 0x0100, 0x0001];
        let mut comp_ctx = [0u64; 4];
        wr::<usize>(comp_ctx.as_mut_ptr() as usize + 0x10, comp_masks.as_ptr() as usize);

        let mut out = [0u64; 6];
        let op = out.as_mut_ptr() as usize;
        let ret = f(
            op,
            ap,
            p3.as_ptr() as usize,
            cp,
            lp,
            comp_ctx.as_ptr() as usize,
        );
        assert_eq!(ret, op);

        out[0] = 0;
        out[1] = 0;
        out[3] = 0;
        out[4] = 0;
        (out, buf_r14, buf_70, buf_sigs, grp_cnt_arr, CALL_LOG)
    };

    for num_pairs in [3u64, 6] {
        let o = run(f_o, num_pairs);
        let n = run(f_n, num_pairs);
        wr::<usize>(g_job_va, s_job);
        crate::diff_eq!(o, n, "1420c5de0 num_pairs={}", num_pairs);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c7f40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: [[usize; 2]; 16] = [[0; 2]; 16];
    static mut CALL_LEN: usize = 0;

    unsafe extern "win64" fn mock_0730(a: usize, b: usize) {
        let i = CALL_LEN;
        if i < 16 {
            CALL_LOG[i] = [a & 0xfff, b & 0xfff];
            CALL_LEN = i + 1;
        }
    }
    unsafe extern "win64" fn mock_3d40(a: usize, b: usize) {
        let i = CALL_LEN;
        if i < 16 {
            CALL_LOG[i] = [a & 0xfff, b & 0xfff];
            CALL_LEN = i + 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_420b_0730, mock_0730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411f_3d40, mock_3d40 as *const ())?;

    let run = |f: F, empty: bool| -> ([u64; 2], [u64; 2], [u32; 4], [u32; 4], [[usize; 2]; 16], usize) {
        CALL_LOG = [[0; 2]; 16];
        CALL_LEN = 0;
        let mut page2 = [0u8; 0x20];
        let p2_node = page2.as_mut_ptr() as usize;
        wr::<usize>(p2_node, 0);
        wr::<u64>(p2_node + 0x08, 2);
        wr::<u32>(p2_node + 0x10, 100);
        wr::<u32>(p2_node + 0x14, 200);

        let mut page1 = [0u8; 0x20];
        let p1_node = page1.as_mut_ptr() as usize;
        wr::<usize>(p1_node, p2_node);
        wr::<u64>(p1_node + 0x08, 2);
        wr::<u32>(p1_node + 0x10, 10);
        wr::<u32>(p1_node + 0x14, 20);

        let mut dst = [0u8; 0x200];
        let mut src = [0u8; 0x200];
        let dp = dst.as_mut_ptr() as usize;
        let sp = src.as_mut_ptr() as usize;

        wr::<u64>(dp + 0x58, 0x1111);
        wr::<u64>(dp + 0x60, 0x2000);
        wr::<u32>(dp + 0x68, 7);

        wr::<u64>(sp + 0x58, 0x9999);
        wr::<u64>(sp + 0x60, 0x3000);
        wr::<u32>(sp + 0x68, 5);
        if !empty {
            wr::<usize>(sp + 0x108, p1_node);
            wr::<usize>(sp + 0x110, p2_node);
        }

        f(dp, sp);
        let mut log = CALL_LOG;
        for item in log.iter_mut().take(CALL_LEN) {
            item[0] = (item[0] + 0x1000 - (dp & 0xfff)) & 0xfff;
            item[1] = (item[1] + 0x1000 - (sp & 0xfff)) & 0xfff;
        }
        (
            [rd::<u64>(dp + 0x58), rd::<u64>(dp + 0x60) ^ (rd::<u32>(dp + 0x68) as u64)],
            [rd::<u64>(sp + 0x58), rd::<u64>(sp + 0x60) ^ (rd::<u32>(sp + 0x68) as u64)],
            rd::<[u32; 4]>(p1_node + 0x10),
            rd::<[u32; 4]>(p2_node + 0x10),
            log,
            CALL_LEN,
        )
    };

    for empty in [false, true] {
        let o = run(f_o, empty);
        let n = run(f_n, empty);
        crate::diff_eq!(o, n, "1420c7f40 empty={}", empty);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420e9b00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOTS_PTR: [usize; 2] = [0; 2];
    static mut KEYS: [u32; 2] = [0; 2];
    static mut IDX: usize = 0;

    unsafe extern "win64" fn mock_76e0(_p2: usize) {}
    unsafe extern "win64" fn mock_8170(_map: usize, out_iter: usize, key_ptr: usize) {
        let i = IDX & 1;
        IDX += 1;
        KEYS[i] = rd::<u32>(key_ptr);
        let slot_addr = SLOTS_PTR[i];
        wr::<usize>(out_iter, slot_addr);
        wr::<usize>(out_iter + 0x08, slot_addr);
        wr::<u8>(out_iter + 0x10, 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_76e0, mock_76e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411f_8170, mock_8170 as *const ())?;

    let run = |f: F| -> ([[u64; 2]; 2], [u32; 2]) {
        let mut slots = [[0u64; 2]; 2];
        SLOTS_PTR = [slots[0].as_mut_ptr() as usize, slots[1].as_mut_ptr() as usize];
        KEYS = [0; 2];
        IDX = 0;

        let heap_str = b"HeapTestString\0";
        let mut entries = [0u8; 0x88 * 2];
        let ep = entries.as_mut_ptr() as usize;

        // Entry 0: inline string ("AbCd\0"), b3 = 4 (cap_bits = 32), len = 4 (len_bits = 32 <= 32)
        wr::<u32>(ep, (4u32 << 24) | 4);
        wr::<[u8; 5]>(ep + 0x08, *b"AbCd\0");

        // Entry 1: heap string, b3 = 2 (cap_bits = 16), len = 14 (len_bits = 112 > 16)
        let e1 = ep + 0x88;
        wr::<u32>(e1, (2u32 << 24) | 14);
        wr::<usize>(e1 + 0x08, heap_str.as_ptr() as usize);

        let mut p1 = [0u8; 0x80];
        let mut p2 = [0u8; 0x20];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x08, ep);
        wr::<u32>(p2p + 0x10, 2);
        wr::<u64>(p2p + 0x18, 0x5000);

        f(p1.as_mut_ptr() as usize, p2p);
        (slots, KEYS)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420e9b00 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1421319a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF_PTRS: [usize; 2] = [0; 2];
    static mut ALLOC_IDX: usize = 0;
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_c0a0(_bytes: u64, _align: u32) -> usize {
        let idx = ALLOC_IDX & 1;
        ALLOC_IDX += 1;
        BUF_PTRS[idx]
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }
    unsafe extern "win64" fn mock_1150(beg_ptr: usize, end_ptr: usize, cnt: u64, _r9: u32) {
        let beg = rd::<usize>(beg_ptr);
        let end = rd::<usize>(end_ptr);
        let n = ((end.wrapping_sub(beg)) / 4).min(cnt as usize);
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(rd::<u32>(beg + i * 4));
        }
        v.sort_unstable();
        for (i, &val) in v.iter().enumerate() {
            wr::<u32>(beg + i * 4, val);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40f9_1150, mock_1150 as *const ())?;

    let run = |f: F, init_flag: u8| -> ([u64; 2], [u64; 2], [u8; 64], u32) {
        let mut buf0 = [0u32; 64];
        let mut buf1 = [0u32; 64];
        BUF_PTRS = [buf0.as_mut_ptr() as usize, buf1.as_mut_ptr() as usize];
        ALLOC_IDX = 0;
        FREE_COUNT = 0;

        let mut pts_data = [0f32; 64 * 3];
        for i in 0..64 {
            pts_data[i * 3] = 3.0 + (i as f32) * 0.25;
            pts_data[i * 3 + 1] = match i % 3 {
                0 => -2.0,
                1 => 2.5,
                _ => 50.0,
            };
            pts_data[i * 3 + 2] = 4.0;
        }

        let mut flags_arr = [init_flag; 64];
        flags_arr[2] |= 0x02; // set neighbor +1/+2 bit 0x2 to exercise the (bVar13 & 4) downgrade check

        let mut grid_info = [0u8; 0x10];
        let gp = grid_info.as_mut_ptr() as usize;
        wr::<f32>(gp, 1.5);
        wr::<u32>(gp + 0x08, 4); // dim_x = 4
        wr::<u32>(gp + 0x0c, 4); // dim_y = 4

        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, pts_data.as_ptr() as usize);
        wr::<usize>(p1p + 0x18, flags_arr.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, gp);

        let mut bitset2 = [0u64; 2];
        let p2: [usize; 1] = [bitset2.as_mut_ptr() as usize];

        let mut bitset3 = [0u64; 2];
        let mut p3 = [0u64; 3];
        wr::<usize>(p3.as_mut_ptr() as usize, bitset3.as_mut_ptr() as usize);
        wr::<u64>(p3.as_mut_ptr() as usize + 0x10, 4); // max_cap = 4 to force reallocation when >4 corners are pushed

        let p4: [f32; 3] = [2.0, 5.0, 1.0];
        let coords: [u32; 6] = [0, 0, 0, 1, 0, 0];
        let p5: [u64; 2] = [2, coords.as_ptr() as u64];

        f(
            p1p,
            p2.as_ptr() as usize,
            p3.as_mut_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
        );
        (bitset2, bitset3, flags_arr, FREE_COUNT)
    };

    for init_flag in [0u8, 0x04, 0x0a] {
        let o = run(f_o, init_flag);
        let n = run(f_n, init_flag);
        crate::diff_eq!(o, n, "1421319a0 init_flag={:#x}", init_flag);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified_throws(0x1_420c_3310, "unknown_1420c3310", rend::cauldron_hot_w5b::unknown_1420c3310 as *const (), Throws::Caught, rend::cauldron_hot_w5b::diff_unknown_1420c3310),
// Replacement::verified_throws(0x1_420c_5de0, "unknown_1420c5de0", rend::cauldron_hot_w5b::unknown_1420c5de0 as *const (), Throws::Caught, rend::cauldron_hot_w5b::diff_unknown_1420c5de0),
// Replacement::verified(0x1_420c_7f40, "unknown_1420c7f40", rend::cauldron_hot_w5b::unknown_1420c7f40 as *const (), rend::cauldron_hot_w5b::diff_unknown_1420c7f40),
// Replacement::verified(0x1_420e_9b00, "unknown_1420e9b00", rend::cauldron_hot_w5b::unknown_1420e9b00 as *const (), rend::cauldron_hot_w5b::diff_unknown_1420e9b00),
// Replacement::verified_throws(0x1_4213_19a0, "unknown_1421319a0", rend::cauldron_hot_w5b::unknown_1421319a0 as *const (), Throws::Caught, rend::cauldron_hot_w5b::diff_unknown_1421319a0),
