//! Decompiled `rend` functions from `e18.rs` (`0x14226ccc0..0x1422a3590`).

use crate::exe::{self, call_va, rd, wr};
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

#[inline]
unsafe fn tls_slot0() -> usize {
    let tls_ptr = exe::tls_pointer();
    if tls_ptr == 0 {
        return 0;
    }
    let tls0 = rd::<usize>(tls_ptr);
    if tls0 != 0 && rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    tls0
}

/// `rend::rend_14226ccc0`
/// Original: `0x14226ccc0` (61 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14226ccc0(flag: u8) {
    let counter_addr = exe::ptr::<i32>(0x1_43a2_b19c) as usize;
    let cur = rd::<i32>(counter_addr);
    if cur > 0 && flag == 1 {
        wr::<i32>(counter_addr, cur.wrapping_sub(1));
        let mut local = [0u64; 3];
        let pair = call_va!(0x1_4207_34e0, (local.as_mut_ptr() as usize, 0u32) -> usize);
        let obj = rd::<usize>(pair + 0x8);
        if obj != 0 {
            let val = rd::<u64>(pair);
            call_va!(0x1_4207_02c0, (obj, val) -> ());
        }
    }
}

/// `rend::rend_1422722f0`
/// Original: `0x1422722f0` (78 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422722f0(this: usize) {
    let tls0 = tls_slot0();
    let dst = rd::<usize>(this);
    let val = rd::<u64>(tls0 + 0x6308);
    wr::<u64>(dst, val);
    let arg = exe::ptr::<u8>(0x1_43a3_3b08) as usize;
    call_va!(0x1_4229_d440, (arg) -> ());
}

/// `rend::rend_142272d60`
/// Original: `0x142272d60` (114 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142272d60(
    this: usize,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
) -> usize {
    wr::<u64>(this + 0x18, a3);
    wr::<u64>(this + 0x20, a4);
    wr::<u64>(this + 0x28, a5);
    wr::<u64>(this + 0x30, a6);
    wr::<u64>(this + 0x38, a7);
    wr::<u64>(this + 0x40, a8);
    wr::<u64>(this + 0x48, a9);
    wr::<u64>(this + 0x50, a10);
    wr::<u64>(this + 0x58, a11);
    wr::<u64>(this + 0x60, a12);
    wr::<u64>(this + 0x68, a13);
    wr::<u64>(this, a0);
    wr::<u64>(this + 0x8, a1);
    wr::<u64>(this + 0x10, a2);
    this
}

/// `rend::rend_142279340`
/// Original: `0x142279340` (150 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_142279340(this: usize, pass_data: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4397_d8aa) as usize) == 0 {
        return;
    }
    let root = rd::<usize>(this);
    let target = rd::<usize>(root + 0x108);
    let str_start = exe::ptr::<u8>(0x1_433f_5998) as usize;
    let str_end = exe::ptr::<u8>(0x1_433f_59ab) as usize;
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut cur = str_start;
    while cur != str_end {
        let ch = rd::<i8>(cur) as i64 as u64;
        cur = cur.wrapping_add(1);
        hash = (hash ^ ch).wrapping_mul(0x0100_0000_01b3);
    }
    let mut found: u64 = 0;
    for i in 0..0x40usize {
        let k = rd::<u64>(pass_data + 0x208 + i * 8);
        if k != 0 && k == hash {
            found = rd::<u64>(pass_data + 0x8 + i * 8);
            break;
        }
    }
    call_va!(0x1_4221_7f30, (target, found) -> ());
}

/// `rend::rend_14227bd50`
/// Original: `0x14227bd50` (22 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14227bd50(this: usize, pass_data: usize) {
    let inner = rd::<usize>(this);
    let r8 = rd::<u64>(inner + 0x418);
    let target = rd::<usize>(inner + 0x1b8);
    call_va!(0x1_421e_ad60, (target, pass_data, r8) -> ());
}

/// `rend::rend_142283010`
/// Original: `0x142283010` (93 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_142283010(this: usize, _pass_data: usize) {
    let root0 = rd::<usize>(this);
    if rd::<usize>(root0 + 0x1b8) != 0 {
        call_va!(0x1_421e_a370, (this) -> ());
    }
    let root1 = rd::<usize>(this);
    let p178 = rd::<usize>(root1 + 0x178);
    call_va!(0x1_423e_b360, (p178) -> ());
    let root2 = rd::<usize>(this);
    let p1b8 = rd::<usize>(root2 + 0x1b8);
    if p1b8 != 0 {
        call_va!(0x1_421e_a7c0, (p1b8) -> ());
    }
    let root3 = rd::<usize>(this);
    let p220 = rd::<usize>(root3 + 0x220);
    call_va!(0x1_4234_8710, (p220) -> ());
}

/// `rend::rend_1422833a0`
/// Original: `0x1422833a0` (189 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_1422833a0(this: usize) {
    let p0 = rd::<usize>(this);
    let sub0 = rd::<usize>(p0);
    if sub0 != 0 {
        call_va!(0x1_423a_9860, (sub0) -> ());
    }
    let slot1 = this + 0x8;
    if rd::<u8>(exe::ptr::<u8>(0x1_4397_d85f) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(slot1) + 0xe0);
        let vfn = rd::<usize>(rd::<usize>(obj) + 0x40);
        call_va!(vfn, (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_a1b8) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(slot1) + 0xe0);
        let vfn = rd::<usize>(rd::<usize>(obj) + 0x48);
        call_va!(vfn, (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_9ca0) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(slot1) + 0xe0);
        let vfn = rd::<usize>(rd::<usize>(obj) + 0x40);
        call_va!(vfn, (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_a238) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(slot1) + 0xe8);
        let vfn = rd::<usize>(rd::<usize>(obj) + 0x48);
        call_va!(vfn, (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_a568) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(slot1) + 0xe8);
        let vfn = rd::<usize>(rd::<usize>(obj) + 0x40);
        call_va!(vfn, (obj) -> ());
    }
}

/// `rend::rend_142283490`
/// Original: `0x142283490` (194 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_142283490(this: usize, _pass_data: usize) {
    #[repr(C, align(16))]
    struct AlignedSpan([u64; 2]);

    let root0 = rd::<usize>(this);
    let b8 = rd::<usize>(root0 + 0xb8);
    let v1 = rd::<usize>(this + 0x8);
    let len1 = rd::<u32>(v1 + 0x8);
    let ptr1 = rd::<u64>(v1);
    let tag = exe::ptr::<u8>(0x1_43bb_d1f0) as usize;
    call_va!(0x1_422f_c260, (b8, tag, ptr1, len1, 0x16u32, 0u64) -> ());

    let root1 = rd::<usize>(this);
    let c0 = rd::<usize>(root1 + 0xc0);
    let v2 = rd::<usize>(this + 0x10);
    let len2 = rd::<u32>(v2 + 0x8) as u64;
    let ptr2 = rd::<u64>(v2);
    let mut span2 = AlignedSpan([len2, ptr2]);
    call_va!(0x1_4239_5a90, (c0, len2, span2.0.as_mut_ptr() as usize) -> ());

    let root2 = rd::<usize>(this);
    let c8 = rd::<usize>(root2 + 0xc8);
    let v3 = rd::<usize>(this + 0x18);
    let len3 = rd::<u32>(v3 + 0x8) as u64;
    let ptr3 = rd::<u64>(v3);
    let mut span3 = AlignedSpan([len3, ptr3]);
    call_va!(0x1_4230_1ff0, (c8, span3.0.as_mut_ptr() as usize, ptr3) -> ());

    let root3 = rd::<usize>(this);
    let d8 = rd::<usize>(root3 + 0xd8);
    call_va!(0x1_4237_9240, (d8) -> ());
}

/// `rend::rend_142283570`
/// Original: `0x142283570` (95 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_142283570(this: usize, _pass_data: usize) {
    let job_sys = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let handle_ptr = exe::ptr::<u8>(0x1_43c8_c930) as usize;
    call_va!(0x1_4248_5780, (job_sys, handle_ptr) -> ());
    let query_ptr = exe::ptr::<u8>(0x1_43c8_c908) as usize;
    call_va!(0x1_4205_f870, (1u32, query_ptr) -> ());
    let root = rd::<usize>(this);
    let sub98 = rd::<usize>(root + 0x98);
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_c788) as usize) != 0 {
        let count = rd::<u32>(sub98 + 0x8);
        let fmt = exe::ptr::<u8>(0x1_433f_d650) as usize;
        call_va!(0x1_423f_e030, (fmt, count) -> ());
    }
}

/// `rend::rend_1422835e0`
/// Original: `0x1422835e0` (40 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422835e0(_this: usize, counter_ptr: usize) {
    let job = rd::<usize>(exe::ptr::<usize>(0x1_43c8_c930) as usize);
    if job != 0 {
        let gen = rd::<u16>(job + 0x6) >> 3;
        let expected = rd::<u16>(exe::ptr::<u16>(0x1_43c8_c938) as usize);
        if expected == gen && rd::<u32>(job) != 0 {
            return;
        }
    }
    (*(counter_ptr as *const AtomicI32)).fetch_sub(1, Ordering::SeqCst);
}

/// `rend::rend_142283610`
/// Original: `0x142283610` (4 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142283610(_this: usize, counter_ptr: usize) {
    (*(counter_ptr as *const AtomicI32)).fetch_add(1, Ordering::SeqCst);
}

/// `rend::rend_142289300`
/// Original: `0x142289300` (79 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142289300(
    _this: usize,
    out: usize,
    width: u32,
    height: u32,
    flags: u32,
) -> usize {
    let fmt: u8 = if rd::<u8>(exe::ptr::<u8>(0x1_4386_9c20) as usize) != 0 {
        9
    } else {
        7
    };
    wr::<u32>(out, width);
    wr::<u32>(out + 0x4, height);
    wr::<u8>(out + 0x10, fmt);
    wr::<u32>(out + 0x8, 1);
    wr::<u32>(out + 0xc, 1);
    wr::<[u8; 16]>(out + 0x1c, [0u8; 16]);
    wr::<u32>(out + 0x18, 0);
    wr::<u32>(out + 0x14, flags | 1);
    wr::<u32>(out + 0x2c, 0xffff_ffff);
    out
}

/// `rend::rend_142289370`
/// Original: `0x142289370` (44 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_142289370(this: usize) {
    let root0 = rd::<usize>(this);
    call_va!(0x1_423c_5a30, (rd::<usize>(root0 + 0x1c8)) -> ());
    let root1 = rd::<usize>(this);
    call_va!(0x1_423e_dc80, (rd::<usize>(root1 + 0x178)) -> ());
}

/// `rend::rend_142289b10`
/// Original: `0x142289b10` (69 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142289b10(
    this: usize,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
) -> usize {
    wr::<u64>(this + 0x18, a3);
    wr::<u64>(this + 0x20, a4);
    wr::<u64>(this + 0x28, a5);
    wr::<u64>(this + 0x30, a6);
    wr::<u64>(this + 0x38, a7);
    wr::<u64>(this + 0x40, a8);
    wr::<u64>(this, a0);
    wr::<u64>(this + 0x8, a1);
    wr::<u64>(this + 0x10, a2);
    this
}

/// `rend::rend_14228aa10`
/// Original: `0x14228aa10` (113 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14228aa10(this: usize) {
    let mut local = [0u64; 3];
    let local_addr = local.as_mut_ptr() as usize;
    let g_fc60 = rd::<u64>(exe::ptr::<u64>(0x1_4397_fc60) as usize);
    wr::<u64>(local_addr + 0x10, g_fc60);
    wr::<u64>(local_addr, 0);
    wr::<u32>(local_addr + 0x8, 1);
    call_va!(0x1_4205_fdc0, (local_addr) -> ());
    call_va!(0x1_4209_3110, () -> ());
    let root0 = rd::<usize>(this);
    call_va!(0x1_4233_6540, (rd::<usize>(root0 + 0x98)) -> ());
    let root1 = rd::<usize>(this);
    call_va!(0x1_423c_ff40, (rd::<usize>(root1 + 0xa0)) -> ());
    let g_fcb8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    call_va!(0x1_4209_3200, (g_fcb8) -> ());
    let q = call_va!(0x1_4205_fad0, () -> u64);
    wr::<u64>(exe::ptr::<u64>(0x1_43c8_c908) as usize, q);
}

/// `rend::rend_14228aa90`
/// Original: `0x14228aa90` (212 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14228aa90(this: usize) {
    let p8 = rd::<usize>(this + 0x8);
    call_va!(0x1_4233_6b40, (rd::<usize>(p8 + 0x98)) -> ());
    let job_sys = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let rbx = rd::<u64>(this + 0x8);
    let rdi = rd::<usize>(job_sys + 0x10);
    let worker_idx = call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool = rd::<usize>(rdi + (worker_idx << 6) + 0x48).wrapping_add(0x40);
    let mut handle = [0u8; 16];
    let handle_ptr = handle.as_mut_ptr() as usize;
    call_va!(0x1_4248_5aa0, (pool, handle_ptr) -> ());
    let node0 = rd::<usize>(handle_ptr);
    (*(node0 as *const AtomicI32)).swap(1, Ordering::SeqCst);
    let node = rd::<usize>(handle_ptr);
    let vftbl = exe::ptr::<u8>(0x1_433f_38f8) as usize as u64;
    let handle_bytes = rd::<[u8; 16]>(handle_ptr);
    wr::<u64>(node + 0x8, 0);
    wr::<u64>(node + 0x18, rbx);
    let job_sys2 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    wr::<u64>(node + 0x10, vftbl);
    wr::<[u8; 16]>(exe::ptr::<u8>(0x1_43c8_c930) as usize, handle_bytes);
    if rd::<u32>(node) == 1 {
        let sched = rd::<usize>(job_sys2 + 0x10);
        let job_arg = rd::<u64>(exe::ptr::<u64>(0x1_43c8_c930) as usize);
        call_va!(0x1_4248_6460, (sched, job_arg) -> ());
        let sem_holder = rd::<usize>(sched);
        let sem = rd::<u64>(sem_holder + 0x1008);
        exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> ());
    }
    let counter_ptr = rd::<usize>(this);
    (*(counter_ptr as *const AtomicI32)).fetch_sub(1, Ordering::SeqCst);
}

