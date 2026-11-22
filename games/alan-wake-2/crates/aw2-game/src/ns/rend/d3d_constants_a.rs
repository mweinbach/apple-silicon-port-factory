use crate::exe::{self, rd, wr};

/// Original: `0x142090de0` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142090de0(dst: usize, src: usize) -> usize {
    wr::<u32>(dst, rd::<u32>(src));
    wr::<u32>(dst + 0x4, rd::<u32>(src + 0x4));
    wr::<u32>(dst + 0x8, rd::<u32>(src + 0x8));
    dst
}

pub unsafe fn diff_d3d_142090de0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let src = [0x1122_3344u32, 0x5566_7788, 0x99aa_bbcc, 0xddee_ff00];
    let mut dst_o = [0u32; 4];
    let mut dst_n = [0u32; 4];
    let ro = f_orig(dst_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    let rn = f_new(dst_n.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(ro, dst_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, dst_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(dst_o, dst_n, "copied words");
    Ok(1)
}

/// Original: `0x142090e00` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142090e00(a1: u8, a2: u8) {
    let eax = (a2 as u32).wrapping_mul(2);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_82c0) as usize, a1);
    wr::<u32>(exe::ptr::<u32>(0x1_43a6_3760) as usize, eax);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_82bc) as usize, 1);
    wr::<u8>(exe::ptr::<u8>(0x1_43a6_374c) as usize, 1);
}

pub unsafe fn diff_d3d_142090e00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g_c0 = exe::ptr::<u8>(0x1_4385_82c0) as usize;
    let g_60 = exe::ptr::<u32>(0x1_43a6_3760) as usize;
    let g_bc = exe::ptr::<u8>(0x1_4385_82bc) as usize;
    let g_4c = exe::ptr::<u8>(0x1_43a6_374c) as usize;
    let s_c0 = rd::<u8>(g_c0);
    let s_60 = rd::<u32>(g_60);
    let s_bc = rd::<u8>(g_bc);
    let s_4c = rd::<u8>(g_4c);
    let mut ran = 0u32;
    for (a1, a2) in [(0u8, 0u8), (1, 1), (7, 3), (0xff, 0x80)] {
        wr::<u8>(g_c0, 0);
        wr::<u32>(g_60, 0);
        wr::<u8>(g_bc, 0);
        wr::<u8>(g_4c, 0);
        f_orig(a1, a2);
        let o = (rd::<u8>(g_c0), rd::<u32>(g_60), rd::<u8>(g_bc), rd::<u8>(g_4c));
        wr::<u8>(g_c0, 0);
        wr::<u32>(g_60, 0);
        wr::<u8>(g_bc, 0);
        wr::<u8>(g_4c, 0);
        f_new(a1, a2);
        let n = (rd::<u8>(g_c0), rd::<u32>(g_60), rd::<u8>(g_bc), rd::<u8>(g_4c));
        crate::diff_eq!(o, n, "globals");
        ran += 1;
    }
    wr::<u8>(g_c0, s_c0);
    wr::<u32>(g_60, s_60);
    wr::<u8>(g_bc, s_bc);
    wr::<u8>(g_4c, s_4c);
    Ok(ran)
}

/// Original: `0x1420911c0` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420911c0() -> u8 {
    let val = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3760) as usize);
    u8::from(val.wrapping_sub(1) <= 1)
}

pub unsafe fn diff_d3d_1420911c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g = exe::ptr::<u32>(0x1_43a6_3760) as usize;
    let saved = rd::<u32>(g);
    let mut ran = 0u32;
    for v in [0u32, 1, 2, 3, 4, u32::MAX] {
        wr::<u32>(g, v);
        let ro = f_orig();
        let rn = f_new();
        crate::diff_eq!(ro, rn, "ret");
        ran += 1;
    }
    wr::<u32>(g, saved);
    Ok(ran)
}

/// Original: `0x1420911e0` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420911e0(fmt: u32) -> u8 {
    u8::from(fmt == 0 || fmt == 0x27)
}

pub unsafe fn diff_d3d_1420911e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0u32;
    for v in [0u32, 1, 0x26, 0x27, 0x28, 0xff, u32::MAX] {
        crate::diff_eq!(f_orig(v), f_new(v), "ret");
        ran += 1;
    }
    Ok(ran)
}

/// Original: `0x142091200` (6 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091200(val: u32) -> u8 {
    u8::from(val == 0)
}

pub unsafe fn diff_d3d_142091200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0u32;
    for v in [0u32, 1, 2, 0x8000_0000, u32::MAX] {
        crate::diff_eq!(f_orig(v), f_new(v), "ret");
        ran += 1;
    }
    Ok(ran)
}

/// Original: `0x142091210` (88 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091210(mode: u32) -> u32 {
    match mode {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 6,
        _ => 3,
    }
}

pub unsafe fn diff_d3d_142091210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0u32;
    for v in [0u32, 1, 2, 3, 4, 5, 6, 7, 100, u32::MAX] {
        crate::diff_eq!(f_orig(v), f_new(v), "ret");
        ran += 1;
    }
    Ok(ran)
}

/// Original: `0x142091270` (180 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091270(this: usize) {
    for off in [0x108usize, 0x100, 0xf8, 0xf0, 0xe8] {
        let p = rd::<usize>(this + off);
        if p != 0 {
            exe::call_va!(0x1_4204_f860, (p) -> ());
            exe::call_va!(0x1_4244_2de0, (p) -> ());
        }
    }
    exe::call_va!(0x1_4034_96f0, (this + 0x80) -> ());
}

/// Original: `0x142091480` (35 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091480(arg1: u32) -> u32 {
    let g = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3788) as usize);
    if g != 0 {
        return g.wrapping_sub(1);
    }
    if (arg1.wrapping_sub(4) & 0xffff_fffd) == 0 {
        6
    } else {
        3
    }
}

pub unsafe fn diff_d3d_142091480(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g_addr = exe::ptr::<u32>(0x1_43a6_3788) as usize;
    let saved = rd::<u32>(g_addr);
    let mut ran = 0u32;
    for g in [0u32, 1, 2, 5] {
        wr::<u32>(g_addr, g);
        for a in [0u32, 1, 2, 3, 4, 5, 6, 7, 8, u32::MAX] {
            crate::diff_eq!(f_orig(a), f_new(a), "ret");
            ran += 1;
        }
    }
    wr::<u32>(g_addr, saved);
    Ok(ran)
}

/// Original: `0x1420914d0` (23 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420914d0() -> u32 {
    let g_addr = exe::ptr::<i32>(0x1_4385_83d4) as usize;
    let val = rd::<i32>(g_addr).max(1);
    wr::<i32>(g_addr, val);
    val as u32
}

pub unsafe fn diff_d3d_1420914d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g_addr = exe::ptr::<i32>(0x1_4385_83d4) as usize;
    let saved = rd::<i32>(g_addr);
    let mut ran = 0u32;
    for v in [-5i32, -1, 0, 1, 2, 10] {
        wr::<i32>(g_addr, v);
        let ro = f_orig();
        let go = rd::<i32>(g_addr);
        wr::<i32>(g_addr, v);
        let rn = f_new();
        let gn = rd::<i32>(g_addr);
        crate::diff_eq!((ro, go), (rn, gn), "clamp");
        ran += 1;
    }
    wr::<i32>(g_addr, saved);
    Ok(ran)
}

/// Original: `0x1420914f0` (101 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420914f0() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) == 0 {
        return;
    }
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let arg_rbx = rd::<usize>(exe::ptr::<usize>(0x1_4385_8398) as usize);
    if fn_ptr == 0 {
        let name = exe::ptr::<u8>(0x1_433d_d168) as usize;
        let status: u32 = exe::call_iat!(0x1_4264_7308, (4u32, name, fn_slot) -> u32);
        if status != 0 {
            return;
        }
        fn_ptr = rd::<usize>(fn_slot);
    }
    exe::call_va!(fn_ptr, (5u32, arg_rbx) -> ());
}

/// Original: `0x1420917a0` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420917a0() -> u64 {
    exe::call_iat!(0x1_4264_7358, (0x3e8u32, 0usize) -> u64)
}

/// Original: `0x142091b10` (11 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091b10() -> u8 {
    u8::from(rd::<u32>(exe::ptr::<u32>(0x1_4385_83f8) as usize) > 0)
}

pub unsafe fn diff_d3d_142091b10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g_addr = exe::ptr::<u32>(0x1_4385_83f8) as usize;
    let saved = rd::<u32>(g_addr);
    let mut ran = 0u32;
    for v in [0u32, 1, 2, 0x8000_0000, u32::MAX] {
        wr::<u32>(g_addr, v);
        crate::diff_eq!(f_orig(), f_new(), "ret");
        ran += 1;
    }
    wr::<u32>(g_addr, saved);
    Ok(ran)
}

