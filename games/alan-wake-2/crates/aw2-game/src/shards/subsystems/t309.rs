#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141ceaa80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ceaac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cebe20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cebe90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x1401de830; const P3: u64 = 0x20; const P4: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cebf00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x20; const P2: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cec4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x28; const P2: u64 = 0x50; const P3: u64 = 0x58;
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
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cec580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x28; const P4: u64 = 0x2c; const P5: u64 = 0x30; const P6: u64 = 0x2b; const P7: u64 = 0x50; const P8: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000004u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cecc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x4b; const P7: u64 = 0x70; const P8: u64 = 0x74; const P9: u64 = 0x78; const P10: u64 = 0x73; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0xa0u64), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0xa8u64), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0xa8u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0xacu64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xb0u64), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0xabu64), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ced320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x4b; const P7: u64 = 0x70; const P8: u64 = 0x74; const P9: u64 = 0x78; const P10: u64 = 0x73; const P11: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P11), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0xa0u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0xa4u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xa8u64), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0xa3u64), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ced9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x48; const P2: u64 = 0x70; const P3: u64 = 0x78; const P4: u64 = 0x74;
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
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ceda40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x4b; const P7: u64 = 0x78; const P8: u64 = 0x7c; const P9: u64 = 0x80; const P10: u64 = 0x7b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cee710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403d79a0; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x30; const P4: u64 = 0x40; const P5: u64 = 0x50;
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
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141ceefa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1405a5010;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cef300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140853830;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf0d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a3e8a8; const P1: u64 = 0x143a3e4c0; const P2: u64 = 0x1426d01e0; const P3: u64 = 0x141cf0ee0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; rax = P1; rsi = rcx; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = ((crate::simd_ops::ru8(rcx.wrapping_add(0x33u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); fl_l = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r12 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; rbp = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = ((rdi as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x38); if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { r9 = (((r9 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x7c00; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rbx = rax.wrapping_add(0x8); r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; fl_l = r9; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 15; continue; }
            15 => { r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(rcx))) as u32) as u32 as u64; r9 = ((r9).wrapping_sub(1)) as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(r12))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(r12))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 16; continue; }
            16 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 18; continue; }
            18 => { bb = 20; continue; }
            19 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rdx; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 21; continue; }
            21 => { rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rdx = rsi.wrapping_add(0x38); crate::simd_ops::wu8(rsi.wrapping_add(0x84u64), ((rax as u8)) as u8); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf0ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426d01e0; const P1: u64 = 0x1432b1658; const P2: u64 = 0x1432b1488; const P3: u64 = 0x1432b1478; const P4: u64 = 0x14310aa30; const P5: u64 = 0x1432b1468; const P6: u64 = 0x1432b1450; const P7: u64 = 0x1432b14c8; const P8: u64 = 0x1432b14b8; const P9: u64 = 0x1430896c8; const P10: u64 = 0x1432b14a8; const P11: u64 = 0x1432b1498; const P12: u64 = 0x1432b1508; const P13: u64 = 0x143175068; const P14: u64 = 0x1432b14f8; const P15: u64 = 0x1432b14f0; const P16: u64 = 0x1432b14d8; const P17: u64 = 0x1432b1568; const P18: u64 = 0x1432b1550; const P19: u64 = 0x1432b1530; const P20: u64 = 0x1432b1520; const P21: u64 = 0x1432b1440; const P22: u64 = 0x1432b1430; const P23: u64 = 0x14312b590;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdi) as u64); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = ((P0) as u64) as u64; rbx = (rcx) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; fl_l = (r8) as u64 as u64; fl_r = (0xcu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = ((P1) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 2; continue; }
            2 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rdx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rdx))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 7; continue; } bb = 4; continue; }
            4 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = ((r8.wrapping_add(0xau64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r11 = ((P2) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xdu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 9; continue; }
            9 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (r11) as u64; bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 11; continue; }
            11 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = ((rdx.wrapping_add(0xau64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xbu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { rcx = ((P3) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 21; continue; } bb = 18; continue; }
            18 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 21; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 20; continue; }
            20 => { rax = (0xau32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 22; continue; }
            22 => { rcx = ((P4) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 23; continue; }
            23 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 28; continue; } bb = 25; continue; }
            25 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 28; continue; } bb = 26; continue; }
            26 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 27; continue; }
            27 => { rax = ((rdx.wrapping_add(0x9u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xau64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 29; continue; }
            29 => { rcx = ((P5) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 31; continue; }
            31 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 35; continue; } bb = 32; continue; }
            32 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 35; continue; } bb = 33; continue; }
            33 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 34; continue; }
            34 => { rax = (0x9u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x12u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 36; continue; }
            36 => { rcx = ((P6) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 37; continue; }
            37 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 38; continue; }
            38 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 42; continue; } bb = 39; continue; }
            39 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 41; continue; }
            41 => { rax = ((rdx.wrapping_add(0x8u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            42 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xau64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 43; continue; }
            43 => { rcx = ((P7) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 44; continue; }
            44 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 45; continue; }
            45 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 49; continue; } bb = 46; continue; }
            46 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 49; continue; } bb = 47; continue; }
            47 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 48; continue; }
            48 => { rax = (0x8u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            49 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x9u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 56; continue; } bb = 50; continue; }
            50 => { rcx = ((P8) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 51; continue; }
            51 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 52; continue; }
            52 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 56; continue; } bb = 53; continue; }
            53 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 56; continue; } bb = 54; continue; }
            54 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 52; continue; } bb = 55; continue; }
            55 => { rax = ((rdx.wrapping_add(0x7u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            56 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xdu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 63; continue; } bb = 57; continue; }
            57 => { rcx = ((P9) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 58; continue; }
            58 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 59; continue; }
            59 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 63; continue; } bb = 60; continue; }
            60 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 63; continue; } bb = 61; continue; }
            61 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 59; continue; } bb = 62; continue; }
            62 => { rax = (0x7u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            63 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 70; continue; } bb = 64; continue; }
            64 => { rcx = ((P10) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 65; continue; }
            65 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 66; continue; }
            66 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 70; continue; } bb = 67; continue; }
            67 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 70; continue; } bb = 68; continue; }
            68 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 66; continue; } bb = 69; continue; }
            69 => { rax = ((rdx.wrapping_add(0x6u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            70 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xcu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 77; continue; } bb = 71; continue; }
            71 => { rcx = ((P11) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 72; continue; }
            72 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 73; continue; }
            73 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 77; continue; } bb = 74; continue; }
            74 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 77; continue; } bb = 75; continue; }
            75 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 73; continue; } bb = 76; continue; }
            76 => { rax = (0x6u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            77 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x16u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 84; continue; } bb = 78; continue; }
            78 => { rcx = ((P12) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 79; continue; }
            79 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 80; continue; }
            80 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 84; continue; } bb = 81; continue; }
            81 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 84; continue; } bb = 82; continue; }
            82 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 80; continue; } bb = 83; continue; }
            83 => { rax = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            84 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x9u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 91; continue; } bb = 85; continue; }
            85 => { rcx = ((P13) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 86; continue; }
            86 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 87; continue; }
            87 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 91; continue; } bb = 88; continue; }
            88 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 91; continue; } bb = 89; continue; }
            89 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 87; continue; } bb = 90; continue; }
            90 => { rax = (0x5u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            91 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xdu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 98; continue; } bb = 92; continue; }
            92 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 97; continue; } bb = 93; continue; }
            93 => { r9 = ((xmm0 as u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r11) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 94; continue; }
            94 => { rax = ((crate::simd_ops::ru8(r11.wrapping_add(r9))) as u32) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 98; continue; } bb = 95; continue; }
            95 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 98; continue; } bb = 96; continue; }
            96 => { r11 = ((r11).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 94; continue; } bb = 97; continue; }
            97 => { rax = (0x4u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            98 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xcu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 105; continue; } bb = 99; continue; }
            99 => { rcx = ((P14) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 111; continue; } bb = 100; continue; }
            100 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 101; continue; }
            101 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 105; continue; } bb = 102; continue; }
            102 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 105; continue; } bb = 103; continue; }
            103 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 101; continue; } bb = 104; continue; }
            104 => { rax = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            105 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x7u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 112; continue; } bb = 106; continue; }
            106 => { rcx = ((P15) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 111; continue; } bb = 107; continue; }
            107 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 108; continue; }
            108 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 112; continue; } bb = 109; continue; }
            109 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 112; continue; } bb = 110; continue; }
            110 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 108; continue; } bb = 111; continue; }
            111 => { rax = (0x3u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            112 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x16u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 119; continue; } bb = 113; continue; }
            113 => { rcx = ((P16) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 114; continue; }
            114 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 115; continue; }
            115 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 119; continue; } bb = 116; continue; }
            116 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 119; continue; } bb = 117; continue; }
            117 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 115; continue; } bb = 118; continue; }
            118 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            119 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x17u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 126; continue; } bb = 120; continue; }
            120 => { rcx = ((P17) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 121; continue; }
            121 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 122; continue; }
            122 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 126; continue; } bb = 123; continue; }
            123 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 126; continue; } bb = 124; continue; }
            124 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 122; continue; } bb = 125; continue; }
            125 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            126 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x16u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 133; continue; } bb = 127; continue; }
            127 => { rcx = ((P18) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 128; continue; }
            128 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 129; continue; }
            129 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 133; continue; } bb = 130; continue; }
            130 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 133; continue; } bb = 131; continue; }
            131 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 129; continue; } bb = 132; continue; }
            132 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            133 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1du64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 140; continue; } bb = 134; continue; }
            134 => { rcx = ((P19) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 135; continue; }
            135 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 136; continue; }
            136 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 140; continue; } bb = 137; continue; }
            137 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 140; continue; } bb = 138; continue; }
            138 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 136; continue; } bb = 139; continue; }
            139 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            140 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xbu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 147; continue; } bb = 141; continue; }
            141 => { rcx = ((P20) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 142; continue; }
            142 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 143; continue; }
            143 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 147; continue; } bb = 144; continue; }
            144 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 147; continue; } bb = 145; continue; }
            145 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 143; continue; } bb = 146; continue; }
            146 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            147 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xcu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 154; continue; } bb = 148; continue; }
            148 => { rcx = ((P21) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 149; continue; }
            149 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 150; continue; }
            150 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 154; continue; } bb = 151; continue; }
            151 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 154; continue; } bb = 152; continue; }
            152 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 150; continue; } bb = 153; continue; }
            153 => { rax = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            154 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xau64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 161; continue; } bb = 155; continue; }
            155 => { rcx = ((P22) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 160; continue; } bb = 156; continue; }
            156 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 157; continue; }
            157 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 161; continue; } bb = 158; continue; }
            158 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 161; continue; } bb = 159; continue; }
            159 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 157; continue; } bb = 160; continue; }
            160 => { rax = (0x2u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            161 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1au64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 168; continue; } bb = 162; continue; }
            162 => { rcx = ((P23) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 167; continue; } bb = 163; continue; }
            163 => { r10 = ((xmm0 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 164; continue; }
            164 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 168; continue; } bb = 165; continue; }
            165 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 168; continue; } bb = 166; continue; }
            166 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 164; continue; } bb = 167; continue; }
            167 => { rax = (0x1u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            168 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf1ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x1c;
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
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf2470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
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
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf3dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf49d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r10 = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(r8), ((rax as u8)) as u8);
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r10)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf4cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx), ((rax as u8)) as u8);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf6430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf64f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431f3368; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf7bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0xc; const P3: u64 = 0xd; const P4: u64 = 0x10; const P5: u64 = 0x14; const P6: u64 = 0x18; const P7: u64 = 0x1c; const P8: u64 = 0x20; const P9: u64 = 0x21; const P10: u64 = 0x22; const P11: u64 = 0x23; const P12: u64 = 0x14240aa00;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64; rdi = r8; crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P8), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P9), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x24u64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x28u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(0x24u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x50u64))) as u32) as u32 as u64; rdx = rbx.wrapping_add(0x54); rcx = rdi.wrapping_add(0x54); crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf7cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143084fd0; const P1: u64 = 0x8; const P2: u64 = 0x14; const P3: u64 = 0x1c; const P4: u64 = 0x28; const P5: u64 = 0x2c; const P6: u64 = 0x30; const P7: u64 = 0x2b; const P8: u64 = 0x54; const P9: u64 = 0x58; const P10: u64 = 0x5c; const P11: u64 = 0x57;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P4), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P4), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P8), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P8), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P11), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cf8290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14021f1e0; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x14021f040; const P5: u64 = 0x30; const P6: u64 = 0x14021eec0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
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
pub unsafe extern "win64" fn fn_141cf8720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfbe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x60u64), rax); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfbf10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432cd9f8; const P1: u64 = 0x31; const P2: u64 = 0x30; const P3: u64 = 0x35; const P4: u64 = 0x37; const P5: u64 = 0x51; const P6: u64 = 0x50; const P7: u64 = 0x53; const P8: u64 = 0x60;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xff0000u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    crate::simd_ops::wu16(rdx.wrapping_add(P3), (0x0u16) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0xffu8) as u8);
    crate::simd_ops::wu16(rdx.wrapping_add(P5), (0x0u16) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0xffu8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfc140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r11 = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r10 = rcx;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfc4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x14u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfd390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1405a5010; const P2: u64 = 0x18;
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
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfd420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430acca0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfde10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfdfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0xcu64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cfe020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430bf290; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0xf;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cff000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432be0a0; const P1: u64 = 0x143874640; const P2: u64 = 0x1432be080; const P3: u64 = 0x1401de410; const P4: u64 = 0x110; const P5: u64 = 0x1432be138; const P6: u64 = 0x1432be120;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    rcx = P0;
    r10 = r8;
    rbx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx);
    r9 = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11);
    r8 = ((r11.wrapping_add(0x13u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0xd0u64)) as u64;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx);
    rdx = ((rax.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rax = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11);
    r9 = P6;
    r8 = ((r11.wrapping_add(0x14u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cff4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140225870; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1c; const P5: u64 = 0x1401e84a0; const P6: u64 = 0x38; const P7: u64 = 0x50; const P8: u64 = 0x68; const P9: u64 = 0x80; const P10: u64 = 0x98; const P11: u64 = 0xb0;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rbp = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rdx = ((rsi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rcx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; rdx = rsi.wrapping_add(0xb8); crate::simd_ops::wu32(rbp.wrapping_add(0xb0u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0xb8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xd0u64))) as u32 as u64; rdx = rsi.wrapping_add(0xd8); rcx = rbp.wrapping_add(0xd8); crate::simd_ops::wu32(rbp.wrapping_add(0xd0u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xf0u64))) as u32 as u64; rdx = rsi.wrapping_add(0xf8); crate::simd_ops::wu32(rbp.wrapping_add(0xf0u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0xf8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xf4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xf4u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x110u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x110u64), ((rax as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141cffcc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430d3420; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x1u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d01980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x1401e84a0; const P11: u64 = 0x48;
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
    rdx = ((rdx).wrapping_add(0x30u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P11)) as u64) as u64;
    rcx = rdi.wrapping_add(0x48);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d02020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x50; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30; const P12: u64 = 0x140397e30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdi = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d02100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430d85d8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x50; const P8: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d03660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x140461550;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x20u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d03ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140c36610;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d04450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432bd148; const P1: u64 = 0x1432bd118; const P2: u64 = 0x1432bd130; const P3: u64 = 0x140551e90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P1;
    rcx = P2;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x1;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d04780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d04940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d04990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430f6be8; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d04f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rdi = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d05710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
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
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d062e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x140c06cf0; const P4: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P4), (xmm0) as u128);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d064d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x140225740; const P4: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d06550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430f41f0; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x50; const P6: u64 = 0x58; const P7: u64 = 0x60; const P8: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d06960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98; const P4: u64 = 0x140c06cf0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d06b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98; const P4: u64 = 0x140225740;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d06bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430f4098; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x53; const P9: u64 = 0x98; const P10: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d06d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432bb858; const P1: u64 = 0x1432bb57c; const P2: u64 = 0x1432bb588; const P3: u64 = 0x1402c61c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P1;
    rcx = P2;
    rdx = ((rdx).wrapping_add(0x98u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x1;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d07030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rbx).wrapping_add(P3), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d07220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d072a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430f3f18; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x53; const P9: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d07870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x30; const P3: u64 = 0x58; const P4: u64 = 0x80; const P5: u64 = 0x84; const P6: u64 = 0x88;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d07910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431057e8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x30; const P6: u64 = 0x34; const P7: u64 = 0x38; const P8: u64 = 0x33; const P9: u64 = 0x58; const P10: u64 = 0x5c; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000004u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x4u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x4u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x5bu64), (0x4u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x84u64), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0x88u64), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d07b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x30; const P3: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdi).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdi).wrapping_add(P3), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d07eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x30; const P3: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rbx = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdi).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdi).wrapping_add(P3), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d08110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432c45d8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x30; const P6: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d08a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x24;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r9 = ((rcx.wrapping_add(P1)) as u64) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r11 = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = rcx; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32); r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r9.wrapping_add(r8), rax); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P5)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d08b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x1432bb9e8; const P2: u64 = 0x1432bb9f8; const P3: u64 = 0x14082b2b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P1;
    rcx = P2;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x2;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d08e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x24;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r9 = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r10 = rdx; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu64(r9.wrapping_add(rcx), rax); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P5)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d09030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x2c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d090a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432c8110; const P1: u64 = 0x1b; const P2: u64 = 0x10; const P3: u64 = 0x19; const P4: u64 = 0x18; const P5: u64 = 0x24; const P6: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0xffu8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rdx.wrapping_add(P3), ((rax as u16)) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d094b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d10550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x19; const P6: u64 = 0x1a; const P7: u64 = 0x1c; const P8: u64 = 0x20; const P9: u64 = 0x24; const P10: u64 = 0x28; const P11: u64 = 0x2c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x30u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(0x30u64), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d11180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x38; const P4: u64 = 0x50; const P5: u64 = 0x68; const P6: u64 = 0x80; const P7: u64 = 0x98; const P8: u64 = 0xb0; const P9: u64 = 0xc8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P8)) as u64) as u64; rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P9)) as u64) as u64; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d11800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x38; const P4: u64 = 0x50; const P5: u64 = 0x68; const P6: u64 = 0x80; const P7: u64 = 0x98; const P8: u64 = 0xb0; const P9: u64 = 0xc8; const P10: u64 = 0xe0; const P11: u64 = 0xf8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P8)) as u64) as u64; rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P9)) as u64) as u64; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P10)) as u64) as u64; rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P11)) as u64) as u64; rcx = rdi.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x110); rcx = rdi.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d11ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438753b0; const P1: u64 = 0x1432baf00; const P2: u64 = 0x1432baf30; const P3: u64 = 0x1432baf18; const P4: u64 = 0x1401de410; const P5: u64 = 0xc; const P6: u64 = 0x143877370; const P7: u64 = 0x1432baf98; const P8: u64 = 0x1432bafc0; const P9: u64 = 0x1432bafa8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r10 = r8;
    rcx = P3;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = ((r11.wrapping_add(0x8u64)) as u32) as u32 as u64;
    rcx = r10;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P5)) as u64) as u64;
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P7;
    rax = P8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = ((r11.wrapping_add(0x15u64)) as u32) as u32 as u64;
    rax = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d11f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x1401e84a0; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x34; const P8: u64 = 0x50; const P9: u64 = 0x68; const P10: u64 = 0x70; const P11: u64 = 0x6c; const P12: u64 = 0x14240aa00;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rbp = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rdx = ((rsi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x38); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; rdx = ((rsi.wrapping_add(P10)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((rax as u32)) as u32); rcx = rbp.wrapping_add(0x70); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x88); rcx = rbp.wrapping_add(0x88); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d12040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430892b8; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x2c; const P6: u64 = 0x34; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x4c; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x60u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x64u64), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0x6cu64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x84u64), ((rax as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0x88u64), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0x88u64), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x90u64), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0x8bu64), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d12f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143094ab8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P5), (0x100u16) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d140d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432bb528; const P1: u64 = 0x1432bb518; const P2: u64 = 0x140947eb0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0x5;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d143a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x14240aa00; const P4: u64 = 0xa8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rbx = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P4), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d14610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143091e50; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x23;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d14790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432b0a48; const P1: u64 = 0x14329ebe8; const P2: u64 = 0x1432bb288; const P3: u64 = 0x14088ba10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P1;
    rcx = P2;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0xa;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d14c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x1401e84a0; const P7: u64 = 0x38;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
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
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d14cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143151e80; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x34; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d15270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x140bf9ea0; const P4: u64 = 0xf8; const P5: u64 = 0x1d0; const P6: u64 = 0x2a8; const P7: u64 = 0x2c0; const P8: u64 = 0x2d8;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rbp = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = rbp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P4)) as u64) as u64; rcx = rbp.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P5)) as u64) as u64; rcx = rbp.wrapping_add(0x1d0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P6)) as u64) as u64; rcx = rbp.wrapping_add(0x2a8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; rcx = rbp.wrapping_add(0x2c0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P8)) as u64) as u64; rcx = rbp.wrapping_add(0x2d8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d15e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d161a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d16990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
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
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d17120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x140bfaa60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x28u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d17590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xa;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx), ((rax as u8)) as u8);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d183d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14315ef90; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x1b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d19dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1c1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x9; const P3: u64 = 0xc; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x1404c61f0; const P9: u64 = 0x38; const P10: u64 = 0x3c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1c290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430a43c8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1c850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x90; const P3: u64 = 0x94; const P4: u64 = 0x98; const P5: u64 = 0x9c; const P6: u64 = 0xa0; const P7: u64 = 0xa4;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
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
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1c900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431628d8; const P1: u64 = 0x8; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x91; const P6: u64 = 0x90; const P7: u64 = 0x93;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x10u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    crate::simd_ops::wu16(rdx.wrapping_add(P5), (0x0u16) as u16);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0xffu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1d010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x38; const P4: u64 = 0x14240aa00; const P5: u64 = 0x80; const P6: u64 = 0x84;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32) as u128;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((xmm0 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((xmm1 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1d0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432c5478; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x34; const P9: u64 = 0x38; const P10: u64 = 0x3c; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0x3bu64), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1e910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x58; const P4: u64 = 0x54; const P5: u64 = 0x1401e84a0; const P6: u64 = 0x70;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1e9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143173f58; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x58; const P6: u64 = 0x60; const P7: u64 = 0x68; const P8: u64 = 0x6c; const P9: u64 = 0x70; const P10: u64 = 0x78; const P11: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x84u64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1f580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x60; const P4: u64 = 0x54; const P5: u64 = 0x58; const P6: u64 = 0x5c; const P7: u64 = 0x1401e84a0; const P8: u64 = 0x78; const P9: u64 = 0x88; const P10: u64 = 0x7c; const P11: u64 = 0x80;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rdx = ((rbx.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x84u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1f640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143163918; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x60; const P6: u64 = 0x68; const P7: u64 = 0x70; const P8: u64 = 0x74; const P9: u64 = 0x88; const P10: u64 = 0x90; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x9cu64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1fa80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x19; const P5: u64 = 0x1a; const P6: u64 = 0x14; const P7: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P7), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1fc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x19; const P6: u64 = 0x1a; const P7: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d1ff20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x14; const P5: u64 = 0x1c; const P6: u64 = 0x4; const P7: u64 = 0x20; const P8: u64 = 0x24;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r9 = ((rcx.wrapping_add(P1)) as u64) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r10 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = rdx; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32); r8 = ((r10.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P4)) as u64) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P6), ((rax as u32)) as u32); r8 = ((r10.wrapping_add(P7)) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P7))) as u32 as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r11)) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P6), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d202a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x4; const P7: u64 = 0x20; const P8: u64 = 0x24;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = ((rdx.wrapping_add(P1)) as u64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; r10 = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r11 = r8; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((r10.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; r8 = ((r10.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(r11))) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P6), ((rax as u32)) as u32); r8 = ((r10.wrapping_add(P7)) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(P7))) as u32 as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r11)) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P6), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d204e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm0 as u64)) as u64);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d22c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1401e84a0; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x39; const P9: u64 = 0x3a; const P10: u64 = 0x3c; const P11: u64 = 0x14240aa00;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rbx = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdi = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64; rdx = ((rbx.wrapping_add(P7)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x68u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x68u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x74u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d22d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430b62b0; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x34; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x44; const P10: u64 = 0x48; const P11: u64 = 0x43;
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
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P8), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P8), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P11), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d24930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x38; const P4: u64 = 0x140538800; const P5: u64 = 0x48; const P6: u64 = 0x140538940;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
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
pub unsafe extern "win64" fn fn_141d249c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143199768; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x34; const P9: u64 = 0x38; const P10: u64 = 0x40; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d255a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x28; const P3: u64 = 0x38; const P4: u64 = 0x48; const P5: u64 = 0x4c; const P6: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P2), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d25620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430eb9e8; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d26220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x31; const P10: u64 = 0x32; const P11: u64 = 0x34; const P12: u64 = 0x14053f5f0; const P13: u64 = 0x1404b9990;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rcx = ((r8.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); rbx = rdx; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdi = r8; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P8), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P9), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P11))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x34u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x3cu64))) as u64; rdx = ((rdx).wrapping_add(0x48u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x3cu64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x58); rcx = rdi.wrapping_add(0x58); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d262f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431d6c78; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x34; const P7: u64 = 0x3c; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
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
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(0x70u64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d27d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434269f8; const P1: u64 = 0x1432b6b00; const P2: u64 = 0x1432b6ad8; const P3: u64 = 0x1432b6ae8; const P4: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm1 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    rax = r8;
    rcx = P1;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P2;
    rcx = P3;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x1;
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d28010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d287e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x30; const P3: u64 = 0x14; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x1402251c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = ((r8.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x30u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    return crate::simd_ops::tail_fwd_xmm(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d28860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431d64e0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38;
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
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2a7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140599e00; const P3: u64 = 0x20; const P4: u64 = 0x1404b9990; const P5: u64 = 0x40; const P6: u64 = 0x60; const P7: u64 = 0x80; const P8: u64 = 0x88; const P9: u64 = 0xb0; const P10: u64 = 0x90; const P11: u64 = 0x1401e84a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; rbx = rdx; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x10u64)) as u64; rdi = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdi.wrapping_add(P7), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rdx = ((rbx.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; let y_addr: u64 = rbx.wrapping_add(P10); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(P10); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xc8); rcx = rdi.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xe0); rcx = rdi.wrapping_add(0xe0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xf8); rcx = rdi.wrapping_add(0xf8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x110u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x110u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2af90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x90; const P3: u64 = 0x118; const P4: u64 = 0x119; const P5: u64 = 0x11c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
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
pub unsafe extern "win64" fn fn_141d2b030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143099790; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x90; const P6: u64 = 0x94; const P7: u64 = 0x98; const P8: u64 = 0x93; const P9: u64 = 0x11c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2bd80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x28; const P5: u64 = 0x40; const P6: u64 = 0x58;
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
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2be20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431ec530; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x3c; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x54u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x6cu64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2c940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140c0a990; const P2: u64 = 0x140c0df10;
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
pub unsafe extern "win64" fn fn_141d2e5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x1401e84a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x20u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2e610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430edbd8; const P1: u64 = 0xc; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d2f320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140bfab90; const P3: u64 = 0x20; const P4: u64 = 0x14240aa00;
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
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d30140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432b5b48; const P1: u64 = 0x1432b5b30; const P2: u64 = 0x14088ba10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0x8;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d305c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x58; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30; const P12: u64 = 0x1401e84a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x50u64), ((rax as u32)) as u32); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d30690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431957e0; const P1: u64 = 0x44; const P2: u64 = 0x4c; const P3: u64 = 0x58; const P4: u64 = 0x60; const P5: u64 = 0x68; const P6: u64 = 0x6c;
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
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d30830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428390; const P1: u64 = 0x143426a48; const P2: u64 = 0x1432b3ef8; const P3: u64 = 0x1432b3f88; const P4: u64 = 0x1432b3ed0; const P5: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rax = r8;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P3;
    rcx = P4;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x2;
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d32100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98; const P4: u64 = 0xe0; const P5: u64 = 0x128; const P6: u64 = 0x12c; const P7: u64 = 0x130; const P8: u64 = 0x134;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d321d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431fb6f8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x53; const P9: u64 = 0x98; const P10: u64 = 0x9c; const P11: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x9bu64), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0xe0u64), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0xe0u64), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0xe4u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xe8u64), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(0xe3u64), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d32850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x30; const P3: u64 = 0x14060e5f0; const P4: u64 = 0x40; const P5: u64 = 0x14060e510; const P6: u64 = 0x50; const P7: u64 = 0x54;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d328e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14311e968; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000004u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x4u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d32d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P5), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d32f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d33550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x34; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30; const P12: u64 = 0x14240aa00; const P13: u64 = 0x1401e84a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rbx = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdi = r8; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; rdx = ((rdx).wrapping_add(0x34u64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x80); rcx = rdi.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x98); rcx = rdi.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xb0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xbcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xbcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xc0u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xc4u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xc4u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d33670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432020f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x34; const P4: u64 = 0x38; const P5: u64 = 0x3c; const P6: u64 = 0x37; const P7: u64 = 0x80; const P8: u64 = 0x88; const P9: u64 = 0x90; const P10: u64 = 0x94; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0xa8u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xacu64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d33ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d346c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r11 = rdx;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = rcx;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P0), rax);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
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
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d34ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d352a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428390; const P1: u64 = 0x143426a48; const P2: u64 = 0x1432b3f88; const P3: u64 = 0x8; const P4: u64 = 0x1432b3ef8; const P5: u64 = 0x1432b3ed0; const P6: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    rax = rdx;
    r9 = P2;
    rdx = ((rcx.wrapping_add(P3)) as u64) as u64;
    r8 = 0x2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d35870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140c0a990; const P2: u64 = 0x140c0df10;
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
pub unsafe extern "win64" fn fn_141d362a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0xd;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d373b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x1432b5b48; const P2: u64 = 0x1432b5b30; const P3: u64 = 0x14088ba10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = 0x8;
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P2;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d38120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14329ebe8; const P1: u64 = 0x20; const P2: u64 = 0x1432b0a48; const P3: u64 = 0x1432bb288; const P4: u64 = 0x14088ba10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0xa;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d39400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14;
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
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((r10.wrapping_add(P4)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d39990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xa;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r10 = rdx;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(r8), ((rax as u8)) as u8);
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r10)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d39c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x19; const P5: u64 = 0x1a; const P6: u64 = 0x14; const P7: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = rcx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r9.wrapping_add(P4))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P4), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P5), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P6), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P7), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3a210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = rdx;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(rcx).wrapping_add(P1), rax);
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r9)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3a660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rbx).wrapping_add(P3), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3a820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x98; const P4: u64 = 0x140c06cf0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3a9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x140c06cf0; const P4: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P4), (xmm0) as u128);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3af50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3b1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = rdx;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r9)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx).wrapping_add(P1), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3c1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x90; const P3: u64 = 0x94;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3df20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e84a0; const P2: u64 = 0x20; const P3: u64 = 0x14240aa00; const P4: u64 = 0xa8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P4), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3e170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432baf00; const P1: u64 = 0x8; const P2: u64 = 0x1438753b0; const P3: u64 = 0x1432baf30; const P4: u64 = 0x1432baf18; const P5: u64 = 0x1401de410; const P6: u64 = 0xc; const P7: u64 = 0x143877370; const P8: u64 = 0x1432baf98; const P9: u64 = 0x1432bafc0; const P10: u64 = 0x1432bafa8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rcx;
    r9 = P0;
    r10 = rdx;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    r8 = ((r11.wrapping_add(0x8u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = r10;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P6)) as u64) as u64;
    rax = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P8;
    rax = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = ((r11.wrapping_add(0x15u64)) as u32) as u32 as u64;
    rax = P10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3e330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143874640; const P1: u64 = 0x1432be080; const P2: u64 = 0xd0; const P3: u64 = 0x1432be0a0; const P4: u64 = 0x1401de410; const P5: u64 = 0x110; const P6: u64 = 0x1432be138; const P7: u64 = 0x1432be120;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r11 = 0;
    fl_l = 0; fl_r = 0;
    rbx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx);
    r9 = P1;
    r10 = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11);
    rax = rcx;
    r8 = ((r11.wrapping_add(0x13u64)) as u32) as u32 as u64;
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = r10;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx);
    rdx = ((rax.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11);
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11);
    r9 = P7;
    r8 = ((r11.wrapping_add(0x14u64)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3e6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d3f200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1403d79a0; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x29;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(r8, rax);
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P5), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d40550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d405a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x13;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141d40b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e84a0; const P1: u64 = 0x18; const P2: u64 = 0x14240aa00; const P3: u64 = 0xa0; const P4: u64 = 0xa1;
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