/// `rend::rend_14228ac50`
/// Original: `0x14228ac50` (192 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14228ac50(this: usize, pass_data: usize) {
    let flag = rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize) as u32;
    let mgr0 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4226_2470, (mgr0, flag) -> ());
    let mgr1 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4226_20f0, (mgr1) -> ());
    let mgr2 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4226_2310, (mgr2) -> ());
    let mgr3 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    let count = rd::<u32>(mgr3 + 0x20) as usize;
    let byte_len = count.wrapping_mul(12);
    let src = rd::<usize>(mgr3 + 0x28);
    let dst = rd::<usize>(mgr3 + 0x5f0);
    call_va!(0x1_4183_acbc, (dst, src, byte_len) -> usize);
    let mgr4 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4226_1da0, (mgr4) -> ());
    let mgr5 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4225_e6c0, (mgr5.wrapping_add(0x2d0)) -> ());
    let sub0 = rd::<usize>(rd::<usize>(this) + 0x1b0);
    call_va!(0x1_4222_e360, (sub0) -> ());
    let sub1 = rd::<usize>(rd::<usize>(this) + 0x1b0);
    call_va!(0x1_4222_dca0, (sub1, pass_data) -> ());
}

/// `rend::rend_14228ad30`
/// Original: `0x14228ad30` (48 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14228ad30(this: usize) -> usize {
    let mgr0 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4226_11b0, (mgr0) -> ());
    let mgr1 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_412f_7fb0, (mgr1.wrapping_add(0x38)) -> usize)
}

/// `rend::rend_14228adb0`
/// Original: `0x14228adb0` (66 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14228adb0(this: usize) {
    let mgr0 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4226_26d0, (mgr0) -> ());
    let mgr1 = rd::<usize>(rd::<usize>(this) + 0x1a8);
    call_va!(0x1_4225_91f0, (mgr1) -> ());
    let sub1e8 = rd::<usize>(rd::<usize>(this) + 0x1e8);
    call_va!(0x1_4230_be30, (sub1e8) -> ());
}

/// `rend::rend_14228b010`
/// Original: `0x14228b010` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14228b010(this: usize) {
    let root = rd::<usize>(this + 0x8);
    let mgr = rd::<usize>(root + 0x1a8);
    call_va!(0x1_4226_1520, (mgr) -> ());
    call_va!(0x1_4234_18c0, () -> ());
    let counter_ptr = rd::<usize>(this);
    (*(counter_ptr as *const AtomicI32)).fetch_sub(1, Ordering::SeqCst);
}

/// `rend::rend_142294cc0`
/// Original: `0x142294cc0` (136 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142294cc0(this: usize) {
    let val = rd::<u64>(this);
    let tls_ptr = exe::tls_pointer();
    let tls0 = rd::<usize>(tls_ptr);
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    let g_14c = exe::ptr::<u32>(0x1_43a0_014c) as usize;
    let g_150 = exe::ptr::<u32>(0x1_43a0_0150) as usize;
    wr::<u32>(g_14c, 0);
    wr::<u32>(g_150, 1);
    wr::<u64>(tls0 + 0x6308, val);
    let mut edx = 0u32;
    let mut r8d = 1u32;
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
        edx = rd::<u32>(g_14c);
        r8d = rd::<u32>(g_150);
    }
    let rcx = rd::<usize>(tls0 + 0x6308);
    call_va!(0x1_421f_0110, (rcx, edx, r8d) -> ());
}

/// `rend::rend_142298200`
/// Original: `0x142298200` (58 bytes, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_142298200(arr: usize, idx: i32, val: f32) {
    rend_142298200_impl(arr, idx, val);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_142298200(
    arr: usize,
    idx: i32,
    _xmm0: f32,
    _xmm1: f32,
    val: f32,
) {
    rend_142298200_impl(arr, idx, val);
}

#[inline(always)]
unsafe fn rend_142298200_impl(arr: usize, idx: i32, val: f32) {
    let mask = rd::<u32>(exe::ptr::<u32>(0x1_4342_8760) as usize);
    let elem_addr = (arr as isize).wrapping_add((idx as isize).wrapping_mul(16)) as usize;
    let mut v = rd::<[u32; 4]>(elem_addr);
    let neg_val = f32::from_bits(val.to_bits() ^ mask);
    let w = f32::from_bits(v[3]);
    let diff = neg_val - w;
    v[3] = diff.to_bits() ^ mask;
    wr::<[u32; 4]>(elem_addr, v);
}

/// `rend::rend_142298590`
/// Original: `0x142298590` (44 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_142298590(elem_a: usize, elem_b: usize) -> usize {
    let vec_a = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6698) as usize);
    call_va!(0x1_4130_4370, (vec_a, elem_a) -> usize);
    let vec_b = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6648) as usize);
    call_va!(0x1_412f_ff40, (vec_b, elem_b) -> usize)
}

/// `rend::rend_1422985d0`
/// Original: `0x1422985d0` (37 bytes, `Throws::No`, `FLOAT_RETURN`).
pub unsafe extern "win64" fn rend_1422985d0(this: usize) -> f64 {
    let d3_bits = rd::<u64>(this + 0x140);
    let d3 = f64::from_bits(d3_bits);
    let mask = rd::<u64>(exe::ptr::<u64>(0x1_4342_8740) as usize);
    let d0 = f64::from_bits(d3_bits ^ mask);
    let denom = rd::<f64>(this + 0x120);
    let d1 = d0 / denom;
    let d2 = d1 * d3;
    let sum = d1 + d3;
    d2 / sum
}

/// `rend::rend_142298600`
/// Original: `0x142298600` (100 bytes, `Throws::No`, `FLOAT_RETURN`).
pub unsafe extern "win64" fn rend_142298600(_this: usize, index: u32) -> f32 {
    let mut r8d = index;
    let inv3 = rd::<f32>(exe::ptr::<f32>(0x1_4342_66dc) as usize);
    let mut scale = inv3;
    let mut acc = 0.0f32;
    while r8d != 0 {
        let q = (((r8d as u64).wrapping_mul(0xaaaa_aaab)) >> 33) as u32;
        let rem = r8d.wrapping_sub(q.wrapping_mul(3));
        let rem_f = (rem as u64) as f32;
        let term = rem_f * scale;
        let cur_f = (r8d as u64) as f32;
        acc += term;
        let next_f = cur_f * inv3;
        r8d = (next_f as i64) as u32;
        scale *= inv3;
    }
    acc
}

/// `rend::rend_142298670`
/// Original: `0x142298670` (93 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142298670(_this: usize, x: u32) -> u32 {
    let mut v = (x << 16) | (x >> 16);
    v = ((v & 0xd555_5555).wrapping_shl(1)) | ((v >> 1) & 0x5555_5555);
    v = ((v & 0xf333_3333).wrapping_shl(2)) | ((v >> 2) & 0x3333_3333);
    v = ((v & 0xff0f_0f0f).wrapping_shl(4)) | ((v >> 4) & 0x0f0f_0f0f);
    ((v & 0xffff_00ff).wrapping_shl(8)) | ((v >> 8) & 0x00ff_00ff)
}

/// `rend::rend_142299a80`
/// Original: `0x142299a80` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142299a80() {
    let g_d38 = exe::ptr::<u8>(0x1_4386_9d38) as usize;
    let g_8da = exe::ptr::<u8>(0x1_4397_d8da) as usize;
    let g_e28 = exe::ptr::<u32>(0x1_4386_9e28) as usize;
    let g_198 = exe::ptr::<u32>(0x1_43a2_b198) as usize;
    let g_d34 = exe::ptr::<u8>(0x1_4386_9d34) as usize;

    let count = if rd::<u8>(g_d38) != 0 && (rd::<u8>(g_8da) & 1) != 0 {
        let v = rd::<u32>(g_e28);
        wr::<u32>(g_198, v);
        wr::<u8>(g_d38, 0);
        wr::<u8>(g_d34, 1);
        v
    } else {
        rd::<u32>(g_198)
    };
    if count != 0 {
        wr::<u32>(g_198, count.wrapping_sub(1));
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("int3", options(nomem, nostack));
    }
}

/// `rend::rend_14229ca60`
/// Original: `0x14229ca60` (235 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14229ca60(this: usize) {
    let g_cc7 = exe::ptr::<u8>(0x1_43a2_8cc7) as usize;
    let edi = rd::<u32>(exe::ptr::<u32>(0x1_4397_ff28) as usize);
    if rd::<u8>(g_cc7) != 0 {
        if rd::<u32>(this + 0x694) == 3 || rd::<u32>(this + 0x5c0) == 1 {
            call_va!(0x1_4238_d420, (rd::<usize>(this + 0x48)) -> ());
        }
        let p58 = rd::<usize>(this + 0x58);
        if p58 != 0 {
            call_va!(0x1_4225_77e0, (p58) -> ());
        }
        let mut out_mode = edi;
        let mut set_cc4 = true;
        let mut call_5c4b0 = false;
        if edi == 2 {
            let h = rd::<u32>(this + 0x67c);
            let w = rd::<u32>(this + 0x678);
            let r8d = rd::<u32>(exe::ptr::<u32>(0x1_43a3_3b9c) as usize);
            let edx = rd::<u32>(exe::ptr::<u32>(0x1_43a3_3b98) as usize);
            let ok = call_va!(0x1_4209_0890, (0usize, edx, r8d, w, h) -> u8);
            if ok != 0 {
                if rd::<u32>(this + 0x654) == 1 {
                    call_5c4b0 = true;
                }
            } else {
                out_mode = 0;
                set_cc4 = false;
            }
        } else if edi == 3 || rd::<u32>(this + 0x654) == 1 {
            call_5c4b0 = true;
        }
        if call_5c4b0 {
            let p48 = rd::<usize>(this + 0x48);
            let w = rd::<u32>(this + 0x678) as u64;
            let h = rd::<u32>(this + 0x67c) as u64;
            let r9 = rd::<u64>(exe::ptr::<u64>(0x1_43a3_3b98) as usize);
            let r8 = (h << 32) | w;
            let idx = rd::<i32>(p48 + 0x38) as isize;
            let sub = rd::<usize>((p48 as isize).wrapping_add(idx.wrapping_mul(8)).wrapping_add(0x8) as usize);
            let fmt_byte = rd::<u8>(sub + 0x18) as u32;
            call_va!(0x1_4205_c4b0, (fmt_byte, 0usize, r8, r9) -> ());
        }
        if set_cc4 {
            wr::<u8>(exe::ptr::<u8>(0x1_43a2_8cc4) as usize, 1);
        }
        wr::<u32>(exe::ptr::<u32>(0x1_4397_ff30) as usize, out_mode);
        wr::<u8>(g_cc7, 0);
    }
    wr::<u32>(this + 0x5c0, rd::<u32>(this + 0x654));
    wr::<u32>(this + 0x694, edi);
}

/// `rend::rend_14229cb60`
/// Original: `0x14229cb60` (91 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14229cb60(this: usize) {
    let d1 = u8::from(rd::<u32>(this + 0x5c0) != rd::<u32>(this + 0x654));
    let d2 = u8::from(rd::<u32>(exe::ptr::<u32>(0x1_4397_ff28) as usize) != rd::<u32>(this + 0x694));
    let c28_qword = rd::<u64>(exe::ptr::<u64>(0x1_43a3_3c28) as usize);
    let g_cc7 = exe::ptr::<u8>(0x1_43a2_8cc7) as usize;
    let g_b98 = exe::ptr::<u64>(0x1_43a3_3b98) as usize;
    let b98_lo = rd::<u32>(g_b98);
    let b98_hi = rd::<u32>(g_b98 + 0x4);
    let c28_hi = rd::<u32>(exe::ptr::<u32>(0x1_43a3_3c2c) as usize);
    let d3 = u8::from(b98_lo != (c28_qword as u32) || b98_hi != c28_hi);
    let new_cc7 = d1 | d2 | rd::<u8>(g_cc7) | d3;
    wr::<u8>(g_cc7, new_cc7);
    if new_cc7 != 0 {
        wr::<u64>(g_b98, c28_qword);
    }
}

/// `rend::rend_14229cbd0`
/// Original: `0x14229cbd0` (29 bytes, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_14229cbd0(_this: usize, dt: f32) {
    rend_14229cbd0_impl(dt);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_14229cbd0(_this: usize, _xmm0: f32, dt: f32) {
    rend_14229cbd0_impl(dt);
}

#[inline(always)]
unsafe fn rend_14229cbd0_impl(dt: f32) {
    let g_170 = exe::ptr::<f64>(0x1_43a0_0170) as usize;
    let g_178 = exe::ptr::<f32>(0x1_43a0_0178) as usize;
    wr::<f64>(g_170, (dt as f64) + rd::<f64>(g_170));
    wr::<f32>(g_178, dt);
}

/// `rend::rend_14229cc00`
/// Original: `0x14229cc00` (55 bytes, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_14229cc00(_this: usize, vec: usize, val: f32) {
    rend_14229cc00_impl(vec, val);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_14229cc00(
    _this: usize,
    vec: usize,
    _xmm0: f32,
    _xmm1: f32,
    val: f32,
) {
    rend_14229cc00_impl(vec, val);
}

#[inline(always)]
unsafe fn rend_14229cc00_impl(vec: usize, val: f32) {
    let count = rd::<u32>(vec + 0x8) as usize;
    let start = rd::<usize>(vec);
    let rcx_arg = count.wrapping_mul(5);
    let end = start.wrapping_add(rcx_arg.wrapping_mul(4));
    call_va!(0x1_412f_1b40, (rcx_arg, start, end) -> ());
    wr::<f32>(exe::ptr::<f32>(0x1_4384_170c) as usize, val);
}

