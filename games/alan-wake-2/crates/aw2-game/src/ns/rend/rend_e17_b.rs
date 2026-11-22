//! Decompiled `rend` shadow-interface, shadow-map, shader-resource, and SDF-volume functions
//! (`0x142225af0..0x14224e680`).

use crate::exe::{self, call_iat, call_va, rd, wr};

#[inline(always)]
unsafe fn tls_slot0() -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    tls0
}

#[inline(always)]
unsafe fn srwlock_enter_recursive(lock_base: usize) {
    let tid = call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_base + 0x8) != tid {
        call_iat!(0x1_4264_55a8, (lock_base) -> ());
        wr::<u32>(lock_base + 0x8, tid);
    }
    wr::<u32>(lock_base + 0xc, rd::<u32>(lock_base + 0xc).wrapping_add(1));
}

#[inline(always)]
unsafe fn srwlock_leave_recursive(lock_base: usize) {
    let new_depth = rd::<u32>(lock_base + 0xc).wrapping_sub(1);
    wr::<u32>(lock_base + 0xc, new_depth);
    if new_depth == 0 {
        wr::<u32>(lock_base + 0x8, 0xffff_ffff);
        call_iat!(0x1_4264_5598, (lock_base) -> ());
    }
}

/// `rend_142225af0`
/// Original: `0x142225af0` (90 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_extract_proj_jitter_scale(out: usize, src: usize) -> usize {
    let mat_d = src.wrapping_add(0xd0);
    let mut mat_a = [0f32; 16];
    let mut mat_b = [0f32; 16];
    let mat_a_ptr = mat_a.as_mut_ptr() as usize;
    let mat_b_ptr = mat_b.as_mut_ptr() as usize;
    call_va!(0x1_403b_3de0, (mat_a_ptr, mat_d) -> usize);
    call_va!(0x1_403b_3de0, (mat_b_ptr, mat_d) -> usize);
    let m20 = rd::<f32>(mat_b_ptr + 0x20);
    let scale_x = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let m21 = rd::<f32>(mat_a_ptr + 0x24);
    let scale_y = rd::<f32>(exe::ptr::<f32>(0x1_4342_7068) as usize);
    wr::<f32>(out, m20 * scale_x);
    wr::<f32>(out + 0x4, m21 * scale_y);
    out
}

pub unsafe fn diff_rend_extract_proj_jitter_scale(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut src = [0u8; 0x160];
    let src_ptr = src.as_mut_ptr() as usize;
    for i in 0..16usize {
        wr::<f64>(src_ptr + 0xd0 + i * 8, (i as f64) * 0.25 - 1.5);
    }
    let mut out_o = [0f32; 2];
    let mut out_n = [0f32; 2];
    let r_o = f_orig(out_o.as_mut_ptr() as usize, src_ptr);
    let r_n = f_new(out_n.as_mut_ptr() as usize, src_ptr);
    crate::diff_eq!(r_o == out_o.as_mut_ptr() as usize, r_n == out_n.as_mut_ptr() as usize, "ret_ptr");
    crate::diff_eq!(out_o[0].to_bits(), out_n[0].to_bits(), "jitter_x");
    crate::diff_eq!(out_o[1].to_bits(), out_n[1].to_bits(), "jitter_y");
    Ok(1)
}

/// `rend::SSAA::vfunc_1`
/// Original: `0x142226180` (121 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_ssaa_vfunc_1(this: usize) {
    if rd::<u8>(this + 0x8) != 0 {
        let handle = rd::<usize>(this + 0x10);
        let obj = rd::<usize>(this + 0x18);
        if obj != 0 {
            let tls0 = tls_slot0();
            let mgr = rd::<usize>(tls0 + 0x6328);
            call_va!(0x1_4206_df80, (mgr) -> ());
            call_va!(0x1_4207_02c0, (obj, handle) -> ());
        }
        wr::<u8>(this + 0x8, 0);
    }
}

pub unsafe fn diff_rend_ssaa_vfunc_1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    for active in [0u8, 1u8] {
        let mut s_o = [0u8; 0x20];
        s_o[0x8] = active;
        let mut s_n = s_o;
        f_orig(s_o.as_mut_ptr() as usize);
        f_new(s_n.as_mut_ptr() as usize);
        crate::diff_eq!(s_o, s_n, "ssaa_vfunc_1");
    }
    Ok(2)
}

/// `rend::ShadowInterfaceDepth::vfunc_7`
/// Original: `0x1422428f0` (83 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_vfunc_7(this: usize) {
    let atlas = rd::<usize>(this + 0x28);
    if atlas != 0 {
        let _ = tls_slot0();
        let atlas = rd::<usize>(this + 0x28);
        let depth = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let dsv = rd::<usize>(atlas);
        let clear_dsv: unsafe extern "win64" fn(usize, usize, f32, u8, u32) =
            exe::func(0x1_4206_ecc0);
        clear_dsv(0, dsv, depth, 0, 2);
    }
}

pub unsafe fn diff_rend_shadow_interface_depth_vfunc_7(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut s_o = [0u8; 0x38];
    let mut s_n = [0u8; 0x38];
    f_orig(s_o.as_mut_ptr() as usize);
    f_new(s_n.as_mut_ptr() as usize);
    crate::diff_eq!(s_o, s_n, "depth_vfunc_7_null");
    Ok(1)
}

/// `rend::ShadowInterfaceDepth::vfunc_4`
/// Original: `0x142242990` (149 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_vfunc_4(this: usize) {
    srwlock_enter_recursive(this + 0x8);
    wr::<u32>(this + 0x54, 0);
    let mut i = 0u32;
    while i < rd::<u32>(this + 0x20) {
        let arr = rd::<usize>(this + 0x18);
        let elem = rd::<usize>(arr.wrapping_add((i as isize as usize).wrapping_mul(8)));
        wr::<u64>(elem + 0x8, 0);
        i = i.wrapping_add(1);
    }
    wr::<u32>(this + 0x70, 0);
    srwlock_leave_recursive(this + 0x8);
}

pub unsafe fn diff_rend_shadow_interface_depth_vfunc_4(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut e0_o = [0x1111u64, 0x2222u64];
    let mut e1_o = [0x3333u64, 0x4444u64];
    let ptrs_o = [e0_o.as_mut_ptr() as u64, e1_o.as_mut_ptr() as u64];
    let mut obj_o = [0u8; 0x80];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<u32>(base_o + 0x10, 0xffff_ffff);
    wr::<u64>(base_o + 0x18, ptrs_o.as_ptr() as u64);
    wr::<u32>(base_o + 0x20, 2);
    wr::<u32>(base_o + 0x54, 7);
    wr::<u32>(base_o + 0x70, 9);

    let mut e0_n = [0x1111u64, 0x2222u64];
    let mut e1_n = [0x3333u64, 0x4444u64];
    let ptrs_n = [e0_n.as_mut_ptr() as u64, e1_n.as_mut_ptr() as u64];
    let mut obj_n = obj_o;
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<u64>(base_n + 0x18, ptrs_n.as_ptr() as u64);

    f_orig(base_o);
    f_new(base_n);
    crate::diff_eq!(e0_o, e0_n, "e0");
    crate::diff_eq!(e1_o, e1_n, "e1");
    crate::diff_eq!(rd::<u32>(base_o + 0x54), rd::<u32>(base_n + 0x54), "f54");
    crate::diff_eq!(rd::<u32>(base_o + 0x70), rd::<u32>(base_n + 0x70), "f70");
    crate::diff_eq!(rd::<u32>(base_o + 0x10), rd::<u32>(base_n + 0x10), "lock_tid");
    crate::diff_eq!(rd::<u32>(base_o + 0x14), rd::<u32>(base_n + 0x14), "lock_depth");
    Ok(1)
}

/// `rend::ShadowInterfaceDepth::vfunc_3`
/// Original: `0x142242a30` (333 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_vfunc_3(this: usize) {
    srwlock_enter_recursive(this + 0x8);
    if rd::<u32>(this + 0x20) != 0 {
        let count_70 = rd::<u32>(this + 0x70);
        if count_70 != 0 {
            let mut out_a = 0u64;
            let mut out_b = 0u64;
            let out_a_ptr = (&mut out_a as *mut u64) as usize;
            let out_b_ptr = (&mut out_b as *mut u64) as usize;
            call_va!(
                0x1_4224_45b0,
                (
                    rd::<usize>(this + 0x68),
                    count_70,
                    rd::<u32>(this + 0x60),
                    rd::<u32>(this + 0x5c),
                    rd::<u32>(this + 0x58),
                    out_b_ptr,
                    out_a_ptr,
                ) -> ()
            );
        }
        let mut i = 0i32;
        if rd::<i32>(this + 0x54) > 0 {
            while i < rd::<i32>(this + 0x54) {
                let arr = rd::<usize>(this + 0x18);
                let item = rd::<usize>(arr.wrapping_add((i as isize as usize).wrapping_mul(8)));
                let slot_idx = rd::<i32>(item + 0x10) as isize as usize;
                let rects = rd::<usize>(this + 0x68);
                let rect = rects.wrapping_add(slot_idx.wrapping_mul(40));
                wr::<u32>(item + 0x24, rd::<u32>(rect));
                wr::<u32>(item + 0x28, rd::<u32>(rect + 0x4));
                wr::<u32>(item + 0x2c, rd::<u32>(rect + 0x8));
                wr::<u32>(item + 0x30, rd::<u32>(rect + 0xc));
                i = i.wrapping_add(1);
            }
            let mut j = 0i32;
            if rd::<i32>(this + 0x54) > 0 {
                while j < rd::<i32>(this + 0x54) {
                    let atlas = rd::<u64>(this + 0x28);
                    let arr = rd::<usize>(this + 0x18);
                    let item = rd::<usize>(arr.wrapping_add((j as isize as usize).wrapping_mul(8)));
                    wr::<u64>(item + 0x8, atlas);
                    j = j.wrapping_add(1);
                }
            }
        }
        let vtbl = rd::<usize>(this);
        let fn_38 = rd::<usize>(vtbl + 0x38);
        call_va!(fn_38, (this) -> ());
    }
    let idx = rd::<i32>(this + 0x38) as isize;
    let tbl = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
    let size = rd::<i32>(tbl.wrapping_add((idx as usize).wrapping_mul(16)).wrapping_add(0x8)) as i64 as u64;
    let src = rd::<usize>(this + 0x30);
    call_va!(0x1_4209_a900, (idx as i64 as u64, 0u32, size, src) -> ());
    srwlock_leave_recursive(this + 0x8);
}

