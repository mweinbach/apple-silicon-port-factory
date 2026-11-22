//! Decompiled hot functions shard M (`0x142239b60`, `0x142240080`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn cvttss2si32(v: f32) -> i32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_cvttss_si32(core::arch::x86_64::_mm_set_ss(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        v as i32
    }
}

#[inline(always)]
unsafe fn acquire_rec_srwlock(lock_ptr: usize) {
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_ptr + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_ptr));
        wr::<u32>(lock_ptr + 0x08, tid);
    }
    wr::<u32>(lock_ptr + 0x0c, rd::<u32>(lock_ptr + 0x0c).wrapping_add(1));
}

#[inline(always)]
unsafe fn release_rec_srwlock(lock_ptr: usize) {
    let rc = rd::<u32>(lock_ptr + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_ptr + 0x0c, rc);
    if rc == 0 {
        crate::simd_ops::atomic_xchg_u32((lock_ptr + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_ptr));
    }
}

#[inline(always)]
unsafe fn update_tls_cb(tls_base: usize, cb_idx: isize, src_ptr: *const u8) {
    let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let entry_off = (cb_idx * 16) as usize;
    let copy_len = rd::<i32>(tbl_base.wrapping_add(entry_off + 0x08)) as isize as usize;
    let tls_off = rd::<i32>(tbl_base.wrapping_add(entry_off));
    if tls_off >= 0 {
        let dst = tls_base
            .wrapping_add(0x63d0)
            .wrapping_add(tls_off as isize as usize);
        let _ = exe::call_va!(0x1_4183_acbc, (dst, src_ptr as usize, copy_len) -> usize);
        let dirty_idx = rd::<i32>(tbl_base.wrapping_add(entry_off + 0x0c)) as isize;
        if dirty_idx != -1 {
            let cnt_addr = tls_base
                .wrapping_add(0x463e0)
                .wrapping_add((dirty_idx * 4) as usize);
            let next_cnt = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, next_cnt);
            if next_cnt == -1 {
                wr::<i32>(cnt_addr, 0);
            }
        }
    }
}

#[inline(always)]
unsafe fn call_cb_update(cb_idx_i32: i32, src_ptr: usize) {
    let cb_idx = cb_idx_i32 as isize;
    let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let copy_len = rd::<i32>(tbl_base.wrapping_add((cb_idx * 16) as usize + 0x08)) as i64 as u64;
    exe::call_va!(
        0x1_4209_a900,
        (cb_idx as i64 as u64, 0u64, copy_len, src_ptr)
    );
}

#[inline(always)]
unsafe fn select_inst_buf_handle(mode: i32, res: &[u64; 12], fcb8: usize) -> u64 {
    match mode {
        2 => {
            if res[3] != 0 {
                res[3]
            } else {
                rd::<u64>(fcb8 + 0xa8)
            }
        }
        3 => {
            if res[4] != 0 {
                res[4]
            } else {
                rd::<u64>(fcb8 + 0x98)
            }
        }
        4 => {
            if res[5] != 0 {
                res[5]
            } else {
                rd::<u64>(fcb8 + 0x98)
            }
        }
        6 => {
            if res[6] != 0 {
                res[6]
            } else {
                rd::<u64>(fcb8 + 0xa0)
            }
        }
        7 => {
            if res[7] != 0 {
                res[7]
            } else {
                rd::<u64>(fcb8 + 0xa0)
            }
        }
        _ => rd::<u64>(fcb8 + 0x98),
    }
}

#[inline(always)]
unsafe fn select_tex_handle(mode: i32, rbx_30: usize) -> u64 {
    match mode {
        0 => rd::<u64>(rbx_30 + 0x78),
        5 => rd::<u64>(rbx_30 + 0x90),
        _ => {
            let fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
            rd::<u64>(fcb8 + 0x88)
        }
    }
}

#[inline(always)]
unsafe fn worker_slot_offset(param_1: usize) -> usize {
    let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let slot_ptr = rd::<usize>((exe::ptr::<usize>(0x1_43c4_aa00) as usize).wrapping_add(wid * 8));
    let slot_idx = if slot_ptr != 0 {
        rd::<u32>(slot_ptr) as usize
    } else {
        0
    };
    param_1.wrapping_add(slot_idx.wrapping_mul(0xd8))
}