/// `rend::rend_14229cc40`
/// Original: `0x14229cc40` (97 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14229cc40(this: usize, val: u64) {
    let sync0 = rd::<usize>(this + 0x5b8);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize, val);
    wr::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize, val as u32);
    if sync0 != 0 {
        (*(sync0 as *const AtomicU64)).swap(val, Ordering::SeqCst);
        let sync = rd::<usize>(this + 0x5b8);
        exe::call_iat!(0x1_4264_55a8, (sync + 0x10) -> ());
        wr::<u32>(sync + 0x18, rd::<u32>(sync + 0x18).wrapping_add(1));
        exe::call_iat!(0x1_4264_5598, (sync + 0x10) -> ());
        exe::call_iat!(0x1_4264_5590, (sync + 0x8) -> ());
    }
}

/// `rend::rend_1422a3020`
/// Original: `0x1422a3020` (193 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422a3020() {
    let a = call_va!(0x1_411d_7e20, () -> u64);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_79b8) as usize, a);
    let b = call_va!(0x1_411d_7e20, () -> u64);
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_83e8) as usize, b);
    let tls0 = rd::<usize>(exe::tls_pointer());
    let c = call_va!(0x1_423c_2c50, () -> u64);
    wr::<u64>(tls0 + 0x6358, c);
    call_va!(0x1_4206_0410, (exe::ptr::<u8>(0x1_4384_1710) as usize) -> ());
    let mut tmp: u64 = 0;
    let tmp_ptr = (&mut tmp as *mut u64) as usize;
    let res_ptr = call_va!(0x1_412f_ad60, (tmp_ptr) -> usize);
    let new_ptr = rd::<u64>(res_ptr);
    wr::<u64>(res_ptr, 0);
    let g_728 = exe::ptr::<u64>(0x1_43a3_6728) as usize;
    let old_ptr = rd::<u64>(g_728);
    wr::<u64>(g_728, new_ptr);
    if old_ptr != 0 {
        call_va!(0x1_4130_2c80, (new_ptr, old_ptr) -> ());
    }
    let rem = rd::<u64>(tmp_ptr);
    if rem != 0 {
        call_va!(0x1_4130_2c80, (0usize, rem) -> ());
    }
    let cur_728 = rd::<usize>(g_728);
    let slot_val = call_va!(0x1_412f_9dd0, (cur_728) -> u64);
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    wr::<u64>(tls0 + 0x6338, slot_val);
    call_va!(0x1_4206_0410, (exe::ptr::<u8>(0x1_4386_15a8) as usize) -> ());
}

/// `anonymous_namespace::RendThreading::vfunc_0`
/// Original: `0x1422a3180` (73 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_threading_vfunc_0_1422a3180() {
    let g_728 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6728) as usize);
    let val = call_va!(0x1_412f_9dd0, (g_728) -> u64);
    let tls0 = tls_slot0();
    wr::<u64>(tls0 + 0x6338, val);
}

/// `rend::rend_1422a31d0`
/// Original: `0x1422a31d0` (126 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422a31d0() {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
    let lock = rsi + 0x8;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let mut edi: u32 = 0;
    if rd::<u32>(rsi + 0x18) > 0 {
        let mut rdx_arr = rd::<usize>(rsi + 0x10);
        let mut idx = 0usize;
        loop {
            let item = rd::<usize>(rdx_arr + idx * 8);
            wr::<u8>(item + 0x6d, rd::<u8>(item + 0x6d) & !0x2);
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412d_0a20, (mgr, item) -> ());
            edi = edi.wrapping_add(1);
            let len = rd::<u32>(rsi + 0x18) as usize;
            idx = (edi as i32 as isize) as usize;
            rdx_arr = rd::<usize>(rsi + 0x10);
            if idx >= len {
                break;
            }
        }
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// `rend::rend_1422a3260`
/// Original: `0x1422a3260` (126 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422a3260(_this: usize) {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
    let lock = rsi + 0x8;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let mut edi: u32 = 0;
    if rd::<u32>(rsi + 0x18) > 0 {
        let mut rdx_arr = rd::<usize>(rsi + 0x10);
        let mut idx = 0usize;
        loop {
            let item = rd::<usize>(rdx_arr + idx * 8);
            wr::<u8>(item + 0x6d, rd::<u8>(item + 0x6d) & !0x2);
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412d_0a20, (mgr, item) -> ());
            edi = edi.wrapping_add(1);
            let len = rd::<u32>(rsi + 0x18) as usize;
            idx = (edi as i32 as isize) as usize;
            rdx_arr = rd::<usize>(rsi + 0x10);
            if idx >= len {
                break;
            }
        }
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// `rend::rend_1422a32f0`
/// Original: `0x1422a32f0` (172 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422a32f0() {
    #[repr(C, align(16))]
    struct AlignedReq([u8; 16]);

    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
    let lock = rsi + 0x8;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let mut ebp: u32 = 0;
    if rd::<u32>(rsi + 0x18) > 0 {
        let mut rdi_arr = rd::<usize>(rsi + 0x10);
        let mut idx = 0usize;
        loop {
            let item = rd::<u64>(rdi_arr + idx * 8);
            let mgr0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412d_0b70, (mgr0, item) -> ());
            let mut req = AlignedReq([0u8; 16]);
            let req_addr = req.0.as_mut_ptr() as usize;
            wr::<u64>(req_addr, item);
            wr::<u32>(req_addr + 0x8, 0);
            wr::<u8>(req_addr + 0xc, 3);
            let mgr1 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412d_0d00, (mgr1, req_addr) -> ());
            ebp = ebp.wrapping_add(1);
            let len = rd::<u32>(rsi + 0x18) as usize;
            idx = (ebp as i32 as isize) as usize;
            rdi_arr = rd::<usize>(rsi + 0x10);
            if idx >= len {
                break;
            }
        }
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// `rend::rend_1422a33b0`
/// Original: `0x1422a33b0` (188 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422a33b0(_this: usize) {
    #[repr(C, align(16))]
    struct AlignedReq([u8; 16]);

    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
    let lock = rsi + 0x8;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let mut ebp: u32 = 0;
    if rd::<u32>(rsi + 0x18) > 0 {
        let mut rdi_arr = rd::<usize>(rsi + 0x10);
        let mut idx = 0usize;
        loop {
            let item = rd::<u64>(rdi_arr + idx * 8);
            let mgr0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412d_0b70, (mgr0, item) -> ());
            let mut req = AlignedReq([0u8; 16]);
            let req_addr = req.0.as_mut_ptr() as usize;
            wr::<u64>(req_addr, item);
            wr::<u32>(req_addr + 0x8, 0);
            wr::<u8>(req_addr + 0xc, 3);
            let mgr1 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412d_0d00, (mgr1, req_addr) -> ());
            ebp = ebp.wrapping_add(1);
            let len = rd::<u32>(rsi + 0x18) as usize;
            idx = (ebp as i32 as isize) as usize;
            rdi_arr = rd::<usize>(rsi + 0x10);
            if idx >= len {
                break;
            }
        }
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// `rend::rend_1422a3490`
/// Original: `0x1422a3490` (235 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_1422a3490(this: usize, arg2: usize, flag: u8) {
    let r10 = rd::<usize>(this + 0x48);
    let mut xmm1 = rd::<f32>(exe::ptr::<f32>(0x1_4342_7080) as usize);
    let mut edx: u32 = 1;
    let mut eax: u32 = 1;
    let r9d: u32;

    if rd::<u8>(r10 + 0x4c) != 0 {
        let r11b = rd::<u8>(r10 + 0x66);
        let ecx = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
        let cond = if ecx == 0 {
            rd::<u8>(r10 + 0x67) == 0
        } else {
            ecx == 2
        };
        if r11b != 0 || cond {
            r9d = 0;
        } else {
            r9d = 2;
        }
        if r11b == 0 && !cond {
            eax = 4;
        }
    } else if rd::<u8>(r10 + 0x66) == 0 {
        xmm1 = rd::<f32>(exe::ptr::<f32>(0x1_4384_1640) as usize);
        r9d = 4;
    } else {
        r9d = 0;
    }

    if flag == 0 || rd::<u8>(r10 + 0x4c) == 0 {
        edx = 0;
    }

    let idx = rd::<i32>(r10 + 0x38) as isize;
    let rcx_arg = rd::<usize>((r10 as isize).wrapping_add(idx.wrapping_mul(8)).wrapping_add(0x20) as usize);
    let xmm0 = rd::<f32>(exe::ptr::<f32>(0x1_4384_1638) as usize);
    let f: unsafe extern "win64" fn(usize, usize, u8, u32, u32, u32, u32, u32, usize) =
        exe::func(0x1_422a_49d0);
    f(
        rcx_arg,
        arg2,
        1,
        r9d,
        eax,
        xmm1.to_bits(),
        edx,
        0,
        xmm0.to_bits() as usize,
    );
}

/// `rend::rend_1422a3590`
/// Original: `0x1422a3590` (152 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_1422a3590(this: usize, out: usize) -> usize {
    let init_val = rd::<u32>(exe::ptr::<u32>(0x1_42ce_ff00) as usize);
    wr::<u32>(out, init_val);
    let p458 = rd::<usize>(this + 0x458);
    if p458 != 0 {
        let mut data_ptr: usize = 0;
        let mut byte_len: u64 = 0;
        let ok = call_va!(
            0x1_420a_0490,
            (
                p458,
                (&mut data_ptr as *mut usize) as usize,
                (&mut byte_len as *mut u64) as usize,
            ) -> u8
        );
        if ok != 0 && (byte_len >> 2) != 0 {
            let raw = rd::<u32>(data_ptr);
            wr::<u32>(out, raw);
            let b2 = rd::<u8>(out + 0x2);
            wr::<u8>(out, b2);
            wr::<u8>(out + 0x2, raw as u8);
            let holder = rd::<usize>(this + 0x458);
            let slot = rd::<usize>(holder);
            if slot != 0 {
                let inner = rd::<usize>(slot + 0x10);
                call_va!(0x1_4209_d6f0, (inner, 0u32) -> ());
                wr::<usize>(holder, 0);
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_rend_142272d60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
    ) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut o = [0u64; 14];
    let mut n = [0u64; 14];
    let r_o = f_o(
        o.as_mut_ptr() as usize,
        10,
        20,
        30,
        40,
        50,
        60,
        70,
        80,
        90,
        100,
        110,
        120,
        130,
        140,
    );
    let r_n = f_n(
        n.as_mut_ptr() as usize,
        10,
        20,
        30,
        40,
        50,
        60,
        70,
        80,
        90,
        100,
        110,
        120,
        130,
        140,
    );
    crate::diff_eq!(r_o == o.as_ptr() as usize, r_n == n.as_ptr() as usize, "ret");
    crate::diff_eq!(o, n, "rend_142272d60");
    Ok(1)
}

unsafe extern "win64" fn mock_vfn_40(this: usize) {
    wr::<u32>(this + 0x8, rd::<u32>(this + 0x8).wrapping_add(0x11));
}

unsafe extern "win64" fn mock_vfn_48(this: usize) {
    wr::<u32>(this + 0x8, rd::<u32>(this + 0x8).wrapping_add(0x100));
}

pub unsafe fn diff_rend_1422833a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 10];
    vtbl[8] = mock_vfn_40 as *const () as usize;
    vtbl[9] = mock_vfn_48 as *const () as usize;

    let g0 = exe::ptr::<u8>(0x1_4397_d85f) as usize;
    let g1 = exe::ptr::<u8>(0x1_4386_a1b8) as usize;
    let g2 = exe::ptr::<u8>(0x1_4386_9ca0) as usize;
    let g3 = exe::ptr::<u8>(0x1_4386_a238) as usize;
    let g4 = exe::ptr::<u8>(0x1_4386_a568) as usize;
    let saved = [rd::<u8>(g0), rd::<u8>(g1), rd::<u8>(g2), rd::<u8>(g3), rd::<u8>(g4)];

    wr::<u8>(g0, 1);
    wr::<u8>(g1, 1);
    wr::<u8>(g2, 0);
    wr::<u8>(g3, 1);
    wr::<u8>(g4, 1);

    let mut obj_e0_o = [vtbl.as_ptr() as usize, 0usize];
    let mut obj_e8_o = [vtbl.as_ptr() as usize, 0usize];
    let mut holder_o = [0u8; 0xf0];
    wr::<usize>(holder_o.as_mut_ptr() as usize + 0xe0, obj_e0_o.as_mut_ptr() as usize);
    wr::<usize>(holder_o.as_mut_ptr() as usize + 0xe8, obj_e8_o.as_mut_ptr() as usize);
    let null_sub = 0usize;
    let this_o = [(&null_sub as *const usize) as usize, holder_o.as_mut_ptr() as usize];

    let mut obj_e0_n = [vtbl.as_ptr() as usize, 0usize];
    let mut obj_e8_n = [vtbl.as_ptr() as usize, 0usize];
    let mut holder_n = [0u8; 0xf0];
    wr::<usize>(holder_n.as_mut_ptr() as usize + 0xe0, obj_e0_n.as_mut_ptr() as usize);
    wr::<usize>(holder_n.as_mut_ptr() as usize + 0xe8, obj_e8_n.as_mut_ptr() as usize);
    let this_n = [(&null_sub as *const usize) as usize, holder_n.as_mut_ptr() as usize];

    f_o(this_o.as_ptr() as usize);
    f_n(this_n.as_ptr() as usize);

    wr::<u8>(g0, saved[0]);
    wr::<u8>(g1, saved[1]);
    wr::<u8>(g2, saved[2]);
    wr::<u8>(g3, saved[3]);
    wr::<u8>(g4, saved[4]);

    crate::diff_eq!(obj_e0_o[1], obj_e0_n[1], "rend_1422833a0 e0");
    crate::diff_eq!(obj_e8_o[1], obj_e8_n[1], "rend_1422833a0 e8");
    Ok(1)
}