pub unsafe fn diff_rend_shadow_interface_depth_vfunc_3(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static PACK_REC: AtomicU64 = AtomicU64::new(0);
    static UPLOAD_REC: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_pack(
        rects: usize,
        count: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        out_b: usize,
        out_a: usize,
    ) {
        wr::<u64>(out_b, 0x11);
        wr::<u64>(out_a, 0x22);
        let tag = (rects as u64)
            ^ ((count as u64) << 8)
            ^ ((a3 as u64) << 16)
            ^ ((a4 as u64) << 24)
            ^ ((a5 as u64) << 32);
        PACK_REC.store(tag, Ordering::SeqCst);
    }

    unsafe extern "win64" fn fake_upload(idx: u64, zero: u32, size: u64, src: usize) {
        let tag = idx ^ ((zero as u64) << 8) ^ (size << 16) ^ (src as u64);
        UPLOAD_REC.store(tag, Ordering::SeqCst);
    }

    unsafe extern "win64" fn fake_vfunc38(this: usize) {
        wr::<u32>(this + 0x40, rd::<u32>(this + 0x40).wrapping_add(0x1357));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_45b0, fake_pack as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_a900, fake_upload as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 8];
    vtbl[7] = fake_vfunc38 as *const () as usize;

    let mut rects = [0u32; 10];
    rects[0] = 10;
    rects[1] = 20;
    rects[2] = 300;
    rects[3] = 400;

    let mut item_o = [0u8; 0x38];
    let mut item_n = [0u8; 0x38];
    wr::<i32>((item_o.as_mut_ptr() as usize) + 0x10, 0);
    wr::<i32>((item_n.as_mut_ptr() as usize) + 0x10, 0);
    let arr_o = [item_o.as_mut_ptr() as usize];
    let arr_n = [item_n.as_mut_ptr() as usize];

    let mut obj_o = [0u8; 0x80];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o, vtbl.as_ptr() as usize);
    wr::<u32>(base_o + 0x10, 0xffff_ffff);
    wr::<usize>(base_o + 0x18, arr_o.as_ptr() as usize);
    wr::<u32>(base_o + 0x20, 1);
    wr::<u64>(base_o + 0x28, 0xdead_beef_cafe_babe);
    wr::<usize>(base_o + 0x30, 0x5555_0000);
    wr::<i32>(base_o + 0x38, 0);
    wr::<i32>(base_o + 0x54, 1);
    wr::<u32>(base_o + 0x58, 1024);
    wr::<u32>(base_o + 0x5c, 2048);
    wr::<u32>(base_o + 0x60, 512);
    wr::<usize>(base_o + 0x68, rects.as_ptr() as usize);
    wr::<u32>(base_o + 0x70, 1);

    let mut obj_n = obj_o;
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<usize>(base_n + 0x18, arr_n.as_ptr() as usize);

    PACK_REC.store(0, Ordering::SeqCst);
    UPLOAD_REC.store(0, Ordering::SeqCst);
    f_orig(base_o);
    let pack_o = PACK_REC.load(Ordering::SeqCst);
    let up_o = UPLOAD_REC.load(Ordering::SeqCst);

    PACK_REC.store(0, Ordering::SeqCst);
    UPLOAD_REC.store(0, Ordering::SeqCst);
    f_new(base_n);
    let pack_n = PACK_REC.load(Ordering::SeqCst);
    let up_n = UPLOAD_REC.load(Ordering::SeqCst);

    crate::diff_eq!(item_o, item_n, "item");
    crate::diff_eq!(rd::<u32>(base_o + 0x40), rd::<u32>(base_n + 0x40), "vfunc38_side_effect");
    crate::diff_eq!(pack_o, pack_n, "pack_call");
    crate::diff_eq!(up_o, up_n, "upload_call");
    Ok(1)
}

/// `rend::ShadowInterfaceDepth::vfunc_2`
/// Original: `0x142242b90` (285 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_vfunc_2(
    this: usize,
    w: u32,
    h: u32,
    p4: u32,
    p5: u32,
    p6: u32,
) -> usize {
    srwlock_enter_recursive(this + 0x8);
    let len = rd::<u32>(this + 0x20) as u64;
    let used = rd::<i32>(this + 0x54) as i64 as u64;
    let vec = this + 0x18;
    if used >= len {
        let new_elem = call_va!(0x1_412f_1dd0, () -> u64);
        let cur_len = rd::<u32>(vec + 0x8) as u64;
        let cur_cap = rd::<u32>(vec + 0xc) as u64;
        if cur_len.wrapping_mul(8) == cur_cap.wrapping_mul(8) {
            call_va!(0x1_4020_bcb0, (vec, cur_len.wrapping_add(1)) -> ());
        }
        let idx = rd::<u32>(vec + 0x8) as usize;
        let base = rd::<usize>(vec);
        wr::<u64>(base + idx * 8, new_elem);
        wr::<u32>(vec + 0x8, (idx as u32).wrapping_add(1));
    }
    let idx = rd::<i32>(this + 0x54) as isize as usize;
    wr::<i32>(this + 0x54, (idx as i32).wrapping_add(1));
    let base = rd::<usize>(vec);
    let elem = rd::<usize>(base.wrapping_add(idx.wrapping_mul(8)));
    wr::<u32>(elem + 0x2c, w);
    wr::<u32>(elem + 0x30, h);
    let slot = call_va!(0x1_4224_4fa0, (this + 0x58, w, h, p4, p5, p6) -> u32);
    wr::<u32>(elem + 0x10, slot);
    wr::<u8>(elem + 0x34, rd::<u8>(this + 0x50));
    srwlock_leave_recursive(this + 0x8);
    elem
}

pub unsafe fn diff_rend_shadow_interface_depth_vfunc_2(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32, u32, u32, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut elem_o = [0u8; 0x40];
    let elem_ptr_o = elem_o.as_mut_ptr() as u64;
    let mut rects_o = [0u8; 80];
    let mut obj_o = [0u8; 0x80];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<u32>(base_o + 0x10, 0xffff_ffff);
    wr::<u64>(base_o + 0x18, (&elem_ptr_o as *const u64) as u64);
    wr::<u32>(base_o + 0x20, 1);
    wr::<u32>(base_o + 0x24, 1);
    wr::<u8>(base_o + 0x50, 7);
    wr::<i32>(base_o + 0x54, 0);
    wr::<u64>(base_o + 0x68, rects_o.as_mut_ptr() as u64);
    wr::<u32>(base_o + 0x70, 0);
    wr::<u32>(base_o + 0x74, 2);

    let mut elem_n = [0u8; 0x40];
    let elem_ptr_n = elem_n.as_mut_ptr() as u64;
    let mut rects_n = [0u8; 80];
    let mut obj_n = obj_o;
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<u64>(base_n + 0x18, (&elem_ptr_n as *const u64) as u64);
    wr::<u64>(base_n + 0x68, rects_n.as_mut_ptr() as u64);

    let r_o = f_orig(base_o, 256, 512, 3, 0x3f80_0000, 42);
    let r_n = f_new(base_n, 256, 512, 3, 0x3f80_0000, 42);
    crate::diff_eq!(r_o == elem_ptr_o as usize, r_n == elem_ptr_n as usize, "ret_elem");
    crate::diff_eq!(elem_o, elem_n, "elem_state");
    crate::diff_eq!(&rects_o[..16], &rects_n[..16], "rects_head");
    crate::diff_eq!(&rects_o[24..40], &rects_n[24..40], "rects_tail");
    crate::diff_eq!(rd::<i32>(base_o + 0x54), rd::<i32>(base_n + 0x54), "used_count");
    crate::diff_eq!(rd::<u32>(base_o + 0x70), rd::<u32>(base_n + 0x70), "rect_len");
    Ok(1)
}

/// `rend::ShadowInterfaceDepth::vfunc_1`
/// Original: `0x142242cc0` (53 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_vfunc_1(this: usize, res: usize) {
    call_va!(0x1_4224_2d00, (this) -> ());
    wr::<u32>(this + 0x5c, rd::<u32>(res));
    wr::<u32>(this + 0x58, rd::<u32>(res + 0x4));
    call_va!(0x1_4224_2db0, (this, res) -> ());
}

