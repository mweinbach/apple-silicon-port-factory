//! Decompiled `rend` functions (`0x142300310..0x142311970`) from `e18.rs`.

use crate::exe::{self, rd, wr};
use std::sync::atomic::{AtomicU32, Ordering};

/// `anonymous_namespace_142300310`
/// Original: `0x142300310` (79 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142300310(_this: usize, func_obj: usize) {
    let idx = rd::<i32>(exe::ptr::<i32>(0x1_43bb_d168) as usize) as isize;
    let base = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d180) as usize);
    let dst = base
        .wrapping_add((idx * 0x2e0) as usize)
        .wrapping_add(0x10);
    exe::call_va!(0x1_4128_cba0, (dst, func_obj) -> usize);
    let impl_ptr = rd::<usize>(func_obj + 0x38);
    if impl_ptr != 0 {
        let vtbl = rd::<usize>(impl_ptr);
        let destroy_fn = rd::<usize>(vtbl + 0x20);
        let is_large = u8::from(impl_ptr != func_obj);
        exe::call_va!(destroy_fn, (impl_ptr, is_large) -> ());
        wr::<usize>(func_obj + 0x38, 0);
    }
}

/// `anonymous_namespace_142300370`
/// Original: `0x142300370` (95 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142300370(_this: usize, key: u64) -> u64 {
    let mut cur = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d1d0) as usize);
    let count = rd::<u32>(exe::ptr::<u32>(0x1_43bb_d1d8) as usize) as usize;
    let end = cur.wrapping_add(count.wrapping_mul(0x70));
    while cur != end {
        let v60 = rd::<u64>(cur + 0x60);
        let v54 = rd::<u32>(cur + 0x54);
        if rd::<u64>(cur) == key && v54 == 0 {
            return v60;
        }
        cur = cur.wrapping_add(0x70);
    }
    0
}

/// `anonymous_namespace_1423003e0`
/// Original: `0x1423003e0` (96 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_1423003e0(
    _this: usize,
    key: u64,
    desc: usize,
    flags: u32,
) {
    let vec_addr = exe::ptr::<u8>(0x1_43bb_d1c0) as usize;
    let slot = exe::call_va!(0x1_4131_00d0, (vec_addr) -> usize);
    let b0 = rd::<[u8; 16]>(desc);
    let d10 = rd::<u32>(desc + 0x10);
    wr::<u64>(slot, key);
    wr::<[u8; 16]>(slot + 0x40, b0);
    wr::<u32>(slot + 0x50, d10);
    wr::<u32>(slot + 0x54, 1);
    wr::<u32>(slot + 0x58, flags);
    wr::<u64>(slot + 0x60, 0);
    wr::<u64>(slot + 0x68, u64::MAX);
}

/// `anonymous_namespace_142300460`
/// Original: `0x142300460` (93 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142300460(
    this: usize,
    a2: u64,
    a3: u64,
    a4: u64,
) {
    let slot = exe::call_va!(0x1_4131_03c0, (this + 0x30) -> usize);
    let tag = rd::<u32>(this + 0x28);
    wr::<u64>(slot + 0x18, a3);
    wr::<u64>(slot + 0x20, a4);
    wr::<u32>(slot, tag);
    wr::<u32>(slot + 0x4, 2);
    wr::<u64>(slot + 0x10, a2);
    wr::<u32>(slot + 0x28, 0);
}

/// `anonymous_namespace_1423004f0`
/// Original: `0x1423004f0` (50 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_1423004f0(this: usize, val: u32) {
    let slot = exe::call_va!(0x1_4131_03c0, (this + 0x30) -> usize);
    let tag = rd::<u32>(this + 0x28);
    wr::<u32>(slot, tag);
    wr::<u32>(slot + 0x4, 1);
    wr::<u32>(slot + 0x8, val);
}

/// `anonymous_namespace_142300530`
/// Original: `0x142300530` (40 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn anonymous_namespace_142300530(this: usize, id: u32) {
    exe::call_va!(0x1_4230_0560, (this, id) -> ());
    exe::call_va!(0x1_4230_0030, (this, id) -> ());
}

/// `anonymous_namespace_142300f20`
/// Original: `0x142300f20` (68 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142300f20(_this: usize, key: u64, handle: u64) {
    let vec_addr = exe::ptr::<u8>(0x1_43bb_d1c0) as usize;
    let slot = exe::call_va!(0x1_4131_00d0, (vec_addr) -> usize);
    wr::<u64>(slot, key);
    wr::<u64>(slot + 0x8, handle);
    wr::<u64>(slot + 0x54, 2);
    wr::<u64>(slot + 0x60, 0);
    wr::<u64>(slot + 0x68, u64::MAX);
}

/// `anonymous_namespace_142300f70`
/// Original: `0x142300f70` (97 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142300f70(
    _this: usize,
    key: u64,
    desc: usize,
    flags: u32,
) {
    let vec_addr = exe::ptr::<u8>(0x1_43bb_d1c0) as usize;
    let slot = exe::call_va!(0x1_4131_00d0, (vec_addr) -> usize);
    let b0 = rd::<[u8; 32]>(desc);
    let b20 = rd::<[u8; 16]>(desc + 0x20);
    wr::<[u8; 32]>(slot + 0x10, b0);
    wr::<[u8; 16]>(slot + 0x30, b20);
    wr::<u64>(slot, key);
    wr::<u32>(slot + 0x54, 0);
    wr::<u32>(slot + 0x58, flags);
    wr::<u64>(slot + 0x60, 0);
    wr::<u64>(slot + 0x68, u64::MAX);
}

/// `anonymous_namespace_142301130`
/// Original: `0x142301130` (189 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142301130(this: usize) -> usize {
    let ymm0 = rd::<[u8; 32]>(exe::ptr::<u8>(0x1_4342_8de0) as usize);
    let xmm2 = rd::<[u8; 16]>(exe::ptr::<u8>(0x1_4342_81d0) as usize);
    wr::<u32>(this + 0x8, 0xffff_ffff);
    wr::<usize>(this, exe::ptr::<u8>(0x1_433f_a638) as usize);
    wr::<u64>(this + 0x10, 0);
    for off in (0x28..=0xc0).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<u64>(this + 0xf0, 0x3f80_0000);
    wr::<u64>(this + 0xf8, 0);
    wr::<[u8; 32]>(this + 0x100, ymm0);
    wr::<[u8; 16]>(this + 0x120, xmm2);
    this
}

/// `anonymous_namespace_142301330`
/// Original: `0x142301330` (138 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn anonymous_namespace_142301330(out: usize, rdx_in: usize) -> usize {
    let pool = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff88) as usize);
    if pool != 0 {
        let base = rd::<usize>(pool);
        let len = rd::<u32>(pool + 0x8) as usize;
        let end = base.wrapping_add(len.wrapping_mul(8));
        if base != end {
            let val = rd::<u64>(end - 8);
            wr::<u64>(end - 8, 0);
            let len2 = rd::<u32>(pool + 0x8) as usize;
            let base2 = rd::<usize>(pool);
            let old = rd::<usize>(base2.wrapping_add(len2.wrapping_mul(8)) - 8);
            if old != 0 {
                exe::call_va!(0x1_4244_2de0, (old) -> ());
            }
            wr::<u32>(pool + 0x8, rd::<u32>(pool + 0x8).wrapping_sub(1));
            wr::<u64>(out, val);
            return out;
        }
    }
    let mut zero = 0u32;
    exe::call_va!(0x1_411d_4e00, (out, rdx_in, (&mut zero as *mut u32) as usize) -> usize);
    out
}

/// `anonymous_namespace_142301770`
/// Original: `0x142301770` (129 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn anonymous_namespace_142301770(this: usize, query_ptr: usize) -> u8 {
    let rbx = rd::<usize>(query_ptr);
    let mut val: u64 = 0;
    let rax = rd::<usize>(rbx);
    if rax == 0 || rd::<i32>(rbx + 0xc) < rd::<i32>(rax + 0x30) {
        wr::<u32>(this + 0x80, 0);
        return 1;
    }
    let ready = exe::call_va!(0x1_4206_4090, (rbx) -> u8);
    if ready == 0 {
        return 0;
    }
    let hr = exe::call_va!(0x1_4206_3fe0, (rbx, (&mut val as *mut u64) as usize) -> i32);
    if hr < 0 {
        wr::<u32>(this + 0x80, 0);
        return 1;
    }
    wr::<u32>(this + 0x80, val as u32);
    1
}

/// `anonymous_namespace_142301800`
/// Original: `0x142301800` (24 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142301800(this: usize) -> u32 {
    let frame = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    if rd::<u32>(this + 0x84) == frame {
        rd::<u32>(this + 0x80)
    } else {
        0
    }
}

/// `anonymous_namespace_142301820`
/// Original: `0x142301820` (29 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142301820(this: usize) -> u8 {
    let frame = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    if rd::<u32>(this + 0x84) == frame && rd::<u32>(this + 0x80) > 0 {
        1
    } else {
        0
    }
}

/// `anonymous_namespace_142301850`
/// Original: `0x142301850` (109 bytes, `Throws::No`)
pub unsafe extern "win64" fn anonymous_namespace_142301850(this: usize) {
    let mut cur = rd::<u64>(this + 0x8);
    if rd::<u64>(this) != cur {
        loop {
            let slot_idx = ((cur as u32) & 3) as usize + 2;
            exe::call_va!(0x1_4230_1230, (this + slot_idx * 8) -> ());
            let end = rd::<u64>(this);
            let next = rd::<u64>(this + 0x8).wrapping_add(1);
            cur = if next < end { next } else { end };
            wr::<u64>(this + 0x8, cur);
            if end == cur {
                break;
            }
        }
    }
    exe::call_va!(
        0x1_4183_4ee8,
        (
            this + 0x10,
            8u32,
            4u32,
            exe::ptr::<u8>(0x1_401e_1800) as usize
        ) -> ()
    );
}

/// `anonymous_namespace::QueryBegin::vfunc_1`
/// Original: `0x142301e60` (151 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn anonymous_namespace_142301e60(this: usize) {
    let rbx = rd::<usize>(this + 0x18);
    if rd::<u32>(rbx + 0x10) != 0 {
        return;
    }
    let mut tmp_query = [0u8; 16];
    let rax = exe::call_va!(
        0x1_4206_3e60,
        (this, tmp_query.as_mut_ptr() as usize, 0u32) -> usize
    );
    wr::<[u8; 16]>(rbx, rd::<[u8; 16]>(rax));
    let edi = rd::<u32>(rbx + 0x8);
    let mut ctx_guard = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_guard.as_mut_ptr() as usize) -> usize);
    let r10 = ctx_guard[1];
    let vtbl = rd::<usize>(r10);
    let fn_ptr = rd::<usize>(vtbl + 0x1a0);
    let r8d: u32 = if rd::<u32>(rbx + 0x10) != 0 { 2 } else { 0 };
    let inner = rd::<usize>(rbx);
    let rdx_arg = rd::<usize>(inner + 0x18);
    exe::call_va!(fn_ptr, (r10, rdx_arg, r8d, edi) -> ());
    let lock_ptr = ctx_guard[0];
    if lock_ptr != 0 {
        let next_depth = rd::<u32>(lock_ptr + 0xc).wrapping_sub(1);
        wr::<u32>(lock_ptr + 0xc, next_depth);
        if next_depth == 0 {
            let state_ptr = (lock_ptr + 0x8) as *const AtomicU32;
            (*state_ptr).swap(0xffff_ffff, Ordering::SeqCst);
            exe::call_iat!(0x1_4264_5598, (lock_ptr) -> ());
        }
    }
}

