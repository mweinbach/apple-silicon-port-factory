//! Decompiled `rend` functions from `e18.rs` (`0x1422a3830..0x1422f6fb0`).

use crate::exe::{self, call_va, rd, wr};

/// `rend::rend_1422a3830`
/// Original: `0x1422a3830` (15 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a3830(idx: i32, val: u64) {
    let base = exe::ptr::<u64>(0x1_43c4_9520) as usize;
    let addr = (base as isize).wrapping_add((idx as isize).wrapping_mul(8)) as usize;
    wr::<u64>(addr, val);
}

pub unsafe fn diff_rend_1422a3830(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let base = exe::ptr::<u64>(0x1_43c4_9520) as usize;
    let saved0 = rd::<u64>(base);
    let saved1 = rd::<u64>(base + 8);

    f_orig(0, 0x1122_3344_5566_7788);
    f_orig(1, 0x99aa_bbcc_ddee_ff00);
    let o0 = rd::<u64>(base);
    let o1 = rd::<u64>(base + 8);

    wr::<u64>(base, saved0);
    wr::<u64>(base + 8, saved1);

    f_new(0, 0x1122_3344_5566_7788);
    f_new(1, 0x99aa_bbcc_ddee_ff00);
    let n0 = rd::<u64>(base);
    let n1 = rd::<u64>(base + 8);

    wr::<u64>(base, saved0);
    wr::<u64>(base + 8, saved1);

    crate::diff_eq!(o0, n0, "slot 0");
    crate::diff_eq!(o1, n1, "slot 1");
    Ok(2)
}

/// `rend::rend_1422a3850`
/// Original: `0x1422a3850` (64 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a3850(this: usize) -> u8 {
    let mut cur = rd::<u32>(this + 0x17c);
    if cur != rd::<u32>(this + 0x180) {
        if cur == 1 {
            call_va!(0x1_401e_65c0, (this + 0x140,) -> ());
        }
        cur = rd::<u32>(this + 0x17c);
        wr::<u32>(this + 0x180, cur);
    }
    u8::from(cur != 1)
}

pub unsafe fn diff_rend_1422a3850(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for (cur, prev) in [(0u32, 0u32), (2, 0), (1, 0), (1, 1)] {
        let mut buf_o = [0u8; 0x190];
        wr::<u32>(buf_o.as_mut_ptr() as usize + 0x17c, cur);
        wr::<u32>(buf_o.as_mut_ptr() as usize + 0x180, prev);
        let mut buf_n = buf_o;
        let r_o = f_orig(buf_o.as_mut_ptr() as usize);
        let r_n = f_new(buf_n.as_mut_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "ret ({cur}, {prev})");
        crate::diff_eq!(buf_o, buf_n, "buf ({cur}, {prev})");
    }
    Ok(4)
}

/// `rend::rend_1422a3c40`
/// Original: `0x1422a3c40` (51 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a3c40(this: usize) {
    let idx = rd::<u32>(this) as usize;
    let table = exe::ptr::<u64>(0x1_43c4_9520) as usize;
    let fn_ptr = rd::<usize>(table + idx * 8);
    let arg = rd::<u64>(this + 0x8);
    call_va!(fn_ptr, (arg,) -> ());
    let ptr_after = rd::<u64>(this + 0x8);
    if ptr_after != 0 {
        call_va!(0x1_4244_2de0, (ptr_after,) -> ());
    }
}

pub unsafe fn diff_rend_1422a3c40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let table = exe::ptr::<u64>(0x1_43c4_9520) as usize;
    let saved = rd::<u64>(table);
    // Point slot 0 to 0x1422688a0 (ret) and pass [this + 8] == 0 so heap free is skipped.
    wr::<u64>(table, 0x1_4226_88a0);

    let mut obj_o = [0u64, 0u64];
    let mut obj_n = obj_o;
    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);
    wr::<u64>(table, saved);
    crate::diff_eq!(obj_o, obj_n, "rend_1422a3c40 null payload");
    Ok(1)
}

/// `rend::rend_1422a3c80`
/// Original: `0x1422a3c80` (10 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a3c80(this: usize, rdx: u64, r8: u64, r9: u64) -> u64 {
    let fn_ptr = rd::<usize>(this);
    call_va!(fn_ptr, (this.wrapping_add(8), rdx, r8, r9) -> u64)
}

pub unsafe fn diff_rend_1422a3c80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    // Use 0x1422ab600 (pure leaf: while rcx > rdx { rdx += rdx }; return rdx) as target.
    let mut obj_o = [0x1_422a_b600u64, 0u64];
    let mut obj_n = [0x1_422a_b600u64, 0u64];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;
    let r_o = f_orig(p_o, p_o as u64 + 16, 0, 0);
    let r_n = f_new(p_n, p_n as u64 + 16, 0, 0);
    crate::diff_eq!(r_o, p_o as u64 + 16, "ret orig");
    crate::diff_eq!(r_n, p_n as u64 + 16, "ret new");
    Ok(1)
}

/// `rend::rend_1422a3c90`
/// Original: `0x1422a3c90` (119 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422a3c90(_unused: usize) {
    let mut done: u8 = 0;
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_4130_4f70, (mgr, &mut done as *mut u8 as usize) -> ());
    while rd::<u8>(&done as *const u8 as usize) == 0 {
        let sched = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
        let worker_idx = call_va!(0x1_4240_7c80, () -> u32) as usize;
        let queue = rd::<usize>(sched + 0x2d8 + worker_idx * 8);
        call_va!(0x1_4247_5db0, (queue, 0xffff_ffffu32) -> ());
    }
}

pub unsafe fn diff_rend_1422a3c90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut DONE_PTR: usize = 0;
    static mut IMMEDIATE_DONE: bool = false;
    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_4f70(mgr: usize, done_ptr: usize) {
        unsafe {
            DONE_PTR = done_ptr;
            rec(0x10 ^ (mgr as u64));
            if IMMEDIATE_DONE {
                wr::<u8>(done_ptr, 1);
            }
        }
    }
    unsafe extern "win64" fn m_7c80() -> u32 {
        unsafe { rec(0x20) };
        1
    }
    unsafe extern "win64" fn m_5db0(queue: usize, timeout: u32) {
        unsafe {
            rec(0x30 ^ (queue as u64) ^ (timeout as u64));
            wr::<u8>(DONE_PTR, 1);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4130_4f70, m_4f70 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, m_7c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4247_5db0, m_5db0 as *const ())?;

    let g_mgr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let g_sched = exe::ptr::<usize>(0x1_43a0_82f0) as usize;
    let saved_mgr = rd::<usize>(g_mgr);
    let saved_sched = rd::<usize>(g_sched);

    let mut sched_buf = [0u8; 0x300];
    wr::<usize>(sched_buf.as_mut_ptr() as usize + 0x2d8 + 8, 0x9999);
    wr::<usize>(g_mgr, 0x5555);
    wr::<usize>(g_sched, sched_buf.as_ptr() as usize);

    for imm in [true, false] {
        IMMEDIATE_DONE = imm;
        TRACE = 1;
        f_orig(0);
        let tr_o = TRACE;

        TRACE = 1;
        f_new(0);
        let tr_n = TRACE;

        crate::diff_eq!(tr_o, tr_n, "1422a3c90 trace imm={imm}");
    }

    wr::<usize>(g_mgr, saved_mgr);
    wr::<usize>(g_sched, saved_sched);
    Ok(2)
}

/// `rend::rend_1422a3d10`
/// Original: `0x1422a3d10` (18 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a3d10(this: usize) -> u64 {
    // xchg qword [rcx+0x170], 0 ; xchg dword [rcx+0x178], 0 (lock-free accumulator drain)
    let old = crate::simd_ops::atomic_xchg_u64((this + 0x170) as u64, 0);
    let _ = crate::simd_ops::atomic_xchg_u32((this + 0x178) as u64, 0);
    old
}

pub unsafe fn diff_rend_1422a3d10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u8; 0x180];
    wr::<u64>(buf_o.as_mut_ptr() as usize + 0x170, 0xcafe_babe_dead_beef);
    wr::<u32>(buf_o.as_mut_ptr() as usize + 0x178, 0x1234_5678);
    let mut buf_n = buf_o;

    let r_o = f_orig(buf_o.as_mut_ptr() as usize);
    let r_n = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// `rend::rend_1422a3d30`
/// Original: `0x1422a3d30` (113 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422a3d30(this: usize) {
    let sub0 = rd::<usize>(this + 0x168);
    if rd::<u32>(sub0 + 0x30) != rd::<u32>(sub0 + 0x34) {
        loop {
            let sched = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
            let worker_idx = call_va!(0x1_4240_7c80, () -> u32) as usize;
            let queue = rd::<usize>(sched + 0x2d8 + worker_idx * 8);
            call_va!(0x1_4247_5db0, (queue, 0xffff_ffffu32) -> ());
            let sub = rd::<usize>(this + 0x168);
            if rd::<u32>(sub + 0x30) == rd::<u32>(sub + 0x34) {
                break;
            }
        }
    }
}

pub unsafe fn diff_rend_1422a3d30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for v in [0u32, 42u32] {
        let mut sub_o = [0u8; 0x40];
        wr::<u32>(sub_o.as_mut_ptr() as usize + 0x30, v);
        wr::<u32>(sub_o.as_mut_ptr() as usize + 0x34, v);
        let mut sub_n = sub_o;
        let mut obj_o = [0u8; 0x170];
        let mut obj_n = [0u8; 0x170];
        wr::<usize>(obj_o.as_mut_ptr() as usize + 0x168, sub_o.as_mut_ptr() as usize);
        wr::<usize>(obj_n.as_mut_ptr() as usize + 0x168, sub_n.as_mut_ptr() as usize);
        f_orig(obj_o.as_mut_ptr() as usize);
        f_new(obj_n.as_mut_ptr() as usize);
        crate::diff_eq!(sub_o, sub_n, "sub_buf");
    }
    Ok(2)
}