pub unsafe fn diff_rend_shadow_interface_depth_vfunc_1(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CALL_SEQ: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_rel(this: usize) {
        wr::<u32>(this + 0x40, 0xaa);
        CALL_SEQ.fetch_add(1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_recreate(this: usize, res: usize) {
        wr::<u32>(this + 0x44, rd::<u32>(res).wrapping_add(rd::<u32>(res + 0x4)));
        CALL_SEQ.fetch_add(10, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_2d00, fake_rel as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4224_2db0, fake_recreate as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let res = [1024u32, 2048u32];
    let mut obj_o = [0u8; 0x80];
    let mut obj_n = [0u8; 0x80];

    CALL_SEQ.store(0, Ordering::SeqCst);
    f_orig(obj_o.as_mut_ptr() as usize, res.as_ptr() as usize);
    let seq_o = CALL_SEQ.load(Ordering::SeqCst);

    CALL_SEQ.store(0, Ordering::SeqCst);
    f_new(obj_n.as_mut_ptr() as usize, res.as_ptr() as usize);
    let seq_n = CALL_SEQ.load(Ordering::SeqCst);

    crate::diff_eq!(obj_o, obj_n, "obj_state");
    crate::diff_eq!(seq_o, seq_n, "call_seq");
    Ok(1)
}

/// `rend_142242d00`
/// Original: `0x142242d00` (160 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_release_atlas(this: usize) {
    let atlas = rd::<usize>(this + 0x28);
    wr::<u64>(this + 0x28, 0);
    if atlas != 0 {
        let tex = rd::<usize>(atlas);
        if tex != 0 {
            if let Err(e) = exe::try_call(exe::ptr::<u8>(0x1_4206_b020) as usize, &[tex as u64], &[]) {
                call_va!(0x1_4244_2de0, (tex) -> ());
                call_va!(0x1_4244_2de0, (atlas) -> ());
                e.rethrow();
            }
            call_va!(0x1_4244_2de0, (tex) -> ());
        }
        call_va!(0x1_4244_2de0, (atlas) -> ());
    }
    let mut i = 0u32;
    while (i as u64) < (rd::<u32>(this + 0x20) as u64) {
        let arr = rd::<usize>(this + 0x18);
        let elem = rd::<usize>(arr.wrapping_add((i as isize as usize).wrapping_mul(8)));
        wr::<u64>(elem + 0x8, 0);
        i = i.wrapping_add(1);
    }
    let idx = rd::<i32>(this + 0x38) as isize;
    let tbl = exe::ptr::<u8>(0x1_43a6_37b8) as usize;
    let size = rd::<i32>(tbl.wrapping_add((idx as usize).wrapping_mul(16))) as i64 as u64;
    let src = rd::<usize>(this + 0x30);
    call_va!(0x1_4209_a900, (idx as i64 as u64, 0u32, size, src) -> ());
}

pub unsafe fn diff_rend_shadow_interface_depth_release_atlas(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_dtor_tex(_tex: usize) {
        TRACE.fetch_add(1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_free(_ptr: usize) {
        TRACE.fetch_add(10, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_upload(idx: u64, zero: u32, size: u64, src: usize) {
        TRACE.fetch_add(100 + idx + (zero as u64) + size + (src as u64), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_b020, fake_dtor_tex as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, fake_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_a900, fake_upload as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tex_dummy = 0x1234_5678usize;
    let atlas = [tex_dummy];
    let mut elem0_o = [0x1111u64, 0x2222u64];
    let mut elem0_n = elem0_o;
    let arr_o = [elem0_o.as_mut_ptr() as usize];
    let arr_n = [elem0_n.as_mut_ptr() as usize];

    let mut obj_o = [0u8; 0x50];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o + 0x18, arr_o.as_ptr() as usize);
    wr::<u32>(base_o + 0x20, 1);
    wr::<usize>(base_o + 0x28, atlas.as_ptr() as usize);
    wr::<usize>(base_o + 0x30, 0x7700);
    wr::<i32>(base_o + 0x38, 0);

    let mut obj_n = obj_o;
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<usize>(base_n + 0x18, arr_n.as_ptr() as usize);

    TRACE.store(0, Ordering::SeqCst);
    f_orig(base_o);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(base_n);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(rd::<u64>(base_o + 0x28), rd::<u64>(base_n + 0x28), "atlas_cleared");
    crate::diff_eq!(elem0_o, elem0_n, "elem0");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

/// `rend_142242fb0`
/// Original: `0x142242fb0` (348 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_dtor(this: usize) {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_17c0) as u64);
    srwlock_enter_recursive(this + 0x8);
    wr::<u32>(this + 0x54, 0);
    let mut i = 0u32;
    while (i as u64) < (rd::<u32>(this + 0x20) as u64) {
        let arr = rd::<usize>(this + 0x18);
        let elem = rd::<usize>(arr.wrapping_add((i as isize as usize).wrapping_mul(8)));
        wr::<u64>(elem + 0x8, 0);
        i = i.wrapping_add(1);
    }
    wr::<u32>(this + 0x70, 0);
    srwlock_leave_recursive(this + 0x8);

    let mut j = 0u32;
    while (j as i32 as i64 as u64) < (rd::<u32>(this + 0x20) as u64) {
        let off = (j as i32 as isize as usize).wrapping_mul(8);
        let arr = rd::<usize>(this + 0x18);
        let elem = rd::<usize>(arr.wrapping_add(off));
        if elem != 0 {
            let vtbl = rd::<usize>(elem);
            let dtor = rd::<usize>(vtbl);
            call_va!(dtor, (elem, 0u32) -> ());
            call_va!(0x1_4244_2de0, (elem) -> ());
        }
        let arr2 = rd::<usize>(this + 0x18);
        wr::<u64>(arr2.wrapping_add(off), 0);
        j = j.wrapping_add(1);
    }
    wr::<u32>(this + 0x20, 0);
    call_va!(0x1_401d_e830, (this + 0x68) -> ());
    call_va!(0x1_4220_1e30, (this + 0x30) -> ());
    let atlas = rd::<usize>(this + 0x28);
    if atlas != 0 {
        let tex = rd::<usize>(atlas);
        if tex != 0 {
            call_va!(0x1_4206_b020, (tex) -> ());
            call_va!(0x1_4244_2de0, (tex) -> ());
        }
        call_va!(0x1_4244_2de0, (atlas) -> ());
    }
    call_va!(0x1_401d_e830, (this + 0x18) -> ());
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_1818) as u64);
}

pub unsafe fn diff_rend_shadow_interface_depth_dtor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static DTOR_TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_elem_dtor(_elem: usize, flags: u32) {
        DTOR_TRACE.fetch_add(1 + (flags as u64) * 1000, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_free(_p: usize) {
        DTOR_TRACE.fetch_add(10, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_rel_buf(_p: usize) {
        DTOR_TRACE.fetch_add(100, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_rel_30(_p: usize) {
        DTOR_TRACE.fetch_add(1000, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_dtor_tex(_p: usize) {
        DTOR_TRACE.fetch_add(10000, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4244_2de0, fake_free as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, fake_rel_buf as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4220_1e30, fake_rel_30 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4206_b020, fake_dtor_tex as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let elem_vtbl = [fake_elem_dtor as *const () as usize];
    let mut elem_o = [elem_vtbl.as_ptr() as u64, 0x9999u64];
    let mut elem_n = elem_o;
    let mut arr_o = [elem_o.as_mut_ptr() as usize];
    let mut arr_n = [elem_n.as_mut_ptr() as usize];
    let atlas = [0x1234_5678usize];

    let mut obj_o = [0u8; 0x80];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<u32>(base_o + 0x10, 0xffff_ffff);
    wr::<usize>(base_o + 0x18, arr_o.as_mut_ptr() as usize);
    wr::<u32>(base_o + 0x20, 1);
    wr::<usize>(base_o + 0x28, atlas.as_ptr() as usize);
    wr::<u32>(base_o + 0x54, 5);
    wr::<u32>(base_o + 0x70, 7);

    let mut obj_n = obj_o;
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<usize>(base_n + 0x18, arr_n.as_mut_ptr() as usize);

    DTOR_TRACE.store(0, Ordering::SeqCst);
    f_orig(base_o);
    let tr_o = DTOR_TRACE.load(Ordering::SeqCst);

    DTOR_TRACE.store(0, Ordering::SeqCst);
    f_new(base_n);
    let tr_n = DTOR_TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(rd::<u64>(base_o), rd::<u64>(base_n), "final_vtbl");
    crate::diff_eq!(rd::<u32>(base_o + 0x20), rd::<u32>(base_n + 0x20), "len_zeroed");
    crate::diff_eq!(rd::<u32>(base_o + 0x54), rd::<u32>(base_n + 0x54), "f54_zeroed");
    crate::diff_eq!(rd::<u32>(base_o + 0x70), rd::<u32>(base_n + 0x70), "f70_zeroed");
    crate::diff_eq!(arr_o, arr_n, "arr_cleared");
    crate::diff_eq!(elem_o, elem_n, "elem_state");
    crate::diff_eq!(tr_o, tr_n, "dtor_trace");
    Ok(1)
}

/// `rend_142243120`
/// Original: `0x142243120` (179 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_shadow_interface_depth_ctor(
    this: usize,
    fmt: u8,
    packed_res: u64,
) -> usize {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_17c0) as u64);
    wr::<u32>(this + 0x10, 0xffff_ffff);
    wr::<u32>(this + 0x14, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    let name_res = exe::ptr::<u8>(0x1_433f_48c8) as usize;
    if let Err(e) = exe::try_call(
        exe::ptr::<u8>(0x1_412c_f0e0) as usize,
        &[(this + 0x30) as u64, name_res as u64],
        &[],
    ) {
        call_va!(0x1_401d_e830, (this + 0x18) -> ());
        e.rethrow();
    }
    wr::<u32>(this + 0x54, 0);
    wr::<u64>(this + 0x68, 0);
    wr::<u64>(this + 0x70, 0);
    let w = packed_res as u32;
    let h = (packed_res >> 32) as u32;
    wr::<u32>(this + 0x5c, w);
    wr::<u32>(this + 0x58, h);
    wr::<u32>(this + 0x60, 0x200);
    wr::<u8>(this + 0x50, fmt);
    wr::<u64>(this + 0x48, exe::ptr::<u8>(0x1_433f_48b8) as u64);
    let mut res_pair = [w, h];
    let res_ptr = res_pair.as_mut_ptr() as usize;
    if let Err(e) = exe::try_call(
        exe::ptr::<u8>(0x1_4224_2db0) as usize,
        &[this as u64, res_ptr as u64],
        &[],
    ) {
        call_va!(0x1_401d_e830, (this + 0x68) -> ());
        call_va!(0x1_4220_1e30, (this + 0x30) -> ());
        call_va!(0x1_401d_e830, (this + 0x18) -> ());
        e.rethrow();
    }
    this
}

pub unsafe fn diff_rend_shadow_interface_depth_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_init_30(dst: usize, name: usize) {
        wr::<usize>(dst, name);
        wr::<u64>(dst + 0x8, 0x1234);
    }
    unsafe extern "win64" fn fake_init_atlas(this: usize, res_ptr: usize) {
        wr::<u64>(this + 0x28, rd::<u64>(res_ptr));
    }

    let _m1 = crate::difftest::Mock::new(0x1_412c_f0e0, fake_init_30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4224_2db0, fake_init_atlas as *const ())?;

    type F = unsafe extern "win64" fn(usize, u8, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = [0x5au8; 0x80];
    let mut obj_n = [0x5au8; 0x80];
    let packed = (2048u64 << 32) | 1024u64;
    let r_o = f_orig(obj_o.as_mut_ptr() as usize, 9, packed);
    let r_n = f_new(obj_n.as_mut_ptr() as usize, 9, packed);

    crate::diff_eq!(
        r_o == obj_o.as_mut_ptr() as usize,
        r_n == obj_n.as_mut_ptr() as usize,
        "ret_this"
    );
    crate::diff_eq!(obj_o, obj_n, "ctor_bytes");
    Ok(1)
}

/// `rend_1422431e0`
/// Original: `0x1422431e0` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_get_atlas_height(this: usize) -> u32 {
    let p1 = rd::<usize>(this + 0x8);
    if p1 != 0 {
        let p2 = rd::<usize>(p1);
        if p2 != 0 {
            return rd::<u32>(p2 + 0x4);
        }
    }
    0
}

pub unsafe fn diff_rend_shadow_map_depth_get_atlas_height(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let dims = [1024u32, 2048u32];
    let atlas = [dims.as_ptr() as usize];
    let null_atlas = [0usize];
    for p1 in [0usize, null_atlas.as_ptr() as usize, atlas.as_ptr() as usize] {
        let obj = [0usize, p1];
        let r_o = f_orig(obj.as_ptr() as usize);
        let r_n = f_new(obj.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "atlas_height");
    }
    Ok(3)
}

/// `rend_142243200`
/// Original: `0x142243200` (23 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_get_atlas_width(this: usize) -> u32 {
    let p1 = rd::<usize>(this + 0x8);
    if p1 != 0 {
        let p2 = rd::<usize>(p1);
        if p2 != 0 {
            return rd::<u32>(p2);
        }
    }
    0
}

pub unsafe fn diff_rend_shadow_map_depth_get_atlas_width(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let dims = [1024u32, 2048u32];
    let atlas = [dims.as_ptr() as usize];
    let null_atlas = [0usize];
    for p1 in [0usize, null_atlas.as_ptr() as usize, atlas.as_ptr() as usize] {
        let obj = [0usize, p1];
        let r_o = f_orig(obj.as_ptr() as usize);
        let r_n = f_new(obj.as_ptr() as usize);
        crate::diff_eq!(r_o, r_n, "atlas_width");
    }
    Ok(3)
}

/// `rend::ShadowMapDepth::vfunc_11`
/// Original: `0x142243220` (65 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_vfunc_11(this: usize) -> u8 {
    let p1 = rd::<usize>(this + 0x8);
    if p1 == 0 {
        return 0;
    }
    let p2 = rd::<usize>(p1);
    let max_w = if p2 != 0 { rd::<i32>(p2) } else { 0 };
    if rd::<i32>(this + 0x2c).wrapping_add(rd::<i32>(this + 0x24)) > max_w {
        return 0;
    }
    let max_h = if p2 != 0 { rd::<i32>(p2 + 0x4) } else { 0 };
    if rd::<i32>(this + 0x30).wrapping_add(rd::<i32>(this + 0x28)) > max_h {
        return 0;
    }
    1
}

pub unsafe fn diff_rend_shadow_map_depth_vfunc_11(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let dims = [1024i32, 1024i32];
    let atlas = [dims.as_ptr() as usize];
    let cases: [(usize, i32, i32, i32, i32); 4] = [
        (0, 0, 0, 128, 128),
        (atlas.as_ptr() as usize, 100, 200, 512, 512),
        (atlas.as_ptr() as usize, 600, 200, 512, 512),
        (atlas.as_ptr() as usize, 100, 600, 512, 512),
    ];
    for (p1, x, y, w, h) in cases {
        let mut obj = [0u8; 0x38];
        let base = obj.as_mut_ptr() as usize;
        wr::<usize>(base + 0x8, p1);
        wr::<i32>(base + 0x24, x);
        wr::<i32>(base + 0x28, y);
        wr::<i32>(base + 0x2c, w);
        wr::<i32>(base + 0x30, h);
        crate::diff_eq!(f_orig(base), f_new(base), "depth_vfunc_11");
    }
    Ok(4)
}

/// `rend::ShadowMapDepth::vfunc_7`
/// Original: `0x142243550` (182 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_vfunc_7(
    this: usize,
    out_x: usize,
    out_y: usize,
    out_w: usize,
    out_h: usize,
) {
    let p1 = rd::<usize>(this + 0x8);
    let (atlas_w, atlas_h_i) = if p1 != 0 {
        let p2 = rd::<usize>(p1);
        let w_i = if p2 != 0 { rd::<i32>(p2) } else { 0 };
        let p2_b = rd::<usize>(p1);
        let h_i = if p2_b != 0 { rd::<i32>(p2_b + 0x4) } else { 0 };
        (w_i as f32, h_i)
    } else {
        (0.0f32, 0i32)
    };
    let c_add = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let c_sub = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let x = (rd::<i32>(this + 0x24) as f32 + c_add) / atlas_w;
    wr::<f32>(out_x, x);
    let y_num = rd::<i32>(this + 0x28) as f32 + c_add;
    let atlas_h = atlas_h_i as f32;
    wr::<f32>(out_y, y_num / atlas_h);
    let w = (rd::<i32>(this + 0x2c) as f32 - c_sub) / atlas_w;
    wr::<f32>(out_w, w);
    let h = (rd::<i32>(this + 0x30) as f32 - c_sub) / atlas_h;
    wr::<f32>(out_h, h);
}

pub unsafe fn diff_rend_shadow_map_depth_vfunc_7(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let dims = [2048i32, 1024i32];
    let atlas = [dims.as_ptr() as usize];
    let mut obj = [0u8; 0x38];
    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base + 0x8, atlas.as_ptr() as usize);
    wr::<i32>(base + 0x24, 64);
    wr::<i32>(base + 0x28, 128);
    wr::<i32>(base + 0x2c, 512);
    wr::<i32>(base + 0x30, 256);
    let mut o = [0f32; 4];
    let mut n = [0f32; 4];
    let op = o.as_mut_ptr() as usize;
    let np = n.as_mut_ptr() as usize;
    f_orig(base, op, op + 4, op + 8, op + 12);
    f_new(base, np, np + 4, np + 8, np + 12);
    for i in 0..4 {
        crate::diff_eq!(o[i].to_bits(), n[i].to_bits(), "depth_vfunc_7");
    }
    Ok(1)
}

/// `rend::ShadowMapDepth::vfunc_6`
/// Original: `0x142243640` (213 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_vfunc_6(
    this: usize,
    out: usize,
    mat_in: usize,
) -> usize {
    let p1 = rd::<usize>(this + 0x8);
    let (atlas_h, atlas_w_i) = if p1 != 0 {
        let p2 = rd::<usize>(p1);
        let h_i = if p2 != 0 { rd::<i32>(p2 + 0x4) } else { 0 };
        let w_i = if p2 != 0 { rd::<i32>(p2) } else { 0 };
        (h_i as f32, w_i)
    } else {
        (0.0f32, 0i32)
    };
    let c_sub = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let c_add = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let h = rd::<i32>(this + 0x30) as f32 - c_sub;
    let w = rd::<i32>(this + 0x2c) as f32 - c_sub;
    let x = rd::<i32>(this + 0x24) as f32 + c_add;
    let y = rd::<i32>(this + 0x28) as f32 + c_add;
    let atlas_w = atlas_w_i as f32;
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4224_2790,
        out as u64,
        mat_in as u64,
        0,
        0,
        w.to_bits() as u64,
        h.to_bits() as u64,
        atlas_w.to_bits() as u64,
        atlas_h.to_bits() as u64,
        [0, 0, x.to_bits() as u128, y.to_bits() as u128],
    );
    out
}

pub unsafe fn diff_rend_shadow_map_depth_vfunc_6(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let dims = [2048i32, 1024i32];
    let atlas = [dims.as_ptr() as usize];
    let mut obj = [0u8; 0x38];
    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base + 0x8, atlas.as_ptr() as usize);
    wr::<i32>(base + 0x24, 32);
    wr::<i32>(base + 0x28, 64);
    wr::<i32>(base + 0x2c, 256);
    wr::<i32>(base + 0x30, 512);
    let mut mat_in = [0f32; 16];
    for (i, v) in mat_in.iter_mut().enumerate() {
        *v = (i as f32) * 0.5 + 1.0;
    }
    let mut out_o = [0u32; 16];
    let mut out_n = [0u32; 16];
    let r_o = f_orig(base, out_o.as_mut_ptr() as usize, mat_in.as_ptr() as usize);
    let r_n = f_new(base, out_n.as_mut_ptr() as usize, mat_in.as_ptr() as usize);
    crate::diff_eq!(r_o == out_o.as_mut_ptr() as usize, r_n == out_n.as_mut_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "depth_vfunc_6_mat");
    Ok(1)
}

/// `rend::ShadowMapDepth::vfunc_5`
/// Original: `0x142243720` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_vfunc_5(
    _this: usize,
    out: usize,
    src: usize,
) -> usize {
    wr::<[u8; 32]>(out, rd::<[u8; 32]>(src));
    wr::<[u8; 32]>(out + 0x20, rd::<[u8; 32]>(src + 0x20));
    out
}

pub unsafe fn diff_rend_shadow_map_depth_vfunc_5(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut src = [0u8; 64];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(3).wrapping_add(7);
    }
    let mut out_o = [0u8; 64];
    let mut out_n = [0u8; 64];
    let r_o = f_orig(0, out_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    let r_n = f_new(0, out_n.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(r_o == out_o.as_mut_ptr() as usize, r_n == out_n.as_mut_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "copy64");
    Ok(1)
}

/// `rend_1422437c0`
/// Original: `0x1422437c0` (39 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_depth_ctor(this: usize) -> usize {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_18d8) as u64);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    wr::<u32>(this + 0x30, 0);
    this
}

pub unsafe fn diff_rend_shadow_map_depth_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut o = [0xffu8; 0x38];
    let mut n = [0xffu8; 0x38];
    let r_o = f_orig(o.as_mut_ptr() as usize);
    let r_n = f_new(n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o == o.as_mut_ptr() as usize, r_n == n.as_mut_ptr() as usize, "ret");
    crate::diff_eq!(o, n, "ctor_bytes");
    Ok(1)
}

/// `rend::ShadowInterfaceColor::vfunc_7`
/// Original: `0x142243820` (116 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_interface_color_vfunc_7(this: usize) {
    let _ = tls_slot0();
    let dsv_holder = rd::<usize>(this + 0x40);
    let depth = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let dsv = rd::<usize>(dsv_holder);
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4206_ecc0,
        0,
        dsv as u64,
        0,
        0,
        2,
        0,
        0,
        0,
        [0, 0, depth.to_bits() as u128, 0],
    );

    let rtv_holder = rd::<usize>(this + 0x48);
    let mut color = rd::<[u8; 16]>(exe::ptr::<u8>(0x1_4342_8240) as usize);
    let rtv = rd::<usize>(rtv_holder);
    call_va!(0x1_4206_edb0, (0usize, rtv, color.as_mut_ptr() as usize) -> ());
}

pub unsafe fn diff_rend_shadow_interface_color_vfunc_7(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static DSV_REC: AtomicU64 = AtomicU64::new(0);
    static RTV_REC: AtomicU64 = AtomicU64::new(0);

    #[cfg(not(target_arch = "aarch64"))]
    unsafe extern "win64" fn fake_clear_dsv(
        _ctx: usize,
        dsv: usize,
        depth: f32,
        stencil: u8,
        flags: u32,
    ) {
        let tag = (dsv as u64)
            ^ ((depth.to_bits() as u64) << 16)
            ^ ((stencil as u64) << 48)
            ^ ((flags as u64) << 56);
        DSV_REC.store(tag, Ordering::SeqCst);
    }

    #[cfg(target_arch = "aarch64")]
    unsafe extern "win64" fn fake_clear_dsv(
        _ctx: usize,
        dsv: usize,
        _r8: usize,
        stencil: u8,
        flags: u32,
        _xmm0: f32,
        _xmm1: f32,
        depth: f32,
    ) {
        let tag = (dsv as u64)
            ^ ((depth.to_bits() as u64) << 16)
            ^ ((stencil as u64) << 48)
            ^ ((flags as u64) << 56);
        DSV_REC.store(tag, Ordering::SeqCst);
    }

    unsafe extern "win64" fn fake_clear_rtv(_ctx: usize, rtv: usize, color_ptr: usize) {
        let c0 = rd::<u64>(color_ptr);
        let c1 = rd::<u64>(color_ptr + 0x8);
        RTV_REC.store((rtv as u64) ^ c0 ^ c1.rotate_left(13), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_ecc0, fake_clear_dsv as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_edb0, fake_clear_rtv as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let orig_tls0 = rd::<usize>(exe::tls_pointer());
    let mut tls_buf = [0u8; 0x80];
    tls_buf[0x70] = 1;
    wr::<usize>(exe::tls_pointer(), tls_buf.as_mut_ptr() as usize);

    let dsv_holder = [0x1111_2222usize];
    let rtv_holder = [0x3333_4444usize];
    let mut obj = [0u8; 0x58];
    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base + 0x40, dsv_holder.as_ptr() as usize);
    wr::<usize>(base + 0x48, rtv_holder.as_ptr() as usize);

    DSV_REC.store(0, Ordering::SeqCst);
    RTV_REC.store(0, Ordering::SeqCst);
    f_orig(base);
    let dsv_o = DSV_REC.load(Ordering::SeqCst);
    let rtv_o = RTV_REC.load(Ordering::SeqCst);

    DSV_REC.store(0, Ordering::SeqCst);
    RTV_REC.store(0, Ordering::SeqCst);
    f_new(base);
    let dsv_n = DSV_REC.load(Ordering::SeqCst);
    let rtv_n = RTV_REC.load(Ordering::SeqCst);

    wr::<usize>(exe::tls_pointer(), orig_tls0);

    crate::diff_eq!(dsv_o, dsv_n, "clear_dsv");
    crate::diff_eq!(rtv_o, rtv_n, "clear_rtv");
    Ok(1)
}

/// `rend::ShadowInterfaceColor::vfunc_4`
/// Original: `0x1422438e0` (65 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_interface_color_vfunc_4(this: usize) {
    let packer = rd::<usize>(this + 0x50);
    wr::<u32>(packer + 0x18, 0);
    let mut slot = this + 0x58;
    for _ in 0..0x200usize {
        let p0 = rd::<usize>(slot);
        wr::<u32>(p0 + 0x28, 0xffff_ffff);
        let p1 = rd::<usize>(slot + 0x8);
        wr::<u32>(p1 + 0x28, 0xffff_ffff);
        slot = slot.wrapping_add(0x10);
    }
}

pub unsafe fn diff_rend_shadow_interface_color_vfunc_4(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut packer_o = [0u32; 8];
    packer_o[6] = 123;
    let mut item_o = [0u8; 0x30];
    wr::<u32>((item_o.as_mut_ptr() as usize) + 0x28, 42);
    let mut obj_o = vec![0u64; 0x410];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o + 0x50, packer_o.as_mut_ptr() as usize);
    for i in 0..0x400usize {
        wr::<usize>(base_o + 0x58 + i * 8, item_o.as_mut_ptr() as usize);
    }

    let mut packer_n = [0u32; 8];
    packer_n[6] = 123;
    let mut item_n = [0u8; 0x30];
    wr::<u32>((item_n.as_mut_ptr() as usize) + 0x28, 42);
    let mut obj_n = vec![0u64; 0x410];
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<usize>(base_n + 0x50, packer_n.as_mut_ptr() as usize);
    for i in 0..0x400usize {
        wr::<usize>(base_n + 0x58 + i * 8, item_n.as_mut_ptr() as usize);
    }

    f_orig(base_o);
    f_new(base_n);
    crate::diff_eq!(packer_o, packer_n, "packer");
    crate::diff_eq!(item_o, item_n, "item");
    Ok(1)
}