/// `unknown_142239b60`
/// Original: `0x142239b60` (7516 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_142239b60(param_1: usize, param_2: i32) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, ());
    }
    let r14_ctx = tls0.wrapping_add(0x467e0);

    if param_2 == 0 {
        if param_1 == rd::<usize>(exe::ptr::<usize>(0x1_4397_fef8) as usize)
            && rd::<u8>(exe::ptr::<u8>(0x1_4397_d8a3) as usize) == 0
        {
            if rd::<u8>(tls0 + 0x70) == 0 {
                exe::call_va!(0x1_4183_49c0, ());
            }
            let arg2 = rd::<u64>(tls0 + 0x6308);
            exe::call_va!(0x1_4223_f300, (param_1, arg2));
        }
    } else if param_2 == 2
        && param_1 == rd::<usize>(exe::ptr::<usize>(0x1_4397_fef8) as usize)
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d8a3) as usize) == 0
    {
        exe::call_va!(0x1_4224_0080, (param_1));
    }

    exe::call_va!(0x1_4223_b920, (param_1));

    let c_1_0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);

    for slot_idx in 0..0x13usize {
        let rdx_slot = param_1.wrapping_add(slot_idx.wrapping_mul(0xd8));
        if rd::<u8>(rdx_slot) == 0 {
            continue;
        }

        let rbx_lock = rdx_slot + 0xa8;
        acquire_rec_srwlock(rbx_lock);

        let raw_70 = rd::<u32>(rdx_slot + 0x70);
        let mut batch_cnt: i32 = if raw_70 < 0x3333 {
            raw_70 as i32
        } else {
            0x3333
        };
        let mut batch_start: i32 = 0;

        if batch_cnt > 0 {
            let mut tmp_350 = [0u64; 12];
            let byte_sz = ((batch_cnt as i64) * 0x50) as u64;
            let ret_f290 = exe::call_va!(
                0x1_4205_f290,
                (
                    tmp_350.as_mut_ptr() as usize,
                    byte_sz,
                    0xffff_ffff_ffff_fffeu64
                ) -> usize
            );
            let alloc_res = rd::<[u64; 12]>(ret_f290);
            let dst_base = alloc_res[10] as usize;
            for k in 0..(batch_cnt as usize) {
                let src_base = rd::<usize>(rdx_slot + 0x68).wrapping_add(k * 0x50);
                let dst_ptr = dst_base.wrapping_add(k * 0x50);
                for q in 0..5usize {
                    wr::<u128>(dst_ptr + q * 0x10, rd::<u128>(src_base + q * 0x10));
                }
            }

            let lock_7ee0 = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
            acquire_rec_srwlock(lock_7ee0);
            let obj_370 = exe::call_va!(0x1_411d_a370, (0usize, 0xffff_ffff_ffff_fffeu64) -> usize);
            wr::<u64>(obj_370 + 0x28, rd::<u64>(obj_370 + 0x28).wrapping_sub(1));
            release_rec_srwlock(lock_7ee0);

            let tls_cur = rd::<usize>(exe::tls_pointer());
            let g_init_14e8 = exe::ptr::<i32>(0x1_43cf_14e8) as usize;
            if rd::<i32>(g_init_14e8) > rd::<i32>(tls_cur + 0x74) {
                exe::call_va!(0x1_4183_4a78, (g_init_14e8));
                if rd::<i32>(g_init_14e8) == -1 {
                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_1520) as usize, 0);
                    let mut name_view = [exe::ptr::<u8>(0x1_433f_1460) as u64, 0x1au64];
                    exe::call_va!(
                        0x1_4220_ea70,
                        (
                            exe::ptr::<u8>(0x1_43cf_1528) as usize,
                            name_view.as_mut_ptr() as usize
                        )
                    );
                    wr::<u32>(exe::ptr::<u32>(0x1_43cf_152c) as usize, 10);
                    wr::<u64>(
                        exe::ptr::<u64>(0x1_43cf_1520) as usize,
                        exe::ptr::<u8>(0x1_43cf_1530) as u64,
                    );
                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_1530) as usize, 0);
                    let _ = exe::call_va!(
                        0x1_4183_4d18,
                        (exe::ptr::<u8>(0x1_4262_a3f0) as usize) -> i32
                    );
                    exe::call_va!(0x1_4183_4a0c, (g_init_14e8));
                }
            }

            let cb_idx_1528 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_1528) as usize) as isize;
            let mode_1528 = rd::<i32>(
                (exe::ptr::<i32>(0x1_4385_8590) as usize).wrapping_add((cb_idx_1528 * 4) as usize),
            );
            let fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
            let val_128 = select_inst_buf_handle(mode_1528, &alloc_res, fcb8);
            update_tls_cb(
                tls_cur,
                cb_idx_1528,
                (&val_128 as *const u64) as *const u8,
            );
            let fcb8_after = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
            let val_1530 = select_inst_buf_handle(mode_1528, &alloc_res, fcb8_after);
            wr::<u64>(exe::ptr::<u64>(0x1_43cf_1530) as usize, val_1530);

            let g_init_155c = exe::ptr::<i32>(0x1_43cf_155c) as usize;
            if rd::<i32>(g_init_155c) > rd::<i32>(tls_cur + 0x74) {
                exe::call_va!(0x1_4183_4a78, (g_init_155c));
                if rd::<i32>(g_init_155c) == -1 {
                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_1508) as usize, 0);
                    let mut name_view = [exe::ptr::<u8>(0x1_433f_1430) as u64, 0x29u64];
                    exe::call_va!(
                        0x1_4220_ea70,
                        (
                            exe::ptr::<u8>(0x1_43cf_1510) as usize,
                            name_view.as_mut_ptr() as usize
                        )
                    );
                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_1514) as usize, 0x15);
                    wr::<u64>(
                        exe::ptr::<u64>(0x1_43cf_1508) as usize,
                        exe::ptr::<u8>(0x1_43cf_1518) as u64,
                    );
                    wr::<u32>(exe::ptr::<u32>(0x1_43cf_151c) as usize, 0);
                    let _ = exe::call_va!(
                        0x1_4183_4d18,
                        (exe::ptr::<u8>(0x1_4262_a3e0) as usize) -> i32
                    );
                    exe::call_va!(0x1_4183_4a0c, (g_init_155c));
                }
            }

            let pair_74: [u32; 2] = [alloc_res[11] as u32, 0x50];
            let cb_idx_1510 = rd::<i32>(exe::ptr::<i32>(0x1_43cf_1510) as usize) as isize;
            update_tls_cb(
                tls_cur,
                cb_idx_1510,
                pair_74.as_ptr() as *const u8,
            );
            wr::<[u32; 2]>(exe::ptr::<u32>(0x1_43cf_1518) as usize, pair_74);
        }

        let mut cmd_idx = 0usize;
        while cmd_idx < (rd::<u32>(rdx_slot + 0x80) as usize) {
            let rdi = rd::<usize>(rdx_slot + 0x78).wrapping_add(cmd_idx.wrapping_mul(0xd0));
            let cmd_kind = rd::<i32>(rdi) as u32;
            match cmd_kind {
                0 => {
                    exe::call_va!(
                        0x1_4206_f4a0,
                        (
                            r14_ctx,
                            rd::<u16>(rdi + 0x08),
                            rd::<u16>(rdi + 0x0a),
                            rd::<u16>(rdi + 0x0c),
                            rd::<u16>(rdi + 0x0e)
                        )
                    );
                }
                1 => {
                    let mat_140 = rd::<[u32; 16]>(rdi + 0x08);
                    let cb_idx = rd::<i32>(param_1 + 0x1018) as isize;
                    let tls_base = rd::<usize>(exe::tls_pointer());
                    update_tls_cb(tls_base, cb_idx, mat_140.as_ptr() as *const u8);
                    wr::<[u32; 16]>(param_1 + 0x1020, mat_140);
                }
                2 => {
                    call_cb_update(rd::<i32>(param_1 + 0x1068), rdi + 0x08);
                    wr::<u32>(param_1 + 0x1070, rd::<u32>(rdi + 0x08));
                }
                3 => {
                    let v4 = rd::<[u32; 4]>(rdi + 0x08);
                    let cb_idx = rd::<i32>(param_1 + 0x1080) as isize;
                    let tls_base = rd::<usize>(exe::tls_pointer());
                    update_tls_cb(tls_base, cb_idx, v4.as_ptr() as *const u8);
                    wr::<[u32; 4]>(param_1 + 0x1088, v4);
                }
                4 => {
                    call_cb_update(rd::<i32>(param_1 + 0x1220), rdi + 0x08);
                    wr::<u32>(param_1 + 0x1228, rd::<u32>(rdi + 0x08));

                    call_cb_update(rd::<i32>(param_1 + 0x1238), rdi + 0x0c);
                    wr::<u32>(param_1 + 0x1240, rd::<u32>(rdi + 0x0c));

                    call_cb_update(rd::<i32>(param_1 + 0x1250), rdi + 0x10);
                    wr::<u32>(param_1 + 0x1258, rd::<u32>(rdi + 0x10));

                    call_cb_update(rd::<i32>(param_1 + 0x1268), rdi + 0x14);
                    wr::<u32>(param_1 + 0x1270, rd::<u32>(rdi + 0x14));

                    let v4_a = rd::<[u32; 4]>(rdi + 0x18);
                    let cb_idx_a = rd::<i32>(param_1 + 0x1280) as isize;
                    let tls_base_a = rd::<usize>(exe::tls_pointer());
                    update_tls_cb(tls_base_a, cb_idx_a, v4_a.as_ptr() as *const u8);
                    wr::<[u32; 4]>(param_1 + 0x1288, v4_a);

                    let v4_b = rd::<[u32; 4]>(rdi + 0x28);
                    let cb_idx_b = rd::<i32>(param_1 + 0x12a0) as isize;
                    let tls_base_b = rd::<usize>(exe::tls_pointer());
                    update_tls_cb(tls_base_b, cb_idx_b, v4_b.as_ptr() as *const u8);
                    wr::<[u32; 4]>(param_1 + 0x12a8, v4_b);
                }
                5 => {
                    if rd::<i32>(rdi + 0x08) == param_2 {
                        let ptr_40 = rd::<usize>(rdi + 0x40);
                        let rbx_30 = rd::<usize>(rdi + 0x30);
                        let idx_20 = rd::<i32>(rdi + 0x20) as isize;
                        let fef0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fef0) as usize);
                        let rdx_90 =
                            rd::<u64>(fef0.wrapping_add((idx_20 * 8) as usize).wrapping_add(0x90));
                        if rd::<u64>(r14_ctx + 0xaf8) != rdx_90 {
                            let new_af8 = if rdx_90 != 0 {
                                rdx_90
                            } else {
                                rd::<u64>(exe::ptr::<u64>(0x1_4397_fd30) as usize)
                            };
                            wr::<u64>(r14_ctx + 0xaf8, new_af8);
                            wr::<u8>(r14_ctx + 0xa80, 0);
                        }

                        let rcx_fbc = if param_2 == 0 {
                            let idx_24 = rd::<i32>(rdi + 0x24) as isize;
                            let fef0_2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fef0) as usize);
                            let rdx_c0 = rd::<u64>(
                                fef0_2
                                    .wrapping_add((idx_24 * 8) as usize)
                                    .wrapping_add(0xc0),
                            );
                            if rd::<u64>(r14_ctx + 0xb00) != rdx_c0 {
                                wr::<u64>(r14_ctx + 0xb00, rdx_c0);
                                wr::<u8>(r14_ctx + 0xa80, 0);
                            }
                            rd::<usize>(exe::ptr::<usize>(0x1_4397_fbc8) as usize)
                        } else {
                            let fd00 = rd::<u64>(exe::ptr::<u64>(0x1_4397_fd00) as usize);
                            if rd::<u64>(r14_ctx + 0xb00) != fd00 {
                                wr::<u64>(r14_ctx + 0xb00, fd00);
                                wr::<u8>(r14_ctx + 0xa80, 0);
                            }
                            rd::<usize>(exe::ptr::<usize>(0x1_4397_fbc0) as usize)
                        };

                        let rdx_b10 = rd::<usize>(r14_ctx + 0xb10);
                        if rdx_b10 == 0
                            || (rcx_fbc != 0
                                && (rd::<u32>(rdx_b10) != rd::<u32>(rcx_fbc)
                                    || rd::<f32>(rdx_b10 + 0x04) != rd::<f32>(rcx_fbc + 0x04)
                                    || rd::<f32>(rdx_b10 + 0x08) != rd::<f32>(rcx_fbc + 0x08)))
                        {
                            wr::<u8>(r14_ctx + 0xa80, 0);
                            wr::<usize>(r14_ctx + 0xb10, rcx_fbc);
                        }

                        if rbx_30 != 0 {
                            let kind_51 = rd::<i8>(rbx_30 + 0x51);
                            if kind_51 == 0 {
                                let cb_idx = rd::<i32>(param_1 + 0x10a0) as isize;
                                let mode = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx * 4) as usize),
                                );
                                let pair = [select_tex_handle(mode, rbx_30), rbx_30 as u64];
                                let tls_base = rd::<usize>(exe::tls_pointer());
                                update_tls_cb(tls_base, cb_idx, pair.as_ptr() as *const u8);
                                let cb_idx2 = rd::<i32>(param_1 + 0x10a0) as isize;
                                let mode2 = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx2 * 4) as usize),
                                );
                                wr::<usize>(param_1 + 0x10b0, rbx_30);
                                wr::<u64>(param_1 + 0x10a8, select_tex_handle(mode2, rbx_30));
                            } else if kind_51 == 1 {
                                let cb_idx_i32 = rd::<i32>(param_1 + 0x10c0);
                                let cb_idx = cb_idx_i32 as isize;
                                let mode = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx * 4) as usize),
                                );
                                let mut pair = [select_tex_handle(mode, rbx_30), rbx_30 as u64];
                                let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
                                let copy_len = rd::<i32>(
                                    tbl_base.wrapping_add((cb_idx * 16) as usize + 0x08),
                                ) as i64 as u64;
                                exe::call_va!(
                                    0x1_4209_a900,
                                    (
                                        (cb_idx_i32 as u32) as u64,
                                        0u64,
                                        copy_len,
                                        pair.as_mut_ptr() as usize
                                    )
                                );
                                let cb_idx2 = rd::<i32>(param_1 + 0x10c0) as isize;
                                let mode2 = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx2 * 4) as usize),
                                );
                                wr::<usize>(param_1 + 0x10d0, rbx_30);
                                wr::<u64>(param_1 + 0x10c8, select_tex_handle(mode2, rbx_30));
                            } else if kind_51 == 2 {
                                let cb_idx_i32 = rd::<i32>(param_1 + 0x10e0);
                                let cb_idx = cb_idx_i32 as isize;
                                let mode = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx * 4) as usize),
                                );
                                let mut pair = [select_tex_handle(mode, rbx_30), rbx_30 as u64];
                                let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
                                let copy_len = rd::<i32>(
                                    tbl_base.wrapping_add((cb_idx * 16) as usize + 0x08),
                                ) as i64 as u64;
                                exe::call_va!(
                                    0x1_4209_a900,
                                    (
                                        (cb_idx_i32 as u32) as u64,
                                        0u64,
                                        copy_len,
                                        pair.as_mut_ptr() as usize
                                    )
                                );
                                let cb_idx2 = rd::<i32>(param_1 + 0x10e0) as isize;
                                let mode2 = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx2 * 4) as usize),
                                );
                                wr::<usize>(param_1 + 0x10f0, rbx_30);
                                wr::<u64>(param_1 + 0x10e8, select_tex_handle(mode2, rbx_30));
                            }
                        }

                        let idx_28 = rd::<i32>(rdi + 0x28) as isize;
                        let fef0_3 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fef0) as usize);
                        let ptr_d8 =
                            rd::<usize>(fef0_3.wrapping_add((idx_28 * 8) as usize).wrapping_add(0xd8));
                        let val_118 = if ptr_d8 != 0 { rd::<u64>(ptr_d8) } else { 0 };
                        let cb_idx_1100 = rd::<i32>(param_1 + 0x1100) as isize;
                        let tls_base = rd::<usize>(exe::tls_pointer());
                        update_tls_cb(
                            tls_base,
                            cb_idx_1100,
                            (&val_118 as *const u64) as *const u8,
                        );
                        wr::<u64>(param_1 + 0x1108, val_118);

                        let dim_y = rd::<i32>(rdi + 0x3c);
                        let inv_y = if dim_y > 0 {
                            c_1_0 / (dim_y as f32)
                        } else {
                            0.0f32
                        };
                        let dim_x = rd::<i32>(rdi + 0x38);
                        let inv_x = if dim_x > 0 {
                            c_1_0 / (dim_x as f32)
                        } else {
                            0.0f32
                        };
                        let mut inv_pair: [f32; 2] = [inv_x, inv_y];
                        call_cb_update(
                            rd::<i32>(param_1 + 0x1118),
                            inv_pair.as_mut_ptr() as usize,
                        );
                        wr::<[f32; 2]>(param_1 + 0x1120, inv_pair);

                        call_cb_update(rd::<i32>(param_1 + 0x1148), rdi + 0x48);
                        for q in 0..4usize {
                            wr::<u128>(
                                param_1 + 0x1150 + q * 0x10,
                                rd::<u128>(rdi + 0x48 + q * 0x10),
                            );
                        }

                        call_cb_update(rd::<i32>(param_1 + 0x1198), rdi + 0x88);
                        wr::<u128>(param_1 + 0x11a0, rd::<u128>(rdi + 0x88));

                        call_cb_update(rd::<i32>(param_1 + 0x11b8), rdi + 0x98);
                        wr::<u128>(param_1 + 0x11c0, rd::<u128>(rdi + 0x98));

                        call_cb_update(rd::<i32>(param_1 + 0x11d8), rdi + 0xa8);
                        wr::<u32>(param_1 + 0x11e0, rd::<u32>(rdi + 0xa8));

                        let tls_cur = rd::<usize>(exe::tls_pointer());
                        let g_init_14e4 = exe::ptr::<i32>(0x1_43cf_14e4) as usize;
                        if rd::<i32>(g_init_14e4) > rd::<i32>(tls_cur + 0x74) {
                            exe::call_va!(0x1_4183_4a78, (g_init_14e4));
                            if rd::<i32>(g_init_14e4) == -1 {
                                wr::<u64>(exe::ptr::<u64>(0x1_43cf_14f0) as usize, 0);
                                let mut name_view =
                                    [exe::ptr::<u8>(0x1_433f_14a8) as u64, 0x13u64];
                                exe::call_va!(
                                    0x1_4220_ea70,
                                    (
                                        exe::ptr::<u8>(0x1_43cf_14f8) as usize,
                                        name_view.as_mut_ptr() as usize
                                    )
                                );
                                wr::<u32>(exe::ptr::<u32>(0x1_43cf_14fc) as usize, 0x14);
                                wr::<u64>(
                                    exe::ptr::<u64>(0x1_43cf_14f0) as usize,
                                    exe::ptr::<u8>(0x1_43cf_1500) as u64,
                                );
                                let _ = exe::call_va!(
                                    0x1_4183_4d18,
                                    (exe::ptr::<u8>(0x1_4262_a3d0) as usize) -> i32
                                );
                                exe::call_va!(0x1_4183_4a0c, (g_init_14e4));
                            }
                        }

                        call_cb_update(
                            rd::<i32>(exe::ptr::<i32>(0x1_43cf_14f8) as usize),
                            rdi + 0xb4,
                        );
                        wr::<u32>(
                            exe::ptr::<u32>(0x1_43cf_1500) as usize,
                            rd::<u32>(rdi + 0xb4),
                        );

                        call_cb_update(rd::<i32>(param_1 + 0x11f0), rdi + 0xac);
                        wr::<u32>(param_1 + 0x11f8, rd::<u32>(rdi + 0xac));

                        call_cb_update(rd::<i32>(param_1 + 0x1208), rdi + 0xb0);
                        wr::<u32>(param_1 + 0x1210, rd::<u32>(rdi + 0xb0));

                        if rd::<u64>(rdi + 0xb8) == 0 {
                            exe::call_va!(0x1_4224_ded0, (ptr_40));
                            let raw_cnt = rd::<i32>(rdi + 0x14);
                            let prim_type = rd::<i32>(rdi + 0x0c);
                            let prim_cnt: i32 = match prim_type {
                                1 => raw_cnt,
                                2 => raw_cnt / 2,
                                4 => raw_cnt / 3,
                                5 => raw_cnt.wrapping_sub(2),
                                _ => 0,
                            };

                            let fef0_base = rd::<usize>(exe::ptr::<usize>(0x1_4397_fef0) as usize);
                            let fef0_0 = rd::<usize>(fef0_base);
                            if fef0_0 == 0 {
                                wr::<usize>(r14_ctx + 0xb18, 0);
                                wr::<u64>(r14_ctx + 0xb20, 0);
                                wr::<u64>(r14_ctx + 0x48, 0);
                                wr::<u8>(r14_ctx + 0xa80, 0);
                            } else {
                                let target_id = rd::<u32>(fef0_0).wrapping_add(2) as u64;
                                if rd::<u64>(r14_ctx + 0xb20) != target_id {
                                    wr::<usize>(r14_ctx + 0xb18, fef0_0);
                                    let target_id2 = rd::<u32>(fef0_0).wrapping_add(2) as u64;
                                    wr::<u64>(r14_ctx + 0xb20, target_id2);
                                    wr::<u64>(r14_ctx + 0x48, 0);
                                    wr::<u8>(r14_ctx + 0xa80, 0);
                                }
                            }

                            let vtx_off = ((rd::<i32>(rdi + 0x10) as isize) << 5) as usize;
                            let vtx_ptr = rd::<usize>(rdx_slot + 0x88).wrapping_add(vtx_off);
                            exe::call_va!(
                                0x1_4205_ab00,
                                (
                                    r14_ctx,
                                    rd::<u32>(rdi + 0x0c),
                                    prim_cnt as u32,
                                    vtx_ptr
                                )
                            );
                        } else {
                            let inst_start = rd::<i32>(rdi + 0x18);
                            let inst_count = rd::<i32>(rdi + 0x1c);
                            if inst_start < batch_start
                                || inst_start.wrapping_add(inst_count)
                                    > batch_start.wrapping_add(batch_cnt)
                            {
                                let rem = rd::<i32>(rdx_slot + 0x70).wrapping_sub(inst_start);
                                batch_cnt = if rem < 0x3333 { rem } else { 0x3333 };
                                let byte_sz = ((batch_cnt as i64) * 0x50) as u64;
                                let src_inst = rd::<usize>(rdx_slot + 0x68)
                                    .wrapping_add(((inst_start as isize) * 0x50) as usize);
                                let mut tmp_3b0 = [0u64; 24];
                                let ret_eec0 = exe::call_va!(
                                    0x1_4205_eec0,
                                    (tmp_3b0.as_mut_ptr() as usize, byte_sz, src_inst) -> usize
                                );
                                let sub_res = rd::<[u64; 12]>(ret_eec0);

                                let g_init_14e0 = exe::ptr::<i32>(0x1_43cf_14e0) as usize;
                                if rd::<i32>(g_init_14e0) > rd::<i32>(tls_cur + 0x74) {
                                    exe::call_va!(0x1_4183_4a78, (g_init_14e0));
                                    if rd::<i32>(g_init_14e0) == -1 {
                                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_14c8) as usize, 0);
                                        let mut name_view = [
                                            exe::ptr::<u8>(0x1_433f_1460) as u64,
                                            0x1au64,
                                        ];
                                        exe::call_va!(
                                            0x1_4220_ea70,
                                            (
                                                exe::ptr::<u8>(0x1_43cf_14d0) as usize,
                                                name_view.as_mut_ptr() as usize
                                            )
                                        );
                                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_14d4) as usize, 10);
                                        wr::<u64>(
                                            exe::ptr::<u64>(0x1_43cf_14c8) as usize,
                                            exe::ptr::<u8>(0x1_43cf_14d8) as u64,
                                        );
                                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_14d8) as usize, 0);
                                        let _ = exe::call_va!(
                                            0x1_4183_4d18,
                                            (exe::ptr::<u8>(0x1_4262_a3c0) as usize) -> i32
                                        );
                                        exe::call_va!(0x1_4183_4a0c, (g_init_14e0));
                                    }
                                }

                                let cb_idx_14d0 =
                                    rd::<i32>(exe::ptr::<i32>(0x1_43cf_14d0) as usize) as isize;
                                let mode_14d0 = rd::<i32>(
                                    (exe::ptr::<i32>(0x1_4385_8590) as usize)
                                        .wrapping_add((cb_idx_14d0 * 4) as usize),
                                );
                                let fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
                                let val_120 = select_inst_buf_handle(mode_14d0, &sub_res, fcb8);
                                update_tls_cb(
                                    tls_cur,
                                    cb_idx_14d0,
                                    (&val_120 as *const u64) as *const u8,
                                );
                                let val_14d8 = select_inst_buf_handle(mode_14d0, &sub_res, fcb8);
                                wr::<u64>(exe::ptr::<u64>(0x1_43cf_14d8) as usize, val_14d8);

                                let g_init_1538 = exe::ptr::<i32>(0x1_43cf_1538) as usize;
                                if rd::<i32>(g_init_1538) > rd::<i32>(tls_cur + 0x74) {
                                    exe::call_va!(0x1_4183_4a78, (g_init_1538));
                                    if rd::<i32>(g_init_1538) == -1 {
                                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_1560) as usize, 0);
                                        let mut name_view = [
                                            exe::ptr::<u8>(0x1_433f_1430) as u64,
                                            0x29u64,
                                        ];
                                        exe::call_va!(
                                            0x1_4220_ea70,
                                            (
                                                exe::ptr::<u8>(0x1_43cf_1568) as usize,
                                                name_view.as_mut_ptr() as usize
                                            )
                                        );
                                        wr::<u64>(exe::ptr::<u64>(0x1_43cf_156c) as usize, 0x15);
                                        wr::<u64>(
                                            exe::ptr::<u64>(0x1_43cf_1560) as usize,
                                            exe::ptr::<u8>(0x1_43cf_1570) as u64,
                                        );
                                        wr::<u32>(exe::ptr::<u32>(0x1_43cf_1574) as usize, 0);
                                        let _ = exe::call_va!(
                                            0x1_4183_4d18,
                                            (exe::ptr::<u8>(0x1_4262_a3b0) as usize) -> i32
                                        );
                                        exe::call_va!(0x1_4183_4a0c, (g_init_1538));
                                    }
                                }

                                let mut pair_a0: [u32; 2] = [sub_res[11] as u32, 0x50];
                                call_cb_update(
                                    rd::<i32>(exe::ptr::<i32>(0x1_43cf_1568) as usize),
                                    pair_a0.as_mut_ptr() as usize,
                                );
                                wr::<[u32; 2]>(
                                    exe::ptr::<u32>(0x1_43cf_1570) as usize,
                                    pair_a0,
                                );
                                batch_start = inst_start;
                            }

                            let g_init_1578 = exe::ptr::<i32>(0x1_43cf_1578) as usize;
                            if rd::<i32>(g_init_1578) > rd::<i32>(tls_cur + 0x74) {
                                exe::call_va!(0x1_4183_4a78, (g_init_1578));
                                if rd::<i32>(g_init_1578) == -1 {
                                    wr::<u64>(exe::ptr::<u64>(0x1_43cf_1540) as usize, 0);
                                    let mut name_view =
                                        [exe::ptr::<u8>(0x1_433f_1480) as u64, 0x20u64];
                                    exe::call_va!(
                                        0x1_4220_ea70,
                                        (
                                            exe::ptr::<u8>(0x1_43cf_1548) as usize,
                                            name_view.as_mut_ptr() as usize
                                        )
                                    );
                                    wr::<u32>(exe::ptr::<u32>(0x1_43cf_154c) as usize, 0x10);
                                    wr::<u64>(
                                        exe::ptr::<u64>(0x1_43cf_1540) as usize,
                                        exe::ptr::<u8>(0x1_43cf_1550) as u64,
                                    );
                                    let _ = exe::call_va!(
                                        0x1_4183_4d18,
                                        (exe::ptr::<u8>(0x1_4262_a3a0) as usize) -> i32
                                    );
                                    exe::call_va!(0x1_4183_4a0c, (g_init_1578));
                                }
                            }

                            let rel_start = inst_start.wrapping_sub(batch_start);
                            let cb_idx_1548 =
                                rd::<i32>(exe::ptr::<i32>(0x1_43cf_1548) as usize) as isize;
                            update_tls_cb(
                                tls_cur,
                                cb_idx_1548,
                                (&rel_start as *const i32) as *const u8,
                            );
                            wr::<i32>(exe::ptr::<i32>(0x1_43cf_1550) as usize, rel_start);

                            let flag_97 = rd::<u8>(ptr_40 + 0x97);
                            if rd::<u8>(tls_cur + 0x70) == 0 {
                                exe::call_va!(0x1_4183_49c0, ());
                            }
                            exe::call_va!(
                                0x1_4206_f680,
                                (
                                    tls_cur + 0x467e0,
                                    ptr_40 + 0x30,
                                    ptr_40 + 0x60,
                                    flag_97
                                )
                            );

                            let idx_b8 = rd::<u64>(rdi + 0xb8) as usize;
                            let tbl_9490 = exe::ptr::<usize>(0x1_43c4_9490) as usize;
                            if rd::<usize>(tbl_9490.wrapping_add(idx_b8 * 8)) == 0 {
                                let new_buf = exe::call_va!(0x1_411e_9610, () -> usize);
                                let idx_b8_2 = rd::<u64>(rdi + 0xb8) as usize;
                                wr::<usize>(tbl_9490.wrapping_add(idx_b8_2 * 8), new_buf);
                                let cnt_14 = rd::<i32>(rdi + 0x14);
                                exe::call_va!(0x1_4209_d670, (new_buf));
                                exe::call_va!(
                                    0x1_4207_5af0,
                                    (
                                        ((cnt_14 as i64) * 12) as u64,
                                        12u64,
                                        0u8,
                                        3u32,
                                        0u32,
                                        0u64,
                                        new_buf
                                    )
                                );
                                wr::<u32>(new_buf + 0x50, 4);
                                let idx_b8_3 = rd::<u64>(rdi + 0xb8) as usize;
                                let cur_buf = rd::<usize>(tbl_9490.wrapping_add(idx_b8_3 * 8));
                                let map_ptr = exe::call_va!(
                                    0x1_4209_c560,
                                    (cur_buf, 0u64, rd::<u64>(cur_buf + 0x10), 2u32) -> usize
                                );
                                let cnt_loop = rd::<i32>(rdi + 0x14);
                                if cnt_loop > 0 {
                                    for k in 0..(cnt_loop as usize) {
                                        let src_idx =
                                            rd::<i32>(rdi + 0x10).wrapping_add(k as i32) as isize;
                                        let src_98 = rd::<usize>(rdx_slot + 0x98)
                                            .wrapping_add((src_idx * 12) as usize);
                                        let dst_p = map_ptr.wrapping_add(k * 12);
                                        wr::<u64>(dst_p, rd::<u64>(src_98));
                                        wr::<u32>(dst_p + 0x08, rd::<u32>(src_98 + 0x08));
                                    }
                                }
                                let idx_b8_4 = rd::<u64>(rdi + 0xb8) as usize;
                                let cur_buf2 = rd::<usize>(tbl_9490.wrapping_add(idx_b8_4 * 8));
                                exe::call_va!(0x1_4209_d6f0, (cur_buf2, 0u32));
                            }

                            let fef0_base = rd::<usize>(exe::ptr::<usize>(0x1_4397_fef0) as usize);
                            let fef0_1 = rd::<usize>(fef0_base + 0x08);
                            if fef0_1 == 0 {
                                wr::<usize>(r14_ctx + 0xb18, 0);
                                wr::<u64>(r14_ctx + 0xb20, 0);
                                wr::<u64>(r14_ctx + 0x48, 0);
                                wr::<u8>(r14_ctx + 0xa80, 0);
                            } else {
                                let target_id = rd::<u32>(fef0_1).wrapping_add(2) as u64;
                                if rd::<u64>(r14_ctx + 0xb20) != target_id {
                                    wr::<usize>(r14_ctx + 0xb18, fef0_1);
                                    let target_id2 = rd::<u32>(fef0_1).wrapping_add(2) as u64;
                                    wr::<u64>(r14_ctx + 0xb20, target_id2);
                                    wr::<u64>(r14_ctx + 0x48, 0);
                                    wr::<u8>(r14_ctx + 0xa80, 0);
                                }
                            }

                            let idx_b8_5 = rd::<u64>(rdi + 0xb8) as usize;
                            let vbuf = rd::<usize>(tbl_9490.wrapping_add(idx_b8_5 * 8));
                            if vbuf != 0 {
                                let r8_0 = rd::<u64>(vbuf);
                                let edx_10 = rd::<u32>(vbuf + 0x10);
                                let r9d_b28 = rd::<i32>(r14_ctx + 0xb28);
                                if r9d_b28 <= 0
                                    || rd::<u64>(r14_ctx + 0xb30) != r8_0
                                    || rd::<u32>(r14_ctx + 0xb38) != edx_10
                                    || rd::<u32>(r14_ctx + 0xb3c) != 0x0c
                                {
                                    wr::<u8>(r14_ctx + 0xb2c, 1);
                                    let new_b28 = if r9d_b28 >= 1 { r9d_b28 } else { 1 };
                                    wr::<i32>(r14_ctx + 0xb28, new_b28);
                                    wr::<u64>(r14_ctx + 0xb30, r8_0);
                                    wr::<u32>(r14_ctx + 0xb38, edx_10);
                                    wr::<u32>(r14_ctx + 0xb3c, 0x0c);
                                }
                            } else if rd::<u64>(r14_ctx + 0xb30) != 0 {
                                wr::<u8>(r14_ctx + 0xb2c, 1);
                                wr::<u64>(r14_ctx + 0xb30, 0);
                                wr::<u64>(r14_ctx + 0xb38, 0);
                            }

                            let raw_cnt = rd::<i32>(rdi + 0x14);
                            let prim_type = rd::<i32>(rdi + 0x0c);
                            let prim_cnt: i32 = match prim_type {
                                1 => raw_cnt,
                                2 => raw_cnt / 2,
                                4 => raw_cnt / 3,
                                5 => raw_cnt.wrapping_sub(2),
                                _ => 0,
                            };

                            let has_pso = if rd::<u64>(r14_ctx + 0x48) != 0 {
                                true
                            } else {
                                let rcx_c48 = rd::<usize>(r14_ctx + 0xc48);
                                if rcx_c48 != 0 {
                                    let r8_b18 = rd::<usize>(r14_ctx + 0xb18);
                                    let edx_id = rd::<u32>(r8_b18).wrapping_add(2);
                                    let pso = exe::call_va!(
                                        0x1_411d_ce60,
                                        (rcx_c48, edx_id, r8_b18) -> u64
                                    );
                                    wr::<u64>(r14_ctx + 0x48, pso);
                                    pso != 0
                                } else {
                                    false
                                }
                            };

                            if has_pso {
                                let vtx_cnt: i32 = match prim_type {
                                    1 => prim_cnt,
                                    2 => prim_cnt.wrapping_mul(2),
                                    4 => prim_cnt.wrapping_mul(3),
                                    5 => prim_cnt.wrapping_add(2),
                                    _ => 0,
                                };
                                exe::call_va!(0x1_4206_f3a0, (r14_ctx, prim_type as u32));
                                let rax_f290 = exe::call_va!(0x1_4206_f290, (r14_ctx) -> u64);
                                if rd::<u8>(r14_ctx + 0xa80) != 0
                                    || exe::call_va!(0x1_4207_0fb0, (rax_f290, r14_ctx) -> u8) != 0
                                {
                                    let fc58 =
                                        rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
                                    exe::call_va!(
                                        0x1_4207_1360,
                                        (r14_ctx, rd::<u64>(fc58 + 0x20))
                                    );
                                    let mut cmd_pair = [0usize; 2];
                                    exe::call_va!(
                                        0x1_411d_5b00,
                                        (cmd_pair.as_mut_ptr() as usize)
                                    );
                                    let cmd_list = cmd_pair[1];
                                    let vtbl = rd::<usize>(cmd_list);
                                    exe::call_va!(
                                        rd::<usize>(vtbl + 0x60),
                                        (
                                            cmd_list,
                                            vtx_cnt as u32,
                                            inst_count as u32,
                                            0u32,
                                            0u32
                                        )
                                    );
                                    if cmd_pair[0] != 0 {
                                        release_rec_srwlock(cmd_pair[0]);
                                    }
                                }
                            }
                        }
                    }
                }
                6 => {
                    let fn_ctx = rd::<u64>(rdi + 0xc0);
                    let fn_ptr = rd::<u64>(rdi + 0xc8);
                    if fn_ctx != 0 || fn_ptr != 0 {
                        exe::call_va!(fn_ptr, (fn_ctx));
                    }
                    exe::call_va!(0x1_4223_b920, (param_1));
                }
                7 => {
                    let arg2 = rd::<u64>(rdi + 0x08);
                    if arg2 != 0 {
                        let arg1 = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff18) as usize);
                        exe::call_va!(0x1_4225_8080, (arg1, arg2));
                    }
                }
                _ => {}
            }
            cmd_idx = cmd_idx.wrapping_add(1);
        }

        release_rec_srwlock(rbx_lock);
    }
}