/// Original: `0x142091b40` (122 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091b40() -> u8 {
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let mut luid = 0u64;
    let luid_ptr = core::ptr::addr_of_mut!(luid) as usize;
    let vtbl = rd::<usize>(dev);
    let get_luid = rd::<usize>(vtbl + 0x158);
    exe::call_va!(get_luid, (dev, luid_ptr) -> usize);

    let mut desc = [0u8; 0x38];
    let base = desc.as_mut_ptr() as usize;
    wr::<u64>(base, 0);
    wr::<u32>(base + 0x08, 0x0677_315f);
    wr::<u32>(base + 0x0c, 0x4492_a746);
    wr::<u32>(base + 0x10, 0x61cb_429f);
    wr::<u32>(base + 0x14, 0xd4c3_c942);
    wr::<u64>(base + 0x18, 1);
    wr::<usize>(base + 0x20, luid_ptr);
    wr::<u32>(base + 0x28, 8);
    wr::<u64>(base + 0x30, 0);

    let status: u32 = exe::call_iat!(0x1_4264_7340, (0x3e8u32, base) -> u32);
    u8::from(status == 0)
}

/// Original: `0x142091bd0` (29 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091bd0(flag: u8) -> u32 {
    let g_d4 = exe::ptr::<i32>(0x1_4385_83d4) as usize;
    let g_d8 = exe::ptr::<u8>(0x1_4385_83d8) as usize;
    let val = rd::<i32>(g_d4).max(1);
    wr::<u8>(g_d8, flag);
    wr::<i32>(g_d4, val);
    val as u32
}

pub unsafe fn diff_d3d_142091bd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g_d4 = exe::ptr::<i32>(0x1_4385_83d4) as usize;
    let g_d8 = exe::ptr::<u8>(0x1_4385_83d8) as usize;
    let s_d4 = rd::<i32>(g_d4);
    let s_d8 = rd::<u8>(g_d8);
    let mut ran = 0u32;
    for (v, f) in [(-2i32, 0u8), (0, 1), (1, 2), (5, 0xff)] {
        wr::<i32>(g_d4, v);
        wr::<u8>(g_d8, 0);
        let ro = f_orig(f);
        let so = (rd::<i32>(g_d4), rd::<u8>(g_d8));
        wr::<i32>(g_d4, v);
        wr::<u8>(g_d8, 0);
        let rn = f_new(f);
        let sn = (rd::<i32>(g_d4), rd::<u8>(g_d8));
        crate::diff_eq!((ro, so), (rn, sn), "state");
        ran += 1;
    }
    wr::<i32>(g_d4, s_d4);
    wr::<u8>(g_d8, s_d8);
    Ok(ran)
}

/// Original: `0x142091c00` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091c00(id: u32) {
    let expected = rd::<u32>(exe::ptr::<u32>(0x1_4385_8470) as usize);
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) == 0 {
        return;
    }
    if expected == 0 || id != expected {
        return;
    }
    let token = rd::<usize>(exe::ptr::<usize>(0x1_4385_8388) as usize);
    exe::call_va!(0x1_411e_2a60, (8u32, token) -> ());
}

/// Original: `0x142091c30` (101 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091c30() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) == 0 {
        return;
    }
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let arg_rbx = rd::<usize>(exe::ptr::<usize>(0x1_4385_8388) as usize);
    if fn_ptr == 0 {
        let name = exe::ptr::<u8>(0x1_433d_d168) as usize;
        let status: u32 = exe::call_iat!(0x1_4264_7308, (4u32, name, fn_slot) -> u32);
        if status != 0 {
            return;
        }
        fn_ptr = rd::<usize>(fn_slot);
    }
    exe::call_va!(fn_ptr, (7u32, arg_rbx) -> ());
}

/// Original: `0x142091ca0` (52 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091ca0() {
    let slot_88 = exe::ptr::<usize>(0x1_4385_8388) as usize;
    let slot_90 = exe::ptr::<usize>(0x1_4385_8390) as usize;
    let token = rd::<usize>(slot_88);
    wr::<usize>(slot_90, token);
    let vtbl = rd::<usize>(token);
    let get_fn = rd::<usize>(vtbl);
    let cur: u32 = exe::call_va!(get_fn, (token) -> u32);
    let mut next = cur.wrapping_add(1);
    let next_ptr = core::ptr::addr_of_mut!(next) as usize;
    exe::call_iat!(0x1_4264_7310, (slot_88, next_ptr) -> u32);
}

/// Original: `0x142091ce0` (96 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091ce0() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83c9) as usize) == 0 {
        return;
    }
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c610) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let arg_rbx = rd::<usize>(exe::ptr::<usize>(0x1_4385_8388) as usize);
    if fn_ptr == 0 {
        let name = exe::ptr::<u8>(0x1_433d_d1b8) as usize;
        let status: u32 = exe::call_iat!(0x1_4264_7308, (3u32, name, fn_slot) -> u32);
        if status != 0 {
            return;
        }
        fn_ptr = rd::<usize>(fn_slot);
    }
    exe::call_va!(fn_ptr, (arg_rbx) -> ());
}

/// Original: `0x142091d60` (184 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091d60() {
    let mut buf = [0u8; 0xa0];
    let base = buf.as_mut_ptr() as usize;
    let rbp_m49 = base;
    let rbp_m39 = base + 0x10;

    wr::<u32>(rbp_m49, 0);
    wr::<u32>(rbp_m49 + 0x04, 3);
    wr::<u32>(rbp_m49 + 0x08, 0x3e8);
    wr::<u32>(rbp_m49 + 0x0c, 0x3e9);

    wr::<u64>(rbp_m39, 0);
    wr::<u32>(rbp_m39 + 0x08, 0x1ca1_0965);
    wr::<u32>(rbp_m39 + 0x0c, 0x432b_bf8e);
    wr::<u32>(rbp_m39 + 0x10, 0x1667_a18d);
    wr::<u32>(rbp_m39 + 0x14, 0x14fb_79d8);
    wr::<u64>(rbp_m39 + 0x18, 1);
    wr::<u8>(rbp_m39 + 0x20, 0);
    wr::<u32>(rbp_m39 + 0x24, 0);
    wr::<u64>(rbp_m39 + 0x28, 0);
    wr::<u32>(rbp_m39 + 0x30, 0);
    wr::<u64>(rbp_m39 + 0x38, 0);
    wr::<u64>(rbp_m39 + 0x40, 0);
    wr::<u64>(rbp_m39 + 0x48, 0);
    wr::<u64>(rbp_m39 + 0x50, 0);
    wr::<u64>(rbp_m39 + 0x58, 0x49);
    wr::<usize>(rbp_m39 + 0x60, rbp_m49);
    wr::<u32>(rbp_m39 + 0x68, 4);
    wr::<u64>(rbp_m39 + 0x6c, 0x0616_1e2b);
    wr::<u64>(rbp_m39 + 0x78, 0);
    wr::<u64>(rbp_m39 + 0x80, 0);
    wr::<u32>(rbp_m39 + 0x88, 1);

    let magic = 0x0002_0007_0000_fedcu64;
    exe::call_iat!(0x1_4264_7318, (rbp_m39, magic) -> u32);
}

/// Original: `0x142091e20` (101 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091e20() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) == 0 {
        return;
    }
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let arg_rbx = rd::<usize>(exe::ptr::<usize>(0x1_4385_8390) as usize);
    if fn_ptr == 0 {
        let name = exe::ptr::<u8>(0x1_433d_d168) as usize;
        let status: u32 = exe::call_iat!(0x1_4264_7308, (4u32, name, fn_slot) -> u32);
        if status != 0 {
            return;
        }
        fn_ptr = rd::<usize>(fn_slot);
    }
    exe::call_va!(fn_ptr, (1u32, arg_rbx) -> ());
}

/// Original: `0x142091e90` (98 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091e90() {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_83ca) as usize) == 0 {
        return;
    }
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c608) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let arg_rbx = rd::<usize>(exe::ptr::<usize>(0x1_4385_8390) as usize);
    if fn_ptr == 0 {
        let name = exe::ptr::<u8>(0x1_433d_d168) as usize;
        let status: u32 = exe::call_iat!(0x1_4264_7308, (4u32, name, fn_slot) -> u32);
        if status != 0 {
            return;
        }
        fn_ptr = rd::<usize>(fn_slot);
    }
    exe::call_va!(fn_ptr, (0u32, arg_rbx) -> ());
}

/// Original: `0x142092010` (120 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092010(this: usize, count: i32) {
    if count <= 0 {
        return;
    }
    let mut ebx = 0i32;
    let mut rem = count as usize;
    while rem != 0 {
        let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
        let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
        let offset = stride.wrapping_mul(ebx as i64 as u64);
        let vtbl = rd::<usize>(dev);
        let dst_desc = rd::<u64>(this + 0x10).wrapping_add(offset);
        let src_desc = rd::<u64>(ctx + 0xb0);
        let fn_ptr = rd::<usize>(vtbl + 0xc0);
        exe::call_va!(fn_ptr, (dev, 0x4du32, dst_desc, src_desc, 0u32) -> ());
        ebx = ebx.wrapping_add(0x4d);
        rem -= 1;
    }
}