pub unsafe fn diff_rend_1422835e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_930 = exe::ptr::<u8>(0x1_43c8_c930) as usize;
    let saved = rd::<[u8; 16]>(g_930);

    // Case 1: null job pointer -> decrements
    wr::<[u8; 16]>(g_930, [0u8; 16]);
    let mut c_o = 5i32;
    let mut c_n = 5i32;
    f_o(0, (&mut c_o as *mut i32) as usize);
    f_n(0, (&mut c_n as *mut i32) as usize);
    let res1 = (c_o, c_n);

    // Case 2: active job pointer with matching generation and non-zero state -> skips decrement
    let mut fake_job = [1u32, (7u32 << 19)];
    wr::<usize>(g_930, fake_job.as_mut_ptr() as usize);
    wr::<u16>(g_930 + 0x8, 7u16);
    f_o(0, (&mut c_o as *mut i32) as usize);
    f_n(0, (&mut c_n as *mut i32) as usize);
    let res2 = (c_o, c_n);

    wr::<[u8; 16]>(g_930, saved);
    crate::diff_eq!(res1.0, res1.1, "rend_1422835e0 null");
    crate::diff_eq!(res2.0, res2.1, "rend_1422835e0 active");
    Ok(2)
}

pub unsafe fn diff_rend_142283610(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut c_o = 10i32;
    let mut c_n = 10i32;
    f_o(0, (&mut c_o as *mut i32) as usize);
    f_n(0, (&mut c_n as *mut i32) as usize);
    crate::diff_eq!(c_o, c_n, "rend_142283610");
    Ok(1)
}

pub unsafe fn diff_rend_142289300(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32, u32, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let g_c20 = exe::ptr::<u8>(0x1_4386_9c20) as usize;
    let saved = rd::<u8>(g_c20);
    for flag in [0u8, 1u8] {
        wr::<u8>(g_c20, flag);
        let mut o = [0x5au8; 0x30];
        let mut n = [0x5au8; 0x30];
        f_o(0, o.as_mut_ptr() as usize, 1920, 1080, 0x10);
        f_n(0, n.as_mut_ptr() as usize, 1920, 1080, 0x10);
        if o != n {
            wr::<u8>(g_c20, saved);
            crate::diff_eq!(o, n, "rend_142289300");
        }
    }
    wr::<u8>(g_c20, saved);
    Ok(2)
}

pub unsafe fn diff_rend_142289b10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64, u64, u64, u64, u64, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut o = [0u64; 9];
    let mut n = [0u64; 9];
    let r_o = f_o(o.as_mut_ptr() as usize, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    let r_n = f_n(n.as_mut_ptr() as usize, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    crate::diff_eq!(r_o == o.as_ptr() as usize, r_n == n.as_ptr() as usize, "ret");
    crate::diff_eq!(o, n, "rend_142289b10");
    Ok(1)
}

pub unsafe fn diff_rend_14228ad30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut words_o = [0x101u64, 0xfff0_0000_0000_0001u64];
    let mut words2_o = [0u64; 2];
    let idx_map = [-1i32; 128];
    let mut mgr_o = [0u8; 0x600];
    let mgr_o_addr = mgr_o.as_mut_ptr() as usize;
    wr::<usize>(mgr_o_addr + 0x38, words_o.as_mut_ptr() as usize);
    wr::<u64>(mgr_o_addr + 0x48, 100);
    wr::<usize>(mgr_o_addr + 0x180, words2_o.as_mut_ptr() as usize);
    wr::<usize>(mgr_o_addr + 0x198, idx_map.as_ptr() as usize);
    let mut root_o = [0u8; 0x1b0];
    wr::<usize>(root_o.as_mut_ptr() as usize + 0x1a8, mgr_o_addr);
    let this_o = [root_o.as_mut_ptr() as usize];

    let mut words_n = [0x101u64, 0xfff0_0000_0000_0001u64];
    let mut words2_n = [0u64; 2];
    let mut mgr_n = [0u8; 0x600];
    let mgr_n_addr = mgr_n.as_mut_ptr() as usize;
    wr::<usize>(mgr_n_addr + 0x38, words_n.as_mut_ptr() as usize);
    wr::<u64>(mgr_n_addr + 0x48, 100);
    wr::<usize>(mgr_n_addr + 0x180, words2_n.as_mut_ptr() as usize);
    wr::<usize>(mgr_n_addr + 0x198, idx_map.as_ptr() as usize);
    let mut root_n = [0u8; 0x1b0];
    wr::<usize>(root_n.as_mut_ptr() as usize + 0x1a8, mgr_n_addr);
    let this_n = [root_n.as_mut_ptr() as usize];

    let r_o = f_o(this_o.as_ptr() as usize);
    let r_n = f_n(this_n.as_ptr() as usize);
    crate::diff_eq!(r_o == mgr_o_addr + 0x38, r_n == mgr_n_addr + 0x38, "ret");
    crate::diff_eq!(words_o, words_n, "rend_14228ad30 words");
    Ok(1)
}

pub unsafe fn diff_rend_14228b010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_b40 = exe::ptr::<u64>(0x1_43bb_db40) as usize;
    let g_ac8 = exe::ptr::<u64>(0x1_43bb_dac8) as usize;
    let saved_b40 = rd::<u64>(g_b40);
    let saved_ac8 = rd::<u64>(g_ac8);
    wr::<u64>(g_b40, 0);
    wr::<u64>(g_ac8, 0);

    let mut mgr = [0u8; 0x600];
    let mut root = [0u8; 0x1b0];
    wr::<usize>(root.as_mut_ptr() as usize + 0x1a8, mgr.as_mut_ptr() as usize);
    let mut cnt_o = 3i32;
    let mut cnt_n = 3i32;
    let this_o = [(&mut cnt_o as *mut i32) as usize, root.as_mut_ptr() as usize];
    let this_n = [(&mut cnt_n as *mut i32) as usize, root.as_mut_ptr() as usize];

    f_o(this_o.as_ptr() as usize);
    f_n(this_n.as_ptr() as usize);

    wr::<u64>(g_b40, saved_b40);
    wr::<u64>(g_ac8, saved_ac8);

    crate::diff_eq!(cnt_o, cnt_n, "rend_14228b010");
    Ok(1)
}

pub unsafe fn diff_rend_142298200(orig: usize, new: usize) -> Result<u32, String> {
    let mut o: [[f32; 4]; 2] = [[1.0, 2.0, 3.0, 4.0], [-5.0, 6.0, -7.0, 8.5]];
    let mut n = o;
    let _ = crate::simd_ops::call_va8_xmm(
        orig as u64,
        o.as_mut_ptr() as usize as u64,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 3.25f32.to_bits() as u128, 0],
    );
    let _ = crate::simd_ops::call_va8_xmm(
        new as u64,
        n.as_mut_ptr() as usize as u64,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 3.25f32.to_bits() as u128, 0],
    );
    let o_bits: [[u32; 4]; 2] = core::mem::transmute(o);
    let n_bits: [[u32; 4]; 2] = core::mem::transmute(n);
    crate::diff_eq!(o_bits, n_bits, "rend_142298200");
    Ok(1)
}

pub unsafe fn diff_rend_1422985d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> f64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    let mut buf = [0u8; 0x150];
    let addr = buf.as_mut_ptr() as usize;
    wr::<f64>(addr + 0x120, 2.5);
    wr::<f64>(addr + 0x140, 10.0);
    let r_o = f_o(addr).to_bits();
    let r_n = f_n(addr).to_bits();
    crate::diff_eq!(r_o, r_n, "rend_1422985d0");
    Ok(1)
}

pub unsafe fn diff_rend_142298600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> f32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    for idx in [0u32, 1, 2, 3, 4, 9, 27, 100] {
        let r_o = f_o(0, idx).to_bits();
        let r_n = f_n(0, idx).to_bits();
        crate::diff_eq!(r_o, r_n, "rend_142298600");
    }
    Ok(8)
}

pub unsafe fn diff_rend_142298670(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);
    for x in [0u32, 1, 2, 0x1234_5678, 0xdead_beef, u32::MAX] {
        crate::diff_eq!(f_o(0, x), f_n(0, x), "rend_142298670");
    }
    Ok(6)
}

pub unsafe fn diff_rend_142299a80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_d38 = exe::ptr::<u8>(0x1_4386_9d38) as usize;
    let g_8da = exe::ptr::<u8>(0x1_4397_d8da) as usize;
    let g_e28 = exe::ptr::<u32>(0x1_4386_9e28) as usize;
    let g_198 = exe::ptr::<u32>(0x1_43a2_b198) as usize;
    let g_d34 = exe::ptr::<u8>(0x1_4386_9d34) as usize;
    let saved = (
        rd::<u8>(g_d38),
        rd::<u8>(g_8da),
        rd::<u32>(g_e28),
        rd::<u32>(g_198),
        rd::<u8>(g_d34),
    );

    wr::<u8>(g_d38, 1);
    wr::<u8>(g_8da, 1);
    wr::<u32>(g_e28, 0);
    wr::<u32>(g_198, 99);
    wr::<u8>(g_d34, 0);
    f_o();
    let st_o = (rd::<u8>(g_d38), rd::<u32>(g_198), rd::<u8>(g_d34));

    wr::<u8>(g_d38, 1);
    wr::<u8>(g_8da, 1);
    wr::<u32>(g_e28, 0);
    wr::<u32>(g_198, 99);
    wr::<u8>(g_d34, 0);
    f_n();
    let st_n = (rd::<u8>(g_d38), rd::<u32>(g_198), rd::<u8>(g_d34));

    wr::<u8>(g_d38, saved.0);
    wr::<u8>(g_8da, saved.1);
    wr::<u32>(g_e28, saved.2);
    wr::<u32>(g_198, saved.3);
    wr::<u8>(g_d34, saved.4);

    crate::diff_eq!(st_o, st_n, "rend_142299a80");
    Ok(1)
}

pub unsafe fn diff_rend_14229cb60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_f28 = exe::ptr::<u32>(0x1_4397_ff28) as usize;
    let g_c28 = exe::ptr::<u64>(0x1_43a3_3c28) as usize;
    let g_b98 = exe::ptr::<u64>(0x1_43a3_3b98) as usize;
    let g_cc7 = exe::ptr::<u8>(0x1_43a2_8cc7) as usize;
    let saved = (
        rd::<u32>(g_f28),
        rd::<u64>(g_c28),
        rd::<u64>(g_b98),
        rd::<u8>(g_cc7),
    );

    let mut buf = [0u8; 0x6a0];
    let addr = buf.as_mut_ptr() as usize;
    wr::<u32>(addr + 0x5c0, 1);
    wr::<u32>(addr + 0x654, 2);
    wr::<u32>(addr + 0x694, 3);

    wr::<u32>(g_f28, 3);
    wr::<u64>(g_c28, 0x0438_0780);
    wr::<u64>(g_b98, 0);
    wr::<u8>(g_cc7, 0);
    f_o(addr);
    let st_o = (rd::<u8>(g_cc7), rd::<u64>(g_b98));

    wr::<u32>(g_f28, 3);
    wr::<u64>(g_c28, 0x0438_0780);
    wr::<u64>(g_b98, 0);
    wr::<u8>(g_cc7, 0);
    f_n(addr);
    let st_n = (rd::<u8>(g_cc7), rd::<u64>(g_b98));

    wr::<u32>(g_f28, saved.0);
    wr::<u64>(g_c28, saved.1);
    wr::<u64>(g_b98, saved.2);
    wr::<u8>(g_cc7, saved.3);

    crate::diff_eq!(st_o, st_n, "rend_14229cb60");
    Ok(1)
}

pub unsafe fn diff_rend_14229cbd0(orig: usize, new: usize) -> Result<u32, String> {
    let g_170 = exe::ptr::<u64>(0x1_43a0_0170) as usize;
    let g_178 = exe::ptr::<u32>(0x1_43a0_0178) as usize;
    let saved = (rd::<u64>(g_170), rd::<u32>(g_178));

    wr::<f64>(g_170, 10.0);
    wr::<f32>(g_178, 0.0);
    let _ = crate::simd_ops::call_va8_xmm(
        orig as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0.5f32.to_bits() as u128, 0, 0],
    );
    let st_o = (rd::<u64>(g_170), rd::<u32>(g_178));

    wr::<f64>(g_170, 10.0);
    wr::<f32>(g_178, 0.0);
    let _ = crate::simd_ops::call_va8_xmm(
        new as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0.5f32.to_bits() as u128, 0, 0],
    );
    let st_n = (rd::<u64>(g_170), rd::<u32>(g_178));

    wr::<u64>(g_170, saved.0);
    wr::<u32>(g_178, saved.1);

    crate::diff_eq!(st_o, st_n, "rend_14229cbd0");
    Ok(1)
}

pub unsafe fn diff_rend_14229cc00(orig: usize, new: usize) -> Result<u32, String> {
    let g_vec = exe::ptr::<u8>(0x1_43bb_ddd8) as usize;
    let g_flt = exe::ptr::<u32>(0x1_4384_170c) as usize;
    let saved_vec = rd::<[u8; 16]>(g_vec);
    let saved_flt = rd::<u32>(g_flt);

    let src_data = [0x11u8; 40];
    let src_vec = [src_data.as_ptr() as u64, 2u64];

    let mut dst_o = [0u8; 60];
    wr::<usize>(g_vec, dst_o.as_mut_ptr() as usize);
    wr::<u32>(g_vec + 0x8, 1);
    wr::<u32>(g_vec + 0xc, 3);
    let _ = crate::simd_ops::call_va8_xmm(
        orig as u64,
        0,
        src_vec.as_ptr() as usize as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 3.5f32.to_bits() as u128, 0],
    );
    let len_o = rd::<u32>(g_vec + 0x8);
    let flt_o = rd::<u32>(g_flt);

    let mut dst_n = [0u8; 60];
    wr::<usize>(g_vec, dst_n.as_mut_ptr() as usize);
    wr::<u32>(g_vec + 0x8, 1);
    wr::<u32>(g_vec + 0xc, 3);
    let _ = crate::simd_ops::call_va8_xmm(
        new as u64,
        0,
        src_vec.as_ptr() as usize as u64,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, 0, 3.5f32.to_bits() as u128, 0],
    );
    let len_n = rd::<u32>(g_vec + 0x8);
    let flt_n = rd::<u32>(g_flt);

    wr::<[u8; 16]>(g_vec, saved_vec);
    wr::<u32>(g_flt, saved_flt);

    crate::diff_eq!((dst_o, len_o, flt_o), (dst_n, len_n, flt_n), "rend_14229cc00");
    Ok(1)
}

