//! Decompiled `rend` hot functions shard ZA20 (`0x14224a7d0..0x142268cc0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// `unknown_14224a7d0`
/// Original: 0x14224a7d0 (388 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14224a7d0(param_1: i32, param_2: i32) {
    wr::<i32>(exe::ptr::<i32>(0x1_43a3_3428) as usize, param_1);
    wr::<i32>(exe::ptr::<i32>(0x1_43a3_342c) as usize, param_2);

    let rdi = rd::<usize>(exe::tls_pointer());
    let epoch = rd::<i32>(rdi + 0x74);
    let g_5c8 = exe::ptr::<i32>(0x1_43cf_15c8) as usize;
    if rd::<i32>(g_5c8) > epoch {
        exe::call_va!(0x1_4183_4a78, (g_5c8));
        if rd::<i32>(g_5c8) == -1 {
            let mut view: [u64; 2] = [exe::ptr::<u8>(0x1_433f_1c90) as usize as u64, 0x0f];
            let g_8b0 = exe::ptr::<i32>(0x1_43cf_18b0) as usize;
            exe::call_va!(0x1_4220_ea70, (g_8b0, view.as_mut_ptr() as usize));
            wr::<u32>(exe::ptr::<u32>(0x1_43cf_18b4) as usize, 1);
            let g_8b8 = exe::ptr::<u64>(0x1_43cf_18b8) as usize;
            wr::<usize>(exe::ptr::<usize>(0x1_43cf_18a8) as usize, g_8b8);
            wr::<u32>(g_8b8, 0);
            wr::<u32>(exe::ptr::<u32>(0x1_43cf_18bc) as usize, 0);
            let dtor = exe::ptr::<u8>(0x1_4262_a830) as usize;
            exe::call_va!(0x1_4183_4d18, (dtor));
            exe::call_va!(0x1_4183_4a0c, (g_5c8));
        }
    }

    let f0 = (param_1 as f32).to_bits();
    let f1 = (param_2 as f32).to_bits();
    let mut pair: u64 = (f0 as u64) | ((f1 as u64) << 32);

    let idx = rd::<i32>(exe::ptr::<i32>(0x1_43cf_18b0) as usize) as isize;
    let base_7b0 = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let entry = (base_7b0 as isize).wrapping_add(idx.wrapping_mul(16)) as usize;
    let off = rd::<i32>(entry) as isize;
    let g_8b8 = exe::ptr::<u64>(0x1_43cf_18b8) as usize;

    if off >= 0 {
        let sz = rd::<i32>(entry + 0x08) as i64 as u64;
        let dst = (rdi as isize).wrapping_add(off).wrapping_add(0x63d0) as usize;
        exe::call_va!(0x1_4183_acbc, (dst, (&mut pair as *mut u64) as usize, sz));
        let slot = rd::<i32>(entry + 0x0c) as isize;
        if slot != -1 {
            let cnt_addr = (rdi as isize)
                .wrapping_add(0x463e0)
                .wrapping_add(slot.wrapping_mul(4)) as usize;
            let new_cnt = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, new_cnt);
            wr::<u64>(g_8b8, pair);
            if new_cnt == -1 {
                wr::<i32>(cnt_addr, 0);
            }
            return;
        }
    }

    wr::<u64>(g_8b8, pair);
}