/// `rend::rend_1422a4800`
/// Original: `0x1422a4800` (183 bytes, `Throws::No`)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422a4800(
    out: usize,
    vec: usize,
    scale_inv: f32,
    white_point: f32,
) -> usize {
    rend_1422a4800_impl(out, vec, scale_inv, white_point)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422a4800(
    out: usize,
    vec: usize,
    _xmm0: f32,
    _xmm1: f32,
    scale_inv: f32,
    white_point: f32,
) -> usize {
    rend_1422a4800_impl(out, vec, scale_inv, white_point)
}

#[inline(always)]
unsafe fn rend_1422a4800_impl(
    out: usize,
    vec: usize,
    scale_inv: f32,
    white_point: f32,
) -> usize {
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let inv = if scale_inv != 0.0 {
        one / scale_inv
    } else {
        0.0
    };
    let r = inv * rd::<f32>(vec);
    let g = inv * rd::<f32>(vec + 0x4);
    let b = inv * rd::<f32>(vec + 0x8);
    let w_r = rd::<f32>(exe::ptr::<f32>(0x1_4342_6660) as usize);
    let w_g = rd::<f32>(exe::ptr::<f32>(0x1_4342_680c) as usize);
    let w_b = rd::<f32>(exe::ptr::<f32>(0x1_4342_6580) as usize);
    let lum = (r * w_r + g * w_g) + b * w_b;
    let denom = lum + one;
    let wp_sq = white_point * white_point;
    let num = (lum / wp_sq) + one;
    let factor = num / denom;
    let out_g = g * factor;
    let out_r = r * factor;
    let out_b = b * factor;
    wr::<f32>(vec + 0x8, out_b);
    wr::<f32>(vec, out_r);
    wr::<f32>(vec + 0x4, out_g);
    let rg_bits = rd::<u64>(vec);
    let b_bits = rd::<u32>(vec + 0x8);
    wr::<u64>(out, rg_bits);
    wr::<u32>(out + 0x8, b_bits);
    out
}

pub unsafe fn diff_rend_1422a4800(orig: usize, new: usize) -> Result<u32, String> {
    for (scale_inv, wp) in [(0.0f32, 1.0f32), (1.0, 2.0), (2.5, 4.0)] {
        let mut vec_o = [0.5f32.to_bits(), 1.25f32.to_bits(), 0.75f32.to_bits()];
        let mut vec_n = vec_o;
        let mut out_o = [0u32; 3];
        let mut out_n = [0u32; 3];
        let (r_o, _) = crate::simd_ops::call_va8_xmm(
            orig as u64,
            out_o.as_mut_ptr() as usize as u64,
            vec_o.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, scale_inv.to_bits() as u128, wp.to_bits() as u128],
        );
        let (r_n, _) = crate::simd_ops::call_va8_xmm(
            new as u64,
            out_n.as_mut_ptr() as usize as u64,
            vec_n.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, scale_inv.to_bits() as u128, wp.to_bits() as u128],
        );
        crate::diff_eq!(r_o as usize == out_o.as_ptr() as usize, r_n as usize == out_n.as_ptr() as usize, "ret");
        crate::diff_eq!(vec_o, vec_n, "vec");
        crate::diff_eq!(out_o, out_n, "out");
    }
    Ok(3)
}

/// `rend::rend_1422a48c0`
/// Original: `0x1422a48c0` (123 bytes, `Throws::No`)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422a48c0(out: usize, vec: usize, gamma: f32) -> usize {
    rend_1422a48c0_impl(out, vec, gamma)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422a48c0(
    out: usize,
    vec: usize,
    _xmm0: f32,
    _xmm1: f32,
    gamma: f32,
) -> usize {
    rend_1422a48c0_impl(out, vec, gamma)
}

#[inline(always)]
unsafe fn rend_1422a48c0_impl(out: usize, vec: usize, gamma: f32) -> usize {
    let one = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let exp = one / gamma;
    let powf_fn: unsafe extern "win64" fn(f32, f32) -> f32 = exe::func(0x1_4183_ad46);
    let r = powf_fn(rd::<f32>(vec), exp);
    wr::<f32>(vec, r);
    let g = powf_fn(rd::<f32>(vec + 0x4), exp);
    wr::<f32>(vec + 0x4, g);
    let b = powf_fn(rd::<f32>(vec + 0x8), exp);
    wr::<f32>(vec + 0x8, b);
    let b_bits = rd::<u32>(vec + 0x8);
    let rg_bits = rd::<u64>(vec);
    wr::<u64>(out, rg_bits);
    wr::<u32>(out + 0x8, b_bits);
    out
}

pub unsafe fn diff_rend_1422a48c0(orig: usize, new: usize) -> Result<u32, String> {
    for gamma in [1.0f32, 2.0, 2.2] {
        let mut vec_o = [0.25f32.to_bits(), 0.5f32.to_bits(), 1.0f32.to_bits()];
        let mut vec_n = vec_o;
        let mut out_o = [0u32; 3];
        let mut out_n = [0u32; 3];
        let (r_o, _) = crate::simd_ops::call_va8_xmm(
            orig as u64,
            out_o.as_mut_ptr() as usize as u64,
            vec_o.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, gamma.to_bits() as u128, 0],
        );
        let (r_n, _) = crate::simd_ops::call_va8_xmm(
            new as u64,
            out_n.as_mut_ptr() as usize as u64,
            vec_n.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, gamma.to_bits() as u128, 0],
        );
        crate::diff_eq!(r_o as usize == out_o.as_ptr() as usize, r_n as usize == out_n.as_ptr() as usize, "ret");
        crate::diff_eq!(vec_o, vec_n, "vec");
        crate::diff_eq!(out_o, out_n, "out");
    }
    Ok(3)
}

/// `rend::rend_1422a4950`
/// Original: `0x1422a4950` (115 bytes, `Throws::No`)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn rend_1422a4950(out: usize, vec: usize, exp: f32) -> usize {
    rend_1422a4950_impl(out, vec, exp)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn rend_1422a4950(
    out: usize,
    vec: usize,
    _xmm0: f32,
    _xmm1: f32,
    exp: f32,
) -> usize {
    rend_1422a4950_impl(out, vec, exp)
}

#[inline(always)]
unsafe fn rend_1422a4950_impl(out: usize, vec: usize, exp: f32) -> usize {
    let powf_fn: unsafe extern "win64" fn(f32, f32) -> f32 = exe::func(0x1_4183_ad46);
    let r = powf_fn(rd::<f32>(vec), exp);
    wr::<f32>(vec, r);
    let g = powf_fn(rd::<f32>(vec + 0x4), exp);
    wr::<f32>(vec + 0x4, g);
    let b = powf_fn(rd::<f32>(vec + 0x8), exp);
    wr::<f32>(vec + 0x8, b);
    let b_bits = rd::<u32>(vec + 0x8);
    let rg_bits = rd::<u64>(vec);
    wr::<u64>(out, rg_bits);
    wr::<u32>(out + 0x8, b_bits);
    out
}

pub unsafe fn diff_rend_1422a4950(orig: usize, new: usize) -> Result<u32, String> {
    for exp in [1.0f32, 2.0, 0.5] {
        let mut vec_o = [0.25f32.to_bits(), 0.5f32.to_bits(), 4.0f32.to_bits()];
        let mut vec_n = vec_o;
        let mut out_o = [0u32; 3];
        let mut out_n = [0u32; 3];
        let (r_o, _) = crate::simd_ops::call_va8_xmm(
            orig as u64,
            out_o.as_mut_ptr() as usize as u64,
            vec_o.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, exp.to_bits() as u128, 0],
        );
        let (r_n, _) = crate::simd_ops::call_va8_xmm(
            new as u64,
            out_n.as_mut_ptr() as usize as u64,
            vec_n.as_mut_ptr() as usize as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, exp.to_bits() as u128, 0],
        );
        crate::diff_eq!(r_o as usize == out_o.as_ptr() as usize, r_n as usize == out_n.as_ptr() as usize, "ret");
        crate::diff_eq!(vec_o, vec_n, "vec");
        crate::diff_eq!(out_o, out_n, "out");
    }
    Ok(3)
}

/// `rend::rend_1422a5af0`
/// Original: `0x1422a5af0` (48 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a5af0(this: usize) -> usize {
    call_va!(0x1_40fb_d5e0, (this + 0x5e0,) -> ());
    let vtbl = exe::ptr::<u8>(0x1_4330_bca8) as usize as u64;
    wr::<u64>(this + 0x300, vtbl);
    wr::<u64>(this + 0x20, vtbl);
    this
}

pub unsafe fn diff_rend_1422a5af0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut ctrl1_o = [0u64, 2u64];
    let mut ctrl2_o = [0u64, 3u64];
    let mut ctrl1_n = ctrl1_o;
    let mut ctrl2_n = ctrl2_o;

    let mut buf_o = [0u8; 0x600];
    let mut buf_n = [0u8; 0x600];
    wr::<usize>(buf_o.as_mut_ptr() as usize + 0x5e8, ctrl1_o.as_mut_ptr() as usize);
    wr::<usize>(buf_o.as_mut_ptr() as usize + 0x5f8, ctrl2_o.as_mut_ptr() as usize);
    wr::<usize>(buf_n.as_mut_ptr() as usize + 0x5e8, ctrl1_n.as_mut_ptr() as usize);
    wr::<usize>(buf_n.as_mut_ptr() as usize + 0x5f8, ctrl2_n.as_mut_ptr() as usize);

    let r_o = f_orig(buf_o.as_mut_ptr() as usize);
    let r_n = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o == buf_o.as_ptr() as usize, r_n == buf_n.as_ptr() as usize, "ret");
    crate::diff_eq!(ctrl1_o, ctrl1_n, "ctrl1");
    crate::diff_eq!(ctrl2_o, ctrl2_n, "ctrl2");
    crate::diff_eq!(&buf_o[..0x5e0], &buf_n[..0x5e0], "vtbl fields");
    Ok(1)
}

/// `rend::rend_1422a5b30`
/// Original: `0x1422a5b30` (100 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422a5b30(this: usize) {
    let idx = rd::<u32>(this) as usize;
    let table = exe::ptr::<u64>(0x1_43bd_6048) as usize;
    let renderer = rd::<u64>(table + idx * 8);
    call_va!(
        0x1_4226_c9b0,
        (renderer, this + 0x10, this + 0x300, this + 0x5e0) -> ()
    );
    call_va!(0x1_40fb_d5e0, (this + 0x5e0,) -> ());
    let vtbl = exe::ptr::<u8>(0x1_4330_bca8) as usize as u64;
    wr::<u64>(this + 0x300, vtbl);
    wr::<u64>(this + 0x20, vtbl);
}

pub unsafe fn diff_rend_1422a5b30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    unsafe extern "win64" fn m_c9b0(renderer: u64, a2: usize, a3: usize, _a4: usize) {
        wr::<u64>(a2, renderer ^ 0x1111);
        wr::<u64>(a3 + 0x8, renderer ^ 0x2222);
    }
    let _m = crate::difftest::Mock::new(0x1_4226_c9b0, m_c9b0 as *const ())?;

    let table = exe::ptr::<u64>(0x1_43bd_6048) as usize;
    let saved_slot = rd::<u64>(table + 8);
    wr::<u64>(table + 8, 0xabcd_ef01_2345_6789);

    let mut ctrl1_o = [0u64, 2u64];
    let mut ctrl2_o = [0u64, 3u64];
    let mut ctrl1_n = ctrl1_o;
    let mut ctrl2_n = ctrl2_o;

    let mut buf_o = [0u8; 0x600];
    let mut buf_n = [0u8; 0x600];
    wr::<u32>(buf_o.as_mut_ptr() as usize, 1);
    wr::<usize>(buf_o.as_mut_ptr() as usize + 0x5e8, ctrl1_o.as_mut_ptr() as usize);
    wr::<usize>(buf_o.as_mut_ptr() as usize + 0x5f8, ctrl2_o.as_mut_ptr() as usize);
    wr::<u32>(buf_n.as_mut_ptr() as usize, 1);
    wr::<usize>(buf_n.as_mut_ptr() as usize + 0x5e8, ctrl1_n.as_mut_ptr() as usize);
    wr::<usize>(buf_n.as_mut_ptr() as usize + 0x5f8, ctrl2_n.as_mut_ptr() as usize);

    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);

    wr::<u64>(table + 8, saved_slot);

    crate::diff_eq!(ctrl1_o, ctrl1_n, "1422a5b30 ctrl1");
    crate::diff_eq!(ctrl2_o, ctrl2_n, "1422a5b30 ctrl2");
    crate::diff_eq!(&buf_o[..0x5e0], &buf_n[..0x5e0], "1422a5b30 buf");
    Ok(1)
}

