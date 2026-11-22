#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12209(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12257(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) & (0xff000005u32))) as u32);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) | (0x5u32))) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x4u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x3u64)) as *mut u8, (0x5u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u32)) & (0xff000010u32))) as u32);
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u32)) | (0x10u32));
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x34u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x38u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x33u64)) as *mut u8, (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12322<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (r8) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (0x0u64) as u64);
    rcx = ((P0) as u64) as u64;
    r8 = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    r9 = ((P1) as u64) as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12327<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) & (0xff000008u32))) as u32);
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) | (0x8u32));
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0xbu64)) as *mut u8, (0x8u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0xcu64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u8, (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12403(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x14u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x18u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x18u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = (rdx) as u64;
    r8 = ((r8).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(rcx).wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rdx = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((r9) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(rcx).wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12429<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (0x4000004u64) as u64);
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u8, (0x0u8) as u8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12451(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12480<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (rdx) as u64;
    rdx = (0u32) as u32 as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x14u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12555(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) & (0xff000004u32))) as u32);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) | (0x4u32))) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x4u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x3u64)) as *mut u8, (0x4u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u32, (((core::ptr::read_unaligned((rdx.wrapping_add(0x28u64)) as *const u32)) & (0xff000004u32))) as u32);
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx.wrapping_add(0x28u64)) as *const u32)) | (0x4u32));
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x2cu64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x30u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x2bu64)) as *mut u8, (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12558<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x1u8) as u8);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12562<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x1u8) as u8);
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12589(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0xcu64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12771<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u32)) as u32 as u64;
    rcx = ((r8.wrapping_add(0x8u64)) as u64) as u64;
    core::ptr::write_unaligned((r8) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x4u64)) as *const u32)) as u32 as u64;
    let fl_res: u64 = (rdx).wrapping_add(0x8u64);
    rdx = (fl_res) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x4u64)) as *mut u32, ((rax as u32)) as u32);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12773(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx.wrapping_add(0x4u64)) as *mut u32, (0x0u32) as u32);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12774<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (r8) as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (0x0u64) as u64);
    rcx = ((P0) as u64) as u64;
    r8 = (0x1u32) as u32 as u64;
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rcx) as u64);
    r9 = ((P1) as u64) as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12892(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    r9 = (rcx) as u64;
    r8 = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r8 = ((r8).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(rcx).wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((rdx) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = ((core::ptr::read_unaligned((r9.wrapping_add(0x9u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(r9).wrapping_add(0x9u64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12893(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = ((rcx).wrapping_sub(core::ptr::read_unaligned((r8) as *const u64))) as u64;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x9u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0x9u64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12894(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x9u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x9u64)) as *mut u8, ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_12948(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u32, (((core::ptr::read_unaligned((rdx) as *const u32)) & (0xff000004u32))) as u32);
    rax = (rdx) as u64;
    let fl_res: u32 = ((core::ptr::read_unaligned((rdx) as *const u32)) | (0x4u32));
    core::ptr::write_unaligned((rdx) as *mut u32, (fl_res) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x3u64)) as *mut u8, (0x4u8) as u8);
    core::ptr::write_unaligned((rdx.wrapping_add(0x4u64)) as *mut u32, (0x0u32) as u32);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13202<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (r8) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13265(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (rdx) as u64;
    rax = ((rax).wrapping_shr(3u32)) as u64;
    rdx = ((rdx).wrapping_sub(rax)) as u64;
    let fl_l: u64 = rdx; let fl_r: u64 = core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64);
    rdx = (fl_l.wrapping_sub(fl_r)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x28u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13266(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = (core::ptr::read_unaligned((rax.wrapping_add(0x18u64)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    rcx = ((rcx).wrapping_shr(7u32)) as u64;
    rcx = (((rcx) & (r8))) as u64;
    r8 = ((r8).wrapping_shr(4u32)) as u64;
    rdx = ((rdx).wrapping_sub(rcx)) as u64;
    rdx = ((rdx).wrapping_shr(4u32)) as u64;
    let fl_res: u64 = ((rdx) & (r8));
    rdx = (fl_res) as u64;
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13267(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (0x0u64) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13268<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13272(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x18u64)) as *const u64)) as u64;
    r8 = ((r8).wrapping_shr(7u32)) as u64;
    let fl_res: u64 = ((r8) & (rax));
    r8 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13273(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13278(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13281<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13283(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0xfffffffffffffff0u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13284(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0xfffffffffffffff8u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13285(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13286(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13293(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x14u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13294(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13298<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u32, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x8u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13305(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_l: u64 = rcx; let fl_r: u64 = rdx;
    rax = (rax & !0xffu64) | (((if ((fl_l) != (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13324<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u8, (0x0u8) as u8);
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13348(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13427<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbx: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13482(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((rcx.wrapping_add(0x10u64)) as u64) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rcx) as *const u64); let fl_r: u64 = rax;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13485(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = 0x10u64;
    rax = (fl_l.wrapping_sub(fl_r)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13487(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13492(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rdx) as *const u64); let fl_r: u64 = rax;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13499(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x38u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13519(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = (0x2aaaaaaaaaaaaaabu64) as u64;
    r8 = ((r8).wrapping_sub(rdx)) as u64;
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
pub unsafe extern "win64" fn tpl_leaf_13521(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13525(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    let fl_res: u64 = (((rax) as i64).wrapping_shr(4u32) as u64);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13526(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13527(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((r8) as *const u128)) as u128;
    core::ptr::write_unaligned((rdx) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13528(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax).wrapping_sub(rdx)) as u64;
    let fl_res: u64 = (((rax) as i64).wrapping_shr(4u32) as u64);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_res: u64 = (rax).wrapping_add(0x10u64);
    rax = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13532(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rdx).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rdx = (fl_res) as u64;
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13567(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x40u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13596(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x28u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13619(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13678<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u8 = core::ptr::read_unaligned((rcx.wrapping_add(0x48u64)) as *const u8); let fl_r: u8 = 0x0u8;
    if !(((fl_l) == (fl_r))) {
        rcx = ((rcx).wrapping_add(0x10u64)) as u64;
        rdx = ((P0) as u64) as u64;
        r8 = (0u32) as u32 as u64;
        crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
    }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13696(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0xffffffffffffffe8u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13697(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rcx) as *const u64); let fl_r: u64 = rax;
    rax = (rax & !0xffu64) | (((if ((fl_l) < (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13698(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = 0x18u64;
    rax = (fl_l.wrapping_sub(fl_r)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13701(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13801(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x9u64)) as *const u8)) as u32) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x9u64)) as *mut u8, ((rax as u8)) as u8);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13819(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u8, (0x0u8) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13848(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0xffffffffffffff90u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13862(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13864(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = 0x70u64;
    rax = (fl_l.wrapping_sub(fl_r)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13866(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13868(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x70u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13884(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64); let fl_r: u64 = rax;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13888(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x9u64)) as *mut u8, (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13915<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((P0) as *mut u64, (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13922(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_13976(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x18u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(0x20u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64)) as *const u64)) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14046(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (r8) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14078(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = (rdx) as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64));
    r8 = (fl_res) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14079(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14083(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14125<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14138(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let fl_l: u64 = rax; let fl_r: u64 = 0x8u64;
    rax = (fl_l.wrapping_sub(fl_r)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14139(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14162(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    rax = (rcx) as u64;
    let fl_res: u64 = (r8).wrapping_add(core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64));
    r8 = (fl_res) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14164(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14165(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x10u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14168(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_l: u64 = core::ptr::read_unaligned((rcx) as *const u64); let fl_r: u64 = 0x0u64;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14173(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x28u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14217<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    core::ptr::write_unaligned((P0) as *mut u64, (0x0u64) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14425(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xb8u64)) as *const u32)) as u32 as u64;
    rcx = (core::ptr::read_unaligned((r8.wrapping_add(0x8u64)) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = core::ptr::read_unaligned((r8) as *const u64);
    rcx = (fl_l.wrapping_sub(fl_r)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0xb8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xbcu64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0xbcu64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0xc0u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(rdx).wrapping_add(0xc0u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14504<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (rdx) as u64;
    r10 = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P1) as u64) as u64;
    core::ptr::write_unaligned((r10.wrapping_add(0xb8u64)) as *mut u64, (0x0u64) as u64);
    core::ptr::write_unaligned((r10) as *mut u64, (rax) as u64);
    rax = (r10) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14572<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (rdx) as u64;
    r8 = (0u32) as u32 as u64;
    rdx = ((P0) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14628<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((((rcx as u8)) as i8 as i64) as u64) as u64;
    rcx = ((P0) as u64) as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(rax.wrapping_mul(4))) as *const u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14669(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14684(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    let fl_res: u64 = (rax).wrapping_add(core::ptr::read_unaligned((rcx) as *const u64));
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14693(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    rdx = (0u32) as u32 as u64;
    let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14703(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u32, ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14704(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rcx) as *const u64); let fl_r: u64 = rax;
    if ((fl_l) != (fl_r)) {
        rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
        return rax;
    }
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    let fl_l: u32 = core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32); let fl_r: u32 = (rax as u32);
    if !(((fl_l) != (fl_r))) {
        rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
        return rax;
    }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14707(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u32, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14718(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14791(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14831(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((r9) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x28u64)) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14834<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    xmm0 = (core::ptr::read_unaligned((rdx) as *const u128)) as u128;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14925(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    let fl_res: u64 = (rdx).wrapping_add(core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64));
    rdx = (fl_res) as u64;
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14929<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = ((P0) as u64) as u64;
    rdx = (0x10u32) as u32 as u64;
    r8 = ((rdx.wrapping_add(0x30u64)) as u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14964(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14965(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14970<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14979(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rax = ((rax).wrapping_sub(core::ptr::read_unaligned((rdx) as *const u64))) as u64;
    let fl_res: u64 = (((rax) as i64).wrapping_shr(3u32) as u64);
    rax = (fl_res) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14981(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (core::ptr::read_unaligned((r8) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((r9) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x28u64)) as *const u64)) as u64;
    xmm0 = (core::ptr::read_unaligned((rax) as *const u128)) as u128;
    rax = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u128, (xmm0) as u128);
    rdx = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x28u64)) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14985(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r10 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    let fl_l: u64 = r9; let fl_r: u64 = 0xffffffffffffffffu64;
    if !(((fl_l) != (fl_r))) {
        r9 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
        let fl_l: u64 = r9; let fl_r: u64 = r8;
        r9 = (fl_l.wrapping_sub(fl_r)) as u64;
    }
    core::ptr::write_unaligned((rdx) as *mut u64, (r9) as u64);
    rax = (rdx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (r10) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14993(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14997(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_14998(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u64 = (core::ptr::read_unaligned((rcx) as *const u64)).wrapping_add(0x8u64);
    core::ptr::write_unaligned((rcx) as *mut u64, (fl_res) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15013(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x28u64)) as *mut u64, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15079(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx.wrapping_add(0x4u64)) as *mut u8, (0x1u8) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15218(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15457(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rdx) as *mut u64, (rcx) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15506(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    core::ptr::write_unaligned((rcx) as *mut u8, (0x1u8) as u8);
    rax = (rcx) as u64;
    xmm0 = (core::ptr::read_unaligned((rdx) as *const u128)) as u128;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15586(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u8, (0x1u8) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15587(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx) as *mut u32, (0x2u32) as u32);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x4u64)) as *mut u8, (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15623<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_l: u8 = core::ptr::read_unaligned((rcx) as *const u8); let fl_r: u8 = 0x0u8;
    if ((fl_l) == (fl_r)) {
        return rax;
    }
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    let fl_l: u64 = rcx; let fl_r: u64 = rcx;
    if !((((fl_l) & (fl_r)) == 0)) {
        crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P0) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
    }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15635<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((core::ptr::read_unaligned((rax.wrapping_add(0x10u64)) as *const u64)) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (core::ptr::read_unaligned((rax.wrapping_add(0x1928u64)) as *const u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P0) as *const u64)) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15722<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u8)) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(0x10u64)) as u64) as u64;
    let fl_res: u64 = (rdx).wrapping_add(0x10u64);
    rdx = (fl_res) as u64;
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u8, ((rax as u8)) as u8);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15733(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((r8) as *const u32)) as u32 as u64;
    let fl_l: u32 = core::ptr::read_unaligned((rdx) as *const u32); let fl_r: u32 = (rax as u32);
    rax = (rax & !0xffu64) | (((if ((fl_l) < (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15752(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rcx.wrapping_add(0x20u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x14u64)) as *mut u32, ((core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32)).wrapping_add(1)) as u32);
    r8 = ((rdx.wrapping_add(0xfu64)) as u64) as u64;
    let fl_res: u64 = ((r8) & (0xfffffffffffffff0u64));
    r8 = (fl_res) as u64;
    rax = ((r8.wrapping_add(0x18u64)) as u64) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x20u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((r8) as *mut u64, (rdx) as u64);
    core::ptr::write_unaligned((r8.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((r8.wrapping_add(0x10u64)) as *mut u32, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15859(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (rcx) as u64;
    xmm0 = (core::ptr::read_unaligned((r8) as *const u128)) as u128;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    xmm0 = (core::ptr::read_unaligned((r9) as *const u128)) as u128;
    core::ptr::write_unaligned((rcx.wrapping_add(0x18u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_15884<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (rcx) as u64;
    rcx = (core::ptr::read_unaligned((P0) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16164<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16295(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0x666666666666666u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16339<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((P0) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(0x58u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16379(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax) as *const u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64); let fl_r: u64 = rcx;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16422(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_l: u64 = rdx; let fl_r: u64 = rcx;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_res: u32 = ((core::ptr::read_unaligned((rcx) as *const u32)) | ((rdx as u32)));
    core::ptr::write_unaligned((rcx) as *mut u32, (fl_res) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16611<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((P0) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((P1) as *mut u64, (rax) as u64);
    rax = ((P2) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16755(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut xmm0: u128 = 0;
    core::ptr::write_unaligned((rcx) as *mut u64, (rdx) as u64);
    rax = (rcx) as u64;
    xmm0 = (core::ptr::read_unaligned((r8) as *const u128)) as u128;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u128, (xmm0) as u128);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_16757(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x18u64)) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17019(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    r8 = (0u32) as u32 as u64;
    core::ptr::write_unaligned((rcx) as *mut u64, (rax) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u64, (r8) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x8u64)) as *const u64)) as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x10u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64;
    core::ptr::write_unaligned((rcx.wrapping_add(0x14u64)) as *mut u32, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    core::ptr::write_unaligned((rdx.wrapping_add(0x8u64)) as *mut u64, (r8) as u64);
    core::ptr::write_unaligned((rdx.wrapping_add(0x10u64)) as *mut u64, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17118<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbx: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rbx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
    let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
    let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
    let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17253<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((P0) as u64) as u64;
    core::ptr::write_unaligned((P1) as *mut u64, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17321(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    core::ptr::write_unaligned((rcx.wrapping_add(0x70u64)) as *mut u8, (0x0u8) as u8);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17379<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r9 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rcx = (core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u64)) as u64;
    rax = (0xffffffffffffu64) as u64;
    rcx = (((rcx) & (rax))) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rdx = (r8) as u64;
    let fl_res: u64 = (rcx).wrapping_add(rax);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17380<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64;
    r11 = (r8) as u64;
    r9 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = ((P0) as u64) as u64;
    r10 = (core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u64)) as u64;
    rax = (0xffffffffffffu64) as u64;
    r10 = (((r10) & (rax))) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rdx = (r11) as u64;
    let fl_res: u64 = (r10).wrapping_add(rax);
    r10 = (fl_res) as u64;
    rcx = (r10) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17491<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((P0) as *const u64); let fl_r: u64 = 0x0u64;
    if !(((fl_l) != (fl_r))) {
        let s5 = crate::simd_ops::read_s5_call(rsp, &stack);
        let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64);
        let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64);
        let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
        crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
        core::ptr::write_unaligned((P2) as *mut u64, (rax) as u64);
    }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17510<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    rdx = ((P0) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17538<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (r8) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17552(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rax = ((rcx.wrapping_add(0x18u64)) as u64) as u64;
    let fl_l: u64 = core::ptr::read_unaligned((rcx) as *const u64); let fl_r: u64 = rax;
    rax = (rax & !0xffu64) | (((if ((fl_l) == (fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17902<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17903(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbp: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17904<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x20u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17905<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x50u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x48u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17907<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x58u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17908<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x40u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17912<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x70u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17913<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x58u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x10u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17914<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x28u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17915<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x70u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x18u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17916<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x18u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17917<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17919<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x30u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17922<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x60u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x20u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17924<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x50u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17930<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x48u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17931<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x70u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x20u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17933<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x40u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x28u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17934<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x60u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x28u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17935<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x60u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x10u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17936<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((P0) as u64) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17941<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x60u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17951(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbp: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbp = (rdx) as u64;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17959<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x20u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17961<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17964<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x40u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x10u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17965<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x50u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x10u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17976<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x68u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17988<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x60u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x40u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17989<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x10u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17990<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x50u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x30u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17991<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x70u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x30u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17996<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x48u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_17997<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x78u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18000<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x58u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18024<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x50u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18031(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut rbp: u64 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<128>::new();
    let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18051<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x50u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18082<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x38u64)) as *const u64)) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18115<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x40u64)) as u64) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18165<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x20u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((core::ptr::read_unaligned((P0) as *const u64)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18174<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x60u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18175<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x58u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18177<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x80u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18178<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x70u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18180<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0xa0u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18181<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x48u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18184<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x38u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18187<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x20u64)) as u64) as u64;
    let fl_res: u64 = (rcx).wrapping_add(0x8u64);
    rcx = (fl_res) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18205<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x28u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18207<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x30u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18208<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x68u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18211<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x20u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18213<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x40u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18214<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x38u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18215<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0xd8u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18225<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x88u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18232<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0xe0u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18235<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0xc0u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18236<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x130u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18252<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0xa0u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18261<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x50u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18263<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x90u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18264<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x48u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18267<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x40u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18271<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x70u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18274<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x120u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18282<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0xb0u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18287<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x78u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18288<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x90u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18291<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = (core::ptr::read_unaligned((rdx.wrapping_add(0x1e0u64)) as *const u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18293<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0x78u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18298<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((P0) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18335<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rcx = ((rdx.wrapping_add(0xd0u64)) as u64) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18434<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let fl_l: u8 = core::ptr::read_unaligned((P0) as *const u8); let fl_r: u8 = 0x0u8;
    if !(((fl_l) == (fl_r))) {
        rcx = ((P1) as u64) as u64;
        crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
    }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_leaf_18436<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    rdx = (0x20u32) as u32 as u64;
    r9 = ((P0) as u64) as u64;
    rcx = ((P1) as u64) as u64;
    r8 = ((rdx.wrapping_sub(0x1du64)) as u32) as u32 as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}
