#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_0<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = (0x1u32) as u32 as u64;
    rdx = ((P0) as u64) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P3) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_1<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (0x2u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P1) as u64) as u64;
    rbx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P3) as u64) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (rbx) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P5) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_2<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rdx = ((P0) as u64) as u64;
    crate::simd_ops::wu64(P1, (rdx) as u64);
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P2))) as i32 as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P2))) as i32 as i64) as u64) as u64;
    fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(rdx).wrapping_add(P5), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_3<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P2) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_4<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rdx = ((P0) as u64) as u64;
    crate::simd_ops::wu64(P1, (rdx) as u64);
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P2))) as i32 as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_5<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_6<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(P0)) as u64;
    r8 = ((P1) as u64) as u64;
    crate::simd_ops::wu64(P2, (r8) as u64);
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P3))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), (r8) as u64); r8 = (crate::simd_ops::ru64(P2)) as u64; }
    rdx = (crate::simd_ops::ru64(P5)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P3))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), (r8) as u64); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_7<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((P0) as u64) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_8<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (0x2u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (rax) as u64;
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P3) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_9<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (rax) as u64;
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P3) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_10<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (0x1u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (rax) as u64;
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P3) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_11(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_12<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_13<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_14<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = (rsp) as u64;
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64);
    crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (rax) as u64);
    crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (rax) as u64);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_15<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_16<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = ((P0) as u64) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_17<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_18<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm1) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_19<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((r9 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_20<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm1) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm1) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rbx.wrapping_add(P8), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rax as u32)) as u32);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_21<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_22<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rdx) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rdi = (rcx) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    rax = (rdi) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_23<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rax = (rdx) as u64;
    rcx = ((P0) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_24<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_25<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_26<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rdx) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rdi = (rcx) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    rax = ((P4) as u64) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P5))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    rax = (rdi) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P5), (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx).wrapping_add(P0), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(P3), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx = (0x10u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rdx = (0x10u32) as u32 as u64;
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = (rcx) as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0x0u32) as u32)) as u32 as u64; fl_r = fl_l;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_32<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x98u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_33<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdi = (rax) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (rax) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    rax = ((P4) as u64) as u64;
    rax = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(P1)) } else { rax }) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_34<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_35<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; }
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::lk_sub_u32(rax, (1) as u32)) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rdx, (rcx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_39<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P1) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_40<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_41<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_42<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_43<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (((rdx as u8)) as u32) as u32 as u64;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((rax as u32)) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_44<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (r8) as u64;
    rbx = (rdx) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_45<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (((rdx as u16)) as u32) as u32 as u64;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (((rax as u16)) as u32) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_46<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (r8) as u64;
    rbx = (rdx) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu16(rbx, ((rax as u16)) as u16); rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_47<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_48<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_sub_u32(rax, ((rdx as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_49<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (rcx) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_50<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (rax) as u64;
    rcx = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_51<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64;
    r11 = (r9) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r10) as u64);
    r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r10) as u64);
    r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r10) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64);
    r9 = (r11) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_52<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_53<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r10) as u64);
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_54<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_55<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_56<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    crate::simd_ops::wu64(rax, (rdx) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rax, (r8) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu32(rax, ((r9 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_57<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; }
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_58<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_shr(2u32)) as u32 as u64; r9 = ((((r9 as u32)) | (0x1u32))) as u32 as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (rsi & !0xffu64) | ((((((rsi as u8)) & (0x1u8))) as u8 as u64)); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; r10 = (rcx) as u64; rcx = ((rbx as u32)) as u32 as u64; r9 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rdx as u32) } else { (r9 as u32) }) as u32 as u64; rdx = ((r9 as u32)) as u32 as u64; rdx = ((((rdx as u32)) | (0x2u32))) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r9 as u32) } else { (rdx as u32) }) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffbbu32))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((P0) as u64) as u64; r9 = ((P1) as u64) as u64; bb = 2; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rax = ((P2) as u64) as u64; rcx = (r10) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            8 => { rbx = (rdi) as u64; bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_59<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_60<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_61<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x30u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_62<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_63<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rdx) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64);
    loop { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_64<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = (rsp) as u64;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x70u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    rax = ((r11.wrapping_add(0x10u64)) as u64) as u64;
    rdi = (rdx) as u64;
    xmm1 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (rax) as u64);
    r9 = ((r11.wrapping_sub(0x38u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64;
    r8 = ((r11.wrapping_sub(0x28u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (rax) as u64);
    rdx = ((r11.wrapping_sub(0x18u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), (rax) as u64);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = (rdi) as u64;
    crate::simd_ops::wu128(rdi, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x70u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_65<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = (rsp) as u64;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x70u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    rax = ((r11.wrapping_add(0x10u64)) as u64) as u64;
    rdi = (rdx) as u64;
    xmm1 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (rax) as u64);
    r9 = ((r11.wrapping_sub(0x38u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64;
    r8 = ((r11.wrapping_sub(0x28u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (rax) as u64);
    rdx = ((r11.wrapping_sub(0x18u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), (rax) as u64);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64;
    crate::simd_ops::wu128(rdi, (xmm0) as u128);
    crate::simd_ops::wu64(rax, ((xmm1 as u64)) as u64);
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x70u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_66<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_67<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_68<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); r10 = ((rdi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P3), ((r11 as u8)) as u8); rax = ((rdi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), (rax) as u64); r8 = ((rdi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P7), ((r11 as u8)) as u8); r9 = ((rdi.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), (r10) as u64); rcx = ((rdi.wrapping_add(P10)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P11), (r8) as u64); rdx = ((rdi.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), (r9) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rdx) as u64); crate::simd_ops::wu64(r8, (r11) as u64); crate::simd_ops::wu64(rcx, (r11) as u64); crate::simd_ops::wu32(rax, ((r11 as u32)) as u32); crate::simd_ops::wu64(r10, (r11) as u64); crate::simd_ops::wu64(r9, (r11) as u64); crate::simd_ops::wu32(rdx, ((r11 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), (rbx) as u64); rax = (crate::simd_ops::ru64(P12)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x74u64), (rax) as u64); rax = (rdi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), (r11) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_69<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rdx) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), (rdx) as u64);
    crate::simd_ops::wu16(rcx.wrapping_add(P11), ((rdx as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_70<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rsi = ((rcx.wrapping_add(0x18u64)) as u64) as u64; rdi = (rcx) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rcx.wrapping_add(0x70u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64 as u64; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rbx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rbx = (((rbx) & (rdi))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx } else { r9 }) as u64; bb = 5; continue; }
            4 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rdi.wrapping_add(0x4cu64)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), (rcx) as u64); rax = ((rdi.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); r8 = ((rdi.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x7cu64), (0x0u8) as u8); rax = ((rdi.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), (rax) as u64); rdx = ((rdi.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x72u64), (0x0u8) as u8); rax = ((rdi.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rax) as u64); rax = (r9) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (r8) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rdx) as u64); crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdx, (0x0u64) as u64); crate::simd_ops::lk_and_u32(rcx, (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; crate::simd_ops::lk_and_u32(rcx, (0x0u32) as u32); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x80u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x68u64), (0x0u64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x74u64), (rcx) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = ((rcx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_72<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_sub_u32(rax, ((rdx as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_73<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (rsi & !0xffu64) | ((((r8 as u8)) as u8 as u64)); rbx = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64); rdx = (rdx & !0xffu64) | (((0x20u8) as u8 as u64)); rcx = (rdi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; crate::simd_ops::wu16(rdi.wrapping_add(P4), ((rax as u16)) as u16); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rax = ((((rax as u32)) & (0x13u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); fl_l = ((((rax as u32)) & (crate::simd_ops::ru32(rdi.wrapping_add(P6))))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = ((P8) as u64) as u64; bb = 8; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P9) as u64) as u64; rax = ((P10) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 8; continue; }
            8 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P13) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_74<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (r8) as u64; rbx = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r9.wrapping_add(P0)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(r9)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r11 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r10 = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); r8 = ((rbx.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x1u8) as u8); rax = ((rbx.wrapping_add(P7)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P8), (rax) as u64); r9 = ((rbx.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P10), (r10) as u64); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (r8) as u64); rdx = ((rbx.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (r9) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (rdx) as u64); crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); crate::simd_ops::lk_and_u32(rax, (0x0u32) as u32); crate::simd_ops::lk_and_u64(r10, (0x0u64) as u64); crate::simd_ops::lk_and_u64(r9, (0x0u64) as u64); crate::simd_ops::lk_and_u32(rdx, (0x0u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), (r11) as u64); rax = (crate::simd_ops::ru64(P12)) as u64; fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x68u64), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x74u64), (rax) as u64); rax = (rbx) as u64; bb = 8; continue; }
            7 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(r8)) as u64;
    fl_l = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64 as u64;
    r9 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rax, (rdx) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rax, (r8) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu32(rax, ((r9 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_76<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0)) { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; } else { rax = (0x1u32) as u32 as u64; }
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = ((rax.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = ((r14.wrapping_add(0x2cu64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rcx = ((P4) as u64) as u64; r8 = (rbp) as u64; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rbx) } else { rcx }) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rcx = ((P7) as u64) as u64; r8 = (rbp) as u64; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rbx) } else { rcx }) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbp) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), (rax) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x20u64), (rax) as u64); rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    rdi = ((rcx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::lk_and_u64(rdi, (0x0u64) as u64);
    rsi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (rax) as u64;
    rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi, (rbx) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; }
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (rsi) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_81<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P6) as u64) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x78u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_82<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x20u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rdi) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_83<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    r10 = ((r9 as u32)) as u32 as u64;
    r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64;
    r11 = (r8) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rcx = (rdx) as u64;
    rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    r8 = ((r10 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64);
    rdx = (r11) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (rax) as u64;
    rdi = ((rdi).wrapping_add(0x10u64)) as u64;
    rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi, (rbx) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; }
    rcx = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_85<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r10 = (r9) as u64;
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    r11 = (r8) as u64;
    rbx = (rdx) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64);
    r8 = (r10) as u64;
    rdx = (r11) as u64;
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rax as u32)) as u32 as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rdx as u32) } else { (rax as u32) }) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_86<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rdx) as u64;
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    rax = (rdx) as u64;
    r8 = ((r9 as u32)) as u32 as u64;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (r9) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (r9) as u64);
    fl_l = (crate::simd_ops::ru16(rdx)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64;
    if (((fl_l as u16)) == ((fl_r as u16))) { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    loop { r8 = ((r8).wrapping_add(1)) as u64; rax = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if !((((fl_l as u16)) != ((fl_r as u16)))) { break; } }
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_87<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rdx) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64);
    loop { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if !((((fl_l as u16)) != ((fl_r as u16)))) { break; } }
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_88<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (r9) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = (r8) as u64;
    rbx = (rdx) as u64;
    rcx = (rdi) as u64;
    rdx = (rax) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (crate::simd_ops::lk_or_u32(rax, (0x2u32) as u32)) as u32 as u64; fl_r = fl_l; } else { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (((((rcx as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32); }
    xmm0 = (crate::simd_ops::ru128(rdi)) as u128;
    rax = (rbx) as u64;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_90<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_91<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rdi = (rdx) as u64; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x25u8) as u8); rbx = (r9) as u64; rbp = (r8) as u64; rsi = (rcx) as u64; fl_l = ((((rdx as u32)) & (0x20u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), (0x2bu8) as u8); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x51u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x52u64)) as u64) as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { r8 } else { rcx }) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx, (0x23u8) as u8); fl_l = ((rcx).wrapping_add(1)) as u64 as u64; rcx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(rcx, (0x6cu8) as u8); rdx = ((((rdx as u32)) & (0xe00u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x400u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x6fu8) as u8 as u64)); bb = 9; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x64u8) as u8 as u64)); bb = 9; continue; }
            8 => { rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x20u8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x58u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x1u64), ((rax as u8)) as u8); rdx = (0x40u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x100u64))) as u32) as u32 as u64; r9 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (rsi) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rdx as u16)) as u16); rdx = (rdi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_92<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rdi = (rdx) as u64; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x25u8) as u8); rbx = (r9) as u64; rbp = (r8) as u64; rsi = (rcx) as u64; fl_l = ((((rdx as u32)) & (0x20u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), (0x2bu8) as u8); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x51u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x52u64)) as u64) as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { r8 } else { rcx }) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx, (0x23u8) as u8); fl_l = ((rcx).wrapping_add(1)) as u64 as u64; rcx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(rcx, (0x6cu8) as u8); rdx = ((((rdx as u32)) & (0xe00u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x400u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x6fu8) as u8 as u64)); bb = 9; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x75u8) as u8 as u64)); bb = 9; continue; }
            8 => { rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x20u8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x58u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x1u64), ((rax as u8)) as u8); rdx = (0x40u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x100u64))) as u32) as u32 as u64; r9 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (rsi) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rdx as u16)) as u16); rdx = (rdi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_93<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rax) as u64); rsi = (r9) as u64; rbx = (r8) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r8 = ((P1) as u64) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rdx = (0x40u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0xf0u64))) as u32) as u32 as u64; r9 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (rdi) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rdx as u16)) as u16); rdx = (rbp) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbp) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_94<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); rdi = (rdx) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x25u8) as u8); rsi = (rcx) as u64; rbx = (r9) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rbp = (r8) as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = ((((rdx as u32)) & (0x20u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), (0x2bu8) as u8); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x51u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x52u64)) as u64) as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { r8 } else { rax }) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rax, (0x23u8) as u8); fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu16(rax, (0x3649u16) as u16); rdx = ((((rdx as u32)) & (0xe00u32))) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x2u64), (0x34u8) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x400u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((0x6fu8) as u8 as u64)); bb = 9; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rcx & !0xffu64) | (((0x64u8) as u8 as u64)); bb = 9; continue; }
            8 => { rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((!((rcx as u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x20u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | (0x58u8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x3u64), ((rcx as u8)) as u8); rdx = (0x40u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x0u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x100u64))) as u32) as u32 as u64; r9 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (rsi) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rdx as u16)) as u16); rdx = (rdi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_95<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); rdi = (rdx) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x25u8) as u8); rsi = (rcx) as u64; rbx = (r9) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as u32 as u64; rbp = (r8) as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = ((((rdx as u32)) & (0x20u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), (0x2bu8) as u8); bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x51u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x52u64)) as u64) as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { r8 } else { rax }) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rax, (0x23u8) as u8); fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu16(rax, (0x3649u16) as u16); rdx = ((((rdx as u32)) & (0xe00u32))) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x2u64), (0x34u8) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x400u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((0x6fu8) as u8 as u64)); bb = 9; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x800u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rcx & !0xffu64) | (((0x75u8) as u8 as u64)); bb = 9; continue; }
            8 => { rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((!((rcx as u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x20u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | (0x58u8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x3u64), ((rcx as u8)) as u8); rdx = (0x40u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x0u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x100u64))) as u32) as u32 as u64; r9 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (rsi) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rdx as u16)) as u16); rdx = (rdi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_96<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = (rsp) as u64;
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64;
    rbx = (rdx) as u64;
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = ((r11.wrapping_sub(0x28u64)) as u64) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), (r8) as u64);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), (r8) as u64);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (r8) as u64);
    r8 = ((r11.wrapping_sub(0x18u64)) as u64) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_97<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), ((xmm0 as u64)) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; xmm6 = (xmm1) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((((rax as u16)) as i16 as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 3; continue; }
            3 => { xmm0 = 0u128; fl_l = ((xmm6 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 15; continue; } bb = 5; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (xmm1) ^ (crate::simd_ops::ru128(P1)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = ((((r8 as u32)) | (0xffffffffu32))) as u32 as u64; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm1 = ((crate::simd_ops::ru64(P3)) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm6 as u64)).to_bits() as u128); bb = 16; continue; }
            9 => { xmm1 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((((rax as u16)) as i16 as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (0x8u32) as u32 as u64; bb = 13; continue; }
            12 => { rcx = (0x10u32) as u32 as u64; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; bb = 16; continue; }
            15 => { xmm0 = (xmm6) as u128; bb = 16; continue; }
            16 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_98<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), (rax) as u64); rbp = (r8) as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 21; continue; } bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rbx = (0x4u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm2 = ((crate::simd_ops::ru64(P2)) as u64) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            5 => { r8 = ((P4) as u64) as u64; rdi = (0u32.wrapping_sub((rdi as u32))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (0x68u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rsp.wrapping_add(0x70u64)) as u64) as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbx as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { fl_l = (((((rdi as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsp.wrapping_add(0x70u64)) as u64) as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (0xffffff98u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rdx = ((rbx as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64; xmm6 = (xmm0) as u128; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 14; continue; } bb = 13; continue; }
            13 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 18; continue; } bb = 14; continue; }
            14 => { xmm1 = ((crate::simd_ops::ru64(P10)) as u64) as u128; fl_l = ((xmm6 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 16; continue; } bb = 15; continue; }
            15 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 18; continue; } bb = 16; continue; }
            16 => { xmm1 = (xmm1) ^ (crate::simd_ops::ru128(P11)); fl_l = ((xmm6 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 20; continue; } bb = 17; continue; }
            17 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 20; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x22u32) as u32); fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_or_u32(rbp, (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 20; continue; }
            20 => { xmm0 = (xmm6) as u128; bb = 22; continue; }
            21 => { rdx = (0x4u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0xd0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_99<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm1 = (xmm0) as u128;
    xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P0))).to_bits() as u128);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_100<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = ((r9 as u32)) as u32 as u64; rsi = (r8) as u64; r14 = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru64(rsi)) as u64) as u128; fl_l = ((xmm2 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P4)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r14 as u32)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_101<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = ((((rdx as u32)) as i32 as i64) as u64) as u64; xmm7 = (xmm2) as u128; rdi = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(rcx))).to_bits() as u128); rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), ((xmm0 as u64)) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 7; continue; } bb = 2; continue; }
            2 => { rbx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64) as u128; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, ((xmm0 as u64)) as u64); bb = 5; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rdi, ((xmm0 as u64)) as u64); fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) >= (((fl_r as u16)) as i16)) { bb = 25; continue; } bb = 5; continue; }
            5 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r8) as u64); bb = 25; continue; }
            7 => { crate::simd_ops::wu64(rdi, (r8) as u64); bb = 8; continue; }
            8 => { rbx = (0x1u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((rax as u32)) as u32); r14 = ((rbx as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 25; continue; } bb = 9; continue; }
            9 => { r12 = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; r13 = (r8) as u64; fl_l = (r12) as u64 as u64; fl_r = (rdi) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rsi = ((rbx as u32)) as u32 as u64; xmm6 = 0u128; bb = 10; continue; }
            10 => { fl_l = (rsi) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = ((rax.wrapping_add(r14)) as u32) as u32 as u64; rax = ((rsi.wrapping_add(r13)) as u64) as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 12; continue; }
            12 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 15; continue; } bb = 14; continue; }
            14 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 17; continue; } bb = 15; continue; }
            15 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm7 as u64)).to_bits() as u128); r14 = (((r14 as u32)).wrapping_add((rbx as u32))) as u32 as u64; crate::simd_ops::wu64(rcx, (r8) as u64); rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(r12.wrapping_add(rcx), ((xmm0 as u64)) as u64); rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rbp.wrapping_add(rsi.wrapping_mul(8)).wrapping_sub(0x50u64), (r8) as u64); rsi = (0x8u32) as u32 as u64; r14 = ((rsi as u32)) as u32 as u64; bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 20; continue; } bb = 19; continue; }
            19 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 25; continue; } bb = 20; continue; }
            20 => { rdx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), ((xmm0 as u64)) as u64); rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x1au32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (((crate::simd_ops::ru16(rbp.wrapping_sub(0x60u64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64) as u128; rdx = ((r15 as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64) as u128; rdx = ((r15 as u32)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) - f64::from_bits(crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64)))).to_bits() as u128); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 24; continue; } bb = 21; continue; }
            21 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x50u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x58u64), ((xmm0 as u64)) as u64); if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 23; continue; } bb = 22; continue; }
            22 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 21; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; r13 = ((r13).wrapping_add(rbx)) as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; r12 = ((r12).wrapping_sub(0x8u64)) as u64; r14 = (((r14 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((rax as u32)) as u32); rsi = ((rsi).wrapping_sub(rbx)) as u64; r8 = (0x0u32) as u32 as u64; fl_l = (r13) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 25; continue; }
            25 => { rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_102<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((((r9 as u32)) as i32 as i64) as u64) as u64; rsi = (r8) as u64; rbx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { xmm2 = ((crate::simd_ops::ru64(r8)) as u64) as u128; fl_l = ((xmm2 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P0)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P0)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 7; continue; } bb = 5; continue; }
            5 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = ((rbx.wrapping_mul(8)) as u64) as u64; r8 = (r14) as u64; rdx = (rdi) as u64; rcx = (r15) as u64; r12 = ((r14.wrapping_add(r15)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru64(rsi)) as u64) as u128; rdx = ((rbx as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rbp) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 8; continue; }
            8 => { rbp = (0x1u32) as u32 as u64; bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(rbp.wrapping_mul(8)))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P0)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 11; continue; } bb = 10; continue; }
            10 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (r14) as u64; rdx = (r15) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru64(rsi.wrapping_add(rbp.wrapping_mul(8)))) as u64) as u128; rdx = ((rbx as u32)) as u32 as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rbx as u32)) as u32 as u64; r8 = (r12) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rdi) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_103<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm2 = (((r8 as u32)) as u32) as u128;
    rbx = (rcx) as u64;
    xmm2 = crate::simd_ops::vcvtdq2pd(xmm2);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_104<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), ((xmm2 as u64)) as u64); xmm6 = (xmm2) as u128; rsi = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x18u64)) as u64) as u64; rcx = ((rax.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rdi = ((rbx.wrapping_add(0x8u64)) as u64) as u64; if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rdi, (rbp) as u64); bb = 15; continue; }
            5 => { rdx = (0x1au32) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (((crate::simd_ops::ru16(rsp.wrapping_add(0x58u64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; crate::simd_ops::wu64(rbx, ((xmm0 as u64)) as u64); xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) - f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu64(rdi, ((xmm6 as u64)) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = ((xmm6 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P3)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 9; continue; } bb = 7; continue; }
            7 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rbp) as u64); bb = 15; continue; }
            9 => { rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x1au32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (((crate::simd_ops::ru16(rsp.wrapping_add(0x58u64))) as i16 as i32) as u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) - f64::from_bits(crate::simd_ops::ru64(rdi))).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = ((xmm6 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P3)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 15; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rbp) as u64); bb = 15; continue; }
            13 => { xmm6 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx, ((xmm6 as u64)) as u64); bb = 15; continue; }
            15 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_105<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_106<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x8u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x108u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rsi = (r9) as u64; rdi = ((r8 as u32)) as u32 as u64; r14 = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = ((rax as u32)) as u32 as u64; r12 = ((((r12 as u32)) & (0x8u32))) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_and_u32(rsi, (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { rax = ((((rax as u32)) & (0xfffffff7u32))) as u32 as u64; r13 = (0x1u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru64(P3)) as u64) as u128; rbx = ((r13.wrapping_add(0x3u64)) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm2 = 0u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (r14) as u64; fl_l = (r14) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 6; continue; }
            6 => { r14 = (r13) as u64; bb = 7; continue; }
            7 => { rax = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((r14).wrapping_add(r13)) as u64; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x40u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdi = (((rdi as u32)).wrapping_add(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; r8 = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru64(P10)) as u64) as u128; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rbx = (0x4u32) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm2 = 0u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            12 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (r14) as u64; fl_l = (r14) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 13; continue; }
            13 => { r14 = (r13) as u64; bb = 14; continue; }
            14 => { rax = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((r14).wrapping_add(r13)) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x40u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 15; continue; }
            15 => { r8 = (rsi) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru64(P12)) as u64) as u128; bb = 21; continue; }
            18 => { rbx = (0x4u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru64(P13)) as u64) as u128; bb = 21; continue; }
            20 => { xmm0 = 0u128; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); bb = 22; continue; }
            22 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (0x8000u32) as u32 as u64; fl_l = (crate::simd_ops::lk_xor_u16(rax, ((rcx as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 24; continue; }
            24 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x108u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_107<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (rcx) as u64); rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rsi = (r9) as u64; rdi = ((r8 as u32)) as u32 as u64; r14 = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = ((rax as u32)) as u32 as u64; r12 = ((((r12 as u32)) & (0x8u32))) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_and_u32(rsi, (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { rax = ((((rax as u32)) & (0xfffffff7u32))) as u32 as u64; r13 = (0x1u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; r9 = ((rbp.wrapping_sub(0x51u64)) as u64) as u64; r8 = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rbx = ((r13.wrapping_add(0x3u64)) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm2 = 0u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (r14) as u64; fl_l = (r14) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 6; continue; }
            6 => { r14 = (r13) as u64; bb = 7; continue; }
            7 => { rax = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbx as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((r14).wrapping_add(r13)) as u64; rcx = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(r14.wrapping_mul(4)).wrapping_sub(0x51u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdi = (((rdi as u32)).wrapping_add(crate::simd_ops::ru32(rbp.wrapping_sub(0x51u64)))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; r8 = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; r9 = ((rbp.wrapping_sub(0x51u64)) as u64) as u64; r8 = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; rcx = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; rbx = (0x4u32) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm2 = 0u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            12 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (r14) as u64; fl_l = (r14) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 13; continue; }
            13 => { r14 = (r13) as u64; bb = 14; continue; }
            14 => { rax = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rbp.wrapping_sub(0x21u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = ((rbx as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = ((r14).wrapping_add(r13)) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(r14.wrapping_mul(4)).wrapping_sub(0x51u64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rbx as u32)) as u32 as u64; r8 = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 15; continue; }
            15 => { r8 = (rsi) as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x51u64))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x59u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((xmm0 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(P12)) as u32) as u128; bb = 20; continue; }
            18 => { rbx = (0x4u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((xmm0 as u32)) as u32); bb = 22; continue; }
            21 => { fl_l = (crate::simd_ops::lk_and_u32(rbp.wrapping_sub(0x59u64), (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 22; continue; }
            22 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = ((rbp.wrapping_sub(0x59u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (0x8000u32) as u32 as u64; fl_l = (crate::simd_ops::lk_xor_u16(rax, ((rcx as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 24; continue; }
            24 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x59u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_108<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8);
    rax = (rdx) as u64;
    rcx = ((P0) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64);
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_109<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P1) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_110<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_111<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; r15 = ((rcx.wrapping_add(0x8u64)) as u64) as u64; rax = ((P0) as u64) as u64; rsi = (rdx) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15, (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rcx) as u64); r14 = ((rbx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(r14, (rcx) as u64); rbp = ((rbx.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rbp, (rcx) as u64); r12 = ((rbx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(r12, (rcx) as u64); r13 = ((rbx.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu32(r13, ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (rcx) as u64); rcx = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rdi = (rcx) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rbp) as u64); rax = ((rbx.wrapping_add(0x4cu64)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (r15) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (r13) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu64(r14, (rcx) as u64); crate::simd_ops::wu64(r12, (rcx) as u64); crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu64(r15, (rcx) as u64); crate::simd_ops::wu64(rbp, (rcx) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(r13, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r14, (rcx) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_112<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (rcx) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx).wrapping_add(P0), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(P3), ((r8 as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P4), (rax) as u64);
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P6), ((rdx as u32)) as u32);
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rax = ((P7) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P0), (rax) as u64);
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_113<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_114<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = (rdx) as u64;
    rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    rdi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64;
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_115<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8);
    rbx = (rdx) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64);
    rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_116<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = (r9) as u64; rax = ((P0) as u64) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; r11 = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rbx = (rdx) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (r10) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (r11) as u64; rcx = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (((rax as u16)) as u32) as u32 as u64; rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_shr(8u32)) as u16 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx, (0x0u8) as u8); bb = 3; continue; }
            3 => { rdx = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rdx = (if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_117<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = (rcx) as u64;
    rdi = (rdx) as u64;
    rbx = (r8) as u64;
    rdx = (rdx & !0xffu64) | (((0xau8) as u8 as u64));
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(rsi)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r9 = (r9 & !0xffu64) | ((((rax as u8)) as u8 as u64));
    r8 = (rbx) as u64;
    rdx = (rdi) as u64;
    rcx = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_118<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rdx) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (0xffffu32) as u32 as u64;
    fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    if !((((fl_l as u16)) == ((fl_r as u16)))) { crate::simd_ops::wu16(rdi, ((rax as u16)) as u16); }
    rax = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_119<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = (rcx) as u64;
    rbx = (rdx) as u64;
    rdx = (rdx & !0xffu64) | (((0xau8) as u8 as u64));
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(rdi)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rdx = (rbx) as u64;
    rcx = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_120<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = (rcx) as u64;
    rdi = (rdx) as u64;
    rbx = (r8) as u64;
    rdx = (rdx & !0xffu64) | (((0xau8) as u8 as u64));
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(rsi)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r9 = (((rax as u16)) as u32) as u32 as u64;
    r8 = (rbx) as u64;
    rdx = (rdi) as u64;
    rcx = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_121<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64;
    rbx = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32);
    r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64));
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r8 as u8)) as u8);
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_122<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = (rdx) as u64;
    r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64));
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64) as u128;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r8 as u8)) as u8);
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_123<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64;
    rbx = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r8) as u64);
    r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64));
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r8 as u8)) as u8);
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_124<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64;
    rbx = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32);
    r8 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((r8 as u16)) as u16);
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_125<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = (rdx) as u64;
    r8 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64) as u128;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((r8 as u16)) as u16);
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_126<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64;
    rbx = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r8) as u64);
    r8 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((r8 as u16)) as u16);
    r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_127<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((P0) as u64) as u64; rdi = ((P0) as u64) as u64; bb = 4; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_128<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x4u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xfffffffbu32) as u32); rcx = ((rbp.wrapping_add(0x138u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_129<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x10u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xffffffefu32) as u32); rcx = ((rbp.wrapping_add(0x158u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_130<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x20u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xffffffdfu32) as u32); rcx = ((rbp.wrapping_add(0x178u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e970c0; const P1: u64 = 0x7001e97168; const P2: u64 = 0x7001e6b3d0; const P3: u64 = 0x7001e76bf0; const P4: u64 = 0x7001e970d0; const P5: u64 = 0x7001e163a4; const P6: u64 = 0x7001e67ab0; const P7: u64 = 0x7001e657dc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[0], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rdx = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64);
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    r8 = ((P3) as u64) as u64;
    rcx = ((P4) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15038); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P6) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97000; const P1: u64 = 0x7001e970a8; const P2: u64 = 0x7001e6b3a8; const P3: u64 = 0x7001e76b90; const P4: u64 = 0x7001e97010; const P5: u64 = 0x7001e163a4; const P6: u64 = 0x7001e67ac0; const P7: u64 = 0x7001e657dc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[1], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rdx = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64);
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    r8 = ((P3) as u64) as u64;
    rcx = ((P4) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15088); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P6) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e151c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69588; const P1: u64 = 0x7001e98cf0; const P2: u64 = 0x7001e17c90; const P3: u64 = 0x7001e6bd28; const P4: u64 = 0x7001e1a548; const P5: u64 = 0x7001e67bb0; const P6: u64 = 0x7001e657dc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P1) as u64) as u64;
    rbx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P3) as u64) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (rbx) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P5) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e69588; const P1: u64 = 0x7001e98fb0; const P2: u64 = 0x7001e17c90; const P3: u64 = 0x7001e6bd28; const P4: u64 = 0x7001e1a548; const P5: u64 = 0x7001e67cb0; const P6: u64 = 0x7001e657dc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (0x1u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P1) as u64) as u64;
    rbx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((P3) as u64) as u64;
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = (rbx) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    rcx = ((P1) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P5) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x8; const P2: u64 = 0x7001e6642c; const P3: u64 = 0x7001e6b3d0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[2], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (rdx) as u64;
    rbx = (rcx) as u64;
    rcx = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15a78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15a94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x8; const P2: u64 = 0x7001e6b3e0; const P3: u64 = 0x7001e6b3d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[3], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15ac4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x8; const P2: u64 = 0x7001e6642c; const P3: u64 = 0x7001e6b3a8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[4], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (rdx) as u64;
    rbx = (rcx) as u64;
    rcx = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15aec); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15b08(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x8; const P2: u64 = 0x7001e6b3b8; const P3: u64 = 0x7001e6b3a8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[5], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    rax = ((P3) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15b38(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x8; const P2: u64 = 0x7001e6642c;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[6], rsp); let _eh = eh.push();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (rdx) as u64;
    rbx = (rcx) as u64;
    rcx = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15b5c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15b6c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[7], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0xa8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15b8f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15b9c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e696a0; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = (rbx) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15bec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[8], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15c03); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[9], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15c22); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e696a0; const P2: u64 = 0xc;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[10], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (r8) as u64); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (r9) as u64); crate::simd_ops::lk_and_u64(rdx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdx.wrapping_add(P0), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64); crate::simd_ops::wu64(rcx, (r8) as u64); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P0), (r9) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15c91); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15cae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15cc4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e696a0; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[11], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdx) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rdi, (rcx) as u64); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P0), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15d2b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15d48); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432; const P2: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[12], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = (rcx) as u64;
    rcx = ((rcx).wrapping_add(0xa8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15d8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0xc0u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15da0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15db4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e696a0; const P2: u64 = 0xc; const P3: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = (rcx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x10u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rdi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0xb0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432; const P2: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[13], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = (rcx) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15e82); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15e94); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6b378; const P1: u64 = 0x7001e66432; const P2: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[14], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = (rcx) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e15ecd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15edf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e15ef4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693c8; const P1: u64 = 0x7001e970c8; const P2: u64 = 0x7001e970d0; const P3: u64 = 0x7001e970c0; const P4: u64 = 0x7001e15c30; const P5: u64 = 0x7001e696a0; const P6: u64 = 0x7001e16254; const P7: u64 = 0x7001e6b3f8; const P8: u64 = 0x7001e66450; const P9: u64 = 0x7001e66456;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[15], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r12 = (rdx) as u64; r15 = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rdi = (((crate::simd_ops::ru32(r8.wrapping_add(rbx).wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0xb0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15f3d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rbx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); crate::simd_ops::lk_add_u32(P1, ((rbx as u32)) as u32); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15f8b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((((rbx as u32)) & (0xfffffffeu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15fc5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e15fe6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            5 => { rcx = ((rax.wrapping_add(0xb0u64)) as u64) as u64; r9 = (rbx) as u64; r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(r12.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16003); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e16003, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e160c1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } 0x7001e160d1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } 0x7001e160d3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 8; continue; } 0x7001e160d5 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 9; continue; } 0x7001e160d7 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x7001e15ef4) } }; rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(r14, (xmm0) as u128); rbx = (0x1u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rbx as u32)) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0xa8u64), (rsi) as u64); r13 = ((r14.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r13) as u64); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(r13, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32 as u64; rax = ((((rax as u32)) | ((rbx as u32)))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x4u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r13.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x10u64), (rsi) as u64); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(r13.wrapping_add(0x18u64), ((rax as u32)) as u32); rsi = ((rbx.wrapping_add(0xeu64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rsi as u32)) as u32); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { (rsi as u32) } else { (rdi as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rdi as u32)) as u32); r8 = ((rdi.wrapping_mul(8)) as u64) as u64; rdx = ((r12.wrapping_add(0x20u64)) as u64) as u64; rcx = ((r13.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1608f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e1608f, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e160c1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } 0x7001e160d1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } 0x7001e160d3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 8; continue; } 0x7001e160d5 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 9; continue; } 0x7001e160d7 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x7001e15ef4) } }; rax = c_rax; xmm0 = c_x0; rsi = (((rsi as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = ((rdi.wrapping_mul(8).wrapping_add(0x20u64)) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e160aa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e160aa, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e160c1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } 0x7001e160d1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } 0x7001e160d3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 8; continue; } 0x7001e160d5 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 9; continue; } 0x7001e160d7 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x7001e15ef4) } }; rax = c_rax; xmm0 = c_x0; rax = ((r14.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(r15, (r13) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (r14) as u64); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { bb = 6; continue; }
            8 => { bb = 6; continue; }
            9 => { bb = 6; continue; }
            10 => { bb = 6; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e160dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e970c8; const P1: u64 = 0x7001e970d0; const P2: u64 = 0x7001e970c0; const P3: u64 = 0x7001e15c30; const P4: u64 = 0x7001e696a0; const P5: u64 = 0x7001e6b3f8; const P6: u64 = 0x7001e66450; const P7: u64 = 0x7001e66456;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[16], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = (r8) as u64; rbp = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_sub(0x38u64), ((rdi as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x30u64), (xmm0) as u128); rbx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_sub(0x38u64), ((rbx as u32)) as u32); crate::simd_ops::lk_add_u32(P0, ((rbx as u32)) as u32); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1613a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((((rbx as u32)) & (0xfffffffeu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16174); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16195); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            5 => { rbx = (0x1u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((rbx as u32)) as u32); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(r8, (rax) as u64); crate::simd_ops::wu64(r8.wrapping_add(0xa8u64), (rdi) as u64); r14 = ((r8.wrapping_add(0x10u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = ((((rax as u32)) | ((rbx as u32)))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rdi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rdi) as u64); rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x18u64), ((rsi as u32)) as u32); rdi = ((rbx.wrapping_add(0xeu64)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdi as u32) } else { (rsi as u32) }) as u32 as u64; r8 = ((rsi.wrapping_mul(8)) as u64) as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; rcx = ((r14.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e161f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (((rdi as u32)).wrapping_sub((rsi as u32))) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = ((r14.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rcx.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1620c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp, (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (r15) as u64); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16234(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rdx) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16238(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rdx) as u64; rdx = (r8) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((rax) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16244(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (r8) as u64; r10 = (rdx) as u64; rdx = (rax) as u64; r8 = ((r9 as u32)) as u32 as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((r10) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16254(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x7001e6643e; const P3: u64 = 0x7001e16244; const P4: u64 = 0x7001e16238; const P5: u64 = 0x14; const P6: u64 = 0x7001e66450; const P7: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbp = (((crate::simd_ops::ru32(r8.wrapping_add(P0))) as i32 as i64) as u64) as u64; rdx = ((r8.wrapping_add(P1)) as u64) as u64; rcx = (rsi) as u64; rbp = ((rbp).wrapping_add(r9)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; r8 = (rax) as u64; rdx = (rbp) as u64; rcx = (rdi) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            5 => { r8 = (((crate::simd_ops::ru32(r8.wrapping_add(P5))) as i32 as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[17], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e16304); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0xffffffffe66cfae0; const P4: u64 = 0x40; const P5: u64 = 0x7001e694c0; const P6: u64 = 0x7001e16348; const P7: u64 = 0x38; const P8: u64 = 0x48; const P9: u64 = 0xc; const P10: u64 = 0x4; const P11: u64 = 0x7001e16234; const P12: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[18], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1994000u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16342); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            6 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; rax = (((crate::simd_ops::ru32(rdx.wrapping_add(P9))) as i32 as i64) as u64) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(P10))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (((crate::simd_ops::ru32(rdx.wrapping_add(P10))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1636f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1638c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e163a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e163a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0xfffffffffffffff8; const P7: u64 = 0x30; const P8: u64 = 0x7001e69188; const P9: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[19], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (r8) as u64; rdi = (rcx) as u64; crate::simd_ops::wu32(rcx, (0xe06d7363u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P2), (0x0u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P4), (0x19930520u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P5), (rdx) as u64); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P7), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16405); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            5 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P9), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), (0x1994000u64) as u64); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e163a4; const P1: u64 = 0x7001e15ef4;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0xc0u64)) as u64;
    rbx = (rcx) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0xc0u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e164c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    fl_l = (crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e164d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66438; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0xffffffffe66cfae0; const P4: u64 = 0x7001e15ef4; const P5: u64 = 0x7001e693c8; const P6: u64 = 0x7001e160dc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[20], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e164dd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe0434f4du32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe0434352u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1994000u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16524); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            8 => { rcx = (0xb0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e16530); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e1653f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e15b08; const P2: u64 = 0x7001e76b90; const P3: u64 = 0x7001e66444; const P4: u64 = 0x7001e165a7; const P5: u64 = 0x7001e16254; const P6: u64 = 0x7001e69008; const P7: u64 = 0x7001e694c0; const P8: u64 = 0x7001e166e3;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; rbp = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (rbx) as u64); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rbp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (rax) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P2) as u64) as u64; rcx = ((rbp) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; rdx = (0x80u32) as u32 as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx).wrapping_sub(0x10u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdx), (xmm1) as u128); rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(0x10u64), (rax) as u64); r10 = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rax = ((rcx.wrapping_sub(0x19930520u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1994000u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = (((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(r9))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 10; continue; }
            10 => { r8 = (((crate::simd_ops::ru32(rax.wrapping_add(r9).wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = (((crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(0xfu64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (0xffffffffffffff0u64) as u64; bb = 12; continue; }
            12 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = (rcx) as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; rbx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (rbx) as u64); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; r9 = ((rbp.wrapping_add(0x40u64)) as u64) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; fl_l = (((rcx) ^ (rbp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; rsp = ((rbp.wrapping_add(0xd0u64)) as u64) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e166f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, (r8) as u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e16718; const P1: u64 = 0x7001e16719; const P2: u64 = 0x7001e1671a; const P3: u64 = 0x7001e1671b; const P4: u64 = 0x7001e1671c; const P5: u64 = 0x7001e1671d; const P6: u64 = 0x7001e1671e; const P7: u64 = 0x7001e1671f; const P8: u64 = 0x8; const P9: u64 = 0x7001e6b388;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P0, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            9 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rax = ((P9) as u64) as u64; rax = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(P8)) } else { rax }) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b388;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    rax = ((P1) as u64) as u64;
    rax = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(P0)) } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e695c8; const P2: u64 = 0x7001e69020; const P3: u64 = 0x7001e69018; const P4: u64 = 0x7001e693c8; const P5: u64 = 0x7001e693a8; const P6: u64 = 0x7001e69010;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbp = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rbp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = ((((r9 as u32)) as i32 as i64) as u64) as u64; r14 = (r8) as u64; crate::simd_ops::wu32(rbp, ((rdx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (rcx) as u64); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rbx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; bb = 3; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 65; continue; } bb = 3; continue; }
            3 => { rsi = (((crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as i32 as i64) as u64) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (rsi) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; bb = 6; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 65; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 64; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; rdx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 65; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = ((rbp.wrapping_add(0x16u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x16u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(r14)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax)) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 64; continue; } bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 19; continue; }
            19 => { rax = (0x3u32) as u32 as u64; bb = 66; continue; }
            20 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 22; continue; }
            22 => { rax = ((rbp.wrapping_add(0x16u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x16u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(r13)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax)) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(0x1u64))) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 64; continue; } bb = 26; continue; }
            26 => { rax = ((rax).wrapping_add(0x2u64)) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 27; continue; }
            27 => { rax = (0x1u32) as u32 as u64; bb = 66; continue; }
            28 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); r8 = (r14) as u64; rdx = (0x9u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r12 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 65; continue; } bb = 29; continue; }
            29 => { rcx = (r12) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = ((rcx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rcx) & (rdx))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 30; continue; }
            30 => { rdx = (0xffffffffffffff0u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x400u64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 35; continue; } bb = 31; continue; }
            31 => { rax = ((rcx.wrapping_add(0xfu64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (rdx) as u64; bb = 33; continue; }
            33 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rdi = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu32(rdi, (0xccccu32) as u32); bb = 37; continue; }
            35 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu32(rax, (0xddddu32) as u32); bb = 37; continue; }
            37 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 38; continue; }
            38 => { rax = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); r8 = (r14) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 40; continue; }
            40 => { rax = (rdi) as u64; bb = 41; continue; }
            41 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rsi = ((rdi as u32)) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 43; continue; }
            43 => { rcx = ((rax.wrapping_sub(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = (0xddddu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 45; continue; }
            45 => { rax = ((rsi as u32)) as u32 as u64; bb = 66; continue; }
            46 => { crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); r9 = ((rsi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; r8 = (r13) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = (0x9u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 47; continue; }
            47 => { rcx = (r14) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rcx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rcx) & (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 48; continue; }
            48 => { fl_l = (rcx) as u64 as u64; fl_r = (0x400u64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 53; continue; } bb = 49; continue; }
            49 => { rax = ((rcx.wrapping_add(0xfu64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (0xffffffffffffff0u64) as u64; bb = 51; continue; }
            51 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rbx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu32(rbx, (0xccccu32) as u32); bb = 55; continue; }
            53 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu32(rax, (0xddddu32) as u32); bb = 55; continue; }
            55 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 56; continue; }
            56 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; r9 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); r8 = (r13) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 60; continue; } bb = 58; continue; }
            58 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = ((r12 as u32)) as u32 as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); r8 = (rdi) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = (crate::simd_ops::ru32(rbp)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = ((rax as u32)) as u32 as u64; bb = 61; continue; }
            59 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 60; continue; }
            60 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (rdi) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 61; continue; }
            61 => { rcx = ((rbx.wrapping_sub(0x10u64)) as u64) as u64; rax = (rdi) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = (0xddddu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 62; continue; }
            62 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rdi) as u64; bb = 43; continue; }
            63 => { rax = (rdi) as u64; bb = 42; continue; }
            64 => { rax = (0x2u32) as u32 as u64; bb = 66; continue; }
            65 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 66; continue; }
            66 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (((rcx) ^ (rbp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rsp = ((rbp.wrapping_add(0x30u64)) as u64) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x7001e695d0; const P5: u64 = 0x7001e69010;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rsi = (r8) as u64; rbp = ((rdx as u32)) as u32 as u64; r14 = (rcx) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rbx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as i32 as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rax) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); r8 = (rsi) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = (0x2u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x3u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e695c8; const P2: u64 = 0x7001e69018; const P3: u64 = 0x7001e693c8; const P4: u64 = 0x7001e69038; const P5: u64 = 0x7001e69030; const P6: u64 = 0x7001e693a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (r13) as u64); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rbp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (rax) as u64); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rsi = (r8) as u64; r15 = ((rdx as u32)) as u32 as u64; r13 = (rcx) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (rdi) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdi = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = 0; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0u32.wrapping_sub(fl_r as u32)) as u32); r9 = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; r8 = (rsi) as u64; rdx = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); rdx = ((((rdx as u32)) & (0x8u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 4; continue; }
            4 => { rax = (r14) as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = ((rax.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rax) & (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 5; continue; }
            5 => { rdx = (0xffffffffffffff0u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x400u64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0xfu64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rdx) as u64; bb = 8; continue; }
            8 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = (rcx) as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; rbx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx, (0xccccu32) as u32); bb = 12; continue; }
            10 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rax, (0xddddu32) as u32); bb = 12; continue; }
            12 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; r9 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); r8 = (rsi) as u64; rdx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); r8 = (rbx) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rcx = (r13) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 16; continue; }
            16 => { rdx = (0x400u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 40; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); r8 = (rbx) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = (r13) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 41; continue; } bb = 20; continue; }
            20 => { bb = 40; continue; }
            21 => { rcx = (rdi) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rcx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rcx) & (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 22; continue; }
            22 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = ((rcx.wrapping_add(0xfu64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (0xffffffffffffff0u64) as u64; bb = 25; continue; }
            25 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rsi = ((rsp.wrapping_add(0x50u64)) as u64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rsi, (0xccccu32) as u32); bb = 29; continue; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rax, (0xddddu32) as u32); bb = 29; continue; }
            29 => { fl_l = ((rsi).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = ((r14 as u32)) as u32 as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); r8 = (rbx) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); rcx = (r13) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; r9 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); r8 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); bb = 35; continue; }
            34 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); bb = 35; continue; }
            35 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((rax as u32)) as u32 as u64; bb = 36; continue; }
            36 => { rcx = ((rsi.wrapping_sub(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = (0xddddu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 37; continue; }
            37 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 41; continue; }
            38 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 39; continue; }
            39 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { rcx = ((rbx.wrapping_sub(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = (0xddddu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            43 => { rax = ((rdi as u32)) as u32 as u64; bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (((rcx) ^ (rbp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rsp = ((rbp.wrapping_add(0x10u64)) as u64) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e695d0; const P1: u64 = 0x1; const P2: u64 = 0x7001e69038;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdi = (r8) as u64; rsi = ((rdx as u32)) as u32 as u64; rbp = (rcx) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (rbx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); r8 = (rdi) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rcx = (rbp) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e16f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693d8; const P1: u64 = 0x7001e693b0; const P2: u64 = 0x8; const P3: u64 = 0x7001e69400; const P4: u64 = 0x7c; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x30; const P8: u64 = 0x40; const P9: u64 = 0x50; const P10: u64 = 0x60; const P11: u64 = 0x70; const P12: u64 = 0x7001e693f0; const P13: u64 = 0x7001e695b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x2u32) as u32 as u64; rcx = (0x100u32) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = (0x4u32) as u32 as u64; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P8))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P9))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P9), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P10))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P10), (xmm0) as u128); rcx = ((rcx).wrapping_add(r8)) as u64; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P11))) as u128; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x1u32) as u32); bb = 5; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x10u64), (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rcx) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rax) as u64); bb = 7; continue; }
            7 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693f0; const P1: u64 = 0x7001e693d8; const P2: u64 = 0x7001e695f0; const P3: u64 = 0x7001e69400; const P4: u64 = 0x7001e16b70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdi = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = ((rbx.wrapping_sub(0x41u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x19u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (((rbx as u32)).wrapping_add(0x20u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; bb = 21; continue; }
            7 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x100u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 10; continue; }
            10 => { bb = 16; continue; }
            11 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rbx.wrapping_mul(2)))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rbx as u32)) as u32 as u64; rcx = ((((rcx) as i64).wrapping_shr(8u32) as u64)) as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rdx = (((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)))) as i16 as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(15u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((((rdi as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rdi as u8)) as u8); r9 = (0x2u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x69u64), ((rbx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x6au64), (0x0u8) as u8); bb = 18; continue; }
            15 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = (rbx) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 16; continue; }
            16 => { rdi = ((rbx as u32)) as u32 as u64; rdi = (((((rdi as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (((rdi as u8)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; fl_l = ((((rdx as u32)) & (0x8000u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            17 => { crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rbx as u8)) as u8); r9 = (0x1u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x69u64), (0x0u8) as u8); bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); r8 = ((rsp.wrapping_add(0x68u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3u32) as u32); rdx = (0x100u32) as u32 as u64; rcx = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x71u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e171a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693f0; const P1: u64 = 0x7001e693d8; const P2: u64 = 0x7001e69610; const P3: u64 = 0x7001e69400; const P4: u64 = 0x7001e16b70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdi = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = ((rbx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x19u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; bb = 21; continue; }
            7 => { rbp = (0x2u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x100u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 10; continue; }
            10 => { bb = 16; continue; }
            11 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rbx.wrapping_mul(2)))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rbx as u32)) as u32 as u64; rcx = ((((rcx) as i64).wrapping_shr(8u32) as u64)) as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rdx = (((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)))) as i16 as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(15u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((((rdi as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x69u64), ((rbx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x6au64), (0x0u8) as u8); bb = 18; continue; }
            15 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = (rbx) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 16; continue; }
            16 => { rdi = ((rbx as u32)) as u32 as u64; rdi = (((((rdi as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (((rdi as u8)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; fl_l = ((((rdx as u32)) & (0x8000u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            17 => { crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rbx as u8)) as u8); rbp = (0x1u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x69u64), (0x0u8) as u8); bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); r8 = ((rsp.wrapping_add(0x68u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3u32) as u32); r9 = ((rbp as u32)) as u32 as u64; rdx = (0x200u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x71u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x27; const P1: u64 = 0x7001e659fc; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x7001e69480; const P4: u64 = 0x7001e17344; const P5: u64 = 0x7001e1a9d8; const P6: u64 = 0x7001e1734a;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x27u64)) as u64; rax = (((rax) & (0xffffffffffffffe0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), (rcx) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1734c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1b1c0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13, 32>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    // stackargs: A=32 max=5
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = (r9) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; r15 = (rcx) as u64; rbp = (r8) as u64; bb = 6; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = ((rsp.wrapping_add(0x90u64)) as u64) as u64; r9 = (rdi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); r8 = (rbx) as u64; rax = ((rsp.wrapping_add(0x42u64)) as u64) as u64; rdx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rcx = (r15) as u64; rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((rsp.wrapping_add(0x42u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi).wrapping_sub(1)) as u64 as u64; rsi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 7; continue; }
            7 => { bb = 10; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_sub(rbx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdi) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { rdi }) as u64; fl_l = ((rbx).wrapping_add(rdi)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_sub(rbp)) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rax = (0x7fffffffu32) as u32 as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17424(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1b480;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13, 32>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    // stackargs: A=32 max=5
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = (r9) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; r15 = (rcx) as u64; rbp = (r8) as u64; bb = 6; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = ((rsp.wrapping_add(0x90u64)) as u64) as u64; r9 = (rdi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); r8 = (rbx) as u64; rax = ((rsp.wrapping_add(0x44u64)) as u64) as u64; rdx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rcx = (r15) as u64; rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((rsp.wrapping_add(0x44u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi).wrapping_sub(1)) as u64 as u64; rsi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 7; continue; }
            7 => { bb = 10; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_sub(rbx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdi) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { rdi }) as u64; fl_l = ((rbx).wrapping_add(rdi)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_sub(rbp)) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rax = (0x7fffffffu32) as u32 as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e174fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x7001e66450; const P5: u64 = 0x1; const P6: u64 = 0x7001e17310; const P7: u64 = 0x7001e659fc; const P8: u64 = 0x7001e1abbc; const P9: u64 = 0x7001e175d9;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (0x7fffffffffffffffu64) as u64; rbx = (r8) as u64; rbp = (rdx) as u64; rsi = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(P2), (0xfu64) as u64); fl_l = (rbx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(P1), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(rsi), (0x0u8) as u8); bb = 12; continue; }
            3 => { rax = (rbx) as u64; rax = (((rax) | (0xfu64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (0x8000000000000000u64) as u64; bb = 6; continue; }
            5 => { rcx = (0x16u32) as u32 as u64; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdi }) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { r14 = (rax) as u64; bb = 11; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            10 => { r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { r8 = (rbx) as u64; crate::simd_ops::wu64(rsi, (r14) as u64); rdx = (rbp) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), (rbx) as u64); rcx = (r14) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(r14.wrapping_add(rbx), (0x0u8) as u8); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e175dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x1; const P3: u64 = 0x7001e17310; const P4: u64 = 0x7001e659fc; const P5: u64 = 0x7001e66450; const P6: u64 = 0x7001e1abbc; const P7: u64 = 0x7001e176a2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0x7fffffffffffffffu64) as u64; rsi = (r8) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; fl_l = (r8) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xfu64) as u64); fl_l = (r8) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(P1), (r8) as u64); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 10; continue; }
            3 => { rax = (rsi) as u64; rax = (((rax) | (0xfu64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (0x8000000000000000u64) as u64; bb = 6; continue; }
            5 => { rcx = (0x16u32) as u32 as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rbx }) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            7 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { r8 = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rdx = (rbp) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), (rsi) as u64); rcx = (rax) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e176a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e17310; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x7001e66450; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e177b7; const P6: u64 = 0x7001e1abbc; const P7: u64 = 0x7001e177fb;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (0x7fffffffffffffffu64) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (rbx) as u64; rax = ((rax).wrapping_sub(r14)) as u64; r13 = (r9) as u64; rdi = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbp = ((r14.wrapping_add(rdx)) as u64) as u64; rdx = (rbp) as u64; rdx = (((rdx) | (0xfu64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (0x8000000000000000u64) as u64; bb = 5; continue; }
            3 => { rcx = (r15) as u64; rax = (rbx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((rcx.wrapping_add(r15)) as u64) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rcx = ((rbx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rsi = (rax) as u64; bb = 10; continue; }
            7 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            9 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rbp) as u64); rbp = ((r14.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (rbx) as u64); r8 = (r14) as u64; rcx = (rsi) as u64; fl_l = (r15) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (r12) as u64; rdx = (r13) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((r15.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu8(r12.wrapping_add(rbp), (0x0u8) as u8); fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rax = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rbx = (rcx) as u64; bb = 14; continue; }
            14 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 17; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            16 => { rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (r12) as u64; rdx = (r13) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(r12.wrapping_add(rbp), (0x0u8) as u8); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdi, (rsi) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e177fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e17310; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x7001e66450; const P3: u64 = 0x7001e6649c; const P4: u64 = 0x7001e69480; const P5: u64 = 0x7001e178fe; const P6: u64 = 0x7001e1abbc; const P7: u64 = 0x7001e1793b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (0x7fffffffffffffffu64) as u64; rax = (rbx) as u64; r15 = (r15 & !0xffu64) | ((((r9 as u8)) as u8 as u64)); rax = ((rax).wrapping_sub(r14)) as u64; rsi = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r12 = ((r14.wrapping_add(rdx)) as u64) as u64; rdx = (r12) as u64; rdx = (((rdx) | (0xfu64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (0x8000000000000000u64) as u64; bb = 5; continue; }
            3 => { rcx = (rbp) as u64; rax = (rbx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((rcx.wrapping_add(rbp)) as u64) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rcx = ((rbx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rdi = (rax) as u64; bb = 10; continue; }
            7 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            9 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (r12) as u64); r8 = (r14) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rbx) as u64); rcx = (rdi) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rbp.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu8(r14.wrapping_add(rdi), ((r15 as u8)) as u8); crate::simd_ops::wu8(r14.wrapping_add(rdi).wrapping_add(0x1u64), (0x0u8) as u8); fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rax = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rbx = (rcx) as u64; bb = 14; continue; }
            14 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 17; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            16 => { rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(r14.wrapping_add(rdi), ((r15 as u8)) as u8); crate::simd_ops::wu8(r14.wrapping_add(rdi).wrapping_add(0x1u64), (0x0u8) as u8); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsi, (rdi) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e1793c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x7001e98cd0; const P2: u64 = 0x7001e98cb8; const P3: u64 = 0x7001e99198; const P4: u64 = 0x7001e49ec0; const P5: u64 = 0x7001e25680; const P6: u64 = 0x7001e19bd0; const P7: u64 = 0x7001e25648; const P8: u64 = 0x7001e696a0; const P9: u64 = 0x7001e1a9f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[21], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rcx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17951); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rsi) as u64); rdi = (crate::simd_ops::ru64(P2)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17976); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(P2)) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P3, ((rax as u32)) as u32); rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu64(P2, (rax) as u64); bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e179a0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(P2)) as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r14 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e179d5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; bb = 11; continue; }
            11 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = (rsi) as u64; bb = 16; continue; }
            14 => { rdx = (rbp) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e179ff); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17a17); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17a26); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P1, (rbx) as u64); bb = 16; continue; }
            16 => { rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17a38); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17a4b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17a54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e49e20; const P1: u64 = 0x7001e98cc8; const P2: u64 = 0x7001e991b0; const P3: u64 = 0x7001e99198; const P4: u64 = 0x7001e49ec0; const P5: u64 = 0x7001e25680; const P6: u64 = 0x7001e1a010; const P7: u64 = 0x7001e25648; const P8: u64 = 0x7001e696a0; const P9: u64 = 0x7001e1a9f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[22], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rcx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17a69); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rsi) as u64); rdi = (crate::simd_ops::ru64(P2)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17a8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(P2)) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P3, ((rax as u32)) as u32); rax = (rax as u32 as i32 as i64) as u64; crate::simd_ops::wu64(P2, (rax) as u64); bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17ab8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(P2)) as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r14 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17aed); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; bb = 11; continue; }
            11 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = (rsi) as u64; bb = 16; continue; }
            14 => { rdx = (rbp) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17b17); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17b2f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17b3e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(P1, (rbx) as u64); bb = 16; continue; }
            16 => { rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17b50); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17b63); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e18ef0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    rbx = (rcx) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64)) as u64; fl_r = fl_l;
    rax = (rcx) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e6bc80; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rdx) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), (rdx) as u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P11), ((rdx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6bc98; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x7001e6bc80; const P10: u64 = 0x10; const P11: u64 = 0x58; const P12: u64 = 0x7001e6bc90; const P13: u64 = 0x7001e1c510;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[23], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (rdx) as u64; rbx = (rcx) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P1), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P2), (rcx) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P4), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P5), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P6), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P7), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P8), (rcx) as u64); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P10), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P11), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x68u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_sub(0x10u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx).wrapping_sub(0x4u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17c7f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6bca8; const P1: u64 = 0x7001e659fc; const P2: u64 = 0x7001e25690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[24], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rsi = ((rcx.wrapping_add(0x8u64)) as u64) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, (rax) as u64); r14 = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rax) as u64); r15 = ((rcx.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(r15, (rax) as u64); rbp = ((rcx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rbp, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), (rax) as u64); r12 = ((rcx.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu32(r12, ((rax as u32)) as u32); r13 = ((rcx.wrapping_add(0x4cu64)) as u64) as u64; crate::simd_ops::wu32(r13, ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (rax) as u64); rcx = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17d03); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e17d14); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rdi = (rax) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (r15) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rbp) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (r13) as u64); crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(rbp, (rax) as u64); crate::simd_ops::wu32(r13, ((rax as u32)) as u32); crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(r15, (rax) as u64); crate::simd_ops::wu32(r12, ((rax as u32)) as u32); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6ba18; const P2: u64 = 0x7001e6b880; const P3: u64 = 0x7001e184f0; const P4: u64 = 0x7001e4d900; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x30; const P8: u64 = 0x28; const P9: u64 = 0x38; const P10: u64 = 0x7001e18dc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[25], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0xc0u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rdx = ((P2) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17e19); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17e23); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm1) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64) as u128;
    crate::simd_ops::wu64(rbx.wrapping_add(P7), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rax as u32)) as u32);
    rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17e4c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0xc0u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e6b910; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x10ffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b910; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x7001e6b880; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x7001e18dc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[26], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x90u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x10ffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32);
    rdx = ((P4) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17ed7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17ee1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x90u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001e6b968; const P2: u64 = 0x8; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(P0), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l;
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b968; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x7001e6b880; const P5: u64 = 0x7001e184f0; const P6: u64 = 0x7001e18dc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[27], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x90u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::lk_or_u32(rcx.wrapping_add(P2), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x4u32) as u32);
    rdx = ((P4) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17f74); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e17f7e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x90u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e17fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e6b9c0; const P2: u64 = 0x7001e6b880; const P3: u64 = 0x7001e184f0; const P4: u64 = 0x7001e4d900; const P5: u64 = 0x10; const P6: u64 = 0x20; const P7: u64 = 0x30; const P8: u64 = 0x28; const P9: u64 = 0x38; const P10: u64 = 0x7001e18dc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[28], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0xc0u64)) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rdx = ((P2) as u64) as u64;
    rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e18009); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e18013); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm1) as u128);
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64) as u128;
    crate::simd_ops::wu64(rbx.wrapping_add(P7), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rax as u32)) as u32);
    rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e1803c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0xc0u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}