/// `rend::rend_1422a5bc0`
/// Original: `0x1422a5bc0` (194 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a5bc0(this: usize, src: usize) -> usize {
    wr::<u32>(this, rd::<u32>(src));
    wr::<u32>(this + 0x10, rd::<u32>(src + 0x10));
    wr::<u32>(this + 0x14, rd::<u32>(src + 0x14));
    wr::<u8>(this + 0x18, rd::<u8>(src + 0x18));
    wr::<u8>(this + 0x19, rd::<u8>(src + 0x19));
    call_va!(0x1_40f3_3c50, (this + 0x20, src + 0x20) -> usize);
    call_va!(0x1_40f3_3c50, (this + 0x300, src + 0x300) -> usize);
    wr::<u64>(this + 0x5e0, 0);
    wr::<u64>(this + 0x5e8, 0);
    let p0 = rd::<u64>(src + 0x5e0);
    let c0 = rd::<u64>(src + 0x5e8);
    wr::<u64>(this + 0x5e0, p0);
    wr::<u64>(this + 0x5e8, c0);
    wr::<u64>(src + 0x5e0, 0);
    wr::<u64>(src + 0x5e8, 0);
    wr::<u64>(this + 0x5f0, 0);
    wr::<u64>(this + 0x5f8, 0);
    let p1 = rd::<u64>(src + 0x5f0);
    let c1 = rd::<u64>(src + 0x5f8);
    wr::<u64>(this + 0x5f0, p1);
    wr::<u64>(this + 0x5f8, c1);
    wr::<u64>(src + 0x5f0, 0);
    wr::<u64>(src + 0x5f8, 0);
    this
}

pub unsafe fn diff_rend_1422a5bc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src_o = [0u8; 0x600];
    for (i, b) in src_o.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(11).wrapping_add(5);
    }
    let mut src_n = src_o;
    let mut dst_o = [0u8; 0x600];
    let mut dst_n = [0u8; 0x600];

    let r_o = f_orig(dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize);
    let r_n = f_new(dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o == dst_o.as_ptr() as usize, r_n == dst_n.as_ptr() as usize, "ret");
    crate::diff_eq!(dst_o, dst_n, "dst");
    crate::diff_eq!(src_o, src_n, "src");
    Ok(1)
}

/// `rend::rend_1422a5cd0`
/// Original: `0x1422a5cd0` (180 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a5cd0(
    this: usize,
    idx_ptr: usize,
    view_bundle: usize,
    view2: usize,
    shared_pair: usize,
) -> usize {
    wr::<u32>(this, rd::<u32>(idx_ptr));
    wr::<u32>(this + 0x10, rd::<u32>(view_bundle));
    wr::<u32>(this + 0x14, rd::<u32>(view_bundle + 0x4));
    wr::<u8>(this + 0x18, rd::<u8>(view_bundle + 0x8));
    wr::<u8>(this + 0x19, rd::<u8>(view_bundle + 0x9));
    call_va!(0x1_40f3_3c50, (this + 0x20, view_bundle + 0x10) -> usize);
    call_va!(0x1_40f3_3c50, (this + 0x300, view2) -> usize);
    wr::<u64>(this + 0x5e0, 0);
    wr::<u64>(this + 0x5e8, 0);
    let ctrl0 = rd::<usize>(shared_pair + 0x8);
    if ctrl0 != 0 {
        crate::simd_ops::lk_add_u32((ctrl0 + 0x8) as u64, 1); // lock inc dword [rax+8]
    }
    wr::<u64>(this + 0x5e0, rd::<u64>(shared_pair));
    wr::<u64>(this + 0x5e8, rd::<u64>(shared_pair + 0x8));
    wr::<u64>(this + 0x5f0, 0);
    wr::<u64>(this + 0x5f8, 0);
    let ctrl1 = rd::<usize>(shared_pair + 0x18);
    if ctrl1 != 0 {
        crate::simd_ops::lk_add_u32((ctrl1 + 0x8) as u64, 1); // lock inc dword [rax+8]
    }
    wr::<u64>(this + 0x5f0, rd::<u64>(shared_pair + 0x10));
    wr::<u64>(this + 0x5f8, rd::<u64>(shared_pair + 0x18));
    this
}

pub unsafe fn diff_rend_1422a5cd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let idx = 7u32;
    let mut vb = [0u8; 0x300];
    let mut v2 = [0u8; 0x2e0];
    for (i, b) in vb.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(3).wrapping_add(1);
    }
    for (i, b) in v2.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(2);
    }

    let mut ctrl0_o = [0u64, 5u64];
    let mut ctrl1_o = [0u64, 9u64];
    let mut ctrl0_n = ctrl0_o;
    let mut ctrl1_n = ctrl1_o;

    let shared_o = [
        0x1111u64,
        ctrl0_o.as_mut_ptr() as u64,
        0x2222u64,
        ctrl1_o.as_mut_ptr() as u64,
    ];
    let shared_n = [
        0x1111u64,
        ctrl0_n.as_mut_ptr() as u64,
        0x2222u64,
        ctrl1_n.as_mut_ptr() as u64,
    ];

    let mut dst_o = [0u8; 0x600];
    let mut dst_n = [0u8; 0x600];

    let r_o = f_orig(
        dst_o.as_mut_ptr() as usize,
        (&idx as *const u32) as usize,
        vb.as_ptr() as usize,
        v2.as_ptr() as usize,
        shared_o.as_ptr() as usize,
    );
    let r_n = f_new(
        dst_n.as_mut_ptr() as usize,
        (&idx as *const u32) as usize,
        vb.as_ptr() as usize,
        v2.as_ptr() as usize,
        shared_n.as_ptr() as usize,
    );
    crate::diff_eq!(r_o == dst_o.as_ptr() as usize, r_n == dst_n.as_ptr() as usize, "ret");
    crate::diff_eq!(&dst_o[..0x5e0], &dst_n[..0x5e0], "dst views");
    crate::diff_eq!(ctrl0_o, ctrl0_n, "ctrl0");
    crate::diff_eq!(ctrl1_o, ctrl1_n, "ctrl1");
    Ok(1)
}

/// `rend::rend_1422a5d90`
/// Original: `0x1422a5d90` (36 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422a5d90(this: usize) {
    let idx = rd::<u32>(this) as usize;
    let table = exe::ptr::<u64>(0x1_43bd_6048) as usize;
    let renderer = rd::<u64>(table + idx * 8);
    call_va!(
        0x1_4226_c9b0,
        (renderer, this + 0x10, this + 0x300, this + 0x5e0) -> ()
    );
}

pub unsafe fn diff_rend_1422a5d90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    unsafe extern "win64" fn m_c9b0(renderer: u64, a2: usize, a3: usize, a4: usize) {
        wr::<u64>(a2, renderer ^ 0x11);
        wr::<u64>(a3, renderer ^ 0x22);
        wr::<u64>(a4, renderer ^ 0x33);
    }
    let _m = crate::difftest::Mock::new(0x1_4226_c9b0, m_c9b0 as *const ())?;

    let table = exe::ptr::<u64>(0x1_43bd_6048) as usize;
    let saved_slot = rd::<u64>(table);
    wr::<u64>(table, 0xfeed_face_cafe_babe);

    let mut buf_o = [0u8; 0x600];
    let mut buf_n = [0u8; 0x600];
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);

    wr::<u64>(table, saved_slot);

    crate::diff_eq!(buf_o, buf_n, "1422a5d90 buf");
    Ok(1)
}

/// `rend::rend_1422a5fc0`
/// Original: `0x1422a5fc0` (39 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422a5fc0(val: u32) {
    let mut slot = val;
    call_va!(0x1_4130_5680, (&mut slot as *mut u32 as usize,) -> ());
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_422a_3d30, (mgr,) -> ());
    call_va!(0x1_4249_d3a0, () -> ());
}

pub unsafe fn diff_rend_1422a5fc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_5680(slot_ptr: usize) {
        unsafe { rec(0x10 ^ (rd::<u32>(slot_ptr) as u64)) };
    }
    unsafe extern "win64" fn m_3d30(mgr: usize) {
        unsafe { rec(0x20 ^ (mgr as u64)) };
    }
    unsafe extern "win64" fn m_d3a0() {
        unsafe { rec(0x30) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4130_5680, m_5680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422a_3d30, m_3d30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4249_d3a0, m_d3a0 as *const ())?;

    let g_mgr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_mgr = rd::<usize>(g_mgr);
    wr::<usize>(g_mgr, 0x4321);

    TRACE = 1;
    f_orig(0x89ab_cdef);
    let tr_o = TRACE;

    TRACE = 1;
    f_new(0x89ab_cdef);
    let tr_n = TRACE;

    wr::<usize>(g_mgr, saved_mgr);

    crate::diff_eq!(tr_o, tr_n, "1422a5fc0 trace");
    Ok(1)
}

/// `rend::rend_1422a5ff0`
/// Original: `0x1422a5ff0` (148 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a5ff0(this: usize) -> usize {
    wr::<u32>(this + 0x8, 0x500);
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x14, 0);
    wr::<u32>(this + 0x1c, 0);
    wr::<u16>(this + 0x20, 0);
    wr::<u8>(this + 0x22, 0);
    wr::<u8>(this + 0x3c, 0);
    wr::<u64>(this + 0x58, 0);
    wr::<u16>(this + 0x60, 0);
    wr::<u64>(this + 0xc, 0x2d0);
    wr::<u32>(this + 0x24, 0x9fffc);
    wr::<u32>(this + 0x28, 0x10000);
    wr::<u32>(this + 0x2c, 0x10000);
    wr::<u32>(this + 0x30, 0x800000);
    wr::<u32>(this + 0x34, 0x8000);
    wr::<u32>(this + 0x38, 0x40000);
    wr::<u8>(this + 0x50, 1);
    wr::<u8>(this + 0x62, 1);
    let v64 = (rd::<u32>(this + 0x64) & 0xff00_0002) | 0x2;
    wr::<u32>(this + 0x64, v64);
    wr::<u32>(this + 0x68, 0);
    wr::<u8>(this + 0x6c, 0);
    wr::<u8>(this + 0x67, 2);
    wr::<u32>(this + 0x40, 0x100);
    wr::<u32>(this + 0x44, 0x20000);
    wr::<u32>(this + 0x48, 0x20000);
    wr::<u32>(this + 0x4c, 0x50000);
    this
}