/// Original: `0x142092090` (132 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092090(this: usize, out: usize, idx: u32) -> usize {
    let val = match idx {
        0 | 3 => rd::<u64>(this + 0x10),
        1 => {
            let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
            rd::<u64>(this + 0x10).wrapping_add(stride.wrapping_mul(0x4d))
        }
        2 | 5 => rd::<u64>(this + 0x40),
        4 => {
            let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
            rd::<u64>(this + 0x10).wrapping_add(stride.wrapping_mul(0x9a))
        }
        _ => 0,
    };
    wr::<u64>(out, val);
    out
}

pub unsafe fn diff_d3d_142092090(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let g_stride = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let saved = rd::<u32>(g_stride);
    wr::<u32>(g_stride, 0x20);

    let mut obj = [0u8; 0x50];
    let base = obj.as_mut_ptr() as usize;
    wr::<u64>(base + 0x10, 0x1000_0000);
    wr::<u64>(base + 0x40, 0x2000_0000);

    let mut ran = 0u32;
    for idx in [0u32, 1, 2, 3, 4, 5, 6, 99] {
        let mut out_o = 0xdead_beef_u64;
        let mut out_n = 0xdead_beef_u64;
        let ro = f_orig(base, core::ptr::addr_of_mut!(out_o) as usize, idx);
        let rn = f_new(base, core::ptr::addr_of_mut!(out_n) as usize, idx);
        crate::diff_eq!(ro, core::ptr::addr_of_mut!(out_o) as usize, "ret orig");
        crate::diff_eq!(rn, core::ptr::addr_of_mut!(out_n) as usize, "ret new");
        crate::diff_eq!(out_o, out_n, "out handle");
        ran += 1;
    }
    wr::<u32>(g_stride, saved);
    Ok(ran)
}

/// Original: `0x142092380` (158 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092380(this: usize, target: usize) {
    let base_handle = rd::<u64>(target + 0x08);
    let cur_count = rd::<u32>(this + 0x278);
    let prev_count = rd::<u32>(target + 0x38);
    let copy_count = cur_count.max(prev_count);
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let src_a = rd::<u64>(this + 0x280);
    let vtbl = rd::<usize>(dev);
    let fn_a = rd::<usize>(vtbl + 0xc0);
    exe::call_va!(fn_a, (dev, copy_count, base_handle, src_a, 0u32) -> ());

    let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let offset = stride.wrapping_mul(0x40400);
    wr::<u32>(target + 0x38, rd::<u32>(this + 0x278));
    let dst_b = base_handle.wrapping_add(offset);
    let vtbl2 = rd::<usize>(dev2);
    let src_b = rd::<u64>(this + 0x2a0);
    let fn_b = rd::<usize>(vtbl2 + 0xc0);
    exe::call_va!(fn_b, (dev2, 0x10000u32, dst_b, src_b, 0u32) -> ());

    wr::<u64>(target + 0x30, rd::<u64>(this + 0x08));
}

/// Original: `0x142092760` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092760(_this: usize, out: usize) {
    wr::<u32>(out + 0x18, 0x50400);
}

pub unsafe fn diff_d3d_142092760(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0u8; 0x20];
    let mut buf_n = [0u8; 0x20];
    f_orig(0, buf_o.as_mut_ptr() as usize);
    f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(buf_o, buf_n, "out+0x18");
    Ok(1)
}

/// Original: `0x142092dc0` (25 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092dc0(_this: usize, idx: u32, val: u64) {
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let cnt = rd::<u64>(ctx + 0x4b8).wrapping_add(1);
    wr::<u64>(ctx + 0x4b8, cnt);
    wr::<u64>(ctx + 0x2b8 + (idx as usize) * 8, val);
}

/// Original: `0x142093030` (218 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142093030(_a1: usize, _a2: usize, out_a: usize, out_b: usize) {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let tid1: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let g_cs = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let g_owner = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let g_rec = exe::ptr::<u32>(0x1_4385_8484) as usize;
    if rd::<u32>(g_owner) != tid1 {
        exe::call_iat!(0x1_4264_55a8, (g_cs) -> ());
        wr::<u32>(g_owner, tid1);
    }
    wr::<u32>(g_rec, rd::<u32>(g_rec).wrapping_add(1));

    let tid2: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rsi + 0x28) != tid2 {
        exe::call_iat!(0x1_4264_55a8, (rsi + 0x20) -> ());
        wr::<u32>(rsi + 0x28, tid2);
    }
    wr::<u32>(rsi + 0x2c, rd::<u32>(rsi + 0x2c).wrapping_add(1));

    let ptr_a8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fca8) as usize);
    wr::<u64>(out_b, rd::<u64>(ptr_a8 + 0x08));
    wr::<u64>(out_a, rd::<u64>(rsi + 0x280));

    let rec2 = rd::<u32>(rsi + 0x2c).wrapping_sub(1);
    wr::<u32>(rsi + 0x2c, rec2);
    if rec2 == 0 {
        wr::<u32>(rsi + 0x28, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (rsi + 0x20) -> ());
    }

    let rec1 = rd::<u32>(g_rec).wrapping_sub(1);
    wr::<u32>(g_rec, rec1);
    if rec1 == 0 {
        wr::<u32>(g_owner, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (g_cs) -> ());
    }
}

/// Original: `0x142093510` (112 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142093510() {
    let rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let g_cs = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let g_owner = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let g_rec = exe::ptr::<u32>(0x1_4385_8484) as usize;
    if rd::<u32>(g_owner) != tid {
        exe::call_iat!(0x1_4264_55a8, (g_cs) -> ());
        wr::<u32>(g_owner, tid);
    }
    wr::<u32>(g_rec, rd::<u32>(g_rec).wrapping_add(1));

    let _ = crate::simd_ops::atomic_xchg_u32((rdi + 0x18) as u64, 2); // xchg dword [rdi+0x18]

    let rec = rd::<u32>(g_rec).wrapping_sub(1);
    wr::<u32>(g_rec, rec);
    if rec == 0 {
        wr::<u32>(g_owner, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (g_cs) -> ());
    }
}

/// Original: `0x142093930` (151 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142093930(this: usize) {
    let count = rd::<u32>(this + 0x38);
    let mut i = 0u32;
    while i != count {
        let head2 = rd::<u32>(this + 0x30) as u64;
        let cap2 = rd::<u32>(this + 0x3c) as u64;
        let buf2 = rd::<usize>(this + 0x28);
        let idx2 = (head2.wrapping_add(i as u64) % cap2) as usize;
        let item = rd::<u64>(buf2 + idx2 * 8);

        let len1 = rd::<u32>(this + 0x20);
        if len1 == rd::<u32>(this + 0x24) {
            exe::call_va!(0x1_4034_97a0, (this + 0x10, len1.wrapping_add(1)) -> ());
        }
        let tail1 = rd::<u32>(this + 0x1c);
        let buf1 = rd::<usize>(this + 0x10);
        wr::<u64>(buf1 + (tail1 as usize) * 8, item);
        let next_tail = tail1.wrapping_add(1);
        wr::<u32>(this + 0x1c, next_tail);
        if next_tail == rd::<u32>(this + 0x24) {
            wr::<u32>(this + 0x1c, 0);
        }
        wr::<u32>(this + 0x20, rd::<u32>(this + 0x20).wrapping_add(1));
        i = i.wrapping_add(1);
    }
    wr::<u64>(this + 0x30, 0);
    wr::<u32>(this + 0x38, 0);
}

pub unsafe fn diff_d3d_142093930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut ring1_o = [0u64; 4];
    let mut ring1_n = [0u64; 4];
    let ring2 = [0x1111u64, 0x2222, 0x3333, 0x4444];

    let mut obj_o = [0u8; 0x48];
    let mut obj_n = [0u8; 0x48];
    for (obj, r1) in [(&mut obj_o, &mut ring1_o), (&mut obj_n, &mut ring1_n)] {
        let b = obj.as_mut_ptr() as usize;
        wr::<usize>(b + 0x10, r1.as_mut_ptr() as usize);
        wr::<u32>(b + 0x18, 0);
        wr::<u32>(b + 0x1c, 3);
        wr::<u32>(b + 0x20, 1);
        wr::<u32>(b + 0x24, 4);
        wr::<usize>(b + 0x28, ring2.as_ptr() as usize);
        wr::<u32>(b + 0x30, 2);
        wr::<u32>(b + 0x34, 1);
        wr::<u32>(b + 0x38, 2);
        wr::<u32>(b + 0x3c, 4);
    }
    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(ring1_o, ring1_n, "ring1 data");
    crate::diff_eq!(&obj_o[0x18..0x40], &obj_n[0x18..0x40], "ring metadata");
    Ok(1)
}