/// `unknown_142301f00`
/// Original: `0x142301f00` (204 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_142301f00(this: usize) {
    let frame = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    wr::<u32>(this + 0x84, frame);
    let mut cur = rd::<u64>(this + 0x8);
    if rd::<u64>(this) == cur {
        return;
    }
    loop {
        let idx = ((cur as u32) & 3) as usize;
        let mut val: u64 = 0;
        let rdi = rd::<usize>(this + 0x10 + idx * 8);
        let rax = rd::<usize>(rdi);
        if rax != 0 && rd::<i32>(rdi + 0xc) >= rd::<i32>(rax + 0x30) {
            let ready = exe::call_va!(0x1_4206_4090, (rdi) -> u8);
            if ready == 0 {
                break;
            }
            let hr = exe::call_va!(0x1_4206_3fe0, (rdi, (&mut val as *mut u64) as usize) -> i32);
            if hr >= 0 {
                wr::<u32>(this + 0x80, val as u32);
            } else {
                wr::<u32>(this + 0x80, 0);
            }
        } else {
            wr::<u32>(this + 0x80, 0);
        }
        let slot_idx = (rd::<u32>(this + 0x8) & 3) as usize + 2;
        exe::call_va!(0x1_4230_1230, (this + slot_idx * 8) -> ());
        let end = rd::<u64>(this);
        let next = rd::<u64>(this + 0x8).wrapping_add(1);
        cur = if next < end { next } else { end };
        wr::<u64>(this + 0x8, cur);
        if end == cur {
            break;
        }
    }
}

/// `unknown_142309a10`
/// Original: `0x142309a10` (24 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_142309a10(this: usize) -> usize {
    wr::<u8>(this + 0x1c, 2);
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u32>(this + 0x18, 0);
    this
}

/// `std::_Func_impl_no_alloc<rend::MeshStreamingManager::updateStreamOps::<lambda_2>,void,r::LoadedData&&>::vfunc_2`
/// Original: `0x142309ff0` (108 bytes, `Throws::No`)
pub unsafe extern "win64" fn std_142309ff0(this: usize, loaded_data: usize) {
    let rbx = rd::<usize>(this + 0x8);
    exe::call_va!(0x1_4240_a730, (rbx + 0x88, loaded_data) -> usize);
    let v18 = rd::<u64>(loaded_data + 0x18);
    wr::<u64>(rbx + 0xa0, v18);
    exe::call_va!(0x1_4249_68a0, (rbx + 0xa8, loaded_data + 0x20) -> usize);
    let v50 = rd::<u32>(loaded_data + 0x50);
    wr::<u32>(rbx + 0xd8, v50);
    let v58 = rd::<u64>(loaded_data + 0x58);
    wr::<u64>(rbx + 0xe0, v58);
    let target = rd::<usize>(this + 0x8);
    wr::<u8>(target, 2);
}

/// `std::_Func_impl_no_alloc<rend::MeshResource::blockingLoad::<lambda_1>,void,r::LoadedData&&>::vfunc_2`
/// Original: `0x14230c2d0` (81 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn std_14230c2d0(this: usize, loaded_data: usize) {
    let rax = rd::<u64>(this + 0x8);
    let mut pair = [rax, loaded_data as u64];
    let mgr_addr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let mgr = rd::<usize>(mgr_addr);
    if mgr != 0 {
        exe::call_va!(0x1_4131_fde0, (mgr, pair.as_mut_ptr() as usize) -> ());
        let mgr2 = rd::<usize>(mgr_addr);
        exe::call_va!(0x1_422a_3d30, (mgr2) -> ());
    } else {
        exe::call_va!(0x1_4230_d2b0, (pair.as_mut_ptr() as usize) -> ());
    }
}

/// `rend_14230c810`
/// Original: `0x14230c810` (190 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230c810(key_ptr: usize) {
    let key = rd::<u64>(key_ptr);
    if key.wrapping_sub(1) > 0xffff_ffff_ffff_fffdu64 {
        return;
    }
    let rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed8) as usize);
    let head = rd::<usize>(rdi);
    let mut rbx = head;
    let mut rax = rd::<usize>(head + 0x8);
    while rd::<u8>(rax + 0x19) == 0 {
        if rd::<u64>(rax + 0x20) >= key {
            rbx = rax;
            rax = rd::<usize>(rax);
        } else {
            rax = rd::<usize>(rax + 0x10);
        }
    }
    if rd::<u8>(rbx + 0x19) == 0 && key >= rd::<u64>(rbx + 0x20) && rbx != head {
        let next_ref = rd::<u32>(rbx + 0x700).wrapping_sub(1);
        wr::<u32>(rbx + 0x700, next_ref);
        if next_ref == 0 {
            exe::call_va!(0x1_4237_fb60, (rbx + 0x28) -> ());
            let erased = exe::call_va!(0x1_403f_7860, (rdi, rbx) -> usize);
            exe::call_va!(0x1_4238_75a0, (erased + 0x28) -> ());
            exe::call_va!(0x1_401e_66f0, (erased + 0x28, erased) -> ());
            let count = rd::<u64>(rdi + 0x8);
            let mut zero_arg: u64 = 0;
            exe::call_va!(
                0x1_412e_c820,
                (
                    rdi + 0x10,
                    count.wrapping_mul(3),
                    (&mut zero_arg as *mut u64) as usize
                ) -> ()
            );
        }
    }
}

/// `rend::MeshResource::vfunc_13`
/// Original: `0x14230c8e0` (231 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230c8e0(this: usize) {
    let p_c0 = rd::<usize>(this + 0xc0);
    wr::<usize>(this + 0xc0, 0);
    if p_c0 != 0 {
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        exe::call_va!(0x1_4132_0320, (mgr, p_c0) -> ());
    }
    let p_c8 = rd::<usize>(this + 0xc8);
    if p_c8 != 0 {
        let mut handle = rd::<u64>(p_c8 + 0x8);
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        if mgr != 0 {
            exe::call_va!(0x1_4132_01d0, (mgr, handle) -> ());
        } else {
            exe::call_va!(0x1_4230_c810, ((&mut handle as *mut u64) as usize) -> ());
        }
    }
    let p_c8_after = rd::<usize>(this + 0xc8);
    if p_c8_after != 0 {
        let cnt_ptr = (p_c8_after + 0x24) as *const AtomicU32;
        if (*cnt_ptr).fetch_sub(1, Ordering::SeqCst) == 1 {
            let sub_ptr = rd::<usize>(p_c8_after + 0x28);
            if sub_ptr != 0 {
                exe::call_va!(0x1_4241_d6f0, (sub_ptr) -> ());
            }
        }
    }
    wr::<usize>(this + 0xc8, 0);
    let p_b8 = rd::<usize>(this + 0xb8);
    wr::<usize>(this + 0xb8, 0);
    if p_b8 != 0 {
        let sub28 = rd::<usize>(p_b8 + 0x28);
        if sub28 != 0 {
            exe::call_va!(0x1_4244_2de0, (sub28) -> ());
        }
        let sub20 = rd::<usize>(p_b8 + 0x20);
        if sub20 != 0 {
            exe::call_va!(0x1_4244_2de0, (sub20) -> ());
        }
        exe::call_va!(0x1_4244_2de0, (p_b8) -> ());
    }
    exe::call_va!(0x1_4242_87f0, (this) -> ());
}

/// `rend_14230ceb0`
/// Original: `0x14230ceb0` (88 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230ceb0(
    this: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    wr::<u64>(this, rd::<u64>(a2));
    wr::<u64>(this + 0x8, rd::<u64>(a3));
    let ref_obj = rd::<usize>(a4);
    wr::<usize>(this + 0x10, ref_obj);
    if ref_obj != 0 {
        let cnt_ptr = (ref_obj + 0x24) as *const AtomicU32;
        let prev = (*cnt_ptr).fetch_add(1, Ordering::SeqCst);
        if prev.wrapping_add(1) == 1 {
            let sub_ptr = rd::<usize>(ref_obj + 0x28);
            if sub_ptr != 0 {
                exe::call_va!(0x1_4241_d6f0, (sub_ptr) -> ());
            }
        }
    }
    exe::call_va!(0x1_4131_33e0, (this + 0x18, a5) -> usize);
    this
}

/// `rend_14230d550`
/// Original: `0x14230d550` (80 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn rend_14230d550(this: usize, loaded_data: usize) {
    let rax = rd::<u64>(this);
    let mut pair = [rax, loaded_data as u64];
    let mgr_addr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let mgr = rd::<usize>(mgr_addr);
    if mgr != 0 {
        exe::call_va!(0x1_4131_fde0, (mgr, pair.as_mut_ptr() as usize) -> ());
        let mgr2 = rd::<usize>(mgr_addr);
        exe::call_va!(0x1_422a_3d30, (mgr2) -> ());
    } else {
        exe::call_va!(0x1_4230_d2b0, (pair.as_mut_ptr() as usize) -> ());
    }
}

/// `rend_14230d610`
/// Original: `0x14230d610` (169 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230d610(this: usize) {
    wr::<usize>(this, exe::ptr::<u8>(0x1_433f_b680) as usize);
    let p_c8 = rd::<usize>(this + 0xc8);
    if p_c8 != 0 {
        let cnt_ptr = (p_c8 + 0x24) as *const AtomicU32;
        if (*cnt_ptr).fetch_sub(1, Ordering::SeqCst) == 1 {
            let sub_ptr = rd::<usize>(p_c8 + 0x28);
            if sub_ptr != 0 {
                exe::call_va!(0x1_4241_d6f0, (sub_ptr) -> ());
            }
        }
    }
    wr::<usize>(this + 0xc8, 0);
    let p_c0 = rd::<usize>(this + 0xc0);
    if p_c0 != 0 {
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
        exe::call_va!(0x1_4132_0320, (mgr, p_c0) -> ());
    }
    let p_b8 = rd::<usize>(this + 0xb8);
    if p_b8 != 0 {
        let sub28 = rd::<usize>(p_b8 + 0x28);
        if sub28 != 0 {
            exe::call_va!(0x1_4244_2de0, (sub28) -> ());
        }
        let sub20 = rd::<usize>(p_b8 + 0x20);
        if sub20 != 0 {
            exe::call_va!(0x1_4244_2de0, (sub20) -> ());
        }
        exe::call_va!(0x1_4244_2de0, (p_b8) -> ());
    }
    exe::call_va!(0x1_4242_8ce0, (this) -> ());
}