pub unsafe fn diff_rend_1422a5ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for fill in [0x00u8, 0x5au8, 0xffu8] {
        let mut buf_o = [fill; 0x78];
        let mut buf_n = [fill; 0x78];
        let r_o = f_orig(buf_o.as_mut_ptr() as usize);
        let r_n = f_new(buf_n.as_mut_ptr() as usize);
        crate::diff_eq!(r_o == buf_o.as_ptr() as usize, r_n == buf_n.as_ptr() as usize, "ret");
        crate::diff_eq!(buf_o, buf_n, "buf fill={fill:#x}");
    }
    Ok(3)
}

/// `rend::rend_1422a6090`
/// Original: `0x1422a6090` (125 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422a6090(this: usize, cfg: usize) -> usize {
    wr::<u32>(exe::ptr::<u32>(0x1_4384_1644) as usize, 2);
    let mut flag: u8 = 1;
    let mut size: u32 = 0x10_0000;
    let alloc = call_va!(
        0x1_413d_4550,
        (this, cfg, &mut size as *mut u32 as usize, &mut flag as *mut u8 as usize) -> u64
    );
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_bda8) as usize, alloc);
    call_va!(0x1_4248_c290, (alloc, 0x4_0000u32) -> ());
    let id = call_va!(0x1_4136_e520, (exe::ptr::<u8>(0x1_43bd_6040) as usize,) -> u32);
    wr::<u32>(this, id);
    call_va!(0x1_4130_53f0, (this, cfg) -> ());
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_422a_3d30, (mgr,) -> ());
    this
}

pub unsafe fn diff_rend_1422a6090(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_4550(
        _this: usize,
        cfg: usize,
        size_ptr: usize,
        flag_ptr: usize,
    ) -> u64 {
        let sz = rd::<u32>(size_ptr) as u64;
        let fl = rd::<u8>(flag_ptr) as u64;
        unsafe { rec(0x10 ^ (cfg as u64) ^ (sz << 8) ^ fl) };
        0x7788_99aa_bbcc_ddeeu64
    }
    unsafe extern "win64" fn m_c290(alloc: u64, sz: u32) {
        unsafe { rec(0x20 ^ alloc ^ (sz as u64)) };
    }
    unsafe extern "win64" fn m_e520(arg: usize) -> u32 {
        unsafe { rec(0x30 ^ (arg as u64)) };
        42
    }
    unsafe extern "win64" fn m_53f0(this: usize, cfg: usize) {
        wr::<u32>(this + 0x4, rd::<u32>(this) ^ (cfg as u32));
        unsafe { rec(0x40 ^ (cfg as u64)) };
    }
    unsafe extern "win64" fn m_3d30(mgr: usize) {
        unsafe { rec(0x50 ^ (mgr as u64)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_413d_4550, m_4550 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_c290, m_c290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4136_e520, m_e520 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4130_53f0, m_53f0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_422a_3d30, m_3d30 as *const ())?;

    let g_1644 = exe::ptr::<u32>(0x1_4384_1644) as usize;
    let g_bda8 = exe::ptr::<u64>(0x1_43a3_bda8) as usize;
    let g_ff58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_1644 = rd::<u32>(g_1644);
    let saved_bda8 = rd::<u64>(g_bda8);
    let saved_ff58 = rd::<usize>(g_ff58);

    wr::<usize>(g_ff58, 0x9876);

    let mut this_o = [0u8; 32];
    let mut this_n = [0u8; 32];

    wr::<u32>(g_1644, 0);
    wr::<u64>(g_bda8, 0);
    TRACE = 1;
    let ro = f_orig(this_o.as_mut_ptr() as usize, 0x1234);
    let tr_o = TRACE;
    let g1_o = rd::<u32>(g_1644);
    let g2_o = rd::<u64>(g_bda8);

    wr::<u32>(g_1644, 0);
    wr::<u64>(g_bda8, 0);
    TRACE = 1;
    let rn = f_new(this_n.as_mut_ptr() as usize, 0x1234);
    let tr_n = TRACE;
    let g1_n = rd::<u32>(g_1644);
    let g2_n = rd::<u64>(g_bda8);

    wr::<u32>(g_1644, saved_1644);
    wr::<u64>(g_bda8, saved_bda8);
    wr::<usize>(g_ff58, saved_ff58);

    crate::diff_eq!(ro == this_o.as_ptr() as usize, rn == this_n.as_ptr() as usize, "1422a6090 ret");
    crate::diff_eq!(tr_o, tr_n, "1422a6090 trace");
    crate::diff_eq!(g1_o, g1_n, "1422a6090 g_1644");
    crate::diff_eq!(g2_o, g2_n, "1422a6090 g_bda8");
    crate::diff_eq!(this_o, this_n, "1422a6090 this");
    Ok(1)
}

/// `rend::rend_1422a6590`
/// Original: `0x1422a6590` (19 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a6590(
    this: usize,
    ptr_val: u64,
    a_ptr: usize,
    b_ptr: usize,
) -> usize {
    wr::<u32>(this + 0x8, rd::<u32>(a_ptr));
    wr::<u32>(this + 0xc, rd::<u32>(b_ptr));
    wr::<u64>(this, ptr_val);
    this
}

pub unsafe fn diff_rend_1422a6590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let a = 0x1357_2468u32;
    let b = 0x9abc_def0u32;
    let mut dst_o = [0u64; 2];
    let mut dst_n = [0u64; 2];
    let r_o = f_orig(
        dst_o.as_mut_ptr() as usize,
        0xfeed_face_cafe_babe,
        (&a as *const u32) as usize,
        (&b as *const u32) as usize,
    );
    let r_n = f_new(
        dst_n.as_mut_ptr() as usize,
        0xfeed_face_cafe_babe,
        (&a as *const u32) as usize,
        (&b as *const u32) as usize,
    );
    crate::diff_eq!(r_o == dst_o.as_ptr() as usize, r_n == dst_n.as_ptr() as usize, "ret");
    crate::diff_eq!(dst_o, dst_n, "dst");
    Ok(1)
}

/// `rend::rend_1422a6700`
/// Original: `0x1422a6700` (24 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422a6700(
    this: usize,
    ptr_val: u64,
    xmm_ptr: usize,
    qword_ptr: usize,
) -> usize {
    let q = rd::<u64>(qword_ptr);
    wr::<u64>(this, ptr_val);
    let v = rd::<[u8; 16]>(xmm_ptr);
    wr::<u64>(this + 0x18, q);
    wr::<[u8; 16]>(this + 0x8, v);
    this
}

pub unsafe fn diff_rend_1422a6700(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let xmm = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let q = 0x1122_3344_5566_7788u64;
    let mut dst_o = [0u64; 4];
    let mut dst_n = [0u64; 4];
    let r_o = f_orig(
        dst_o.as_mut_ptr() as usize,
        0xdead_beef_cafe_0000,
        xmm.as_ptr() as usize,
        (&q as *const u64) as usize,
    );
    let r_n = f_new(
        dst_n.as_mut_ptr() as usize,
        0xdead_beef_cafe_0000,
        xmm.as_ptr() as usize,
        (&q as *const u64) as usize,
    );
    crate::diff_eq!(r_o == dst_o.as_ptr() as usize, r_n == dst_n.as_ptr() as usize, "ret");
    crate::diff_eq!(dst_o, dst_n, "dst");
    Ok(1)
}

/// `rend::rend_1422aa0a0`
/// Original: `0x1422aa0a0` (89 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422aa0a0(
    _this: usize,
    buf: usize,
    flag: u8,
    count: u32,
    stride: u32,
    align: u32,
    mode: u32,
) {
    let req = count.wrapping_mul(stride) as u64;
    if rd::<u64>(buf + 0x10) >= req {
        return;
    }
    let r8 = stride as u64;
    let r9_32 = align.wrapping_sub(1).wrapping_add(req as u32);
    let neg_align = align.wrapping_neg() as u64;
    let r9 = (r9_32 as u64) & neg_align;
    let rcx = (r8.wrapping_sub(1)).wrapping_add(r9);
    let rem = rcx % r8;
    let aligned_bytes = rcx.wrapping_sub(rem);
    if flag != 0 {
        call_va!(0x1_4209_ccd0, (buf, aligned_bytes, r8, mode) -> ());
    } else {
        call_va!(0x1_4209_cf70, (buf, aligned_bytes, r8, mode) -> ());
    }
}

pub unsafe fn diff_rend_1422aa0a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8, u32, u32, u32, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for (cap, count, stride) in [(1024u64, 16u32, 64u32), (2048u64, 10u32, 64u32)] {
        let mut buf_o = [0u64, 0u64, cap, 0u64];
        let mut buf_n = buf_o;
        f_orig(0, buf_o.as_mut_ptr() as usize, 0, count, stride, 256, 0);
        f_new(0, buf_n.as_mut_ptr() as usize, 0, count, stride, 256, 0);
        crate::diff_eq!(buf_o, buf_n, "sufficient cap");
    }
    Ok(2)
}

/// `rend::rend_1422aa220`
/// Original: `0x1422aa220` (190 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422aa220(this: usize) {
    call_va!(0x1_4207_3740, (this,) -> ());
    let count = rd::<u32>(this + 0x1e8);
    let p_508 = rd::<usize>(this + 0x508);
    let p_500 = rd::<usize>(this + 0x500);
    let p_8 = rd::<usize>(this + 0x8);

    let mut desc = [0u8; 0x30];
    let desc_ptr = desc.as_mut_ptr() as usize;
    wr::<u64>(desc_ptr, rd::<u64>(p_500));
    wr::<u32>(desc_ptr + 0x8, 0);
    wr::<u32>(desc_ptr + 0xc, 4);
    wr::<u32>(desc_ptr + 0x10, count);
    wr::<u32>(desc_ptr + 0x14, 0);
    wr::<u64>(desc_ptr + 0x18, rd::<u64>(p_8));
    wr::<u64>(desc_ptr + 0x20, 0);
    wr::<u64>(desc_ptr + 0x28, rd::<u64>(p_508));

    let mut lock_guard = [0u64; 2];
    let guard_ptr = call_va!(0x1_411d_b050, (lock_guard.as_mut_ptr() as usize,) -> usize);
    let cmd_list = rd::<usize>(guard_ptr + 0x8);
    let vtbl = rd::<usize>(cmd_list);
    let fn_ptr = rd::<usize>(vtbl + 0x240);
    call_va!(fn_ptr, (cmd_list, desc_ptr, 0u64, 0u64) -> ());

    let lock = rd::<usize>(lock_guard.as_ptr() as usize);
    if lock != 0 {
        let next = rd::<i32>(lock + 0xc).wrapping_sub(1);
        wr::<i32>(lock + 0xc, next);
        if next == 0 {
            wr::<u32>(lock + 0x8, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (lock,) -> ());
        }
    }

    let mut fence_out = [0u8; 16];
    let res = call_va!(0x1_4207_34e0, (fence_out.as_mut_ptr() as usize, 1u32) -> usize);
    wr::<[u8; 16]>(this + 0x510, rd::<[u8; 16]>(res));
}

