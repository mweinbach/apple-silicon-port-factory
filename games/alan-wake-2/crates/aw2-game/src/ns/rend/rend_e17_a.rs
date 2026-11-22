//! Decompiled `rend` functions from `e17.rs` (`0x1422014a0..0x142223500`).

use crate::exe::{self, call_va, rd, wr};
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU8, Ordering};

#[inline]
unsafe fn acquire_spinlock_u8(lock_addr: usize) {
    let mut backoff: u64 = 0x64;
    let lock = lock_addr as *const AtomicU8;
    loop {
        while rd::<u8>(lock_addr) != 0 {
            core::hint::spin_loop();
        }
        if (*lock).swap(1, Ordering::SeqCst) == 0 {
            break;
        }
        let backoff_ptr = (&mut backoff as *mut u64) as usize;
        call_va!(0x1_4241_7470, (backoff_ptr) -> ());
    }
}

/// `rend::UserRendererSettings::ClassTypeInfoImplementation::vfunc_14`
/// Original: `0x1422014a0` (153 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422014a0(_this: usize, src: usize, dst: usize) {
    for off in (0..0xb0).step_by(0x10) {
        wr::<[u8; 16]>(dst + off, rd::<[u8; 16]>(src + off));
    }
    wr::<u64>(dst + 0xb0, rd::<u64>(src + 0xb0));
    wr::<u32>(dst + 0xb8, rd::<u32>(src + 0xb8));
}

pub unsafe fn diff_rend_1422014a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0xc0];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(29).wrapping_add(7);
    }
    let mut dst_o = [0x5au8; 0xc0];
    let mut dst_n = [0x5au8; 0xc0];
    f_orig(0, src.as_ptr() as usize, dst_o.as_mut_ptr() as usize);
    f_new(0, src.as_ptr() as usize, dst_n.as_mut_ptr() as usize);
    crate::diff_eq!(dst_o, dst_n, "user_renderer_settings_copy");
    Ok(1)
}

/// `rend::rend_142201d70`
/// Original: `0x142201d70` (186 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142201d70(this: usize) {
    let idx = rd::<i32>(this + 0x8) as isize;
    let base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let entry = base.wrapping_add((idx * 16) as usize);
    let off = rd::<i32>(entry);
    if off >= 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        let size = rd::<i32>(entry + 0x8) as isize as usize;
        let buf1 = tls0.wrapping_add(0x63d0).wrapping_add(off as usize);
        let buf2 = rd::<usize>(this);
        let cmp = call_va!(0x1_4183_acda, (buf1, buf2, size) -> i32);
        if cmp == 0 {
            call_va!(0x1_4183_acc2, (buf1, 0i32, size) -> usize);
        }
        let slot = rd::<i32>(entry + 0xc);
        if slot != -1 {
            let cnt_addr = (tls0 + 0x463e0).wrapping_add(((slot as isize) * 4) as usize);
            let next = rd::<u32>(cnt_addr).wrapping_add(1);
            wr::<u32>(cnt_addr, if next == u32::MAX { 0 } else { next });
        }
    }
}

pub unsafe fn diff_rend_142201d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let table_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let saved_entry = rd::<[u8; 16]>(table_base);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0u8; 0x46420];
    let mut tls_n = vec![0u8; 0x46420];
    let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut obj = [0u8; 16];
    wr::<usize>(obj.as_mut_ptr() as usize, data.as_ptr() as usize);
    wr::<i32>(obj.as_mut_ptr() as usize + 0x8, 0);

    // Case 1: off < 0
    wr::<i32>(table_base, -1);
    f_orig(obj.as_ptr() as usize);
    f_new(obj.as_ptr() as usize);

    // Case 2: off >= 0, matching memcmp, counter wrap-around (-2 -> -1 -> 0)
    wr::<i32>(table_base, 0x10);
    wr::<i32>(table_base + 0x8, 8);
    wr::<i32>(table_base + 0xc, 1);
    tls_o[0x63e0..0x63e8].copy_from_slice(&data);
    tls_n[0x63e0..0x63e8].copy_from_slice(&data);
    wr::<i32>(tls_o.as_mut_ptr() as usize + 0x463e4, -2);
    wr::<i32>(tls_n.as_mut_ptr() as usize + 0x463e4, -2);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    f_orig(obj.as_ptr() as usize);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    f_new(obj.as_ptr() as usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<[u8; 16]>(table_base, saved_entry);

    crate::diff_eq!(&tls_o[0x63e0..0x63e8], &tls_n[0x63e0..0x63e8], "tls_buf_cleared");
    crate::diff_eq!(
        rd::<i32>(tls_o.as_ptr() as usize + 0x463e4),
        rd::<i32>(tls_n.as_ptr() as usize + 0x463e4),
        "tls_counter"
    );
    Ok(2)
}

/// `rend::rend_142201e30`
/// Original: `0x142201e30` (186 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142201e30(this: usize) {
    rend_142201d70(this);
}

pub unsafe fn diff_rend_142201e30(orig: usize, new: usize) -> Result<u32, String> {
    diff_rend_142201d70(orig, new)
}

/// `rend::rend_142201ef0`
/// Original: `0x142201ef0` (113 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142201ef0(
    this: usize,
    cstr: usize,
    a3: u32,
    a4: u64,
) -> usize {
    wr::<u64>(this, 0);
    let mut len = 0usize;
    while rd::<u8>(cstr.wrapping_add(len)) != 0 {
        len = len.wrapping_add(1);
    }
    #[repr(C, align(16))]
    struct AlignedView([u64; 2]);
    let mut view = AlignedView([cstr as u64, len as u64]);
    let view_ptr = view.0.as_mut_ptr() as usize;
    call_va!(0x1_4220_ea70, (this + 0x8, view_ptr) -> usize);
    wr::<u32>(this + 0xc, a3);
    wr::<u64>(this, a4);
    this
}

pub unsafe fn diff_rend_142201ef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let empty = b"\0";
    let mut out_o = [0xffu8; 16];
    let mut out_n = [0xffu8; 16];
    let ro = f_orig(
        out_o.as_mut_ptr() as usize,
        empty.as_ptr() as usize,
        0x1234_5678,
        0xdead_beef_cafe_babe,
    );
    let rn = f_new(
        out_n.as_mut_ptr() as usize,
        empty.as_ptr() as usize,
        0x1234_5678,
        0xdead_beef_cafe_babe,
    );
    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(out_o, out_n, "out_struct");
    Ok(1)
}

/// `rend::rend_142209180`
/// Original: `0x142209180` (55 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142209180(this: usize) -> u8 {
    if rd::<u8>(this + 0x175) != 0 {
        return 1;
    }
    for i in 0..8usize {
        if rd::<u8>(this + 0x158 + i) == 1 {
            return 1;
        }
    }
    0
}

pub unsafe fn diff_rend_142209180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf = [0u8; 0x180];
    let ptr = buf.as_mut_ptr() as usize;
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "all_zero");

    wr::<u8>(ptr + 0x175, 1);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "flag_175_set");
    wr::<u8>(ptr + 0x175, 0);

    wr::<u8>(ptr + 0x158 + 5, 2);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "slot_eq_2");

    wr::<u8>(ptr + 0x158 + 5, 1);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "slot_eq_1");
    Ok(4)
}

/// `rend::TextureResource::vfunc_7`
/// Original: `0x1422091c0` (201 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_1422091c0(this: usize) {
    let mgr_addr = exe::ptr::<usize>(0x1_4397_fea8) as usize;
    let mgr = rd::<usize>(mgr_addr);
    if mgr != 0 {
        let lock = mgr + 0x298;
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        wr::<u32>(mgr + 0x2a0, rd::<u32>(mgr + 0x2a0).wrapping_add(1));
        let sched = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        call_va!(0x1_4248_5780, (sched, mgr + 0x28) -> ());
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
    let prev_1a8 = rd::<usize>(this + 0x1a8);
    call_va!(0x1_4220_92d0, (this) -> ());
    call_va!(0x1_4220_8830, (this) -> ());
    (*((this + 0x80) as *const AtomicU32)).swap(1, Ordering::SeqCst);
    call_va!(0x1_4220_8f20, (this) -> ());
    if prev_1a8 != 0 {
        call_va!(0x1_4220_8970, (this) -> ());
    }
    let mgr2 = rd::<usize>(mgr_addr);
    if mgr2 != 0 {
        let lock = mgr2 + 0x298;
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        wr::<u32>(mgr2 + 0x2a0, rd::<u32>(mgr2 + 0x2a0).wrapping_sub(1));
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

pub unsafe fn diff_rend_1422091c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe fn rec(tag: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(tag);
        }
    }
    unsafe extern "win64" fn m_5780(sched: usize, arg2: usize) {
        let cnt_in_flight = rd::<u32>(arg2 - 0x28 + 0x2a0) as u64;
        unsafe { rec(0x10 ^ (sched as u64) ^ (cnt_in_flight << 16)) };
    }
    unsafe extern "win64" fn m_92d0(this: usize) {
        wr::<u32>(this + 0x10, 0x92d0);
        unsafe { rec(0x20) };
    }
    unsafe extern "win64" fn m_8830(this: usize) {
        wr::<u32>(this + 0x14, 0x8830);
        unsafe { rec(0x30) };
    }
    unsafe extern "win64" fn m_8f20(this: usize) {
        let st = rd::<u32>(this + 0x80) as u64;
        unsafe { rec(0x40 ^ (st << 8)) };
    }
    unsafe extern "win64" fn m_8970(this: usize) {
        wr::<u32>(this + 0x18, 0x8970);
        unsafe { rec(0x50) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_5780, m_5780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_92d0, m_92d0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4220_8830, m_8830 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4220_8f20, m_8f20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4220_8970, m_8970 as *const ())?;

    let mgr_g = exe::ptr::<usize>(0x1_4397_fea8) as usize;
    let sched_g = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_mgr = rd::<usize>(mgr_g);
    let saved_sched = rd::<usize>(sched_g);

    for prev_1a8 in [0usize, 0x1234usize] {
        let mut mgr_o = [0u8; 0x2b0];
        let mut mgr_n = [0u8; 0x2b0];
        wr::<u32>(mgr_o.as_mut_ptr() as usize + 0x2a0, 5);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + 0x2a0, 5);

        let mut obj_o = [0u8; 0x1c0];
        wr::<usize>(obj_o.as_mut_ptr() as usize + 0x1a8, prev_1a8);
        let mut obj_n = obj_o;

        wr::<usize>(sched_g, 0x7777);
        wr::<usize>(mgr_g, mgr_o.as_mut_ptr() as usize);
        TRACE = 1;
        f_orig(obj_o.as_mut_ptr() as usize);
        let tr_o = TRACE;

        wr::<usize>(mgr_g, mgr_n.as_mut_ptr() as usize);
        TRACE = 1;
        f_new(obj_n.as_mut_ptr() as usize);
        let tr_n = TRACE;

        wr::<usize>(mgr_g, saved_mgr);
        wr::<usize>(sched_g, saved_sched);

        crate::diff_eq!(tr_o, tr_n, "trace prev_1a8={prev_1a8}");
        crate::diff_eq!(mgr_o, mgr_n, "mgr prev_1a8={prev_1a8}");
        crate::diff_eq!(obj_o, obj_n, "obj prev_1a8={prev_1a8}");
    }
    Ok(2)
}