pub unsafe fn diff_rend_14229cc40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_918 = exe::ptr::<u64>(0x1_43a3_5918) as usize;
    let g_180 = exe::ptr::<u32>(0x1_43a0_0180) as usize;
    let saved = (rd::<u64>(g_918), rd::<u32>(g_180));

    let mut sync_o = [0u64; 4];
    let mut this_o = [0u8; 0x5c0];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x5b8, sync_o.as_mut_ptr() as usize);
    f_o(this_o.as_ptr() as usize, 0x1234_5678_9abc_def0);
    let st_o = (rd::<u64>(g_918), rd::<u32>(g_180), sync_o[0], sync_o[3]);

    let mut sync_n = [0u64; 4];
    let mut this_n = [0u8; 0x5c0];
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x5b8, sync_n.as_mut_ptr() as usize);
    f_n(this_n.as_ptr() as usize, 0x1234_5678_9abc_def0);
    let st_n = (rd::<u64>(g_918), rd::<u32>(g_180), sync_n[0], sync_n[3]);

    wr::<u64>(g_918, saved.0);
    wr::<u32>(g_180, saved.1);

    crate::diff_eq!(st_o, st_n, "rend_14229cc40");
    Ok(1)
}

pub unsafe fn diff_rend_threading_vfunc_0_1422a3180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_728 = exe::ptr::<usize>(0x1_43a3_6728) as usize;
    let tls0 = tls_slot0();
    let saved_728 = rd::<usize>(g_728);
    let saved_tls = rd::<u64>(tls0 + 0x6338);

    let items = [0xaaaa_bbbb_cccc_ddddu64, 0x1111_2222_3333_4444u64];
    let mut ring_o = [0u8; 0x30];
    let r_o_addr = ring_o.as_mut_ptr() as usize;
    wr::<u32>(r_o_addr + 0x8, 0xffff_ffff);
    wr::<usize>(r_o_addr + 0x10, items.as_ptr() as usize);
    wr::<u32>(r_o_addr + 0x20, 2);
    wr::<u32>(r_o_addr + 0x24, 2);
    wr::<usize>(g_728, r_o_addr);
    f_o();
    let out_o = (rd::<u64>(tls0 + 0x6338), rd::<u32>(r_o_addr + 0x18), rd::<u32>(r_o_addr + 0x20));

    let mut ring_n = [0u8; 0x30];
    let r_n_addr = ring_n.as_mut_ptr() as usize;
    wr::<u32>(r_n_addr + 0x8, 0xffff_ffff);
    wr::<usize>(r_n_addr + 0x10, items.as_ptr() as usize);
    wr::<u32>(r_n_addr + 0x20, 2);
    wr::<u32>(r_n_addr + 0x24, 2);
    wr::<usize>(g_728, r_n_addr);
    f_n();
    let out_n = (rd::<u64>(tls0 + 0x6338), rd::<u32>(r_n_addr + 0x18), rd::<u32>(r_n_addr + 0x20));

    wr::<usize>(g_728, saved_728);
    wr::<u64>(tls0 + 0x6338, saved_tls);

    crate::diff_eq!(out_o, out_n, "rend_threading_vfunc_0_1422a3180");
    Ok(1)
}

pub unsafe fn diff_rend_1422a31d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_e88 = exe::ptr::<usize>(0x1_4397_fe88) as usize;
    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved = (rd::<usize>(g_e88), rd::<usize>(g_f58));

    let mut item_o = [0xffu8; 0x80];
    let item_o_ptr = item_o.as_mut_ptr() as usize;
    let mut ring_buf_o = [0u8; 256];
    let rb_o = ring_buf_o.as_mut_ptr() as usize;
    let mut ring_o = [0u64, rb_o as u64, (rb_o + 256) as u64, 0, rb_o as u64, (rb_o + 32) as u64, 8u64 << 32];
    let mut mgr_o = [0u8; 0x1b0];
    wr::<usize>(mgr_o.as_mut_ptr() as usize + 0x168, ring_o.as_mut_ptr() as usize);
    let mut list_o = [0u64, 0u64, (&item_o_ptr as *const usize) as u64, 1u64];
    wr::<usize>(g_e88, list_o.as_mut_ptr() as usize);
    wr::<usize>(g_f58, mgr_o.as_mut_ptr() as usize);
    f_o();

    let mut item_n = [0xffu8; 0x80];
    let item_n_ptr = item_n.as_mut_ptr() as usize;
    let mut ring_buf_n = [0u8; 256];
    let rb_n = ring_buf_n.as_mut_ptr() as usize;
    let mut ring_n = [0u64, rb_n as u64, (rb_n + 256) as u64, 0, rb_n as u64, (rb_n + 32) as u64, 8u64 << 32];
    let mut mgr_n = [0u8; 0x1b0];
    wr::<usize>(mgr_n.as_mut_ptr() as usize + 0x168, ring_n.as_mut_ptr() as usize);
    let mut list_n = [0u64, 0u64, (&item_n_ptr as *const usize) as u64, 1u64];
    wr::<usize>(g_e88, list_n.as_mut_ptr() as usize);
    wr::<usize>(g_f58, mgr_n.as_mut_ptr() as usize);
    f_n();

    wr::<usize>(g_e88, saved.0);
    wr::<usize>(g_f58, saved.1);

    crate::diff_eq!(item_o[0x6d], item_n[0x6d], "rend_1422a31d0 flag");
    crate::diff_eq!(ring_o[6], ring_n[6], "rend_1422a31d0 count");
    Ok(1)
}

pub unsafe fn diff_rend_1422a3260(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_e88 = exe::ptr::<usize>(0x1_4397_fe88) as usize;
    let saved = rd::<usize>(g_e88);
    let mut list_o = [0u64; 4];
    let mut list_n = [0u64; 4];
    wr::<usize>(g_e88, list_o.as_mut_ptr() as usize);
    f_o(0);
    wr::<usize>(g_e88, list_n.as_mut_ptr() as usize);
    f_n(0);
    wr::<usize>(g_e88, saved);

    crate::diff_eq!(list_o, list_n, "rend_1422a3260");
    Ok(1)
}

pub unsafe fn diff_rend_1422a32f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_e88 = exe::ptr::<usize>(0x1_4397_fe88) as usize;
    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved = (rd::<usize>(g_e88), rd::<usize>(g_f58));

    let item_ptr = 0x5000u64;
    let mut ring_buf_o = [0u8; 256];
    let rb_o = ring_buf_o.as_mut_ptr() as usize;
    let mut ring_o = [0u64, rb_o as u64, (rb_o + 256) as u64, 0, rb_o as u64, (rb_o + 32) as u64, 8u64 << 32];
    let mut mgr_o = [0u8; 0x1b0];
    wr::<usize>(mgr_o.as_mut_ptr() as usize + 0x168, ring_o.as_mut_ptr() as usize);
    let mut list_o = [0u64, 0u64, (&item_ptr as *const u64) as u64, 1u64];
    wr::<usize>(g_e88, list_o.as_mut_ptr() as usize);
    wr::<usize>(g_f58, mgr_o.as_mut_ptr() as usize);
    f_o();

    let mut ring_buf_n = [0u8; 256];
    let rb_n = ring_buf_n.as_mut_ptr() as usize;
    let mut ring_n = [0u64, rb_n as u64, (rb_n + 256) as u64, 0, rb_n as u64, (rb_n + 32) as u64, 8u64 << 32];
    let mut mgr_n = [0u8; 0x1b0];
    wr::<usize>(mgr_n.as_mut_ptr() as usize + 0x168, ring_n.as_mut_ptr() as usize);
    let mut list_n = [0u64, 0u64, (&item_ptr as *const u64) as u64, 1u64];
    wr::<usize>(g_e88, list_n.as_mut_ptr() as usize);
    wr::<usize>(g_f58, mgr_n.as_mut_ptr() as usize);
    f_n();

    wr::<usize>(g_e88, saved.0);
    wr::<usize>(g_f58, saved.1);

    crate::diff_eq!(ring_o[6], ring_n[6], "rend_1422a32f0 count");
    crate::diff_eq!(ring_buf_o[32..104], ring_buf_n[32..104], "rend_1422a32f0 packets");
    Ok(1)
}

pub unsafe fn diff_rend_1422a33b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_e88 = exe::ptr::<usize>(0x1_4397_fe88) as usize;
    let saved = rd::<usize>(g_e88);
    let mut list_o = [0u64; 4];
    let mut list_n = [0u64; 4];
    wr::<usize>(g_e88, list_o.as_mut_ptr() as usize);
    f_o(0);
    wr::<usize>(g_e88, list_n.as_mut_ptr() as usize);
    f_n(0);
    wr::<usize>(g_e88, saved);

    crate::diff_eq!(list_o, list_n, "rend_1422a33b0");
    Ok(1)
}

pub unsafe fn diff_rend_1422a3590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    // Case 1: null [this + 0x458]
    let this_null = [0u8; 0x460];
    let mut out_o = 0u32;
    let mut out_n = 0u32;
    f_o(this_null.as_ptr() as usize, (&mut out_o as *mut u32) as usize);
    f_n(this_null.as_ptr() as usize, (&mut out_n as *mut u32) as usize);
    crate::diff_eq!(out_o, out_n, "rend_1422a3590 null");

    // Case 2: non-null empty readback holder
    let mut rb_o = [0u64; 5];
    let mut rb_n = [0u64; 5];
    let mut this_o = [0u8; 0x460];
    let mut this_n = [0u8; 0x460];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x458, rb_o.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x458, rb_n.as_mut_ptr() as usize);
    let mut out2_o = 0x1234_5678u32;
    let mut out2_n = 0x1234_5678u32;
    f_o(this_o.as_ptr() as usize, (&mut out2_o as *mut u32) as usize);
    f_n(this_n.as_ptr() as usize, (&mut out2_n as *mut u32) as usize);
    crate::diff_eq!(out2_o, out2_n, "rend_1422a3590 empty");
    Ok(2)
}

pub unsafe fn diff_rend_14226ccc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_02C0: (usize, u64) = (0, 0);
    unsafe extern "win64" fn mock_734e0(out_pair: usize, _idx: u32) -> usize {
        wr::<u64>(out_pair, 0x7788_99aa);
        wr::<usize>(out_pair + 0x8, 0x1234_5678);
        out_pair
    }
    unsafe extern "win64" fn mock_702c0(obj: usize, val: u64) {
        unsafe {
            CALLED_02C0 = (obj, val);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4207_34e0, mock_734e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_02c0, mock_702c0 as *const ())?;
    let counter_addr = exe::ptr::<i32>(0x1_43a2_b19c) as usize;
    let saved_cnt = rd::<i32>(counter_addr);

    for (init_cnt, flag) in [(2i32, 0u8), (2i32, 1u8), (0i32, 1u8)] {
        wr::<i32>(counter_addr, init_cnt);
        CALLED_02C0 = (0, 0);
        f_o(flag);
        let st_o = (rd::<i32>(counter_addr), CALLED_02C0);

        wr::<i32>(counter_addr, init_cnt);
        CALLED_02C0 = (0, 0);
        f_n(flag);
        let st_n = (rd::<i32>(counter_addr), CALLED_02C0);

        if st_o != st_n {
            wr::<i32>(counter_addr, saved_cnt);
            crate::diff_eq!(st_o, st_n, "rend_14226ccc0");
        }
    }
    wr::<i32>(counter_addr, saved_cnt);
    Ok(3)
}

pub unsafe fn diff_rend_1422722f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_9D440: usize = 0;
    unsafe extern "win64" fn mock_9d440(arg: usize) {
        unsafe {
            CALLED_9D440 = arg;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4229_d440, mock_9d440 as *const ())?;

    let tls0 = tls_slot0();
    let saved_70 = rd::<u8>(tls0 + 0x70);
    let saved_6308 = rd::<u64>(tls0 + 0x6308);
    wr::<u8>(tls0 + 0x70, 1);
    wr::<u64>(tls0 + 0x6308, 0xdead_beef_1234_5678);

    let mut dst_o = 0u64;
    let this_o = [(&mut dst_o as *mut u64) as usize];
    CALLED_9D440 = 0;
    f_o(this_o.as_ptr() as usize);
    let st_o = (dst_o, CALLED_9D440);

    let mut dst_n = 0u64;
    let this_n = [(&mut dst_n as *mut u64) as usize];
    CALLED_9D440 = 0;
    f_n(this_n.as_ptr() as usize);
    let st_n = (dst_n, CALLED_9D440);

    wr::<u8>(tls0 + 0x70, saved_70);
    wr::<u64>(tls0 + 0x6308, saved_6308);

    crate::diff_eq!(st_o, st_n, "rend_1422722f0");
    Ok(1)
}