/// `rend::ShadowInterfaceColor::vfunc_3`
/// Original: `0x142243930` (195 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_shadow_interface_color_vfunc_3(this: usize) {
    let packer = rd::<usize>(this + 0x50);
    let count = rd::<u32>(packer + 0x18);
    if count != 0 {
        let mut out_a = 0u64;
        let mut out_b = 0u64;
        call_va!(
            0x1_4224_45b0,
            (
                rd::<usize>(packer + 0x10),
                count,
                rd::<u32>(packer + 0x8),
                rd::<u32>(packer + 0x4),
                rd::<u32>(packer),
                (&mut out_b as *mut u64) as usize,
                (&mut out_a as *mut u64) as usize,
            ) -> ()
        );
    }
    let mut slot = this + 0x58;
    for _ in 0..0x400usize {
        let entry = rd::<usize>(slot);
        let idx = rd::<i32>(entry + 0x28);
        if idx >= 0 {
            let pkr = rd::<usize>(this + 0x50);
            let rects = rd::<usize>(pkr + 0x10);
            let rect = rects.wrapping_add((idx as isize as usize).wrapping_mul(40));
            wr::<u32>(entry + 0x3c, rd::<u32>(rect));
            wr::<u32>(entry + 0x40, rd::<u32>(rect + 0x4));
            wr::<u32>(entry + 0x44, rd::<u32>(rect + 0x8));
            wr::<u32>(entry + 0x48, rd::<u32>(rect + 0xc));
        }
        slot = slot.wrapping_add(8);
    }
    let vtbl = rd::<usize>(this);
    let fn_38 = rd::<usize>(vtbl + 0x38);
    call_va!(fn_38, (this) -> ());
    let idx = rd::<i32>(this + 0x30) as isize;
    let tbl = exe::ptr::<u8>(0x1_43a6_37b8) as usize;
    let size = rd::<i32>(tbl.wrapping_add((idx as usize).wrapping_mul(16))) as i64 as u64;
    let src = rd::<usize>(this + 0x28);
    call_va!(0x1_4209_a900, (idx as i64 as u64, 0u32, size, src) -> ());
}