/// `rend::rend_1422092d0`
/// Original: `0x1422092d0` (197 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_1422092d0(this: usize) {
    if rd::<u8>(this + 0xb8) != 0 {
        let p_1d0 = rd::<usize>(this + 0x1d0);
        if p_1d0 != 0 {
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
            call_va!(0x1_421f_28a0, (mgr, p_1d0) -> ());
            wr::<usize>(this + 0x1d0, 0);
        }
    }
    wr::<u16>(this + 0xbc, 0xffff);
    wr::<usize>(this + 0x1a0, 0);

    let mut local_c0 = [0u8; 0xa0];
    let mut local_20 = [0u8; 0xa0];
    let c0_ptr = local_c0.as_mut_ptr() as usize;
    let l20_ptr = local_20.as_mut_ptr() as usize;

    call_va!(0x1_4207_f3a0, (c0_ptr, this + 0x108) -> usize);
    let mgr_f58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_4207_f3a0, (l20_ptr, c0_ptr) -> usize);
    call_va!(0x1_412e_30b0, (mgr_f58, l20_ptr) -> ());
    call_va!(0x1_4207_f310, (c0_ptr) -> ());

    if rd::<u8>(this + 0xb9) != 0 {
        let cnt = exe::ptr::<AtomicI32>(0x1_43a2_b148);
        (*cnt).fetch_sub(1, Ordering::SeqCst);
        wr::<u8>(this + 0xb9, 0);
    }
    call_va!(0x1_412d_fcb0, (this + 0x1a8) -> ());
}

pub unsafe fn diff_rend_1422092d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_28a0(mgr: usize, p: usize) {
        unsafe { rec(0x100 ^ (mgr as u64) ^ (p as u64)) };
    }
    unsafe extern "win64" fn m_f3a0(dst: usize, src: usize) -> usize {
        wr::<u64>(dst, rd::<u64>(src));
        dst
    }
    unsafe extern "win64" fn m_30b0(mgr: usize, l20: usize) {
        unsafe { rec(0x200 ^ (mgr as u64) ^ rd::<u64>(l20)) };
    }
    unsafe extern "win64" fn m_f310(c0: usize) {
        unsafe { rec(0x300 ^ rd::<u64>(c0)) };
    }
    unsafe extern "win64" fn m_fcb0(p_1a8: usize) {
        wr::<u64>(p_1a8, 0);
        unsafe { rec(0x400) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_421f_28a0, m_28a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_f3a0, m_f3a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412e_30b0, m_30b0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_f310, m_f310 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_412d_fcb0, m_fcb0 as *const ())?;

    let g_e88 = exe::ptr::<usize>(0x1_4397_fe88) as usize;
    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let g_cnt = exe::ptr::<i32>(0x1_43a2_b148) as usize;
    let saved_e88 = rd::<usize>(g_e88);
    let saved_f58 = rd::<usize>(g_f58);
    let saved_cnt = rd::<i32>(g_cnt);

    wr::<usize>(g_e88, 0xaaaa);
    wr::<usize>(g_f58, 0xbbbb);

    let mut obj_o = [0u8; 0x1e0];
    let bo = obj_o.as_mut_ptr() as usize;
    wr::<u8>(bo + 0xb8, 1);
    wr::<u8>(bo + 0xb9, 1);
    wr::<u64>(bo + 0x108, 0x1234_5678);
    wr::<u64>(bo + 0x1a0, 0x9999);
    wr::<u64>(bo + 0x1a8, 0x8888);
    wr::<usize>(bo + 0x1d0, 0x5555);
    let mut obj_n = obj_o;

    wr::<i32>(g_cnt, 10);
    TRACE = 1;
    f_orig(obj_o.as_mut_ptr() as usize);
    let tr_o = TRACE;
    let cnt_o = rd::<i32>(g_cnt);

    wr::<i32>(g_cnt, 10);
    TRACE = 1;
    f_new(obj_n.as_mut_ptr() as usize);
    let tr_n = TRACE;
    let cnt_n = rd::<i32>(g_cnt);

    wr::<usize>(g_e88, saved_e88);
    wr::<usize>(g_f58, saved_f58);
    wr::<i32>(g_cnt, saved_cnt);

    crate::diff_eq!(tr_o, tr_n, "1422092d0 trace");
    crate::diff_eq!(cnt_o, cnt_n, "1422092d0 cnt");
    crate::diff_eq!(obj_o, obj_n, "1422092d0 obj");
    Ok(1)
}

/// `rend::TextureResource::vfunc_13`
/// Original: `0x1422093e0` (242 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_1422093e0(this: usize) {
    let c4 = rd::<u32>(this + 0xc4);
    let mgr_ea8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fea8) as usize);
    call_va!(0x1_4220_7e60, (mgr_ea8, c4) -> ());
    wr::<u32>(this + 0xc4, 0xffff_ffff);

    let p_1d0 = rd::<usize>(this + 0x1d0);
    if p_1d0 != 0 {
        let mgr_e88 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
        call_va!(0x1_421f_28a0, (mgr_e88, p_1d0) -> ());
        wr::<usize>(this + 0x1d0, 0);
    }
    wr::<usize>(this + 0x1a0, 0);

    let mut local_c0 = [0u8; 0xa0];
    let mut local_20 = [0u8; 0xa0];
    let c0_ptr = local_c0.as_mut_ptr() as usize;
    let l20_ptr = local_20.as_mut_ptr() as usize;

    call_va!(0x1_4207_f3a0, (c0_ptr, this + 0x108) -> usize);
    let mgr_f58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_4207_f3a0, (l20_ptr, c0_ptr) -> usize);
    call_va!(0x1_412e_2f50, (mgr_f58, l20_ptr) -> ());
    call_va!(0x1_4207_f310, (c0_ptr) -> ());
    call_va!(0x1_401d_e830, (this + 0x1b8) -> ());

    if rd::<u8>(this + 0xb9) != 0 {
        let cnt = exe::ptr::<AtomicI32>(0x1_43a2_b148);
        (*cnt).fetch_sub(1, Ordering::SeqCst);
        wr::<u8>(this + 0xb9, 0);
    }
    (*((this + 0xd0) as *const AtomicU32)).swap(0, Ordering::SeqCst);
    wr::<u16>(this + 0xbc, 0xffff);
    call_va!(0x1_4242_87f0, (this) -> ());
}

pub unsafe fn diff_rend_1422093e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_7e60(mgr: usize, id: u32) {
        unsafe { rec(0x10 ^ (mgr as u64) ^ (id as u64)) };
    }
    unsafe extern "win64" fn m_28a0(mgr: usize, p: usize) {
        unsafe { rec(0x20 ^ (mgr as u64) ^ (p as u64)) };
    }
    unsafe extern "win64" fn m_f3a0(dst: usize, src: usize) -> usize {
        wr::<u64>(dst, rd::<u64>(src));
        dst
    }
    unsafe extern "win64" fn m_2f50(mgr: usize, l20: usize) {
        unsafe { rec(0x30 ^ (mgr as u64) ^ rd::<u64>(l20)) };
    }
    unsafe extern "win64" fn m_f310(c0: usize) {
        unsafe { rec(0x40 ^ rd::<u64>(c0)) };
    }
    unsafe extern "win64" fn m_e830(v: usize) {
        wr::<u64>(v, 0);
        unsafe { rec(0x50) };
    }
    unsafe extern "win64" fn m_87f0(this: usize) {
        wr::<u32>(this + 0x10, 0x87f0);
        unsafe { rec(0x60) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4220_7e60, m_7e60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_421f_28a0, m_28a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_f3a0, m_f3a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412e_2f50, m_2f50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4207_f310, m_f310 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401d_e830, m_e830 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4242_87f0, m_87f0 as *const ())?;

    let g_ea8 = exe::ptr::<usize>(0x1_4397_fea8) as usize;
    let g_e88 = exe::ptr::<usize>(0x1_4397_fe88) as usize;
    let g_f58 = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let g_cnt = exe::ptr::<i32>(0x1_43a2_b148) as usize;
    let saved_ea8 = rd::<usize>(g_ea8);
    let saved_e88 = rd::<usize>(g_e88);
    let saved_f58 = rd::<usize>(g_f58);
    let saved_cnt = rd::<i32>(g_cnt);

    wr::<usize>(g_ea8, 0x1111);
    wr::<usize>(g_e88, 0x2222);
    wr::<usize>(g_f58, 0x3333);

    let mut obj_o = [0u8; 0x1e0];
    let bo = obj_o.as_mut_ptr() as usize;
    wr::<u8>(bo + 0xb9, 1);
    wr::<u32>(bo + 0xc4, 77);
    wr::<u32>(bo + 0xd0, 9);
    wr::<u64>(bo + 0x108, 0xdead_beef);
    wr::<u64>(bo + 0x1a0, 0x4444);
    wr::<u64>(bo + 0x1b8, 0x5555);
    wr::<usize>(bo + 0x1d0, 0x6666);
    let mut obj_n = obj_o;

    wr::<i32>(g_cnt, 8);
    TRACE = 1;
    f_orig(obj_o.as_mut_ptr() as usize);
    let tr_o = TRACE;
    let cnt_o = rd::<i32>(g_cnt);

    wr::<i32>(g_cnt, 8);
    TRACE = 1;
    f_new(obj_n.as_mut_ptr() as usize);
    let tr_n = TRACE;
    let cnt_n = rd::<i32>(g_cnt);

    wr::<usize>(g_ea8, saved_ea8);
    wr::<usize>(g_e88, saved_e88);
    wr::<usize>(g_f58, saved_f58);
    wr::<i32>(g_cnt, saved_cnt);

    crate::diff_eq!(tr_o, tr_n, "1422093e0 trace");
    crate::diff_eq!(cnt_o, cnt_n, "1422093e0 cnt");
    crate::diff_eq!(obj_o, obj_n, "1422093e0 obj");
    Ok(1)
}

/// `rend::TextureResource::vfunc_5`
/// Original: `0x1422094e0` (67 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422094e0(this: usize) -> u8 {
    if rd::<u8>(this + 0x175) != 0 {
        return 0;
    }
    for i in 0..8usize {
        if rd::<u8>(this + 0x158 + i) == 1 {
            return 0;
        }
    }
    u8::from(rd::<u32>(this + 0x80) == 5)
}

pub unsafe fn diff_rend_1422094e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf = [0u8; 0x180];
    let ptr = buf.as_mut_ptr() as usize;
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "state_0");

    wr::<u32>(ptr + 0x80, 5);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "state_5_ready");

    wr::<u8>(ptr + 0x158 + 3, 1);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "state_5_slot_busy");
    wr::<u8>(ptr + 0x158 + 3, 0);

    wr::<u8>(ptr + 0x175, 1);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "state_5_flag_175");
    Ok(4)
}