pub unsafe fn diff_rend_1422aa220(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut LOCK_PTR: usize = 0;
    static mut CAPTURED_DESC: [u8; 0x30] = [0u8; 0x30];

    unsafe extern "win64" fn m_3740(this: usize) {
        wr::<u32>(this + 0x10, 0x3740);
    }
    unsafe extern "win64" fn m_b050(guard_out: usize) -> usize {
        wr::<usize>(guard_out, unsafe { LOCK_PTR });
        wr::<usize>(guard_out + 0x8, unsafe { CMD_LIST_PTR });
        guard_out
    }
    unsafe extern "win64" fn m_vfunc_240(_cmd_list: usize, desc: usize, a3: u64, a4: u64) {
        let mut d = rd::<[u8; 0x30]>(desc);
        d[0x8] ^= (a3 as u8) ^ (a4 as u8);
        unsafe {
            CAPTURED_DESC = d;
        }
    }
    unsafe extern "win64" fn m_34e0(out: usize, flag: u32) -> usize {
        wr::<u64>(out, 0x1122_3344_5566_7788 ^ (flag as u64));
        wr::<u64>(out + 0x8, 0x99aa_bbcc_ddee_ff00);
        out
    }

    let _m1 = crate::difftest::Mock::new(0x1_4207_3740, m_3740 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_b050, m_b050 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_34e0, m_34e0 as *const ())?;

    let mut vtbl = [0usize; 0x250 / 8];
    vtbl[0x240 / 8] = m_vfunc_240 as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let v8 = 0xaaaa_bbbb_cccc_ddddu64;
    let v500 = 0x1111_2222_3333_4444u64;
    let v508 = 0x5555_6666_7777_8888u64;

    let mut lock_o = [0u8; 0x10];
    wr::<u32>(lock_o.as_mut_ptr() as usize + 0x8, 123);
    wr::<i32>(lock_o.as_mut_ptr() as usize + 0xc, 1);
    let mut lock_n = lock_o;

    let mut this_o = [0u8; 0x530];
    let po = this_o.as_mut_ptr() as usize;
    wr::<usize>(po + 0x8, (&v8 as *const u64) as usize);
    wr::<u32>(po + 0x1e8, 64);
    wr::<usize>(po + 0x500, (&v500 as *const u64) as usize);
    wr::<usize>(po + 0x508, (&v508 as *const u64) as usize);
    let mut this_n = this_o;

    LOCK_PTR = lock_o.as_mut_ptr() as usize;
    CAPTURED_DESC = [0u8; 0x30];
    f_orig(this_o.as_mut_ptr() as usize);
    let desc_o = CAPTURED_DESC;

    LOCK_PTR = lock_n.as_mut_ptr() as usize;
    CAPTURED_DESC = [0u8; 0x30];
    f_new(this_n.as_mut_ptr() as usize);
    let desc_n = CAPTURED_DESC;

    crate::diff_eq!(desc_o, desc_n, "1422aa220 desc");
    crate::diff_eq!(lock_o, lock_n, "1422aa220 lock");
    crate::diff_eq!(this_o, this_n, "1422aa220 this");
    Ok(1)
}

/// `rend::rend_1422aa2f0`
/// Original: `0x1422aa2f0` (165 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422aa2f0(this: usize, id: u32) {
    let dense_base = rd::<usize>(this + 0x4a8);
    let sparse_elem = rd::<usize>(this + 0x280).wrapping_add((id as usize) << 4);
    let len = rd::<u32>(this + 0x4b0) as usize;
    let dense_idx = rd::<u32>(sparse_elem + 0xc) as usize;
    let last_ptr = dense_base.wrapping_add((len * 3).wrapping_sub(3) * 4);
    let cur_ptr = dense_base.wrapping_add(dense_idx * 12);
    let saved_u32 = rd::<u32>(cur_ptr + 0x8);
    if cur_ptr != last_ptr {
        wr::<u64>(cur_ptr, rd::<u64>(last_ptr));
        wr::<u32>(cur_ptr + 0x8, rd::<u32>(last_ptr + 0x8));
    }
    let new_len = rd::<u32>(this + 0x4b0).wrapping_sub(1);
    wr::<u32>(this + 0x4b0, new_len);
    let idx_after = rd::<u32>(sparse_elem + 0xc);
    if (idx_after as u64) < (new_len as u64) {
        let dense2 = rd::<usize>(this + 0x4a8);
        let moved_id = rd::<u32>(dense2 + (idx_after as usize) * 12 + 0x4) as usize;
        let moved_sparse = rd::<usize>(this + 0x280).wrapping_add(moved_id << 4);
        wr::<u32>(moved_sparse + 0xc, idx_after);
        let idx_again = rd::<u32>(sparse_elem + 0xc) as usize;
        let dense3 = rd::<usize>(this + 0x4a8);
        wr::<u32>(dense3 + idx_again * 12 + 0x8, saved_u32);
    }
    let g_ptr = exe::ptr::<u32>(0x1_43a2_b298) as usize;
    wr::<u32>(g_ptr, rd::<u32>(g_ptr).wrapping_sub(1));
    wr::<u32>(sparse_elem + 0xc, 0xffff_ffff);
}

pub unsafe fn diff_rend_1422aa2f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ptr = exe::ptr::<u32>(0x1_43a2_b298) as usize;
    let saved_g = rd::<u32>(g_ptr);

    let mut sparse_o = [0u32; 8];
    sparse_o[3] = 0; // id 0 -> dense_idx 0
    sparse_o[7] = 1; // id 1 -> dense_idx 1
    let mut sparse_n = sparse_o;

    let mut dense_o = [1u32, 0, 100, 1, 1, 200];
    let mut dense_n = dense_o;

    let mut this_o = [0u8; 0x4c0];
    let mut this_n = [0u8; 0x4c0];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x280, sparse_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x4a8, dense_o.as_mut_ptr() as usize);
    wr::<u32>(this_o.as_mut_ptr() as usize + 0x4b0, 2);

    wr::<usize>(this_n.as_mut_ptr() as usize + 0x280, sparse_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x4a8, dense_n.as_mut_ptr() as usize);
    wr::<u32>(this_n.as_mut_ptr() as usize + 0x4b0, 2);

    wr::<u32>(g_ptr, 10);
    f_orig(this_o.as_mut_ptr() as usize, 0);
    let g_o = rd::<u32>(g_ptr);

    wr::<u32>(g_ptr, 10);
    f_new(this_n.as_mut_ptr() as usize, 0);
    let g_n = rd::<u32>(g_ptr);

    wr::<u32>(g_ptr, saved_g);

    crate::diff_eq!(g_o, g_n, "global counter");
    crate::diff_eq!(sparse_o, sparse_n, "sparse");
    crate::diff_eq!(dense_o, dense_n, "dense");
    crate::diff_eq!(
        rd::<u32>(this_o.as_ptr() as usize + 0x4b0),
        rd::<u32>(this_n.as_ptr() as usize + 0x4b0),
        "len"
    );
    Ok(1)
}

/// `rend::rend_1422aa3a0`
/// Original: `0x1422aa3a0` (162 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422aa3a0(this: usize, id: u32) -> u64 {
    let sparse_elem = rd::<usize>(this + 0x280).wrapping_add((id as usize) << 4);
    let len0 = rd::<u32>(this + 0x4b0);
    wr::<u32>(sparse_elem + 0xc, len0);
    let vec = this + 0x4a8;
    let len = rd::<u32>(vec + 0x8) as u64;
    let cap = rd::<u32>(vec + 0xc) as u64;
    if len.wrapping_mul(12) == cap.wrapping_mul(12) {
        call_va!(0x1_410c_0800, (vec, len.wrapping_add(1)) -> ());
    }
    let idx = rd::<u32>(vec + 0x8) as usize;
    let base = rd::<usize>(vec);
    wr::<u64>(base + idx * 12, 0);
    wr::<u32>(base + idx * 12 + 0x8, 0);
    let new_len = (idx as u32).wrapping_add(1);
    wr::<u32>(vec + 0x8, new_len);
    let new_len_usize = rd::<u32>(vec + 0x8) as usize;
    let base2 = rd::<usize>(vec);
    let slot = base2 + new_len_usize * 12 - 12;
    wr::<u8>(slot, 1);
    wr::<u32>(slot + 0x4, id);
    let g_ptr = exe::ptr::<u32>(0x1_43a2_b298) as usize;
    let g_val = rd::<u32>(g_ptr);
    wr::<u32>(slot + 0x8, g_val);
    let next = g_val.wrapping_add(1);
    wr::<u32>(g_ptr, next % 0xffff_fffe);
    (next / 0xffff_fffe) as u64
}

pub unsafe fn diff_rend_1422aa3a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ptr = exe::ptr::<u32>(0x1_43a2_b298) as usize;
    let saved_g = rd::<u32>(g_ptr);

    let mut sparse_o = [0u32; 8];
    let mut sparse_n = sparse_o;
    let mut dense_o = [0u32; 12];
    let mut dense_n = dense_o;

    let mut this_o = [0u8; 0x4c0];
    let mut this_n = [0u8; 0x4c0];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x280, sparse_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x4a8, dense_o.as_mut_ptr() as usize);
    wr::<u32>(this_o.as_mut_ptr() as usize + 0x4b0, 1);
    wr::<u32>(this_o.as_mut_ptr() as usize + 0x4b4, 4);

    wr::<usize>(this_n.as_mut_ptr() as usize + 0x280, sparse_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x4a8, dense_n.as_mut_ptr() as usize);
    wr::<u32>(this_n.as_mut_ptr() as usize + 0x4b0, 1);
    wr::<u32>(this_n.as_mut_ptr() as usize + 0x4b4, 4);

    wr::<u32>(g_ptr, 42);
    let r_o = f_orig(this_o.as_mut_ptr() as usize, 1);
    let g_o = rd::<u32>(g_ptr);

    wr::<u32>(g_ptr, 42);
    let r_n = f_new(this_n.as_mut_ptr() as usize, 1);
    let g_n = rd::<u32>(g_ptr);

    wr::<u32>(g_ptr, saved_g);

    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(g_o, g_n, "global counter");
    crate::diff_eq!(sparse_o, sparse_n, "sparse");
    crate::diff_eq!(dense_o, dense_n, "dense");
    Ok(1)
}

/// `rend::rend_1422ab600`
/// Original: `0x1422ab600` (17 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422ab600(target: u64, mut cur: u64) -> u64 {
    while target > cur {
        cur = cur.wrapping_add(cur);
    }
    cur
}

pub unsafe fn diff_rend_1422ab600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for (target, cur) in [(0u64, 1u64), (1, 1), (2, 1), (3, 1), (100, 8), (1024, 16)] {
        crate::diff_eq!(f_orig(target, cur), f_new(target, cur), "ab600({target}, {cur})");
    }
    Ok(6)
}

