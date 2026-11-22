#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x108; const P2: u64 = 0x4; const P3: u64 = 0x14221f5e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdi = rdx; rsi = P0; rbx = rcx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0xcu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x108; const P2: u64 = 0x4; const P3: u64 = 0x14221f5e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdi = r8; rsi = P0; rbx = rdx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0xcu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x1438583d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(P2, ((rcx as u8)) as u8); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x1438583d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(P2, ((rcx as u8)) as u8); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x428;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { r8 = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rcx = 0x1; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(r8.wrapping_mul(8)))) as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx.wrapping_add(P1), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = rax; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x428;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r10 = P0;
    loop { rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx.wrapping_add(P1), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = rax; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x48; const P2: u64 = 0x14207a1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14);
    rdi = rdx;
    r14 = P0;
    rsi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x48; const P2: u64 = 0x14207a1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14);
    rdi = r8;
    r14 = P0;
    rsi = rdx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x4; const P2: u64 = 0x48;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rdx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rcx as u32)) as u32); fl_l = rax; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d8fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x48; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r10 = P0;
    loop { rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = rax; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d9010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143bd6048; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0xbc; const P4: u64 = 0x142267910;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi);
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rsi = P1;
    loop { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = rbx; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc4u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d9070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143bd6048; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0xbc; const P4: u64 = 0x142267910;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi);
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rsi = P1;
    loop { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = rbx; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc4u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d90f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143867210; const P1: u64 = 0x143079e38; const P2: u64 = 0x140000000; const P3: u64 = 0x143426628; const P4: u64 = 0x1434268d0; const P5: u64 = 0x143426780; const P6: u64 = 0x143426784; const P7: u64 = 0x143cf4bf0; const P8: u64 = 0x143881b40; const P9: u64 = 0x14342677c; const P10: u64 = 0x143426564; const P11: u64 = 0x1438672ac; const P12: u64 = 0x1438672c1; const P13: u64 = 0x140498ea0; const P14: u64 = 0x14262ee30; const P15: u64 = 0x141834d18; const P16: u64 = 0x141834a0c; const P17: u64 = 0x141834a78; const P18: u64 = 0x143a001ac; const P19: u64 = 0x143881b38; const P20: u64 = 0x143881b3c; const P21: u64 = 0x143881b30; const P22: u64 = 0x1434280a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); rbp = r8; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r12); rdi = rdx; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r13); r12 = P0; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); r13 = P1; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); r15 = P2; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm10) as u128); rax = ((crate::exe::tls_pointer() as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm12) as u128); r14 = (crate::simd_ops::ru64(rax)) as u64; xmm12 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm13) as u128); xmm1 = 0u128; xmm9 = (xmm1) as u128; xmm1 = (xmm11) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm10 = (xmm1) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm14) as u128); xmm14 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm1 = 0u128; xmm0 = crate::simd_ops::vcmpss(xmm2, (xmm2 as u32), (xmm1 as u32), 0x0u8); xmm3 = crate::simd_ops::vblendvps(xmm2, xmm10, xmm0); xmm13 = crate::simd_ops::vrcpss(xmm3, (xmm3 as u32)); xmm0 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) + f32::from_bits(xmm13 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm15) as u128); xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); rcx = 0x74; xmm15 = 0u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(rdi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; rsi = (crate::simd_ops::ru64(r15.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x3bd6048u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r14))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 3; continue; }
            3 => { xmm2 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm0 = 0u128; xmm5 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm1 = (xmm7) as u128; xmm6 = (xmm1) as u128; xmm1 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm4 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vcmpss(xmm10, (xmm10 as u32), (xmm6 as u32), 0x2u8); xmm3 = (xmm6 & !0xffff_ffffu128) | (xmm0 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vcmpss(xmm6, (xmm6 as u32), (xmm9 as u32), 0x2u8); xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm9 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm9 as u32) }).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(xmm10 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm10 as u32) }).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = crate::simd_ops::vblendvps(xmm2, xmm5, xmm0); xmm0 = ((crate::simd_ops::ru32(P9)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; xmm2 = crate::simd_ops::vblendvps(xmm1, xmm4, xmm3); crate::simd_ops::wu32(rsi.wrapping_add(0x68cu64), ((xmm2 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm9 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm9 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm10 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm10 as u32) }).to_bits() as u128); xmm0 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); bb = 8; continue; }
            5 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm9 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm9 as u32) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(xmm10 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm10 as u32) }).to_bits() as u128); xmm0 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); bb = 8; continue; }
            7 => { crate::simd_ops::wu32(P11, ((xmm8 as u32)) as u32); bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(P12)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0xf0); rcx = r12; xmm0 = ((crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xb4u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xf0u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; rcx = 0x74; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = P14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            12 => { rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 13; continue; }
            13 => { rcx = (((crate::simd_ops::ru32(P18)) as i32 as i64) as u64) as u64; crate::simd_ops::wu32(P19, (0x2u32) as u32); crate::simd_ops::wu8(P20, (0x0u8) as u8); crate::simd_ops::wu64(P21, r13); xmm0 = (crate::simd_ops::ru128(P22)) as u128; crate::simd_ops::wu128(P8, (xmm0) as u128); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 14; continue; }
            14 => { rdx = P21; crate::simd_ops::wu64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x3a00290u64), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P18, ((rcx as u32)) as u32); bb = 11; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d9610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141302900; const P1: u64 = 0x142269af0; const P2: u64 = 0x14206aad0; const P3: u64 = 0x14183acbc; const P4: u64 = 0x14206a7e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); r14 = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); rax = rdx.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r11 = (crate::simd_ops::ru64(rax)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r13 = rax; r15 = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(r11, ((rbx as u32)) as u32); crate::simd_ops::wu32(r10, ((r15 as u32)) as u32); rcx = ((((rcx as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(r11)) as i32) as u32) as u32 as u64; r9 = ((((rcx as u32)) as i32 as i64) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r13)) as u64; rax = rsp.wrapping_add(0x30); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x14u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx).wrapping_shl(2u32)) as u64; rbx = ((fl_l as u64)) as u64; rbp = ((r15 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { r8 = rbx; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; rdi = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 21; continue; }
            8 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { r12 = (crate::simd_ops::ru64(rsi)) as u64; bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rax = ((r11 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; r9 = ((((r9 as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x30u64)))) as u64; r14 = ((r12.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = ((rbx.wrapping_sub(0x4u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; r8 = r14.wrapping_add(0x8); fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rsi = r9.wrapping_add(0x4); rbp = ((rax as u32)) as u32 as u64; rdi = ((rax.wrapping_mul(4)) as u32) as u32 as u64; r10 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 12; continue; }
            12 => { rcx = ((crate::simd_ops::ru16(rsi.wrapping_sub(0x4u64))) as u32) as u32 as u64; r8 = r8.wrapping_add(0x10); rdx = ((rcx as u32)) as u32 as u64; rsi = rsi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x18u64), ((rdx as u32)) as u32); rcx = ((crate::simd_ops::ru16(rsi.wrapping_sub(0xau64))) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x14u64), ((rdx as u32)) as u32); rcx = ((crate::simd_ops::ru16(rsi.wrapping_sub(0x8u64))) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x10u64), ((rdx as u32)) as u32); rcx = ((crate::simd_ops::ru16(rsi.wrapping_sub(0x6u64))) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0xcu64), ((rdx as u32)) as u32); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { bb = 19; continue; }
            15 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { r8 = ((r14.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r9 = ((r9.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; r10 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rcx = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; r8 = r8.wrapping_add(0x4); rdx = ((rcx as u32)) as u32 as u64; r9 = r9.wrapping_add(0x2); rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x4u64), ((rdx as u32)) as u32); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r13)) as u64; r9 = rsp.wrapping_add(0x30); r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r14); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 23; continue; }
            23 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d9940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433f5d58; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x1401df980; const P3: u64 = 0x1401dfc80; const P4: u64 = 0x141302900; const P5: u64 = 0x14206aad0; const P6: u64 = 0x14241c0a0; const P7: u64 = 0x14183acc2; const P8: u64 = 0x14241c190; const P9: u64 = 0x14206a7e0; const P10: u64 = 0x14240ab10; const P11: u64 = 0x1401de590; const P12: u64 = 0x1423d9cf4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423d9940, funcinfo_rva: 0x37d98e0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbp = (r8) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (r14) as u64); rdi = ((rdx.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), (rdi) as u64); r15 = ((P0) as u64) as u64; rbx = ((P1) as u64) as u64; bb = 2; continue; }
            2 => { rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d99a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d99c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; bb = 6; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rax = (((xmm0 >> 64) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 35; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = (0xffffffffffffffffu64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rax = ((xmm0 as u64)) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x3u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 7; continue; }
            7 => { rcx = (r15) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (r10) as u64 as u64; fl_r = (r15) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_sub(1)) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 32; continue; } bb = 10; continue; }
            10 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 32; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9a68); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rax) as u64; rax = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r13)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9a87); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(r13.wrapping_add(0xcu64))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 31; continue; } bb = 13; continue; }
            13 => { rcx = ((r12 as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((rcx.wrapping_mul(4)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((r15.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9abb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; r8 = (rdi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9acb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = ((r11 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; r14 = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x80u64)))) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 18; continue; }
            18 => { r8 = ((r14.wrapping_add(0x8u64)) as u64) as u64; rsi = ((r9.wrapping_add(0x4u64)) as u64) as u64; rax = ((rbx.wrapping_sub(0x4u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; rdi = ((rax.wrapping_mul(4)) as u32) as u32 as u64; r10 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 19; continue; }
            19 => { rcx = ((crate::simd_ops::ru16(rsi.wrapping_sub(0x4u64))) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x8u64), ((rdx as u32)) as u32); rcx = ((crate::simd_ops::ru16(rsi.wrapping_sub(0x2u64))) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_sub(0x4u64), ((rdx as u32)) as u32); rcx = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(10u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(23u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x38000000u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x80000000u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); rdx = ((crate::simd_ops::ru16(rsi.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(10u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1fu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x70u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(10u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x80000000u32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((rcx as u32)) as u32); rsi = ((rsi.wrapping_add(0x8u64)) as u64) as u64; r8 = ((r8.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 24; continue; } bb = 21; continue; }
            21 => { bb = 26; continue; }
            22 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { r8 = ((r14.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r9 = ((r9.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; r10 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 25; continue; }
            25 => { rdx = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(10u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1fu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x70u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(10u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(13u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(16u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x80000000u32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; crate::simd_ops::wu32(r8, ((rcx as u32)) as u32); r9 = ((r9.wrapping_add(0x2u64)) as u64) as u64; r8 = ((r8.wrapping_add(0x4u64)) as u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 28; continue; }
            28 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9c76); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = ((P0) as u64) as u64; bb = 31; continue; }
            31 => { r9 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r13)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9c9d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((P1) as u64) as u64; bb = 32; continue; }
            32 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9cac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = ((rsi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (rsi) as u64); rdi = ((rdi).wrapping_add(0x10cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), (rdi) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 34; continue; }
            34 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9cef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423d9d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x140f35a30; const P2: u64 = 0x14340e528; const P3: u64 = 0x1401df980; const P4: u64 = 0x142491120; const P5: u64 = 0x14248fde0; const P6: u64 = 0x140f35bf0; const P7: u64 = 0x141834fc0; const P8: u64 = 0x1422a3490; const P9: u64 = 0x141834ee8; const P10: u64 = 0x14226a490; const P11: u64 = 0x142487880; const P12: u64 = 0x1425371a0; const P13: u64 = 0x142533200; const P14: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423d9d20, funcinfo_rva: 0x37d9924, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x178, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x50u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x150u64)) as u64; r14 = (r8) as u64; rbx = ((rdx.wrapping_add(0x4u64)) as u64) as u64; rcx = ((P0) as u64) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r13 = ((P1) as u64) as u64; r12 = ((P2) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rsi = ((crate::simd_ops::ru8(rbx.wrapping_add(0x108u64))) as u32) as u32 as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9d86); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9d9a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9daa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9dbe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r13) as u64); r9 = ((P6) as u64) as u64; rdx = (0x28u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x24u64)) as u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9de5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9df5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (r13) as u64; rdx = (0x28u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x24u64)) as u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9e0b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9e41); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 11; continue; }
            11 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9e7e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9f00); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9f12); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { rax = (r15) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r15) as u64); bb = 22; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 22; continue; }
            22 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { r10 = ((rax.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9f9d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            27 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9faf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423d9fb8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x110u64)) as u64; fl_l = (r14) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = ((P0) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 31; continue; }
            31 => { r11 = ((rsp.wrapping_add(0x150u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 32; continue; }
            32 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x14340e528; const P2: u64 = 0x1401df980; const P3: u64 = 0x142491120; const P4: u64 = 0x14248fde0; const P5: u64 = 0x14226a490; const P6: u64 = 0x142487880; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200; const P9: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423da020, funcinfo_rva: 0x37d997c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = (r8) as u64; rbx = ((rdx.wrapping_add(0x4u64)) as u64) as u64; r12 = ((P0) as u64) as u64; r15 = ((P1) as u64) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)))) as u64; rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da07e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da090); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da0a0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da0b2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (r15) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (r14) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da0db); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 9; continue; }
            9 => { rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da116); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (r15) as u64); fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da194); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da1a6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rax = (r14) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (r14) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), (0x0u8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (r14) as u64); bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 20; continue; }
            20 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { r10 = ((rax.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da22b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            25 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            26 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da23d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da246); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x10cu64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 29; continue; }
            29 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 30; continue; }
            30 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x142299ad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14);
    rdi = r8;
    r14 = P0;
    rsi = rdx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x60; const P2: u64 = 0x1412f1b40; const P3: u64 = 0x14384170c; const P4: u64 = 0x1403b8050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32) as u128; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P3, ((xmm6 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x70u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x60; const P2: u64 = 0x1412f1b40; const P3: u64 = 0x14384170c; const P4: u64 = 0x1403b8050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    rdi = r8;
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    loop { xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32) as u128; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P3, ((xmm6 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x70u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00170; const P1: u64 = 0xfffffffffffffffc; const P2: u64 = 0xc; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x20; const P5: u64 = 0x143a00178; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0xfffffffffffffff0; const P9: u64 = 0x4; const P10: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp, (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(P0)) as u64) as u128; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P1)) as u64) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; r8 = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = ((rax).wrapping_add(1)) as u64; rax = ((fl_l as u64)) as u64; r9 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32) as u128; r8 = ((r8.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(P5, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32) as u128; crate::simd_ops::wu32(P5, ((xmm1 as u32)) as u32); xmm5 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P7))) as u32) as u128; crate::simd_ops::wu32(P5, ((xmm0 as u32)) as u32); xmm4 = ((crate::simd_ops::ru32(r8.wrapping_add(P8))) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm5 as u64) + f64::from_bits(xmm6 as u64)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm3 as u64)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm4 as u32)) as f64).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); crate::simd_ops::wu32(P5, ((xmm4 as u32)) as u32); fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P0, ((xmm6 as u64)) as u64); bb = 4; continue; }
            4 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rcx.wrapping_add(P9)) as u64) as u64; fl_l = rdx; fl_r = r9; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            6 => { xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((rax.wrapping_add(P10)) as u64) as u64; xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu32(P5, ((xmm1 as u32)) as u32); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(P0, ((xmm6 as u64)) as u64); bb = 8; continue; }
            8 => { xmm6 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00170; const P1: u64 = 0xfffffffffffffffc; const P2: u64 = 0xc; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x20; const P5: u64 = 0x143a00178; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0xfffffffffffffff0; const P9: u64 = 0x4; const P10: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp, (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(P0)) as u64) as u128; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_add(P1)) as u64) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; rcx = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = ((rax).wrapping_add(1)) as u64; rax = ((fl_l as u64)) as u64; r9 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(P5, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32) as u128; crate::simd_ops::wu32(P5, ((xmm1 as u32)) as u32); xmm5 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32) as u128; crate::simd_ops::wu32(P5, ((xmm0 as u32)) as u32); xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm0 as u32)) as f64).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm5 as u64) + f64::from_bits(xmm6 as u64)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm3 as u64)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = (xmm4 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm4 as u32)) as f64).to_bits() as u128); xmm6 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); crate::simd_ops::wu32(P5, ((xmm4 as u32)) as u32); fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P0, ((xmm6 as u64)) as u64); bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rdx.wrapping_add(P9)) as u64) as u64; fl_l = r8; fl_r = r9; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; bb = 6; continue; }
            6 => { xmm1 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((rax.wrapping_add(P10)) as u64) as u64; xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); crate::simd_ops::wu32(P5, ((xmm1 as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(P0, ((xmm6 as u64)) as u64); bb = 8; continue; }
            8 => { xmm6 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x8; const P2: u64 = 0x143a35918; const P3: u64 = 0x143a00180; const P4: u64 = 0x5b8; const P5: u64 = 0x10; const P6: u64 = 0x1426455a8; const P7: u64 = 0x18; const P8: u64 = 0x142645598; const P9: u64 = 0x142645590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rbp = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rdi = rcx; r14 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(P2, rax); crate::simd_ops::wu32(P3, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx, rax); rax = (tmp_xchg) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rsi.wrapping_add(P7), (1) as u32); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x8; const P2: u64 = 0x143a35918; const P3: u64 = 0x143a00180; const P4: u64 = 0x5b8; const P5: u64 = 0x10; const P6: u64 = 0x1426455a8; const P7: u64 = 0x18; const P8: u64 = 0x142645598; const P9: u64 = 0x142645590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rbp = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rdi = rdx; r14 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(P2, rax); crate::simd_ops::wu32(P3, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx, rax); rax = (tmp_xchg) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rsi.wrapping_add(P7), (1) as u32); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x143a00160;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(P2, rcx); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x143a00160;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(P2, rcx); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x4; const P2: u64 = 0x14; const P3: u64 = 0x48c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r9 = P0;
    loop { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; rcx = ((rcx.wrapping_add(P2)) as u64) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x4; const P2: u64 = 0x14; const P3: u64 = 0x48c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r9 = P0;
    loop { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; rdx = ((rdx.wrapping_add(P2)) as u64) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423da8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14207a9c0; const P2: u64 = 0x143a33c28; const P3: u64 = 0x143a33c2c; const P4: u64 = 0x143a33958; const P5: u64 = 0x143a3395c; const P6: u64 = 0x143a00134; const P7: u64 = 0x143a339e8; const P8: u64 = 0x143a339ec; const P9: u64 = 0x143a33a68; const P10: u64 = 0x143a33a6c; const P11: u64 = 0x14397d8d7; const P12: u64 = 0x14397ff28; const P13: u64 = 0x143a28cc7; const P14: u64 = 0x143a33b98; const P15: u64 = 0x143a33b9c; const P16: u64 = 0x14397d863; const P17: u64 = 0x143a33b08; const P18: u64 = 0x143a33b0c; const P19: u64 = 0x143a28d35; const P20: u64 = 0x143bbd1b8; const P21: u64 = 0x1422fde70; const P22: u64 = 0x1422577e0; const P23: u64 = 0x143cf4bec; const P24: u64 = 0x143cf4560; const P25: u64 = 0x14397fcb8; const P26: u64 = 0x14183acbc; const P27: u64 = 0x143cf4570; const P28: u64 = 0x143cf4568; const P29: u64 = 0x141834a78; const P30: u64 = 0x143cf4558; const P31: u64 = 0x1433f4860; const P32: u64 = 0x14220ea70; const P33: u64 = 0x143cf4564; const P34: u64 = 0x14262d340; const P35: u64 = 0x141834d18; const P36: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423da8e0, funcinfo_rva: 0x37d9b08, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rcx = ((rdx.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rcx) as u64); r12 = ((P0) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x18u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x3bd6048u64))) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_sub(0x8u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_sub(0xcu64))) as u32 as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_sub(0x10u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rcx.wrapping_sub(0x14u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; r9 = (r9 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423da952); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x44u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x41u64))) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = (0x1u32) as u32 as u64; bb = 7; continue; }
            6 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (0x2u32) as u32 as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            13 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            14 => { crate::simd_ops::wu32(P7, ((r13 as u32)) as u32); crate::simd_ops::wu32(P8, ((rbp as u32)) as u32); crate::simd_ops::wu32(P2, ((r12 as u32)) as u32); crate::simd_ops::wu32(P3, ((r15 as u32)) as u32); crate::simd_ops::wu32(P6, ((rbx as u32)) as u32); crate::simd_ops::wu32(P4, ((rsi as u32)) as u32); crate::simd_ops::wu32(P5, ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x67cu64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x678u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x94u64), ((rcx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P9)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P10)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            17 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            18 => { rbp = (rbp & !0xffu64) | ((((((rbp as u8)) | ((rdx as u8)))) as u8 as u64)); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(P9, (rax) as u64); rcx = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; crate::simd_ops::wu8(P11, (if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x694u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P12)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x654u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x5c0u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | ((rcx as u8)))) as u8 as u64)); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) | (crate::simd_ops::ru8(P13)))) as u8 as u64)); rcx = (crate::simd_ops::ru64(P14)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(P15)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 22; continue; }
            21 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            22 => { fl_l = ((((rdx as u8)) | ((r8 as u8)))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(P13, ((rdx as u8)) as u8); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(P14, (rcx) as u64); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(P16)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (if (((fl_l as u8)) != ((fl_r as u8))) { rcx } else { rax }) as u64; rcx = (rax) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P17)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P18)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(P17, (rax) as u64); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { r12 = ((P0) as u64) as u64; bb = 28; continue; }
            28 => { crate::simd_ops::lk_or_u8(P19, ((rbp as u8)) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rcx) as u64); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64 as u64; fl_r = (0x1u64) as u64 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 30; continue; }
            30 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(P17, (rax) as u64); bb = 32; continue; }
            32 => { rcx = ((P20) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dab1e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dab2c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            34 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; r14 = (crate::simd_ops::ru64(rax)) as u64; rax = (0x74u32) as u32 as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(r14))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P23)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 48; continue; } bb = 35; continue; }
            35 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); rdi = (((crate::simd_ops::ru32(P24)) as i32 as i64) as u64) as u64; r12 = ((P0) as u64) as u64; rsi = (crate::simd_ops::ru32(r12.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x3858590u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(P25)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; bb = 40; continue; }
            39 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdi = ((rdi).wrapping_add(rdi)) as u64; rax = (((crate::simd_ops::ru32(r12.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x3a637b0u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 44; continue; } bb = 41; continue; }
            41 => { r8 = (((crate::simd_ops::ru32(r12.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x3a637b8u64))) as i32 as i64) as u64) as u64; rcx = (0x63d0u32) as u32 as u64; rax = ((rax).wrapping_add(r14)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dabbc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (((crate::simd_ops::ru32(r12.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x3a637bcu64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rdx = (rax) as u64; rcx = (0x463e0u32) as u32 as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((r15 as u32)) as u32); bb = 44; continue; }
            44 => { crate::simd_ops::wu64(P27, (r15) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(P28, (rax) as u64); bb = 28; continue; }
            47 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(P28, (rax) as u64); bb = 28; continue; }
            48 => { rcx = ((P23) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dac27); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P29) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P23)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 49; continue; }
            49 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P30, (r15) as u64); rax = ((P31) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x16u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P24) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dac70); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P32) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(P33, (0x9u32) as u32); rax = ((P28) as u64) as u64; crate::simd_ops::wu64(P30, (rax) as u64); crate::simd_ops::wu64(P28, (r15) as u64); crate::simd_ops::wu64(P27, (r15) as u64); rcx = ((P34) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423daca2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P35) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P23) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dacaf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P36) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 36; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dace0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6048; const P1: u64 = 0x142057270; const P2: u64 = 0x142056f90; const P3: u64 = 0x143a631e8; const P4: u64 = 0x1401de830; const P5: u64 = 0x142205dd0; const P6: u64 = 0x1412df250; const P7: u64 = 0x1412eb160; const P8: u64 = 0x142442de0; const P9: u64 = 0x1413522d0; const P10: u64 = 0x143a082f8; const P11: u64 = 0x142485780; const P12: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423dace0, funcinfo_rva: 0x37d9b44, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = (r8) as u64; r13 = (rdx) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((P0) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(r14.wrapping_mul(4)))) as u32 as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dad29); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dad2e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dad3b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = (crate::simd_ops::ru64(r15.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbp.wrapping_add(0xb8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dad6b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dad8e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rbp.wrapping_add(0x90u64), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dadc2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dadca); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xa0u64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0xb0u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dae03); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0xb0u64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x1f0u64))) as u64; rdx = ((rbx.wrapping_add(0x28u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(P10)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dae29); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x288u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dae43); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rbx.wrapping_add(0x288u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0xf8u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dae7a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (rsi) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423daea4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (rsi) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423daece); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (rsi) as u64); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x1f8u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x1f8u64), (rsi) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423daeea); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 25; continue; }
            25 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r12) as u64 as u64; rcx = ((P0) as u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 27; continue; }
            27 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423daf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6040; const P1: u64 = 0x142417470; const P2: u64 = 0x143bde060; const P3: u64 = 0x143bde064; const P4: u64 = 0x143bde050; const P5: u64 = 0x1412ebcc0; const P6: u64 = 0x143bde05c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P6, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P6, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P2, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bd6040; const P1: u64 = 0x142417470; const P2: u64 = 0x143bde060; const P3: u64 = 0x143bde064; const P4: u64 = 0x143bde050; const P5: u64 = 0x1412ebcc0; const P6: u64 = 0x143bde05c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r14 = rdx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P6, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P6, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P2, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x6c; const P1: u64 = 0x143bd6048; const P2: u64 = 0xffffffffffffff94; const P3: u64 = 0xffffffffffffff9c; const P4: u64 = 0x14229f840; const P5: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi);
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rsi = P1;
    loop { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff60; const P1: u64 = 0x14241d6f0; const P2: u64 = 0x141308530;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rsi = r8; rbp = (crate::simd_ops::ru64(P0)) as u64; rax = rsp.wrapping_add(0xc8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = ((rbp).wrapping_add(0x250u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; rbx = rdx.wrapping_add(0x28); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); r14 = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r14 as u32)) as u32); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x4cu64), ((xmm1 as u64)) as u64); r15 = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((r15 as u32)) as u32); xmm2 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), ((xmm2 as u64)) as u64); r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r12 as u32)) as u32); xmm3 = ((crate::simd_ops::ru64(rbx.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x64u64), ((xmm3 as u64)) as u64); r13 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((((rax) as i64).wrapping_shr(6u32) as u64)) as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = ((((rcx) as i64).wrapping_shr(6u32) as u64)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64) as u128; xmm2 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64) as u128; xmm3 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x1cu64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x24u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x30u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x34u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x3cu64), ((r13 as u32)) as u32); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x8u64), (1) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), (0x0u64) as u64); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 22; continue; }
            22 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffdfu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffdfu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffbfu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x40u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffbfu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x40u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff7u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x8u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff7u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x8u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffbu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffbu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423db8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x74u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dba00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dbb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dbc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0; const P1: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; r15 = P0; r11 = r8; rdi = rdx; xmm1 = 0u128; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp, ((xmm1 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rbx = r8.wrapping_sub(0x4); crate::simd_ops::wu32(rax.wrapping_sub(0x30u64), ((xmm1 as u32)) as u32); rbp = (crate::simd_ops::ru32(rax.wrapping_sub(0x30u64))) as u32 as u64; rsi = rdx.wrapping_add(0x8); rbx = ((rbx).wrapping_shr(2u32)) as u64; fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); crate::simd_ops::wu32(rax.wrapping_sub(0x34u64), ((xmm1 as u32)) as u32); xmm2 = ((crate::simd_ops::ru64(rsp)) as u64) as u128; crate::simd_ops::wu32(rax.wrapping_sub(0x2cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x28u64), ((xmm1 as u32)) as u32); xmm3 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x2cu64))) as u64) as u128; crate::simd_ops::wu32(rax.wrapping_sub(0x24u64), ((xmm1 as u32)) as u32); r14 = (crate::simd_ops::ru32(rax.wrapping_sub(0x24u64))) as u32 as u64; r10 = ((rbx.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0xa0u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xa8u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(0xacu64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xb4u64), ((rbp as u32)) as u32); crate::simd_ops::lk_and_u32(r9.wrapping_add(0x58u64), (0xffffffefu32) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0xa0u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xa8u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(0xacu64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xb4u64), ((rbp as u32)) as u32); crate::simd_ops::lk_and_u32(r9.wrapping_add(0x58u64), (0xffffffefu32) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0xa0u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xa8u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(0xacu64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xb4u64), ((rbp as u32)) as u32); crate::simd_ops::lk_and_u32(r9.wrapping_add(0x58u64), (0xffffffefu32) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0xa0u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xa8u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(0xacu64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xb4u64), ((rbp as u32)) as u32); crate::simd_ops::lk_and_u32(r9.wrapping_add(0x58u64), (0xffffffefu32) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rsi = rsi.wrapping_add(0x10); r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r10; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm2 = ((crate::simd_ops::ru64(rsp)) as u64) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x14u64), ((xmm1 as u32)) as u32); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64) as u128; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r10.wrapping_mul(4)))) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0xa0u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(r9.wrapping_add(0xacu64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0xa8u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0xb4u64), ((rbx as u32)) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::lk_and_u32(r9.wrapping_add(0x58u64), (0xffffffefu32) as u32); rcx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = r10; fl_r = r11; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dbf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143bc5fe8; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0xfffffffffffffff4; const P4: u64 = 0x1c; const P5: u64 = 0xa0; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xa8; const P8: u64 = 0xffffffffffffffe4; const P9: u64 = 0xac; const P10: u64 = 0xffffffffffffffec; const P11: u64 = 0xb4; const P12: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rax = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = P1; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64) as u128; rax = ((rax.wrapping_add(P4)) as u64) as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r10 = ((r10).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r8.wrapping_mul(8))))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P7), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(P9), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P11), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(r10)) as u32 as u64; crate::simd_ops::lk_or_u32(r10.wrapping_add(0x58u64), (0x10u32) as u32); rcx = ((r9 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r9) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8, rcx); crate::simd_ops::wu8(r10.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143bc5fe8; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0xfffffffffffffff4; const P4: u64 = 0x1c; const P5: u64 = 0xa0; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xa8; const P8: u64 = 0xffffffffffffffe4; const P9: u64 = 0xac; const P10: u64 = 0xffffffffffffffec; const P11: u64 = 0xb4; const P12: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rax = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = P1; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64) as u128; rax = ((rax.wrapping_add(P4)) as u64) as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r10 = ((r10).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8))))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P7), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(P9), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P11), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(r10)) as u32 as u64; crate::simd_ops::lk_or_u32(r10.wrapping_add(0x58u64), (0x10u32) as u32); rcx = ((r9 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r9) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8, rcx); crate::simd_ops::wu8(r10.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bc5fe8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x1403b3290; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = P1; rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(0xc8u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bc5fe8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x1403b3290; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = P1; rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(0xc8u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x1u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x58; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x1u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143bc5fe8; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x40; const P5: u64 = 0x10; const P6: u64 = 0xffffffffffffffb0; const P7: u64 = 0x18; const P8: u64 = 0xffffffffffffffb4; const P9: u64 = 0x1c; const P10: u64 = 0xffffffffffffffbc; const P11: u64 = 0x24; const P12: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rax = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = P1; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64) as u128; rax = ((rax.wrapping_add(P4)) as u64) as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r10 = ((r10).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r8.wrapping_mul(8))))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P7), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(P9), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P11), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x30u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x34u64))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x3cu64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(r10)) as u32 as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r9) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8, rcx); crate::simd_ops::wu8(r10.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143bc5fe8; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x40; const P5: u64 = 0x10; const P6: u64 = 0xffffffffffffffb0; const P7: u64 = 0x18; const P8: u64 = 0xffffffffffffffb4; const P9: u64 = 0x1c; const P10: u64 = 0xffffffffffffffbc; const P11: u64 = 0x24; const P12: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rax = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = P1; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64) as u128; rax = ((rax.wrapping_add(P4)) as u64) as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r10 = ((r10).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8))))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P7), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(P9), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(P11), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x30u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x34u64))) as u64) as u128; crate::simd_ops::wu64(r10.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x3cu64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru32(r10)) as u32 as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r9) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8, rcx); crate::simd_ops::wu8(r10.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r8 = 0; fl_l = 0; fl_r = 0; rdi = P1; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64)))).to_bits() as u128); rbx = rbx.wrapping_add(0x20); rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r9 = 0; fl_l = 0; fl_r = 0; rdi = P1; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64)))).to_bits() as u128); rbx = rbx.wrapping_add(0x20); rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64)))).to_bits() as u128); rdx = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dc9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dcae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x6cu64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dcbf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dcd00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x64u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dce10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dcf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0; const P1: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); r10 = 0; fl_l = 0; fl_r = 0; rbp = P0; rbx = r8; r11 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsi = rdx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rdi = r8.wrapping_sub(0x4); rdi = ((rdi).wrapping_shr(2u32)) as u64; fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; r10 = ((rdi.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x8u64))) as u32 as u64; rsi = rsi.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x24u64))) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(r9.wrapping_add(0x5cu64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x1cu64))) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(r9.wrapping_add(0x5cu64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x14u64))) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(r9.wrapping_add(0x5cu64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0xcu64))) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(r9.wrapping_add(0x5cu64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r10; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r11.wrapping_add(r10.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r10 = ((r10).wrapping_add(1)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x5cu64), ((rax as u32)) as u32); rcx = ((r8 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); fl_l = r10; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x40; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x48; const P6: u64 = 0x143bbdb30; const P7: u64 = 0xb8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = rdx; r10 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rbx = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r10)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; xmm0 = ((crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64) as u128; r10 = ((r10.wrapping_add(P2)) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r8.wrapping_mul(8))))) as u64; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P5), ((rax as u32)) as u32); rax = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu8(r9.wrapping_add(P7), (0x0u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x40; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x48; const P6: u64 = 0x143bbdb30; const P7: u64 = 0xb8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = r8; r10 = rdx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rbx = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r10)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; xmm0 = ((crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64) as u128; r10 = ((r10.wrapping_add(P2)) as u64) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8))))) as u64; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P5), ((rax as u32)) as u32); rax = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu8(r9.wrapping_add(P7), (0x0u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); r10 = rcx.wrapping_add(0x38); rbx = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x38u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; xmm3 = ((crate::simd_ops::ru32(r10.wrapping_sub(0x1cu64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r10.wrapping_sub(0x20u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(r10.wrapping_sub(0x24u64))) as u32) as u128; r10 = r10.wrapping_add(0x50); r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_sub(0x84u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x58u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x80u64))) as u64) as u128; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x78u64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x48u64), ((rax as u32)) as u32); r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x68u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x60u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x5cu64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x54u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x50u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x48u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x44u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x3cu64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(P1)) as u64; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); crate::simd_ops::wu32(r9.wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x60u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x64u64), ((xmm3 as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x143bbdb30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); r10 = rdx.wrapping_add(0x38); rbx = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x38u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; xmm3 = ((crate::simd_ops::ru32(r10.wrapping_sub(0x1cu64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r10.wrapping_sub(0x20u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(r10.wrapping_sub(0x24u64))) as u32) as u128; r10 = r10.wrapping_add(0x50); r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_sub(0x84u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x58u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x80u64))) as u64) as u128; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x78u64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x48u64), ((rax as u32)) as u32); r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x68u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x60u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x5cu64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x54u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x50u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x48u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r10.wrapping_sub(0x44u64))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x3cu64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(P1)) as u64; r8 = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), rax); crate::simd_ops::wu8(r9.wrapping_add(0xb8u64), (0x0u8) as u8); crate::simd_ops::wu32(r9.wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x60u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x64u64), ((xmm3 as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0; const P1: u64 = 0x1422ea170; const P2: u64 = 0x142417470; const P3: u64 = 0x143bce000; const P4: u64 = 0x143bce004; const P5: u64 = 0x143bcdff0; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143bcdffc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423dd660, funcinfo_rva: 0x37da12c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r15) as u64); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dd6ad); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (0x1u32) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dd6f1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dd70f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe0; const P1: u64 = 0x1422ea170; const P2: u64 = 0x142417470; const P3: u64 = 0x143bce000; const P4: u64 = 0x143bce004; const P5: u64 = 0x143bcdff0; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143bcdffc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423dd780, funcinfo_rva: 0x37da164, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r8) as u64; r14 = (rdx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r15) as u64); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dd7dd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (0x1u32) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dd821); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dd83f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x1422ea2e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    rdi = rdx;
    rbp = P0;
    rsi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r8.wrapping_mul(8))))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bc5fe8; const P1: u64 = 0x1422ea2e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    rdi = r8;
    rbp = P0;
    rsi = rdx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; r9 = ((r9).wrapping_shr(8u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r9.wrapping_mul(8))))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dd9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c06168; const P1: u64 = 0x4; const P2: u64 = 0x14; const P3: u64 = 0x38;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r10 = P0;
    loop { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; r8 = ((r8).wrapping_shr(12u32)) as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(P1))) as u128; r9 = ((r9.wrapping_add(P2)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(r8.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), (xmm0) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dda00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143c06168; const P1: u64 = 0x4; const P2: u64 = 0x14; const P3: u64 = 0x38;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = r8;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r10 = P0;
    loop { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; r8 = ((r8).wrapping_shr(12u32)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; rdx = ((rdx.wrapping_add(P2)) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(r8.wrapping_mul(8)))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), (xmm0) as u128); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dda50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06160;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ddb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06160;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x34u64), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ddc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06168; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14020bcb0; const P4: u64 = 0x1c2; const P5: u64 = 0x1c3; const P6: u64 = 0x14205da30; const P7: u64 = 0x14205f070; const P8: u64 = 0x1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rsi = rcx; r14 = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 11; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rdi.wrapping_add(P8), (0x1u8) as u8); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 13; continue; }
            13 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ddd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06168; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14020bcb0; const P4: u64 = 0x1c2; const P5: u64 = 0x1c3; const P6: u64 = 0x14205da30; const P7: u64 = 0x14205f070; const P8: u64 = 0x1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = r8; rsi = rdx; r14 = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 11; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rdi.wrapping_add(P8), (0x1u8) as u8); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 13; continue; }
            13 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dde70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143c06168; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0x24; const P4: u64 = 0x8; const P5: u64 = 0xc; const P6: u64 = 0x1c; const P7: u64 = 0x1422f6c40; const P8: u64 = 0x14397ff80; const P9: u64 = 0x68; const P10: u64 = 0x6c; const P11: u64 = 0x60; const P12: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1cu64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x2cu64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) | (0x3u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffcu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rcx as u32)) as u32); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(P8)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rbx.wrapping_add(P11)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x68u64), (1) as u32); rdi = ((rdi).wrapping_add(0x2cu64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ddf70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143c06168; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0x24; const P4: u64 = 0x8; const P5: u64 = 0xc; const P6: u64 = 0x1c; const P7: u64 = 0x1422f6c40; const P8: u64 = 0x14397ff80; const P9: u64 = 0x68; const P10: u64 = 0x6c; const P11: u64 = 0x60; const P12: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rdi = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1cu64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x2cu64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) | (0x3u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffcu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rcx as u32)) as u32); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(P8)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rbx.wrapping_add(P11)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x68u64), (1) as u32); rdi = ((rdi).wrapping_add(0x2cu64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c06160; const P1: u64 = 0x14397ff80; const P2: u64 = 0x1422f6c40; const P3: u64 = 0x1401de830; const P4: u64 = 0x142417470; const P5: u64 = 0x143c0e180; const P6: u64 = 0x143c0e184; const P7: u64 = 0x143c0e170; const P8: u64 = 0x1412ebcc0; const P9: u64 = 0x143c0e17c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; r14 = rdx; rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rdi.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; r9 = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r11 = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((r10 as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((r11.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(0x68u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x68u64))) as u32 as u64; bb = 9; continue; }
            8 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rcx = ((r10 as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; r10 = ((rcx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; r8 = ((r11.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(r14.wrapping_add(rdi.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            16 => { rdx = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(P9)) as u32 as u64; rax = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(P9)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P9, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(P9, ((rbp as u32)) as u32); bb = 20; continue; }
            20 => { crate::simd_ops::lk_add_u32(P5, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 22; continue; }
            22 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427178; const P1: u64 = 0x143c06168; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x143159a38; const P6: u64 = 0x1c; const P7: u64 = 0x143159a40; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = 0; fl_l = 0; fl_r = 0; r9 = rdx; r10 = rcx; r8 = ((r11 as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(rsp, rbx); rbx = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8))))) as u64; crate::simd_ops::wu64(rdx, r11); crate::simd_ops::wu64(rdx.wrapping_add(P2), r11); crate::simd_ops::wu64(rdx.wrapping_add(P3), r11); crate::simd_ops::wu32(rdx.wrapping_add(P4), ((r11 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P6), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P9), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P10), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P11), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x44u64), (0x3f800000u64) as u64); fl_l = r8; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427178; const P1: u64 = 0x143c06168; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x143159a38; const P6: u64 = 0x1c; const P7: u64 = 0x143159a40; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = rdx; r9 = ((r11 as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(rsp, rbx); rbx = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = ((rcx).wrapping_shr(12u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8))))) as u64; crate::simd_ops::wu64(rdx, r11); crate::simd_ops::wu64(rdx.wrapping_add(P2), r11); crate::simd_ops::wu64(rdx.wrapping_add(P3), r11); crate::simd_ops::wu32(rdx.wrapping_add(P4), ((r11 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P6), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P9), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P10), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(P11), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x44u64), (0x3f800000u64) as u64); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x142303160; const P2: u64 = 0x142337590; const P3: u64 = 0x143a36c98;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); r15 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r13); r13 = 0x1; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r14); r14 = rdx.wrapping_add(0x1c); rdx = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_sub(0x1cu64))) as u32 as u64; r8 = ((crate::simd_ops::ru8(r14.wrapping_sub(0x4u64))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rsi = (((crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rbx = (crate::simd_ops::ru32(r14)) as u32 as u64; rdi = (crate::simd_ops::ru32(r14.wrapping_sub(0x18u64))) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_sub(0x14u64))) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_sub(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_sub(0xcu64))) as u32 as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); r8 = ((rdi as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rbp; r12 = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rsi = ((rsi).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru64(r10)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r9 = r12; r8 = ((rbx as u32)) as u32 as u64; rdx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(P3)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x2u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((r13).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 10; continue; }
            10 => { r14 = ((r14).wrapping_add(0x28u64)) as u64; rdx = P0; fl_l = r15; fl_r = r13; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            13 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(0x48u64))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; bb = 6; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x1433d6e80; const P2: u64 = 0x142303160; const P3: u64 = 0x143cf8b70; const P4: u64 = 0x143cf8b64; const P5: u64 = 0x141311210; const P6: u64 = 0x143a36c98; const P7: u64 = 0x141834a78; const P8: u64 = 0x14220ea70; const P9: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423de560, funcinfo_rva: 0x37da444, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r12 = (rdx) as u64; rdi = ((rcx.wrapping_add(0xcu64)) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (0x74u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0xcu64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r13 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8))))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = ((rdx as u32)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; r12 = ((P1) as u64) as u64; bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de5eb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r15))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de611); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x3u32) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru64(P6)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x1u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = (0x1u32) as u32 as u64; rdx = ((rax).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = (0x74u32) as u32 as u64; r8 = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(0x14u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r12) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de6c8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de700); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de70d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x1433d6e80; const P2: u64 = 0x142303160; const P3: u64 = 0x143cf8b70; const P4: u64 = 0x143cf8b64; const P5: u64 = 0x141311210; const P6: u64 = 0x143a36c98; const P7: u64 = 0x141834a78; const P8: u64 = 0x14220ea70; const P9: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423de720, funcinfo_rva: 0x37da480, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r12 = (r8) as u64; rdi = ((rdx.wrapping_add(0xcu64)) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (0x74u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0xcu64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r13 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8))))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = ((rdx as u32)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; r12 = ((P1) as u64) as u64; bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de7ab); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r15))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de7d1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x3u32) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru64(P6)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x1u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = (0x1u32) as u32 as u64; rdx = ((rax).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (0x74u32) as u32 as u64; r8 = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(0x14u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de888); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de8c0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423de8cd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x4; const P2: u64 = 0x142303ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    rsi = rdx;
    rbp = P0;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(P1))) as u32) as u128; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r8.wrapping_mul(8))))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x4; const P2: u64 = 0x142303ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    rsi = r8;
    rbp = P0;
    rdi = rdx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(P1))) as u32) as u128; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r8.wrapping_mul(8))))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423de9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x1433fa688; const P2: u64 = 0x142303160; const P3: u64 = 0x143cf8b74; const P4: u64 = 0x143cf8b6c; const P5: u64 = 0x141311210; const P6: u64 = 0x143a36c98; const P7: u64 = 0x141834a78; const P8: u64 = 0x14220ea70; const P9: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423de9e0, funcinfo_rva: 0x37da524, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); r12 = (rdx) as u64; rdx = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), (rbp) as u64); fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r9 = ((P0) as u64) as u64; r8 = (0x74u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(rbp.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r13 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(rcx.wrapping_mul(8))))) as u64; xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = ((r8 as u32)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; r12 = ((P1) as u64) as u64; rbp = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dea8b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r15))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423deab1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x0u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((xmm6 as u32)) as u32); r9 = (crate::simd_ops::ru64(P6)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x1u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rbp).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = (0x74u32) as u32 as u64; r9 = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), (rbp) as u64); fl_l = (rbp) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423deb5c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423deb94); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423deba1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423debc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x1433fa688; const P2: u64 = 0x142303160; const P3: u64 = 0x143cf8b74; const P4: u64 = 0x143cf8b6c; const P5: u64 = 0x141311210; const P6: u64 = 0x143a36c98; const P7: u64 = 0x141834a78; const P8: u64 = 0x14220ea70; const P9: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423debc0, funcinfo_rva: 0x37da560, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); r12 = (r8) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rbp) as u64); r9 = ((P0) as u64) as u64; r8 = (0x74u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(rbp.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r13 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(rcx.wrapping_mul(8))))) as u64; xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = ((r8 as u32)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; r12 = ((P1) as u64) as u64; rbp = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dec6b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r15))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dec91); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x0u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((xmm6 as u32)) as u32); r9 = (crate::simd_ops::ru64(P6)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x1u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rbp).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r8 = (0x74u32) as u32 as u64; r9 = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rbp) as u64); fl_l = (rbp) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423ded3c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423ded74); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423ded81); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423deda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x1433fa670; const P2: u64 = 0x142303160; const P3: u64 = 0x143cf8b68; const P4: u64 = 0x143cf8b7c; const P5: u64 = 0x141311210; const P6: u64 = 0x143a36c98; const P7: u64 = 0x141834a78; const P8: u64 = 0x14220ea70; const P9: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423deda0, funcinfo_rva: 0x37da59c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r12 = (rdx) as u64; rdi = ((rcx.wrapping_add(0xcu64)) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (0x74u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0xcu64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r13 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8))))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = ((rdx as u32)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; r12 = ((P1) as u64) as u64; bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dee2b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r15))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423dee51); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x2u32) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru64(P6)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x1u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = (0x1u32) as u32 as u64; rdx = ((rax).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = (0x74u32) as u32 as u64; r8 = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r12) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423def02); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423def3a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423def47); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423def60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x1433fa670; const P2: u64 = 0x142303160; const P3: u64 = 0x143cf8b68; const P4: u64 = 0x143cf8b7c; const P5: u64 = 0x141311210; const P6: u64 = 0x143a36c98; const P7: u64 = 0x141834a78; const P8: u64 = 0x14220ea70; const P9: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423def60, funcinfo_rva: 0x37da5d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r12 = (r8) as u64; rdi = ((rdx.wrapping_add(0xcu64)) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (0x74u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0xcu64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r13 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8))))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = ((rdx as u32)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; r12 = ((P1) as u64) as u64; bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423defeb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r15))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df011); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax, (0x2u32) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru64(P6)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x38u64), (0x1u8) as u8); rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rax = (0x1u32) as u32 as u64; rdx = ((rax).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x78u64), (rdx) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x138u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (0x74u32) as u32 as u64; r8 = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df0c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df0fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df107); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423df120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143be60a8; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0x142303160; const P4: u64 = 0x141311300; const P5: u64 = 0x143a36c98; const P6: u64 = 0x8; const P7: u64 = 0x20; const P8: u64 = 0x38; const P9: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); r14 = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rsi = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32) as u128; rdx = ((rdx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rdx.wrapping_mul(8))))) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), ((xmm6 as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(P8), ((rsi as u8)) as u8); rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rsi).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P9), (rdx) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0xcu64)) as u64; fl_l = rdi; fl_r = rsi; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423df200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143be60a8; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0x142303160; const P4: u64 = 0x141311300; const P5: u64 = 0x143a36c98; const P6: u64 = 0x8; const P7: u64 = 0x20; const P8: u64 = 0x38; const P9: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbx = ((rdx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); r14 = P1; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rsi = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32) as u128; rdx = ((rdx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rdx.wrapping_mul(8))))) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax, ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), ((xmm6 as u32)) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((crate::simd_ops::ru16(rax.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(P8), ((rsi as u8)) as u8); rcx = ((r8 as u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rsi).wrapping_shl((((rcx) as u32) & 63))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; fl_l = (crate::simd_ops::lk_or_u64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P9), (rdx) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0xcu64)) as u64; fl_l = rdi; fl_r = rsi; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423df300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x14397fed0; const P2: u64 = 0x14222ec60; const P3: u64 = 0x14397ff48; const P4: u64 = 0x14397fed8; const P5: u64 = 0x14237e0e0; const P6: u64 = 0x14237fff0; const P7: u64 = 0x1401de830; const P8: u64 = 0x14313f720; const P9: u64 = 0x142645890; const P10: u64 = 0x1423df477;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423df300, funcinfo_rva: 0x37da68c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = (r8) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((rdx.wrapping_add(0x8u64)) as u64) as u64; r12 = ((P0) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x50u64))) as u32 as u64; r14 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x120u64)))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df378); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x98u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; r8 = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x140u64))) as u64; rax = (crate::simd_ops::ru64(P4)) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 8; continue; }
            7 => { rdi = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x90u64))) as u64; rdx = (rbx) as u64; rcx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df408); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), (0xffffffffu32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; rcx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df42c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df435); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = ((rbp).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423df471); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423df480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x14397fed0; const P2: u64 = 0x14222ec60; const P3: u64 = 0x14397ff48; const P4: u64 = 0x14397fed8; const P5: u64 = 0x14237e8f0; const P6: u64 = 0x14313f720; const P7: u64 = 0x142645890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); r14 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x50u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x120u64)))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = ((rbp as u32)) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x1a8u64)))) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x140u64))) as u64; rax = (crate::simd_ops::ru64(P4)) as u64; r9 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 8; continue; }
            7 => { rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = r9.wrapping_add(0x8); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 18; continue; }
            18 => { return rax; }
            19 => { rcx = P6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1423df60e, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423df620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x14397fed0; const P2: u64 = 0x14222ec60; const P3: u64 = 0x14397ff48; const P4: u64 = 0x14397fed8; const P5: u64 = 0x14237e8f0; const P6: u64 = 0x14313f720; const P7: u64 = 0x142645890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); r14 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = rdx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x50u64))) as u32 as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P1)) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x120u64)))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = ((rbp as u32)) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x1a8u64)))) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x140u64))) as u64; rax = (crate::simd_ops::ru64(P4)) as u64; r9 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 8; continue; }
            7 => { rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = r9.wrapping_add(0x8); rcx = ((rcx).wrapping_add(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 18; continue; }
            18 => { return rax; }
            19 => { rcx = P6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1423df7ae, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423df8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x14397fed0; const P2: u64 = 0x14397ff48; const P3: u64 = 0x50; const P4: u64 = 0x48; const P5: u64 = 0x120; const P6: u64 = 0x14222ec60; const P7: u64 = 0x1a8; const P8: u64 = 0x14222fb60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rbp = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); r12 = P0; r13 = (crate::simd_ops::ru64(P1)) as u64; r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdx = ((rsi as u32)) as u32 as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(P5)))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rsi as u32)) as u32 as u64; rcx = rdi; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(P7)))) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dfb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x14397fed0; const P2: u64 = 0x14222ec60; const P3: u64 = 0x14397ff48; const P4: u64 = 0x14222f750; const P5: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); r15 = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r13); r13 = P0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r14); r14 = rdx.wrapping_add(0x10); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_sub(0x10u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; r12 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r12 = ((r12).wrapping_shl(5u32)) as u64; r12 = ((r12).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x120u64)))) as u64; rcx = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rax = ((rdi as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x98u64))) as u32 as u64; rcx = r12; rdi = (crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(P3)) as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x1a8u64)))) as u64; rbx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rbx; rax = ((rax as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x5b8u64)))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x680u64))) as u64; r8 = ((rsi as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx, rax); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { r14 = ((r14).wrapping_add(0x18u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dfc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0x50; const P4: u64 = 0x48; const P5: u64 = 0x14397fed0; const P6: u64 = 0x120; const P7: u64 = 0x14222ec60; const P8: u64 = 0xfffffffffffffff8; const P9: u64 = 0x14222f950; const P10: u64 = 0x8; const P11: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rbx = ((rdx.wrapping_add(P3)) as u64) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P5)) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(P6)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P10), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P11), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dfd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0x50; const P4: u64 = 0x48; const P5: u64 = 0x14397fed0; const P6: u64 = 0x120; const P7: u64 = 0x14222ec60; const P8: u64 = 0xfffffffffffffff8; const P9: u64 = 0x14222f950; const P10: u64 = 0x8; const P11: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = ((rdx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rbx = ((rdx.wrapping_add(P3)) as u64) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P5)) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(P6)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((rdi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P10), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P11), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dfef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x142302e60; const P5: u64 = 0x8; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = ((rdx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r8.wrapping_mul(8))))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423dffa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x4; const P2: u64 = 0x3be60a8; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x3bee0d8; const P6: u64 = 0x142305d50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbp = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r14 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rsi = 0; fl_l = 0; fl_r = 0; r15 = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rsi.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(16u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P2)))) as u64; rdx = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5)))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 10; continue; }
            7 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff0u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P3)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x4; const P2: u64 = 0x3be60a8; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x3bee0d8; const P6: u64 = 0x142305d50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbp = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r14 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rsi = 0; fl_l = 0; fl_r = 0; r15 = P0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rsi.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2)))) as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 10; continue; }
            7 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff0u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P3)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x143be60a8; const P2: u64 = 0x142302fe0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi);
    r11 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = P1;
    rbx = rdx;
    loop { rax = (crate::simd_ops::ru32(r11.wrapping_sub(0x14u64))) as u32 as u64; r9 = r11.wrapping_add(0xc); xmm2 = ((crate::simd_ops::ru32(r11.wrapping_sub(0x4u64))) as u32) as u128; r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rax = (((rax as u16)) as u32) as u32 as u64; rdx = r11.wrapping_sub(0x10); rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; r9 = r11; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(r8.wrapping_mul(8))))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = ((r11).wrapping_add(0x2cu64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e01f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x143be60a8; const P2: u64 = 0x142302fe0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi);
    r11 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rdi = P1;
    rbx = r8;
    loop { rax = (crate::simd_ops::ru32(r11.wrapping_sub(0x14u64))) as u32 as u64; r9 = r11.wrapping_add(0xc); xmm2 = ((crate::simd_ops::ru32(r11.wrapping_sub(0x4u64))) as u32) as u128; r8 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rax = (((rax as u16)) as u32) as u32 as u64; rdx = r11.wrapping_sub(0x10); rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; r9 = r11; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(r8.wrapping_mul(8))))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = ((r11).wrapping_add(0x2cu64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x4f; const P2: u64 = 0x48; const P3: u64 = 0x14397ff48; const P4: u64 = 0x218; const P5: u64 = 0x10; const P6: u64 = 0x141311700; const P7: u64 = 0x8; const P8: u64 = 0x142304c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdi = ((r14 as u32)) as u32 as u64; r15 = P0; rsi = rdx; rbp = rcx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(rdi.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_shr(16u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rdx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P1), ((rdx as u8)) as u8); rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P4)))) as u64; crate::simd_ops::wu8(rax, ((rdx as u8)) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), r14); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x4f; const P2: u64 = 0x48; const P3: u64 = 0x14397ff48; const P4: u64 = 0x218; const P5: u64 = 0x10; const P6: u64 = 0x141311700; const P7: u64 = 0x8; const P8: u64 = 0x142304c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdi = ((r14 as u32)) as u32 as u64; r15 = P0; rsi = r8; rbp = rdx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(rdi.wrapping_mul(8)))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P1), ((rdx as u8)) as u8); rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P4)))) as u64; crate::simd_ops::wu8(rax, ((rdx as u8)) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), r14); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e03e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff48; const P1: u64 = 0x4; const P2: u64 = 0x143be60a8; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0xc; const P5: u64 = 0x48; const P6: u64 = 0xfffffffffffffff4; const P7: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x8u64)) as u64;
    r10 = rdx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
    r11 = (crate::simd_ops::ru64(P0)) as u64;
    rax = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp, rbx);
    rbx = P2;
    loop { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u16)) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r8.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x208u64))) as u64; r8 = ((rdx.wrapping_add(rdx)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x208u64))) as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P7), ((r9 as u8)) as u8); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp)) as u64;
    rsp = ((rsp).wrapping_add(0x8u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff48; const P1: u64 = 0x4; const P2: u64 = 0x143be60a8; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0xc; const P5: u64 = 0x48; const P6: u64 = 0xfffffffffffffff4; const P7: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x8u64)) as u64;
    r10 = r8;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
    r11 = (crate::simd_ops::ru64(P0)) as u64;
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rsp, rbx);
    rbx = P2;
    loop { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u16)) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r8.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x208u64))) as u64; r8 = ((rdx.wrapping_add(rdx)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x208u64))) as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P7), ((r9 as u8)) as u8); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp)) as u64;
    rsp = ((rsp).wrapping_add(0x8u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff48; const P1: u64 = 0x143be60a0; const P2: u64 = 0x14386c220;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); r11 = (crate::simd_ops::ru64(P0)) as u64; rsi = P1; r10 = 0; fl_l = 0; fl_r = 0; rbx = r8; rdi = rdx; xmm3 = 0u128; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(0xfffffffffffffffcu64)) as u64; r9 = rdx.wrapping_add(0x8); r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(1)) as u64; r8 = ((fl_l as u64)) as u64; r10 = ((r8.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r9.wrapping_sub(0x8u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_sub(0x4u64))) as u32) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x248u64))) as u32) as u128; xmm1 = crate::simd_ops::vcmpss(xmm3, (xmm3 as u32), (xmm2 as u32), 0x2u8); xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm1); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x1f8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r9)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x248u64))) as u32) as u128; xmm1 = crate::simd_ops::vcmpss(xmm3, (xmm3 as u32), (xmm2 as u32), 0x2u8); xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm1); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x1f8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as u32) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x248u64))) as u32) as u128; xmm1 = crate::simd_ops::vcmpss(xmm3, (xmm3 as u32), (xmm2 as u32), 0x2u8); xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm1); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x1f8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x248u64))) as u32) as u128; xmm1 = crate::simd_ops::vcmpss(xmm3, (xmm3 as u32), (xmm2 as u32), 0x2u8); xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm1); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; r9 = ((r9).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x1f8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { fl_l = r10; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r10.wrapping_mul(8)))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x248u64))) as u32) as u128; xmm1 = crate::simd_ops::vcmpss(xmm3, (xmm3 as u32), (xmm2 as u32), 0x2u8); xmm0 = crate::simd_ops::vblendvps(xmm0, xmm2, xmm1); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x1f8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((xmm0 as u32)) as u32); fl_l = r10; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x14397ff48; const P2: u64 = 0x143be60a8; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x40; const P5: u64 = 0x48; const P6: u64 = 0x650; const P7: u64 = 0xffffffffffffffa8; const P8: u64 = 0x5d8; const P9: u64 = 0xffffffffffffffb0; const P10: u64 = 0x8; const P11: u64 = 0xffffffffffffffb4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rax = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp, rdi); rdi = P2; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u16)) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(r8.wrapping_mul(8)))) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(P5))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; r9 = ((r10 as u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r9) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8, rcx); r8 = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64) as u128; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P8)))) as u64; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x34u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x24u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((rcx as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x14397ff48; const P2: u64 = 0x143be60a8; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x40; const P5: u64 = 0x48; const P6: u64 = 0x650; const P7: u64 = 0xffffffffffffffa8; const P8: u64 = 0x5d8; const P9: u64 = 0xffffffffffffffb0; const P10: u64 = 0x8; const P11: u64 = 0xffffffffffffffb4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r11 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rax = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp, rdi); rdi = P2; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u16)) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(r8.wrapping_mul(8)))) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(P5))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; r9 = ((r10 as u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) & !((1u64) << (((r9) as u32 & 63)))) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((r9) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8, rcx); r8 = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64) as u128; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P8)))) as u64; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P10), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x44u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x34u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x24u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((rcx as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143be60a8; const P1: u64 = 0x10; const P2: u64 = 0x142304820; const P3: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi);
    rdi = r8;
    rsi = P0;
    rbx = rdx;
    loop { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r8.wrapping_mul(8))))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e09e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143be60a8; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0x1423057e0; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi);
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rsi = P1;
    loop { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r8.wrapping_mul(8))))) as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143be60a8; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0x1423057e0; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi);
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rsi = P1;
    loop { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rdx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143be60a8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0xc0; const P4: u64 = 0x1403b3290; const P5: u64 = 0x142304c80; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e0ad0, funcinfo_rva: 0x37dac10, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = ((P1) as u64) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e0b32); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e0b44); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e0b6a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, (rbp) as u64); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143be60a8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0xc0; const P4: u64 = 0x1403b3290; const P5: u64 = 0x142304c80; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e0ba0, funcinfo_rva: 0x37dac44, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = ((P1) as u64) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(16u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e0c02); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e0c14); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e0c3a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, (rbp) as u64); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c0e190; const P1: u64 = 0x142326f60; const P2: u64 = 0x142417470; const P3: u64 = 0x143c161b0; const P4: u64 = 0x143c161b4; const P5: u64 = 0x143c161a0; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143c161ac;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x130u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c0e190; const P1: u64 = 0x142326f60; const P2: u64 = 0x142417470; const P3: u64 = 0x143c161b0; const P4: u64 = 0x143c161b4; const P5: u64 = 0x143c161a0; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143c161ac;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r14 = rdx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x130u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x94; const P1: u64 = 0x140000000; const P2: u64 = 0xffffffffffffff6c; const P3: u64 = 0xffffffffffffff70; const P4: u64 = 0x3bee0d8; const P5: u64 = 0xfffffffffffffffc; const P6: u64 = 0x3c0e198; const P7: u64 = 0x142327360; const P8: u64 = 0xffffffffffffff74; const P9: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = P1; bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4)))) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x130u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P6)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x9cu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e0f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x94; const P1: u64 = 0x140000000; const P2: u64 = 0xffffffffffffff6c; const P3: u64 = 0xffffffffffffff70; const P4: u64 = 0x3bee0d8; const P5: u64 = 0xfffffffffffffffc; const P6: u64 = 0x3c0e198; const P7: u64 = 0x142327360; const P8: u64 = 0xffffffffffffff74; const P9: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rdi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rbx = ((rdx.wrapping_add(P0)) as u64) as u64; rsi = P1; bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4)))) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x130u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P6)))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x9cu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bee0d0; const P1: u64 = 0x142324020; const P2: u64 = 0x142417470; const P3: u64 = 0x143bf60f0; const P4: u64 = 0x143bf60f4; const P5: u64 = 0x143bf60e0; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143bf60ec;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bee0d0; const P1: u64 = 0x142324020; const P2: u64 = 0x142417470; const P3: u64 = 0x143bf60f0; const P4: u64 = 0x143bf60f4; const P5: u64 = 0x143bf60e0; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143bf60ec;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r14 = rdx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bee0d8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x14231efe0; const P4: u64 = 0x50; const P5: u64 = 0x14240ab10; const P6: u64 = 0x1401ee280;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xe0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bee0d8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x14231efe0; const P4: u64 = 0x50; const P5: u64 = 0x14240ab10; const P6: u64 = 0x1401ee280;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; rsi = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xe0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8cu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e13d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rdi = r8;
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8cu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x143a000f0; const P2: u64 = 0x142345b20; const P3: u64 = 0x142347330; const P4: u64 = 0x142349990; const P5: u64 = 0x142347460; const P6: u64 = 0x142347250; const P7: u64 = 0x142347150; const P8: u64 = 0x142346c50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rbp = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rcx = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); r14 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rsi = (crate::simd_ops::ru64(P1)) as u64; rdx = ((rax as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8))))) as u64; rcx = rsi.wrapping_add(0x170); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc4u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xbcu64))) as u64; rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb4u64))) as u64; rcx = rsi.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x60); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xb4u64), rdx); rdx = rsp.wrapping_add(0x68); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xbcu64), rdx); rdx = rsp.wrapping_add(0x70); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0xffffffff; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xc4u64), rdx); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xb4u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xbcu64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb4u64))) as u64; rcx = rsi.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xbcu64))) as u64; rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc4u64))) as u64; rcx = rsi.wrapping_add(0x170); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0xb4u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xbcu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc4u64), (0xffffffffu32) as u32); bb = 7; continue; }
            7 => { rbp = ((rbp).wrapping_add(0x10u64)) as u64; rcx = P0; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 9; continue; }
            9 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e17f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x143a000f0; const P2: u64 = 0x142345b20; const P3: u64 = 0x142347330; const P4: u64 = 0x142349990; const P5: u64 = 0x142347460; const P6: u64 = 0x142347250; const P7: u64 = 0x142347150; const P8: u64 = 0x142346c50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rbp = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); rdx = P0; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); r14 = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rsi = (crate::simd_ops::ru64(P1)) as u64; rcx = ((rax as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = rsi.wrapping_add(0x170); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc4u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xbcu64))) as u64; rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb4u64))) as u64; rcx = rsi.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x60); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xb4u64), rdx); rdx = rsp.wrapping_add(0x68); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xbcu64), rdx); rdx = rsp.wrapping_add(0x70); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0xffffffff; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xc4u64), rdx); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xb4u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xbcu64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb4u64))) as u64; rcx = rsi.wrapping_add(0xd0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xbcu64))) as u64; rcx = rsi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc4u64))) as u64; rcx = rsi.wrapping_add(0x170); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0xb4u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xbcu64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc4u64), (0xffffffffu32) as u32); bb = 7; continue; }
            7 => { rbp = ((rbp).wrapping_add(0x10u64)) as u64; rdx = P0; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 9; continue; }
            9 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e19d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x4; const P2: u64 = 0xcc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { r8 = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rcx = (((r8 as u8)) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; r8 = (((rcx) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8).wrapping_add(P2), ((rcx as u32)) as u32); fl_l = rax; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x4; const P2: u64 = 0xcc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bce018; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0xbc; const P4: u64 = 0xc4; const P5: u64 = 0x58; const P6: u64 = 0x110; const P7: u64 = 0x48; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = P1; rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r8).wrapping_add(P4), rcx); crate::simd_ops::wu8(rdx.wrapping_add(r8).wrapping_add(P5), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8).wrapping_add(P7), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bce018; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0xbc; const P4: u64 = 0xc4; const P5: u64 = 0x58; const P6: u64 = 0x110; const P7: u64 = 0x48; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = P1; rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r8).wrapping_add(P4), rcx); crate::simd_ops::wu8(rdx.wrapping_add(r8).wrapping_add(P5), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8).wrapping_add(P7), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rsi); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bce018; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x142349990; const P4: u64 = 0xf0; const P5: u64 = 0xb0; const P6: u64 = 0xf4; const P7: u64 = 0xb4; const P8: u64 = 0xfc; const P9: u64 = 0xbc; const P10: u64 = 0x24; const P11: u64 = 0x28; const P12: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = P1; rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(0xb8u64)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), rcx); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rcx); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P10), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P11))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rbp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143bce018; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x142349990; const P4: u64 = 0xf0; const P5: u64 = 0xb0; const P6: u64 = 0xf4; const P7: u64 = 0xb4; const P8: u64 = 0xfc; const P9: u64 = 0xbc; const P10: u64 = 0x24; const P11: u64 = 0x28; const P12: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = P1; rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(0xb8u64)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), rcx); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rcx); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P10), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P11))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rbp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e1ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x4c; const P2: u64 = 0x58;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r10 = rdx; r11 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); rbx = P0; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rcx = (((r8 as u8)) as u32) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rcx = ((crate::simd_ops::ru8(r11.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u32 as u64; r9 = ((r9).wrapping_shr(8u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r9.wrapping_mul(8))))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rcx as u8)) as u8); crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x1u8) as u8); bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x4; const P2: u64 = 0x4c; const P3: u64 = 0x58;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; r9 = r8; r10 = rdx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r11 = P0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rcx = ((crate::simd_ops::ru8(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(r8.wrapping_mul(8))))) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rcx as u8)) as u8); crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x1u8) as u8); bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e20c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e21e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; r9 = rcx; fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = rdx.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rcx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r8 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r8; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r8; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = P0; r10 = rdx; fl_l = r8; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = r8.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r11 = ((r11).wrapping_shr(2u32)) as u64; rbx = rdx.wrapping_add(0x8); fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r9 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rbx = rbx.wrapping_add(0x20); rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x24u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x20u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x1cu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x18u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x14u64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x10u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0xcu64))) as u32 as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = r9; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x1u8) as u8); fl_l = r9; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143bce018; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x10; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x18; const P7: u64 = 0xfffffffffffffff4; const P8: u64 = 0x1c; const P9: u64 = 0xfffffffffffffffc; const P10: u64 = 0x24; const P11: u64 = 0x8; const P12: u64 = 0x142349550;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = ((rcx.wrapping_add(P0)) as u64) as u64; r11 = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = rdx; crate::simd_ops::wu64(rdx.wrapping_add(P4), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P8), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r9)) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P0), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x58u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r9.wrapping_add(0x40); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e25e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143bce018; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x10; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x18; const P7: u64 = 0xfffffffffffffff4; const P8: u64 = 0x1c; const P9: u64 = 0xfffffffffffffffc; const P10: u64 = 0x24; const P11: u64 = 0x8; const P12: u64 = 0x142349550;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r10 = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = ((rdx.wrapping_add(P0)) as u64) as u64; r11 = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = rdx; crate::simd_ops::wu64(rdx.wrapping_add(P4), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P8), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r9)) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(P0), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x58u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r9.wrapping_add(0x40); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x54; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x1u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e26f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x54; const P2: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_shr(8u32)) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x1u32))) as u32 as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce010; const P1: u64 = 0x143bbdad0; const P2: u64 = 0x143bbdae8; const P3: u64 = 0x143bbdae0; const P4: u64 = 0x143bbdaec; const P5: u64 = 0x1401decf0; const P6: u64 = 0x14183acc8; const P7: u64 = 0x1401de830; const P8: u64 = 0x142417470; const P9: u64 = 0x143bd6030; const P10: u64 = 0x143bd6034; const P11: u64 = 0x143bd6020; const P12: u64 = 0x1412ebcc0; const P13: u64 = 0x143bd602c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = r8; r13 = rdx; rbp = 0; fl_l = 0; fl_r = 0; rdx = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64)))) as u64; r15 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r15.wrapping_mul(8)), (0x0u64) as u64); r14 = (crate::simd_ops::ru32(P2)) as u32 as u64; rdi = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rdi.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_sub(rdi)) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r14 = rdx; rax = rdx.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(rdi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; rbx = rcx; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; rdx = (crate::simd_ops::ru64(P3)) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = P3; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rdi); crate::simd_ops::wu32(P2, ((r14 as u32)) as u32); crate::simd_ops::wu32(P4, ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { rax = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(P2, (1) as u32); rcx = rsi.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(r13.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x64u64) as u64); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            10 => { rdx = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P10)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = P11; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(P13)) as u32 as u64; rax = (crate::simd_ops::ru64(P11)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(P13)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P13, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P10)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(P13, (0x0u32) as u32); bb = 14; continue; }
            14 => { crate::simd_ops::lk_add_u32(P9, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = r12; rdx = P0; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 16; continue; }
            16 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x142349bb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    rdi = rdx;
    rbp = P0;
    rsi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; r8 = ((r8).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r8.wrapping_mul(8))))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e29f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bce018; const P1: u64 = 0x142349bb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp);
    rdi = r8;
    rbp = P0;
    rsi = rdx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xd0u64) as i64) as u64) as u64; r9 = ((r9).wrapping_shr(8u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r9.wrapping_mul(8))))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00110; const P1: u64 = 0x8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x142369f10; const P4: u64 = 0x1411b8c50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rsi = (crate::simd_ops::ru64(P0)) as u64;
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    loop { r8 = rbx; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x118u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a00110; const P1: u64 = 0x8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x142369f10; const P4: u64 = 0x1411b8c50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rsi = (crate::simd_ops::ru64(P0)) as u64;
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    loop { r8 = rbx; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x118u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bde070; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    r10 = 0;
    fl_l = 0; fl_r = 0;
    r11 = rdx;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi);
    rdi = P0;
    loop { rax = (crate::simd_ops::ru32(rbx.wrapping_add(r10.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(r10.wrapping_mul(8)))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r8 = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P2))) as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; r10 = ((r10).wrapping_add(1)) as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rdx = (((r9) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(P2))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8); fl_l = r10; fl_r = r11; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bde070; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    r10 = 0;
    fl_l = 0; fl_r = 0;
    r11 = r8;
    rbx = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi);
    rdi = P0;
    loop { rax = (crate::simd_ops::ru32(rbx.wrapping_add(r10.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(r10.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u64; r10 = ((r10).wrapping_add(1)) as u64; rcx = ((r9 as u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0x3fu32))) as u32 as u64; rdx = (((r9) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(P2))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8); fl_l = r10; fl_r = r11; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bde078; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { r8 = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rcx = ((crate::simd_ops::ru8(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r8 = (((r8) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(r8).wrapping_add(P2), ((rcx as u8)) as u8); fl_l = rax; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bde078; const P1: u64 = 0x4; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r10 = rdx;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(r10.wrapping_add(r9.wrapping_mul(8)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u32) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8); fl_l = r9; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bde078; const P1: u64 = 0x10e4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = rdx;
    r10 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { r8 = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = ((r8 as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; rdx = (((r8) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), (0x1u8) as u8); fl_l = rax; fl_r = r9; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143bde078; const P1: u64 = 0x10e4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r10 = 0;
    fl_l = 0; fl_r = 0;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r11 = P0;
    loop { rax = (crate::simd_ops::ru32(rdx.wrapping_add(r10.wrapping_mul(4)))) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; r9 = ((r9).wrapping_shr(6u32)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(P1), (0x1u8) as u8); fl_l = r10; fl_r = r8; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bde070; const P1: u64 = 0x142366bf0; const P2: u64 = 0x142417470; const P3: u64 = 0x143be6090; const P4: u64 = 0x143be6094; const P5: u64 = 0x143be6080; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143be608c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e2db0, funcinfo_rva: 0x37db30c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r15) as u64); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e2dfc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (0x1u32) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e2e41); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e2e5f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e2ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143bde070; const P1: u64 = 0x142366bf0; const P2: u64 = 0x142417470; const P3: u64 = 0x143be6090; const P4: u64 = 0x143be6094; const P5: u64 = 0x143be6080; const P6: u64 = 0x1412ebcc0; const P7: u64 = 0x143be608c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e2ed0, funcinfo_rva: 0x37db344, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r8) as u64; r14 = (rdx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; r15 = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r15) as u64); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e2f2c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x64u64) as u64); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (0x1u32) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(P0, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e2f71); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            8 => { rdx = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e2f8f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u32(P3, (1) as u32); crate::simd_ops::wu8(P0, (0x0u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x143bde078; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0xfffffffffffffff0; const P5: u64 = 0x142366db0; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e3000, funcinfo_rva: 0x37db37c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; r14 = ((P1) as u64) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r9 = (rbx) as u64; r8 = ((rbx.wrapping_add(P3)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3055); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e307b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, (rbp) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e30a4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (rbp) as u64); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e30e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x143bde078; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0xfffffffffffffff0; const P5: u64 = 0x142366db0; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e30e0, funcinfo_rva: 0x37db3b0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; r14 = ((P1) as u64) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (((rax) as i64).wrapping_mul((0x1120u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r9 = (rbx) as u64; r8 = ((rbx.wrapping_add(P3)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3135); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e315b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, (rbp) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3184); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (rbp) as u64); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e31e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x78; const P2: u64 = 0x68; const P3: u64 = 0x141350710; const P4: u64 = 0x70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = rdx; rbp = rcx; r14 = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_add(rdi.wrapping_mul(4)))) as u32) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(P1)))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0x0u8) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u16(rbx.wrapping_add(P4), (1) as u16); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x78; const P2: u64 = 0x68; const P3: u64 = 0x141350710; const P4: u64 = 0x70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = r8; rbp = rdx; r14 = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_add(rdi.wrapping_mul(4)))) as u32) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(P1)))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0x0u8) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u16(rbx.wrapping_add(P4), (1) as u16); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x142370490; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rbp; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x8; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x142370490; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(P0)) as u64; rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rdx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rbp; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x1412eb160; const P2: u64 = 0x142442de0; const P3: u64 = 0x14209d670; const P4: u64 = 0x1411e7fc0; const P5: u64 = 0x14209a1e0; const P6: u64 = 0x1413522d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r13 = r8; rax = rdx; rsi = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; r12 = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { r15 = (crate::simd_ops::ru64(rax.wrapping_add(r12.wrapping_mul(8)))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = rbx; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdi = ((rdx as u32)) as u32 as u64; rdi = ((rdi).wrapping_sub(1)) as u64; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdi = ((r8.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, r14); rbp = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0xa0u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; bb = 13; continue; }
            12 => { fl_l = ((rbx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((rdx as u32)) as u32 as u64; bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64; rdx = ((rax as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 15; continue; }
            15 => { rdx = rbx; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 17; continue; }
            17 => { rdi = ((rcx as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(0xffffffffffffffe0u64)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, r14); rbp = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r14); r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); r14 = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffe0u64)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64)))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0xb0u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; bb = 27; continue; }
            26 => { fl_l = ((rbx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 28; continue; }
            28 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = r12; fl_r = r13; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 30; continue; }
            30 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x143bee0d8; const P2: u64 = 0xe0; const P3: u64 = 0xd8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rdi = rcx; rsi = P1; r11 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r11.wrapping_mul(4)))) as u32 as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r10 = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r9 = rcx; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; bb = 9; continue; }
            8 => { fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 10; continue; }
            10 => { r11 = ((r11).wrapping_add(1)) as u64; fl_l = r11; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x143bee0d8; const P2: u64 = 0xe0; const P3: u64 = 0xd8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = (crate::simd_ops::ru64(P0)) as u64; rbx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rdi = rdx; rsi = P1; r11 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r11.wrapping_mul(4)))) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r10 = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8))))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r8 = rcx; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; bb = 9; continue; }
            8 => { fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 10; continue; }
            10 => { r11 = ((r11).wrapping_add(1)) as u64; fl_l = r11; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e38c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x143bee0d8; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; r14 = rcx; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx).wrapping_add(0xd8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; r15 = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rdi.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), (1) as u32); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x143bee0d8; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; r14 = rdx; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx).wrapping_add(0xd8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; r15 = P1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rdi.wrapping_mul(4)))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(8u32)) as u64; rax = (((rax as u8)) as u32) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x4f8u64) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8))))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), (1) as u32); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x8; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(rbx.wrapping_mul(8)), r14); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x8; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x14241d6f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rdi = rdx; rbp = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(rbx.wrapping_mul(8)), r14); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x14241d6f0; const P2: u64 = 0x1412df620; const P3: u64 = 0x142205e90; const P4: u64 = 0x14397ff58; const P5: u64 = 0x1412d0a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1423e3ba0, funcinfo_rva: 0x37db5f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (r8) as u64; rbx = (rdx) as u64; r14 = (crate::simd_ops::ru64(P0)) as u64; rsi = ((r14.wrapping_add(0x88u64)) as u64) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x90u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (0x1u32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3c37); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3c6d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; crate::simd_ops::wu64(rax, (rdi) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rcx = ((r14.wrapping_add(0xb8u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3c93); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x1d0u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::lk_and_u8(rdx.wrapping_add(0x6du64), (0xfdu8) as u8); rcx = (crate::simd_ops::ru64(P4)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3cb2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1423e3cd8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx, (r15) as u64); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 21; continue; }
            21 => { return rax; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x14386cb40; const P2: u64 = 0x30; const P3: u64 = 0x34; const P4: u64 = 0x28; const P5: u64 = 0x14020bcb0; const P6: u64 = 0x40; const P7: u64 = 0x44; const P8: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r15 = rcx; r14 = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(r14.wrapping_add(P2), (1) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(P7))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(P6))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e3f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001e0; const P1: u64 = 0x14386cb40; const P2: u64 = 0x30; const P3: u64 = 0x34; const P4: u64 = 0x28; const P5: u64 = 0x14020bcb0; const P6: u64 = 0x40; const P7: u64 = 0x44; const P8: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = r8; r15 = rdx; r14 = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(r15.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(r14.wrapping_add(P2), (1) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(P7))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(P6))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e4160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001f8; const P1: u64 = 0x140209da0; const P2: u64 = 0x141357d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); r15 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(rbp.wrapping_mul(8)))) as u64; r8 = rsp.wrapping_add(0x60); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rcx = rbx.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc0u64))) as u32 as u64; r9 = rdi; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rdx = ((r9.wrapping_add(rcx)) as u64) as u64; r8 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rdx, rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x38u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rcx); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x24u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rcx as u32)) as u32); rcx = rbx.wrapping_add(0x88); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = rsp.wrapping_add(0x60); rcx = rbx.wrapping_add(0x88); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0xc0u64), (1) as u32); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0xd0u64), (1) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xd8u64), (0x1u8) as u8); fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e4320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001f8; const P1: u64 = 0x140209da0; const P2: u64 = 0x141357d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rsi = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(rbp.wrapping_mul(8)))) as u64; r8 = rsp.wrapping_add(0x60); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rcx = rbx.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc0u64))) as u32 as u64; r9 = rdi; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rdx = ((r9.wrapping_add(rcx)) as u64) as u64; r8 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rdx, rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x38u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rcx); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x24u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x48u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x4cu64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_sub(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rcx as u32)) as u32); rcx = rbx.wrapping_add(0x88); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xb8u64))) as u64; r8 = ((r8).wrapping_add(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = rsp.wrapping_add(0x60); rcx = rbx.wrapping_add(0x88); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0xc0u64), (1) as u32); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0xd0u64), (1) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xd8u64), (0x1u8) as u8); fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e4510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001f8; const P1: u64 = 0x143428760; const P2: u64 = 0x140209da0; const P3: u64 = 0x1424b16a0; const P4: u64 = 0x140242c40; const P5: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rax.wrapping_sub(0x48); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; r12 = (crate::simd_ops::ru64(P0)) as u64; r14 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rdi = rdx.wrapping_add(0x28); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r13); rcx = r12.wrapping_add(0x88); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0xb8u64), (xmm14) as u128); xmm14 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm12) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm13) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xc8u64), (xmm15) as u128); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x28u64))) as u64; r8 = rbp.wrapping_add(0x50); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = rdi.wrapping_sub(0x18); rsi = (crate::simd_ops::ru64(r12.wrapping_add(0xb8u64))) as u64; rdx = 0x50; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rbx = r15; rbx = ((rbx).wrapping_shl(6u32)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rsi), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(r12.wrapping_add(0xd8u64), (0x1u8) as u8); rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rsi).wrapping_add(0x8u64), rax); rcx = rsp.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(rsi).wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rsi).wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_sub(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(rsi).wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rsi).wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(rsi).wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rsi).wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(rsi).wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rsi).wrapping_add(0x3cu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(0x10u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(0x1cu64))) as u64) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi).wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x4cu64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi).wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(0x34u64))) as u64) as u128; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(0xc8u64)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(rsi).wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x64u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32) as u128; xmm13 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; xmm12 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; xmm15 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32) as u128; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm7 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm1) ^ (xmm14); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm5 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32) as u128; xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm14 = (xmm0) ^ (xmm14); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm9 = crate::simd_ops::vinsertps(xmm9, xmm10, 0x10u8); xmm13 = crate::simd_ops::vinsertps(xmm13, xmm8, 0x10u8); xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm3 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm1) ^ (crate::simd_ops::ru128(P1)); xmm0 = 0u128; xmm9 = crate::simd_ops::vinsertps(xmm9, xmm11, 0x20u8); xmm9 = crate::simd_ops::vinsertps(xmm9, xmm0, 0x30u8); xmm13 = crate::simd_ops::vinsertps(xmm13, xmm12, 0x20u8); xmm13 = crate::simd_ops::vinsertps(xmm13, xmm0, 0x30u8); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm9) as u128); crate::simd_ops::wu128(rdx, (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm13) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm13) as u128); xmm15 = crate::simd_ops::vinsertps(xmm15, xmm5, 0x10u8); xmm15 = crate::simd_ops::vinsertps(xmm15, xmm2, 0x20u8); xmm15 = crate::simd_ops::vinsertps(xmm15, xmm0, 0x30u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm15) as u128); xmm0 = crate::simd_ops::vinsertps(xmm0, xmm14, 0x10u8); xmm14 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, xmm4, 0x20u8); xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(P5)) as u128), 0x30u8); crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x48u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x24u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm15) as u128); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(0x60u64)) as u64; rcx = r12.wrapping_add(0x88); fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x100u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x110u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; rsp = ((rsp).wrapping_add(0x130u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423e48d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001f8; const P1: u64 = 0x140209da0; const P2: u64 = 0x141357f20; const P3: u64 = 0x143427660; const P4: u64 = 0x1413582c0; const P5: u64 = 0x143428de0; const P6: u64 = 0x1434281d0; const P7: u64 = 0x14238ff30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rbp = r8; r15 = rdx; rsi = (crate::simd_ops::ru64(P0)) as u64; rdi = rsi.wrapping_add(0xb8); r14 = rsi.wrapping_add(0xc8); r13 = 0; fl_l = 0; fl_r = 0; xmm6 = 0u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0xc0u64))) as u32 as u64; r8 = rsp.wrapping_add(0x90); rdx = rsp.wrapping_add(0x20); rcx = rsi.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((((rax) as i64).wrapping_shr(6u32) as u64)) as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = ((((rcx) as i64).wrapping_shr(6u32) as u64)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm6 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm2 = (xmm1) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r13 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x2cu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), (((xmm1 >> 32) as u32) as u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x44u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (((xmm1 >> 96) as u32) as u32) as u32); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x44u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x48u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x54u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x58u64), rdx); let y_addr: u64 = P5; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(P6)) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rax), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x8u64), r13); let y_addr: u64 = rcx.wrapping_add(rax).wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(rax).wrapping_add(0x30u64), (xmm2) as u128); crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0x50u64), r13); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); r8 = r15.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsi.wrapping_add(0xd8u64), (0x1u8) as u8); r15 = ((r15).wrapping_add(0x60u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