/// `r_142255340`
/// Original: 0x142255340 (380 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_142255340(param_1: usize) {
    let rbx = param_1.wrapping_add(0x2b8);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx));
        wr::<u32>(rbx + 0x08, tid);
    }
    wr::<u32>(rbx + 0x0c, rd::<u32>(rbx + 0x0c).wrapping_add(1));
    wr::<u32>(
        param_1 + 0x2e8,
        1u32.wrapping_sub(rd::<u32>(param_1 + 0x2e8)),
    );
    let new_cnt = rd::<u32>(rbx + 0x0c).wrapping_sub(1);
    wr::<u32>(rbx + 0x0c, new_cnt);
    if new_cnt == 0 {
        crate::simd_ops::atomic_xchg_u32((rbx + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rbx));
    }

    let other_idx = 1i32.wrapping_sub(rd::<i32>(param_1 + 0x2e8)) as isize;
    let r14 = (param_1 as isize)
        .wrapping_add(0x2c8)
        .wrapping_add(other_idx.wrapping_mul(0x10)) as usize;

    let mut rdi = rd::<usize>(r14);
    let len = rd::<u32>(r14 + 0x08) as usize;
    let rsi = rdi.wrapping_add(len.wrapping_mul(0x28));

    while rdi != rsi {
        let arg_rcx = rd::<u64>(rdi + 0x18);
        let arg_rdx = rd::<u64>(rdi + 0x08);
        exe::call_va!(0x1_4239_af80, (arg_rcx, arg_rdx));
        let obj0 = rd::<usize>(rdi);
        exe::call_va!(0x1_4225_5e30, (param_1, obj0, rdi + 0x18));
        let obj0_b = rd::<usize>(rdi);
        crate::simd_ops::atomic_xchg_u32((obj0_b + 0x80) as u64, 4);
        let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
        exe::call_va!(0x1_4242_0ed0, (g_2b8, obj0_b));
        let g_ff00 = exe::ptr::<u32>(0x1_4397_ff00) as u64;
        let _ = crate::simd_ops::atomic_cmpxchg_u32(g_ff00, 1, 0);
        let rbp = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
        exe::call_iat!(0x1_4264_55a8, (rbp + 0x250));
        wr::<u32>(rbp + 0x258, rd::<u32>(rbp + 0x258).wrapping_add(1));
        exe::call_iat!(0x1_4264_5598, (rbp + 0x250));
        exe::call_iat!(0x1_4264_5590, (rbp + 0x248));
        rdi = rdi.wrapping_add(0x28);
    }

    let mut rbx2 = rd::<usize>(r14);
    let len2 = rd::<u32>(r14 + 0x08) as usize;
    let rdi2 = rbx2.wrapping_add(len2.wrapping_mul(0x28));
    while rbx2 != rdi2 {
        exe::call_va!(0x1_412f_4530, (rbx2));
        rbx2 = rbx2.wrapping_add(0x28);
    }
    wr::<u32>(r14 + 0x08, 0);
}

/// `r_142258d40`
/// Original: 0x142258d40 (150 bytes, Throws::No)
pub unsafe extern "win64" fn r_142258d40() -> u8 {
    let b_88e = rd::<u8>(exe::ptr::<u8>(0x1_4397_d88e) as usize);
    let al = rd::<u8>(exe::ptr::<u8>(0x1_4397_d88b) as usize);
    let cl = rd::<u8>(exe::ptr::<u8>(0x1_4397_d86e) as usize);
    let dl = b_88e != 0 && (al != 0 || cl != 0);

    let b_886 = rd::<u8>(exe::ptr::<u8>(0x1_4397_d886) as usize);
    let r9b = rd::<u8>(exe::ptr::<u8>(0x1_4397_d89b) as usize);
    let r8b = b_886 != 0 && (al != 0 || r9b != 0);

    if al == 0
        && r9b == 0
        && !r8b
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d873) as usize) == 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d896) as usize) == 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d86a) as usize) == 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d87e) as usize) == 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d85a) as usize) == 0
        && cl == 0
        && !dl
        && rd::<u8>(exe::ptr::<u8>(0x1_4397_d883) as usize) == 0
    {
        0
    } else {
        1
    }
}