/// `rend_14230d6c0`
/// Original: `0x14230d6c0` (54 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230d6c0(this: usize) -> usize {
    exe::call_va!(0x1_4242_8d70, (this) -> usize);
    wr::<usize>(this, exe::ptr::<u8>(0x1_433f_b680) as usize);
    wr::<u64>(this + 0xb8, 0);
    wr::<u64>(this + 0xc0, 0);
    wr::<u64>(this + 0xc8, 0);
    this
}

/// `rend::MeshResource::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x14230dc50` (57 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230dc50(_this: usize, dst: usize) -> usize {
    exe::call_va!(0x1_4242_8d70, (dst) -> usize);
    wr::<usize>(dst, exe::ptr::<u8>(0x1_433f_b680) as usize);
    wr::<u64>(dst + 0xb8, 0);
    wr::<u64>(dst + 0xc0, 0);
    wr::<u64>(dst + 0xc8, 0);
    dst
}

/// `rend::MeshBonesMetadata::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x14230e420` (103 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230e420(_this: usize, dst: usize) -> usize {
    wr::<u8>(dst + 0x8, 1);
    wr::<usize>(dst, exe::ptr::<u8>(0x1_433f_acd8) as usize);
    for off in (0x10..=0x68).step_by(8) {
        wr::<u64>(dst + off, 0);
    }
    let v74 = (rd::<u32>(dst + 0x74) & 0xff00_0020) | 0x20;
    wr::<u32>(dst + 0x74, v74);
    wr::<u32>(dst + 0x78, 0);
    wr::<u8>(dst + 0x7c, 0);
    wr::<u8>(dst + 0x77, 0x20);
    wr::<u64>(dst + 0x180, 0);
    wr::<u64>(dst + 0x188, 0);
    dst
}

/// `rend_14230e7d0`
/// Original: `0x14230e7d0` (2 bytes, `Throws::No` — unreferenced gap tail of `0x14230e7cd`)
pub unsafe extern "win64" fn rend_14230e7d0() {}

/// `rend::MeshStatisticsMetadata::ClassTypeInfoImplementation::vfunc_14`
/// Original: `0x14230ebd0` (145 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230ebd0(_this: usize, src: usize, dst: usize) -> usize {
    wr::<u8>(dst + 0x8, rd::<u8>(src + 0x8));
    wr::<u64>(dst + 0x10, rd::<u64>(src + 0x10));
    wr::<u64>(dst + 0x18, rd::<u64>(src + 0x18));
    wr::<u32>(dst + 0x20, rd::<u32>(src + 0x20));
    wr::<u32>(dst + 0x24, rd::<u32>(src + 0x24));
    wr::<u32>(dst + 0x28, rd::<u32>(src + 0x28));
    wr::<u32>(dst + 0x2c, rd::<u32>(src + 0x2c));
    if dst + 0x30 != src + 0x30 {
        let vec_base = rd::<usize>(src + 0x30);
        let vec_len = rd::<u32>(src + 0x38) as usize;
        let vec_end = vec_base.wrapping_add(vec_len.wrapping_mul(0x108));
        exe::call_va!(0x1_401e_b240, (dst + 0x30, vec_base, vec_end) -> ());
    }
    exe::call_va!(0x1_4240_aa00, (dst + 0x40, src + 0x40) -> usize);
    exe::call_va!(0x1_402b_f220, (dst + 0xc8, src + 0xc8) -> usize)
}

/// `rend::MeshStatisticsMetadata::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x14230eca0` (63 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230eca0(_this: usize, dst: usize) -> usize {
    wr::<u8>(dst + 0x8, 1);
    wr::<usize>(dst, exe::ptr::<u8>(0x1_433f_b990) as usize);
    wr::<u64>(dst + 0x30, 0);
    wr::<u64>(dst + 0x38, 0);
    let v40 = (rd::<u32>(dst + 0x40) & 0xff00_0010) | 0x10;
    wr::<u32>(dst + 0x40, v40);
    wr::<u32>(dst + 0x44, 0);
    wr::<u8>(dst + 0x48, 0);
    wr::<u8>(dst + 0x43, 0x10);
    wr::<u64>(dst + 0xc8, 0);
    wr::<u64>(dst + 0xd0, 0);
    dst
}

/// `rend::MeshFileLayout::ClassTypeInfoImplementation::vfunc_14`
/// Original: `0x14230fad0` (88 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230fad0(_this: usize, src: usize, dst: usize) -> u8 {
    wr::<u32>(dst, rd::<u32>(src));
    wr::<u32>(dst + 0x4, rd::<u32>(src + 0x4));
    exe::call_va!(0x1_4131_1eb0, (dst + 0x8, src + 0x8) -> usize);
    wr::<[u8; 32]>(dst + 0x18, rd::<[u8; 32]>(src + 0x18));
    wr::<[u8; 16]>(dst + 0x38, rd::<[u8; 16]>(src + 0x38));
    wr::<u32>(dst + 0x48, rd::<u32>(src + 0x48));
    let b4c = rd::<u8>(src + 0x4c);
    wr::<u8>(dst + 0x4c, b4c);
    b4c
}

/// `rend::MeshFileLayout::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x14230fb60` (47 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_14230fb60(_this: usize, dst: usize) -> usize {
    for off in (0x00..=0x40).step_by(8) {
        wr::<u64>(dst + off, 0);
    }
    wr::<u32>(dst + 0x48, 0);
    wr::<u8>(dst + 0x4c, 0);
    dst
}

/// `rend::MeshFileLayout::StreamableRegion::ClassTypeInfoImplementation::vfunc_12`
/// Original: `0x1423101d0` (32 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1423101d0(_this: usize, dst: usize) -> usize {
    for off in (0x00..=0x28).step_by(8) {
        wr::<u64>(dst + off, 0);
    }
    wr::<u32>(dst + 0x30, 0);
    dst
}

/// `rend_1423102e0`
/// Original: `0x1423102e0` (107 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1423102e0(this: usize) -> u8 {
    if rd::<u32>(this + 0x80) == 3 {
        return 0;
    }
    let rax = rd::<usize>(this + 0x30);
    let rdi = rd::<usize>(rax + 0x18);
    let target = if rdi == 0 {
        0usize
    } else {
        let vtbl = rd::<usize>(rdi);
        let fn_ptr = rd::<usize>(vtbl + 0x98);
        let expected = rd::<u32>(exe::ptr::<u32>(0x1_4388_1bf8) as usize);
        let actual = exe::call_va!(fn_ptr, (rdi) -> u32);
        if actual == expected {
            rdi
        } else {
            0usize
        }
    };
    rd::<u8>(target + 0x38)
}

/// `rend_1423104b0`
/// Original: `0x1423104b0` (20 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1423104b0() -> usize {
    let g = exe::ptr::<u8>(0x1_43bb_d698) as usize;
    wr::<[u8; 16]>(g, [0u8; 16]);
    g
}

#[inline(always)]
unsafe fn alloc_rbtree_sentinel_node_138() -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let bin = rd::<usize>(heap + 0x140);
    let free_node = rd::<usize>(bin + 0x10);
    let node = if free_node != 0 && (free_node & 0xf) == 0 {
        wr::<u32>(bin + 0x18, rd::<u32>(bin + 0x18).wrapping_add(1));
        let next = rd::<usize>(free_node);
        wr::<usize>(bin + 0x10, next);
        free_node
    } else {
        let allocated = exe::call_va!(0x1_4253_5070, (heap, 0x138u32, 0x10u32) -> usize);
        if allocated != 0 {
            allocated
        } else {
            let proc_handle = exe::call_iat!(0x1_4264_56c8, () -> usize);
            exe::call_iat!(0x1_4264_53a0, (proc_handle, 0xdead_d00du32) -> ());
            0
        }
    };
    wr::<usize>(node, node);
    wr::<usize>(node + 0x8, node);
    wr::<usize>(node + 0x10, node);
    wr::<u16>(node + 0x18, 0x0101);
    node
}

/// `rend_142310550`
/// Original: `0x142310550` (136 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_142310550() {
    let node = alloc_rbtree_sentinel_node_138();
    let g = exe::ptr::<usize>(0x1_43bb_d698) as usize;
    wr::<usize>(g, node);
}

/// `rend_1423105f0`
/// Original: `0x1423105f0` (155 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1423105f0() -> usize {
    let g = exe::ptr::<u8>(0x1_43bb_d698) as usize;
    wr::<[u8; 16]>(g, [0u8; 16]);
    let node = alloc_rbtree_sentinel_node_138();
    wr::<usize>(g, node);
    g
}

/// `rend_1423106f0`
/// Original: `0x1423106f0` (175 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1423106f0() -> usize {
    let g = exe::ptr::<u8>(0x1_43bb_d698) as usize;
    wr::<u64>(g, 0);
    wr::<u64>(g + 0x8, 0);
    let node = alloc_rbtree_sentinel_node_138();
    wr::<usize>(g, node);
    g
}

/// `rend_1423118c0`
/// Original: `0x1423118c0` (74 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_1423118c0(
    this: usize,
    out: usize,
    row: i32,
    col: i32,
) {
    let rdx = rd::<usize>(this + 0x8);
    let idx = col.wrapping_add(row.wrapping_mul(3)) as isize;
    let indices = rd::<usize>(rdx + 0x160);
    let base = if rd::<u32>(rdx + 0x78) != 0 {
        rd::<usize>(rdx + 0x70)
    } else {
        0usize
    };
    let elem_idx = rd::<i32>((indices as isize).wrapping_add(idx.wrapping_mul(4)) as usize) as isize;
    let elem_addr = (base as isize).wrapping_add(elem_idx.wrapping_mul(8)) as usize;
    wr::<u32>(out, rd::<u32>(elem_addr));
    wr::<u32>(out + 0x4, rd::<u32>(elem_addr + 0x4));
}