/// `rend::TextureResource::vfunc_12`
/// Original: `0x142209530` (232 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142209530(
    this: usize,
    a2: u32,
    a3: u64,
    a4: u64,
    a5: u64,
) {
    wr::<u64>(this + 0x28, a5);
    wr::<u32>(this + 0x20, a2);
    wr::<u64>(this + 0x8, a3);
    wr::<u64>(this + 0x30, a4);
    (*((this + 0x80) as *const AtomicU32)).swap(1, Ordering::SeqCst);

    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fea8) as usize);
    let lock_2a8 = mgr + 0x2a8;
    exe::call_iat!(0x1_4264_55a8, (lock_2a8) -> ());

    let p_2c0 = rd::<usize>(mgr + 0x2c0);
    let len_2c8 = rd::<u32>(mgr + 0x2c8) as usize;
    if p_2c0 != p_2c0.wrapping_add(len_2c8.wrapping_mul(4)) {
        let lock_2d8 = mgr + 0x2d8;
        let acquired = exe::call_iat!(0x1_4264_5788, (lock_2d8) -> u8);
        if acquired != 0 {
            call_va!(0x1_4220_7930, (mgr) -> ());
            exe::call_iat!(0x1_4264_5598, (lock_2d8) -> ());
        }
    }

    let p_2b0 = rd::<usize>(mgr + 0x2b0);
    let len_2b8 = rd::<u32>(mgr + 0x2b8);
    let end_2b0 = p_2b0.wrapping_add((len_2b8 as usize).wrapping_mul(4));
    let id = if p_2b0 != end_2b0 {
        let val = rd::<u32>(end_2b0 - 4);
        wr::<u32>(mgr + 0x2b8, len_2b8.wrapping_sub(1));
        val
    } else {
        let val = rd::<u32>(mgr + 0x2d0);
        wr::<u32>(mgr + 0x2d0, val.wrapping_add(1));
        val
    };

    exe::call_iat!(0x1_4264_5598, (lock_2a8) -> ());
    wr::<u32>(this + 0xc4, id);
    call_va!(0x1_4220_8830, (this) -> ());
}

pub unsafe fn diff_rend_142209530(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u64, u64, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut CALLED_7930: u32 = 0;
    static mut CALLED_8830: u32 = 0;
    unsafe extern "win64" fn m_7930(_mgr: usize) {
        unsafe {
            CALLED_7930 += 1;
        }
    }
    unsafe extern "win64" fn m_8830(_this: usize) {
        unsafe {
            CALLED_8830 += 1;
        }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4220_7930, m_7930 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_8830, m_8830 as *const ())?;

    let g_ea8 = exe::ptr::<usize>(0x1_4397_fea8) as usize;
    let saved_ea8 = rd::<usize>(g_ea8);

    for use_freelist in [true, false] {
        let mut free_o = [42u32, 99u32];
        let mut free_n = free_o;
        let pending = [1u32];
        let mut mgr_o = [0u8; 0x2f0];
        let mut mgr_n = [0u8; 0x2f0];
        let mo = mgr_o.as_mut_ptr() as usize;
        let mn = mgr_n.as_mut_ptr() as usize;
        wr::<usize>(mo + 0x2b0, free_o.as_mut_ptr() as usize);
        wr::<u32>(mo + 0x2b8, if use_freelist { 2 } else { 0 });
        wr::<usize>(mo + 0x2c0, pending.as_ptr() as usize);
        wr::<u32>(mo + 0x2c8, if use_freelist { 1 } else { 0 });
        wr::<u32>(mo + 0x2d0, 500);

        wr::<usize>(mn + 0x2b0, free_n.as_mut_ptr() as usize);
        wr::<u32>(mn + 0x2b8, if use_freelist { 2 } else { 0 });
        wr::<usize>(mn + 0x2c0, pending.as_ptr() as usize);
        wr::<u32>(mn + 0x2c8, if use_freelist { 1 } else { 0 });
        wr::<u32>(mn + 0x2d0, 500);

        let mut obj_o = [0u8; 0xe0];
        let mut obj_n = [0u8; 0xe0];

        wr::<usize>(g_ea8, mo);
        CALLED_7930 = 0;
        CALLED_8830 = 0;
        f_orig(obj_o.as_mut_ptr() as usize, 11, 22, 33, 44);
        let c79_o = CALLED_7930;
        let c88_o = CALLED_8830;

        wr::<usize>(g_ea8, mn);
        CALLED_7930 = 0;
        CALLED_8830 = 0;
        f_new(obj_n.as_mut_ptr() as usize, 11, 22, 33, 44);
        let c79_n = CALLED_7930;
        let c88_n = CALLED_8830;

        wr::<usize>(g_ea8, saved_ea8);

        crate::diff_eq!(c79_o, c79_n, "142209530 c7930 freelist={use_freelist}");
        crate::diff_eq!(c88_o, c88_n, "142209530 c8830 freelist={use_freelist}");
        crate::diff_eq!(obj_o, obj_n, "142209530 obj freelist={use_freelist}");
        crate::diff_eq!(
            &mgr_o[0x2b8..0x2d8],
            &mgr_n[0x2b8..0x2d8],
            "142209530 mgr counts freelist={use_freelist}"
        );
    }
    Ok(2)
}

/// `rend::rend_142209650`
/// Original: `0x142209650` (283 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142209650(this: usize, target_id: u32) {
    acquire_spinlock_u8(this + 0x1b0);

    let base = rd::<usize>(this + 0x1b8);
    let count = rd::<u32>(this + 0x1c0) as usize;
    let end = base.wrapping_add(count.wrapping_mul(0x18));

    let mut write_ptr = base;
    while write_ptr != end {
        if rd::<u32>(write_ptr + 0x10) == target_id {
            break;
        }
        write_ptr = write_ptr.wrapping_add(0x18);
    }

    if write_ptr != end {
        let mut read_ptr = write_ptr.wrapping_add(0x18);
        while read_ptr != end {
            if rd::<u32>(read_ptr + 0x10) != target_id {
                let w1 = rd::<u64>(read_ptr + 0x8);
                wr::<u64>(write_ptr + 0x8, w1);
                let w0 = rd::<u64>(read_ptr);
                wr::<u64>(write_ptr, w0);
                let id = rd::<u32>(read_ptr + 0x10);
                wr::<u32>(write_ptr + 0x10, id);
                write_ptr = write_ptr.wrapping_add(0x18);
            }
            read_ptr = read_ptr.wrapping_add(0x18);
        }
    }

    let cur_base = rd::<usize>(this + 0x1b8);
    let cur_count = rd::<u32>(this + 0x1c0) as usize;
    let cur_end = cur_base.wrapping_add(cur_count.wrapping_mul(0x18));
    if write_ptr != cur_end {
        if write_ptr == cur_base {
            wr::<u32>(this + 0x1c0, 0);
            crate::simd_ops::wu8((this + 0x1b0) as u64, 0); // spinlock release
            return;
        }
        let removed = ((cur_end - write_ptr) / 0x18) as u32;
        wr::<u32>(this + 0x1c0, (cur_count as u32).wrapping_sub(removed));
    }
    crate::simd_ops::wu8((this + 0x1b0) as u64, 0); // spinlock release
}

pub unsafe fn diff_rend_142209650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut arr_o = [0u64; 9];
    arr_o[0] = 0x11;
    arr_o[1] = 0x22;
    arr_o[2] = 7;
    arr_o[3] = 0x33;
    arr_o[4] = 0x44;
    arr_o[5] = 8;
    arr_o[6] = 0x55;
    arr_o[7] = 0x66;
    arr_o[8] = 7;
    let mut arr_n = arr_o;

    let mut obj_o = [0u8; 0x1d0];
    let mut obj_n = [0u8; 0x1d0];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x1b8, arr_o.as_mut_ptr() as usize);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0x1c0, 3);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x1b8, arr_n.as_mut_ptr() as usize);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0x1c0, 3);

    f_orig(obj_o.as_mut_ptr() as usize, 7);
    f_new(obj_n.as_mut_ptr() as usize, 7);
    crate::diff_eq!(
        rd::<u32>(obj_o.as_ptr() as usize + 0x1c0),
        rd::<u32>(obj_n.as_ptr() as usize + 0x1c0),
        "remaining_count"
    );
    crate::diff_eq!(&arr_o[0..3], &arr_n[0..3], "remaining_elem");

    f_orig(obj_o.as_mut_ptr() as usize, 8);
    f_new(obj_n.as_mut_ptr() as usize, 8);
    crate::diff_eq!(
        rd::<u32>(obj_o.as_ptr() as usize + 0x1c0),
        rd::<u32>(obj_n.as_ptr() as usize + 0x1c0),
        "empty_count"
    );
    Ok(2)
}

/// `rend::rend_142209780`
/// Original: `0x142209780` (222 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142209780(this: usize, cb: usize) -> u32 {
    let w1 = rd::<u64>(cb + 0x8);
    let w0 = rd::<u64>(cb);
    let id_addr = exe::ptr::<u32>(0x1_4397_fe54) as usize;
    let id = rd::<u32>(id_addr);
    wr::<u32>(id_addr, id.wrapping_add(1));

    acquire_spinlock_u8(this + 0x1b0);

    let vec = this + 0x1b8;
    let len = rd::<u32>(vec + 0x8) as u64;
    let cap = rd::<u32>(vec + 0xc) as u64;
    if len.wrapping_mul(0x18) == cap.wrapping_mul(0x18) {
        let min_cap = len.wrapping_add(1);
        let grow = cap.wrapping_add(cap >> 1);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        call_va!(0x1_412e_3940, (vec, new_cap) -> ());
    }

    let cur_len = rd::<u32>(vec + 0x8) as usize;
    let base = rd::<usize>(vec);
    let slot = base + cur_len * 0x18;
    wr::<u64>(slot + 0x8, w1);
    wr::<u64>(slot, w0);
    wr::<u32>(slot + 0x10, id);
    wr::<u32>(vec + 0x8, (cur_len as u32).wrapping_add(1));
    crate::simd_ops::wu8((this + 0x1b0) as u64, 0); // spinlock release
    id
}

pub unsafe fn diff_rend_142209780(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let id_addr = exe::ptr::<u32>(0x1_4397_fe54) as usize;
    let saved_id = rd::<u32>(id_addr);

    let mut arr_o = [0u64; 6];
    let mut arr_n = [0u64; 6];
    let mut obj_o = [0u8; 0x1d0];
    let mut obj_n = [0u8; 0x1d0];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x1b8, arr_o.as_mut_ptr() as usize);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0x1c0, 0);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0x1c4, 2);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x1b8, arr_n.as_mut_ptr() as usize);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0x1c0, 0);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0x1c4, 2);

    let cb = [0x1111_2222_3333_4444u64, 0x5555_6666_7777_8888u64];

    wr::<u32>(id_addr, 42);
    let ro = f_orig(obj_o.as_mut_ptr() as usize, cb.as_ptr() as usize);
    let id_after_o = rd::<u32>(id_addr);

    wr::<u32>(id_addr, 42);
    let rn = f_new(obj_n.as_mut_ptr() as usize, cb.as_ptr() as usize);
    let id_after_n = rd::<u32>(id_addr);

    wr::<u32>(id_addr, saved_id);

    crate::diff_eq!(ro, rn, "ret_id");
    crate::diff_eq!(id_after_o, id_after_n, "next_id");
    crate::diff_eq!(arr_o[0..2], arr_n[0..2], "pushed_cb_ptrs");
    crate::diff_eq!(arr_o[2] as u32, arr_n[2] as u32, "pushed_cb_id");
    crate::diff_eq!(
        rd::<u32>(obj_o.as_ptr() as usize + 0x1c0),
        rd::<u32>(obj_n.as_ptr() as usize + 0x1c0),
        "new_len"
    );
    Ok(1)
}

