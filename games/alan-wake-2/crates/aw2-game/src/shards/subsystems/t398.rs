#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_142595e2c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x50;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142595e58(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x50;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425965b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413dbb50; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x190);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413db2b0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x38;
    rcx = rbp.wrapping_add(0x60);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x20;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425966b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413dbb50; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x168);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413db2b0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x38;
    rcx = rbp.wrapping_add(0x240);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425967c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413db2b0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x38;
    rcx = rbp.wrapping_add(0x320);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413dbb50; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x228);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425968b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x20;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425968f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x10;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x1413c4e40; const P2: u64 = 0x78; const P3: u64 = 0x140203130;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x28;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413dbb50; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x110);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142675810; const P1: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = 0xbb;
    r8 = P0;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = 0xffffffff;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1411f9db0; const P2: u64 = 0x1411f9e70; const P3: u64 = 0x1411fae80; const P4: u64 = 0x20; const P5: u64 = 0x1411fb120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x1411f9e70; const P2: u64 = 0x141202490; const P3: u64 = 0x1411fa150;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142596fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1411f9e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142597040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1411fa150; const P2: u64 = 0x1416a6870;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425970c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x1411fa150; const P2: u64 = 0x1411fb6c0; const P3: u64 = 0x30; const P4: u64 = 0x1411fb920;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142597480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f9e90;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rdx = ((rdx).wrapping_add(0x38u64)) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425976d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1411f9db0; const P2: u64 = 0x78; const P3: u64 = 0x1411fa150;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425976f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f9e90;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    rdx = ((rdx).wrapping_add(0x38u64)) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142597be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x1416bb6d0; const P2: u64 = 0x1411fa420; const P3: u64 = 0x1416f5be0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1411fa420; const P2: u64 = 0x14170e830;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xd0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x1411f9e70; const P2: u64 = 0x8; const P3: u64 = 0x14170e830; const P4: u64 = 0x30; const P5: u64 = 0x1411fc280;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x1417269e0; const P2: u64 = 0x1416bdbe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x141732870; const P2: u64 = 0x141732840; const P3: u64 = 0x1416a6a10; const P4: u64 = 0x1417328a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x188u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(0x18u64)) as u64;
    rax = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = ((rdx).wrapping_add(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1411fa150; const P2: u64 = 0x1417410b0; const P3: u64 = 0x1416adae0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x100u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142598f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1416bb6c0; const P2: u64 = 0x1411f9e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1411f9e70; const P2: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425990e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x1416a60d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x141764cb0; const P2: u64 = 0x141232420;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x378u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1416adae0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x3d8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425991a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1416adae0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x3e8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425991f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1416bdbe0; const P2: u64 = 0x1417580d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1416b6c40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x140u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1417580d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x1b8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425992b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1417580d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x208u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425992d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1416bdbe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x240u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1416adae0; const P2: u64 = 0x1416bdbe0; const P3: u64 = 0x141764ca0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1411f9db0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x188u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1411f9e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1411f9e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1411f9e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1411f9e70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rcx).wrapping_add(0xd0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425997a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426af640; const P1: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = 0x44;
    r8 = P0;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425997d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x141775a00; const P2: u64 = 0x20; const P3: u64 = 0x141775a20; const P4: u64 = 0xe0; const P5: u64 = 0x1417788f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x141778a30; const P2: u64 = 0x141778a10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425998a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x1417789b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0xe0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x141778a30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x160u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x141778a10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x180u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x130; const P1: u64 = 0x141775a20; const P2: u64 = 0x30; const P3: u64 = 0x14177b5b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425999a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x130; const P1: u64 = 0x14177b520; const P2: u64 = 0x14177b460;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x141778860; const P2: u64 = 0x40; const P3: u64 = 0x141778a10; const P4: u64 = 0x14177e920; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x141775a20; const P2: u64 = 0x141778860; const P3: u64 = 0x28; const P4: u64 = 0x141781fd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x141782030;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x14177d5d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x130u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x14177d5d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x150u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x141778860;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x190u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x141775a20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x1b0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x141778a30; const P2: u64 = 0x141775a40; const P3: u64 = 0x40; const P4: u64 = 0x141778a10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x141778860; const P2: u64 = 0x30; const P3: u64 = 0x20; const P4: u64 = 0x70; const P5: u64 = 0x141775a00; const P6: u64 = 0xc0; const P7: u64 = 0x141782210;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x141778a30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x158u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x141782060;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x178u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x141778860;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x198u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426af910; const P1: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = 0x45;
    r8 = P0;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x141778a30; const P2: u64 = 0x141778a10; const P3: u64 = 0x141778860;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142599fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x141778a10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x228u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x141782210; const P2: u64 = 0x141778860;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xc8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x141778860; const P2: u64 = 0x141778a30; const P3: u64 = 0x141778a10; const P4: u64 = 0x14178b7f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc50; const P1: u64 = 0x141778860; const P2: u64 = 0x141778a30; const P3: u64 = 0x141778a10; const P4: u64 = 0x50; const P5: u64 = 0x14178e9b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x141778a30; const P2: u64 = 0x141778a10; const P3: u64 = 0xe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426c23b0; const P1: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = 0x33;
    r8 = P0;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x141778a10; const P2: u64 = 0x141778a30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x141778a10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x14178b7f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426c2960; const P1: u64 = 0x1401de410;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = 0x35;
    r8 = P0;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x100; const P1: u64 = 0x14179f950; const P2: u64 = 0x141778860; const P3: u64 = 0x141778a30; const P4: u64 = 0x141778a10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a5ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rdx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0xc0000005;
    rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rax = ((rcx as u32)) as u32 as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a604(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834f54;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acf2; const P1: u64 = 0x8; const P2: u64 = 0x14183acf8; const P3: u64 = 0x14183acfe;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rbp = rdx;
    crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rcx);
    crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rcx);
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rcx);
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0xe06d7363;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; } else { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a68f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834f54;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a6bb(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834f54;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a6ea(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a708(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183ae00;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rcx;
    rcx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x108u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x190u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x218u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x2a0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259a7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x280; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x170; const P3: u64 = 0x60; const P4: u64 = 0x58; const P5: u64 = 0x1413cfa30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259adb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1b0; const P1: u64 = 0x1401e1560; const P2: u64 = 0x170; const P3: u64 = 0xf0; const P4: u64 = 0x130; const P5: u64 = 0xb0; const P6: u64 = 0x70; const P7: u64 = 0x50; const P8: u64 = 0x1401e80a0; const P9: u64 = 0x200; const P10: u64 = 0x1401ee260;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259b367(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397da50; const P1: u64 = 0x142442de0; const P2: u64 = 0x8; const P3: u64 = 0x14307df10; const P4: u64 = 0x1423f2b40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rbp = rdx;
    rbx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x420u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = P3;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259bad6(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rdx = 0x178;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2a0u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259c3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff400; const P1: u64 = 0x14240ab10; const P2: u64 = 0x141897e60; const P3: u64 = 0x1401e3aa0; const P4: u64 = 0x1401de3c0; const P5: u64 = 0x140251600; const P6: u64 = 0x1402516d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x3a0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x3a0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x600u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x600u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x600u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x3f0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x600u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0x580); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x530); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0x340); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x340); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x2f0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x2f0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x240); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0x240); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x240); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x240); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0x240); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x600u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = rdx.wrapping_add(0x3a0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = rdx.wrapping_add(0x3a0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = rdx.wrapping_add(0x3a0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            131 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            132 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            133 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            134 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            135 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            136 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            137 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            138 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            139 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            140 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            141 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            142 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            143 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            144 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            145 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            146 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            147 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            148 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            149 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            150 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            151 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            152 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            153 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            154 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            155 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            156 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            157 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            158 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            159 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            160 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            161 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            162 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            163 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            164 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            165 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            166 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            167 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            168 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            169 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            170 => { rcx = rdx.wrapping_add(0x280); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            171 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            172 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            173 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            174 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            175 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            176 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            177 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            178 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            179 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            180 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            181 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            182 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            183 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            184 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            185 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            186 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            187 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            188 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            189 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            190 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            191 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            192 => { rcx = rdx.wrapping_add(0x220); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            193 => { rcx = rdx.wrapping_add(0x220); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            194 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            195 => { rcx = rdx.wrapping_add(0x250); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            196 => { rcx = rdx.wrapping_add(0x250); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            197 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x350u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            198 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            199 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259cdc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140260f30; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401ff400; const P3: u64 = 0x1401e3aa0; const P4: u64 = 0x1401de3c0; const P5: u64 = 0x14025a5b0; const P6: u64 = 0x1402503a0; const P7: u64 = 0x14025c600; const P8: u64 = 0x1402516d0; const P9: u64 = 0x1401fb4d0; const P10: u64 = 0x1402128c0; const P11: u64 = 0x14025c550; const P12: u64 = 0x14025ca10; const P13: u64 = 0x14025c5b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0x300); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x300); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x520u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x180); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x420); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x4d0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x180); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x4d0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x520u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x4d0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = rdx.wrapping_add(0x3c0); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x5e8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x3c0); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = rdx.wrapping_add(0x1b0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = rdx.wrapping_add(0x1b0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = rdx.wrapping_add(0x3c0); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x5b8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x5e8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = rdx.wrapping_add(0x3c0); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x5b8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            131 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x5e8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            132 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            133 => { rcx = rdx.wrapping_add(0x360); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            134 => { rcx = rdx.wrapping_add(0x360); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            135 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            136 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            137 => { rcx = rdx.wrapping_add(0x1b0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            138 => { rcx = rdx.wrapping_add(0x1b0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            139 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            140 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            141 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            142 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            143 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            144 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            145 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            146 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            147 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            148 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            149 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            150 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x188u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            151 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            152 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            153 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            154 => { rcx = rdx.wrapping_add(0x520); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259da90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x1401e81c0; const P2: u64 = 0xd8; const P3: u64 = 0xe8; const P4: u64 = 0x14240ab10; const P5: u64 = 0xd0; const P6: u64 = 0x1401de3c0; const P7: u64 = 0x14025bac0; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259dc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3d0; const P1: u64 = 0x140267c50; const P2: u64 = 0x3d8; const P3: u64 = 0x20; const P4: u64 = 0x1412a89f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259dd24(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x28;
    rcx = rbp.wrapping_add(0x80);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259de50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x140267ef0; const P2: u64 = 0x140260f30; const P3: u64 = 0x128; const P4: u64 = 0x14240ab10; const P5: u64 = 0x1401de3c0; const P6: u64 = 0x10f267ef0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x2e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x308u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x328u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x348u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x368u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x388u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x3a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x3c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x3e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x428u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x458u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x478u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259df40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x140267ef0; const P2: u64 = 0x10f267ef0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x498u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x4b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x4d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x4f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x518u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x538u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x558u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x578u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x598u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x5b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x5d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x5f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x618u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x638u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x658u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x678u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259e070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x120; const P1: u64 = 0x140267ef0; const P2: u64 = 0x1401de400;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x698u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x6b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x6d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x6f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x718u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x738u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x758u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x778u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x798u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x7b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x7d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x7e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259e1d8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x28;
    rcx = rbp.wrapping_add(0x50);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259e400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x190; const P1: u64 = 0x14023c130; const P2: u64 = 0x60; const P3: u64 = 0x2a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259e500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020bdf0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401ecf00; const P3: u64 = 0x1401de3c0; const P4: u64 = 0x14028b740; const P5: u64 = 0x14028dfe0; const P6: u64 = 0x1418d8ee0; const P7: u64 = 0x14028dfd0; const P8: u64 = 0x1418d8eb0; const P9: u64 = 0x1418d8a10; const P10: u64 = 0x14028dff0; const P11: u64 = 0x1418d8660; const P12: u64 = 0x1418d7800; const P13: u64 = 0x1401f9d70; const P14: u64 = 0x14028c580; const P15: u64 = 0x14028cc10; const P16: u64 = 0x1402d5b40; const P17: u64 = 0x1401fc300; const P18: u64 = 0x14028c540; const P19: u64 = 0x14028d050; const P20: u64 = 0x14028b130; const P21: u64 = 0x1401ee260; const P22: u64 = 0x1401e4ba0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x210); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0xd0); fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = rdx.wrapping_add(0x288); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x170); fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x4c0u64))) as u64; return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0xe8); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259fe40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x408; const P1: u64 = 0x14240ab10; const P2: u64 = 0x270; const P3: u64 = 0x1401e6930; const P4: u64 = 0x140; const P5: u64 = 0x70; const P6: u64 = 0x1402174e0; const P7: u64 = 0x190; const P8: u64 = 0xc0; const P9: u64 = 0x40; const P10: u64 = 0x14039f010;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259feb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x64; const P1: u64 = 0x14240ab10; const P2: u64 = 0xe0; const P3: u64 = 0x1401de3c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14259ff50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403b30c0; const P1: u64 = 0x1403b31d0; const P2: u64 = 0x140280170; const P3: u64 = 0x14023c130; const P4: u64 = 0x14240ab10; const P5: u64 = 0x1401e6930; const P6: u64 = 0x1401de3c0; const P7: u64 = 0x1403b3f50; const P8: u64 = 0x1401e3aa0; const P9: u64 = 0x1401df5a0; const P10: u64 = 0x1401ecf00; const P11: u64 = 0x140217a20; const P12: u64 = 0x14021b720;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x230); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x328); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x200u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x180u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x270u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0xa8); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0xa8); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0xd8); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x350); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a04d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x14025a590; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x1401de3f0; const P4: u64 = 0x1412a89d0; const P5: u64 = 0x140267c50; const P6: u64 = 0x1403cc890;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x150); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x98); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x98); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x7c); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2f8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2f8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2f8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2f8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x7c); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x210); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2f8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2f8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x7c); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x270); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a0690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403d13f0; const P1: u64 = 0x1403d13c0; const P2: u64 = 0x14240ab10; const P3: u64 = 0x1401de3c0; const P4: u64 = 0x1403d1490; const P5: u64 = 0x14026d670;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1f8u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1e0u64))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1f8u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x158u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x140u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x34); fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x140u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x34); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x88); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x148u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x140u64))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x148u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x88); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a0aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x150; const P4: u64 = 0x30; const P5: u64 = 0x140266d90; const P6: u64 = 0xc0; const P7: u64 = 0x68;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a11a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x28;
    rcx = rbp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a124e(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x28;
    rcx = rbp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a1900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x14240ab10; const P2: u64 = 0xb8; const P3: u64 = 0x1401ecf00; const P4: u64 = 0x20; const P5: u64 = 0x58; const P6: u64 = 0x140217a20; const P7: u64 = 0x38; const P8: u64 = 0x14021b720; const P9: u64 = 0xc0; const P10: u64 = 0x118; const P11: u64 = 0x110; const P12: u64 = 0x10f267c50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a1eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020bdf0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401ff400; const P3: u64 = 0x1401e3aa0; const P4: u64 = 0x1401ecf00; const P5: u64 = 0x14043be00; const P6: u64 = 0x1401de3c0; const P7: u64 = 0x1402516d0; const P8: u64 = 0x1402503a0; const P9: u64 = 0x140251600; const P10: u64 = 0x14026d680; const P11: u64 = 0x1403e5be0; const P12: u64 = 0x1403e6fe0; const P13: u64 = 0x140217a50; const P14: u64 = 0x1401fb4d0; const P15: u64 = 0x14043bd70; const P16: u64 = 0x1401e68b0; const P17: u64 = 0x14026d670; const P18: u64 = 0x1402467f0; const P19: u64 = 0x1419c93d0; const P20: u64 = 0x1419c93a0; const P21: u64 = 0x1419ddee0; const P22: u64 = 0x140217a20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0xa8); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x210); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x3b0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x3b0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x230); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x230); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x3b0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x260); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x3b0u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x180u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x180u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0x230); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x180u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = rdx.wrapping_add(0x1a0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x180u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x290u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x290u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = rdx.wrapping_add(0x1b0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = rdx.wrapping_add(0x1b0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = rdx.wrapping_add(0x98); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = rdx.wrapping_add(0x88); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            131 => { rcx = rdx.wrapping_add(0x98); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            132 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            133 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            134 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            135 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            136 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            137 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x220u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            138 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            139 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            140 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            141 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x398u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            142 => { rcx = rdx.wrapping_add(0xf8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            143 => { rcx = rdx.wrapping_add(0xf8); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            144 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            145 => { rcx = rdx.wrapping_add(0x108); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            146 => { rcx = rdx.wrapping_add(0x118); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            147 => { rcx = rdx.wrapping_add(0x128); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            148 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            149 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            150 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            151 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            152 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            153 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            154 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            155 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            156 => { rcx = rdx.wrapping_add(0xa8); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            157 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            158 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            159 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            160 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            161 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x190u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            162 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x180u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            163 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            164 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            165 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            166 => { rcx = rdx.wrapping_add(0xe8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            167 => { rcx = rdx.wrapping_add(0xe8); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            168 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            169 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            170 => { rcx = rdx.wrapping_add(0x178); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a26fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x48;
    rcx = rbp.wrapping_add(0xa0);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a2938(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x48;
    rcx = rbp.wrapping_add(0x70);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a297a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x48;
    rcx = rbp.wrapping_add(0x70);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a29f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff400; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401e3aa0; const P3: u64 = 0x1401de3c0; const P4: u64 = 0x1403f54d0; const P5: u64 = 0x140472910; const P6: u64 = 0x140472ab0; const P7: u64 = 0x1401ecf00; const P8: u64 = 0x14026d670; const P9: u64 = 0x140472950; const P10: u64 = 0x1404728b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x200u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x218u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x230); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x230); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2b8u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c8u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x230u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x248u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x140u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x90u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x148u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x148u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x340); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x340); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x3d0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0x3d0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0xb8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0xb8); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x110u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x118u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x4a0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x4a0u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x2e0); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1f8u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x208u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x160u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = rdx.wrapping_add(0xf8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = rdx.wrapping_add(0xf8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x170u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = rdx.wrapping_add(0x108); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1d8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a3060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x14048be00; const P2: u64 = 0x40; const P3: u64 = 0x30; const P4: u64 = 0x14240ab10; const P5: u64 = 0x220; const P6: u64 = 0x1401de3c0; const P7: u64 = 0xb0; const P8: u64 = 0x90; const P9: u64 = 0x14020bdf0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a34c9(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141a0ad44;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = rdx;
    rax = P0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a3706(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140498cc0;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7f0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x110u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a37e5(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141a0ed9b;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = P0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a3910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x1404a73f0; const P3: u64 = 0x14039f010; const P4: u64 = 0x1401e4ed0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(0x130u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(0x130u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x300u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(0x180u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(0x180u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(0xa0u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(0xa0u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(0x78u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(0x78u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = ((rdx.wrapping_add(0x268u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = ((rdx.wrapping_add(0x1d8u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = ((rdx.wrapping_add(0x1d8u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x318u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = ((rdx.wrapping_add(0x220u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = ((rdx.wrapping_add(0x220u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x318u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            131 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            132 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            133 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            134 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            135 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            136 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            137 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            138 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            139 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            140 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            141 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            142 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            143 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            144 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            145 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            146 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            147 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            148 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            149 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            150 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            151 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            152 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            153 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            154 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            155 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            156 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            157 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            158 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            159 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            160 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            161 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            162 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            163 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            164 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            165 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            166 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            167 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            168 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            169 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            170 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            171 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            172 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            173 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            174 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            175 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            176 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            177 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            178 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            179 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            180 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            181 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            182 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            183 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            184 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            185 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            186 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            187 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            188 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            189 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            190 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            191 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            192 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            193 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            194 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            195 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            196 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            197 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            198 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            199 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            200 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            201 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            202 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            203 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            204 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            205 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            206 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            207 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            208 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            209 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            210 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            211 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            212 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            213 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            214 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            215 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            216 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            217 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            218 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            219 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            220 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            221 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            222 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            223 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            224 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            225 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            226 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            227 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            228 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            229 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            230 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            231 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            232 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            233 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            234 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            235 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            236 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            237 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            238 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            239 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            240 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            241 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            242 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            243 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            244 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            245 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            246 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            247 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            248 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            249 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            250 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            251 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            252 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            253 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            254 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            255 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            256 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            257 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            258 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            259 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            260 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            261 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            262 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            263 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            264 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            265 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            266 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            267 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            268 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            269 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            270 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            271 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            272 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            273 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            274 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            275 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            276 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            277 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            278 => { rcx = ((rdx.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            279 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            280 => { rcx = ((rdx.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            281 => { rcx = ((rdx.wrapping_add(0xe0u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            282 => { rcx = ((rdx.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            283 => { rcx = ((rdx.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a46d6(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14200d040;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x180u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a4730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x240; const P1: u64 = 0x142483ab0; const P2: u64 = 0x30; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1401de3c0; const P5: u64 = 0x20; const P6: u64 = 0x1404a73e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a4860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10c; const P1: u64 = 0x14240ab10; const P2: u64 = 0xf0; const P3: u64 = 0x14041fd10; const P4: u64 = 0x70; const P5: u64 = 0x1402516d0; const P6: u64 = 0x30; const P7: u64 = 0xd0; const P8: u64 = 0x48; const P9: u64 = 0x1401e68b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a4940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x1401e5420; const P2: u64 = 0x1401e5380; const P3: u64 = 0x14020bdf0; const P4: u64 = 0x1404c1570; const P5: u64 = 0x30; const P6: u64 = 0x14240ab10; const P7: u64 = 0x50; const P8: u64 = 0xa8; const P9: u64 = 0xa0; const P10: u64 = 0x140267c50; const P11: u64 = 0x140217a20; const P12: u64 = 0x1401ecf00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xf0u64))) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a4bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x64; const P1: u64 = 0x14240ab10; const P2: u64 = 0x60; const P3: u64 = 0x1401de3f0; const P4: u64 = 0x280; const P5: u64 = 0x90; const P6: u64 = 0x110; const P7: u64 = 0x14023c130; const P8: u64 = 0x70; const P9: u64 = 0x1401e68b0; const P10: u64 = 0xa0; const P11: u64 = 0x1401ff400; const P12: u64 = 0x1401de3c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1f8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1e0u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1f8u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a5040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x30; const P3: u64 = 0x1401de3c0; const P4: u64 = 0x14052f2f0; const P5: u64 = 0x14052f2a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a53e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3f0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x34;
    rcx = rbp.wrapping_add(0xa0);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a5730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x140217a90; const P3: u64 = 0x1401fb4d0; const P4: u64 = 0x1402128c0; const P5: u64 = 0x14028b470; const P6: u64 = 0x141ac0220; const P7: u64 = 0x14020bdf0; const P8: u64 = 0x140586d20; const P9: u64 = 0x1401ecf00; const P10: u64 = 0x141ac0d40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x230u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x108u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x110u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x110u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x110u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x110u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x110u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x34); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = ((rcx).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a5bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x1401ff400; const P2: u64 = 0x1401e3aa0; const P3: u64 = 0x1401de3c0; const P4: u64 = 0x1401e4ea0; const P5: u64 = 0x1401e68b0; const P6: u64 = 0x1403f5cd0; const P7: u64 = 0x1403d13f0; const P8: u64 = 0x14052b070; const P9: u64 = 0x140588fc0; const P10: u64 = 0x1401ecee0; const P11: u64 = 0x1403d13c0; const P12: u64 = 0x14055e7f0; const P13: u64 = 0x140220c80; const P14: u64 = 0x140588f10; const P15: u64 = 0x140226c00; const P16: u64 = 0x140588f20; const P17: u64 = 0x1403e5be0; const P18: u64 = 0x1403e6fe0; const P19: u64 = 0x140217d10; const P20: u64 = 0x1401e1560; const P21: u64 = 0x14023c130; const P22: u64 = 0x140589000; const P23: u64 = 0x140589030; const P24: u64 = 0x1401ffe60; const P25: u64 = 0x140589040; const P26: u64 = 0x140589070; const P27: u64 = 0x1405890a0; const P28: u64 = 0x141ac85d0; const P29: u64 = 0x141ac85a0; const P30: u64 = 0x141ac8570; const P31: u64 = 0x141ac8540; const P32: u64 = 0x141ac8510; const P33: u64 = 0x141ac84e0; const P34: u64 = 0x141ac84b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1c8u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x74); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x74); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x9c); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2a0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = rdx.wrapping_add(0xd4); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = rdx.wrapping_add(0xd4); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = rdx.wrapping_add(0xfc); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x1a0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x290u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = rdx.wrapping_add(0x130); fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x290u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0x164); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = rdx.wrapping_add(0x164); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = rdx.wrapping_add(0x18c); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = rdx.wrapping_add(0x164); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1b8u64))) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x114); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = rdx.wrapping_add(0x114); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = rdx.wrapping_add(0x13c); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = rdx.wrapping_add(0xb8); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = rdx.wrapping_add(0xc8); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = rdx.wrapping_add(0xd8); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x128u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            131 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            132 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            133 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            134 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            135 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            136 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            137 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            138 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            139 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            140 => { rcx = rdx.wrapping_add(0x35c); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            141 => { rcx = rdx.wrapping_add(0x35c); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            142 => { rcx = rdx.wrapping_add(0x384); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            143 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            144 => { rcx = rdx.wrapping_add(0x408); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            145 => { rcx = rdx.wrapping_add(0x4d8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            146 => { rcx = rdx.wrapping_add(0x4d8); return crate::simd_ops::call_fwd(P24, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            147 => { rcx = rdx.wrapping_add(0x50c); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            148 => { rcx = rdx.wrapping_add(0x554); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            149 => { rcx = rdx.wrapping_add(0x5a0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            150 => { rcx = rdx.wrapping_add(0x5e0); return crate::simd_ops::call_fwd(P25, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            151 => { rcx = rdx.wrapping_add(0x5f0); return crate::simd_ops::call_fwd(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            152 => { rcx = rdx.wrapping_add(0x600); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            153 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P28, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            154 => { rcx = rdx.wrapping_add(0xf8); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            155 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P30, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            156 => { rcx = rdx.wrapping_add(0x188); return crate::simd_ops::call_fwd(P31, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            157 => { rcx = rdx.wrapping_add(0x1d0); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            158 => { rcx = rdx.wrapping_add(0x218); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            159 => { rcx = rdx.wrapping_add(0x260); return crate::simd_ops::call_fwd(P34, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a63a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x110; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x1110; const P4: u64 = 0x1405a4df0; const P5: u64 = 0x4b0; const P6: u64 = 0x4f8; const P7: u64 = 0x55c; const P8: u64 = 0x5b0; const P9: u64 = 0x608; const P10: u64 = 0x1403db200; const P11: u64 = 0x780; const P12: u64 = 0x1403db1a0; const P13: u64 = 0x1401ff400; const P14: u64 = 0x1403db100; const P15: u64 = 0x140217a20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x7c8); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x550); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0xa0c); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0xa58); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x4b0); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x1c0); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a6798(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ecf00; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x890);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a68a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ecf00; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x910);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a69b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ecf00; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0x990);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a6abc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ecf00; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0xa10);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a6bc8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ecf00; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0xa90);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a6cd4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ecf00; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x18;
    rcx = rbp.wrapping_add(0xb10);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a6f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x30; const P3: u64 = 0x80; const P4: u64 = 0x14240ab10; const P5: u64 = 0x60; const P6: u64 = 0x1401fb4d0; const P7: u64 = 0xc0; const P8: u64 = 0x90; const P9: u64 = 0x1401e18a0; const P10: u64 = 0x48; const P11: u64 = 0x1401de400; const P12: u64 = 0x1405eed80; const P13: u64 = 0x1403e6fe0; const P14: u64 = 0x1405eee80; const P15: u64 = 0x1401e5380;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x50); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x50); fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a8610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x158; const P1: u64 = 0x1401e1560; const P2: u64 = 0x168; const P3: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a86d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x1401e5420; const P2: u64 = 0x1401e5380; const P3: u64 = 0x40; const P4: u64 = 0x80; const P5: u64 = 0x14240ab10; const P6: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a89c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x1401e5420; const P2: u64 = 0x1401e5380; const P3: u64 = 0x1403cd300;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a8e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x248; const P1: u64 = 0x1401e1560; const P2: u64 = 0x240; const P3: u64 = 0x250; const P4: u64 = 0x60; const P5: u64 = 0x70;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a8f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1401e1560; const P2: u64 = 0x90; const P3: u64 = 0xc0; const P4: u64 = 0xb8; const P5: u64 = 0x20; const P6: u64 = 0x1403e6fe0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a9010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x248; const P1: u64 = 0x1401e1560; const P2: u64 = 0x240; const P3: u64 = 0x238; const P4: u64 = 0x230; const P5: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425a9800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1a0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x50; const P3: u64 = 0x14026d680; const P4: u64 = 0xa0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425aac40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1b0; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x160; const P3: u64 = 0x110; const P4: u64 = 0xc0; const P5: u64 = 0x70; const P6: u64 = 0x200; const P7: u64 = 0x14240ab10; const P8: u64 = 0x248; const P9: u64 = 0x290; const P10: u64 = 0x2d8; const P11: u64 = 0x320; const P12: u64 = 0x140f5c3a0; const P13: u64 = 0x1401fb4d0; const P14: u64 = 0x1402128c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x320); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x368); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ab07c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140266d90; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = rbp.wrapping_add(0x1f8);
    rcx = ((rcx).wrapping_add(0xd8u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x38;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ab0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x108; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x550; const P4: u64 = 0x3b0; const P5: u64 = 0x140266d90; const P6: u64 = 0xe0; const P7: u64 = 0x3e8; const P8: u64 = 0xb8; const P9: u64 = 0x420; const P10: u64 = 0x90; const P11: u64 = 0x458;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x490); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ab250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140f63670; const P1: u64 = 0x1401e18a0; const P2: u64 = 0x1401fb4d0; const P3: u64 = 0x140f63680; const P4: u64 = 0x14240ab10; const P5: u64 = 0x1401de3c0; const P6: u64 = 0x140f636c0; const P7: u64 = 0x1402128c0; const P8: u64 = 0x1401efe90; const P9: u64 = 0x140f63630; const P10: u64 = 0x1401e81c0; const P11: u64 = 0x140f63640; const P12: u64 = 0x140f63370; const P13: u64 = 0x140f63990; const P14: u64 = 0x140f63900; const P15: u64 = 0x140f632c0; const P16: u64 = 0x140f63a00; const P17: u64 = 0x14026d670; const P18: u64 = 0x1401e3aa0; const P19: u64 = 0x141e14610; const P20: u64 = 0x142646d10; const P21: u64 = 0x140262f30; const P22: u64 = 0x141e15940; const P23: u64 = 0x142646d40; const P24: u64 = 0x140f633f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0xc8); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = rdx.wrapping_add(0x168); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = rdx.wrapping_add(0x168); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = rdx.wrapping_add(0xc8); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = rdx.wrapping_add(0xc8); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x378u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P23), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xc0u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P24, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            131 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            132 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            133 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            134 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            135 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            136 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            137 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            138 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            139 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            140 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            141 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            142 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            143 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            144 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            145 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            146 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            147 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            148 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            149 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            150 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            151 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            152 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            153 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425abc08(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x19;
    rdx = 0x10;
    rcx = rbp.wrapping_add(0x1b0);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425abc35(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = rbp.wrapping_add(0x1b0);
    rcx = ((rcx).wrapping_add(0x190u64)) as u64;
    r9 = P0;
    r8 = 0x19;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425abc69(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = rbp.wrapping_add(0x1b0);
    rcx = ((rcx).wrapping_add(0x320u64)) as u64;
    r9 = P0;
    r8 = 0x19;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425abd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2b0; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x60; const P3: u64 = 0x1401e3aa0; const P4: u64 = 0x1a0; const P5: u64 = 0x14240ab10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}