/// Original: `0x1420939d0` (77 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420939d0(this: usize, item: u64) {
    let ring = this + 0x28;
    let len = rd::<u32>(ring + 0x10);
    if len == rd::<u32>(ring + 0x14) {
        exe::call_va!(0x1_4034_97a0, (ring, len.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(ring + 0x0c);
    let buf = rd::<usize>(ring);
    wr::<u64>(buf + (tail as usize) * 8, item);
    let next_tail = tail.wrapping_add(1);
    wr::<u32>(ring + 0x0c, next_tail);
    wr::<u32>(ring + 0x10, rd::<u32>(ring + 0x10).wrapping_add(1));
    if next_tail == rd::<u32>(ring + 0x14) {
        wr::<u32>(ring + 0x0c, 0);
    }
}

pub unsafe fn diff_d3d_1420939d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u64; 4];
    let mut buf_n = [0u64; 4];
    let mut obj_o = [0u8; 0x48];
    let mut obj_n = [0u8; 0x48];
    for (obj, buf) in [(&mut obj_o, &mut buf_o), (&mut obj_n, &mut buf_n)] {
        let b = obj.as_mut_ptr() as usize;
        wr::<usize>(b + 0x28, buf.as_mut_ptr() as usize);
        wr::<u32>(b + 0x34, 3);
        wr::<u32>(b + 0x38, 1);
        wr::<u32>(b + 0x3c, 4);
    }
    f_orig(obj_o.as_mut_ptr() as usize, 0xcafe_babe);
    f_new(obj_n.as_mut_ptr() as usize, 0xcafe_babe);
    crate::diff_eq!(buf_o, buf_n, "ring buf");
    crate::diff_eq!(&obj_o[0x30..0x40], &obj_n[0x30..0x40], "ring state");
    Ok(1)
}

/// Original: `0x142093a30` (103 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142093a30(this: usize, out: usize) -> usize {
    if rd::<u32>(this + 0x20) == 0 {
        let grow = rd::<u32>(this + 0x44);
        exe::call_va!(0x1_4209_3aa0, (this, grow) -> ());
    }
    let head = rd::<u32>(this + 0x18);
    let cap = rd::<u32>(this + 0x24);
    let idx = ((head as u64) % (cap as u64)) as usize;
    let buf = rd::<usize>(this + 0x10);
    wr::<u64>(out, rd::<u64>(buf + idx * 8));
    let len = rd::<u32>(this + 0x20);
    if len != 0 {
        let next_head = head.wrapping_add(1);
        wr::<u32>(this + 0x18, if next_head == cap { 0 } else { next_head });
        wr::<u32>(this + 0x20, len.wrapping_sub(1));
    }
    out
}

pub unsafe fn diff_d3d_142093a30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let buf = [0x10u64, 0x20, 0x30, 0x40];
    let mut obj_o = [0u8; 0x50];
    let mut obj_n = [0u8; 0x50];
    for obj in [&mut obj_o, &mut obj_n] {
        let b = obj.as_mut_ptr() as usize;
        wr::<usize>(b + 0x10, buf.as_ptr() as usize);
        wr::<u32>(b + 0x18, 3);
        wr::<u32>(b + 0x1c, 1);
        wr::<u32>(b + 0x20, 2);
        wr::<u32>(b + 0x24, 4);
    }
    let mut out_o = 0u64;
    let mut out_n = 0u64;
    let ro = f_orig(obj_o.as_mut_ptr() as usize, core::ptr::addr_of_mut!(out_o) as usize);
    let rn = f_new(obj_n.as_mut_ptr() as usize, core::ptr::addr_of_mut!(out_n) as usize);
    crate::diff_eq!(ro, core::ptr::addr_of_mut!(out_o) as usize, "ret orig");
    crate::diff_eq!(rn, core::ptr::addr_of_mut!(out_n) as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "popped item");
    crate::diff_eq!(&obj_o[0x18..0x28], &obj_n[0x18..0x28], "ring state");
    Ok(1)
}

/// Original: `0x142093c80` (53 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142093c80(this: usize) {
    exe::call_va!(0x1_401d_e830, (this + 0x48) -> ());
    exe::call_va!(0x1_4034_96f0, (this + 0x28) -> ());
    exe::call_va!(0x1_4034_96f0, (this + 0x10) -> ());
    exe::call_va!(0x1_411e_2f60, (this) -> ());
}

/// Original: `0x142093cc0` (52 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142093cc0(this: usize, a2: u32, a3: u32) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x08, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    wr::<u64>(this + 0x30, 0);
    wr::<u64>(this + 0x38, 0);
    wr::<u32>(this + 0x40, a2);
    wr::<u32>(this + 0x44, a3);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    this
}

pub unsafe fn diff_d3d_142093cc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0xaa_u8; 0x58];
    let mut buf_n = [0xaa_u8; 0x58];
    let ro = f_orig(buf_o.as_mut_ptr() as usize, 0x1234_5678, 0x9abc_def0);
    let rn = f_new(buf_n.as_mut_ptr() as usize, 0x1234_5678, 0x9abc_def0);
    crate::diff_eq!(ro, buf_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, buf_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(buf_o, buf_n, "initialized struct");
    Ok(1)
}

/// Original: `0x142094000` (30 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142094000(_this: usize, out: usize, a3: usize, a4: usize) -> usize {
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    exe::call_va!(0x1_4209_6ec0, (ctx, out, a3, a4) -> usize);
    out
}

/// Original: `0x142095af0` (103 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142095af0(
    this: usize,
    out: usize,
    _unused_r8: u64,
    src_desc: u64,
) -> usize {
    let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let rax = rd::<u64>(this + 0x10);
    let rbx = rd::<u64>(this);
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let off_4d = stride.wrapping_mul(0x4d);
    let off_3d = stride.wrapping_mul(0x3d);
    let dst_desc = rax.wrapping_add(off_4d).wrapping_add(off_3d);
    let vtbl = rd::<usize>(dev);
    let fn_ptr = rd::<usize>(vtbl + 0xc0);
    exe::call_va!(fn_ptr, (dev, 1u32, dst_desc, src_desc, 0u32) -> ());

    let stride2 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let res = rbx.wrapping_add(off_4d).wrapping_add(stride2.wrapping_mul(0x3d));
    wr::<u64>(out, res);
    out
}

/// Original: `0x142095ba0` (133 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142095ba0(this: usize) {
    for i in 0..4usize {
        wr::<u64>(this + 0x178 + i * 8, 0);
    }
    for i in 0..4usize {
        wr::<u32>(this + 0x1a8 + i * 4, u32::MAX);
    }
    wr::<u64>(this + 0x1b8, 0);
    wr::<u64>(this + 0x1c0, 0);
    wr::<u64>(this + 0x1c8, 0);
}

pub unsafe fn diff_d3d_142095ba0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0x5a_u8; 0x1d8];
    let mut buf_n = [0x5a_u8; 0x1d8];
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(buf_o, buf_n, "cleared range");
    Ok(1)
}

/// Original: `0x142095c30` (124 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142095c30(this: usize) {
    for i in 0..12usize {
        wr::<u64>(this + 0xc0 + i * 8, 0);
    }
    for i in 0..4usize {
        wr::<u32>(this + 0x130 + i * 4, u32::MAX);
    }
    wr::<u64>(this + 0x140, 0);
    wr::<u64>(this + 0x148, 0);
    wr::<u64>(this + 0x150, 0);
}

pub unsafe fn diff_d3d_142095c30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0x5a_u8; 0x160];
    let mut buf_n = [0x5a_u8; 0x160];
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(buf_o, buf_n, "cleared range");
    Ok(1)
}

/// Original: `0x1420964f0` (160 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420964f0(this: usize, do_copy: u8, count: u8) {
    let rbx = this + 0x30;
    exe::call_va!(0x1_4209_3d00, (this, 0x4du32, rbx, rbx + 0x10, rbx + 0x20) -> ());
    let rdx_bytes = (count as usize) << 2;
    exe::call_va!(0x1_4209_2930, (this, rdx_bytes, rbx + 0x08, rbx + 0x18, rbx + 0x28) -> ());
    if do_copy != 0 {
        let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
        let dst_desc = rd::<u64>(rbx + 0x10);
        let vtbl = rd::<usize>(dev);
        let src_desc = rd::<u64>(ctx + 0xb0);
        let fn_ptr = rd::<usize>(vtbl + 0xc0);
        exe::call_va!(fn_ptr, (dev, 0x4du32, dst_desc, src_desc, 0u32) -> ());
    }
}

/// Original: `0x1420965a0` (133 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420965a0(this: usize, do_init: u8, has_extra: u8, extra_frames: u8) {
    let frames = (extra_frames as u32).wrapping_add(2);
    let desc_count = frames.wrapping_mul(0x4d);
    exe::call_va!(0x1_4209_3d00, (this, desc_count, this, this + 0x10, this + 0x20) -> ());
    let extra_size = if has_extra != 0 { frames.wrapping_mul(4) } else { 0 };
    exe::call_va!(0x1_4209_2930, (0usize, extra_size, this + 0x08, this + 0x18, this + 0x28) -> ());
    if do_init != 0 {
        exe::call_va!(0x1_4209_2010, (this, frames as i32) -> ());
    }
}

/// Original: `0x142097400` (95 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142097400(target: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let base = tls0 + 0x467e0;
    let cur = rd::<usize>(base + 0xb00);
    if cur == target && cur != 0 {
        wr::<usize>(base + 0xb00, 0);
        wr::<u8>(base + 0xa80, 0);
    }
}