/// `rend::rend_142209880` (`rend::TextureResource::~TextureResource`)
/// Original: `0x142209880` (80 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_142209880(this: usize) {
    let vtbl = exe::ptr::<u8>(0x1_433e_ffe0) as usize;
    wr::<usize>(this, vtbl);
    call_va!(0x1_401d_e830, (this + 0x1b8) -> ());
    call_va!(0x1_412d_fd10, (this + 0x1a8) -> ());
    call_va!(0x1_4207_f310, (this + 0x108) -> ());
    wr::<u64>(this + 0xf8, 0);
    call_va!(0x1_4242_8ce0, (this) -> ());
}

pub unsafe fn diff_rend_142209880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u8; 0x1d8];
    let mut buf_n = [0u8; 0x1d8];
    rend_1422098e0(buf_o.as_mut_ptr() as usize);
    rend_1422098e0(buf_n.as_mut_ptr() as usize);
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(&buf_o[..0x40], &buf_n[..0x40], "texture_resource_dtor_head");
    crate::diff_eq!(&buf_o[0x80..], &buf_n[0x80..], "texture_resource_dtor_tail");
    Ok(1)
}

/// `rend::rend_1422098e0` (`rend::TextureResource::TextureResource`)
/// Original: `0x1422098e0` (167 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422098e0(this: usize) -> usize {
    call_va!(0x1_4242_8d70, (this) -> usize);
    wr::<u32>(this + 0xb8, 0);
    let vtbl = exe::ptr::<u8>(0x1_433e_ffe0) as usize;
    wr::<u16>(this + 0xbc, 0xffff);
    wr::<usize>(this, vtbl);
    wr::<u64>(this + 0xd0, 0);
    wr::<u64>(this + 0xf8, 0);
    wr::<u32>(this + 0xc0, 0);
    wr::<u32>(this + 0xc4, 0xffff_ffff);
    wr::<u64>(this + 0xc8, 0);
    wr::<u32>(this + 0x100, 0);
    call_va!(0x1_4207_f660, (this + 0x108) -> usize);
    wr::<u64>(this + 0x1a8, 0);
    wr::<u8>(this + 0x1b0, 0);
    wr::<u64>(this + 0x1a0, 0);
    wr::<u64>(this + 0x1b8, 0);
    wr::<u64>(this + 0x1c0, 0);
    wr::<u8>(this + 0x1c8, 0);
    wr::<u64>(this + 0x1d0, 0);
    this
}

pub unsafe fn diff_rend_1422098e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xa5u8; 0x1d8];
    let mut buf_n = [0xa5u8; 0x1d8];
    let ro = f_orig(buf_o.as_mut_ptr() as usize);
    let rn = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, buf_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(&buf_o[..0x40], &buf_n[..0x40], "texture_resource_ctor_head");
    crate::diff_eq!(&buf_o[0x80..], &buf_n[0x80..], "texture_resource_ctor_tail");
    Ok(1)
}

/// `rend::TextureResource::ClassTypeInfoImplementation::vfunc_13`
/// Original: `0x142209ed0` (80 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_142209ed0(_this: usize, obj: usize) {
    rend_142209880(obj);
}

pub unsafe fn diff_rend_142209ed0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u8; 0x1d8];
    let mut buf_n = [0u8; 0x1d8];
    rend_1422098e0(buf_o.as_mut_ptr() as usize);
    rend_1422098e0(buf_n.as_mut_ptr() as usize);
    f_orig(0, buf_o.as_mut_ptr() as usize);
    f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(&buf_o[..0x40], &buf_n[..0x40], "texture_resource_vfunc_13_head");
    crate::diff_eq!(&buf_o[0x80..], &buf_n[0x80..], "texture_resource_vfunc_13_tail");
    Ok(1)
}

/// `rend::TextureResource::ClassTypeInfoImplementation::vfunc_11`
/// Original: `0x142209f40` (176 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_142209f40(_this: usize, ptr: usize) {
    if ptr == 0 {
        return;
    }
    let mut i = 0usize;
    if rd::<u32>(ptr - 0x8) > 0 {
        let vtbl = exe::ptr::<u8>(0x1_433e_ffe0) as usize;
        loop {
            let stride = rd::<usize>(ptr - 0x10);
            let elem = ptr.wrapping_add(i.wrapping_mul(stride));
            wr::<usize>(elem, vtbl);
            call_va!(0x1_401d_e830, (elem + 0x1b8) -> ());
            call_va!(0x1_412d_fd10, (elem + 0x1a8) -> ());
            call_va!(0x1_4207_f310, (elem + 0x108) -> ());
            wr::<u64>(elem + 0xf8, 0);
            call_va!(0x1_4242_8ce0, (elem) -> ());
            i = i.wrapping_add(1);
            let count = rd::<u32>(ptr - 0x8) as usize;
            if i >= count {
                break;
            }
        }
    }
    call_va!(0x1_4244_2c80, (ptr) -> ());
}

pub unsafe fn diff_rend_142209f40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut FREED_PTR: usize = 0;
    unsafe extern "win64" fn mock_free_array(p: usize) {
        unsafe {
            FREED_PTR = p;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4244_2c80, mock_free_array as *const ())?;

    // Null pointer case
    f_orig(0, 0);
    f_new(0, 0);

    // 1-element array case
    let mut raw_o = [0u8; 0x10 + 0x1d8];
    let mut raw_n = [0u8; 0x10 + 0x1d8];
    let base_o = raw_o.as_mut_ptr() as usize + 0x10;
    let base_n = raw_n.as_mut_ptr() as usize + 0x10;
    wr::<usize>(base_o - 0x10, 0x1d8);
    wr::<u32>(base_o - 0x8, 1);
    wr::<usize>(base_n - 0x10, 0x1d8);
    wr::<u32>(base_n - 0x8, 1);
    rend_1422098e0(base_o);
    rend_1422098e0(base_n);

    FREED_PTR = 0;
    f_orig(0, base_o);
    let freed_o = FREED_PTR;

    FREED_PTR = 0;
    f_new(0, base_n);
    let freed_n = FREED_PTR;

    crate::diff_eq!(freed_o, base_o, "freed_ptr_orig");
    crate::diff_eq!(freed_n, base_n, "freed_ptr_new");
    crate::diff_eq!(&raw_o[0x10..0x50], &raw_n[0x10..0x50], "destroyed_elem_head");
    crate::diff_eq!(&raw_o[0x90..], &raw_n[0x90..], "destroyed_elem_tail");
    Ok(2)
}

/// `rend::TextureResource::ClassTypeInfoImplementation::vfunc_10`
/// Original: `0x14220a000` (93 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14220a000(_this: usize, ptr: usize) {
    if ptr != 0 {
        let vtbl = exe::ptr::<u8>(0x1_433e_ffe0) as usize;
        wr::<usize>(ptr, vtbl);
        call_va!(0x1_401d_e830, (ptr + 0x1b8) -> ());
        call_va!(0x1_412d_fd10, (ptr + 0x1a8) -> ());
        call_va!(0x1_4207_f310, (ptr + 0x108) -> ());
        wr::<u64>(ptr + 0xf8, 0);
        call_va!(0x1_4242_8ce0, (ptr) -> ());
        call_va!(0x1_4244_2de0, (ptr) -> ());
    }
}

pub unsafe fn diff_rend_14220a000(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut FREED_PTR: usize = 0;
    unsafe extern "win64" fn mock_free(p: usize) {
        unsafe {
            FREED_PTR = p;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4244_2de0, mock_free as *const ())?;

    f_orig(0, 0);
    f_new(0, 0);

    let mut buf_o = [0u8; 0x1d8];
    let mut buf_n = [0u8; 0x1d8];
    rend_1422098e0(buf_o.as_mut_ptr() as usize);
    rend_1422098e0(buf_n.as_mut_ptr() as usize);

    FREED_PTR = 0;
    f_orig(0, buf_o.as_mut_ptr() as usize);
    let freed_o = FREED_PTR;

    FREED_PTR = 0;
    f_new(0, buf_n.as_mut_ptr() as usize);
    let freed_n = FREED_PTR;

    crate::diff_eq!(freed_o, buf_o.as_mut_ptr() as usize, "freed_orig");
    crate::diff_eq!(freed_n, buf_n.as_mut_ptr() as usize, "freed_new");
    crate::diff_eq!(&buf_o[..0x40], &buf_n[..0x40], "destroyed_obj_head");
    crate::diff_eq!(&buf_o[0x80..], &buf_n[0x80..], "destroyed_obj_tail");
    Ok(2)
}

/// `rend::TextureMetadata::vfunc_17`
/// Original: `0x14220a210` (89 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220a210(
    this: usize,
    vec_obj: usize,
    _r8: u64,
    r9: u64,
) {
    let type_name = rd::<u64>(exe::ptr::<u64>(0x1_4388_1950) as usize);
    call_va!(
        0x1_4243_b0b0,
        (vec_obj, this as u64, type_name, r9, 8u32, 8u32) -> ()
    );
    let len = rd::<u32>(vec_obj + 0x8);
    let base = rd::<usize>(vec_obj);
    let val = rd::<u32>(base + (len as usize) * 0x30 - 0x8);
    wr::<u32>(vec_obj + 0x8, len.wrapping_sub(1));
    wr::<u32>(vec_obj + 0x328, val);
}

pub unsafe fn diff_rend_14220a210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xaa_u8; 4 * 0x30];
    let mut buf_n = [0xaa_u8; 4 * 0x30];
    let mut obj_o = [0u8; 0x340];
    let mut obj_n = [0u8; 0x340];
    wr::<usize>(obj_o.as_mut_ptr() as usize, buf_o.as_mut_ptr() as usize);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0x8, 1);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0xc, 4);
    wr::<usize>(obj_n.as_mut_ptr() as usize, buf_n.as_mut_ptr() as usize);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0x8, 1);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0xc, 4);

    f_orig(0x1234_5678, obj_o.as_mut_ptr() as usize, 0, 0);
    f_new(0x1234_5678, obj_n.as_mut_ptr() as usize, 0, 0);
    crate::diff_eq!(buf_o, buf_n, "entry_buf");
    crate::diff_eq!(&obj_o[0x8..], &obj_n[0x8..], "vec_obj");
    Ok(1)
}

/// `rend::TextureMetadata::ClassTypeInfoImplementation::vfunc_35`
/// Original: `0x14220a650` (84 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220a650(
    _this: usize,
    a2: u64,
    vec_obj: usize,
    r9: u64,
) {
    let type_name = rd::<u64>(exe::ptr::<u64>(0x1_4388_1950) as usize);
    call_va!(
        0x1_4243_b0b0,
        (vec_obj, a2, type_name, r9, 8u32, 8u32) -> ()
    );
    let len = rd::<u32>(vec_obj + 0x8);
    let base = rd::<usize>(vec_obj);
    let val = rd::<u32>(base + (len as usize) * 0x30 - 0x8);
    wr::<u32>(vec_obj + 0x8, len.wrapping_sub(1));
    wr::<u32>(vec_obj + 0x328, val);
}