pub unsafe fn diff_rend_shadow_interface_color_vfunc_3(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static PACK_REC: AtomicU64 = AtomicU64::new(0);
    static UPLOAD_REC: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_pack(
        rects: usize,
        count: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        out_b: usize,
        out_a: usize,
    ) {
        wr::<u64>(out_b, 0xaa);
        wr::<u64>(out_a, 0xbb);
        let tag = (rects as u64)
            ^ ((count as u64) << 8)
            ^ ((a3 as u64) << 16)
            ^ ((a4 as u64) << 24)
            ^ ((a5 as u64) << 32);
        PACK_REC.store(tag, Ordering::SeqCst);
    }

    unsafe extern "win64" fn fake_upload(idx: u64, zero: u32, size: u64, src: usize) {
        UPLOAD_REC.store(idx ^ ((zero as u64) << 8) ^ (size << 16) ^ (src as u64), Ordering::SeqCst);
    }

    unsafe extern "win64" fn fake_vfunc38(this: usize) {
        wr::<u32>(this + 0x34, rd::<u32>(this + 0x34).wrapping_add(0x2468));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_45b0, fake_pack as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_a900, fake_upload as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 8];
    vtbl[7] = fake_vfunc38 as *const () as usize;

    let mut rects = [0u32; 10];
    rects[0] = 11;
    rects[1] = 22;
    rects[2] = 333;
    rects[3] = 444;

    let mut packer = [0u8; 0x20];
    let pkr = packer.as_mut_ptr() as usize;
    wr::<u32>(pkr, 1024);
    wr::<u32>(pkr + 0x4, 2048);
    wr::<u32>(pkr + 0x8, 512);
    wr::<usize>(pkr + 0x10, rects.as_ptr() as usize);
    wr::<u32>(pkr + 0x18, 1);

    let mut entry0_o = [0u8; 0x50];
    let mut entry0_n = [0u8; 0x50];
    wr::<i32>((entry0_o.as_mut_ptr() as usize) + 0x28, 0);
    wr::<i32>((entry0_n.as_mut_ptr() as usize) + 0x28, 0);
    let mut inactive = [0u8; 0x50];
    wr::<i32>((inactive.as_mut_ptr() as usize) + 0x28, -1);

    let mut obj_o = vec![0u64; 0x410];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o, vtbl.as_ptr() as usize);
    wr::<usize>(base_o + 0x28, 0x8888_0000);
    wr::<i32>(base_o + 0x30, 0);
    wr::<usize>(base_o + 0x50, pkr);
    wr::<usize>(base_o + 0x58, entry0_o.as_mut_ptr() as usize);
    for i in 1..0x400usize {
        wr::<usize>(base_o + 0x58 + i * 8, inactive.as_ptr() as usize);
    }

    let mut obj_n = obj_o.clone();
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<usize>(base_n + 0x58, entry0_n.as_mut_ptr() as usize);

    PACK_REC.store(0, Ordering::SeqCst);
    UPLOAD_REC.store(0, Ordering::SeqCst);
    f_orig(base_o);
    let pack_o = PACK_REC.load(Ordering::SeqCst);
    let up_o = UPLOAD_REC.load(Ordering::SeqCst);

    PACK_REC.store(0, Ordering::SeqCst);
    UPLOAD_REC.store(0, Ordering::SeqCst);
    f_new(base_n);
    let pack_n = PACK_REC.load(Ordering::SeqCst);
    let up_n = UPLOAD_REC.load(Ordering::SeqCst);

    crate::diff_eq!(entry0_o, entry0_n, "entry0");
    crate::diff_eq!(rd::<u32>(base_o + 0x34), rd::<u32>(base_n + 0x34), "vfunc38");
    crate::diff_eq!(pack_o, pack_n, "pack");
    crate::diff_eq!(up_o, up_n, "upload");
    Ok(1)
}

/// `rend::ShadowInterfaceColor::vfunc_2`
/// Original: `0x142243a00` (244 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_interface_color_vfunc_2(
    this: usize,
    w: u32,
    h: u32,
    p4: u32,
    p5: u32,
    p6: u32,
) -> usize {
    let lock = rd::<usize>(this + 0x8);
    srwlock_enter_recursive(lock);
    let mut chosen = 0usize;
    let mut slot = this + 0x58;
    let mut found = 0usize;
    for _ in 0..0x400usize {
        let cand = rd::<usize>(slot);
        if rd::<i32>(cand + 0x28) < 0 {
            found = cand;
            break;
        }
        slot = slot.wrapping_add(8);
    }
    if found != 0 {
        let packer = rd::<usize>(this + 0x50);
        let idx = call_va!(0x1_4224_4fa0, (packer, w, h, p4, p5, p6) -> u32);
        wr::<u32>(found + 0x28, idx);
        chosen = found;
        wr::<u64>(found + 0x10, rd::<u64>(this + 0x48));
        wr::<u64>(found + 0x18, rd::<u64>(this + 0x40));
        wr::<u8>(found + 0x8, rd::<u8>(this + 0x18));
        wr::<u8>(found + 0x9, 9);
    }
    srwlock_leave_recursive(lock);
    chosen
}

pub unsafe fn diff_rend_shadow_interface_color_vfunc_2(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32, u32, u32, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut lock_o = [0u64, 0xffff_ffffu64];
    let mut rects_o = [0u8; 80];
    let mut packer_o = [0u8; 0x20];
    let pkr_o = packer_o.as_mut_ptr() as usize;
    wr::<u64>(pkr_o + 0x10, rects_o.as_mut_ptr() as u64);
    wr::<u32>(pkr_o + 0x18, 0);
    wr::<u32>(pkr_o + 0x1c, 2);
    let mut entry_o = [0u8; 0x50];
    wr::<i32>((entry_o.as_mut_ptr() as usize) + 0x28, -1);
    let mut obj_o = vec![0u64; 0x410];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o + 0x8, lock_o.as_mut_ptr() as usize);
    wr::<u8>(base_o + 0x18, 5);
    wr::<u64>(base_o + 0x40, 0xaaaa_bbbb);
    wr::<u64>(base_o + 0x48, 0xcccc_dddd);
    wr::<usize>(base_o + 0x50, pkr_o);
    wr::<usize>(base_o + 0x58, entry_o.as_mut_ptr() as usize);

    let mut lock_n = [0u64, 0xffff_ffffu64];
    let mut rects_n = [0u8; 80];
    let mut packer_n = [0u8; 0x20];
    let pkr_n = packer_n.as_mut_ptr() as usize;
    wr::<u64>(pkr_n + 0x10, rects_n.as_mut_ptr() as u64);
    wr::<u32>(pkr_n + 0x18, 0);
    wr::<u32>(pkr_n + 0x1c, 2);
    let mut entry_n = [0u8; 0x50];
    wr::<i32>((entry_n.as_mut_ptr() as usize) + 0x28, -1);
    let mut obj_n = vec![0u64; 0x410];
    let base_n = obj_n.as_mut_ptr() as usize;
    wr::<usize>(base_n + 0x8, lock_n.as_mut_ptr() as usize);
    wr::<u8>(base_n + 0x18, 5);
    wr::<u64>(base_n + 0x40, 0xaaaa_bbbb);
    wr::<u64>(base_n + 0x48, 0xcccc_dddd);
    wr::<usize>(base_n + 0x50, pkr_n);
    wr::<usize>(base_n + 0x58, entry_n.as_mut_ptr() as usize);

    let r_o = f_orig(base_o, 128, 256, 2, 0x3f00_0000, 99);
    let r_n = f_new(base_n, 128, 256, 2, 0x3f00_0000, 99);
    crate::diff_eq!(
        r_o == entry_o.as_mut_ptr() as usize,
        r_n == entry_n.as_mut_ptr() as usize,
        "ret_entry"
    );
    crate::diff_eq!(entry_o, entry_n, "entry_state");
    crate::diff_eq!(&rects_o[..16], &rects_n[..16], "rects_head");
    crate::diff_eq!(&rects_o[24..40], &rects_n[24..40], "rects_tail");
    crate::diff_eq!(packer_o[0x18..0x20], packer_n[0x18..0x20], "packer_counts");
    Ok(1)
}

/// `rend::ShadowInterfaceColor::vfunc_1`
/// Original: `0x142243b00` (54 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_shadow_interface_color_vfunc_1(this: usize, res: usize) {
    let val = rd::<u64>(res);
    let pkr0 = rd::<usize>(this + 0x50);
    wr::<u64>(this + 0x10, val);
    wr::<u32>(pkr0 + 0x4, rd::<u32>(res));
    let pkr1 = rd::<usize>(this + 0x50);
    wr::<u32>(pkr1, rd::<u32>(res + 0x4));
    call_va!(0x1_4224_3b40, (this) -> ());
    call_va!(0x1_4224_3c20, (this) -> ());
}