unsafe fn run_sl_marker_diff(
    orig: usize,
    new: usize,
    flag_va: u64,
    fn_slot_va: u64,
    arg_va: u64,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_marker_cb(marker: u32, ctx_ptr: usize) {
        wr::<u32>(ctx_ptr, marker);
        wr::<u32>(ctx_ptr + 4, rd::<u32>(ctx_ptr + 4).wrapping_add(1));
    }

    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_flag = exe::ptr::<u8>(flag_va) as usize;
    let g_fn = exe::ptr::<usize>(fn_slot_va) as usize;
    let g_arg = exe::ptr::<usize>(arg_va) as usize;

    let saved_flag = rd::<u8>(g_flag);
    let saved_fn = rd::<usize>(g_fn);
    let saved_arg = rd::<usize>(g_arg);

    // 1. Flag == 0 (no-op)
    let mut rec_o = [0u32; 2];
    let mut rec_n = [0u32; 2];
    wr::<u8>(g_flag, 0);
    wr::<usize>(g_fn, fake_marker_cb as *const () as usize);
    wr::<usize>(g_arg, rec_o.as_mut_ptr() as usize);
    f_orig();
    wr::<usize>(g_arg, rec_n.as_mut_ptr() as usize);
    f_new();

    // 2. Flag == 1 (calls cached callback)
    wr::<u8>(g_flag, 1);
    wr::<usize>(g_arg, rec_o.as_mut_ptr() as usize);
    f_orig();
    wr::<usize>(g_arg, rec_n.as_mut_ptr() as usize);
    f_new();

    wr::<u8>(g_flag, saved_flag);
    wr::<usize>(g_fn, saved_fn);
    wr::<usize>(g_arg, saved_arg);

    crate::diff_eq!(rec_o, rec_n, "sl marker callback record");
    Ok(2)
}

pub unsafe fn diff_d3d_1420914f0(orig: usize, new: usize) -> Result<u32, String> {
    run_sl_marker_diff(orig, new, 0x1_4385_83ca, 0x1_43c8_c608, 0x1_4385_8398)
}

pub unsafe fn diff_d3d_142091c00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_exp = exe::ptr::<u32>(0x1_4385_8470) as usize;
    let g_ca = exe::ptr::<u8>(0x1_4385_83ca) as usize;
    let s_exp = rd::<u32>(g_exp);
    let s_ca = rd::<u8>(g_ca);

    // 1. flag == 0
    wr::<u8>(g_ca, 0);
    wr::<u32>(g_exp, 7);
    f_orig(7);
    f_new(7);

    // 2. flag == 1, expected == 0
    wr::<u8>(g_ca, 1);
    wr::<u32>(g_exp, 0);
    f_orig(0);
    f_new(0);

    // 3. flag == 1, expected == 7, id != expected
    wr::<u32>(g_exp, 7);
    f_orig(8);
    f_new(8);

    wr::<u32>(g_exp, s_exp);
    wr::<u8>(g_ca, s_ca);
    Ok(3)
}

pub unsafe fn diff_d3d_142091c30(orig: usize, new: usize) -> Result<u32, String> {
    run_sl_marker_diff(orig, new, 0x1_4385_83ca, 0x1_43c8_c608, 0x1_4385_8388)
}

pub unsafe fn diff_d3d_142091ce0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_cb1(ctx_ptr: usize) {
        wr::<u32>(ctx_ptr, rd::<u32>(ctx_ptr).wrapping_add(0x42));
    }

    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_flag = exe::ptr::<u8>(0x1_4385_83c9) as usize;
    let g_fn = exe::ptr::<usize>(0x1_43c8_c610) as usize;
    let g_arg = exe::ptr::<usize>(0x1_4385_8388) as usize;

    let saved_flag = rd::<u8>(g_flag);
    let saved_fn = rd::<usize>(g_fn);
    let saved_arg = rd::<usize>(g_arg);

    let mut rec_o = 0u32;
    let mut rec_n = 0u32;
    wr::<usize>(g_fn, fake_cb1 as *const () as usize);

    wr::<u8>(g_flag, 0);
    wr::<usize>(g_arg, core::ptr::addr_of_mut!(rec_o) as usize);
    f_orig();
    wr::<usize>(g_arg, core::ptr::addr_of_mut!(rec_n) as usize);
    f_new();

    wr::<u8>(g_flag, 1);
    wr::<usize>(g_arg, core::ptr::addr_of_mut!(rec_o) as usize);
    f_orig();
    wr::<usize>(g_arg, core::ptr::addr_of_mut!(rec_n) as usize);
    f_new();

    wr::<u8>(g_flag, saved_flag);
    wr::<usize>(g_fn, saved_fn);
    wr::<usize>(g_arg, saved_arg);

    crate::diff_eq!(rec_o, rec_n, "d3d_142091ce0 cb");
    Ok(2)
}

pub unsafe fn diff_d3d_142091e20(orig: usize, new: usize) -> Result<u32, String> {
    run_sl_marker_diff(orig, new, 0x1_4385_83ca, 0x1_43c8_c608, 0x1_4385_8390)
}

pub unsafe fn diff_d3d_142091e90(orig: usize, new: usize) -> Result<u32, String> {
    run_sl_marker_diff(orig, new, 0x1_4385_83ca, 0x1_43c8_c608, 0x1_4385_8390)
}

unsafe extern "win64" fn fake_copy_descriptors_simple(
    dev: usize,
    num_desc: u32,
    dst_start: u64,
    src_start: u64,
    heap_type: u32,
) {
    let idx = (rd::<u64>(dev + 0x08) as usize).min(3);
    wr::<u64>(dev + 0x08, (idx + 1) as u64);
    let slot = dev + 0x10 + idx * 0x18;
    wr::<u64>(slot, (num_desc as u64) | ((heap_type as u64) << 32));
    wr::<u64>(slot + 0x08, dst_start);
    wr::<u64>(slot + 0x10, src_start);
}

pub unsafe fn diff_d3d_142092010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xc0 / 8] = fake_copy_descriptors_simple as *const () as usize;

    let mut dev_o = [0u64; 16];
    let mut dev_n = [0u64; 16];
    dev_o[0] = vtbl.as_ptr() as u64;
    dev_n[0] = vtbl.as_ptr() as u64;

    let mut ctx = [0u8; 0xc0];
    wr::<u64>(ctx.as_mut_ptr() as usize + 0xb0, 0x7777_0000);

    let mut this_obj = [0u8; 0x20];
    wr::<u64>(this_obj.as_mut_ptr() as usize + 0x10, 0x1000_0000);

    let g_stride = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let g_ctx = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let s_stride = rd::<u32>(g_stride);
    let s_dev = rd::<usize>(g_dev);
    let s_ctx = rd::<usize>(g_ctx);

    wr::<u32>(g_stride, 0x20);
    wr::<usize>(g_ctx, ctx.as_ptr() as usize);

    // 1. count <= 0
    f_orig(this_obj.as_ptr() as usize, 0);
    f_new(this_obj.as_ptr() as usize, 0);

    // 2. count == 2
    wr::<usize>(g_dev, dev_o.as_mut_ptr() as usize);
    f_orig(this_obj.as_ptr() as usize, 2);
    wr::<usize>(g_dev, dev_n.as_mut_ptr() as usize);
    f_new(this_obj.as_ptr() as usize, 2);

    wr::<u32>(g_stride, s_stride);
    wr::<usize>(g_dev, s_dev);
    wr::<usize>(g_ctx, s_ctx);

    crate::diff_eq!(dev_o[1..10], dev_n[1..10], "copy descriptors calls");
    Ok(2)
}

pub unsafe fn diff_d3d_142092380(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xc0 / 8] = fake_copy_descriptors_simple as *const () as usize;

    let mut dev_o = [0u64; 16];
    let mut dev_n = [0u64; 16];
    dev_o[0] = vtbl.as_ptr() as u64;
    dev_n[0] = vtbl.as_ptr() as u64;

    let mut this_obj = vec![0u8; 0x2b0];
    wr::<u64>(this_obj.as_mut_ptr() as usize + 0x08, 0x9999_aaaa);
    wr::<u32>(this_obj.as_mut_ptr() as usize + 0x278, 128);
    wr::<u64>(this_obj.as_mut_ptr() as usize + 0x280, 0x2000_0000);
    wr::<u64>(this_obj.as_mut_ptr() as usize + 0x2a0, 0x3000_0000);

    let mut target_o = [0u8; 0x40];
    let mut target_n = [0u8; 0x40];
    for t in [&mut target_o, &mut target_n] {
        wr::<u64>(t.as_mut_ptr() as usize + 0x08, 0x5000_0000);
        wr::<u32>(t.as_mut_ptr() as usize + 0x38, 64);
    }

    let g_stride = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let s_stride = rd::<u32>(g_stride);
    let s_dev = rd::<usize>(g_dev);

    wr::<u32>(g_stride, 0x20);
    wr::<usize>(g_dev, dev_o.as_mut_ptr() as usize);
    f_orig(this_obj.as_ptr() as usize, target_o.as_mut_ptr() as usize);
    wr::<usize>(g_dev, dev_n.as_mut_ptr() as usize);
    f_new(this_obj.as_ptr() as usize, target_n.as_mut_ptr() as usize);

    wr::<u32>(g_stride, s_stride);
    wr::<usize>(g_dev, s_dev);

    crate::diff_eq!(target_o, target_n, "target state");
    crate::diff_eq!(dev_o[1..10], dev_n[1..10], "copy descriptors calls");
    Ok(1)
}