/// `rend::rend_1422ab620`
/// Original: `0x1422ab620` (45 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422ab620(this: usize) -> u32 {
    let base = rd::<usize>(this + 0x1a8);
    let len = rd::<u32>(this + 0x1b0) as usize;
    let end = base.wrapping_add(len * 8);
    if base != end {
        let start = rd::<u32>(end - 0x8);
        let count = rd::<u32>(end - 0x4);
        if start.wrapping_add(count) == rd::<u32>(this + 0x1a0) {
            return start;
        }
    }
    rd::<u32>(this + 0x1a0)
}

pub unsafe fn diff_rend_1422ab620(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let entries = [10u32, 5u32, 20u32, 12u32];
    let mut buf = [0u8; 0x1c0];
    let p = buf.as_mut_ptr() as usize;
    wr::<usize>(p + 0x1a8, entries.as_ptr() as usize);

    // Case 1: len == 0
    wr::<u32>(p + 0x1b0, 0);
    wr::<u32>(p + 0x1a0, 50);
    crate::diff_eq!(f_orig(p), f_new(p), "empty");

    // Case 2: len == 2, 20 + 12 == 32 (matches +0x1a0)
    wr::<u32>(p + 0x1b0, 2);
    wr::<u32>(p + 0x1a0, 32);
    crate::diff_eq!(f_orig(p), f_new(p), "match");

    // Case 3: len == 2, 20 + 12 != 40
    wr::<u32>(p + 0x1a0, 40);
    crate::diff_eq!(f_orig(p), f_new(p), "mismatch");
    Ok(3)
}

/// `rend::rend_1422aca60`
/// Original: `0x1422aca60` (180 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_1422aca60(this: usize) {
    let mut cur = rd::<usize>(this + 0x4e0);
    let end = cur.wrapping_add((rd::<u32>(this + 0x4e8) as usize) << 4);
    while cur != end {
        let sched = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        call_va!(0x1_4248_5780, (sched, cur) -> ());
        cur = cur.wrapping_add(0x10);
    }
    let arr = rd::<usize>(this + 0x4f0);
    let cnt = rd::<u32>(this + 0x4f8) as usize;
    if arr != arr.wrapping_add(cnt * 8) && rd::<u64>(arr) != 0 {
        call_va!(0x1_4205_f870, (cnt as u64, arr) -> ());
    }
    wr::<u32>(this + 0x4e8, 0);
    wr::<u32>(this + 0x4f8, 0);
    let sub = this + 0x3c0;
    let vtbl = rd::<usize>(sub);
    let fn_ptr = rd::<usize>(vtbl + 0x8);
    call_va!(fn_ptr, (sub,) -> ());
    let res = rd::<u64>(this + 0x518);
    if res != 0 {
        let h = rd::<u64>(this + 0x510);
        call_va!(0x1_4207_02c0, (res, h) -> ());
    }
}

pub unsafe fn diff_rend_1422aca60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    // vtbl[1] = 0x1422a3d10 (zeroes [rcx + 0x170] and [rcx + 0x178], i.e. this + 0x530 and 0x538)
    let vtbl = [0u64, 0x1_422a_3d10u64];
    let arr = [0u64, 0u64];
    let mut buf_o = [0u8; 0x550];
    let p_o = buf_o.as_mut_ptr() as usize;
    wr::<usize>(p_o + 0x3c0, vtbl.as_ptr() as usize);
    wr::<usize>(p_o + 0x4f0, arr.as_ptr() as usize);
    wr::<u32>(p_o + 0x4f8, 2);
    wr::<u64>(p_o + 0x530, 0x1234_5678_9abc_def0);
    wr::<u32>(p_o + 0x538, 0x7788_99aa);
    let mut buf_n = buf_o;
    let p_n = buf_n.as_mut_ptr() as usize;
    wr::<usize>(p_n + 0x3c0, vtbl.as_ptr() as usize);
    wr::<usize>(p_n + 0x4f0, arr.as_ptr() as usize);

    f_orig(p_o);
    f_new(p_n);
    crate::diff_eq!(buf_o, buf_n, "rend_1422aca60");
    Ok(1)
}

/// `rend::rend_1422aceb0`
/// Original: `0x1422aceb0` (14 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422aceb0() {
    let v = rd::<u8>(exe::ptr::<u8>(0x1_4386_a820) as usize);
    wr::<u8>(exe::ptr::<u8>(0x1_4384_3fcf) as usize, v);
}

pub unsafe fn diff_rend_1422aceb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let src_ptr = exe::ptr::<u8>(0x1_4386_a820) as usize;
    let dst_ptr = exe::ptr::<u8>(0x1_4384_3fcf) as usize;
    let saved_src = rd::<u8>(src_ptr);
    let saved_dst = rd::<u8>(dst_ptr);

    for val in [0u8, 1u8, 0x5au8] {
        wr::<u8>(src_ptr, val);
        wr::<u8>(dst_ptr, !val);
        f_orig();
        let o = rd::<u8>(dst_ptr);

        wr::<u8>(dst_ptr, !val);
        f_new();
        let n = rd::<u8>(dst_ptr);
        crate::diff_eq!(o, n, "copied byte {val}");
    }

    wr::<u8>(src_ptr, saved_src);
    wr::<u8>(dst_ptr, saved_dst);
    Ok(3)
}

/// `rend::rend_1422e1190`
/// Original: `0x1422e1190` (176 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422e1190(this: usize) {
    let id = rd::<i32>(this);
    if id != -1 {
        call_va!(0x1_4233_01f0, (id,) -> ());
        let id2 = rd::<i32>(this);
        call_va!(0x1_4233_0010, (id2,) -> ());
    }
    call_va!(0x1_401d_e830, (this + 0x110,) -> ());
    call_va!(0x1_401d_e830, (this + 0x100,) -> ());
    call_va!(0x1_401d_e830, (this + 0xe8,) -> ());
    call_va!(0x1_401d_e830, (this + 0xd0,) -> ());
    call_va!(0x1_401d_e830, (this + 0xc0,) -> ());
    let vtbl = exe::ptr::<u8>(0x1_433f_8f18) as usize as u64;
    wr::<u64>(this + 0x28, vtbl);
    call_va!(0x1_412e_b320, (this + 0x28,) -> ());
    call_va!(0x1_401d_e830, (this + 0x18,) -> ());
    let sub = rd::<usize>(this + 0x8);
    if sub != 0 {
        call_va!(0x1_4209_d670, (sub,) -> ());
        call_va!(0x1_411e_7fc0, (sub + 0x58,) -> ());
        call_va!(0x1_4244_2de0, (sub,) -> ());
    }
}

pub unsafe fn diff_rend_1422e1190(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u8; 0x128];
    let p_o = buf_o.as_mut_ptr() as usize;
    wr::<i32>(p_o, -1);
    for off in [0x20usize, 0xc8, 0xd8, 0xf0, 0x108, 0x118] {
        wr::<u64>(p_o + off, 0xdead_beef_cafe_babe);
    }
    let mut buf_n = buf_o;

    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(buf_o, buf_n, "rend_1422e1190");
    Ok(1)
}

/// `rend::ParticleSystemResource::vfunc_13`
/// Original: `0x1422f5790` (52 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f5790(this: usize) {
    let ptr_c0 = rd::<u64>(this + 0xc0);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_4130_f980, (mgr, ptr_c0) -> ());
    wr::<u64>(this + 0xc0, 0);
    call_va!(0x1_4242_87f0, (this,) -> ());
}

pub unsafe fn diff_rend_1422f5790(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_f980(mgr: usize, ptr_c0: u64) {
        unsafe {
            TRACE = (mgr as u64) ^ ptr_c0;
        }
    }
    unsafe extern "win64" fn m_87f0(this: usize) {
        wr::<u32>(this + 0x10, 0x87f0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4130_f980, m_f980 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4242_87f0, m_87f0 as *const ())?;

    let g_mgr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_mgr = rd::<usize>(g_mgr);
    wr::<usize>(g_mgr, 0x1357_2468);

    let mut buf_o = [0u8; 0xd0];
    wr::<u64>(buf_o.as_mut_ptr() as usize + 0xc0, 0xcafe_babe);
    let mut buf_n = buf_o;

    TRACE = 0;
    f_orig(buf_o.as_mut_ptr() as usize);
    let tr_o = TRACE;

    TRACE = 0;
    f_new(buf_n.as_mut_ptr() as usize);
    let tr_n = TRACE;

    wr::<usize>(g_mgr, saved_mgr);

    crate::diff_eq!(tr_o, tr_n, "1422f5790 trace");
    crate::diff_eq!(buf_o, buf_n, "1422f5790 buf");
    Ok(1)
}

/// `rend::rend_1422f5c20`
/// Original: `0x1422f5c20` (55 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f5c20(this: usize) -> usize {
    call_va!(0x1_4242_8d70, (this,) -> usize);
    let vtbl_b8 = exe::ptr::<u8>(0x1_433f_a148) as usize as u64;
    wr::<u64>(this + 0xc0, 0);
    let vtbl_0 = exe::ptr::<u8>(0x1_433f_9d88) as usize as u64;
    wr::<u64>(this + 0xb8, vtbl_b8);
    wr::<u64>(this, vtbl_0);
    this
}

pub unsafe fn diff_rend_1422f5c20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0x5au8; 0xd0];
    let mut buf_n = [0x5au8; 0xd0];
    let p_o = buf_o.as_mut_ptr() as usize;
    let p_n = buf_n.as_mut_ptr() as usize;
    let r_o = f_orig(p_o);
    let r_n = f_new(p_n);
    crate::diff_eq!(r_o, p_o, "ret orig");
    crate::diff_eq!(r_n, p_n, "ret new");
    // Normalize self-referential intrusive list pointers at +0x40..+0x80 before comparing bytes
    for off in [0x40usize, 0x48, 0x50, 0x60, 0x68, 0x70] {
        let rel_o = rd::<usize>(p_o + off).wrapping_sub(p_o);
        let rel_n = rd::<usize>(p_n + off).wrapping_sub(p_n);
        crate::diff_eq!(rel_o, rel_n, "intrusive ptr +{off:#x}");
        wr::<usize>(p_o + off, 0);
        wr::<usize>(p_n + off, 0);
    }
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// `rend::ParticleSystemResource::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x1422f6200` (58 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6200(_this: usize, dst: usize) -> usize {
    call_va!(0x1_4242_8d70, (dst,) -> usize);
    let vtbl_b8 = exe::ptr::<u8>(0x1_433f_a148) as usize as u64;
    wr::<u64>(dst + 0xc0, 0);
    let vtbl_0 = exe::ptr::<u8>(0x1_433f_9d88) as usize as u64;
    wr::<u64>(dst + 0xb8, vtbl_b8);
    wr::<u64>(dst, vtbl_0);
    dst
}