pub unsafe fn diff_rend_14220a650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xaa_u8; 4 * 0x30];
    let mut buf_n = [0xaa_u8; 4 * 0x30];
    let mut obj_o = [0u8; 0x340];
    let mut obj_n = [0u8; 0x340];
    wr::<usize>(obj_o.as_mut_ptr() as usize, buf_o.as_mut_ptr() as usize);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0x8, 1);
    wr::<u32>(obj_o.as_mut_ptr() as usize + 0xc, 4);
    wr::<usize>(obj_n.as_mut_ptr() as usize, buf_n.as_mut_ptr() as usize);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0x8, 1);
    wr::<u32>(obj_n.as_mut_ptr() as usize + 0xc, 4);

    f_orig(0, 0x8765_4321, obj_o.as_mut_ptr() as usize, 0);
    f_new(0, 0x8765_4321, obj_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(buf_o, buf_n, "entry_buf");
    crate::diff_eq!(&obj_o[0x8..], &obj_n[0x8..], "vec_obj");
    Ok(1)
}

/// `rend::TextureMetadata::ClassTypeInfoImplementation::vfunc_14`
/// Original: `0x14220a710` (215 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220a710(_this: usize, src: usize, dst: usize) {
    wr::<u8>(dst + 0x8, rd::<u8>(src + 0x8));
    wr::<u32>(dst + 0x10, rd::<u32>(src + 0x10));
    wr::<u16>(dst + 0x14, rd::<u16>(src + 0x14));
    wr::<u16>(dst + 0x16, rd::<u16>(src + 0x16));
    wr::<u16>(dst + 0x18, rd::<u16>(src + 0x18));
    wr::<u16>(dst + 0x1a, rd::<u16>(src + 0x1a));
    wr::<u8>(dst + 0x1c, rd::<u8>(src + 0x1c));
    wr::<u8>(dst + 0x1d, rd::<u8>(src + 0x1d));
    wr::<u8>(dst + 0x1e, rd::<u8>(src + 0x1e));
    wr::<u8>(dst + 0x1f, rd::<u8>(src + 0x1f));

    let src_arr = src + 0x20;
    let dst_arr = dst + 0x20;
    if dst_arr != src_arr {
        wr::<u64>(dst + 0x60, 0);
        let count = rd::<u64>(src + 0x60) as usize;
        for i in 0..count {
            wr::<u64>(dst_arr + i * 8, rd::<u64>(src_arr + i * 8));
        }
        wr::<u64>(dst + 0x60, rd::<u64>(src + 0x60));
    }

    wr::<u32>(dst + 0x68, rd::<u32>(src + 0x68));
    wr::<u32>(dst + 0x6c, rd::<u32>(src + 0x6c));
    wr::<u32>(dst + 0x70, rd::<u32>(src + 0x70));
    wr::<u32>(dst + 0x74, rd::<u32>(src + 0x74));
    wr::<u32>(dst + 0x78, rd::<u32>(src + 0x78));
}

pub unsafe fn diff_rend_14220a710(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0x80];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(17).wrapping_add(3);
    }
    wr::<u64>(src.as_mut_ptr() as usize + 0x60, 5);

    let mut dst_o = [0u8; 0x80];
    let mut dst_n = [0u8; 0x80];
    f_orig(0, src.as_ptr() as usize, dst_o.as_mut_ptr() as usize);
    f_new(0, src.as_ptr() as usize, dst_n.as_mut_ptr() as usize);
    crate::diff_eq!(dst_o, dst_n, "texture_metadata_copy");
    Ok(1)
}

/// `rend::TextureMetadata::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x14220a820` (65 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220a820(_this: usize, dst: usize) -> usize {
    let vtbl = exe::ptr::<u8>(0x1_433e_fd70) as usize;
    wr::<u8>(dst + 0x8, 1);
    wr::<usize>(dst, vtbl);
    wr::<u64>(dst + 0x14, 0);
    wr::<u64>(dst + 0x60, 0);
    wr::<u32>(dst + 0x10, 0xffff_ffff);
    wr::<u32>(dst + 0x1c, 0x0000_ff01);
    wr::<u32>(dst + 0x68, 1);
    wr::<u64>(dst + 0x6c, 1);
    wr::<u64>(dst + 0x74, 0x3f80_0000);
    dst
}

pub unsafe fn diff_rend_14220a820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut dst_o = [0x5au8; 0x80];
    let mut dst_n = [0x5au8; 0x80];
    let ro = f_orig(0, dst_o.as_mut_ptr() as usize);
    let rn = f_new(0, dst_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, dst_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, dst_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(dst_o, dst_n, "texture_metadata_init");
    Ok(1)
}

/// `rend::rend_14220a9b0`
/// Original: `0x14220a9b0` (100 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220a9b0(
    out: usize,
    meta: usize,
    flag: u8,
    span: usize,
) -> usize {
    wr::<u32>(out + 0x10, 0xff);
    wr::<u64>(out, 0);
    wr::<u32>(out + 0x8, 0);
    wr::<u64>(out + 0x18, 0);
    wr::<u64>(out + 0x20, 0);
    wr::<u32>(out + 0xc, 0xffff_ffff);
    wr::<u8>(out + 0x10, rd::<u8>(meta + 0x1d));
    wr::<u8>(out + 0x11, rd::<u8>(meta + 0x1e));
    wr::<u32>(out, rd::<u16>(meta + 0x16) as u32);
    wr::<u32>(out + 0x4, rd::<u16>(meta + 0x18) as u32);
    wr::<u32>(out + 0x8, rd::<u16>(meta + 0x1a) as u32);
    wr::<u8>(out + 0x12, rd::<u8>(meta + 0x1c));
    wr::<u8>(out + 0x13, flag);
    wr::<u32>(out + 0xc, rd::<u32>(meta + 0x10));
    wr::<u64>(out + 0x20, rd::<u64>(span + 0x8));
    wr::<u64>(out + 0x18, rd::<u64>(span));
    out
}

pub unsafe fn diff_rend_14220a9b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut meta = [0u8; 0x30];
    let m_ptr = meta.as_mut_ptr() as usize;
    wr::<u32>(m_ptr + 0x10, 0x42);
    wr::<u16>(m_ptr + 0x16, 1920);
    wr::<u16>(m_ptr + 0x18, 1080);
    wr::<u16>(m_ptr + 0x1a, 1);
    wr::<u8>(m_ptr + 0x1c, 2);
    wr::<u8>(m_ptr + 0x1d, 10);
    wr::<u8>(m_ptr + 0x1e, 1);

    let span = [0x1122_3344_5566_7788u64, 0x99aa_bbcc_ddee_ff00u64];
    let mut out_o = [0xffu8; 0x28];
    let mut out_n = [0xffu8; 0x28];

    let ro = f_orig(out_o.as_mut_ptr() as usize, m_ptr, 7, span.as_ptr() as usize);
    let rn = f_new(out_n.as_mut_ptr() as usize, m_ptr, 7, span.as_ptr() as usize);
    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(out_o, out_n, "out_desc");
    Ok(1)
}

/// `rend::TextureResource::vfunc_9`
/// Original: `0x14220aa20` (105 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220aa20(this: usize) -> u64 {
    if rd::<u8>(this + 0xb8) == 0 {
        return rd::<u64>(this + 0x168).wrapping_add(rd::<u64>(this + 0x160));
    }
    let p_1d0 = rd::<usize>(this + 0x1d0);
    if p_1d0 == 0 {
        return 0;
    }
    let sub = rd::<usize>(p_1d0 + 0x60);
    let vtbl = rd::<usize>(sub);
    let fn_ptr = rd::<usize>(vtbl + 0x20);
    let r8 = call_va!(fn_ptr, (sub) -> u64);
    let p10 = rd::<usize>(p_1d0 + 0x10);
    let rdx = rd::<i32>(p_1d0 + 0x38) as i64 as u64;
    rd::<u64>(p10 + 0xe0).wrapping_add(rdx).wrapping_add(r8)
}

pub unsafe fn diff_rend_14220aa20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_vfunc_20(_this: usize) -> u64 {
        0x5000
    }

    let mut obj = [0u8; 0x1e0];
    let ptr = obj.as_mut_ptr() as usize;
    wr::<u64>(ptr + 0x160, 0x1200);
    wr::<u64>(ptr + 0x168, 0x3400);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "b8_zero");

    wr::<u8>(ptr + 0xb8, 1);
    wr::<usize>(ptr + 0x1d0, 0);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "b8_one_null_1d0");

    let mut vtbl = [0usize; 8];
    vtbl[0x20 / 8] = mock_vfunc_20 as *const () as usize;
    let sub = [vtbl.as_ptr() as usize];
    let mut p10 = [0u8; 0xf0];
    wr::<u64>(p10.as_mut_ptr() as usize + 0xe0, 0x10_0000);
    let mut d0 = [0u8; 0x70];
    wr::<usize>(d0.as_mut_ptr() as usize + 0x10, p10.as_ptr() as usize);
    wr::<i32>(d0.as_mut_ptr() as usize + 0x38, 0x400);
    wr::<usize>(d0.as_mut_ptr() as usize + 0x60, sub.as_ptr() as usize);
    wr::<usize>(ptr + 0x1d0, d0.as_ptr() as usize);
    crate::diff_eq!(f_orig(ptr), f_new(ptr), "b8_one_with_1d0");
    Ok(3)
}

/// `rend::rend_14220aa90`
/// Original: `0x14220aa90` (303 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14220aa90(this: usize) -> u8 {
    if rd::<u8>(this + 0xb9) != 0 {
        let last = rd::<u8>(this + 0xbb).wrapping_sub(1) as usize;
        if rd::<u8>(this + 0x158 + last) == 2 {
            wr::<u8>(this + 0xb9, 0);
            let cnt = exe::ptr::<AtomicI32>(0x1_43a2_b148);
            (*cnt).fetch_sub(1, Ordering::SeqCst);
        }
    }

    let mut al = rd::<u8>(this + 0xbb);
    let mut dl: u8 = 0;
    let mut sil = rd::<u8>(this + 0xbc);
    if al != 0 {
        loop {
            let state = rd::<u8>(this + 0x158 + (dl as usize));
            if state == 2 && (dl <= rd::<u8>(this + 0xbd) || dl < sil) {
                sil = dl;
            }
            al = rd::<u8>(this + 0xbb);
            dl = dl.wrapping_add(1);
            if dl >= al {
                break;
            }
        }
    }

    let mut bpl = u8::from(sil == rd::<u8>(this + 0xbd));
    let mut dil: u8 = 0;
    if al != 1 {
        loop {
            if dil != sil && dil != rd::<u8>(this + 0xbd) {
                let state = rd::<u8>(this + 0x158 + (dil as usize));
                if state == 1 {
                    bpl = 0;
                } else if state == 2 {
                    call_va!(0x1_4207_e500, (this + 0x108, dil as u32) -> ());
                }
            }
            let cl = rd::<u8>(this + 0xbb).wrapping_sub(1);
            dil = dil.wrapping_add(1);
            if dil >= cl {
                break;
            }
        }
    }

    if sil != rd::<u8>(this + 0xbc) {
        wr::<u8>(this + 0xbc, sil);
        let ptr = rd::<u64>(this + 0x118 + (sil as usize) * 8);
        wr::<u64>(this + 0x1a0, ptr);
        call_va!(0x1_4220_8970, (this) -> ());
    }
    bpl
}