pub unsafe fn diff_d3d_142092dc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut ctx_o = vec![0u8; 0x4d0];
    let mut ctx_n = vec![0u8; 0x4d0];
    wr::<u64>(ctx_o.as_mut_ptr() as usize + 0x4b8, 10);
    wr::<u64>(ctx_n.as_mut_ptr() as usize + 0x4b8, 10);

    let g_ctx = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let s_ctx = rd::<usize>(g_ctx);

    wr::<usize>(g_ctx, ctx_o.as_mut_ptr() as usize);
    f_orig(0, 5, 0xdead_beef_cafe_babe);
    wr::<usize>(g_ctx, ctx_n.as_mut_ptr() as usize);
    f_new(0, 5, 0xdead_beef_cafe_babe);

    wr::<usize>(g_ctx, s_ctx);

    crate::diff_eq!(ctx_o, ctx_n, "ctx state");
    Ok(1)
}

pub unsafe fn diff_d3d_142093030(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);

    let mut rsi_buf = vec![0u8; 0x290];
    wr::<u32>(rsi_buf.as_mut_ptr() as usize + 0x28, tid);
    wr::<u32>(rsi_buf.as_mut_ptr() as usize + 0x2c, 1);
    wr::<u64>(rsi_buf.as_mut_ptr() as usize + 0x280, 0x1111_2222_3333_4444);

    let a8_buf = [0u64, 0x5555_6666_7777_8888u64];

    let g_owner = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let g_rec = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let g_b8 = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let g_a8 = exe::ptr::<usize>(0x1_4397_fca8) as usize;

    let s_owner = rd::<u32>(g_owner);
    let s_rec = rd::<u32>(g_rec);
    let s_b8 = rd::<usize>(g_b8);
    let s_a8 = rd::<usize>(g_a8);

    wr::<u32>(g_owner, tid);
    wr::<u32>(g_rec, 1);
    wr::<usize>(g_b8, rsi_buf.as_mut_ptr() as usize);
    wr::<usize>(g_a8, a8_buf.as_ptr() as usize);

    let mut out_a_o = 0u64;
    let mut out_b_o = 0u64;
    let mut out_a_n = 0u64;
    let mut out_b_n = 0u64;

    f_orig(
        0,
        0,
        core::ptr::addr_of_mut!(out_a_o) as usize,
        core::ptr::addr_of_mut!(out_b_o) as usize,
    );
    f_new(
        0,
        0,
        core::ptr::addr_of_mut!(out_a_n) as usize,
        core::ptr::addr_of_mut!(out_b_n) as usize,
    );

    wr::<u32>(g_owner, s_owner);
    wr::<u32>(g_rec, s_rec);
    wr::<usize>(g_b8, s_b8);
    wr::<usize>(g_a8, s_a8);

    crate::diff_eq!((out_a_o, out_b_o), (out_a_n, out_b_n), "out_a and out_b");
    Ok(1)
}

pub unsafe fn diff_d3d_142093510(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    let mut rdi_o = [0u8; 0x30];
    let mut rdi_n = [0u8; 0x30];

    let g_owner = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let g_rec = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let g_b8 = exe::ptr::<usize>(0x1_4397_fcb8) as usize;

    let s_owner = rd::<u32>(g_owner);
    let s_rec = rd::<u32>(g_rec);
    let s_b8 = rd::<usize>(g_b8);

    wr::<u32>(g_owner, tid);
    wr::<u32>(g_rec, 1);

    wr::<usize>(g_b8, rdi_o.as_mut_ptr() as usize);
    f_orig();
    wr::<usize>(g_b8, rdi_n.as_mut_ptr() as usize);
    f_new();

    wr::<u32>(g_owner, s_owner);
    wr::<u32>(g_rec, s_rec);
    wr::<usize>(g_b8, s_b8);

    crate::diff_eq!(rdi_o, rdi_n, "rdi + 0x18");
    Ok(1)
}

pub unsafe fn diff_d3d_142095af0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xc0 / 8] = fake_copy_descriptors_simple as *const () as usize;

    let mut dev_o = [0u64; 16];
    let mut dev_n = [0u64; 16];
    dev_o[0] = vtbl.as_ptr() as u64;
    dev_n[0] = vtbl.as_ptr() as u64;

    let mut this_obj = [0u8; 0x20];
    wr::<u64>(this_obj.as_mut_ptr() as usize, 0x8000_0000);
    wr::<u64>(this_obj.as_mut_ptr() as usize + 0x10, 0x4000_0000);

    let g_stride = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let s_stride = rd::<u32>(g_stride);
    let s_dev = rd::<usize>(g_dev);

    let mut out_o = 0u64;
    let mut out_n = 0u64;

    wr::<u32>(g_stride, 0x20);
    wr::<usize>(g_dev, dev_o.as_mut_ptr() as usize);
    let ro = f_orig(
        this_obj.as_ptr() as usize,
        core::ptr::addr_of_mut!(out_o) as usize,
        0,
        0x1234_5678,
    );
    wr::<usize>(g_dev, dev_n.as_mut_ptr() as usize);
    let rn = f_new(
        this_obj.as_ptr() as usize,
        core::ptr::addr_of_mut!(out_n) as usize,
        0,
        0x1234_5678,
    );

    wr::<u32>(g_stride, s_stride);
    wr::<usize>(g_dev, s_dev);

    crate::diff_eq!(ro, core::ptr::addr_of_mut!(out_o) as usize, "ret orig");
    crate::diff_eq!(rn, core::ptr::addr_of_mut!(out_n) as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "out handle");
    crate::diff_eq!(dev_o[1..6], dev_n[1..6], "copy descriptors call");
    Ok(1)
}

pub unsafe fn diff_d3d_142097400(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x47400];
    let mut tls_n = vec![0u8; 0x47400];
    for tls in [&mut tls_o, &mut tls_n] {
        tls[0x70] = 1;
        wr::<usize>(tls.as_mut_ptr() as usize + 0x467e0 + 0xb00, 0x4444_5555);
        wr::<u8>(tls.as_mut_ptr() as usize + 0x467e0 + 0xa80, 1);
    }

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    // 1. Non-matching target -> unchanged
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    f_orig(0x9999);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    f_new(0x9999);
    wr::<usize>(tls_slot, saved_tls0);
    crate::diff_eq!(
        &tls_o[0x467e0 + 0xa80..0x467e0 + 0xb08],
        &tls_n[0x467e0 + 0xa80..0x467e0 + 0xb08],
        "non-matching target"
    );

    // 2. Matching target -> clears +0xb00 and +0xa80
    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    f_orig(0x4444_5555);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    f_new(0x4444_5555);
    wr::<usize>(tls_slot, saved_tls0);
    crate::diff_eq!(
        &tls_o[0x467e0 + 0xa80..0x467e0 + 0xb08],
        &tls_n[0x467e0 + 0xa80..0x467e0 + 0xb08],
        "matching target"
    );

    Ok(2)
}

pub unsafe fn diff_d3d_142091270(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_14204f860(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x11));
    }
    unsafe extern "win64" fn fake_142442de0(p: usize) {
        wr::<u32>(p + 4, rd::<u32>(p) ^ 0x55aa);
    }
    unsafe extern "win64" fn fake_1403496f0(p80: usize) {
        wr::<u64>(p80, 0xdead_beef_cafe_babe);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4204_f860, fake_14204f860 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, fake_142442de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4034_96f0, fake_1403496f0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut subs_o = [[1u32, 0u32], [2, 0], [3, 0], [4, 0], [5, 0]];
    let mut subs_n = subs_o;
    let mut obj_o = vec![0u8; 0x110];
    let mut obj_n = vec![0u8; 0x110];

    let offs = [0xe8usize, 0xf0, 0xf8, 0x100, 0x108];
    for (i, &off) in offs.iter().enumerate() {
        if i != 1 && i != 3 {
            wr::<usize>(obj_o.as_mut_ptr() as usize + off, subs_o[i].as_mut_ptr() as usize);
            wr::<usize>(obj_n.as_mut_ptr() as usize + off, subs_n[i].as_mut_ptr() as usize);
        }
    }

    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(subs_o, subs_n, "d3d_142091270 sub objects");
    crate::diff_eq!(&obj_o[0x80..0x88], &obj_n[0x80..0x88], "d3d_142091270 +0x80");
    Ok(1)
}

pub unsafe fn diff_d3d_1420917a0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_sl_set_feature_loaded(feature: u32, loaded: usize) -> u64 {
        ((feature as u64) << 32) | ((loaded as u64) ^ 0x1234_5678)
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_7358, fake_sl_set_feature_loaded as *const ())?;

    type F = unsafe extern "win64" fn() -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let ro = f_orig();
    let rn = f_new();
    crate::diff_eq!(ro, rn, "d3d_1420917a0 ret");
    Ok(1)
}