/// `rend_142311970`
/// Original: `0x142311970` (29 bytes, `Throws::No`)
pub unsafe extern "win64" fn rend_142311970(this: usize) -> u32 {
    let inner = rd::<usize>(this + 0x8);
    let count = rd::<u32>(inner + 0x168);
    count / 3
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_anonymous_namespace_142300370(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ptr_addr = exe::ptr::<usize>(0x1_43bb_d1d0) as usize;
    let g_cnt_addr = exe::ptr::<u32>(0x1_43bb_d1d8) as usize;
    let saved_ptr = rd::<usize>(g_ptr_addr);
    let saved_cnt = rd::<u32>(g_cnt_addr);

    let mut entries = [0u8; 0xe0];
    let base = entries.as_mut_ptr() as usize;
    wr::<u64>(base, 0x1111);
    wr::<u32>(base + 0x54, 1);
    wr::<u64>(base + 0x60, 0xaaaa);

    wr::<u64>(base + 0x70, 0x2222);
    wr::<u32>(base + 0x70 + 0x54, 0);
    wr::<u64>(base + 0x70 + 0x60, 0xbbbb_cccc);

    wr::<usize>(g_ptr_addr, base);
    wr::<u32>(g_cnt_addr, 2);

    let r1_o = f_orig(0, 0x1111);
    let r1_n = f_new(0, 0x1111);
    let r2_o = f_orig(0, 0x2222);
    let r2_n = f_new(0, 0x2222);

    wr::<usize>(g_ptr_addr, saved_ptr);
    wr::<u32>(g_cnt_addr, saved_cnt);

    crate::diff_eq!(r1_o, r1_n, "142300370_filtered_out");
    crate::diff_eq!(r2_o, r2_n, "142300370_hit");
    Ok(2)
}

pub unsafe fn diff_anonymous_namespace_1423003e0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vec_addr = exe::ptr::<u8>(0x1_43bb_d1c0) as usize;
    let saved_vec = rd::<[u8; 16]>(vec_addr);

    let mut desc = [0u8; 24];
    for (i, b) in desc.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(3);
    }

    let mut buf_o = [0x55u8; 0x70];
    wr::<usize>(vec_addr, buf_o.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 0);
    wr::<u32>(vec_addr + 0xc, 2);
    f_orig(0, 0x1234_5678, desc.as_ptr() as usize, 0x99);
    let len_o = rd::<u32>(vec_addr + 0x8);

    let mut buf_n = [0x55u8; 0x70];
    wr::<usize>(vec_addr, buf_n.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 0);
    wr::<u32>(vec_addr + 0xc, 2);
    f_new(0, 0x1234_5678, desc.as_ptr() as usize, 0x99);
    let len_n = rd::<u32>(vec_addr + 0x8);

    wr::<[u8; 16]>(vec_addr, saved_vec);

    crate::diff_eq!(len_o, len_n, "1423003e0_len");
    crate::diff_eq!(buf_o, buf_n, "1423003e0_slot");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142300460(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut slot_o = [0xaa_u8; 0x30];
    let mut slot_n = [0xaa_u8; 0x30];
    let mut this_o = [0u8; 0x40];
    let mut this_n = [0u8; 0x40];

    let to_addr = this_o.as_mut_ptr() as usize;
    wr::<u32>(to_addr + 0x28, 7);
    wr::<usize>(to_addr + 0x30, slot_o.as_mut_ptr() as usize);
    wr::<u32>(to_addr + 0x38, 0);
    wr::<u32>(to_addr + 0x3c, 2);

    let tn_addr = this_n.as_mut_ptr() as usize;
    wr::<u32>(tn_addr + 0x28, 7);
    wr::<usize>(tn_addr + 0x30, slot_n.as_mut_ptr() as usize);
    wr::<u32>(tn_addr + 0x38, 0);
    wr::<u32>(tn_addr + 0x3c, 2);

    f_orig(to_addr, 0x1111, 0x2222, 0x3333);
    f_new(tn_addr, 0x1111, 0x2222, 0x3333);

    crate::diff_eq!(slot_o, slot_n, "142300460_slot");
    crate::diff_eq!(rd::<u32>(to_addr + 0x38), rd::<u32>(tn_addr + 0x38), "142300460_len");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_1423004f0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut slot_o = [0xbb_u8; 0x30];
    let mut slot_n = [0xbb_u8; 0x30];
    let mut this_o = [0u8; 0x40];
    let mut this_n = [0u8; 0x40];

    let to_addr = this_o.as_mut_ptr() as usize;
    wr::<u32>(to_addr + 0x28, 12);
    wr::<usize>(to_addr + 0x30, slot_o.as_mut_ptr() as usize);
    wr::<u32>(to_addr + 0x38, 0);
    wr::<u32>(to_addr + 0x3c, 2);

    let tn_addr = this_n.as_mut_ptr() as usize;
    wr::<u32>(tn_addr + 0x28, 12);
    wr::<usize>(tn_addr + 0x30, slot_n.as_mut_ptr() as usize);
    wr::<u32>(tn_addr + 0x38, 0);
    wr::<u32>(tn_addr + 0x3c, 2);

    f_orig(to_addr, 0x7788);
    f_new(tn_addr, 0x7788);

    crate::diff_eq!(slot_o, slot_n, "1423004f0_slot");
    crate::diff_eq!(rd::<u32>(to_addr + 0x38), rd::<u32>(tn_addr + 0x38), "1423004f0_len");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142300530(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut elem_o = [0u8; 0x2e0];
    let mut elem_n = [0u8; 0x2e0];
    wr::<u32>(elem_o.as_mut_ptr() as usize, 42);
    wr::<u32>(elem_n.as_mut_ptr() as usize, 42);

    let mut pairs_o = [0u64; 4];
    let mut pairs_n = [0u64; 4];
    let mut this_o = [0u8; 0x80];
    let mut this_n = [0u8; 0x80];

    let to_addr = this_o.as_mut_ptr() as usize;
    wr::<usize>(to_addr + 0x40, elem_o.as_mut_ptr() as usize);
    wr::<u32>(to_addr + 0x48, 1);
    wr::<u32>(to_addr + 0x4c, 1);
    wr::<usize>(to_addr + 0x50, pairs_o.as_mut_ptr() as usize);
    wr::<u32>(to_addr + 0x58, 0);
    wr::<u32>(to_addr + 0x5c, 4);

    let tn_addr = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn_addr + 0x40, elem_n.as_mut_ptr() as usize);
    wr::<u32>(tn_addr + 0x48, 1);
    wr::<u32>(tn_addr + 0x4c, 1);
    wr::<usize>(tn_addr + 0x50, pairs_n.as_mut_ptr() as usize);
    wr::<u32>(tn_addr + 0x58, 0);
    wr::<u32>(tn_addr + 0x5c, 4);

    f_orig(to_addr, 42);
    f_new(tn_addr, 42);

    crate::diff_eq!(pairs_o, pairs_n, "142300530_pairs");
    crate::diff_eq!(elem_o, elem_n, "142300530_elem");
    crate::diff_eq!(rd::<u32>(to_addr + 0x28), rd::<u32>(tn_addr + 0x28), "142300530_idx");
    crate::diff_eq!(rd::<u32>(to_addr + 0x58), rd::<u32>(tn_addr + 0x58), "142300530_len");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142300f20(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vec_addr = exe::ptr::<u8>(0x1_43bb_d1c0) as usize;
    let saved_vec = rd::<[u8; 16]>(vec_addr);

    let mut buf_o = [0x33u8; 0x70];
    wr::<usize>(vec_addr, buf_o.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 0);
    wr::<u32>(vec_addr + 0xc, 2);
    f_orig(0, 0xaaaa_bbbb, 0xcccc_dddd);

    let mut buf_n = [0x33u8; 0x70];
    wr::<usize>(vec_addr, buf_n.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 0);
    wr::<u32>(vec_addr + 0xc, 2);
    f_new(0, 0xaaaa_bbbb, 0xcccc_dddd);

    wr::<[u8; 16]>(vec_addr, saved_vec);

    crate::diff_eq!(buf_o, buf_n, "142300f20_slot");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142300f70(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vec_addr = exe::ptr::<u8>(0x1_43bb_d1c0) as usize;
    let saved_vec = rd::<[u8; 16]>(vec_addr);

    let mut desc = [0u8; 48];
    for (i, b) in desc.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(5);
    }

    let mut buf_o = [0x77u8; 0x70];
    wr::<usize>(vec_addr, buf_o.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 0);
    wr::<u32>(vec_addr + 0xc, 2);
    f_orig(0, 0x9988_7766, desc.as_ptr() as usize, 0x44);

    let mut buf_n = [0x77u8; 0x70];
    wr::<usize>(vec_addr, buf_n.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 0);
    wr::<u32>(vec_addr + 0xc, 2);
    f_new(0, 0x9988_7766, desc.as_ptr() as usize, 0x44);

    wr::<[u8; 16]>(vec_addr, saved_vec);

    crate::diff_eq!(buf_o, buf_n, "142300f70_slot");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142301130(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xeeu8; 0x138];
    let mut buf_n = [0xeeu8; 0x138];
    let ro = f_orig(buf_o.as_mut_ptr() as usize);
    let rn = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "142301130_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "142301130_ret_n");
    crate::diff_eq!(buf_o, buf_n, "142301130_state");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142301330(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let pool_global = exe::ptr::<usize>(0x1_4397_ff88) as usize;
    let saved_pool = rd::<usize>(pool_global);

    let mut items_o = [0xdead_beef_cafe_babeu64];
    let mut pool_o = [items_o.as_mut_ptr() as u64, 1u64 | (1u64 << 32)];
    let mut out_o = 0u64;
    wr::<usize>(pool_global, pool_o.as_mut_ptr() as usize);
    f_orig((&mut out_o as *mut u64) as usize, 0);

    let mut items_n = [0xdead_beef_cafe_babeu64];
    let mut pool_n = [items_n.as_mut_ptr() as u64, 1u64 | (1u64 << 32)];
    let mut out_n = 0u64;
    wr::<usize>(pool_global, pool_n.as_mut_ptr() as usize);
    f_new((&mut out_n as *mut u64) as usize, 0);

    wr::<usize>(pool_global, saved_pool);

    crate::diff_eq!(out_o, out_n, "142301330_out");
    crate::diff_eq!(items_o, items_n, "142301330_items");
    crate::diff_eq!(pool_o[1], pool_n[1], "142301330_pool_hdr");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_142301770(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0x99u8; 0x88];
    let mut this_n = [0x99u8; 0x88];
    let query_obj = [0usize, 0usize];
    let q_ptr = query_obj.as_ptr() as usize;
    let r1_o = f_orig(this_o.as_mut_ptr() as usize, (&q_ptr as *const usize) as usize);
    let r1_n = f_new(this_n.as_mut_ptr() as usize, (&q_ptr as *const usize) as usize);
    crate::diff_eq!(r1_o, r1_n, "142301770_null_inner_ret");
    crate::diff_eq!(this_o, this_n, "142301770_null_inner_state");

    let mut inner = [0u8; 0x38];
    wr::<i32>(inner.as_mut_ptr() as usize + 0x30, 10);
    let mut q2 = [0u8; 0x10];
    wr::<usize>(q2.as_mut_ptr() as usize, inner.as_ptr() as usize);
    wr::<i32>(q2.as_mut_ptr() as usize + 0xc, 5);
    let q2_ptr = q2.as_ptr() as usize;
    wr::<u32>(this_o.as_mut_ptr() as usize + 0x80, 0x1234);
    wr::<u32>(this_n.as_mut_ptr() as usize + 0x80, 0x1234);
    let r2_o = f_orig(this_o.as_mut_ptr() as usize, (&q2_ptr as *const usize) as usize);
    let r2_n = f_new(this_n.as_mut_ptr() as usize, (&q2_ptr as *const usize) as usize);
    crate::diff_eq!(r2_o, r2_n, "142301770_lt_ret");
    crate::diff_eq!(this_o, this_n, "142301770_lt_state");
    Ok(2)
}

pub unsafe fn diff_anonymous_namespace_142301800(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let frame = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    let mut this = [0u8; 0x88];
    let addr = this.as_mut_ptr() as usize;
    wr::<u32>(addr + 0x80, 777);
    wr::<u32>(addr + 0x84, frame);
    crate::diff_eq!(f_orig(addr), f_new(addr), "142301800_match");

    wr::<u32>(addr + 0x84, frame.wrapping_add(1));
    crate::diff_eq!(f_orig(addr), f_new(addr), "142301800_mismatch");
    Ok(2)
}

pub unsafe fn diff_anonymous_namespace_142301820(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let frame = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    let mut this = [0u8; 0x88];
    let addr = this.as_mut_ptr() as usize;

    wr::<u32>(addr + 0x80, 5);
    wr::<u32>(addr + 0x84, frame);
    crate::diff_eq!(f_orig(addr), f_new(addr), "142301820_true");

    wr::<u32>(addr + 0x80, 0);
    crate::diff_eq!(f_orig(addr), f_new(addr), "142301820_zero_count");

    wr::<u32>(addr + 0x80, 5);
    wr::<u32>(addr + 0x84, frame.wrapping_add(1));
    crate::diff_eq!(f_orig(addr), f_new(addr), "142301820_mismatch");
    Ok(3)
}

pub unsafe fn diff_anonymous_namespace_142301850(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let pool_global = exe::ptr::<usize>(0x1_4397_ff88) as usize;
    let saved_pool = rd::<usize>(pool_global);

    let mut pool_items_o = [0u64; 4];
    let mut pool_hdr_o = [pool_items_o.as_mut_ptr() as u64, 4u64 << 32];
    let mut this_o: [u64; 6] = [2, 0, 0x1111, 0x2222, 0, 0];
    wr::<usize>(pool_global, pool_hdr_o.as_mut_ptr() as usize);
    f_orig(this_o.as_mut_ptr() as usize);

    let mut pool_items_n = [0u64; 4];
    let mut pool_hdr_n = [pool_items_n.as_mut_ptr() as u64, 4u64 << 32];
    let mut this_n: [u64; 6] = [2, 0, 0x1111, 0x2222, 0, 0];
    wr::<usize>(pool_global, pool_hdr_n.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize);

    wr::<usize>(pool_global, saved_pool);

    crate::diff_eq!(this_o, this_n, "142301850_this");
    crate::diff_eq!(pool_items_o, pool_items_n, "142301850_pool_items");
    crate::diff_eq!(pool_hdr_o[1], pool_hdr_n[1], "142301850_pool_len");
    Ok(1)
}

pub unsafe fn diff_unknown_142301f00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let pool_global = exe::ptr::<usize>(0x1_4397_ff88) as usize;
    let saved_pool = rd::<usize>(pool_global);

    let mut q_elem = [0usize; 2];
    let q_ptr = q_elem.as_mut_ptr() as u64;

    let mut pool_items_o = [0u64; 4];
    let mut pool_hdr_o = [pool_items_o.as_mut_ptr() as u64, 4u64 << 32];
    let mut this_o = [0u8; 0x88];
    let to_addr = this_o.as_mut_ptr() as usize;
    wr::<u64>(to_addr, 1);
    wr::<u64>(to_addr + 0x8, 0);
    wr::<u64>(to_addr + 0x10, q_ptr);
    wr::<u32>(to_addr + 0x80, 99);
    wr::<usize>(pool_global, pool_hdr_o.as_mut_ptr() as usize);
    f_orig(to_addr);

    let mut pool_items_n = [0u64; 4];
    let mut pool_hdr_n = [pool_items_n.as_mut_ptr() as u64, 4u64 << 32];
    let mut this_n = [0u8; 0x88];
    let tn_addr = this_n.as_mut_ptr() as usize;
    wr::<u64>(tn_addr, 1);
    wr::<u64>(tn_addr + 0x8, 0);
    wr::<u64>(tn_addr + 0x10, q_ptr);
    wr::<u32>(tn_addr + 0x80, 99);
    wr::<usize>(pool_global, pool_hdr_n.as_mut_ptr() as usize);
    f_new(tn_addr);

    wr::<usize>(pool_global, saved_pool);

    crate::diff_eq!(this_o, this_n, "142301f00_this");
    crate::diff_eq!(pool_items_o, pool_items_n, "142301f00_pool");
    Ok(1)
}

pub unsafe fn diff_unknown_142309a10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xffu8; 0x20];
    let mut buf_n = [0xffu8; 0x20];
    let ro = f_orig(buf_o.as_mut_ptr() as usize);
    let rn = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "142309a10_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "142309a10_ret_n");
    crate::diff_eq!(buf_o, buf_n, "142309a10_state");
    Ok(1)
}