pub unsafe fn diff_rend_142279340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_17F30: (usize, u64) = (0, 0);
    unsafe extern "win64" fn mock_17f30(target: usize, found: u64) {
        unsafe {
            CALLED_17F30 = (target, found);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4221_7f30, mock_17f30 as *const ())?;

    let g_flag = exe::ptr::<u8>(0x1_4397_d8aa) as usize;
    let saved_flag = rd::<u8>(g_flag);

    let str_start = exe::ptr::<u8>(0x1_433f_5998) as usize;
    let str_end = exe::ptr::<u8>(0x1_433f_59ab) as usize;
    let mut expected_hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut cur = str_start;
    while cur != str_end {
        let ch = rd::<i8>(cur) as i64 as u64;
        cur = cur.wrapping_add(1);
        expected_hash = (expected_hash ^ ch).wrapping_mul(0x0100_0000_01b3);
    }

    let mut root = [0u8; 0x110];
    wr::<usize>(root.as_mut_ptr() as usize + 0x108, 0x4455_6677);
    let this = [root.as_ptr() as usize];

    for (flag, hit) in [(0u8, false), (1u8, false), (1u8, true)] {
        let mut pass_data = [0u8; 0x410];
        let pd_addr = pass_data.as_mut_ptr() as usize;
        if hit {
            wr::<u64>(pd_addr + 0x208 + 3 * 8, expected_hash);
            wr::<u64>(pd_addr + 0x8 + 3 * 8, 0x99aa_bbcc_ddee);
        }
        wr::<u8>(g_flag, flag);
        CALLED_17F30 = (0, 0);
        f_o(this.as_ptr() as usize, pd_addr);
        let st_o = CALLED_17F30;

        CALLED_17F30 = (0, 0);
        f_n(this.as_ptr() as usize, pd_addr);
        let st_n = CALLED_17F30;

        if st_o != st_n {
            wr::<u8>(g_flag, saved_flag);
            crate::diff_eq!(st_o, st_n, "rend_142279340");
        }
    }
    wr::<u8>(g_flag, saved_flag);
    Ok(3)
}

pub unsafe fn diff_rend_14227bd50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_EAD60: (usize, usize, u64) = (0, 0, 0);
    unsafe extern "win64" fn mock_ead60(target: usize, pass_data: usize, r8: u64) {
        unsafe {
            CALLED_EAD60 = (target, pass_data, r8);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_421e_ad60, mock_ead60 as *const ())?;

    let mut inner = [0u8; 0x420];
    let in_addr = inner.as_mut_ptr() as usize;
    wr::<usize>(in_addr + 0x1b8, 0x1122_3344);
    wr::<u64>(in_addr + 0x418, 0x5566_7788_99aa);
    let this = [in_addr];

    CALLED_EAD60 = (0, 0, 0);
    f_o(this.as_ptr() as usize, 0xaabb_ccdd);
    let st_o = CALLED_EAD60;

    CALLED_EAD60 = (0, 0, 0);
    f_n(this.as_ptr() as usize, 0xaabb_ccdd);
    let st_n = CALLED_EAD60;

    crate::diff_eq!(st_o, st_n, "rend_14227bd50");
    Ok(1)
}

pub unsafe fn diff_rend_142283010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_83010: [usize; 4] = [0; 4];
    unsafe extern "win64" fn mock_a370(a: usize) {
        unsafe { LOG_83010[0] = a; }
    }
    unsafe extern "win64" fn mock_b360(a: usize) {
        unsafe { LOG_83010[1] = a; }
    }
    unsafe extern "win64" fn mock_a7c0(a: usize) {
        unsafe { LOG_83010[2] = a; }
    }
    unsafe extern "win64" fn mock_8710(a: usize) {
        unsafe { LOG_83010[3] = a; }
    }

    let _m1 = crate::difftest::Mock::new(0x1_421e_a370, mock_a370 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_423e_b360, mock_b360 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_421e_a7c0, mock_a7c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4234_8710, mock_8710 as *const ())?;

    for p1b8 in [0usize, 0x3333usize] {
        let mut root = [0u8; 0x228];
        let r_addr = root.as_mut_ptr() as usize;
        wr::<usize>(r_addr + 0x178, 0x2222);
        wr::<usize>(r_addr + 0x1b8, p1b8);
        wr::<usize>(r_addr + 0x220, 0x4444);
        let this = [r_addr];

        LOG_83010 = [0; 4];
        f_o(this.as_ptr() as usize, 0);
        let st_o = LOG_83010;

        LOG_83010 = [0; 4];
        f_n(this.as_ptr() as usize, 0);
        let st_n = LOG_83010;

        crate::diff_eq!(st_o, st_n, "rend_142283010");
    }
    Ok(2)
}

pub unsafe fn diff_rend_142283490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_83490: [u64; 12] = [0; 12];
    unsafe extern "win64" fn mock_fc260(b8: usize, tag: usize, ptr1: u64, len1: u32, a5: u32, a6: u64) {
        unsafe {
            LOG_83490[0] = b8 as u64;
            LOG_83490[1] = tag as u64;
            LOG_83490[2] = ptr1;
            LOG_83490[3] = len1 as u64;
            LOG_83490[4] = a5 as u64;
            LOG_83490[5] = a6;
        }
    }
    unsafe extern "win64" fn mock_95a90(c0: usize, len2: u64, span_ptr: usize) {
        unsafe {
            LOG_83490[6] = c0 as u64;
            LOG_83490[7] = len2;
            LOG_83490[8] = rd::<u64>(span_ptr) ^ rd::<u64>(span_ptr + 8);
        }
    }
    unsafe extern "win64" fn mock_01ff0(c8: usize, span_ptr: usize, ptr3: u64) {
        unsafe {
            LOG_83490[9] = c8 as u64;
            LOG_83490[10] = (rd::<u64>(span_ptr) ^ rd::<u64>(span_ptr + 8)) ^ ptr3;
        }
    }
    unsafe extern "win64" fn mock_79240(d8: usize) {
        unsafe {
            LOG_83490[11] = d8 as u64;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_422f_c260, mock_fc260 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4239_5a90, mock_95a90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4230_1ff0, mock_01ff0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4237_9240, mock_79240 as *const ())?;

    let mut root = [0u8; 0xe0];
    let r_addr = root.as_mut_ptr() as usize;
    wr::<usize>(r_addr + 0xb8, 0x1001);
    wr::<usize>(r_addr + 0xc0, 0x1002);
    wr::<usize>(r_addr + 0xc8, 0x1003);
    wr::<usize>(r_addr + 0xd8, 0x1004);

    let v1 = [0xaaaa_1111u64, 5u64];
    let v2 = [0xbbbb_2222u64, 7u64];
    let v3 = [0xcccc_3333u64, 9u64];
    let this = [
        r_addr,
        v1.as_ptr() as usize,
        v2.as_ptr() as usize,
        v3.as_ptr() as usize,
    ];

    LOG_83490 = [0; 12];
    f_o(this.as_ptr() as usize, 0);
    let st_o = LOG_83490;

    LOG_83490 = [0; 12];
    f_n(this.as_ptr() as usize, 0);
    let st_n = LOG_83490;

    crate::diff_eq!(st_o, st_n, "rend_142283490");
    Ok(1)
}

pub unsafe fn diff_rend_142283570(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_83570: [usize; 6] = [0; 6];
    unsafe extern "win64" fn mock_485780(a: usize, b: usize) {
        unsafe {
            LOG_83570[0] = a;
            LOG_83570[1] = b;
        }
    }
    unsafe extern "win64" fn mock_05f870(a: u32, b: usize) {
        unsafe {
            LOG_83570[2] = a as usize;
            LOG_83570[3] = b;
        }
    }
    unsafe extern "win64" fn mock_3fe030(a: usize, b: u32) {
        unsafe {
            LOG_83570[4] = a;
            LOG_83570[5] = b as usize;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_5780, mock_485780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_f870, mock_05f870 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_423f_e030, mock_3fe030 as *const ())?;

    let g_job = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let g_flag = exe::ptr::<u8>(0x1_4386_c788) as usize;
    let saved_job = rd::<usize>(g_job);
    let saved_flag = rd::<u8>(g_flag);
    wr::<usize>(g_job, 0x7777_8888);

    let sub98 = [0u64, 42u64];
    let mut root = [0u8; 0xa0];
    wr::<usize>(root.as_mut_ptr() as usize + 0x98, sub98.as_ptr() as usize);
    let this = [root.as_ptr() as usize];

    for flag in [0u8, 1u8] {
        wr::<u8>(g_flag, flag);
        LOG_83570 = [0; 6];
        f_o(this.as_ptr() as usize, 0);
        let st_o = LOG_83570;

        LOG_83570 = [0; 6];
        f_n(this.as_ptr() as usize, 0);
        let st_n = LOG_83570;

        if st_o != st_n {
            wr::<usize>(g_job, saved_job);
            wr::<u8>(g_flag, saved_flag);
            crate::diff_eq!(st_o, st_n, "rend_142283570");
        }
    }

    wr::<usize>(g_job, saved_job);
    wr::<u8>(g_flag, saved_flag);
    Ok(2)
}

pub unsafe fn diff_rend_142289370(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_89370: (usize, usize) = (0, 0);
    unsafe extern "win64" fn mock_c5a30(a: usize) {
        unsafe { LOG_89370.0 = a; }
    }
    unsafe extern "win64" fn mock_edc80(a: usize) {
        unsafe { LOG_89370.1 = a; }
    }

    let _m1 = crate::difftest::Mock::new(0x1_423c_5a30, mock_c5a30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_423e_dc80, mock_edc80 as *const ())?;

    let mut root = [0u8; 0x1d0];
    wr::<usize>(root.as_mut_ptr() as usize + 0x178, 0x1111_2222);
    wr::<usize>(root.as_mut_ptr() as usize + 0x1c8, 0x3333_4444);
    let this = [root.as_ptr() as usize];

    LOG_89370 = (0, 0);
    f_o(this.as_ptr() as usize);
    let st_o = LOG_89370;

    LOG_89370 = (0, 0);
    f_n(this.as_ptr() as usize);
    let st_n = LOG_89370;

    crate::diff_eq!(st_o, st_n, "rend_142289370");
    Ok(1)
}

pub unsafe fn diff_rend_14228aa10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_8AA10: [u64; 7] = [0; 7];
    unsafe extern "win64" fn mock_5fdc0(local_addr: usize) {
        unsafe {
            LOG_8AA10[0] = rd::<u64>(local_addr);
            LOG_8AA10[1] = rd::<u32>(local_addr + 0x8) as u64;
            LOG_8AA10[2] = rd::<u64>(local_addr + 0x10);
        }
    }
    unsafe extern "win64" fn mock_93110() {
        unsafe { LOG_8AA10[3] = 1; }
    }
    unsafe extern "win64" fn mock_36540(a: usize) {
        unsafe { LOG_8AA10[4] = a as u64; }
    }
    unsafe extern "win64" fn mock_cff40(a: usize) {
        unsafe { LOG_8AA10[5] = a as u64; }
    }
    unsafe extern "win64" fn mock_93200(a: usize) {
        unsafe { LOG_8AA10[6] = a as u64; }
    }
    unsafe extern "win64" fn mock_5fad0() -> u64 {
        0xfeed_face_cafe_babe
    }

    let _m1 = crate::difftest::Mock::new(0x1_4205_fdc0, mock_5fdc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_3110, mock_93110 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4233_6540, mock_36540 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_423c_ff40, mock_cff40 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_3200, mock_93200 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4205_fad0, mock_5fad0 as *const ())?;

    let g_fc60 = exe::ptr::<u64>(0x1_4397_fc60) as usize;
    let g_fcb8 = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let g_c908 = exe::ptr::<u64>(0x1_43c8_c908) as usize;
    let saved = (rd::<u64>(g_fc60), rd::<usize>(g_fcb8), rd::<u64>(g_c908));

    wr::<u64>(g_fc60, 0x1111_2222_3333);
    wr::<usize>(g_fcb8, 0x4444_5555);
    let mut root = [0u8; 0xa8];
    wr::<usize>(root.as_mut_ptr() as usize + 0x98, 0x6666);
    wr::<usize>(root.as_mut_ptr() as usize + 0xa0, 0x7777);
    let this = [root.as_ptr() as usize];

    wr::<u64>(g_c908, 0);
    LOG_8AA10 = [0; 7];
    f_o(this.as_ptr() as usize);
    let st_o = (LOG_8AA10, rd::<u64>(g_c908));

    wr::<u64>(g_c908, 0);
    LOG_8AA10 = [0; 7];
    f_n(this.as_ptr() as usize);
    let st_n = (LOG_8AA10, rd::<u64>(g_c908));

    wr::<u64>(g_fc60, saved.0);
    wr::<usize>(g_fcb8, saved.1);
    wr::<u64>(g_c908, saved.2);

    crate::diff_eq!(st_o, st_n, "rend_14228aa10");
    Ok(1)
}

pub unsafe fn diff_rend_14228aa90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODE_PTR_8AA90: usize = 0;
    static mut LOG_8AA90: [u64; 5] = [0; 5];
    unsafe extern "win64" fn mock_36b40(a: usize) {
        unsafe { LOG_8AA90[0] = a as u64; }
    }
    unsafe extern "win64" fn mock_07c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_85aa0(pool: usize, handle_ptr: usize) {
        unsafe {
            LOG_8AA90[1] = pool as u64;
            wr::<usize>(handle_ptr, NODE_PTR_8AA90);
            wr::<u64>(handle_ptr + 0x8, 0x9988_7766);
        }
    }
    unsafe extern "win64" fn mock_86460(sched: usize, job_arg: u64) {
        unsafe {
            LOG_8AA90[2] = sched as u64;
            LOG_8AA90[3] = job_arg;
        }
    }
    unsafe extern "win64" fn mock_iat_5580(sem: u64, count: u32, prev: usize) {
        unsafe {
            LOG_8AA90[4] = sem ^ (count as u64) ^ (prev as u64);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4233_6b40, mock_36b40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_07c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_85aa0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_6460, mock_86460 as *const ())?;
    let _im = crate::difftest::IatMock::new(0x1_4264_5580, mock_iat_5580 as *const ())?;

    let g_job_sys = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let g_c930 = exe::ptr::<u8>(0x1_43c8_c930) as usize;
    let saved_job_sys = rd::<usize>(g_job_sys);
    let saved_c930 = rd::<[u8; 16]>(g_c930);

    let mut sem_holder = [0u8; 0x1010];
    wr::<u64>(sem_holder.as_mut_ptr() as usize + 0x1008, 0x5555_6666);
    let mut sched = [0u8; 0xc0];
    wr::<usize>(sched.as_mut_ptr() as usize, sem_holder.as_ptr() as usize);
    wr::<usize>(sched.as_mut_ptr() as usize + 0x40 + 0x48, 0x2000);
    let mut job_sys = [0u8; 0x20];
    wr::<usize>(job_sys.as_mut_ptr() as usize + 0x10, sched.as_ptr() as usize);
    wr::<usize>(g_job_sys, job_sys.as_ptr() as usize);

    let mut p8 = [0u8; 0xa0];
    wr::<usize>(p8.as_mut_ptr() as usize + 0x98, 0x3333_4444);

    let mut node_o = [0u64; 4];
    let mut cnt_o = 5i32;
    let this_o = [(&mut cnt_o as *mut i32) as usize, p8.as_ptr() as usize];
    NODE_PTR_8AA90 = node_o.as_mut_ptr() as usize;
    LOG_8AA90 = [0; 5];
    f_o(this_o.as_ptr() as usize);
    let st_o = (cnt_o, node_o, LOG_8AA90[0], LOG_8AA90[1], LOG_8AA90[4]);

    let mut node_n = [0u64; 4];
    let mut cnt_n = 5i32;
    let this_n = [(&mut cnt_n as *mut i32) as usize, p8.as_ptr() as usize];
    NODE_PTR_8AA90 = node_n.as_mut_ptr() as usize;
    LOG_8AA90 = [0; 5];
    f_n(this_n.as_ptr() as usize);
    let st_n = (cnt_n, node_n, LOG_8AA90[0], LOG_8AA90[1], LOG_8AA90[4]);

    wr::<usize>(g_job_sys, saved_job_sys);
    wr::<[u8; 16]>(g_c930, saved_c930);

    crate::diff_eq!(st_o, st_n, "rend_14228aa90");
    Ok(1)
}