/// `r_14225c9a0`
/// Original: 0x14225c9a0 (728 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14225c9a0(param_1: usize) {
    let mut rdi = rd::<u32>(param_1 + 0x20) as u64;
    if rdi != 0 {
        let base = rd::<usize>(param_1 + 0x18);
        let mut ptr = base.wrapping_add(((rdi - 1) as usize).wrapping_mul(8));
        loop {
            if rd::<u64>(ptr) != 0 {
                break;
            }
            ptr = ptr.wrapping_sub(8);
            rdi -= 1;
            if rdi == 0 {
                break;
            }
        }
    }

    exe::call_va!(0x1_40ff_f1e0, (param_1, rdi));
    exe::call_va!(0x1_4124_3000, (param_1 + 0x18, rdi));
    exe::call_va!(0x1_412f_8600, (param_1 + 0x28, rdi));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x638, rdi));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x650, rdi));
    exe::call_va!(0x1_40a0_2480, (param_1 + 0x5c8, rdi));
    exe::call_va!(0x1_40a0_2480, (param_1 + 0x5d8, rdi));

    wr::<u64>(param_1 + 0x4e8, rdi);
    let mut r8_a = rd::<u64>(param_1 + 0x4a8);
    if rdi >= r8_a {
        let t = r8_a.wrapping_add(rdi).wrapping_sub(1);
        r8_a = t.wrapping_sub(t % r8_a);
    }
    wr::<u64>(param_1 + 0x4e0, r8_a);

    wr::<u64>(param_1 + 0x588, rdi);
    let mut r8_b = rd::<u64>(param_1 + 0x548);
    if rdi >= r8_b {
        let t = r8_b.wrapping_add(rdi).wrapping_sub(1);
        r8_b = t.wrapping_sub(t % r8_b);
    }
    wr::<u64>(param_1 + 0x580, r8_b);

    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x38, rdi));
    let sub_50 = param_1.wrapping_add(0x50);
    let vtbl_50 = rd::<usize>(sub_50);
    exe::call_va!(rd::<usize>(vtbl_50 + 0x28), (sub_50, rdi));
    exe::call_va!(0x1_40a0_2700, (param_1 + 0x120, rdi));

    let rbx_shl4 = rdi.wrapping_shl(4);
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x160, rbx_shl4));
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x170, rbx_shl4));
    exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x180, rdi));
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x198, rdi));
    exe::call_va!(0x1_4028_e490, (param_1 + 0x1a8, rdi));
    exe::call_va!(0x1_4028_e490, (param_1 + 0x1b8, rdi));
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x1c8, rdi));
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x1d8, rdi));
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x1e8, rdi));
    exe::call_va!(0x1_406d_cc50, (param_1 + 0x1f8, rdi));

    let rbx = param_1.wrapping_add(0x208);
    let rbp = rd::<u32>(rbx + 0x08) as u64;
    let mut dst_ptr = rd::<usize>(rbx).wrapping_add((rbp as usize).wrapping_mul(2));
    let cap = rd::<u32>(rbx + 0x0c) as u64;
    if rdi > cap {
        let mut grow = (cap >> 1).wrapping_add(cap);
        if grow < rdi {
            grow = rdi;
        }
        exe::call_va!(0x1_40f4_f300, (rbx, grow));
        let cur_len = rd::<u32>(rbx + 0x08) as usize;
        dst_ptr = rd::<usize>(rbx).wrapping_add(cur_len.wrapping_mul(2));
        let diff = rdi.wrapping_sub(rbp);
        if diff != 0 {
            exe::call_va!(0x1_4183_acc2, (dst_ptr, 0u32, diff.wrapping_mul(2)));
        }
        wr::<u32>(rbx + 0x08, rdi as u32);
    } else if rdi > rbp {
        let diff = rdi.wrapping_sub(rbp);
        if diff != 0 {
            exe::call_va!(0x1_4183_acc2, (dst_ptr, 0u32, diff.wrapping_mul(2)));
        }
        wr::<u32>(rbx + 0x08, rdi as u32);
    } else if rdi != rbp {
        wr::<u32>(rbx + 0x08, rdi as u32);
    }

    exe::call_va!(0x1_4028_e490, (param_1 + 0x218, rdi));

    let g_2f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let rbx_sched = rd::<usize>(g_2f8 + 0x10);
    let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let worker_pool = rd::<usize>(rbx_sched + worker_idx * 0x40 + 0x48).wrapping_add(0x40);
    let mut job_pair: [u64; 2] = [0; 2];
    exe::call_va!(
        0x1_4248_5aa0,
        (worker_pool, job_pair.as_mut_ptr() as usize)
    );
    let job_ptr = job_pair[0] as usize;
    crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
    let job_ptr2 = job_pair[0] as usize;
    wr::<u64>(job_ptr2 + 0x08, 0);
    let vtbl_job = exe::ptr::<u8>(0x1_433f_3328) as usize;
    wr::<usize>(job_ptr2 + 0x10, vtbl_job);
    wr::<usize>(job_ptr2 + 0x18, param_1);
    wr::<u64>(param_1 + 0x628, job_pair[0]);
    wr::<u64>(param_1 + 0x630, job_pair[1]);

    let g_2f8_2 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let cur_job = rd::<usize>(param_1 + 0x628);
    if rd::<u32>(cur_job) == 1 {
        let sched = rd::<usize>(g_2f8_2 + 0x10);
        let job_arg = rd::<usize>(param_1 + 0x628);
        exe::call_va!(0x1_4248_6460, (sched, job_arg));
        let owner = rd::<usize>(sched);
        let sem = rd::<usize>(owner + 0x1008);
        exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
    }
}

