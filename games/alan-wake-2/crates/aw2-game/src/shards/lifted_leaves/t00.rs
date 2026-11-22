#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0009<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((P0) as *const u32)) as u32 as u64;
    rcx = ((P1) as u64) as u64;
    rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64;
    let fl_res: u32 = (((rax as u32)) | (0x2u32));
    rax = (fl_res) as u32 as u64;
    core::ptr::write_unaligned((P2) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((P3) as *mut u8, (0x2u8) as u8);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0039<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (((core::ptr::read_unaligned((P0) as *const u32)) as i32 as i64) as u64) as u64;
    let fl_l: u32 = (rcx as u32); let fl_r: u32 = 0x1000u32;
    if !((((fl_l) as i32) >= ((fl_r) as i32))) {
        r8 = ((P1) as u64) as u64;
        rdx = ((P2) as u64) as u64;
        core::ptr::write_unaligned((r8.wrapping_add(rcx.wrapping_mul(8))) as *mut u64, (rdx) as u64);
        let fl_res: u32 = ((rcx as u32)).wrapping_add(1);
        rcx = (fl_res) as u32 as u64;
        core::ptr::write_unaligned((P3) as *mut u32, ((rcx as u32)) as u32);
    }
    rcx = ((P4) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0054<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P1) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0075<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((P0) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((P1) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0102<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = ((P0) as u64) as u64;
    rcx = ((P1) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P2) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0143<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P2) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0158(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_l: u64 = rcx; let fl_r: u64 = rdx;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0167<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P0) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0185<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rdx = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x580e0u64)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax.wrapping_add(rdx.wrapping_mul(8))) as *const u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0187(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx) as *const u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0221<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = ((P1) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0304(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u8, (0x0u8) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0305(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0324(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0356<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((P0) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u32, ((rax as u32)) as u32);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0374(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u32, (0x0u32) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0393<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0434(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0492<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (rcx) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0502<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = rax;
    if !((((fl_l) & (fl_r)) == 0)) {
        rdx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
        rcx = (rax) as u64;
        crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
    }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0521<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r9 = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    r8 = ((core::ptr::read_unaligned((P1) as *const u16)) as u32) as u32 as u64;
    rdx = ((core::ptr::read_unaligned((P2) as *const u16)) as u32) as u32 as u64;
    r10 = (core::ptr::read_unaligned((rax.wrapping_add(0x580e0u64)) as *const u64)) as u64;
    rcx = ((core::ptr::read_unaligned((P3) as *const u16)) as u32) as u32 as u64;
    r9 = (core::ptr::read_unaligned((r10.wrapping_add(r9.wrapping_mul(8))) as *const u64)) as u64;
    r8 = (core::ptr::read_unaligned((r10.wrapping_add(r8.wrapping_mul(8))) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((r10.wrapping_add(rdx.wrapping_mul(8))) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((r10.wrapping_add(rcx.wrapping_mul(8))) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0560<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rdx = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    rcx = ((core::ptr::read_unaligned((P1) as *const u16)) as u32) as u32 as u64;
    r8 = (core::ptr::read_unaligned((rax.wrapping_add(0x580e0u64)) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((r8.wrapping_add(rdx.wrapping_mul(8))) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(rcx.wrapping_mul(8))) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0589<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    rdx = ((core::ptr::read_unaligned((P1) as *const u16)) as u32) as u32 as u64;
    rcx = ((core::ptr::read_unaligned((P2) as *const u16)) as u32) as u32 as u64;
    r9 = (core::ptr::read_unaligned((rax.wrapping_add(0x580e0u64)) as *const u64)) as u64;
    r8 = (core::ptr::read_unaligned((r9.wrapping_add(r8.wrapping_mul(8))) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((r9.wrapping_add(rdx.wrapping_mul(8))) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((r9.wrapping_add(rcx.wrapping_mul(8))) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0711(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = ((rdx) & (0xfffffffffffffffcu64));
    rdx = (fl_res) as u64;
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0712(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u8 = (((rdx as u8)) & (0x1u8));
    rdx = (rdx & !0xffu64) | (((fl_res) as u8 as u64));
    rax = (((rdx as u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0899(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (rdx) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) & (0xfffffffffffffffcu64))) as u64;
    let fl_res: u64 = ((rax) ^ (rdx));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r8 = (rdx) as u64;
    rdx = (rcx) as u64;
    rcx = (r8) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((rax) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0921<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u32, ((r8 as u32)) as u32);
    r8 = (0x8u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0923<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0924<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x70u64)) as *const u64)) as u64;
    r10 = (r9) as u64;
    r9 = ((r8 as u32)) as u32 as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    r8 = (rdx) as u64;
    rdx = (r10) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0927<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (r9) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0949<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (0u32) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0951<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (rdx) as u64;
    rdx = (rcx) as u64;
    rcx = (rax) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_0988<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1021<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u32, ((r8 as u32)) as u32);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1054(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1142(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u8 = core::ptr::read_unaligned((rax.wrapping_add(0x19u64)) as *const u8); let fl_r: u8 = 0x0u8;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1143(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_res: u64 = (rax).wrapping_add(0x20u64);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1144(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u8, ((rax as u8)) as u8);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_res: u64 = (rax).wrapping_add(0x1cu64);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1262(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1498(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64)) as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1534(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1554<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_1589<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rdx = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    rcx = (core::ptr::read_unaligned((rax.wrapping_add(0x580e0u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(rdx.wrapping_mul(8))) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_3694(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    rax = (rdx) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    r8 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_5158<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x58u64)) as *mut u64, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_5159<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    r8 = (core::ptr::read_unaligned((P0) as *const u32)) as u32 as u64;
    rax = ((P1) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x58u64)) as *mut u64, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_5371<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u32, ((r8 as u32)) as u32);
    r8 = (0x4u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_7323<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (0u32) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    rdx = ((rdx).wrapping_add(0x48u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_7771<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (rdx) as u64;
    r8 = (0u32) as u32 as u64;
    rdx = (rcx) as u64;
    rcx = (rax) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_7960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_7981<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let fl_l: u8 = core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u8); let fl_r: u8 = 0x0u8;
    if !(((fl_l) == (fl_r))) {
        let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
        let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
        let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
        let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
        crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8031<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rdx = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x580e0u64)) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((rax.wrapping_add(rdx.wrapping_mul(8))) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8286(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8485<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (core::ptr::read_unaligned((r8) as *const u128)) as u128;
    r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u128, (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8651(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8744<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rdx.wrapping_add(0x10u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8781<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = ((core::ptr::read_unaligned((rcx.wrapping_add(0x20u64)) as *const u8)) as u32) as u32 as u64;
    r8 = (r9) as u64;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r9 = (rdx) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x30u64)) as *mut u8, ((rax as u8)) as u8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8789<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (rdx) as u64;
    rdx = (0x8u32) as u32 as u64;
    let fl_res: u64 = (rcx).wrapping_shl(4u32);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_8957(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u8, (0x0u8) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_9052<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_9140<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx.wrapping_add(0x38u64)) as *mut u64, (0x0u64) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x38u64)) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_9321<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (0x0u64) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = ((P1) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_9729<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    let fl_l: u64 = rcx; let fl_r: u64 = rdx;
    if !(((fl_l) == (fl_r))) {
        rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
        core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    }
    rax = ((P1) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10066(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10109(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10113(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    r8 = ((rcx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((rdx) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(r8)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10114(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10129(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10130<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10133(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx) as *const u8)) as u32) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(rdx)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10134<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10136<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10195(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((r9) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10202(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u8, (0x1u8) as u8);
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10217(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (((rdx as u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10221(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rdx) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u8, ((rax as u8)) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10225(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = ((core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10235<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (0u32) as u32 as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P0) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10260<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((r8 as u32)) as u32 as u64;
    rcx = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rax.wrapping_mul(8))) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10263(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u32)) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(4))) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10284(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x28u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10289(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10290<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (core::ptr::read_unaligned((rdx) as *const u128)) as u128;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u128, (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10296(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r9 = (0u32) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8))) as *mut u64, (r9) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64)) as *mut u64, (r9) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8))) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rcx) as *mut u64, (r9) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r9) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10300<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((r8 as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rdx).wrapping_add(rax);
    rdx = (fl_res) as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10305(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rcx) as *const u128)) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    let fl_res: u64 = (rax).wrapping_add(rax);
    rax = (fl_res) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rax.wrapping_mul(8))) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10311(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    rdx = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((r8) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10314<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r9 = (rcx) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rdx = (r9) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = (((r8 as u16)) as u32) as u32 as u64;
    let fl_res: u64 = (r9).wrapping_add(r9);
    r9 = (fl_res) as u64;
    rdx = (core::ptr::read_unaligned((rax.wrapping_add(r9.wrapping_mul(8))) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10321(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r9 = (((r8 as u16)) as u32) as u32 as u64;
    r8 = (core::ptr::read_unaligned((rax.wrapping_add(0x20u64)) as *const u64)) as u64;
    r8 = ((core::ptr::read_unaligned((r8.wrapping_add(r9.wrapping_mul(2))) as *const u16)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64));
    r8 = (fl_res) as u64;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rdx = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10323<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r9 = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    r8 = (core::ptr::read_unaligned((rax.wrapping_add(0x20u64)) as *const u64)) as u64;
    r8 = ((core::ptr::read_unaligned((r8.wrapping_add(r9.wrapping_mul(2))) as *const u16)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64));
    r8 = (fl_res) as u64;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rdx = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10324<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r9 = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r8 = ((core::ptr::read_unaligned((P0) as *const u16)) as u32) as u32 as u64;
    rax = (core::ptr::read_unaligned((r9) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((rax.wrapping_add(0x20u64)) as *const u64)) as u64;
    r8 = ((core::ptr::read_unaligned((rdx.wrapping_add(r8.wrapping_mul(2))) as *const u16)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (r9) as u64);
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((r9.wrapping_add(0x8u64)) as *const u64));
    r8 = (fl_res) as u64;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rdx = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10335(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10339(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    let fl_res: u32 = (core::ptr::read_unaligned((rdx) as *const u32)).wrapping_add((rax as u32));
    core::ptr::write_unaligned((rdx) as *mut u32, (fl_res) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10341(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    let fl_res: u32 = (core::ptr::read_unaligned((rdx) as *const u32)).wrapping_add((rax as u32));
    core::ptr::write_unaligned((rdx) as *mut u32, (fl_res) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10351(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((r8) as *const u32)) as u32 as u64;
    let fl_res: u32 = (core::ptr::read_unaligned((rdx) as *const u32)).wrapping_add((rax as u32));
    core::ptr::write_unaligned((rdx) as *mut u32, (fl_res) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10364(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx) as *const u16)) as u32) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(2))) as *mut u16, ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10412<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P0) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10432<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u8, ((rdx as u8)) as u8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r9 = ((rsp.wrapping_add(0x48u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u32, (0x0u32) as u32);
    rdx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10444<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10466<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    r9 = ((r8 as u32)) as u32 as u64;
    let fl_res: u64 = (r9).wrapping_add(r9);
    r9 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r9.wrapping_mul(8))) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10471<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((r8 as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rdx).wrapping_add(rax);
    rdx = (fl_res) as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = ((core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10473<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u8, ((rax as u8)) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10476<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u8, ((rdx as u8)) as u8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10477<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u32, ((rdx as u32)) as u32);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10488(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64)) as *const u64)) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10505(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = ((r8 as u32)) as u32 as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    rdx = (0u32) as u32 as u64;
    core::ptr::write_unaligned((r8) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((r8) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10508(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rcx) as *const u128)) as u128;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(4))) as *mut u128, (xmm0) as u128);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r9 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (0x0u64) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(rax.wrapping_mul(8))) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10542(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax.wrapping_add(rdx.wrapping_mul(4))) as *const u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10545<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = (0u32) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    rax = (r8) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    r8 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10553<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    rcx = ((r8.wrapping_add(0x8u64)) as u64) as u64;
    let fl_res: u64 = (rdx).wrapping_add(0x8u64);
    rdx = (fl_res) as u64;
    core::ptr::write_unaligned((r8) as *mut u64, (rax) as u64);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10564(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r9 = (rdx) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = ((r9).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rcx)) as *mut u64, (rax) as u64);
    rax = ((rdx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rdx = (rax) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = ((core::ptr::read_unaligned((rax) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rcx)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10566<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rdx.wrapping_add(0x8u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r9 = (rdx) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = ((r9).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rcx)) as *mut u64, (rax) as u64);
    rax = ((rdx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rdx = (rax) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rcx)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10571(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10572<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (r8) as u64;
    rcx = (rdx) as u64;
    rdx = (rax) as u64;
    r8 = (((r9 as u8)) as u32) as u32 as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10573(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r9 = (rdx) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = ((r9).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rcx)) as *mut u64, (rax) as u64);
    rax = ((rdx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rdx = (rax) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rax) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rcx)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10603(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rdx) as *const u128)) as u128;
    core::ptr::write_unaligned((r8) as *mut u128, (xmm0) as u128);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10604(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    r9 = (rdx) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = ((r9).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rcx)) as *mut u32, ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(0x4u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rdx = (rax) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rax) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rcx)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10605(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u32, (0x0u32) as u32);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10606(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = (rdx) as u64;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    r9 = ((r9).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rcx)) as *mut u32, ((rax as u32)) as u32);
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = ((rcx).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x4u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x4u64)) as *mut u32, ((rax as u32)) as u32);
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10612(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10618(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10626(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10627(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10628(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10668<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64;
    rdx = (core::ptr::read_unaligned((rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64)) as *const u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10674<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((r8 as u32)) as u32 as u64;
    r9 = (rdx) as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8))) as *mut u64, (rax) as u64);
    r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64;
    rdx = ((rcx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = ((r9.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10678(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx) as *const u16)) as u32) as u32 as u64;
    r8 = ((r8 as u32)) as u32 as u64;
    rdx = ((rdx.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8)) as *mut u16, ((rax as u16)) as u16);
    rax = ((core::ptr::read_unaligned((rcx.wrapping_add(0x2u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8).wrapping_add(0x2u64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10697(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10698<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    core::ptr::write_unaligned((P1) as *mut u64, (rax) as u64);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10727<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r9 = (0u32) as u32 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8))) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64)) as *mut u64, (r9) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x14u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r9) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10762(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (((rcx as u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10791<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbx: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (0x0u64) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    core::ptr::write_unaligned((rbx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10861<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    r9 = ((r8 as u32)) as u32 as u64;
    let fl_res: u64 = (r9).wrapping_shl(5u32);
    r9 = (fl_res) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rdx)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rdx).wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rdx).wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rdx).wrapping_add(0x14u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x18u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rdx).wrapping_add(0x18u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10888<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((P0) as *const u32)) as u32 as u64;
    rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64;
    core::ptr::write_unaligned((P1) as *mut u32, ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10909(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x10u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10983(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (rcx) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10984(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (0x0u64) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rsp) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rax) as *mut u64, (rcx) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x10u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x18u64)) as *const u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_10995(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (rcx) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rsp.wrapping_add(0x10u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rax) as *mut u64, (rcx) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11061<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (0x0u64) as u64);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11077(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rdx) as *const u128)) as u128;
    core::ptr::write_unaligned((r8) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11105<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    r9 = ((r8 as u32)) as u32 as u64;
    let fl_res: u64 = (r9).wrapping_add(r9);
    r9 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r9.wrapping_mul(8))) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11109(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x18u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11114(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = (0x2aaaaaaaaaaaaaabu64) as u64;
    r8 = ((r8).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64;
    rax = (rdx) as u64;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(rdx);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11115(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    let fl_res: u32 = ((rax as u32)).wrapping_sub(1);
    rax = (fl_res) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11118(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11124(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = (0x2aaaaaaaaaaaaaabu64) as u64;
    r8 = ((r8).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64;
    rax = (rdx) as u64;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(rdx);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11125(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11126(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (rdx) as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    r8 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11127(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x30u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11137(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11153(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = ((r8).wrapping_shl(5u32)) as u64;
    rax = (rdx) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    r8 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11154(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    let fl_res: u64 = (((rax) as i64).wrapping_shr(5u32) as u64);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11155(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11157(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11158(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x20u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11168(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0x38e38e38e38e38eu64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rax) as *const u64); let fl_r: u64 = 0x0u64;
    if ((fl_l) == (fl_r)) {
        rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
        return rax;
    }
    rdx = (core::ptr::read_unaligned((rax.wrapping_add(0x28u64)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax.wrapping_add(0x30u64)) as *const u32)) as u32 as u64;
    rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = rcx;
    if !(((fl_l) == (fl_r))) {
        rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
        return rax;
    }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11175<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbx: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    core::ptr::write_unaligned((rbx.wrapping_add(0x39u64)) as *mut u8, (0x1u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11178(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rax) as *const u64); let fl_r: u64 = 0x0u64;
    if ((fl_l) == (fl_r)) {
        rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
        return rax;
    }
    rdx = (core::ptr::read_unaligned((rax.wrapping_add(0x28u64)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rax.wrapping_add(0x30u64)) as *const u32)) as u32 as u64;
    rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = rcx;
    if !(((fl_l) == (fl_r))) {
        rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
        return rax;
    }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11205(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x18u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (0x0u64) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x20u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11206(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x18u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x20u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11207(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(0x18u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64)) as *const u64)) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u8 = core::ptr::read_unaligned((rdx.wrapping_add(0x19u64)) as *const u8); let fl_r: u8 = 0x0u8;
    if !(((fl_l) != (fl_r))) {
        loop {
            rcx = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
            rax = (rdx) as u64;
            rdx = (rcx) as u64;
            let fl_l: u8 = core::ptr::read_unaligned((rcx.wrapping_add(0x19u64)) as *const u8); let fl_r: u8 = 0x0u8;
            if !(((fl_l) == (fl_r))) { break; }
        }
        return rax;
    }
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64); let fl_r: u64 = rax;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11299<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x38u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x40u64)) as *mut u16, ((rax as u16)) as u16);
    core::ptr::write_unaligned((rdx.wrapping_add(0x48u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x50u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x58u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11328(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x28u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x38u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11329<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11345(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x1cu64)) as *mut u8, (0x1u8) as u8);
    rdx = (core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u32, ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11346(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x18u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x18u64)) as *mut u32, ((rax as u32)) as u32);
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x1cu64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x1cu64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11347<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x1cu64)) as *mut u8, ((rax as u8)) as u8);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11456<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (rdx) as u64;
    r9 = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P1) as u64) as u64;
    rax = (r9) as u64;
    core::ptr::write_unaligned((r9) as *mut u64, (rcx) as u64);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11478<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11479(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    xmm0 = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u128)) as u128;
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u128, (xmm0) as u128);
    xmm0 = (core::ptr::read_unaligned((rdx.wrapping_add(0x20u64)) as *const u128)) as u128;
    core::ptr::write_unaligned((r8.wrapping_add(0x20u64)) as *mut u128, (xmm0) as u128);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11480<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11534<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x90u64)) as *mut u64, (rax) as u64);
    rax = ((P1) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x40u64)) as *mut u64, (rax) as u64);
    rax = ((P2) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11562(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u128)) as u128;
    core::ptr::write_unaligned((rdx) as *mut u128, (xmm0) as u128);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11614<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x1cu64)) as *mut u8, ((rax as u8)) as u8);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11735(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = ((rcx).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11736(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11737<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11750<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rcx = ((r8.wrapping_add(0x10u64)) as u64) as u64;
    let fl_res: u64 = (rdx).wrapping_add(0x10u64);
    rdx = (fl_res) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11764<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11772(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r8 = ((rcx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((rdx) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(r8)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11798<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11799<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = (rdx) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rdx) as u64);
    rcx = (r9) as u64;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11801<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11807<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11860<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (0u32) as u32 as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, ((rcx as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11879<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (0u32) as u32 as u64;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, ((rcx as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u32, ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11886<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (rdx) as u64;
    r8 = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P1) as u64) as u64;
    rax = (r8) as u64;
    core::ptr::write_unaligned((r8) as *mut u64, (rcx) as u64);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11894<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11914<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11949<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11968<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_11987<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12018<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x1u8) as u8);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12022<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = ((rdx.wrapping_add(0xb8u64)) as u64) as u64;
    rax = ((rax).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12087(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) & (0xff000010u32))) as u32);
    rax = (rdx) as u64;
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx) as *const u32)) | (0x10u32));
    core::ptr::write_unaligned((rdx) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x3u64)) as *mut u8, (0x10u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x4u64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12090<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = (rdx) as u64;
    rax = ((core::ptr::read_unaligned((rdx) as *const u8)) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    r9 = ((r9).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    core::ptr::write_unaligned((r9.wrapping_add(rcx)) as *mut u8, ((rax as u8)) as u8);
    rcx = (rdx) as u64;
    rcx = ((rcx).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    let fl_res: u64 = (rcx).wrapping_add(core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64));
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12093(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) & (0xff000008u32))) as u32);
    rax = (rdx) as u64;
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx) as *const u32)) | (0x8u32));
    core::ptr::write_unaligned((rdx) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x3u64)) as *mut u8, (0x8u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x4u64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12100<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = (0u32) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rcx) as u64);
    rax = (r8) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    r8 = (0u32) as u32 as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12106<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = (r8) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12117<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) & (0xff000008u32))) as u32);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) | (0x8u32))) as u32);
    rcx = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0xbu64)) as *mut u8, (0x8u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0xcu64)) as *mut u32, ((rcx as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u8, ((rcx as u8)) as u8);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x50u64)) as *mut u64, (rcx) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x58u64)) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12131<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) & (0xff000008u32))) as u32);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) | (0x8u32))) as u32);
    rcx = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0xbu64)) as *mut u8, (0x8u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0xcu64)) as *mut u32, ((rcx as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u8, ((rcx as u8)) as u8);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x50u64)) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12141<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) & (0xff000008u32))) as u32);
    rax = ((P0) as u64) as u64;
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) | (0x8u32));
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0xbu64)) as *mut u8, (0x8u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0xcu64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u8, (0x0u8) as u8);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12157(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u128)) as u128;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12169(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = core::ptr::read_unaligned((rdx) as *const u64);
    rax = (fl_l.wrapping_sub(fl_r)) as u64;
    xmm0 = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u128)) as u128;
    core::ptr::write_unaligned((rax.wrapping_add(rcx).wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    rax = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rax = (fl_l.wrapping_sub(fl_r)) as u64;
    xmm0 = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u128)) as u128;
    core::ptr::write_unaligned((rax.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12175<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) & (0xff000010u32))) as u32);
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) | (0x10u32));
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0xbu64)) as *mut u8, (0x10u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0xcu64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u8, (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12176<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12177(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (rdx) as u64;
    rdx = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((rax) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rcx).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12178(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    r8 = ((rcx.wrapping_add(0x8u64)) as u64) as u64;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((rdx) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(r8)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12179(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12184(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12188(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12199<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = (r8) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    return rax;
}