pub unsafe fn diff_rend_14228ac50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_8AC50: [u64; 8] = [0; 8];
    unsafe extern "win64" fn mock_62470(mgr: usize, flag: u32) {
        unsafe { LOG_8AC50[0] = (mgr as u64) ^ (flag as u64); }
    }
    unsafe extern "win64" fn mock_620f0(mgr: usize) {
        unsafe { LOG_8AC50[1] = mgr as u64; }
    }
    unsafe extern "win64" fn mock_62310(mgr: usize) {
        unsafe { LOG_8AC50[2] = mgr as u64; }
    }
    unsafe extern "win64" fn mock_61da0(mgr: usize) {
        unsafe { LOG_8AC50[3] = mgr as u64; }
    }
    unsafe extern "win64" fn mock_5e6c0(mgr_off: usize) {
        unsafe { LOG_8AC50[4] = mgr_off as u64; }
    }
    unsafe extern "win64" fn mock_2e360(sub: usize) {
        unsafe { LOG_8AC50[5] = sub as u64; }
    }
    unsafe extern "win64" fn mock_2dca0(sub: usize, pass_data: usize) {
        unsafe {
            LOG_8AC50[6] = sub as u64;
            LOG_8AC50[7] = pass_data as u64;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4226_2470, mock_62470 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4226_20f0, mock_620f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4226_2310, mock_62310 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4226_1da0, mock_61da0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4225_e6c0, mock_5e6c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4222_e360, mock_2e360 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4222_dca0, mock_2dca0 as *const ())?;

    let g_flag = exe::ptr::<u8>(0x1_4397_d8ca) as usize;
    let saved_flag = rd::<u8>(g_flag);
    wr::<u8>(g_flag, 1);

    let src_bytes = [0x3cu8; 24];
    let mut dst_o = [0u8; 24];
    let mut dst_n = [0u8; 24];

    let mut mgr = [0u8; 0x600];
    let mgr_addr = mgr.as_mut_ptr() as usize;
    wr::<u32>(mgr_addr + 0x20, 2);
    wr::<usize>(mgr_addr + 0x28, src_bytes.as_ptr() as usize);

    let mut root = [0u8; 0x1b8];
    let r_addr = root.as_mut_ptr() as usize;
    wr::<usize>(r_addr + 0x1a8, mgr_addr);
    wr::<usize>(r_addr + 0x1b0, 0x8888_9999);
    let this = [r_addr];

    wr::<usize>(mgr_addr + 0x5f0, dst_o.as_mut_ptr() as usize);
    LOG_8AC50 = [0; 8];
    f_o(this.as_ptr() as usize, 0x1234);
    let log_o = LOG_8AC50;

    wr::<usize>(mgr_addr + 0x5f0, dst_n.as_mut_ptr() as usize);
    LOG_8AC50 = [0; 8];
    f_n(this.as_ptr() as usize, 0x1234);
    let log_n = LOG_8AC50;

    wr::<u8>(g_flag, saved_flag);

    crate::diff_eq!(dst_o, dst_n, "rend_14228ac50 memcpy");
    crate::diff_eq!(log_o, log_n, "rend_14228ac50 calls");
    Ok(1)
}

pub unsafe fn diff_rend_14228adb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_8ADB0: [usize; 3] = [0; 3];
    unsafe extern "win64" fn mock_626d0(a: usize) {
        unsafe { LOG_8ADB0[0] = a; }
    }
    unsafe extern "win64" fn mock_591f0(a: usize) {
        unsafe { LOG_8ADB0[1] = a; }
    }
    unsafe extern "win64" fn mock_0be30(a: usize) {
        unsafe { LOG_8ADB0[2] = a; }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4226_26d0, mock_626d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_91f0, mock_591f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4230_be30, mock_0be30 as *const ())?;

    let mut root = [0u8; 0x1f0];
    let r_addr = root.as_mut_ptr() as usize;
    wr::<usize>(r_addr + 0x1a8, 0x1111_2222);
    wr::<usize>(r_addr + 0x1e8, 0x3333_4444);
    let this = [r_addr];

    LOG_8ADB0 = [0; 3];
    f_o(this.as_ptr() as usize);
    let st_o = LOG_8ADB0;

    LOG_8ADB0 = [0; 3];
    f_n(this.as_ptr() as usize);
    let st_n = LOG_8ADB0;

    crate::diff_eq!(st_o, st_n, "rend_14228adb0");
    Ok(1)
}

pub unsafe fn diff_rend_142294cc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_94CC0: (usize, u32, u32) = (0, 0, 0);
    unsafe extern "win64" fn mock_f0110(rcx: usize, edx: u32, r8d: u32) {
        unsafe {
            LOG_94CC0 = (rcx, edx, r8d);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_421f_0110, mock_f0110 as *const ())?;

    let tls0 = tls_slot0();
    let g_14c = exe::ptr::<u32>(0x1_43a0_014c) as usize;
    let g_150 = exe::ptr::<u32>(0x1_43a0_0150) as usize;
    let saved = (
        rd::<u8>(tls0 + 0x70),
        rd::<u64>(tls0 + 0x6308),
        rd::<u32>(g_14c),
        rd::<u32>(g_150),
    );
    wr::<u8>(tls0 + 0x70, 1);

    let this = [0x7654_3210usize];
    wr::<u32>(g_14c, 99);
    wr::<u32>(g_150, 99);
    wr::<u64>(tls0 + 0x6308, 0);
    LOG_94CC0 = (0, 0, 0);
    f_o(this.as_ptr() as usize);
    let st_o = (LOG_94CC0, rd::<u64>(tls0 + 0x6308), rd::<u32>(g_14c), rd::<u32>(g_150));

    wr::<u32>(g_14c, 99);
    wr::<u32>(g_150, 99);
    wr::<u64>(tls0 + 0x6308, 0);
    LOG_94CC0 = (0, 0, 0);
    f_n(this.as_ptr() as usize);
    let st_n = (LOG_94CC0, rd::<u64>(tls0 + 0x6308), rd::<u32>(g_14c), rd::<u32>(g_150));

    wr::<u8>(tls0 + 0x70, saved.0);
    wr::<u64>(tls0 + 0x6308, saved.1);
    wr::<u32>(g_14c, saved.2);
    wr::<u32>(g_150, saved.3);

    crate::diff_eq!(st_o, st_n, "rend_142294cc0");
    Ok(1)
}

pub unsafe fn diff_rend_142298590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_98590: [usize; 4] = [0; 4];
    unsafe extern "win64" fn mock_04370(vec_a: usize, elem_a: usize) -> usize {
        unsafe {
            LOG_98590[0] = vec_a;
            LOG_98590[1] = elem_a;
        }
        0x1111
    }
    unsafe extern "win64" fn mock_fff40(vec_b: usize, elem_b: usize) -> usize {
        unsafe {
            LOG_98590[2] = vec_b;
            LOG_98590[3] = elem_b;
        }
        0x2222
    }

    let _m1 = crate::difftest::Mock::new(0x1_4130_4370, mock_04370 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412f_ff40, mock_fff40 as *const ())?;

    let g_a = exe::ptr::<usize>(0x1_43a3_6698) as usize;
    let g_b = exe::ptr::<usize>(0x1_43a3_6648) as usize;
    let saved = (rd::<usize>(g_a), rd::<usize>(g_b));
    wr::<usize>(g_a, 0xaaaa);
    wr::<usize>(g_b, 0xbbbb);

    LOG_98590 = [0; 4];
    let r_o = f_o(0xcccc, 0xdddd);
    let st_o = (r_o, LOG_98590);

    LOG_98590 = [0; 4];
    let r_n = f_n(0xcccc, 0xdddd);
    let st_n = (r_n, LOG_98590);

    wr::<usize>(g_a, saved.0);
    wr::<usize>(g_b, saved.1);

    crate::diff_eq!(st_o, st_n, "rend_142298590");
    Ok(1)
}

pub unsafe fn diff_rend_14229ca60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RET_90890: u8 = 1;
    static mut LOG_9CA60: [u64; 9] = [0; 9];
    unsafe extern "win64" fn mock_8d420(a: usize) {
        unsafe { LOG_9CA60[0] = a as u64; }
    }
    unsafe extern "win64" fn mock_577e0(a: usize) {
        unsafe { LOG_9CA60[1] = a as u64; }
    }
    unsafe extern "win64" fn mock_90890(_rcx: usize, edx: u32, r8d: u32, w: u32, h: u32) -> u8 {
        unsafe {
            LOG_9CA60[2] = edx as u64;
            LOG_9CA60[3] = r8d as u64;
            LOG_9CA60[4] = w as u64;
            LOG_9CA60[5] = h as u64;
            RET_90890
        }
    }
    unsafe extern "win64" fn mock_5c4b0(fmt: u32, _rdx: usize, r8: u64, r9: u64) {
        unsafe {
            LOG_9CA60[6] = fmt as u64;
            LOG_9CA60[7] = r8;
            LOG_9CA60[8] = r9;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4238_d420, mock_8d420 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_77e0, mock_577e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_0890, mock_90890 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4205_c4b0, mock_5c4b0 as *const ())?;

    let g_cc7 = exe::ptr::<u8>(0x1_43a2_8cc7) as usize;
    let g_f28 = exe::ptr::<u32>(0x1_4397_ff28) as usize;
    let g_b98 = exe::ptr::<u64>(0x1_43a3_3b98) as usize;
    let g_cc4 = exe::ptr::<u8>(0x1_43a2_8cc4) as usize;
    let g_f30 = exe::ptr::<u32>(0x1_4397_ff30) as usize;
    let saved = (
        rd::<u8>(g_cc7),
        rd::<u32>(g_f28),
        rd::<u64>(g_b98),
        rd::<u8>(g_cc4),
        rd::<u32>(g_f30),
    );

    let mut sub = [0u8; 0x20];
    wr::<u8>(sub.as_mut_ptr() as usize + 0x18, 7);
    let mut p48 = [0u8; 0x40];
    wr::<usize>(p48.as_mut_ptr() as usize + 0x10, sub.as_ptr() as usize);
    wr::<i32>(p48.as_mut_ptr() as usize + 0x38, 1);

    let run_case = |f: F, cc7: u8, edi: u32, ok_ret: u8| unsafe {
        RET_90890 = ok_ret;
        LOG_9CA60 = [0; 9];
        wr::<u8>(g_cc7, cc7);
        wr::<u32>(g_f28, edi);
        wr::<u64>(g_b98, 0x0438_0780);
        wr::<u8>(g_cc4, 0);
        wr::<u32>(g_f30, 99);
        let mut this = [0u8; 0x6a0];
        let t_addr = this.as_mut_ptr() as usize;
        wr::<usize>(t_addr + 0x48, p48.as_ptr() as usize);
        wr::<usize>(t_addr + 0x58, 0x5555);
        wr::<u32>(t_addr + 0x5c0, 1);
        wr::<u32>(t_addr + 0x654, 1);
        wr::<u32>(t_addr + 0x678, 1280);
        wr::<u32>(t_addr + 0x67c, 720);
        wr::<u32>(t_addr + 0x694, 3);
        f(t_addr);
        (
            LOG_9CA60,
            rd::<u8>(g_cc7),
            rd::<u8>(g_cc4),
            rd::<u32>(g_f30),
            rd::<u32>(t_addr + 0x5c0),
            rd::<u32>(t_addr + 0x694),
        )
    };

    for (cc7, edi, ok_ret) in [(0u8, 2u32, 1u8), (1u8, 2u32, 1u8), (1u8, 2u32, 0u8), (1u8, 3u32, 1u8)] {
        let st_o = run_case(f_o, cc7, edi, ok_ret);
        let st_n = run_case(f_n, cc7, edi, ok_ret);
        if st_o != st_n {
            wr::<u8>(g_cc7, saved.0);
            wr::<u32>(g_f28, saved.1);
            wr::<u64>(g_b98, saved.2);
            wr::<u8>(g_cc4, saved.3);
            wr::<u32>(g_f30, saved.4);
            crate::diff_eq!(st_o, st_n, "rend_14229ca60");
        }
    }

    wr::<u8>(g_cc7, saved.0);
    wr::<u32>(g_f28, saved.1);
    wr::<u64>(g_b98, saved.2);
    wr::<u8>(g_cc4, saved.3);
    wr::<u32>(g_f30, saved.4);
    Ok(4)
}