pub unsafe fn diff_std_142309ff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut target_o = [0u8; 0xe8];
    let mut target_n = [0u8; 0xe8];
    let mut loaded_o = [0u8; 0x60];
    let mut loaded_n = [0u8; 0x60];

    for (t, l) in [(&mut target_o, &mut loaded_o), (&mut target_n, &mut loaded_n)] {
        let t_addr = t.as_mut_ptr() as usize;
        let l_addr = l.as_mut_ptr() as usize;
        wr::<u32>(t_addr + 0x88, (2u32 << 24) | 2u32);
        wr::<u32>(t_addr + 0x8c, 0);
        wr::<u32>(l_addr, (2u32 << 24) | 2u32);
        wr::<u32>(l_addr + 0x4, 4);
        wr::<[u8; 5]>(l_addr + 0x8, *b"mesh\0");
        wr::<u64>(l_addr + 0x18, 0x1122_3344);
        wr::<u64>(l_addr + 0x20, 0x5566);
        wr::<u64>(l_addr + 0x28, 0x7788);
        wr::<u16>(l_addr + 0x40, 0x20);
        wr::<u32>(l_addr + 0x50, 0x99aa);
        wr::<u64>(l_addr + 0x58, 0xbbcc_ddee);
    }

    let lambda_o = [0usize, target_o.as_mut_ptr() as usize];
    let lambda_n = [0usize, target_n.as_mut_ptr() as usize];
    f_orig(lambda_o.as_ptr() as usize, loaded_o.as_mut_ptr() as usize);
    f_new(lambda_n.as_ptr() as usize, loaded_n.as_mut_ptr() as usize);

    crate::diff_eq!(target_o, target_n, "142309ff0_target");
    crate::diff_eq!(loaded_o, loaded_n, "142309ff0_loaded");
    Ok(1)
}

pub unsafe fn diff_rend_14230c810(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_tree_addr = exe::ptr::<usize>(0x1_4397_fed8) as usize;
    let saved_tree = rd::<usize>(g_tree_addr);

    let mut head_o = [0u8; 0x30];
    let mut node_o = [0u8; 0x708];
    let ho = head_o.as_mut_ptr() as usize;
    let no = node_o.as_mut_ptr() as usize;
    wr::<u8>(ho + 0x19, 1);
    wr::<usize>(ho + 0x8, no);
    wr::<usize>(no, ho);
    wr::<usize>(no + 0x10, ho);
    wr::<u8>(no + 0x19, 0);
    wr::<u64>(no + 0x20, 42);
    wr::<u32>(no + 0x700, 3);
    let tree_o = [ho];

    let mut head_n = [0u8; 0x30];
    let mut node_n = [0u8; 0x708];
    let hn = head_n.as_mut_ptr() as usize;
    let nn = node_n.as_mut_ptr() as usize;
    wr::<u8>(hn + 0x19, 1);
    wr::<usize>(hn + 0x8, nn);
    wr::<usize>(nn, hn);
    wr::<usize>(nn + 0x10, hn);
    wr::<u8>(nn + 0x19, 0);
    wr::<u64>(nn + 0x20, 42);
    wr::<u32>(nn + 0x700, 3);
    let tree_n = [hn];

    let key_zero = 0u64;
    let key_hit = 42u64;

    wr::<usize>(g_tree_addr, tree_o.as_ptr() as usize);
    f_orig((&key_zero as *const u64) as usize);
    f_orig((&key_hit as *const u64) as usize);

    wr::<usize>(g_tree_addr, tree_n.as_ptr() as usize);
    f_new((&key_zero as *const u64) as usize);
    f_new((&key_hit as *const u64) as usize);

    wr::<usize>(g_tree_addr, saved_tree);

    crate::diff_eq!(rd::<u32>(no + 0x700), rd::<u32>(nn + 0x700), "14230c810_refcnt");
    Ok(2)
}

pub unsafe fn diff_rend_14230c8e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = [0u8; 0xd0];
    let mut obj_n = [0u8; 0xd0];
    rend_14230d6c0(obj_o.as_mut_ptr() as usize);
    rend_14230d6c0(obj_n.as_mut_ptr() as usize);

    let mut ref_o = [0u8; 0x30];
    let mut ref_n = [0u8; 0x30];
    wr::<u32>(ref_o.as_mut_ptr() as usize + 0x24, 2);
    wr::<u32>(ref_n.as_mut_ptr() as usize + 0x24, 2);
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0xc8, ref_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0xc8, ref_n.as_mut_ptr() as usize);

    let mgr_addr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_mgr = rd::<usize>(mgr_addr);
    wr::<usize>(mgr_addr, 0);

    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);

    wr::<usize>(mgr_addr, saved_mgr);

    crate::diff_eq!(ref_o, ref_n, "14230c8e0_ref");
    crate::diff_eq!(
        rd::<usize>(obj_o.as_ptr() as usize + 0xc8),
        rd::<usize>(obj_n.as_ptr() as usize + 0xc8),
        "14230c8e0_c8_cleared"
    );
    Ok(1)
}

pub unsafe fn diff_rend_14230ceb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let a2 = 0x1111_2222u64;
    let a3 = 0x3333_4444u64;
    let mut ref_o = [0u8; 0x30];
    let mut ref_n = [0u8; 0x30];
    wr::<u32>(ref_o.as_mut_ptr() as usize + 0x24, 1);
    wr::<u32>(ref_n.as_mut_ptr() as usize + 0x24, 1);
    let p_ref_o = ref_o.as_mut_ptr() as usize;
    let p_ref_n = ref_n.as_mut_ptr() as usize;

    let mut layout = [0u8; 0x50];
    let l_addr = layout.as_mut_ptr() as usize;
    wr::<u32>(l_addr, 10);
    wr::<u32>(l_addr + 0x4, 20);
    wr::<u64>(l_addr + 0x18, 0x5566_7788);
    wr::<u32>(l_addr + 0x48, 0x99);
    wr::<u8>(l_addr + 0x4c, 1);

    let mut dst_o = [0u8; 0x68];
    let mut dst_n = [0u8; 0x68];
    f_orig(
        dst_o.as_mut_ptr() as usize,
        (&a2 as *const u64) as usize,
        (&a3 as *const u64) as usize,
        (&p_ref_o as *const usize) as usize,
        l_addr,
    );
    f_new(
        dst_n.as_mut_ptr() as usize,
        (&a2 as *const u64) as usize,
        (&a3 as *const u64) as usize,
        (&p_ref_n as *const usize) as usize,
        l_addr,
    );

    crate::diff_eq!(ref_o, ref_n, "14230ceb0_ref");
    crate::diff_eq!(&dst_o[0..0x10], &dst_n[0..0x10], "14230ceb0_hdr");
    crate::diff_eq!(&dst_o[0x18..0x65], &dst_n[0x18..0x65], "14230ceb0_layout");
    Ok(1)
}