pub unsafe fn diff_rend_1422f6200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0x5au8; 0xd0];
    let mut buf_n = [0x5au8; 0xd0];
    let p_o = buf_o.as_mut_ptr() as usize;
    let p_n = buf_n.as_mut_ptr() as usize;
    let r_o = f_orig(0, p_o);
    let r_n = f_new(0, p_n);
    crate::diff_eq!(r_o, p_o, "ret orig");
    crate::diff_eq!(r_n, p_n, "ret new");
    for off in [0x40usize, 0x48, 0x50, 0x60, 0x68, 0x70] {
        let rel_o = rd::<usize>(p_o + off).wrapping_sub(p_o);
        let rel_n = rd::<usize>(p_n + off).wrapping_sub(p_n);
        crate::diff_eq!(rel_o, rel_n, "intrusive ptr +{off:#x}");
        wr::<usize>(p_o + off, 0);
        wr::<usize>(p_n + off, 0);
    }
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// `rend::rend_1422f6aa0`
/// Original: `0x1422f6aa0` (166 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6aa0(this: usize, item: usize) {
    let len = rd::<u32>(this + 0x8) as u64;
    let cap = rd::<u32>(this + 0xc) as u64;
    if len.wrapping_mul(8) == cap.wrapping_mul(8) {
        call_va!(0x1_4020_bcb0, (this, len.wrapping_add(1)) -> ());
    }
    let idx = rd::<u32>(this + 0x8) as usize;
    let base = rd::<usize>(this);
    wr::<usize>(base + idx * 8, item);
    wr::<u32>(this + 0x8, (idx as u32).wrapping_add(1));

    let count0 = rd::<u8>(item + 0x1c2);
    let mut i: usize = 0;
    if rd::<u8>(item + 0x1c3) != 0 {
        if count0 != 0 {
            loop {
                call_va!(0x1_4205_da30, () -> ());
                i += 1;
                if i >= rd::<u8>(item + 0x1c2) as usize {
                    break;
                }
            }
        }
    } else if count0 != 0 {
        loop {
            call_va!(0x1_4205_f070, () -> ());
            i += 1;
            if i >= rd::<u8>(item + 0x1c2) as usize {
                break;
            }
        }
    }
    wr::<u8>(item + 0x1c4, 1);
}

pub unsafe fn diff_rend_1422f6aa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for flag_1c3 in [0u8, 1u8] {
        let mut arr_o = [0usize; 4];
        let mut arr_n = [0usize; 4];
        let mut vec_o = [arr_o.as_mut_ptr() as u64, (4u64 << 32) | 1];
        let mut vec_n = [arr_n.as_mut_ptr() as u64, (4u64 << 32) | 1];
        let mut item_o = [0u8; 0x1d0];
        item_o[0x1c2] = 0;
        item_o[0x1c3] = flag_1c3;
        let mut item_n = item_o;

        f_orig(vec_o.as_mut_ptr() as usize, item_o.as_mut_ptr() as usize);
        f_new(vec_n.as_mut_ptr() as usize, item_n.as_mut_ptr() as usize);
        crate::diff_eq!(vec_o[1], vec_n[1], "vec len/cap");
        crate::diff_eq!(
            arr_o[1] == item_o.as_ptr() as usize,
            arr_n[1] == item_n.as_ptr() as usize,
            "pushed ptr"
        );
        crate::diff_eq!(item_o, item_n, "item");
    }
    Ok(2)
}

/// `rend::rend_1422f6b50`
/// Original: `0x1422f6b50` (144 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6b50(
    this: usize,
    pos_ptr: usize,
    bounds_ptr: usize,
    flag: u8,
) {
    wr::<u64>(this + 0x10, rd::<u64>(pos_ptr));
    wr::<u32>(this + 0x18, rd::<u32>(pos_ptr + 0x8));
    wr::<[u8; 16]>(this + 0x1c, rd::<[u8; 16]>(bounds_ptr));
    wr::<u64>(this + 0x2c, rd::<u64>(bounds_ptr + 0x10));
    let old_48 = rd::<u32>(this + 0x48);
    let new_48 = if flag != 0 { old_48 | 3 } else { old_48 & !3 };
    wr::<u32>(this + 0x48, new_48);
    rend_1422f6c40(this);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff80) as usize);
    let len = rd::<u32>(mgr + 0x68) as u64;
    let cap = rd::<u32>(mgr + 0x6c) as u64;
    if len.wrapping_mul(8) == cap.wrapping_mul(8) {
        call_va!(0x1_4020_bcb0, (mgr + 0x60, len.wrapping_add(1)) -> ());
    }
    let idx = rd::<u32>(mgr + 0x68) as usize;
    let base = rd::<usize>(mgr + 0x60);
    wr::<usize>(base + idx * 8, this);
    wr::<u32>(mgr + 0x68, (idx as u32).wrapping_add(1));
}

pub unsafe fn diff_rend_1422f6b50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ptr = exe::ptr::<usize>(0x1_4397_ff80) as usize;
    let saved_g = rd::<usize>(g_ptr);

    let pos = [1u32, 2, 3];
    let bounds = [4u32, 5, 6, 7, 8, 9];

    for flag in [0u8, 1u8] {
        let mut arr_o = [0usize; 4];
        let mut arr_n = [0usize; 4];
        let mut mgr_o = [0u8; 0x80];
        let mut mgr_n = [0u8; 0x80];
        wr::<usize>(mgr_o.as_mut_ptr() as usize + 0x60, arr_o.as_mut_ptr() as usize);
        wr::<u32>(mgr_o.as_mut_ptr() as usize + 0x68, 1);
        wr::<u32>(mgr_o.as_mut_ptr() as usize + 0x6c, 4);

        wr::<usize>(mgr_n.as_mut_ptr() as usize + 0x60, arr_n.as_mut_ptr() as usize);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + 0x68, 1);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + 0x6c, 4);

        let mut this_o = [0u8; 0x50];
        wr::<u32>(this_o.as_mut_ptr() as usize + 0x48, 0x12);
        let mut this_n = this_o;

        wr::<usize>(g_ptr, mgr_o.as_mut_ptr() as usize);
        f_orig(
            this_o.as_mut_ptr() as usize,
            pos.as_ptr() as usize,
            bounds.as_ptr() as usize,
            flag,
        );

        wr::<usize>(g_ptr, mgr_n.as_mut_ptr() as usize);
        f_new(
            this_n.as_mut_ptr() as usize,
            pos.as_ptr() as usize,
            bounds.as_ptr() as usize,
            flag,
        );

        wr::<usize>(g_ptr, saved_g);

        crate::diff_eq!(this_o, this_n, "this flag={flag}");
        crate::diff_eq!(
            rd::<u32>(mgr_o.as_ptr() as usize + 0x68),
            rd::<u32>(mgr_n.as_ptr() as usize + 0x68),
            "mgr len"
        );
        crate::diff_eq!(
            arr_o[1] == this_o.as_ptr() as usize,
            arr_n[1] == this_n.as_ptr() as usize,
            "pushed this"
        );
    }
    Ok(2)
}

/// `rend::rend_1422f6bf0`
/// Original: `0x1422f6bf0` (65 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6bf0(this: usize) {
    rend_1422f6c40(this);
    let v_1c = rd::<u64>(exe::ptr::<u64>(0x1_4315_9a38) as usize);
    wr::<u64>(this + 0x1c, v_1c);
    let v_24 = rd::<u32>(exe::ptr::<u32>(0x1_4315_9a40) as usize);
    let v_28 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7178) as usize);
    wr::<u32>(this + 0x24, v_24);
    wr::<u32>(this + 0x28, v_28);
    wr::<u32>(this + 0x2c, v_28);
    wr::<u32>(this + 0x30, v_28);
}

pub unsafe fn diff_rend_1422f6bf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u8; 0x40];
    let mut buf_n = [0u8; 0x40];
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(buf_o, buf_n, "rend_1422f6bf0");
    Ok(1)
}

/// `rend::rend_1422f6c40`
/// Original: `0x1422f6c40` (103 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6c40(this: usize) {
    let mut cur = rd::<usize>(this);
    let len = rd::<u32>(this + 0x8) as usize;
    let end = cur.wrapping_add(len * 8);
    if cur != end {
        while cur != end {
            let elem = rd::<u64>(cur);
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff80) as usize);
            call_va!(0x1_422f_d780, (mgr, elem) -> ());
            cur = cur.wrapping_add(8);
        }
        wr::<u32>(this + 0x8, 0);
        return;
    }
    wr::<u32>(this + 0x8, 0);
}

pub unsafe fn diff_rend_1422f6c40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let elems = [0u64, 0u64];
    for cnt in [0u32, 2u32] {
        let mut vec_o = [elems.as_ptr() as u64, (4u64 << 32) | (cnt as u64)];
        let mut vec_n = vec_o;
        f_orig(vec_o.as_mut_ptr() as usize);
        f_new(vec_n.as_mut_ptr() as usize);
        crate::diff_eq!(vec_o, vec_n, "rend_1422f6c40 cnt={cnt}");
    }
    Ok(2)
}

/// `rend::rend_1422f6cb0`
/// Original: `0x1422f6cb0` (116 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6cb0(this: usize) {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff80) as usize);
    let mut cur = rd::<usize>(mgr + 0x60);
    let mut len = rd::<u32>(mgr + 0x68) as usize;
    if cur != cur.wrapping_add(len * 8) {
        let mut base = cur;
        loop {
            if rd::<usize>(cur) == this {
                let last = base.wrapping_add((len - 1) * 8);
                if cur != last {
                    wr::<u64>(cur, rd::<u64>(last));
                }
                let new_len = rd::<u32>(mgr + 0x68).wrapping_sub(1);
                wr::<u32>(mgr + 0x68, new_len);
                len = new_len as usize;
            } else {
                cur = cur.wrapping_add(8);
            }
            base = rd::<usize>(mgr + 0x60);
            if cur == base.wrapping_add(len * 8) {
                break;
            }
        }
    }
    rend_1422f6c40(this);
    call_va!(0x1_401d_e830, (this,) -> ());
}

pub unsafe fn diff_rend_1422f6cb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ptr = exe::ptr::<usize>(0x1_4397_ff80) as usize;
    let saved_g = rd::<usize>(g_ptr);

    let mut this_o = [0u64, 0x1234_0000_0000u64];
    let mut this_n = [0u64, 0x1234_0000_0000u64];
    let mut arr_o = [this_o.as_mut_ptr() as usize, 0x9999usize];
    let mut arr_n = [this_n.as_mut_ptr() as usize, 0x9999usize];

    let mut mgr_o = [0u8; 0x80];
    let mut mgr_n = [0u8; 0x80];
    wr::<usize>(mgr_o.as_mut_ptr() as usize + 0x60, arr_o.as_mut_ptr() as usize);
    wr::<u32>(mgr_o.as_mut_ptr() as usize + 0x68, 2);
    wr::<usize>(mgr_n.as_mut_ptr() as usize + 0x60, arr_n.as_mut_ptr() as usize);
    wr::<u32>(mgr_n.as_mut_ptr() as usize + 0x68, 2);

    wr::<usize>(g_ptr, mgr_o.as_mut_ptr() as usize);
    f_orig(this_o.as_mut_ptr() as usize);

    wr::<usize>(g_ptr, mgr_n.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize);

    wr::<usize>(g_ptr, saved_g);

    crate::diff_eq!(this_o, this_n, "this");
    crate::diff_eq!(arr_o[0], arr_n[0], "swap removed elem");
    crate::diff_eq!(
        rd::<u32>(mgr_o.as_ptr() as usize + 0x68),
        rd::<u32>(mgr_n.as_ptr() as usize + 0x68),
        "mgr len"
    );
    Ok(1)
}