/// `unknown_142268cc0`
/// Original: 0x142268cc0 (266 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142268cc0(param_1: usize) {
    let rdi = rd::<usize>(param_1 + 0x48);
    let mut local_obj: usize = 0;
    let g_fc58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(
        0x1_4208_d1f0,
        (g_fc58, (&mut local_obj as *mut usize) as usize, 1u8)
    );
    if local_obj != 0 {
        let vtbl = rd::<usize>(local_obj);
        exe::call_va!(rd::<usize>(vtbl + 0x10), (local_obj));
    }

    let g_2b11c = exe::ptr::<u32>(0x1_43a2_b11c) as usize;
    let g_2b08c = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let g_549ac = exe::ptr::<f32>(0x1_4385_49ac) as usize;
    let g_63738 = exe::ptr::<u32>(0x1_43a6_3738) as usize;
    let g_63710 = exe::ptr::<u32>(0x1_43a6_3710) as usize;
    let g_57fe8 = exe::ptr::<f32>(0x1_4385_7fe8) as usize;

    let eax = rd::<u32>(g_2b11c);
    let r9d = rd::<u32>(g_2b08c);
    let xmm1 = rd::<f32>(g_549ac);
    let mut edx = rd::<u32>(g_63738);
    wr::<u32>(g_2b11c, edx);
    let r8d = rd::<u32>(g_63710);
    wr::<u32>(g_2b08c, r8d);
    let xmm0 = rd::<f32>(g_57fe8);
    wr::<f32>(g_549ac, xmm0);

    if eax != edx || r9d != r8d || !(xmm1 == xmm0) {
        exe::call_va!(0x1_4207_a1e0, (rdi, 0u32, 1u8));
        edx = rd::<u32>(g_2b11c);
    }

    let rcx = rd::<usize>(param_1 + 0x48);
    let al: u8 = if rd::<u8>(rcx + 0x66) != 0 {
        1
    } else if edx != 0 {
        u8::from(edx == 2)
    } else if rd::<u8>(rcx + 0x67) != 0 {
        0
    } else {
        1
    };
    wr::<u8>(param_1 + 0x49d, al);

    crate::simd_ops::atomic_xchg_u8(
        exe::ptr::<u8>(0x1_4397_d82f) as u64,
        rd::<u8>(rcx + 0x4c),
    );
    let rcx2 = rd::<usize>(param_1 + 0x48);
    crate::simd_ops::atomic_xchg_u8(
        exe::ptr::<u8>(0x1_4397_d82e) as u64,
        rd::<u8>(rcx2 + 0x4f),
    );
    let rcx3 = rd::<usize>(param_1 + 0x48);
    crate::simd_ops::atomic_xchg_u32(
        exe::ptr::<u32>(0x1_4397_ff20) as u64,
        rd::<u32>(rcx3 + 0x50),
    );
    let rax4 = rd::<usize>(param_1 + 0x48);
    crate::simd_ops::atomic_xchg_u8(
        exe::ptr::<u8>(0x1_4397_d82a) as u64,
        rd::<u8>(rax4 + 0x64),
    );
    let rax5 = rd::<usize>(param_1 + 0x48);
    crate::simd_ops::atomic_xchg_u8(
        exe::ptr::<u8>(0x1_4397_d82b) as u64,
        rd::<u8>(rax5 + 0x65),
    );
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14224a7d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut INIT_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_4a78(p: usize) {
        INIT_CALLS += 1;
        wr::<i32>(p, -1);
    }
    unsafe extern "win64" fn mock_ea70(out_idx: usize, _view: usize) {
        INIT_CALLS += 10;
        wr::<i32>(out_idx, 0);
    }
    unsafe extern "win64" fn mock_4d18(_dtor: usize) -> i32 {
        INIT_CALLS += 100;
        0
    }
    unsafe extern "win64" fn mock_4a0c(p: usize) {
        INIT_CALLS += 1000;
        wr::<i32>(p, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4183_4a78, mock_4a78 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4183_4d18, mock_4d18 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_4a0c as *const ())?;

    let g_3428 = exe::ptr::<i32>(0x1_43a3_3428) as usize;
    let g_342c = exe::ptr::<i32>(0x1_43a3_342c) as usize;
    let g_5c8 = exe::ptr::<i32>(0x1_43cf_15c8) as usize;
    let g_8a8 = exe::ptr::<usize>(0x1_43cf_18a8) as usize;
    let g_8b0 = exe::ptr::<i32>(0x1_43cf_18b0) as usize;
    let g_8b4 = exe::ptr::<u32>(0x1_43cf_18b4) as usize;
    let g_8b8 = exe::ptr::<u64>(0x1_43cf_18b8) as usize;
    let g_7b0 = exe::ptr::<[i32; 4]>(0x1_43a6_37b0) as usize;

    let saved_3428 = rd::<i32>(g_3428);
    let saved_342c = rd::<i32>(g_342c);
    let saved_5c8 = rd::<i32>(g_5c8);
    let saved_8a8 = rd::<usize>(g_8a8);
    let saved_8b0 = rd::<i32>(g_8b0);
    let saved_8b4 = rd::<u32>(g_8b4);
    let saved_8b8 = rd::<u64>(g_8b8);
    let saved_7b0 = rd::<[i32; 4]>(g_7b0);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_blk = vec![0u8; 0x46420];
    let tp = tls_blk.as_mut_ptr() as usize;

    let run = |f: F, mode: u8| -> (i32, i32, u64, u64, i32, u32, usize, u32) {
        INIT_CALLS = 0;
        wr::<i32>(tp + 0x74, 0);
        wr::<u64>(tp + 0x63d0 + 0x10, 0);
        wr::<i32>(tp + 0x463e0 + 8, if mode == 2 { -2 } else { 5 });

        wr::<i32>(g_3428, 0);
        wr::<i32>(g_342c, 0);
        wr::<i32>(g_5c8, if mode == 3 { 1 } else { 0 });
        wr::<usize>(g_8a8, 0);
        wr::<i32>(g_8b0, 0);
        wr::<u32>(g_8b4, 0);
        wr::<u64>(g_8b8, 0);
        wr::<[i32; 4]>(
            g_7b0,
            if mode == 0 {
                [-1, 0, 8, -1]
            } else {
                [0x10, 0, 8, 2]
            },
        );

        wr::<usize>(tls_slot, tp);
        f(12, -34);
        wr::<usize>(tls_slot, saved_tls0);

        (
            rd::<i32>(g_3428),
            rd::<i32>(g_342c),
            rd::<u64>(g_8b8),
            rd::<u64>(tp + 0x63d0 + 0x10),
            rd::<i32>(tp + 0x463e0 + 8),
            rd::<u32>(g_8b4),
            rd::<usize>(g_8a8),
            INIT_CALLS,
        )
    };

    let restore = || {
        wr::<i32>(g_3428, saved_3428);
        wr::<i32>(g_342c, saved_342c);
        wr::<i32>(g_5c8, saved_5c8);
        wr::<usize>(g_8a8, saved_8a8);
        wr::<i32>(g_8b0, saved_8b0);
        wr::<u32>(g_8b4, saved_8b4);
        wr::<u64>(g_8b8, saved_8b8);
        wr::<[i32; 4]>(g_7b0, saved_7b0);
        wr::<usize>(tls_slot, saved_tls0);
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        if o != n {
            restore();
        }
        crate::diff_eq!(o, n, "14224a7d0 mode={}", mode);
    }
    restore();
    Ok(4)
}