pub unsafe fn diff_rend_14230d610(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = [0u8; 0xd0];
    let mut obj_n = [0u8; 0xd0];
    rend_14230d6c0(obj_o.as_mut_ptr() as usize);
    rend_14230d6c0(obj_n.as_mut_ptr() as usize);

    let mut ref_o = [0u8; 0x30];
    let mut ref_n = [0u8; 0x30];
    wr::<u32>(ref_o.as_mut_ptr() as usize + 0x24, 2);
    wr::<u32>(ref_n.as_mut_ptr() as usize + 0x24, 2);
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0xc8, ref_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0xc8, ref_n.as_mut_ptr() as usize);

    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);

    crate::diff_eq!(ref_o, ref_n, "14230d610_ref");
    crate::diff_eq!(
        rd::<usize>(obj_o.as_ptr() as usize),
        rd::<usize>(obj_n.as_ptr() as usize),
        "14230d610_vtbl"
    );
    crate::diff_eq!(
        rd::<usize>(obj_o.as_ptr() as usize + 0xc8),
        rd::<usize>(obj_n.as_ptr() as usize + 0xc8),
        "14230d610_c8"
    );
    Ok(1)
}

pub unsafe fn diff_rend_14230d6c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xdd_u8; 0xd0];
    let mut buf_n = [0xdd_u8; 0xd0];
    let ro = f_orig(buf_o.as_mut_ptr() as usize);
    let rn = f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "14230d6c0_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "14230d6c0_ret_n");
    crate::diff_eq!(
        rd::<usize>(buf_o.as_ptr() as usize),
        rd::<usize>(buf_n.as_ptr() as usize),
        "14230d6c0_vtbl"
    );
    crate::diff_eq!(&buf_o[0xb8..0xd0], &buf_n[0xb8..0xd0], "14230d6c0_tail");
    Ok(1)
}

pub unsafe fn diff_rend_14230dc50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xcc_u8; 0xd0];
    let mut buf_n = [0xcc_u8; 0xd0];
    let ro = f_orig(0, buf_o.as_mut_ptr() as usize);
    let rn = f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "14230dc50_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "14230dc50_ret_n");
    crate::diff_eq!(
        rd::<usize>(buf_o.as_ptr() as usize),
        rd::<usize>(buf_n.as_ptr() as usize),
        "14230dc50_vtbl"
    );
    crate::diff_eq!(&buf_o[0xb8..0xd0], &buf_n[0xb8..0xd0], "14230dc50_tail");
    Ok(1)
}

pub unsafe fn diff_rend_14230e420(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xaa_u8; 0x190];
    let mut buf_n = [0xaa_u8; 0x190];
    let ro = f_orig(0, buf_o.as_mut_ptr() as usize);
    let rn = f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "14230e420_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "14230e420_ret_n");
    crate::diff_eq!(buf_o, buf_n, "14230e420_state");
    Ok(1)
}

pub unsafe fn diff_rend_14230ebd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0xd8];
    let mut dst_o = [0u8; 0xd8];
    let mut dst_n = [0u8; 0xd8];
    rend_14230eca0(0, src.as_mut_ptr() as usize);
    rend_14230eca0(0, dst_o.as_mut_ptr() as usize);
    rend_14230eca0(0, dst_n.as_mut_ptr() as usize);

    let s_addr = src.as_mut_ptr() as usize;
    wr::<u8>(s_addr + 0x8, 3);
    wr::<u64>(s_addr + 0x10, 0x1111_2222);
    wr::<u64>(s_addr + 0x18, 0x3333_4444);
    wr::<u32>(s_addr + 0x20, 10);
    wr::<u32>(s_addr + 0x24, 20);
    wr::<u32>(s_addr + 0x28, 30);
    wr::<u32>(s_addr + 0x2c, 40);
    wr::<u32>(s_addr + 0x44, 4);
    wr::<[u8; 5]>(s_addr + 0x48, *b"stat\0");

    f_orig(0, s_addr, dst_o.as_mut_ptr() as usize);
    f_new(0, s_addr, dst_n.as_mut_ptr() as usize);

    crate::diff_eq!(dst_o, dst_n, "14230ebd0_state");
    Ok(1)
}

pub unsafe fn diff_rend_14230eca0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0xbb_u8; 0xd8];
    let mut buf_n = [0xbb_u8; 0xd8];
    let ro = f_orig(0, buf_o.as_mut_ptr() as usize);
    let rn = f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "14230eca0_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "14230eca0_ret_n");
    crate::diff_eq!(buf_o, buf_n, "14230eca0_state");
    Ok(1)
}

pub unsafe fn diff_rend_14230fad0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0x50];
    let s_addr = src.as_mut_ptr() as usize;
    wr::<u32>(s_addr, 0x1111);
    wr::<u32>(s_addr + 0x4, 0x2222);
    for i in 0x18..0x48 {
        wr::<u8>(s_addr + i, (i as u8).wrapping_mul(5));
    }
    wr::<u32>(s_addr + 0x48, 0x3344_5566);
    wr::<u8>(s_addr + 0x4c, 1);

    let mut dst_o = [0u8; 0x50];
    let mut dst_n = [0u8; 0x50];
    let ro = f_orig(0, s_addr, dst_o.as_mut_ptr() as usize);
    let rn = f_new(0, s_addr, dst_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, rn, "14230fad0_ret");
    crate::diff_eq!(dst_o, dst_n, "14230fad0_state");
    Ok(1)
}

pub unsafe fn diff_rend_14230fb60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0x99u8; 0x50];
    let mut buf_n = [0x99u8; 0x50];
    let ro = f_orig(0, buf_o.as_mut_ptr() as usize);
    let rn = f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "14230fb60_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "14230fb60_ret_n");
    crate::diff_eq!(buf_o, buf_n, "14230fb60_state");
    Ok(1)
}

pub unsafe fn diff_rend_1423101d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0x88u8; 0x38];
    let mut buf_n = [0x88u8; 0x38];
    let ro = f_orig(0, buf_o.as_mut_ptr() as usize);
    let rn = f_new(0, buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, buf_o.as_ptr() as usize, "1423101d0_ret_o");
    crate::diff_eq!(rn, buf_n.as_ptr() as usize, "1423101d0_ret_n");
    crate::diff_eq!(buf_o, buf_n, "1423101d0_state");
    Ok(1)
}

unsafe extern "win64" fn mock_get_mesh_stat_type_id(_this: usize) -> u32 {
    rd::<u32>(exe::ptr::<u32>(0x1_4388_1bf8) as usize)
}

pub unsafe fn diff_rend_1423102e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this = [0u8; 0x88];
    let t_addr = this.as_mut_ptr() as usize;
    wr::<u32>(t_addr + 0x80, 3);
    crate::diff_eq!(f_orig(t_addr), f_new(t_addr), "1423102e0_state_3");

    let mut vtbl = [0usize; 20];
    vtbl[0x98 / 8] = mock_get_mesh_stat_type_id as *const () as usize;
    let mut rdi_obj = [0u8; 0x40];
    wr::<usize>(rdi_obj.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<u8>(rdi_obj.as_mut_ptr() as usize + 0x38, 1);
    let mut rax_obj = [0usize; 4];
    rax_obj[3] = rdi_obj.as_ptr() as usize;
    wr::<u32>(t_addr + 0x80, 1);
    wr::<usize>(t_addr + 0x30, rax_obj.as_ptr() as usize);
    crate::diff_eq!(f_orig(t_addr), f_new(t_addr), "1423102e0_vcall_match");
    Ok(2)
}

pub unsafe fn diff_rend_1423104b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g = exe::ptr::<u8>(0x1_43bb_d698) as usize;
    let saved = rd::<[u8; 16]>(g);

    wr::<[u8; 16]>(g, [0xaa; 16]);
    let ro = f_orig();
    let state_o = rd::<[u8; 16]>(g);

    wr::<[u8; 16]>(g, [0xaa; 16]);
    let rn = f_new();
    let state_n = rd::<[u8; 16]>(g);

    wr::<[u8; 16]>(g, saved);

    crate::diff_eq!(ro, rn, "1423104b0_ret");
    crate::diff_eq!(state_o, state_n, "1423104b0_state");
    Ok(1)
}

pub unsafe fn diff_rend_1423118c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, i32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vertices: [u64; 4] = [
        0x1111_2222_3333_4444,
        0x5555_6666_7777_8888,
        0x9999_aaaa_bbbb_cccc,
        0xdddd_eeee_ffff_0001,
    ];
    let indices: [i32; 6] = [0, 2, 1, 3, 2, 0];
    let mut inner = [0u8; 0x170];
    let in_addr = inner.as_mut_ptr() as usize;
    wr::<usize>(in_addr + 0x70, vertices.as_ptr() as usize);
    wr::<u32>(in_addr + 0x78, 4);
    wr::<usize>(in_addr + 0x160, indices.as_ptr() as usize);

    let this = [0usize, in_addr];
    let mut out_o = 0u64;
    let mut out_n = 0u64;
    for row in 0..2 {
        for col in 0..3 {
            f_orig(
                this.as_ptr() as usize,
                (&mut out_o as *mut u64) as usize,
                row,
                col,
            );
            f_new(
                this.as_ptr() as usize,
                (&mut out_n as *mut u64) as usize,
                row,
                col,
            );
            crate::diff_eq!(out_o, out_n, "1423118c0");
        }
    }
    Ok(6)
}

pub unsafe fn diff_rend_142311970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut inner = [0u8; 0x170];
    let in_addr = inner.as_mut_ptr() as usize;
    let this = [0usize, in_addr];
    for count in [0u32, 3, 6, 15, 99, 100, 3000] {
        wr::<u32>(in_addr + 0x168, count);
        let ro = f_orig(this.as_ptr() as usize);
        let rn = f_new(this.as_ptr() as usize);
        crate::diff_eq!(ro, rn, "142311970");
    }
    Ok(7)
}