pub unsafe fn diff_d3d_142091b40(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_get_adapter_luid(dev: usize, out_luid: usize) -> usize {
        wr::<u64>(out_luid, rd::<u64>(dev + 8));
        out_luid
    }
    unsafe extern "win64" fn fake_sl_is_feature_supported(feature: u32, desc: usize) -> u32 {
        let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        let luid_ptr = rd::<usize>(desc + 0x20);
        let luid = rd::<u64>(luid_ptr);
        let mut h = (feature as u64)
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(desc))
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(desc + 0x08) as u64)
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(desc + 0x0c) as u64)
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(desc + 0x10) as u64)
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(desc + 0x14) as u64)
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(desc + 0x18))
            .wrapping_mul(31)
            .wrapping_add(luid)
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(desc + 0x28) as u64)
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(desc + 0x30));
        if luid_ptr == 0 {
            h = 0;
        }
        wr::<u64>(dev + 0x10, h);
        if (luid & 1) == 0 { 0 } else { 5 }
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_7340, fake_sl_is_feature_supported as *const ())?;

    type F = unsafe extern "win64" fn() -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 48];
    vtbl[0x158 / 8] = fake_get_adapter_luid as *const () as usize;

    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let s_dev = rd::<usize>(g_dev);
    let mut ran = 0u32;

    for luid in [0x1122_3344_5566_7788u64, 0x1122_3344_5566_7789u64] {
        let mut dev_o = [vtbl.as_ptr() as u64, luid, 0u64];
        let mut dev_n = [vtbl.as_ptr() as u64, luid, 0u64];

        wr::<usize>(g_dev, dev_o.as_mut_ptr() as usize);
        let ro = f_orig();
        wr::<usize>(g_dev, dev_n.as_mut_ptr() as usize);
        let rn = f_new();

        crate::diff_eq!(ro, rn, "d3d_142091b40 ret");
        crate::diff_eq!(dev_o[2], dev_n[2], "d3d_142091b40 desc hash");
        ran += 1;
    }

    wr::<usize>(g_dev, s_dev);
    Ok(ran)
}

pub unsafe fn diff_d3d_142091ca0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_token_get(token: usize) -> u32 {
        rd::<u32>(token + 8)
    }
    unsafe extern "win64" fn fake_sl_get_new_frame_token(slot_88: usize, next_ptr: usize) -> u32 {
        let next = rd::<u32>(next_ptr);
        wr::<usize>(slot_88, (next as usize) | 0xabcd_0000);
        0
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_7310, fake_sl_get_new_frame_token as *const ())?;

    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vtbl = [fake_token_get as *const () as usize];
    let token_obj = [vtbl.as_ptr() as u64, 41u64];
    let token_ptr = token_obj.as_ptr() as usize;

    let slot_88 = exe::ptr::<usize>(0x1_4385_8388) as usize;
    let slot_90 = exe::ptr::<usize>(0x1_4385_8390) as usize;
    let s_88 = rd::<usize>(slot_88);
    let s_90 = rd::<usize>(slot_90);

    wr::<usize>(slot_88, token_ptr);
    wr::<usize>(slot_90, 0);
    f_orig();
    let o_state = (rd::<usize>(slot_88), rd::<usize>(slot_90));

    wr::<usize>(slot_88, token_ptr);
    wr::<usize>(slot_90, 0);
    f_new();
    let n_state = (rd::<usize>(slot_88), rd::<usize>(slot_90));

    wr::<usize>(slot_88, s_88);
    wr::<usize>(slot_90, s_90);

    crate::diff_eq!(o_state, n_state, "d3d_142091ca0 token state");
    Ok(1)
}

pub unsafe fn diff_d3d_142091d60(orig: usize, new: usize) -> Result<u32, String> {
    static REC: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    unsafe extern "win64" fn fake_sl_init(pref: usize, sdk_ver: u64) -> u32 {
        let features = rd::<usize>(pref + 0x60);
        let mut h = sdk_ver;
        for &off in &[0x00usize, 0x08, 0x10, 0x18, 0x28, 0x38, 0x40, 0x48, 0x50, 0x58, 0x6c, 0x78, 0x80] {
            h = h.wrapping_mul(1099511628211).wrapping_add(rd::<u64>(pref + off));
        }
        for &off in &[0x24usize, 0x30, 0x68, 0x88] {
            h = h.wrapping_mul(1099511628211).wrapping_add(rd::<u32>(pref + off) as u64);
        }
        h = h.wrapping_mul(1099511628211).wrapping_add(rd::<u8>(pref + 0x20) as u64);
        for i in 0..4usize {
            h = h.wrapping_mul(1099511628211).wrapping_add(rd::<u32>(features + i * 4) as u64);
        }
        REC.store(h, std::sync::atomic::Ordering::SeqCst);
        0
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_7318, fake_sl_init as *const ())?;

    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    REC.store(0, std::sync::atomic::Ordering::SeqCst);
    f_orig();
    let ho = REC.swap(0, std::sync::atomic::Ordering::SeqCst);

    f_new();
    let hn = REC.swap(0, std::sync::atomic::Ordering::SeqCst);

    crate::diff_eq!(ho, hn, "d3d_142091d60 slInit hash");
    Ok(1)
}

pub unsafe fn diff_d3d_142093c80(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_1401de830(p: usize) {
        wr::<u64>(p, rd::<u64>(p).wrapping_add(0x1111));
    }
    unsafe extern "win64" fn fake_1403496f0(p: usize) {
        wr::<u64>(p, rd::<u64>(p).wrapping_add(0x2222));
    }
    unsafe extern "win64" fn fake_1411e2f60(p: usize) {
        wr::<u64>(p, rd::<u64>(p).wrapping_add(0x3333));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_e830, fake_1401de830 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4034_96f0, fake_1403496f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_2f60, fake_1411e2f60 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = [0x01u8; 0x60];
    let mut obj_n = [0x01u8; 0x60];
    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(obj_o, obj_n, "d3d_142093c80 dtor fields");
    Ok(1)
}

pub unsafe fn diff_d3d_142094000(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_142096ec0(ctx: usize, out: usize, a3: usize, a4: usize) -> usize {
        wr::<u64>(out, (ctx as u64) ^ (a3 as u64) ^ (a4 as u64));
        wr::<u64>(out + 8, (a3 as u64).wrapping_add(a4 as u64));
        out
    }

    let _m = crate::difftest::Mock::new(0x1_4209_6ec0, fake_142096ec0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ctx = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let s_ctx = rd::<usize>(g_ctx);
    wr::<usize>(g_ctx, 0x5555_0000);

    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    let ro = f_orig(0, out_o.as_mut_ptr() as usize, 0x1234, 0x5678);
    let rn = f_new(0, out_n.as_mut_ptr() as usize, 0x1234, 0x5678);

    wr::<usize>(g_ctx, s_ctx);

    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "d3d_142094000 out");
    Ok(1)
}

unsafe extern "win64" fn fake_alloc_desc_142093d00(
    _this: usize,
    count: u32,
    out_gpu: usize,
    out_cpu: usize,
    out_chunk: usize,
) {
    wr::<u64>(out_gpu, 0x1000_0000 | (count as u64));
    wr::<u64>(out_cpu, 0x2000_0000 | (count as u64));
    wr::<u64>(out_chunk, 0x3000_0000 | (count as u64));
}

unsafe extern "win64" fn fake_alloc_extra_142092930(
    _this: usize,
    bytes: usize,
    out_gpu: usize,
    out_cpu: usize,
    out_chunk: usize,
) {
    wr::<u64>(out_gpu, 0x4000_0000 | (bytes as u64));
    wr::<u64>(out_cpu, 0x5000_0000 | (bytes as u64));
    wr::<u64>(out_chunk, 0x6000_0000 | (bytes as u64));
}

pub unsafe fn diff_d3d_1420964f0(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4209_3d00, fake_alloc_desc_142093d00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_2930, fake_alloc_extra_142092930 as *const ())?;

    type F = unsafe extern "win64" fn(usize, u8, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xc0 / 8] = fake_copy_descriptors_simple as *const () as usize;

    let mut dev_o = [0u64; 16];
    let mut dev_n = [0u64; 16];
    dev_o[0] = vtbl.as_ptr() as u64;
    dev_n[0] = vtbl.as_ptr() as u64;

    let mut ctx = [0u8; 0xc0];
    wr::<u64>(ctx.as_mut_ptr() as usize + 0xb0, 0x7777_8888);

    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let g_ctx = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let s_dev = rd::<usize>(g_dev);
    let s_ctx = rd::<usize>(g_ctx);
    wr::<usize>(g_ctx, ctx.as_ptr() as usize);

    let mut ran = 0u32;
    for (do_copy, count) in [(0u8, 3u8), (1u8, 5u8)] {
        let mut obj_o = [0u8; 0x60];
        let mut obj_n = [0u8; 0x60];

        wr::<usize>(g_dev, dev_o.as_mut_ptr() as usize);
        f_orig(obj_o.as_mut_ptr() as usize, do_copy, count);

        wr::<usize>(g_dev, dev_n.as_mut_ptr() as usize);
        f_new(obj_n.as_mut_ptr() as usize, do_copy, count);

        crate::diff_eq!(obj_o, obj_n, "d3d_1420964f0 obj");
        crate::diff_eq!(dev_o[1..8], dev_n[1..8], "d3d_1420964f0 dev calls");
        ran += 1;
    }

    wr::<usize>(g_dev, s_dev);
    wr::<usize>(g_ctx, s_ctx);
    Ok(ran)
}