pub unsafe fn diff_r_142255340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: Vec<u64> = Vec::new();

    unsafe extern "win64" fn mock_tid() -> u32 {
        0x1234
    }
    unsafe extern "win64" fn mock_acq(_lock: usize) {
        TRACE.push(1);
    }
    unsafe extern "win64" fn mock_rel(_lock: usize) {
        TRACE.push(2);
    }
    unsafe extern "win64" fn mock_wake(_cv: usize) {
        TRACE.push(3);
    }
    unsafe extern "win64" fn mock_af80(a: u64, b: u64) {
        TRACE.push(a ^ b);
    }
    unsafe extern "win64" fn mock_5e30(_p1: usize, obj: usize, slot_18: usize) {
        TRACE.push((rd::<u32>(obj + 0x80) as u64) | ((rd::<u64>(slot_18)) << 8));
    }
    unsafe extern "win64" fn mock_0ed0(g: usize) {
        TRACE.push(g as u64);
    }
    unsafe extern "win64" fn mock_4530(elem: usize) {
        TRACE.push(rd::<u64>(elem + 0x08));
    }

    let _im1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_tid as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _im3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;
    let _im4 = crate::difftest::IatMock::new(0x1_4264_5590, mock_wake as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4239_af80, mock_af80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_5e30, mock_5e30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4242_0ed0, mock_0ed0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412f_4530, mock_4530 as *const ())?;

    let g_2b8 = exe::ptr::<usize>(0x1_43a0_82b8) as usize;
    let g_ff00 = exe::ptr::<u32>(0x1_4397_ff00) as usize;
    let g_298 = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let saved_2b8 = rd::<usize>(g_2b8);
    let saved_ff00 = rd::<u32>(g_ff00);
    let saved_298 = rd::<usize>(g_298);

    let run = |f: F, mode: u8| -> (u32, u32, u32, u32, u32, u32, u32, Vec<u64>) {
        TRACE.clear();
        let mut mgr = vec![0u8; 0x300];
        let mut obj0 = vec![0u8; 0x90];
        let mut items = vec![0u8; 0x50];
        let mut sync_obj = vec![0u8; 0x260];

        let mp = mgr.as_mut_ptr() as usize;
        let op = obj0.as_mut_ptr() as usize;
        let ip = items.as_mut_ptr() as usize;
        let sp = sync_obj.as_mut_ptr() as usize;

        wr::<usize>(g_2b8, 0x7777);
        wr::<u32>(g_ff00, 1);
        wr::<usize>(g_298, sp);
        wr::<u32>(sp + 0x258, 10);

        wr::<u32>(op + 0x80, 1);
        wr::<usize>(ip, op);
        wr::<u64>(ip + 0x08, 0x11);
        wr::<u64>(ip + 0x18, 0x22);

        wr::<u32>(mp + 0x2c0, if mode == 0 { u32::MAX } else { 0x1234 });
        wr::<u32>(mp + 0x2c4, if mode == 0 { 0 } else { 1 });
        wr::<u32>(mp + 0x2e8, 0);
        wr::<usize>(mp + 0x2c8, ip);
        wr::<u32>(mp + 0x2d0, if mode == 0 { 1 } else { 0 });

        f(mp);

        (
            rd::<u32>(mp + 0x2c0),
            rd::<u32>(mp + 0x2c4),
            rd::<u32>(mp + 0x2e8),
            rd::<u32>(mp + 0x2d0),
            rd::<u32>(op + 0x80),
            rd::<u32>(g_ff00),
            rd::<u32>(sp + 0x258),
            TRACE.clone(),
        )
    };

    let restore = || {
        wr::<usize>(g_2b8, saved_2b8);
        wr::<u32>(g_ff00, saved_ff00);
        wr::<usize>(g_298, saved_298);
    };

    for mode in 0..2u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        if o != n {
            restore();
        }
        crate::diff_eq!(o, n, "142255340 mode={}", mode);
    }
    restore();
    Ok(2)
}