pub unsafe fn diff_rend_14220aa90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let cnt_addr = exe::ptr::<i32>(0x1_43a2_b148) as usize;
    let saved_cnt = rd::<i32>(cnt_addr);

    let mut obj_o = [0u8; 0x1e0];
    let b_o = obj_o.as_mut_ptr() as usize;
    wr::<u8>(b_o + 0xb9, 1);
    wr::<u8>(b_o + 0xbb, 3);
    wr::<u8>(b_o + 0xbc, 2);
    wr::<u8>(b_o + 0xbd, 1);
    wr::<u8>(b_o + 0x158 + 0, 1);
    wr::<u8>(b_o + 0x158 + 1, 2);
    wr::<u8>(b_o + 0x158 + 2, 2);
    wr::<u64>(b_o + 0x118 + 8, 0xdead_beef);
    let mut obj_n = obj_o;

    wr::<i32>(cnt_addr, 5);
    let ro = f_orig(obj_o.as_mut_ptr() as usize);
    let cnt_o = rd::<i32>(cnt_addr);

    wr::<i32>(cnt_addr, 5);
    let rn = f_new(obj_n.as_mut_ptr() as usize);
    let cnt_n = rd::<i32>(cnt_addr);

    wr::<i32>(cnt_addr, saved_cnt);

    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(cnt_o, cnt_n, "global_cnt");
    crate::diff_eq!(obj_o, obj_n, "obj_state");
    Ok(1)
}

/// `rend::rend_14220abd0`
/// Original: `0x14220abd0` (73 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14220abd0(this: usize, idx: u8) -> u8 {
    if rd::<u8>(this + 0x158 + (idx as usize)) == 0 {
        let ok = call_va!(0x1_4220_8b10, (this, idx as u32) -> u8);
        if ok == 0 {
            return 0;
        }
    }
    wr::<u8>(this + 0xbd, idx);
    1
}

pub unsafe fn diff_rend_14220abd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut MOCK_RET: u8 = 0;
    unsafe extern "win64" fn mock_8b10(_this: usize, _idx: u32) -> u8 {
        unsafe { MOCK_RET }
    }
    let _m = crate::difftest::Mock::new(0x1_4220_8b10, mock_8b10 as *const ())?;

    let mut obj_o = [0u8; 0x170];
    obj_o[0x158 + 2] = 1;
    let mut obj_n = obj_o;

    let ro1 = f_orig(obj_o.as_mut_ptr() as usize, 2);
    let rn1 = f_new(obj_n.as_mut_ptr() as usize, 2);
    crate::diff_eq!(ro1, rn1, "present_ret");
    crate::diff_eq!(obj_o[0xbd], obj_n[0xbd], "present_bd");

    MOCK_RET = 0;
    let ro2 = f_orig(obj_o.as_mut_ptr() as usize, 3);
    let rn2 = f_new(obj_n.as_mut_ptr() as usize, 3);
    crate::diff_eq!(ro2, rn2, "miss_fail_ret");
    crate::diff_eq!(obj_o[0xbd], obj_n[0xbd], "miss_fail_bd");

    MOCK_RET = 1;
    let ro3 = f_orig(obj_o.as_mut_ptr() as usize, 3);
    let rn3 = f_new(obj_n.as_mut_ptr() as usize, 3);
    crate::diff_eq!(ro3, rn3, "miss_ok_ret");
    crate::diff_eq!(obj_o[0xbd], obj_n[0xbd], "miss_ok_bd");
    Ok(3)
}

/// `rend::rend_14220ac20`
/// Original: `0x14220ac20` (296 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14220ac20(this: usize) {
    let lock_ptr = this.wrapping_add(0x1c8);
    let mut locked = false;
    if rd::<u8>(this + 0xb8) != 0 {
        if lock_ptr == 0 {
            call_va!(0x1_412d_fa60, (1u32) -> ());
            return;
        }
        acquire_spinlock_u8(lock_ptr);
        locked = true;
    }

    let prev = (*((this + 0xd4) as *const AtomicI32)).fetch_sub(1, Ordering::SeqCst);
    if prev == 1 {
        let mut p_1d0 = rd::<usize>(this + 0x1d0);
        if p_1d0 != 0 {
            wr::<u8>(this + 0xbc, 0xff);
            let p28 = rd::<usize>(this + 0x28);
            let c168 = rd::<usize>(p28 + 0x168);
            let c170 = rd::<usize>(p28 + 0x170);
            let fallback = if c168 == 0 && c170 == 0 {
                rd::<usize>(p28 + 0x158)
            } else {
                let p30 = rd::<u64>(this + 0x30);
                let res = call_va!(c170, (c168, p30) -> usize);
                p_1d0 = rd::<usize>(this + 0x1d0);
                res
            };
            let new_1a0 = if fallback != 0 {
                rd::<u64>(fallback + 0x1a0)
            } else {
                0
            };
            wr::<u64>(this + 0x1a0, new_1a0);
            wr::<usize>(this + 0x1d0, 0);
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fe88) as usize);
            call_va!(0x1_421f_28a0, (mgr, p_1d0) -> ());
            call_va!(0x1_4220_8970, (this) -> ());
        }
    }

    if locked {
        wr::<u8>(lock_ptr, 0);
    }
}

pub unsafe fn diff_rend_14220ac20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut RELEASED_1D0: usize = 0;
    unsafe extern "win64" fn mock_28a0(_mgr: usize, p: usize) {
        unsafe {
            RELEASED_1D0 = p;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_421f_28a0, mock_28a0 as *const ())?;

    let mut fallback = [0u8; 0x1b0];
    wr::<u64>(fallback.as_mut_ptr() as usize + 0x1a0, 0xcafe_babe);
    let mut p28 = [0u8; 0x180];
    wr::<usize>(p28.as_mut_ptr() as usize + 0x158, fallback.as_ptr() as usize);

    let mut obj_o = [0u8; 0x1e0];
    let bo = obj_o.as_mut_ptr() as usize;
    wr::<usize>(bo + 0x28, p28.as_ptr() as usize);
    wr::<u8>(bo + 0xb8, 1);
    wr::<i32>(bo + 0xd4, 1);
    wr::<usize>(bo + 0x1d0, 0x7777);
    let mut obj_n = obj_o;

    RELEASED_1D0 = 0;
    f_orig(obj_o.as_mut_ptr() as usize);
    let rel_o = RELEASED_1D0;

    RELEASED_1D0 = 0;
    f_new(obj_n.as_mut_ptr() as usize);
    let rel_n = RELEASED_1D0;

    crate::diff_eq!(rel_o, rel_n, "released_1d0");
    crate::diff_eq!(obj_o, obj_n, "obj_after_release");
    Ok(1)
}

/// `rend::rend_14220ad50`
/// Original: `0x14220ad50` (187 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14220ad50(this: usize) {
    let lock_ptr = this.wrapping_add(0x1c8);
    let mut locked = false;
    if rd::<u8>(this + 0xb8) != 0 {
        if lock_ptr == 0 {
            call_va!(0x1_412d_fa60, (1u32) -> ());
            return;
        }
        acquire_spinlock_u8(lock_ptr);
        locked = true;
    }

    let prev = (*((this + 0xd4) as *const AtomicI32)).fetch_add(1, Ordering::SeqCst);
    if rd::<u8>(this + 0xb8) != 0 && prev == 0 && rd::<i32>(this + 0x80) >= 4 {
        call_va!(0x1_4220_8640, (this) -> ());
    }

    if locked {
        wr::<u8>(lock_ptr, 0);
    }
}

pub unsafe fn diff_rend_14220ad50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut CALLED_8640: u32 = 0;
    unsafe extern "win64" fn mock_8640(_this: usize) {
        unsafe {
            CALLED_8640 += 1;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4220_8640, mock_8640 as *const ())?;

    let mut obj_o = [0u8; 0x1e0];
    let bo = obj_o.as_mut_ptr() as usize;
    wr::<i32>(bo + 0x80, 4);
    wr::<u8>(bo + 0xb8, 1);
    wr::<i32>(bo + 0xd4, 0);
    let mut obj_n = obj_o;

    CALLED_8640 = 0;
    f_orig(obj_o.as_mut_ptr() as usize);
    let c_o = CALLED_8640;

    CALLED_8640 = 0;
    f_new(obj_n.as_mut_ptr() as usize);
    let c_n = CALLED_8640;

    crate::diff_eq!(c_o, c_n, "called_8640");
    crate::diff_eq!(obj_o, obj_n, "obj_after_acquire");
    Ok(1)
}

/// `rend::rend_14220ae60`
/// Original: `0x14220ae60` (77 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220ae60(this: usize) -> u16 {
    let p30 = rd::<usize>(this + 0x30);
    let meta = rd::<usize>(p30 + 0x18);
    let casted = if meta == 0 {
        0
    } else {
        let vtbl = rd::<usize>(meta);
        let fn_ptr = rd::<usize>(vtbl + 0x98);
        let expected = rd::<u32>(exe::ptr::<u32>(0x1_4388_1958) as usize);
        let actual = call_va!(fn_ptr, (meta) -> u32);
        if actual == expected { meta } else { 0 }
    };
    rd::<u16>(casted + 0x18)
}

pub unsafe fn diff_rend_14220ae60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u16;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_get_type_id(_this: usize) -> u32 {
        rd::<u32>(exe::ptr::<u32>(0x1_4388_1958) as usize)
    }

    let mut vtbl = [0usize; 24];
    vtbl[0x98 / 8] = mock_get_type_id as *const () as usize;
    let mut meta = [0u8; 0x30];
    wr::<usize>(meta.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<u16>(meta.as_mut_ptr() as usize + 0x18, 1080);

    let mut p30 = [0u8; 0x20];
    wr::<usize>(p30.as_mut_ptr() as usize + 0x18, meta.as_ptr() as usize);

    let mut obj = [0u8; 0x40];
    wr::<usize>(obj.as_mut_ptr() as usize + 0x30, p30.as_ptr() as usize);

    crate::diff_eq!(
        f_orig(obj.as_ptr() as usize),
        f_new(obj.as_ptr() as usize),
        "height_1080"
    );
    Ok(1)
}

/// `rend::rend_14220aec0`
/// Original: `0x14220aec0` (77 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220aec0(this: usize) -> u16 {
    let p30 = rd::<usize>(this + 0x30);
    let meta = rd::<usize>(p30 + 0x18);
    let casted = if meta == 0 {
        0
    } else {
        let vtbl = rd::<usize>(meta);
        let fn_ptr = rd::<usize>(vtbl + 0x98);
        let expected = rd::<u32>(exe::ptr::<u32>(0x1_4388_1958) as usize);
        let actual = call_va!(fn_ptr, (meta) -> u32);
        if actual == expected { meta } else { 0 }
    };
    rd::<u16>(casted + 0x16)
}