/// `rend::rend_1422f6d30`
/// Original: `0x1422f6d30` (101 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6d30(this: usize) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u32>(this + 0x18, 0);
    let v_1c = rd::<u64>(exe::ptr::<u64>(0x1_4315_9a38) as usize);
    wr::<u64>(this + 0x1c, v_1c);
    let v_24 = rd::<u32>(exe::ptr::<u32>(0x1_4315_9a40) as usize);
    let v_28 = rd::<u32>(exe::ptr::<u32>(0x1_4342_7178) as usize);
    wr::<u32>(this + 0x24, v_24);
    wr::<u32>(this + 0x28, v_28);
    wr::<u32>(this + 0x2c, v_28);
    wr::<u32>(this + 0x30, v_28);
    wr::<u32>(this + 0x34, 0x3f80_0000);
    wr::<u32>(this + 0x38, 0x3f80_0000);
    wr::<u32>(this + 0x3c, 0x3f80_0000);
    wr::<u32>(this + 0x40, 0x3f80_0000);
    wr::<u64>(this + 0x44, 0x3f80_0000);
    this
}

pub unsafe fn diff_rend_1422f6d30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0x5au8; 0x50];
    let mut buf_n = [0x5au8; 0x50];
    let r_o = f_orig(buf_o.as_mut_ptr() as usize);
    let r_n = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o == buf_o.as_ptr() as usize, r_n == buf_n.as_ptr() as usize, "ret");
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// `anonymous_namespace::RenderInstancesFromGfxGraphCommand::vfunc_0`
/// Original: `0x1422f6ef0` (47 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6ef0(this: usize, flags: u32) -> usize {
    let vtbl = exe::ptr::<u8>(0x1_433f_a570) as usize as u64;
    wr::<u64>(this + 0x20, vtbl);
    wr::<u64>(this, vtbl);
    if (flags & 1) != 0 {
        call_va!(0x1_4183_4d30, (this, 0xf8u32) -> ());
    }
    this
}

pub unsafe fn diff_rend_1422f6ef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for flags in [0u32, 2u32] {
        let mut buf_o = [0u64; 6];
        let mut buf_n = [0u64; 6];
        let r_o = f_orig(buf_o.as_mut_ptr() as usize, flags);
        let r_n = f_new(buf_n.as_mut_ptr() as usize, flags);
        crate::diff_eq!(r_o == buf_o.as_ptr() as usize, r_n == buf_n.as_ptr() as usize, "ret");
        crate::diff_eq!(buf_o, buf_n, "buf flags={flags}");
    }
    Ok(2)
}

/// `anonymous_namespace::RenderParticleFromGfxGraphCommand::vfunc_0`
/// Original: `0x1422f6fb0` (47 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1422f6fb0(this: usize, flags: u32) -> usize {
    let vtbl = exe::ptr::<u8>(0x1_433f_a570) as usize as u64;
    wr::<u64>(this + 0x20, vtbl);
    wr::<u64>(this, vtbl);
    if (flags & 1) != 0 {
        call_va!(0x1_4183_4d30, (this, 0x140u32) -> ());
    }
    this
}

pub unsafe fn diff_rend_1422f6fb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for flags in [0u32, 2u32] {
        let mut buf_o = [0u64; 6];
        let mut buf_n = [0u64; 6];
        let r_o = f_orig(buf_o.as_mut_ptr() as usize, flags);
        let r_n = f_new(buf_n.as_mut_ptr() as usize, flags);
        crate::diff_eq!(r_o == buf_o.as_ptr() as usize, r_n == buf_n.as_ptr() as usize, "ret");
        crate::diff_eq!(buf_o, buf_n, "buf flags={flags}");
    }
    Ok(2)
}

// FLOAT_RETURN: none
// REGISTRY:
// Replacement::verified(0x1422a3830, "rend::rend_1422a3830", crate::ns::rend::rend_e18_b::rend_1422a3830 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a3830),
// Replacement::verified(0x1422a3850, "rend::rend_1422a3850", crate::ns::rend::rend_e18_b::rend_1422a3850 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a3850),
// Replacement::verified(0x1422a3c40, "rend::rend_1422a3c40", crate::ns::rend::rend_e18_b::rend_1422a3c40 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a3c40),
// Replacement::verified(0x1422a3c80, "rend::rend_1422a3c80", crate::ns::rend::rend_e18_b::rend_1422a3c80 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a3c80),
// Replacement::verified_throws(0x1422a3c90, "rend::rend_1422a3c90", crate::ns::rend::rend_e18_b::rend_1422a3c90 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422a3c90),
// Replacement::verified(0x1422a3d10, "rend::rend_1422a3d10", crate::ns::rend::rend_e18_b::rend_1422a3d10 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a3d10),
// Replacement::verified_throws(0x1422a3d30, "rend::rend_1422a3d30", crate::ns::rend::rend_e18_b::rend_1422a3d30 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422a3d30),
// Replacement::verified(0x1422a4800, "rend::rend_1422a4800", crate::ns::rend::rend_e18_b::rend_1422a4800 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a4800),
// Replacement::verified(0x1422a48c0, "rend::rend_1422a48c0", crate::ns::rend::rend_e18_b::rend_1422a48c0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a48c0),
// Replacement::verified(0x1422a4950, "rend::rend_1422a4950", crate::ns::rend::rend_e18_b::rend_1422a4950 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a4950),
// Replacement::verified(0x1422a5af0, "rend::rend_1422a5af0", crate::ns::rend::rend_e18_b::rend_1422a5af0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a5af0),
// Replacement::verified_throws(0x1422a5b30, "rend::rend_1422a5b30", crate::ns::rend::rend_e18_b::rend_1422a5b30 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422a5b30),
// Replacement::verified(0x1422a5bc0, "rend::rend_1422a5bc0", crate::ns::rend::rend_e18_b::rend_1422a5bc0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a5bc0),
// Replacement::verified(0x1422a5cd0, "rend::rend_1422a5cd0", crate::ns::rend::rend_e18_b::rend_1422a5cd0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a5cd0),
// Replacement::verified_throws(0x1422a5d90, "rend::rend_1422a5d90", crate::ns::rend::rend_e18_b::rend_1422a5d90 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422a5d90),
// Replacement::verified_throws(0x1422a5fc0, "rend::rend_1422a5fc0", crate::ns::rend::rend_e18_b::rend_1422a5fc0 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422a5fc0),
// Replacement::verified(0x1422a5ff0, "rend::rend_1422a5ff0", crate::ns::rend::rend_e18_b::rend_1422a5ff0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a5ff0),
// Replacement::verified_throws(0x1422a6090, "rend::rend_1422a6090", crate::ns::rend::rend_e18_b::rend_1422a6090 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422a6090),
// Replacement::verified(0x1422a6590, "rend::rend_1422a6590", crate::ns::rend::rend_e18_b::rend_1422a6590 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a6590),
// Replacement::verified(0x1422a6700, "rend::rend_1422a6700", crate::ns::rend::rend_e18_b::rend_1422a6700 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422a6700),
// Replacement::verified_throws(0x1422aa0a0, "rend::rend_1422aa0a0", crate::ns::rend::rend_e18_b::rend_1422aa0a0 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422aa0a0),
// Replacement::verified_throws(0x1422aa220, "rend::rend_1422aa220", crate::ns::rend::rend_e18_b::rend_1422aa220 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422aa220),
// Replacement::verified(0x1422aa2f0, "rend::rend_1422aa2f0", crate::ns::rend::rend_e18_b::rend_1422aa2f0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422aa2f0),
// Replacement::verified(0x1422aa3a0, "rend::rend_1422aa3a0", crate::ns::rend::rend_e18_b::rend_1422aa3a0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422aa3a0),
// Replacement::verified(0x1422ab600, "rend::rend_1422ab600", crate::ns::rend::rend_e18_b::rend_1422ab600 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422ab600),
// Replacement::verified(0x1422ab620, "rend::rend_1422ab620", crate::ns::rend::rend_e18_b::rend_1422ab620 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422ab620),
// Replacement::verified_throws(0x1422aca60, "rend::rend_1422aca60", crate::ns::rend::rend_e18_b::rend_1422aca60 as *const (), Throws::Discard, crate::ns::rend::rend_e18_b::diff_rend_1422aca60),
// Replacement::verified(0x1422aceb0, "rend::rend_1422aceb0", crate::ns::rend::rend_e18_b::rend_1422aceb0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422aceb0),
// Replacement::verified(0x1422e1190, "rend::rend_1422e1190", crate::ns::rend::rend_e18_b::rend_1422e1190 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422e1190),
// Replacement::verified(0x1422f5790, "rend::ParticleSystemResource::vfunc_13", crate::ns::rend::rend_e18_b::rend_1422f5790 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f5790),
// Replacement::verified(0x1422f5c20, "rend::rend_1422f5c20", crate::ns::rend::rend_e18_b::rend_1422f5c20 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f5c20),
// Replacement::verified(0x1422f6200, "rend::ParticleSystemResource::ClassTypeInfoImplementation::vfunc_12", crate::ns::rend::rend_e18_b::rend_1422f6200 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6200),
// Replacement::verified(0x1422f6aa0, "rend::rend_1422f6aa0", crate::ns::rend::rend_e18_b::rend_1422f6aa0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6aa0),
// Replacement::verified(0x1422f6b50, "rend::rend_1422f6b50", crate::ns::rend::rend_e18_b::rend_1422f6b50 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6b50),
// Replacement::verified(0x1422f6bf0, "rend::rend_1422f6bf0", crate::ns::rend::rend_e18_b::rend_1422f6bf0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6bf0),
// Replacement::verified(0x1422f6c40, "rend::rend_1422f6c40", crate::ns::rend::rend_e18_b::rend_1422f6c40 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6c40),
// Replacement::verified(0x1422f6cb0, "rend::rend_1422f6cb0", crate::ns::rend::rend_e18_b::rend_1422f6cb0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6cb0),
// Replacement::verified(0x1422f6d30, "rend::rend_1422f6d30", crate::ns::rend::rend_e18_b::rend_1422f6d30 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6d30),
// Replacement::verified(0x1422f6ef0, "anonymous_namespace::RenderInstancesFromGfxGraphCommand::vfunc_0", crate::ns::rend::rend_e18_b::rend_1422f6ef0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6ef0),
// Replacement::verified(0x1422f6fb0, "anonymous_namespace::RenderParticleFromGfxGraphCommand::vfunc_0", crate::ns::rend::rend_e18_b::rend_1422f6fb0 as *const (), crate::ns::rend::rend_e18_b::diff_rend_1422f6fb0),