pub unsafe fn diff_r_142258d40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    const VAS: [u64; 11] = [
        0x1_4397_d85a,
        0x1_4397_d86a,
        0x1_4397_d86e,
        0x1_4397_d873,
        0x1_4397_d87e,
        0x1_4397_d883,
        0x1_4397_d886,
        0x1_4397_d88b,
        0x1_4397_d88e,
        0x1_4397_d896,
        0x1_4397_d89b,
    ];
    let mut saved = [0u8; 11];
    for (i, &va) in VAS.iter().enumerate() {
        saved[i] = rd::<u8>(exe::ptr::<u8>(va) as usize);
    }

    let restore = || {
        for (i, &va) in VAS.iter().enumerate() {
            wr::<u8>(exe::ptr::<u8>(va) as usize, saved[i]);
        }
    };

    let run = |f: F, mask: u16| -> u8 {
        for (i, &va) in VAS.iter().enumerate() {
            wr::<u8>(exe::ptr::<u8>(va) as usize, ((mask >> i) & 1) as u8);
        }
        f()
    };

    for mask in [0u16, 1 << 6, 1 << 8, (1 << 8) | (1 << 2), 1 << 5, 1 << 7, 1 << 0] {
        let o = run(f_o, mask);
        let n = run(f_n, mask);
        if o != n {
            restore();
        }
        crate::diff_eq!(o, n, "142258d40 mask={:#x}", mask);
    }
    restore();
    Ok(7)
}