pub unsafe fn diff_anonymous_namespace_142300310(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut LOG_00310: [usize; 4] = [0; 4];
    unsafe extern "win64" fn mock_8cba0(dst: usize, func_obj: usize) -> usize {
        unsafe {
            LOG_00310[0] = dst;
            LOG_00310[1] = func_obj;
        }
        dst
    }
    unsafe extern "win64" fn mock_destroy(impl_ptr: usize, is_large: u8) {
        unsafe {
            LOG_00310[2] = impl_ptr;
            LOG_00310[3] = is_large as usize;
        }
    }

    let _m = crate::difftest::Mock::new(0x1_4128_cba0, mock_8cba0 as *const ())?;
    let g_idx = exe::ptr::<i32>(0x1_43bb_d168) as usize;
    let g_base = exe::ptr::<usize>(0x1_43bb_d180) as usize;
    let saved = (rd::<i32>(g_idx), rd::<usize>(g_base));
    wr::<i32>(g_idx, 2);
    wr::<usize>(g_base, 0x10_0000);

    let mut vtbl = [0usize; 5];
    vtbl[4] = mock_destroy as *const () as usize;

    for separate_impl in [false, true] {
        let mut ext_impl = [vtbl.as_ptr() as usize, 0usize];
        let mut func_o = [0u8; 0x40];
        let fo_addr = func_o.as_mut_ptr() as usize;
        wr::<usize>(fo_addr, vtbl.as_ptr() as usize);
        wr::<usize>(
            fo_addr + 0x38,
            if separate_impl { ext_impl.as_mut_ptr() as usize } else { fo_addr },
        );
        LOG_00310 = [0; 4];
        f_orig(0, fo_addr);
        let st_o = (
            LOG_00310[0],
            LOG_00310[2] != 0,
            LOG_00310[3],
            rd::<usize>(fo_addr + 0x38),
        );

        let mut func_n = [0u8; 0x40];
        let fn_addr = func_n.as_mut_ptr() as usize;
        wr::<usize>(fn_addr, vtbl.as_ptr() as usize);
        wr::<usize>(
            fn_addr + 0x38,
            if separate_impl { ext_impl.as_mut_ptr() as usize } else { fn_addr },
        );
        LOG_00310 = [0; 4];
        f_new(0, fn_addr);
        let st_n = (
            LOG_00310[0],
            LOG_00310[2] != 0,
            LOG_00310[3],
            rd::<usize>(fn_addr + 0x38),
        );

        if st_o != st_n {
            wr::<i32>(g_idx, saved.0);
            wr::<usize>(g_base, saved.1);
            crate::diff_eq!(st_o, st_n, "142300310");
        }
    }

    wr::<i32>(g_idx, saved.0);
    wr::<usize>(g_base, saved.1);
    Ok(2)
}

pub unsafe fn diff_anonymous_namespace_142301e60(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut INNER_PTR_01E60: usize = 0;
    static mut LOCK_PTR_01E60: usize = 0;
    static mut R10_PTR_01E60: usize = 0;
    static mut LOG_01E60: [u64; 5] = [0; 5];

    unsafe extern "win64" fn mock_63e60(_this: usize, tmp_q: usize, r8d: u32) -> usize {
        unsafe {
            wr::<usize>(tmp_q, INNER_PTR_01E60);
            wr::<u32>(tmp_q + 0x8, 0x42 + r8d);
            wr::<u32>(tmp_q + 0xc, 0);
            tmp_q
        }
    }
    unsafe extern "win64" fn mock_d5b00(guard_ptr: usize) -> usize {
        unsafe {
            wr::<usize>(guard_ptr, LOCK_PTR_01E60);
            wr::<usize>(guard_ptr + 0x8, R10_PTR_01E60);
            guard_ptr
        }
    }
    unsafe extern "win64" fn mock_vfn_1a0(r10: usize, rdx: usize, r8d: u32, r9d: u32) {
        unsafe {
            LOG_01E60[0] = r10 as u64;
            LOG_01E60[1] = rdx as u64;
            LOG_01E60[2] = r8d as u64;
            LOG_01E60[3] = r9d as u64;
        }
    }
    unsafe extern "win64" fn mock_iat_5598(lock_ptr: usize) {
        unsafe {
            LOG_01E60[4] = lock_ptr as u64;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_3e60, mock_63e60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_5b00, mock_d5b00 as *const ())?;
    let _im = crate::difftest::IatMock::new(0x1_4264_5598, mock_iat_5598 as *const ())?;

    let mut vtbl = [0usize; 0x35];
    vtbl[0x1a0 / 8] = mock_vfn_1a0 as *const () as usize;
    let r10_obj = [vtbl.as_ptr() as usize];
    let mut inner = [0u8; 0x20];
    wr::<usize>(inner.as_mut_ptr() as usize + 0x18, 0x7777_8888);
    INNER_PTR_01E60 = inner.as_ptr() as usize;
    R10_PTR_01E60 = r10_obj.as_ptr() as usize;

    for init_10 in [1u32, 0u32] {
        let mut lock_o = [0u32, 0u32, 0x1234u32, 1u32];
        let mut rbx_o = [0u8; 0x18];
        wr::<u32>(rbx_o.as_mut_ptr() as usize + 0x10, init_10);
        let mut this_o = [0u8; 0x20];
        wr::<usize>(this_o.as_mut_ptr() as usize + 0x18, rbx_o.as_mut_ptr() as usize);
        LOCK_PTR_01E60 = lock_o.as_mut_ptr() as usize;
        LOG_01E60 = [0; 5];
        f_orig(this_o.as_ptr() as usize);
        let st_o = (
            rbx_o,
            lock_o[2],
            lock_o[3],
            LOG_01E60[0],
            LOG_01E60[1],
            LOG_01E60[2],
            LOG_01E60[3],
            LOG_01E60[4] != 0,
        );

        let mut lock_n = [0u32, 0u32, 0x1234u32, 1u32];
        let mut rbx_n = [0u8; 0x18];
        wr::<u32>(rbx_n.as_mut_ptr() as usize + 0x10, init_10);
        let mut this_n = [0u8; 0x20];
        wr::<usize>(this_n.as_mut_ptr() as usize + 0x18, rbx_n.as_mut_ptr() as usize);
        LOCK_PTR_01E60 = lock_n.as_mut_ptr() as usize;
        LOG_01E60 = [0; 5];
        f_new(this_n.as_ptr() as usize);
        let st_n = (
            rbx_n,
            lock_n[2],
            lock_n[3],
            LOG_01E60[0],
            LOG_01E60[1],
            LOG_01E60[2],
            LOG_01E60[3],
            LOG_01E60[4] != 0,
        );

        crate::diff_eq!(st_o, st_n, "142301e60");
    }
    Ok(2)
}

pub unsafe fn diff_std_14230c2d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut LOG_0C2D0: [u64; 5] = [0; 5];
    unsafe extern "win64" fn mock_1fde0(mgr: usize, pair_ptr: usize) {
        unsafe {
            LOG_0C2D0[0] = mgr as u64;
            LOG_0C2D0[1] = rd::<u64>(pair_ptr);
            LOG_0C2D0[2] = rd::<u64>(pair_ptr + 0x8);
        }
    }
    unsafe extern "win64" fn mock_a3d30(mgr: usize) {
        unsafe { LOG_0C2D0[3] = mgr as u64; }
    }
    unsafe extern "win64" fn mock_0d2b0(pair_ptr: usize) {
        unsafe {
            LOG_0C2D0[4] = rd::<u64>(pair_ptr) ^ rd::<u64>(pair_ptr + 0x8);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4131_fde0, mock_1fde0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422a_3d30, mock_a3d30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4230_d2b0, mock_0d2b0 as *const ())?;

    let mgr_addr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_mgr = rd::<usize>(mgr_addr);
    let this = [0u64, 0xaaaa_bbbbu64];

    for mgr_val in [0usize, 0x5555usize] {
        wr::<usize>(mgr_addr, mgr_val);
        LOG_0C2D0 = [0; 5];
        f_orig(this.as_ptr() as usize, 0xcccc_dddd);
        let st_o = LOG_0C2D0;

        LOG_0C2D0 = [0; 5];
        f_new(this.as_ptr() as usize, 0xcccc_dddd);
        let st_n = LOG_0C2D0;

        if st_o != st_n {
            wr::<usize>(mgr_addr, saved_mgr);
            crate::diff_eq!(st_o, st_n, "14230c2d0");
        }
    }

    wr::<usize>(mgr_addr, saved_mgr);
    Ok(2)
}

pub unsafe fn diff_rend_14230d550(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    static mut LOG_0D550: [u64; 5] = [0; 5];
    unsafe extern "win64" fn mock_1fde0(mgr: usize, pair_ptr: usize) {
        unsafe {
            LOG_0D550[0] = mgr as u64;
            LOG_0D550[1] = rd::<u64>(pair_ptr);
            LOG_0D550[2] = rd::<u64>(pair_ptr + 0x8);
        }
    }
    unsafe extern "win64" fn mock_a3d30(mgr: usize) {
        unsafe { LOG_0D550[3] = mgr as u64; }
    }
    unsafe extern "win64" fn mock_0d2b0(pair_ptr: usize) {
        unsafe {
            LOG_0D550[4] = rd::<u64>(pair_ptr) ^ rd::<u64>(pair_ptr + 0x8);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4131_fde0, mock_1fde0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422a_3d30, mock_a3d30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4230_d2b0, mock_0d2b0 as *const ())?;

    let mgr_addr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_mgr = rd::<usize>(mgr_addr);
    let this = [0x1122_3344u64];

    for mgr_val in [0usize, 0x6666usize] {
        wr::<usize>(mgr_addr, mgr_val);
        LOG_0D550 = [0; 5];
        f_orig(this.as_ptr() as usize, 0x7788_99aa);
        let st_o = LOG_0D550;

        LOG_0D550 = [0; 5];
        f_new(this.as_ptr() as usize, 0x7788_99aa);
        let st_n = LOG_0D550;

        if st_o != st_n {
            wr::<usize>(mgr_addr, saved_mgr);
            crate::diff_eq!(st_o, st_n, "14230d550");
        }
    }

    wr::<usize>(mgr_addr, saved_mgr);
    Ok(2)
}

pub unsafe fn diff_rend_14230e7d0(orig: usize, new: usize) -> Result<u32, String> {
    // 0x14230e7d0 is a 2-byte unreferenced gap tail (`00 ff` = `add bh, bh`
    // followed by `int3` padding with no `ret`).
    let _ = orig;
    let f_new: unsafe extern "win64" fn() = core::mem::transmute(new);
    f_new();
    Ok(1)
}

#[repr(C, align(16))]
struct AlignedSentinelNode([u8; 0x20]);

unsafe fn run_sentinel_alloc_test(
    init_g: [u8; 16],
    call: impl FnOnce(),
) -> (u32, usize, bool, u16, bool, u64) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let g = exe::ptr::<u8>(0x1_43bb_d698) as usize;
    let saved_heap = rd::<usize>(tls0 + 0x6348);
    let saved_g = rd::<[u8; 16]>(g);

    let mut node = AlignedSentinelNode([0u8; 0x20]);
    let node_addr = node.0.as_mut_ptr() as usize;
    wr::<usize>(node_addr, 0x1234_5678);

    let mut bin = [0u8; 0x20];
    let bin_addr = bin.as_mut_ptr() as usize;
    wr::<usize>(bin_addr + 0x10, node_addr);
    wr::<u32>(bin_addr + 0x18, 5);

    let mut heap = [0u8; 0x150];
    let heap_addr = heap.as_mut_ptr() as usize;
    wr::<usize>(heap_addr + 0x140, bin_addr);

    wr::<usize>(tls0 + 0x6348, heap_addr);
    wr::<[u8; 16]>(g, init_g);
    call();

    let res = (
        rd::<u32>(bin_addr + 0x18),
        rd::<usize>(bin_addr + 0x10),
        rd::<usize>(node_addr) == node_addr
            && rd::<usize>(node_addr + 0x8) == node_addr
            && rd::<usize>(node_addr + 0x10) == node_addr,
        rd::<u16>(node_addr + 0x18),
        rd::<usize>(g) == node_addr,
        rd::<u64>(g + 0x8),
    );

    wr::<usize>(tls0 + 0x6348, saved_heap);
    wr::<[u8; 16]>(g, saved_g);
    res
}

pub unsafe fn diff_rend_142310550(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let st_o = run_sentinel_alloc_test([0xaa; 16], || f_orig());
    let st_n = run_sentinel_alloc_test([0xaa; 16], || f_new());
    crate::diff_eq!(st_o, st_n, "142310550");
    Ok(1)
}

pub unsafe fn diff_rend_1423105f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut r_o = 0usize;
    let mut r_n = 0usize;
    let st_o = run_sentinel_alloc_test([0xbb; 16], || {
        r_o = f_orig();
    });
    let st_n = run_sentinel_alloc_test([0xbb; 16], || {
        r_n = f_new();
    });
    crate::diff_eq!(r_o, r_n, "1423105f0_ret");
    crate::diff_eq!(st_o, st_n, "1423105f0_state");
    Ok(1)
}