pub unsafe fn diff_rend_1422a3020(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SEQ_7E20: u64 = 0;
    static mut RES_AD60: u64 = 0;
    static mut LOG_A3020: [u64; 6] = [0; 6];
    unsafe extern "win64" fn mock_7e20() -> u64 {
        unsafe {
            SEQ_7E20 += 0x100;
            SEQ_7E20
        }
    }
    unsafe extern "win64" fn mock_2c50() -> u64 {
        0x3333_4444
    }
    unsafe extern "win64" fn mock_0410(a: usize) {
        unsafe {
            if LOG_A3020[0] == 0 {
                LOG_A3020[0] = a as u64;
            } else {
                LOG_A3020[1] = a as u64;
            }
        }
    }
    unsafe extern "win64" fn mock_ad60(tmp_ptr: usize) -> usize {
        unsafe {
            wr::<u64>(tmp_ptr, 0x7777);
            RES_AD60 = 0x8888;
            core::ptr::addr_of_mut!(RES_AD60) as usize
        }
    }
    unsafe extern "win64" fn mock_2c80(_rcx: usize, rdx: u64) {
        unsafe {
            if LOG_A3020[2] == 0 {
                LOG_A3020[2] = rdx;
            } else {
                LOG_A3020[3] = rdx;
            }
        }
    }
    unsafe extern "win64" fn mock_9dd0(cur: usize) -> u64 {
        unsafe {
            LOG_A3020[4] = cur as u64;
            0x9999_aaaa
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_7e20, mock_7e20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_423c_2c50, mock_2c50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4206_0410, mock_0410 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412f_ad60, mock_ad60 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4130_2c80, mock_2c80 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_412f_9dd0, mock_9dd0 as *const ())?;

    let tls0 = tls_slot0();
    let g_9b8 = exe::ptr::<u64>(0x1_43a3_79b8) as usize;
    let g_3e8 = exe::ptr::<u64>(0x1_43a3_83e8) as usize;
    let g_728 = exe::ptr::<u64>(0x1_43a3_6728) as usize;
    let saved = (
        rd::<u8>(tls0 + 0x70),
        rd::<u64>(tls0 + 0x6358),
        rd::<u64>(tls0 + 0x6338),
        rd::<u64>(g_9b8),
        rd::<u64>(g_3e8),
        rd::<u64>(g_728),
    );
    wr::<u8>(tls0 + 0x70, 1);

    SEQ_7E20 = 0;
    LOG_A3020 = [0; 6];
    wr::<u64>(g_728, 0x6666);
    f_o();
    let st_o = (
        LOG_A3020,
        rd::<u64>(tls0 + 0x6358),
        rd::<u64>(tls0 + 0x6338),
        rd::<u64>(g_9b8),
        rd::<u64>(g_3e8),
        rd::<u64>(g_728),
    );

    SEQ_7E20 = 0;
    LOG_A3020 = [0; 6];
    wr::<u64>(g_728, 0x6666);
    f_n();
    let st_n = (
        LOG_A3020,
        rd::<u64>(tls0 + 0x6358),
        rd::<u64>(tls0 + 0x6338),
        rd::<u64>(g_9b8),
        rd::<u64>(g_3e8),
        rd::<u64>(g_728),
    );

    wr::<u8>(tls0 + 0x70, saved.0);
    wr::<u64>(tls0 + 0x6358, saved.1);
    wr::<u64>(tls0 + 0x6338, saved.2);
    wr::<u64>(g_9b8, saved.3);
    wr::<u64>(g_3e8, saved.4);
    wr::<u64>(g_728, saved.5);

    crate::diff_eq!(st_o, st_n, "rend_1422a3020");
    Ok(1)
}

pub unsafe fn diff_rend_1422a3490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_A3490: [u64; 9] = [0; 9];
    unsafe extern "win64" fn mock_a49d0(
        a0: usize,
        a1: usize,
        a2: u8,
        a3: u32,
        a4: u32,
        a5_bits: u32,
        a6: u32,
        a7: u32,
        a8_bits: usize,
    ) {
        unsafe {
            LOG_A3490 = [
                a0 as u64,
                a1 as u64,
                a2 as u64,
                a3 as u64,
                a4 as u64,
                a5_bits as u64,
                a6 as u64,
                a7 as u64,
                (a8_bits as u32) as u64,
            ];
        }
    }

    let _m = crate::difftest::Mock::new(0x1_422a_49d0, mock_a49d0 as *const ())?;

    let g_11c = exe::ptr::<u32>(0x1_43a2_b11c) as usize;
    let saved_11c = rd::<u32>(g_11c);

    for (b4c, b66, b67, ecx, flag) in [
        (0u8, 0u8, 0u8, 0u32, 0u8),
        (0u8, 1u8, 0u8, 0u32, 1u8),
        (1u8, 0u8, 0u8, 0u32, 1u8),
        (1u8, 0u8, 1u8, 0u32, 1u8),
        (1u8, 0u8, 1u8, 1u32, 1u8),
        (1u8, 0u8, 1u8, 2u32, 0u8),
    ] {
        wr::<u32>(g_11c, ecx);
        let mut r10 = [0u8; 0x70];
        let r10_addr = r10.as_mut_ptr() as usize;
        wr::<usize>(r10_addr + 0x28, 0x4321_8765);
        wr::<i32>(r10_addr + 0x38, 1);
        wr::<u8>(r10_addr + 0x4c, b4c);
        wr::<u8>(r10_addr + 0x66, b66);
        wr::<u8>(r10_addr + 0x67, b67);
        let mut this = [0u8; 0x50];
        wr::<usize>(this.as_mut_ptr() as usize + 0x48, r10_addr);

        LOG_A3490 = [0; 9];
        f_o(this.as_ptr() as usize, 0x1122, flag);
        let st_o = LOG_A3490;

        LOG_A3490 = [0; 9];
        f_n(this.as_ptr() as usize, 0x1122, flag);
        let st_n = LOG_A3490;

        if st_o != st_n {
            wr::<u32>(g_11c, saved_11c);
            crate::diff_eq!(st_o, st_n, "rend_1422a3490");
        }
    }

    wr::<u32>(g_11c, saved_11c);
    Ok(6)
}

// REGISTRY:
// FLOAT_RETURN: 0x1_4229_85d0 (returns f64 in xmm0), 0x1_4229_8600 (returns f32 in xmm0)
//
// Replacement::verified_throws(0x1_4226_ccc0, "rend::rend_14226ccc0", rend::rend_e18_a::rend_14226ccc0 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_14226ccc0),
// Replacement::verified(0x1_4227_22f0, "rend::rend_1422722f0", rend::rend_e18_a::rend_1422722f0 as *const (), rend::rend_e18_a::diff_rend_1422722f0),
// Replacement::verified(0x1_4227_2d60, "rend::rend_142272d60", rend::rend_e18_a::rend_142272d60 as *const (), rend::rend_e18_a::diff_rend_142272d60),
// Replacement::verified_throws(0x1_4227_9340, "rend::rend_142279340", rend::rend_e18_a::rend_142279340 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_142279340),
// Replacement::verified_throws(0x1_4227_bd50, "rend::rend_14227bd50", rend::rend_e18_a::rend_14227bd50 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_14227bd50),
// Replacement::verified_throws(0x1_4228_3010, "rend::rend_142283010", rend::rend_e18_a::rend_142283010 as *const (), Throws::Caught, rend::rend_e18_a::diff_rend_142283010),
// Replacement::verified_throws(0x1_4228_33a0, "rend::rend_1422833a0", rend::rend_e18_a::rend_1422833a0 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_1422833a0),
// Replacement::verified_throws(0x1_4228_3490, "rend::rend_142283490", rend::rend_e18_a::rend_142283490 as *const (), Throws::Caught, rend::rend_e18_a::diff_rend_142283490),
// Replacement::verified_throws(0x1_4228_3570, "rend::rend_142283570", rend::rend_e18_a::rend_142283570 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_142283570),
// Replacement::verified(0x1_4228_35e0, "rend::rend_1422835e0", rend::rend_e18_a::rend_1422835e0 as *const (), rend::rend_e18_a::diff_rend_1422835e0),
// Replacement::verified(0x1_4228_3610, "rend::rend_142283610", rend::rend_e18_a::rend_142283610 as *const (), rend::rend_e18_a::diff_rend_142283610),
// Replacement::verified(0x1_4228_9300, "rend::rend_142289300", rend::rend_e18_a::rend_142289300 as *const (), rend::rend_e18_a::diff_rend_142289300),
// Replacement::verified_throws(0x1_4228_9370, "rend::rend_142289370", rend::rend_e18_a::rend_142289370 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_142289370),
// Replacement::verified(0x1_4228_9b10, "rend::rend_142289b10", rend::rend_e18_a::rend_142289b10 as *const (), rend::rend_e18_a::diff_rend_142289b10),
// Replacement::verified_throws(0x1_4228_aa10, "rend::rend_14228aa10", rend::rend_e18_a::rend_14228aa10 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_14228aa10),
// Replacement::verified_throws(0x1_4228_aa90, "rend::rend_14228aa90", rend::rend_e18_a::rend_14228aa90 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_14228aa90),
// Replacement::verified_throws(0x1_4228_ac50, "rend::rend_14228ac50", rend::rend_e18_a::rend_14228ac50 as *const (), Throws::Caught, rend::rend_e18_a::diff_rend_14228ac50),
// Replacement::verified(0x1_4228_ad30, "rend::rend_14228ad30", rend::rend_e18_a::rend_14228ad30 as *const (), rend::rend_e18_a::diff_rend_14228ad30),
// Replacement::verified_throws(0x1_4228_adb0, "rend::rend_14228adb0", rend::rend_e18_a::rend_14228adb0 as *const (), Throws::Caught, rend::rend_e18_a::diff_rend_14228adb0),
// Replacement::verified(0x1_4228_b010, "rend::rend_14228b010", rend::rend_e18_a::rend_14228b010 as *const (), rend::rend_e18_a::diff_rend_14228b010),
// Replacement::verified(0x1_4229_4cc0, "rend::rend_142294cc0", rend::rend_e18_a::rend_142294cc0 as *const (), rend::rend_e18_a::diff_rend_142294cc0),
// Replacement::verified(0x1_4229_8200, "rend::rend_142298200", rend::rend_e18_a::rend_142298200 as *const (), rend::rend_e18_a::diff_rend_142298200),
// Replacement::verified_throws(0x1_4229_8590, "rend::rend_142298590", rend::rend_e18_a::rend_142298590 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_142298590),
// Replacement::verified(0x1_4229_85d0, "rend::rend_1422985d0", rend::rend_e18_a::rend_1422985d0 as *const (), rend::rend_e18_a::diff_rend_1422985d0),
// Replacement::verified(0x1_4229_8600, "rend::rend_142298600", rend::rend_e18_a::rend_142298600 as *const (), rend::rend_e18_a::diff_rend_142298600),
// Replacement::verified(0x1_4229_8670, "rend::rend_142298670", rend::rend_e18_a::rend_142298670 as *const (), rend::rend_e18_a::diff_rend_142298670),
// Replacement::verified(0x1_4229_9a80, "rend::rend_142299a80", rend::rend_e18_a::rend_142299a80 as *const (), rend::rend_e18_a::diff_rend_142299a80),
// Replacement::verified_throws(0x1_4229_ca60, "rend::rend_14229ca60", rend::rend_e18_a::rend_14229ca60 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_14229ca60),
// Replacement::verified(0x1_4229_cb60, "rend::rend_14229cb60", rend::rend_e18_a::rend_14229cb60 as *const (), rend::rend_e18_a::diff_rend_14229cb60),
// Replacement::verified(0x1_4229_cbd0, "rend::rend_14229cbd0", rend::rend_e18_a::rend_14229cbd0 as *const (), rend::rend_e18_a::diff_rend_14229cbd0),
// Replacement::verified(0x1_4229_cc00, "rend::rend_14229cc00", rend::rend_e18_a::rend_14229cc00 as *const (), rend::rend_e18_a::diff_rend_14229cc00),
// Replacement::verified(0x1_4229_cc40, "rend::rend_14229cc40", rend::rend_e18_a::rend_14229cc40 as *const (), rend::rend_e18_a::diff_rend_14229cc40),
// Replacement::verified(0x1_422a_3020, "rend::rend_1422a3020", rend::rend_e18_a::rend_1422a3020 as *const (), rend::rend_e18_a::diff_rend_1422a3020),
// Replacement::verified(0x1_422a_3180, "anonymous_namespace::RendThreading::vfunc_0", rend::rend_e18_a::rend_threading_vfunc_0_1422a3180 as *const (), rend::rend_e18_a::diff_rend_threading_vfunc_0_1422a3180),
// Replacement::verified(0x1_422a_31d0, "rend::rend_1422a31d0", rend::rend_e18_a::rend_1422a31d0 as *const (), rend::rend_e18_a::diff_rend_1422a31d0),
// Replacement::verified(0x1_422a_3260, "rend::rend_1422a3260", rend::rend_e18_a::rend_1422a3260 as *const (), rend::rend_e18_a::diff_rend_1422a3260),
// Replacement::verified(0x1_422a_32f0, "rend::rend_1422a32f0", rend::rend_e18_a::rend_1422a32f0 as *const (), rend::rend_e18_a::diff_rend_1422a32f0),
// Replacement::verified(0x1_422a_33b0, "rend::rend_1422a33b0", rend::rend_e18_a::rend_1422a33b0 as *const (), rend::rend_e18_a::diff_rend_1422a33b0),
// Replacement::verified_throws(0x1_422a_3490, "rend::rend_1422a3490", rend::rend_e18_a::rend_1422a3490 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_1422a3490),
// Replacement::verified_throws(0x1_422a_3590, "rend::rend_1422a3590", rend::rend_e18_a::rend_1422a3590 as *const (), Throws::Discard, rend::rend_e18_a::diff_rend_1422a3590),