pub unsafe fn diff_r_14225c9a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: Vec<(u32, usize, u64)> = Vec::new();
    static mut MGR_BASE: usize = 0;
    static mut JOB_MEM: [u64; 4] = [0; 4];
    static mut U16_BUF: [u16; 16] = [0; 16];

    unsafe extern "win64" fn mock_f1e0(rcx: usize, rdx: u64) {
        CALL_LOG.push((1, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_3000(rcx: usize, rdx: u64) {
        CALL_LOG.push((2, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_8600(rcx: usize, rdx: u64) {
        CALL_LOG.push((3, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_2480(rcx: usize, rdx: u64) {
        CALL_LOG.push((4, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_vt28(rcx: usize, rdx: u64) {
        CALL_LOG.push((5, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_2700(rcx: usize, rdx: u64) {
        CALL_LOG.push((6, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_cc50(rcx: usize, rdx: u64) {
        CALL_LOG.push((7, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_e490(rcx: usize, rdx: u64) {
        CALL_LOG.push((8, rcx.wrapping_sub(MGR_BASE), rdx));
    }
    unsafe extern "win64" fn mock_f300(rbx: usize, grow: u64) {
        CALL_LOG.push((9, rbx.wrapping_sub(MGR_BASE), grow));
        wr::<u32>(rbx + 0x0c, grow as u32);
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_pair: usize) {
        wr::<usize>(out_pair, core::ptr::addr_of_mut!(JOB_MEM) as usize);
        wr::<u64>(out_pair + 0x08, 0xCAFE_BEEF);
    }
    unsafe extern "win64" fn mock_6460(_sched: usize, job: usize) {
        CALL_LOG.push((10, 0, rd::<u32>(job) as u64));
    }
    unsafe extern "win64" fn mock_sem(h: usize, cnt: u32, prev: usize) -> i32 {
        CALL_LOG.push((11, h, (cnt as u64) | ((prev as u64) << 32)));
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4124_3000, mock_3000 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_8600, mock_8600 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40a0_2480, mock_2480 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40a0_2700, mock_2700 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_406d_cc50, mock_cc50 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4028_e490, mock_e490 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40f4_f300, mock_f300 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5580, mock_sem as *const ())?;

    let g_2f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<usize>(g_2f8);

    let mut vtbl_50 = [0usize; 8];
    vtbl_50[5] = mock_vt28 as *const () as usize; // +0x28

    let run = |f: F, mode: u8| -> (u64, u64, u64, u64, u32, u32, [u16; 8], [u64; 3], Vec<(u32, usize, u64)>) {
        CALL_LOG.clear();
        JOB_MEM = [0; 4];
        U16_BUF = [0x7777; 16];

        let mut mgr = vec![0u8; 0x680];
        let mp = mgr.as_mut_ptr() as usize;
        MGR_BASE = mp;

        let arr_18: [u64; 4] = match mode {
            0 => [11, 22, 0, 0],
            1 => [11, 22, 33, 0],
            _ => [11, 0, 0, 0],
        };
        wr::<usize>(mp + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(mp + 0x20, 4);
        wr::<usize>(mp + 0x50, vtbl_50.as_ptr() as usize);
        wr::<u64>(mp + 0x4a8, 2);
        wr::<u64>(mp + 0x548, 4);

        wr::<usize>(mp + 0x208, core::ptr::addr_of_mut!(U16_BUF) as usize);
        match mode {
            0 => {
                wr::<u32>(mp + 0x210, 1);
                wr::<u32>(mp + 0x214, 1);
            }
            1 => {
                wr::<u32>(mp + 0x210, 1);
                wr::<u32>(mp + 0x214, 4);
            }
            _ => {
                wr::<u32>(mp + 0x210, 3);
                wr::<u32>(mp + 0x214, 4);
            }
        }

        let mut sem_owner = vec![0u8; 0x1020];
        let sop = sem_owner.as_mut_ptr() as usize;
        wr::<usize>(sop + 0x1008, 0x5555);

        let mut worker_obj = [0u8; 0x80];
        let mut sched = [0u8; 0x80];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sop);
        wr::<usize>(sp + 0x48, worker_obj.as_mut_ptr() as usize);

        let mut top_2f8 = [0u8; 0x20];
        let tp = top_2f8.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x10, sp);
        wr::<usize>(g_2f8, tp);

        f(mp);

        let mut u16_head = [0u16; 8];
        u16_head.copy_from_slice(&U16_BUF[..8]);
        let job_summary = [
            JOB_MEM[0],
            JOB_MEM[1],
            (JOB_MEM[3] as usize).wrapping_sub(mp) as u64,
        ];
        (
            rd::<u64>(mp + 0x4e0),
            rd::<u64>(mp + 0x4e8),
            rd::<u64>(mp + 0x580),
            rd::<u64>(mp + 0x588),
            rd::<u32>(mp + 0x210),
            rd::<u32>(mp + 0x214),
            u16_head,
            job_summary,
            CALL_LOG.clone(),
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        if o != n {
            wr::<usize>(g_2f8, saved_2f8);
        }
        crate::diff_eq!(o, n, "14225c9a0 mode={}", mode);
    }
    wr::<usize>(g_2f8, saved_2f8);
    Ok(3)
}

pub unsafe fn diff_unknown_142268cc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VT10_CALLS: u32 = 0;
    static mut A1E0_CALLS: u32 = 0;
    static mut MOCK_OBJ_PTR: usize = 0;

    unsafe extern "win64" fn mock_vt10(_this: usize) {
        VT10_CALLS += 1;
    }
    unsafe extern "win64" fn mock_d1f0(_rcx: usize, out_ptr: usize, _flag: u8) -> usize {
        wr::<usize>(out_ptr, MOCK_OBJ_PTR);
        out_ptr
    }
    unsafe extern "win64" fn mock_a1e0(_rcx: usize, _rdx: u32, _r8: u8) {
        A1E0_CALLS += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4208_d1f0, mock_d1f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_a1e0, mock_a1e0 as *const ())?;

    let g_fc58 = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let g_2b11c = exe::ptr::<u32>(0x1_43a2_b11c) as usize;
    let g_2b08c = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let g_549ac = exe::ptr::<u32>(0x1_4385_49ac) as usize;
    let g_63738 = exe::ptr::<u32>(0x1_43a6_3738) as usize;
    let g_63710 = exe::ptr::<u32>(0x1_43a6_3710) as usize;
    let g_57fe8 = exe::ptr::<u32>(0x1_4385_7fe8) as usize;
    let g_d82f = exe::ptr::<u8>(0x1_4397_d82f) as usize;
    let g_d82e = exe::ptr::<u8>(0x1_4397_d82e) as usize;
    let g_ff20 = exe::ptr::<u32>(0x1_4397_ff20) as usize;
    let g_d82a = exe::ptr::<u8>(0x1_4397_d82a) as usize;
    let g_d82b = exe::ptr::<u8>(0x1_4397_d82b) as usize;

    let saved = (
        rd::<usize>(g_fc58),
        rd::<u32>(g_2b11c),
        rd::<u32>(g_2b08c),
        rd::<u32>(g_549ac),
        rd::<u32>(g_63738),
        rd::<u32>(g_63710),
        rd::<u32>(g_57fe8),
        rd::<u8>(g_d82f),
        rd::<u8>(g_d82e),
        rd::<u32>(g_ff20),
        rd::<u8>(g_d82a),
        rd::<u8>(g_d82b),
    );

    let restore = || {
        wr::<usize>(g_fc58, saved.0);
        wr::<u32>(g_2b11c, saved.1);
        wr::<u32>(g_2b08c, saved.2);
        wr::<u32>(g_549ac, saved.3);
        wr::<u32>(g_63738, saved.4);
        wr::<u32>(g_63710, saved.5);
        wr::<u32>(g_57fe8, saved.6);
        wr::<u8>(g_d82f, saved.7);
        wr::<u8>(g_d82e, saved.8);
        wr::<u32>(g_ff20, saved.9);
        wr::<u8>(g_d82a, saved.10);
        wr::<u8>(g_d82b, saved.11);
    };

    let mut vtbl = [0usize; 4];
    vtbl[2] = mock_vt10 as *const () as usize;
    let mut mock_obj = [0usize; 2];
    mock_obj[0] = vtbl.as_ptr() as usize;

    let mut run = |f: F, mode: u8| -> (u8, u32, u32, [u8; 4], u32, u32, u32) {
        VT10_CALLS = 0;
        A1E0_CALLS = 0;
        MOCK_OBJ_PTR = if mode == 0 {
            0
        } else {
            mock_obj.as_mut_ptr() as usize
        };

        let mut p1 = vec![0u8; 0x4b0];
        let mut sub48 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        let sp = sub48.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x48, sp);

        wr::<u8>(sp + 0x4c, 0x11);
        wr::<u8>(sp + 0x4f, 0x22);
        wr::<u32>(sp + 0x50, 0x3344_5566);
        wr::<u8>(sp + 0x64, 0x77);
        wr::<u8>(sp + 0x65, 0x88);
        wr::<u8>(sp + 0x66, u8::from(mode == 1));
        wr::<u8>(sp + 0x67, u8::from(mode == 2));

        wr::<u32>(g_2b11c, 1);
        wr::<u32>(g_2b08c, 10);
        wr::<f32>(g_549ac, 1.0);
        wr::<u32>(
            g_63738,
            match mode {
                0 | 2 => 0,
                3 => 2,
                _ => 1,
            },
        );
        wr::<u32>(g_63710, 10);
        wr::<f32>(g_57fe8, if mode == 3 { 2.0 } else { 1.0 });

        f(p1p);

        (
            rd::<u8>(p1p + 0x49d),
            VT10_CALLS,
            A1E0_CALLS,
            [
                rd::<u8>(g_d82f),
                rd::<u8>(g_d82e),
                rd::<u8>(g_d82a),
                rd::<u8>(g_d82b),
            ],
            rd::<u32>(g_ff20),
            rd::<u32>(g_2b11c),
            rd::<u32>(g_549ac),
        )
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        if o != n {
            restore();
        }
        crate::diff_eq!(o, n, "142268cc0 mode={}", mode);
    }
    restore();
    Ok(4)
}