/// `unknown_142240080`
/// Original: `0x142240080` (2244 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_142240080(param_1: usize) {
    let c_1_0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize);
    let c_2_0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6abc) as usize);
    let c_neg2_0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_70a0) as usize);
    let c_neg1_0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7080) as usize);
    let c_255 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eec) as usize);
    let row2 = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7a60) as usize);

    let mut local_res10: u32 = 0;

    for outer_idx in 0..0x40usize {
        let r15_off = outer_idx.wrapping_mul(0x158);
        let lock_rbx = (exe::ptr::<u8>(0x1_43c1_6ac0) as usize).wrapping_add(r15_off);
        acquire_rec_srwlock(lock_rbx);

        let mut rdi_ptr = (exe::ptr::<u8>(0x1_43c1_6a10) as usize).wrapping_add(r15_off);
        for _ in 0..3usize {
            let mut mat_f8: [u32; 16] = [
                c_2_0,
                0,
                0,
                0,
                0,
                c_neg2_0,
                0,
                0,
                row2[0],
                row2[1],
                row2[2],
                row2[3],
                c_neg1_0,
                c_1_0,
                0,
                c_1_0,
            ];
            exe::call_va!(0x1_4223_dbf0, (param_1, mat_f8.as_mut_ptr() as usize));

            let s0 = worker_slot_offset(param_1);
            wr::<u32>(s0 + 0x18, 2);

            let s1 = worker_slot_offset(param_1);
            wr::<u32>(s1 + 0x20, 0);

            let s2 = worker_slot_offset(param_1);
            wr::<u64>(s2 + 0x08, 0);

            let s3 = worker_slot_offset(param_1);
            wr::<u32>(s3 + 0x10, 0);

            let cnt1 = rd::<u32>(rdi_ptr - 0x08) as usize;
            let byte_len1 = cnt1.wrapping_mul(0xa0);
            let mut begin1 = rd::<u64>(rdi_ptr - 0x10);
            let mut end1 = begin1.wrapping_add(byte_len1 as u64);
            let comp_byte = local_res10 as u8;
            exe::call_va!(
                0x1_412e_dc60,
                (
                    (&mut begin1 as *mut u64) as usize,
                    (&mut end1 as *mut u64) as usize,
                    ((byte_len1 as i64) / 0xa0) as u64,
                    comp_byte
                )
            );

            let cnt2 = rd::<u32>(rdi_ptr + 0x08) as usize;
            let byte_len2 = cnt2.wrapping_mul(0xa0);
            let mut begin2 = rd::<u64>(rdi_ptr);
            let mut end2 = begin2.wrapping_add(byte_len2 as u64);
            exe::call_va!(
                0x1_412e_dc60,
                (
                    (&mut begin2 as *mut u64) as usize,
                    (&mut end2 as *mut u64) as usize,
                    ((byte_len2 as i64) / 0xa0) as u64,
                    comp_byte
                )
            );

            for list_idx in 0..2usize {
                let vec_hdr = if list_idx == 0 {
                    rdi_ptr - 0x10
                } else {
                    rdi_ptr
                };
                let mut rbx = rd::<usize>(vec_hdr);
                let end_rsi =
                    rbx.wrapping_add((rd::<u32>(vec_hdr + 0x08) as usize).wrapping_mul(0xa0));
                while rbx != end_rsi {
                    let c3 = cvttss2si32(c_255 * rd::<f32>(rbx + 0x1c)) as u32;
                    let c0 = (cvttss2si32(c_255 * rd::<f32>(rbx + 0x10)) as u8) as u32;
                    let c1 = (cvttss2si32(c_255 * rd::<f32>(rbx + 0x14)) as u8) as u32;
                    let c2 = (cvttss2si32(c_255 * rd::<f32>(rbx + 0x18)) as u8) as u32;
                    let packed_color = (((((c3 << 8) | c0) << 8) | c1) << 8) | c2;

                    let sc = worker_slot_offset(param_1);
                    wr::<u32>(sc + 0x14, packed_color);

                    let idx_50 = rd::<i32>(rbx + 0x50) as isize;
                    let val_1c = rd::<u32>(
                        (exe::ptr::<u32>(0x1_42a4_8580) as usize)
                            .wrapping_add((idx_50 * 4) as usize),
                    );
                    let st = worker_slot_offset(param_1);
                    wr::<u32>(st + 0x1c, val_1c);

                    let val_04 = rd::<i32>(rbx + 0x04) as i64 as u64;
                    let r13_mat = rbx + 0x20;
                    let item_kind = rd::<i32>(rbx) as u32;

                    match item_kind {
                        0 => {
                            let g_160 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_0160) as usize);
                            let sa = worker_slot_offset(param_1);
                            wr::<u64>(sa + 0x30, g_160);

                            let val_74 = rd::<u32>(rbx + 0x74);
                            let sb = worker_slot_offset(param_1);
                            wr::<u32>(sb + 0x38, val_74);

                            let idx_78 = rd::<i32>(rbx + 0x78) as isize;
                            let r9d = u32::from(rd::<u32>(rbx + 0x70) != 1) + 1;
                            let arg5 = rd::<u32>(
                                (exe::ptr::<u32>(0x1_42a4_84d0) as usize)
                                    .wrapping_add((idx_78 * 4) as usize),
                            );
                            exe::call_va!(
                                0x1_4223_7400,
                                (param_1, rbx + 0x58, rd::<u64>(rbx + 0x68), r9d, arg5)
                            );

                            let sc2 = worker_slot_offset(param_1);
                            wr::<u64>(sc2 + 0x30, 0);
                        }
                        1 => {
                            exe::call_va!(
                                0x1_4223_d920,
                                (
                                    param_1,
                                    rd::<u64>(rbx + 0x58),
                                    rd::<u32>(rbx + 0x60),
                                    r13_mat
                                )
                            );
                        }
                        3 => {
                            exe::call_va!(
                                0x1_4223_cef0,
                                (
                                    param_1,
                                    rd::<u64>(rbx + 0x58),
                                    rd::<u64>(rbx + 0x60),
                                    rd::<u32>(rbx + 0x68),
                                    r13_mat
                                )
                            );
                        }
                        5 => {
                            let val_88 = rd::<u64>(rbx + 0x88);
                            let sa = worker_slot_offset(param_1);
                            wr::<u64>(sa + 0x08, val_88);

                            let idx_90 = rd::<i32>(rbx + 0x90) as isize;
                            let val_90 = rd::<u32>(
                                (exe::ptr::<u32>(0x1_42a4_8560) as usize)
                                    .wrapping_add((idx_90 * 4) as usize),
                            );
                            let sb = worker_slot_offset(param_1);
                            wr::<u32>(sb + 0x10, val_90);

                            let idx_94 = rd::<i32>(rbx + 0x94) as isize;
                            let val_94 = rd::<u32>(
                                (exe::ptr::<u32>(0x1_42a4_8550) as usize)
                                    .wrapping_add((idx_94 * 4) as usize),
                            );
                            let sc2 = worker_slot_offset(param_1);
                            wr::<u32>(sc2 + 0x24, val_94);

                            let mut pos_78: [u32; 3] =
                                [rd::<u32>(rbx + 0x58), rd::<u32>(rbx + 0x5c), 0];
                            exe::call_va!(
                                0x1_4223_7e50,
                                (
                                    param_1,
                                    pos_78.as_mut_ptr() as usize,
                                    rbx + 0x64,
                                    rd::<u32>(rbx + 0x98),
                                    0u32,
                                    0u64,
                                    0u8,
                                    rbx + 0x78,
                                    rbx + 0x6c,
                                    0usize
                                )
                            );

                            let sd = worker_slot_offset(param_1);
                            wr::<u64>(sd + 0x08, 0);

                            let se = worker_slot_offset(param_1);
                            wr::<u32>(se + 0x10, 0);

                            let sf = worker_slot_offset(param_1);
                            wr::<u32>(sf + 0x24, 2);
                        }
                        7 | 9 => {
                            let (count_off, mode_r8) = if item_kind == 7 {
                                (0x60usize, 2u32)
                            } else {
                                (0x68usize, 4u32)
                            };
                            local_res10 = rd::<u32>(rbx + count_off);
                            let mut r15_src = rd::<usize>(rbx + 0x58);
                            let r13_slot = worker_slot_offset(param_1);
                            let rdi_lock = r13_slot + 0xa8;
                            acquire_rec_srwlock(rdi_lock);

                            let ok = exe::call_va!(
                                0x1_4223_8720,
                                (
                                    0usize,
                                    r13_slot,
                                    mode_r8,
                                    val_04,
                                    r13_mat,
                                    rbx + 0x10,
                                    local_res10
                                ) -> u8
                            );
                            if ok != 0 {
                                let cur_len = rd::<u32>(r13_slot + 0xa0) as u64;
                                let needed =
                                    ((local_res10 as i32 as i64) as u64).wrapping_add(cur_len);
                                let cur_cap = rd::<u32>(r13_slot + 0xa4) as u64;
                                if needed > cur_cap {
                                    let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                                    if new_cap < needed {
                                        new_cap = needed;
                                    }
                                    exe::call_va!(0x1_4087_4640, (r13_slot + 0x98, new_cap));
                                }
                                if (local_res10 as i32) > 0 {
                                    for _ in 0..(local_res10 as usize) {
                                        exe::call_va!(0x1_412e_dae0, (r13_slot + 0x98, r15_src));
                                        r15_src = r15_src.wrapping_add(8);
                                    }
                                }
                            }
                            release_rec_srwlock(rdi_lock);
                        }
                        _ => {}
                    }

                    rbx = rbx.wrapping_add(0xa0);
                }
            }

            rdi_ptr = rdi_ptr.wrapping_add(0x40);
        }

        release_rec_srwlock(lock_rbx);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142239b60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;
    static mut INST_DST_BUF: [u8; 0x140] = [0; 0x140];
    static mut OBJ_370_BUF: [u8; 0x40] = [0; 0x40];
    static mut VBUF_STORAGE: [u8; 0x80] = [0; 0x80];
    static mut VBUF_MAP_MEM: [u8; 0x40] = [0; 0x40];
    static mut CMD_LOCK_BUF: [u8; 0x20] = [0; 0x20];
    static mut CMD_LIST_OBJ: [usize; 2] = [0; 2];
    static mut CMD_VTBL: [usize; 16] = [0; 16];

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn noop0() {
        unsafe { rec(0x10) };
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    unsafe extern "win64" fn mock_f300(_p1: usize, arg2: u64) {
        unsafe { rec(0xf300 ^ arg2) };
    }
    unsafe extern "win64" fn mock_0080(_p1: usize) {
        unsafe { rec(0x0080) };
    }
    unsafe extern "win64" fn mock_b920(_p1: usize) {
        unsafe { rec(0xb920) };
    }
    unsafe extern "win64" fn mock_f290_alloc(out: usize, sz: u64, tag: u64) -> usize {
        unsafe {
            rec(0xf290 ^ sz ^ tag);
            for i in 0..12usize {
                wr::<u64>(out + i * 8, 0);
            }
            wr::<u64>(out + 3 * 8, 0xaaaa_1111);
            wr::<u64>(out + 4 * 8, 0xbbbb_2222);
            wr::<u64>(
                out + 10 * 8,
                core::ptr::addr_of_mut!(INST_DST_BUF) as usize as u64,
            );
            wr::<u64>(out + 11 * 8, 0x120);
            out
        }
    }
    unsafe extern "win64" fn mock_a370(_rcx: usize, tag: u64) -> usize {
        unsafe {
            rec(0xa370 ^ tag);
            core::ptr::addr_of_mut!(OBJ_370_BUF) as usize
        }
    }
    unsafe extern "win64" fn mock_f4a0(_ctx: usize, a: u16, b: u16, c: u16, d: u16) {
        unsafe {
            rec(0xf4a0 ^ (a as u64) ^ ((b as u64) << 16) ^ ((c as u64) << 32) ^ ((d as u64) << 48));
        }
    }
    unsafe extern "win64" fn mock_a900(cb_idx: i32, off: u64, len: u64, src: usize) {
        unsafe {
            let v0 = if len >= 4 { rd::<u32>(src) as u64 } else { 0 };
            rec(0xa900 ^ (cb_idx as u32 as u64) ^ (off << 16) ^ (len << 32) ^ (v0 << 1));
        }
    }
    unsafe extern "win64" fn mock_eec0(out: usize, sz: u64, src_inst: usize) -> usize {
        unsafe {
            rec(0xeec0 ^ sz ^ (rd::<u32>(src_inst) as u64));
            for i in 0..12usize {
                wr::<u64>(out + i * 8, 0);
            }
            wr::<u64>(out + 3 * 8, 0xcccc_3333);
            wr::<u64>(out + 11 * 8, 0x240);
            out
        }
    }
    unsafe extern "win64" fn mock_f680(_ctx: usize, p30: usize, p60: usize, flag: u8) {
        unsafe {
            rec(0xf680 ^ (rd::<u32>(p30) as u64) ^ (rd::<u32>(p60) as u64) ^ (flag as u64));
        }
    }
    unsafe extern "win64" fn mock_9610() -> usize {
        let p = core::ptr::addr_of_mut!(VBUF_STORAGE) as usize;
        wr::<u64>(p, 0x7777_8888);
        wr::<u64>(p + 0x10, 24);
        p
    }
    unsafe extern "win64" fn mock_d670(_buf: usize) {
        unsafe { rec(0xd670) };
    }
    unsafe extern "win64" fn mock_5af0(
        sz: u64,
        stride: u64,
        a3: u8,
        flags: u32,
        a5: u32,
        _a6: u64,
        buf: usize,
    ) {
        unsafe {
            rec(
                0x5af0
                    ^ sz
                    ^ (stride << 16)
                    ^ ((a3 as u64) << 24)
                    ^ ((flags as u64) << 32)
                    ^ (a5 as u64)
                    ^ (rd::<u64>(buf) & 0xffff),
            );
        }
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _a2: u64, sz: u64, mode: u32) -> usize {
        unsafe {
            rec(0xc560 ^ sz ^ ((mode as u64) << 16));
            core::ptr::addr_of_mut!(VBUF_MAP_MEM) as usize
        }
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, a2: u32) {
        unsafe { rec(0xd6f0 ^ (a2 as u64)) };
    }
    unsafe extern "win64" fn mock_ce60(_c48: usize, id: u32, _b18: usize) -> u64 {
        unsafe {
            rec(0xce60 ^ (id as u64));
            0x9999_aaaa
        }
    }
    unsafe extern "win64" fn mock_f3a0(_ctx: usize, prim: u32) {
        unsafe { rec(0xf3a0 ^ (prim as u64)) };
    }
    unsafe extern "win64" fn mock_f290_state(_ctx: usize) -> u64 {
        0x5555
    }
    unsafe extern "win64" fn mock_0fb0(_rcx: u64, ctx: usize) -> u8 {
        unsafe { rec(0x0fb0 ^ rd::<u64>(ctx + 0x48)) };
        1
    }
    unsafe extern "win64" fn mock_1360(_ctx: usize, arg2: u64) {
        unsafe { rec(0x1360 ^ arg2) };
    }
    unsafe extern "win64" fn mock_draw_instanced(
        _cmd: usize,
        vtx_cnt: u32,
        inst_cnt: u32,
        a4: u32,
        a5: u32,
    ) {
        unsafe {
            rec(
                0x6060
                    ^ (vtx_cnt as u64)
                    ^ ((inst_cnt as u64) << 16)
                    ^ ((a4 as u64) << 32)
                    ^ ((a5 as u64) << 48),
            );
        }
    }
    unsafe extern "win64" fn mock_5b00(out_pair: usize) {
        let lock_p = core::ptr::addr_of_mut!(CMD_LOCK_BUF) as usize;
        wr::<u32>(lock_p + 0x0c, 2);
        let cmd_p = core::ptr::addr_of_mut!(CMD_LIST_OBJ) as usize;
        wr::<usize>(out_pair, lock_p);
        wr::<usize>(out_pair + 0x08, cmd_p);
    }
    unsafe extern "win64" fn mock_ded0(p40: usize) {
        unsafe { rec(0xded0 ^ (rd::<u8>(p40 + 0x97) as u64)) };
    }
    unsafe extern "win64" fn mock_ab00(_ctx: usize, prim_type: u32, prim_cnt: u32, vtx_ptr: usize) {
        unsafe {
            rec(
                0xab00
                    ^ (prim_type as u64)
                    ^ ((prim_cnt as u64) << 16)
                    ^ (rd::<u32>(vtx_ptr) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_case6_cb(ctx: u64) {
        unsafe { rec(0xc6c6 ^ ctx) };
    }
    unsafe extern "win64" fn mock_8080(a1: u64, a2: u64) {
        unsafe { rec(0x8080 ^ a1 ^ a2) };
    }

    let _m01 = crate::difftest::Mock::new(0x1_4183_49c0, noop0 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4223_f300, mock_f300 as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_4224_0080, mock_0080 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4223_b920, mock_b920 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_4205_f290, mock_f290_alloc as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_411d_a370, mock_a370 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4183_4a78, noop1 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_4220_ea70, noop1 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4183_4d18, noop1 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4183_4a0c, noop1 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4206_f4a0, mock_f4a0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4205_eec0, mock_eec0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4206_f680, mock_f680 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_411e_9610, mock_9610 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m20 = crate::difftest::Mock::new(0x1_411d_ce60, mock_ce60 as *const ())?;
    let _m21 = crate::difftest::Mock::new(0x1_4206_f3a0, mock_f3a0 as *const ())?;
    let _m22 = crate::difftest::Mock::new(0x1_4206_f290, mock_f290_state as *const ())?;
    let _m23 = crate::difftest::Mock::new(0x1_4207_0fb0, mock_0fb0 as *const ())?;
    let _m24 = crate::difftest::Mock::new(0x1_4207_1360, mock_1360 as *const ())?;
    let _m25 = crate::difftest::Mock::new(0x1_411d_5b00, mock_5b00 as *const ())?;
    let _m26 = crate::difftest::Mock::new(0x1_4224_ded0, mock_ded0 as *const ())?;
    let _m27 = crate::difftest::Mock::new(0x1_4205_ab00, mock_ab00 as *const ())?;
    let _m28 = crate::difftest::Mock::new(0x1_4225_8080, mock_8080 as *const ())?;

    unsafe {
        CMD_VTBL[12] = mock_draw_instanced as *const () as usize;
        CMD_LIST_OBJ[0] = core::ptr::addr_of!(CMD_VTBL) as usize;
    }

    let snap_cf =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_14c0) as *mut u8, 0xc0);
    let snap_cb =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a6_37b0) as *mut u8, 0x100);
    let snap_mode =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_8590) as *mut u8, 0x40);
    let snap_7ee0 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_7ee0) as *mut u8, 0x10);
    let snap_9490 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c4_9490) as *mut u8, 0x20);
    let snap_globals =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fbc0) as *mut u8, 0x360);
    let snap_d8a3 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_d8a3) as *mut u8, 1);

    let saved_tls0 = rd::<usize>(exe::tls_pointer());

    let run = |f: F| -> (u64, Vec<u8>, [u8; 0x140], [u8; 0x40], [u8; 0x80], Vec<u8>) {
        let mut fake_tls = vec![0u8; 0x48000];
        let tp = fake_tls.as_mut_ptr() as usize;
        wr::<u8>(tp + 0x70, 1);
        wr::<i32>(tp + 0x74, 0);
        wr::<u64>(tp + 0x6308, 0x4242_9999);
        wr::<usize>(tp + 0x467e0 + 0xc48, 0x1234_0000);

        for g_va in [
            0x1_43cf_14e0u64,
            0x1_43cf_14e4,
            0x1_43cf_14e8,
            0x1_43cf_1538,
            0x1_43cf_155c,
            0x1_43cf_1578,
        ] {
            wr::<i32>(exe::ptr::<i32>(g_va) as usize, -1);
        }
        for (i, g_va) in [
            0x1_43cf_14d0u64,
            0x1_43cf_14f8,
            0x1_43cf_1510,
            0x1_43cf_1528,
            0x1_43cf_1548,
            0x1_43cf_1568,
        ]
        .iter()
        .enumerate()
        {
            wr::<i32>(exe::ptr::<i32>(*g_va) as usize, i as i32);
        }

        let cb_tbl = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
        let cb_sizes: [i32; 12] = [8, 4, 8, 8, 4, 8, 64, 16, 16, 8, 16, 16];
        for (i, &sz) in cb_sizes.iter().enumerate() {
            wr::<i32>(cb_tbl + i * 16, (i * 0x40) as i32);
            wr::<i32>(cb_tbl + i * 16 + 0x08, sz);
            wr::<i32>(cb_tbl + i * 16 + 0x0c, i as i32);
        }

        let mode_tbl = exe::ptr::<i32>(0x1_4385_8590) as usize;
        for i in 0..12usize {
            wr::<i32>(mode_tbl + i * 4, (i & 3) as i32);
        }

        let mut fcb8_buf = [0u8; 0xc0];
        let fcb8_p = fcb8_buf.as_mut_ptr() as usize;
        wr::<u64>(fcb8_p + 0x88, 0x1111_0088);
        wr::<u64>(fcb8_p + 0x98, 0x1111_0098);
        wr::<u64>(fcb8_p + 0xa0, 0x1111_00a0);
        wr::<u64>(fcb8_p + 0xa8, 0x1111_00a8);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize, fcb8_p);

        let mut fc58_buf = [0u8; 0x30];
        wr::<u64>(fc58_buf.as_mut_ptr() as usize + 0x20, 0x7777_0020);
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fc58) as usize,
            fc58_buf.as_mut_ptr() as usize,
        );

        let fbc_item: [u32; 3] = [7, 1.5f32.to_bits(), 2.5f32.to_bits()];
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fbc8) as usize,
            fbc_item.as_ptr() as usize,
        );
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fbc0) as usize,
            fbc_item.as_ptr() as usize,
        );
        wr::<u64>(exe::ptr::<u64>(0x1_4397_fd00) as usize, 0xdead_fd00);
        wr::<u64>(exe::ptr::<u64>(0x1_4397_fd30) as usize, 0xdead_fd30);
        wr::<u64>(exe::ptr::<u64>(0x1_4397_ff18) as usize, 0xdead_ff18);

        let fef0_sub0: u32 = 10;
        let fef0_sub1: u32 = 20;
        let d8_target: u64 = 0x8888_9999;
        let mut fef0_arr = [0u64; 32];
        fef0_arr[0] = (&fef0_sub0 as *const u32) as usize as u64;
        fef0_arr[1] = (&fef0_sub1 as *const u32) as usize as u64;
        fef0_arr[0x12] = 0x5555_0012;
        fef0_arr[0x18] = 0x5555_0018;
        fef0_arr[0x1b] = (&d8_target as *const u64) as usize as u64;
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fef0) as usize,
            fef0_arr.as_mut_ptr() as usize,
        );

        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        let lock_7ee0 = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
        wr::<u32>(lock_7ee0 + 0x08, tid);
        wr::<u32>(lock_7ee0 + 0x0c, 1);

        wr::<usize>(exe::ptr::<usize>(0x1_43c4_9490) as usize + 8, 0);

        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
            INST_DST_BUF = [0; 0x140];
            OBJ_370_BUF = [0; 0x40];
            VBUF_STORAGE = [0; 0x80];
            VBUF_MAP_MEM = [0; 0x40];
            wr::<u64>(core::ptr::addr_of_mut!(OBJ_370_BUF) as usize + 0x28, 5);
        }

        let mut inst_src = [0u8; 0xa0];
        for (i, b) in inst_src.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        let vtx_src: [u32; 16] = [11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222, 333, 0, 0, 0, 0];
        let mut p40_obj = [0u8; 0xa0];
        wr::<u32>(p40_obj.as_mut_ptr() as usize + 0x30, 0x3030);
        wr::<u32>(p40_buf_addr(&mut p40_obj, 0x60), 0x6060);
        p40_obj[0x97] = 7;

        let mut rbx_30_obj = [0u8; 0xa0];
        rbx_30_obj[0x51] = 0;
        wr::<u64>(rbx_30_obj.as_mut_ptr() as usize + 0x78, 0xabcd_0078);
        wr::<u64>(rbx_30_obj.as_mut_ptr() as usize + 0x90, 0xabcd_0090);

        // Build 8 commands (kinds 0, 1, 2, 3, 4, 5 non-instanced, 5 instanced, 6, 7)
        let mut cmds = vec![0u8; 9 * 0xd0];
        let cp = cmds.as_mut_ptr() as usize;

        // cmd 0: kind 0
        wr::<i32>(cp, 0);
        wr::<u16>(cp + 0x08, 10);
        wr::<u16>(cp + 0x0a, 20);
        wr::<u16>(cp + 0x0c, 30);
        wr::<u16>(cp + 0x0e, 40);

        // cmd 1: kind 1
        let c1 = cp + 0xd0;
        wr::<i32>(c1, 1);
        for k in 0..16usize {
            wr::<u32>(c1 + 0x08 + k * 4, 0x100 + k as u32);
        }

        // cmd 2: kind 2
        let c2 = cp + 2 * 0xd0;
        wr::<i32>(c2, 2);
        wr::<u32>(c2 + 0x08, 0x2222_3333);

        // cmd 3: kind 3
        let c3 = cp + 3 * 0xd0;
        wr::<i32>(c3, 3);
        wr::<[u32; 4]>(c3 + 0x08, [1, 2, 3, 4]);

        // cmd 4: kind 4
        let c4 = cp + 4 * 0xd0;
        wr::<i32>(c4, 4);
        wr::<u32>(c4 + 0x08, 0x401);
        wr::<u32>(c4 + 0x0c, 0x402);
        wr::<u32>(c4 + 0x10, 0x403);
        wr::<u32>(c4 + 0x14, 0x404);
        wr::<[u32; 4]>(c4 + 0x18, [5, 6, 7, 8]);
        wr::<[u32; 4]>(c4 + 0x28, [9, 10, 11, 12]);

        // cmd 5: kind 5 (non-instanced: +0xb8 == 0)
        let c5 = cp + 5 * 0xd0;
        wr::<i32>(c5, 5);
        wr::<i32>(c5 + 0x08, 0);
        wr::<i32>(c5 + 0x0c, 4); // triangle list (/3)
        wr::<i32>(c5 + 0x10, 0);
        wr::<i32>(c5 + 0x14, 6);
        wr::<usize>(c5 + 0x30, rbx_30_obj.as_mut_ptr() as usize);
        wr::<i32>(c5 + 0x38, 4);
        wr::<i32>(c5 + 0x3c, 8);
        wr::<usize>(c5 + 0x40, p40_obj.as_mut_ptr() as usize);
        wr::<u32>(c5 + 0x48, 0x5555);
        wr::<u32>(c5 + 0x88, 0x8888);
        wr::<u32>(c5 + 0x98, 0x9898);
        wr::<u32>(c5 + 0xa8, 0xa8a8);
        wr::<u32>(c5 + 0xac, 0xacac);
        wr::<u32>(c5 + 0xb0, 0xb0b0);
        wr::<u32>(c5 + 0xb4, 0xb4b4);
        wr::<u64>(c5 + 0xb8, 0);

        // cmd 6: kind 5 (instanced: +0xb8 == 1, triggers re-batch because inst_start=1, inst_count=2 > batch_cnt=1)
        let c6 = cp + 6 * 0xd0;
        unsafe {
            core::ptr::copy_nonoverlapping(c5 as *const u8, c6 as *mut u8, 0xd0);
        }
        wr::<i32>(c6 + 0x18, 1);
        wr::<i32>(c6 + 0x1c, 1);
        wr::<u64>(c6 + 0xb8, 1);

        // cmd 7: kind 6
        let c7 = cp + 7 * 0xd0;
        wr::<i32>(c7, 6);
        wr::<u64>(c7 + 0xc0, 0x7777);
        wr::<usize>(c7 + 0xc8, mock_case6_cb as *const () as usize);

        // cmd 8: kind 7
        let c8 = cp + 8 * 0xd0;
        wr::<i32>(c8, 7);
        wr::<u64>(c8 + 0x08, 0x8888_1234);

        let mut p1 = vec![0u8; 0x1300];
        let pp = p1.as_mut_ptr() as usize;
        wr::<u8>(pp, 1);
        wr::<usize>(pp + 0x68, inst_src.as_ptr() as usize);
        wr::<u32>(pp + 0x70, 1);
        wr::<usize>(pp + 0x78, cp);
        wr::<u32>(pp + 0x80, 9);
        wr::<usize>(pp + 0x88, vtx_src.as_ptr() as usize);
        wr::<usize>(pp + 0x98, vtx_src.as_ptr() as usize);
        wr::<u32>(pp + 0xb0, tid);
        wr::<u32>(pp + 0xb4, 1);

        wr::<i32>(pp + 0x1018, 6);
        wr::<i32>(pp + 0x1080, 7);
        wr::<i32>(pp + 0x10a0, 8);
        wr::<i32>(pp + 0x1100, 9);
        wr::<i32>(pp + 0x1280, 10);
        wr::<i32>(pp + 0x12a0, 11);

        wr::<usize>(exe::ptr::<usize>(0x1_4397_fef8) as usize, pp);
        wr::<u8>(exe::ptr::<u8>(0x1_4397_d8a3) as usize, 0);

        wr::<usize>(exe::tls_pointer(), tp);
        f(pp, 0);
        wr::<usize>(exe::tls_pointer(), saved_tls0);

        let tls_cb_slice = fake_tls[0x63d0..0x66d0].to_vec();
        (
            unsafe { TRACE_HASH },
            p1[0x1000..0x12c0].to_vec(),
            unsafe { INST_DST_BUF },
            unsafe { VBUF_MAP_MEM },
            unsafe { VBUF_STORAGE },
            tls_cb_slice,
        )
    };

    #[inline(always)]
    fn p40_buf_addr(buf: &mut [u8; 0xa0], off: usize) -> usize {
        buf.as_mut_ptr() as usize + off
    }

    let o = run(f_o);
    let n = run(f_n);

    wr::<usize>(exe::tls_pointer(), saved_tls0);
    snap_cf.restore();
    snap_cb.restore();
    snap_mode.restore();
    snap_7ee0.restore();
    snap_9490.restore();
    snap_globals.restore();
    snap_d8a3.restore();

    crate::diff_eq!(o, n, "142239b60 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142240080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x0100_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_dbf0(_p1: usize, mat_ptr: usize) {
        unsafe {
            let m0 = rd::<u32>(mat_ptr) as u64;
            let m5 = rd::<u32>(mat_ptr + 0x14) as u64;
            let m10 = rd::<u32>(mat_ptr + 0x28) as u64;
            let m12 = rd::<u32>(mat_ptr + 0x30) as u64;
            rec(0xdbf0 ^ m0 ^ (m5 << 16) ^ (m10 << 32) ^ (m12 << 48));
        }
    }
    unsafe extern "win64" fn mock_wid() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_dc60(begin_ptr: usize, end_ptr: usize, cnt: u64, _comp: u8) {
        unsafe {
            let span = rd::<u64>(end_ptr).wrapping_sub(rd::<u64>(begin_ptr));
            rec(0xdc60 ^ span ^ (cnt << 16));
        }
    }
    unsafe extern "win64" fn mock_7400(
        p1: usize,
        _p58: usize,
        val_68: u64,
        r9d: u32,
        arg5: u32,
    ) {
        unsafe {
            rec(
                0x7400
                    ^ rd::<u64>(p1 + 0x30)
                    ^ (rd::<u32>(p1 + 0x38) as u64)
                    ^ val_68
                    ^ ((r9d as u64) << 16)
                    ^ ((arg5 as u64) << 32),
            );
        }
    }
    unsafe extern "win64" fn mock_d920(p1: usize, val_58: u64, val_60: u32, mat_ptr: usize) {
        unsafe {
            rec(
                0xd920
                    ^ (rd::<u32>(p1 + 0x14) as u64)
                    ^ val_58
                    ^ ((val_60 as u64) << 16)
                    ^ (rd::<u32>(mat_ptr) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_cef0(
        p1: usize,
        val_58: u64,
        val_60: u64,
        val_68: u32,
        mat_ptr: usize,
    ) {
        unsafe {
            rec(
                0xcef0
                    ^ (rd::<u32>(p1 + 0x14) as u64)
                    ^ val_58
                    ^ val_60
                    ^ ((val_68 as u64) << 16)
                    ^ (rd::<u32>(mat_ptr) as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_7e50(
        p1: usize,
        pos_ptr: usize,
        p64: usize,
        val_98: u32,
        a5: u32,
        _a6: u64,
        a7: u8,
        p78: usize,
        p6c: usize,
        a10: usize,
    ) {
        unsafe {
            let pos = rd::<[u32; 3]>(pos_ptr);
            rec(
                0x7e50
                    ^ rd::<u64>(p1 + 0x08)
                    ^ (rd::<u32>(p1 + 0x10) as u64)
                    ^ (rd::<u32>(p1 + 0x24) as u64)
                    ^ (pos[0] as u64)
                    ^ ((pos[1] as u64) << 16)
                    ^ ((pos[2] as u64) << 32)
                    ^ (val_98 as u64)
                    ^ (a5 as u64)
                    ^ (a7 as u64)
                    ^ ((p78.wrapping_sub(p64)) as u64)
                    ^ ((p6c.wrapping_sub(p64)) as u64)
                    ^ (a10 as u64),
            );
        }
    }
    unsafe extern "win64" fn mock_8720(
        _rcx: usize,
        _slot: usize,
        mode: u32,
        val_04: u64,
        mat_ptr: usize,
        col_ptr: usize,
        cnt: u32,
    ) -> u8 {
        unsafe {
            rec(
                0x8720
                    ^ (mode as u64)
                    ^ (val_04 << 8)
                    ^ (rd::<u32>(mat_ptr) as u64)
                    ^ (rd::<u32>(col_ptr) as u64)
                    ^ ((cnt as u64) << 32),
            );
        }
        1
    }
    unsafe extern "win64" fn mock_4640(vec_98: usize, new_cap: u64) {
        unsafe {
            rec(0x4640 ^ new_cap);
            wr::<u32>(vec_98 + 0x0c, new_cap as u32);
        }
    }
    unsafe extern "win64" fn mock_dae0(_vec_98: usize, elem_ptr: usize) {
        unsafe {
            rec(0xdae0 ^ rd::<u64>(elem_ptr));
        }
    }

    let _m01 = crate::difftest::Mock::new(0x1_4223_dbf0, mock_dbf0 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4240_7c80, mock_wid as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_412e_dc60, mock_dc60 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4223_7400, mock_7400 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_4223_d920, mock_d920 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_4223_cef0, mock_cef0 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4223_7e50, mock_7e50 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_4223_8720, mock_8720 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4087_4640, mock_4640 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_412e_dae0, mock_dae0 as *const ())?;

    let snap_arr =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c1_6a00) as *mut u8, 0x5600);
    let snap_aa00 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c4_aa00) as *mut u8, 8);
    let snap_0160 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_0160) as *mut u8, 8);

    let run = |f: F| -> (u64, [u8; 0xd8]) {
        let arr_base = exe::ptr::<u8>(0x1_43c1_6a00) as usize;
        unsafe {
            core::ptr::write_bytes(arr_base as *mut u8, 0, 0x5600);
        }
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        for i in 0..0x40usize {
            let lock = arr_base + 0xc0 + i * 0x158;
            wr::<u32>(lock + 0x08, tid);
            wr::<u32>(lock + 0x0c, 1);
        }

        let slot_zero: u32 = 0;
        wr::<usize>(
            exe::ptr::<usize>(0x1_43c4_aa00) as usize,
            (&slot_zero as *const u32) as usize,
        );
        wr::<u64>(exe::ptr::<u64>(0x1_43a0_0160) as usize, 0xfeed_face_1234);

        let extra_elems: [u64; 2] = [0x1111_2222, 0x3333_4444];
        let mut items = vec![0u8; 6 * 0xa0];
        let ip = items.as_mut_ptr() as usize;

        for (idx, &kind) in [0i32, 1, 3, 5, 7, 9].iter().enumerate() {
            let it = ip + idx * 0xa0;
            wr::<i32>(it, kind);
            wr::<i32>(it + 0x04, (idx as i32) + 10);
            wr::<f32>(it + 0x10, 0.25);
            wr::<f32>(it + 0x14, 0.5);
            wr::<f32>(it + 0x18, 0.75);
            wr::<f32>(it + 0x1c, 1.0);
            wr::<u32>(it + 0x20, 0x1234_0000 + idx as u32);
            wr::<i32>(it + 0x50, 0);
            match kind {
                0 => {
                    wr::<u64>(it + 0x68, 0xaaaa_0068);
                    wr::<u32>(it + 0x70, 2);
                    wr::<u32>(it + 0x74, 0x3f80_0000);
                    wr::<i32>(it + 0x78, 0);
                }
                1 => {
                    wr::<u64>(it + 0x58, 0xbbbb_0058);
                    wr::<u32>(it + 0x60, 3);
                }
                3 => {
                    wr::<u64>(it + 0x58, 0xcccc_0058);
                    wr::<u64>(it + 0x60, 0xcccc_0060);
                    wr::<u32>(it + 0x68, 5);
                }
                5 => {
                    wr::<f32>(it + 0x58, 1.25);
                    wr::<f32>(it + 0x5c, -2.5);
                    wr::<u64>(it + 0x88, 0xdddd_0088);
                    wr::<i32>(it + 0x90, 0);
                    wr::<i32>(it + 0x94, 0);
                    wr::<u32>(it + 0x98, 7);
                }
                7 => {
                    wr::<usize>(it + 0x58, extra_elems.as_ptr() as usize);
                    wr::<u32>(it + 0x60, 2);
                }
                9 => {
                    wr::<usize>(it + 0x58, extra_elems.as_ptr() as usize);
                    wr::<u32>(it + 0x68, 2);
                }
                _ => {}
            }
        }

        wr::<usize>(arr_base, ip);
        wr::<u32>(arr_base + 0x08, 3);
        wr::<usize>(arr_base + 0x10, ip + 3 * 0xa0);
        wr::<u32>(arr_base + 0x18, 3);

        let mut p1 = [0u8; 0xd8];
        let pp = p1.as_mut_ptr() as usize;
        wr::<u32>(pp + 0xa0, 1);
        wr::<u32>(pp + 0xa4, 1);
        wr::<u32>(pp + 0xb0, tid);
        wr::<u32>(pp + 0xb4, 1);

        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }
        f(pp);
        (unsafe { TRACE_HASH }, p1)
    };

    let o = run(f_o);
    let n = run(f_n);

    snap_arr.restore();
    snap_aa00.restore();
    snap_0160.restore();

    crate::diff_eq!(o, n, "142240080 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4223_9b60, "unknown_142239b60", rend::cauldron_hot_m::unknown_142239b60 as *const (), Throws::Caught, rend::cauldron_hot_m::diff_unknown_142239b60),
// Replacement::verified(0x1_4224_0080, "unknown_142240080", rend::cauldron_hot_m::unknown_142240080 as *const (), rend::cauldron_hot_m::diff_unknown_142240080),