pub unsafe fn diff_rend_14220aec0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u16;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_get_type_id(_this: usize) -> u32 {
        rd::<u32>(exe::ptr::<u32>(0x1_4388_1958) as usize)
    }

    let mut vtbl = [0usize; 24];
    vtbl[0x98 / 8] = mock_get_type_id as *const () as usize;
    let mut meta = [0u8; 0x30];
    wr::<usize>(meta.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<u16>(meta.as_mut_ptr() as usize + 0x16, 1920);

    let mut p30 = [0u8; 0x20];
    wr::<usize>(p30.as_mut_ptr() as usize + 0x18, meta.as_ptr() as usize);

    let mut obj = [0u8; 0x40];
    wr::<usize>(obj.as_mut_ptr() as usize + 0x30, p30.as_ptr() as usize);

    crate::diff_eq!(
        f_orig(obj.as_ptr() as usize),
        f_new(obj.as_ptr() as usize),
        "width_1920"
    );
    Ok(1)
}

/// `rend::rend_14220af40`
/// Original: `0x14220af40` (157 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220af40(this: usize, key: u64) -> u8 {
    let count = rd::<u64>(this + 0xc0a0) as usize;
    let mut cur = rd::<usize>(this + 0xc0e0);
    let end = cur.wrapping_add(count.wrapping_mul(0x38));
    if cur == end {
        return 0;
    }

    let empty_key = rd::<u64>(this + 0xc0a8);
    while cur != end {
        let k0 = rd::<u64>(cur);
        if k0 != empty_key && (rd::<u64>(this + 0xc090) == 0 || rd::<u64>(this + 0xc088) != k0) {
            break;
        }
        cur = cur.wrapping_add(0x38);
    }

    while cur != end {
        if rd::<u64>(cur + 0x10) == key {
            let target = rd::<usize>(cur + 0x18);
            wr::<u8>(target + 0x8, 1);
            return 1;
        }
        cur = cur.wrapping_add(0x38);
        if cur == end {
            return 0;
        }
        let empty_k = rd::<u64>(this + 0xc0a8);
        while cur != end {
            let k0 = rd::<u64>(cur);
            if k0 != empty_k && (rd::<u64>(this + 0xc090) == 0 || rd::<u64>(this + 0xc088) != k0) {
                break;
            }
            cur = cur.wrapping_add(0x38);
        }
    }
    0
}

pub unsafe fn diff_rend_14220af40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut target_o = [0u8; 16];
    let mut target_n = [0u8; 16];
    let mut buckets_o = [0u8; 2 * 0x38];
    let mut buckets_n = [0u8; 2 * 0x38];

    // Bucket 0: empty sentinel (0xffff), Bucket 1: active with key 0x42
    wr::<u64>(buckets_o.as_mut_ptr() as usize, 0xffff);
    wr::<u64>(buckets_o.as_mut_ptr() as usize + 0x38, 1);
    wr::<u64>(buckets_o.as_mut_ptr() as usize + 0x38 + 0x10, 0x42);
    wr::<usize>(
        buckets_o.as_mut_ptr() as usize + 0x38 + 0x18,
        target_o.as_mut_ptr() as usize,
    );

    wr::<u64>(buckets_n.as_mut_ptr() as usize, 0xffff);
    wr::<u64>(buckets_n.as_mut_ptr() as usize + 0x38, 1);
    wr::<u64>(buckets_n.as_mut_ptr() as usize + 0x38 + 0x10, 0x42);
    wr::<usize>(
        buckets_n.as_mut_ptr() as usize + 0x38 + 0x18,
        target_n.as_mut_ptr() as usize,
    );

    let mut this_o = vec![0u8; 0xc100];
    let mut this_n = vec![0u8; 0xc100];
    wr::<u64>(this_o.as_mut_ptr() as usize + 0xc0a0, 2);
    wr::<u64>(this_o.as_mut_ptr() as usize + 0xc0a8, 0xffff);
    wr::<usize>(
        this_o.as_mut_ptr() as usize + 0xc0e0,
        buckets_o.as_mut_ptr() as usize,
    );
    wr::<u64>(this_n.as_mut_ptr() as usize + 0xc0a0, 2);
    wr::<u64>(this_n.as_mut_ptr() as usize + 0xc0a8, 0xffff);
    wr::<usize>(
        this_n.as_mut_ptr() as usize + 0xc0e0,
        buckets_n.as_mut_ptr() as usize,
    );

    let r_miss_o = f_orig(this_o.as_mut_ptr() as usize, 0x99);
    let r_miss_n = f_new(this_n.as_mut_ptr() as usize, 0x99);
    crate::diff_eq!(r_miss_o, r_miss_n, "miss_ret");

    let r_hit_o = f_orig(this_o.as_mut_ptr() as usize, 0x42);
    let r_hit_n = f_new(this_n.as_mut_ptr() as usize, 0x42);
    crate::diff_eq!(r_hit_o, r_hit_n, "hit_ret");
    crate::diff_eq!(target_o, target_n, "target_flag");
    Ok(2)
}

/// `rend::TextureAtlasInterface::vfunc_1`
/// Original: `0x14220b020` (12 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14220b020(this: usize) {
    let sub = rd::<usize>(this + 0x48);
    wr::<u32>(sub + 0x18, 0);
}

pub unsafe fn diff_rend_14220b020(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut sub_o = [0xffu8; 0x20];
    let mut sub_n = [0xffu8; 0x20];
    let mut obj_o = [0u8; 0x50];
    let mut obj_n = [0u8; 0x50];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x48, sub_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x48, sub_n.as_mut_ptr() as usize);
    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(sub_o, sub_n, "atlas_clear_18");
    Ok(1)
}

/// `rend::rend_14221f5e0`
/// Original: `0x14221f5e0` (360 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14221f5e0(this: usize, pair: usize) {
    let count: u32 = if rd::<u8>(this + 0x31) != 0 { 4 } else { 1 };
    let mut vec_50 = [0u64; 2];
    let buf = call_va!(0x1_401d_ec00, (this, count) -> usize);
    let vec_50_ptr = vec_50.as_mut_ptr() as usize;
    wr::<usize>(vec_50_ptr, buf);
    wr::<u32>(vec_50_ptr + 0xc, count);
    call_va!(0x1_4183_acc2, (buf, 0i32, (count as usize) * 8) -> usize);
    wr::<u32>(vec_50_ptr + 0x8, count);

    let mut local_70 = [0u64; 2];
    let mut local_60 = [0u64; 2];
    let l70_ptr = local_70.as_mut_ptr() as usize;
    let l60_ptr = local_60.as_mut_ptr() as usize;
    call_va!(0x1_412e_a6a0, (l70_ptr, count) -> usize);
    call_va!(0x1_412e_a6a0, (l60_ptr, count) -> usize);

    let w0 = rd::<u32>(pair);
    let w1 = rd::<u32>(pair + 0x4);
    call_va!(
        0x1_4221_fc20,
        (this, 1u32, w0, w1, 0u32, l60_ptr, l70_ptr, vec_50_ptr, count) -> ()
    );

    call_va!(0x1_401e_59a0, (l60_ptr) -> ());
    call_va!(0x1_401e_59a0, (l70_ptr) -> ());
    if buf != 0 {
        crate::ns::r::mem::region_free(buf as *mut u8);
    }
}

pub unsafe fn diff_rend_14221f5e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut ALLOC_BUF: usize = 0;
    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_ec00(_this: usize, count: u32) -> usize {
        unsafe { rec(0x10 ^ (count as u64)) };
        unsafe { ALLOC_BUF }
    }
    unsafe extern "win64" fn m_a6a0(vec: usize, count: u32) -> usize {
        wr::<u64>(vec, 0xaaaa_0000 | (count as u64));
        wr::<u64>(vec + 0x8, count as u64);
        unsafe { rec(0x20 ^ (count as u64)) };
        vec
    }
    unsafe extern "win64" fn m_fc20(
        _this: usize,
        a2: u32,
        w0: u32,
        w1: u32,
        a5: u8,
        l60: usize,
        l70: usize,
        v50: usize,
        count: u32,
    ) {
        unsafe {
            rec(0x30
                ^ (a2 as u64)
                ^ ((w0 as u64) << 8)
                ^ ((w1 as u64) << 16)
                ^ (a5 as u64)
                ^ rd::<u64>(l60)
                ^ rd::<u64>(l70)
                ^ (rd::<u32>(v50 + 0x8) as u64)
                ^ ((count as u64) << 32));
        }
    }
    unsafe extern "win64" fn m_59a0(vec: usize) {
        unsafe { rec(0x40 ^ rd::<u64>(vec)) };
    }
    unsafe extern "win64" fn m_3200(chunk: usize, span: usize, is_local: u8, p: usize) {
        unsafe { rec(0x50 ^ (chunk as u64) ^ (span as u64) ^ (is_local as u64) ^ (p as u64)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ec00, m_ec00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_a6a0, m_a6a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4221_fc20, m_fc20 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_59a0, m_59a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4253_3200, m_3200 as *const ())?;

    // Use a 32-byte scratch region in exe .data so `(p - 1) & !0x1ff_ffff` lands in mapped exe memory.
    let scratch = exe::ptr::<u8>(0x1_43c1_c400) as *mut u8;
    let snap = crate::difftest::Snapshot::take(scratch, 32);
    ALLOC_BUF = scratch as usize;

    for flag_31 in [0u8, 1u8] {
        let mut this_buf = [0u8; 0x40];
        this_buf[0x31] = flag_31;
        let pair = [128u32, 256u32];

        std::ptr::write_bytes(scratch, 0x5a, 32);
        TRACE = 1;
        f_orig(this_buf.as_ptr() as usize, pair.as_ptr() as usize);
        let tr_o = TRACE;
        let mem_o = snap.current();

        std::ptr::write_bytes(scratch, 0x5a, 32);
        TRACE = 1;
        f_new(this_buf.as_ptr() as usize, pair.as_ptr() as usize);
        let tr_n = TRACE;
        let mem_n = snap.current();

        snap.restore();
        crate::diff_eq!(tr_o, tr_n, "14221f5e0 trace flag={flag_31}");
        crate::diff_eq!(mem_o, mem_n, "14221f5e0 memset flag={flag_31}");
    }
    Ok(2)
}

/// `rend::rend_1422207e0`
/// Original: `0x1422207e0` (206 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422207e0(
    _this: usize,
    out: usize,
    width: u32,
    height: u32,
) -> usize {
    let b3 = rd::<u8>(exe::ptr::<u8>(0x1_43c1_c3f3) as usize) as u32;
    let inline_cap = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cur_cap = (rd::<u32>(exe::ptr::<u32>(0x1_43c1_c3f0) as usize) & 0x00ff_ffff) << 3;
    let data_addr = exe::ptr::<u8>(0x1_43c1_c3f8) as usize;
    let cstr_ptr = if cur_cap > inline_cap {
        rd::<usize>(data_addr)
    } else {
        data_addr
    };
    let fmt_ptr = exe::ptr::<u8>(0x1_433f_0778) as usize;
    let formatted = call_va!(0x1_423f_e030, (fmt_ptr, cstr_ptr, width, height) -> usize);

    let hdr = (rd::<u32>(out) & 0xff00_0020) | 0x20;
    wr::<u32>(out, hdr);
    wr::<u8>(out + 0x3, 0x20);
    wr::<u32>(out + 0x4, 0);
    wr::<u8>(out + 0x8, 0);

    if formatted == 0 {
        wr::<u32>(out + 0x4, 0);
        let cap_words = rd::<u32>(out) & 0x00ff_ffff;
        let dst_buf = if cap_words > 0x20 {
            rd::<usize>(out + 0x8)
        } else {
            out + 0x8
        };
        wr::<u8>(dst_buf, 0);
    } else {
        let mut len = 0usize;
        while rd::<u8>(formatted.wrapping_add(len)) != 0 {
            len = len.wrapping_add(1);
        }
        let mut view = [formatted as u64, len as u64];
        call_va!(
            0x1_4240_9b50,
            (out, view.as_mut_ptr() as usize, 0usize, len) -> usize
        );
    }
    out
}