pub unsafe fn diff_rend_shadow_interface_color_vfunc_1(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_rel(this: usize) {
        wr::<u32>(this + 0x34, 0x11);
        SEQ.fetch_add(1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_init(this: usize) {
        wr::<u32>(this + 0x38, 0x22);
        SEQ.fetch_add(10, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_3b40, fake_rel as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4224_3c20, fake_init as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let res = [1024u32, 2048u32];
    let mut pkr_o = [0u32; 4];
    let mut pkr_n = [0u32; 4];
    let mut obj_o = [0u8; 0x60];
    let mut obj_n = [0u8; 0x60];
    wr::<usize>((obj_o.as_mut_ptr() as usize) + 0x50, pkr_o.as_mut_ptr() as usize);
    wr::<usize>((obj_n.as_mut_ptr() as usize) + 0x50, pkr_n.as_mut_ptr() as usize);

    SEQ.store(0, Ordering::SeqCst);
    f_orig(obj_o.as_mut_ptr() as usize, res.as_ptr() as usize);
    let seq_o = SEQ.load(Ordering::SeqCst);

    SEQ.store(0, Ordering::SeqCst);
    f_new(obj_n.as_mut_ptr() as usize, res.as_ptr() as usize);
    let seq_n = SEQ.load(Ordering::SeqCst);

    crate::diff_eq!(pkr_o, pkr_n, "packer");
    crate::diff_eq!(obj_o[0..0x48], obj_n[0..0x48], "obj_head");
    crate::diff_eq!(seq_o, seq_n, "seq");
    Ok(1)
}

/// `rend_142243b40`
/// Original: `0x142243b40` (205 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_shadow_interface_color_release_atlas(this: usize) {
    let rtv = rd::<usize>(this + 0x48);
    wr::<u64>(this + 0x48, 0);
    if rtv != 0 {
        if let Err(e) = exe::try_call(exe::ptr::<u8>(0x1_4204_f860) as usize, &[rtv as u64], &[]) {
            call_va!(0x1_4244_2de0, (rtv) -> ());
            e.rethrow();
        }
        call_va!(0x1_4244_2de0, (rtv) -> ());
    }
    let dsv = rd::<usize>(this + 0x40);
    wr::<u64>(this + 0x40, 0);
    if dsv != 0 {
        let tex = rd::<usize>(dsv);
        if tex != 0 {
            if let Err(e) = exe::try_call(exe::ptr::<u8>(0x1_4206_b020) as usize, &[tex as u64], &[]) {
                call_va!(0x1_4244_2de0, (tex) -> ());
                call_va!(0x1_4244_2de0, (dsv) -> ());
                e.rethrow();
            }
            call_va!(0x1_4244_2de0, (tex) -> ());
        }
        call_va!(0x1_4244_2de0, (dsv) -> ());
    }
    let mut slot = this + 0x58;
    for _ in 0..0x200usize {
        let p0 = rd::<usize>(slot);
        wr::<u64>(p0 + 0x18, 0);
        let p0_b = rd::<usize>(slot);
        wr::<u64>(p0_b + 0x10, 0);
        let p1 = rd::<usize>(slot + 0x8);
        wr::<u64>(p1 + 0x18, 0);
        let p1_b = rd::<usize>(slot + 0x8);
        wr::<u64>(p1_b + 0x10, 0);
        slot = slot.wrapping_add(0x10);
    }
    let idx = rd::<i32>(this + 0x30) as isize;
    let tbl = exe::ptr::<u8>(0x1_43a6_37b8) as usize;
    let size = rd::<i32>(tbl.wrapping_add((idx as usize).wrapping_mul(16))) as i64 as u64;
    let src = rd::<usize>(this + 0x28);
    call_va!(0x1_4209_a900, (idx as i64 as u64, 0u32, size, src) -> ());
}

pub unsafe fn diff_rend_shadow_interface_color_release_atlas(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_dtor_rtv(_p: usize) {
        TRACE.fetch_add(1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_dtor_tex(_p: usize) {
        TRACE.fetch_add(10, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_free(_p: usize) {
        TRACE.fetch_add(100, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_upload(idx: u64, zero: u32, size: u64, src: usize) {
        TRACE.fetch_add(1000 + idx + (zero as u64) + size + (src as u64), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_f860, fake_dtor_rtv as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_b020, fake_dtor_tex as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_2de0, fake_free as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_a900, fake_upload as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dsv = [0x9999_1111usize];
    let mut entry_o = [0u64, 0u64, 0xaaaa_bbbbu64, 0xcccc_ddddu64];
    let mut entry_n = entry_o;

    let mut obj_o = vec![0u64; 0x410];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o + 0x28, 0x4444_0000);
    wr::<i32>(base_o + 0x30, 0);
    wr::<usize>(base_o + 0x40, dsv.as_ptr() as usize);
    wr::<usize>(base_o + 0x48, 0x8888_2222);
    for i in 0..0x400usize {
        wr::<usize>(base_o + 0x58 + i * 8, entry_o.as_mut_ptr() as usize);
    }

    let mut obj_n = obj_o.clone();
    let base_n = obj_n.as_mut_ptr() as usize;
    for i in 0..0x400usize {
        wr::<usize>(base_n + 0x58 + i * 8, entry_n.as_mut_ptr() as usize);
    }

    TRACE.store(0, Ordering::SeqCst);
    f_orig(base_o);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(base_n);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(rd::<u64>(base_o + 0x40), rd::<u64>(base_n + 0x40), "dsv_cleared");
    crate::diff_eq!(rd::<u64>(base_o + 0x48), rd::<u64>(base_n + 0x48), "rtv_cleared");
    crate::diff_eq!(entry_o, entry_n, "entry_cleared");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

/// `rend_142243e40`
/// Original: `0x142243e40` (253 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_shadow_interface_color_dtor(this: usize) {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_19a8) as u64);
    call_va!(0x1_4224_3b40, (this) -> ());
    let mut slot = this + 0x58;
    for _ in 0..0x400usize {
        let entry = rd::<usize>(slot);
        if entry != 0 {
            let vtbl = rd::<usize>(entry);
            let dtor = rd::<usize>(vtbl);
            call_va!(dtor, (entry, 0u32) -> ());
            call_va!(0x1_4244_2de0, (entry) -> ());
        }
        wr::<u64>(slot, 0);
        slot = slot.wrapping_add(8);
    }
    let packer = rd::<usize>(this + 0x50);
    if packer != 0 {
        call_va!(0x1_401d_e830, (packer + 0x10) -> ());
        call_va!(0x1_4244_2de0, (packer) -> ());
    }
    let rtv = rd::<usize>(this + 0x48);
    if rtv != 0 {
        call_va!(0x1_4204_f860, (rtv) -> ());
        call_va!(0x1_4244_2de0, (rtv) -> ());
    }
    let dsv = rd::<usize>(this + 0x40);
    if dsv != 0 {
        let tex = rd::<usize>(dsv);
        if tex != 0 {
            call_va!(0x1_4206_b020, (tex) -> ());
            call_va!(0x1_4244_2de0, (tex) -> ());
        }
        call_va!(0x1_4244_2de0, (dsv) -> ());
    }
    call_va!(0x1_4220_1e30, (this + 0x28) -> ());
    let lock = rd::<usize>(this + 0x8);
    if lock != 0 {
        call_va!(0x1_4244_2de0, (lock) -> ());
    }
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_1818) as u64);
}

pub unsafe fn diff_rend_shadow_interface_color_dtor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static DTOR_TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_rel_atlas(_this: usize) {
        DTOR_TRACE.fetch_add(1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_entry_dtor(_entry: usize, flags: u32) {
        DTOR_TRACE.fetch_add(10 + (flags as u64) * 1000, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_free(_p: usize) {
        DTOR_TRACE.fetch_add(100, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_rel_buf(_p: usize) {
        DTOR_TRACE.fetch_add(1000, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_dtor_rtv(_p: usize) {
        DTOR_TRACE.fetch_add(10000, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_dtor_tex(_p: usize) {
        DTOR_TRACE.fetch_add(100000, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_rel_28(_p: usize) {
        DTOR_TRACE.fetch_add(1000000, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_3b40, fake_rel_atlas as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, fake_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_e830, fake_rel_buf as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4204_f860, fake_dtor_rtv as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4206_b020, fake_dtor_tex as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4220_1e30, fake_rel_28 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let entry_vtbl = [fake_entry_dtor as *const () as usize];
    let entry0 = [entry_vtbl.as_ptr() as usize];
    let dsv = [0x1111_2222usize];

    let mut obj_o = vec![0u64; 0x410];
    let base_o = obj_o.as_mut_ptr() as usize;
    wr::<usize>(base_o + 0x8, 0x3333_4444);
    wr::<usize>(base_o + 0x40, dsv.as_ptr() as usize);
    wr::<usize>(base_o + 0x48, 0x5555_6666);
    wr::<usize>(base_o + 0x50, 0x7777_8888);
    wr::<usize>(base_o + 0x58, entry0.as_ptr() as usize);

    let mut obj_n = obj_o.clone();
    let base_n = obj_n.as_mut_ptr() as usize;

    DTOR_TRACE.store(0, Ordering::SeqCst);
    f_orig(base_o);
    let tr_o = DTOR_TRACE.load(Ordering::SeqCst);

    DTOR_TRACE.store(0, Ordering::SeqCst);
    f_new(base_n);
    let tr_n = DTOR_TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(obj_o, obj_n, "obj_after_dtor");
    crate::diff_eq!(tr_o, tr_n, "dtor_trace");
    Ok(1)
}

/// `rend_142243f50`
/// Original: `0x142243f50` (379 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_shadow_interface_color_ctor(
    this: usize,
    param_2: u64,
    param_3: u32,
    fmt: u8,
) -> usize {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_19a8) as u64);
    call_va!(0x1_412f_2120, (this + 0x8) -> ());
    wr::<u64>(this + 0x10, 0);
    let name_res = exe::ptr::<u8>(0x1_433f_4948) as usize;
    if let Err(e) = exe::try_call(
        exe::ptr::<u8>(0x1_412c_f0e0) as usize,
        &[(this + 0x28) as u64, name_res as u64],
        &[],
    ) {
        let lock = rd::<usize>(this + 0x8);
        if lock != 0 {
            call_va!(0x1_4244_2de0, (lock) -> ());
        }
        e.rethrow();
    }
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    let def_res = rd::<u64>(exe::ptr::<u64>(0x1_4386_9ff8) as usize);
    wr::<u64>(this + 0x10, def_res);
    wr::<u8>(this + 0x18, fmt);
    wr::<u64>(this + 0x20, exe::ptr::<u8>(0x1_433f_4938) as u64);

    let mut local_p2 = param_2;
    let mut local_p3 = param_3;
    let p2_ptr = (&mut local_p2 as *mut u64) as usize;
    let p3_ptr = (&mut local_p3 as *mut u32) as usize;
    let ret_slot = call_va!(
        0x1_412e_6ac0,
        (p2_ptr, 0usize, this + 0x10, this + 0x14, p3_ptr) -> usize
    );
    let new_pkr = rd::<u64>(ret_slot);
    wr::<u64>(ret_slot, 0);
    let old_pkr = rd::<usize>(this + 0x50);
    wr::<u64>(this + 0x50, new_pkr);
    if old_pkr != 0 {
        call_va!(0x1_401d_e830, (old_pkr + 0x10) -> ());
        call_va!(0x1_4244_2de0, (old_pkr) -> ());
    }
    if local_p2 != 0 {
        let tmp_pkr = local_p2 as usize;
        call_va!(0x1_401d_e830, (tmp_pkr + 0x10) -> ());
        call_va!(0x1_4244_2de0, (tmp_pkr) -> ());
    }
    let mut slot = this + 0x58;
    for _ in 0..0x400usize {
        let item = call_va!(0x1_412f_1f50, () -> u64);
        wr::<u64>(slot, item);
        slot = slot.wrapping_add(8);
    }
    if let Err(e) = exe::try_call(exe::ptr::<u8>(0x1_4224_3c20) as usize, &[this as u64], &[]) {
        let pkr = rd::<usize>(this + 0x50);
        if pkr != 0 {
            call_va!(0x1_401d_e830, (pkr + 0x10) -> ());
            call_va!(0x1_4244_2de0, (pkr) -> ());
        }
        call_va!(0x1_4220_1e30, (this + 0x28) -> ());
        let lock = rd::<usize>(this + 0x8);
        if lock != 0 {
            call_va!(0x1_4244_2de0, (lock) -> ());
        }
        e.rethrow();
    }
    this
}

pub unsafe fn diff_rend_shadow_interface_color_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ITEM_COUNTER: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_init_lock(dst: usize) {
        wr::<u64>(dst, 0xaa00_bb00);
    }
    unsafe extern "win64" fn fake_init_28(dst: usize, name: usize) {
        wr::<usize>(dst, name);
        wr::<u64>(dst + 0x8, 0x1234);
        wr::<u64>(dst + 0x10, 0x5678);
    }
    unsafe extern "win64" fn fake_make_packer(
        out_ptr: usize,
        _unused: usize,
        w_ptr: usize,
        h_ptr: usize,
        p3_ptr: usize,
    ) -> usize {
        let v = (rd::<u32>(w_ptr) as u64)
            ^ ((rd::<u32>(h_ptr) as u64) << 16)
            ^ ((rd::<u32>(p3_ptr) as u64) << 32)
            ^ 0x9000_0000;
        wr::<u64>(out_ptr, v);
        out_ptr
    }
    unsafe extern "win64" fn fake_make_item() -> u64 {
        ITEM_COUNTER.fetch_add(1, Ordering::SeqCst) + 0x1000
    }
    unsafe extern "win64" fn fake_init_atlas(this: usize) {
        wr::<u64>(this + 0x40, 0xd570);
        wr::<u64>(this + 0x48, 0x4770);
    }

    let _m1 = crate::difftest::Mock::new(0x1_412f_2120, fake_init_lock as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412c_f0e0, fake_init_28 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412e_6ac0, fake_make_packer as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412f_1f50, fake_make_item as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4224_3c20, fake_init_atlas as *const ())?;

    type F = unsafe extern "win64" fn(usize, u64, u32, u8) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = vec![0x5au8; 0x2060];
    let mut obj_n = vec![0x5au8; 0x2060];

    ITEM_COUNTER.store(0, Ordering::SeqCst);
    let r_o = f_orig(obj_o.as_mut_ptr() as usize, 0, 512, 7);

    ITEM_COUNTER.store(0, Ordering::SeqCst);
    let r_n = f_new(obj_n.as_mut_ptr() as usize, 0, 512, 7);

    crate::diff_eq!(
        r_o == obj_o.as_mut_ptr() as usize,
        r_n == obj_n.as_mut_ptr() as usize,
        "ret_this"
    );
    crate::diff_eq!(obj_o, obj_n, "ctor_bytes");
    Ok(1)
}

/// `rend::ShadowMapColor::vfunc_11`
/// Original: `0x1422442e0` (95 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_color_vfunc_11(this: usize) -> u8 {
    let r10 = rd::<usize>(this + 0x10);
    if r10 == 0 {
        return 0;
    }
    let rcx = rd::<usize>(this + 0x18);
    if rcx == 0 {
        return 0;
    }
    let right = rd::<i32>(this + 0x44).wrapping_add(rd::<i32>(this + 0x3c));
    if right > rd::<i32>(r10 + 0x8) {
        return 0;
    }
    let rdx = rd::<usize>(rcx);
    let dsv_w = if rdx != 0 { rd::<i32>(rdx) } else { 0 };
    if right > dsv_w {
        return 0;
    }
    let bottom = rd::<i32>(this + 0x40).wrapping_add(rd::<i32>(this + 0x48));
    if bottom > rd::<i32>(r10 + 0xc) {
        return 0;
    }
    let dsv_h = if rdx != 0 { rd::<i32>(rdx + 0x4) } else { 0 };
    if bottom > dsv_h {
        return 0;
    }
    1
}

pub unsafe fn diff_rend_shadow_map_color_vfunc_11(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let rtv = [0i32, 0, 1024, 1024];
    let dsv_dims = [1024i32, 1024i32];
    let dsv = [dsv_dims.as_ptr() as usize];
    let cases: [(usize, usize, i32, i32, i32, i32); 5] = [
        (0, dsv.as_ptr() as usize, 0, 0, 128, 128),
        (rtv.as_ptr() as usize, 0, 0, 0, 128, 128),
        (rtv.as_ptr() as usize, dsv.as_ptr() as usize, 64, 64, 256, 256),
        (rtv.as_ptr() as usize, dsv.as_ptr() as usize, 900, 64, 256, 256),
        (rtv.as_ptr() as usize, dsv.as_ptr() as usize, 64, 900, 256, 256),
    ];
    for (p10, p18, x, y, w, h) in cases {
        let mut obj = [0u8; 0x50];
        let base = obj.as_mut_ptr() as usize;
        wr::<usize>(base + 0x10, p10);
        wr::<usize>(base + 0x18, p18);
        wr::<i32>(base + 0x3c, x);
        wr::<i32>(base + 0x40, y);
        wr::<i32>(base + 0x44, w);
        wr::<i32>(base + 0x48, h);
        crate::diff_eq!(f_orig(base), f_new(base), "color_vfunc_11");
    }
    Ok(5)
}

/// `rend::ShadowMapColor::vfunc_7`
/// Original: `0x1422443b0` (130 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_color_vfunc_7(
    this: usize,
    out_x: usize,
    out_y: usize,
    out_w: usize,
    out_h: usize,
) {
    let rtv = rd::<usize>(this + 0x10);
    let c_add = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let c_sub = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let x_num = rd::<i32>(this + 0x3c) as f32 + c_add;
    let atlas_w = rd::<i32>(rtv + 0x8) as f32;
    let x = x_num / atlas_w;
    let atlas_h = rd::<i32>(rtv + 0xc) as f32;
    wr::<f32>(out_x, x);
    let y = (rd::<i32>(this + 0x40) as f32 + c_add) / atlas_h;
    wr::<f32>(out_y, y);
    let w = (rd::<i32>(this + 0x44) as f32 - c_sub) / atlas_w;
    wr::<f32>(out_w, w);
    let h = (rd::<i32>(this + 0x48) as f32 - c_sub) / atlas_h;
    wr::<f32>(out_h, h);
}

pub unsafe fn diff_rend_shadow_map_color_vfunc_7(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let rtv = [0i32, 0, 2048, 1024];
    let mut obj = [0u8; 0x50];
    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base + 0x10, rtv.as_ptr() as usize);
    wr::<i32>(base + 0x3c, 16);
    wr::<i32>(base + 0x40, 32);
    wr::<i32>(base + 0x44, 512);
    wr::<i32>(base + 0x48, 256);
    let mut o = [0f32; 4];
    let mut n = [0f32; 4];
    let op = o.as_mut_ptr() as usize;
    let np = n.as_mut_ptr() as usize;
    f_orig(base, op, op + 4, op + 8, op + 12);
    f_new(base, np, np + 4, np + 8, np + 12);
    for i in 0..4 {
        crate::diff_eq!(o[i].to_bits(), n[i].to_bits(), "color_vfunc_7");
    }
    Ok(1)
}

/// `rend::ShadowMapColor::vfunc_5`
/// Original: `0x142244470` (167 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shadow_map_color_vfunc_5(
    this: usize,
    out: usize,
    mat_in: usize,
) -> usize {
    let rtv = rd::<usize>(this + 0x10);
    let c_sub = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
    let c_add = rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let h = rd::<i32>(this + 0x48) as f32 - c_sub;
    let w = rd::<i32>(this + 0x44) as f32 - c_sub;
    let atlas_h = rd::<i32>(rtv + 0xc) as f32;
    let atlas_w = rd::<i32>(rtv + 0x8) as f32;
    let x = rd::<i32>(this + 0x3c) as f32 + c_add;
    let y = rd::<i32>(this + 0x40) as f32 + c_add;
    let _ = crate::simd_ops::call_va8_xmm(
        0x1_4224_2790,
        out as u64,
        mat_in as u64,
        0,
        0,
        w.to_bits() as u64,
        h.to_bits() as u64,
        atlas_w.to_bits() as u64,
        atlas_h.to_bits() as u64,
        [0, 0, x.to_bits() as u128, y.to_bits() as u128],
    );
    out
}

pub unsafe fn diff_rend_shadow_map_color_vfunc_5(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let rtv = [0i32, 0, 2048, 1024];
    let mut obj = [0u8; 0x50];
    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base + 0x10, rtv.as_ptr() as usize);
    wr::<i32>(base + 0x3c, 64);
    wr::<i32>(base + 0x40, 128);
    wr::<i32>(base + 0x44, 512);
    wr::<i32>(base + 0x48, 256);
    let mut mat_in = [0f32; 16];
    for (i, v) in mat_in.iter_mut().enumerate() {
        *v = (i as f32) * 0.25 + 0.5;
    }
    let mut out_o = [0u32; 16];
    let mut out_n = [0u32; 16];
    let r_o = f_orig(base, out_o.as_mut_ptr() as usize, mat_in.as_ptr() as usize);
    let r_n = f_new(base, out_n.as_mut_ptr() as usize, mat_in.as_ptr() as usize);
    crate::diff_eq!(r_o == out_o.as_mut_ptr() as usize, r_n == out_n.as_mut_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "color_vfunc_5_mat");
    Ok(1)
}

/// `rend::ShaderResource::vfunc_13`
/// Original: `0x14224be10` (58 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shader_resource_vfunc_13(this: usize) {
    let handle = rd::<usize>(this + 0xb8);
    wr::<u64>(this + 0xb8, 0);
    if handle != 0 {
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        call_va!(0x1_412f_4210, (mgr, handle) -> ());
    }
    call_va!(0x1_4242_87f0, (this) -> ());
}

pub unsafe fn diff_rend_shader_resource_vfunc_13(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut o = [0u8; 0xc8];
    let mut n = [0u8; 0xc8];
    wr::<u32>((o.as_mut_ptr() as usize) + 0x80, 4);
    wr::<u32>((n.as_mut_ptr() as usize) + 0x80, 4);
    f_orig(o.as_mut_ptr() as usize);
    f_new(n.as_mut_ptr() as usize);
    crate::diff_eq!(o, n, "shader_res_vfunc_13");
    Ok(1)
}

/// `rend_14224bef0`
/// Original: `0x14224bef0` (155 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_shader_resource_load_from_stream(
    this: usize,
    file: u64,
) -> u32 {
    let ctx = call_va!(0x1_412f_3280, () -> u64);
    let mut stream = [exe::ptr::<u8>(0x1_4340_e9e0) as u64, file];
    let stream_ptr = stream.as_mut_ptr() as usize;
    if let Ok(ret) = exe::try_call(
        exe::ptr::<u8>(0x1_423c_1970) as usize,
        &[stream_ptr as u64, ctx],
        &[exe::cxx_type::OBJECT_STREAM_ERRORS],
    ) {
        if (ret.rax as u8) != 0 {
            let mut pair = [this as u64, ctx];
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
            call_va!(0x1_412f_3f20, (mgr, pair.as_mut_ptr() as usize) -> ());
        }
    }
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_412f_40a0, (mgr, ctx) -> ());
    crate::simd_ops::atomic_xchg_u32((this + 0x80) as u64, 4) // xchg dword [rbx+0x80], eax
}

pub unsafe fn diff_rend_shader_resource_load_from_stream(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static LOAD_TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_alloc_ctx() -> u64 {
        0x7777_8888
    }
    unsafe extern "win64" fn fake_parse(stream: usize, ctx: u64) -> u8 {
        let vtbl = rd::<u64>(stream);
        let file = rd::<u64>(stream + 0x8);
        LOAD_TRACE.fetch_add(vtbl ^ file ^ ctx, Ordering::SeqCst);
        1
    }
    unsafe extern "win64" fn fake_register(mgr: usize, pair: usize) {
        let ctx = rd::<u64>(pair + 0x8);
        LOAD_TRACE.fetch_add((mgr as u64) ^ ctx.rotate_left(7), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_release_ctx(mgr: usize, ctx: u64) {
        LOAD_TRACE.fetch_add((mgr as u64) ^ ctx.rotate_left(19), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_412f_3280, fake_alloc_ctx as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_423c_1970, fake_parse as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_3f20, fake_register as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412f_40a0, fake_release_ctx as *const ())?;

    type F = unsafe extern "win64" fn(usize, u64) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_mgr = exe::ptr::<u8>(0x1_4397_ff58) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_mgr, 8);
    wr::<usize>(g_mgr as usize, 0x1234_0000);

    let mut obj_o = [0u8; 0x90];
    let mut obj_n = [0u8; 0x90];
    wr::<u32>((obj_o.as_mut_ptr() as usize) + 0x80, 2);
    wr::<u32>((obj_n.as_mut_ptr() as usize) + 0x80, 2);

    LOAD_TRACE.store(0, Ordering::SeqCst);
    let r_o = f_orig(obj_o.as_mut_ptr() as usize, 0xfeed_face);
    let tr_o = LOAD_TRACE.load(Ordering::SeqCst);

    LOAD_TRACE.store(0, Ordering::SeqCst);
    let r_n = f_new(obj_n.as_mut_ptr() as usize, 0xfeed_face);
    let tr_n = LOAD_TRACE.load(Ordering::SeqCst);

    snap.restore();

    crate::diff_eq!(r_o, r_n, "ret_prev_state");
    crate::diff_eq!(obj_o, obj_n, "obj_state");
    crate::diff_eq!(tr_o, tr_n, "load_trace");
    Ok(1)
}

/// `rend::ShaderResource::ClassTypeInfoImplementation::vfunc_13`
/// Original: `0x14224c5a0` (47 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shader_resource_typeinfo_vfunc_13(_this: usize, obj: usize) {
    let handle = rd::<usize>(obj + 0xb8);
    if handle != 0 {
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        call_va!(0x1_412f_4210, (mgr, handle) -> ());
    }
    call_va!(0x1_4242_8ce0, (obj) -> ());
}

pub unsafe fn diff_rend_shader_resource_typeinfo_vfunc_13(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut o = [0u8; 0xc8];
    let mut n = [0u8; 0xc8];
    let op = o.as_mut_ptr() as usize;
    let np = n.as_mut_ptr() as usize;
    wr::<u32>(op + 0x80, 3);
    wr::<u32>(np + 0x80, 3);
    f_orig(0, op);
    f_new(0, np);
    crate::diff_eq!(rd::<u64>(op), rd::<u64>(np), "vtbl");
    crate::diff_eq!(rd::<usize>(op + 0x40) == op, rd::<usize>(np + 0x40) == np, "self40");
    crate::diff_eq!(rd::<usize>(op + 0x60) == op, rd::<usize>(np + 0x60) == np, "self60");
    crate::diff_eq!(rd::<u32>(op + 0x20), rd::<u32>(np + 0x20), "f20");
    crate::diff_eq!(rd::<u32>(op + 0x80), rd::<u32>(np + 0x80), "f80");
    Ok(1)
}

/// `rend::ShaderResource::ClassTypeInfoImplementation::vfunc_11`
/// Original: `0x14224c620` (111 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shader_resource_typeinfo_vfunc_11(_this: usize, ptr: usize) {
    if ptr == 0 {
        return;
    }
    let mut i = 0u64;
    if rd::<u32>(ptr - 0x8) > 0 {
        loop {
            let stride = rd::<u64>(ptr - 0x10);
            let elem = ptr.wrapping_add((i.wrapping_mul(stride)) as usize);
            let handle = rd::<usize>(elem + 0xb8);
            if handle != 0 {
                let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
                call_va!(0x1_412f_4210, (mgr, handle) -> ());
            }
            call_va!(0x1_4242_8ce0, (elem) -> ());
            i = i.wrapping_add(1);
            if i >= (rd::<u32>(ptr - 0x8) as u64) {
                break;
            }
        }
    }
    call_va!(0x1_4244_2c80, (ptr) -> ());
}

pub unsafe fn diff_rend_shader_resource_typeinfo_vfunc_11(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    f_orig(0, 0);
    f_new(0, 0);
    Ok(1)
}

/// `rend::ShaderResource::ClassTypeInfoImplementation::vfunc_10`
/// Original: `0x14224c6a0` (60 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_shader_resource_typeinfo_vfunc_10(_this: usize, ptr: usize) {
    if ptr == 0 {
        return;
    }
    let handle = rd::<usize>(ptr + 0xb8);
    if handle != 0 {
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        call_va!(0x1_412f_4210, (mgr, handle) -> ());
    }
    call_va!(0x1_4242_8ce0, (ptr) -> ());
    call_va!(0x1_4244_2de0, (ptr) -> ());
}

pub unsafe fn diff_rend_shader_resource_typeinfo_vfunc_10(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    f_orig(0, 0);
    f_new(0, 0);
    Ok(1)
}

/// `rend::SDFVolumeResource::vfunc_13`
/// Original: `0x14224e5b0` (37 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_sdf_volume_resource_vfunc_13(this: usize) {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    call_va!(0x1_412f_5570, (mgr, this) -> ());
    call_va!(0x1_4242_87f0, (this) -> ());
}

pub unsafe fn diff_rend_sdf_volume_resource_vfunc_13(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static UNREG_MGR: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_unreg(mgr: usize, this: usize) {
        wr::<u32>(this + 0x90, 0xaa55);
        UNREG_MGR.store(mgr as u64, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_412f_5570, fake_unreg as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_mgr = exe::ptr::<u8>(0x1_4397_ff58) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_mgr, 8);
    wr::<usize>(g_mgr as usize, 0xabcd_0000);

    let mut o = [0u8; 0xc8];
    let mut n = [0u8; 0xc8];
    wr::<u32>((o.as_mut_ptr() as usize) + 0x80, 4);
    wr::<u32>((n.as_mut_ptr() as usize) + 0x80, 4);

    UNREG_MGR.store(0, Ordering::SeqCst);
    f_orig(o.as_mut_ptr() as usize);
    let m_o = UNREG_MGR.load(Ordering::SeqCst);

    UNREG_MGR.store(0, Ordering::SeqCst);
    f_new(n.as_mut_ptr() as usize);
    let m_n = UNREG_MGR.load(Ordering::SeqCst);

    snap.restore();

    crate::diff_eq!(o, n, "sdf_vol_vfunc_13");
    crate::diff_eq!(m_o, m_n, "unreg_mgr");
    Ok(1)
}

/// `rend_14224e680`
/// Original: `0x14224e680` (81 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_init_dual_shared_ptr_entry(
    out: usize,
    tag: u64,
    sp_a: usize,
    sp_b: usize,
) -> usize {
    wr::<u64>(out, tag);
    wr::<u64>(out + 0x8, 0);
    wr::<u64>(out + 0x10, 0);
    let ctrl_a = rd::<usize>(sp_a + 0x8);
    if ctrl_a != 0 {
        crate::simd_ops::lk_add_u32((ctrl_a + 0x8) as u64, 1); // lock inc dword [rax+8]
    }
    wr::<u64>(out + 0x8, rd::<u64>(sp_a));
    wr::<u64>(out + 0x10, rd::<u64>(sp_a + 0x8));
    wr::<u64>(out + 0x18, 0);
    wr::<u64>(out + 0x20, 0);
    let ctrl_b = rd::<usize>(sp_b + 0x8);
    if ctrl_b != 0 {
        crate::simd_ops::lk_add_u32((ctrl_b + 0x8) as u64, 1); // lock inc dword [rax+8]
    }
    wr::<u64>(out + 0x18, rd::<u64>(sp_b));
    wr::<u64>(out + 0x20, rd::<u64>(sp_b + 0x8));
    out
}

pub unsafe fn diff_rend_init_dual_shared_ptr_entry(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut ctrl_a_o = [0u32, 0u32, 5u32, 0u32];
    let mut ctrl_b_o = [0u32, 0u32, 9u32, 0u32];
    let sp_a_o = [0x1111_2222u64, ctrl_a_o.as_mut_ptr() as u64];
    let sp_b_o = [0x3333_4444u64, ctrl_b_o.as_mut_ptr() as u64];
    let mut out_o = [0u64; 5];

    let mut ctrl_a_n = [0u32, 0u32, 5u32, 0u32];
    let mut ctrl_b_n = [0u32, 0u32, 9u32, 0u32];
    let sp_a_n = [0x1111_2222u64, ctrl_a_n.as_mut_ptr() as u64];
    let sp_b_n = [0x3333_4444u64, ctrl_b_n.as_mut_ptr() as u64];
    let mut out_n = [0u64; 5];

    let r_o = f_orig(
        out_o.as_mut_ptr() as usize,
        0xdead_beef,
        sp_a_o.as_ptr() as usize,
        sp_b_o.as_ptr() as usize,
    );
    let r_n = f_new(
        out_n.as_mut_ptr() as usize,
        0xdead_beef,
        sp_a_n.as_ptr() as usize,
        sp_b_n.as_ptr() as usize,
    );
    crate::diff_eq!(r_o == out_o.as_mut_ptr() as usize, r_n == out_n.as_mut_ptr() as usize, "ret");
    crate::diff_eq!(ctrl_a_o, ctrl_a_n, "ctrl_a");
    crate::diff_eq!(ctrl_b_o, ctrl_b_n, "ctrl_b");
    crate::diff_eq!(out_o[0], out_n[0], "tag");
    crate::diff_eq!(out_o[1], out_n[1], "ptr_a");
    crate::diff_eq!(out_o[3], out_n[3], "ptr_b");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: none
// Replacement::verified(0x1_4222_5af0, "rend_142225af0", rend::rend_e17_b::rend_extract_proj_jitter_scale as *const (), rend::rend_e17_b::diff_rend_extract_proj_jitter_scale),
// Replacement::verified_throws(0x1_4222_6180, "rend_142226180", rend::rend_e17_b::rend_ssaa_vfunc_1 as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_ssaa_vfunc_1),
// Replacement::verified(0x1_4224_28f0, "rend_1422428f0", rend::rend_e17_b::rend_shadow_interface_depth_vfunc_7 as *const (), rend::rend_e17_b::diff_rend_shadow_interface_depth_vfunc_7),
// Replacement::verified(0x1_4224_2990, "rend_142242990", rend::rend_e17_b::rend_shadow_interface_depth_vfunc_4 as *const (), rend::rend_e17_b::diff_rend_shadow_interface_depth_vfunc_4),
// Replacement::verified_throws(0x1_4224_2a30, "rend_142242a30", rend::rend_e17_b::rend_shadow_interface_depth_vfunc_3 as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_shadow_interface_depth_vfunc_3),
// Replacement::verified(0x1_4224_2b90, "rend_142242b90", rend::rend_e17_b::rend_shadow_interface_depth_vfunc_2 as *const (), rend::rend_e17_b::diff_rend_shadow_interface_depth_vfunc_2),
// Replacement::verified_throws(0x1_4224_2cc0, "rend_142242cc0", rend::rend_e17_b::rend_shadow_interface_depth_vfunc_1 as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_shadow_interface_depth_vfunc_1),
// Replacement::verified_throws(0x1_4224_2d00, "rend_142242d00", rend::rend_e17_b::rend_shadow_interface_depth_release_atlas as *const (), Throws::Caught, rend::rend_e17_b::diff_rend_shadow_interface_depth_release_atlas),
// Replacement::verified_throws(0x1_4224_2fb0, "rend_142242fb0", rend::rend_e17_b::rend_shadow_interface_depth_dtor as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_shadow_interface_depth_dtor),
// Replacement::verified_throws(0x1_4224_3120, "rend_142243120", rend::rend_e17_b::rend_shadow_interface_depth_ctor as *const (), Throws::Caught, rend::rend_e17_b::diff_rend_shadow_interface_depth_ctor),
// Replacement::verified(0x1_4224_31e0, "rend_1422431e0", rend::rend_e17_b::rend_shadow_map_depth_get_atlas_height as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_get_atlas_height),
// Replacement::verified(0x1_4224_3200, "rend_142243200", rend::rend_e17_b::rend_shadow_map_depth_get_atlas_width as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_get_atlas_width),
// Replacement::verified(0x1_4224_3220, "rend_142243220", rend::rend_e17_b::rend_shadow_map_depth_vfunc_11 as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_vfunc_11),
// Replacement::verified(0x1_4224_3550, "rend_142243550", rend::rend_e17_b::rend_shadow_map_depth_vfunc_7 as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_vfunc_7),
// Replacement::verified(0x1_4224_3640, "rend_142243640", rend::rend_e17_b::rend_shadow_map_depth_vfunc_6 as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_vfunc_6),
// Replacement::verified(0x1_4224_3720, "rend_142243720", rend::rend_e17_b::rend_shadow_map_depth_vfunc_5 as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_vfunc_5),
// Replacement::verified(0x1_4224_37c0, "rend_1422437c0", rend::rend_e17_b::rend_shadow_map_depth_ctor as *const (), rend::rend_e17_b::diff_rend_shadow_map_depth_ctor),
// Replacement::verified(0x1_4224_3820, "rend_142243820", rend::rend_e17_b::rend_shadow_interface_color_vfunc_7 as *const (), rend::rend_e17_b::diff_rend_shadow_interface_color_vfunc_7),
// Replacement::verified(0x1_4224_38e0, "rend_1422438e0", rend::rend_e17_b::rend_shadow_interface_color_vfunc_4 as *const (), rend::rend_e17_b::diff_rend_shadow_interface_color_vfunc_4),
// Replacement::verified_throws(0x1_4224_3930, "rend_142243930", rend::rend_e17_b::rend_shadow_interface_color_vfunc_3 as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_shadow_interface_color_vfunc_3),
// Replacement::verified(0x1_4224_3a00, "rend_142243a00", rend::rend_e17_b::rend_shadow_interface_color_vfunc_2 as *const (), rend::rend_e17_b::diff_rend_shadow_interface_color_vfunc_2),
// Replacement::verified_throws(0x1_4224_3b00, "rend_142243b00", rend::rend_e17_b::rend_shadow_interface_color_vfunc_1 as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_shadow_interface_color_vfunc_1),
// Replacement::verified_throws(0x1_4224_3b40, "rend_142243b40", rend::rend_e17_b::rend_shadow_interface_color_release_atlas as *const (), Throws::Caught, rend::rend_e17_b::diff_rend_shadow_interface_color_release_atlas),
// Replacement::verified_throws(0x1_4224_3e40, "rend_142243e40", rend::rend_e17_b::rend_shadow_interface_color_dtor as *const (), Throws::Discard, rend::rend_e17_b::diff_rend_shadow_interface_color_dtor),
// Replacement::verified_throws(0x1_4224_3f50, "rend_142243f50", rend::rend_e17_b::rend_shadow_interface_color_ctor as *const (), Throws::Caught, rend::rend_e17_b::diff_rend_shadow_interface_color_ctor),
// Replacement::verified(0x1_4224_42e0, "rend_1422442e0", rend::rend_e17_b::rend_shadow_map_color_vfunc_11 as *const (), rend::rend_e17_b::diff_rend_shadow_map_color_vfunc_11),
// Replacement::verified(0x1_4224_43b0, "rend_1422443b0", rend::rend_e17_b::rend_shadow_map_color_vfunc_7 as *const (), rend::rend_e17_b::diff_rend_shadow_map_color_vfunc_7),
// Replacement::verified(0x1_4224_4470, "rend_142244470", rend::rend_e17_b::rend_shadow_map_color_vfunc_5 as *const (), rend::rend_e17_b::diff_rend_shadow_map_color_vfunc_5),
// Replacement::verified(0x1_4224_be10, "rend_14224be10", rend::rend_e17_b::rend_shader_resource_vfunc_13 as *const (), rend::rend_e17_b::diff_rend_shader_resource_vfunc_13),
// Replacement::verified_throws(0x1_4224_bef0, "rend_14224bef0", rend::rend_e17_b::rend_shader_resource_load_from_stream as *const (), Throws::Caught, rend::rend_e17_b::diff_rend_shader_resource_load_from_stream),
// Replacement::verified(0x1_4224_c5a0, "rend_14224c5a0", rend::rend_e17_b::rend_shader_resource_typeinfo_vfunc_13 as *const (), rend::rend_e17_b::diff_rend_shader_resource_typeinfo_vfunc_13),
// Replacement::verified(0x1_4224_c620, "rend_14224c620", rend::rend_e17_b::rend_shader_resource_typeinfo_vfunc_11 as *const (), rend::rend_e17_b::diff_rend_shader_resource_typeinfo_vfunc_11),
// Replacement::verified(0x1_4224_c6a0, "rend_14224c6a0", rend::rend_e17_b::rend_shader_resource_typeinfo_vfunc_10 as *const (), rend::rend_e17_b::diff_rend_shader_resource_typeinfo_vfunc_10),
// Replacement::verified(0x1_4224_e5b0, "rend_14224e5b0", rend::rend_e17_b::rend_sdf_volume_resource_vfunc_13 as *const (), rend::rend_e17_b::diff_rend_sdf_volume_resource_vfunc_13),
// Replacement::verified(0x1_4224_e680, "rend_14224e680", rend::rend_e17_b::rend_init_dual_shared_ptr_entry as *const (), rend::rend_e17_b::diff_rend_init_dual_shared_ptr_entry),
