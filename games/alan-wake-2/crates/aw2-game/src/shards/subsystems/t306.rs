#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141bde770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430a9d08; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be0580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432220e0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be0b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1406dc930; const P2: u64 = 0x10; const P3: u64 = 0x140206290; const P4: u64 = 0x1405ef6b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rbx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rbx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rcx = ((rbx.wrapping_add(P0)) as u64) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be0be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143231768; const P1: u64 = 0x8; const P2: u64 = 0x1406d6d80;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be1d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1404b9990;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be2270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r11 = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r10 = rcx;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(r8), rax);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be26b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be2930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143246140; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be33b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x1406dcd10; const P3: u64 = 0x1406e24a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be42e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x28; const P5: u64 = 0x40; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x34; const P9: u64 = 0x38; const P10: u64 = 0x39; const P11: u64 = 0x140265ba0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P10), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be4380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143220730; const P1: u64 = 0x10; const P2: u64 = 0x141be4390;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rdx, (rax) as u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be4390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x24; const P3: u64 = 0x30; const P4: u64 = 0x40; const P5: u64 = 0x48;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be6460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x143244390; const P2: u64 = 0x1432443b0; const P3: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = 0x3;
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P2;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be66f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x1406dcd10; const P3: u64 = 0x1406e24a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be6a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P1), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be70b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1406f2520; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x14072b600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rdi = ((rdi).wrapping_add(0x30u64)) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be7160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324abc8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x1u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be7870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1406f2520; const P2: u64 = 0x18; const P3: u64 = 0x14072b8a0; const P4: u64 = 0x28; const P5: u64 = 0x140724430;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be7910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324a988; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be7bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140701b70; const P2: u64 = 0x140706530;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be7fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140701b70; const P2: u64 = 0x140706530;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be8550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140701b70; const P3: u64 = 0x140706530;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be8a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140701b70; const P3: u64 = 0x140706530;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be9510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1406f2520; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141be95a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324a8c0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), (0x7fffffffu64) as u64);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bea470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140763e00; const P2: u64 = 0x18; const P3: u64 = 0x1407640a0; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x14240aa00; const P7: u64 = 0x78; const P8: u64 = 0x140636a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bea520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324d790; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x88; const P6: u64 = 0x30; const P7: u64 = 0x33; const P8: u64 = 0x34; const P9: u64 = 0x38; const P10: u64 = 0x80; const P11: u64 = 0x90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P6), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P6), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x84u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bec080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x19; const P7: u64 = 0x14240aa00; const P8: u64 = 0x48; const P9: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bec860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x14240aa00; const P4: u64 = 0x58; const P5: u64 = 0x140636a20; const P6: u64 = 0x88; const P7: u64 = 0x8c; const P8: u64 = 0x90; const P9: u64 = 0x94; const P10: u64 = 0x98;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = rbp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x90u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x94u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P10))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x98u64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bec930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324c4a8; const P1: u64 = 0x68; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x13; const P5: u64 = 0x14; const P6: u64 = 0x18; const P7: u64 = 0x60; const P8: u64 = 0x70; const P9: u64 = 0x78; const P10: u64 = 0x80; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x60u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x64u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141becad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143250348; const P2: u64 = 0x14075f9e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r10 = rdx;
    rdx = 0x1248;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8), ((rax as u32)) as u32); rax = 0x14d8; crate::simd_ops::wu64(rax.wrapping_add(r8), r9); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    r8 = 0x1;
    r9 = P1;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141becb50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x140728030;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r9 = rdx;
    rdx = 0x1274;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8), ((rax as u32)) as u32); rax = 0x12c0; crate::simd_ops::wu64(rax.wrapping_add(r8), (0x0u64) as u64); }
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = r9;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141becc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143250348; const P1: u64 = 0x14075f9e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r10 = r8;
    r9 = 0x1248;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(rcx))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32); rax = 0x14d8; crate::simd_ops::wu64(rax.wrapping_add(rcx), r8); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8);
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    r8 = 0x1;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141becd50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140728030;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r9 = 0x1274;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(rcx))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32); rax = 0x12c0; crate::simd_ops::wu64(rax.wrapping_add(rcx), (0x0u64) as u64); }
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bed190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x14240aa00; const P4: u64 = 0x60; const P5: u64 = 0x140636a20; const P6: u64 = 0x90; const P7: u64 = 0xd8; const P8: u64 = 0x108; const P9: u64 = 0x150;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rbp = r8; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = rbp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0xd8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x108); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0x150); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bee0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x14240aa00; const P11: u64 = 0x48; const P12: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P11)) as u64) as u64;
    rcx = rdi.wrapping_add(0x48);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bee1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324eac8; const P1: u64 = 0x88; const P2: u64 = 0xc; const P3: u64 = 0x14; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x33; const P7: u64 = 0x34; const P8: u64 = 0x38; const P9: u64 = 0x80; const P10: u64 = 0x90; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x84u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bee900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x14240aa00; const P7: u64 = 0x48; const P8: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bef210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x14240aa00; const P8: u64 = 0x68; const P9: u64 = 0x140636a20; const P10: u64 = 0x98;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bef2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324bfa8; const P1: u64 = 0x78; const P2: u64 = 0xc; const P3: u64 = 0x20; const P4: u64 = 0x23; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x70; const P8: u64 = 0x80; const P9: u64 = 0x88; const P10: u64 = 0x90; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x74u64), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141befac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), (rax) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141befae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(0x41u8).wrapping_sub((if false { 1u8 } else { 0u8 }))) as u8 as u64));
    crate::simd_ops::wu8(rax.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141befb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324c568; const P1: u64 = 0xc; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf0290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140763e00; const P2: u64 = 0x18; const P3: u64 = 0x1407640a0; const P4: u64 = 0x28; const P5: u64 = 0x2c; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x14240aa00; const P10: u64 = 0x88; const P11: u64 = 0x140636a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); rdx = ((rbx.wrapping_add(P8)) as u64) as u64; rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P10)) as u64) as u64; rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf0360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324bee0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x98; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x43; const P10: u64 = 0x44; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P8), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P8), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P9), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x90u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x94u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf14d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf2c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14240aa00; const P5: u64 = 0x48; const P6: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf2cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324e6d0; const P1: u64 = 0x70; const P2: u64 = 0xc; const P3: u64 = 0x18; const P4: u64 = 0x1b; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x68; const P8: u64 = 0x78; const P9: u64 = 0x80; const P10: u64 = 0x88; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x6cu64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf3430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x14240aa00; const P8: u64 = 0x48; const P9: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf3c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x14240aa00; const P8: u64 = 0x48; const P9: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf43f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140763e00; const P2: u64 = 0x18; const P3: u64 = 0x1407640a0; const P4: u64 = 0x28; const P5: u64 = 0x2c; const P6: u64 = 0x30; const P7: u64 = 0x14240aa00; const P8: u64 = 0x78; const P9: u64 = 0x140636a20; const P10: u64 = 0xa8; const P11: u64 = 0xf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rbp.wrapping_add(0x18); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P5), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x30); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x78); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xa8); rcx = ((rsi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xf0); rcx = ((rsi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x120); rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x168); rcx = rsi.wrapping_add(0x168); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x198u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x199u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x199u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x19cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x19cu64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf4c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x14240aa00; const P11: u64 = 0x78; const P12: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; rdx = ((rdx).wrapping_add(0x30u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0xa8); rcx = rbp.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0xf0); rcx = rbp.wrapping_add(0xf0); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x120u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x120u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x124u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x124u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf4f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140748c80; const P2: u64 = 0x30; const P3: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rdi).wrapping_add(P2), rax);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf5330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140748c80; const P2: u64 = 0x30; const P3: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rcx = rdi;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdi).wrapping_add(P2), rax);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf5540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x30; const P10: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P9), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf55d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324d858; const P1: u64 = 0xc; const P2: u64 = 0x20; const P3: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf5ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x50; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38; const P12: u64 = 0x14240aa00; const P13: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P10), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x98); rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0xc8u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0xc8u64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf5de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324ea08; const P1: u64 = 0xa8; const P2: u64 = 0xc; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x40; const P6: u64 = 0x50; const P7: u64 = 0x53; const P8: u64 = 0x54; const P9: u64 = 0x58; const P10: u64 = 0xa0; const P11: u64 = 0xb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P6), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P6), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0xa0u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0xa4u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf6560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140763e00; const P2: u64 = 0x18; const P3: u64 = 0x1407640a0; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0x70; const P7: u64 = 0x140636a20; const P8: u64 = 0xa0; const P9: u64 = 0xa4; const P10: u64 = 0xa8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rbp.wrapping_add(0x18); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = rbp.wrapping_add(0x28); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x70); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa4u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf6640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324be18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x80; const P6: u64 = 0x28; const P7: u64 = 0x2b; const P8: u64 = 0x2c; const P9: u64 = 0x30; const P10: u64 = 0x78; const P11: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P6), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P6), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x7cu64), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0xa0u64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf6ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x14240aa00; const P9: u64 = 0x48; const P10: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf6f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324b6d0; const P1: u64 = 0x80; const P2: u64 = 0xc; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x2b; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x78; const P9: u64 = 0x88; const P10: u64 = 0x90; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P4), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P4), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x7cu64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf76d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140763e00; const P2: u64 = 0x18; const P3: u64 = 0x1407640a0; const P4: u64 = 0x28; const P5: u64 = 0x2c; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x14240aa00; const P9: u64 = 0x80; const P10: u64 = 0x140636a20; const P11: u64 = 0xb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rbp.wrapping_add(0x18); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x38); rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x80); rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xb0); rcx = ((rsi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xf8); rcx = rsi.wrapping_add(0xf8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf8000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x21; const P9: u64 = 0x22; const P10: u64 = 0x14240aa00; const P11: u64 = 0x70; const P12: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P7), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P8), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P9))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P9), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0xa0); rcx = rbp.wrapping_add(0xa0); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0xe8); rcx = rbp.wrapping_add(0xe8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x118); rcx = rbp.wrapping_add(0x118); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x160); rcx = rbp.wrapping_add(0x160); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x190); rcx = rbp.wrapping_add(0x190); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1d8); rcx = rbp.wrapping_add(0x1d8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf89d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14324cb68; const P1: u64 = 0xc; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf90d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x14240aa00; const P5: u64 = 0x68; const P6: u64 = 0x140636a20; const P7: u64 = 0x98; const P8: u64 = 0xe0; const P9: u64 = 0x110; const P10: u64 = 0x158; const P11: u64 = 0x188;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rdx = rbp.wrapping_add(0x20); rcx = ((rsi.wrapping_add(P3)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x68); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x98); rcx = ((rsi.wrapping_add(P7)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0xe0); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x110); rcx = ((rsi.wrapping_add(P9)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x158); rcx = ((rsi.wrapping_add(P10)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x188u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P11), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x18cu64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x18cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x194u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x194u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x198u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x198u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1a0u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x1a0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a4u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1a4u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bf99b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0x70; const P7: u64 = 0x140636a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bfa1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x25; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x14240aa00; const P9: u64 = 0x78; const P10: u64 = 0x140636a20; const P11: u64 = 0xa8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P4), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x25u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P5), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x30); rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x78); rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xa8); rcx = ((rsi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xf0); rcx = rsi.wrapping_add(0xf0); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bfc0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x14240aa00; const P10: u64 = 0x80; const P11: u64 = 0x140636a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P4), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P5), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P7), ((rax as u8)) as u8); rdx = rbp.wrapping_add(0x38); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x80); rcx = ((rsi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xb0u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bfc1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432568d0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x90; const P5: u64 = 0x20; const P6: u64 = 0x2c; const P7: u64 = 0x38; const P8: u64 = 0x3b; const P9: u64 = 0x3c; const P10: u64 = 0x40; const P11: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x88u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0xb0u64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bfd170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bff470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143251fd0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x27; const P7: u64 = 0x28; const P8: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000005u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x5u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bff600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406ec9b0; const P1: u64 = 0x14384e410; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x143251450; const P4: u64 = 0x1401de410;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    r11 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x28);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bff790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406ec9b0; const P1: u64 = 0x14384e410; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x143251450; const P4: u64 = 0x1401de410;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r11 = rdx;
    rbx = r8;
    rcx = r11;
    rdx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x28);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bffc30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bffcc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143254a50; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c003d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x21;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c00460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x143253b50; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P4), (0x101u16) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c00b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0x70; const P7: u64 = 0x140636a20; const P8: u64 = 0xa0; const P9: u64 = 0xe8; const P10: u64 = 0x118; const P11: u64 = 0x11c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x28); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x70); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xa0); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xe8); rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x118u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x11cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P11), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x120); rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x168); rcx = rsi.wrapping_add(0x168); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c024b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x14240aa00; const P5: u64 = 0x68; const P6: u64 = 0x140636a20; const P7: u64 = 0x98; const P8: u64 = 0x99;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = rbp.wrapping_add(0x20); rcx = ((rsi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x68); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x98u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P7), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x99u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P8), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c02cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14079b8c0; const P1: u64 = 0x128; const P2: u64 = 0x12c; const P3: u64 = 0x130; const P4: u64 = 0x134;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c03c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14079b2c0; const P1: u64 = 0x143253c10; const P2: u64 = 0x118;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rdx;
    r8 = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = P1;
    rax = r8;
    crate::simd_ops::wu64(r8, rcx);
    crate::simd_ops::wu32(r8.wrapping_add(P2), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c04320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rbx = (rbx & !0xff00u64) | (((0xfeu8) as u8 as u64) << 8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c056a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0x70; const P7: u64 = 0x140636a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c05740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432538d0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x80; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x2b; const P8: u64 = 0x2c; const P9: u64 = 0x30; const P10: u64 = 0x78; const P11: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P6), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P6), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x7cu64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c06c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406ec9b0; const P1: u64 = 0x14384e390; const P2: u64 = 0x143251388; const P3: u64 = 0x1; const P4: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    r11 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x20);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = ((rax.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c06ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406ec9b0; const P1: u64 = 0x14384e390; const P2: u64 = 0x143251388; const P3: u64 = 0x1; const P4: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r11 = rdx;
    rbx = r8;
    rcx = r11;
    rdx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x20);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = ((rax.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c07370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x25;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c08f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14079b390; const P1: u64 = 0x118; const P2: u64 = 0x14240aa00; const P3: u64 = 0x160; const P4: u64 = 0x140636a20; const P5: u64 = 0x190; const P6: u64 = 0x1d8; const P7: u64 = 0x208; const P8: u64 = 0x250; const P9: u64 = 0x280;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P1)) as u64) as u64; rcx = rbp.wrapping_add(0x118); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x160); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0x190); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0x1d8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x208); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x250); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x280u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0bd30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14079b390; const P1: u64 = 0x118; const P2: u64 = 0x11c; const P3: u64 = 0x120; const P4: u64 = 0x121;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0c570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0x70; const P7: u64 = 0x140636a20; const P8: u64 = 0xa0; const P9: u64 = 0xe8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x28); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x70); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xa0); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xe8); rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0ce60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143252ed0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x27; const P7: u64 = 0x28; const P8: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0cff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406ec9b0; const P1: u64 = 0x14384e410; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x143251798; const P4: u64 = 0x1401de410;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    r11 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x20);
    rax = P2;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0d180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406ec9b0; const P1: u64 = 0x14384e410; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x143251798; const P4: u64 = 0x1401de410;
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r11 = rdx;
    rbx = r8;
    rcx = r11;
    rdx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x20);
    rax = P2;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0d620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0x70; const P7: u64 = 0x140636a20; const P8: u64 = 0xa0; const P9: u64 = 0xe8; const P10: u64 = 0x118; const P11: u64 = 0x160;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbp = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((rax as u32)) as u32); rdx = rbp.wrapping_add(0x28); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x70); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xa0); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0xe8); rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x118); rcx = ((rsi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_add(0x160); rcx = ((rsi.wrapping_add(P11)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c0fe40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x3c; const P3: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c106e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x3c; const P3: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c10e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x44; const P4: u64 = 0x45; const P5: u64 = 0x48;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c10ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14325adf8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c11680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432594f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x90; const P8: u64 = 0x38; const P9: u64 = 0x3b; const P10: u64 = 0x3c; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P7)) as u64) as u64;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P8), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P8), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P9), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x88u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c11df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x14240aa00; const P3: u64 = 0x80; const P4: u64 = 0x140636a20; const P5: u64 = 0xb0; const P6: u64 = 0xb8; const P7: u64 = 0xb4; const P8: u64 = 0x100; const P9: u64 = 0x130;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P1)) as u64) as u64; rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rdx = ((rsi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xb0u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0xb8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x100); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x130u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c12610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x48; const P3: u64 = 0x50; const P4: u64 = 0x54; const P5: u64 = 0x60; const P6: u64 = 0x68; const P7: u64 = 0x6c; const P8: u64 = 0x70;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c126a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14325bdd8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x1432590f0; const P8: u64 = 0x40; const P9: u64 = 0x14325a740; const P10: u64 = 0x58; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = P7;
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = P9;
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x60u64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c12e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x14240aa00; const P3: u64 = 0x80; const P4: u64 = 0x140636a20; const P5: u64 = 0xb0; const P6: u64 = 0xf8; const P7: u64 = 0x128; const P8: u64 = 0x170; const P9: u64 = 0x1a0; const P10: u64 = 0x1a4;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P1)) as u64) as u64; rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0xb0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x128); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x170); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P9))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x1a0u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a4u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c14050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x3c; const P4: u64 = 0x14240aa00; const P5: u64 = 0x88; const P6: u64 = 0x140636a20; const P7: u64 = 0xb8; const P8: u64 = 0x100;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rbp = r8;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rdx = ((rsi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rcx = rbp.wrapping_add(0x40);
    rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbp.wrapping_add(0x3cu64), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rcx = rbp.wrapping_add(0x88);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P7)) as u64) as u64;
    rcx = rbp.wrapping_add(0xb8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P8)) as u64) as u64;
    rcx = rbp.wrapping_add(0x100);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c14310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rdi).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c14680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rcx = rdi;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c15010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x50; const P3: u64 = 0x3c; const P4: u64 = 0x40; const P5: u64 = 0x44; const P6: u64 = 0x48; const P7: u64 = 0x4c; const P8: u64 = 0x14240aa00; const P9: u64 = 0x98; const P10: u64 = 0x140636a20; const P11: u64 = 0xc8;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x50); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0xc8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x110); rcx = rbp.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x140); rcx = rbp.wrapping_add(0x140); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x188); rcx = rbp.wrapping_add(0x188); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1b8); rcx = rbp.wrapping_add(0x1b8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x200); rcx = rbp.wrapping_add(0x200); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x230u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x230u64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c15830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x14240aa00; const P3: u64 = 0x80; const P4: u64 = 0x140636a20; const P5: u64 = 0xb0; const P6: u64 = 0xf8; const P7: u64 = 0x128; const P8: u64 = 0x12c; const P9: u64 = 0x130; const P10: u64 = 0x134;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P1)) as u64) as u64; rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0xb0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x12cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x130u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x134u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c17620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x14240aa00; const P3: u64 = 0xc0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c176a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14325d918; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x3b; const P9: u64 = 0x3c; const P10: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c18290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rbx).wrapping_add(P2), rax);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rbx).wrapping_add(P3), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c185f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rcx = rbx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rbx).wrapping_add(P2), rax);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rbx).wrapping_add(P3), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c187f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c18f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x140265ba0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P1), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c19210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x39;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    r9 = ((rbx.wrapping_add(P2)) as u64) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(r8), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c19380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = 0xccccccfe; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c19570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x39;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rcx = rbx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    r9 = ((rbx.wrapping_add(P2)) as u64) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c19790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x39;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1a650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x48; const P3: u64 = 0x3c; const P4: u64 = 0x40; const P5: u64 = 0x44; const P6: u64 = 0x14240aa00; const P7: u64 = 0x90; const P8: u64 = 0x140636a20; const P9: u64 = 0xc0;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rbp = r8;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rdx = ((rsi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rcx = rbp.wrapping_add(0x48);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P7)) as u64) as u64;
    rcx = rbp.wrapping_add(0x90);
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0xc0u64), ((rax as u32)) as u32);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1a710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432591a0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0xa0; const P8: u64 = 0x38; const P9: u64 = 0x44; const P10: u64 = 0x48; const P11: u64 = 0x4b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P7)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P10), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P10), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P11), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0x4cu64), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x50u64), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x9cu64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1ae80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x50; const P3: u64 = 0x3c; const P4: u64 = 0x40; const P5: u64 = 0x44; const P6: u64 = 0x48; const P7: u64 = 0x14240aa00; const P8: u64 = 0x98; const P9: u64 = 0x140636a20; const P10: u64 = 0xc8; const P11: u64 = 0x110;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x50); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x44u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; rcx = rbp.wrapping_add(0xc8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1b650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x14240aa00; const P3: u64 = 0x80; const P4: u64 = 0x140636a20; const P5: u64 = 0xb0; const P6: u64 = 0xf8; const P7: u64 = 0x128; const P8: u64 = 0x170; const P9: u64 = 0x1a0; const P10: u64 = 0x1a4; const P11: u64 = 0x1a8;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P1)) as u64) as u64; rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0xb0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x128); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x170); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1acu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x1acu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1adu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x1adu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1b0u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1b0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1b4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1b4u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1b8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x1b8u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1b9u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x1b9u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1bau64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x1bau64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1b910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14325e4f0; const P1: u64 = 0x38; const P2: u64 = 0x14384e4e0; const P3: u64 = 0x1; const P4: u64 = 0x1401de410; const P5: u64 = 0x40; const P6: u64 = 0x14384e500; const P7: u64 = 0x1432510f0; const P8: u64 = 0x1426cdcb0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rcx;
    r9 = P0;
    r10 = rdx;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P5)) as u64) as u64;
    r8 = 0x2;
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P7;
    rax = P8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1bad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14384e4e0; const P1: u64 = 0x14325e4f0; const P2: u64 = 0x1; const P3: u64 = 0x1401de410; const P4: u64 = 0x40; const P5: u64 = 0x14384e500; const P6: u64 = 0x1432510f0; const P7: u64 = 0x1426cdcb0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    r10 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x38u64)) as u64;
    r8 = ((rcx.wrapping_add(P2)) as u32) as u32 as u64;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P4)) as u64) as u64;
    r8 = 0x2;
    rax = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P6;
    rax = P7;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1bf90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x48; const P3: u64 = 0x3c; const P4: u64 = 0x40; const P5: u64 = 0x14240aa00; const P6: u64 = 0x90; const P7: u64 = 0x140636a20; const P8: u64 = 0xc0; const P9: u64 = 0x108; const P10: u64 = 0x138; const P11: u64 = 0x180;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x48); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x3cu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0x108); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; rcx = rbp.wrapping_add(0x138); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0x180); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1b0); rcx = rbp.wrapping_add(0x1b0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1f8); rcx = rbp.wrapping_add(0x1f8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1d7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x3c; const P4: u64 = 0x14240aa00; const P5: u64 = 0x88; const P6: u64 = 0x140636a20; const P7: u64 = 0xb8; const P8: u64 = 0xc0; const P9: u64 = 0xbc; const P10: u64 = 0x108;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x40); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P7))) as u32) as u32 as u64; rdx = ((rsi.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0xb8u64), ((rax as u8)) as u8); rcx = rbp.wrapping_add(0xc0); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xbcu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; rcx = rbp.wrapping_add(0x108); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1daa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    r9 = ((rbx.wrapping_add(P2)) as u64) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1de10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1407b4640; const P2: u64 = 0x38; const P3: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rcx = rbx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    r9 = ((rbx.wrapping_add(P2)) as u64) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c1e7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x50; const P3: u64 = 0x40; const P4: u64 = 0x44; const P5: u64 = 0x48; const P6: u64 = 0x14240aa00; const P7: u64 = 0x98; const P8: u64 = 0x140636a20; const P9: u64 = 0xc8; const P10: u64 = 0x110; const P11: u64 = 0x140;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); rcx = rbp.wrapping_add(0x50); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0xc8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; rcx = rbp.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0x140); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x188); rcx = rbp.wrapping_add(0x188); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1b8); rcx = rbp.wrapping_add(0x1b8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x200); rcx = rbp.wrapping_add(0x200); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x230); rcx = rbp.wrapping_add(0x230); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x278); rcx = rbp.wrapping_add(0x278); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c21020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x14240aa00; const P4: u64 = 0x88; const P5: u64 = 0x140636a20; const P6: u64 = 0xb8; const P7: u64 = 0x100; const P8: u64 = 0x130; const P9: u64 = 0x134;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = r8; rbp = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x40); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = rbp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0xb8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x100); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x130u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P9))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x134u64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c21ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c22220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1407cabd0; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x14240aa00; const P4: u64 = 0x88; const P5: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P1), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c222a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143262b20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x98; const P8: u64 = 0x40; const P9: u64 = 0x43; const P10: u64 = 0x44; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P7)) as u64) as u64;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P8), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P8), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P9), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x98u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x90u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x94u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c22ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14240aa00; const P3: u64 = 0x58; const P4: u64 = 0x140636a20; const P5: u64 = 0x88; const P6: u64 = 0xa0; const P7: u64 = 0x8c; const P8: u64 = 0x90; const P9: u64 = 0x94; const P10: u64 = 0x98; const P11: u64 = 0x9c;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; rsi = rdx; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x10u64)) as u64; rbp = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rdx = ((rsi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0xa0); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x90u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x94u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x9cu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0xe8); rcx = rbp.wrapping_add(0xe8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x118u64))) as u32 as u64; rdx = rsi.wrapping_add(0x120); rcx = rbp.wrapping_add(0x120); crate::simd_ops::wu32(rbp.wrapping_add(0x118u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x168); rcx = rbp.wrapping_add(0x168); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x198); rcx = rbp.wrapping_add(0x198); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1e0); rcx = rbp.wrapping_add(0x1e0); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c232f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14240aa00; const P3: u64 = 0x58; const P4: u64 = 0x140636a20; const P5: u64 = 0x88; const P6: u64 = 0xd0; const P7: u64 = 0x100; const P8: u64 = 0x104; const P9: u64 = 0x108; const P10: u64 = 0x10c; const P11: u64 = 0x110;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; rsi = rdx; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x10u64)) as u64; rbp = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = rbp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x10cu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x110u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x114u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x114u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c23580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140763d90; const P1: u64 = 0x143426c20; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x10; const P4: u64 = 0x14325ede8; const P5: u64 = 0x14020f270;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    rax = P2;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r9 = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c23760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140763d90; const P1: u64 = 0x143426c20; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x10; const P4: u64 = 0x14325ede8; const P5: u64 = 0x14020f270;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    rdi = r8;
    rcx = rbx;
    rdx = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    rax = P2;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r9 = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c23c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x14240aa00; const P4: u64 = 0x48; const P5: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    rdx = rbx;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c25750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x14240aa00; const P4: u64 = 0x60; const P5: u64 = 0x140636a20; const P6: u64 = 0x90; const P7: u64 = 0xd8; const P8: u64 = 0x108; const P9: u64 = 0x150; const P10: u64 = 0x180;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = rbp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0xd8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x108); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0x150); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P10))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x180u64), rax); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c25fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x14240aa00; const P8: u64 = 0x48; const P9: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = rdi;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    rdx = rbx;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c26070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432610e0; const P1: u64 = 0x88; const P2: u64 = 0x18; const P3: u64 = 0x30; const P4: u64 = 0x33; const P5: u64 = 0x34; const P6: u64 = 0x38; const P7: u64 = 0x80; const P8: u64 = 0x90; const P9: u64 = 0x98; const P10: u64 = 0xa0; const P11: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x84u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c267c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1403d79a0; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    rbx = rdx;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c26b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r8 = rcx;
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r10 = 0x1414;
    rcx = 0x12b0;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(rdx))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(rdx), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdx, (0x0u64) as u64); } else { fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = r9;
    fl_l = rcx; fl_r = r8;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx), rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c27030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r9 = rdx;
    r10 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r8 = 0x1414;
    rdx = 0x12b0;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, (0x0u64) as u64); } else { fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; }
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = r10;
    fl_l = rcx; fl_r = r9;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx), rax);
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c27ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x14240aa00; const P7: u64 = 0x68; const P8: u64 = 0x140636a20; const P9: u64 = 0x98; const P10: u64 = 0xe0; const P11: u64 = 0x110;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; rcx = rbp.wrapping_add(0xe0); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P11)) as u64) as u64; rcx = rbp.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x158); rcx = rbp.wrapping_add(0x158); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x188); rcx = rbp.wrapping_add(0x188); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x1d0); rcx = rbp.wrapping_add(0x1d0); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x200); rcx = rbp.wrapping_add(0x200); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x248); rcx = rbp.wrapping_add(0x248); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x278u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x278u64), rax); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c28390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1407f8ae0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c28ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14240aa00; const P3: u64 = 0x58; const P4: u64 = 0x140636a20; const P5: u64 = 0x88; const P6: u64 = 0xd0;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    rsi = rdx;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rbp = r8;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P3)) as u64) as u64;
    rcx = rbp.wrapping_add(0x58);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rcx = rbp.wrapping_add(0x88);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rsi.wrapping_add(P6)) as u64) as u64;
    rcx = rbp.wrapping_add(0xd0);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c29340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c29b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14240aa00; const P3: u64 = 0x48; const P4: u64 = 0x140636a20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rdi = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2a290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu16(rdx, (0x0u16) as u16);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f666666u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2a8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu16(rdx, (0x1u16) as u16);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x43b40000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x41000000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2ac50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0xc; const P3: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r10 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P2)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2af20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, (0x42b40000u32) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x14u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f4ccccdu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x3e19999au32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x3d4ccccdu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2b460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x48;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rdx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128; ymm0_hi = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2b4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu16(rdx, (0x101u16) as u16);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3e800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f000000u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x3f000000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x40400000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x3dcccccdu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), (0x42b40000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), (0x14u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (0x3f4ccccdu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), (0x3e19999au32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), (0x3d4ccccdu32) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0x34u64), (0x1u16) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), (0x43b40000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), (0x41000000u32) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0x40u64), (0x0u16) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(0x44u64), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x48u64), (0x3f666666u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2bb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P7))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P7), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P8), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2bbc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427660; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x24; const P4: u64 = 0x18; const P5: u64 = 0x2c; const P6: u64 = 0x14; const P7: u64 = 0x20; const P8: u64 = 0x143159a38; const P9: u64 = 0x30; const P10: u64 = 0x143159a40; const P11: u64 = 0x143427178;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(P0)) as u128;
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128);
    crate::simd_ops::wu64(rdx, (0x3f800000u64) as u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x24u64))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), (((xmm1 >> 64) as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (((xmm1 >> 96) as u32) as u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (((xmm1 >> 32) as u32) as u32) as u32);
    xmm2 = (xmm1) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((xmm2 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P9), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(P10)) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x44u64), ((xmm0 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2c020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P2), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2c070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    let y_addr: u64 = rax.wrapping_add(rdx); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r9.wrapping_add(P2)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r9.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2c2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2c310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x20; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0xbf800000u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2c920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e84a0; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2c9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2d6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2dcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x3; const P2: u64 = 0x4; const P3: u64 = 0x8; const P4: u64 = 0x50; const P5: u64 = 0x60; const P6: u64 = 0x68; const P7: u64 = 0x70; const P8: u64 = 0x48; const P9: u64 = 0x54;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2e460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x28; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x4u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x40000000u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), (0x40a00000u64) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x3f800000u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), (0x40000000u64) as u64);
    crate::simd_ops::wu16(rdx.wrapping_add(P7), (0x101u16) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2f300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340e9e0; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14309f824; const P3: u64 = 0x14307b558; const P4: u64 = 0x140802000; const P5: u64 = 0x1401ee280;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 117>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141c2f300, funcinfo_rva: 0x3743c58, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x3a8, conts: &[0x141c2f470] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x3a0u64)) as u64; rdi = (rcx) as u64; rsi = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdx) as u64); rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141c2f332); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x141c2f332, ({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x141c2f470 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x141c2f300) } }; rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x310u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141c2f345); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x141c2f345, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x141c2f470 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x141c2f300) } }; rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x348u64), ((r14 as u32)) as u32); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x350u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x358u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x360u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x340u64), (r14) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x40u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rcx = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rbx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), (r14) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), (r14) as u64); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), (r14) as u64); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), (r14) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x350u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141c2f3f0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x141c2f3f0, ({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x141c2f470 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x141c2f300) } }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), (rax) as u64); bb = 3; continue; }
            3 => { rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x368u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x370u64), (rax) as u64); rbx = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x378u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x388u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x390u64), (rax) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x398u64), (0x100u16) as u16); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141c2f44d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x141c2f44d, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x141c2f470 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x141c2f300) } }; rax = c_rax; xmm0 = c_x0; rax = (0x4u32) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x378u64), (rbx) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141c2f46a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x141c2f46a, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x141c2f470 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x141c2f300) } }; rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { r11 = ((rsp.wrapping_add(0x3a0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2f550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143263498; const P1: u64 = 0x1401de830; const P2: u64 = 0xe8; const P3: u64 = 0x1401ffd50; const P4: u64 = 0xb8; const P5: u64 = 0x14240ab10; const P6: u64 = 0x142428ce0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x100u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c2f5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142428d70; const P1: u64 = 0x143263498; const P2: u64 = 0xb8; const P3: u64 = 0xbb; const P4: u64 = 0xbc; const P5: u64 = 0xc0; const P6: u64 = 0xe8; const P7: u64 = 0xf0; const P8: u64 = 0x100; const P9: u64 = 0x108;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r10 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r11 = P1;
    crate::simd_ops::wu64(r10, r11);
    crate::simd_ops::lk_and_u32(r10.wrapping_add(P2), (0xff000005u32) as u32);
    crate::simd_ops::lk_or_u32(r10.wrapping_add(P2), (0x5u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(r10.wrapping_add(P3), (0x5u8) as u8);
    crate::simd_ops::wu32(r10.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(r10.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu64(r10.wrapping_add(P6), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P7), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P8), rax);
    crate::simd_ops::wu64(r10.wrapping_add(P9), rax);
    rax = r10;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c301a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140805230;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0xb8u64)) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = rsp.wrapping_add(0x20);
    rax = ((rax).wrapping_add(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c30780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143263498; const P1: u64 = 0x100; const P2: u64 = 0x1401de830; const P3: u64 = 0xe8; const P4: u64 = 0x1401ffd50; const P5: u64 = 0xb8; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142428ce0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c307f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x143263498; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0x100; const P4: u64 = 0x1401de830; const P5: u64 = 0xe8; const P6: u64 = 0x1401ffd50; const P7: u64 = 0xb8; const P8: u64 = 0x14240ab10; const P9: u64 = 0x142428ce0; const P10: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r14 = P1; bb = 3; continue; }
            3 => { rdi = rbp; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(rsi.wrapping_add(P2))) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rdi, r14); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c308a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143263498; const P1: u64 = 0x100; const P2: u64 = 0x1401de830; const P3: u64 = 0xe8; const P4: u64 = 0x1401ffd50; const P5: u64 = 0xb8; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142428ce0; const P8: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rax = P0; crate::simd_ops::wu64(rdx, rax); rcx = ((rdx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}