pub unsafe fn diff_rend_1422207e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_base = exe::ptr::<u8>(0x1_43c1_c3f0) as usize;
    let saved_g = rd::<[u8; 16]>(g_base);
    wr::<u32>(g_base, 0x2000_0020);
    wr::<u32>(g_base + 0x4, 0);
    wr::<u8>(g_base + 0x8, 0);

    let mut out_o = [0u8; 0x108];
    let mut out_n = [0u8; 0x108];
    let ro = f_orig(0, out_o.as_mut_ptr() as usize, 16, 32);
    let rn = f_new(0, out_n.as_mut_ptr() as usize, 16, 32);

    wr::<[u8; 16]>(g_base, saved_g);

    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(&out_o[0..32], &out_n[0..32], "formatted_lsao_path");
    Ok(1)
}

/// `rend::rend_1422222e0`
/// Original: `0x1422222e0` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_1422222e0(out: usize, val: u64, flag_ptr: usize) -> usize {
    wr::<u8>(out + 0x8, rd::<u8>(flag_ptr));
    wr::<u64>(out, val);
    out
}

pub unsafe fn diff_rend_1422222e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let flag = 0x5au8;
    let mut out_o = [0u8; 16];
    let mut out_n = [0u8; 16];
    let ro = f_orig(
        out_o.as_mut_ptr() as usize,
        0x1122_3344_5566_7788,
        (&flag as *const u8) as usize,
    );
    let rn = f_new(
        out_n.as_mut_ptr() as usize,
        0x1122_3344_5566_7788,
        (&flag as *const u8) as usize,
    );
    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(out_o, out_n, "out_pair");
    Ok(1)
}

/// `rend::rend_142223500`
/// Original: `0x142223500` (132 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_142223500(out: usize) -> usize {
    #[repr(C, align(16))]
    struct AlignedView([u64; 2]);

    let lsao_ptr = exe::ptr::<u8>(0x1_4307_8fc4) as u64;
    let mut in_view1 = AlignedView([lsao_ptr, 4]);
    let mut view_40 = AlignedView([0, 0]);
    call_va!(
        0x1_401d_f850,
        (view_40.0.as_mut_ptr() as usize, in_view1.0.as_mut_ptr() as usize) -> usize
    );

    let empty_ptr = exe::ptr::<u8>(0x1_426c_dcb0) as u64;
    let mut in_view2 = AlignedView([empty_ptr, 0]);
    let mut view_50 = AlignedView([0, 0]);
    call_va!(
        0x1_401d_f850,
        (view_50.0.as_mut_ptr() as usize, in_view2.0.as_mut_ptr() as usize) -> usize
    );

    call_va!(
        0x1_401e_06c0,
        (out, view_50.0.as_mut_ptr() as usize, view_40.0.as_mut_ptr() as usize) -> usize
    );
    out
}

pub unsafe fn diff_rend_142223500(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut out_o = [0u8; 32];
    let mut out_n = [0u8; 32];
    let ro = f_orig(out_o.as_mut_ptr() as usize);
    let rn = f_new(out_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(out_o, out_n, "lsao_path_out");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: none (no function in rend_e17_a.rs returns f32/f64 in xmm0).
//
// Replacement::verified(0x1_4220_14a0, "rend::UserRendererSettings::ClassTypeInfoImplementation::vfunc_14", rend::rend_e17_a::rend_1422014a0 as *const (), rend::rend_e17_a::diff_rend_1422014a0),
// Replacement::verified(0x1_4220_1d70, "rend::rend_142201d70", rend::rend_e17_a::rend_142201d70 as *const (), rend::rend_e17_a::diff_rend_142201d70),
// Replacement::verified(0x1_4220_1e30, "rend::rend_142201e30", rend::rend_e17_a::rend_142201e30 as *const (), rend::rend_e17_a::diff_rend_142201e30),
// Replacement::verified(0x1_4220_1ef0, "rend::rend_142201ef0", rend::rend_e17_a::rend_142201ef0 as *const (), rend::rend_e17_a::diff_rend_142201ef0),
// Replacement::verified(0x1_4220_9180, "rend::rend_142209180", rend::rend_e17_a::rend_142209180 as *const (), rend::rend_e17_a::diff_rend_142209180),
// Replacement::verified_throws(0x1_4220_91c0, "rend::TextureResource::vfunc_7", rend::rend_e17_a::rend_1422091c0 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_1422091c0),
// Replacement::verified_throws(0x1_4220_92d0, "rend::rend_1422092d0", rend::rend_e17_a::rend_1422092d0 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_1422092d0),
// Replacement::verified_throws(0x1_4220_93e0, "rend::TextureResource::vfunc_13", rend::rend_e17_a::rend_1422093e0 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_1422093e0),
// Replacement::verified(0x1_4220_94e0, "rend::TextureResource::vfunc_5", rend::rend_e17_a::rend_1422094e0 as *const (), rend::rend_e17_a::diff_rend_1422094e0),
// Replacement::verified(0x1_4220_9530, "rend::TextureResource::vfunc_12", rend::rend_e17_a::rend_142209530 as *const (), rend::rend_e17_a::diff_rend_142209530),
// Replacement::verified(0x1_4220_9650, "rend::rend_142209650", rend::rend_e17_a::rend_142209650 as *const (), rend::rend_e17_a::diff_rend_142209650),
// Replacement::verified(0x1_4220_9780, "rend::rend_142209780", rend::rend_e17_a::rend_142209780 as *const (), rend::rend_e17_a::diff_rend_142209780),
// Replacement::verified_throws(0x1_4220_9880, "rend::rend_142209880", rend::rend_e17_a::rend_142209880 as *const (), Throws::Discard, rend::rend_e17_a::diff_rend_142209880),
// Replacement::verified(0x1_4220_98e0, "rend::rend_1422098e0", rend::rend_e17_a::rend_1422098e0 as *const (), rend::rend_e17_a::diff_rend_1422098e0),
// Replacement::verified_throws(0x1_4220_9ed0, "rend::TextureResource::ClassTypeInfoImplementation::vfunc_13", rend::rend_e17_a::rend_142209ed0 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_142209ed0),
// Replacement::verified_throws(0x1_4220_9f40, "rend::TextureResource::ClassTypeInfoImplementation::vfunc_11", rend::rend_e17_a::rend_142209f40 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_142209f40),
// Replacement::verified_throws(0x1_4220_a000, "rend::TextureResource::ClassTypeInfoImplementation::vfunc_10", rend::rend_e17_a::rend_14220a000 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_14220a000),
// Replacement::verified(0x1_4220_a210, "rend::TextureMetadata::vfunc_17", rend::rend_e17_a::rend_14220a210 as *const (), rend::rend_e17_a::diff_rend_14220a210),
// Replacement::verified(0x1_4220_a650, "rend::TextureMetadata::ClassTypeInfoImplementation::vfunc_35", rend::rend_e17_a::rend_14220a650 as *const (), rend::rend_e17_a::diff_rend_14220a650),
// Replacement::verified(0x1_4220_a710, "rend::TextureMetadata::ClassTypeInfoImplementation::vfunc_14", rend::rend_e17_a::rend_14220a710 as *const (), rend::rend_e17_a::diff_rend_14220a710),
// Replacement::verified(0x1_4220_a820, "rend::TextureMetadata::ClassTypeInfoImplementation::vfunc_12", rend::rend_e17_a::rend_14220a820 as *const (), rend::rend_e17_a::diff_rend_14220a820),
// Replacement::verified(0x1_4220_a9b0, "rend::rend_14220a9b0", rend::rend_e17_a::rend_14220a9b0 as *const (), rend::rend_e17_a::diff_rend_14220a9b0),
// Replacement::verified(0x1_4220_aa20, "rend::TextureResource::vfunc_9", rend::rend_e17_a::rend_14220aa20 as *const (), rend::rend_e17_a::diff_rend_14220aa20),
// Replacement::verified_throws(0x1_4220_aa90, "rend::rend_14220aa90", rend::rend_e17_a::rend_14220aa90 as *const (), Throws::Discard, rend::rend_e17_a::diff_rend_14220aa90),
// Replacement::verified_throws(0x1_4220_abd0, "rend::rend_14220abd0", rend::rend_e17_a::rend_14220abd0 as *const (), Throws::Discard, rend::rend_e17_a::diff_rend_14220abd0),
// Replacement::verified_throws(0x1_4220_ac20, "rend::rend_14220ac20", rend::rend_e17_a::rend_14220ac20 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_14220ac20),
// Replacement::verified_throws(0x1_4220_ad50, "rend::rend_14220ad50", rend::rend_e17_a::rend_14220ad50 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_14220ad50),
// Replacement::verified(0x1_4220_ae60, "rend::rend_14220ae60", rend::rend_e17_a::rend_14220ae60 as *const (), rend::rend_e17_a::diff_rend_14220ae60),
// Replacement::verified(0x1_4220_aec0, "rend::rend_14220aec0", rend::rend_e17_a::rend_14220aec0 as *const (), rend::rend_e17_a::diff_rend_14220aec0),
// Replacement::verified(0x1_4220_af40, "rend::rend_14220af40", rend::rend_e17_a::rend_14220af40 as *const (), rend::rend_e17_a::diff_rend_14220af40),
// Replacement::verified(0x1_4220_b020, "rend::TextureAtlasInterface::vfunc_1", rend::rend_e17_a::rend_14220b020 as *const (), rend::rend_e17_a::diff_rend_14220b020),
// Replacement::verified_throws(0x1_4221_f5e0, "rend::rend_14221f5e0", rend::rend_e17_a::rend_14221f5e0 as *const (), Throws::Caught, rend::rend_e17_a::diff_rend_14221f5e0),
// Replacement::verified(0x1_4222_07e0, "rend::rend_1422207e0", rend::rend_e17_a::rend_1422207e0 as *const (), rend::rend_e17_a::diff_rend_1422207e0),
// Replacement::verified(0x1_4222_22e0, "rend::rend_1422222e0", rend::rend_e17_a::rend_1422222e0 as *const (), rend::rend_e17_a::diff_rend_1422222e0),
// Replacement::verified_throws(0x1_4222_3500, "rend::rend_142223500", rend::rend_e17_a::rend_142223500 as *const (), Throws::Discard, rend::rend_e17_a::diff_rend_142223500),