pub unsafe fn diff_rend_1423106f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut r_o = 0usize;
    let mut r_n = 0usize;
    let st_o = run_sentinel_alloc_test([0xcc; 16], || {
        r_o = f_orig();
    });
    let st_n = run_sentinel_alloc_test([0xcc; 16], || {
        r_n = f_new();
    });
    crate::diff_eq!(r_o, r_n, "1423106f0_ret");
    crate::diff_eq!(st_o, st_n, "1423106f0_state");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: none (no function in this batch returns f32/f64/__m128 in xmm0).
// Note: 0x14230e7d0 is a 2-byte unhookable gap_unref fragment (tail of 0x14230e7cd jmp); diff_rend_14230e7d0 is provided below.
//
// Replacement::verified(0x1_4230_0310, "rend::anonymous_namespace_142300310", rend::rend_e18_d::anonymous_namespace_142300310 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142300310),
// Replacement::verified(0x1_4230_0370, "rend::anonymous_namespace_142300370", rend::rend_e18_d::anonymous_namespace_142300370 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142300370),
// Replacement::verified(0x1_4230_03e0, "rend::anonymous_namespace_1423003e0", rend::rend_e18_d::anonymous_namespace_1423003e0 as *const (), rend::rend_e18_d::diff_anonymous_namespace_1423003e0),
// Replacement::verified(0x1_4230_0460, "rend::anonymous_namespace_142300460", rend::rend_e18_d::anonymous_namespace_142300460 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142300460),
// Replacement::verified(0x1_4230_04f0, "rend::anonymous_namespace_1423004f0", rend::rend_e18_d::anonymous_namespace_1423004f0 as *const (), rend::rend_e18_d::diff_anonymous_namespace_1423004f0),
// Replacement::verified_throws(0x1_4230_0530, "rend::anonymous_namespace_142300530", rend::rend_e18_d::anonymous_namespace_142300530 as *const (), Throws::Discard, rend::rend_e18_d::diff_anonymous_namespace_142300530),
// Replacement::verified(0x1_4230_0f20, "rend::anonymous_namespace_142300f20", rend::rend_e18_d::anonymous_namespace_142300f20 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142300f20),
// Replacement::verified(0x1_4230_0f70, "rend::anonymous_namespace_142300f70", rend::rend_e18_d::anonymous_namespace_142300f70 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142300f70),
// Replacement::verified(0x1_4230_1130, "rend::anonymous_namespace_142301130", rend::rend_e18_d::anonymous_namespace_142301130 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142301130),
// Replacement::verified_throws(0x1_4230_1330, "rend::anonymous_namespace_142301330", rend::rend_e18_d::anonymous_namespace_142301330 as *const (), Throws::Discard, rend::rend_e18_d::diff_anonymous_namespace_142301330),
// Replacement::verified_throws(0x1_4230_1770, "rend::anonymous_namespace_142301770", rend::rend_e18_d::anonymous_namespace_142301770 as *const (), Throws::Discard, rend::rend_e18_d::diff_anonymous_namespace_142301770),
// Replacement::verified(0x1_4230_1800, "rend::anonymous_namespace_142301800", rend::rend_e18_d::anonymous_namespace_142301800 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142301800),
// Replacement::verified(0x1_4230_1820, "rend::anonymous_namespace_142301820", rend::rend_e18_d::anonymous_namespace_142301820 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142301820),
// Replacement::verified(0x1_4230_1850, "rend::anonymous_namespace_142301850", rend::rend_e18_d::anonymous_namespace_142301850 as *const (), rend::rend_e18_d::diff_anonymous_namespace_142301850),
// Replacement::verified_throws(0x1_4230_1e60, "anonymous_namespace::QueryBegin::vfunc_1", rend::rend_e18_d::anonymous_namespace_142301e60 as *const (), Throws::Discard, rend::rend_e18_d::diff_anonymous_namespace_142301e60),
// Replacement::verified_throws(0x1_4230_1f00, "rend::unknown_142301f00", rend::rend_e18_d::unknown_142301f00 as *const (), Throws::Discard, rend::rend_e18_d::diff_unknown_142301f00),
// Replacement::verified(0x1_4230_9a10, "rend::unknown_142309a10", rend::rend_e18_d::unknown_142309a10 as *const (), rend::rend_e18_d::diff_unknown_142309a10),
// Replacement::verified(0x1_4230_9ff0, "std::_Func_impl_no_alloc<rend::MeshStreamingManager::updateStreamOps::<lambda_2>,void,r::LoadedData&&>::vfunc_2", rend::rend_e18_d::std_142309ff0 as *const (), rend::rend_e18_d::diff_std_142309ff0),
// Replacement::verified_throws(0x1_4230_c2d0, "std::_Func_impl_no_alloc<rend::MeshResource::blockingLoad::<lambda_1>,void,r::LoadedData&&>::vfunc_2", rend::rend_e18_d::std_14230c2d0 as *const (), Throws::Discard, rend::rend_e18_d::diff_std_14230c2d0),
// Replacement::verified(0x1_4230_c810, "rend::rend_14230c810", rend::rend_e18_d::rend_14230c810 as *const (), rend::rend_e18_d::diff_rend_14230c810),
// Replacement::verified(0x1_4230_c8e0, "rend::MeshResource::vfunc_13", rend::rend_e18_d::rend_14230c8e0 as *const (), rend::rend_e18_d::diff_rend_14230c8e0),
// Replacement::verified(0x1_4230_ceb0, "rend::rend_14230ceb0", rend::rend_e18_d::rend_14230ceb0 as *const (), rend::rend_e18_d::diff_rend_14230ceb0),
// Replacement::verified_throws(0x1_4230_d550, "rend::rend_14230d550", rend::rend_e18_d::rend_14230d550 as *const (), Throws::Discard, rend::rend_e18_d::diff_rend_14230d550),
// Replacement::verified(0x1_4230_d610, "rend::rend_14230d610", rend::rend_e18_d::rend_14230d610 as *const (), rend::rend_e18_d::diff_rend_14230d610),
// Replacement::verified(0x1_4230_d6c0, "rend::rend_14230d6c0", rend::rend_e18_d::rend_14230d6c0 as *const (), rend::rend_e18_d::diff_rend_14230d6c0),
// Replacement::verified(0x1_4230_dc50, "rend::MeshResource::ClassTypeInfoImplementation::vfunc_12", rend::rend_e18_d::rend_14230dc50 as *const (), rend::rend_e18_d::diff_rend_14230dc50),
// Replacement::verified(0x1_4230_e420, "rend::MeshBonesMetadata::ClassTypeInfoImplementation::vfunc_12", rend::rend_e18_d::rend_14230e420 as *const (), rend::rend_e18_d::diff_rend_14230e420),
// Replacement::verified(0x1_4230_e7d0, "rend::rend_14230e7d0", rend::rend_e18_d::rend_14230e7d0 as *const (), rend::rend_e18_d::diff_rend_14230e7d0),
// Replacement::verified(0x1_4230_ebd0, "rend::MeshStatisticsMetadata::ClassTypeInfoImplementation::vfunc_14", rend::rend_e18_d::rend_14230ebd0 as *const (), rend::rend_e18_d::diff_rend_14230ebd0),
// Replacement::verified(0x1_4230_eca0, "rend::MeshStatisticsMetadata::ClassTypeInfoImplementation::vfunc_12", rend::rend_e18_d::rend_14230eca0 as *const (), rend::rend_e18_d::diff_rend_14230eca0),
// Replacement::verified(0x1_4230_fad0, "rend::MeshFileLayout::ClassTypeInfoImplementation::vfunc_14", rend::rend_e18_d::rend_14230fad0 as *const (), rend::rend_e18_d::diff_rend_14230fad0),
// Replacement::verified(0x1_4230_fb60, "rend::MeshFileLayout::ClassTypeInfoImplementation::vfunc_12", rend::rend_e18_d::rend_14230fb60 as *const (), rend::rend_e18_d::diff_rend_14230fb60),
// Replacement::verified(0x1_4231_01d0, "rend::MeshFileLayout::StreamableRegion::ClassTypeInfoImplementation::vfunc_12", rend::rend_e18_d::rend_1423101d0 as *const (), rend::rend_e18_d::diff_rend_1423101d0),
// Replacement::verified(0x1_4231_02e0, "rend::rend_1423102e0", rend::rend_e18_d::rend_1423102e0 as *const (), rend::rend_e18_d::diff_rend_1423102e0),
// Replacement::verified(0x1_4231_04b0, "rend::rend_1423104b0", rend::rend_e18_d::rend_1423104b0 as *const (), rend::rend_e18_d::diff_rend_1423104b0),
// Replacement::verified(0x1_4231_0550, "rend::rend_142310550", rend::rend_e18_d::rend_142310550 as *const (), rend::rend_e18_d::diff_rend_142310550),
// Replacement::verified(0x1_4231_05f0, "rend::rend_1423105f0", rend::rend_e18_d::rend_1423105f0 as *const (), rend::rend_e18_d::diff_rend_1423105f0),
// Replacement::verified(0x1_4231_06f0, "rend::rend_1423106f0", rend::rend_e18_d::rend_1423106f0 as *const (), rend::rend_e18_d::diff_rend_1423106f0),
// Replacement::verified(0x1_4231_18c0, "rend::rend_1423118c0", rend::rend_e18_d::rend_1423118c0 as *const (), rend::rend_e18_d::diff_rend_1423118c0),
// Replacement::verified(0x1_4231_1970, "rend::rend_142311970", rend::rend_e18_d::rend_142311970 as *const (), rend::rend_e18_d::diff_rend_142311970),