pub unsafe fn diff_d3d_1420965a0(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4209_3d00, fake_alloc_desc_142093d00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_2930, fake_alloc_extra_142092930 as *const ())?;

    type F = unsafe extern "win64" fn(usize, u8, u8, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xc0 / 8] = fake_copy_descriptors_simple as *const () as usize;

    let mut dev_o = [0u64; 16];
    let mut dev_n = [0u64; 16];
    dev_o[0] = vtbl.as_ptr() as u64;
    dev_n[0] = vtbl.as_ptr() as u64;

    let mut ctx = [0u8; 0xc0];
    wr::<u64>(ctx.as_mut_ptr() as usize + 0xb0, 0x9999_aaaa);

    let g_stride = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let g_dev = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let g_ctx = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let s_stride = rd::<u32>(g_stride);
    let s_dev = rd::<usize>(g_dev);
    let s_ctx = rd::<usize>(g_ctx);

    wr::<u32>(g_stride, 0x20);
    wr::<usize>(g_ctx, ctx.as_ptr() as usize);

    let mut ran = 0u32;
    for (do_init, has_extra, extra_frames) in [(0u8, 0u8, 1u8), (1u8, 1u8, 1u8)] {
        let mut obj_o = [0u8; 0x38];
        let mut obj_n = [0u8; 0x38];

        wr::<usize>(g_dev, dev_o.as_mut_ptr() as usize);
        f_orig(obj_o.as_mut_ptr() as usize, do_init, has_extra, extra_frames);

        wr::<usize>(g_dev, dev_n.as_mut_ptr() as usize);
        f_new(obj_n.as_mut_ptr() as usize, do_init, has_extra, extra_frames);

        crate::diff_eq!(obj_o, obj_n, "d3d_1420965a0 obj");
        crate::diff_eq!(dev_o[1..12], dev_n[1..12], "d3d_1420965a0 dev calls");
        ran += 1;
    }

    wr::<u32>(g_stride, s_stride);
    wr::<usize>(g_dev, s_dev);
    wr::<usize>(g_ctx, s_ctx);
    Ok(ran)
}

// REGISTRY:
// Replacement::verified(0x142090de0, "d3d::d3d_142090de0", crate::ns::rend::d3d_constants_a::d3d_142090de0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142090de0),
// Replacement::verified(0x142090e00, "d3d::d3d_142090e00", crate::ns::rend::d3d_constants_a::d3d_142090e00 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142090e00),
// Replacement::verified(0x1420911c0, "d3d::d3d_1420911c0", crate::ns::rend::d3d_constants_a::d3d_1420911c0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_1420911c0),
// Replacement::verified(0x1420911e0, "d3d::d3d_1420911e0", crate::ns::rend::d3d_constants_a::d3d_1420911e0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_1420911e0),
// Replacement::verified(0x142091200, "d3d::d3d_142091200", crate::ns::rend::d3d_constants_a::d3d_142091200 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091200),
// Replacement::verified(0x142091210, "d3d::d3d_142091210", crate::ns::rend::d3d_constants_a::d3d_142091210 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091210),
// Replacement::verified_throws(0x142091270, "d3d::d3d_142091270", crate::ns::rend::d3d_constants_a::d3d_142091270 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_a::diff_d3d_142091270),
// Replacement::verified(0x142091480, "d3d::d3d_142091480", crate::ns::rend::d3d_constants_a::d3d_142091480 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091480),
// Replacement::verified(0x1420914d0, "d3d::d3d_1420914d0", crate::ns::rend::d3d_constants_a::d3d_1420914d0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_1420914d0),
// Replacement::verified(0x1420914f0, "d3d::d3d_1420914f0", crate::ns::rend::d3d_constants_a::d3d_1420914f0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_1420914f0),
// Replacement::verified(0x1420917a0, "d3d::d3d_1420917a0", crate::ns::rend::d3d_constants_a::d3d_1420917a0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_1420917a0),
// Replacement::verified(0x142091b10, "d3d::d3d_142091b10", crate::ns::rend::d3d_constants_a::d3d_142091b10 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091b10),
// Replacement::verified(0x142091b40, "d3d::d3d_142091b40", crate::ns::rend::d3d_constants_a::d3d_142091b40 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091b40),
// Replacement::verified(0x142091bd0, "d3d::d3d_142091bd0", crate::ns::rend::d3d_constants_a::d3d_142091bd0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091bd0),
// Replacement::verified(0x142091c00, "d3d::d3d_142091c00", crate::ns::rend::d3d_constants_a::d3d_142091c00 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091c00),
// Replacement::verified(0x142091c30, "d3d::d3d_142091c30", crate::ns::rend::d3d_constants_a::d3d_142091c30 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091c30),
// Replacement::verified(0x142091ca0, "d3d::d3d_142091ca0", crate::ns::rend::d3d_constants_a::d3d_142091ca0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091ca0),
// Replacement::verified(0x142091ce0, "d3d::d3d_142091ce0", crate::ns::rend::d3d_constants_a::d3d_142091ce0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091ce0),
// Replacement::verified(0x142091d60, "d3d::d3d_142091d60", crate::ns::rend::d3d_constants_a::d3d_142091d60 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091d60),
// Replacement::verified(0x142091e20, "d3d::d3d_142091e20", crate::ns::rend::d3d_constants_a::d3d_142091e20 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091e20),
// Replacement::verified(0x142091e90, "d3d::d3d_142091e90", crate::ns::rend::d3d_constants_a::d3d_142091e90 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142091e90),
// Replacement::verified(0x142092010, "d3d::d3d_142092010", crate::ns::rend::d3d_constants_a::d3d_142092010 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142092010),
// Replacement::verified(0x142092090, "d3d::d3d_142092090", crate::ns::rend::d3d_constants_a::d3d_142092090 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142092090),
// Replacement::verified(0x142092380, "d3d::d3d_142092380", crate::ns::rend::d3d_constants_a::d3d_142092380 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142092380),
// Replacement::verified(0x142092760, "d3d::d3d_142092760", crate::ns::rend::d3d_constants_a::d3d_142092760 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142092760),
// Replacement::verified(0x142092dc0, "d3d::d3d_142092dc0", crate::ns::rend::d3d_constants_a::d3d_142092dc0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142092dc0),
// Replacement::verified(0x142093030, "d3d::d3d_142093030", crate::ns::rend::d3d_constants_a::d3d_142093030 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142093030),
// Replacement::verified(0x142093510, "d3d::d3d_142093510", crate::ns::rend::d3d_constants_a::d3d_142093510 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142093510),
// Replacement::verified(0x142093930, "d3d::d3d_142093930", crate::ns::rend::d3d_constants_a::d3d_142093930 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142093930),
// Replacement::verified(0x1420939d0, "d3d::d3d_1420939d0", crate::ns::rend::d3d_constants_a::d3d_1420939d0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_1420939d0),
// Replacement::verified(0x142093a30, "d3d::d3d_142093a30", crate::ns::rend::d3d_constants_a::d3d_142093a30 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142093a30),
// Replacement::verified(0x142093c80, "d3d::d3d_142093c80", crate::ns::rend::d3d_constants_a::d3d_142093c80 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142093c80),
// Replacement::verified(0x142093cc0, "d3d::d3d_142093cc0", crate::ns::rend::d3d_constants_a::d3d_142093cc0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142093cc0),
// Replacement::verified_throws(0x142094000, "d3d::d3d_142094000", crate::ns::rend::d3d_constants_a::d3d_142094000 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_a::diff_d3d_142094000),
// Replacement::verified(0x142095af0, "d3d::d3d_142095af0", crate::ns::rend::d3d_constants_a::d3d_142095af0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142095af0),
// Replacement::verified(0x142095ba0, "d3d::d3d_142095ba0", crate::ns::rend::d3d_constants_a::d3d_142095ba0 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142095ba0),
// Replacement::verified(0x142095c30, "d3d::d3d_142095c30", crate::ns::rend::d3d_constants_a::d3d_142095c30 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142095c30),
// Replacement::verified_throws(0x1420964f0, "d3d::d3d_1420964f0", crate::ns::rend::d3d_constants_a::d3d_1420964f0 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_a::diff_d3d_1420964f0),
// Replacement::verified_throws(0x1420965a0, "d3d::d3d_1420965a0", crate::ns::rend::d3d_constants_a::d3d_1420965a0 as *const (), Throws::Discard, crate::ns::rend::d3d_constants_a::diff_d3d_1420965a0),
// Replacement::verified(0x142097400, "d3d::d3d_142097400", crate::ns::rend::d3d_constants_a::d3d_142097400 as *const (), crate::ns::rend::d3d_constants_a::diff_d3d_142097400),

